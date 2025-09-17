use eframe::egui::{self, *};

use crate::{ui::{components::quick_add_expense::QuickAddExpense, windows::{cost_item_window::CostItemWindow, home_window::HomeWindow}}, AppEvent, AppView, RustedBudgetAppState};

pub struct MainWindow {
    show_menu: bool,
    home_window: HomeWindow,
    cost_item_window: CostItemWindow,
    quick_add: QuickAddExpense,
    // Future: settings_window: SettingsWindow,
}

impl MainWindow {
    pub fn new() -> Self {
        Self {
            show_menu: true,
            home_window: HomeWindow::new(),
            cost_item_window: CostItemWindow::new(),
            quick_add: QuickAddExpense::new(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, app_state: &RustedBudgetAppState) -> Vec<AppEvent> {
        let mut events = Vec::new();

        // Top panel with hamburger menu
        TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.add_space(2.0);
            ui.horizontal(|ui| {
                if ui.button(if self.show_menu { "☰" } else { "☰" }).clicked() {
                    self.show_menu = !self.show_menu;
                    events.push(AppEvent::ToggleMenu);
                }
            });
            ui.add_space(2.0);
        });

        // Side menu
        SidePanel::left("left_menu")
            .resizable(true)
            .min_width(120.0)
            .max_width(200.0)
            .show_animated(ctx, self.show_menu, |ui| {
                ui.vertical(|ui| {
                    ui.heading("Budget Tracker");
                    ui.separator();
                    
                    ui.add_space(10.0);
                    
                    if ui.selectable_label(
                        app_state.current_view == AppView::Home, 
                        "🏠 Dashboard"
                    ).clicked() {
                        events.push(AppEvent::ChangeView(AppView::Home));
                    }
                    
                    if ui.selectable_label(
                        app_state.current_view == AppView::CostItems, 
                        "📃 CostItem"
                    ).clicked() {
                        events.push(AppEvent::ChangeView(AppView::CostItems));
                    }
                    
                    ui.add_space(ui.available_height() - 60.0);
                    
                    ui.separator();
                    if ui.selectable_label(
                        app_state.current_view == AppView::Settings, 
                        "⚙ Settings"
                    ).clicked() {
                        events.push(AppEvent::ChangeView(AppView::Settings));
                    }
                });
            }
        );

        // Content area with header
        CentralPanel::default().show(ctx, |ui| {
            // Content header
            ui.horizontal(|ui| {
                // Title on the left
                let title = match app_state.current_view {
                    AppView::Home => "🏠 Dashboard",
                    AppView::CostItems => "CostItems",
                    AppView::Settings => "Settings",
                };            
                ui.heading(title);
                
                // Push button to the right
                // ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                //     match app_state.current_view {
                //         AppView::Home => {
                //             if ui.button("+ Add Item").clicked() {
                //                 // Handle add item
                //             }
                //         },
                //         AppView::CostItems => {
                //             // Quick Add Component
                //             let mut quick_add_events = self.quick_add.show(ui);
                //             events.append(&mut quick_add_events);
                //         },
                //         AppView::Settings => {
                //             if ui.button("Save").clicked() {
                //                 events.push(AppEvent::SaveData);
                //             }
                //         },
                //     }
                // });
            });
            
            ui.separator();
            ui.add_space(10.0);

            // Content area - delegate to appropriate view
            match app_state.current_view {
                AppView::Home => {
                    let mut home_events = self.home_window.show(ui, &app_state.budget_data);
                    events.append(&mut home_events);
                },
                AppView::CostItems => {
                    let mut cost_item_events = self.cost_item_window.show(ui, &app_state.budget_data);
                    events.append(&mut cost_item_events);
                },
                AppView::Settings => {
                    // Future: self.settings_window.show(ui, &mut app_state);
                    ui.centered_and_justified(|ui| {
                        ui.label("Settings view coming soon...");
                    });
                },
            }
        });

        events
    }
}