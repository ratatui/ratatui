use clap::ValueEnum;
use color_eyre::Result;

use crate::{run_cargo, Run};

/// Check backend
#[derive(Clone, Debug, clap::Args)]
pub struct CheckBackend {
    /// Backend to check
    pub backend: Backend,
}

/// Test backend
#[derive(Clone, Debug, clap::Args)]
pub struct TestBackend {
    /// Backend to test
    pub backend: Backend,
}

#[derive(Clone, Debug, ValueEnum, PartialEq, Eq)]
pub enum Backend {
    Crossterm,
    Termion,
    Termwiz,
}

impl Run for CheckBackend {
    fn run(self) -> Result<()> {
        if cfg!(windows) && self.backend == Backend::Termion {
            tracing::error!("termion backend is not supported on Windows");
        }
        let backend = match self.backend {
            Backend::Crossterm => "crossterm",
            Backend::Termion => "termion",
            Backend::Termwiz => "termwiz",
        };
        run_cargo(vec![
            "check",
            "--all-targets",
            "--no-default-features",
            "--features",
            backend,
        ])
    }
}

impl Run for TestBackend {
    fn run(self) -> Result<()> {
        if cfg!(windows) && self.backend == Backend::Termion {
            tracing::error!("termion backend is not supported on Windows");
        }
        let backend = match self.backend {
            Backend::Crossterm => "crossterm",
            Backend::Termion => "termion",
            Backend::Termwiz => "termwiz",
        };
        run_cargo(vec![
            "test",
            "--all-targets",
            "--no-default-features",
            "--features",
            backend,
        ])
    }
}
