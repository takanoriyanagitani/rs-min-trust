import { readFile } from "node:fs/promises"

import { transform_trusted_new15 } from "../../../../js/transform_trusted_new15.mjs"

const PAGE_SIZE = 65536

const FULL_PAGE = PAGE_SIZE
const HALF_PAGE = PAGE_SIZE >> 1

const main = async () => {
    const cnt64 = HALF_PAGE >> 3
    return [
        readFile,
        wasm_bytes => transform_trusted_new15(
            wasm_bytes,
            "memory",
            "in_buf64k",
            "out_buf32k",
            "bitwise_or32k",
        ),
        async transformer => {
            const trusted = new BigUint64Array(cnt64)
            trusted.fill(1n)
            const untrusted = new BigUint64Array(cnt64)
            untrusted.fill(2n)
            const bitwiseOr = await transformer(trusted, untrusted)
            return bitwiseOr[cnt64-1]
        },
        console.info,
    ].reduce(
        (state, f) => state.then(f),
        Promise.resolve("target/wasm32-unknown-unknown/release-wasm/rs_bitwise_or.wasm"),
    )
}

Promise.resolve()
.then(main)
.catch(console.error)
