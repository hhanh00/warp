# Diverse Wallet

> T/S/O Input Notes

We don't always get to choose who sends us funds and what wallet they use. When
they have a transparent-only wallet, we receive transparent notes. We will only
get Orchard notes when they have a modern wallet. So what will `zcash-warp` do
with these notes?

In this section, we diversify our wallet by transferring some of our orchard
funds into the transparent and sapling pools.

Then, in the next section, we'll experiment with various spending transactions to see how our wallet picks our notes.

Let's make:
- 2 transparent notes
- 2 sapling note

## Restart blockchain

I will restart from the beginning because zcashd stops working if blocks are not generated at least every few hours.

To do so, run the follow commands to stop, clear the blockchain and restart
a new one. Then generate 101 blocks, shield 1 block reward and commit the
transaction to the blockchain.

```
$ ./src/zcash-cli --datadir=regtest_dir stop
$ rm -rf regtest_dir/regtest/
$ ./src/zcashd --datadir=regtest_dir --daemon
$ ./src/zcash-cli --datadir=regtest_dir generate 101
$ ./src/zcash-cli --datadir=regtest_dir z_shieldcoinbase '*' uregtest1jk5cs77cx43humsmyd3f7eah0lvgvhuy8k3hhdgn6rmeu60933vljv2ytrasaf3d5utfz5mxzgwu8jhlg665d4j4v5e5clvkjjv3eks0wy8kjf75zqzl3vm3y2w4e96qzlkhc73yg3w
$ ./src/zcash-cli --datadir=regtest_dir generate 1
```

Also, delete LWD database and restart it.

```
$ rm -rf lwd-data/
$ ./lightwalletd --no-tls-very-insecure --zcash-conf-path ~/projects/zcash/regtest_dir/zcash.conf --data-dir lwd-data --log-file /dev/stdout --grpc-bind-addr 0.0.0.0:9168
```

In `zcash-warp`, reset and sync.

```
zcash-warp〉reset                                         09/23/2024 09:04:26 PM
zcash-warp〉sync                                          09/23/2024 09:04:28 PM
zcash-warp〉balance 1                                     09/23/2024 09:05:17 PM
Balance: BalanceT { transparent: 0, sapling: 0, orchard: 624985000 }
```

This doesn't delete our accounts. We can check the balance of account #1.

## Pool Transfers using the `pools` filter

To make 2 transparent notes, 2 sapling notes and 2 orchard notes (+1 change),
let's use the following transaction (JSON).

You may have noticed that the receivers have the same address, but
the `pools` value differs.
The first two recipients have a `pools` of 1, forcing the wallet to
send to the transparent receiver.
With a `pools` value of 2, it sends to the Sapling pool and with 4
it sends to the Orchard pool.

