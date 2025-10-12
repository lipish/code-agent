//! 将非结构化的 approach 字符串转换为结构化的执行步骤

use crate::types::*;
use regex::Regex;
use std::collections::HashMap;
use uuid::Uuid;

/// Approach 解析器
pub struct ApproachParser {
    /// 步骤识别模式
    step_patterns: Vec<Regex>,
    /// 文件操作模式
    file_patterns: Vec<Regex>,
    /// 命令执行模式
    command_patterns: Vec<Regex>,
    /// 编程语言识别
    language_patterns: HashMap<String, Regex>,
}

impl ApproachParser {
    /// 创建新的解析器
    pub fn new() -> Self {
        let step_patterns = vec![
            Regex::new(r"(?i)^\s*(\d+)[\.)]\s*(.+)$").unwrap(),  // "1. 步骤内容"
            Regex::new(r"(?i)^[-*]\s*(.+)$").unwrap(),            // "- 步骤内容"
            Regex::new(r"(?i)^Step\s+\d+:\s*(.+)$").unwrap(),     // "Step 1: 内容"
            Regex::new(r"(?i)^阶段\s*\d+[:：]\s*(.+)$").unwrap(),   // "阶段1：内容"
        ];

        let file_patterns = vec![
            Regex::new(r"(?i)(创建|生成|写入|create|write|generate).+(文件|file)").unwrap(),
            Regex::new(r"(?i)(读取|读|read|load).+(文件|file|数据|data)").unwrap(),
            Regex::new(r"(?i)(修改|更新|编辑|update|modify|edit).+(文件|file)").unwrap(),
            Regex::new(r"(?i)(删除|移除|remove|delete).+(文件|file)").unwrap(),
        ];

        let command_patterns = vec![
            Regex::new(r"(?i)(运行|执行|调用|run|execute|call).+(命令|command|脚本|script)").unwrap(),
            Regex::new(r"(?i)(编译|构建|build|compile)").unwrap(),
            Regex::new(r"(?i)(测试|test|验证|verify)").unwrap(),
            Regex::new(r"(?i)(部署|deploy|发布|publish)").unwrap(),
        ];

        let mut language_patterns = HashMap::new();
        language_patterns.insert("python".to_string(), Regex::new(r"(?i)(python|py|\.py)").unwrap());
        language_patterns.insert("javascript".to_string(), Regex::new(r"(?i)(javascript|js|node|\.js)").unwrap());
        language_patterns.insert("rust".to_string(), Regex::new(r"(?i)(rust|cargo|\.rs)").unwrap());
        language_patterns.insert("java".to_string(), Regex::new(r"(?i)(java|maven|gradle|\.java)").unwrap());
        language_patterns.insert("go".to_string(), Regex::new(r"(?i)(golang|go|\.go)").unwrap());

        Self {
            step_patterns,
            file_patterns,
            command_patterns,
            language_patterns,
        }
    }

    /// 将 TaskPlan 转换为带有结构化步骤的 TaskPlan
    pub fn enhance_task_plan(&self, plan: &TaskPlan) -> TaskPlan {
        let mut enhanced_plan = plan.clone();
        
        // 解析 approach 字符串，提取步骤
        let steps = self.extract_steps(&plan.approach);
        
        let mut structured_steps = Vec::new();
        for (index, step_text) in steps.iter().enumerate() {
            let execution_step = self.create_structured_step(
                index,
                step_text,
                &plan.complexity,
            );
            structured_steps.push(execution_step);
        }
        
        // 添加步骤依赖关系
        let dependencies = self.create_dependencies(&structured_steps);
        
        // 更新计划
        enhanced_plan.structured_steps = Some(structured_steps);
        enhanced_plan.step_dependencies = Some(dependencies);
        enhanced_plan.estimated_steps = Some(enhanced_plan.structured_steps.as_ref().unwrap().len() as u32);
        
        enhanced_plan
    }

    /// 从 approach 文本中提取步骤
    fn extract_steps(&self, approach: &str) -> Vec<String> {
        let mut steps = Vec::new();
        let lines: Vec<&str> = approach.lines().collect();

        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            // 尝试匹配各种步骤模式
            for pattern in &self.step_patterns {
                if let Some(captures) = pattern.captures(trimmed) {
                    if let Some(step_content) = captures.get(captures.len() - 1) {
                        steps.push(step_content.as_str().trim().to_string());
                        break;
                    }
                }
            }
        }

