// std imports {{{
use std::io::Write;

// }}}

// 3rd party imports {{{
use clap::Clap;

// }}}

#[derive(Clap)]
pub struct CliArgs {}

pub async fn handler(_args: CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    let alpm = alpm::Alpm::new("/", "/var/lib/pacman")?;

    let db = alpm.localdb();
    let packages = db.pkgs().iter().collect::<Vec<_>>();

    // TODO: pretty print with a header e.g.
    // Package Version
    // ------- ----------
    // rust    1:1.50.0-2
    for package in packages {
        println!(
            "{name} {version}",
            name = package.name(),
            version = package.version()
        );
    }

    Ok(())
}
