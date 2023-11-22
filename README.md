# Stylus Benchmarks

This code repository aims to benchmark the gas costs between EVM and Stylus contracts for ERC20 tokens.

## Findings

**ERC20 Functions**

| Function     | Stylus (WASM) | Solidity (Shanghai EVM) |
|--------------|---------------|------------------------|
| fund/mint    |  36345        | 34123                  |
| transfer     |  37417        | 35000                  |
| approve      |  31573        | 27005                  |
| transferFrom |  44683        | 41212                  |

**ED25519 Verification**

|           | Stylus (WASM)    | Solidity (Shanghai EVM) |
|-----------|------------------|-------------------------|
| verify    |  480628          | 996582                  |


Note:
https://github.com/chengwenxi/Ed25519/tree/main was used for the EVM implementation of ED25519 verification

## Reproducing Benchmarks

### Prepare the environment

1. Compile the Stylus contracts
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

#### ERC20

1. Uncomment the `erc20` file in the `lib.rs`, comment the `ed25519` file

2. Deploy the erc20 contract
```shell
cargo stylus deploy -e http://localhost:8547 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
```

3. Run the benches script - change the addresses of the ERC20 prior to launching the script
```shell
bash ./stylus-bench-erc20.sh
```

#### ED25519

1. Uncomment the `ed25519` file in `lib.rs`, comment the `erc20` file
2. Deploy the `ed25519` verifier contract

```shell
cargo stylus deploy -e http://localhost:8547 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
```

4. Execute the following cast
```shell
cast send --rpc-url http://localhost:8547 --gas-limit 1000000 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 {CONTRACT_ADDRESS} "verify(bytes,bytes,bytes32)" 0xb0d8bdfd9f4d1023dae836b2e41da5019d20c60965dc40943e2c10f2ad4ee49ab0d8bdfd9f4d1023dae836b2e41da5019d20c60965dc 0xa6161c95fd4e3237b7dd12cc3052aaa69382510ecb5b89c2fbeb8b6efb78266b81160af2842235a0257fc1d3e968c2c1c9f56f117da3186effcaeda256c38a0d 0x06cf14cfae0ff9fe7fdf773202029a3e8976465c8919f4840d1c3c77c8162435
```