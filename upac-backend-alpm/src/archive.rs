use crate::errors::Result;
use crate::helpers::{decode_gzip, find_entry, is_meta_file, open_archive, unpack_entry};
use crate::info::{MtreeEntry, PkgInfo};

use upac_core_lib::{ExtractedPackage, FileEntry};

use std::io::{BufReader, Cursor, Read};
use std::path::Path;

pub fn read_pkginfo(path: &Path) -> Result<PkgInfo> {
    let mut archive = open_archive(path)?;
    let entry = find_entry(&mut archive, ".PKGINFO")?;
    let mut content = Vec::new();
    entry.read_to_end(&mut content)?;
    PkgInfo::parse(Cursor::new(content))
}

fn parse_mtree(path: &Path) -> Result<Vec<MtreeEntry>> {
    let mut archive = open_archive(path)?;
    let mut entry = find_entry(&mut archive, ".MTREE")?;
    let mut content = Vec::new();
    entry.read_to_end(&mut content)?;
    let content = decode_gzip(Cursor::new(content))?;

    let entries = content
        .lines()
        .map(str::trim)
        .filter(|line| {
            !line.is_empty()
                && !line.starts_with('#')
                && !line.starts_with("/set")
                && !is_meta_file(line.split_whitespace().next().unwrap_or(""))
        })
        .filter_map(MtreeEntry::parse)
        .collect();

    Ok(entries)
}

pub fn extract(path: &Path, temp_dir: &Path) -> Result<ExtractedPackage> {
    let pkginfo = read_pkginfo(path)?;
    let mtree = parse_mtree(path)?;

    let mut archive = open_archive(path)?;
    for mut entry in archive.entries()? {
        let file_name = entry?.path()?.to_string_lossy().to_string();

        if is_meta_file(&file_name) {
            continue;
        }

        unpack_entry(&mut entry, temp_dir)?;
    }

    let files = mtree
        .into_iter()
        .map(|m| FileEntry {
            relative_path: m.path,
            permissions: m.permissions,
            owner: m.owner,
            group: m.group,
        })
        .collect();

    let format = path
        .extension()
        .map(|e| e.to_string_lossy().to_string())
        .unwrap_or_default();

    Ok(ExtractedPackage {
        name: pkginfo.name,
        version: pkginfo.version,
        format,
        files,
    })
}
