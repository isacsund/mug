// 3rd party imports {{{
use clap::Clap;
use raur::Raur;
// }}}

#[derive(Clap)]
pub struct CliArgs {
    package: String,
}

pub async fn handler(args: CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    let raur = raur::Handle::new();

    let packages = raur.search(args.package).await?;

    for package in packages {
        let stats = format!("+{} ~{:.2}", package.num_votes, package.popularity);

        print!(
            "{}/{} {} [{}]",
            "aur",
            &package.name,
            &package.version,
            stats,
        );

        print!("\n");
    }

    Ok(())
}
