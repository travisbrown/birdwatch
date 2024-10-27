use super::Status;
use std::collections::{hash_map::Entry, HashMap};
use std::fs::File;
use std::path::Path;

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
    pub fn read<P: AsRef<Path>>(path: P) -> Result<HashMap<u64, Self>, crate::Error> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b'\t')
            .from_reader(File::open(path)?);
        let mut entries = HashMap::new();

        for entry in reader.deserialize::<Self>() {
            let note_status_history_entry = entry?;

            match entries.entry(note_status_history_entry.note_id) {
                Entry::Occupied(_) => {
                    return Err(crate::Error::DuplicateNoteStatusHistoryEntry(
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
