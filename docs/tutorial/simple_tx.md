# Basic Payment Transaction

We use the command `pay` to make a simple payment.

```
zcash-warp〉help pay                                      09/23/2024 02:43:11 PM
Usage: pay <ACCOUNT> <ADDRESS> <TO_POOLS> <AMOUNT> <FROM_POOLS> <FEE_PAID_BY_SENDER> <USE_CHANGE>

Arguments:
  <ACCOUNT>
  <ADDRESS>
  <TO_POOLS>
  <AMOUNT>
  <FROM_POOLS>
  <FEE_PAID_BY_SENDER>
  <USE_CHANGE>

Options:
  -h, --help  Print help
```

- `account`: Account number where to take the funds from
- `address`: Address where we send the funds to
- `to_pools`: Pool mask applied on the previous address that
restricts the receivers. For example, if you specify a UA that
has T+S+O and use a `to_pools` of 1, the transaction will send
to the transparent receiver[^1].
- `amount`: The amount in ZATS
- from_pools: Similar to `to_pools` but indicates the pools
to take funds from.
- `fee_paid_by_sender`: 1 or 0. A value of 1 means that we
pay the transaction fee. A value 0 means the recipient pays.
In the later case, the amount received will be less than specified.
- `use_change`: 1 or 0. 1 means that the transaction included a
change output. Most of the time, we should use a change
output otherwise the transaction cannot be built.

## Second Account
Let's create a second account. We'll make a basic transfer to it.

1. Generate a new seed or use this one:
`title fork smoke flash network submit happy jar repair glance narrow spell hammer language practice salad curious zebra crane table dynamic salon vintage avoid`

```
zcash-warp〉account create "title fork smoke flash network submit happy jar repair glance narrow spell hammer language practice salad curious zebra crane table dynamic salon vintage avoid" "TEST" 1
```

This is our second account.

```
zcash-warp〉account list                                  09/23/2024 02:55:37 PM
[
  {
    "coin": 0,
    "id": 1,
    "key_type": 127,
    "name": "DEMO",
    "birth": 1,
    "balance": 624985000
  },
  {
    "coin": 0,
    "id": 2,
    "key_type": 127,
    "name": "TEST",
    "birth": 1,
    "balance": 0
  }
]
```

Grab the Orchard address of the second account
```
zcash-warp〉address 2 4                                   09/23/2024 02:56:27 PM
Address: uregtest1x2c8rjte4uqpus002dfjul72h42yc6mw6npj08e7shnl0wjdvfl2f7a7x6vuh0k9l2gfdc55gsdv7xfydwp4cvheqxrj2hccpgmy5cpt
```

Let's make a payment from account 1 to account 2 of 1 ZEC.

```
zcash-warp〉pay 1 zregtestsapling154t4hyfhzjn7pdtyfs9znamk872krs239jzgmr5fnp98u8gqmhndc28c3el0k95d7reus097upc 7 100000000 7 1 1
{
  "height": 102,
  "recipients": [
    {
      "address": "zregtestsapling154t4hyfhzjn7pdtyfs9znamk872krs239jzgmr5fnp98u8gqmhndc28c3el0k95d7reus097upc",
      "amount": 100000000,
      "change": false
    },
    {
      "address": "uregtest1qae0mnr0p9jenge23nc47f359kwkyqvq0kl3qfgyqmk5k37jcvqlzafype9r99yxwfhs38q3w0a2qgam489deht2fguv6hg6g5m0c5vk",
      "amount": 524965000,
      "change": true
    }
  ],
  "transparent_ins": 0,
  "sapling_net": -100000000,
  "orchard_net": 100020000,
  "fee": 20000,
  "privacy_level": 2,
  "num_inputs": [
    0,
    0,
    1
  ],
  "num_outputs": [
    0,
    1,
    1
  ],
  "data": null,
  "keys": null
}
```

::: info
The previous command means to "pay address `ureg..` (use any receiver)
an amount of 1 ZEC using any of our notes (any source pool). We'll
pay for the transaction fee and use a change if necessary".
:::

The wallet replied with a transaction description. However the transaction
is not published yet.

## Recipients

In the recipients section, we see the two outputs. The first one
is the payee information: address, amount & memo.
The second recipient is our own source address and corresponds to the change.

