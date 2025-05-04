use actix_web::{web, Scope};
use crate::modules::calendar::calendar_controller::CalendarController;
use crate::modules::calendar::calendar_schema::{
    CreateCalendarSettingsRequest,
    CreateAvailabilityRequest,
    UpdateAvailabilityRequest,
    CheckAvailabilityRequest,
    CheckTimeSlotRequest,
    CreateEventTypeRequest,
    UpdateEventTypeRequest
};
use crate::modules::user::user_schema::Claims;
use crate::errors::error::AppError;
use crate::middleware::auth::AuthMiddleware;
use crate::app::AppState;

pub fn calendar_routes() -> Result<Scope, AppError> {
    let app_state = AppState::get();
    let controller = CalendarController::new(app_state.db.clone());
    let controller = web::Data::new(controller);

    Ok(web::scope("/calendar")
        .app_data(controller.clone())
        .service(
            web::resource("/settings")
                .wrap(AuthMiddleware)
                .route(web::get().to(|claims: web::ReqData<Claims>, controller: web::Data<CalendarController>| {
                    async move { controller.get_settings(claims).await }
                }))
                .route(web::post().to(|claims: web::ReqData<Claims>, data: web::Json<CreateCalendarSettingsRequest>, controller: web::Data<CalendarController>| {
                    async move { controller.create_settings(claims, data).await }
                }))
                .route(web::put().to(|claims: web::ReqData<Claims>, data: web::Json<CreateCalendarSettingsRequest>, controller: web::Data<CalendarController>| {
                    async move { controller.update_settings(claims, data).await }
                }))
                .route(web::delete().to(|claims: web::ReqData<Claims>, controller: web::Data<CalendarController>| {
                    async move { controller.delete_settings(claims).await }
                }))
        )
        .service(
            web::resource("/availability/check")
                .wrap(AuthMiddleware)
                .route(web::post().to(|claims: web::ReqData<Claims>, data: web::Json<CheckTimeSlotRequest>, controller: web::Data<CalendarController>| {
                    async move { controller.check_time_slot(claims, data).await }
                }))
        )
        .service(
            web::resource("/availability")
                .wrap(AuthMiddleware)
                .route(web::post().to(|claims: web::ReqData<Claims>, data: web::Json<CreateAvailabilityRequest>, controller: web::Data<CalendarController>| {
                    async move { controller.create_availability(claims, data).await }
                }))
        )
        .service(
            web::resource("/availability/{id}")
                .wrap(AuthMiddleware)
                .route(web::put().to(|claims: web::ReqData<Claims>, id: web::Path<String>, data: web::Json<UpdateAvailabilityRequest>, controller: web::Data<CalendarController>| {
                    async move { controller.update_availability(claims, id, data).await }
                }))
                .route(web::delete().to(|claims: web::ReqData<Claims>, id: web::Path<String>, controller: web::Data<CalendarController>| {
                    async move { controller.delete_availability(claims, id).await }
                }))
        )
        .service(
            web::resource("/check-availability")
                .wrap(AuthMiddleware)
                .route(web::post().to(|claims: web::ReqData<Claims>, data: web::Json<CheckAvailabilityRequest>, controller: web::Data<CalendarController>| {
                    async move { controller.check_availability(claims, data).await }
                }))
        )
        .service(
            web::resource("/event-types")
                .wrap(AuthMiddleware)
                .route(web::get().to(|claims: web::ReqData<Claims>, controller: web::Data<CalendarController>| {
                    async move { controller.list_event_types(claims).await }
                }))
                .route(web::post().to(|claims: web::ReqData<Claims>, data: web::Json<CreateEventTypeRequest>, controller: web::Data<CalendarController>| {
                    async move { controller.create_event_type(claims, data).await }
                }))
        )
        .service(
            web::resource("/event-types/{id}")
                .wrap(AuthMiddleware)
                .route(web::put().to(|claims: web::ReqData<Claims>, id: web::Path<String>, data: web::Json<UpdateEventTypeRequest>, controller: web::Data<CalendarController>| {
                    async move { controller.update_event_type(claims, id, data).await }
                }))
                .route(web::delete().to(|claims: web::ReqData<Claims>, id: web::Path<String>, controller: web::Data<CalendarController>| {
                    async move { controller.delete_event_type(claims, id).await }
                }))
        )
    )
}