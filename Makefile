debug: target/debug/bandwidthaid
	@target/debug/bandwidthaid
release: src/main.rs
	@cargo build --release
