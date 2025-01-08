use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Duration;

// Function to reset vector by recreating it
fn reset_by_recreate(len: usize, init_value: u64) -> Vec<u64> {
    vec![init_value; len]
}

// Function to reset vector by modifying dirty positions
fn reset_by_loop(vec: &mut Vec<u64>, dirty_positions: &[usize], init_value: u64) {
    for &pos in dirty_positions {
        vec[pos] = init_value;
    }
}

fn bench_resets(c: &mut Criterion) {
    let vec_sizes = [1000000, 10000000, 100000000];
    let dirty_percentages = [1, 5, 10, 25, 50, 75, 100];
    let init_value = 0u64;

    let mut group = c.benchmark_group("vector_reset");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(50);

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
                        black_box(reset_by_recreate(size, init_value));
                    });
                },
            );

            // Benchmark loop reset approach
            group.bench_with_input(
                BenchmarkId::new("loop", format!("{}_{}", size, dirty_percent)),
                &(size, &dirty_positions),
                |b, &(_, dirty_positions)| {
                    let mut vec = vec![1u64; size];  // Initialize with 1 to ensure we actually need to reset
                    b.iter(|| {
                        reset_by_loop(black_box(&mut vec), dirty_positions, init_value);
                    });
                },
            );
        }
    }
    group.finish();
}

criterion_group!(benches, bench_resets);
criterion_main!(benches);
