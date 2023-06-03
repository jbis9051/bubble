ios:
	cargo build --target=x86_64-apple-ios --release # x86 iOS simulator
	cargo build --target=aarch64-apple-ios-sim --release # arm iOS simulator
	cargo build --target=aarch64-apple-ios --release # arm iOS device
ios-sim:
	cargo build --target=aarch64-apple-ios-sim --release # arm iOS simulator
	cp ../target/aarch64-apple-ios-sim/release/libfrontend.a ../../js/bubble_rust/rust