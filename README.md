# Dummy Variables in Econometric Policy Analysis

Published: yes
Medium: [https://medium.com/@kyle-t-jones/using-dummy-variables-in-econometric-analysis-of-public-policy-5faebaa890f0](https://medium.com/@kyle-t-jones/using-dummy-variables-in-econometric-analysis-of-public-policy-5faebaa890f0)


This project demonstrates using dummy variables to analyze policy effects in econometric models.

## Business context

Public policy decisions often involve categorical variables, such as geographic regions, political affiliations, industry sectors, or regulatory statuses. These variables influence economic, environmental, and political outcomes, guiding policymakers in designing effective interventions. However, traditional regression models require numerical inputs, making categorical data challenging to incorporate directly. Dummy variables offer a solution.

Dummy variables, or indicator variables, convert categorical data into numerical form, enabling their inclusion in regression models. They capture differences across groups, estimating the impact of categorical factors on outcomes. Policymakers use dummy variables to compare effects across categories, such as regional economic growth or regulatory effectiveness.

Public policies often target specific regions, industries, or constituencies. Policymakers need to measure these differences systematically. Dummy variables allow them to:

## Project Structure

```
.
├── README.md           # This file
├── main.py            # Main entry point
├── config.yaml        # Configuration file
├── requirements.txt   # Python dependencies
├── src/               # Core functions
│   ├── core.py        # Dummy variable functions
│   └── plotting.py    # Tufte-style plotting utilities
├── tests/             # Unit tests
├── data/              # Data files
├── images/            # Generated plots and figures
├── rust/                   # Rust port (core + PyO3 + CLI bench)
├── benchmark_rust.py       # Python vs Rust benchmark
├── src/compute_kernel.py   # Python/numpy reference kernel
```

## Configuration

Edit `config.yaml` to customize:
- Data source or synthetic generation
- Policy date
- Effect size
- Model options (include trend)
- Output settings

## Dummy Variables

Dummy variables capture:
- Policy interventions: Before/after policy implementation
- Structural breaks: Changes in relationships
- Seasonal effects: Time-based patterns

## Caveats

- By default, generates synthetic data with policy effect.
- Policy date must be within data range.
- Trend inclusion helps control for time effects.

## Rust performance port

Side-by-side **Python vs Rust** implementation of the numeric hot loop — OLS coefficients. Reference PyO3 benchmark: **see `benchmark_rust.py`** on a release build (local machine; run `benchmark_rust.py` to reproduce).

| Path | Role |
|------|------|
| `src/compute_kernel.py` | Python/numpy reference kernel |
| `rust/core/` | Pure Rust library |
| `rust/py/` | PyO3 bindings |
| `rust/bench/` | Standalone CLI benchmark |
| `benchmark_rust.py` | Python vs Rust timing + correctness check |

```bash
# Rust-only CLI benchmark
cd rust && cargo run --release -p using_dummy_variables_in_econometric_analysis_of_public_policy_bench

# Python vs Rust (PyO3)
pip install maturin numpy
maturin develop --release -m rust/py/Cargo.toml
python benchmark_rust.py
```

Python ML training, solvers, and orchestration stay in Python; Rust targets the numeric hot loops. Stochastic generators validate output shapes; deterministic kernels match at tight floating-point tolerance.


## Disclaimer

Educational/demo code only. Not financial, safety, or engineering advice. Use at your own risk. Verify results independently before any production or operational use.

## License

MIT — see [LICENSE](LICENSE).