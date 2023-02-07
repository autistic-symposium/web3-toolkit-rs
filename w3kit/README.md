## 🕹 w3kit: a web3 toolkit in rust
<br>

### tl;dr 



##### 🛠 **[this package](https://github.com/go-outside-labs/web3-toolkit-rs/tree/main/w3kit)** contains an ongoing crate with a set of scripts for several blockchains (check it at **[crates.io](https://crates.io/crates/w3kit)**).

##### 📚 for a fun in-depth reading about rust, check my mirror post [w3kit: a web3 toolkit in rust]().

##### 💡 for a rusty boilerplate for running stat searchers, check **[coingator](https://github.com/go-outside-labs/searcher-coingator-rs)**.

<br>

<br>


<img width="555" src="https://user-images.githubusercontent.com/1130416/217120907-56bf0ab9-5cec-4f81-8359-818df3f5f242.png">


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



#### setup

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

### ethereum

<br>

#### connection to the chain

<br>

###### through an http connection:

```
> w3kit http -b ethereum

✅ connecting to "ethereum"
✅ current block: 16572583
```
<br>

###### through a websocket connection:

```
> w3kit ws -b ethereum

✅ connecting to "ethereum"
✅ current block: 16572598
```

<br>

#### retrieving an account data


```
> w3kit account -b ethereum -a 0xbA4C081942E6a25cAF87c5489B91b449c67f3078

✅ connecting to "ethereum"
✅ fetching account info: "0xbA4C081942E6a25cAF87c5489B91b449c67f3078"
✅ retrieving balances...
      💰 account 0xba4c081942e6a25caf87c5489b91b449c67f3078 👉 0.0672775267238201 ETH
```

<br>

#### retrieving a coin price


```
>  w3kit coin ethereum

✅ fetching marketcap for ethereum
✅ GET https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd&include_market_cap=true
      🪙 price     👉 $1625.35
      📊 marketcap 👉 195950687355.82028
```

<br>

---

### arbitrum


<br>

#### connection to the chain

<br>

###### through an http connection:

```
> w3kit http -b arbitrum 
```

<br>

###### through a websocket connection:

```
> w3kit ws -b arbitrum 
```

<br>

#### retrieving an account data


```
> w3kit account -b arbitrum -a <account>
```



<br>

---

### avalanche


<br>

#### connection to the chain

<br>

###### through an http connection:

```
> w3kit http -b avalanche 
```

<br>

###### through a websocket connection:

```
> w3kit ws -b avalanche
```

<br>

#### retrieving an account data


```
> w3kit account -b avalanche -a <account>
```



<br>

---

### near


<br>

#### connection to the chain

<br>

###### through an http connection:

```
> w3kit http -b near
```

<br>

###### through a websocket connection:

```
> w3kit ws -b near
```

<br>

#### retrieving an account data


```
> w3kit account -b near -a <account>
```


<br>

---

### optimism


<br>

#### connection to the chain

<br>

###### through an http connection:

```
> w3kit http -b optimism
```

<br>

###### through a websocket connection:

```
> w3kit ws -b optimism
```


<br>

#### retrieving an account data


```
> w3kit account -b optimism -a <account>
```

<br>

---

### polygon


<br>

#### connection to the chain

<br>

###### through an http connection:

```
> w3kit http -b polygon
```

<br>

###### through a websocket connection:

```
> w3kit ws -b polygon
```

<br>

#### retrieving an account data


```
> w3kit account -b polygon -a <account>
```


<br>

---

### solana


<br>

#### connection to the chain

<br>

###### through an http connection:

```
> w3kit http -b solana -a 
```

<br>

###### through a websocket connection:

```
> w3kit ws -b solana -a 
```

<br>

#### retrieving an account data


```
> w3kit account -b solana -a <account>
```


<br>
