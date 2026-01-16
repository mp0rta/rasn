use pyo3::create_exception;
use pyo3::exceptions::PyValueError;

// Errors exposed to Python.
create_exception!(rasn_py, RasnCodecError, PyValueError);
create_exception!(rasn_py, RasnEncodeError, PyValueError);
create_exception!(rasn_py, RasnDecodeError, PyValueError);
