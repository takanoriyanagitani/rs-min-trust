import { readFile } from "node:fs/promises"

import { transform_new } from "./transform.mjs"

const PAGE_SIZE = 65536

const FULL_PAGE = PAGE_SIZE
const HALF_PAGE = PAGE_SIZE >> 1

const copy64 = (i=new BigUint64Array(), o=new BigUint64Array(), len=HALF_PAGE) => {
    i.forEach((j,k) => { o[k] = i[k] })
    return o
}

const transform_trusted_new15 = async (
    wasm_bytes = new Uint8Array(),
    mem_name = "memory",
    input_offset_getter_name = "in_buf64k",
    output_offset_getter_name = "out_buf32k",
    main_name = "",
) => {
    const result = await WebAssembly.instantiate(wasm_bytes)
    const { instance } = result
    const { exports } = instance

    const off_i1 = exports[input_offset_getter_name]()
    const off_i2 = off_i1 + HALF_PAGE
    const off_o  = exports[output_offset_getter_name]()

    const f = exports[main_name]

    const mem = exports[mem_name]
    const buf = mem.buffer
    const cnt64 = HALF_PAGE >> 3
    const v1 = new BigUint64Array(buf, off_i1, cnt64)
    const v2 = new BigUint64Array(buf, off_i2, cnt64)
    const vo = new BigUint64Array(buf, off_o,  cnt64)

    const transformer = async (trusted64, untrusted64) => {
        copy64(trusted64, v1)
        copy64(untrusted64, v2)
        const ret = f(off_i1, off_i2, off_o)
        const out = new BigUint64Array(cnt64)
        return 0 <= ret
            ? Promise.resolve(copy64(vo, out))
            : Promise.reject(new Error(`Negative count: ${ret}`))
    }
    return Promise.resolve(transformer)
}

const transform_new_with_untrusted15 = async (
    untrusted = (input) => Promise.resolve(new Uint8Array()),
    wasm_bytes = new Uint8Array(),
    mem_name = "memory",
    input_offset_getter_name = "in_buf64k",
    output_offset_getter_name = "out_buf32k",
    main_name = "",
) => {
    const trusted = await transform_trusted_new15(
        wasm_bytes,
        mem_name,
        input_offset_getter_name,
        output_offset_getter_name,
        main_name,
    )
    return transform_new(trusted, untrusted)
}

export {
    transform_trusted_new15,
    transform_new_with_untrusted15,
}
