## 📚 web3-toolkit-rs

<br>

##### this package contains an ongoing library and set of rust scripts for the blockchains.

<br>

#### ethereum-toolkit

* [connect to ethereum through http, websockets, or batching](src/ethereum-toolkit/connections.rs)
* [retrieve ethereum wallets or account information](src/ethereum-toolkit/accounts.rs)

<br>


#### arbitrum-toolkit

* [connect to arbitrum through http, websockets, or batching](src/arbitrum-toolkit/connections.rs)
* [retrieve arbitrum wallets or account information](src/arbitrum-toolkit/accounts.rs)

<br>

#### avalanche-toolkit

* [connect to avalanche through http, websockets, or batching](src/avalanche-toolkit/connections.rs)
* [retrieve avalanche wallets or account information](src/avalanche-toolkit/accounts.rs)

<br>

#### near-toolkit

* [connect to near through http, websockets, or batching](src/near-toolkit/connections.rs)
* [retrieve near wallets or account information](src/near-toolkit/accounts.rs)

<br>

#### optimism-toolkit

* [connect to optimism through http, websockets, or batching](src/optimism-toolkit/connections.rs)
* [retrieve optimism wallets or account information](src/optimism-toolkit/accounts.rs)

<br>

#### polygon-toolkit

* [connect to polygon through http, websockets, or batching](src/polygon-toolkit/connections.rs)
* [retrieve polygon wallets or account information](src/polygon-toolkit/accounts.rs)

<br>

#### solana-toolkit

* [connect to solana through http, websockets, or batching](src/solana-toolkit/connections.rs)
* [retrieve solana wallets or account information](src/solana-toolkit/accounts.rs)

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