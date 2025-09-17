use std::collections::HashMap;
use eframe::egui::*;
use egui_extras::{Column, TableBuilder};

use crate::{models::cost_item::{CostCycle, CostItem}, AppEvent};

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

pub struct CostItemTable {
    tag_inputs: HashMap<usize, String>,
    sort_column: Option<SortColumn>,
    sort_order: SortOrder,
}

impl CostItemTable {
    pub fn new() -> Self {
        Self {
            tag_inputs: HashMap::new(),
            sort_column: None,
            sort_order: SortOrder::Ascending,
        }
    }

    pub fn show(&mut self, ui: &mut Ui, items: &[CostItem], show_summary: bool, table_id: String) -> Vec<AppEvent> {
        let mut events = Vec::new();

        ScrollArea::vertical()
            .id_salt(table_id.clone() + "_scroll")
            .auto_shrink([false, true])
            .show(ui, |ui| {
                ui.push_id(&table_id, |ui| {
                    TableBuilder::new(ui)
                        .striped(true)
                        .resizable(true) // You can add this back now
                        .cell_layout(Layout::left_to_right(Align::Center))
                        .column(Column::auto().at_least(40.0).at_most(50.0))   // Delete
                        .column(Column::auto().at_least(120.0).at_most(180.0)) // What
                        .column(Column::auto().at_least(80.0).at_most(100.0))  // Cost
                        .column(Column::auto().at_least(120.0).at_most(150.0)) // Category
                        .column(Column::auto().at_least(80.0).at_most(100.0))  // Cycle
                        .column(Column::auto().at_least(160.0).at_most(260.0)) // Tags
                        .header(25.0, |mut header| {
                            self.render_headers(&mut header);
                        })
                        .body(|mut body| {
                            let sorted_items = self.get_sorted_items(items);
                            for (original_index, item) in sorted_items {
                                self.render_row(&mut body, item, &mut events);
                            }
                        });
                });
            });

        events
    }

    fn render_headers(&mut self, header: &mut egui_extras::TableRow) {
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
                self.handle_sort_click(SortColumn::What);
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
                self.handle_sort_click(SortColumn::Cost);
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
                self.handle_sort_click(SortColumn::Category);
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
                self.handle_sort_click(SortColumn::Cycle);
            }
        });

        header.col(|ui| {
            ui.strong("Tags");
        });
    }

    fn handle_sort_click(&mut self, column: SortColumn) {
        if matches!(self.sort_column, Some(current) if current == column) {
            self.sort_order = match self.sort_order {
                SortOrder::Ascending => SortOrder::Descending,
                SortOrder::Descending => SortOrder::Ascending,
            };
        } else {
            self.sort_column = Some(column);
            self.sort_order = SortOrder::Ascending;
        }
    }

    fn render_row(
        &mut self,
        body: &mut egui_extras::TableBody,
        item: &CostItem,
        events: &mut Vec<AppEvent>,
    ) {
        body.row(55.0, |mut row| {
            // Delete button
            row.col(|ui| {
                if ui.small_button("🗑").on_hover_text("Delete item").clicked() {
                    events.push(AppEvent::DeleteCostItem(item.id));
                }
            });
            
            // What field
            row.col(|ui| {
                let mut temp_what = item.what.clone();
                let response = ui.text_edit_singleline(&mut temp_what);
                if response.changed() {
                    let mut updated_item = item.clone();
                    updated_item.what = temp_what;
                    events.push(AppEvent::UpdateCostItem { 
                        id: updated_item.id, 
                        item: updated_item 
                    });
                }
            });
            
            // Cost field
            row.col(|ui| {
                let mut temp_cost = item.cost;
                let response = ui.add(DragValue::new(&mut temp_cost).prefix("$").speed(0.1));
                if response.changed() {
                    let mut updated_item = item.clone();
                    updated_item.cost = temp_cost;
                    events.push(AppEvent::UpdateCostItem { 
                        id: item.id, 
                        item: updated_item 
                    });
                }
            });
            
            row.col(|ui| {
                ui.label(format!("{:?}", item.cost_category));
            });
            
            // Cycle column
            row.col(|ui| {
                ui.label(format!("{:?}", item.cost_cycle));
            });
            
            // Tags column
            row.col(|ui| {
                if let Some(tags) = &item.tags {
                    ui.label(tags.join(", "));
                } else {
                    ui.label("(no tags)");
                }
            });
        });
    }

    fn render_summary(&self, ui: &mut Ui, items: &[CostItem]) {
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            ui.add_space(40.0);
            ui.strong("Monthly Total:");
            ui.add_space(20.0);
            
            let mut total_monthly: f32 = 0.0;
            for item in items {
                match item.cost_cycle {
                    CostCycle::Daily => total_monthly += item.cost * 30.44,
                    CostCycle::Weekly => total_monthly += item.cost * 4.348,
                    CostCycle::Monthly => total_monthly += item.cost,
                    CostCycle::Yearly => total_monthly += item.cost / 12.0,
                }
            }

            ui.strong(format!("${:.2}", total_monthly));
            
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(format!("({} items)", items.len()));
            });
        });
    }

    fn get_sorted_items<'a>(&self, items: &'a [CostItem]) -> Vec<(usize, &'a CostItem)> {
        let mut indexed_items: Vec<(usize, &CostItem)> = items.iter().enumerate().collect();
        
        if let Some(column) = self.sort_column {
            indexed_items.sort_by(|(_, a), (_, b)| {
                let comparison = match column {
                    SortColumn::What => a.what.cmp(&b.what),
                    SortColumn::Cost => {
                        let a_monthly = self.to_monthly_cost(a);
                        let b_monthly = self.to_monthly_cost(b);
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

    fn to_monthly_cost(&self, item: &CostItem) -> f32 {
        match item.cost_cycle {
            CostCycle::Daily => item.cost * 30.44,
            CostCycle::Weekly => item.cost * 4.348,
            CostCycle::Monthly => item.cost,
            CostCycle::Yearly => item.cost / 12.0,
        }
    }
}