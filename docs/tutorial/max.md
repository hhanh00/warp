# Max Transfer

When you want to transfer the entire balance from one account to another
address, it seems natural to expect to make a transaction for the total amount.
However, if we try to do that, we will get an error saying we are coming short.
Indeed, the transaction has to pay some fees. You can consider transferring a
slightly lower amount to account for the fees, but you may end up leaving a
small balance behind if you make a slight miscalculation. Moreover, with ZIP-317
fee mechanism, the fees is rather complicated. Not only they depend on the
transaction type, but also on the number of input notes and dummy output notes.

## Sender Pays Fees

The option 'sender pays fees' simplifies this use case. Usually, the recipient
expects the sender to pay for the transaction fees. When you pay a merchant with
a credit card, there is often a surcharge.

In my country, the surcharge is on top of the displayed price. For example, if
the item costs $1, I will end up paying $1.02. In other countries, the surcharge
is included in the quoted price. In the same situation, I would pay $1, but the
merchant would receive $0.98 because $0.02 is paid to the credit card company.
The option 'sender pays fees' indicates whether you want the first option or the
second option.

Going back to our original scenario, if you want to completely transfer the
balance of your account, you should set the transaction amount to be the total
balance and turn off 'sender pays fees'. The fees are automatically subtracted
from the transaction and the source account will be empty.

```
zcash-warp〉pay 1 zregtestsapling154t4hyfhzjn7pdtyfs9znamk872krs239jzgmr5fnp98u8gqmhndc28c3el0k95d7reus097upc 7 624950000 7 0 1
```
```json
{
  "height": 103,
  "recipients": [
    {
      "address": "zregtestsapling154t4hyfhzjn7pdtyfs9znamk872krs239jzgmr5fnp98u8gqmhndc28c3el0k95d7reus097upc",
      "amount": 624915000,
      "change": false
    },
    {
      "address": "uregtest1qae0mnr0p9jenge23nc47f359kwkyqvq0kl3qfgyqmk5k37jcvqlzafype9r99yxwfhs38q3w0a2qgam489deht2fguv6hg6g5m0c5vk",
      "amount": 0,
      "change": true
    }
  ],
  "transparent_ins": 200000000,
  "sapling_net": -324915000,
  "orchard_net": 124950000,
  "fee": 35000,
  "privacy_level": 1,
  "num_inputs": [
    2,
    3,
    2
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

The command specified an amount of 624950000, but the transaction sends 624915000
to the destination address. It is exactly 35000 less that accounts for the fees.

## Use Change

However, we also see that we have an output change of 0. If you don't want to
have this output, you can unset `use_change`. The transaction builder will
not have an output change, but your transaction *must* work without one.

::: info
The wallet will not create a transaction that has more than the amount
of fees prescribed by ZIP-317.
:::

```
zcash-warp〉pay 1 zregtestsapling154t4hyfhzjn7pdtyfs9znamk872krs239jzgmr5fnp98u8gqmhndc28c3el0k95d7reus097upc 7 624950000 7 0 0
```
```json
{
  "height": 103,
  "recipients": [
    {
      "address": "zregtestsapling154t4hyfhzjn7pdtyfs9znamk872krs239jzgmr5fnp98u8gqmhndc28c3el0k95d7reus097upc",
      "amount": 624915000,
      "change": false
    }
  ],
  "transparent_ins": 200000000,
  "sapling_net": -324915000,
  "orchard_net": 124950000,
  "fee": 35000,
  "privacy_level": 1,
  "num_inputs": [
    2,
    3,
    2
  ],
  "num_outputs": [
    0,
    1,
    0
  ],
  "data": null,
  "keys": null
}
```

In this example, we pay the same fee even without the change output because
we have 2 orchard inputs.

However, there are some situations where you would end up paying 5000 ZATS
less.
