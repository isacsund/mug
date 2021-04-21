// 3rd party imports {{{
use alpm::Alpm;
use alpm::SigLevel;

// }}}

// Own imports {{{
use crate::error::Error;
// }}}

/// The default path to package database.
const DB_PATH: &str = "/var/lib/pacman";

/// The default installation root.
const ROOT: &str = "/";

/// A handle for interacting with alpm.
pub struct Handle {
    client: Alpm,
}

impl Handle {
    /// Create a new handle with default settings.
    pub fn new() -> Result<Self, Error> {
        let alpm = Alpm::new(ROOT, DB_PATH)?;

        alpm.register_syncdb("community", SigLevel::NONE)?;
        alpm.register_syncdb("core", SigLevel::NONE)?;
        alpm.register_syncdb("extra", SigLevel::NONE)?;
        alpm.register_syncdb("multilib", SigLevel::NONE)?;

        Ok(Self { client: alpm })
    }

    /// Getter for this handle's alpm client.
    pub fn client(&self) -> &Alpm {
        &self.client
    }
}
