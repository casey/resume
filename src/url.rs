use crate::common::*;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Url(#[serde(deserialize_with = "url_serde::deserialize")] url::Url);

impl Display for Url {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    self.0.fmt(f)
  }
}
