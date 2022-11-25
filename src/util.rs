use std::path::PathBuf;

pub fn file_exists(path: &str) -> Result<PathBuf, String> {
  let path_buf = PathBuf::from(path);

  if !path_buf.is_file() {
    return Err::<PathBuf, String>("Path to config file is invalid".to_owned());
  }

  Ok(path_buf)
}
