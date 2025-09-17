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
    pub id: u64,
    pub what: String,
    pub cost: f32,
    pub cost_cycle: CostCycle,
    pub cost_category: ExpenceCategory,
    pub tags: Option<Vec<String>>,
}

impl Default for CostItem {
    fn default() -> Self {
        Self {
            id: 0,
            what: String::new(),
            cost: 0.0,
            cost_cycle: CostCycle::Weekly,
            cost_category: ExpenceCategory::Other,
            tags: None,
        }
    }
}