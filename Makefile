build:
	cargo build

run: build
	./target/debug/advent-of-code-2022 --day $(day)
