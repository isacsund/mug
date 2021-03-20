// 3rd party imports {{{
use reqwest::Client;
// }}}

/// The default URL used for the AUR.
pub static AUR_URL: &str = "https://aur.archlinux.org/";

/// A handle for making AUR requests.
#[derive(Clone, Debug)]
pub struct Handle {
    /// The reqwest client.
    client: Client,
    /// The AUR URL.
    url: Url,
}

impl Handle {
    /// Create a new handle with default settings
    pub fn new() -> Self {
        let url = Url::parse(AUR_URL).expect("Failed to parse URL");

        Handle {
            client: Client::new(),
            url,
        }
    }
}
