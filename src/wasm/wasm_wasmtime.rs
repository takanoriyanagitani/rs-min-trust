use crate::{err::Error, wasm::common_wasmtime::transform_new_with_untrusted};

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
    transform_new_with_untrusted(
        untrusted,
        wasm_bytes,
        mem_name,
        input_offset_getter_name,
        output_offset_getter_name,
        main_name,
        (65536, 65536),
        [0u8; 65536],
    )
}

pub fn transform_new_with_untrusted15<U>(
    untrusted: U,
    wasm_bytes: &[u8],
    mem_name: &str,
    input_offset_getter_name: &str,
    output_offset_getter_name: &str,
    main_name: &str,
) -> Result<impl FnMut([u8; 32768], [u8; 32768]) -> Result<[u8; 32768], Error>, Error>
where
    U: FnMut([u8; 32768]) -> Result<[u8; 32768], Error>,
{
    transform_new_with_untrusted(
        untrusted,
        wasm_bytes,
        mem_name,
        input_offset_getter_name,
        output_offset_getter_name,
        main_name,
        (32768, 32768),
        [0u8; 32768],
    )
}
