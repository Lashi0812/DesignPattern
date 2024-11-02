use crate::subscriber::{Subscriber, SubscriberDetails};

pub struct PhoneSubscriber {
    pub details: SubscriberDetails,
    pub usage: i32, // minutes used
}

impl Subscriber for PhoneSubscriber {
    fn calculate_bill(&self) -> i32 {
        self.details.base_rate * self.usage
    }
}
