use crate::common::*;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub(crate) enum Body {
  Items { items: Vec<Item> },
  List { entries: Vec<String> },
}
