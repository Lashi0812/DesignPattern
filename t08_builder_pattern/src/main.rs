use chrono::NaiveDate;
use consuming::user_dto_builder::UserDTOBuilder;

mod consuming;
fn main() {
    consuming_builder_pattern();
}

fn consuming_builder_pattern() {
    let user_builder = consuming::user_web_dto_builder::UserWebDTOBuilder::new()
        .with_first_name("Lakshmi")
        .with_last_name("Narayanan")
        .with_birth_date(NaiveDate::from_ymd_opt(1996, 10, 08).unwrap())
        .with_address(consuming::address::Address::new());

    let user = user_builder.build();
    // let user2 = user_builder.with_last_name("changed").build();

    println!("{:?}", user);
}
