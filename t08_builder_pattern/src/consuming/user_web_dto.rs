use super::user_dto::UserDTO;

#[derive(Debug)]
pub struct UserWebDTO {
    name: String,
    address: String,
    age: String,
}

impl UserWebDTO {
    pub fn new(name: String, address: String, age: String) -> Self {
        Self { name, address, age }
    }
}

impl UserDTO for UserWebDTO {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_address(&self) -> String {
        self.address.clone()
    }
    fn get_age(&self) -> String {
        self.age.clone()
    }
}
