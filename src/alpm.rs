// 3rd party imports {{{
use alpm::Alpm;
use alpm::SigLevel;

// }}}

// Own imports {{{
use crate::config::Config;
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
    /// Create a new handle from the the given `Config`.
    pub fn from(config: &Config) -> Result<Self, Error> {
        let mut alpm = Alpm::new(ROOT, DB_PATH)?;

        for repo in &config.repos {
            let db = alpm.register_syncdb_mut(repo.name.clone(), SigLevel::NONE)?;
            db.set_servers(repo.servers.iter())?;
        }

        Ok(Self { client: alpm })
    }

    /// Getter for this handle's alpm client.
    pub fn client(&self) -> &Alpm {
        &self.client
    }

    /// Check whether a package comes from an official repository.
    pub fn is_official_package(&self, package: &alpm::Package) -> bool {
        let dbs = self.client().syncdbs();

        for db in dbs {
            if db.pkg(package.name()).is_ok() {
                return true
            }
        }

        false
    }
}
