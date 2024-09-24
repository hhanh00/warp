# Zcash Warp

## Build

::: info
Skip this section if you have already cloned and built the `zcash-warp`
library and CLI.
:::

```
$ git clone https://github.com/hhanh00/warp.git
$ cd warp
$ git submodule update --init
$ cargo b -r
$ cd zcash-warp
```

This builds `zcash-warp` in `../target/release`.

## Configuration

In the `zcash-warp` directory, create a `App.toml` file
with the following content:

```
db_path="zec.db"
lwd_url="http://127.0.0.1:9168"
warp_url="http://127.0.0.1:8000"
warp_end_height=0
confirmations=1
regtest=true
```

- `lwd_url` is the URL to the Lightwalletd server. If you plan to run
it on a different machine, change the IP address accordingly.
- `warp_url` is the URL of the WARP 2 server. We are not going to use
it on this tutorial since we are running on a tiny blockchain. It
is fine to leave the URL as it is.
- A `warp_end_height` of 0 disables WARP 2.
- `confirmations` is set to 1 so that we don't have to wait for more
than 1 confirmation before we can spend the funds we receive.
- Finally, `regtest=true` enables regtest mode.

::: tip
If you set regtest to false, `zcash-warp` connects to the mainnet chain.
:::

Now, run `zcash-warp`. If everything is fine, it replies with its
command prompt.

```
zcash-warp〉
```

Try entering `help` to get a list of available commands.

Next, we will create a new account and get its addresses.
