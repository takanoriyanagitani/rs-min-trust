use wasmer::{
    Cranelift, Function, Imports, Instance, Memory, MemoryView, Module, Store, TypedFunction,
    WasmTypeList,
};

use crate::err::Error;

fn store_new() -> Store {
    Store::new(Cranelift::default())
}

fn wasm_bytes2module(bytes: &[u8], s: &Store) -> Result<Module, Error> {
    Module::new(s, bytes)
        .map_err(|e| Error::UnableToParseWasm(format!("Unable to create module(wasmer): {e}")))
}

fn imports_new_empty() -> Imports {
    Imports::new()
}

fn module2instance(m: &Module, s: &mut Store, i: &Imports) -> Result<Instance, Error> {
    Instance::new(s, m, i).map_err(|e| {
        Error::UnableToCreateInstance(format!("Unable to create an instance(wasmer): {e}"))
    })
}

fn instance2typed_func<I, O>(
    i: &Instance,
    s: &Store,
    name: &str,
) -> Result<TypedFunction<I, O>, Error>
where
    I: WasmTypeList,
    O: WasmTypeList,
{
    let f: &Function = i
        .exports
        .get_function(name)
        .map_err(|e| Error::UnableToGetFunc(format!("Unable to get a func(wasmer): {e}")))?;
    f.typed(s)
        .map_err(|e| Error::InvalidFunction(format!("Invalid function(wasmer): {e}")))
}

fn wasm2host(m: &MemoryView, offset: u64, buf: &mut [u8]) -> Result<(), Error> {
    m.read(offset, buf)
        .map_err(|e| Error::UnableToGetBytesFromWasm(format!("Unable to read bytes(wasmer): {e}")))
}

fn host2wasm(m: &MemoryView, offset: u64, data: &[u8]) -> Result<(), Error> {
    m.write(offset, data)
        .map_err(|e| Error::UnableToCopyBytesToWasm(format!("Unable to write bytes(wasmer): {e}")))
}
