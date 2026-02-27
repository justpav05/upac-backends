use crate::archive;
use crate::errors::BackendError;

use upac_core_lib::{Backend, ExtractedPackage, PackageMetadata};

use std::path::Path;

pub type Result<T> = std::result::Result<T, BackendError>;

pub struct AlpmBackend {
    supported_formats: SupportedFormats,
}

pub enum SupportedFormats {
    PkgTarZst,
    PkgTarXz,
    PkgTarGz,
}

impl SupportedFormats {
    fn as_str(&self) -> &str {
        match self {
            Self::PkgTarZst => "pkg.tar.zst",
            Self::PkgTarXz => "pkg.tar.xz",
            Self::PkgTarGz => "pkg.tar.gz",
        }
    }
}

impl Backend for AlpmBackend {
    fn new() -> Self {
        Self {
            supported_formats: SupportedFormats::PkgTarZst,
        }
    }

    fn supported_formats(&self) -> Vec<&str> {
        vec![
            SupportedFormats::PkgTarZst.as_str(),
            SupportedFormats::PkgTarXz.as_str(),
            SupportedFormats::PkgTarGz.as_str(),
        ]
    }

    fn detect(&self, path: &Path) -> bool {
        self.supported_formats()
            .iter()
            .any(|ext| path.to_string_lossy().ends_with(ext))
    }

    fn read_metadata(&self, path: &Path) -> Result<PackageMetadata> {
        let pkginfo = archive::read_pkginfo(path)?;
        Ok(pkginfo.into())
    }

    fn extract(&self, path: &Path, temp_dir: &Path) -> Result<ExtractedPackage> {
        archive::extract(path, temp_dir)
    }
}
