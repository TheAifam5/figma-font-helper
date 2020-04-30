#![warn(rust_2018_idioms)]
#![warn(clippy::all)]
#![feature(generators, generator_trait)]

use std::{env, io};

use log::warn;
use pretty_env_logger;

use actix_web::{web, App, HttpResponse, HttpServer};
// use font_kit::source::SystemSource;

use openssl::{
  error::ErrorStack,
  pkcs12::Pkcs12,
  ssl::{SslAcceptor, SslAcceptorBuilder, SslMethod},
};

use ffh::{middleware, route, ServerState};

#[actix_rt::main]
async fn main() -> io::Result<()> {
  env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
  pretty_env_logger::init();

  HttpServer::new(|| {
    App::new()
      .app_data(web::Data::new(ServerState::new().unwrap()))
      .wrap(middleware::Compress::default())
      .wrap(
        middleware::DefaultHeaders::new()
          .header("Access-Control-Allow-Origin", "https://www.figma.com"),
      )
      // guard server to allow only requests from figma
      .wrap(middleware::AllowFigmaOnly)
      // enable logger - always register actix-web Logger middleware last
      .wrap(middleware::Logger::default())
      // register version
      .service(route::version::handler)
      // register font_file
      .service(route::fontfile::handler)
      // register font_files
      .service(route::fontfiles::handler)
      // register update
      .service(route::update::handler)
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
