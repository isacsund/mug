// 3rd party imports {{{
use clap::Clap;

// }}}

// Own imports {{{
use crate::aur;
use crate::config::Config;
use crate::error::Error;
// }}}

// Pretty print lists
macro_rules! printlist {
    ($h:expr, $v:expr) => {{
        println!("{}", $h);
        for v in $v {
            println!("{:indent$}{}", "", v, indent = 2)
        }
    }};
}

#[derive(Clap)]
pub struct CliArgs {
    packages: Vec<String>,
}

pub async fn handler(args: CliArgs, config: Config) -> Result<(), Error> {
    let aur = aur::Handle::from(&config);

    let packages = aur.info(args.packages.iter()).await?;

    let missing: Vec<&String> = args
        .packages
        .iter()
        .filter(|&name| {
            !packages
                .iter()
                .map(|package| &package.name)
                .any(|p| p == name)
        })
        .collect();

    printlist!("Package was not found:", &missing);
    println!();

    for package in packages {
        println!("Repository: aur");
        println!("Name: {}", &package.name);
        println!("Version: {}", &package.version);
        println!(
            "Description: {}",
            &package.description.unwrap_or_else(|| "".to_string())
        );
        println!("URL: {}", &package.url.unwrap_or_else(|| "".to_string()));
        println!(
            "Maintainer: {}",
            &package.maintainer.unwrap_or_else(|| "".to_string())
        );
        printlist!("Groups:", &package.groups);
        printlist!("Licenses:", &package.license);
        printlist!("Provides:", &package.provides);
        printlist!("Depends on:", &package.depends);
        printlist!("Make dependencies:", &package.make_depends);
        printlist!("Check dependencies:", &package.check_depends);
        printlist!("Optional dependencies:", &package.opt_depends);
        printlist!("Conflicts with:", &package.conflicts);
        println!("Votes: {}", &package.num_votes.to_string());
        println!("Popularity: {}", &package.popularity.to_string());

        println!();
    }

    Ok(())
}
