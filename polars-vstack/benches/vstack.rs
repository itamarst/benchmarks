use divan::{bench, AllocProfiler, Bencher};
use polars_core::prelude::*;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    // Run registered benchmarks.
    divan::main();
}

// Define a `fibonacci` function and register it for benchmarking.
#[bench(args=[100, 1000, 10_000, 100_000])]
fn vstack(bencher: Bencher, n: u64) {
    bencher
        .counter(n * 1000 * 8)
        .with_inputs(|| {
            (0..n)
                .map(|i| {
                    let series = Series::new("val", vec![i; 1000]);
                    DataFrame::new(vec![series]).unwrap()
                })
                .collect::<Vec<DataFrame>>()
        })
        .bench_refs(|dfs| {
            let series = Series::new("val", vec![0u64; 1000]);
            let mut acc_df = DataFrame::new(vec![series]).unwrap();

            for df in dfs {
                acc_df.vstack_mut(df).unwrap();
            }
            acc_df
        })
}
