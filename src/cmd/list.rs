// 3rd party imports {{{
use clap::Clap;

// }}}

// Own imports {{{
use crate::alpm;
use crate::error::Error;
// }}}

#[derive(Clap)]
pub struct CliArgs {
    /// List only packages installed from unofficial repositories.
    #[clap(short, long)]
    unofficial: bool,
}

pub async fn handler(args: CliArgs) -> Result<(), Error> {
    let alpm = alpm::Handle::new()?;

    let db = alpm.client().localdb();
    let mut packages = db.pkgs().iter().collect::<Vec<_>>();

    // Filter out official packages if the `--unofficial` flag is present.
    if args.unofficial {
        packages.retain(|p| !alpm.is_official_package(p));
    }

    // TODO: pretty print with a header e.g.
    // Package Version
    // ------- ----------
    // rust    1:1.50.0-2
    for package in packages {
        println!(
            "{name} {version}",
            name = package.name(),
            version = package.version(),
        );
    }

    Ok(())
}
