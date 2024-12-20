use crate::pipeline::Pipeline;
use pyo3::prelude::*;


// TODO!


macro_rules! create_interface {
    ($name: ident, $type: ident) => {
        #[pyclass]
        pub struct $name {
            inner: Pipeline<$type>,
        }
        #[pymethods]
        impl $name {
            #[new]
            pub fn new() -> Self {
                Self {
                    inner: Pipeline::default(),
                }
            }
        }
    };
}

create_interface!(StringPipeline, String);

#[pymodule]
fn fluxion(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<StringPipeline>()?;
    Ok(())
}