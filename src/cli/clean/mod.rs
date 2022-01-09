use crate::cfg::ConfigOptsBuild;
use anyhow::Result;
use std::path::PathBuf;
use structopt::StructOpt;

/// Build the Rust WASM app and all of its assets.
#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "clean")]
pub struct Clean {}