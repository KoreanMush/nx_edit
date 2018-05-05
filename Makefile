.PHONY: clippy debug release native windows_native

debug:
	cargo clippy

release:
	cargo build --release && strip ./target/release/nx_edit

native:
	cargo rustc --release -- -C target-cpu=native && strip ./target/release/nx_edit

windows_native:
	cargo rustc --release -- -C target-cpu=native
