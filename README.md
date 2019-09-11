# straightedge-node

[Straightedge](https://straighted.ge) is an:
- On-chain Governed,
- Proof-of-Stake (PoS) Blockchain
- with a WASM Runtime.

Developer resources are available at [Straightedge's Discord](https://discord.gg/ZRj9zRU).

For more details about the project, visit the [Straightedge website](https://straighted.ge) or [Twitter](http://twitter.com/heystraightedge) for the latest.

We are now pegged to: https://github.com/hicommonwealth/substrate.

## Get the software

Open a command line interface on the server you're installing on, and run the following commands:

- To download the latest version of the software:
```
wget https://github.com/heystraightedge/straightedge-node/archive/master.zip
```
- To unzip:
```
unzip master.zip
```

## Setup and build

- To setup your environment and build the software, run the following command from within the `straightedge-node-master` folder:

```
./setup.sh
```

This can take some time, so perhaps go outside, get some fresh air, and come back later.

You will know it is complete when you see:

```
   Compiling straightedge v0.8.0 (/home/straightedge-0/straightedge-node-master)
    Finished release [optimized] target(s) in 12m 04s
```

### Run

- Run the software by running the following commands from within the `straightedge-node-master` folder:

```
./target/release/straightedge
```



<!-- ### Initial Setup

```
curl https://sh.rustup.rs -sSf | sh
rustup update stable
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
cargo install --git https://github.com/alexcrichton/wasm-gc
```

You will also need to install the following packages:

Linux:
```
sudo apt install cmake pkg-config libssl-dev git clang libclang-dev
```

Mac:
```
brew install cmake pkg-config openssl git llvm
```

### Building

```
cargo build --release
```

### Running

Ensure you have a fresh start if updating from another version:
```
./scripts/purge-chain.sh
```
To start up the Straightedge node and connect to the latest testnet, run:
```
./target/release/edgeware --chain=edgeware-testnet-v8 --name <INSERT_NAME>
```

## Generating a keypair

To create a keypair, install subkey with `cargo install --force --git https://github.com/paritytech/substrate subkey`. Then run the following:
```
subkey generate
```
To create an ED25519 keypair, run the following:
```
subkey -e generate
```
To create derived keypairs, use the mnemonic generated from a method above and run (use `-e` for ED25519):
```
subkey inspect "<mnemonic>"//<derive_path>
```
For example:
```
subkey inspect "west paper guide park design weekend radar chaos space giggle execute canoe"//edgewarerocks
```
Then proceed to the *Running* instructions or follow the instructions below for the manual setup.


## Implemented Modules

### Edge

* [Identity](https://github.com/hicommonwealth/edgeware-node/tree/master/modules/edge-identity)
* [Voting](https://github.com/hicommonwealth/edgeware-node/tree/master/modules/edge-voting)
* [Signaling](https://github.com/hicommonwealth/edgeware-node/tree/master/modules/edge-signaling)
* [TreasuryReward](https://github.com/hicommonwealth/edgeware-node/tree/master/modules/edge-treasury-reward)

### SRML
* [Aura](https://github.com/hicommonwealth/substrate/tree/master/srml/aura)
* [AuthorityDiscovery](https://github.com/hicommonwealth/substrate/tree/master/srml/authority-discovery)
* [Authorship](https://github.com/hicommonwealth/substrate/tree/master/srml/authorship)
* [Indices](https://github.com/hicommonwealth/substrate/tree/master/srml/indices)
* [Balances](https://github.com/hicommonwealth/substrate/tree/master/srml/balances)
* [Contracts](https://github.com/hicommonwealth/substrate/tree/master/srml/contracts)
* [Council](https://github.com/hicommonwealth/substrate/tree/master/srml/council)
* [Democracy](https://github.com/hicommonwealth/substrate/tree/master/srml/democracy)
* [Elections](https://github.com/hicommonwealth/substrate/tree/master/srml/elections)
* [FinalityTracker](https://github.com/hicommonwealth/substrate/tree/master/srml/finality-tracker)
* [Grandpa](https://github.com/hicommonwealth/substrate/tree/master/srml/grandpa)
* [ImOnline](https://github.com/hicommonwealth/substrate/tree/master/srml/im-online)
* [Offences](https://github.com/hicommonwealth/substrate/tree/master/srml/offences)
* [Session](https://github.com/hicommonwealth/substrate/tree/master/srml/session)
* [Staking](https://github.com/hicommonwealth/substrate/tree/master/srml/staking)
* [Sudo](https://github.com/hicommonwealth/substrate/tree/master/srml/sudo)
* [System](https://github.com/hicommonwealth/substrate/tree/master/srml/system)
* [Timestamp](https://github.com/hicommonwealth/substrate/tree/master/srml/timestamp)
* [Treasury](https://github.com/hicommonwealth/substrate/tree/master/srml/treasury)

## Developing on Straightedge

### Running A Local Chain

To run a chain locally for development purposes: `./target/release/straightedge --chain=local --alice --validator`

To allow apps in your browser to connect, as well as anyone else on your local network, add the `--rpc-cors=all` flag.

To force your local to create new blocks, even if offline, add the `--force-authoring` flag.

### Adding A Module

1. Add its github repo to:
  - [Cargo.toml](Cargo.toml)
  - [node/runtime/Cargo.toml](node/runtime/Cargo.toml)
  - [node/runtime/wasm/Cargo.toml](node/runtime/wasm/Cargo.toml) (be sure to have `default-features = false`)
2. Changes to [the runtime](node/runtime/src/lib.rs):
  - Add it as an `extern crate`.
  - Implement its `Trait` with production types.
  - Add it to the `construct_runtime` macro with all implemented components.
3. If its storage contains `config` elements, then you need to modify [the chain spec](node/service/src/chain_spec.rs):
  - Add it to the `straightedge_runtime`'s list of `Config` types.
  - Add it to the `testnet_genesis` function, initializing all storage fields set to `config()`.
4. Build and run the chain.

## Session Key Setup
If you plan to validate on Edgeware or a testnet with any non-default keys, then you will need to store the keys so that the node has access to them, for signing transactions and authoring new blocks. Keys in Edgeware are stored in the keystore in the file system. To store keys into this keystore, you need to use one of the two provided RPC calls. If your keys are encrypted or should be encrypted by the keystore, you need to provide the key using one of the cli arguments --password, --password-interactive or --password-filename.

### Recommended RPC call
For most users who want to run a validator node, the author_rotateKeys RPC call is sufficient. The RPC call will generate N Session keys for you and return their public keys. N is the number of session keys configured in the runtime. The output of the RPC call can be used as input for the session::set_keys transaction.
```
curl -H 'Content-Type: application/json' --data '{ "jsonrpc":"2.0", "method":"author_rotateKeys", "id":1 }' localhost:9933
```

### Advanced RPC call
If the Session keys need to match a fixed seed, they can be set individually key by key. The RPC call expects the key seed and the key type. The key types supported by default in Edgeware are `aura`, `gran`, and `imon`.
```
curl -H 'Content-Type: application/json' --data '{ "jsonrpc":"2.0", "method":"author_insertKey", "params":["KEY_TYPE", "SEED"],"id":1 }' localhost:9933
```
`KEY_TYPE` - needs to be replaced with the 4-character key type identifier. `SEED` - is the seed of the key.
-->