```json
{
    "recipients": [
        {
            "address": "uregtest1chggrm62vhkf5ftv3mawjvavuwjgx7x3wj582tlq5769cl6xf6q6whrukc5z2mxyz603j6lzda8pgyy8dy59qta58uncp6cetwmweklawr2ty34jqw6e8d704rly7m7eh9wcs50m0hdvsdg5akzys3uh95wugw9n78a5gj0mc0m69rsu2lf32tl8glje9p6870e9wwy6qeeyvg6x7ae",
            "amount": 100000000,
            "pools": 1,
            "memo": null,
            "memo_bytes": null
        },
        {
            "address": "uregtest1chggrm62vhkf5ftv3mawjvavuwjgx7x3wj582tlq5769cl6xf6q6whrukc5z2mxyz603j6lzda8pgyy8dy59qta58uncp6cetwmweklawr2ty34jqw6e8d704rly7m7eh9wcs50m0hdvsdg5akzys3uh95wugw9n78a5gj0mc0m69rsu2lf32tl8glje9p6870e9wwy6qeeyvg6x7ae",
            "amount": 100000000,
            "pools": 1,
            "memo": null,
            "memo_bytes": null
        },
        {
            "address": "uregtest1chggrm62vhkf5ftv3mawjvavuwjgx7x3wj582tlq5769cl6xf6q6whrukc5z2mxyz603j6lzda8pgyy8dy59qta58uncp6cetwmweklawr2ty34jqw6e8d704rly7m7eh9wcs50m0hdvsdg5akzys3uh95wugw9n78a5gj0mc0m69rsu2lf32tl8glje9p6870e9wwy6qeeyvg6x7ae",
            "amount": 100000000,
            "pools": 2,
            "memo": null,
            "memo_bytes": null
        },
        {
            "address": "uregtest1chggrm62vhkf5ftv3mawjvavuwjgx7x3wj582tlq5769cl6xf6q6whrukc5z2mxyz603j6lzda8pgyy8dy59qta58uncp6cetwmweklawr2ty34jqw6e8d704rly7m7eh9wcs50m0hdvsdg5akzys3uh95wugw9n78a5gj0mc0m69rsu2lf32tl8glje9p6870e9wwy6qeeyvg6x7ae",
            "amount": 100000000,
            "pools": 2,
            "memo": null,
            "memo_bytes": null
        },
        {
            "address": "uregtest1chggrm62vhkf5ftv3mawjvavuwjgx7x3wj582tlq5769cl6xf6q6whrukc5z2mxyz603j6lzda8pgyy8dy59qta58uncp6cetwmweklawr2ty34jqw6e8d704rly7m7eh9wcs50m0hdvsdg5akzys3uh95wugw9n78a5gj0mc0m69rsu2lf32tl8glje9p6870e9wwy6qeeyvg6x7ae",
            "amount": 100000000,
            "pools": 4,
            "memo": null,
            "memo_bytes": null
        },
        {
            "address": "uregtest1chggrm62vhkf5ftv3mawjvavuwjgx7x3wj582tlq5769cl6xf6q6whrukc5z2mxyz603j6lzda8pgyy8dy59qta58uncp6cetwmweklawr2ty34jqw6e8d704rly7m7eh9wcs50m0hdvsdg5akzys3uh95wugw9n78a5gj0mc0m69rsu2lf32tl8glje9p6870e9wwy6qeeyvg6x7ae",
            "amount": 100000000,
            "pools": 4,
            "memo": null,
            "memo_bytes": null
        }
    ],
    "src_pools": 7,
    "sender_pay_fees": true,
    "use_change": true,
    "height": 102,
    "expiration": 202
}
```

