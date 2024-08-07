.PHONY: all
all:
	make build
	make pack
	make graph
	make info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

.PHONY: pack
pack: build
	substreams pack

.PHONY: graph
graph:
	substreams graph

.PHONY: info
info:
	substreams info

.PHONY: run
run:
	substreams run db_out -e eth.substreams.pinax.network:443 -s 913198 --stop-block 915005

.PHONY: gui
gui:
	substreams gui db_out -e eth.substreams.pinax.network:443 -s 913198 --stop-block 915005
