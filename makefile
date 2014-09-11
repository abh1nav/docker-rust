all: clean compile

clean:
	rm -Rf target

compile:
	cargo build

test:
	cargo test
