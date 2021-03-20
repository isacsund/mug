// 3rd party imports {{{
use clap::{
    crate_authors,
    crate_version,
    Clap,
};
// }}}

mod aur;
mod cmd;

#[derive(Clap)]
enum SubCommand {
    Info(cmd::info::CliArgs),
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

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let result = match args.subcmd {
        SubCommand::Info(args) => cmd::info::handler(args).await,
        SubCommand::Search(args) => cmd::search::handler(args).await,
    };

    match result {
        Ok(_) => std::process::exit(0),
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1)
        }
    }
}
