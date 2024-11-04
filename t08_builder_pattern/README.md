### Builder Pattern

The **Builder Pattern** is a design pattern that simplifies the construction of complex objects by breaking down the creation process into discrete steps. It is particularly useful when objects require multiple optional or configurable parameters.

### Types of Builder Patterns

1. **Consuming Builder Pattern**:
   - **Ownership Transfer**: Each builder method takes `self` (by value), consuming the builder instance after each method call.
   - **Immutability and Finalization**: The builder instance is consumed and finalized with each call, which ensures immutability and consistency.
   - **One-time Use**: After calling `build`, the builder cannot be reused for another object.

   **Use Case**: Useful when building an object in a single sequence without reuse.

2. **Non-Consuming Builder Pattern**:
   - **Mutable Reference**: Each method takes `&mut self`, allowing the same builder instance to be modified without being consumed.
   - **Reusability**: The builder instance can be reused, making it efficient for creating multiple similar objects.
   - **Chaining Support**: Method chaining is supported by returning `&mut Self` from each method.

   **Use Case**: Ideal for constructing multiple similar objects or keeping a builder template with default values.

