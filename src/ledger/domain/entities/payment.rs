use crate::ledger::domain::value_objects::amount::Amount;

pub struct Payment {
    pub id: String,
    pub amount: Amount,
    pub name: String,
    pub email: String,
    pub payment_type: String,
    pub date: String,
}

pub struct PaymentProps {
    pub id: String,
    pub amount: Amount,
    pub name: String,
    pub email: String,
    pub payment_type: String,
    pub date: String,
}

impl Payment {
    pub fn new(props: PaymentProps) -> Self {
        Self {
            id: props.id,
            amount: props.amount,
            name: props.name,
            email: props.email,
            payment_type: props.payment_type,
            date: props.date,
        }
    }
}
