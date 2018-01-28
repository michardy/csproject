release:
	cargo build --release
	strip target/release/csproject

install:
	cp target/release/csproject /bin/csproject
