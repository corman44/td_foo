build-wasm:
    cargo build --release --target wasm32-unknown-unknown
    wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "td_foo" ./target/wasm32-unknown-unknown/release/td_foo.wasm

debug:
    RUST_LOG=debug cargo run

info:
    RUST_LOG=info cargo run

error:
    RUST_LOG=error cargo run

dewatch: 
    cargo watch -x "check" -s "just debug"

inwatch:
    cargo watch -x "check" -s "just info"

tree:
    cargo tree > tree.txt

watch:
    cargo watch -x "check"

work:
    cargo watch -x "check" -s "just error"