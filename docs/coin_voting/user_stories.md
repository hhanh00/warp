# Election Board / Pollster

On July 2024,
The community wanted to poll the Zcash Holders regarding
the future of the DevFund past Nov 24.

> "What of the following proposals do you support?"

The options were:
- "None of these options",
- "Manufacturing Consent; Re-Establishing a Dev Fund for ECC, ZF, ZCG, Qedit, FPF, and ZecHub (by NoamChom)",
- "Establishing a Hybrid Dev Fund for ZF, ZCG and a Dev Fund Reserve (by Jack Gavigan)",
- "Lockbox For Decentralized Grants Allocation (perpetual 50% option) (by Skylar Saveland)",
- "Hybrid Deferred Dev Fund: Transitioning to a Non-Direct Funding Model (by Jason McGee, Peacemonger, GGuy)",
- "Lockbox For Decentralized Grants Allocation (20% option) (by Kris Nuttycombe)",
- "Masters Of The Universe? (by NoamChom)",
- "End the Dev Fund and return 100% of block rewards to miners"

From YWallet, users could use their funds to vote for their favorite proposal.

[Voting in YWallet](https://youtu.be/c-aFzW6kWNk)

The results were 
[published on the forums](https://forum.zcashcommunity.com/t/coin-weighted-poll/47964/160).

## What did we do to make it happen?

All the parameters that define the election/vote are stored in a
JSON file.

For instance, the previous vote had these settings:

```json
{
    "id": 2,
    "name": "Devfund Poll Proposals 2",
    "start_height": 2540000,
    "end_height": 2574200,
    "close_height": 2576000,
    "submit_url": "/submit/2",
    "question": "What proposal do you support?",
    "candidates": [
        "None of these options",
        "Manufacturing Consent; Re-Establishing a Dev Fund for ECC, ZF, ZCG, Qedit, FPF, and ZecHub (by NoamChom)",
        "Establishing a Hybrid Dev Fund for ZF, ZCG and a Dev Fund Reserve (by Jack Gavigan)",
        "Lockbox For Decentralized Grants Allocation (perpetual 50% option) (by Skylar Saveland)",
        "Hybrid Deferred Dev Fund: Transitioning to a Non-Direct Funding Model (by Jason McGee, Peacemonger, GGuy)",
        "Lockbox For Decentralized Grants Allocation (20% option) (by Kris Nuttycombe)",
        "Masters Of The Universe? (by NoamChom)",
        "End the Dev Fund and return 100% of block rewards to miners"
    ],
    "cmx": "233ea22fc2067c7141bb418f6cb71c308933eac205c3d79879c1c6acbed0a357",
    "nf": "03517198be86743e34ee057d5f41dc97b0a68da7d58eab0fee072af00288d472",
    "status": "Opened"
}
```

It was made manually but Coin Voting V2 will have a tool to assist
in its creation.

Let's look at its content.

`question` and `candidates` are the question asked and the possible answers.
Voting does not support free-form answers. The voters must choose
amongst one of the given options.

However, there is no limit on the number of options.

The name of the election is displayed on the voting UI and must uniquely
identify the election. It gets hashed and serves as a way to distinguish between
ballots.

::: tip
Ballots that use the same note cannot be linked
because of the unique election ID
:::

It is important to make sure that another election does not use
the same name.

::: warning
To be sure of its uni
:::

