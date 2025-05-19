use super::implementation::FibonacciEncrypted;
use tfhe::prelude::*;

#[test]
fn test_fibonacci_sequence() {
    let mut fib = FibonacciEncrypted::new();
    let k = 10;
    
    // Compute encrypted sequence
    let encrypted_sequence = fib.compute_sequence(k).unwrap();
    
    // Decrypt sequence
    let decrypted_sequence = fib.decrypt_sequence(&encrypted_sequence);
    
    // Expected first 10 Fibonacci numbers
    let expected = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
    
    assert_eq!(decrypted_sequence, expected);
}

#[test]
fn test_initial_values() {
    let mut fib = FibonacciEncrypted::new();
    let (a, b) = fib.encrypt_initial_values();
    
    let a_decrypted: u32 = a.decrypt(&fib.client_key);
    let b_decrypted: u32 = b.decrypt(&fib.client_key);
    
    assert_eq!(a_decrypted, 0);
    assert_eq!(b_decrypted, 1);
} 