import { readFile } from "node:fs/promises"

import { transform_new_with_untrusted15 } from "../../../../js/transform_trusted_new15.mjs"

const PAGE_SIZE = 65536

const FULL_PAGE = PAGE_SIZE
const HALF_PAGE = PAGE_SIZE >> 1

const main = async () => {
    const cnt64 = HALF_PAGE >> 3
    return [
        readFile,
        wasm_bytes => transform_new_with_untrusted15(
            input2untrusted => input2untrusted.fill(2n),
            wasm_bytes,
            "memory",
            "in_buf64k",
            "out_buf32k",
            "bitwise_or32k",
        ),
        async transformer => {
            const input2trusted = new BigUint64Array(cnt64)
            input2trusted.fill(1n)
            const input2untrusted = new BigUint64Array(cnt64)
            const bitwiseOr = await transformer(input2trusted, input2untrusted)
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
