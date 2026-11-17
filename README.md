# GNG Jupyter Project


> ⚠️ Experimental — Not Production Ready
>
> This package is in an **early, unstable state** (v0.0.4).
> The public API, CLI, and behaviour are **not stable** and may change
> — or be removed entirely — in any subsequent release without notice.
> Use at your own risk. Do **not** rely on it in production yet.


This package is a personal portfolio project created for educational and demonstration purposes.

## Overview

This project provides a Jupyter-compatible Python wrapper around the Rust-based Growing Neural Gas (GNG) algorithm implementation.


This project is a Python wrapper that depends on the `neurogas` crate (available on crates.io). It provides a convenient interface for using the GNG algorithm in Jupyter notebooks and Python environments.

## Documentation
**For full documentation, visit:** 

https://fuzzyart.github.io/neurogas_docu/

**Examples:**

 coming soon


## Installation

To install this package, you can use pip:



```bash
pip install neurogas
```


## Usage


```python
import numpy as np

import neurogas as gng_py

if __name__ == "__main__":
    data= np.array(
        [
            [0.0, 0.0], [0.1, 0.1], [-0.2, -0.1],
            [2.0, 5.0], [2.1, 4.9], [1.8, 5.2],
        ],

    )

    gng = gng_py.Gng()

    gng.set_parameters(
        input_width=2,
        weight_rng_min=-1.1,
        weight_rng_max=1.1,
        edge_removal_age=50,
        neuron_creation_interval=200,
        max_epochs=100,
        max_neurons=10,
        target_error=0.096,
        epsilon_w=0.1,
        epsilon_n=0.006,
        alpha=0.5,
        beta=0.995,
        rng_seed=123,
    )

    gng.init_dataset_vec(data.flatten())
    gng.fit()

    neurons = gng.get_neurons()
    edges = gng.get_edges()

    print(f"Number of neurons: {len(neurons)}")
    print(f"Number of edges: {len(edges)}")
    print("###NEURONS####")
    print("(Nr., [x, y])") # neuron id and coordinates of each neuron in the output
    print(neurons)
    print("###EDGES###")
    print("(start_id, end_id)")  # start and end id of each edge in the outpu
    print(edges)
```


## Dependencies

- [neurogas](https://crates.io/crates/neurogas) crate (version 0.0.4)
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

## 🛠 Building from Source
If you want to build or modify the package locally, follow these steps to set up your environment and compile the Rust extensions using maturin.

### 1. Prerequisites
Ensure you have the Rust toolchain and Python installed. You will also need maturin for building Python packages from Rust.

### 2. Setup the Virtual Environment
Create and activate a virtual environment, then upgrade your packaging tools:

```bash
# Create and activate virtual environment
python -m venv venv
source venv/bin/activate

# Upgrade pip and packaging tools
python -m pip install --upgrade pip
python -m pip install --upgrade twine maturin
```

### 3. Configure Rust Targets (Optional for Cross-Compilation)
If you are compiling for your native host architecture, you can skip adding targets. If you need to cross-compile for Windows (MSVC) from another OS, add the target:

```Bash
# Add the Windows MSVC target
rustup target add x86_64-pc-windows-msvc


# Verify installed targets
rustup target list --installed
```
### 4. Build the Package
You can build release wheels using maturin:

- For your native host architecture:

```bash
maturin build --release
```

- For Windows targets (cross-compilation):

```bash
maturin build --release --target x86_64-pc-windows-msvc
```










## Changelog
- 0.0.5: Added Windows wheel

## License

MIT License
