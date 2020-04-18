#[cfg(not(feature = "use_std"))]
use core::env;

/// The version of your Cargo package (with major, minor and patch version)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CargoPackageVersion {
    major: u64,
    minor: u64,
    patch: u64,
    pre_release: &'static str,
}

#[allow(missing_docs)]
impl CargoPackageVersion {
    /// Get the version of your cargo package
    pub fn this() -> Self {
        Self {
            major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            patch: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
            pre_release: option_env!("CARGO_PGK_VERSION_PRE").unwrap_or(""),
        }
    }

    pub fn major(&self) -> u64 {
        self.major
    }

    pub fn minor(&self) -> u64 {
        self.minor
    }

    pub fn patch(&self) -> u64 {
        self.patch
    }

    pub fn pre_release(&self) -> &'static str {
        self.pre_release
    }
}

/// Allows you to access info about your Cargo package
/// (which is available in the various `CARGO_PKG_*` environment variables)
/// in a nice way
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CargoPackageInfo {
    version: CargoPackageVersion,
    name: &'static str,
    authors: &'static str,
    description: &'static str,
    homepage: &'static str,
    repository: &'static str,
}

impl CargoPackageInfo {
    /// Get the info for your cargo package
    pub fn this() -> Self {
        Self {
            version: CargoPackageVersion::this(),
            name: env!("CARGO_PKG_NAME"),
            authors: env!("CARGO_PKG_AUTHORS"),
            description: env!("CARGO_PKG_DESCRIPTION"),
            homepage: env!("CARGO_PKG_HOMEPAGE"),
            repository: env!("CARGO_PKG_REPOSITORY"),
        }
    }
}

#[allow(missing_docs)]
impl CargoPackageInfo {
    pub fn version(&self) -> CargoPackageVersion {
        self.version
    }
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn authors(&self) -> &'static str {
        self.authors
    }
    pub fn description(&self) -> &'static str {
        self.description
    }
    pub fn homepage(&self) -> &'static str {
        self.homepage
    }
    pub fn repository(&self) -> &'static str {
        self.repository
    }
}

/// The profile you are compiling with
///
/// In addition to this, the build script also provides a `profile` key
/// so that `#[cfg(profile = "debug")]` and `#[cfg(profile = "release")]`
/// can be used for conditional compilation.
#[allow(missing_docs)]
#[derive(Hash, Debug, Copy, Clone, Eq, PartialEq)]
pub enum CargoProfile {
    Debug,
    Release,
}

impl CargoProfile {
    /// The profile you are compiling with
    ///
    /// In case that is neither `debug` nor `release`, this returns `None`
    pub const fn current() -> Option<Self> {
        #[cfg(profile = "debug")]
        return Some(CargoProfile::Debug);
        #[cfg(profile = "release")]
        return Some(CargoProfile::Release);
        // in all other cases (currently this shouldn't be possible)
        #[allow(unused)]
        None
    }
}
