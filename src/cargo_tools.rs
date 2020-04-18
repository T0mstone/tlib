#[cfg(not(feature = "use_std"))]
use core::{self as std, env};

use std::cmp::Ordering;
use std::hint::unreachable_unchecked;

// note: maybe add a PackageVersionDifference type in the future

/// The version of a Cargo package (with major, minor and patch version)
///
/// For more information, see the [`version`] section in the Cargo book as well as the [official semver website](https://semver.org)
///
/// [`version`]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field
#[derive(Debug, Copy, Clone, Eq, Hash)]
pub struct CargoPackageVersion<'a> {
    major: u64,
    minor: u64,
    patch: u64,
    pre_release: Option<&'a str>,
}

impl<'a, 'b> PartialEq<CargoPackageVersion<'b>> for CargoPackageVersion<'a> {
    fn eq(&self, other: &CargoPackageVersion<'b>) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
            && self.pre_release == other.pre_release
    }
}

impl<'a> Ord for CargoPackageVersion<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .unwrap_or_else(|| unsafe { unreachable_unchecked() })
    }
}

impl<'a, 'b> PartialOrd<CargoPackageVersion<'b>> for CargoPackageVersion<'a> {
    fn partial_cmp(&self, other: &CargoPackageVersion<'b>) -> Option<Ordering> {
        // IMPORTANT: never return `None` from this in any future changes
        //      or else the `Ord` implementation will break
        Some(if self.major == other.major {
            if self.minor == other.minor {
                if self.patch == other.patch {
                    match (self.pre_release, other.pre_release) {
                        (Some(sp), Some(op)) => sp.cmp(op),
                        (None, Some(_)) => Ordering::Greater,
                        (Some(_), None) => Ordering::Less,
                        (None, None) => Ordering::Equal,
                    }
                } else {
                    self.patch.cmp(&other.patch)
                }
            } else {
                self.minor.cmp(&other.minor)
            }
        } else {
            self.major.cmp(&other.major)
        })
    }
}

impl CargoPackageVersion<'static> {
    /// Get the version of your cargo package
    pub fn current() -> Self {
        let major = env!("CARGO_PKG_VERSION_MAJOR").parse();
        let minor = env!("CARGO_PKG_VERSION_MINOR").parse();
        let patch = env!("CARGO_PKG_VERSION_PATCH").parse();
        let pre_release = env!("CARGO_PKG_VERSION_PRE");

        let patch_default = if major.is_err() && minor.is_err() {
            1
        } else {
            0
        };

        Self {
            major: major.unwrap_or(0),
            minor: minor.unwrap_or(0),
            patch: patch.unwrap_or(patch_default),
            pre_release: if pre_release.is_empty() {
                None
            } else {
                Some(pre_release)
            },
        }
    }
}

impl<'a> CargoPackageVersion<'a> {
    /// The major version is the first number in the version string
    pub fn major(&self) -> u64 {
        self.major
    }

    /// The minor version is the second number in the version string
    pub fn minor(&self) -> u64 {
        self.minor
    }

    /// The patch version is the third number in the version string
    pub fn patch(&self) -> u64 {
        self.patch
    }

    /// The pre-release version is an optional appendix
    pub fn pre_release(&self) -> Option<&'a str> {
        self.pre_release
    }
}

/// Information about a Cargo package
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CargoPackageInfo {
    version: CargoPackageVersion<'static>,
    name: &'static str,
    authors: &'static str,
    description: &'static str,
    homepage: &'static str,
    repository: &'static str,
}

impl CargoPackageInfo {
    /// Get the info for your cargo package
    ///
    /// You are able to get the same info via
    /// the various `CARGO_PKG_*` environment variables
    /// but this has a nicer API
    pub fn current() -> Self {
        Self {
            version: CargoPackageVersion::current(),
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

/// A Cargo compilation profile
#[derive(Hash, Debug, Copy, Clone, Eq, PartialEq)]
pub enum CargoProfile {
    /// Cargo's `debug` profile
    Debug,
    /// Cargo's `release` profile
    Release,
    /// Currently, this should be unreachable,
    /// but in theory, Cargo supports arbitrary values for `profile`
    Other(&'static str),
}

impl CargoProfile {
    /// The profile you are compiling with
    pub const fn current() -> Self {
        #[cfg(profile = "debug")]
        return CargoProfile::Debug;
        #[cfg(profile = "release")]
        return CargoProfile::Release;
        // in all other cases (currently this shouldn't be possible)
        #[allow(unused)]
        CargoProfile::Other(env!("profile"))
    }
}
