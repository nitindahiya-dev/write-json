<h2># JSON Serialization in Rust</h2>

<p>This repository contains a simple example of JSON serialization and deserialization in Rust using the `serde` crate. The example defines an `Article` struct with nested `Paragraph` structs, serializes an instance of `Article` to a JSON string, and prints the result.</p>

<p>## Dependencies</p>
<br>
<li> [serde](https://crates.io/crates/serde) - A framework for serializing and deserializing Rust data structures efficiently and generically.</li>
<li> [serde_json](https://crates.io/crates/serde_json) - A JSON serialization and deserialization library for Rust.</li>
<br>
<p>To add these dependencies to your project, include the following in your `Cargo.toml` file:</p>

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
