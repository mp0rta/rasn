use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(RasnPy)]
pub fn derive_rasn_py(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    let expanded = quote! {
        #[pyo3::pymethods]
        impl #ident {
            #[new]
            pub fn new(a: i64, b: bool, payload: Vec<u8>) -> Self {
                Self { a, b, payload }
            }

            pub fn encode<'py>(
                &self,
                py: pyo3::Python<'py>,
                codec: crate::codec::Codec,
            ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::types::PyBytes>> {
                let bytes = crate::py_dispatch::encode_bytes_py(self, codec)?;
                Ok(pyo3::types::PyBytes::new(py, &bytes))
            }


            #[classmethod]
            pub fn decode(
                _cls: &pyo3::Bound<'_, pyo3::types::PyType>,
                _py: pyo3::Python,
                data: &pyo3::Bound<'_, pyo3::types::PyAny>,
                codec: crate::codec::Codec,
            ) -> pyo3::PyResult<Self> {
                let bytes = crate::bytes_like::to_vec(data)?;
                crate::py_dispatch::decode_bytes_py::<Self>(&bytes, codec)
            }

        }
    };

    TokenStream::from(expanded)
}
