pub fn approx_eq(lhs: f64, rhs: f64) -> bool {
    (rhs - lhs).abs() < 0.000_000_000_001
}
