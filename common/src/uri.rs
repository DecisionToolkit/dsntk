//! # URI

use self::errors::*;
use crate::Result;
use uriparse::{URI, URIReference};

pub type Uri = String;

pub fn to_uri(value: &str) -> Result<Uri> {
  if let Ok(uri_reference) = URIReference::try_from(value)
    && let Ok(uri) = URI::try_from(uri_reference)
  {
    if uri.has_query() || uri.has_fragment() {
      return Err(err_invalid_uri(value));
    }
    return Ok(uri.to_string().trim().trim_end_matches('/').to_string());
  }
  Err(err_invalid_uri(value))
}

/// Returns a string with URL encoded path segments.
pub fn encode_segments(input: &str) -> String {
  input.split('/').map(|s| urlencoding::encode(s).to_string()).collect::<Vec<String>>().join("/")
}

mod errors {
  use crate::{DsntkError, ToErrorMessage};

  /// Errors reported by [Uri](super::Uri).
  #[derive(ToErrorMessage)]
  struct UriError(String);

  /// Creates an error indicating an invalid URI.
  pub fn err_invalid_uri(s: &str) -> DsntkError {
    UriError(format!("invalid URI: '{s}'")).into()
  }
}
