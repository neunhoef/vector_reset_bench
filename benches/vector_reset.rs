use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Duration;

// Structure to hold our vector and its state
struct TestVector {
    data: Vec<u64>,
    checksum: u64,  // Add checksum to force evaluation
}

impl TestVector {
    fn new(len: usize, init_value: u64) -> Self {
        let data = vec![init_value; len];
        let checksum = init_value * (len as u64);  // Simple checksum
        Self { data, checksum }
    }

    // Force compiler to actually process the vector
    fn verify(&self) -> u64 {
        self.data.iter().sum::<u64>()
    }
}

// Function to reset vector by recreating it
fn reset_by_recreate(len: usize, init_value: u64) -> TestVector {
    TestVector::new(len, init_value)
}

// Function to reset vector by modifying dirty positions
fn reset_by_loop(vec: &mut TestVector, dirty_positions: &[usize], init_value: u64) {
    for &pos in dirty_positions {
        vec.data[pos] = init_value;
    }
    // Update checksum
    vec.checksum = vec.verify();
}

fn bench_resets(c: &mut Criterion) {
    let vec_sizes = [10_000_000, 1_000_000, 100_000_000];
    let dirty_percentages = [1, 5, 10, 25, 50, 75, 100];
    let init_value = 0u64;

    let mut group = c.benchmark_group("vector_reset");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(30);  // Reduced sample size for very large vectors

    for &size in &vec_sizes {
        // Create list of all possible positions
        let all_positions: Vec<usize> = (0..size).collect();
        
        for &dirty_percent in &dirty_percentages {
            let dirty_count = (size as f64 * (dirty_percent as f64 / 100.0)) as usize;
            
            // Randomly select positions to be marked as dirty
            let mut rng = thread_rng();
            let mut positions = all_positions.clone();
            positions.shuffle(&mut rng);
            let dirty_positions: Vec<usize> = positions.into_iter().take(dirty_count).collect();

            // Benchmark recreation approach
            group.bench_with_input(
                BenchmarkId::new("recreate", format!("{}_{}", size, dirty_percent)),
                &(size, &dirty_positions),
                |b, &(size, _)| {
                    b.iter(|| {
                        let vec = reset_by_recreate(size, init_value);
                        black_box(vec.verify());  // Force evaluation
                    });
                },
            );

            // Benchmark loop reset approach
            group.bench_with_input(
                BenchmarkId::new("loop", format!("{}_{}", size, dirty_percent)),
                &(size, &dirty_positions),
                |b, &(_, dirty_positions)| {
                    let mut vec = TestVector::new(size, 1);  // Initialize with 1s
                    b.iter(|| {
                        reset_by_loop(&mut vec, dirty_positions, init_value);
                        black_box(vec.verify());  // Force evaluation
                    });
                },
            );
        }
    }
    group.finish();
}

criterion_group!(benches, bench_resets);
criterion_main!(benches);

