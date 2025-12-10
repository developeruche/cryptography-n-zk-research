use ark_ff::Field;

pub struct StackedMle<F: Field> {
    /// The is a dense representation of the columns
    pub q: Vec<F>,
    /// Represent the start points for each columns
    pub metadata: Vec<usize>,
}

impl<F: Field> StackedMle<F> {
    pub fn new(columns: &[Vec<F>]) -> Self {
        let sum_of_len = columns.iter().map(|col| col.len()).sum::<usize>();
        let padded_len = sum_of_len.next_power_of_two();

        let mut q = columns.iter().flatten().copied().collect::<Vec<_>>();
        let mut metadata = Vec::new();
        let mut offset = 0;
        for col in columns.iter() {
            offset += col.len();
            metadata.push(offset);
        }

        if q.len() < padded_len {
            q.resize(padded_len, F::zero());
        }

        let m_len = metadata.len();
        metadata[m_len - 1] = padded_len;

        Self { q, metadata }
    }

    pub fn col(&self, index: usize) -> Option<usize> {
        let mut col = None;

        for (i, &val) in self.metadata.iter().enumerate() {
            if index < val {
                col = Some(i);
                break;
            }
        }

        col
    }

    pub fn row(&self, index: usize) -> usize {
        let col_index = self.col(index).unwrap();
        let rol_index = if col_index == 0 {
            index
        } else {
            index - self.metadata[col_index - 1]
        };

        rol_index
    }

    pub fn eval(&self, z_c: &[F], z_r: &[F]) -> F {
        let row_eq = eq_funcs(z_r);
        let col_eq = eq_funcs(z_c);

        let mut res = F::ZERO;
        for (i, &coeff) in self.q.iter().enumerate() {
            res += coeff * row_eq[self.row(i)] * col_eq[self.col(i).unwrap()];
        }

        res
    }
}

fn to_bits<F: Field>(val: usize, len: usize) -> Vec<F> {
    let mut bits = Vec::with_capacity(len);
    for i in 0..len {
        if (val >> i) & 1 == 1 {
            bits.push(F::one());
        } else {
            bits.push(F::zero());
        }
    }
    bits
}

pub fn eq_1_func<F: Field>(z_i: F, b_i: F) -> F {
    z_i * b_i + (F::one() - z_i) * (F::one() - b_i)
}

pub fn eq_funcs<F: Field>(points: &[F]) -> Vec<F> {
    let bh = generate_hypercube_vec(points.len());
    let mut evals = Vec::with_capacity(bh.len());

    // Iterate over every point 'b' in the hypercube {0,1}^m
    for b in bh {
        let mut product_term = F::one();

        for (i, &bit) in b.iter().enumerate() {
            // If b_i = 1, term is z_i
            // If b_i = 0, term is (1 - z_i)
            let term = if bit == 1 {
                points[i]
            } else {
                F::one() - points[i]
            };

            product_term *= term;
        }
        evals.push(product_term);
    }

    evals
}

