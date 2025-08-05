# CRUSH Development Guidelines

## Build Commands
- Build: `cargo build`
- Build (release): `cargo build --release`
- Run: `cargo run`
- Run with args: `cargo run -- [args]`

## Test Commands
- Run all tests: `cargo test`
- Run specific test: `cargo test <test_name>`
- Run tests with output: `cargo test -- --nocapture`
- Run tests in specific file: `cargo test --lib <module_name>`

## Lint/Format Commands
- Format code: `cargo fmt`
- Check formatting: `cargo fmt -- --check`
- Lint: `cargo clippy`
- Lint with fixes: `cargo clippy --fix`
- Check without building: `cargo check`

## Code Style Guidelines
- Use 2 spaces for indentation (no tabs)
- Follow Rust naming conventions: snake_case for variables/functions, PascalCase for types
- Prefer explicit typing in public APIs
- Use Rust error handling patterns with Result<T, E>
- Limit line length to 100 characters
- Group imports: standard library, external crates, local modules
- Use `pub(crate)` for module-internal public items
- Write documentation comments for public APIs

## Project Structure
- `src/` - Main source code
- `src/editor/` - Editor implementation
- `test/` - Test files