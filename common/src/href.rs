//! # URI reference
//!
//! This [HRef] struct utilizes an **href** attribute whose value must be a valid URI reference
//! [RFC 3986](https://datatracker.ietf.org/doc/html/rfc3986), where the path components
//! may be absolute or relative, the reference has no query component,
//! and the fragment consists of the value of the **id** of the referenced DMN element.

use self::errors::*;
use crate::DsntkError;
use uriparse::URIReference;

/// URI reference used for utilizing `href` attribute.
#[derive(Debug, Clone, PartialEq)]
pub struct HRef {
  /// Namespace built from URI's path components.
  namespace: Option<String>,
  /// DMN element's identifier built from URI's fragment.
  id: String,
}

impl HRef {
  /// Returns the optional namespace.
  pub fn namespace(&self) -> Option<&String> {
    self.namespace.as_ref()
  }

  /// Returns the identifier.
  pub fn id(&self) -> &str {
    &self.id
  }
}

impl TryFrom<&str> for HRef {
  type Error = DsntkError;

  /// Converts [HRef] from string.
  fn try_from(s: &str) -> Result<Self, Self::Error> {
    match URIReference::try_from(s) {
      Ok(mut uri_reference) => {
        uri_reference.normalize();
        let (scheme, authority, path, query, fragment) = uri_reference.into_parts();
        if query.is_some() {
          return Err(err_query_not_allowed(s));
        }
        if fragment.is_none() {
          return Err(err_fragment_is_missing(s));
        }
        let id = fragment.unwrap().to_string();
        let base_uri = URIReference::builder()
          .with_scheme(scheme)
          .with_authority(authority)
          .with_path(path)
          .build()
          .unwrap() // this unwrap is ok: scheme, authority and path are valid
          .to_string();
        let namespace = if base_uri.is_empty() { None } else { Some(base_uri) };
        Ok(Self { namespace, id })
      }
      Err(reason) => Err(err_invalid_reference(s, reason.to_string())),
    }
  }
}

mod errors {
  use crate::{DsntkError, ToErrorMessage};

  /// Errors reported by [HRef](crate::href::HRef).
  #[derive(ToErrorMessage)]
  struct HRefError(String);

  /// Creates an error indicating an invalid reference.
  pub fn err_invalid_reference(s: &str, reason: String) -> DsntkError {
    HRefError(format!("invalid reference '{s}', reason: {reason}")).into()
  }

  /// Creates an error indicating the missing fragment.
  pub fn err_fragment_is_missing(s: &str) -> DsntkError {
    HRefError(format!("fragment is missing in reference: '{s}'")).into()
  }

  /// Creates an error indicating that query is not allowed.
  pub fn err_query_not_allowed(s: &str) -> DsntkError {
    HRefError(format!("query is not allowed in reference: '{s}'")).into()
  }
}
