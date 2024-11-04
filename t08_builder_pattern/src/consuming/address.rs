#[derive(Debug, Default)]
pub struct Address {
    pub house_number: String,
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
}

impl Address {
    pub fn new() -> Self {
        return  Self::default();
    }
}
