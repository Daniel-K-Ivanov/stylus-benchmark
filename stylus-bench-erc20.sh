
#!/bin/bash

# Define global variables
RPC="http://localhost:8547"
ERC20_ADDRESS="0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0"
USER="0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266"
USER_PK="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
USER2="0x70997970c51812dc3a010c7d01b50e0d17dc79c8"
USER2_PK="0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d"

# Acquire funds using the "gibFunds" function
echo "Acquiring funds..."
cast send --rpc-url $RPC --private-key $USER_PK $ERC20_ADDRESS "fund(uint256)" 1000000 --gas-limit 1000000
echo "Acquiring funds 2..."
cast send --rpc-url $RPC --private-key $USER_PK $ERC20_ADDRESS "fund(uint256)" 1 --gas-limit 1000000

# Transfer funds to another address
echo "Transferring funds..."
cast send --rpc-url $RPC --private-key $USER_PK $ERC20_ADDRESS "transfer(address,uint256)" $USER2 1
echo "Transferring funds 2..."
cast send --rpc-url $RPC --private-key $USER_PK $ERC20_ADDRESS "transfer(address,uint256)" $USER2 1

# Approve and transfer funds from
echo "Approving and transferring funds..."
cast send --rpc-url $RPC --private-key $USER_PK $ERC20_ADDRESS "approve(address,uint256)" $USER2 1000
echo "Allowance is"
cast call --rpc-url $RPC $ERC20_ADDRESS "allowance(address,address)" $USER $USER2
echo "Balance is"
cast call --rpc-url $RPC $ERC20_ADDRESS "balanceOf(address)" $USER
echo "Transfer from"
cast send --rpc-url $RPC --private-key $USER2_PK $ERC20_ADDRESS "transferFrom(address,address,uint256)" $USER $USER2 10