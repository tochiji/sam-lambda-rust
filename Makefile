build-MyFunction:
	cargo lambda build --release --arm64
	cp ./target/lambda/my-function/bootstrap $(ARTIFACTS_DIR)