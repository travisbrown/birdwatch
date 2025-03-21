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
        let mut archive_file = ArchiveFile::open(path)?;

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b'\t')
            .from_reader(archive_file.reader()?);
        let mut entries = HashMap::new();

        for entry in reader.deserialize::<Self>() {
            let note_status_history_entry = entry?;

            match entries.entry(note_status_history_entry.note_id) {
                Entry::Occupied(_) => {
                    return Err(crate::Error::DuplicateNote(
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

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
pub struct NoteEntry {
    #[serde(rename = "noteId")]
    pub note_id: u64,
    #[serde(rename = "noteAuthorParticipantId")]
    pub participant_id: String,
    #[serde(rename = "createdAtMillis")]
    pub created_at_ms: u64,
    #[serde(rename = "tweetId")]
    pub tweet_id: u64,
    #[serde(rename = "classification")]
    pub classification: super::Classification,
}

impl NoteEntry {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<HashMap<u64, Self>, crate::Error> {
        let mut archive_file = ArchiveFile::open(path)?;

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b'\t')
            .from_reader(archive_file.reader()?);
        let mut entries = HashMap::new();

        for entry in reader.deserialize::<Self>() {
            let note_entry = entry?;

            match entries.entry(note_entry.note_id) {
                Entry::Occupied(_) => {
                    return Err(crate::Error::DuplicateNote(note_entry.note_id));
                }
                Entry::Vacant(entry) => {
                    entry.insert(note_entry);
                }
            }
        }

        Ok(entries)
    }
}

pub enum ArchiveFile {
    Zip(zip::ZipArchive<File>),
    File(File),
}

impl ArchiveFile {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, crate::Error> {
        let extension = path
            .as_ref()
            .extension()
            .and_then(|extension| extension.to_str());

        let file = File::open(&path)?;

        if extension == Some("zip") {
            Ok(Self::Zip(zip::ZipArchive::new(file)?))
        } else {
            Ok(Self::File(file))
        }
    }

    pub fn reader<'a>(&'a mut self) -> Result<Box<dyn std::io::Read + 'a>, crate::Error> {
        match self {
            Self::Zip(archive) => Ok(Box::new(archive.by_index(0)?)),
            Self::File(file) => Ok(Box::new(file)),
        }
    }
}
