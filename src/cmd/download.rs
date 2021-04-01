// 3rd party imports {{{
use clap::Clap;

// }}}

// Own imports {{{
use crate::aur;
use crate::error::Error;
// }}}

#[derive(Clap)]
pub struct CliArgs {
    packages: Vec<String>,
}

pub async fn handler(args: CliArgs) -> Result<(), Error> {
    let aur = aur::Handle::new();

    let _ = aur.download(args.packages)?;

    Ok(())
}
