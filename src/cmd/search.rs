// 3rd party imports {{{
use clap::Clap;
// }}}

// Own imports {{{
use crate::aur;
// }}}

#[derive(Clap)]
pub struct CliArgs {
    package: String,
}

pub async fn handler(args: CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    let aur = aur::Handle::new();

    let packages = aur.search(args.package).await?;

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
