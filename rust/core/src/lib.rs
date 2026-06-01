//! OLS coefficients via normal equations (X'X)^-1 X'y — small p only.

pub fn ols_coefficients(x: &[f64], y: &[f64], n: usize, p: usize) -> Vec<f64> {
    assert_eq!(x.len(), n * p);
    assert_eq!(y.len(), n);
    let mut xtx = vec![0.0; p * p];
    let mut xty = vec![0.0; p];
    for i in 0..n {
        for a in 0..p {
            let xa = x[i * p + a];
            xty[a] += xa * y[i];
            for b in 0..p {
                xtx[a * p + b] += xa * x[i * p + b];
            }
        }
    }
  // Gauss-Jordan for p<=5 typical
    gauss_solve(&mut xtx, &mut xty, p)
}

fn gauss_solve(a: &mut [f64], b: &mut [f64], n: usize) -> Vec<f64> {
    let mut aug = vec![0.0; n * (n + 1)];
    for i in 0..n {
        for j in 0..n {
            aug[i * (n + 1) + j] = a[i * n + j];
        }
        aug[i * (n + 1) + n] = b[i];
    }
    for col in 0..n {
        let mut pivot = col;
        for row in (col + 1)..n {
            if aug[row * (n + 1) + col].abs() > aug[pivot * (n + 1) + col].abs() {
                pivot = row;
            }
        }
        for k in 0..=n {
            aug.swap(col * (n + 1) + k, pivot * (n + 1) + k);
        }
        let div = aug[col * (n + 1) + col];
        if div.abs() < 1e-18 {
            continue;
        }
        for k in 0..=n {
            aug[col * (n + 1) + k] /= div;
        }
        for row in 0..n {
            if row == col {
                continue;
            }
            let factor = aug[row * (n + 1) + col];
            for k in 0..=n {
                aug[row * (n + 1) + k] -= factor * aug[col * (n + 1) + k];
            }
        }
    }
    (0..n).map(|i| aug[i * (n + 1) + n]).collect()
}
