use std::rc::Rc;

use actix_service::{Service, Transform};
use actix_web::{
  dev::{ServiceRequest, ServiceResponse},
  error,
  http::header::{ORIGIN, REFERER},
  Error,
};
use futures::future::{err, ok, Either, Ready};

pub struct AllowFigmaOnly;

impl<S, B> Transform<S, ServiceRequest> for AllowFigmaOnly
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Transform = AllowFigmaOnlyMiddleware<S>;
  type InitError = ();
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ok(AllowFigmaOnlyMiddleware { service: Rc::new(service) })
  }
}
pub struct AllowFigmaOnlyMiddleware<S> {
  service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AllowFigmaOnlyMiddleware<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

  actix_web::dev::forward_ready!(service);

  fn call(&self, req: ServiceRequest) -> Self::Future {
    let request_host = {
      if let Some(value) = req.headers().get(ORIGIN) {
        value.to_str().ok()
      } else if let Some(value) = req.headers().get(REFERER) {
        value.to_str().ok()
      } else {
        None
      }
    };

    if request_host.is_some() && request_host.unwrap() == "https://www.figma.com" {
      return Either::Left(self.service.call(req));
    }

    Either::Right(err(error::ErrorForbidden("Forbidden")))
  }
}