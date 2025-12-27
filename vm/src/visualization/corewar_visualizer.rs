
use eframe::egui;
use egui::Color32;
use std::sync::mpsc::Receiver;
use crate::{Arena, Process, VmSnapshot};
use std::collections::HashSet;
pub struct CorewarVisualizer {
    rx: Receiver<VmSnapshot>,
    arena: Arena,
    processes: Vec<Process>,
    cycle: usize,
    cell_size: f32,
    cols_per_row: usize,
    winners: HashSet<i32>,
    game_over: bool,
    // Track which player owns each cell (0 = unowned)
    cell_owners: Vec<i32>,
}

impl CorewarVisualizer {
    pub fn new(rx: Receiver<VmSnapshot>) -> Self {
        Self {
            rx,
            arena: Arena::new(),
            processes: Vec::new(),
            cycle: 0,
            cell_size: 12.0,
            cols_per_row: 32,
            winners: HashSet::new(),
            game_over: false,
            cell_owners: vec![0; 4096], // Initialize with 0 (unowned)
        }
    }
}

impl eframe::App for CorewarVisualizer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Read all available snapshots (non-blocking)
        while let Ok(snapshot) = self.rx.try_recv() {
            self.cycle = snapshot.cycle;
            self.arena = snapshot.arena;
            self.processes = snapshot.processes;
            self.winners = snapshot.winners;
            self.game_over = snapshot.game_over;

            // Resize cell_owners if arena size changed
            if self.cell_owners.len() != self.arena.memory.len() {
                self.cell_owners.resize(self.arena.memory.len(), 0);
            }

            // Update cell ownership: whenever a process is at a position, claim it
            for p in &self.processes {
                let pc = p.pc.get();
                if pc < self.cell_owners.len() {
                    self.cell_owners[pc] = p.player_id;
                }
            }
        }

        // Show winner overlay when VM says game is over
        if self.game_over {
            self.show_winner_overlay(ctx);
        }

        // Top panel for controls and info
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("🎮 Corewar Arena").size(20.0));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.game_over {
                        ui.label(
                            egui::RichText::new("🏁 GAME OVER")
                                .size(16.0)
                                .color(Color32::from_rgb(255, 200, 100))
                                .strong(),
                        );
                        ui.separator();
                    }
                    ui.label(
                        egui::RichText::new(format!("⚡ {} Processes", self.processes.len()))
                            .size(14.0)
                            .color(Color32::from_rgb(100, 200, 255)),
                    );
                    ui.separator();
                    ui.label(
                        egui::RichText::new(format!("Cycle: {}", self.cycle))
                            .size(14.0)
                            .strong(),
                    );
                });
            });
            ui.add_space(4.0);
        });

        // Bottom panel for processes
        egui::TopBottomPanel::bottom("processes")
            .resizable(true)
            .default_height(200.0)
            .min_height(100.0)
            .show(ctx, |ui| {
                ui.add_space(4.0);
                ui.heading("Active Processes");
                ui.separator();
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        self.draw_processes(ui);
                    });
            });

        // Side panel for controls
        egui::SidePanel::right("controls")
            .default_width(200.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.add_space(4.0);
                ui.heading("Display Settings");
                ui.separator();
                ui.add_space(8.0);

                ui.label("Cell Size:");
                ui.add(egui::Slider::new(&mut self.cell_size, 12.0..=30.0).suffix("px"));

                ui.add_space(8.0);
                ui.label("Columns:");
                ui.add(egui::Slider::new(&mut self.cols_per_row, 16..=128).logarithmic(true));

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(8.0);

                // Arena stats
                ui.label(egui::RichText::new("Arena Info").strong());
                ui.add_space(4.0);
                ui.label(format!("Memory Size: {} bytes", self.arena.memory.len()));
                ui.label(format!(
                    "Grid: {}x{}",
                    self.cols_per_row,
                    (self.arena.memory.len() + self.cols_per_row - 1) / self.cols_per_row
                ));
            });

        // Central panel for arena
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    self.draw_arena(ui);
                });
        });

        // Request continuous repaint for smooth animation
        ctx.request_repaint();
    }
}

