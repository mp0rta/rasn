use pyo3::prelude::*;
use pyo3::types::PyType;

/// Python-facing codec selector.
///
/// This is a thin wrapper around `rasn::Codec`.
/// We intentionally don't expose `rasn::Codec` across the PyO3 boundary.
#[pyclass(frozen)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Codec(pub(crate) rasn::Codec);

impl Codec {
    /// Rust-only accessor used by internal dispatch/macro-generated code.
    pub(crate) fn inner(&self) -> rasn::Codec {
        self.0
    }
}

// move Codec
impl From<Codec> for rasn::Codec {
    fn from(c: Codec) -> Self {
        c.0
    }
}

#[pymethods]
impl Codec {
    #[classmethod]
    fn aper(_cls: &Bound<'_, PyType>) -> Self {
        Self(rasn::Codec::Aper)
    }
    #[classmethod]
    fn ber(_cls: &Bound<'_, PyType>) -> Self {
        Self(rasn::Codec::Ber)
    }
    #[classmethod]
    fn cer(_cls: &Bound<'_, PyType>) -> Self {
        Self(rasn::Codec::Cer)
    }
    #[classmethod]
    fn der(_cls: &Bound<'_, PyType>) -> Self {
        Self(rasn::Codec::Der)
    }
    #[classmethod]
    fn uper(_cls: &Bound<'_, PyType>) -> Self {
        Self(rasn::Codec::Uper)
    }
    #[classmethod]
    fn jer(_cls: &Bound<'_, PyType>) -> Self {
        Self(rasn::Codec::Jer)
    }
    #[classmethod]
    fn oer(_cls: &Bound<'_, PyType>) -> Self {
        Self(rasn::Codec::Oer)
    }
    #[classmethod]
    fn coer(_cls: &Bound<'_, PyType>) -> Self {
        Self(rasn::Codec::Coer)
    }
    #[classmethod]
    fn xer(_cls: &Bound<'_, PyType>) -> Self {
        Self(rasn::Codec::Xer)
    }

    fn __repr__(&self) -> String {
        format!("Codec({:?})", self.0)
    }
}
