// 3rd party imports {{{
use clap::Clap;
use raur::Raur;

// }}}

// Own imports {{{
use crate::aur;
use crate::config::Config;
use crate::error::Error;
// }}}

#[derive(Clap)]
pub struct CliArgs {
    packages: Vec<String>,
}

pub async fn handler(args: CliArgs, config: Config) -> Result<(), Error> {
    let aur = aur::Handle::new(&config)?;

    let pkgs = aur.rpc.info(&args.packages).await?;
    let pkgs = pkgs.iter().map(|p| p.name.as_str()).collect::<Vec<_>>();
    aur.download.download(&pkgs).await?;
    aur.download.merge(&pkgs)?;
    aur.download.mark_seen(&pkgs)?;

    Ok(())
}
