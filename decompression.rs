/// Decompresses data compressed with AAPC - RLE-only variant.
///
/// Reverses per-block RLE and escaped literals.
pub fn decompress(compressed: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(compressed.len() * 2);
    let mut idx = 0;
    let block_count = u32::from_be_bytes(compressed[idx..idx + 4].try_into().unwrap());
    idx += 4;

    for _ in 0..block_count {
        let comp_len = u32::from_be_bytes(compressed[idx..idx + 4].try_into().unwrap()) as usize;
        idx += 4;
        let block_end = idx + comp_len;

        while idx < block_end {
            let flag = compressed[idx];
            idx += 1;

            if flag == 255 {
                // Escaped literal
                let byte = compressed[idx];
                idx += 1;
                output.push(byte);
            } else if flag == 254 {
                // RLE
                let run_len = compressed[idx] as usize;
                idx += 1;
                let byte = compressed[idx];
                idx += 1;
                output.resize(output.len() + run_len, byte);
            } else {
                // Normal literal
                output.push(flag);
            }
        }
    }
    output
}