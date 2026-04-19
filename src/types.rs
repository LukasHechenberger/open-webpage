use napi_derive::napi;
use schemars::JsonSchema;

#[napi(object)]
#[derive(Debug, Default, JsonSchema)]
pub struct OpenWebpageOptions {
  /// The URL to open
  pub url: String,

  /// The window's title
  pub title: Option<String>,

  /// If the webpage should be opened fullscreen
  pub fullscreen: Option<bool>,

  /// Enables devtools
  pub devtools: Option<bool>,

  // MARK: Platform-specific
  /// **macOS only** If the titlebar should be hidden
  pub titlebar_hidden: Option<bool>,

  /// **macOS only** If the window title should be hidden
  pub title_hidden: Option<bool>,
}
