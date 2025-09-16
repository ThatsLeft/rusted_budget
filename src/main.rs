use std::vec;

use eframe::egui::{self};
use egui::*;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1640.0, 1100.0])
            .with_title("Budgetting"),
        ..Default::default()
    };

    eframe::run_native(
        "Budgetting",
        options,
        Box::new(|_cc| Ok(Box::new(Budgetting::new()))),
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CostCycle {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpenceCategory {
    Housing,
    Transportation,
    Groceries,
    Healthcare,
    PersonalCare,
    DiningOut,
    Entertainment,
    Shopping,
    Savings,
    DebtPayments,
    Utilities,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IncomeCategory {
    Salary,
    Freelance,
    Investment,
    SideHustle,
    Bonus,
    Gift,
    Other,
}

#[derive(Debug, Clone)]
struct IncomeItem {
    source: String,
    category: IncomeCategory,
    amount: f32,
    income_cycle: CostCycle,
    tags: Option<Vec<String>>,
}

impl Default for IncomeItem {
    fn default() -> Self {
        Self {
            source: String::new(),
            category: IncomeCategory::Salary,
            amount: 0.0,
            income_cycle: CostCycle::Monthly,
            tags: None,
        }
    }
}

#[derive(Debug, Clone)]
struct CostItem {
    what: String,
    estimate: f32,
    actual_cost: f32,
    cost_cycle: CostCycle,
    cost_category: ExpenceCategory,
    tags: Option<Vec<String>>,
    new_tag_input: String,
}

impl Default for CostItem {
    fn default() -> Self {
        Self {
            what: String::new(),
            estimate: 0.0,
            actual_cost: 0.0,
            cost_cycle: CostCycle::Weekly,
            cost_category: ExpenceCategory::Other,
            tags: None,
            new_tag_input: String::new(),
        }
    }
}


struct Budgetting {
    cost_items: Vec<CostItem>,
    show_menu: bool,
    incom_items: Vec<IncomeItem>,
}

impl Budgetting {
    fn new() -> Self {
        Self {
            cost_items: vec![
                CostItem {
                    what: "Coffee".to_string(),
                    estimate: 5.0,
                    actual_cost: 4.50,
                    cost_cycle: CostCycle::Daily,
                    cost_category: ExpenceCategory::Other,
                    tags: Some(vec!["food".to_string()]),
                    new_tag_input: String::new(),
                },
                CostItem {
                    what: "Lunch".to_string(),
                    estimate: 15.0,
                    actual_cost: 18.75,
                    cost_cycle: CostCycle::Daily,
                    cost_category: ExpenceCategory::Other,
                    tags: Some(vec!["food".to_string()]),
                    new_tag_input: String::new(),
                },
                CostItem {
                    what: "Ruter månedskort".to_string(),
                    estimate: 10.0,
                    actual_cost: 12.00,
                    cost_cycle: CostCycle::Monthly,
                    cost_category: ExpenceCategory::Other,
                    tags: Some(vec!["transportation".to_string()]),
                    new_tag_input: String::new(),
                },
                CostItem {
                    what: "Books".to_string(),
                    estimate: 25.0,
                    actual_cost: 22.99,
                    cost_cycle: CostCycle::Daily,
                    cost_category: ExpenceCategory::Other,
                    tags: Some(vec!["hobby".to_string()]),
                    new_tag_input: String::new(),
                },
            ],
            show_menu: true,
            incom_items: Vec::new(),
        }
    }
}

impl eframe::App for Budgetting {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Top panel
        TopBottomPanel::top("top_bar").show(ctx, 
        |ui| {
            ui.horizontal(|ui| {
                if ui.button(if self.show_menu {"<-"} else { "->"}).clicked() {
                    self.show_menu = !self.show_menu;
                }
            });
        });

        // Side menu
        SidePanel::left("left_menu")
            .resizable(true)
            .max_width(105.0)
            .show_animated(ctx, self.show_menu, |ui| {
                ui.heading("Menu");
                ui.separator();
                if ui.button("Dashboard").clicked() {}
                if ui.button("Transactions").clicked() {}
                if ui.button("Settings").clicked() {}
            });

        // Central window
        CentralPanel::default().show(ctx, |ui| {
            ui.set_width(ui.available_width());
            ui.heading("Budget Tracker");
            ui.add_space(10.0);

                // Buttons
            ui.horizontal(|ui| {
                // Buttons
                ui.add_space(20.0);
                if ui.button("Add Daily Cost").clicked() {
                    self.cost_items.push(CostItem {
                        what: "New Item".to_string(),
                        estimate: 0.0,
                        actual_cost: 0.0,
                        cost_cycle: CostCycle::Daily,
                        cost_category: ExpenceCategory::Other,
                        tags: None,
                        new_tag_input: String::new(),
                });
                }

                if ui.button("Add Weekly Cost").clicked() {
                    self.cost_items.push(CostItem {
                        what: "New Item".to_string(),
                        estimate: 0.0,
                        actual_cost: 0.0,
                        cost_cycle: CostCycle::Weekly,
                        cost_category: ExpenceCategory::Other,
                        tags: None,
                        new_tag_input: String::new(),
                    });
                }

                if ui.button("Add Monthly Cost").clicked() {
                    self.cost_items.push(CostItem {
                        what: "New Item".to_string(),
                        estimate: 0.0,
                        actual_cost: 0.0,
                        cost_cycle: CostCycle::Monthly,
                        cost_category: ExpenceCategory::Other,
                        tags: None,
                        new_tag_input: String::new(),
                    });
                }

                if ui.button("Add Yealy Cost").clicked() {
                    self.cost_items.push(CostItem {
                        what: "New Item".to_string(),
                        estimate: 0.0,
                        actual_cost: 0.0,
                        cost_cycle: CostCycle::Yearly,
                        cost_category: ExpenceCategory::Other,
                        tags: None,
                        new_tag_input: String::new(),
                    });
                }
            });

            ui.add_space(20.0);
            // Top section: table and cost items
            {
                // Create the table
                use egui_extras::{Column, TableBuilder};
                ui.add_space(20.0);
            
                // Constrain the entire table area to prevent overflow
                let available_width = ui.available_width();
                ui.allocate_ui_with_layout(
                    vec2(available_width, 320.0), // Fixed height container
                    Layout::top_down(Align::Min),
                    |ui| {
                        ScrollArea::vertical()
                            .max_height(300.0)
                            .auto_shrink([false, true])
                            .show(ui, |ui| {
                                TableBuilder::new(ui)
                                    .striped(true)
                                    .resizable(true)
                                    .cell_layout(Layout::left_to_right(Align::Center))
                                    .column(Column::auto().at_least(100.0).at_most(150.0)) // What
                                    .column(Column::auto().at_least(80.0).at_most(100.0))  // Estimate
                                    .column(Column::auto().at_least(80.0).at_most(100.0))  // Actual Cost
                                    .column(Column::auto().at_least(100.0).at_most(120.0)) // Expence Category
                                    .column(Column::auto().at_least(100.0).at_most(120.0)) // Cost Cycle
                                    .column(Column::remainder().at_least(200.0)) // Tags - uses remaining space
                                    .header(25.0, |mut header| {
                                        header.col(|ui_left| {
                                            ui_left.strong("What");
                                        });
                                        header.col(|ui_left| {
                                            ui_left.strong("Estimate");
                                        });
                                        header.col(|ui_left| {
                                            ui_left.strong("Actual Cost");
                                        });
                                        header.col(|ui_left| {
                                            ui_left.strong("Expence Category");
                                        });
                                        header.col(|ui_left| {
                                            ui_left.strong("Cost Cycle");
                                        });
                                        header.col(|ui_left| {
                                            ui_left.strong("Tags");
                                        });
                                    })
                                    .body(|mut body| {
                                        for row in &mut self.cost_items {
                                            body.row(50.0, |mut row_ui| { // Increased row height for tags
                                                row_ui.col(|ui_left| {
                                                    ui_left.text_edit_singleline(&mut row.what);
                                                });
                                                row_ui.col(|ui_left| {
                                                    ui_left.add(DragValue::new(&mut row.estimate).prefix("$").speed(0.1));
                                                });
                                                row_ui.col(|ui_left| {
                                                    ui_left.add(DragValue::new(&mut row.actual_cost).prefix("$").speed(0.1));
                                                });
                                                row_ui.col(|ui_left| {
                                                    let id = ui_left.id().with(row.what.clone());
                                                    ComboBox::from_id_salt(id)
                                                        .selected_text(match row.cost_category {
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
                                                            ExpenceCategory::Other => "Other",
                                                        })
                                                        .show_ui(ui_left, |ui_left| {
                                                            ui_left.selectable_value(&mut row.cost_category, ExpenceCategory::Housing, "Housing");
                                                            ui_left.selectable_value(&mut row.cost_category, ExpenceCategory::Transportation, "Transportation");
                                                            ui_left.selectable_value(&mut row.cost_category, ExpenceCategory::Groceries, "Groceries");
                                                            ui_left.selectable_value(&mut row.cost_category, ExpenceCategory::Healthcare, "Healthcare");
                                                            ui_left.selectable_value(&mut row.cost_category, ExpenceCategory::PersonalCare, "Personal Care");
                                                            ui_left.selectable_value(&mut row.cost_category, ExpenceCategory::DiningOut, "Dining Out");
                                                            ui_left.selectable_value(&mut row.cost_category, ExpenceCategory::Entertainment, "Entertainment");
                                                            ui_left.selectable_value(&mut row.cost_category, ExpenceCategory::Shopping, "Shopping");
                                                            ui_left.selectable_value(&mut row.cost_category, ExpenceCategory::Savings, "Savings");
                                                            ui_left.selectable_value(&mut row.cost_category, ExpenceCategory::DebtPayments, "Debt Payments");
                                                            ui_left.selectable_value(&mut row.cost_category, ExpenceCategory::Utilities, "Utilities");
                                                            ui_left.selectable_value(&mut row.cost_category, ExpenceCategory::Other, "Other");
                                                        });
                                                });
                                                row_ui.col(|ui_left| {
                                                    let id = ui_left.id().with(row.what.clone());
                                                    ComboBox::from_id_salt(id)
                                                        .selected_text(match row.cost_cycle {
                                                            CostCycle::Daily => "Daily",
                                                            CostCycle::Weekly => "Weekly",
                                                            CostCycle::Monthly => "Monthly",
                                                            CostCycle::Yearly => "Yearly",
                                                        })
                                                        .show_ui(ui_left, |ui_left| {
                                                            ui_left.selectable_value(&mut row.cost_cycle, CostCycle::Daily, "Daily");
                                                            ui_left.selectable_value(&mut row.cost_cycle, CostCycle::Weekly, "Weekly");
                                                            ui_left.selectable_value(&mut row.cost_cycle, CostCycle::Monthly, "Monthly");
                                                            ui_left.selectable_value(&mut row.cost_cycle, CostCycle::Yearly, "Yearly");
                                                        });
                                                });
                                                row_ui.col(|ui_left| {
                                                    // Tags column with proper width constraint
                                                    let available_width = ui_left.available_width();
                                                    
                                                    ui_left.vertical(|ui| {
                                                        // Tags display area
                                                        ScrollArea::horizontal()
                                                            .id_salt(ui.id().with(("tags_scroll", &row.what)))
                                                            .max_width(available_width)
                                                            .max_height(20.0)
                                                            .auto_shrink([false, true])
                                                            .show(ui, |ui| {
                                                                ui.horizontal(|ui| {
                                                                    if let Some(tags) = &mut row.tags {
                                                                        let mut remove_idx: Option<usize> = None;
                                                                        for (i, t) in tags.iter().enumerate() {
                                                                            ui.horizontal(|ui| {
                                                                                ui.label(RichText::new(format!("#{}", t)).monospace().size(11.0));
                                                                                if ui.small_button("×").on_hover_text("Remove").clicked() {
                                                                                    remove_idx = Some(i);
                                                                                }
                                                                            });
                                                                        }
                                                                        if let Some(i) = remove_idx {
                                                                            tags.remove(i);
                                                                        }
                                                                    } else {
                                                                        ui.label(RichText::new("(no tags)").italics().size(10.0));
                                                                    }
                                                                });
                                                            });
                                                        
                                                        ui.add_space(4.0);
                                                        
                                                        // Input area
                                                        ui.horizontal(|ui| {
                                                            let mut commit = false;
                                                            
                                                            let button_width = 35.0;
                                                            let input_width = (available_width - button_width - 8.0).max(60.0);
                                                            
                                                            let resp = ui.add_sized(
                                                                [input_width, 16.0],
                                                                TextEdit::singleline(&mut row.new_tag_input)
                                                                    .hint_text("add tag")
                                                                    .font(TextStyle::Small)
                                                            );
                                                            
                                                            if resp.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                                                                commit = true;
                                                            }
                                                            
                                                            if ui.add_sized([button_width, 16.0], Button::new("Add").small()).clicked() {
                                                                commit = true;
                                                            }
                                                            
                                                            if commit && !row.new_tag_input.trim().is_empty() {
                                                                let tag = row.new_tag_input.trim().replace(char::is_whitespace, "_").to_lowercase();
                                                                if tag.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                                                                    let tags = row.tags.get_or_insert_with(Vec::new);
                                                                    if !tags.iter().any(|t| t == &tag) {
                                                                        tags.push(tag);
                                                                    }
                                                                    row.new_tag_input.clear();
                                                                }
                                                            }
                                                        });
                                                    });
                                                });
                                            });
                                        }
                                        
                                        // Sum row
                                        body.row(30.0, |mut row_ui| {
                                            row_ui.col(|ui| {
                                                ui.strong("Total:");
                                            });
                                            row_ui.col(|ui| {
                                                let total_estimate: f32 = self.cost_items.iter().map(|item| item.estimate).sum();
                                                ui.strong(format!("${:.2}", total_estimate));
                                            });
                                            row_ui.col(|ui| {
                                                let total_actual: f32 = self.cost_items.iter().map(|item| item.actual_cost).sum();
                                                ui.strong(format!("${:.2}", total_actual));
                                            });
                                            row_ui.col(|_ui| {}); // Empty cell for cost cycle column
                                            row_ui.col(|_ui| {}); // Empty cell for tags column
                                        });
                                    });
                            });
                    }
                );
            }

            // Bottom section: two columns (left header, right pie)
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);
            
            ui.columns(2, |mut cols| {
                let left = &mut cols[0];
                left.heading("Comming");
                left.add_space(8.0);

                // Per-category totals and simple bar graph
                let categories: [(ExpenceCategory, &str); 12] = [
                    (ExpenceCategory::Housing, "Housing"),
                    (ExpenceCategory::Transportation, "Transportation"),
                    (ExpenceCategory::Groceries, "Groceries"),
                    (ExpenceCategory::Healthcare, "Healthcare"),
                    (ExpenceCategory::PersonalCare, "Personal Care"),
                    (ExpenceCategory::DiningOut, "Dining Out"),
                    (ExpenceCategory::Entertainment, "Entertainment"),
                    (ExpenceCategory::Shopping, "Shopping"),
                    (ExpenceCategory::Savings, "Savings"),
                    (ExpenceCategory::DebtPayments, "Debt Payments"),
                    (ExpenceCategory::Utilities, "Utilities"),
                    (ExpenceCategory::Other, "Other"),
                ];

                let mut totals: Vec<f32> = vec![0.0; categories.len()];
                for item in &self.cost_items {
                    let idx = match item.cost_category {
                        ExpenceCategory::Housing => 0,
                        ExpenceCategory::Transportation => 1,
                        ExpenceCategory::Groceries => 2,
                        ExpenceCategory::Healthcare => 3,
                        ExpenceCategory::PersonalCare => 4,
                        ExpenceCategory::DiningOut => 5,
                        ExpenceCategory::Entertainment => 6,
                        ExpenceCategory::Shopping => 7,
                        ExpenceCategory::Savings => 8,
                        ExpenceCategory::DebtPayments => 9,
                        ExpenceCategory::Utilities => 10,
                        ExpenceCategory::Other => 11,
                    };
                    totals[idx] += item.actual_cost;
                }

                let max_total = totals
                    .iter()
                    .copied()
                    .fold(0.0_f32, f32::max)
                    .max(1.0);

                for (i, (_cat, label)) in categories.iter().enumerate() {
                    let total = totals[i];

                    left.horizontal(|ui| {
                        ui.label(RichText::new(*label).strong());
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(format!("${:.2}", total));
                        });
                    });

                    let available_width = left.available_width();
                    let bar_height = 14.0;
                    let (rect, _resp) = left.allocate_exact_size(vec2(available_width, bar_height), Sense::hover());
                    let painter = left.painter();

                    // Background bar
                    painter.rect_filled(rect, 3.0, Color32::from_gray(230));

                    // Filled portion
                    let frac = (total / max_total).clamp(0.0, 1.0);
                    let filled_width = rect.width() * frac;
                    let filled_rect = Rect {
                        min: rect.min,
                        max: pos2(rect.min.x + filled_width, rect.max.y),
                    };
                    painter.rect_filled(filled_rect, 3.0, Color32::from_rgb(100, 170, 255));

                    left.add_space(8.0);
                }

                /// pi chart
                let right = &mut cols[1];
                right.label("Actual incom used");

                // fraction used
                let total_actual: f32 = self.cost_items.iter().map(|i| i.actual_cost).sum();
                let total_incom: f32 = self.incom_items.iter().map(|i|i.amount).sum();
                let income = total_incom.max(0.0);
                let frac = if income > 0.0 { total_actual / income } else { 0.0 };
                let frac = frac.clamp(0.0, 1.0);

                // draw pie
                let desired = vec2(right.available_width().min(220.0), 220.0);
                let (rect, _resp) = right.allocate_exact_size(desired, Sense::hover());
                let painter = right.painter();
                let center = rect.center();
                let radius = rect.width().min(rect.height()) * 0.45;

                // background circle
                painter.circle_filled(center, radius, Color32::LIGHT_GRAY);

                // sector
                let steps = 96;
                let start = -std::f32::consts::FRAC_PI_2;
                let end = start + frac * (2.0 * std::f32::consts::PI);
                let mut points = Vec::with_capacity(steps + 2);
                points.push(center);
                for i in 0..=steps {
                    let t = i as f32 / steps as f32;
                    let a = start + t * (end - start);
                    points.push(center + vec2(a.cos() * radius, a.sin() * radius));
                }
                painter.add(Shape::convex_polygon(points, Color32::from_rgb(100, 170, 255), Stroke::NONE));

                // label
                let percent = (frac * 100.0).round() as i32;
                painter.text(
                    center,
                    Align2::CENTER_CENTER,
                    format!("{percent}%"),
                    TextStyle::Heading.resolve(right.style()),
                    Color32::BLACK,
                );
            });
        });
    }
}