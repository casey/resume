use crate::common::*;

pub(crate) fn tidy(html: &str) -> Result<String> {
  let command = &[
    "tidy",
    "-indent",
    "-quiet",
    "-wrap",
    "100",
    "--tidy-mark",
    "no",
  ]
  .iter()
  .cloned()
  .map(str::to_string)
  .collect::<Vec<String>>();

  let mut child = Command::new(command[0].clone())
    .args(&command[1..])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()?;

  child.stdin.as_mut().unwrap().write_all(html.as_bytes())?;

  let output = child.wait_with_output()?;

  if !output.status.success() {
    Err(io::Error::new(
      io::ErrorKind::Other,
      format!("Tidy failed: {}\ninput:\n{}", output.status, html),
    ))?;
  }

  let text = String::from_utf8(output.stdout)?;

  Ok(text)
}
