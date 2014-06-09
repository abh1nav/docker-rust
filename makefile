all: compile

compile:
	rustc docker.rs

test:
	rustc --test docker.rs
	./docker

run:
	./docker