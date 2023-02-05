## 🕹 w3kit: a web3 toolkit in rust
<br>

### tl;dr 

<br>

* 🛠 **[this package](https://crates.io/crates/w3kit)** contains an ongoing library and set of scripts for the blockchains.
* 💡 for a rusty boilerplate for running stat searchers, check **[coingator](https://github.com/go-outside-labs/searcher-coingator-rs)**.

<br>

<br>


<img width="550" src="https://user-images.githubusercontent.com/1130416/216848107-0f541895-7ed7-4d2d-969d-786781ee6037.png">



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

#### ethereum

* [connect to ethereum through http or websockets](src/ethereum/connections.rs)
* [retrieve ethereum wallets or account information](src/ethereum/accounts.rs)

<br>


#### arbitrum

* [connect to arbitrum through http, websockets](src/arbitrum/connections.rs)
* [retrieve arbitrum wallets or account information](src/arbitrum/accounts.rs)

<br>

#### avalanche

* [connect to avalanche through http, websockets](src/avalanche/connections.rs)
* [retrieve avalanche wallets or account information](src/avalanche/accounts.rs)

<br>

#### near

* [connect to near through http, websockets](src/near/connections.rs)
* [retrieve near wallets or account information](src/near/accounts.rs)

<br>

#### optimism

* [connect to optimism through http, websockets](src/optimism/connections.rs)
* [retrieve optimism wallets or account information](src/optimism/accounts.rs)

<br>

#### polygon

* [connect to polygon through http, websockets, or batching](src/polygon/connections.rs)
* [retrieve polygon wallets or account information](src/polygon/accounts.rs)

<br>

#### solana

* [connect to solana through http, websockets](src/solana/connections.rs)
* [retrieve solana wallets or account information](src/solana/accounts.rs)



