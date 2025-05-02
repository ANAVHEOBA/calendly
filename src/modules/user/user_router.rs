use actix_web::{web, Scope};
use crate::modules::user::user_controller::UserController;
use crate::errors::error::AppError;

pub fn user_routes() -> Result<Scope, AppError> {
    let controller = UserController::new()?;
    let controller = web::Data::new(controller);
    
    Ok(web::scope("/api/users")
        .app_data(controller.clone())
        .service(
            web::resource("/register")
                .route(web::post().to(|data, controller: web::Data<UserController>| {
                    async move { controller.register(data).await }
                }))
        )
        .service(
            web::resource("/login")
                .route(web::post().to(|data, controller: web::Data<UserController>| {
                    async move { controller.login(data).await }
                }))
        )
        .service(
            web::resource("/verify-email")
                .route(web::post().to(|data, controller: web::Data<UserController>| {
                    async move { controller.verify_email(data).await }
                }))
        )
        .service(
            web::resource("/refresh-token")
                .route(web::post().to(|data, controller: web::Data<UserController>| {
                    async move { controller.refresh_token(data).await }
                }))
        )

        .service(
            web::resource("/forgot-password")
                .route(web::post().to(|data, controller: web::Data<UserController>| {
                    async move { controller.forgot_password(data).await }
                }))
        )
        .service(
            web::resource("/reset-password")
                .route(web::post().to(|data, controller: web::Data<UserController>| {
                    async move { controller.reset_password(data).await }
                }))
        ))
}
