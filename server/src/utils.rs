use std::env;
use std::path::{Path, PathBuf};

/// Returns canonical current directory.
pub fn current_dir() -> Vec<PathBuf> {
  vec![env::current_dir().expect("failed to access the current directory").canonicalize().unwrap()]
}

pub fn paths_from_variable(variable: &str) -> Vec<PathBuf> {
  let Ok(dir_variable) = env::var(variable) else {
    return vec![];
  };
  if dir_variable.is_empty() {
    return vec![];
  }
  dir_variable
    .split(list_separator())
    .filter_map(|dir| {
      let path = Path::new(&dir);
      if path.exists() && path.is_dir() {
        Some(path.canonicalize().unwrap())
      } else {
        eprintln!("invalid directory specified in environment variable '{}': [{}]", variable, dir);
        None
      }
    })
    .collect()
}

pub fn paths_from_arguments(args: Vec<String>) -> Vec<PathBuf> {
  if args.is_empty() {
    return vec![];
  }
  args
    .iter()
    .filter_map(|dir| {
      let path = Path::new(&dir);
      if path.exists() && path.is_dir() {
        Some(path.canonicalize().unwrap())
      } else {
        eprintln!("invalid directory specified as command line argument: [{}]", dir);
        None
      }
    })
    .collect()
}

/// Returns list separator.
#[cfg(not(target_os = "windows"))]
fn list_separator() -> char {
  ':'
}

/// Returns list separator.
#[cfg(target_os = "windows")]
fn list_separator() -> char {
  ';'
}

#[cfg(test)]
mod tests {
  use super::*;

  const DSNTK_DIR_TEST: &str = "DSNTK_DIR_TEST";

  #[test]
  fn getting_current_dir_should_work() {
    assert!(current_dir().first().unwrap().to_string_lossy().ends_with("/dsntk/server"));
  }

  fn getting_paths_from_variable_01() {
    assert_eq!(Vec::<PathBuf>::new(), paths_from_variable("non_existing_variable_name"));
  }

  fn getting_paths_from_variable_02() {
    env::set_var(DSNTK_DIR_TEST, "");
    assert_eq!(Vec::<PathBuf>::new(), paths_from_variable(DSNTK_DIR_TEST));
  }

  fn getting_paths_from_variable_03() {
    env::set_var(DSNTK_DIR_TEST, current_dir().first().unwrap());
    assert!(paths_from_variable(DSNTK_DIR_TEST).first().unwrap().to_string_lossy().ends_with("/dsntk/server"));
  }

  fn getting_paths_from_variable_04() {
    env::set_var(
      DSNTK_DIR_TEST,
      format!(
        "{}{}{}",
        current_dir().first().unwrap().to_string_lossy(),
        list_separator(),
        current_dir().first().unwrap().to_string_lossy()
      ),
    );
    let paths = paths_from_variable(DSNTK_DIR_TEST);
    assert_eq!(2, paths.len());
    assert!(paths.first().unwrap().to_string_lossy().ends_with("/dsntk/server"));
    assert!(paths.get(1).unwrap().to_string_lossy().ends_with("/dsntk/server"));
  }

  fn getting_paths_from_variable_05() {
    env::set_var(DSNTK_DIR_TEST, format!("{}{}{}", current_dir().first().unwrap().to_string_lossy(), list_separator(), ""));
    let paths = paths_from_variable(DSNTK_DIR_TEST);
    assert_eq!(1, paths.len());
    assert!(paths.first().unwrap().to_string_lossy().ends_with("/dsntk/server"));
  }

  /// All these test cases have to be run sequentially
  /// to avoid environment variable interference.
  #[test]
  fn getting_paths_from_variable_should_work() {
    getting_paths_from_variable_01();
    getting_paths_from_variable_02();
    getting_paths_from_variable_03();
    getting_paths_from_variable_04();
    getting_paths_from_variable_05();
  }

  #[test]
  fn getting_paths_from_empty_command_line_arguments_should_work() {
    let args = vec![];
    let paths = paths_from_arguments(args);
    assert_eq!(0, paths.len());
  }

  #[test]
  fn getting_single_path_from_command_line_arguments_should_work() {
    let args = vec![current_dir().first().unwrap().to_string_lossy().to_string()];
    let paths = paths_from_arguments(args);
    assert_eq!(1, paths.len());
    assert!(paths.first().unwrap().to_string_lossy().ends_with("/dsntk/server"));
  }

  #[test]
  fn getting_multiple_paths_from_command_line_arguments_should_work() {
    let args = vec![
      current_dir().first().unwrap().to_string_lossy().to_string(),
      current_dir().first().unwrap().to_string_lossy().to_string(),
    ];
    let paths = paths_from_arguments(args);
    assert_eq!(2, paths.len());
    assert!(paths.first().unwrap().to_string_lossy().ends_with("/dsntk/server"));
    assert!(paths.get(1).unwrap().to_string_lossy().ends_with("/dsntk/server"));
  }
}