pub fn generate_hypercube_vec(m: usize) -> Vec<Vec<u8>> {
    if m >= 63 {
        panic!("m is too large for a vector-based hypercube");
    }
    let num_rows = 1 << m;
    let mut result = Vec::with_capacity(num_rows);

    for i in 0..num_rows {
        let mut row = Vec::with_capacity(m);
        for bit_index in (0..m).rev() {
            row.push(((i >> bit_index) & 1) as u8);
        }

        result.push(row);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;
    use ark_test_curves::bls12_381::Fr;

    #[test]
    fn test_stackedcol() {
        let columns = vec![
            vec![Fr::from(1), Fr::from(2), Fr::from(3)],
            vec![Fr::from(4)],
            vec![Fr::from(5)],
            vec![Fr::from(6), Fr::from(7)],
            vec![Fr::from(8), Fr::from(9), Fr::from(10)],
        ];
        let stacked_mle = StackedMle::new(&columns);

        assert_eq!(stacked_mle.col(0), Some(0));
        assert_eq!(stacked_mle.col(1), Some(0));
        assert_eq!(stacked_mle.col(2), Some(0));
        assert_eq!(stacked_mle.col(3), Some(1));
        assert_eq!(stacked_mle.col(4), Some(2));
        assert_eq!(stacked_mle.col(5), Some(3));
        assert_eq!(stacked_mle.col(6), Some(3));
        assert_eq!(stacked_mle.col(7), Some(4));
        assert_eq!(stacked_mle.col(8), Some(4));
        assert_eq!(stacked_mle.col(9), Some(4));
    }

    #[test]
    fn test_stacked_rol() {
        let columns = vec![
            vec![Fr::from(1), Fr::from(2), Fr::from(3)],
            vec![Fr::from(4)],
            vec![Fr::from(5)],
            vec![Fr::from(6), Fr::from(7)],
            vec![Fr::from(8), Fr::from(9), Fr::from(10)],
        ];
        let stacked_mle = StackedMle::new(&columns);

        assert_eq!(stacked_mle.row(0), 0);
        assert_eq!(stacked_mle.row(1), 1);
        assert_eq!(stacked_mle.row(2), 2);
        assert_eq!(stacked_mle.row(3), 0);
        assert_eq!(stacked_mle.row(4), 0);
        assert_eq!(stacked_mle.row(5), 0);
        assert_eq!(stacked_mle.row(6), 1);
        assert_eq!(stacked_mle.row(7), 0);
        assert_eq!(stacked_mle.row(8), 1);
        assert_eq!(stacked_mle.row(9), 2);
    }

    #[test]
    fn test_stacked_eval() {
        let columns = vec![
            vec![Fr::from(1), Fr::from(2), Fr::from(3)],
            vec![Fr::from(4)],
            vec![Fr::from(5)],
            vec![Fr::from(6), Fr::from(7)],
            vec![Fr::from(8), Fr::from(9), Fr::from(10)],
        ];
        let stacked_mle = StackedMle::new(&columns);

        assert_eq!(
            stacked_mle.eval(
                &vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(0)],
                &vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(0)]
            ),
            Fr::from(1)
        );
        assert_eq!(
            stacked_mle.eval(
                &vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(0)],
                &vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(1)]
            ),
            Fr::from(2)
        );
        assert_eq!(
            stacked_mle.eval(
                &vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(0)],
                &vec![Fr::from(0), Fr::from(0), Fr::from(1), Fr::from(0)]
            ),
            Fr::from(3)
        );
        assert_eq!(
            stacked_mle.eval(
                &vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(1)],
                &vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(0)]
            ),
            Fr::from(4)
        );
        assert_eq!(
            stacked_mle.eval(
                &vec![Fr::from(0), Fr::from(0), Fr::from(1), Fr::from(0)],
                &vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(0)]
            ),
            Fr::from(5)
        );
        assert_eq!(
            stacked_mle.eval(
                &vec![Fr::from(0), Fr::from(0), Fr::from(1), Fr::from(1)],
                &vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(0)]
            ),
            Fr::from(6)
        );
        assert_eq!(
            stacked_mle.eval(
                &vec![Fr::from(0), Fr::from(0), Fr::from(1), Fr::from(1)],
                &vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(1)]
            ),
            Fr::from(7)
        );
        assert_eq!(
            stacked_mle.eval(
                &vec![Fr::from(0), Fr::from(1), Fr::from(0), Fr::from(0)],
                &vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(0)]
            ),
            Fr::from(8)
        );
        assert_eq!(
            stacked_mle.eval(
                &vec![Fr::from(0), Fr::from(1), Fr::from(0), Fr::from(0)],
                &vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(1)]
            ),
            Fr::from(9)
        );
        assert_eq!(
            stacked_mle.eval(
                &vec![Fr::from(0), Fr::from(1), Fr::from(0), Fr::from(0)],
                &vec![Fr::from(0), Fr::from(0), Fr::from(1), Fr::from(0)]
            ),
            Fr::from(10)
        );
    }
}
