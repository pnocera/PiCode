//! Performance benchmarks for PiCode

pub mod memory_benchmarks;
pub mod llm_benchmarks;
pub mod workspace_benchmarks;

use super::TestContext;
use std::time::{Duration, Instant};

/// Performance test utilities
pub struct PerformanceTestRunner {
    pub iterations: usize,
    pub context: TestContext,
}

impl PerformanceTestRunner {
    pub fn new() -> picode::Result<Self> {
        Ok(Self {
            iterations: 100,
            context: TestContext::new()?,
        })
    }

    pub fn benchmark<F, R>(&self, name: &str, mut operation: F) -> BenchmarkResult
    where
        F: FnMut() -> R,
    {
        let mut times = Vec::with_capacity(self.iterations);
        
        for _ in 0..self.iterations {
            let start = Instant::now();
            let _ = operation();
            let duration = start.elapsed();
            times.push(duration);
        }
        
        BenchmarkResult::new(name.to_string(), times)
    }

    pub async fn benchmark_async<F, Fut, R>(&self, name: &str, mut operation: F) -> BenchmarkResult
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = R>,
    {
        let mut times = Vec::with_capacity(self.iterations);
        
        for _ in 0..self.iterations {
            let start = Instant::now();
            let _ = operation().await;
            let duration = start.elapsed();
            times.push(duration);
        }
        
        BenchmarkResult::new(name.to_string(), times)
    }
}

/// Benchmark results analysis
pub struct BenchmarkResult {
    pub name: String,
    pub times: Vec<Duration>,
    pub mean: Duration,
    pub median: Duration,
    pub min: Duration,
    pub max: Duration,
}

impl BenchmarkResult {
    pub fn new(name: String, mut times: Vec<Duration>) -> Self {
        times.sort();
        
        let mean = Duration::from_nanos(
            times.iter().map(|d| d.as_nanos()).sum::<u128>() as u64 / times.len() as u64
        );
        
        let median = times[times.len() / 2];
        let min = times[0];
        let max = times[times.len() - 1];
        
        Self {
            name,
            times,
            mean,
            median,
            min,
            max,
        }
    }

    pub fn print_summary(&self) {
        println!("Benchmark: {}", self.name);
        println!("  Mean:   {:?}", self.mean);
        println!("  Median: {:?}", self.median);
        println!("  Min:    {:?}", self.min);
        println!("  Max:    {:?}", self.max);
        println!("  Samples: {}", self.times.len());
    }

    pub fn assert_performance(&self, max_mean: Duration, max_p99: Duration) {
        assert!(
            self.mean <= max_mean,
            "Mean duration {:?} exceeds limit {:?}",
            self.mean,
            max_mean
        );

        let p99_index = (self.times.len() as f64 * 0.99) as usize;
        let p99 = self.times[p99_index.min(self.times.len() - 1)];
        
        assert!(
            p99 <= max_p99,
            "P99 duration {:?} exceeds limit {:?}",
            p99,
            max_p99
        );
    }
}

/// Core performance tests
#[cfg(test)]
mod tests {
    use super::*;
    use picode::core::*;

    #[tokio::test]
    async fn benchmark_session_creation() {
        let runner = PerformanceTestRunner::new().expect("Failed to create performance runner");
        
        let result = runner.benchmark("session_creation", || {
            let session_id = SessionId::new();
            let _session = Session::new(session_id, "perf-test".to_string());
        });
        
        result.print_summary();
        // Session creation should be very fast
        result.assert_performance(
            Duration::from_millis(1),
            Duration::from_millis(5)
        );
    }

    #[tokio::test]
    async fn benchmark_pane_creation() {
        let runner = PerformanceTestRunner::new().expect("Failed to create performance runner");
        
        let result = runner.benchmark("pane_creation", || {
            let pane_id = PaneId::new();
            let _pane = Pane::new(pane_id, PaneType::Terminal);
        });
        
        result.print_summary();
        // Pane creation should be very fast
        result.assert_performance(
            Duration::from_millis(1),
            Duration::from_millis(5)
        );
    }

    #[tokio::test]
    async fn benchmark_command_building() {
        let runner = PerformanceTestRunner::new().expect("Failed to create performance runner");
        
        let result = runner.benchmark("command_building", || {
            let _command = CommandBuilder::new("echo")
                .arg("test")
                .arg("performance")
                .build();
        });
        
        result.print_summary();
        // Command building should be fast
        result.assert_performance(
            Duration::from_millis(2),
            Duration::from_millis(10)
        );
    }

    #[tokio::test]
    async fn benchmark_workspace_creation() {
        let runner = PerformanceTestRunner::new().expect("Failed to create performance runner");
        
        let result = runner.benchmark("workspace_creation", || {
            let config = WorkspaceConfig {
                root_path: runner.context.temp_dir.path().to_path_buf(),
                name: "perf-test".to_string(),
                layout: "default".to_string(),
            };
            let _workspace = Workspace::new(config);
        });
        
        result.print_summary();
        // Workspace creation may involve file I/O, allow more time
        result.assert_performance(
            Duration::from_millis(10),
            Duration::from_millis(50)
        );
    }
}