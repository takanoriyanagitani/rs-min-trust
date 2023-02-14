use wasmtime::{Engine, Instance, Linker, Memory, Module, Store, TypedFunc};

use crate::{
    err::Error,
    transform_new,
    wasm::common_wasmtime::{
        call, host2wasm, instance2func, mem_find, module2instance, offset_convert, wasm2host,
        wasm2module,
    },
};

fn _transform_trusted_new16<T>(
    mut s: Store<T>,
    mem: Memory,
    input_offset: (i32, usize),
    output_offset: (i32, usize),
    main: TypedFunc<(i32, i32, i32), i64>,
    mut buf: [u8; 65536],
) -> impl FnMut([u8; 65536], [u8; 65536]) -> Result<[u8; 65536], Error> {
    let input_offset4untrusted: i32 = input_offset.0 + 65536;
    let o_input2trusted: usize = input_offset.1;
    let o_output: usize = output_offset.1;
    let o_output_from_untrusted: usize = o_input2trusted + 65536;
    move |input2trusted: [u8; 65536], output_from_untrusted: [u8; 65536]| {
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
