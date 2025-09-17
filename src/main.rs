mod ui;
mod models;

use std::collections::HashMap;

use eframe::egui::{self};
use egui::*;

use crate::{models::{cost_item::CostItem, income_item::IncomeItem}, ui::windows::MainWindow};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 1000.0])
            .with_title("Budgetting"),
        ..Default::default()
    };

    eframe::run_native(
        "Budgetting",
        options,
        Box::new(|cc| {
            let mut style = (*cc.egui_ctx.style()).clone();
            if let Some(ts) = style.text_styles.get_mut(&TextStyle::Body) {
                ts.size = 16.0; // increase default body text size
            }
            cc.egui_ctx.set_style(style);

            Ok(Box::new(RustedBudgetApp::new()))
        }),
    )
}

#[derive(Debug, Clone)]
pub enum AppEvent {
    // Navigation events
    ChangeView(AppView),
    
    // Cost item events
    AddCostItem(CostItem),
    UpdateCostItem { id: u64, item: CostItem },
    DeleteCostItem(u64),
    
    // Income item events
    AddIncomeItem(IncomeItem),
    UpdateIncomeItem { index: usize, item: IncomeItem },
    DeleteIncomeItem(usize),
    
    // UI events
    ToggleMenu,
    
    // Future events
    SaveData,
    LoadData,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppView {
    Home, 
    CostItems,
    Settings,
}

struct BudgetData {
    cost_items: HashMap<u64, CostItem>,
    income_items: Vec<IncomeItem>,
    next_cost_id: u64,
}

impl Default for BudgetData {
    fn default() -> Self {
        Self { 
            cost_items: HashMap::new(), 
            income_items: Vec::new(),
            next_cost_id: 0, 
        }
    }
}

struct RustedBudgetAppState {
    current_view: AppView,
    budget_data: BudgetData,
}

impl Default for RustedBudgetAppState {
    fn default() -> Self {
        Self { 
            current_view: AppView::Home, 
            budget_data: Default::default(), 
        }
    }
}

impl RustedBudgetAppState {
    fn handle_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::ChangeView(app_view) => {
                self.current_view = app_view;
            },
            AppEvent::AddCostItem(mut cost_item) => {
                cost_item.id = self.budget_data.next_cost_id;
                self.budget_data.cost_items.insert(self.budget_data.next_cost_id, cost_item);
                println!("Added cost item: {:?}", self.budget_data.cost_items.get_key_value(&self.budget_data.next_cost_id));
                self.budget_data.next_cost_id += 1;
            },
            AppEvent::UpdateCostItem { id, item } => {
                if self.budget_data.cost_items.contains_key(&id) {
                    self.budget_data.cost_items.insert(id, item);
                }
            },
            AppEvent::DeleteCostItem(id) => {
                if self.budget_data.cost_items.contains_key(&id) {
                    self.budget_data.cost_items.remove(&id);
                }
            },
            AppEvent::AddIncomeItem(income_item) => {
                self.budget_data.income_items.push(income_item);
            },
            AppEvent::UpdateIncomeItem { index, item } => {
                if index < self.budget_data.income_items.len() {
                    self.budget_data.income_items[index] = item;
                }
            },
            AppEvent::DeleteIncomeItem(index) => {
                if index < self.budget_data.income_items.len() {
                    let removed = self.budget_data.income_items.remove(index);
                    println!("Removed income item: {:?}", removed);
                }
            },
            AppEvent::ToggleMenu => {
                
            },
            AppEvent::SaveData => {
                println!("Saving data...");
            },
            AppEvent::LoadData => {
                println!("Loading data...");
            },
        }
    }
}

struct RustedBudgetApp {
    state: RustedBudgetAppState,
    main_window: MainWindow,
}

impl RustedBudgetApp {
    fn new() -> Self {
        Self { 
            state: RustedBudgetAppState::default(), 
            main_window: MainWindow::new(),
        }
    }
}

impl eframe::App for RustedBudgetApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Collect events from UI
        let events = self.main_window.show(ctx, &self.state);
        
        // Process all events
        for event in events {
            self.state.handle_event(event);
        }
    }
}