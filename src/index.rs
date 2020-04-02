use crate::common::*;

#[derive(Deserialize, Template)]
#[serde(deny_unknown_fields)]
#[template(path = "index.html")]
pub(crate) struct Index {
  pub(crate) name:     String,
  pub(crate) links:    Vec<Link>,
  pub(crate) sections: Vec<Section>,
  pub(crate) style:    Style,
}
