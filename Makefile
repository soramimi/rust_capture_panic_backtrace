
build:
	cargo +nightly build

run:
	cargo +nightly run 2>/dev/null

rustup:
	rustup install nightly

clean:
	rm -f Cargo.lock
	rm -fr target
