struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
}

fn main() {
    let c: Container<Option<String>> = Container::new(None);
}
