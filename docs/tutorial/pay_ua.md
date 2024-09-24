# Paying a Multi-Receiver UA

> Multi-Receivers UA are addresses that combine two or more types of addresses.
For example a Transparent and a Sapling, or a Transparent, a Sapling and an
Orchard address.

Multi-Receivers UAs present a new challenge to the wallet because they allow the
transaction algorithm to choose between two or more addresses.

Most wallets systematically choose the highest pool they support, but better
solutions may exist.

Consider a situation where the wallet has the following balance:
```
transparent: 200000000, sapling: 300000000, orchard: 124950000
```

Let's suppose we are paying 2.5 ZEC to a full receiver UA. What do you think the
wallet should do?

It would not have enough Orchard funds available if it were hard-wired to
deliver to the Orchard receiver.

However, by picking the Sapling receiver, the transaction achieves the highest
privacy since our Sapling balance can cover the required amount.

```
zcash-warp〉pay 1 uregtest1chggrm62vhkf5ftv3mawjvavuwjgx7x3wj582tlq5769cl6xf6q6whrukc5z2mxyz603j6lzda8pgyy8dy59qta58uncp6cetwmweklawr2ty34jqw6e8d704rly7m7eh9wcs50m0hdvsdg5akzys3uh95wugw9n78a5gj0mc0m69rsu2lf32tl8glje9p6870e9wwy6qeeyvg6x7ae 7 250000000 7 1 1
```

```json
{
  "height": 103,
  "recipients": [
    {
      "address": "zregtestsapling1h0ud99tf3qe9lmpmncs8rz0vwluhav0etuuejmvxcjnykd6gvp8tllh77ac0dm60zqmyq3c2a3t",
      "amount": 250000000,
      "change": false
    },
    {
      "address": "zregtestsapling1h0ud99tf3qe9lmpmncs8rz0vwluhav0etuuejmvxcjnykd6gvp8tllh77ac0dm60zqmyq3c2a3t",
      "amount": 49985000,
      "change": true
    }
  ],
  "transparent_ins": 0,
  "sapling_net": 15000,
  "orchard_net": 0,
  "fee": 15000,
  "privacy_level": 3,
  "num_inputs": [
    0,
    3,
    0
  ],
  "num_outputs": [
    0,
    2,
    0
  ],
  "data": null,
  "keys": null
}
```

As we can see from the transaction report, the wallet opted to use Sapling
notes.

::: tip
Feel free to experiment with a combination of receivers. What do
you think the wallet should do in each situation? Does it match
your expectations?
:::

The wallet follows a few ground rules:
- Shielded funds are preferable to transparent funds.
- If a recipient indicates a single shielded receiver address, the wallet will try to use funds from the same pool.
- We will never send to a transparent receiver of a UA.

They are other rules but they are more flexible.

Next, we'll use payment URI in place of the JSON object.
