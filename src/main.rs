use std::time::Instant;
use std::thread;

fn main() {
    //number to calculate prime numbers
    let n = 1000000;
    //average time of 10 runs of seive_of_eratosthenes
    let start = Instant::now();
    let mut thread_count = 1;
    for _ in 1..84 {
        spawn_threads(n);
        println! ("threads done: {}", thread_count*12);
        thread_count += 1;
    }
    let duration = start.elapsed();
    //print average time of 10 runs
    println!("Average time of 996 runs of seive_of_eratosthenes: {:?}", duration / (83 * 12));
    println!("Total time: {:?}", duration);

    //algorithm to generate a score based on time to complete
    let score = 1000000000 / (duration.as_secs() * 1000000 + duration.subsec_micros() as u64);
    println!("Score: {}", score);
}

//function to calculate prime numbers from a list using Trial Division
fn trial_division(n: u64) -> Vec<u64> {
    let mut prime_numbers = Vec::new();
    for i in 2..n + 1 {
        let mut is_prime = true;
        for j in 2..i {
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            prime_numbers.push(i);
        }
    }
    return prime_numbers;
}

//function to calculate prime numbers using Sieve of Eratosthenes
fn sieve_of_eratosthenes(n: u64) -> Vec<u64> {
    let mut primes = vec![true; (n + 1) as usize];
    let mut p = 2;
    while p * p <= n {
        if primes[p as usize] == true {
            for i in (p * p..n + 1).step_by(p as usize) {
                primes[i as usize] = false;
            }
        }
        p += 1;
    }
    let mut prime_numbers = Vec::new();
    for i in 2..n + 1 {
        if primes[i as usize] == true {
            prime_numbers.push(i);
        }
    }
    return prime_numbers;
}

//function to spwan 10 threads that will run the sieve_of_eratosthenes function
fn spawn_threads(n: u64) {
    //create a vec of handles
    let mut handles = vec![];
    for i in 0..12 {
        let handle = thread::spawn(move || {
            let prime_numbers = sieve_of_eratosthenes(n);
        });
        //push the handle to the vec
        handles.push(handle);
    }
    //wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}