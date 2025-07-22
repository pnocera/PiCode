use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// OpenAPI specification parser and validator
#[derive(Debug, Clone)]
pub struct OpenApiSpec {
    /// Raw OpenAPI specification
    pub spec: Value,
    /// Parsed API info
    pub info: ApiInfo,
    /// Available servers
    pub servers: Vec<Server>,
    /// API paths and operations
    pub paths: HashMap<String, PathItem>,
    /// Component schemas
    pub components: Option<Components>,
}

/// API information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiInfo {
    pub title: String,
    pub version: String,
    pub description: Option<String>,
    pub terms_of_service: Option<String>,
    pub contact: Option<Contact>,
    pub license: Option<License>,
}

/// Server information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub url: String,
    pub description: Option<String>,
    pub variables: Option<HashMap<String, ServerVariable>>,
}

/// Server variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerVariable {
    pub default: String,
    pub description: Option<String>,
    #[serde(rename = "enum")]
    pub enum_values: Option<Vec<String>>,
}

/// Contact information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub name: Option<String>,
    pub url: Option<String>,
    pub email: Option<String>,
}

/// License information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub name: String,
    pub url: Option<String>,
}

/// Path item containing HTTP operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathItem {
    pub get: Option<Operation>,
    pub post: Option<Operation>,
    pub put: Option<Operation>,
    pub delete: Option<Operation>,
    pub patch: Option<Operation>,
    pub head: Option<Operation>,
    pub options: Option<Operation>,
    pub trace: Option<Operation>,
    pub parameters: Option<Vec<Parameter>>,
}

/// HTTP operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub operation_id: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub parameters: Option<Vec<Parameter>>,
    pub request_body: Option<RequestBody>,
    pub responses: HashMap<String, Response>,
    pub tags: Option<Vec<String>>,
    pub security: Option<Vec<HashMap<String, Vec<String>>>>,
}

/// Parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "in")]
    pub location: String, // query, header, path, cookie
    pub description: Option<String>,
    pub required: Option<bool>,
    pub schema: Option<Schema>,
    pub style: Option<String>,
    pub explode: Option<bool>,
}

/// Request body definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestBody {
    pub description: Option<String>,
    pub content: HashMap<String, MediaType>,
    pub required: Option<bool>,
}

/// Response definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub description: String,
    pub headers: Option<HashMap<String, Header>>,
    pub content: Option<HashMap<String, MediaType>>,
}

/// Media type definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaType {
    pub schema: Option<Schema>,
    pub example: Option<Value>,
    pub examples: Option<HashMap<String, Example>>,
}

/// Header definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub description: Option<String>,
    pub required: Option<bool>,
    pub schema: Option<Schema>,
    pub style: Option<String>,
    pub explode: Option<bool>,
}

/// Example definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub value: Option<Value>,
    pub external_value: Option<String>,
}

/// Schema definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    #[serde(rename = "type")]
    pub schema_type: Option<String>,
    pub format: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub default: Option<Value>,
    pub example: Option<Value>,
    #[serde(rename = "enum")]
    pub enum_values: Option<Vec<Value>>,
    
    // Numeric constraints
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub exclusive_minimum: Option<bool>,
    pub exclusive_maximum: Option<bool>,
    pub multiple_of: Option<f64>,
    
    // String constraints
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub pattern: Option<String>,
    
    // Array constraints
    pub items: Option<Box<Schema>>,
    pub min_items: Option<usize>,
    pub max_items: Option<usize>,
    pub unique_items: Option<bool>,
    
    // Object constraints
    pub properties: Option<HashMap<String, Schema>>,
    pub additional_properties: Option<Box<Schema>>,
    pub required: Option<Vec<String>>,
    pub min_properties: Option<usize>,
    pub max_properties: Option<usize>,
    
    // Composition
    pub all_of: Option<Vec<Schema>>,
    pub any_of: Option<Vec<Schema>>,
    pub one_of: Option<Vec<Schema>>,
    pub not: Option<Box<Schema>>,
    
    // Reference
    #[serde(rename = "$ref")]
    pub reference: Option<String>,
}

/// Components section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Components {
    pub schemas: Option<HashMap<String, Schema>>,
    pub responses: Option<HashMap<String, Response>>,
    pub parameters: Option<HashMap<String, Parameter>>,
    pub examples: Option<HashMap<String, Example>>,
    pub request_bodies: Option<HashMap<String, RequestBody>>,
    pub headers: Option<HashMap<String, Header>>,
    pub security_schemes: Option<HashMap<String, SecurityScheme>>,
}

/// Security scheme definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityScheme {
    #[serde(rename = "type")]
    pub scheme_type: String,
    pub description: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "in")]
    pub location: Option<String>,
    pub scheme: Option<String>,
    pub bearer_format: Option<String>,
    pub flows: Option<OAuthFlows>,
    pub open_id_connect_url: Option<String>,
}

/// OAuth flows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthFlows {
    pub implicit: Option<OAuthFlow>,
    pub password: Option<OAuthFlow>,
    pub client_credentials: Option<OAuthFlow>,
    pub authorization_code: Option<OAuthFlow>,
}

