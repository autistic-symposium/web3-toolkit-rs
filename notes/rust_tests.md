## 🦀 writing tests in rust

<br>

### unit tests

<br>

* the crates **[assert_cmd](https://docs.rs/assert_cmd)** has several helpers that makes easier, while the crate **[predicates](https://docs.rs/predicates)** helps to write assertions that can be tested against.

<br>

---

### integration tests

<br>

* you want to write integrations tests for all types of behavior that a user can observe. you don't necessary need to cover all edge cases: it's usually enough to write examples for the different types and rely on unit tests for edge cases.