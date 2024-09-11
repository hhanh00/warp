## Installation

- Rust: Go to [install](https://www.rust-lang.org/tools/install)
- Clone and update submodules
- *Install zcash sapling parameters* (it's done automatically if you 
have zcashd)
    - [Spend](https://download.z.cash/downloads/sapling-spend.params)
    - [Output](https://download.z.cash/downloads/sapling-output.params)
- Install sqlite3
    - on Ubuntu: `apt-get install libsqlite3-dev`
- (Optional) Install protoc (protobuf compiler). If you don't have it,
the build will fail. But since the generated files are checked in, you 
can simply delete both `build.rs` and retry.

## Build
`cargo b -r`

## Configure
- Edit `App.toml` and input your seed phrase & birth height. Make sure a copy of this file is located in the `warp` folder.
## Usage
Run `zcash-warp`

## Quick Start

```
create-database
create-account
reset
sync
balance 1
list-txs 1
```

## Commands
### create-database

Must be run this first to create the database file

### create-account

Must be run once to import the seed from the config file

### reset

Must be run after account creation and whenever you want to reset
the application state. This deletes the synchronization data
and brings the wallet back to its birth height.

### sync

Start synchronization from last sync height.

### help

Show the help page. There are several other commands to show the account balance,
addresses, transactions, notes, generate diversified addresses, make payments, etc.
