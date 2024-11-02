use chrono::{DateTime, Local};

// Define the trait for entity behavior
pub trait EntityData {
    fn id(&self) -> u64;
    fn set_id(&mut self, id: u64);
}

// Basic implementation of EntityData
#[derive(Debug, Default)]
pub struct BasicEntity {
    id: u64,
}

impl EntityData for BasicEntity {
    fn id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }
}

// Generic Order that can work with any EntityData implementation
#[derive(Debug)]
pub struct Order<E: EntityData> {
    entity: E,
    order_placed_on: DateTime<Local>,
    total_value: f64,
}

impl<E: EntityData> Order<E> {
    pub fn new(entity: E) -> Self {
        Self {
            entity,
            order_placed_on: Local::now(),
            total_value: 0.0,
        }
    }

    // Delegate to entity
    pub fn id(&self) -> u64 {
        self.entity.id()
    }

    pub fn set_id(&mut self, id: u64) {
        self.entity.set_id(id)
    }

    // Order-specific methods
    pub fn total_value(&self) -> f64 {
        self.total_value
    }

    pub fn set_total_value(&mut self, value: f64) {
        self.total_value = value;
    }
}

// Generic User that can work with any EntityData implementation
#[derive(Debug)]
pub struct User<E: EntityData> {
    entity: E,
    name: String,
    last_login: DateTime<Local>,
}

impl<E: EntityData> User<E> {
    pub fn new(entity: E, name: String) -> Self {
        Self {
            entity,
            name,
            last_login: Local::now(),
        }
    }

    // Delegate to entity
    pub fn id(&self) -> u64 {
        self.entity.id()
    }

    pub fn set_id(&mut self, id: u64) {
        self.entity.set_id(id)
    }

    // User-specific methods
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

// You could also create specialized entities for different use cases
#[derive(Debug)]
pub struct AuditableEntity {
    id: u64,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl EntityData for AuditableEntity {
    fn id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }
}

impl AuditableEntity {
    pub fn new() -> Self {
        let now = Local::now();
        Self {
            id: 0,
            created_at: now,
            updated_at: now,
        }
    }
}

fn main() {
    // Basic usage
    let order = Order::new(BasicEntity::default());

    // With auditable entity
    let auditable_order = Order::new(AuditableEntity::new());

    // Type aliases can make it cleaner
    type BasicOrder = Order<BasicEntity>;
    type AuditableOrder = Order<AuditableEntity>;
}
