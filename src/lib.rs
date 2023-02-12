pub mod err;

#[cfg(feature = "wasm_wasmtime")]
pub mod wasm;

pub fn transform_new<T, U, I, J, M, O, E>(
    mut trusted: T,
    mut untrusted: U,
) -> impl FnMut(I, J) -> Result<O, E>
where
    U: FnMut(J) -> Result<M, E>,
    T: FnMut(I, M) -> Result<O, E>,
{
    move |input2trusted: I, input2untrusted: J| {
        let result_from_untrusted: M = untrusted(input2untrusted)?;
        trusted(input2trusted, result_from_untrusted)
    }
}
