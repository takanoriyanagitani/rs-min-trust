const transform_new = (
    trusted = (input2trusted, output_from_untrusted) => Promise.resolve({}),
    untrusted = (input2untrusted) => Promise.resolve({}),
) => {
    return async (input2trusted, input2untrusted) => {
        const output_from_untrusted = await untrusted(input2untrusted)
        return trusted(input2trusted, output_from_untrusted)
    }
}

export { transform_new }
