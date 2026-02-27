use crate::archive;
use crate::errors::BackendError;

use upac_core_lib::{Backend, ExtractedPackage, PackageMetadata};

use std::path::Path;
use std::result;

type Result<T> = result::Result<T, upac_core_lib::BackendError>;

pub struct AlpmBackend;

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

impl From<BackendError> for upac_core_lib::BackendError {
    fn from(err: BackendError) -> Self {
        match err {
            BackendError::UnsupportedFormat(s) => upac_core_lib::BackendError::UnsupportedFormat(s),
            BackendError::InvalidPackage(s)    => upac_core_lib::BackendError::InvalidPackage(s),
            BackendError::Io(e)                => upac_core_lib::BackendError::Io(e),
        }
    }
}

impl Backend for AlpmBackend {
	fn name(&self) -> &str { "alpm" }

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
        let pkginfo = archive::read_pkginfo(path).map_err(upac_core_lib::BackendError::from)?;
        Ok(pkginfo.into())
    }

    fn extract(&self, path: &Path, temp_dir: &Path) -> Result<ExtractedPackage> {
        archive::extract(path, temp_dir).map_err(upac_core_lib::BackendError::from)
    }
}
