"""OLS coefficients via normal equations (X'X)^-1 X'y — small p only."""

from __future__ import annotations

import numpy as np


def _gauss_solve(a: list[float], b: list[float], n: int) -> list[float]:
    aug = [0.0] * (n * (n + 1))
    for i in range(n):
        for j in range(n):
            aug[i * (n + 1) + j] = a[i * n + j]
        aug[i * (n + 1) + n] = b[i]
    for col in range(n):
        pivot = col
        for row in range(col + 1, n):
            if abs(aug[row * (n + 1) + col]) > abs(aug[pivot * (n + 1) + col]):
                pivot = row
        for k in range(n + 1):
            aug[col * (n + 1) + k], aug[pivot * (n + 1) + k] = (
                aug[pivot * (n + 1) + k],
                aug[col * (n + 1) + k],
            )
        div = aug[col * (n + 1) + col]
        if abs(div) < 1e-18:
            continue
        for k in range(n + 1):
            aug[col * (n + 1) + k] /= div
        for row in range(n):
            if row == col:
                continue
            factor = aug[row * (n + 1) + col]
            for k in range(n + 1):
                aug[row * (n + 1) + k] -= factor * aug[col * (n + 1) + k]
    return [aug[i * (n + 1) + n] for i in range(n)]


def ols_coefficients(x: np.ndarray, y: np.ndarray, n: int, p: int) -> np.ndarray:
    x_arr = np.asarray(x, dtype=float)
    y_arr = np.asarray(y, dtype=float)
    xtx = [0.0] * (p * p)
    xty = [0.0] * p
    for i in range(n):
        for a in range(p):
            xa = x_arr[i * p + a]
            xty[a] += xa * y_arr[i]
            for b in range(p):
                xtx[a * p + b] += xa * x_arr[i * p + b]
    coefs = _gauss_solve(xtx, xty, p)
    return np.asarray(coefs, dtype=float)
