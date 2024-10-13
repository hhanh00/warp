# Coin Voting v2

> Vote anonymously using your shielded ZEC, 
where your stake equals your voting power.

## Opportunities

- Governance
- Proof of Balance
- Polls
- Airdrops

## High Level Description

I organized a few coin-weighted votes last summer about the devfund.

https://forum.zcashcommunity.com/t/the-last-minute-shielded-petition/48244
https://forum.zcashcommunity.com/t/coin-weighted-poll/47964

1. The pollster, in this case, me, would like to have the opinion of
the community of Zcash holders.
1. He defines an eligibility window of blocks. Only Orchard notes
created during these blocks can be used for voting.
1. At the end of the window, zcash holders can "cast" their
votes using the ZEC they have.
1. They can vote using their wallet app (Ywallet at the moment)

::: tip
Voting is **secure** and **private**. All the source code will
be published under the MIT license.
:::

This was Coin Voting v1.

Learning from these experiments, I am proposing an improved version.

## Coin Voting V2

1. The eligibility window *may* be extended to cover all of the Orchard notes
at the expense of requiring more compute and bandwidth
1. Splitting notes is not necessary. Voters can cast multiple
ballots (for different options)
1. Voters can *delegate* their voting power to other people
1. Some voters can be *granted* voting power not associated with ZEC 
1. The pollster can prove in ZK that the tally of the votes
is correct

::: info
- Delegation
- Multi ballots
- Grants

are new features that introduce a new circuit code
:::

