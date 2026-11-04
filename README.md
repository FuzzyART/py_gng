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

The Python wrapper is exposed as the `neurogas` module, and the main context type is `PyContext`.

```python
import numpy as np
import neurogas as gng_py

# Example dataset: two 2D clusters
points = np.array([
    [0.0, 0.0], [0.1, 0.1], [-0.2, -0.1],
    [5.0, 5.0], [5.1, 4.9], [4.8, 5.2],
], dtype=float)

ctx = gng_py.PyContext()
ctx.create_system()
ctx.set_parameters(
    input_width=2,
    weight_rng_min=-1.1,
    weight_rng_max=1.1,
    edge_removal_age=50,
    neuron_creation_interval=200,
    max_epochs=20,
    max_neurons=40,
    target_error=0.096,
    epsilon_w=0.1,
    epsilon_n=0.006,
    alpha=0.5,
    beta=0.995,
)

ctx.init_dataset_vec(points.reshape(-1))
ctx.fit()

neurons = ctx.get_neurons()
edges = ctx.get_edges()
print(neurons)
print(edges)
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
