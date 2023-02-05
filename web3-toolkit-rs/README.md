## 📚 web3-toolkit-rs

<br>

##### on-going development of a library and set of rust scripts with my fav on-chain ops: 

* [wrapper to connect through http, websockets, or batching](src/web3_connection.rs)
* [wrapper to retrieve wallets or account information](src/accounts.rs)

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

### wrapper to connect through http, websockets, or batching

<br>

---

### wrapper to retrieve wallets or account information

<br>