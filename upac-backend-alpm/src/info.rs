use crate::errors::{BackendError, Result};

use upac_core_lib::PackageMetadata;

use std::path::PathBuf;
use std::io::BufRead;


pub struct PkgInfo {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub dependencies: Vec<String>,
    pub size: Option<u64>,
}

impl From<PkgInfo> for PackageMetadata {
    fn from(info: PkgInfo) -> Self {
        PackageMetadata {
            description: info.description,
            maintainer: None,
            homepage: None,
            license: None,
        }
    }
}

impl PkgInfo {
    pub fn parse(reader: impl BufRead) -> Result<Self> {
        let mut name = None;
        let mut version = None;
        let mut description = None;
        let mut dependencies = Vec::new();
        let mut size = None;

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let Some((key, value)) = line.split_once(" = ") else {
                continue;
            };

            match key.trim() {
                "pkgname" => name        = Some(value.trim().to_string()),
                "pkgver"  => version     = Some(value.trim().to_string()),
                "pkgdesc" => description = Some(value.trim().to_string()),
                "depend"  => dependencies.push(value.trim().to_string()),
                "size"    => size        = value.trim().parse::<u64>().ok(),
                _         => {}
            }
        }

        Ok(PkgInfo {
            name: name.ok_or_else(|| BackendError::InvalidPackage("Missing pkgname".into()))?,
            version: version.ok_or_else(|| BackendError::InvalidPackage("Missing pkgver".into()))?,
            description,
            dependencies,
            size,
        })
    }
}

pub struct MtreeEntry {
    pub path: PathBuf,
    pub permissions: u32,
    pub owner: u32,
    pub group: u32,
}

impl MtreeEntry {
    pub fn parse(line: &str) -> Option<Self> {
        let mut parts = line.split_whitespace();
        let path = parts.next()?;

        let path = PathBuf::from(path.trim_start_matches("./"));

        let mut permissions = 0o755u32;
        let mut owner = 0u32;
        let mut group = 0u32;
        let mut is_dir = false;

        for part in parts {
            let Some((key, value)) = part.split_once('=') else {
                continue;
            };

            match key {
                "mode" => permissions = u32::from_str_radix(value, 8).unwrap_or(0o755),
                "uid"  => owner       = value.parse().unwrap_or(0),
                "gid"  => group       = value.parse().unwrap_or(0),
                "type" => is_dir      = value == "dir",
                _      => {}
            }
        }

        if is_dir {
            return None;
        }

        Some(MtreeEntry { path, permissions, owner, group })
    }
}
