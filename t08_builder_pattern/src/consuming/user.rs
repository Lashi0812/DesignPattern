use super::address::Address;

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct User {
    first_name: String,
    last_name: String,
    address: Address,
    birth_date: chrono::NaiveDate,
}

#[allow(dead_code)]
impl User {
    pub fn new() -> Self {
        return Self::default();
    }
}
