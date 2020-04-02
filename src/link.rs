use crate::common::*;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Link {
  pub(crate) text: String,
  pub(crate) url:  Url,
}
