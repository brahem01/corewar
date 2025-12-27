use std::sync::mpsc::Receiver;
use eframe::{NativeOptions, egui};
use crate::VmSnapshot;

use crate::visualization::corewar_visualizer::CorewarVisualizer;

pub fn run_gui(rx: Receiver<VmSnapshot>) {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([800.0, 600.0])
            .with_resizable(true)
            .with_title("Corewar woriers"),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Corewar Visualization",
        options,
        Box::new(|cc| {
            Ok(Box::new(CorewarVisualizer::new(rx)))
        }),
    );
}