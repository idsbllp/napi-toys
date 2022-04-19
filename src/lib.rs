use napi_derive::napi;
use v_htmlescape::escape;
use napi::{Result};

#[napi]
pub fn escape_html(str: String) -> Result<String> {
  Ok(escape(str.as_str()).to_string())
}
