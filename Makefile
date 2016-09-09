all: build

build: setup
	cargo build

setup:
	wget -N -P data http://norvig.com/big.txt

test: setup
	cargo test


.PHONY: all build setup test
