use crate::models::cost_item::CostCycle;

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
pub struct IncomeItem {
    pub source: String,
    pub category: IncomeCategory,
    pub amount: f32,
    pub income_cycle: CostCycle,
    pub tags: Option<Vec<String>>,
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