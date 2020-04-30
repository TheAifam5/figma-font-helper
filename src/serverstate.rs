use crate::provider::{
  FontDatabase, FontDatabaseErr, FontProvider, FontProviderErr, PlatformFontProvider,
};
use snafu::{Backtrace, Snafu};

#[derive(Debug, Snafu)]
pub enum ServerStateErr {
  #[snafu(context(false))]
  ProviderError { source: FontProviderErr, backtrace: Backtrace },

  #[snafu(context(false))]
  DatabaseError { source: FontDatabaseErr, backtrace: Backtrace },
}

type Result<T, E = ServerStateErr> = std::result::Result<T, E>;

pub struct ServerState {
  pub protocol_version: usize,
  pub database: FontDatabase,
}

impl ServerState {
  pub fn new() -> Result<Self, ServerStateErr> {
    Ok(Self {
      protocol_version: 21,
      database: FontDatabase::new(Box::new(PlatformFontProvider::new()?))?,
    })
  }
}