        // 如果没有找到明确的步骤格式，将每个句子作为一个步骤
        if steps.is_empty() {
            steps = approach
                .split(&['.', '。', ';', '；'][..])
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty() && s.len() > 10) // 过滤掉太短的句子
                .collect();
        }

        steps
    }

    /// 创建单个结构化执行步骤
    fn create_structured_step(
        &self,
        index: usize,
        step_text: &str,
        complexity: &TaskComplexity,
    ) -> StructuredExecutionStep {
        let step_id = {
            let uuid_str = Uuid::new_v4().to_string();
            format!("step_{}", &uuid_str[..8])
        };
        let step_type = self.infer_step_type(step_text);
        
        let estimated_duration = match complexity {
            TaskComplexity::Simple => Some(5),   // 5分钟
            TaskComplexity::Moderate => Some(15), // 15分钟
            TaskComplexity::Complex => Some(30),  // 30分钟
        };

        StructuredExecutionStep {
            id: step_id,
            name: format!("步骤 {}", index + 1),
            description: step_text.to_string(),
            step_type,
            estimated_duration,
            preconditions: self.extract_preconditions(step_text),
            expected_outputs: self.extract_expected_outputs(step_text),
            validation_criteria: self.extract_validation_criteria(step_text),
            rollback_actions: vec![], // 可以根据步骤类型自动生成
        }
    }

    /// 推断步骤类型
    fn infer_step_type(&self, step_text: &str) -> StructuredStepType {
        let lower_text = step_text.to_lowercase();

        // 检查文件操作
        for pattern in &self.file_patterns {
            if pattern.is_match(&lower_text) {
                return self.infer_file_operation(&lower_text);
            }
        }

        // 检查命令执行
        for pattern in &self.command_patterns {
            if pattern.is_match(&lower_text) {
                return self.infer_command_execution(&lower_text);
            }
        }

        // 检查代码生成
        for (language, pattern) in &self.language_patterns {
            if pattern.is_match(&lower_text) {
                return StructuredStepType::CodeGeneration {
                    language: language.clone(),
                    template: None,
                    output_file: "generated_code".to_string(),
                    parameters: HashMap::new(),
                };
            }
        }

        // 检查数据分析
        if lower_text.contains("analyze") || lower_text.contains("分析") || 
           lower_text.contains("统计") || lower_text.contains("计算") {
            return StructuredStepType::DataAnalysis {
                input_sources: vec![],
                analysis_type: "general".to_string(),
                output_format: "report".to_string(),
                parameters: HashMap::new(),
            };
        }

        // 检查测试
        if lower_text.contains("test") || lower_text.contains("测试") || 
           lower_text.contains("验证") || lower_text.contains("检查") {
            return StructuredStepType::TestExecution {
                test_type: "unit".to_string(),
                test_files: vec![],
                test_framework: "auto".to_string(),
                parameters: HashMap::new(),
            };
        }

        // 检查配置
        if lower_text.contains("配置") || lower_text.contains("config") || 
           lower_text.contains("设置") || lower_text.contains("setup") {
            return StructuredStepType::SystemConfiguration {
                config_type: "general".to_string(),
                config_file: "config".to_string(),
                settings: HashMap::new(),
            };
        }

        // 默认为工具调用
        StructuredStepType::ToolInvocation {
            tool_name: "general_action".to_string(),
            parameters: {
                let mut params = HashMap::new();
                params.insert("description".to_string(), 
                              serde_json::Value::String(step_text.to_string()));
                params
            },
        }
    }

    /// 推断文件操作类型
    fn infer_file_operation(&self, text: &str) -> StructuredStepType {
        let operation_type = if text.contains("创建") || text.contains("create") || text.contains("生成") {
            FileOperationType::Create
        } else if text.contains("读取") || text.contains("read") || text.contains("load") {
            FileOperationType::Read
        } else if text.contains("修改") || text.contains("update") || text.contains("edit") {
            FileOperationType::Update
        } else if text.contains("删除") || text.contains("delete") || text.contains("remove") {
            FileOperationType::Delete
        } else if text.contains("复制") || text.contains("copy") {
            FileOperationType::Copy
        } else if text.contains("移动") || text.contains("move") {
            FileOperationType::Move
        } else if text.contains("搜索") || text.contains("search") || text.contains("查找") {
            FileOperationType::Search
        } else if text.contains("替换") || text.contains("replace") {
            FileOperationType::Replace
        } else {
            FileOperationType::Update
        };

        StructuredStepType::FileOperation {
            operation_type,
            file_path: "target_file".to_string(),
            parameters: HashMap::new(),
        }
    }

    /// 推断命令执行
    fn infer_command_execution(&self, text: &str) -> StructuredStepType {
        let command = if text.contains("编译") || text.contains("build") {
            "build".to_string()
        } else if text.contains("测试") || text.contains("test") {
            "test".to_string()
        } else if text.contains("运行") || text.contains("run") {
            "run".to_string()
        } else if text.contains("部署") || text.contains("deploy") {
            "deploy".to_string()
        } else {
            "execute".to_string()
        };

        StructuredStepType::CommandExecution {
            command,
            arguments: vec![],
            working_directory: None,
            environment: HashMap::new(),
        }
    }

    /// 提取前置条件
    fn extract_preconditions(&self, step_text: &str) -> Vec<String> {
        let mut preconditions = Vec::new();
        
        // 简单的前置条件提取逻辑
        if step_text.contains("after") || step_text.contains("之后") || 
           step_text.contains("完成") || step_text.contains("基于") {
            preconditions.push("Previous steps completed".to_string());
        }

        if step_text.contains("file") || step_text.contains("文件") {
            preconditions.push("Required files exist".to_string());
        }

        if step_text.contains("install") || step_text.contains("安装") {
            preconditions.push("Dependencies installed".to_string());
        }

        preconditions
    }

    /// 提取预期输出
    fn extract_expected_outputs(&self, step_text: &str) -> Vec<String> {
        let mut outputs = Vec::new();

        if step_text.contains("生成") || step_text.contains("create") || step_text.contains("generate") {
            outputs.push("Generated artifact".to_string());
        }

        if step_text.contains("报告") || step_text.contains("report") {
            outputs.push("Analysis report".to_string());
        }

        if step_text.contains("文件") || step_text.contains("file") {
            outputs.push("Output file".to_string());
        }

        if step_text.contains("结果") || step_text.contains("result") {
            outputs.push("Processing result".to_string());
        }

        outputs
    }

    /// 提取验证标准
    fn extract_validation_criteria(&self, step_text: &str) -> Vec<String> {
        let mut criteria = Vec::new();

        if step_text.contains("测试") || step_text.contains("test") {
            criteria.push("All tests pass".to_string());
        }

        if step_text.contains("验证") || step_text.contains("verify") || step_text.contains("检查") {
            criteria.push("Validation successful".to_string());
        }

        if step_text.contains("编译") || step_text.contains("compile") || step_text.contains("build") {
            criteria.push("Build successful".to_string());
        }

        if step_text.contains("部署") || step_text.contains("deploy") {
            criteria.push("Deployment successful".to_string());
        }

        // 默认验证标准
        if criteria.is_empty() {
            criteria.push("Step completed successfully".to_string());
        }

        criteria
    }

    /// 创建步骤间的依赖关系
    fn create_dependencies(&self, steps: &[StructuredExecutionStep]) -> Vec<StepDependency> {
        let mut dependencies = Vec::new();
        
        // 简单的顺序依赖：每个步骤依赖于前一个步骤
        for i in 1..steps.len() {
            let current_step_id = steps[i].id.clone();
            let previous_step_id = steps[i - 1].id.clone();

            dependencies.push(StepDependency {
                step_id: current_step_id,
                depends_on: previous_step_id,
                dependency_type: DependencyType::StrictDependency,
                condition: None,
            });
        }

        dependencies
    }
}