```
zcash-warp〉multi-pay 1 '{"recipients":[{"address":"uregtest1chggrm62vhkf5ftv3mawjvavuwjgx7x3wj582tlq5769cl6xf6q6whrukc5z2mxyz603j6lzda8pgyy8dy59qta58uncp6cetwmweklawr2ty34jqw6e8d704rly7m7eh9wcs50m0hdvsdg5akzys3uh95wugw9n78a5gj0mc0m69rsu2lf32tl8glje9p6870e9wwy6qeeyvg6x7ae","amount":100000000,"pools":1,"memo":null,"memo_bytes":null},{"address":"uregtest1chggrm62vhkf5ftv3mawjvavuwjgx7x3wj582tlq5769cl6xf6q6whrukc5z2mxyz603j6lzda8pgyy8dy59qta58uncp6cetwmweklawr2ty34jqw6e8d704rly7m7eh9wcs50m0hdvsdg5akzys3uh95wugw9n78a5gj0mc0m69rsu2lf32tl8glje9p6870e9wwy6qeeyvg6x7ae","amount":100000000,"pools":1,"memo":null,"memo_bytes":null},{"address":"uregtest1chggrm62vhkf5ftv3mawjvavuwjgx7x3wj582tlq5769cl6xf6q6whrukc5z2mxyz603j6lzda8pgyy8dy59qta58uncp6cetwmweklawr2ty34jqw6e8d704rly7m7eh9wcs50m0hdvsdg5akzys3uh95wugw9n78a5gj0mc0m69rsu2lf32tl8glje9p6870e9wwy6qeeyvg6x7ae","amount":100000000,"pools":2,"memo":null,"memo_bytes":null},{"address":"uregtest1chggrm62vhkf5ftv3mawjvavuwjgx7x3wj582tlq5769cl6xf6q6whrukc5z2mxyz603j6lzda8pgyy8dy59qta58uncp6cetwmweklawr2ty34jqw6e8d704rly7m7eh9wcs50m0hdvsdg5akzys3uh95wugw9n78a5gj0mc0m69rsu2lf32tl8glje9p6870e9wwy6qeeyvg6x7ae","amount":100000000,"pools":2,"memo":null,"memo_bytes":null},{"address":"uregtest1chggrm62vhkf5ftv3mawjvavuwjgx7x3wj582tlq5769cl6xf6q6whrukc5z2mxyz603j6lzda8pgyy8dy59qta58uncp6cetwmweklawr2ty34jqw6e8d704rly7m7eh9wcs50m0hdvsdg5akzys3uh95wugw9n78a5gj0mc0m69rsu2lf32tl8glje9p6870e9wwy6qeeyvg6x7ae","amount":100000000,"pools":4,"memo":null,"memo_bytes":null},{"address":"uregtest1chggrm62vhkf5ftv3mawjvavuwjgx7x3wj582tlq5769cl6xf6q6whrukc5z2mxyz603j6lzda8pgyy8dy59qta58uncp6cetwmweklawr2ty34jqw6e8d704rly7m7eh9wcs50m0hdvsdg5akzys3uh95wugw9n78a5gj0mc0m69rsu2lf32tl8glje9p6870e9wwy6qeeyvg6x7ae","amount":100000000,"pools":4,"memo":null,"memo_bytes":null}],"src_pools":7,"sender_pay_fees":true,"use_change":true,"height":102,"expiration":202}'
{
  "height": 102,
  "recipients": [
    {
      "address": "tmFtne18VWwk7x8p2wUwfnAqtntMVikctYi",
      "amount": 100000000,
      "change": false
    },
    {
      "address": "tmFtne18VWwk7x8p2wUwfnAqtntMVikctYi",
      "amount": 100000000,
      "change": false
    },
    {
      "address": "zregtestsapling1h0ud99tf3qe9lmpmncs8rz0vwluhav0etuuejmvxcjnykd6gvp8tllh77ac0dm60zqmyq3c2a3t",
      "amount": 100000000,
      "change": false
    },
    {
      "address": "zregtestsapling1h0ud99tf3qe9lmpmncs8rz0vwluhav0etuuejmvxcjnykd6gvp8tllh77ac0dm60zqmyq3c2a3t",
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
      "amount": 24950000,
      "change": true
    }
  ],
  "transparent_ins": 0,
  "sapling_net": -200000000,
  "orchard_net": 400035000,
  "fee": 35000,
  "privacy_level": 1,
  "num_inputs": [
    0,
    0,
    1
  ],
  "num_outputs": [
    2,
    2,
    3
  ],
  "data": null,
  "keys": null
}

```

```json
  "fee": 35000,
  "privacy_level": 1,
  "num_inputs": [
    0,
    0,
    1
  ],
  "num_outputs": [
    2,
    2,
    3
  ],
```

The transaction creates 2 t-notes, 2 s-notes and o-notes, for a fee of 7 logical 
actions.

::: tip
Don't forget to broadcast, mine and sync.
:::

## Summary

```
zcash-warp〉balance 1                                     09/23/2024 09:20:10 PM
Balance: BalanceT { transparent: 200000000, sapling: 200000000, orchard: 224950000 }
```

Next, let's see what happens when we pay a transparent address.
