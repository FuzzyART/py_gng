from setuptools import setup, find_packages

setup(
    name="neurogas_py",
    version="0.1.0",
    author="Your Name",
    author_email="your.email@example.com",
    description="Jupyter-compatible Python wrapper for the Rust-based GNG algorithm",
    long_description=open("README.md").read(),
    long_description_content_type="text/markdown",
    url="https://github.com/yourusername/gng-jupyter",
    packages=find_packages(),
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
    python_requires=">=3.7",
    install_requires=[
        "numpy",
    ],
    setup_requires=[
        "setuptools",
        "wheel",
        "pyo3-pack",
    ],
    package_data={
        "neurogas_py": ["*.so", "*.pyd", "*.dylib", "*.dll"],
    },
    zip_safe=False,
)
