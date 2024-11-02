use crate::subscriber::{Subscriber, SubscriberDetails};

pub struct ISPSubscriber {
    pub details: SubscriberDetails,
    pub data_usage: i32, // GB used
    pub free_usage: i32, // GB available for free
}

impl Subscriber for ISPSubscriber {
    fn calculate_bill(&self) -> i32 {
        let chargeable_usage = if self.data_usage > self.free_usage {
            self.data_usage - self.free_usage
        } else {
            0
        };
        chargeable_usage * self.details.base_rate
    }
}
