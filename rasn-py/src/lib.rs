use pyo3::prelude::*;

pub mod codec;
pub mod py_dispatch;
pub mod bytes_like;
pub mod types;
mod codec_core;
mod error;

use crate::codec::Codec;

pub use rasn_py_macros::RasnPy;

#[pymodule]
fn rasn_py(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    // Public API
    m.add_class::<Codec>()?;
    m.add_class::<types::E2eType>()?; 

    // Export exception types
    m.add("RasnCodecError", py.get_type::<error::RasnCodecError>())?;
    m.add("RasnEncodeError", py.get_type::<error::RasnEncodeError>())?;
    m.add("RasnDecodeError", py.get_type::<error::RasnDecodeError>())?;

    Ok(())
}

