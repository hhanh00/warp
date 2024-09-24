# Splitting Notes

As we saw earlier, notes must be fully spent.
Usually, this is not an issue because we receive
the change from the same transaction.

::: info
The change comes from
our spending transaction. Whereas in real life,
the payee receives the note and then returns
the change in a separate transaction.
:::

However, if we want to make several transactions
in a row, having to wait for the change becomes
a problem. The transaction needs to be mined
and often the wallet waits a few blocks[^1]
before it makes incoming notes available for spending.

::: tip
You can configure the number of confirmations
in `zcash-warp`. We set it to 1. A typical value is
3 for mobile wallets.
:::

By "pre-splitting" your wallet notes, you make
smaller denominations. This is similar to keeping
for small bills/coins rather than holding a large note.

## Multi-pay

There is no automated way to split notes in `zcash-warp`[^2].
So we will do it manually by making a transaction
that pays ourselves multiple times.

The input of the multi-pay command is a JSON object.

The JSON object for the basic transaction in the previous section
is

```json
{
    "recipients": [
        {
            "address": "zregtestsapling154t4hyfhzjn7pdtyfs9znamk872krs239jzgmr5fnp98u8gqmhndc28c3el0k95d7reus097upc",
            "amount": 100000000,
            "pools": 7,
            "memo": null,
            "memo_bytes": null
        }
    ],
    "src_pools": 7,
    "sender_pay_fees": true,
    "use_change": true,
    "height": 103,
    "expiration": 203
}
```

Most of the data is easily recognizable. There are a few new fields.
- `memo` and `memo_bytes` specify the memo of the note. Only one
of these fields should be populated. Use `memo` if it is a text
and `memo_bytes` if it should be arbitrary binary data.
- `height` is the current block height. It must be one of the
synchronization point.

::: info
Warp Sync works in leaps and bounds. When you sync up to the
current block height, it may stop at a few intermediate
points, but it will not include every block.
:::

You can list the sync points with the command `checkpoint list`
or simply use the current block height with `last-height`.

Expiration height is the block # at which the transaction
should expire. A value of 50 to 100 blocks after the
`height` is usual.

To split the 5.25 ZEC for the first wallet, we first 
edit the JSON object and replace the destination address
with the account #1 Orchard address (`address 1 4`)
and duplicate it 5 times.

```json
{
    "recipients": [
        {
            "address": "uregtest1qae0mnr0p9jenge23nc47f359kwkyqvq0kl3qfgyqmk5k37jcvqlzafype9r99yxwfhs38q3w0a2qgam489deht2fguv6hg6g5m0c5vk",
            "amount": 100000000,
            "pools": 7,
            "memo": null,
            "memo_bytes": null
        },
        {
            "address": "uregtest1qae0mnr0p9jenge23nc47f359kwkyqvq0kl3qfgyqmk5k37jcvqlzafype9r99yxwfhs38q3w0a2qgam489deht2fguv6hg6g5m0c5vk",
            "amount": 100000000,
            "pools": 7,
            "memo": null,
            "memo_bytes": null
        },
        {
            "address": "uregtest1qae0mnr0p9jenge23nc47f359kwkyqvq0kl3qfgyqmk5k37jcvqlzafype9r99yxwfhs38q3w0a2qgam489deht2fguv6hg6g5m0c5vk",
            "amount": 100000000,
            "pools": 7,
            "memo": null,
            "memo_bytes": null
        },
        {
            "address": "uregtest1qae0mnr0p9jenge23nc47f359kwkyqvq0kl3qfgyqmk5k37jcvqlzafype9r99yxwfhs38q3w0a2qgam489deht2fguv6hg6g5m0c5vk",
            "amount": 100000000,
            "pools": 7,
            "memo": null,
            "memo_bytes": null
        },
        {
            "address": "uregtest1qae0mnr0p9jenge23nc47f359kwkyqvq0kl3qfgyqmk5k37jcvqlzafype9r99yxwfhs38q3w0a2qgam489deht2fguv6hg6g5m0c5vk",
            "amount": 100000000,
            "pools": 7,
            "memo": null,
            "memo_bytes": null
        }
    ],
    "src_pools": 7,
    "sender_pay_fees": true,
    "use_change": true,
    "height": 103,
    "expiration": 203
}
```

Then we use `multi-pay` to make a transaction with that JSON.

::: warning
The JSON object has to be on one line. Use a JSON minifier
to compactify.
:::

```
zcash-warp〉multi-pay 1 '{"recipients":[{"address":"uregtest1qae0mnr0p9jenge23nc47f359kwkyqvq0kl3qfgyqmk5k37jcvqlzafype9r99yxwfhs38q3w0a2qgam489deht2fguv6hg6g5m0c5vk","amount":100000000,"pools":7,"memo":null,"memo_bytes":null},{"address":"uregtest1qae0mnr0p9jenge23nc47f359kwkyqvq0kl3qfgyqmk5k37jcvqlzafype9r99yxwfhs38q3w0a2qgam489deht2fguv6hg6g5m0c5vk","amount":100000000,"pools":7,"memo":null,"memo_bytes":null},{"address":"uregtest1qae0mnr0p9jenge23nc47f359kwkyqvq0kl3qfgyqmk5k37jcvqlzafype9r99yxwfhs38q3w0a2qgam489deht2fguv6hg6g5m0c5vk","amount":100000000,"pools":7,"memo":null,"memo_bytes":null},{"address":"uregtest1qae0mnr0p9jenge23nc47f359kwkyqvq0kl3qfgyqmk5k37jcvqlzafype9r99yxwfhs38q3w0a2qgam489deht2fguv6hg6g5m0c5vk","amount":100000000,"pools":7,"memo":null,"memo_bytes":null},{"address":"uregtest1qae0mnr0p9jenge23nc47f359kwkyqvq0kl3qfgyqmk5k37jcvqlzafype9r99yxwfhs38q3w0a2qgam489deht2fguv6hg6g5m0c5vk","amount":100000000,"pools":7,"memo":null,"memo_bytes":null}],"src_pools":7,"sender_pay_fees":true,"use_change":true,"height":103,"expiration":203}'
```

