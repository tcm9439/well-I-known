# Rust Note
## Cargo
### run
`cargo run --bin binary-name`
- run the binary in dev mode

### test
Test specific mod:
`cargo test --package well-i-known-server -- db::access_right::tests --nocapture`

### build
`cargo build`
- build in dev profile (debug mode)

`cargo build --release`
- build the release version
- output in /target/release

## types
| Own     | Ref  |
| ------- | ---- |
| String  | &str |
| PathBuf | Path |
| Vec     | &[]  |

## Print
```rust
// print using Display
println!("Var: {}", var)

// print using Debug
println!("Var: {:?}", var)

// format -> produce String
format!("Var: {}", var)
```
