mod isp_subscriber;
mod phone_subscriber;
mod subscriber;

use crate::subscriber::Subscriber;

fn main() {
    // Create an instance of PhoneSubscriber
    let phone_subscriber = phone_subscriber::PhoneSubscriber {
        details: subscriber::SubscriberDetails {
            subscriber_id: "P123".to_string(),
            address: "1234 Elm St".to_string(),
            phone_number: "555-1234".to_string(),
            base_rate: 2, // cost per minute
        },
        usage: 120, // minutes used
    };

    // Create an instance of ISPSubscriber
    let isp_subscriber = isp_subscriber::ISPSubscriber {
        details: subscriber::SubscriberDetails {
            subscriber_id: "I456".to_string(),
            address: "5678 Maple Ave".to_string(),
            phone_number: "555-5678".to_string(),
            base_rate: 3, // cost per GB
        },
        data_usage: 50, // GB used
        free_usage: 10, // GB free usage
    };

    // Calculate and print the total bills
    let total_bill_phone = phone_subscriber.calculate_bill();
    let total_bill_isp = isp_subscriber.calculate_bill();

    println!("Total bill for Phone Subscriber: ${}", total_bill_phone);
    println!("Total bill for ISP Subscriber: ${}", total_bill_isp);
}
