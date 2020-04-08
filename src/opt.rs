pub(crate) use crate::common::*;

#[derive(StructOpt)]
pub(crate) struct Opt {
  #[structopt(parse(from_os_str))]
  pub(crate) input: PathBuf,
}
