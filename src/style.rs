use crate::common::*;

#[derive(Deserialize, Template)]
#[serde(deny_unknown_fields)]
#[template(path = "index.css", escape = "none")]
pub(crate) struct Style {
  #[serde(default)]
  imports:        Vec<String>,
  main_font:      String,
  header_font:    String,
  #[serde(default)]
  letter_spacing: String,
}
