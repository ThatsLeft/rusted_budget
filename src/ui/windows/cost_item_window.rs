use std::collections::HashMap;

use eframe::egui::*;

use crate::{models::cost_item::{CostCycle, CostItem, ExpenceCategory}, ui::components::{cost_item_table::CostItemTable, quick_add_expense::{self, QuickAddExpense}}, AppEvent};
use crate::BudgetData;

#[derive(Clone, Copy, PartialEq)]
pub enum SortColumn {
    What,
    Cost,
    Category,
    Cycle,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

pub struct CostItemWindow {
    quick_add: QuickAddExpense,
    yearly_table: CostItemTable,
    monthly_table: CostItemTable,
    tag_inputs: HashMap<usize, String>,
    sort_column: Option<SortColumn>,
    sort_order: SortOrder,
}

impl CostItemWindow {
    pub fn new() -> Self {
        Self {
            quick_add: QuickAddExpense::new(),
            yearly_table: CostItemTable::new(),
            monthly_table: CostItemTable::new(),
            tag_inputs: HashMap::new(),
            sort_column: None,
            sort_order: SortOrder::Ascending,
        }
    }

    pub fn show(&mut self, ui: &mut Ui, budget_data: &BudgetData) -> Vec<AppEvent> {
        let mut events = Vec::new();
    
        ui.vertical(|ui|{
            ui.add_space(10.0);
    
            // Buttons
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                
                if ui.button("Add Daily Cost").clicked() {
                    events.push(AppEvent::AddCostItem(
                        CostItem {
                            id: 0,
                            what: "New Item".to_string(),
                            cost: 0.0,
                            cost_cycle: CostCycle::Daily,
                            cost_category: ExpenceCategory::Other,
                            tags: None,
                        })
                    );
                }
    
                if ui.button("Add Weekly Cost").clicked() {
                    events.push(AppEvent::AddCostItem(
                        CostItem {
                            id: 0,
                            what: "New Item".to_string(),
                            cost: 0.0,
                            cost_cycle: CostCycle::Weekly,
                            cost_category: ExpenceCategory::Other,
                            tags: None,
                        })
                    );
                }
    
                if ui.button("Add Monthly Cost").clicked() {
                    events.push(AppEvent::AddCostItem(
                        CostItem {
                            id: 0,
                            what: "New Item".to_string(),
                            cost: 0.0,
                            cost_cycle: CostCycle::Monthly,
                            cost_category: ExpenceCategory::Other,
                            tags: None,
                        })
                    );
                }
    
                if ui.button("Add Yearly Cost").clicked() {
                    events.push(AppEvent::AddCostItem(
                        CostItem {
                            id: 0,
                            what: "New Item".to_string(),
                            cost: 0.0,
                            cost_cycle: CostCycle::Yearly,
                            cost_category: ExpenceCategory::Other,
                            tags: None,
                        })
                    );
                }
            });
    
            ui.add_space(15.0);
    
            // Quick Add Component
            ui.group(|ui| {
                ui.label(RichText::new("Quick Add").strong());
                ui.separator();
                
                let mut quick_add_events = self.quick_add.show(ui);
                events.append(&mut quick_add_events);
            });
    
            ui.add_space(10.0);
    
            {
                // Inline editable table
                use egui_extras::{Column, TableBuilder};
                ui.add_space(20.0);
            
                let available_width = ui.available_width();
                ui.allocate_ui_with_layout(
                    vec2(available_width, 800.0), // Increased height for inline editing
                    Layout::top_down(Align::Min),
                    |ui| {
                        ScrollArea::vertical()
                            .max_height(350.0)
                            .auto_shrink([false, true])
                            .show(ui, |ui| {
                                TableBuilder::new(ui)
                                    .striped(true)
                                    .resizable(true)
                                    .cell_layout(Layout::left_to_right(Align::Center))
                                    .column(Column::auto().at_least(40.0).at_most(50.0))   // Delete
                                    .column(Column::auto().at_least(120.0).at_most(180.0)) // What
                                    .column(Column::auto().at_least(80.0).at_most(100.0))  // Cost
                                    .column(Column::auto().at_least(120.0).at_most(150.0)) // Category
                                    .column(Column::auto().at_least(80.0).at_most(100.0))  // Cycle
                                    .column(Column::auto().at_least(160.0).at_most(260.0)) // Tags
                                    .header(25.0, |mut header| {
                                        header.col(|ui| {
                                            ui.strong("Del");
                                        });
                                        header.col(|ui| {
                                            let mut text = "What".to_string();
                                            if matches!(self.sort_column, Some(SortColumn::What)) {
                                                text.push_str(match self.sort_order {
                                                    SortOrder::Ascending => " (asc)",
                                                    SortOrder::Descending => " (desc)",
                                                });
                                            }
                                            if ui.button(text).clicked() {
                                                if matches!(self.sort_column, Some(SortColumn::What)) {
                                                    self.sort_order = match self.sort_order {
                                                        SortOrder::Ascending => SortOrder::Descending,
                                                        SortOrder::Descending => SortOrder::Ascending,
                                                    };
                                                } else {
                                                    self.sort_column = Some(SortColumn::What);
                                                    self.sort_order = SortOrder::Ascending;
                                                }
                                            }
                                        });
                                        header.col(|ui| {
                                            let mut text = "Cost".to_string();
                                            if matches!(self.sort_column, Some(SortColumn::Cost)) {
                                                text.push_str(match self.sort_order {
                                                    SortOrder::Ascending => " (asc)",
                                                    SortOrder::Descending => " (desc)",
                                                });
                                            }
                                            if ui.button(text).clicked() {
                                                if matches!(self.sort_column, Some(SortColumn::Cost)) {
                                                    self.sort_order = match self.sort_order {
                                                        SortOrder::Ascending => SortOrder::Descending,
                                                        SortOrder::Descending => SortOrder::Ascending,
                                                    };
                                                } else {
                                                    self.sort_column = Some(SortColumn::Cost);
                                                    self.sort_order = SortOrder::Ascending;
                                                }
                                            }
                                        });
                                        header.col(|ui| {
                                            let mut text = "Category".to_string();
                                            if matches!(self.sort_column, Some(SortColumn::Category)) {
                                                text.push_str(match self.sort_order {
                                                    SortOrder::Ascending => " (asc)",
                                                    SortOrder::Descending => " (desc)",
                                                });
                                            }
                                            if ui.button(text).clicked() {
                                                if matches!(self.sort_column, Some(SortColumn::Category)) {
                                                    self.sort_order = match self.sort_order {
                                                        SortOrder::Ascending => SortOrder::Descending,
                                                        SortOrder::Descending => SortOrder::Ascending,
                                                    };
                                                } else {
                                                    self.sort_column = Some(SortColumn::Category);
                                                    self.sort_order = SortOrder::Ascending;
                                                }
                                            }
                                        });
                                        header.col(|ui| {
                                            let mut text = "Cycle".to_string();
                                            if matches!(self.sort_column, Some(SortColumn::Cycle)) {
                                                text.push_str(match self.sort_order {
                                                    SortOrder::Ascending => " (asc)",
                                                    SortOrder::Descending => " (desc)",
                                                });
                                            }
                                            if ui.button(text).clicked() {
                                                if matches!(self.sort_column, Some(SortColumn::Cycle)) {
                                                    self.sort_order = match self.sort_order {
                                                        SortOrder::Ascending => SortOrder::Descending,
                                                        SortOrder::Descending => SortOrder::Ascending,
                                                    };
                                                } else {
                                                    self.sort_column = Some(SortColumn::Cycle);
                                                    self.sort_order = SortOrder::Ascending;
                                                }
                                            }
                                        });
                                        header.col(|ui| {
                                            ui.strong("Tags");
                                        });
                                    })
                                    .body(|mut body| {
                                        let sorted_items = self.get_sorted_items(&budget_data.cost_items);

                                        for (item_id, item) in sorted_items {
                                            body.row(55.0, |mut row| {
                                                // Delete button
                                                row.col(|ui| {
                                                    if ui.small_button("🗑").on_hover_text("Delete item").clicked() {
                                                        events.push(AppEvent::DeleteCostItem(item_id));
                                                    }
                                                });
                                                
                                                // Editable What field
                                                row.col(|ui| {
                                                    let mut temp_what = item.what.clone();
                                                    let response = ui.text_edit_singleline(&mut temp_what);
                                                    if response.changed() {
                                                        let mut updated_item = item.clone();
                                                        updated_item.what = temp_what;
                                                        events.push(AppEvent::UpdateCostItem { 
                                                            id: item_id, 
                                                            item: updated_item 
                                                        });
                                                    }
                                                });
                                                
                                                // Editable Cost field
                                                row.col(|ui| {
                                                    let mut temp_cost = item.cost;
                                                    let response = ui.add(DragValue::new(&mut temp_cost).prefix("$").speed(0.1));
                                                    if response.changed() {
                                                        let mut updated_item = item.clone();
                                                        updated_item.cost = temp_cost;
                                                        events.push(AppEvent::UpdateCostItem { 
                                                            id: item_id, 
                                                            item: updated_item 
                                                        });
                                                    }
                                                });
                                                
                                                // Editable Category ComboBox
                                                row.col(|ui| {
                                                    let id = ui.id().with(format!("category_{}", item_id));
                                                    let mut temp_category = item.cost_category.clone();
                                                    
                                                    ComboBox::from_id_salt(id)
                                                        .selected_text(match temp_category {
                                                            ExpenceCategory::Housing => "Housing",
                                                            ExpenceCategory::Transportation => "Transportation",
                                                            ExpenceCategory::Groceries => "Groceries",
                                                            ExpenceCategory::Healthcare => "Healthcare",
                                                            ExpenceCategory::PersonalCare => "Personal Care",
                                                            ExpenceCategory::DiningOut => "Dining Out",
                                                            ExpenceCategory::Entertainment => "Entertainment",
                                                            ExpenceCategory::Shopping => "Shopping",
                                                            ExpenceCategory::Savings => "Savings",
                                                            ExpenceCategory::DebtPayments => "Debt Payments",
                                                            ExpenceCategory::Utilities => "Utilities",
                                                            ExpenceCategory::Insurance => "Insurance",
                                                            ExpenceCategory::Other => "Other",
                                                        })
                                                        .show_ui(ui, |ui| {
                                                            let mut changed = false;
                                                            changed |= ui.selectable_value(&mut temp_category, ExpenceCategory::Housing, "Housing").changed();
                                                            changed |= ui.selectable_value(&mut temp_category, ExpenceCategory::Transportation, "Transportation").changed();
                                                            changed |= ui.selectable_value(&mut temp_category, ExpenceCategory::Groceries, "Groceries").changed();
                                                            changed |= ui.selectable_value(&mut temp_category, ExpenceCategory::Healthcare, "Healthcare").changed();
                                                            changed |= ui.selectable_value(&mut temp_category, ExpenceCategory::PersonalCare, "Personal Care").changed();
                                                            changed |= ui.selectable_value(&mut temp_category, ExpenceCategory::DiningOut, "Dining Out").changed();
                                                            changed |= ui.selectable_value(&mut temp_category, ExpenceCategory::Entertainment, "Entertainment").changed();
                                                            changed |= ui.selectable_value(&mut temp_category, ExpenceCategory::Shopping, "Shopping").changed();
                                                            changed |= ui.selectable_value(&mut temp_category, ExpenceCategory::Savings, "Savings").changed();
                                                            changed |= ui.selectable_value(&mut temp_category, ExpenceCategory::DebtPayments, "Debt Payments").changed();
                                                            changed |= ui.selectable_value(&mut temp_category, ExpenceCategory::Utilities, "Utilities").changed();
                                                            changed |= ui.selectable_value(&mut temp_category, ExpenceCategory::Insurance, "Insurance").changed();
                                                            changed |= ui.selectable_value(&mut temp_category, ExpenceCategory::Other, "Other").changed();
                                                            
                                                            if changed {
                                                                let mut updated_item = item.clone();
                                                                updated_item.cost_category = temp_category;
                                                                events.push(AppEvent::UpdateCostItem { 
                                                                    id: item_id, 
                                                                    item: updated_item 
                                                                });
                                                            }
                                                        });
                                                });
                                                
                                                // Editable Cycle ComboBox
                                                row.col(|ui| {
                                                    let id = ui.id().with(format!("cycle_{}", item_id));
                                                    let mut temp_cycle = item.cost_cycle.clone();
                                                    
                                                    ComboBox::from_id_salt(id)
                                                        .selected_text(match temp_cycle {
                                                            CostCycle::Daily => "Daily",
                                                            CostCycle::Weekly => "Weekly",
                                                            CostCycle::Monthly => "Monthly",
                                                            CostCycle::Yearly => "Yearly",
                                                        })
                                                        .show_ui(ui, |ui| {
                                                            let mut changed = false;
                                                            changed |= ui.selectable_value(&mut temp_cycle, CostCycle::Daily, "Daily").changed();
                                                            changed |= ui.selectable_value(&mut temp_cycle, CostCycle::Weekly, "Weekly").changed();
                                                            changed |= ui.selectable_value(&mut temp_cycle, CostCycle::Monthly, "Monthly").changed();
                                                            changed |= ui.selectable_value(&mut temp_cycle, CostCycle::Yearly, "Yearly").changed();
                                                            
                                                            if changed {
                                                                let mut updated_item = item.clone();
                                                                updated_item.cost_cycle = temp_cycle;
                                                                events.push(AppEvent::UpdateCostItem { 
                                                                    id: item_id, 
                                                                    item: updated_item 
                                                                });
                                                            }
                                                        });
                                                });
                                                
                                                // Tags column with inline editing
                                                row.col(|ui| {
                                                    let available_width = ui.available_width();
                                                    
                                                    ui.vertical(|ui| {
                                                        // Tags display area
                                                        ScrollArea::horizontal()
                                                            .id_salt(ui.id().with(("tags_scroll", item_id)))
                                                            .max_width(available_width)
                                                            .max_height(20.0)
                                                            .auto_shrink([false, true])
                                                            .show(ui, |ui| {
                                                                ui.horizontal(|ui| {
                                                                    if let Some(tags) = &item.tags {
                                                                        for (tag_idx, tag) in tags.iter().enumerate() {
                                                                            ui.horizontal(|ui| {
                                                                                ui.label(RichText::new(format!("#{}", tag)).monospace().size(11.0));
                                                                                if ui.small_button("×").on_hover_text("Remove tag").clicked() {
                                                                                    let mut updated_item = item.clone();
                                                                                    if let Some(ref mut tags) = updated_item.tags {
                                                                                        tags.remove(tag_idx);
                                                                                        if tags.is_empty() {
                                                                                            updated_item.tags = None;
                                                                                        }
                                                                                    }
                                                                                    events.push(AppEvent::UpdateCostItem { 
                                                                                        id: item_id, 
                                                                                        item: updated_item 
                                                                                    });
                                                                                }
                                                                            });
                                                                        }
                                                                    } else {
                                                                        ui.label(RichText::new("(no tags)").italics().size(10.0));
                                                                    }
                                                                });
                                                            });
                                                        
                                                        ui.add_space(4.0);
                                                        
                                                        // Tag input area - use item_id for HashMap key
                                                        ui.horizontal(|ui| {
                                                            let button_width = 35.0;
                                                            let input_width = (available_width - button_width - 8.0).max(60.0);
                                                            
                                                            // Get the current tag input value (clone it to avoid borrowing issues)
                                                            let current_input = self.tag_inputs.get(&(item_id as usize)).cloned().unwrap_or_default();
                                                            let mut tag_input = current_input;
                                                            
                                                            let resp = ui.add_sized(
                                                                [input_width, 16.0],
                                                                TextEdit::singleline(&mut tag_input)
                                                                    .hint_text("add tag")
                                                                    .font(TextStyle::Small)
                                                            );
                                                            
                                                            // Update the HashMap with the current value
                                                            if resp.changed() {
                                                                self.tag_inputs.insert(item_id as usize, tag_input.clone());
                                                            }
                                                            
                                                            let mut commit = false;
                                                            
                                                            if resp.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                                                                commit = true;
                                                            }
                                                            
                                                            if ui.add_sized([button_width, 16.0], Button::new("Add").small()).clicked() {
                                                                commit = true;
                                                            }
                                                            
                                                            if commit && !tag_input.trim().is_empty() {
                                                                let tag = tag_input.trim().replace(char::is_whitespace, "_").to_lowercase();
                                                                if tag.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                                                                    let mut updated_item = item.clone();
                                                                    let tags = updated_item.tags.get_or_insert_with(Vec::new);
                                                                    if !tags.iter().any(|t| t == &tag) {
                                                                        tags.push(tag);
                                                                        events.push(AppEvent::UpdateCostItem { 
                                                                            id: item_id, 
                                                                            item: updated_item 
                                                                        });
                                                                        // Clear the input in the HashMap after successful add
                                                                        self.tag_inputs.insert(item_id as usize, String::new());
                                                                    }
                                                                }
                                                            }
                                                        });
                                                    });
                                                });
                                            });
                                        }
                                    });
                            });
                    }
                );
    
