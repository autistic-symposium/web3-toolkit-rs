## 🕹 w3kit: a web3 toolkit in rust
<br>

### tl;dr 

<br>

* 🛠 **[this package](https://crates.io/crates/w3kit)** contains an ongoing library and set of scripts for the blockchains.
* 💡 for a rusty boilerplate for running stat searchers, check **[coingator](https://github.com/go-outside-labs/searcher-coingator-rs)**.

<br>

<br>

<img width="550" alt="Screen Shot 2023-02-05 at 12 33 12 PM" src="https://user-images.githubusercontent.com/1130416/216843363-da1460c8-7072-414b-8841-7776f3c42548.png">


<br>

<br>

---

### install and setup

<br>

#### creating an `.env` file

<br>


```
cp .env.example .env
vim .env
```

and add the following info:

```
PROVIDER_URL_WS=
PROVIDER_URL_HTTP=
```


<br>

#### installing as a package

<br>

```
cargo install w3kit
```

<br>

#### building for development

<br>

to build the library from this [GitHub repository](https://github.com/go-outside-labs/web3-toolkit-rs/tree/main/w3kit) you can run:

```
make build
```

<br>

note that all [cargo](https://doc.rust-lang.org/cargo/) commands that are relevant for this work are encoded in the `Makefile`.

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

