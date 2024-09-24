# Synchronization

Back to `zcash-warp`, let's test the synchronization
of our wallet.

First, we reset the wallet to the birth height or 
to a given height. This downloads initial data from
the LWD server and initializes the wallet.

```
zcash-warp〉reset                                         09/23/2024 02:31:02 PM
zcash-warp〉                                              09/23/2024 02:31:02 PM
```

Next, we synchronize it with the latest block.

```
zcash-warp〉sync                                          09/23/2024 02:31:29 PM
zcash-warp〉                                              09/23/2024 02:31:32 PM
```

Let's display the account balance.

::: tip
Most commands take the account number as the first argument.
:::

```
zcash-warp〉balance 1                                     09/23/2024 02:34:21 PM
Balance: BalanceT { transparent: 0, sapling: 0, orchard: 624985000 }
```

We have now 6.25 ZEC (minus a small amount of transaction fees).

::: warning
`zcash-warp` displays all amounts in ZATS. There are 100000000 (one hundred
million zats per zec).
:::

As expected, `zcashd` sent the coinbase to the Orchard receiver since we
provided a UA that has every receiver types.

With the funds secured in our wallet, we will not need to use `zcashd`
wallet functionalities any further.

Next, let's make some simple transactions to get familiar with its
usage.
