# SumCheck Protocol Benchmarks

This directory contains the benchmark results comparing the Traditional SumCheck and Linear Time SumCheck implementations.

## Summary Files

- [sumcheck_comparison.md](sumcheck_comparison.md) - Detailed analysis of benchmark results with tables and observations

## Benchmark Plots

The following plots provide visual comparisons of the performance metrics:

- [prove_comparison.svg](prove_comparison.svg) - Violin plot comparing proving time
- [verify_comparison.svg](verify_comparison.svg) - Violin plot comparing verification time
- [scaling_comparison.svg](scaling_comparison.svg) - Line chart showing scaling behavior with different input sizes
- [full_protocol_comparison.svg](full_protocol_comparison.svg) - Violin plot comparing full protocol execution time

## Key Findings

1. **Basic Performance**: Linear Time SumCheck is ~33% faster for the specific test case polynomials.
2. **Scaling**: Traditional SumCheck scales better with increasing input sizes.
3. **Throughput**: Traditional SumCheck has higher element throughput across all tested sizes.

## Running the Benchmarks

To run the benchmarks yourself:

```bash
cargo bench --bench sumcheck_benchmarks_plots
```

To explore all the generated reports:

```bash
open target/criterion/index.html
```

## Criterion Reports

Full benchmark reports are available in the `target/criterion` directory after running the benchmarks. These include:
- Detailed performance statistics
- Interactive charts
- Violin plots showing distribution of execution times
- Regression analysis