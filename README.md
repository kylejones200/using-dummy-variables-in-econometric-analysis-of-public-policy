# Dummy Variables in Econometric Policy Analysis

Published: yes
Medium: [https://medium.com/@kyle-t-jones/using-dummy-variables-in-econometric-analysis-of-public-policy-5faebaa890f0](https://medium.com/@kyle-t-jones/using-dummy-variables-in-econometric-analysis-of-public-policy-5faebaa890f0)


This project demonstrates using dummy variables to analyze policy effects in econometric models.

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
└── images/            # Generated plots and figures
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

## Disclaimer

Educational/demo code only. Not financial, safety, or engineering advice. Use at your own risk. Verify results independently before any production or operational use.

## License

MIT — see [LICENSE](LICENSE).
