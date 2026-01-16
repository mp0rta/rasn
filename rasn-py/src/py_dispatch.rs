use pyo3::prelude::*;

use crate::codec::Codec;
use crate::codec_core::{self, CoreError};
use crate::error::{RasnCodecError, RasnDecodeError, RasnEncodeError};

fn core_err_to_pyerr(e: CoreError) -> PyErr {
    match e {
        CoreError::InvalidUtf8(_) => RasnCodecError::new_err(e.to_string()),
        CoreError::Encode(_) => RasnEncodeError::new_err(e.to_string()),
        CoreError::Decode(_) => RasnDecodeError::new_err(e.to_string()),
    }
}

/// For text codecs (JER/XER), input must be valid UTF-8 bytes.
pub(crate) fn decode_bytes_py<T: rasn::Decode>(bytes: &[u8], codec: Codec) -> PyResult<T> {
    codec_core::decode_bytes(codec.inner(), bytes).map_err(core_err_to_pyerr)
}

pub(crate) fn encode_bytes_py<T: rasn::Encode>(value: &T, codec: Codec) -> PyResult<Vec<u8>> {
    codec_core::encode_bytes(codec.inner(), value).map_err(core_err_to_pyerr)
}
