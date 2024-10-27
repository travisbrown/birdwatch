use cli_helpers::prelude::*;
use model::{NoteEntry, NoteStatusHistoryEntry};
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
        Command::UpdateNoteStatusHistory {
            aliases,
            unknown_aliases,
            notes,
            note_status_history,
        } => {
            let aliases = model::ParticipantAliasMapping::read(aliases)?;
            let mut unknown_aliases_values =
                model::ParticipantNoteIdMapping::read(&unknown_aliases)?;
            let note_status_history = model::NoteStatusHistoryEntry::read(note_status_history)?;

            ::log::info!("{}", note_status_history.len());

            let mut note_entries = model::NoteEntry::read(&notes)?;

            for NoteStatusHistoryEntry {
                note_id,
                participant_id,
                created_at_ms,
                current_status,
            } in note_status_history.values()
            {
                let entry = note_entries.entry(*note_id).or_insert(NoteEntry {
                    note_id: *note_id,
                    created_at_ms: 0,
                    alias: None,
                    tweet_id: None,
                    user_id: None,
                    misleading: None,
                    helpful: None,
                });

                entry.created_at_ms = *created_at_ms;
                entry.alias = aliases.get(participant_id).cloned();
                entry.helpful = current_status.is_helpful();

                if entry.alias.is_none() {
                    let entries = unknown_aliases_values
                        .entry(participant_id.clone())
                        .or_default();

                    entries.push(*note_id);
                    entries.sort();
                    entries.dedup();
                }
            }

            model::NoteEntry::write(notes, note_entries)?;
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
    #[error("Duplicate note status history entry")]
    DuplicateNoteStatusHistoryEntry(u64),
    #[error("Duplicate note entry")]
    DuplicateNoteEntry(u64),
    #[error("Invalid millisecond timestamp")]
    InvalidTimestamp(u64),
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
    UpdateNoteStatusHistory {
        #[clap(long, default_value = "data/aliases.csv")]
        aliases: PathBuf,
        #[clap(long, default_value = "workspace/unknown-aliases.csv")]
        unknown_aliases: PathBuf,
        #[clap(long, default_value = "data/notes/")]
        notes: PathBuf,
        #[clap(long)]
        note_status_history: PathBuf,
    },
}
