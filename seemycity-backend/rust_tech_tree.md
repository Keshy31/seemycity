# Rust Learning Progression (RPG Tech Tree)

This document tracks concepts learned during the SeeMyCity backend development.

## Level 1: Foundations

### 1. Cargo - The Build Tool & Package Manager (Unlocked)

*   **Concept**: Cargo handles building your code, managing external libraries (crates), running projects, testing, and more. It's the central tool for Rust development.
*   **Key Files**: `Cargo.toml` (project manifest, dependencies), `Cargo.lock` (locks dependency versions).
*   **Key Commands**:
    *   `cargo new <name>`: Create a new project.
    *   `cargo build`: Compile the project.
    *   `cargo run`: Compile and run the project.
    *   `cargo check`: Quickly check code for errors without full compilation.
*   **Example (`Cargo.toml`)**:
    ```toml
    [package]
    name = "my_project"
    version = "0.1.0"
    edition = "2021" # Or "2024" as we used

    [dependencies]
    # Add external crates here, e.g.:
    # rand = "0.8" 
    ```

### 2. Core Syntax & Basic Types (Unlocked)

*   **Concept**: Fundamental building blocks of Rust code. Statements end with semicolons. Variables are immutable by default (`let`) unless made mutable (`let mut`). Basic data types like integers (`i32`, `u64`, etc.), floats (`f64`), booleans (`bool`), characters (`char`), string literals (`&str`), and tuples.
*   **Example**:
    ```rust
    // main.rs
    fn main() {
        let message: &str = "Hello, Rust!"; // message is an immutable string slice (&str)
        let mut count: i32 = 0;           // count is a mutable 32-bit integer (i32)
        let is_ready: bool = true;        // is_ready is a boolean (bool)
        let price_tuple = ("Book", 9.99); // A tuple holding &str and f64

        println!("{}", message); // Use println! macro for output
        count += 1;
        println!("Count is: {}", count);
        // Access tuple elements by index
        println!("Item: {}, Price: {}", price_tuple.0, price_tuple.1); 
    }
    ```

### 3. Functions (Unlocked)

*   **Concept**: Reusable blocks of code. Defined with `fn`. Arguments and return types must be specified using type annotations. The last expression in a function body is implicitly returned (no semicolon needed for the return expression).
*   **Example**:
    ```rust
    // Function definition
    fn add_one(x: i32) -> i32 {
        x + 1 // Implicit return, no semicolon
    }

    fn main() {
        let five: i32 = 5;
        let six: i32 = add_one(five);
        println!("{} + 1 = {}", five, six);
    }
    ```

### 4. Modules - Declaration & Basic Structure (Unlocked - Partially)

*   **Concept**: Organize code into logical units and control visibility (public/private). Files (`some_module.rs`) and directories (`dir_module/mod.rs`) can form modules. Declare modules using the `mod` keyword in their parent module/file (like `main.rs` or `lib.rs`).
*   **Status**: We have created the file structure (`src/api/mod.rs`, `src/config.rs`, etc.). Next step is to declare them using `mod` in `main.rs`.
*   **Example (File Structure)**:
    ```
    src/
    ├── main.rs
    └── utils.rs // Defines the 'utils' module
    ```
*   **Example (Declaration - *to be added in main.rs*)**:
    ```rust
    // In main.rs (conceptual, we'll add this next)
    mod utils; // Declares that src/utils.rs exists and makes the 'utils' module available

    fn main() {
       // Later, we might call a function like:
       // utils::helper_function(); 
    }
    ```

### 5. Attributes & Macros (Unlocked)

*   **Concept**:
    *   **Attributes (`#[...]`)**: Metadata attached to code items (functions, structs, modules, etc.) that modify behavior or provide info to the compiler or other tools (like `actix-web` or `derive`).
    *   **Macros (`name!(...)`)**: Code that writes other code (metaprogramming). Used for reducing boilerplate (`println!`, `vec!`), creating domain-specific languages, conditional compilation, etc. Identified by the `!`.
