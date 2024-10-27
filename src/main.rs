use cli_helpers::prelude::*;
use model::{source::NoteStatusHistoryEntry, Classification, NoteEntry};
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
            notes_data,
            note_status_history,
        } => {
            let aliases = model::ParticipantAliasMapping::read(aliases)?;
            let mut unknown_aliases_values =
                model::ParticipantNoteIdMapping::read(&unknown_aliases)?;
            let note_status_history = NoteStatusHistoryEntry::read(note_status_history)?;
            let mut note_entries = model::NoteEntry::read(&notes_data)?;

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

            model::NoteEntry::write(notes_data, note_entries)?;
            model::ParticipantNoteIdMapping::write(unknown_aliases, unknown_aliases_values)?;
        }
        Command::UpdateNotes {
            aliases,
            unknown_aliases,
            notes_data,
            notes,
        } => {
            let aliases = model::ParticipantAliasMapping::read(aliases)?;
            let mut unknown_aliases_values =
                model::ParticipantNoteIdMapping::read(&unknown_aliases)?;
            let note_status_history = model::source::NoteEntry::read(notes)?;
            let mut note_entries = model::NoteEntry::read(&notes_data)?;

            for model::source::NoteEntry {
                note_id,
                participant_id,
                created_at_ms,
                tweet_id,
                classification,
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

                if entry
                    .tweet_id
                    .filter(|entry_tweet_id| entry_tweet_id != tweet_id)
                    .is_some()
                {
                    ::log::error!(
                        "Tweet ID changed (note ID: {}): {}, {}",
                        note_id,
                        entry.tweet_id.unwrap(),
                        tweet_id,
                    );
                }

                if entry
                    .misleading
                    .filter(|entry_misleading| {
                        *entry_misleading != (*classification == Classification::Misleading)
                    })
                    .is_some()
                {
                    ::log::error!(
                        "Classification changed (note ID: {}): {}, {:?}",
                        note_id,
                        entry.misleading.unwrap(),
                        classification,
                    );
                }

                entry.created_at_ms = *created_at_ms;
                entry.alias = aliases.get(participant_id).cloned();
                entry.tweet_id = Some(*tweet_id);
                entry.misleading = Some(*classification == Classification::Misleading);

                if entry.alias.is_none() {
                    let entries = unknown_aliases_values
                        .entry(participant_id.clone())
                        .or_default();

                    entries.push(*note_id);
                    entries.sort();
                    entries.dedup();
                }
            }

            model::NoteEntry::write(notes_data, note_entries)?;
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
    #[error("Duplicate note value")]
    DuplicateNote(u64),
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
        notes_data: PathBuf,
        #[clap(long)]
        note_status_history: PathBuf,
    },
    UpdateNotes {
        #[clap(long, default_value = "data/aliases.csv")]
        aliases: PathBuf,
        #[clap(long, default_value = "workspace/unknown-aliases.csv")]
        unknown_aliases: PathBuf,
        #[clap(long, default_value = "data/notes/")]
        notes_data: PathBuf,
        #[clap(long)]
        notes: PathBuf,
    },
}
