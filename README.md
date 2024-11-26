# Eg-walker Paper Benchmark

This repo is forked from [josephg/egwalker-paper](https://github.com/josephg/egwalker-paper) and adds Loro's benchmark.

## Benchmark

| ms  | automerge | yrs   | loro (snapshot mode) | DT (opt load) | loro (updates mode) | DT (merge) |
| --- | --------- | ----- | -------------------- | ------------- | ------------------- | ---------- |
| S1  | 648.6     | 9.92  | 624.23 us            | 111.35 us     | 26.38               | 2.3615     |
| S2  | 801.18    | 13.15 | 429.80 us            | 63.59 us      | 47.87               | 3.6813     |
| S3  | 1578.8    | 10.70 | 675.22 us            | 48.51 us      | 76.88               | 4.7401     |
| C1  | 13786     | 13.66 | 1.169 ms             | 238.80 us     | 471.94              | 59.063     |
| C2  | 28605     | 9.54  | 1.458 ms             | 187.29 us     | 617.44              | 84.079     |
| A1  | 627.79    | 12.50 | 442.48 us            | 20.70 us      | 145.27              | 10.03      |
| A2  | 667.03    | 13.80 | 680.75 us            | 82.47 us      | 181.85              | 27.19      |

> The benchmarks were performed on M2 MAX CPU.
> Loro has two encoding modes `snapshot` and `update`, you can find more details [here](https://www.loro.dev/docs/tutorial/encoding)

## Reproducing

Run the following command to reproduce the benchmark results.

```bash
sh ./step2b-benchmarks.sh
```

> We have converted the datasets (S1/S2/S3/C1/C2/A1/A2) to `datasets/xxx-snapshot.loro` and `datasets/xxx-updates.loro`. You can convert them yourself by running `convert_main()` in `paper-benchmarks/src/convert1.rs`.
