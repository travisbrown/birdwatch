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

        for mapping in reader.deserialize::<ParticipantNoteIdMapping>() {
            let ParticipantNoteIdMapping {
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
                note_ids
                    .into_iter()
                    .map(move |note_id| ParticipantNoteIdMapping {
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
