# Consuming Builder Pattern in Rust 

## Core Concept
The consuming builder pattern is a creational design pattern that enables step-by-step construction of complex objects while maintaining ownership semantics in Rust. Each builder method consumes and returns ownership of the builder instance.

## Key Components

### 1. Target Object (Product)
```rust
struct User {
    first_name: String,
    last_name: String,
    address: Address,
    birth_date: NaiveDate,
}
```
- The final object to be constructed
- Often complex with multiple fields
- May implement traits for behavior

### 2. Builder Trait
```rust
trait UserDTOBuilder {
    fn with_first_name(self, first_name: impl Into<String>) -> Self;
    fn with_address(self, address: Address) -> Self;
    fn build(self) -> impl UserDTO + Debug;
}
```
- Defines the interface for construction
- Methods take ownership (`self` instead of `&mut self`)
- Each method returns `Self` for method chaining
- Final `build()` method consumes the builder

### 3. Concrete Builder
```rust
struct UserWebDTOBuilder {
    first_name: Option<String>,
    last_name: Option<String>,
    address: Option<String>,
    age: Option<String>,
}
```
- Implements the builder trait
- Holds intermediate state during construction
- Uses Option<T> for optional fields
- Implements Default for initialization

### 4. Builder Implementation
```rust
impl UserDTOBuilder for UserWebDTOBuilder {
    fn with_first_name(mut self, first_name: impl Into<String>) -> Self {
        self.first_name = Some(first_name.into());
        self
    }
    
    fn build(self) -> impl UserDTO + Debug {
        UserWebDTO::new(
            // ... construct final object
        )
    }
}
```
- Implements each builder method
- Takes ownership of self
- Returns ownership after modification
- Final build method validates and constructs target

## Key Characteristics

### 1. Ownership Model
- Builder methods consume `self`
- Each method returns ownership back
- Builder cannot be reused after `build()`
- Enforces single-use builder pattern

### 2. Type Safety
- Compiler enforces field initialization
- No partial object construction
- Type system ensures required fields
- Compile-time validation

### 3. Method Chaining
```rust
let user = UserWebDTOBuilder::new()
    .with_first_name("John")
    .with_last_name("Doe")
    .build();
```
- Fluent interface
- Each method returns Self
- Enables natural API usage
- Clear construction sequence

### 4. Memory Safety
- No dangling references
- Clear ownership at each step
- Single-use builder prevents bugs
- Resource cleanup guaranteed

