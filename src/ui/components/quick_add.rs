use eframe::egui::*;

use crate::models::cost_item::{CostCycle, CostItem, ExpenceCategory};

pub struct QuickAdd {
    input: String,
    show_category_suggestions: bool,
    show_cycle_suggestions: bool,
    filtered_categories: Vec<(String, ExpenceCategory)>,
    filtered_cycles: Vec<(String, CostCycle)>,
}

impl Default for QuickAdd {
    fn default() -> Self {
        Self {
            input: String::new(),
            show_category_suggestions: false,
            show_cycle_suggestions: false,
            filtered_categories: Vec::new(),
            filtered_cycles: Vec::new(),
        }
    }
}

impl QuickAdd {
    pub fn new() -> Self {
        Self::default()
    }

    // Returns Some(CostItem)
    pub fn show(&mut self, ui: &mut Ui) -> Option<CostItem> {
        let mut result = None;

        // Define all categories and cycles
        let all_categories = vec![
            ("housing".to_string(), ExpenceCategory::Housing),
            ("transportation".to_string(), ExpenceCategory::Transportation),
            ("groceries".to_string(), ExpenceCategory::Groceries),
            ("healthcare".to_string(), ExpenceCategory::Healthcare),
            ("personalcare".to_string(), ExpenceCategory::PersonalCare),
            ("diningout".to_string(), ExpenceCategory::DiningOut),
            ("entertainment".to_string(), ExpenceCategory::Entertainment),
            ("shopping".to_string(), ExpenceCategory::Shopping),
            ("savings".to_string(), ExpenceCategory::Savings),
            ("debt".to_string(), ExpenceCategory::DebtPayments),
            ("utilities".to_string(), ExpenceCategory::Utilities),
            ("insurance".to_string(), ExpenceCategory::Insurance),
            ("other".to_string(), ExpenceCategory::Other),
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
            ui.add_space(20.0);
            
            ui.label("Quick add:");
            let response = ui.add_sized(
                [300.0, 20.0],
                TextEdit::singleline(&mut self.input)
                    .hint_text("e.g. 'Coffee 5 \\daily @dining' or 'Rent 1200 @housing'")
            );
            
            // Check if we should show suggestions
            let should_show_category_suggestions = self.input.contains('@') && 
                self.input.split('@').last().unwrap_or("").len() >= 0;
            
            let should_show_cycle_suggestions = self.input.contains('\\') && 
                self.input.split('\\').last().unwrap_or("").len() >= 0;
            
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
                        result = Some(item);
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

        result
    }

    fn parse_input(&self) -> Option<CostItem> {
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

        let category = if let Some(cat_name) = pos_cat.first() {
            match cat_name.to_lowercase().as_str() {
                "housing" | "house" | "rent" | "mortgage" => ExpenceCategory::Housing,
                "transportation" | "transport" | "travel" | "car" | "bus" | "train" | "gas" | "fuel" => ExpenceCategory::Transportation,
                "groceries" | "grocery" | "food" | "supermarket" => ExpenceCategory::Groceries,
                "healthcare" | "health" | "medical" | "doctor" | "medicine" | "pharmacy" => ExpenceCategory::Healthcare,
                "personalcare" | "personal" | "care" | "hygiene" | "beauty" | "haircut" => ExpenceCategory::PersonalCare,
                "diningout" | "dining" | "restaurant" | "takeout" | "coffee" | "lunch" => ExpenceCategory::DiningOut,
                "entertainment" | "fun" | "movie" | "games" | "netflix" | "streaming" | "concert" => ExpenceCategory::Entertainment,
                "shopping" | "shop" | "clothes" | "clothing" | "retail" => ExpenceCategory::Shopping,
                "savings" | "save" | "investment" | "invest" => ExpenceCategory::Savings,
                "debt" | "debtpayments" | "loan" | "credit" | "payment" => ExpenceCategory::DebtPayments,
                "utilities" | "utility" | "electricity" | "water" | "internet" | "phone" => ExpenceCategory::Utilities,
                "insurance" | "insure" | "policy" => ExpenceCategory::Insurance,
                _ => ExpenceCategory::Other,
            }
        } else {
            // Fallback to keyword matching
            if input.contains("rent") || input.contains("mortgage") || input.contains("utilities") {
                ExpenceCategory::Housing
            } else if input.contains("transport") || input.contains("gas") || input.contains("car") || input.contains("bus") || input.contains("ruter") {
                ExpenceCategory::Transportation
            } else if input.contains("food") || input.contains("grocery") || input.contains("groceries") {
                ExpenceCategory::Groceries
            } else if input.contains("restaurant") || input.contains("dining") || input.contains("coffee") || input.contains("lunch") {
                ExpenceCategory::DiningOut
            } else if input.contains("movie") || input.contains("entertainment") || input.contains("netflix") {
                ExpenceCategory::Entertainment
            } else if input.contains("doctor") || input.contains("health") || input.contains("medicine") {
                ExpenceCategory::Healthcare
            } else if input.contains("save") || input.contains("saving") {
                ExpenceCategory::Savings
            } else if input.contains("insurance") || input.contains("insure") {
                ExpenceCategory::Insurance
            } else {
                ExpenceCategory::Other
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
        
        Some(CostItem {
            what: final_name,
            cost: amount,
            cost_cycle: cycle,
            cost_category: category,
            tags: None,
            new_tag_input: String::new(),
        })
    }
}