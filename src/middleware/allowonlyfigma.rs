use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::{
  dev::{ServiceRequest, ServiceResponse},
  http::header::{ORIGIN, REFERER},
  Error, HttpResponse,
};
use futures::future::{ok, Either, Ready};

pub struct AllowFigmaOnly;

impl<S, B> Transform<S> for AllowFigmaOnly
where
  S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
{
  type Request = ServiceRequest;
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Transform = AllowFigmaOnlyMiddleware<S>;
  type InitError = ();
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ok(AllowFigmaOnlyMiddleware { service })
  }
}
pub struct AllowFigmaOnlyMiddleware<S> {
  service: S,
}

impl<S, B> Service for AllowFigmaOnlyMiddleware<S>
where
  S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
{
  type Request = ServiceRequest;
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.service.poll_ready(cx)
  }

  fn call(&mut self, req: ServiceRequest) -> Self::Future {
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

    return Either::Right(ok(
      req.into_response(
        HttpResponse::Unauthorized()
          .set_header("Access-Control-Allow-Origin", "https://www.figma.com")
          .finish()
          .into_body(),
      ),
    ));
  }
}
