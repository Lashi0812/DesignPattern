use crate::consuming::{address::Address, user_web_dto::UserWebDTO};

pub trait UserDTOBuilder {
    fn with_first_name(&mut self, first_name: impl Into<String>) -> &mut Self;
    fn with_last_name(&mut self, last_name: impl Into<String>) -> &mut Self;
    fn with_birth_date(&mut self, birth_date: chrono::NaiveDate) -> &mut Self;
    fn with_address(&mut self, address: Address) -> &mut Self;
    fn build(&self) -> UserWebDTO;
}
