use using_dummy_variables_in_econometric_analysis_of_public_policy_core::ols_coefficients;

fn main() {
    let n = 500usize;
    let p = 4usize;
    let x: Vec<f64> = (0..n * p).map(|i| (i as f64 * 0.01).sin()).collect();
    let y: Vec<f64> = (0..n).map(|i| (i as f64 * 0.02).cos() + 1.0).collect();
    for _ in 0..2000 {
        let _ = ols_coefficients(&x, &y, n, p);
    }
}
