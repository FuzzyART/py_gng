#![allow(warnings)]
use numpy::PyArray1;
use numpy::PyArrayMethods;
use pyo3::prelude::*;

pub mod internal {
    // Re-export the original types and functions from neurogas crate
    pub use neurogas::gas::core::{
        fit as core_fit, get_edges as core_get_edges, get_model_string as core_get_model_string,
        get_neurons as core_get_neurons, init_dataset as core_init_dataset,
        init_dataset_vec as core_init_dataset_vec, load_config as core_load_config,
        save_model_json as core_save_model_json, set_input_width as core_set_input_width,
        set_parameters as core_set_parameters, Handler,
    };
}

pub struct Context {
    cont_params: internal::Handler,
}

impl Context {
    pub fn new() -> Self {
        Context {
            cont_params: internal::Handler::init(),
        }
    }

    pub fn create_system(&mut self) {
        self.cont_params.create_system();
    }

    pub fn load_config(&mut self, filename_config: &str) {
        // If core expects &String, convert from &str
        internal::core_load_config(&mut self.cont_params, &filename_config.to_string());
    }

    pub fn init_dataset_vec(&mut self, data: &Vec<f64>) {
        internal::core_init_dataset_vec(&mut self.cont_params, data);
    }

    pub fn init_dataset(&mut self, filename_dataset: &str) {
        internal::core_init_dataset(&mut self.cont_params, &filename_dataset.to_string());
    }

    pub fn fit(&mut self) {
        internal::core_fit(&mut self.cont_params);
    }

    pub fn save_model_json(&mut self, filename_output: &str) {
        internal::core_save_model_json(&mut self.cont_params, filename_output.to_string());
    }

    pub fn get_model_string(&mut self) -> String {
        internal::core_get_model_string(&mut self.cont_params)
    }

    pub fn get_neurons(&mut self) -> Vec<(usize, Vec<f64>)> {
        internal::core_get_neurons(&mut self.cont_params)
    }

    pub fn get_edges(&mut self) -> Vec<(usize, usize)> {
        internal::core_get_edges(&mut self.cont_params)
    }

    pub fn set_input_width(&mut self, input_width: usize) {
        internal::core_set_input_width(&mut self.cont_params, input_width);
    }

    pub fn foo(&mut self) -> String {
        "hello int".to_string()
    }

    pub fn foo_vec(&mut self) -> (Vec<usize>, Vec<f64>) {
        let res1: Vec<usize> = vec![1, 3];
        let res2: Vec<f64> = vec![1.0, 2.2];
        (res1, res2)
    }

    pub fn set_parameters(
        &mut self,
        input_width: usize,
        weight_rng_min: f64,
        weight_rng_max: f64,
        edge_removal_age: usize,
        neuron_creation_interval: usize,
        max_epochs: usize,
        max_neurons: usize,
        target_error: f64,
        epsilon_w: f64,
        epsilon_n: f64,
        alpha: f64,
        beta: f64,
    ) {
        internal::core_set_parameters(
            &mut self.cont_params,
            input_width,
            weight_rng_min,
            weight_rng_max,
            edge_removal_age,
            neuron_creation_interval,
            max_epochs,
            max_neurons,
            target_error,
            epsilon_w,
            epsilon_n,
            alpha,
            beta,
        );
    }
}

/// Python wrapper for Context struct
#[pyclass]
pub struct PyContext {
    context: Context,
}

#[pymethods]
impl PyContext {
    #[new]
    fn new() -> Self {
        PyContext {
            context: Context::new(),
        }
    }

    pub fn set_parameters(
        &mut self,
        input_width: usize,
        weight_rng_min: f64,
        weight_rng_max: f64,
        edge_removal_age: usize,
        neuron_creation_interval: usize,
        max_epochs: usize,
        max_neurons: usize,
        target_error: f64,
        epsilon_w: f64,
        epsilon_n: f64,
        alpha: f64,
        beta: f64,
    ) {
        self.context.set_parameters(
            input_width,
            weight_rng_min,
            weight_rng_max,
            edge_removal_age,
            neuron_creation_interval,
            max_epochs,
            max_neurons,
            target_error,
            epsilon_w,
            epsilon_n,
            alpha,
            beta,
        );
    }

    fn create_system(&mut self) {
        self.context.create_system();
    }

    fn load_config(&mut self, filename_config: &str) -> PyResult<()> {
        self.context.load_config(filename_config);
        Ok(())
    }

    fn init_dataset_vec(
        &mut self,
        _py: Python<'_>,
        dataset: Bound<'_, PyArray1<f64>>,
    ) -> PyResult<()> {
        let slice = unsafe { dataset.as_slice()? };
        let vec_data: Vec<f64> = slice.to_vec();
        self.context.init_dataset_vec(&vec_data);
        Ok(())
    }

    fn init_dataset(&mut self, filename_dataset: &str) -> PyResult<()> {
        self.context.init_dataset(filename_dataset);
        Ok(())
    }

    fn fit(&mut self) -> PyResult<()> {
        self.context.fit();
        Ok(())
    }

    fn foo(&mut self) -> PyResult<String> {
        Ok(self.context.foo())
    }

    fn foo_val(&mut self) -> PyResult<f64> {
        Ok(6.4)
    }

    fn foo_vec(&mut self) -> PyResult<(Vec<usize>, Vec<f64>)> {
        Ok(self.context.foo_vec())
    }

    fn save_model_json(&mut self, filename_output: &str) -> PyResult<()> {
        self.context.save_model_json(filename_output);
        Ok(())
    }

    fn get_model_string(&mut self) -> PyResult<String> {
        Ok(self.context.get_model_string())
    }

    fn get_neurons(&mut self) -> PyResult<Vec<(usize, Vec<f64>)>> {
        Ok(self.context.get_neurons())
    }

    fn get_edges(&mut self) -> PyResult<Vec<(usize, usize)>> {
        Ok(self.context.get_edges())
    }
}

/// Define the Python module - renamed to match your package name
#[pymodule]
mod neurogas {
    #[pymodule_export]
    use super::PyContext;
}