                // Summary row
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.add_space(40.0);
                    ui.strong("Monthly Total:");
                    ui.add_space(20.0);
                    
                    let mut total_monthly: f32 = 0.0;
                    for item in budget_data.cost_items.values() {
                        match item.cost_cycle {
                            CostCycle::Daily => total_monthly += item.cost * 30.44,
                            CostCycle::Weekly => total_monthly += item.cost * 4.348,
                            CostCycle::Monthly => total_monthly += item.cost,
                            CostCycle::Yearly => total_monthly += item.cost / 12.0,
                        }
                    }
    
                    ui.strong(format!("${:.2}", total_monthly));
                    
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(format!("({} items)", budget_data.cost_items.len()));
                    });
                });
            }

            ui.separator();
            ui.add_space(10.0);

            let yearly_items: Vec<CostItem> = budget_data.cost_items.values()
                .filter(|item| matches!(item.cost_cycle, CostCycle::Yearly))
                .cloned()
                .collect();

            let monthly_items: Vec<CostItem> = budget_data.cost_items.values()
                .filter(|item| matches!(item.cost_cycle, CostCycle::Monthly))
                .cloned()
                .collect();

            ui.vertical(|ui| {
                ui.heading("Yearly Items");
                let mut yearly_events = self.yearly_table.show(ui, &yearly_items, true, "yearly_table".to_string());
                events.append(&mut yearly_events);
                
                ui.add_space(20.0);
                
                ui.heading("Monthly Items");
                let mut monthly_events = self.monthly_table.show(ui, &monthly_items, true, "monthly_table".to_string());
                events.append(&mut monthly_events);
            });
        });
    
        events
    }

    // Updated to work with HashMap instead of Vec
    fn get_sorted_items<'a>(&self, items: &'a HashMap<u64, CostItem>) -> Vec<(u64, &'a CostItem)> {
        let mut indexed_items: Vec<(u64, &CostItem)> = items.iter().map(|(id, item)| (*id, item)).collect();
        
        if let Some(column) = self.sort_column {
            indexed_items.sort_by(|(_, a), (_, b)| {
                let comparison = match column {
                    SortColumn::What => a.what.cmp(&b.what),
                    SortColumn::Cost => {
                        // Convert to monthly cost for fair comparison
                        let a_monthly = match a.cost_cycle {
                            CostCycle::Daily => a.cost * 30.44,
                            CostCycle::Weekly => a.cost * 4.348,
                            CostCycle::Monthly => a.cost,
                            CostCycle::Yearly => a.cost / 12.0,
                        };
                        let b_monthly = match b.cost_cycle {
                            CostCycle::Daily => b.cost * 30.44,
                            CostCycle::Weekly => b.cost * 4.348,
                            CostCycle::Monthly => b.cost,
                            CostCycle::Yearly => b.cost / 12.0,
                        };
                        a_monthly.partial_cmp(&b_monthly).unwrap_or(std::cmp::Ordering::Equal)
                    },
                    SortColumn::Category => {
                        format!("{:?}", a.cost_category).cmp(&format!("{:?}", b.cost_category))
                    },
                    SortColumn::Cycle => {
                        format!("{:?}", a.cost_cycle).cmp(&format!("{:?}", b.cost_cycle))
                    },
                };
                
                match self.sort_order {
                    SortOrder::Ascending => comparison,
                    SortOrder::Descending => comparison.reverse(),
                }
            });
        }
        
        indexed_items
    }
}