impl CorewarVisualizer {
    fn draw_arena(&self, ui: &mut egui::Ui) {
        use egui::{FontId, Sense, Vec2};

        let font_id = FontId::monospace(self.cell_size * 0.55);
        let spacing = 1.0;

        // Create a process position lookup for PC highlighting
        let mut pc_map = std::collections::HashMap::new();
        for p in &self.processes {
            pc_map.insert(p.pc.get(), p.player_id);
        }

        egui::Grid::new("arena")
            .num_columns(self.cols_per_row)
            .spacing([spacing, spacing])
            .show(ui, |ui| {
                for (i, byte) in self.arena.memory.iter().enumerate() {
                    // Get the owner of this cell from our tracked ownership
                    let owner_id = if i < self.cell_owners.len() {
                        self.cell_owners[i]
                    } else {
                        0
                    };
                    
                    // Color based on cell owner (every cell gets player color if owned)
                    let base_color = if owner_id < 0 {
                        player_color(owner_id)
                    } else {
                        Color32::from_rgb(30, 30, 35) // Unowned cells
                    };

                    // Make PC positions brighter
                    let bg_color = if pc_map.contains_key(&i) {
                        brighten_color(base_color, 1.4)
                    } else {
                        base_color
                    };

                    let text_color = if owner_id != 0 {
                        Color32::WHITE
                    } else {
                        Color32::from_rgb(150, 150, 160)
                    };

                    // Allocate space and draw cell
                    let cell_size = Vec2::splat(self.cell_size);
                    let (rect, response) = ui.allocate_exact_size(cell_size, Sense::hover());

                    // Draw background
                    ui.painter().rect_filled(rect, 1.0, bg_color);

                    // Add border for PC positions
                    if pc_map.contains_key(&i) {
                        ui.painter().rect_stroke(
                            rect,
                            1.0,
                            egui::Stroke::new(2.0, Color32::WHITE),
                            egui::epaint::StrokeKind::Outside,
                        );
                    }

                    // Draw hex value
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        format!("{:02X}", byte),
                        font_id.clone(),
                        text_color,
                    );

                    // Tooltip on hover
                    if response.hovered() {
                        response.on_hover_ui(|ui| {
                            ui.label(format!("Address: 0x{:04X} ({})", i, i));
                            ui.label(format!("Value: 0x{:02X} ({})", byte, byte));
                            if owner_id != 0 {
                                ui.colored_label(
                                    player_color(owner_id),
                                    format!("Owner: Player {}", -owner_id),
                                );
                            } else {
                                ui.colored_label(Color32::GRAY, "Unowned");
                            }
                            if let Some(&player_id) = pc_map.get(&i) {
                                ui.colored_label(
                                    brighten_color(player_color(player_id-1), 1.4),
                                    format!("⚡ Player {} PC", -player_id),
                                );
                            }
                        });
                    }

                    if (i + 1) % self.cols_per_row == 0 {
                        ui.end_row();
                    }
                }
            });
    }

    fn draw_processes(&self, ui: &mut egui::Ui) {
        use egui::RichText;

        if self.processes.is_empty() {
            ui.colored_label(Color32::GRAY, "No active processes");
            return;
        }

        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(8.0, 8.0);
            for p in &self.processes {
                let color = player_color(p.player_id);

                ui.group(|ui| {
                    ui.set_min_width(220.0);
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.colored_label(
                                color,
                                RichText::new(format!("Player {}", p.id))
                                    .strong()
                                    .size(14.0),
                            );
                            ui.separator();
                            ui.label(format!("PID: {}", p.id));
                            ui.separator();
                            // ui.label(format!("name: {}", p.name));
                        });

                        ui.add_space(4.0);

                        // Process details
                        egui::Grid::new(format!("process_{}", p.id))
                            .num_columns(2)
                            .spacing([8.0, 4.0])
                            .show(ui, |ui| {
                                ui.label(RichText::new("PC:").strong());
                                ui.label(format!("0x{:04X} ({})", p.pc.get(), p.pc.get()));
                                ui.end_row();

                                ui.label(RichText::new("Carry:").strong());
                                if p.carry {
                                    ui.colored_label(Color32::from_rgb(100, 255, 100), "✓ True");
                                } else {
                                    ui.colored_label(Color32::from_rgb(255, 100, 100), "✗ False");
                                }
                                ui.end_row();

                                ui.label(RichText::new("Wait:").strong());
                                ui.label(format!("{} cycles", p.remaining_cycles));
                                if p.remaining_cycles > 0 {
                                    ui.visuals_mut().widgets.active.bg_fill = Color32::DARK_GREEN;
                                }
                                ui.end_row();

                                ui.label(RichText::new("Instruction:").strong());
                                ui.label(if p.current_instruction.is_some() {
                                    &p.current_instruction_name
                                } else {
                                    "—"
                                });
                                ui.end_row();
                            });
                    });
                });
                ui.add_space(4.0);
            }
        });
    }

    fn show_winner_overlay(&self, ctx: &egui::Context) {
        use egui::{Align2, Color32, RichText};

        egui::Area::new(egui::Id::new("winner_overlay"))
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                egui::Frame::none()
                    .fill(Color32::from_black_alpha(200))
                    .show(ui, |ui| {
                        ui.set_min_width(500.0);
                        ui.set_min_height(300.0);
                        ui.vertical_centered(|ui| {
                            ui.add_space(40.0);

                            if !self.winners.is_empty() {
                                // There's a winner!
                                let winner = &self.winners.iter().next().unwrap();
                                let winner_color = player_color(**winner);

                                ui.label(RichText::new("🏆").size(80.0));
                                ui.add_space(20.0);

                                ui.label(
                                    RichText::new("WINNER!")
                                        .size(48.0)
                                        .color(Color32::from_rgb(255, 215, 0))
                                        .strong(),
                                );
                                ui.add_space(20.0);

                                ui.label(
                                    RichText::new(format!("Player {}", self.winners.iter().next().unwrap()))
                                        .size(36.0)
                                        .color(winner_color)
                                        .strong(),
                                );
                                ui.add_space(10.0);

                                ui.label(
                                    RichText::new(format!("Victory at cycle {}", self.cycle))
                                        .size(20.0)
                                        .color(Color32::LIGHT_GRAY),
                                );
                                ui.add_space(10.0);

                                ui.label(
                                    RichText::new(format!("Final Process ID: {}", winner))
                                        .size(16.0)
                                        .color(Color32::LIGHT_GRAY),
                                );
                            } else {
                                // Draw - no winner
                                ui.label(RichText::new("🏳️").size(80.0));
                                ui.add_space(20.0);

                                ui.label(
                                    RichText::new("DRAW")
                                        .size(48.0)
                                        .color(Color32::from_rgb(200, 200, 200))
                                        .strong(),
                                );
                                ui.add_space(20.0);

                                ui.label(
                                    RichText::new("No processes remaining")
                                        .size(20.0)
                                        .color(Color32::LIGHT_GRAY),
                                );
                                ui.add_space(10.0);

                                ui.label(
                                    RichText::new(format!("Game ended at cycle {}", self.cycle))
                                        .size(16.0)
                                        .color(Color32::LIGHT_GRAY),
                                );
                            }

                            ui.add_space(40.0);
                        });
                    });
            });
    }
}

/// Assign distinct, vibrant colors to each player
fn player_color(mut player_id: i32) -> Color32 {
    if player_id<0 {player_id*=-1}
    match player_id {
        1 => Color32::from_rgb(80, 150, 255),  // Vibrant blue
        2 => Color32::from_rgb(255, 80, 80),   // Vibrant red
        3 => Color32::from_rgb(80, 255, 120),  // Vibrant green
        4 => Color32::from_rgb(255, 220, 80),  // Vibrant yellow
        _ => Color32::GRAY,
    }
}

/// Brighten a color by a factor
fn brighten_color(color: Color32, factor: f32) -> Color32 {
    let r = (color.r() as f32 * factor).min(255.0) as u8;
    let g = (color.g() as f32 * factor).min(255.0) as u8;
    let b = (color.b() as f32 * factor).min(255.0) as u8;
    Color32::from_rgb(r, g, b)
}