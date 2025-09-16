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
    Insurance,
    Other,
}

#[derive(Debug, Clone)]
pub struct CostItem {
    pub what: String,
    pub cost: f32,
    pub cost_cycle: CostCycle,
    pub cost_category: ExpenceCategory,
    pub tags: Option<Vec<String>>,
    pub new_tag_input: String,
}

impl Default for CostItem {
    fn default() -> Self {
        Self {
            what: String::new(),
            cost: 0.0,
            cost_cycle: CostCycle::Weekly,
            cost_category: ExpenceCategory::Other,
            tags: None,
            new_tag_input: String::new(),
        }
    }
}