use std::fmt::Debug;

use super::{address::Address, user_dto::UserDTO};

pub trait UserDTOBuilder {
    fn with_first_name(self, first_name: impl Into<String>) -> Self;
    fn with_last_name(self, last_name: impl Into<String>) -> Self;
    fn with_birth_date(self, birth_date: chrono::NaiveDate) -> Self;
    fn with_address(self, address: Address) -> Self;
    fn build(self) -> impl UserDTO + Debug;
}