We have a note of ~6.25 ZEC. Notes are not divisible. They must always
be spent fully. Therefore, we are bringing 6.25 ZEC in. If we don't include
a change output, the difference would go to the miner. The wallet app
adds a change output because `use_change` is 1. However, if we used
0, `zcash-warp` will refuse to make this transaction because the change
cannot go anywhere.

```
  "transparent_ins": 0,
  "sapling_net": -100000000,
  "orchard_net": 100020000,
  "fee": 20000,
```
This section describes the amount going in and out of each pool.

::: warning
All the transparent data is public and the net change of the
shielded pools is also public.
:::

This transaction does not have any transparent data, it is fully
shielded.

However, we can see that `sapling_net` and `orchard_net`
are not 0. `sapling_net` is -100000000: 1 ZEC is going *into*
the Sapling pool.  `orchard_net` is 100020000: 1 ZEC (+20000 ZATS)
is going *out* of the Orchard pool.

We can expect that since our wallet only has an Orchard note,
and we are sending to a Sapling address.

> 1 ZEC went through the turnstile. This information is *public*.

::: info
The address and amount spent and sent to shielded addresses are *hidden*.
:::

## Privacy level

The privacy level is a measure of how much information is revealed
by your transaction.

```
  "privacy_level": 2,
```

| Privacy Level |    |
| ----------    | ---|
| 0 | The transaction has *both* transparent inputs and outputs |
| 1 | The transaction has *either* transparent inputs or outputs |
| 2 | The transaction has *no* transparent components but a significant amount
|   | of funds is going through the turnstile
| 3 | The transaction reveals minimum information. Only the fees is public

## Number of inputs and outputs per pool

```
  "num_inputs": [
    0,
    0,
    1
  ],
  "num_outputs": [
    0,
    1,
    1
  ],
```

This shows that the transaction has 1 orchard input, 1 sapling output and 1 orchard output.
The number of inputs and outputs determine the fees.

## ZIP 317

We calculate the fees by multiplying the number of logical actions by their unit
price of 5000 ZATS.

The total number of logical actions (TLA) is the sum of transparent LA, sapling LA and orchard LA.

For each pool, the LA is the MAX between the number of inputs and outputs.

In our example, there are no transparent inputs and outputs, TLA = 0.
There is 0 sapling input and 1 sapling output, SLA = 1.
There is 1 orchard input and 1 orchard output, OLA = 1.

::: warning
However, for SLA and OLA, the minimum value (if not 0) is *TWO*[^2].
:::

Therefore, the real value for SLA is 2. The same applies to OLA.

Finally, the TLA = 0 + 2 + 2 = 4

The fee is 4 x 5000 = 20000.

We will keep analyzing our transaction plans in the following sections.

Let's execute this transaction now.

```
zcash-warp〉broadcast-latest                              09/23/2024 03:33:18 PM
"a782d0b23b61fea589be9798e5f3498c3508bf5a40ef18392586ca4eb1450542"
```

> Your transaction ID will be different from this one.

`zcash-warp` sent the transaction to LWD. LWD relayed it to `zcashd`. It is now
in the mempool. As always, we need to mine a block to get it included in the
blockchain.

```
$ ./src/zcash-cli --datadir=regtest_dir generate 1
```

And then sync our wallet to get the updated state.

```
zcash-warp〉sync                                          09/23/2024 03:35:58 PM
zcash-warp〉balance 2                                     09/23/2024 03:36:00 PM
Balance: BalanceT { transparent: 0, sapling: 100000000, orchard: 0 }
zcash-warp〉balance 1                                     09/23/2024 03:36:20 PM
Balance: BalanceT { transparent: 0, sapling: 0, orchard: 524965000 }
zcash-warp〉                                              09/23/2024 03:36:20 PM
```

Account #2 has 1 ZEC in the sapling pool and Account #1 has 5.25 ZEC in Orchard.

Next, we will make a transaction that has more than one recipient in order
to split our 5.25 ZEC into smaller denominations.

---
[^1]: The pool mask is the sum of 3 values. 1 for Transparent,
2 for Sapling and 4 for Orchard.
[^2]: This is due to the padding introduced by the transaction builder.
The resulting transaction has dummy inputs and outputs that you
have to pay.