*   **Example**:
    ```rust
    #[derive(Debug)] // Attribute macro to automatically implement the Debug trait for printing
    struct Point { x: i32, y: i32 }

    fn main() {
        // println! is a macro
        println!("Hello!");

        // Using the Point struct derived with Debug
        let p = Point { x: 10, y: 20 };
        println!("Point is: {:?}", p); // {:?} uses the Debug format provided by #[derive(Debug)]

        // #[actix_web::main] and #[get("/")] are other attribute examples we used.
    }
    ```

### 6. Async Basics (Unlocked)

*   **Concept**: Perform operations (especially I/O like network requests or file access) without blocking the current thread, enabling concurrency and responsiveness. `async fn` defines an asynchronous function which returns a `Future`. `await` pauses the `async fn`'s execution until the awaited `Future` completes. Requires an async runtime (like Tokio) to execute the `Future`s.
*   **Example (Conceptual - needs `tokio` dependency and `#[tokio::main]`)**:
    ```rust
    async fn fetch_data_from_network() -> String {
        // In real code, this would use an async HTTP client like reqwest:
        // let body = reqwest::get("[https://example.com](https://example.com)").await?.text().await?;
        
        // Simulate network delay:
        println!("Simulating network fetch...");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await; // Pause for 1 sec
        println!("Simulated fetch complete!");
        "Data from network".to_string()
    }

    #[tokio::main] // Attribute macro from tokio crate to setup the runtime
    async fn main() {
        let data = fetch_data_from_network().await;
        println!("Received: {}", data);
    }
    ```

### 7. Error Handling - `Result` & `?` (Unlocked)

*   **Concept**: Rust handles recoverable errors using the `Result<T, E>` enum, preferring it over exceptions for expected failure conditions. `Ok(T)` holds a success value of type `T`, `Err(E)` holds an error value of type `E`. The `?` operator simplifies error propagation: if the result is `Ok(v)`, it unwraps to `v`; if it's `Err(e)`, it immediately returns `Err(e)` from the current function.
*   **Example**:
    ```rust
    use std::fs::File;
    use std::io::{self, Read};

    // Function that returns a Result indicating potential failure
    fn read_config_file() -> Result<String, io::Error> {
        // Attempt to open file, ? returns Err if it fails
        let mut file_handle = File::open("config.txt")?; 
        
        let mut contents = String::new();
        // Attempt to read file, ? returns Err if it fails
        file_handle.read_to_string(&mut contents)?; 
        
        // If both operations succeeded, wrap the result in Ok
        Ok(contents) 
    }

    fn main() {
        // Use 'match' to handle the Result explicitly
        match read_config_file() {
            Ok(config) => println!("Config loaded: {}", config),
            Err(e) => println!("Error loading config: {}", e),
        }
    }
    ```

### 8. Actix Web - Basic Server (Unlocked)

*   **Concept**: A high-performance, asynchronous web framework for Rust. Key components used so far: `HttpServer` (builds the server), `App` (configures application routes, middleware, state), handler functions (async functions decorated with `#[get]`, `#[post]`, etc. to process requests), and the `Responder` trait (types that can be converted into HTTP responses).
*   **Example (Simplified from our project)**:
    ```rust
    use actix_web::{get, App, HttpServer, Responder, HttpResponse};

    // Handler for GET /status
    #[get("/status")]
    async fn status() -> impl Responder {
        HttpResponse::Ok().body("Server is up!") // Return an explicit HTTP 200 OK
    }

    #[actix_web::main]
    async fn main() -> std::io::Result<()> {
        println!("Starting server at [http://127.0.0.1](http://127.0.0.1):8080");
        HttpServer::new(|| {
            // App factory closure: configures the application instance
            App::new().service(status) // Register the /status route handler
        })
        .bind(("127.0.0.1", 8080))? // Bind to address and port
        .run() // Run the server
        .await // Await server termination
    }
    ```

### 9. Scope & Imports (`use`) (Unlocked)

*   **Concept**: The `use` keyword brings items (structs, functions, traits, enums, modules, etc.) from other modules or external crates into the current scope. This avoids needing to write the full path every time and makes code cleaner.
*   **Example**:
    ```rust
    // Without use: Requires full path
    // fn main() {
    //     let mut map = std::collections::HashMap::new();
    //     map.insert(1, "one");
    // }

    // With use: Import HashMap into scope
    use std::collections::HashMap; 

    fn main() {
        let mut map = HashMap::new(); // Can now use HashMap directly
        map.insert(1, "one");
        println!("{:?}", map);
    }
    ```

