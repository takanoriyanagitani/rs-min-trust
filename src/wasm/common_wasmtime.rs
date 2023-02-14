use wasmtime::{Engine, Instance, Linker, Memory, Module, Store, TypedFunc};

use crate::{err::Error, transform_new};

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
