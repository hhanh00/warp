# Accounts

New accounts are derived from a random 24 seed phrases.

Existing accounts can be restored from the same type of
seed phrase obviously, but also other various types of keys.

First, the seed phrase can be 12, 18, 21 or 24 words
long. You can append your own password to it (like TREZOR).

From a seed phrase, ZKool derives several related keys.

::: warning
- **You should keep the seed phrase.**
- You do not have and (should not) keep the sub keys since
they can be derived from the seed phrase. They are provided
in case you need to restore your account in an app that does
not support seed phrases.
:::

## Summary

| Type of Key             | Transparent | Add. Transparent | Sapling | Divers. Sapling | Orchard | Divers. Orchard |
|-------------            |-------------|------------------|---------|-----------------|---------|-----------------|
|   Seed                  |     Full    |     Full         |  Full   |      Full       |  Full   |   Full          |
|  Unified Viewing Key    |    View     |    View          |  View   |      View       |  View   |   View          |
|  Sapling Secret Key     |     No      |      No          |  Full   |     Full        |  No     |   No            |
|  Sapling Viewing Key    |     No      |      No          |  View   |      View       |  No     |   No            |
|  XPub Transparent SK    |    Full     |    Full          |   No    |      No         |  No     |   No            |
|  Xpub Transparent VK    |    View     |    View          |   No    |      No         |  No     |   No            |
|  Transparent SK         |    Full     |     No           |   No    |      No         |  No     |   No            |      |                 |         |                 |
|  Transparent Address    |    View     |     No           |   No    |      No         |  No     |   No            |      |                 |         |                 |

::: info
- Full: Can view history and balance; Can sign new transactions.
- View: Can view history and balance.
- Unified Viewing Key can have different combinations of receivers. This table
assumes your UVK has all of them, but we support keys that do not.
- Xpub keys do not have a *standard* encoding and are not recognized in other apps.
- Transparent SK are sometimes just called "Secret Keys" in non-shielded wallets.
:::

## Address Diversification

Accounts can have *multiple* addresses that can be used interchangeably. Use
them when you want to give your account address to different people and prevent
them from linking your identity.

::: tip
When you want to keep your balance separate, use different account indices or
seed phrases.
:::

The shielded protocols (both Sapling and Orchard) support address
diversification natively. A single key can derive many addresses. These are
called diversified addresses. The default address is just the first diversified
address. Scanning several diversified addresses has no additional cost because
they all have the same key.

::: warning
Scanning shielded keys is **exponentially**
more expensive than transparent keys. Use diversified keys whenever possible.
They cost nothing to scan: 1000 keys = 1 key
:::

The transparent protocol does not support address diversification; therefore, it
is emulated by deriving different keys and addresses. The wallet scans them
independently. This is because the servers have indexed them, making scanning a
simple retrieval from a database.

> A slight cost is incurred for each transparent address, but it is tiny compared
to shielded keys.

However, the legacy secret key does not have the information needed to derive
additional addresses[^1]. They predate BIP-32 that introduced key derivation.

## Unified Viewing Keys

Unified Viewing Keys combine the information needed to derive addresses for
various pools. For example, a UVK with every receiver has the transparent
extended public key, the sapling viewing key, and the orchard viewing key. Then,
the wallet can derive the unified address at any index by merging the
transparent address at that index with the sapling address at that diversified
index and the orchard address at that same diversified index.

The default address uses the address index 0.

However, a significant issue arises. In the Sapling pool, only a subset of
diversified indexes is valid, and we cannot predict which ones are in advance.
Approximately 50% of the time, a given diversified address index is invalid.
Despite this, there are still around 2^10 valid indices.

This means we let UVK and UA use different indices for their receivers or skip
the invalid Sapling indices for the other receivers.

Some wallets skip invalid Sapling indices, but others don't.

Skipping improves unlinkability since two UA will always contain a
different receiver. 

However, wallets and accounts made before the creation of UA may use different
indices. The best solution is a shielded wallet that supports multiple
transparent addresses and diversified shielded addresses. 
Then, it could skip the index 0 if it is invalid for
Sapling but still monitor its transparent address.

ZecWallet supported multiple transparent addresses, but not diversified
shielded addresses.

| Wallet    | Multiple Transparent Addresses | Diversified Shielded Addresses |
|-----------|--------------------------------|--------------------------------|
| ZecWallet | Y                              | N                              |
| YWallet   | N                              | Y                              |
| Zingo     | N                              | N                              |
| Zashi     | N                              | N                              |
| Zkool     | Y                              | Y                              |

::: warning
Account seed phrases are not always transferable between wallet apps.
:::

Stick with the app you used but if you cannot find your funds, this is
most likely caused by different address derivations[^2].

[^1]: It does not have the "chain code".
[^2]: We haven't talked about change/internal addresses. Wallets can
also use different methods.
