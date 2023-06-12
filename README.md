# Elrond-staking-contract

staking Contract
Elrond blockchain - MultiversX network

Contract address: erd1qqqqqqqqqqqqqpgqtpz4yht3ern76l8333h22rk95dj7pdk2f9dqpl3qjs

Explorer: https://devnet-explorer.multiversx.com/accounts/erd1qqqqqqqqqqqqqpgqtpz4yht3ern76l8333h22rk95dj7pdk2f9dqpl3qjs

## Interact with the smart contract

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
    --function="getStakingPosition" \
    --arguments erd1duy4zc20pdy666chknywet227n9yjmc6uvfgjcy5yvcxncddf9dqxqwzjl`

- Call the unstake function

`mxpy --verbose contract call erd1qqqqqqqqqqqqqpgqtpz4yht3ern76l8333h22rk95dj7pdk2f9dqpl3qjs \
    --proxy=https://devnet-gateway.multiversx.com --chain=D \
    --send --recall-nonce --pem=/home/gianm/rust-web3/staking-contract/walletKey.pem \
    --gas-limit=10000000 \
    --function="unstake" \
    --arguments 500000`

## Test locally

`cargo test --test rust_test`

    mxpy --verbose contract deploy --bytecode=/home/gianm/rust-web3/staking-contract/output/staking-contract.wasm \
    --recall-nonce --pem=/home/gianm/rust-web3/staking-contract/walletKey.pem \
    --gas-limit=30000000 \
    --send --outfile="deploy-devnet.interaction.json" --wait-result \
    --proxy=https://devnet-gateway.multiversx.com --chain=D \
    --arguments 30000

- Call the stake function

`mxpy --verbose contract call erd1qqqqqqqqqqqqqpgq2fzx05z7g6cecerxya7ushed73rq7mhhf9dqzq59zc \
    --proxy=https://devnet-gateway.multiversx.com --chain=D \
    --send --recall-nonce --pem=/home/gianm/rust-web3/staking-contract/walletKey.pem \
    --gas-limit=20000000 \
    --value=100000000000000000 \
    --function="stake"`

mxpy --verbose contract call erd1qqqqqqqqqqqqqpgq2fzx05z7g6cecerxya7ushed73rq7mhhf9dqzq59zc \
 --proxy=https://devnet-gateway.multiversx.com --chain=D \
 --send --recall-nonce --pem=/home/gianm/rust-web3/staking-contract/walletKey.pem \
 --gas-limit=20000000 \
 --function="unstake" \
 --arguments 100000000000000

- Call the stake function

`mxpy --verbose contract call erd1qqqqqqqqqqqqqpgqavrxjkxutzujk993nps2alazuklm5du4f9dqv4m00u \
    --proxy=https://devnet-gateway.multiversx.com --chain=D \
    --send --recall-nonce --pem=/home/gianm/rust-web3/staking-contract/walletKey.pem \
    --gas-limit=20000000 \
    --value=1000000000000000000 \
    --function="stake"`

mxpy --verbose contract call erd1qqqqqqqqqqqqqpgqavrxjkxutzujk993nps2alazuklm5du4f9dqv4m00u \
 --proxy=https://devnet-gateway.multiversx.com --chain=D \
 --send --recall-nonce --pem=/home/gianm/rust-web3/staking-contract/walletKey.pem \
 --gas-limit=20000000 \
 --function="unstake" \
 --arguments 10000000000000000

     mxpy --verbose contract query erd1qqqqqqqqqqqqqpgqavrxjkxutzujk993nps2alazuklm5du4f9dqv4m00u \
    --proxy=https://devnet-gateway.multiversx.com \
    --function="getStakingPosition" \
    --arguments erd1duy4zc20pdy666chknywet227n9yjmc6uvfgjcy5yvcxncddf9dqxqwzjl

    mxpy --verbose contract upgrade erd1qqqqqqqqqqqqqpgqavrxjkxutzujk993nps2alazuklm5du4f9dqv4m00u \
    --bytecode=/home/gianm/rust-web3/staking-contract/output/staking-contract.wasm \
    --recall-nonce --pem=/home/gianm/rust-web3/staking-contract/walletKey.pem \
    --gas-limit=30000000 \
    --send --outfile="upgrade-devnet.interaction.json" \
    --proxy=https://devnet-gateway.multiversx.com --chain=D \
    --arguments 300
