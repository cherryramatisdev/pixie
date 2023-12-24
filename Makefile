install:
	rm -rf target
	rm -rf ~/.local/bin/pixie
	cargo build --release
	mv ./target/release/cli ~/.local/bin/pixie
	pixie --help
