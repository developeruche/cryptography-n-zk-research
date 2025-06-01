//! This is an implemention of a linear time sum check protocol.
//! as discribed in the Libra paper.

/// A trait for a linear time sum check protocol.
pub trait LinearTimeSumCheck {
    fn phase_one();
    fn phase_two();
    fn sum_check();
}
