//! The form for starting the registration process

use askama::Template;
use url::Url;

use crate::manifest::SerializedManifest;

/// The form for starting the registration process
///
/// To register a new GitHub App, the user must send a POST request to the server with the manifest.
/// This is done by submitting a form in the browser so that the user is authenticated with GitHub.
/// This struct defines the fields of the form and renders the template.
#[derive(Clone, Eq, PartialEq, Debug, Template)]
#[template(path = "form.html", escape = "none")]
pub struct Form {
    /// The endpoint of the GitHub API
    form_endpoint: Url,

    /// The manifest for the GitHub App
    manifest: SerializedManifest,
}

#[bon::bon]
impl Form {
    /// Creates a new instance of the form
    #[builder]
    pub(crate) fn new(github: Url, manifest: SerializedManifest) -> Self {
        let base_url = if github.domain() == Some("api.github.com") {
            Url::parse("https://github.com").expect("failed to parse hard-coded URL")
        } else {
            github
        };

        let form_endpoint = base_url
            .join("/settings/apps/new")
            .expect("failed to parse hard-coded URL path");

        Self {
            form_endpoint,
            manifest,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_auto_traits<T: Send + Sync + Unpin>() {}

    #[test]
    fn form_is_send_sync_and_unpin() {
        assert_auto_traits::<Form>();
    }
}
