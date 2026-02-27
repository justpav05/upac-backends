use crate::errors::{BackendError, Result};

use flate2::read::GzDecoder;
use tar::{Archive, Entry};
use xz2::read::XzDecoder;

use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::path::Path;

pub enum ArchiveDecoder<R: Read> {
    Zst(zstd::Decoder<'static, BufReader<R>>),
    Xz(XzDecoder<R>),
    Gz(GzDecoder<R>),
}

impl<R: Read> Read for ArchiveDecoder<R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        match self {
            ArchiveDecoder::Zst(decoder) => decoder.read(buffer),
            ArchiveDecoder::Xz(decoder) => decoder.read(buffer),
            ArchiveDecoder::Gz(decoder) => decoder.read(buffer),
        }
    }
}

// Открыть архив нужным декодером
pub fn open_archive(path: &Path) -> Result<Archive<ArchiveDecoder<File>>> {
    let file_name = path.to_string_lossy();
    let file = File::open(path)?;

    let decoder = if file_name.ends_with(".pkg.tar.zst") {
        ArchiveDecoder::Zst(zstd::Decoder::new(file)?)
    } else if file_name.ends_with(".pkg.tar.xz") {
        ArchiveDecoder::Xz(XzDecoder::new(file))
    } else if file_name.ends_with(".pkg.tar.gz") {
        ArchiveDecoder::Gz(GzDecoder::new(file))
    } else {
        return Err(BackendError::UnsupportedFormat(file_name.to_string()));
    };

    Ok(Archive::new(decoder))
}

// Найти конкретный файл внутри архива
pub fn find_entry<'a, R: Read>(
    archive: &'a mut Archive<R>,
    file_name: &str,
) -> Result<impl Read + 'a> {
    for entry in archive.entries()? {
        let entry = entry?;
        if entry.path()?.to_str() == Some(file_name) {
            return Ok(entry);
        }
    }
    Err(BackendError::InvalidPackage(format!(
        "{file_name} not found in archive"
    )))
}

// Распаковать gzip поверх уже распакованного entry (.MTREE так устроен)
pub fn decode_gzip(reader: impl Read) -> Result<String> {
    let mut decoder = GzDecoder::new(reader);
    let mut content = String::new();
    decoder.read_to_string(&mut content)?;
    Ok(content)
}

// Проверить — мета-файл ли это (не устанавливается на диск)
pub fn is_meta_file(file_name: &str) -> bool {
    matches!(
        file_name,
        ".PKGINFO" | ".MTREE" | ".BUILDINFO" | ".CHANGELOG"
    )
}

// Распаковать один entry архива в temp_dir
pub fn unpack_entry<R: Read>(entry: &mut Entry<R>, temp_dir: &Path) -> Result<()> {
    let entry_path = entry.path()?.to_path_buf();
    let destination = temp_dir.join(&entry_path);

    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }

    entry.unpack(&destination)?;
    Ok(())
}
