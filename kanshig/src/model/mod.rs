//! Model module for kanshi config data structures

pub mod add_output;
pub mod config;
pub mod niri_output;
pub mod output;
pub mod profile;

pub use add_output::AddOutputFocus;
pub use add_output::AddOutputWindowState;
pub use config::KanshiConfig;
pub use niri_output::NiriOutputs;
pub use output::OutputDefinition;
pub use output::UnifiedOutput;
pub use profile::Profile;
