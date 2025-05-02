use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::modules::user::user_schema::Claims;
use crate::errors::error::AppError;
use crate::config::environment::Environment;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|value| value.to_str().ok());

        let token = match auth_header {
            Some(header) if header.starts_with("Bearer ") => {
                Some(header[7..].to_string())
            }
            _ => None,
        };

        let token = match token {
            Some(token) => token,
            None => {
                return Box::pin(async move {
                    Err(AppError::Unauthorized("No token provided".to_string()).into())
                });
            }
        };

        let env = Environment::load();
        let validation = Validation::default();
        let token_data = match decode::<Claims>(
            &token,
            &DecodingKey::from_secret(env.get_jwt_secret().as_bytes()),
            &validation,
        ) {
            Ok(data) => data,
            Err(_) => {
                return Box::pin(async move {
                    Err(AppError::Unauthorized("Invalid token".to_string()).into())
                });
            }
        };

        req.extensions_mut().insert(token_data.claims);

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
