use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerState {
  pub protocol_version: usize,
}

impl Default for ServerState {
  fn default() -> Self {
    Self { protocol_version: 21 }
  }
}
