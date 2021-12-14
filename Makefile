check:
	@sudo wondershaper
build: src/main.rs
	@cargo build --release
run: src/main.rs
	@cargo run --release
bin: target/debug/bandwidth-saving
	@target/release/bandwidth-saving
