# Rust Learning Guide (for Go/TS Developers)

## 🤖 AI Context Prompt
*Paste this into a new chat to set the context for your learning session:*

> "I am an experienced Go and TypeScript developer learning Rust. I do not want to learn from scratch with 'Hello World'. I want to learn by **building a real project** (like an Axum web server).
>
> **My Learning Style:**
> 1. **No Textbook Theory:** Explain concepts using Go/TS analogies (e.g., "Structs are like Go structs", "Traits are like TS Interfaces").
> 2. **Step-by-Step Refactoring:** Start with a messy `main.rs` and guide me through refactoring it into idiomatic Rust (Modules, Error Handling, State Management).
> 3. **One-Liners:** Give me concise syntax cheatsheets.
>
> **Current Goal:** Build a file upload/download server using `axum`, `tokio`, and `serde`."

---

## 🗺️ The "Learn by Building" Roadmap

### Phase 1: The Basics (Syntax & Data)
*Goal: Get a server running and return JSON.*
- [x] **Hello World:** Set up `cargo new`, add `axum` dependencies.
- [x] **Routing:** Define `GET` and `POST` routes.
- [x] **Structs & JSON:** Use `serde` to serialize structs into JSON (Go `struct` tags equivalent).
- [x] **Modules:** Move logic to `handlers.rs` (Go `package` equivalent).

### Phase 2: State Management (The "Global Variable" Fix)
*Goal: Remove hardcoded constants and share database/config connections.*
- [ ] **Command Line Args:** Use `clap` to parse flags (like Go's `flag` package).
- [ ] **Shared State:** Use `Arc<AppState>` to share config across threads safely.
- [ ] **Dependency Injection:** Extract state in handlers using `State<T>`.

### Phase 3: Robustness (Error Handling)
*Goal: Stop using `.unwrap()` and handle errors gracefully.*
- [ ] **Custom Error Type:** Create an `enum AppError` that implements `IntoResponse`.
- [ ] **The `?` Operator:** Refactor code to propagate errors instead of panicking.
- [ ] **Result Type:** Understand `Result<T, E>` deeply.

### Phase 4: Database & Async
*Goal: Persist data.*
- [ ] **Async Runtime:** Understand how `tokio` works (Event Loop).
- [ ] **Database:** Connect `sqlx` (SQLite/Postgres) to store file metadata.

### Side Quest: Fetching Data (Client)
*Goal: Make an HTTP request to an external API.*
- [x] **Dependencies:** Add `reqwest` (Client) and `tokio` (Runtime).
- [x] **Async/Await:** Use `#[tokio::main]` and `.await`.
- [x] **Deserialization:** Parse JSON response into a struct.

---

## ⚡ Rust Cheatsheet (Go/TS Edition)

### Variables
```rust
let x = 5;          // const x = 5; (TS)
let mut y = 10;     // let y = 10; (TS) / var y = 10 (Go)
```

### Async & Await
```rust
// JS: async function foo() { await bar(); }
// Rust:
async fn foo() {
    bar().await; // .await is a keyword at the END
}
```

### Structs (Data Shapes)
```rust
// Go: type User struct { ... }
struct User {
    username: String,
    active: bool,
}
```

### Option & Result (No Nulls!)
```rust
// Go: if err != nil { return err }
// Rust:
let f = File::open("foo.txt")?; // Returns error automatically if it fails

// TS: const x = val || default;
// Rust:
let x = option_val.unwrap_or(default_val);
```

### Memory (Ownership)
- **Move:** `let a = b;` (If `b` is complex, `a` now owns the data. `b` is dead.)
- **Borrow (Read):** `&b` (Like a pointer `*b` in Go, but read-only).
- **Borrow (Write):** `&mut b` (Exclusive pointer, only one allowed).

### Modules
- **File System:** `src/handlers.rs` does NOT exist until you add `mod handlers;` in `src/main.rs`.
- **Visibility:** Everything is private. Use `pub` to export (like Capitalizing in Go).
