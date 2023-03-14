## some good tricks when writing rust (ongoing)

<br>

#### packages

* if you are testing a published crate locally, you can build with `make build` and then run the binary from `./targets/debug/<package name>`

<br>

##### `Some()`

* `Option::as_deref` converts `&Option<Vec<T>>` into `Option<&[T]>`
* `unwrap_or_default` returns `&[T]`
