# Zcashd

## Configuration

If you have `zcashd` compiled from source, it will be in the
`src` folder of the project root.

We want to run the server in Regtest. In this mode,
the server creates and connects to its own Blockchain
(independent from the main chain). Therefore, you have
full control over when blocks are created and which transactions
are included.

::: info
On regtest, ZEC has **no value**.
:::

1. First create a directory (for example `regtest_dir`) where you will put the configuration and data files
2. In this directory, create a `zcash.conf` file with the following content:
```
regtest=1
rpcuser=a
rpcpassword=a
nuparams=c2d6d0b4:1
rpcallowip=192.168.0.0/24
experimentalfeatures=1
lightwalletd=1
txindex=1
```

- The rpcuser and rpcpassword can be anything you want. Don't forget to activate
regtest otherwise the server will connect to the mainnet.
- `nuparam=c2d6d0b4:1` tells the server that NU-5 (the Orchard Network Upgrade)
should be activated at Block #1.
- `rpcallowip` is the network mask that restricts which clients can connect.
In your local net, it is often `192.168.0.0/24`.
- The last 3 options are for `lightwalletd`

Now start the server with the command:
```
$ ./src/zcashd --datadir=regtest_dir --daemon
```
(assuming you used `regtest_dir`)

::: warning
Do not type the $ sign. It just indicates that this line is *your*
input vs the output of the command.
:::

After a short while, the server should give you back control of the command line.

::: tip
The `regtest_dir` should have a `regtest` sub directory and the `zcash.conf` file.
:::

## Check

We check that the server is running properly by sending a simple command to it.

```
$ ./src/zcash-cli --datadir=regtest_dir getinfo
```

It should return something like:
```
{
  "version": 5080050,
  "build": "v5.8.0",
  "subversion": "/MagicBean:5.8.0/",
  "protocolversion": 170100,
  "walletversion": 60000,
  "balance": 0.00000000,
  "blocks": 0,                       <-------- Check this
  "timeoffset": 0,
  "connections": 0,
  "proxy": "",
  "difficulty": 1,
  "testnet": false,
  "keypoololdest": 1727010648,
  "keypoolsize": 101,
  "paytxfee": 0.00000000,
  "relayfee": 0.00000100,
  "errors": "",
  "errorstimestamp": 1727010650
}
```

A block height of 0 indicates that we are running on our blockchain
since we are still at the genesis.

Next, we'll setup our Zcash Warp wallet.
