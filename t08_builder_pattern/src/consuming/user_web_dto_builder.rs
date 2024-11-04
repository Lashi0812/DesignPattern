use chrono::{Datelike, Local};
use std::fmt::Debug;

use super::{user_dto::UserDTO, user_dto_builder::UserDTOBuilder, user_web_dto::UserWebDTO};

#[derive(Default)]
pub struct UserWebDTOBuilder {
    first_name: Option<String>,
    last_name: Option<String>,
    address: Option<String>,
    age: Option<String>,
}

impl UserWebDTOBuilder {
    pub fn new() -> Self {
        UserWebDTOBuilder::default()
    }
}

impl UserDTOBuilder for UserWebDTOBuilder {
    fn with_first_name(mut self, first_name: impl Into<String>) -> Self {
        let _ = self.first_name.insert(first_name.into());
        self
    }

    fn with_last_name(mut self, last_name: impl Into<String>) -> Self {
        let _ = self.last_name.insert(last_name.into());
        self
    }

    fn with_birth_date(mut self, birth_date: chrono::NaiveDate) -> Self {
        let age = Local::now().year() - birth_date.year();
        let _ = self.age.insert(age.to_string());
        self
    }

    fn with_address(mut self, address: super::address::Address) -> Self {
        let _ = self.address.insert(format!(
            "{}, {}, {}, {}, {}",
            address.house_number, address.street, address.city, address.state, address.zip_code
        ));
        self
    }

    fn build(self) -> impl UserDTO + Debug {
        UserWebDTO::new(
            format!("{} {}", self.first_name.unwrap(), self.last_name.unwrap()),
            self.address.unwrap(),
            self.age.unwrap(),
        )
    }
}
