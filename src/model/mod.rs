use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantAliasMapping {
    pub participant_id: String,
    pub alias: String,
}

impl ParticipantAliasMapping {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<HashMap<String, String>, super::Error> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(File::open(path)?);
        let mut mappings = HashMap::new();

        for mapping in reader.deserialize::<ParticipantAliasMapping>() {
            let ParticipantAliasMapping {
                participant_id,
                alias,
            } = mapping?;

            match mappings.entry(participant_id.clone()) {
                Entry::Occupied(_) => {
                    return Err(super::Error::DuplicateAliasMapping(participant_id, alias));
                }
                Entry::Vacant(entry) => {
                    entry.insert(alias);
                }
            }
        }

        Ok(mappings)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct ParticipantNoteIdMapping {
    pub participant_id: String,
    pub note_id: u64,
}

impl ParticipantNoteIdMapping {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<HashMap<String, Vec<u64>>, super::Error> {
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
            entry.sort();
            entry.dedup();
        }

        Ok(mappings)
    }

    pub fn write<P: AsRef<Path>>(
        path: P,
        values: HashMap<String, Vec<u64>>,
    ) -> Result<(), super::Error> {
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

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Status {
    #[serde(rename = "NEEDS_MORE_RATINGS")]
    NeedsMoreRatings,
    #[serde(rename = "CURRENTLY_RATED_NOT_HELPFUL")]
    NotHelpful,
    #[serde(rename = "CURRENTLY_RATED_HELPFUL")]
    Helpful,
}

impl Status {
    pub fn is_helpful(&self) -> Option<bool> {
        match self {
            Self::NeedsMoreRatings => None,
            Self::NotHelpful => Some(false),
            Self::Helpful => Some(true),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
pub struct NoteStatusHistoryEntry {
    #[serde(rename = "noteId")]
    pub note_id: u64,
    #[serde(rename = "noteAuthorParticipantId")]
    pub participant_id: String,
    #[serde(rename = "createdAtMillis")]
    pub created_at_ms: u64,
    #[serde(rename = "currentStatus")]
    pub current_status: Status,
}

impl NoteStatusHistoryEntry {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<HashMap<u64, Self>, super::Error> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b'\t')
            .from_reader(File::open(path)?);
        let mut entries = HashMap::new();

        for entry in reader.deserialize::<Self>() {
            let note_status_history_entry = entry?;

            match entries.entry(note_status_history_entry.note_id) {
                Entry::Occupied(_) => {
                    return Err(super::Error::DuplicateNoteStatusHistoryEntry(
                        note_status_history_entry.note_id,
                    ));
                }
                Entry::Vacant(entry) => {
                    entry.insert(note_status_history_entry);
                }
            }
        }

        Ok(entries)
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
    pub fn read<P: AsRef<Path>>(path: P) -> Result<HashMap<u64, Self>, super::Error> {
        let mut paths = std::fs::read_dir(&path)?
            .map(|entry| Ok(entry.map_err(super::Error::from)?.path()))
            .collect::<Result<Vec<_>, super::Error>>()?;

        paths.sort();

        let mut entries = HashMap::new();

        for path in paths {
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(true)
                .from_reader(File::open(path)?);

            for entry in reader.deserialize::<Self>() {
                let note_entry = entry?;

                match entries.entry(note_entry.note_id) {
                    Entry::Occupied(_) => {
                        return Err(super::Error::DuplicateNoteEntry(note_entry.note_id));
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(note_entry);
                    }
                }
            }
        }

        Ok(entries)
    }

    pub fn write<P: AsRef<Path>>(path: P, values: HashMap<u64, Self>) -> Result<(), super::Error> {
        let mut values = values
            .values()
            .map(|note_entry| {
                let timestamp =
                    chrono::DateTime::from_timestamp_millis(note_entry.created_at_ms as i64)
                        .ok_or_else(|| super::Error::InvalidTimestamp(note_entry.created_at_ms))?;

                let month = timestamp.format("%Y-%m").to_string();

                Ok((month, note_entry))
            })
            .collect::<Result<Vec<_>, super::Error>>()?;

        values.sort();

        for (month, month_values) in &values.into_iter().chunk_by(|(month, _)| month.clone()) {
            let mut month_values = month_values
                .map(|(_, note_entries)| note_entries)
                .collect::<Vec<_>>();

            month_values.sort();

            let month_path = path.as_ref().join(format!("{}.csv", month));

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
