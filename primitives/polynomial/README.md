# Polynomial Crate

A Rust implementation of various polynomial types and operations used in cryptography and zero-knowledge proofs. This crate provides efficient implementations of univariate, multivariate, and multilinear polynomials along with common operations like evaluation, interpolation, and arithmetic.

## Features

- **Multiple Polynomial Types**:
  - Univariate Polynomials
  - Multivariate Polynomials 
  - Multilinear Polynomials
  - Virtual Polynomials (specialized for product check IOP)
  - Composed Multilinear Polynomials

- **Core Operations**:
  - Polynomial evaluation
  - Lagrange interpolation
  - FFT/IFFT for efficient operations
  - Arithmetic operations (addition, multiplication, division)
  - Partial evaluations
  - Domain operations

- **Optimization Features**:
  - Efficient evaluation using FFT
  - Specialized implementations for cryptographic applications
  - Memory efficient representations
  - Support for parallel computation

## Usage Examples

### Creating and Evaluating a Univariate Polynomial

```rust
use polynomial::univariant::UnivariantPolynomial;
use ark_test_curves::bls12_381::Fr;

// Create a polynomial: 2x^2 + 3x + 1
let poly = UnivariantPolynomial::new(vec![Fr::from(1), Fr::from(3), Fr::from(2)]);

// Evaluate at x = 2
let result = poly.evaluate(&Fr::from(2));
```

### Working with Multilinear Polynomials

```rust
use polynomial::multilinear::Multilinear;
use ark_test_curves::bls12_381::Fr;

// Create a multilinear polynomial
let evaluations = vec![Fr::from(3), Fr::from(1), Fr::from(2), Fr::from(5)];
let num_vars = 2;
let polynomial = Multilinear::new(evaluations, num_vars);

// Evaluate at a point
let point = vec![Fr::from(5), Fr::from(6)];
let eval_result = polynomial.evaluate(&point);
```

### Polynomial Interpolation

```rust
use polynomial::univariant::UnivariantPolynomial;
use ark_test_curves::bls12_381::Fr;

let point_ys = vec![Fr::from(0), Fr::from(4), Fr::from(16)];
let domain = vec![Fr::from(0), Fr::from(2), Fr::from(4)];

let interpolated_poly = UnivariantPolynomial::interpolate(point_ys, domain);
```

## Dependencies

- `ark-ff`: Finite field arithmetic
- `ark-test-curves`: Test curves for cryptographic operations
- `ark-serialize`: Serialization support
- `digest`: Cryptographic digest functions
- `ark-std`: Standard library utilities
- `anyhow`: Error handling
- `rand`: Random number generation

## Features

- `std`: Use standard library (enabled by default)
- `parallel`: Enable parallel computation support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
polynomial = { version = "0.1.0" }
```

## Testing

Run the test suite:

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.