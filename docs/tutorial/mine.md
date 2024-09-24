# Mining

You can mine/generate new blocks using the zcash-cli tool.

```
$ ./src/zcash-cli --datadir=regtest_dir generate 101
```

This generates 101 blocks and our wallet in `zcashd` receives
all the mining reward.

However, it takes 100 blocks before a mining reward matures
and can be spent. Therefore, we only have one block worth
of reward (6.25 ZEC) available.

```
$ ./src/zcash-cli --datadir=regtest_dir getwalletinfo
{
  "walletversion": 60000,
  "balance": 6.25000000,                      <---------- Available
  "unconfirmed_balance": 0.00000000,
  "immature_balance": 625.00000000,           <---------- Mined but not yet spendable
  "shielded_balance": "0.00",
  "shielded_unconfirmed_balance": "0.00",
  "txcount": 101,
  "keypoololdest": 1727010648,
  "keypoolsize": 101,
  "paytxfee": 0.00000000,
  "mnemonic_seedfp": "22a8545b10f7092215b7481776093c913f701566b5aefe607481007e160a44dc"
}
```

It's more than enough for this tutorial. Next, we must shield these funds
by sending them to Sapling or Orchard address. 

::: info
Coinbase rewards cannot
be sent to a transparent address.
:::

We are going to send these funds to the Zcash-warp wallet. But first,
we need to set up the `lightwalletd` server.
