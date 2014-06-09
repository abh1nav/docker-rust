all: compile run

compile:
	rustc main.rs

run:
	./main