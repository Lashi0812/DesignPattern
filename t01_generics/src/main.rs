use std::marker::PhantomData;

// error[E0392]: type parameter `Breed` is never used
//  --> t01_generics/src/main.rs:1:12
//   |
// 1 | struct Dog<Breed>{
//   |            ^^^^^ unused type parameter
//   |
//   = help: consider removing `Breed`, referring to it in a field, or using a marker such as `PhantomData`
struct Dog<Breed> {
    name: String,
    breed: PhantomData<Breed>,
}

struct Labrador {}
struct Retriever {}
struct Poodle {}
struct Dachshund {}

impl Dog<Labrador> {
    fn breed_name(&self) -> &str {
        "Labrador"
    }
}

impl Dog<Retriever> {
    fn breed_name(&self) -> &str {
        "Retriever"
    }
}

impl Dog<Poodle> {
    fn breed_name(&self) -> &str {
        "Poodle"
    }
}

impl Dog<Dachshund> {
    fn breed_name(&self) -> &str {
        "Dachshund"
    }
}
fn main() {
    let my_poodle: Dog<Poodle> = Dog {
        name: "Rex".to_string(),
        breed: PhantomData,
    };

    println!(
        "My dog is a {} ,named {}",
        my_poodle.breed_name(),
        my_poodle.name
    );
}
