## 🕹 w3kit: a web3 toolkit in rust
<br>

### tl;dr 

<br>

* 🛠 **[this package](https://crates.io/crates/w3kit)** contains an ongoing library and set of rust scripts for the blockchains.
* 💡 for a rusty boilerplate for running stat searchers, check **[coingator](https://github.com/go-outside-labs/searcher-coingator-rs)**.

<br>

----

### overview




<br>

---

### install and setup

<br>

set config info in a `.env` file:

```
cp .env.example .env
vim .env
```

<br>

build the library ([cargo](https://doc.rust-lang.org/cargo/) commands are encoded in the `Makefile`):

```
make build
```

<br>


----

### features

<br>
#### ethereum-toolkit

* [connect to ethereum through http, websockets, or batching](src/ethereum/connections.rs)
* [retrieve ethereum wallets or account information](src/ethereum/accounts.rs)

<br>


#### arbitrum-toolkit

* [connect to arbitrum through http, websockets, or batching](src/arbitrum/connections.rs)
* [retrieve arbitrum wallets or account information](src/arbitrum/accounts.rs)

<br>

#### avalanche-toolkit

* [connect to avalanche through http, websockets, or batching](src/avalanche/connections.rs)
* [retrieve avalanche wallets or account information](src/avalanche/accounts.rs)

<br>

#### near-toolkit

* [connect to near through http, websockets, or batching](src/near/connections.rs)
* [retrieve near wallets or account information](src/near/accounts.rs)

<br>

#### optimism-toolkit

* [connect to optimism through http, websockets, or batching](src/optimism/connections.rs)
* [retrieve optimism wallets or account information](src/optimism/accounts.rs)

<br>

#### polygon-toolkit

* [connect to polygon through http, websockets, or batching](src/polygon/connections.rs)
* [retrieve polygon wallets or account information](src/polygon/accounts.rs)

<br>

#### solana-toolkit

* [connect to solana through http, websockets, or batching](src/solana/connections.rs)
* [retrieve solana wallets or account information](src/solana/accounts.rs)

<br>




---



### running