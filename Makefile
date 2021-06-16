build-Hello:
	cargo build --bin hello --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/hello $(ARTIFACTS_DIR)/bootstrap

build-Goodbye:
	cargo build --bin good_bye --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/good_bye $(ARTIFACTS_DIR)/bootstrap