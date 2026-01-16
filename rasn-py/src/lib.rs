use pyo3::prelude::*;
use pyo3::PyErr;

mod codec;
mod codec_core;
mod error;
pub(crate) mod py_dispatch;

use crate::codec::Codec;

#[pymodule]
fn rasn_py(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    // Public API
    m.add_class::<Codec>()?;

    // Export exception types
    m.add("RasnCodecError", py.get_type::<error::RasnCodecError>())?;
    m.add("RasnEncodeError", py.get_type::<error::RasnEncodeError>())?;
    m.add("RasnDecodeError", py.get_type::<error::RasnDecodeError>())?;

    Ok(())
}

