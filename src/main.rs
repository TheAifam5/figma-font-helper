#![forbid(future_incompatible, rust_2018_compatibility, warnings, clippy::all)]
#![deny(unsafe_code, nonstandard_style, unused, rust_2018_idioms)]

use anyhow::Result;

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};

use actix_web::{web, App, HttpResponse, HttpServer};

use openssl::{
  error::ErrorStack,
  pkcs12::Pkcs12,
  ssl::{SslAcceptor, SslAcceptorBuilder, SslMethod},
};

use chrono::Local;
use ffh::{middleware, route, ServerState};
use std::env;

#[actix_rt::main]
async fn main() -> Result<()> {
  env::set_var("RUST_LOG", "actix_server=info,actix_web=info");

  let log_file_path = {
    let mut path = env::temp_dir();
    path.push(format!("ffh_{}.log", Local::now().timestamp()));
    path
  };

  let stdout = ConsoleAppender::builder().build();

  let requests = FileAppender::builder().build(log_file_path.clone())?;

  let config = Config::builder()
    .appender(Appender::builder().build("stdout", Box::new(stdout)))
    .appender(Appender::builder().build("file", Box::new(requests)))
    .build(Root::builder().appenders(vec!["stdout", "file"]).build(LevelFilter::Info))?;

  let _handle = log4rs::init_config(config)?;

  log::info!("Log path: {}", log_file_path.to_string_lossy());

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
  .await?;

  Ok(())
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
