check:
	@sudo wondershaper
build: src/main.rs
	@cargo build --release
run: src/main.rs
	@cargo run --release
bin: target/release/bandwidthaid
	@target/release/bandwidthaid
