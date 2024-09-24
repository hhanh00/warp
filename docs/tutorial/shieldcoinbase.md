# Shield Coinbase Rewards

Using the wallet address we obtained from the [`zcash-warp` wallet]
(/tutorial/warp) we shield our mining rewards as follows:

> Don't forget to replace the address below with your actual wallet address.

```
$ ./src/zcash-cli --datadir=regtest_dir z_shieldcoinbase '*' uregtest1scpvtn0leqj97hfux6k8205ev5amfgq0nfxke4dwgvxyf26h7cywazrn7xga8x2y85xpqvuug4c2fhh3ehv8ul0na3vgksl8xcuwuufahz732usy4qfk84algzmq20ayl578k60rdhp956qvzr9vhdsfzc6kv93uktrvj4d6l7azp339xpvy22fk8cr2weqcrkjllymp2saygxfrans
```

After you execute this command, `zcashd` started processing the request
but this is performed asynchronously, ie the command returned before
the transaction is created.

You can check the status of the command with the following command

```
$ ./src/zcash-cli --datadir=regtest_dir z_getoperationresult
[
  {
    "id": "opid-af2dbe69-f437-4385-8bc7-485d648ae3d7",
    "status": "success",               <-------- SUCCESS 
    "creation_time": 1727056601,
    "result": {
      "txid": "c1ba0a773542bac38c5c0622b3596539ac33180f2f1e9a00eacdc04c8c4d0540"
    },
    "execution_secs": 2.791093,
    "method": "z_shieldcoinbase",
    "params": {
      "fromaddress": "*",
      "toaddress": "uregtest1scpvtn0leqj97hfux6k8205ev5amfgq0nfxke4dwgvxyf26h7cywazrn7xga8x2y85xpqvuug4c2fhh3ehv8ul0na3vgksl8xcuwuufahz732usy4qfk84algzmq20ayl578k60rdhp956qvzr9vhdsfzc6kv93uktrvj4d6l7azp339xpvy22fk8cr2weqcrkjllymp2saygxfrans"
    }
  }
]
```

Finally, the transaction must be included in a new block.

```
$ ./src/zcash-cli --datadir=regtest_dir generate 1
```

Now, we should have our new minted coins in the `zcash-warp` wallet.

Next, we will synchronize `zcash-warp` and check our new balance.
