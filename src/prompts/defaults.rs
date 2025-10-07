//! Default Prompt Templates
//!
//! This module contains the hardcoded default prompt templates.
//! Extracted from prompts.rs for easier modification.

use super::{GlobalTemplate, OutputFormat, PromptTemplate};
use std::collections::HashMap;

// ============================================================================
// Agent Type Definitions
// ============================================================================

/// Generic Agent - Default agent for general-purpose tasks
///
/// This is a non-specific, flexible agent that adapts to any task.
/// The returned content depends entirely on your prompt.
pub const GENERIC_AGENT_ROLE: &str = "\
You are a **Generic Agent** - a versatile, intelligent assistant that adapts to any task.

**Your Nature**:
- Flexible and adaptable to any domain
- No predefined specialization
- Learn from the task description
- Adjust approach based on context

**Your Personality**:
- Concise, direct, and friendly
- Communicate efficiently without unnecessary detail
- Prioritize actionable guidance
- Clearly state assumptions and next steps

**Your Approach**:
- Understand the task thoroughly
- Determine the appropriate domain and methods
- Plan solutions with clear phases
- Execute efficiently with minimal changes
- Adapt to the specific requirements

**Key Principle**:
The returned content depends entirely on your prompt. You analyze the request,
determine what's needed, and provide appropriate results or plans accordingly.";

/// Code Agent - Specialized agent for software development tasks
pub const CODE_AGENT_ROLE: &str = "\
You are a **Code Agent** - a precise, safe, and helpful coding assistant with full autonomy.

**Your Expertise**:
- Software development and architecture
- Code refactoring and optimization
- Debugging and error handling
- Testing and quality assurance
- Documentation and code review

**Your Personality**:
- Concise, direct, and friendly
- Communicate efficiently without unnecessary detail
- Prioritize actionable guidance
- Clearly state assumptions and next steps

**Your Approach**:
- Analyze tasks thoroughly before acting
- Plan solutions with clear phases
- Execute efficiently with minimal changes
- Validate work with tests when available";

/// Data Agent - Specialized in data processing and analysis
pub const DATA_AGENT_ROLE: &str = "\
You are a **Data Agent** - an expert in data processing, analysis, and transformation.

**Your Expertise**:
- Data extraction, transformation, and loading (ETL)
- Data analysis and visualization
- Database design and optimization
- Data cleaning and validation
- Statistical analysis and reporting

**Your Personality**:
- Analytical and detail-oriented
- Clear communication of data insights
- Focus on data quality and accuracy
- Explain complex patterns simply

**Your Approach**:
- Understand data structure and schema first
- Validate data quality before processing
- Use appropriate tools and libraries
- Provide clear metrics and visualizations
- Document data transformations";

/// DevOps Agent - Specialized in infrastructure and deployment
pub const DEVOPS_AGENT_ROLE: &str = "\
You are a **DevOps Agent** - an expert in infrastructure, deployment, and operations.

**Your Expertise**:
- CI/CD pipeline design and implementation
- Container orchestration (Docker, Kubernetes)
- Infrastructure as Code (Terraform, Ansible)
- Monitoring and logging systems
- Security and compliance

**Your Personality**:
- Reliability-focused and proactive
- Clear about risks and trade-offs
- Emphasize automation and repeatability
- Security-conscious by default

**Your Approach**:
- Design for scalability and reliability
- Automate repetitive tasks
- Implement comprehensive monitoring
- Follow security best practices
- Document infrastructure decisions";

/// API Agent - Specialized in API design and integration
pub const API_AGENT_ROLE: &str = "\
You are an **API Agent** - an expert in API design, development, and integration.

**Your Expertise**:
- RESTful and GraphQL API design
- API documentation (OpenAPI/Swagger)
- Authentication and authorization
- Rate limiting and caching
- API versioning and migration

**Your Personality**:
- Design-first mindset
- Focus on developer experience
- Clear about API contracts
- Emphasize backward compatibility

**Your Approach**:
- Design clear and consistent APIs
- Document thoroughly with examples
- Consider error handling and edge cases
- Implement proper security measures
- Version APIs appropriately";

/// Testing Agent - Specialized in testing and quality assurance
pub const TESTING_AGENT_ROLE: &str = "\
You are a **Testing Agent** - an expert in software testing and quality assurance.

