use crate::ledger::domain::value_objects::amount::Amount;

pub struct ExpenseProps {
    pub id: String,
    pub amount: Amount,
    pub date: String,
}

pub struct Expense {
    pub id: String,
    pub amount: Amount,
    pub date: String,
}

impl Expense {
    pub fn new(props: ExpenseProps) -> Self {
        Self {
            id: props.id,
            amount: props.amount,
            date: props.date,
        }
    }
}
