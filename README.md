# count-unsafe

> This is a fork of [`count-unsafe`](https://github.com/mkroening/count-unsafe).
> It wasn't up-to-date enough with stable Rust features and I changed the way it prints.

Count-unsafe counts the amount of unsafe Rust code in a given path.

This project is built on the [geiger] library.
In contrast to [cargo-geiger] though, this application does not integrate with cargo and simply counts unsafe code in all Rust source files in a given path.

[geiger]: https://crates.io/crates/geiger
[cargo-geiger]: https://crates.io/crates/cargo-geiger

## Installation

This project is available on [crates.io]:

[crates.io]: https://crates.io/crates/count-unsafe

```console
cargo install count-unsafe
```

## Example

```console
$ count-unsafe src
             |  Safe  | Unsafe
------------ | ------ | ------
Functions    |  189   |   30  
Expressions  | 12750  |  2393 
Item impls   |   98   |   13  
Item traits  |   5    |   1   
Methods      |  451   |   16 
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
