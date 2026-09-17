# GNG Jupyter Project

This project provides a Jupyter-compatible Python wrapper around the Rust-based Growing Neural Gas (GNG) algorithm implementation.

## Overview

This project is a Python wrapper that depends on the `neurogas` crate (available on crates.io). It provides a convenient interface for using the GNG algorithm in Jupyter notebooks and Python environments.

## Features

- Python bindings for the GNG algorithm
- Jupyter notebook compatible
- Easy integration with existing Python data science workflows
- Full access to all GNG algorithm parameters and functionality

## Installation

To install this package, you can use pip:

```bash
pip install .
```

Or for development:

```bash
pip install -e .
```

## Usage

```python
from gng_jupyter import PyContext

# Create a new GNG context
context = PyContext()

# Set parameters
context.set_parameters(
    input_width=2,
    weight_rng_min=0.0,
    weight_rng_max=1.0,
    edge_removal_age=100,
    neuron_creation_interval=1000,
    max_train_iterations=10000,
    target_error=0.01,
    epsilon_w=0.05,
    epsilon_n=0.005,
    alpha=0.5,
    beta=0.99
)

# Initialize dataset
context.init_dataset("path/to/dataset.csv")

# Train the model
context.fit()

# Save model
context.save_model_json("model.json")
```

## Dependencies

- [neurogas](https://crates.io/crates/neurogas) crate (version 0.0.2)
- Python 3.7+
- PyO3
- NumPy

## Development
required pip packages for compiling:
- pyo3-pack 
- maturin

required pip packages for the examples:
 - ipykernel
 - install pandas
 - scikit-learn
 - matplotlib


To build this project locally:

```bash
cargo build
```

To install in development mode:

```bash
pip install -e .
```

## License

MIT License
