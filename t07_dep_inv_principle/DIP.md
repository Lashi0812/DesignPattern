## Dependency Inversion Principle (DIP)

### Core Concept of DIP
The Dependency Inversion Principle (DIP) emphasizes two key points:
1. **High-level modules** should not depend on low-level modules. Both should depend on **abstractions**.
2. **Abstractions** should not depend on details. Instead, details should depend on abstractions.

In other words, rather than a high-level module directly depending on specific, concrete classes or implementations (which would tightly couple them), both the high-level module and the low-level details should rely on shared, abstract interfaces. This makes the system more flexible and adaptable to changes.

### Applying DIP in the Code

1. **High-Level Module (`MessageService`)**:
   - `MessageService` is the high-level module responsible for the core business logic (sending a message).
   - Instead of directly depending on specific types of formatters or writers (e.g., `JsonFormatter` or `ConsoleWriter`), `MessageService` depends on abstract traits: `Formatter` and `Writer`.
   - By doing so, `MessageService` only cares about the *ability* to format and write messages—it doesn’t need to know *how* they’re formatted or written.

2. **Abstractions (Traits `Formatter` and `Writer`)**:
   - `Formatter` and `Writer` are the abstractions that represent how a message should be formatted and where it should be written.
   - These traits define the structure of these operations but not the details, which allows for any number of possible implementations.
   - This makes `MessageService` adaptable: it can work with any formatter or writer that implements the necessary traits.

3. **Low-Level Details (`JsonFormatter`, `TextFormatter`, `ConsoleWriter`, `FileWriter`)**:
   - These structs are specific implementations (low-level details) of `Formatter` and `Writer`.
   - Rather than `MessageService` depending on these concrete types, these low-level implementations conform to the `Formatter` and `Writer` abstractions.
   - If you want to add a new type of formatter or writer, `MessageService` doesn’t need to change at all; the new types just need to implement the corresponding traits.

### Benefits of DIP in the Code Example

- **Flexibility and Extensibility**: `MessageService` can use any formatter or writer as long as they implement `Formatter` and `Writer`. This means you can easily switch from `JsonFormatter` to `TextFormatter`, or from `ConsoleWriter` to `FileWriter`, without modifying `MessageService`.
  
- **Loose Coupling**: Since `MessageService` depends on `Formatter` and `Writer` traits instead of concrete implementations, it is decoupled from the specific details of formatting and writing. This allows both the high-level logic and low-level details to evolve independently.
  
- **Easier Testing and Maintenance**: The separation of abstractions and implementations allows for easier testing. You can create mock implementations of `Formatter` and `Writer` for unit tests, which simplifies testing the core logic in `MessageService` without needing actual file I/O or console output.

