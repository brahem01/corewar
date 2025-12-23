use std::sync::mpsc::Receiver;
use eframe::{NativeOptions, egui};
use vm::VmSnapshot;

use crate::corewar_visualizer::CorewarVisualizer;

pub fn run_gui(rx: Receiver<VmSnapshot>) {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([800.0, 600.0])
            .with_resizable(true)
            .with_title("Corewar Visualization"),
        
        // Enable smooth rendering
        vsync: true,
        
        // Hardware acceleration
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        
        // Multisampling for smoother graphics
        multisampling: 4,
        
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Corewar Visualization",
        options,
        Box::new(|cc| {
            // Customize the visual style
            customize_style(&cc.egui_ctx);
            
            Ok(Box::new(CorewarVisualizer::new(rx)))
        }),
    );
}

fn customize_style(ctx: &egui::Context) {
    use egui::{FontId, TextStyle, Visuals, Stroke};
    
    // Set custom fonts
    let mut style = (*ctx.style()).clone();
    
    // Customize text styles
    style.text_styles = [
        (TextStyle::Heading, FontId::proportional(24.0)),
        (TextStyle::Body, FontId::proportional(14.0)),
        (TextStyle::Monospace, FontId::monospace(13.0)),
        (TextStyle::Button, FontId::proportional(14.0)),
        (TextStyle::Small, FontId::proportional(12.0)),
    ].into();
    
    // Customize spacing
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.button_padding = egui::vec2(8.0, 4.0);
    style.spacing.window_margin = egui::Margin::same(8);
    
    ctx.set_style(style);
    
    // Customize colors (dark theme with better contrast)
    let mut visuals = Visuals::dark();
    
    // Better panel colors
    visuals.panel_fill = egui::Color32::from_rgb(20, 20, 25);
    visuals.window_fill = egui::Color32::from_rgb(25, 25, 30);
    visuals.extreme_bg_color = egui::Color32::from_rgb(15, 15, 20);
    
    // Window shadow (using integer types for egui 0.33)
    visuals.window_shadow = egui::epaint::Shadow {
        offset: [2, 4],
        blur: 16,
        spread: 0,
        color: egui::Color32::from_black_alpha(100),
    };
    
    visuals.popup_shadow = egui::epaint::Shadow {
        offset: [2, 4],
        blur: 12,
        spread: 0,
        color: egui::Color32::from_black_alpha(80),
    };
    
    // Stroke colors
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, egui::Color32::from_rgb(60, 60, 70));
    visuals.selection.bg_fill = egui::Color32::from_rgb(80, 120, 200);
    
    ctx.set_visuals(visuals);
}