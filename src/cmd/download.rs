// 3rd party imports {{{
use clap::Clap;

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
    let aur = aur::Handle::from(&config);

    let _ = aur.download(args.packages)?;

    Ok(())
}
