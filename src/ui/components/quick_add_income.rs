use eframe::egui::*;
use crate::{models::{cost_item::CostCycle, income_item::{IncomeCategory, IncomeItem}}, AppEvent};

pub struct QuickAddIncome {
    input: String,
    show_category_suggestions: bool,
    show_cycle_suggestions: bool,
    filtered_categories: Vec<(String, IncomeCategory)>,
    filtered_cycles: Vec<(String, CostCycle)>,
}

impl Default for QuickAddIncome {
    fn default() -> Self {
        Self { 
            input: String::new(), 
            show_category_suggestions: false, 
            show_cycle_suggestions: false, 
            filtered_categories: Vec::new(), 
            filtered_cycles: Vec::new()
        }
    }
}

impl QuickAddIncome {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show(&mut self, ui: &mut Ui) -> Vec<AppEvent> {
        let mut events = Vec::new();

        let all_categories = vec![
            ("salary".to_string(), IncomeCategory::Salary),
            ("freelance".to_string(), IncomeCategory::Freelance),
            ("investment".to_string(), IncomeCategory::Investment),
            ("sidehustle".to_string(), IncomeCategory::SideHustle),
            ("bonus".to_string(), IncomeCategory::Bonus),
            ("gift".to_string(), IncomeCategory::Gift),
            ("other".to_string(), IncomeCategory::Other),
        ];

        let all_cycles = vec![
            ("daily".to_string(), CostCycle::Daily),
            ("weekly".to_string(), CostCycle::Weekly),
            ("monthly".to_string(), CostCycle::Monthly),
            ("yearly".to_string(), CostCycle::Yearly),
            ("d".to_string(), CostCycle::Daily),
            ("w".to_string(), CostCycle::Weekly),
            ("m".to_string(), CostCycle::Monthly),
            ("y".to_string(), CostCycle::Yearly),
        ];

        ui.horizontal(|ui| {
            ui.add_space(10.0);
            
            ui.label("Income:");
            let response = ui.add_sized(
                [350.0, 20.0],
                TextEdit::singleline(&mut self.input)
                    .hint_text("e.g. 'Coffee 5 \\daily @dining' or 'Rent 1200 @housing'")
            );
            
            if ui.button("Add").clicked() {
                if let Some(item) = self.parse_input() {
                    events.push(AppEvent::AddIncomeItem(item));
                    self.input.clear();
                    self.show_category_suggestions = false;
                    self.show_cycle_suggestions = false;
                }
            }

            // Check if we should show suggestions
            let should_show_category_suggestions = self.input.contains('@'); // && self.input.split('@').last().unwrap_or("").len() >= 0;
            
            let should_show_cycle_suggestions = self.input.contains('\\'); //&& self.input.split('\\').last().unwrap_or("").len() >= 0;
            
            if should_show_category_suggestions {
                // Get the text after the last @
                let after_at = self.input.split('@').last().unwrap_or("").to_lowercase();
                
                // Filter categories based on input
                self.filtered_categories = all_categories
                    .iter()
                    .filter(|(name, _)| name.starts_with(&after_at))
                    .cloned()
                    .collect();
                
                self.show_category_suggestions = !self.filtered_categories.is_empty();
            } else {
                self.show_category_suggestions = false;
            }

            if should_show_cycle_suggestions {
                // Get the text after the last \
                let after_backslash = self.input.split('\\').last().unwrap_or("").to_lowercase();
                
                // Filter cycles based on input
                self.filtered_cycles = all_cycles
                    .iter()
                    .filter(|(name, _)| name.starts_with(&after_backslash))
                    .cloned()
                    .collect();
                
                self.show_cycle_suggestions = !self.filtered_cycles.is_empty();
            } else {
                self.show_cycle_suggestions = false;
            }
            
            // Handle Enter key
            if response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                if !self.input.trim().is_empty() {
                    if let Some(item) = self.parse_input() {
                        events.push(AppEvent::AddIncomeItem(item));
                        self.input.clear();
                        self.show_category_suggestions = false;
                        self.show_cycle_suggestions = false;
                        response.request_focus();
                    }
                }
            }
            
            ui.add_space(15.0);
        });

        // Show suggestions dropdown
        if self.show_category_suggestions && !self.filtered_categories.is_empty() {
            ui.add_space(2.0);
            ui.horizontal(|ui| {
                ui.add_space(140.0); // Align with text field
                
                ui.vertical(|ui| {
                    ui.style_mut().visuals.window_fill = Color32::from_gray(240);
                    ui.style_mut().visuals.window_stroke = Stroke::new(1.0, Color32::from_gray(150));
                    
                    Frame::popup(ui.style())
                        .show(ui, |ui| {
                            ui.set_max_width(200.0);
                            ui.label("Categories:");
                            
                            for (name, _category) in &self.filtered_categories {
                                if ui.selectable_label(false, format!("@{}", name)).clicked() {
                                    // Replace the partial @category with the selected one
                                    let parts: Vec<&str> = self.input.rsplitn(2, '@').collect();
                                    if parts.len() == 2 {
                                        self.input = format!("{}@{} ", parts[1], name);
                                    } else {
                                        self.input = format!("@{} ", name);
                                    }
                                    self.show_category_suggestions = false;
                                }
                            }
                        });
                });
            });
        }

