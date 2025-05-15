pub mod ping;
pub mod docker;
pub mod command;
pub mod ssh;
pub mod portainer;

pub mod ossystems;
pub use ossystems::OsSystems;

pub mod traits;
pub use traits::Executor;
