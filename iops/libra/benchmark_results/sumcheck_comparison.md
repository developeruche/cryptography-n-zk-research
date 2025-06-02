# SumCheck Protocol Benchmark Results

## Performance Comparison

The following tables summarize the benchmark results comparing the Traditional SumCheck and Linear Time SumCheck implementations.

### Basic Performance (microseconds)

| Operation | Traditional SumCheck | Linear Time SumCheck | Improvement |
|-----------|--------------------:|---------------------:|------------:|
| Prove     | 7.41 µs             | 4.97 µs              | 32.9%       |
| Verify    | 4.35 µs             | 3.02 µs              | 30.6%       |
| Full Protocol | 11.64 µs        | 7.89 µs              | 32.2%       |

### Scaling Performance (microseconds)

| Input Size | Traditional SumCheck | Linear Time SumCheck | Traditional vs Linear Time |
|------------|--------------------:|---------------------:|---------------------------:|
| 2          | 1.30 µs             | 2.58 µs              | Traditional 2.0x faster    |
| 4          | 4.41 µs             | 8.22 µs              | Traditional 1.9x faster    |
| 8          | 38.68 µs            | 65.53 µs             | Traditional 1.7x faster    |
| 16         | 8.61 ms             | 13.29 ms             | Traditional 1.5x faster    |

### Throughput (elements/second)

| Input Size | Traditional SumCheck | Linear Time SumCheck |
|------------|--------------------:|---------------------:|
| 2          | 1.54 Melem/s        | 0.77 Melem/s         |
| 4          | 907 Kelem/s         | 487 Kelem/s          |
| 8          | 207 Kelem/s         | 122 Kelem/s          |
| 16         | 1.86 Kelem/s        | 1.20 Kelem/s         |

## Analysis

1. **Basic Performance**: For the specific test case from the test module, the Linear Time SumCheck implementation is approximately 33% faster for both proving and verification.

2. **Scaling Behavior**: When we benchmark with varying input sizes, the Traditional SumCheck implementation performs better:
   - For size 2 inputs, Traditional SumCheck is 2.0x faster
   - For size 16 inputs, Traditional SumCheck is 1.5x faster
   - The performance gap appears to decrease as the input size increases

3. **Algorithm Characteristics**:
   - The Linear Time SumCheck shows better performance for the specific polynomial structure used in the test cases
   - The Traditional SumCheck seems to have better general scaling properties across different input sizes
   - The Linear Time approach may be optimized for specific sparse polynomial representations

4. **Considerations for Usage**:
   - For polynomials with structure similar to the test case, Linear Time SumCheck offers better performance
   - For general-purpose use with varied polynomial structures, Traditional SumCheck may be more reliable
   - The choice between implementations should consider the specific polynomial structure and size in the application

## Future Work

1. Benchmark with different polynomial structures to understand when Linear Time SumCheck offers advantages
2. Profile memory usage of both implementations
3. Investigate optimization opportunities in both implementations
4. Test with larger input sizes to better understand asymptotic behavior