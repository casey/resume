use crate::common::*;

mod body;
mod common;
mod index;
mod item;
mod link;
mod section;
mod style;
mod tidy;
mod url;

fn run() -> Result<()> {
  eprintln!("Reading index.yaml…");
  let yaml = fs::read_to_string("index.yaml")?;

  eprintln!("Deserializing index…");
  let index = serde_yaml::from_str::<Index>(&yaml)?;

  eprintln!("Rendering index.html…");
  let html = index.render()?;

  eprintln!("Tidying html…");
  let tidy = tidy(&html)?;

  let dst = "www/index.html";

  eprintln!("Writing `{}`…", dst);
  fs::write(dst, tidy)?;

  eprintln!("Rendering CSS…");
  let css = index.style.render()?;

  let dst = "www/index.css";
  eprintln!("Writing `{}`…", dst);
  fs::write(dst, css)?;

  Ok(())
}

fn main() {
  if let Err(err) = run() {
    eprintln!("{}", err);
    process::exit(1);
  }
}
