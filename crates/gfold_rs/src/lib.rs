#![allow(confusable_idents)]
#![allow(unused)]
#![warn(missing_docs)]

const VERSION: &str = env!("CARGO_PKG_VERSION");

mod error;

mod prelude;

mod utils;

pub mod trajectories;

pub mod plotting;

pub mod guidance;

pub mod state;

pub mod rocket_config;
