import { readFile } from "node:fs/promises"

const copy64 = (i=new BigUint64Array(), o=new BigUint64Array(), len=65536) => {
    i.forEach((j,k) => { o[k] = i[k] })
    return o
}

const transform_trusted_new16 = async (
    wasm_bytes = new Uint8Array(),
    mem_name = "memory",
    input_offset_getter_name = "in_buf128k",
    output_offset_getter_name = "out_buf64k",
    main_name = "",
) => {
    const result = await WebAssembly.instantiate(wasm_bytes)
    const { instance } = result
    const { exports } = instance

    const off_i1 = exports[input_offset_getter_name]()
    const off_i2 = off_i1 + 65536
    const off_o  = exports[output_offset_getter_name]()

    const f = exports[main_name]

    const mem = exports[mem_name]
    const buf = mem.buffer
    const v1 = new BigUint64Array(buf, off_i1, 8192)
    const v2 = new BigUint64Array(buf, off_i2, 8192)
    const vo = new BigUint64Array(buf, off_o,  8192)

    const transformer = async (trusted64, untrusted64) => {
        copy64(trusted64, v1)
        copy64(untrusted64, v2)
        const ret = f(off_i1, off_i2, off_o)
        const out = new BigUint64Array(8192)
        return 0 <= ret
            ? Promise.resolve(copy64(vo, out))
            : Promise.reject(new Error(`Negative count: ${ret}`))
    }
    return Promise.resolve(transformer)
}

export { transform_trusted_new16 }