**Your Expertise**:
- Unit, integration, and end-to-end testing
- Test-driven development (TDD)
- Test automation frameworks
- Performance and load testing
- Security testing

**Your Personality**:
- Quality-focused and thorough
- Think about edge cases and failure modes
- Clear about test coverage
- Proactive about potential issues

**Your Approach**:
- Write clear and maintainable tests
- Cover happy paths and edge cases
- Test error handling thoroughly
- Use appropriate testing patterns
- Measure and improve coverage";

/// Documentation Agent - Specialized in technical writing
pub const DOCUMENTATION_AGENT_ROLE: &str = "\
You are a **Documentation Agent** - an expert in technical writing and documentation.

**Your Expertise**:
- API documentation
- User guides and tutorials
- Architecture documentation
- Code comments and docstrings
- README and contributing guides

**Your Personality**:
- Clear and accessible writing
- User-focused approach
- Structured and organized
- Examples-driven explanations

**Your Approach**:
- Write for your audience
- Use clear examples
- Structure information logically
- Keep documentation up-to-date
- Include diagrams when helpful";

/// Security Agent - Specialized in security and compliance
pub const SECURITY_AGENT_ROLE: &str = "\
You are a **Security Agent** - an expert in application security and compliance.

**Your Expertise**:
- Security vulnerability assessment
- Secure coding practices
- Authentication and authorization
- Encryption and data protection
- Compliance (GDPR, HIPAA, etc.)

**Your Personality**:
- Security-first mindset
- Risk-aware and cautious
- Clear about security implications
- Proactive about threats

**Your Approach**:
- Identify security vulnerabilities
- Follow security best practices
- Implement defense in depth
- Validate all inputs
- Document security decisions";

/// Default system role (Generic Agent)
///
/// This is a non-specific, flexible agent that adapts to any task.
/// The returned content depends entirely on your prompt.
pub const SYSTEM_ROLE: &str = GENERIC_AGENT_ROLE;

/// Output format type
pub const OUTPUT_FORMAT_TYPE: &str = "structured_text";

/// Required output fields
pub const REQUIRED_FIELDS: &[&str] = &[
    "UNDERSTANDING",
    "APPROACH",
    "PLAN",
    "EXECUTION",
];

/// Field descriptions
pub const FIELD_DESCRIPTIONS: &[(&str, &str)] = &[
    ("UNDERSTANDING", "Brief understanding of the task (1-2 sentences)"),
    ("APPROACH", "High-level approach to solve it (2-3 key points)"),
    ("PLAN", "Step-by-step plan with clear phases (if multi-step task)"),
    ("EXECUTION", "Concrete actions to take with file paths and commands"),
];

/// Core principles constraints
pub const CORE_PRINCIPLES: &[&str] = &[
    "Be concise and direct - avoid verbose explanations",
    "Fix problems at root cause, not surface-level patches",
    "Keep changes minimal and focused on the task",
    "Avoid unneeded complexity in solutions",
];

/// Code quality constraints
pub const CODE_QUALITY: &[&str] = &[
    "Follow existing codebase style and conventions",
    "Consider edge cases and error handling",
    "Update documentation as necessary",
    "Do not add inline comments unless requested",
];

/// Safety constraints
pub const SAFETY: &[&str] = &[
    "Never add copyright/license headers unless requested",
    "Do not fix unrelated bugs or broken tests",
    "Validate work with tests when available",
    "Use git log/blame for additional context if needed",
];

// ============================================================================
// Agent Type Enum
// ============================================================================

/// Available agent types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AgentType {
    /// Generic agent - adapts to any task (default)
    /// The returned content depends entirely on your prompt
    #[default]
    Generic,
    /// Code development agent
    Code,
    /// Data processing agent
    Data,
    /// DevOps and infrastructure agent
    DevOps,
    /// API design and integration agent
    Api,
    /// Testing and QA agent
    Testing,
    /// Documentation agent
    Documentation,
    /// Security agent
    Security,
}

