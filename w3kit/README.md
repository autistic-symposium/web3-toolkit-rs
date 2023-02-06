## 🕹 w3kit: a web3 toolkit in rust
<br>

### tl;dr 

<br>

* 🛠 **[this package](https://crates.io/crates/w3kit)** contains an ongoing library and set of scripts for several blockchains.
* 💡 for a rusty boilerplate for running stat searchers, check **[coingator](https://github.com/go-outside-labs/searcher-coingator-rs)**.

<br>

<br>



<img width="555" src="https://user-images.githubusercontent.com/1130416/216885451-7536cb4e-2c2f-4ede-ab96-69c1c9ce3b1d.png">


<br>

<br>

---

### installation

<br>


#### as a package

<br>

```
cargo install w3kit
```

<br>

#### for development

<br>

to build the library from this [GitHub repository](https://github.com/go-outside-labs/web3-toolkit-rs/tree/main/w3kit) you can run:

```
make build
```

<br>

note that all [cargo](https://doc.rust-lang.org/cargo/) commands relevant to this work are encoded in the `Makefile`.

<br>



#### creating a `.env`

<br>

create an `.env` file:

```
cp .env.example .env
vim .env
```

then add the config for the desired chain:

```
PROVIDER_URL_WS=
PROVIDER_URL_HTTP=
```

<br>

----

### available features

<br>

#### ethereum

<br>

##### 1. retrieve an account's balance through an http connection:

```
> w3kit http -b ethereum -a 0xbA4C081942E6a25cAF87c5489B91b449c67f3078

✅ connecting to "ethereum"
✅ retrieving balances...
      💰 account 0xba4c081942e6a25caf87c5489b91b449c67f3078 👉 0.0672775267238201 ETH
```
<br>

2. retrieve an account's balance through a websocket connection:

```
> w3kit ws -b ethereum -a 0xbA4C081942E6a25cAF87c5489B91b449c67f3078

✅ connecting to "ethereum"
✅ retrieving balances...
      💰 account 0xba4c081942e6a25caf87c5489b91b449c67f3078 👉 0.0672775267238201 ETH
```

<br>

---

#### arbitrum


<br>

##### 1. retrieve an account's balance through an http connection:

```
> w3kit http -b arbitrum -a 
```

<br>

##### 2. retrieve an account's balance through a websocket connection:

```
> w3kit ws -b arbitrum -a 

```

<br>

---

#### avalanche


<br>

##### 1. retrieve an account's balance through an http connection:

```
> w3kit http -b avalanche -a 
```

<br>

##### 2. retrieve an account's balance through a websocket connection:

```
> w3kit ws -b avalanche -a 

```

<br>

---

#### near


<br>

##### 1. retrieve an account's balance through an http connection:

```
> w3kit http -b near -a 
```

<br>

##### 2. retrieve an account's balance through a websocket connection:

```
> w3kit ws -b near -a 

```

<br>

---

#### optimism


<br>

##### 1. retrieve an account's balance through an http connection:

```
> w3kit http -b optimism -a 
```

<br>

##### 2. retrieve an account's balance through a websocket connection:

```
> w3kit ws -b optimism -a 

```

<br>

---

#### polygon


<br>

##### 1. retrieve an account's balance through an http connection:

```
> w3kit http -b polygon -a 
```

<br>

##### 2. retrieve an account's balance through a websocket connection:

```
> w3kit ws -b polygon -a 
```

<br>

---

#### solana


<br>

##### 1. retrieve an account's balance through an http connection:

```
> w3kit http -b solana -a 
```

<br>

##### 2. retrieve an account's balance through a websocket connection:

```
> w3kit ws -b solana -a 
```

<br>
