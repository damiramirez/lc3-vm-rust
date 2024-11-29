.PHONY run test:

FILENAME   ?= ./examples/2048.obj

run:
	cargo run $(FILENAME)

test:
	cargo test
