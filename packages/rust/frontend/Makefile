all: typescript ios-sim android
ios:
	cargo build --target=x86_64-apple-ios --release # x86 iOS simulator
	cargo build --target=aarch64-apple-ios-sim --release # arm iOS simulator
	cargo build --target=aarch64-apple-ios --release # arm iOS device
ios-sim:
	cargo build --target=aarch64-apple-ios-sim --release # arm iOS simulator
	cp ../target/aarch64-apple-ios-sim/release/libfrontend.a ../../js/bubble_rust/rust
android:
	cargo ndk -t armeabi-v7a -t arm64-v8a -o ../../js/bubble_rust/android/src/main/jniLibs build --release
typescript:
	cargo run --package bridge_gen --bin bridge_gen -- ./src ../../js/bubble_rust/src/gen.ts
android-docker:
	rm -rf ../../js/bubble_rust/android/src/main/jniLibs
	mkdir -p ../../js/bubble_rust/android/src/main/jniLibs
	docker build --platform linux/amd64 -t rust-android .
	docker run --platform linux/amd64 --rm -v $(shell pwd)/../:/home/rust -v $(shell pwd)/../../js/bubble_rust/android/src/main/jniLibs:/home/out -it rust-android \
    "cargo ndk --platform 23 -t armeabi-v7a -t arm64-v8a -o ../out build --release -v"

