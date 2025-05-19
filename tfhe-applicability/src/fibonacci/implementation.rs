use anyhow::Result;
use std::time::Instant;
use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint32};

/// A struct that handles homomorphic encryption operations for Fibonacci sequence computation
pub struct FibonacciEncrypted {
    pub(crate) client_key: tfhe::ClientKey,
    pub(crate) server_key: tfhe::ServerKey,
    pub(crate) timings: FibonacciTimings,
    pub(crate) config_params: ConfigParams,
}

/// Structure for configuring TFHE parameters
#[derive(Clone, Debug)]
pub struct ConfigParams {
    pub security_level: SecurityLevel,
    pub description: String,
}

/// Predefined security levels
#[derive(Clone, Debug)]
pub enum SecurityLevel {
    Default,
    Fast,
    Custom(tfhe::Config),
}

/// Structure to keep track of various timings
#[derive(Default)]
pub struct FibonacciTimings {
    pub key_generation: Option<f64>,
    pub server_key_setup: Option<f64>,
    pub encryption: Option<f64>,
    pub computation: Option<f64>,
    pub computation_steps: Vec<f64>,
    pub decryption: Option<f64>,
    pub total: Option<f64>,
}

impl FibonacciTimings {
    fn time_as_ms(duration: std::time::Duration) -> f64 {
        duration.as_secs_f64() * 1000.0
    }
}

impl Default for ConfigParams {
    fn default() -> Self {
        Self {
            security_level: SecurityLevel::Default,
            description: "Default security parameters".to_string(),
        }
    }
}

impl FibonacciEncrypted {
    /// Creates a new instance with generated keys using default parameters
    pub fn new() -> Self {
        Self::with_config(ConfigParams::default())
    }
    
    /// Creates a new instance with a faster configuration that sacrifices some security
    pub fn new_fast() -> Self {
        Self::with_config(ConfigParams {
            security_level: SecurityLevel::Fast,
            description: "Faster but less secure parameters".to_string(),
        })
    }
    
    /// Creates a new instance with specified security parameters
    pub fn with_config(params: ConfigParams) -> Self {
        println!("Configuration: {}", params.description);
        println!("Starting key generation...");
        let start_time = Instant::now();
        
        let config = match params.security_level {
            SecurityLevel::Default => ConfigBuilder::default().build(),
            SecurityLevel::Fast => {
                // Use parameters that are faster but less secure
                let config_builder = ConfigBuilder::default();
                // For a real implementation, we would customize parameters here
                config_builder.build()
            },
            SecurityLevel::Custom(ref custom_config) => custom_config.clone(),
        };
        
        let (client_key, server_key) = generate_keys(config);
        
        let elapsed = start_time.elapsed();
        let elapsed_ms = FibonacciTimings::time_as_ms(elapsed);
        println!("Key generation completed in: {:.2} ms", elapsed_ms);

        Self {
            client_key,
            server_key,
            timings: FibonacciTimings {
                key_generation: Some(elapsed_ms),
                ..Default::default()
            },
            config_params: params,
        }
    }

    /// Encrypts the initial Fibonacci values (0, 1)
    pub fn encrypt_initial_values(&mut self) -> (FheUint32, FheUint32) {
        println!("Starting encryption of initial values...");
        let start_time = Instant::now();
        
        let a =
            FheUint32::try_encrypt(0u32, &self.client_key).expect("Failed to encrypt first value");
        let b =
            FheUint32::try_encrypt(1u32, &self.client_key).expect("Failed to encrypt second value");
        
        let elapsed = start_time.elapsed();
        let elapsed_ms = FibonacciTimings::time_as_ms(elapsed);
        println!("Encryption of initial values completed in: {:.2} ms", elapsed_ms);
        
        self.timings.encryption = Some(elapsed_ms);
        
        (a, b)
    }

