//! PiCode Verification Script
//! 
//! Standalone verification runner for validating PiCode implementation

use std::process::Command;
use std::env;

mod tests;
use tests::test_runner::{ValidationRunner, ValidationConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 PiCode Verification Agent");
    println!("============================");
    
    let args: Vec<String> = env::args().collect();
    
    // Parse command line arguments
    let config = if args.len() > 1 {
        match args[1].as_str() {
            "quick" => ValidationConfig {
                run_unit_tests: true,
                run_integration_tests: false,
                run_e2e_tests: true,
                run_performance_tests: false,
                run_security_tests: true,
                generate_report: true,
                fail_on_warning: false,
            },
            "full" => ValidationConfig::default(),
            "security" => ValidationConfig {
                run_unit_tests: false,
                run_integration_tests: false,
                run_e2e_tests: false,
                run_performance_tests: false,
                run_security_tests: true,
                generate_report: true,
                fail_on_warning: true,
            },
            "perf" => ValidationConfig {
                run_unit_tests: false,
                run_integration_tests: false,
                run_e2e_tests: false,
                run_performance_tests: true,
                run_security_tests: false,
                generate_report: true,
                fail_on_warning: false,
            },
            _ => {
                println!("Usage: {} [quick|full|security|perf]", args[0]);
                println!("  quick    - Run unit, e2e, and security tests");
                println!("  full     - Run all test suites (default)");
                println!("  security - Run only security validation");
                println!("  perf     - Run only performance benchmarks");
                return Ok(());
            }
        }
    } else {
        ValidationConfig::default()
    };

    // Check compilation status first
    println!("🔧 Checking compilation status...");
    let compile_result = Command::new("cargo")
        .args(&["check", "--workspace"])
        .output()?;

    if !compile_result.status.success() {
        println!("❌ Compilation check failed:");
        println!("{}", String::from_utf8_lossy(&compile_result.stderr));
        println!("\n⚠️  Note: Implementation appears incomplete. Running validation against current state.");
    } else {
        println!("✅ Compilation check passed");
    }

    // Run comprehensive validation
    let runner = ValidationRunner::with_config(config)?;
    let report = runner.run_validation().await?;

    // Exit with appropriate code
    match report.overall_status {
        tests::test_runner::ValidationStatus::Passed => {
            println!("\n🎉 All validations passed!");
            std::process::exit(0);
        },
        tests::test_runner::ValidationStatus::PartiallyPassed => {
            println!("\n⚠️  Some validations failed, but core functionality works");
            std::process::exit(1);
        },
        tests::test_runner::ValidationStatus::Failed => {
            println!("\n❌ Validation failed - critical issues found");
            std::process::exit(2);
        },
        tests::test_runner::ValidationStatus::NotRun => {
            println!("\n❓ Validation incomplete");
            std::process::exit(3);
        },
    }
}