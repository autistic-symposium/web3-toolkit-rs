## 🥷🏻🎸⛓️ playing pvp in the metaweb: rust edition

<br>


### go-outside-labs rust projects

<br>

* **💡 start with [rust tl; dr](rust_tldr.md)** and check **[rust tricks](rust_tricks.md)** at some point.

<br>

* **[🕹 w3kit](w3kit)**
    - **w3kit** is an ongoing development of a library for on-chain ops on **several blockchains**.
    - this package (crate) is published in **[crates.io](https://crates.io/crates/w3kit)** and can be imported as a library with `cargo add w3kit`.

<br>

* **[🤖 coingator](https://github.com/go-outside-labs/searcher-coingator-rs)**
    - a rusty boilerplate for building and running **statistical searchers**.
    - mirror post: **[bot #4: coingator, a rusty stat searcher](https://mirror.xyz/steinkirch.eth/FiDw34-pTvKidFP3xSeh4UdhJ5ClgvoxKhtlCdQ-I5Q)**.

<br>

* **[🧪 foundry science](https://github.com/go-outside-labs/blockchain-science-py/)**
    - leverage foundry and **"vm cheatcodes"** to analyze evm-based blockchains. 
    - example: historical data on avalanche c-chain to simulate **sandwich attacks in the gmx protocol**.

<br>


----

### resources for learning rust

<br>

#### basics

* [installing ](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch01-01-installation.html) guide
* [cargo](https://doc.rust-lang.org/cargo/): dep manager and build tool
* [rustfmt](https://github.com/rust-lang/rustfmt): code style
* [rust standard library](https://doc.rust-lang.org/std/index.html)

<br>

#### learning


* [rust docs](https://doc.rust-lang.org/stable/book/)
* [gentle intro to rust](https://stevedonovan.github.io/rust-gentle-intro/readme.html)
* [ethereum foundation on rust](https://ethereum.org/en/developers/docs/programming-languages/rust/)
* [google rust course](https://github.com/google/comprehensive-rust)


<br>


#### developing
* [beginner's guide to error handling in rust](https://www.sheshbabu.com/posts/rust-error-handling/)
* [publishing a crate in crate.ios](https://doc.rust-lang.org/cargo/reference/publishing.html)


<br>

#### foundry and tests

* [cargo book on tests](https://doc.rust-lang.org/cargo/guide/tests.html)
* [introducing foundry, by paradigm](https://www.paradigm.xyz/2021/12/introducing-the-foundry-ethereum-development-toolbox)
* [invariant testing weth with foundry](https://mirror.xyz/horsefacts.eth/Jex2YVaO65dda6zEyfM_-DXlXhOWCAoSpOx5PLocYgw)


<br>

#### libraries and crates

<br>

##### general

* [dotenv](https://crates.io/crates/dotenv): library for dotenv vars
* [clap](https://docs.rs/clap/latest/clap/): library for menu and argparse
* [reqwest](https://docs.rs/reqwest/latest/reqwest/): higher-level HTTP Client.
* [tokio](https://tokio.rs/tokio/tutorial/async): library for async, multi-threading
* [serde](https://serde.rs/): framework for serializing
* [chrono](https://docs.rs/chrono/latest/chrono/): library for date and time
* [comfy-table](https://crates.io/crates/comfy-table): pprint beautiful data tables
* [once_cell](https://crates.io/crates/once_cell): library for single assignments cells

<br>

##### web3-related

* [web3](https://crates.io/crates/web3): ethereum JSON-RPC client
* [ethers-provider](https://crates.io/crates/ethers-providers)
* [near api](https://crates.io/crates/near-api-tokio)

<br>

#### tools for searchers

<br>

* [ether.js vs. ether-rs mempool stream benchmark](https://github.com/CodeForcer/rust-pending-stream)


<br>