### 10. Module System (`mod`, `pub mod`) (Unlocked)

*   **Concept**: Rust's module system allows organizing code into logical units called modules. You can nest modules inside each other.
    *   `mod module_name;` in `main.rs` or `lib.rs` tells the compiler to look for `src/module_name.rs` or `src/module_name/mod.rs`.
    *   Modules and their contents (functions, structs, etc.) are private by default.
    *   The `pub` keyword makes an item (module, function, struct, etc.) visible/usable outside its defining module.
    *   `pub mod module_name;` makes the *module itself* public, allowing other parts of the crate to refer to it.
*   **Example** (`main.rs`):
    ```rust
    // Declare the 'handlers' module, making the module itself public
    pub mod handlers;

    fn main() {
        // We can now refer to the handlers module
        // (but we still need `use` or full paths to access items *inside* it)
    }
    ```
*   **Example** (`src/handlers/mod.rs`):
    ```rust
    // This function needs to be public to be called from main.rs
    pub fn some_handler() {
        println!("Handling request...");
    }

    // This function is private to the handlers module
    fn internal_helper() {
        println!("Helper logic.");
    }
    ```

### 11. Paths and `use` (`use crate::...`) (Unlocked)

*   **Concept**: To use items (functions, structs, etc.) defined in other modules, you need to bring them into the current scope using the `use` keyword or refer to them via their full path.
    *   `crate` refers to the root of your current crate (where `main.rs` or `lib.rs` lives).
    *   `::` is the path separator.
    *   `use crate::module::item;` brings `item` from `module` into the current scope.
    *   You can also use relative paths (`super::` for parent module, `self::` for current module).
*   **Example** (`main.rs`):
    ```rust
    pub mod handlers; // Declare the module

    // Bring the public 'some_handler' function into main.rs's scope
    use crate::handlers::some_handler;

    fn main() {
        // Now we can call it directly
        some_handler();

        // Alternatively, use the full path (if not using `use`)
        // crate::handlers::some_handler();
    }
    ```

### 12. Structs (Basic Definition) (Unlocked)

*   **Concept**: Custom data structures that group related data together. Defined using the `struct` keyword.
*   **Fields**: Structs contain named fields, each with a specific data type.
*   **Visibility**: Structs and their fields can be made public (`pub`) to be accessible outside their module.
*   **Example**:
    ```rust
    // Define a struct named Point
    struct Point {
        x: i32,
        y: i32,
    }

    // Define a public struct with public fields
    pub struct Color {
        pub red: u8,
        pub green: u8,
        pub blue: u8,
    }

    fn main() {
        // Create an instance of Point
        let origin = Point { x: 0, y: 0 };
        // Access fields using dot notation
        println!("Origin x: {}", origin.x);

        let blue = Color { red: 0, green: 0, blue: 255 };
        println!("Blue's red component: {}", blue.red);
    }
    ```

### 13. Enums (`Option<T>`) (Unlocked)

*   **Concept**: Enums (enumerations) allow defining a type by listing its possible *variants*. `Option<T>` is a predefined enum in the standard library used to represent a value that might be present or absent.
*   **Variants**: `Option<T>` has two variants:
    *   `Some(T)`: Indicates a value of type `T` is present.
    *   `None`: Indicates the value is absent.
*   **Use Case**: Handling optional data, potentially missing values (like from configuration, database results, or API responses).
*   **Example**:
    ```rust
    fn find_user(id: u32) -> Option<String> {
        if id == 1 {
            Some("Alice".to_string()) // User found
        } else {
            None // User not found
        }
    }

    fn main() {
        let user1 = find_user(1);
        let user2 = find_user(2);

        // Need to handle both Some and None cases (e.g., with match or if let)
        match user1 {
            Some(name) => println!("User 1: {}", name),
            None => println!("User 1 not found."),
        }
        // Output: User 1: Alice

        if let Some(name) = user2 {
            println!("User 2: {}", name);
        } else {
            println!("User 2 not found.");
        }
        // Output: User 2 not found.
    }
    ```

### 14. Traits (Deriving Common Traits) (Unlocked)

