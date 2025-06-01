//! This modules holds algorithms defined in the Libra paper.
use p3_field::{ExtensionField, Field};

/// Algorithm 4: Initialize Phase 1
/// This goal of this algorithm is to create the A_h_g book-keeping table.
/// fn takes in this following parameters
/// - f_1
/// - f_3
/// - g (Instead of computing this table in this function and recompute this later in this protocol, we would just take it in a parameter) - i_gz
pub(crate) fn initialize_phase_1<F: Field, E: ExtensionField<F>>(
    f_1: &[(usize, usize, usize)],
    f_3: &[F],
    i_gz: &[E],
) -> Vec<E> {
    let mut res = vec![E::zero(); f_3.len()];

    for (z, x, y) in f_1 {
        // It is assumed the operation poly outputs 1 where there is a valid gate
        res[*x] += i_gz[*z] * f_3[*y];
    }

    res
}

/// Algorithm 5: Initialize Phase 2
/// This is very similer to algorithm 4, this goal here is to create the A_f_1 table.
/// fn takes in this following parameters
/// - f_1
/// - g (Instead of computing this table in this function and recompute this later in this protocol, we would just take it in a parameter) - i_gz
/// - u (Instead of computing this table in this function and recompute this later in this protocol, we would just take it in a parameter) - i_uz
pub(crate) fn initialize_phase_2<F: Field, E: ExtensionField<F>>(
    f_1: &[(usize, usize, usize)],
    i_gz: &[E],
    i_uz: &[E],
) -> Vec<E> {
    let mut res = vec![E::zero(); i_uz.len()];

    for (z, x, y) in f_1 {
        // It is assumed the operation poly outputs 1 where there is a valid gate
        res[*y] += i_gz[*z] * i_uz[*x];
    }

    res
}
