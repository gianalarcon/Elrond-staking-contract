# Elrond-staking-contract

Elrond blockchain - MultiversX network

Contract address: erd1qqqqqqqqqqqqqpgqtpz4yht3ern76l8333h22rk95dj7pdk2f9dqpl3qjs

Explorer: https://devnet-explorer.multiversx.com/accounts/erd1qqqqqqqqqqqqqpgqtpz4yht3ern76l8333h22rk95dj7pdk2f9dqpl3qjs

## Interact with the smart contract

Assumptions: Before we start to interact with the SC, we assume the SC have enough funds in order to provide rewards to every staker.

Clone the repo and go in the directory

`git clone https://github.com/gianmarcoalarcon/Elrond-staking-contract.git`

- Query the stakers list

`mxpy --verbose contract query erd1qqqqqqqqqqqqqpgqtpz4yht3ern76l8333h22rk95dj7pdk2f9dqpl3qjs \
    --proxy=https://devnet-gateway.multiversx.com \
    --function="getStakedAddresses"`

- Call the stake function

`mxpy --verbose contract call erd1qqqqqqqqqqqqqpgqtpz4yht3ern76l8333h22rk95dj7pdk2f9dqpl3qjs \
    --proxy=https://devnet-gateway.multiversx.com --chain=D \
    --send --recall-nonce --pem={PATH}/walletKey.pem \
    --gas-limit=10000000 \
    --value=1000000000000 \
    --function="stake"`

- Calculate the rewards per user

`mxpy --verbose contract query erd1qqqqqqqqqqqqqpgqtpz4yht3ern76l8333h22rk95dj7pdk2f9dqpl3qjs \
    --proxy=https://devnet-gateway.multiversx.com \
    --function="calculateRewardsForUser" \
    --arguments {UserAddress}`

- Call the unstake function

`mxpy --verbose contract call erd1qqqqqqqqqqqqqpgqtpz4yht3ern76l8333h22rk95dj7pdk2f9dqpl3qjs \
    --proxy=https://devnet-gateway.multiversx.com --chain=D \
    --send --recall-nonce --pem=/home/gianm/rust-web3/staking-contract/walletKey.pem \
    --gas-limit=10000000 \
    --function="unstake" \
    --arguments 500000`

## Test locally

`cargo test --test rust_test`
