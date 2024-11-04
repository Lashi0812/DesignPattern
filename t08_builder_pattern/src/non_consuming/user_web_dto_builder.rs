use chrono::{Datelike, Local};

use crate::consuming::{address::Address, user_web_dto::UserWebDTO};

use super::user_dto_builder::UserDTOBuilder;

#[derive(Default, Clone)]
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
    fn with_first_name(&mut self, first_name: impl Into<String>) -> &mut Self {
        let _ = self.first_name.insert(first_name.into());
        self
    }

    fn with_last_name(&mut self, last_name: impl Into<String>) -> &mut Self {
        let _ = self.last_name.insert(last_name.into());
        self
    }

    fn with_birth_date(&mut self, birth_date: chrono::NaiveDate) -> &mut Self {
        let age = Local::now().year() - birth_date.year();
        let _ = self.age.insert(age.to_string());
        self
    }

    fn with_address(&mut self, address: Address) -> &mut Self {
        let _ = self.address.insert(format!(
            "{}, {}, {}, {}, {}",
            address.house_number, address.street, address.city, address.state, address.zip_code
        ));
        self
    }

    fn build(&self) -> UserWebDTO {
        UserWebDTO::new(
            format!(
                "{} {}",
                self.first_name.clone().unwrap(),
                self.last_name.clone().unwrap()
            ),
            self.address.clone().unwrap(),
            self.age.clone().unwrap(),
        )
    }
}
