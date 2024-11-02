#[allow(dead_code)]
pub struct SubscriberDetails {
    pub subscriber_id: String,
    pub address: String,
    pub phone_number: String,
    pub base_rate: i32,
}

pub trait Subscriber {
    fn calculate_bill(&self) -> i32;
}

