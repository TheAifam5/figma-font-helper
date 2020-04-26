use crate::provider::FontProvider;

pub struct FontDatabase {
  provider: Box<dyn FontProvider>,
}

impl FontDatabase {
  pub fn new(provider: Box<dyn FontProvider>) -> Self {
    Self { provider }
  }

  pub async fn refresh(&mut self) {}

  pub async fn force_refresh(&mut self) {
    let _fonts = self.provider.get_all_fonts();
  }
}
