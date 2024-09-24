# Setup Lightwalletd

Actually, Lightwalletd (LWD) does not require much configuration.
Everything can be specified on the command line.

```
$ ./lightwalletd --no-tls-very-insecure --zcash-conf-path ./regtest_dir/zcash.conf --data-dir lwd-data --log-file /dev/stdout --grpc-bind-addr 0.0.0.0:9168
```

- `no-tls-very-insecure` indicates that we are not using SSL, i.e encrypted
HTTP
- `zcash-conf-path` must point to the same `zcashd` configuration file
- `data-dir` gives the path to the LWD data directory
- `log-file` is self-explanatory
- `grpc-bind-addr` is where LWD should listen. Here it on port **9168**
This must match the configuration of `zcash-warp` LWD_URL.

LWD should quickly import the first 101 blocks and wait for block #102.

```
{"app":"lightwalletd","level":"info","msg":"Waiting for block: 102","time":"2024-09-23T08:58:25+08:00"}
```
