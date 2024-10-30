trait MyTrait {
    fn hello_trait(&self);
}

struct MyStruct1;
struct MyStruct2;

impl MyStruct1 {
    fn hello_struct(&self) {
        println!("hello struct 1");
    }
}

impl MyStruct2 {
    fn hello_struct(&self) {
        println!("hello struct 2");
    }
}

impl MyTrait for MyStruct1 {
    fn hello_trait(&self) {
        self.hello_struct();
    }
}

impl MyTrait for MyStruct2 {
    fn hello_trait(&self) {
        self.hello_struct();
    }
}

fn main() {
    let mut v = Vec::<Box<dyn MyTrait>>::new();
    v.push(Box::new(MyStruct1 {}));
    v.push(Box::new(MyStruct2 {}));
    v.iter().for_each(|i| i.hello_trait());
}
