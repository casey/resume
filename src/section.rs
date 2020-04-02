use crate::common::*;

#[derive(Deserialize)]
pub(crate) struct Section {
  pub(crate) name:       String,
  #[serde(default)]
  pub(crate) emphasized: bool,
  #[serde(flatten)]
  pub(crate) body:       Body,
}
