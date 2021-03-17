// 3rd party imports {{{
use clap::{
    Clap, 
    crate_authors,
    crate_version,
};
// }}}

mod cmd;

#[derive(Clap)]
enum SubCommand {
    Search(cmd::search::CliArgs),
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
