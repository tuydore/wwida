use std::{path::PathBuf, str::FromStr};

use clap::Parser;

use crate::components::tasks::Tasks;

use self::command::Command;

pub(crate) mod command;
pub(crate) mod summary;
pub(crate) mod update;

fn get_filepath() -> PathBuf {
    if let Ok(s) = std::env::var("WWIDA_CACHE") {
        let filepath = PathBuf::from_str(&s).unwrap();
        std::fs::create_dir_all(filepath.parent().expect("wwida cache file has no parent directory"))
            .expect("could not create wwida cache file parent directory");
        filepath
    } else {
        let mut filepath = home::home_dir().expect("no home directory found");
        filepath.push(".wwida_cache");
        filepath
    }
}

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about=None)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    command: Command,
}

impl Cli {
    pub(crate) fn run(self) -> anyhow::Result<()> {
        let filepath = get_filepath();
        let mut tasks = Tasks::load(&filepath)?;
        self.command.run(&mut tasks)?;
        tasks.save(&filepath)?;
        Ok(())
    }
}
