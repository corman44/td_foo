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

work:
    cargo watch -x "check" -s "just error"