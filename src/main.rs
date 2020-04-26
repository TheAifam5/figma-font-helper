#![warn(rust_2018_idioms)]
#![warn(clippy::all)]
#![feature(generators, generator_trait)]

use std::{env, io};

use log::{info, warn};
use pretty_env_logger;

use actix_service::Service;
use actix_web::http::header::{ORIGIN, REFERER};
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
// use font_kit::source::SystemSource;
use futures::future::{ok, Either};
use serde::Deserialize;

use openssl::error::ErrorStack;
use openssl::ssl::SslAcceptorBuilder;
use openssl::{
  pkcs12::Pkcs12,
  ssl::{SslAcceptor, SslMethod},
};

// mod dto;
mod providers;
// mod query;
// mod routes;

#[derive(Deserialize)]
pub struct ServerState {
  pub protocol_version: usize,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
  env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
  pretty_env_logger::init();

  // let my_str = include_str!("spanish.in");

  info!("Initializing font database...");

  HttpServer::new(|| {
    App::new()
      // .app_data(web::Data::new(SystemSource::new()))
      .app_data(web::Data::new(ServerState::default()))
      .wrap(middleware::Compress::default())
      .wrap(
        middleware::DefaultHeaders::new()
          .header("Access-Control-Allow-Origin", "https://www.figma.com"),
      )
      // guard server to allow only requests from figma
      .wrap_fn(|req, srv| {
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
          return Either::Left(srv.call(req));
        }

        return Either::Right(ok(
          req.into_response(
            HttpResponse::Unauthorized()
              .set_header("Access-Control-Allow-Origin", "https://www.figma.com")
              .finish()
              .into_body(),
          ),
        ));
      })
      // enable logger - always register actix-web Logger middleware last
      .wrap(middleware::Logger::default())
      // register version
      // .service(routes::version::handler)
      // register font_file
      // .service(routes::fontfile::handler)
      // register font_files
      // .service(routes::fontfiles::handler)
      // register update
      // .service(routes::update::handler)
      // default
      .default_service(
        // 404 for GET request
        web::resource("").route(web::route().to(HttpResponse::NotFound)),
      )
  })
  .bind(("127.0.0.1", 18412))?
  .bind_openssl(("127.0.0.1", 7335), create_ssl_acceptor()?)?
  .run()
  .await
}

impl Default for ServerState {
  fn default() -> Self {
    Self { protocol_version: 21 }
  }
}

fn create_ssl_acceptor() -> Result<SslAcceptorBuilder, ErrorStack> {
  let pkcs12 = include_bytes!("../assets/figma.pfx");
  let pkcs12 = Pkcs12::from_der(pkcs12)?;
  let password = include_str!("../assets/figma.txt");

  let identity = pkcs12.parse(password)?;
  let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;

  for x in &identity.chain.unwrap() {
    acceptor.add_client_ca(x)?;
  }
  acceptor.set_certificate(&identity.cert)?;
  acceptor.set_private_key(&identity.pkey)?;

  Ok(acceptor)
}
