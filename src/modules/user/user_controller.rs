use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::{thread_rng, Rng};
use crate::modules::user::{
    user_model::User,
    user_schema::{
        CreateUserRequest, LoginRequest, UserResponse, AuthResponse, Claims,
        VerifyEmailRequest, VerificationResponse, RefreshTokenRequest,
        ForgotPasswordRequest, ResetPasswordRequest, TokenResponse,
    },
    user_crud::UserRepository,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::config::environment::Environment;
use crate::services::email::EmailService;
use crate::errors::error::AppError;
use mongodb::bson::DateTime as BsonDateTime;

#[derive(Clone)]
pub struct UserController {
    repository: UserRepository,
    env: Environment,
    email_service: EmailService,
}

impl UserController {
    pub fn new() -> Result<Self, AppError> {
        let env = Environment::load();
        let email_service = EmailService::new(&env)?;
        
        Ok(Self {
            repository: UserRepository::new(),
            env,
            email_service,
        })
    }

    fn generate_jwt(&self, user: &User) -> Result<String, AppError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::minutes(15))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user.id.as_ref().unwrap().to_hex(),
            exp: expiration,
            iat: Utc::now().timestamp(),
            email: user.email.clone(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.env.get_jwt_secret().as_bytes()),
        )
        .map_err(|_| AppError::InternalServerError("JWT encoding failed".to_string()))
    }

    fn generate_refresh_token() -> String {
        let mut rng = thread_rng();
        let token: String = (0..32)
            .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
            .collect();
        token
    }

    fn generate_verification_token() -> String {
        let mut rng = thread_rng();
        let token: String = (0..32)
            .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
            .collect();
        token
    }

    pub async fn register(
        &self,
        user_data: web::Json<CreateUserRequest>,
    ) -> Result<HttpResponse, AppError> {
        // Check if user already exists
        if let Some(_) = self.repository.find_by_email(&user_data.email).await? {
            return Ok(HttpResponse::BadRequest().json("Email already registered"));
        }

        // Hash password
        let hashed_password = hash(user_data.password.as_bytes(), DEFAULT_COST)
            .map_err(|_| AppError::InternalServerError("Password hashing failed".to_string()))?;

        // Create new user
        let mut user = User::new(
            user_data.email.clone(),
            hashed_password,
            user_data.name.clone(),
        );

        // Generate verification token
        let verification_token = Self::generate_verification_token();
        user.set_verification_token(verification_token.clone());

        let created_user = self.repository.create(user).await?;

        // Send verification email
        self.email_service.send_verification_email(&created_user.email, &verification_token).await?;

        Ok(HttpResponse::Created().json(UserResponse {
            id: created_user.id.unwrap().to_hex(),
            email: created_user.email,
            name: created_user.name,
            is_verified: created_user.is_verified,
        }))
    }

    pub async fn login(
        &self,
        credentials: web::Json<LoginRequest>,
    ) -> Result<HttpResponse, AppError> {
        let mut user = self.repository
            .find_by_email(&credentials.email)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

        if !verify(&credentials.password, &user.password)
            .map_err(|_| AppError::InternalServerError("Password verification failed".to_string()))? {
            return Ok(HttpResponse::Unauthorized().json("Invalid credentials"));
        }

        if !user.is_verified {
            return Ok(HttpResponse::Unauthorized().json("Please verify your email first"));
        }

        let access_token = self.generate_jwt(&user)?;
        let refresh_token = Self::generate_refresh_token();
        user.set_refresh_token(refresh_token.clone());
        
        self.repository.update(&user.id.unwrap().to_hex(), &user).await?;

        Ok(HttpResponse::Ok().json(AuthResponse {
            access_token,
            refresh_token,
            user: UserResponse {
                id: user.id.unwrap().to_hex(),
                email: user.email,
                name: user.name,
                is_verified: user.is_verified,
            },
        }))
    }

    pub async fn verify_email(
        &self,
        verification_data: web::Json<VerifyEmailRequest>,
    ) -> Result<HttpResponse, AppError> {
        let mut user = self.repository
            .find_by_verification_token(&verification_data.token)
            .await?
            .ok_or_else(|| AppError::BadRequest("Invalid verification token".to_string()))?;

        user.verify();
        
        self.repository.update(&user.id.unwrap().to_hex(), &user).await?;

        Ok(HttpResponse::Ok().json(VerificationResponse {
            message: "Email verified successfully".to_string(),
        }))
    }

    pub async fn refresh_token(
        &self,
        token_data: web::Json<RefreshTokenRequest>,
    ) -> Result<HttpResponse, AppError> {
        let mut user = self.repository
            .find_by_refresh_token(&token_data.refresh_token)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid refresh token".to_string()))?;

        let access_token = self.generate_jwt(&user)?;
        let refresh_token = Self::generate_refresh_token();
        user.set_refresh_token(refresh_token.clone());
        
        self.repository.update(&user.id.unwrap().to_hex(), &user).await?;

        Ok(HttpResponse::Ok().json(TokenResponse {
            access_token,
            refresh_token,
        }))
    }

    pub async fn forgot_password(
        &self,
        request: web::Json<ForgotPasswordRequest>,
    ) -> Result<HttpResponse, AppError> {
        let mut user = self.repository
            .find_by_email(&request.email)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        let reset_token = Self::generate_verification_token();
        user.set_password_reset_token(reset_token.clone());
        
        self.repository.update(&user.id.unwrap().to_hex(), &user).await?;

        self.email_service.send_password_reset_email(&request.email, &reset_token).await?;

        Ok(HttpResponse::Ok().json(VerificationResponse {
            message: "Password reset email sent".to_string(),
        }))
    }

    pub async fn reset_password(
        &self,
        request: web::Json<ResetPasswordRequest>,
    ) -> Result<HttpResponse, AppError> {
        let mut user = self.repository
            .find_by_password_reset_token(&request.token)
            .await?
            .ok_or_else(|| AppError::BadRequest("Invalid reset token".to_string()))?;

        // Check if token is expired
        if let Some(expires) = user.password_reset_expires {
            if expires < BsonDateTime::now() {
                return Ok(HttpResponse::BadRequest().json("Reset token has expired"));
            }
        }

        // Hash new password
        let hashed_password = hash(request.new_password.as_bytes(), DEFAULT_COST)
            .map_err(|_| AppError::InternalServerError("Password hashing failed".to_string()))?;

        user.password = hashed_password;
        user.clear_password_reset_token();
        
        self.repository.update(&user.id.unwrap().to_hex(), &user).await?;

        Ok(HttpResponse::Ok().json(VerificationResponse {
            message: "Password reset successful".to_string(),
        }))
    }
}
