use wasmtime::{Engine, Instance, Linker, Memory, Module, Store, TypedFunc};

use crate::{err::Error, transform_new};

fn wasm2module(e: &Engine, wasm_bytes: &[u8]) -> Result<Module, Error> {
    Module::new(e, wasm_bytes)
        .map_err(|e| Error::UnableToParseWasm(format!("Unable to create a module: {e}")))
}

fn module2instance<T>(l: &Linker<T>, s: &mut Store<T>, m: &Module) -> Result<Instance, Error> {
    l.instantiate(s, m)
        .map_err(|e| Error::UnableToCreateInstance(format!("Link failure: {e}")))
}

fn mem_find<T>(s: &mut Store<T>, i: &Instance, memname: &str) -> Result<Memory, Error> {
    i.get_memory(s, memname)
        .ok_or_else(|| Error::MemoryNotFound(format!("Unable to get a memory(name={memname})")))
}

fn instance2func<T, I, O>(
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

fn call<T, I, O>(s: &mut Store<T>, f: &TypedFunc<I, O>, args: I) -> Result<O, Error>
where
    I: wasmtime::WasmParams,
    O: wasmtime::WasmResults,
{
    f.call(s, args)
        .map_err(|e| Error::UnableToCallFunc(format!("Unable to call func: {e}")))
}

fn offset_convert(original: i32) -> Result<usize, Error> {
    original
        .try_into()
        .map_err(|e| Error::InvalidOffset(format!("Unable to convert an offset: {e}")))
}

fn host2wasm<T>(m: &Memory, s: &mut Store<T>, offset: usize, data: &[u8]) -> Result<(), Error> {
    m.write(s, offset, data)
        .map_err(|e| Error::UnableToCopyBytes(format!("writes to mem failed: {e}")))
}

fn wasm2host<T>(m: &Memory, s: &mut Store<T>, offset: usize, dest: &mut [u8]) -> Result<(), Error> {
    m.read(s, offset, dest)
        .map_err(|e| Error::UnableToCopyFromWasm(format!("Unable to read bytes: {e}")))
}

fn _transform_trusted_new16<T>(
    mut s: Store<T>,
    mem: Memory,
    input_offset: (i32, usize),
    output_offset: (i32, usize),
    main: TypedFunc<(i32, i32, i32), i64>,
    mut buf: [u8; 65536],
) -> impl FnMut([u8; 65536], [u8; 65536]) -> Result<[u8; 65536], Error> {
    move |input2trusted: [u8; 65536], output_from_untrusted: [u8; 65536]| {
        let input_offset4untrusted: i32 = input_offset.0 + 65536;
        let o_input2trusted: usize = input_offset.1;
        let o_output: usize = output_offset.1;
        let o_output_from_untrusted: usize = o_input2trusted + 65536;
        host2wasm(&mem, &mut s, o_input2trusted, &input2trusted)?;
        host2wasm(
            &mem,
            &mut s,
            o_output_from_untrusted,
            &output_from_untrusted,
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
        wasm2host(&mem, &mut s, o_output, &mut buf)?;
        Ok(buf)
    }
}

pub fn transform_trusted_new16(
    wasm_bytes: &[u8],
    mem_name: &str,
    input_offset_getter_name: &str,
    output_offset_getter_name: &str,
    main_name: &str,
) -> Result<impl FnMut([u8; 65536], [u8; 65536]) -> Result<[u8; 65536], Error>, Error> {
    let e: Engine = Engine::default();
    let m: Module = wasm2module(&e, wasm_bytes)?;
    let mut s: Store<()> = Store::new(&e, ());
    let l: Linker<()> = Linker::new(&e);
    let i: Instance = module2instance(&l, &mut s, &m)?;
    let mem: Memory = mem_find(&mut s, &i, mem_name)?;
    let input_offset_getter: TypedFunc<(), i32> =
        instance2func(&i, &mut s, input_offset_getter_name)?;
    let output_offset_getter: TypedFunc<(), i32> =
        instance2func(&i, &mut s, output_offset_getter_name)?;
    let main_program: TypedFunc<(i32, i32, i32), i64> = instance2func(&i, &mut s, main_name)?;
    let input_offset: i32 = call(&mut s, &input_offset_getter, ())?;
    let output_offset: i32 = call(&mut s, &output_offset_getter, ())?;
    let io_usz: usize = offset_convert(input_offset)?;
    let oo_usz: usize = offset_convert(output_offset)?;
    let buf = [0u8; 65536];
    Ok(_transform_trusted_new16(
        s,
        mem,
        (input_offset, io_usz),
        (output_offset, oo_usz),
        main_program,
        buf,
    ))
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
