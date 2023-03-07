@all:
	cargo run -- examples/samples/main.cpp -c examples/samples/main.sgr

sgr:
	vim examples/samples/main.sgr

main:
	vim src/main.rs
