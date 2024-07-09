use actix_web::web::Data;
use anyhow::{Context, Result};

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};

use actix_web::{web, App, HttpResponse, HttpServer};

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

  let mut server = HttpServer::new(|| {
    App::new()
      .app_data(Data::new(ServerState::new().unwrap()))
      .wrap(middleware::Compress::default())
      .wrap(
        middleware::DefaultHeaders::new()
          .add(("Access-Control-Allow-Origin", "https://www.figma.com"))
          .add(("Access-Control-Allow-Private-Network", "true")),
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
        web::to(HttpResponse::NotFound),
      )
  })
  .bind(("127.0.0.1", 44950))?;

  cfg_if::cfg_if! {
    if #[cfg(all(feature = "rustls", not(feature = "openssl")))] {
      server = server.bind_rustlsi()?;
    } else if #[cfg(all(feature = "openssl", not(feature = "rustls")))] {
      server = server.bind_openssl(("127.0.0.1", 7335), create_ssl_acceptor()?)?;
    }
  };

  server.run().await?;

  Ok(())
}

#[cfg(all(feature = "openssl", not(feature = "rustls")))]
fn create_ssl_acceptor() -> Result<openssl::ssl::SslAcceptorBuilder> {
  use openssl::{
    pkcs12::Pkcs12,
    ssl::{SslAcceptor, SslMethod},
  };

  let pkcs12 = include_bytes!("../assets/figma.pfx");
  let pkcs12 = Pkcs12::from_der(pkcs12)?;
  let password = include_str!("../assets/figma.txt");

  let identity = pkcs12.parse2(password)?;
  let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;

  for ca in identity.ca.context("No CA found")?.iter() {
    acceptor.add_client_ca(ca)?;
  }

  let cert = identity.cert.context("No cert found")?;
  acceptor.set_certificate(&cert)?;

  let pkey = identity.pkey.context("No private key found")?;
  acceptor.set_private_key(&pkey)?;

  Ok(acceptor)
}
