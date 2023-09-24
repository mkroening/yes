# The fastest `yes` on the planet

## Usage

1.  Install `pv` ([Pipe Viewer](https://www.ivarch.com/programs/pv.shtml)) to view pipes and witness incredible speeds.

2.  Yes.

    ```bash
    RUSTFLAGS="-C target-cpu=native" cargo run --release | pv -r > /dev/null
    ```

3.  Profit.

    On my machine (M2, macOS 13.5.2):
    | Implementation | Throughput |
    |----------------|------------|
    | macOS          | 4.4 GiB/s  |
    | This           | 6.3 GiB/s  |

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
