# Solana-account-initializer-program

## Build th program as BPF format
```cargo build-bpf --manifest-path=./solana_test_lib/Cargo.toml --bpf-out-dir=dist/program```

## Connect to Solana dev/test/main network
```solana config set --url https://DEPLOY_NETWORK_HOST_NAME.com```

## Deploy to Solana network
```solana program deploy dist/program/solana_test_lib.so```