*   **Concept**: Traits define shared functionality (like interfaces in other languages). The `#[derive]` attribute automatically implements some common traits for your structs and enums.
*   **Common Derivable Traits**:
    *   `Debug`: Enables printing the struct/enum using `{:?}` format (for debugging).
    *   `Clone`: Enables creating a copy of an instance (`.clone()` method).
    *   `Copy`: Enables simple bit-wise copying (only for simple types that don't manage resources like memory).
    *   `PartialEq`, `Eq`: Enables equality comparison (`==`).
    *   `PartialOrd`, `Ord`: Enables ordering comparison (`<`, `>`, etc.).
*   **Example**:
    ```rust
    // Automatically implement Debug and Clone traits for Point
    #[derive(Debug, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }

    fn main() {
        let p1 = Point { x: 10, y: 20 };
        // Use Debug formatting
        println!("Point p1: {:?}", p1);
        // Use Clone
        let p2 = p1.clone();
        println!("Point p2 (cloned): {:?}", p2);
    }
    ```

### 15. Serialization (`serde`) (Unlocked)

*   **Concept**: `serde` is a framework for *ser*ializing Rust data structures into formats like JSON, YAML, etc., and *de*serializing them back.
*   **Usage**: Add `serde` to `Cargo.toml` (with features like `derive`). Use `#[derive(Serialize, Deserialize)]` on your structs/enums.
*   **Example** (`src/models.rs`):
    ```rust
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct User {
        pub id: u32,
        pub username: String,
        pub active: bool,
        pub email: Option<String>, // Serde handles Option nicely (-> null)
    }
    ```
*   **Example** (Using it, often with `serde_json`):
    ```rust
    // Assuming User struct from above
    // use crate::models::User;

    fn main() -> Result<(), serde_json::Error> {
        let user = User {
            id: 1,
            username: "alice".to_string(),
            active: true,
            email: Some("alice@example.com".to_string()),
        };

        // Serialize to JSON string
        let json_string = serde_json::to_string_pretty(&user)?;
        println!("Serialized JSON:\n{}", json_string);

        // Deserialize from JSON string
        let json_data = r#"{
          "id": 2,
          "username": "bob",
          "active": false,
          "email": null
        }"#;
        let user2: User = serde_json::from_str(json_data)?;
        println!("\nDeserialized User: {:?}", user2);

        Ok(())
    }
    ```

### 16. Configuration Management (`dotenvy`, `std::env`)
- **Concept:** Loading application configuration from environment variables, often sourced from a `.env` file.
- **Mechanism:**
  - `dotenvy::dotenv().ok();`: Attempts to load variables from a `.env` file into the environment. `.ok()` ignores errors if the file is missing.
  - `std::env::var("VAR_NAME")`: Reads the value of a specific environment variable. Returns a `Result<String, VarError>`.
- **Use Case:** Keeping sensitive data (like DB credentials) and environment-specific settings out of source code.
- **Status:** Implemented in `main.rs` and `config.rs`.

### 17. Error Handling (`Result`, `match`, `?`, Custom Errors)
- **Concept:** Explicitly handling potential failure points in the code.
- **Mechanisms:**
  - `Result<T, E>`: Standard enum for representing success (`Ok(T)`) or failure (`Err(E)`).
  - `match`: Used to handle both `Ok` and `Err` variants of a `Result`.
  - `?` operator: Propagates errors up the call stack. If used on an `Err(E)`, the function immediately returns `Err(E)` (requires the function's return type to be compatible).
  - Custom Error Enums (e.g., `ConfigError`): Defining specific error types for better context.
  - Implementing `std::error::Error` and `std::fmt::Display` for custom errors enables integration with Rust's error handling ecosystem.
  - `eprintln!`: Prints messages to the standard error stream (`stderr`), typically used for error logging.
  - `std::process::exit(1)`: Terminates the program immediately with a non-zero exit code, indicating failure.
- **Status:** Used extensively in `config.rs` and `main.rs` for loading config and creating the DB pool.

### 18. Asynchronous Programming (`async`/`await`)
- **Concept:** Handling operations (like I/O, network requests) that might take time without blocking the main thread.
- **Keywords:**
  - `async fn`: Declares a function as asynchronous. It returns a `Future`.
  - `.await`: Pauses the execution of the `async fn` until the awaited `Future` completes.
- **Runtime:** Requires an async runtime (like Tokio, automatically managed by `#[actix_web::main]`) to execute `Future`s.
- **Status:** Used for `main`, handlers (`hello`, `get_municipalities`), and `db::create_pool`.

### 19. Database Connection Pooling (`sqlx`)
- **Concept:** Managing a pool of reusable database connections for efficiency and performance.
- **Crate:** `sqlx` (specifically `sqlx::postgres`).
- **Components:**
  - `PgPoolOptions`: Used to configure pool settings (max connections, timeouts).
  - `PgPool`: The connection pool itself. It's cloneable (`Arc`-based) and thread-safe.
  - `.connect(&db_url).await`: Establishes the connections and creates the pool.
  - `sqlx::query("...").fetch_one(&pool).await`: Example of executing a query using a connection from the pool.
- **Status:** Implemented in `db/mod.rs` and used in `main.rs`.

### 20. Actix Web Application State (`web::Data`)
- **Concept:** Sharing data (like a database pool or configuration) across handlers within an Actix application.
- **Mechanism:**
  - `web::Data::new(shared_data)`: Wraps the data to be shared.
  - `.app_data(web_data)`: Registers the wrapped data with the `App`.
  - Handler argument `pool: web::Data<DbPool>`: Actix injects the shared data into the handler.
  - `pool.get_ref()`: Accesses the inner data (`&DbPool`) from the `web::Data` wrapper.
- **Status:** Used in `main.rs` to share the `DbPool`.

### 21. Type Aliases (`type`)
- **Concept:** Creating a new name (alias) for an existing type.
- **Syntax:** `type NewName = ExistingType;` (e.g., `type DbPool = PgPool;`)
- **Use Case:** Improving readability, reducing verbosity.
- **Status:** Used in `db/mod.rs`.

### 22. Module Organization, Privacy, and Encapsulation (Unlocked)

*   **Concept:** Use Rust’s module system to organize code, control visibility with `pub`, `pub(crate)`, and keep struct fields private unless necessary.
*   **Encapsulation:** Expose internal data safely using getter methods rather than making fields public.
*   **Example:**
    ```rust
    pub struct MyStruct {
        field: String,
    }
    impl MyStruct {
        pub fn field(&self) -> &str { &self.field }
    }
    ```
*   **Best Practices:**
    - Only make things `pub` if you must.
    - Use modules to keep code organized and readable.
    - Separate business logic from API/networking logic.
    - Use getter methods to allow read-only access to internal state, preserving flexibility and safety.

---

#### Deepened Skills

- **Async Networking:**  
  All HTTP/network code is now async, using `.await` and propagating errors with `?`.

- **Error Handling:**  
  Consistently use custom error enums and `Result<T, E>` for robust error propagation.

- **Logging:**  
  Use the `log` crate (`log::info!`, `log::debug!`, `log::trace!`) for structured, contextual logging.

- **Testing & Cargo Workflows:**  
  Regularly run `cargo check` and `cargo test` to verify correctness after changes.

---

#### Summary Table: Idiomatic Rust in Your Project

| Principle        | How You Did It                            |
|------------------|-------------------------------------------|
| Encapsulation    | Private fields + public getters           |
| Modularity       | Separate modules per concern              |
| Async            | All network code is async                 |
| Error Handling   | Custom error types, `Result`, `?`         |
| Logging          | `log::info!`, `log::debug!`, etc.         |
| Minimal Exposure | Only what’s needed is `pub`               |

### 23. Asynchronous Programming

*   **`async`/`.await`:** Writing asynchronous functions (`async fn`) and waiting for `Future`s to complete (`.await`).
    *   *Application:* Used for Actix web handlers, `sqlx` database calls, and `reqwest` HTTP requests.
*   **Async Runtimes:** Understanding the need for a runtime (Tokio).
    *   *Application:* Used `#[actix_web::main]` for the main application entry point and `#[tokio::test]` for async integration tests.

### 24. Error Handling

*   **`Result<T, E>`:** Standard way of handling recoverable errors.
*   **`?` Operator:** Propagating errors up the call stack concisely.
    *   *Application:* Used frequently in functions that perform I/O (API calls, DB queries).
*   **Custom Error Types:** Defining specific error enums for different failure modes.
    *   *Application:* Created `ApiClientError` using `thiserror` to represent different API interaction failures (request errors, API errors, parsing errors).
*   **`thiserror` Crate:** Simplifying the creation of custom error types that implement `std::error::Error`.
    *   *Application:* Used `#[derive(Error)]` and `#[error(...)]` attributes.

### 25. External Crates & Ecosystem

*   **`Cargo.toml`:** Managing dependencies.
*   **`actix-web`:** Web framework for building the API.
*   **`sqlx`:** Interacting with the PostgreSQL database asynchronously with compile-time checks.
*   **`reqwest`:** Making HTTP requests to the Municipal Money API.
*   **`serde` (`serde`, `serde_json`):** Deserializing JSON responses from the API into Rust structs (`#[derive(Deserialize)]`).
*   **`dotenvy`:** Loading configuration from `.env` files.
*   **`thiserror`:** Creating custom error types.
*   **`log` / `env_logger`:** Logging application events and errors (setup pending).
*   **`tokio`:** Async runtime.

### 26. Testing

*   **Unit Tests:** Writing tests within modules (`#[cfg(test)]`).
*   **Integration Tests:** Placing tests in the `tests/` directory.
    *   *Application:* Created `tests/muni_money_integration_test.rs`.
*   **Test Attributes:** `#[test]`, `#[tokio::test]` (for async tests).
*   **Ignoring Tests:** Using `#[ignore]` for tests that shouldn't run by default (e.g., network-dependent tests).
    *   *Application:* Marked API integration tests with `#[ignore]`.
*   **Running Ignored Tests:** `cargo test -- --ignored`.
*   **Assertions:** `assert!`, `assert_eq!`, `assert!(result.is_ok())`, etc.

### 27. Configuration Management

*   **Environment Variables:** Reading configuration settings (like API keys, database URLs) from the environment.
*   **`.env` Files:** Using `dotenvy` to load variables from a file during development/testing.
    *   *Application:* Setup `dotenvy::dotenv().ok()` in `main.rs` and tests.

### 28. API Client Development

*   **Structuring Client Code:** Separating concerns (client logic, request functions, type definitions, error handling).
    *   *Application:* Used `client.rs`, `financials.rs`, `types.rs` within `src/api/muni_money/`.
*   **Making Requests:** Using `reqwest::Client` to send GET requests.
*   **Handling Responses:** Checking status codes, parsing JSON bodies (`response.json().await?`).
*   **Error Handling:** Mapping `reqwest` errors and API-level errors (e.g., non-2xx status codes) to a custom error type (`ApiClientError`).
*   **Debugging:** Using `println!` or `log` macros to inspect request URLs, parameters, and responses during development/testing.
    *   *Application:* Added print statements in tests to see API call results and diagnose failures (like the timeout issue).

### 29. Project Structure & Modules

*   **Crates:** Understanding the difference between binary (`main.rs`) and library (`lib.rs`) crates.
    *   *Application:* Structured `seemycity-backend` as both a library and a binary to allow integration tests to easily access application code.
*   **Modules:** Organizing code using `mod` and `pub mod`.
    *   *Application:* Created modules like `api`, `db`, `handlers`, `models`, `config`, `errors`.
*   **Paths & `use`:** Importing items from other modules/crates (`use crate::...`, `use external_crate::...`).
    *   *Application:* Extensively used to bring functions, structs, and traits into scope. Resolved import issues when refactoring to lib/binary structure.
*   **`main.rs` vs `lib.rs`:** Understanding their roles as entry points for binary execution and library definition, respectively.

### 30. Asynchronous Programming

*   **`async`/`.await`:** Writing asynchronous functions (`async fn`) and waiting for `Future`s to complete (`.await`).
    *   *Application:* Used for Actix web handlers, `sqlx` database calls, and `reqwest` HTTP requests.
*   **Async Runtimes:** Understanding the need for a runtime (Tokio).
    *   *Application:* Used `#[actix_web::main]` for the main application entry point and `#[tokio::test]` for async integration tests.
