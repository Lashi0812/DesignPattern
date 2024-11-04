use chrono::NaiveDate;

mod consuming;
mod non_consuming;
fn main() {
    consuming_builder_pattern();
    non_consuming_builder_pattern();
}

fn consuming_builder_pattern() {
    use consuming::user_dto_builder::UserDTOBuilder;

    let user_builder: consuming::user_web_dto_builder::UserWebDTOBuilder = consuming::user_web_dto_builder::UserWebDTOBuilder::new()
        .with_first_name("Lakshmi")
        .with_last_name("Narayanan")
        .with_birth_date(NaiveDate::from_ymd_opt(1996, 10, 08).unwrap())
        .with_address(consuming::address::Address::new());

    let user = user_builder.build();
    // let user2 = user_builder.with_last_name("changed").build();

    println!("{:?}", user);
}


fn non_consuming_builder_pattern() {
    use non_consuming::user_dto_builder::UserDTOBuilder;
    
    let  mut user_builder = non_consuming::user_web_dto_builder::UserWebDTOBuilder::new();
    user_builder
        .with_first_name("Lakshmi")
        .with_last_name("Narayanan")
        .with_birth_date(NaiveDate::from_ymd_opt(1996, 10, 08).unwrap())
        .with_address(consuming::address::Address::new());

    let user = user_builder.clone().build();
    println!("{:?}", user);


    let user2 = user_builder.with_last_name("changed").build();
    print!("{:?}", user2);
}