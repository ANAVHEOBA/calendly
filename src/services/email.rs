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
        code: &str,
    ) -> Result<(), AppError> {
        let email = Message::builder()
            .from(self.from_email.parse().unwrap())
            .to(to_email.parse().unwrap())
            .subject("Your Calendly Verification Code")
            .body(format!(
                r#"
                <h1>Welcome to Calendly!</h1>
                <p>Your verification code is:</p>
                <h2 style="font-size: 24px; padding: 10px; background-color: #f5f5f5; text-align: center;">{}</h2>
                <p>Please enter this code to verify your email address.</p>
                <p>This code will expire in 30 minutes.</p>
                <p>If you didn't create a Calendly account, please ignore this email.</p>
                "#,
                code
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
        code: &str,
    ) -> Result<(), AppError> {
        let email = Message::builder()
            .from(self.from_email.parse().unwrap())
            .to(to_email.parse().unwrap())
            .subject("Reset Your Calendly Password")
            .body(format!(
                r#"
                <h1>Password Reset Code</h1>
                <p>Your password reset code is:</p>
                <h2 style="font-size: 24px; padding: 10px; background-color: #f5f5f5; text-align: center;">{}</h2>
                <p>Enter this code to reset your password.</p>
                <p>This code will expire in 30 minutes.</p>
                <p>If you didn't request a password reset, please ignore this email.</p>
                "#,
                code
            ))
            .map_err(|e| AppError::EmailError(e.to_string()))?;

        self.mailer
            .send(&email)
            .map_err(|e| AppError::EmailError(e.to_string()))?;

        Ok(())
    }
}
