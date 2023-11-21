# Stylus Benchmarks

This code repository aims to benchmark the gas costs between EVM and Stylus contracts for ERC20 tokens.

## Findings

ERC20 Functions
| Function     | Stylus (WASM) | Solidity (EVM) |
|--------------|---------------|----------------|
| fund/mint    |  36345        | TBD    |
| transfer     |  37417        | TBD    |
| approve      |  31573        | TBD    |
| transferFrom |  44683        | TBD    |

Merkle Inclusion Proofs
| Depth     | Stylus (WASM) | Solidity (EVM) |
|-----------|---------------|----------------|
| 10        |  TBD        | TBD    |
| 100       |  TBD        | TBD    |
| 1000      |  TBD        | TBD    |



## Reproducing the benchmarks

1. Compilte the Stylus contracts
```shell
cargo build --package stylus-benchmark --release
```

2. Clone Arbitrum Nova development node
```shell
git clone -b stylus --recurse-submodules https://github.com/OffchainLabs/nitro-testnode.git && cd nitro-testnode
```

3. Run Local development node with Stylus support 
```shell
./test-node.bash --init
```

4. Fund the users addresses

```shell
./test-node.bash script send-l2 --to address_0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266 --ethamount 100
```
```shell
./test-node.bash script send-l2 --to address_0x70997970c51812dc3a010c7d01b50e0d17dc79c8 --ethamount 100
```

5. Deploy the contracts
```shell
cargo stylus deploy -e http://localhost:8547 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
```

6. Run benches
```shell
bash ./stylus-bench.sh
```