use crate::{
    AppEvent, BudgetData,
    models::cost_item::{CostCycle, ExpenceCategory},
    ui::components::{quick_add_expense::QuickAddExpense, quick_add_income::QuickAddIncome},
};
use eframe::egui::*;

pub struct HomeWindow {
    quick_add_expense: QuickAddExpense,
    quick_add_income: QuickAddIncome,
}

impl HomeWindow {
    pub fn new() -> Self {
        Self {
            quick_add_expense: QuickAddExpense::new(),
            quick_add_income: QuickAddIncome::new(),
        }
    }

    pub fn show(&mut self, ui: &mut Ui, budget_data: &BudgetData) -> Vec<AppEvent> {
        let mut events = Vec::new();

        let ui_col_height = ui.available_height() * 0.35;

        ui.horizontal(|ui| {
            ui.add_space(20.0);
            ui.columns(2, |cols| {
                let left_income = &mut cols[0];

                left_income.vertical(|left| {
                    left.set_height(ui_col_height);

                    left.heading("Income");
                    // Quick Add Component
                    left.group(|ui| {
                        ui.label(RichText::new("Add Income").strong());
                        ui.separator();

                        let mut quick_add_events = self.quick_add_income.show(ui);
                        events.append(&mut quick_add_events);
                    });

                    left.add_space(10.0);
                    left.label(RichText::new("Income").strong());
                    left.separator();

                    // Income items list - use most of remaining space
                    let scroll_height = left.available_height() - 30.0; // Reserve 30px for total

                    if budget_data.income_items.is_empty() {
                        left.centered_and_justified(|ui| {
                            ui.label(RichText::new("No income sources yet").italics());
                        });
                    } else {
                        ScrollArea::vertical()
                            .id_salt("incom_scroll_view")
                            .max_height(scroll_height)
                            .show(left, |ui| {
                                for (i, income) in budget_data.income_items.iter().enumerate() {
                                    ui.horizontal(|ui| {
                                        ui.strong(&income.source);
                                        ui.label(format!("{:?}", income.category));

                                        if ui.small_button("🗑").on_hover_text("Delete").clicked()
                                        {
                                            events.push(AppEvent::DeleteIncomeItem(i));
                                        }
                                        ui.strong(format!("${:.2}", income.amount));
                                        ui.label(format!("{:?}", income.income_cycle));
                                    });
                                    ui.separator();
                                }
                            });
                    }

                    // Consume remaining space to push total to bottom
                    left.allocate_response(left.available_size(), Sense::hover());

                    // Income total - at bottom
                    left.horizontal(|ui| {
                        ui.label("Monthly Total:");
                        let mut total_monthly_income: f32 = 0.0;
                        for income in &budget_data.income_items {
                            match income.income_cycle {
                                CostCycle::Daily => total_monthly_income += income.amount * 30.44,
                                CostCycle::Weekly => total_monthly_income += income.amount * 4.348,
                                CostCycle::Monthly => total_monthly_income += income.amount,
                                CostCycle::Yearly => total_monthly_income += income.amount / 12.0,
                            }
                        }
                        ui.strong(format!("${:.2}", total_monthly_income));
                    });
                });

                let right_expenses = &mut cols[1];
                right_expenses.vertical(|right| {
                    right.set_height(ui_col_height);

                    right.heading("Expenses");

                    // Quick Add Component
                    right.group(|ui| {
                        ui.label(RichText::new("Quick Add").strong());
                        ui.separator();

                        let mut quick_add_events = self.quick_add_expense.show(ui);
                        events.append(&mut quick_add_events);
                    });

                    right.add_space(10.0);
                    right.label(RichText::new("Expenses").strong());
                    right.separator();

                    // Expenses scroll area - use most of remaining space
                    let scroll_height = right.available_height() - 50.0; // Reserve 50px for totals

                    if budget_data.cost_items.is_empty() {
                        right.centered_and_justified(|ui| {
                            ui.label(RichText::new("No items yet. Use Quick Add above!").italics());
                        });
                    } else {
                        ScrollArea::vertical()
                            .id_salt("expenses_scroll_view")
                            .max_height(scroll_height)
                            .show(right, |ui| {
                                for (_i, item) in budget_data.cost_items.values().enumerate() {
                                    ui.horizontal(|ui| {
                                        ui.label(&item.what);
                                        ui.label(format!("${:.2}", item.cost));
                                        ui.label(format!("{:?}", item.cost_cycle));
                                        ui.label(format!("{:?}", item.cost_category));

                                        if ui
                                            .small_button("🗑")
                                            .on_hover_text("Delete item")
                                            .clicked()
                                        {
                                            events.push(AppEvent::DeleteCostItem(item.id));
                                        }
                                    });
                                    ui.separator();
                                }
                            });
                    }

                    right.allocate_response(right.available_size(), Sense::hover());

                    right.horizontal(|ui| {
                        // Expense totals - at bottom
                        ui.label("Total items:");
                        ui.strong(budget_data.cost_items.len().to_string());
                        ui.add_space(20.0);

                        ui.label("Monthly Total:");
                        let mut total_monthly_expenses: f32 = 0.0;
                        for item in &budget_data.cost_items {
                            match item.1.cost_cycle {
                                CostCycle::Daily => total_monthly_expenses += item.1.cost * 30.44,
                                CostCycle::Weekly => total_monthly_expenses += item.1.cost * 4.348,
                                CostCycle::Monthly => total_monthly_expenses += item.1.cost,
                                CostCycle::Yearly => total_monthly_expenses += item.1.cost / 12.0,
                            }
                        }
                        ui.strong(format!("${:.2}", total_monthly_expenses));
                    });
                });
            });
        });

        ui.separator();
        ui.add_space(20.0);

        // Bottom section - Summary (uses remaining 35% of space)
        ui.heading("Summary by month");
        ui.add_space(10.0);

        ui.columns(2, |cols| {
            let left = &mut cols[0];
            left.heading("Expenses");
            left.add_space(8.0);

            ScrollArea::vertical()
                .id_salt("expenses")
                .max_height(500.0) // Adjust height as needed
                .auto_shrink([false, true])
                .show(left, |ui| {
                    // Per-category totals and simple bar graph
                    let categories: [(ExpenceCategory, &str); 13] = [
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
                        (ExpenceCategory::Insurance, "Insurance"),
                        (ExpenceCategory::Other, "Other"),
                    ];

                    let mut totals: Vec<f32> = vec![0.0; categories.len()];
                    for item in &budget_data.cost_items {
                        let monthly_cost = match item.1.cost_cycle {
                            CostCycle::Daily => item.1.cost * 30.44,
                            CostCycle::Weekly => item.1.cost * 4.348,
                            CostCycle::Monthly => item.1.cost,
                            CostCycle::Yearly => item.1.cost / 12.0,
                        };

                        let idx = match item.1.cost_category {
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
                            ExpenceCategory::Insurance => 11,
                            ExpenceCategory::Other => 12,
                        };
                        totals[idx] += monthly_cost;
                    }

                    let max_total = totals.iter().copied().fold(0.0_f32, f32::max).max(1.0);

                    let palette: [Color32; 13] = [
                        Color32::from_rgb(0xE6, 0x7E, 0x22), // orange
                        Color32::from_rgb(0x1F, 0x77, 0xB4), // blue
                        Color32::from_rgb(0x2C, 0xA0, 0x2C), // green
                        Color32::from_rgb(0xD6, 0x27, 0x28), // red
                        Color32::from_rgb(0x94, 0x67, 0xBD), // purple
                        Color32::from_rgb(0x8C, 0x56, 0x4B), // brown
                        Color32::from_rgb(0xE3, 0x77, 0xC2), // pink
                        Color32::from_rgb(0x7F, 0x7F, 0x7F), // gray
                        Color32::from_rgb(0xBC, 0xBD, 0x22), // olive
                        Color32::from_rgb(0x17, 0xBE, 0xCF), // cyan
                        Color32::from_rgb(0xFF, 0xA5, 0x00), // dark orange
                        Color32::from_rgb(0xFF, 0xA5, 0x0C), // dark orange
                        Color32::from_rgb(0xA0, 0xA0, 0xA0), // light gray
                    ];

                    for (i, (_cat, label)) in categories.iter().enumerate() {
                        let total = totals[i];

                        ui.horizontal(|ui| {
                            ui.label(RichText::new(*label).strong());
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.add_space(15.0);
                                ui.label(format!("${:.2}", total));
                            });
                        });

                        let available_width = ui.available_width();
                        let bar_height = 14.0;
                        let (rect, _resp) = ui
                            .allocate_exact_size(vec2(available_width, bar_height), Sense::hover());
                        let painter = ui.painter();

                        // Background bar
                        painter.rect_filled(rect, 3.0, Color32::from_gray(230));

                        // Filled portion
                        let frac = (total / max_total).clamp(0.0, 1.0);
                        let filled_width = rect.width() * frac;
                        let filled_rect = Rect {
                            min: rect.min,
                            max: pos2(rect.min.x + filled_width, rect.max.y),
                        };
                        painter.rect_filled(filled_rect, 3.0, palette[i % palette.len()]);

                        ui.add_space(8.0);
                    }
                });

            // pi chart
            let right = &mut cols[1];
            right.label("Actual incom used");
            right.spacing_mut().indent = 24.0;

            right.indent("pie_indent", |right| {
                // fraction used
                let mut total_monthly_cost: f32 = 0.0;
                for item in &budget_data.cost_items {
                    match item.1.cost_cycle {
                        CostCycle::Daily => {
                            total_monthly_cost += item.1.cost * 30.44;
                        }
                        CostCycle::Weekly => {
                            total_monthly_cost += item.1.cost * 4.348;
                        }
                        CostCycle::Monthly => {
                            total_monthly_cost += item.1.cost;
                        }
                        CostCycle::Yearly => {
                            total_monthly_cost += item.1.cost / 12.0;
                        }
                    }
                }

                let total_incom: f32 = budget_data.income_items.iter().map(|i| i.amount).sum();
                let income = total_incom.max(0.0);
                let frac = if income > 0.0 {
                    total_monthly_cost / income
                } else {
                    0.0
                };
                let frac = frac.clamp(0.0, 1.0);

                // draw pie
                let desired = vec2(right.available_width().min(220.0), 220.0);
                let (rect, _resp) = right.allocate_exact_size(desired, Sense::hover());
                let painter = right.painter();
                let center = rect.center();
                let radius = rect.width().min(rect.height()) * 0.45;

                // background circle
                painter.circle_filled(center, radius, Color32::LIGHT_GRAY);

                // sector (skip drawing when frac <= 0 to avoid thin streak)
                // Skip tiny slices (<1%) to avoid rendering artifacts
                if frac >= 0.01 {
                    let max_steps = 96usize;
                    let start = -std::f32::consts::FRAC_PI_2;
                    let end = start + frac * (2.0 * std::f32::consts::PI);
                    let mut steps = (max_steps as f32 * frac).ceil() as usize;
                    if steps < 2 {
                        steps = 2;
                    }
                    if steps > max_steps {
                        steps = max_steps;
                    }
                    let mut points = Vec::with_capacity(steps + 2);
                    points.push(center);
                    for i in 0..=steps {
                        let t = i as f32 / steps as f32;
                        let a = start + t * (end - start);
                        points.push(center + vec2(a.cos() * radius, a.sin() * radius));
                    }
                    painter.add(Shape::convex_polygon(
                        points,
                        Color32::from_rgb(196, 9, 198),
                        Stroke::NONE,
                    ));
                }

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

            right.add_space(16.0);
            right.label("Expenses by category");
            right.spacing_mut().indent = 24.0;

            right.indent("pie_categories", |right| {
                // compute totals by category
                let categories: [(ExpenceCategory, &str); 13] = [
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
                    (ExpenceCategory::Insurance, "Insurance"),
                    (ExpenceCategory::Other, "Other"),
                ];

                let mut totals: Vec<f32> = vec![0.0; categories.len()];
                for item in &budget_data.cost_items {
                    let monthly_cost = match item.1.cost_cycle {
                        CostCycle::Daily => item.1.cost * 30.44,
                        CostCycle::Weekly => item.1.cost * 4.348,
                        CostCycle::Monthly => item.1.cost,
                        CostCycle::Yearly => item.1.cost / 12.0,
                    };

                    let idx = match item.1.cost_category {
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
                        ExpenceCategory::Insurance => 11,
                        ExpenceCategory::Other => 12,
                    };
                    totals[idx] += monthly_cost;
                }

                let total_actual: f32 = totals.iter().sum();

                // draw pie with slices
                let desired = vec2(right.available_width().min(220.0), 220.0);
                let (rect, _resp) = right.allocate_exact_size(desired, Sense::hover());
                let painter = right.painter();
                let center = rect.center();
                let radius = rect.width().min(rect.height()) * 0.45;

                // background circle
                painter.circle_filled(center, radius, Color32::LIGHT_GRAY);

                if total_actual > 0.0 {
                    // simple color palette
                    let palette: [Color32; 13] = [
                        Color32::from_rgb(0xE6, 0x7E, 0x22), // orange
                        Color32::from_rgb(0x1F, 0x77, 0xB4), // blue
                        Color32::from_rgb(0x2C, 0xA0, 0x2C), // green
                        Color32::from_rgb(0xD6, 0x27, 0x28), // red
                        Color32::from_rgb(0x94, 0x67, 0xBD), // purple
                        Color32::from_rgb(0x8C, 0x56, 0x4B), // brown
                        Color32::from_rgb(0xE3, 0x77, 0xC2), // pink
                        Color32::from_rgb(0x7F, 0x7F, 0x7F), // gray
                        Color32::from_rgb(0xBC, 0xBD, 0x22), // olive
                        Color32::from_rgb(0x17, 0xBE, 0xCF), // cyan
                        Color32::from_rgb(0xFF, 0xA5, 0x00), // dark orange
                        Color32::from_rgb(0xFF, 0xA5, 0x0C), // dark orange
                        Color32::from_rgb(0xA0, 0xA0, 0xA0), // light gray
                    ];

                    let mut start = -std::f32::consts::FRAC_PI_2;
                    let two_pi = 2.0 * std::f32::consts::PI;
                    let max_steps = 96usize;

                    for (i, total) in totals.iter().enumerate() {
                        if *total <= 0.0 {
                            continue;
                        }
                        let frac = (*total / total_actual).clamp(0.0, 1.0);
                        if frac < 0.01 {
                            // skip tiny slices
                            continue;
                        }
                        let end = start + frac * two_pi;

                        let mut steps = (max_steps as f32 * frac).ceil() as usize;
                        if steps < 2 {
                            steps = 2;
                        }
                        if steps > max_steps {
                            steps = max_steps;
                        }

                        let mut points = Vec::with_capacity(steps + 2);
                        points.push(center);
                        for s in 0..=steps {
                            let t = s as f32 / steps as f32;
                            let a = start + t * (end - start);
                            points.push(center + vec2(a.cos() * radius, a.sin() * radius));
                        }
                        painter.add(Shape::convex_polygon(
                            points,
                            palette[i % palette.len()],
                            Stroke::NONE,
                        ));

                        start = end;
                    }
                }
            });
        });

        events
    }
}
