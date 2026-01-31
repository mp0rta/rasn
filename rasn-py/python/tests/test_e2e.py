# python/tests/test_e2e.py
import pytest

import rasn_py
from rasn_py import Codec, RasnDecodeError
from rasn_py import E2eType

def test_der_roundtrip():
    t = E2eType(42, True, b"hello")
    b = t.encode(Codec.der())
    t2 = E2eType.decode(b, Codec.der())
    assert t2.a == 42
    assert t2.b is True
    assert bytes(t2.payload) == b"hello"

def test_decode_rejects_garbage():
    with pytest.raises(RasnDecodeError):
        E2eType.decode(b"\x00", Codec.der())

def test_decode_accepts_bytearray():
    t = E2eType(1, False, b"\x01\x02")
    b = t.encode(Codec.der())
    t2 = E2eType.decode(bytearray(b), Codec.der())
    assert t2.a == 1
    assert t2.b is False
    assert bytes(t2.payload) == b"\x01\x02"
