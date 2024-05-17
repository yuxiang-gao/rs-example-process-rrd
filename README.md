# Example of using the Rerun crates to parse an RRD

Note: This directly uses internal-facing Rerun APIs. These are not stable and may change at any time.

To create a sample `data.rrd`:
```
cargo run --bin log
```

To process the `data.rrd`:
```
cargo run --bin process -- data.rrd
```
