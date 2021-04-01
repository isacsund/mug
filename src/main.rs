// 3rd party imports {{{
use clap::{
    crate_authors,
    crate_version,
    Clap,
};
// }}}

mod alpm;
mod aur;
mod cmd;
mod error;

#[derive(Clap)]
enum SubCommand {
    Download(cmd::download::CliArgs),
    Info(cmd::info::CliArgs),
    List(cmd::list::CliArgs),
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
        SubCommand::Download(args) => cmd::download::handler(args).await,
        SubCommand::List(args) => cmd::list::handler(args).await,
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
