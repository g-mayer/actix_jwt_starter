use actix_web::{
    body::{BoxBody, EitherBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use log::warn;
use std::future::{ready, Ready};

use crate::authentication::jwt::services::validate_token;

pub struct AuthenticationCheck;

impl<S, B> Transform<S, ServiceRequest> for AuthenticationCheck
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationCheckMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationCheckMiddleware { service }))
    }
}

pub struct AuthenticationCheckMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationCheckMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        match validate_token(&req) {
            Ok(claims) => {
                req.extensions_mut().insert(claims.clone());

                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res.map_into_left_body())
                })
            }

            Err(err) => {
                warn!("Unable to authenticate in middleware, {:?}", err);
                let error_response = HttpResponse::Unauthorized().finish();
                let request_path = req.into_parts().0;
                let response =
                    ServiceResponse::new(request_path, error_response.map_into_boxed_body());
                Box::pin(async move { Ok(response.map_into_right_body()) })
            }
        }
    }
}
