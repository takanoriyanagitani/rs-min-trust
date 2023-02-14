use wasmtime::{Engine, Instance, Linker, Memory, Module, Store, TypedFunc};

use crate::{err::Error, transform_new};

pub fn transform_trusted_new16(
    wasm_bytes: &[u8],
    mem_name: &str,
    input_offset_getter_name: &str,
    output_offset_getter_name: &str,
    main_name: &str,
) -> Result<impl FnMut([u8; 65536], [u8; 65536]) -> Result<[u8; 65536], Error>, Error> {
    let e: Engine = Engine::default();
    let m: Module = Module::new(&e, wasm_bytes)
        .map_err(|e| Error::UnableToParseWasm(format!("Unable to create a module: {e}")))?;
    let mut s: Store<()> = Store::new(&e, ());
    let l: Linker<_> = Linker::new(&e);
    let i: Instance = l
        .instantiate(&mut s, &m)
        .map_err(|e| Error::UnableToCreateInstance(format!("Link failure: {e}")))?;
    let mem: Memory = i
        .get_memory(&mut s, mem_name)
        .ok_or_else(|| Error::MemoryNotFound(format!("Unable to get a memory(name={mem_name})")))?;
    let input_offset_getter: TypedFunc<(), i32> = i
        .get_typed_func::<(), i32>(&mut s, input_offset_getter_name)
        .map_err(|e| Error::OffsetFuncMissing(format!("Unable to get a offset func: {e}")))?;
    let output_offset_getter: TypedFunc<(), i32> = i
        .get_typed_func::<(), i32>(&mut s, output_offset_getter_name)
        .map_err(|e| Error::OffsetFuncMissing(format!("Unable to get a offset func: {e}")))?;
    let main_program: TypedFunc<(i32, i32, i32), i64> = i
        .get_typed_func::<(i32, i32, i32), i64>(&mut s, main_name)
        .map_err(|e| Error::MainFuncMissing(format!("Unable to get a main func: {e}")))?;
    let input_offset: i32 = input_offset_getter
        .call(&mut s, ())
        .map_err(|e| Error::UnableToGetOffset(format!("Unable to get offset: {e}")))?;
    let input_offset4untrusted: i32 = input_offset + 65536;
    let output_offset: i32 = output_offset_getter
        .call(&mut s, ())
        .map_err(|e| Error::UnableToGetOffset(format!("Unable to get offset: {e}")))?;
    let io_usz: usize = input_offset
        .try_into()
        .map_err(|e| Error::InvalidInputOffset(format!("Invalid offset({input_offset}): {e}")))?;
    let oo_usz: usize = output_offset
        .try_into()
        .map_err(|e| Error::InvalidOutputOffset(format!("Invalid offset({output_offset}):{e}")))?;
    let mut buf = [0u8; 65536];
    Ok(
        move |input2trusted: [u8; 65536], output_from_untrusted: [u8; 65536]| {
            let o_input2trusted: usize = io_usz;
            let o_output_from_untrusted: usize = io_usz + 65536;
            mem.write(&mut s, o_input2trusted, &input2trusted)
                .map_err(|e| {
                    Error::UnableToCopyInputBytes(format!("writes to memory failed: {e}"))
                })?;
            mem.write(&mut s, o_output_from_untrusted, &output_from_untrusted)
                .map_err(|e| {
                    Error::UnableToCopyInputBytes(format!("writes to memory failed: {e}"))
                })?;
            match main_program.call(
                &mut s,
                (input_offset, input_offset4untrusted, output_offset),
            ) {
                Ok(0) => Ok(()),
                Ok(i) => Err(Error::TrustedFuncError(format!(
                    "Main func non-0 exit: {i}"
                ))),
                Err(e) => Err(Error::MainFuncMisbehave(format!("Unexpected error: {e}"))),
            }?;
            mem.read(&mut s, oo_usz, &mut buf)
                .map_err(|e| Error::UnableToGetOutputFromTrusted(format!("Unable to read: {e}")))?;
            Ok(buf)
        },
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
