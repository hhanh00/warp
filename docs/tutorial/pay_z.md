# Paying a Shielded Address

Sapling or Orchard behave identically. In this section, we'll use the Orchard pool and leave the Sapling as an exercise for the reader.

```
zcash-warp〉pay 1 uregtest1qae0mnr0p9jenge23nc47f359kwkyqvq0kl3qfgyqmk5k37jcvqlzafype9r99yxwfhs38q3w0a2qgam489deht2fguv6hg6g5m0c5vk 7 150000000 7 1 1
```

We'll omit the subsequent commands for brevity since they only differ from this one by the amount. Also, we'll simplify the command output and trim the receivers. 

```json
{
  "transparent_ins": 0,
  "sapling_net": 0,
  "orchard_net": 10000,
  "fee": 10000,
  "privacy_level": 3,
  "num_inputs": [
    0,
    0,
    2
  ],
  "num_outputs": [
    0,
    0,
    2
  ],
}
```

Compared with the previous section, where we were sending to a transparent address, the wallet now uses the Orchard pool rather than the Sapling pool. Sending from Sapling would reveal the amounts. When the destination is a transparent address, the amount and the address are both revealed anyway, and any shielded pool offers the same privacy.

::: tip
The wallet tries to avoid crossing the turnstile.
:::

We have a privacy level of 3, the highest level possible.

As we increase the transaction amount, we expect the wallet to use the Orchard pool, then the Sapling pool, and finally the Transparent pool.

::: info
Try using the same amounts as the previous section and see what
differs.
:::

At an amount of 4.5, we get similar results as before.

```json
{
  "transparent_ins": 100000000,
  "sapling_net": 200000000,
  "orchard_net": -299970000,
  "fee": 30000,
  "privacy_level": 1,
  "num_inputs": [
    1,
    2,
    3
  ],
  "num_outputs": [
    0,
    0,
    2
  ],
}
```

At an amount of 4.5, we get similar results as before. The recipient is shielded, hence the higher privacy level.

As mentioned earlier, Sapling has the same behavior. Let's focus
on recipients that use a multi-receiver unified address.
