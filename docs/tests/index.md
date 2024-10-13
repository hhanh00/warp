# Tests

## Setup

### Zcashd

`zcash.conf`
```toml
regtest=1
rpcuser=a
rpcpassword=a
nuparams=c2d6d0b4:1
rpcallowip=192.168.0.0/24
experimentalfeatures=1
lightwalletd=1
txindex=1
```

Stop, clean, restart, mine and shield.

```bash
$ ./src/zcash-cli --datadir=regtest_dir stop
$ rm -rf regtest_dir/regtest/
$ ./src/zcashd --datadir=regtest_dir --daemon
$ ./src/zcash-cli --datadir=regtest_dir generate 101
$ ./src/zcash-cli --datadir=regtest_dir z_shieldcoinbase '*' uregtest1qae0mnr0p9jenge23nc47f359kwkyqvq0kl3qfgyqmk5k37jcvqlzafype9r99yxwfhs38q3w0a2qgam489deht2fguv6hg6g5m0c5vk
$ ./src/zcash-cli --datadir=regtest_dir generate 3
```

### Lightwalletd

Cleanup and run

```bash
$ rm -rf lwd-data/
$ ./lightwalletd --no-tls-very-insecure --zcash-conf-path ~/projects/zcash/regtest_dir/zcash.conf --data-dir lwd-data --log-file /dev/stdout --grpc-bind-addr 0.0.0.0:9168
```

### Warp Configuration

`App.toml`
```toml
db_path="zec.db"
lwd_url="http://desk:9168"
warp_url="http://127.0.0.1:8000"
warp_end_height=0
confirmations=1
regtest=true
```
