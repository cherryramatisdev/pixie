install:
	rm -rf target
	rm -rf ~/.local/bin/pixie
	cargo build --release
	mv ./target/release/pixie ~/.local/bin
	pixie --help
