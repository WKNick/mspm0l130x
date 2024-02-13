# `mspm0l130x`

> Peripheral access API for MSPM0L130X microcontroller

Generated using [svd2rust]. The following registers had to be removed or changed to be compatable with svd2rust such as CMDWEPROTC, STATMODE, and WWDTCTL0. Changes consisted of removing the nonexistent register CMDWEPROTC, and changing STATMODE and WWDTCTL0 to not containe refrences to accesses which did not include "once". Also added GENCLKCFG register and inturrupts to SVD file

[svd2rust]: https://github.com/japaric/svd2rust

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.