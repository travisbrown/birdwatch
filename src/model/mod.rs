use itertools::Itertools;
use std::collections::{btree_map::Entry as BEntry, hash_map::Entry as HEntry, HashMap, BTreeMap};
use std::fs::File;
use std::path::Path;

pub mod source;

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Status {
    #[serde(rename = "NEEDS_MORE_RATINGS")]
    NeedsMoreRatings,
    #[serde(rename = "CURRENTLY_RATED_NOT_HELPFUL")]
    NotHelpful,
    #[serde(rename = "CURRENTLY_RATED_HELPFUL")]
    Helpful,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Classification {
    #[serde(rename = "NOT_MISLEADING")]
    NotMisleading,
    #[serde(rename = "MISINFORMED_OR_POTENTIALLY_MISLEADING")]
    Misleading,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct ParticipantAliasMapping {
    pub participant_id: String,
    pub alias: String,
}

impl ParticipantAliasMapping {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<HashMap<String, String>, crate::Error> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(File::open(path)?);
        let mut mappings = HashMap::new();

        for mapping in reader.deserialize::<Self>() {
            let Self {
                participant_id,
                alias,
            } = mapping?;

            match mappings.entry(participant_id.clone()) {
                HEntry::Occupied(_) => {
                    return Err(crate::Error::DuplicateAliasMapping(participant_id, alias));
                }
                HEntry::Vacant(entry) => {
                    entry.insert(alias);
                }
            }
        }

        Ok(mappings)
    }

    pub fn write<P: AsRef<Path>>(
        path: P,
        values: HashMap<String, String>,
    ) -> Result<(), crate::Error> {
        let mut values = values
            .into_iter()
            .map(|(participant_id, alias)| Self {
                participant_id,
                alias,
            })
            .collect::<Vec<_>>();

        values.sort();

        let mut writer = csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(File::create(path)?);

        for value in values {
            writer.serialize(value)?;
        }

        Ok(writer.flush()?)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct ParticipantNoteIdMapping {
    pub participant_id: String,
    pub note_id: u64,
}

impl ParticipantNoteIdMapping {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<HashMap<String, Vec<u64>>, crate::Error> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(File::open(path)?);
        let mut mappings = HashMap::new();

        for mapping in reader.deserialize::<Self>() {
            let Self {
                participant_id,
                note_id,
            } = mapping?;

            let entry: &mut Vec<u64> = mappings.entry(participant_id.clone()).or_default();
            entry.push(note_id);
            entry.sort_unstable();
            entry.dedup();
        }

        Ok(mappings)
    }

    pub fn write<P: AsRef<Path>>(
        path: P,
        values: HashMap<String, Vec<u64>>,
    ) -> Result<(), crate::Error> {
        let mut values = values
            .into_iter()
            .flat_map(|(participant_id, note_ids)| {
                note_ids.into_iter().map(move |note_id| Self {
                    participant_id: participant_id.clone(),
                    note_id,
                })
            })
            .collect::<Vec<_>>();

        values.sort();

        let mut writer = csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(File::create(path)?);

        for value in values {
            writer.serialize(value)?;
        }

        Ok(writer.flush()?)
    }
}

impl Status {
    pub const fn is_helpful(self) -> Option<bool> {
        match self {
            Self::NeedsMoreRatings => None,
            Self::NotHelpful => Some(false),
            Self::Helpful => Some(true),
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct NoteEntry {
    #[serde(rename = "Note ID")]
    pub note_id: u64,
    #[serde(rename = "Created at", alias = "Timestamp")]
    pub created_at_ms: u64,
    #[serde(rename = "Alias")]
    pub alias: Option<String>,
    #[serde(rename = "Tweet ID")]
    pub tweet_id: Option<u64>,
    #[serde(rename = "User ID")]
    pub user_id: Option<u64>,
    #[serde(rename = "Misleading")]
    pub misleading: Option<bool>,
    #[serde(rename = "Helpful", alias = "Accepted")]
    pub helpful: Option<bool>,
}

impl NoteEntry {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<BTreeMap<u64, Self>, crate::Error> {
        let mut paths = std::fs::read_dir(&path)?
            .map(|entry| Ok(entry.map_err(crate::Error::from)?.path()))
            .collect::<Result<Vec<_>, crate::Error>>()?;

        paths.sort();

        let mut entries = BTreeMap::new();

        for path in paths {
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(true)
                .from_reader(File::open(path)?);

            for entry in reader.deserialize::<Self>() {
                let note_entry = entry?;

                match entries.entry(note_entry.note_id) {
                    BEntry::Occupied(_) => {
                        return Err(crate::Error::DuplicateNote(note_entry.note_id));
                    }
                    BEntry::Vacant(entry) => {
                        entry.insert(note_entry);
                    }
                }
            }
        }

        Ok(entries)
    }

    pub fn write<P: AsRef<Path>>(path: P, values: &BTreeMap<u64, Self>) -> Result<(), crate::Error> {
        let mut values = values
            .values()
            .map(|note_entry| {
                let timestamp = chrono::DateTime::from_timestamp_millis(
                    i64::try_from(note_entry.created_at_ms).unwrap_or(i64::MAX),
                )
                .ok_or(crate::Error::InvalidTimestamp(note_entry.created_at_ms))?;

                let month = timestamp.format("%Y-%m").to_string();

                Ok((month, note_entry))
            })
            .collect::<Result<Vec<_>, crate::Error>>()?;

        values.sort();

        for (month, month_values) in &values.into_iter().chunk_by(|(month, _)| month.clone()) {
            let mut month_values = month_values
                .map(|(_, note_entries)| note_entries)
                .collect::<Vec<_>>();

            month_values.sort();

            let month_path = path.as_ref().join(format!("{month}.csv"));

            let mut writer = csv::WriterBuilder::new()
                .has_headers(true)
                .from_writer(File::create(month_path)?);

            for month_value in month_values {
                writer.serialize(month_value)?;
            }

            writer.flush()?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct NoteIdAliasMapping {
    pub note_id: u64,
    pub alias: String,
}

impl NoteIdAliasMapping {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<BTreeMap<u64, String>, crate::Error> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(File::open(path)?);
        let mut mappings = BTreeMap::new();

        for mapping in reader.deserialize::<Self>() {
            let Self { note_id, alias } = mapping?;

            match mappings.entry(note_id) {
                BEntry::Occupied(_) => {
                    return Err(crate::Error::DuplicateNote(note_id));
                }
                BEntry::Vacant(entry) => {
                    entry.insert(alias);
                }
            }
        }

        Ok(mappings)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TweetIdUserIdMapping {
    pub tweet_id: u64,
    pub user_id: u64,
}

impl TweetIdUserIdMapping {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<BTreeMap<u64, u64>, crate::Error> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(File::open(path)?);
        let mut mappings = BTreeMap::new();

        for mapping in reader.deserialize::<Self>() {
            let Self { tweet_id, user_id } = mapping?;

            match mappings.entry(tweet_id) {
                BEntry::Occupied(_) => {
                    return Err(crate::Error::DuplicateTweetUser(tweet_id));
                }
                BEntry::Vacant(entry) => {
                    entry.insert(user_id);
                }
            }
        }

        Ok(mappings)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UserIdScreenNameMapping {
    pub user_id: u64,
    pub screen_name: String,
}

impl UserIdScreenNameMapping {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<BTreeMap<u64, String>, crate::Error> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(File::open(path)?);
        let mut mappings = BTreeMap::new();

        for mapping in reader.deserialize::<Self>() {
            let Self {
                user_id,
                screen_name,
            } = mapping?;

            // If the screen name has changed, we use the latest one.
            mappings.insert(user_id, screen_name);
        }

        Ok(mappings)
    }
}
