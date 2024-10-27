use cli_helpers::prelude::*;
use std::path::PathBuf;

mod model;

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    opts.verbose.init_logging()?;

    match opts.command {
        Command::CleanUnknownParticipants {
            aliases,
            unknown_aliases,
        } => {
            let aliases = model::ParticipantAliasMapping::read(aliases)?;
            let mut unknown_aliases_values =
                model::ParticipantNoteIdMapping::read(&unknown_aliases)?;

            unknown_aliases_values
                .retain(|participant_id, _| !aliases.contains_key(participant_id));

            model::ParticipantNoteIdMapping::write(unknown_aliases, unknown_aliases_values)?;
        }
    }

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("CSV")]
    Csv(#[from] csv::Error),
    #[error("Log initialization error")]
    LogInitialization(#[from] cli_helpers::Error),
    #[error("Duplicate alias mapping")]
    DuplicateAliasMapping(String, String),
}

#[derive(Debug, Parser)]
#[clap(name = "birdwatch", version, author)]
struct Opts {
    #[clap(flatten)]
    verbose: Verbosity,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    CleanUnknownParticipants {
        #[clap(long, default_value = "data/aliases.csv")]
        aliases: PathBuf,
        #[clap(long, default_value = "workspace/unknown-aliases.csv")]
        unknown_aliases: PathBuf,
    },
}
