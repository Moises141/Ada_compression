/// Compresses input data using Ada's Adaptive Pattern Compressor (AAPC) - RLE-only variant.
///
/// Breaks data into 256KB blocks, applies adaptive RLE for runs >=3.
/// Literals conflicting with flags (254, 255) are escaped with 255.
/// No dictionary in this version for simplicity and reliability.
pub fn compress(data: &[u8]) -> Vec<u8> {
    const BLOCK_SIZE: usize = 256 * 1024;
    const MIN_RUN: usize = 3;

    let mut output = Vec::with_capacity(data.len() / 2);
    let block_count = (data.len() + BLOCK_SIZE - 1) / BLOCK_SIZE;
    output.extend_from_slice(&(block_count as u32).to_be_bytes());

    for block in data.chunks(BLOCK_SIZE) {
        let mut i = 0;
        let mut encoded = Vec::new();
        while i < block.len() {
            let mut run_len = 1;
            let byte = block[i];
            while i + run_len < block.len() && block[i + run_len] == byte && run_len < 255 {
                run_len += 1;
            }
            if run_len >= MIN_RUN {
                encoded.push(254);
                encoded.push(run_len as u8);
                encoded.push(byte);
                i += run_len;
            } else {
                if byte == 254 || byte == 255 {
                    encoded.push(255);
                    encoded.push(byte);
                } else {
                    encoded.push(byte);
                }
                i += 1;
            }
        }
        let comp_len = encoded.len() as u32;
        output.extend_from_slice(&comp_len.to_be_bytes());
        output.extend(encoded);
    }
    output
}