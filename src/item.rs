use crate::common::*;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Item {
  pub(crate) name:   String,
  #[serde(default)]
  pub(crate) link:   Option<Url>,
  #[serde(default)]
  pub(crate) notes:  Option<Vec<String>>,
  #[serde(default)]
  pub(crate) points: Option<Vec<String>>,
  #[serde(default)]
  pub(crate) grid:   Option<Vec<String>>,
  pub(crate) aside:  Option<String>,
}
