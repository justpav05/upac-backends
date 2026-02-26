use std::io::BufRead;
use crate::error::{BackendError, Result};

pub struct PkgInfo {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub dependencies: Vec<String>,
    pub size: Option<u64>,
}

struct PkgInfoBuilder {
    name: Option<String>,
    version: Option<String>,
    description: Option<String>,
    dependencies: Vec<String>,
    size: Option<u64>,
}

impl From<PkgInfo> for PackageMetadata {
    fn from(info: PkgInfo) -> Self {
        PackageMetadata {
            name: info.name,
            version: info.version,
            description: info.description,
            dependencies: info.dependencies,
        }
    }
}

pub struct MtreeEntry {
    pub path: PathBuf,
    pub permissions: u32,
    pub owner: u32,
    pub group: u32,
}

impl PkgInfoBuilder {
    fn new() -> Self {
        Self {
            name: None,
            version: None,
            description: None,
            dependencies: Vec::new(),
            size: None,
        }
    }

    pub fn parse(reader: impl BufRead) -> Result<Self> {
        let mut builder = PkgInfoBuilder::new();

        for line in reader.lines() {
            builder.parse_line(&line?);
        }

        builder.build()
    }

    fn parse_line(&mut self, line: &str) {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            return;
        }

        let Some((key, value)) = line.split_once(" = ") else {
            return;
        };

        self.apply(key.trim(), value.trim());
    }

    /// Применяет распarsенную пару ключ-значение
    fn apply(&mut self, key: &str, value: &str) {
        match key {
            "pkgname" => self.name        = Some(value.to_string()),
            "pkgver"  => self.version     = Some(value.to_string()),
            "pkgdesc" => self.description = Some(value.to_string()),
            "depend"  => self.dependencies.push(value.to_string()),
            "size"    => self.size        = value.parse::<u64>().ok(),
            _         => {}
        }
    }

    fn build(self) -> Result<PkgInfo> {
        Ok(PkgInfo {
            name: self.name
                .ok_or_else(|| BackendError::InvalidPackage("Missing pkgname".into()))?,
            version: self.version
                .ok_or_else(|| BackendError::InvalidPackage("Missing pkgver".into()))?,
            description: self.description,
            dependencies: self.dependencies,
            size: self.size,
        })
    }
}
