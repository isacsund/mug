// 3rd party imports {{{
use clap::Clap;
// }}}

// Own imports {{{
use crate::aur;
// }}}

// Pretty print lists
macro_rules! printlist {
    ($h:expr, $v:expr) => {{
        println!("{}", $h);
        for v in $v {
            println!("{:indent$}{}", "", v, indent=2)
        }
    }};
}

#[derive(Clap)]
pub struct CliArgs {
    packages: Vec<String>,
}

pub async fn handler(args: CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    let aur = aur::Handle::new();

    // TODO: check if all packages were found
    // raur.info doesn't return any errors if the package isn't found
    let packages = aur.info(&args.packages).await?;

    for package in packages {
        println!("Repository: aur");
        println!("Name: {}", &package.name);
        println!("Version: {}", &package.version);
        println!("Description: {}", &package.description.unwrap_or("".to_string()));
        println!("URL: {}", &package.url.unwrap_or("".to_string()));
        println!("Maintainer: {}", &package.maintainer.unwrap_or("".to_string()));
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
