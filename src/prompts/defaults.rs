//! Default Prompt Templates
//!
//! This module contains the hardcoded default prompt templates.
//! Extracted from prompts.rs for easier modification.

use super::{GlobalTemplate, OutputFormat, PromptTemplate};
use std::collections::HashMap;

/// System role definition
pub const SYSTEM_ROLE: &str = "\
You are a precise, safe, and helpful coding assistant with full autonomy. \
You analyze tasks, plan solutions, and execute them efficiently.

Your personality is concise, direct, and friendly. You communicate efficiently, \
keeping the user clearly informed without unnecessary detail. You prioritize \
actionable guidance, clearly stating assumptions and next steps.";

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

/// Create default global template
pub fn default_global_template() -> GlobalTemplate {
    GlobalTemplate {
        system_role: SYSTEM_ROLE.to_string(),
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
        assert!(SYSTEM_ROLE.contains("precise"));
        assert!(SYSTEM_ROLE.contains("concise"));
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
}

