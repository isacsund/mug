// 3rd party imports {{{
use clap::Clap;
// }}}

#[derive(Clap)]
pub struct CliArgs {
    package: String,
}

pub async fn handler(args: CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
