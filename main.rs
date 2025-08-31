use clap::{Parser, Subcommand};
use rand::Rng;
use std::fs::{self, read, write};
use std::io::{self};
use std::path::Path;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

mod compression;
mod decompression;

#[derive(Parser)]
#[command(name = "Ada_compression")]
#[command(about = "Ada's Adaptive Pattern Compressor CLI", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Compress a file
    Compress {
        /// Input file path
        input: String,
        /// Output file path
        output: String,
    },
    /// Decompress a file
    Decompress {
        /// Input file path
        input: String,
        /// Output file path
        output: String,
    },
    /// Run tests (generated data, or specify a file)
    Test {
        /// Optional: Path to a real file for testing
        file: Option<String>,
    },
    /// Test all files in the 'test_data' folder
    TestFolder,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compress { input, output } => {
            if cli.verbose {
                println!("Verbose: Reading input file {}", input);
            }
            let data = match read(&input) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("Error reading input {}: {}", input, e);
                    return Err(e);
                }
            };
            let start = Instant::now();
            let compressed = compression::compress(&data);
            let duration = start.elapsed();
            if cli.verbose {
                println!("Verbose: Writing compressed output to {}", output);
            }
            match write(&output, &compressed) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error writing output {}: {}", output, e);
                    return Err(e);
                }
            }
            println!("Compressed {} ({} bytes) to {} ({} bytes) in {:?}. Ratio: {:.2}",
                     input, data.len(), output, compressed.len(), duration,
                     compressed.len() as f64 / data.len() as f64);
        }
        Commands::Decompress { input, output } => {
            if cli.verbose {
                println!("Verbose: Reading compressed input {}", input);
            }
            let compressed = match read(&input) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error reading input {}: {}", input, e);
                    return Err(e);
                }
            };
            let start = Instant::now();
            let decompressed = decompression::decompress(&compressed);
            let duration = start.elapsed();
            if cli.verbose {
                println!("Verbose: Writing decompressed output to {}", output);
            }
            match write(&output, &decompressed) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error writing output {}: {}", output, e);
                    return Err(e);
                }
            }
            println!("Decompressed {} ({} bytes) to {} ({} bytes) in {:?}.",
                     input, compressed.len(), output, decompressed.len(), duration);
        }
        Commands::Test { file } => {
            if let Some(input_path) = file {
                run_file_test(&input_path, cli.verbose)?;
            } else {
                run_generated_test(cli.verbose);
            }
        }
        Commands::TestFolder => {
            run_folder_test(cli.verbose)?;
        }
    }
    Ok(())
}

fn run_generated_test(verbose: bool) {
    println!("Continuing our symphony of compression, courtesy of Ada Lovelace!");

    // Enhanced test data: mix repetitive and random for realism
    let mut rng = rand::thread_rng();
    let mut test_data: Vec<u8> = Vec::with_capacity(1024 * 1024); // 1MB
    for _ in 0..1024 {
        let repeat_byte = rng.gen();
        let run_len = rng.gen_range(1..100);
        for _ in 0..run_len {
            test_data.push(repeat_byte);
        }
        for _ in 0..rng.gen_range(1..50) {
            test_data.push(rng.gen());
        }
    }

    if verbose {
        println!("Verbose: Generated test data of {} bytes", test_data.len());
    }

    // Compress
    let start = Instant::now();
    let compressed = compression::compress(&test_data);
    let compress_time = start.elapsed();
    let ratio = compressed.len() as f64 / test_data.len() as f64;

    println!("Original size: {} bytes", test_data.len());
    println!("Compressed size: {} bytes (ratio: {:.2})", compressed.len(), ratio);
    println!("Compression time: {:?}", compress_time);

    // Decompress
    let start = Instant::now();
    let decompressed = decompression::decompress(&compressed);
    let decompress_time = start.elapsed();

    println!("Decompressed size: {} bytes", decompressed.len());
    println!("Decompression time: {:?}", decompress_time);

    // Verify
    assert_eq!(test_data, decompressed, "Decompression mismatch!");
    println!("Harmony restored: Data is identical.");
}

fn run_file_test(input_path: &str, verbose: bool) -> io::Result<()> {
    println!("Testing with real file: {}", input_path);
    let test_data = read(input_path)?;

    if verbose {
        println!("Verbose: Loaded file of {} bytes", test_data.len());
    }

    // Compress
    let start = Instant::now();
    let compressed = compression::compress(&test_data);
    let compress_time = start.elapsed();
    let ratio = compressed.len() as f64 / test_data.len() as f64;

    println!("Original size: {} bytes", test_data.len());
    println!("Compressed size: {} bytes (ratio: {:.2})", compressed.len(), ratio);
    println!("Compression time: {:?}", compress_time);

    // Decompress
    let start = Instant::now();
    let decompressed = decompression::decompress(&compressed);
    let decompress_time = start.elapsed();

    println!("Decompressed size: {} bytes", decompressed.len());
    println!("Decompression time: {:?}", decompress_time);

    // Verify
    assert_eq!(test_data, decompressed, "Decompression mismatch!");
    println!("Harmony restored: Data is identical.");

    Ok(())
}

fn run_folder_test(verbose: bool) -> io::Result<()> {
    let folder_path = Path::new("test_data");
    if !folder_path.exists() || !folder_path.is_dir() {
        eprintln!("Error: 'test_data' folder does not exist or is not a directory.");
        return Ok(());
    }

    let mut log_entries = Vec::new();
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
            let data = read(&path)?;
            let orig_size = data.len() as f64;

            if verbose {
                println!("Verbose: Processing file {} ({} bytes)", file_name, orig_size as usize);
            }

            // Compress
            let compress_start = Instant::now();
            let compressed = compression::compress(&data);
            let compress_time = compress_start.elapsed();
            let comp_size = compressed.len() as f64;
            let ratio = comp_size / orig_size;
            let compress_speed = if compress_time.as_secs_f64() > 0.0 { orig_size / compress_time.as_secs_f64() } else { 0.0 };

            // Decompress
            let decompress_start = Instant::now();
            let decompressed = decompression::decompress(&compressed);
            let decompress_time = decompress_start.elapsed();
            let decompress_speed = if decompress_time.as_secs_f64() > 0.0 { orig_size / decompress_time.as_secs_f64() } else { 0.0 };

            // Verify
            assert_eq!(data, decompressed, "Decompression mismatch for file: {}", file_name);

            // Timestamp for log (basic Unix seconds)
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();

            // Log entry
            log_entries.push(format!(
                "Timestamp: {}s\nFile: {}\nOriginal Size: {} bytes\nCompressed Size: {} bytes\nRatio: {:.2}\nCompress Time: {:?}\nCompress Speed: {:.2} bytes/s\nDecompress Time: {:?}\nDecompress Speed: {:.2} bytes/s\n---",
                timestamp, file_name, orig_size as usize, comp_size as usize, ratio, compress_time, compress_speed, decompress_time, decompress_speed
            ));

            println!("Tested {} successfully.", file_name);
        }
    }

    if log_entries.is_empty() {
        println!("No files found in 'test_data' folder.");
    } else {
        let log_content = log_entries.join("\n\n");
        write("test_log.txt", log_content)?;
        println!("All tests complete. Log written to 'test_log.txt'.");
    }

    Ok(())
}