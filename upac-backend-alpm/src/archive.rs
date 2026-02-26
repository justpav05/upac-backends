use flate2::read::GzDecoder;
use xz2::read::XzDecoder;

enum ArchiveDecoder<R: std::io::Read> {
    Zst(zstd::Decoder<'static, BufReader<R>>),
    Xz(XzDecoder<R>),
    Gz(GzDecoder<R>),
}

impl<R: std::io::Read> std::io::Read for ArchiveDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            ArchiveDecoder::Zst(d) => d.read(buf),
            ArchiveDecoder::Xz(d)  => d.read(buf),
            ArchiveDecoder::Gz(d)  => d.read(buf),
        }
    }
}

fn open_archive(path: &Path) -> Result<Archive<ArchiveDecoder<File>>> {
    let name = path.to_string_lossy();
    let file = File::open(path)?;

    let decoder = if name.ends_with(".pkg.tar.zst") {
        ArchiveDecoder::Zst(zstd::Decoder::new(file)?)
    } else if name.ends_with(".pkg.tar.xz") {
        ArchiveDecoder::Xz(XzDecoder::new(file))
    } else if name.ends_with(".pkg.tar.gz") {
        ArchiveDecoder::Gz(GzDecoder::new(file))
    } else {
        return Err(BackendError::UnsupportedFormat(name.to_string()));
    };

    Ok(Archive::new(decoder))
}

pub fn read_pkginfo(path: &Path) -> Result<PkgInfo> {
    let entry = find_entry(&mut open_archive(path)?, ".PKGINFO")?;
    PkgInfo::parse(BufReader::new(entry))
}

fn find_entry<'a, R: Read>(
    archive: &'a mut Archive<R>,
    name: &str,
) -> Result<impl Read + 'a> {
    for entry in archive.entries()? {
        let entry = entry?;
        if entry.path()?.to_str() == Some(name) {
            return Ok(entry);
        }
    }
    Err(BackendError::InvalidPackage(format!("{name} not found in archive")))
}
