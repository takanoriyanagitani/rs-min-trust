use wasmtime::{Engine, Instance, Linker, Memory, Module, Store, TypedFunc};

use crate::{err::Error, transform_new, wasm::common_wasmtime::transform_trusted_new_generic};

pub fn transform_trusted_new16(
    wasm_bytes: &[u8],
    mem_name: &str,
    input_offset_getter_name: &str,
    output_offset_getter_name: &str,
    main_name: &str,
) -> Result<impl FnMut([u8; 65536], [u8; 65536]) -> Result<[u8; 65536], Error>, Error> {
    transform_trusted_new_generic(
        wasm_bytes,
        mem_name,
        input_offset_getter_name,
        output_offset_getter_name,
        main_name,
        (65536, 65536),
        [0u8; 65536],
    )
}

pub fn transform_new_with_untrusted16<U>(
    untrusted: U,
    wasm_bytes: &[u8],
    mem_name: &str,
    input_offset_getter_name: &str,
    output_offset_getter_name: &str,
    main_name: &str,
) -> Result<impl FnMut([u8; 65536], [u8; 65536]) -> Result<[u8; 65536], Error>, Error>
where
    U: FnMut([u8; 65536]) -> Result<[u8; 65536], Error>,
{
    Ok(transform_new(
        transform_trusted_new16(
            wasm_bytes,
            mem_name,
            input_offset_getter_name,
            output_offset_getter_name,
            main_name,
        )?,
        untrusted,
    ))
}