We get the following transaction:
```
{
  "height": 103,
  "recipients": [
    {
      "address": "uregtest1qae0mnr0p9jenge23nc47f359kwkyqvq0kl3qfgyqmk5k37jcvqlzafype9r99yxwfhs38q3w0a2qgam489deht2fguv6hg6g5m0c5vk",
      "amount": 100000000,
      "change": false
    },
    {
      "address": "uregtest1qae0mnr0p9jenge23nc47f359kwkyqvq0kl3qfgyqmk5k37jcvqlzafype9r99yxwfhs38q3w0a2qgam489deht2fguv6hg6g5m0c5vk",
      "amount": 100000000,
      "change": false
    },
    {
      "address": "uregtest1qae0mnr0p9jenge23nc47f359kwkyqvq0kl3qfgyqmk5k37jcvqlzafype9r99yxwfhs38q3w0a2qgam489deht2fguv6hg6g5m0c5vk",
      "amount": 100000000,
      "change": false
    },
    {
      "address": "uregtest1qae0mnr0p9jenge23nc47f359kwkyqvq0kl3qfgyqmk5k37jcvqlzafype9r99yxwfhs38q3w0a2qgam489deht2fguv6hg6g5m0c5vk",
      "amount": 100000000,
      "change": false
    },
    {
      "address": "uregtest1qae0mnr0p9jenge23nc47f359kwkyqvq0kl3qfgyqmk5k37jcvqlzafype9r99yxwfhs38q3w0a2qgam489deht2fguv6hg6g5m0c5vk",
      "amount": 100000000,
      "change": false
    },
    {
      "address": "uregtest1qae0mnr0p9jenge23nc47f359kwkyqvq0kl3qfgyqmk5k37jcvqlzafype9r99yxwfhs38q3w0a2qgam489deht2fguv6hg6g5m0c5vk",
      "amount": 24935000,
      "change": true
    }
  ],
  "transparent_ins": 0,
  "sapling_net": 0,
  "orchard_net": 30000,
  "fee": 30000,
  "privacy_level": 3,
  "num_inputs": [
    0,
    0,
    1
  ],
  "num_outputs": [
    0,
    0,
    6
  ],
  "data": null,
  "keys": null
}
```

This is an Orchard to Orchard transaction based on the `num_inputs`
and `num_outputs`. There are 6 outputs: 5 x 1 ZEC + change.
The Orchard Logical Actions is MAX(1, 6) = 6. We pay 6 x 5000 = 30000
of fees.
Fees are always public, and we see they came from the Orchard pool
since `orchard_net` = 30000.

As usual, let's submit the transaction, mine and sync.

```
zcash-warp〉broadcast-latest                              09/23/2024 05:26:03 PM
$ ./src/zcash-cli --datadir=regtest_dir generate 1
zcash-warp〉sync                                          09/23/2024 05:26:58 PM
```

## Balance

Looking at the balance, not much has changed. We just have slightly
less because we had to pay the transaction fees.

```
zcash-warp〉balance 1                                     09/23/2024 05:32:42 PM
Balance: BalanceT { transparent: 0, sapling: 0, orchard: 524935000 }
```

However, the wallet has 6 notes.

```
[
  {
    "id_note": 4,
    "height": 104,
    "confirmations": 1,
    "timestamp": 1727076805,
    "value": 100000000,
    "orchard": true,
    "excluded": false
  },
  {
    "id_note": 5,
    "height": 104,
    "confirmations": 1,
    "timestamp": 1727076805,
    "value": 100000000,
    "orchard": true,
    "excluded": false
  },
  {
    "id_note": 6,
    "height": 104,
    "confirmations": 1,
    "timestamp": 1727076805,
    "value": 100000000,
    "orchard": true,
    "excluded": false
  },
  {
    "id_note": 7,
    "height": 104,
    "confirmations": 1,
    "timestamp": 1727076805,
    "value": 100000000,
    "orchard": true,
    "excluded": false
  },
  {
    "id_note": 8,
    "height": 104,
    "confirmations": 1,
    "timestamp": 1727076805,
    "value": 100000000,
    "orchard": true,
    "excluded": false
  },
  {
    "id_note": 9,
    "height": 104,
    "confirmations": 1,
    "timestamp": 1727076805,
    "value": 24935000,
    "orchard": true,
    "excluded": false
  }
]
```

Let's diversify our wallet by having some transparent, sapling
and orchard notes.

---
[^1]: Some wallet wait as many as TEN blocks.
[^2]: Ywallet automates note splitting.
