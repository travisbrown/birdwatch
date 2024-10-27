use cli_helpers::prelude::*;
use model::{source::NoteStatusHistoryEntry, Classification, NoteEntry};
use std::{collections::HashMap, path::PathBuf};

mod model;

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    opts.verbose.init_logging()?;

    match opts.command {
        Command::CleanUnknownParticipants {
            aliases,
            unknown_aliases,
            notes_data,
        } => {
            let aliases = model::ParticipantAliasMapping::read(aliases)?;
            let mut unknown_aliases_values =
                model::ParticipantNoteIdMapping::read(&unknown_aliases)?;
            let mut note_entries = model::NoteEntry::read(&notes_data)?;

            unknown_aliases_values.retain(|participant_id, note_ids| {
                let alias = aliases.get(participant_id);

                if let Some(alias) = alias {
                    for note_id in note_ids {
                        match note_entries.get_mut(note_id) {
                            Some(note_entry) => {
                                if note_entry
                                    .alias
                                    .as_ref()
                                    .filter(|note_alias| *note_alias != alias)
                                    .is_some()
                                {
                                    ::log::error!(
                                        "Alias changed (note ID: {}): {}, {}",
                                        note_id,
                                        note_entry.alias.as_ref().unwrap(),
                                        alias,
                                    );
                                }

                                note_entry.alias = Some(alias.clone());
                            }
                            None => {
                                ::log::error!("Missing note entry: {}", note_id);
                            }
                        }
                    }
                }

                alias.is_none()
            });

            model::ParticipantNoteIdMapping::write(unknown_aliases, unknown_aliases_values)?;
            model::NoteEntry::write(notes_data, note_entries)?;
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
        Command::UpdateAliases {
            aliases,
            unknown_aliases,
            new_aliases,
        } => {
            let mut aliases_values = model::ParticipantAliasMapping::read(&aliases)?;
            let unknown_aliases_values = model::ParticipantNoteIdMapping::read(&unknown_aliases)?;
            let new_alias_mappings = model::NoteIdAliasMapping::read(new_aliases)?;

            let unknown_alias_note_ids = unknown_aliases_values
                .iter()
                .flat_map(|(participant_id, note_ids)| {
                    note_ids
                        .iter()
                        .map(|note_id| (*note_id, participant_id.clone()))
                })
                .collect::<HashMap<_, _>>();

            for (note_id, alias) in new_alias_mappings {
                if let Some(participant_id) = unknown_alias_note_ids.get(&note_id) {
                    aliases_values.insert(participant_id.clone(), alias);
                }
            }

            model::ParticipantAliasMapping::write(aliases, aliases_values)?;
        }
        Command::UpdateTweetUsers {
            notes_data,
            new_tweet_users,
        } => {
            let new_tweet_user_mappings = model::TweetIdUserIdMapping::read(new_tweet_users)?;
            let mut note_entries = model::NoteEntry::read(&notes_data)?;

            for (note_id, note_entry) in &mut note_entries {
                if let Some(tweet_id) = note_entry.tweet_id {
                    if let Some(user_id) = new_tweet_user_mappings.get(&tweet_id) {
                        if note_entry
                            .user_id
                            .filter(|note_user_id| *note_user_id != *user_id)
                            .is_some()
                        {
                            ::log::error!(
                                "User ID changed (note ID: {}): {}, {:?}",
                                note_id,
                                note_entry.user_id.unwrap(),
                                user_id,
                            );
                        }

                        note_entry.user_id = Some(*user_id);
                    }
                }
            }

            model::NoteEntry::write(notes_data, note_entries)?;
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
    #[error("Duplicate tweet user")]
    DuplicateTweetUser(u64),
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
        #[clap(long, default_value = "data/notes/")]
        notes_data: PathBuf,
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
    UpdateAliases {
        #[clap(long, default_value = "data/aliases.csv")]
        aliases: PathBuf,
        #[clap(long, default_value = "workspace/unknown-aliases.csv")]
        unknown_aliases: PathBuf,
        #[clap(long)]
        new_aliases: PathBuf,
    },
    UpdateTweetUsers {
        #[clap(long, default_value = "data/notes/")]
        notes_data: PathBuf,
        #[clap(long)]
        new_tweet_users: PathBuf,
    },
}