        if self.show_cycle_suggestions && !self.filtered_cycles.is_empty() {
            ui.add_space(2.0);
            ui.horizontal(|ui| {
                ui.add_space(140.0); // Align with text field
                
                ui.vertical(|ui| {
                    ui.style_mut().visuals.window_fill = Color32::from_gray(240);
                    ui.style_mut().visuals.window_stroke = Stroke::new(1.0, Color32::from_gray(150));
                    
                    Frame::popup(ui.style())
                        .show(ui, |ui| {
                            ui.set_max_width(200.0);
                            ui.label("Cycles:");
                            
                            for (name, _cycle) in &self.filtered_cycles {
                                if ui.selectable_label(false, format!("\\{}", name)).clicked() {
                                    // Replace the partial \cycle with the selected one
                                    let parts: Vec<&str> = self.input.rsplitn(2, '\\').collect();
                                    if parts.len() == 2 {
                                        self.input = format!("{}\\{} ", parts[1], name);
                                    } else {
                                        self.input = format!("\\{} ", name);
                                    }
                                    self.show_cycle_suggestions = false;
                                }
                            }
                        });
                });
            });
        }

        events
    }

    fn parse_input(&self) -> Option<IncomeItem> {
        let input = self.input.trim().to_lowercase();
        
        // Extract cycle from \cycle syntax, fallback to keyword matching
        let pos_cycle: Vec<&str> = input
            .split_whitespace()
            .filter(|word| word.starts_with('\\'))
            .map(|word| &word[1..])
            .collect();

        let (name, cycle) = if let Some(cycle_name) = pos_cycle.first() {
            // Remove the \cycle word from input for name extraction
            let cleaned_input = input
                .split_whitespace()
                .filter(|word| !word.starts_with('\\') && !word.starts_with('@'))
                .collect::<Vec<_>>()
                .join(" ");
            
            let cycle = match cycle_name.to_lowercase().as_str() {
                "daily" | "day" | "d" => CostCycle::Daily,
                "weekly" | "week" | "w" => CostCycle::Weekly,
                "monthly" | "month" | "m" => CostCycle::Monthly,
                "yearly" | "year" | "annual" | "y" => CostCycle::Yearly,
                _ => CostCycle::Monthly,
            };
            
            (cleaned_input, cycle)
        } else {
            let base_input = input
                .split_whitespace()
                .filter(|word| !word.starts_with('@'))
                .collect::<Vec<_>>()
                .join(" ");

            // Fallback to keyword matching
            if base_input.contains("daily") {
                (base_input.replace("daily", "").trim().to_string(), CostCycle::Daily)
            } else if base_input.contains("weekly") {
                (base_input.replace("weekly", "").trim().to_string(), CostCycle::Weekly)
            } else if base_input.contains("yearly") || base_input.contains("annual") {
                (base_input.replace("yearly", "").replace("annual", "").trim().to_string(), CostCycle::Yearly)
            } else if base_input.contains("monthly") {
                (base_input.replace("monthly", "").trim().to_string(), CostCycle::Monthly)
            } else {
                (base_input, CostCycle::Monthly)
            }
        };
        
        // Extract category from @category syntax
        let pos_cat: Vec<&str> = input
            .split_whitespace()
            .filter(|word| word.starts_with('@'))
            .map(|word| &word[1..])
            .collect();

        let income_category = if let Some(cat_name) = pos_cat.first() {
            match cat_name.to_lowercase().as_str() {
                "salary" | "wage" | "job" | "work" | "employment" | "paycheck" => IncomeCategory::Salary,
                "freelance" | "freelancing" | "contract" | "contractor" | "consulting" | "gig" => IncomeCategory::Freelance,
                "investment" | "invest" | "dividend" | "capital" | "stock" | "crypto" | "trading" | "portfolio" => IncomeCategory::Investment,
                "sidehustle" | "side" | "hustle" | "business" | "startup" | "venture" | "entrepreneurship" => IncomeCategory::SideHustle,
                "bonus" | "commission" | "incentive" | "reward" | "tip" | "gratuity" => IncomeCategory::Bonus,
                "gift" | "present" | "donation" | "inheritance" | "windfall" | "lottery" => IncomeCategory::Gift,
                _ => IncomeCategory::Other,
            }
        } else {
            // Fallback to keyword matching
            if input.contains("salary") || input.contains("wage") || input.contains("job") || input.contains("work") {
                IncomeCategory::Salary
            } else if input.contains("freelance") || input.contains("contract") || input.contains("consulting") {
                IncomeCategory::Freelance
            } else if input.contains("invest") || input.contains("dividend") || input.contains("stock") || input.contains("crypto") {
                IncomeCategory::Investment
            } else if input.contains("business") || input.contains("side") || input.contains("hustle") {
                IncomeCategory::SideHustle
            } else if input.contains("bonus") || input.contains("commission") || input.contains("tip") {
                IncomeCategory::Bonus
            } else if input.contains("gift") || input.contains("present") || input.contains("inheritance") {
                IncomeCategory::Gift
            } else {
                IncomeCategory::Other
            }
        };
        
        // Extract number and name
        let (final_name_with_number, amount) = {
            let words: Vec<&str> = name.split_whitespace().collect();
            let mut parsed_amount = 0.0;
            let mut name_parts = Vec::new();
            
            for word in words {
                let clean_word = word.trim_start_matches('$').trim_start_matches('€');
                if let Ok(num) = clean_word.parse::<f32>() {
                    parsed_amount = num;
                } else {
                    name_parts.push(word);
                }
            }
            
            (name_parts.join(" "), parsed_amount)
        };
        
        // Capitalize first letter of name
        let final_name = if final_name_with_number.trim().is_empty() {
            "New Item".to_string()
        } else {
            let mut chars: Vec<char> = final_name_with_number.chars().collect();
            if !chars.is_empty() {
                chars[0] = chars[0].to_uppercase().next().unwrap_or(chars[0]);
            }
            chars.into_iter().collect()
        };
        
        Some(IncomeItem {
            source: final_name,
            category: income_category,
            amount,
            income_cycle: cycle,
            tags: None,
        })
    }
}