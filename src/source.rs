use birdsite_birdwatch::model::{NoteEntry, NoteStatusHistoryEntry};
use std::collections::{hash_map::Entry, HashMap};
use std::fs::File;
use std::path::Path;

pub fn read_note_status_history_entries<P: AsRef<Path>>(
    path: P,
) -> Result<HashMap<u64, NoteStatusHistoryEntry<'static>>, crate::Error> {
    let mut archive_file = ArchiveFile::open(path)?;

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .from_reader(archive_file.reader()?);
    let mut entries = HashMap::new();

    for entry in reader.deserialize::<NoteStatusHistoryEntry<'static>>() {
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

pub fn read_note_entries<P: AsRef<Path>>(
    path: P,
) -> Result<HashMap<u64, NoteEntry<'static>>, crate::Error> {
    let mut archive_file = ArchiveFile::open(path)?;

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .from_reader(archive_file.reader()?);
    let mut entries = HashMap::new();

    for entry in reader.deserialize::<NoteEntry<'static>>() {
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
