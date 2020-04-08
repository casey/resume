pub(crate) use std::{
  fmt::{self, Display, Formatter},
  fs,
  io::{self, Write},
  path::PathBuf,
  process,
  process::{Command, Stdio},
};

pub(crate) use askama::Template;
pub(crate) use serde::Deserialize;
pub(crate) use structopt::StructOpt;

pub(crate) use crate::tidy::tidy;

pub(crate) use crate::{
  body::Body, index::Index, item::Item, link::Link, opt::Opt, section::Section, style::Style,
  url::Url,
};

pub(crate) type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;
