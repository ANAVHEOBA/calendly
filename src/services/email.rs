use lettre::{
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};
use crate::config::environment::Environment;
use crate::errors::error::AppError;

#[derive(Clone)]
pub struct EmailService {
    mailer: SmtpTransport,
    from_email: String,
}

impl EmailService {
    pub fn new(env: &Environment) -> Result<Self, AppError> {
        let credentials = Credentials::new(
            env.email_user.clone(),
            env.email_password.clone(),
        );

        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .map_err(|e| AppError::EmailError(e.to_string()))?
            .credentials(credentials)
            .build();

        Ok(Self {
            mailer,
            from_email: env.email_user.clone(),
        })
    }

    pub async fn send_verification_email(
        &self,
        to_email: &str,
        token: &str,
    ) -> Result<(), AppError> {
        let verification_url = format!("http://localhost:3000/verify-email?token={}", token);
        
        let email = Message::builder()
            .from(self.from_email.parse().unwrap())
            .to(to_email.parse().unwrap())
            .subject("Verify your email")
            .body(format!(
                r#"
                <h1>Welcome to Calendly!</h1>
                <p>Please click the link below to verify your email address:</p>
                <a href="{}">Verify Email</a>
                "#,
                verification_url
            ))
            .map_err(|e| AppError::EmailError(e.to_string()))?;

        self.mailer
            .send(&email)
            .map_err(|e| AppError::EmailError(e.to_string()))?;

        Ok(())
    }

    pub async fn send_password_reset_email(
        &self,
        to_email: &str,
        token: &str,
    ) -> Result<(), AppError> {
        let reset_url = format!("http://localhost:3000/reset-password?token={}", token);
        
        let email = Message::builder()
            .from(self.from_email.parse().unwrap())
            .to(to_email.parse().unwrap())
            .subject("Reset your password")
            .body(format!(
                r#"
                <h1>Password Reset Request</h1>
                <p>Click the link below to reset your password:</p>
                <a href="{}">Reset Password</a>
                <p>If you didn't request this, please ignore this email.</p>
                "#,
                reset_url
            ))
            .map_err(|e| AppError::EmailError(e.to_string()))?;

        self.mailer
            .send(&email)
            .map_err(|e| AppError::EmailError(e.to_string()))?;

        Ok(())
    }
}
