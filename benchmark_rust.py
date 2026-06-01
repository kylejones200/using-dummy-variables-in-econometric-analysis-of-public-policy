#!/usr/bin/env python3
"""Python vs Rust kernel benchmark."""

from __future__ import annotations

import time
import sys
from pathlib import Path

import numpy as np

ROOT = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT / "src"))
from compute_kernel import ols_coefficients  # noqa: E402

def main() -> None:
    n, p = 500, 4
    x = np.ascontiguousarray(np.sin(np.arange(n * p) * 0.01))
    y = np.ascontiguousarray(np.cos(np.arange(n) * 0.02) + 1.0)
    t0 = time.perf_counter()
    for _ in range(200):
        ols_coefficients(x, y, n, p)
    py_s = time.perf_counter() - t0
    try:
        import using_dummy_variables_in_econometric_analysis_of_public_policy_rs as rs
    except ImportError:
        print("Build: maturin develop --release -m rust/py/Cargo.toml")
        print(f"Python {py_s:.3f}s")
        return
    rs_s = rs.bench_kernel_py(x, y, n, p, 2000)
    print(f"Python {py_s:.3f}s Rust {rs_s:.3f}s speedup {py_s / max(rs_s, 1e-9):.1f}x")
    np.testing.assert_allclose(
        ols_coefficients(x, y, n, p),
        np.asarray(rs.ols_coefficients_py(x, y, n, p)),
        rtol=1e-10,
    )
    print("Correctness: OK")

if __name__ == "__main__":
    main()
