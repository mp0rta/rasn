use pyo3::prelude::*;
use pyo3::types::PyAny;

pub(crate) fn to_vec(obj: &Bound<'_, PyAny>) -> PyResult<Vec<u8>> {
    obj.extract::<Vec<u8>>().map_err(|_| {
        pyo3::exceptions::PyTypeError::new_err(
            "expected bytes-like (bytes, bytearray, or memoryview)",
        )
    })
}
