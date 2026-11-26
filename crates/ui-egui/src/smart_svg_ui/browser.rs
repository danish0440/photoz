//! Browser builds retain internal Edit Contents without desktop editor sessions.

use serde_json::Value;

use crate::PhotozApp;
use crate::dialogs::params::ParamFamily;

/// No external editor sessions exist in the browser.
#[derive(Default)]
pub(crate) struct State {}

impl State {
    /// The editor-choice dialog is never shown in the browser.
    pub(crate) fn modal_open(&self) -> bool {
        false
    }
}

/// Leave every command to normal dispatch, so Edit Contents uses the internal editor.
pub(crate) fn intercept(_app: &mut PhotozApp, _id: &str, _params: &Value) -> Option<Result<Value, String>> {
    None
}

/// No sessions to poll.
pub(crate) fn poll(_app: &mut PhotozApp, _ctx: &egui::Context) {}

/// No dialog to host.
pub(crate) fn host(_app: &mut PhotozApp, _ctx: &egui::Context) {}

/// The `PARAM_FAMILIES` row: the browser has no editor-choice dialog, so the row owns no key.
pub(crate) const fn family() -> ParamFamily {
    ParamFamily::new(|_| false, |_, _, _| {})
}
