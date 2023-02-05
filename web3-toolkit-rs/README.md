## 📚 web3-toolkit-rs

<br>

##### this package contains an ongoing library and set of rust scripts for the blockchains.

<br>

#### ethereum

* [connect to ethereum through http, websockets, or batching](src/ethereum/connections.rs)
* [retrieve ethereum wallets or account information](src/ethereum/accounts.rs)

<br>


#### arbitrum

* [connect to arbitrum through http, websockets, or batching](src/arbitrum/connections.rs)
* [retrieve arbitrum wallets or account information](src/arbitrum/accounts.rs)

<br>

#### near

* [connect to avalanche through http, websockets, or batching](src/near/connections.rs)
* [retrieve avalanche wallets or account information](src/near/accounts.rs)

<br>

#### optimism

* [connect to optimism through http, websockets, or batching](src/arbitrum/connections.rs)
* [retrieve optimism wallets or account information](src/arbitrum/accounts.rs)

<br>

#### polygon

* [connect to polygon through http, websockets, or batching](src/arbitrum/connections.rs)
* [retrieve polygon wallets or account information](src/arbitrum/accounts.rs)

<br>

#### solana

* [connect to solana through http, websockets, or batching](src/arbitrum/connections.rs)
* [retrieve solana wallets or account information](src/arbitrum/accounts.rs)

<br>




---

### setting up

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

### running