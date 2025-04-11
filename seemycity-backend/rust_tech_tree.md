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
