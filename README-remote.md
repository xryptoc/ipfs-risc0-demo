## demo

install & run - `Remote`

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

4.  ```cp template.remote.env .env``` and fill in your ```BONSAI_API_KEY```

5. run server

```
cargo run
```

6. gen proof

```shell
curl --location --request POST 'http://127.0.0.1:3001/generateproof' \
--header 'Content-Type: application/json' \
--data-raw '{
    "hash":"<your ipfs cid>",
    "start": <start-number>,
    "end": <end-number>
}'
```