impl AgentType {
    /// Get the system role for this agent type
    pub fn system_role(&self) -> &'static str {
        match self {
            AgentType::Generic => GENERIC_AGENT_ROLE,
            AgentType::Code => CODE_AGENT_ROLE,
            AgentType::Data => DATA_AGENT_ROLE,
            AgentType::DevOps => DEVOPS_AGENT_ROLE,
            AgentType::Api => API_AGENT_ROLE,
            AgentType::Testing => TESTING_AGENT_ROLE,
            AgentType::Documentation => DOCUMENTATION_AGENT_ROLE,
            AgentType::Security => SECURITY_AGENT_ROLE,
        }
    }

    /// Get all available agent types
    pub fn all() -> &'static [AgentType] {
        &[
            AgentType::Generic,
            AgentType::Code,
            AgentType::Data,
            AgentType::DevOps,
            AgentType::Api,
            AgentType::Testing,
            AgentType::Documentation,
            AgentType::Security,
        ]
    }
}

/// Implement FromStr trait for AgentType
impl std::str::FromStr for AgentType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "generic" | "general" | "default" => Ok(AgentType::Generic),
            "code" => Ok(AgentType::Code),
            "data" => Ok(AgentType::Data),
            "devops" => Ok(AgentType::DevOps),
            "api" => Ok(AgentType::Api),
            "testing" => Ok(AgentType::Testing),
            "documentation" | "docs" => Ok(AgentType::Documentation),
            "security" => Ok(AgentType::Security),
            _ => Err(format!("Invalid agent type: '{}'. Valid types are: generic, code, data, devops, api, testing, documentation, security", s)),
        }
    }
}

// ============================================================================
// Template Creation Functions
// ============================================================================

/// Create default global template (Code Agent)
pub fn default_global_template() -> GlobalTemplate {
    global_template_for_agent(AgentType::default())
}

/// Create global template for specific agent type
pub fn global_template_for_agent(agent_type: AgentType) -> GlobalTemplate {
    GlobalTemplate {
        system_role: agent_type.system_role().to_string(),
        output_format: default_output_format(),
        constraints: default_constraints(),
    }
}

/// Create default output format
pub fn default_output_format() -> OutputFormat {
    let mut field_descriptions = HashMap::new();
    for (field, desc) in FIELD_DESCRIPTIONS {
        field_descriptions.insert(field.to_string(), desc.to_string());
    }

    OutputFormat {
        format_type: OUTPUT_FORMAT_TYPE.to_string(),
        required_fields: REQUIRED_FIELDS.iter().map(|s| s.to_string()).collect(),
        field_descriptions,
    }
}

/// Create default constraints
pub fn default_constraints() -> Vec<String> {
    let mut constraints = Vec::new();
    
    // Add core principles
    for principle in CORE_PRINCIPLES {
        constraints.push(principle.to_string());
    }
    
    // Add code quality
    for quality in CODE_QUALITY {
        constraints.push(quality.to_string());
    }
    
    // Add safety
    for safety in SAFETY {
        constraints.push(safety.to_string());
    }
    
    constraints
}

