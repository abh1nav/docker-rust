all: clean compile

clean:
	rm -f ./docker ./libdocker*

compile:
	rustc lib.rs

test:
	rustc --test lib.rs
	./docker