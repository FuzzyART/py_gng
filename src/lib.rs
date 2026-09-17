#![allow(warnings)]
use numpy::PyArray1;
use numpy::PyArrayMethods;
use pyo3::prelude::*;

pub mod internal {
    pub use neurogas::gas::core::{
        fit as core_fit, get_edges as core_get_edges,
        get_model_string as core_get_model_string,
        get_neurons as core_get_neurons, init_dataset as core_init_dataset,
        init_dataset_vec as core_init_dataset_vec, load_config as core_load_config,
        save_model_json as core_save_model_json, set_input_width as core_set_input_width,
        set_parameters as core_set_parameters, Handler,
    };
}

#[pyclass]
pub struct PyContext {
    handler: internal::Handler,
}

impl PyContext {
    fn new_handler() -> Self {
        PyContext {
            handler: internal::Handler::init(),
        }
    }
}

#[pymethods]
impl PyContext {
    #[new]
    fn __new__() -> Self {
        PyContext::new_handler()
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
            &mut self.handler, input_width, weight_rng_min, weight_rng_max,
            edge_removal_age, neuron_creation_interval, max_epochs, max_neurons,
            target_error, epsilon_w, epsilon_n, alpha, beta,
        );
    }

    fn create_system(&mut self) {
        self.handler.create_system();
    }

    fn load_config(&mut self, filename_config: &str) -> PyResult<()> {
        internal::core_load_config(&mut self.handler, &filename_config.to_string());
        Ok(())
    }

    fn init_dataset_vec(
        &mut self, _py: Python<'_>, dataset: Bound<'_, PyArray1<f64>>,
    ) -> PyResult<()> {
        let slice = unsafe { dataset.as_slice()? };
        internal::core_init_dataset_vec(&mut self.handler, &slice.to_vec());
        Ok(())
    }

    fn init_dataset(&mut self, filename_dataset: &str) -> PyResult<()> {
        internal::core_init_dataset(&mut self.handler, &filename_dataset.to_string());
        Ok(())
    }

    fn fit(&mut self) -> PyResult<()> {
        internal::core_fit(&mut self.handler);
        Ok(())
    }

    fn foo(&self) -> String { "hello int".to_string() }
    fn foo_val(&self) -> f64 { 6.4 }
    fn foo_vec(&self) -> (Vec<usize>, Vec<f64>) {
        (vec![1, 3], vec![1.0, 2.2])
    }

    fn save_model_json(&mut self, filename_output: &str) -> PyResult<()> {
        internal::core_save_model_json(&mut self.handler, filename_output.to_string());
        Ok(())
    }

    fn get_model_string(&mut self) -> String {
        internal::core_get_model_string(&mut self.handler)
    }

    fn get_neurons(&mut self) -> Vec<(usize, Vec<f64>)> {
        internal::core_get_neurons(&mut self.handler)
    }

    fn get_edges(&mut self) -> Vec<(usize, usize)> {
        internal::core_get_edges(&mut self.handler)
    }

    fn set_input_width(&mut self, input_width: usize) {
        internal::core_set_input_width(&mut self.handler, input_width);
    }
}

#[pymodule]
mod neurogas {
    #[pymodule_export]
    use super::PyContext;
}

