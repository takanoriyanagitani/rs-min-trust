use wasmtime::{Engine, Instance, Linker, Memory, Module, Store, TypedFunc};

use crate::err::Error;

pub fn wasm2module(e: &Engine, wasm_bytes: &[u8]) -> Result<Module, Error> {
    Module::new(e, wasm_bytes)
        .map_err(|e| Error::UnableToParseWasm(format!("Unable to create a module: {e}")))
}

pub fn module2instance<T>(l: &Linker<T>, s: &mut Store<T>, m: &Module) -> Result<Instance, Error> {
    l.instantiate(s, m)
        .map_err(|e| Error::UnableToCreateInstance(format!("Link failure: {e}")))
}

pub fn mem_find<T>(s: &mut Store<T>, i: &Instance, memname: &str) -> Result<Memory, Error> {
    i.get_memory(s, memname)
        .ok_or_else(|| Error::MemoryNotFound(format!("Unable to get a memory(name={memname})")))
}

pub fn instance2func<T, I, O>(
    i: &Instance,
    s: &mut Store<T>,
    name: &str,
) -> Result<TypedFunc<I, O>, Error>
where
    I: wasmtime::WasmParams,
    O: wasmtime::WasmResults,
{
    i.get_typed_func(s, name)
        .map_err(|e| Error::UnableToGetFunc(format!("Unable to get func(name={name}): {e}")))
}

pub fn call<T, I, O>(s: &mut Store<T>, f: &TypedFunc<I, O>, args: I) -> Result<O, Error>
where
    I: wasmtime::WasmParams,
    O: wasmtime::WasmResults,
{
    f.call(s, args)
        .map_err(|e| Error::UnableToCallFunc(format!("Unable to call func: {e}")))
}

pub fn offset_convert(original: i32) -> Result<usize, Error> {
    original
        .try_into()
        .map_err(|e| Error::InvalidOffset(format!("Unable to convert an offset: {e}")))
}

pub fn host2wasm<T>(m: &Memory, s: &mut Store<T>, offset: usize, data: &[u8]) -> Result<(), Error> {
    m.write(s, offset, data)
        .map_err(|e| Error::UnableToCopyBytes(format!("writes to mem failed: {e}")))
}

pub fn wasm2host<T>(
    m: &Memory,
    s: &mut Store<T>,
    offset: usize,
    dest: &mut [u8],
) -> Result<(), Error> {
    m.read(s, offset, dest)
        .map_err(|e| Error::UnableToCopyFromWasm(format!("Unable to read bytes: {e}")))
}

pub fn transform_trusted_new<T, U>(
    mut s: Store<T>,
    mem: Memory,
    input_offset: (i32, usize),
    output_offset: (i32, usize),
    main: TypedFunc<(i32, i32, i32), i64>,
    mut buf: U,
    size: (i32, usize),
) -> impl FnMut(U, U) -> Result<U, Error>
where
    U: AsMut<[u8]> + Copy,
{
    let input_offset4untrusted: i32 = input_offset.0 + size.0;
    let o_input2trusted: usize = input_offset.1;
    let o_output: usize = output_offset.1;
    let o_output_from_untrusted: usize = o_input2trusted + size.1;
    move |mut input2trusted: U, mut output_from_untrusted: U| {
        host2wasm(&mem, &mut s, o_input2trusted, input2trusted.as_mut())?;
        host2wasm(
            &mem,
            &mut s,
            o_output_from_untrusted,
            output_from_untrusted.as_mut(),
        )?;
        match main.call(
            &mut s,
            (input_offset.0, input_offset4untrusted, output_offset.0),
        ) {
            Ok(0) => Ok(()),
            Ok(i) => Err(Error::TrustedFuncError(format!(
                "Main func non-0 exit: {i}"
            ))),
            Err(e) => Err(Error::MainFuncMisbehave(format!("Unexpected error: {e}"))),
        }?;
        wasm2host(&mem, &mut s, o_output, buf.as_mut())?;
        Ok(buf)
    }
}
