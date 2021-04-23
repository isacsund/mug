use crate::error::Error;
use crate::Config;

/// A handle for making AUR requests.
#[derive(Clone, Debug)]
pub struct Handle {
    /// The raur handle.
    pub rpc: raur::Handle,
    /// The aur-fetch handle
    pub download: aur_fetch::Handle,
}

impl Handle {
    /// Create a new handle from a config file.
    pub fn new(config: &Config) -> Result<Self, Error> {
        let mut download = aur_fetch::Handle::with_combined_cache_dir(&config.build_dir);
        download.aur_url = config.aur_url.clone();

        let handle = Handle {
            rpc: raur::Handle::new_with_url(config.aur_url.join("rpc")?.as_str()),
            download,
        };

        Ok(handle)
    }
}
