// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{
    config::{Config, XConfig},
    installer::Installer,
    utils::project_root,
    Result,
};
use anyhow::Context;
use x_core::XCoreContext;
use std::path::Path;

/// Global context shared across x commands.
pub struct XContext {
    core: XCoreContext,
    config: Config,
    installer: Installer,
}

impl XContext {
    /// Creates a new `GlobalContext` by reading the config in the project root.
    pub fn new() -> Result<Self> {
        Self::with_config(XConfig::from_project_root()?)
    }

    pub fn with_project_root(root: &'static Path) -> Result<Self> {
        Self::with_config_and_project_root(XConfig::from_file(root.join("x.toml"))?, root)
    }
    /// Creates a new `GlobalContext` based on the given config.
    pub fn with_config(x_config: XConfig) -> Result<Self> {
        let current_dir =
            std::env::current_dir().with_context(|| "error while fetching current dir")?;
        let XConfig { core, config } = x_config;
        Ok(Self {
            core: XCoreContext::new(project_root(), current_dir, core)?,
            installer: Installer::new(config.cargo_config().clone(), config.tools()),
            config,
        })
    }

    pub fn with_config_and_project_root(x_config: XConfig, root: &'static Path) -> Result<Self> {
        let current_dir =
            std::env::current_dir().with_context(|| "error while fetching current dir")?;
        let XConfig { core, config } = x_config;
        Ok(Self {
            core: XCoreContext::new(root, current_dir, core)?,
            installer: Installer::new(config.cargo_config().clone(), config.tools()),
            config,
        })
    }

    /// Returns a reference to the config.
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Returns a reference to the core context.
    pub fn core(&self) -> &XCoreContext {
        &self.core
    }

    /// Returns a reference to Installer, configured to install versions from config.
    pub fn installer(&self) -> &Installer {
        &self.installer
    }
}
