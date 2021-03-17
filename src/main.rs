// 3rd party imports {{{
use clap::{
    Clap, 
    crate_authors,
    crate_version,
};
// }}}

#[derive(Clap)]
enum SubCommand {
}

#[derive(Clap)]
#[clap(
    author = crate_authors!(),
    version = crate_version!(),
)]
struct CliArgs {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

fn main() {
    let _args = CliArgs::parse();
}
