// std imports {{{
use std::process::Command;
use std::path::PathBuf;
// }}}

// 3rd party imports {{{
use reqwest::{
    Client,
    Url,
};
use serde_derive::Deserialize;

// }}}

// Own imports {{{
use crate::config::Config;
use crate::error::Error;
// }}}

// AUR package definition {{{
/// The package info that a query will return.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Package {
    /// The ID of the package. Mostly used internally,
    /// to not have to reference a package by name.
    #[serde(rename = "ID")]
    pub id: u32,
    /// The name of the package.
    pub name: String,
    /// The ID associated with the git location of the package.
    #[serde(rename = "PackageBaseID")]
    pub package_base_id: u32,
    /// This is the git URL, usually matches the name of the package.
    pub package_base: String,
    /// The package version.
    pub version: String,
    /// The package description.
    pub description: Option<String>,
    /// The URL belonging to the upstream software.
    #[serde(default, rename = "URL")]
    pub url: Option<String>,
    /// The number of votes for the package.
    pub num_votes: u32,
    /// How often the package is downloaded. Decays over time.
    pub popularity: f64,
    /// This is the date that it was marked out-of-date.
    pub out_of_date: Option<i64>,
    /// The name of the package maintainer, if there is one.
    pub maintainer: Option<String>,
    /// The time that the package was first submitted.
    pub first_submitted: i64,
    /// When the package was last updated.
    pub last_modified: i64,
    /// Path to download this package as a tarball.
    /// This must be appended to the domain name, as it does not include it.
    #[serde(default, rename = "URLPath")]
    pub url_path: String,
    /// The names of the groups this package belongs to.
    #[serde(default)]
    pub groups: Vec<String>,
    /// The dependencies of the package itself.
    #[serde(default)]
    pub depends: Vec<String>,
    /// The dependencies that are only relevant
    /// while the package is being built.
    #[serde(default)]
    pub make_depends: Vec<String>,
    /// Optional dependencies needed to enable
    /// certain features.
    #[serde(default)]
    pub opt_depends: Vec<String>,
    /// Dependencies needed for the 'check' stage.
    #[serde(default)]
    pub check_depends: Vec<String>,
    /// The list of packages that this package conflicts with.
    #[serde(default)]
    pub conflicts: Vec<String>,
    /// The list of packages that this package is capable of replacing.
    #[serde(default)]
    pub replaces: Vec<String>,
    /// The namespace this package provides. For example, a *-git
    /// versions of packages provide the same package as non-git versions.
    #[serde(default)]
    pub provides: Vec<String>,
    /// The licenses the package is signed by.
    #[serde(default)]
    pub license: Vec<String>,
    /// Keywords relevant to the package for searching on the AUR.
    #[serde(default)]
    pub keywords: Vec<String>,
}
// }}}

#[derive(Deserialize)]
struct Response {
    #[serde(rename = "type")]
    response_type: String,
    error: Option<String>,
    results: Vec<Package>,
}

/// Result type for this crate
type Result<T> = std::result::Result<T, Error>;

/// A handle for making AUR requests.
#[derive(Clone, Debug)]
pub struct Handle<'a> {
    /// The reqwest client.
    client: Client,
    /// The AUR URL.
    url: &'a Url,
    /// Build directory for packages.
    build_dir: &'a PathBuf,

}

impl<'a> Handle<'a> {
    /// Create a new handle from a config file.
    pub fn from(config: &'a Config) -> Self {
        Handle {
            client: Client::new(),
            url: &config.aur_url,
            build_dir: &config.build_dir,
        }
    }

    /// Download package build files from AUR
    pub fn download<S, I>(&self, packages: I) -> Result<()>
    where
        S: AsRef<str> + Send + Sync,
        I: IntoIterator<Item = S>,
    {
        for package in packages {
            let url = self.url.join(package.as_ref()).expect("Failed to construct package URL");

            let output = Command::new("git")
                .current_dir(self.build_dir)
                .args(&[
                    "clone", url.as_str(),
                ])
                .output()
                .expect("failed to execute process");

            use std::io::Write;
            std::io::stdout().write_all(&output.stdout).unwrap();
            std::io::stderr().write_all(&output.stderr).unwrap();
            println!("{}", output.status);
        }

        Ok(())
    }

    /// A helper function for making a request with given parameters.
    async fn request(&self, params: &[(&str, &str)]) -> Result<Vec<Package>> {
        let url = self.url.join("rpc")?;
        let url = Url::parse_with_params(url.as_str(), params)?;

        let response = self.client.get(url).send().await?;
        let response: Response = response.json().await?;

        if response.response_type == "error" {
            Err(Error::Aur(
                response
                    .error
                    .unwrap_or_else(|| "No error message provided".to_string()),
            ))
        } else {
            Ok(response.results)
        }
    }

    /// Performs an AUR info request.
    pub async fn info<S>(&self, packages: &[S]) -> Result<Vec<Package>>
    where
        S: AsRef<str> + Send + Sync,
    {
        let mut params = packages
            .iter()
            .map(|name| ("arg[]", name.as_ref()))
            .collect::<Vec<_>>();
        params.extend(&[("v", "5"), ("type", "info")]);

        self.request(&params).await
    }

    /// Performs an AUR search request.
    pub async fn search<S>(&self, query: S) -> Result<Vec<Package>>
    where
        S: AsRef<str> + Send + Sync,
    {
        let params = &[
            ("v", "5"),
            ("type", "search"),
            ("by", "name-desc"),
            ("arg", query.as_ref()),
        ];

        self.request(params).await
    }
}
