# Ada_compression
A modular Rust library for efficient custom data compression and decompression.

![img](https://github.com/user-attachments/assets/ab1f8d00-f943-4889-80f9-9866bfeb559a)



## Pros
**Superior Simplicity and Speed**:Unlike gzip or Brotli, which employ sliding-window dictionaries (LZ77/LZ78 variants) and Huffman/entropy coding, our method requires no complex state tracking or probabilistic modeling.Compression and decompression are linear-time operations (O(n)), making it blazingly fast—often orders of magnitude quicker than DEFLATE on large datasets.
**Reliability on Repetitive Data**:outper forms generic algorithms on highly redundant inputs, such as uncompressed images
**Modularity and Extensibility**:As a library, it's primed for evolution—future variants could integrate dictionaries or hybrid methods. Compared to black-box tools like zip, our open design invites customization
**Efficiency in Niche Scenarios**:On data with long runs (e.g., 100+ identical bytes), it can surpass LZ4's lightweight mode in ratio while maintaining comparable speed.

## Cons
Versatility: We target runs exclusively, ignoring substrings or statistical redundancies
Compression Ratio: For general or random data, our RLE yields modest savings (ratios near 1.0, or even slight inflation due to flags).
Overhead and Edge Cases: Escaping adds minor bloat (up to 2x on adversarial data with many 254/255 bytes), whereas modern compressors like Snappy or Zstd minimize this through adaptive techniques.
