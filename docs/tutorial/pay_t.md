# Pay a Transparent Address

::: info
Paying a transparent address has a maximum privacy level
of 1. This is obviously not private.
:::

In this section, we will generate several transactions to see their structure, but we will not submit them because we want to keep our wallet composition the same.

Our transparent address is `tmFtne18VWwk7x8p2wUwfnAqtntMVikctYi`.

We can use the single pay version.

Let's send 0.5 ZEC to `tmFtne18VWwk7x8p2wUwfnAqtntMVikctYi`.

```
zcash-warp〉pay 1 tmFtne18VWwk7x8p2wUwfnAqtntMVikctYi 7 50000000 7 1 1
{
  "height": 103,
  "recipients": [
    {
      "address": "tmFtne18VWwk7x8p2wUwfnAqtntMVikctYi",
      "amount": 50000000,
      "change": false
    },
    {
      "address": "zregtestsapling1h0ud99tf3qe9lmpmncs8rz0vwluhav0etuuejmvxcjnykd6gvp8tllh77ac0dm60zqmyq3c2a3t",
      "amount": 49985000,
      "change": true
    }
  ],
  "transparent_ins": 0,
  "sapling_net": 50015000,
  "orchard_net": 0,
  "fee": 15000,
  "privacy_level": 1,
  "num_inputs": [
    0,
    1,
    0
  ],
  "num_outputs": [
    1,
    1,
    0
  ],
  "data": null,
  "keys": null
}
```

The wallet uses shielded funds first, preferably from the Sapling pool (as we like to keep our Orchard notes), as this gives a higher level of privacy than transparency. Effectively, we do a z2t transaction.

When we bump the amount to 1.5 ZEC, we run out of the first note, and the wallet selects the second Sapling note. The fee does not change because the minimum
sapling charge is 2 logical actions.

```
zcash-warp〉pay 1 tmFtne18VWwk7x8p2wUwfnAqtntMVikctYi 7 150000000 7 1 1
{
  "height": 103,
  "recipients": [
    {
      "address": "tmFtne18VWwk7x8p2wUwfnAqtntMVikctYi",
      "amount": 150000000,
      "change": false
    },
    {
      "address": "zregtestsapling1h0ud99tf3qe9lmpmncs8rz0vwluhav0etuuejmvxcjnykd6gvp8tllh77ac0dm60zqmyq3c2a3t",
      "amount": 49985000,
      "change": true
    }
  ],
  "transparent_ins": 0,
  "sapling_net": 150015000,
  "orchard_net": 0,
  "fee": 15000,
  "privacy_level": 1,
  "num_inputs": [
    0,
    2,
    0
  ],
  "num_outputs": [
    1,
    1,
    0
  ],
  "data": null,
  "keys": null
}
```

Let's increase it further to 3.5 ZEC.

```
zcash-warp〉pay 1 tmFtne18VWwk7x8p2wUwfnAqtntMVikctYi 7 250000000 7 1 1
{
  "height": 103,
  "recipients": [
    {
      "address": "tmFtne18VWwk7x8p2wUwfnAqtntMVikctYi",
      "amount": 250000000,
      "change": false
    },
    {
      "address": "uregtest1qae0mnr0p9jenge23nc47f359kwkyqvq0kl3qfgyqmk5k37jcvqlzafype9r99yxwfhs38q3w0a2qgam489deht2fguv6hg6g5m0c5vk",
      "amount": 49975000,
      "change": true
    }
  ],
  "transparent_ins": 0,
  "sapling_net": 200000000,
  "orchard_net": 50025000,
  "fee": 25000,
  "privacy_level": 1,
  "num_inputs": [
    0,
    2,
    1
  ],
  "num_outputs": [
    1,
    0,
    1
  ],
  "data": null,
  "keys": null
}
```

At this point, we use all the Sapling funds (2 ZEC) and some of the 0.5 ZEC

We use all the Sapling funds (2 ZEC) and a part of the 0.5 ZEC in Orchard.
The wallet sends the change back to the most private pool it used to source funds from. In this case, it is Orchard. Previously, it was Sapling.

At 4.1 ZEC, we should spend both Sapling and the 3 Orchard notes.

> Try it out yourself.

At 4.5, we have to use every pool available, and we get

```json
{
  "transparent_ins": 100000000,
  "sapling_net": 200000000,
  "orchard_net": 150030000,
  "fee": 30000,
  "privacy_level": 0,
  "num_inputs": [
    1,
    2,
    3
  ],
  "num_outputs": [
    1,
    0,
    1
  ],
}
```

In conclusion, when we send funds to a transparent address, the wallet picks the notes from private to transparent.

Next, let's try it out with a shielded address.