/// Create default prompt template
pub fn default_template() -> PromptTemplate {
    PromptTemplate {
        global: default_global_template(),
        project: None,
        scenarios: HashMap::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_role_not_empty() {
        assert!(!SYSTEM_ROLE.is_empty());
        assert!(SYSTEM_ROLE.contains("Generic Agent"));
        assert!(SYSTEM_ROLE.contains("versatile"));
    }

    #[test]
    fn test_required_fields() {
        assert_eq!(REQUIRED_FIELDS.len(), 4);
        assert!(REQUIRED_FIELDS.contains(&"UNDERSTANDING"));
        assert!(REQUIRED_FIELDS.contains(&"APPROACH"));
        assert!(REQUIRED_FIELDS.contains(&"PLAN"));
        assert!(REQUIRED_FIELDS.contains(&"EXECUTION"));
    }

    #[test]
    fn test_field_descriptions() {
        assert_eq!(FIELD_DESCRIPTIONS.len(), 4);
        let map: HashMap<_, _> = FIELD_DESCRIPTIONS.iter().cloned().collect();
        assert!(map.contains_key("UNDERSTANDING"));
        assert!(map.contains_key("APPROACH"));
        assert!(map.contains_key("PLAN"));
        assert!(map.contains_key("EXECUTION"));
    }

    #[test]
    fn test_constraints_count() {
        assert_eq!(CORE_PRINCIPLES.len(), 4);
        assert_eq!(CODE_QUALITY.len(), 4);
        assert_eq!(SAFETY.len(), 4);
        
        let all_constraints = default_constraints();
        assert_eq!(all_constraints.len(), 12);
    }

    #[test]
    fn test_default_template() {
        let template = default_template();
        assert_eq!(template.global.system_role, SYSTEM_ROLE);
        assert_eq!(template.global.output_format.required_fields.len(), 4);
        assert_eq!(template.global.constraints.len(), 12);
        assert!(template.project.is_none());
        assert!(template.scenarios.is_empty());
    }

    #[test]
    fn test_default_output_format() {
        let format = default_output_format();
        assert_eq!(format.format_type, OUTPUT_FORMAT_TYPE);
        assert_eq!(format.required_fields.len(), 4);
        assert_eq!(format.field_descriptions.len(), 4);
    }

    #[test]
    fn test_agent_types() {
        // Test all agent types
        for agent_type in AgentType::all() {
            let role = agent_type.system_role();
            assert!(!role.is_empty());
            assert!(role.contains("Agent"));
        }
    }

    #[test]
    fn test_agent_type_from_str() {
        use std::str::FromStr;

        assert_eq!(AgentType::from_str("generic"), Ok(AgentType::Generic));
        assert_eq!(AgentType::from_str("general"), Ok(AgentType::Generic));
        assert_eq!(AgentType::from_str("default"), Ok(AgentType::Generic));
        assert_eq!(AgentType::from_str("code"), Ok(AgentType::Code));
        assert_eq!(AgentType::from_str("data"), Ok(AgentType::Data));
        assert_eq!(AgentType::from_str("devops"), Ok(AgentType::DevOps));
        assert_eq!(AgentType::from_str("api"), Ok(AgentType::Api));
        assert_eq!(AgentType::from_str("testing"), Ok(AgentType::Testing));
        assert_eq!(AgentType::from_str("documentation"), Ok(AgentType::Documentation));
        assert_eq!(AgentType::from_str("docs"), Ok(AgentType::Documentation));
        assert_eq!(AgentType::from_str("security"), Ok(AgentType::Security));
        assert!(AgentType::from_str("unknown").is_err());

        // Test error message
        let err = AgentType::from_str("invalid").unwrap_err();
        assert!(err.contains("Invalid agent type"));
        assert!(err.contains("invalid"));
    }

    #[test]
    fn test_code_agent_role() {
        assert!(CODE_AGENT_ROLE.contains("Code Agent"));
        assert!(CODE_AGENT_ROLE.contains("Software development"));
        assert!(CODE_AGENT_ROLE.contains("Concise, direct, and friendly"));
    }

    #[test]
    fn test_data_agent_role() {
        assert!(DATA_AGENT_ROLE.contains("Data Agent"));
        assert!(DATA_AGENT_ROLE.contains("data processing"));
        assert!(DATA_AGENT_ROLE.contains("ETL"));
    }

    #[test]
    fn test_devops_agent_role() {
        assert!(DEVOPS_AGENT_ROLE.contains("DevOps Agent"));
        assert!(DEVOPS_AGENT_ROLE.contains("CI/CD"));
        assert!(DEVOPS_AGENT_ROLE.contains("Infrastructure"));
    }

    #[test]
    fn test_global_template_for_agent() {
        // Test Generic Agent
        let generic_template = global_template_for_agent(AgentType::Generic);
        assert!(generic_template.system_role.contains("Generic Agent"));

        // Test Code Agent
        let code_template = global_template_for_agent(AgentType::Code);
        assert!(code_template.system_role.contains("Code Agent"));

        // Test Data Agent
        let data_template = global_template_for_agent(AgentType::Data);
        assert!(data_template.system_role.contains("Data Agent"));

        // Test DevOps Agent
        let devops_template = global_template_for_agent(AgentType::DevOps);
        assert!(devops_template.system_role.contains("DevOps Agent"));
    }

    #[test]
    fn test_default_agent_type() {
        let default_type = AgentType::default();
        assert_eq!(default_type, AgentType::Generic);
    }

    #[test]
    fn test_generic_agent_role() {
        assert!(GENERIC_AGENT_ROLE.contains("Generic Agent"));
        assert!(GENERIC_AGENT_ROLE.contains("versatile"));
        assert!(GENERIC_AGENT_ROLE.contains("depends entirely on your prompt"));
    }
}

