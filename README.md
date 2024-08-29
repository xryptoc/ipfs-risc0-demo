## demo

install & run - `Local`

1. install risc0 toolchain

```
cargo install cargo-binstall
cargo binstall cargo-risczero
cargo risczero install
```

2. start ipfs node

```
./start_ipfs.sh
```

3. upload file to ipfs by ipfs-ui ```http://127.0.0.1:8089```

4. ```cp template.local.env .env```

5. gen proof

> proof receip will save to ```receip.bin```

```
cargo run --bin prove <ipfs_cid> <start> <end>
```

6. verify

```
cargo run --bin verify
```