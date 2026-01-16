use rasn::Codec as RasnCodec;

/// Core errors independent from PyO3.
#[derive(Debug)]
pub enum CoreError {
    /// Input was expected to be UTF-8 when decoding a text codec (JER/XER).
    InvalidUtf8(std::str::Utf8Error),
    /// `rasn` encode error.
    Encode(rasn::error::EncodeError),
    /// `rasn` decode error.
    Decode(rasn::error::DecodeError),
}

impl std::fmt::Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreError::InvalidUtf8(e) => write!(f, "invalid utf-8 input for text codec: {e}"),
            CoreError::Encode(e) => write!(f, "{e}"),
            CoreError::Decode(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for CoreError {}

const fn is_text_codec(c: RasnCodec) -> bool {
    matches!(c, RasnCodec::Jer | RasnCodec::Xer)
}

/// Encode `value` using `codec` and always return bytes.
///
/// For text codecs (JER/XER), the output is UTF-8 bytes.
pub fn encode_bytes<T: rasn::Encode>(codec: RasnCodec, value: &T) -> Result<Vec<u8>, CoreError> {
    if is_text_codec(codec) {
        return codec
            .encode_to_string(value)
            .map(|s| s.into_bytes())
            .map_err(CoreError::Encode);
    }

    codec.encode_to_binary(value).map_err(CoreError::Encode)
}

/// Decode bytes into `T` using `codec`.
///
/// For text codecs (JER/XER), `input` must be valid UTF-8.
pub fn decode_bytes<T: rasn::Decode>(codec: RasnCodec, input: &[u8]) -> Result<T, CoreError> {
    if is_text_codec(codec) {
        let s = std::str::from_utf8(input).map_err(CoreError::InvalidUtf8)?;
        return codec.decode_from_str(s).map_err(CoreError::Decode);
    }

    codec.decode_from_binary(input).map_err(CoreError::Decode)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rasn::prelude::*;

    #[derive(AsnType, Decode, Encode, Debug, PartialEq, Eq)]
    struct Mini {
        a: u64,
        b: Utf8String,
    }

    #[test]
    fn roundtrip_der_ok() {
        let v = Mini {
            a: 42,
            b: "hello".into(),
        };
        let bytes = encode_bytes(RasnCodec::Der, &v).unwrap();
        let out: Mini = decode_bytes(RasnCodec::Der, &bytes).unwrap();
        assert_eq!(v, out);
    }

    #[test]
    fn decode_der_rejects_garbage() {
        let garbage = [0xff, 0x00, 0x01, 0x02];
        let r: Result<Mini, _> = decode_bytes(RasnCodec::Der, &garbage);
        assert!(matches!(r, Err(CoreError::Decode(_))));
    }

    #[test]
    fn decode_jer_requires_valid_utf8_bytes() {
        let not_utf8 = [0xff, 0xfe, 0xfd];
        let r: Result<Mini, _> = decode_bytes(RasnCodec::Jer, &not_utf8);
        assert!(matches!(r, Err(CoreError::InvalidUtf8(_))));
    }
}
