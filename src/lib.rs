//! Rust package crate for `atelier-os`.

/// Core type exported by the crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtelierOs {
    /// Human-readable project name.
    pub name: String,
}

impl AtelierOs {
    /// Create a new instance of the project metadata holder.
    pub fn new() -> Self {
        Self {
            name: "atelier-os".to_string(),
        }
    }

    /// Returns the package version for this crate.
    pub fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}
