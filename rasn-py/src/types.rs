// rasn-py/src/types.rs
use pyo3::prelude::*;
use rasn::prelude::*;

use crate::RasnPy;

#[pyclass(name = "E2eType", module = "rasn_py")]
#[derive(AsnType, Encode, Decode, RasnPy, Clone, Debug, PartialEq, Eq)]
pub struct E2eType {
    #[pyo3(get)]
    pub a: i64,
    #[pyo3(get)]
    pub b: bool,
    // for using DER safety like OcterString.
    #[pyo3(get)]
    pub payload: Vec<u8>,
}
