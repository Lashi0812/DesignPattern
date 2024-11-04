# Non-Consuming Builder Pattern in Rust

## Core Concept
The non-consuming builder pattern uses mutable references (`&mut self`) instead of taking ownership, allowing the builder to be reused multiple times. This pattern is useful when you need to create multiple similar objects with slight variations.

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
- Final object to be constructed
- Can be created multiple times from same builder
- Often implements Clone or Debug

### 2. Builder Trait
```rust
trait UserBuilder {
    // Note &mut self instead of self
    fn with_first_name(&mut self, first_name: impl Into<String>) -> &mut Self;
    fn with_last_name(&mut self, last_name: impl Into<String>) -> &mut Self;
    fn build(&self) -> Result<User, BuilderError>;
}
```
- Methods take `&mut self` instead of ownership
- Returns mutable reference for chaining
- build() takes `&self` since it doesn't consume
- Can implement Clone for builder reuse

### 3. Concrete Builder
```rust
#[derive(Default)]
struct UserWebBuilder {
    first_name: Option<String>,
    last_name: Option<String>,
    address: Option<Address>,
    birth_date: Option<NaiveDate>,
}

impl UserWebBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}
```
- Holds intermediate state
- Can be reset and reused
- Often implements Default
- May include reset functionality

### 4. Builder Implementation
```rust
impl UserBuilder for UserWebBuilder {
    fn with_first_name(&mut self, first_name: impl Into<String>) -> &mut Self {
        self.first_name = Some(first_name.into());
        self
    }

    fn build(&self) -> Result<User, BuilderError> {
        Ok(User {
            first_name: self.first_name.clone().ok_or(BuilderError::MissingField)?,
            // ... other fields
        })
    }
}
```
- Uses mutable references
- Clones data when building
- Can validate without consuming
- Returns Result for error handling

## Key Characteristics

### 1. Reference Model
- Methods take `&mut self`
- Returns `&mut Self` for chaining
- Builder remains valid after build()
- Can create multiple objects

### 2. State Management
```rust
let mut builder = UserWebBuilder::new();

let user1 = builder
    .with_first_name("John")
    .with_last_name("Doe")
    .build()?;

// Builder can be reused
let user2 = builder
    .with_first_name("Jane")
    .build()?;

// Can reset builder state
builder.reset();
```
- Mutable state persists
- Can be reset explicitly
- State can be reused
- Modifications are cumulative

### 3. Memory Implications
- May require more cloning
- Data lives longer
- Potential for larger memory footprint
- References must be managed

### 3. Clone Support
```rust
#[derive(Clone)]
struct UserWebBuilder {
    // fields...
}
```
- Clone builder state
- Create variations
- Preserve templates
- Branch construction

