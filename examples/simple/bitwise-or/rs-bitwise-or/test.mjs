import { readFile } from "node:fs/promises"

import { transform_trusted_new16 } from "../../../../js/transform_trusted_new16.mjs"

const main = async () => {
    return [
        readFile,
        wasm_bytes => transform_trusted_new16(
            wasm_bytes,
            "memory",
            "in_buf128k",
            "out_buf64k",
            "bitwise_or64k",
        ),
        async transformer => {
            const trusted = new BigUint64Array(8192)
            trusted.fill(1n)
            const untrusted = new BigUint64Array(8192)
            untrusted.fill(2n)
            const bitwiseOr = await transformer(trusted, untrusted)
            return bitwiseOr[0]
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
