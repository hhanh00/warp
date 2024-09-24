# Account Creation

## Generate a Seed Phrase

Using the `zcash-warp` command prompt, generate a new
24-word seed phrase:
```
zcash-warp〉generate-seed                                 09/22/2024 11:44:54 PM
bus abandon spell scout nuclear bamboo mystery scare network sentence intact north charge level choose family mule design tobacco night online sad invest bleak
zcash-warp〉                                              09/22/2024 11:44:54 PM
```

::: info
Your seed phrase will be different from this one.
:::

## Create a new database
If you are using `zcash-warp` for the first time, you have to create
a new database

```
zcash-warp〉create-database                               09/22/2024 11:51:21 PM
```

## Create a new account

Then, create an account using that seed phrase.

```
zcash-warp〉account create "bus abandon spell scout nuclear bamboo mystery scare network sentence intact north charge level choose family mule design tobacco night online sad invest bleak" "DEMO" 1
zcash-warp〉                                              09/22/2024 11:51:54 PM
```

- The 2nd argument, "DEMO" is the name of the account. 
- The 3rd argument, 1, is its birth height. We are saying that there are no transactions
on this account before block 1. The app does not need to process the blocks before.

::: tip
On Mainnet, using a birth height is **highly** recommended because it
can save you a lot of time.
:::

## Balance

Its balance should be 0 at this point.

```
zcash-warp〉balance 1                                     09/22/2024 11:52:18 PM
Balance: BalanceT { transparent: 0, sapling: 0, orchard: 0 }
zcash-warp〉                                              09/22/2024 11:52:18 PM
```

## Addresses

Let's query this account addresses.

Every account has *7* different addresses:
- a transparent address,
- a sapling address
- an orchard only Unified Addresses
- and UA that combine 2 or 3 receivers: T+S, T+O, S+O and T+S+O

We ask for an address with the command `address <account> <mask>`.
- The first account has number 1.
- The mask is the sum of 1 for Transparent, 2 for Sapling and 
4 for Orchard. For example, if you want the transparent & orchard UA, use mask=5 because
5 = 1+4. 

```
zcash-warp〉address 1 5                                   09/22/2024 11:56:58 PM
Address: uregtest1jk5cs77cx43humsmyd3f7eah0lvgvhuy8k3hhdgn6rmeu60933vljv2ytrasaf3d5utfz5mxzgwu8jhlg665d4j4v5e5clvkjjv3eks0wy8kjf75zqzl3vm3y2w4e96qzlkhc73yg3w
zcash-warp〉                                              09/22/2024 11:56:58 PM
```

::: info
Your address will be different if you are using a different seed.
:::

Next, we'll mine some blocks, collect the block reward and send some funds
to the Zcash-warp wallet.
