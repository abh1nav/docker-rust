all: clean compile

clean:
	rm -Rf build
	mkdir build

compile:
	rustc lib.rs --out-dir=build

test:
	rustc --test lib.rs
	./docker