impl Default for ApproachParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhance_simple_approach() {
        let parser = ApproachParser::new();
        let plan = TaskPlan {
            understanding: "Test task".to_string(),
            approach: "1. Create a file\n2. Write content\n3. Test the file".to_string(),
            complexity: TaskComplexity::Simple,
            estimated_steps: Some(3),
            requirements: vec!["File creation".to_string()],
            structured_steps: None,
            step_dependencies: None,
            // Service layer fields with defaults
            steps: vec![],
            required_tools: vec![],
            estimated_time: None,
            created_at: None,
        };

        let enhanced_plan = parser.enhance_task_plan(&plan);
        assert!(enhanced_plan.has_structured_steps());
        assert_eq!(enhanced_plan.structured_steps.as_ref().unwrap().len(), 3);
        assert_eq!(enhanced_plan.step_dependencies.as_ref().unwrap().len(), 2); // 2 dependencies for 3 steps
    }

    #[test]
    fn test_step_type_inference() {
        let parser = ApproachParser::new();
        
        // Test file operation
        let file_step = parser.infer_step_type("Create a new configuration file");
        matches!(file_step, StructuredStepType::FileOperation { .. });

        // Test command execution
        let command_step = parser.infer_step_type("Run the build command");
        matches!(command_step, StructuredStepType::CommandExecution { .. });

        // Test code generation
        let code_step = parser.infer_step_type("Generate Python code for data processing");
        matches!(code_step, StructuredStepType::CodeGeneration { .. });
    }
}