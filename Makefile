@all:

tests:
	cargo test -r

tsg:
	cargo run -r -- -D tests/graph/tsg/java

prujit:
	cargo run -r -- -W -f java ../arcan-2/test-data/dependency-benchmark-prujit-et-al/src -v
