use anyhow::Result;
use std::env;
use std::time::Instant;
use tfhe_applicability::fibonacci::implementation::FibonacciEncrypted;

fn main() -> Result<()> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    
    // Default parameters
    let mut k = 10;  // Default number of Fibonacci numbers
    let mut fast_mode = false;  // Default security level
    
    // Parse arguments
    for arg in args.iter().skip(1) {
        if arg.starts_with("--num=") {
            if let Ok(num) = arg.trim_start_matches("--num=").parse::<usize>() {
                k = num;
            } else {
                println!("Invalid number format. Using default: {}", k);
            }
        } else if arg == "--fast" {
            fast_mode = true;
        } else if arg == "--help" || arg == "-h" {
            print_help();
            return Ok(());
        }
    }
    
    println!("=== TFHE Fibonacci Benchmark ===");
    let total_start = Instant::now();
    
    println!("\nParameters:");
    println!("- Computing {} Fibonacci numbers", k);
    println!("- Security mode: {}", if fast_mode { "Fast (lower security)" } else { "Default" });
    
    println!("\n=== Setup Phase ===");
    // Create a new instance with generated keys based on the security mode
    let mut fib = if fast_mode {
        FibonacciEncrypted::new_fast()
    } else {
        FibonacciEncrypted::new()
    };

    println!("\n=== Computation Phase ===");
    // Compute the encrypted sequence
    let encrypted_sequence = fib.compute_sequence(k)?;

    println!("\n=== Decryption Phase ===");
    // Decrypt and print the sequence
    let decrypted_sequence = fib.decrypt_sequence(&encrypted_sequence);

    println!("\n=== Results ===");
    println!("Fibonacci sequence:");
    for (i, num) in decrypted_sequence.iter().enumerate() {
        println!("F({}) = {}", i, num);
    }
    
    let total_time = total_start.elapsed();
    let total_ms = total_time.as_secs_f64() * 1000.0;
    println!("\n=== Summary ===");
    println!("Total execution time: {:.2} ms", total_ms);
    
    // Print the detailed timing summary
    fib.print_timing_summary();

    Ok(())
}

fn print_help() {
    println!("TFHE Fibonacci Sequence Calculator");
    println!("Usage: fibonacci [OPTIONS]");
    println!("\nOptions:");
    println!("  --num=N       Number of Fibonacci numbers to compute (default: 10)");
    println!("  --fast        Use faster but less secure encryption parameters");
    println!("  --help, -h    Display this help message");
}

