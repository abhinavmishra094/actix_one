use crate::utils;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::error::ErrorUnauthorized;
use actix_web::Error;
use futures::future::{err, ok, Either, Ready};

pub struct AuthorizationService;

impl<S, B> Transform<S> for AuthorizationService
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = TokenAuthorizationMiddelWare<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(TokenAuthorizationMiddelWare { service: service })
    }
}

pub struct TokenAuthorizationMiddelWare<S> {
    service: S,
}

impl<S, B> Service for TokenAuthorizationMiddelWare<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let _auth = req.headers().get("Authorization");
        match _auth {
            Some(_) => {
                let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
                let token = _split[1].trim();

                match utils::jwtauth::validate_token(token) {
                    Ok(_token) => Either::Left(self.service.call(req)),
                    Err(_e) => Either::Right(err(ErrorUnauthorized("not authorized"))),
                }
            }
            None => Either::Right(err(ErrorUnauthorized("not authorized"))),
        }
    }
}
