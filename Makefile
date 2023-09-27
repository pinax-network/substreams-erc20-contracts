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
	substreams run graph_out -e eth.substreams.pinax.network:9000 -s 1000000

.PHONY: gui
gui:
	substreams gui graph_out -e eth.substreams.pinax.network:9000 

.PHONE: deploy_local
deploy_local: pack
	graph codegen
	graph build --ipfs http://localhost:5001 subgraph.yaml
	graph create --node http://127.0.0.1:8020 erc20_contract_info
	graph deploy --node http://127.0.0.1:8020 --ipfs http://127.0.0.1:5001 --version-label v0.0.1 erc20_contract_info subgraph.yaml