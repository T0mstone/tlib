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
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CargoProfile {
    Debug,
    Release,
    Unknown,
}

impl CargoProfile {
    #[cfg(profile_debug)]
    /// The profile you are compiling with
    pub fn this() -> Self {
        CargoProfile::Debug
    }

    #[cfg(profile_release)]
    /// The profile you are compiling with
    pub fn this() -> Self {
        CargoProfile::Release
    }

    #[cfg(not(any(profile_debug, profile_release)))]
    /// The profile you are compiling with
    pub fn this() -> Self {
        CargoProfile::Unknown
    }
}