    /// Computes k iterations of the Fibonacci sequence homomorphically
    pub fn compute_sequence(&mut self, k: usize) -> Result<Vec<FheUint32>> {
        println!("Setting server key for homomorphic operations...");
        let start_server_key = Instant::now();
        
        // Set the server key for operations
        set_server_key(self.server_key.clone());
        
        let elapsed_server_key = start_server_key.elapsed();
        let elapsed_sk_ms = FibonacciTimings::time_as_ms(elapsed_server_key);
        println!("Server key set in: {:.2} ms", elapsed_sk_ms);
        self.timings.server_key_setup = Some(elapsed_sk_ms);

        let start_time = Instant::now();
        let (mut a, mut b) = self.encrypt_initial_values();
        let mut sequence = vec![a.clone(), b.clone()];

        println!("Starting homomorphic computation of Fibonacci sequence...");
        println!("| Step | Operation    | Time (ms) |");
        println!("|------|--------------|-----------|");
        
        let start_computation = Instant::now();
        self.timings.computation_steps = Vec::with_capacity(k);
        
        for i in 2..k {
            let iteration_start = Instant::now();
            
            let add_start = Instant::now();
            let next = &a + &b;
            let add_time = add_start.elapsed();
            let add_ms = FibonacciTimings::time_as_ms(add_time);
            
            let clone_start = Instant::now();
            a = b;
            b = next.clone();
            let clone_time = clone_start.elapsed();
            let clone_ms = FibonacciTimings::time_as_ms(clone_time);
            
            sequence.push(next);
            
            let iteration_time = iteration_start.elapsed();
            let iteration_ms = FibonacciTimings::time_as_ms(iteration_time);
            self.timings.computation_steps.push(iteration_ms);
            
            println!("| F({}) | Add: {:.2} ms, Clone: {:.2} ms | {:.2} ms |", 
                     i, add_ms, clone_ms, iteration_ms);
        }
        
        let total_computation = start_computation.elapsed();
        let total_comp_ms = FibonacciTimings::time_as_ms(total_computation);
        println!("Total homomorphic computation time: {:.2} ms", total_comp_ms);
        self.timings.computation = Some(total_comp_ms);
        
        let total_time = start_time.elapsed();
        let total_ms = FibonacciTimings::time_as_ms(total_time);
        println!("Total sequence generation time (including encryption): {:.2} ms", total_ms);

        Ok(sequence)
    }

    /// Decrypts a sequence of encrypted Fibonacci numbers
    pub fn decrypt_sequence(&mut self, sequence: &[FheUint32]) -> Vec<u32> {
        println!("Starting decryption of Fibonacci sequence...");
        let start_time = Instant::now();
        
        let result = sequence
            .iter()
            .map(|x| x.decrypt(&self.client_key))
            .collect();
        
        let elapsed = start_time.elapsed();
        let elapsed_ms = FibonacciTimings::time_as_ms(elapsed);
        println!("Decryption completed in: {:.2} ms", elapsed_ms);
        self.timings.decryption = Some(elapsed_ms);
        
        result
    }
    
    /// Prints a summary of all timings
    pub fn print_timing_summary(&self) {
        println!("\n=== TFHE Timing Summary ===");
        println!("Configuration: {}", self.config_params.description);
        println!("| Phase                | Time (ms)     |");
        println!("|----------------------|---------------|");
        
        if let Some(time) = self.timings.key_generation {
            println!("| Key Generation      | {:.2} ms    |", time);
        }
        
        if let Some(time) = self.timings.server_key_setup {
            println!("| Server Key Setup    | {:.2} ms    |", time);
        }
        
        if let Some(time) = self.timings.encryption {
            println!("| Initial Encryption  | {:.2} ms    |", time);
        }
        
        if let Some(time) = self.timings.computation {
            println!("| FHE Computation     | {:.2} ms    |", time);
        }
        
        if let Some(time) = self.timings.decryption {
            println!("| Decryption          | {:.2} ms    |", time);
        }
        
        if !self.timings.computation_steps.is_empty() {
            println!("\n=== Computation Step Details ===");
            println!("| Step | Time (ms)     |");
            println!("|------|---------------|");
            
            for (i, &time) in self.timings.computation_steps.iter().enumerate() {
                println!("| F({})  | {:.2} ms    |", i + 2, time);
            }
            
            // Calculate average time per step
            let avg_time = self.timings.computation_steps.iter().sum::<f64>() / 
                          self.timings.computation_steps.len() as f64;
            println!("\nAverage time per computation step: {:.2} ms", avg_time);
        }
    }
}