/// OAuth flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthFlow {
    pub authorization_url: Option<String>,
    pub token_url: Option<String>,
    pub refresh_url: Option<String>,
    pub scopes: HashMap<String, String>,
}

impl OpenApiSpec {
    /// Parse OpenAPI specification from JSON
    pub fn from_json(json_str: &str) -> Result<Self> {
        let spec: Value = serde_json::from_str(json_str)?;
        Self::from_value(spec)
    }

    /// Parse OpenAPI specification from YAML
    pub fn from_yaml(yaml_str: &str) -> Result<Self> {
        let spec: Value = serde_yaml::from_str(yaml_str)?;
        Self::from_value(spec)
    }

    /// Parse OpenAPI specification from JSON value
    pub fn from_value(spec: Value) -> Result<Self> {
        // Parse info section
        let info: ApiInfo = serde_json::from_value(
            spec["info"].clone()
        ).map_err(|e| anyhow::anyhow!("Failed to parse info section: {}", e))?;

        // Parse servers section
        let servers: Vec<Server> = if let Some(servers_value) = spec.get("servers") {
            serde_json::from_value(servers_value.clone())
                .unwrap_or_else(|_| vec![])
        } else {
            vec![]
        };

        // Parse paths section
        let paths: HashMap<String, PathItem> = if let Some(paths_value) = spec.get("paths") {
            serde_json::from_value(paths_value.clone())
                .unwrap_or_else(|_| HashMap::new())
        } else {
            HashMap::new()
        };

        // Parse components section
        let components: Option<Components> = if let Some(components_value) = spec.get("components") {
            serde_json::from_value(components_value.clone()).ok()
        } else {
            None
        };

        Ok(Self {
            spec,
            info,
            servers,
            paths,
            components,
        })
    }

    /// Get all available operations
    pub fn get_operations(&self) -> Vec<(String, String, &Operation)> {
        let mut operations = Vec::new();

        for (path, path_item) in &self.paths {
            if let Some(op) = &path_item.get {
                operations.push((path.clone(), "GET".to_string(), op));
            }
            if let Some(op) = &path_item.post {
                operations.push((path.clone(), "POST".to_string(), op));
            }
            if let Some(op) = &path_item.put {
                operations.push((path.clone(), "PUT".to_string(), op));
            }
            if let Some(op) = &path_item.delete {
                operations.push((path.clone(), "DELETE".to_string(), op));
            }
            if let Some(op) = &path_item.patch {
                operations.push((path.clone(), "PATCH".to_string(), op));
            }
        }

        operations
    }

    /// Find operations by tag
    pub fn get_operations_by_tag(&self, tag: &str) -> Vec<(String, String, &Operation)> {
        self.get_operations()
            .into_iter()
            .filter(|(_, _, op)| {
                op.tags
                    .as_ref()
                    .map(|tags| tags.contains(&tag.to_string()))
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Get operation by operation ID
    pub fn get_operation_by_id(&self, operation_id: &str) -> Option<(String, String, &Operation)> {
        self.get_operations()
            .into_iter()
            .find(|(_, _, op)| {
                op.operation_id
                    .as_ref()
                    .map(|id| id == operation_id)
                    .unwrap_or(false)
            })
    }

    /// Validate that the specification is well-formed
    pub fn validate(&self) -> Result<Vec<String>> {
        let mut warnings = Vec::new();

        // Check for required fields
        if self.info.title.is_empty() {
            warnings.push("API title is empty".to_string());
        }

        if self.info.version.is_empty() {
            warnings.push("API version is empty".to_string());
        }

        // Check servers
        if self.servers.is_empty() {
            warnings.push("No servers defined".to_string());
        }

        // Check paths
        if self.paths.is_empty() {
            warnings.push("No paths defined".to_string());
        }

        // Check for operations without operation IDs
        for (path, method, op) in self.get_operations() {
            if op.operation_id.is_none() {
                warnings.push(format!("Operation {}:{} has no operationId", method, path));
            }
        }

        Ok(warnings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_minimal_spec() {
        let spec_json = r#"
        {
            "openapi": "3.0.0",
            "info": {
                "title": "Test API",
                "version": "1.0.0"
            },
            "paths": {}
        }
        "#;

        let spec = OpenApiSpec::from_json(spec_json).expect("Should parse minimal spec");
        assert_eq!(spec.info.title, "Test API");
        assert_eq!(spec.info.version, "1.0.0");
        assert!(spec.paths.is_empty());
    }

    #[test]
    fn test_validation() {
        let spec_json = r#"
        {
            "openapi": "3.0.0",
            "info": {
                "title": "",
                "version": ""
            },
            "paths": {}
        }
        "#;

        let spec = OpenApiSpec::from_json(spec_json).expect("Should parse spec");
        let warnings = spec.validate().expect("Should validate");
        
        assert!(warnings.contains(&"API title is empty".to_string()));
        assert!(warnings.contains(&"API version is empty".to_string()));
        assert!(warnings.contains(&"No servers defined".to_string()));
        assert!(warnings.contains(&"No paths defined".to_string()));
    }
}