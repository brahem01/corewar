#[cfg(feature = "gui")]
mod corewar_visualizer;

#[cfg(feature = "gui")]
pub mod gui;

#[cfg(feature = "gui")]
pub use gui::run_gui;
