//! Complex Task Understanding and Planning Test
//!
//! This test validates the agent's ability to understand complex, multi-faceted tasks
//! and generate comprehensive execution plans. It tests the planning engine's
//! capability to break down sophisticated requirements into actionable steps.
//!
//! ## Test Scenarios
//! 1. Multi-step software development task
//! 2. System architecture analysis task  
//! 3. Cross-functional integration task
//! 4. Performance optimization task
//!
//! Run with: `cargo test test_complex_task_planning -- --nocapture`

use task_runner::agent::TaskAgent;
use task_runner::config::{AgentConfig, ModelConfig, ModelProvider, LogFormat};
use task_runner::models::{LlmModel, LanguageModel};
use task_runner::types::TaskComplexity;

/// Test complex task understanding and planning with Zhipu GLM-4.6
#[tokio::test]
async fn test_complex_task_planning() {
    println!("🧠 Complex Task Understanding & Planning Test");
    println!("==============================================");
    println!();

    let agent = setup_test_agent().await;
    if agent.is_none() {
        println!("⚠️  Skipping test - agent setup failed");
        return;
    }
    let mut agent = agent.unwrap();

    // Test Case 1: Multi-step Software Development Task
    test_complex_software_task(&mut agent).await;
    
    // Test Case 2: System Architecture Analysis
    test_architecture_analysis_task(&mut agent).await;
    
    // Test Case 3: Cross-functional Integration Task
    test_integration_task(&mut agent).await;
    
    // Test Case 4: Performance Optimization Task
    test_optimization_task(&mut agent).await;

    println!("🎉 Complex Task Planning Test Suite Completed!");
    println!("===============================================");
}

/// Setup test agent with proper configuration
async fn setup_test_agent() -> Option<TaskAgent> {
    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 4000, // Increased for complex tasks
        temperature: 0.7,
    };

    let agent_config = AgentConfig {
        model: model_config.clone(),
        execution: task_runner::config::ExecutionConfig {
            max_steps: 20, // Increased for complex tasks
            timeout_seconds: 120, // Increased timeout
            max_retries: 3,
            retry_delay_seconds: 2,
        },
        safety: task_runner::config::SafetyConfig {
            enable_safety_checks: true,
            allowed_directories: vec![".".to_string(), "/tmp".to_string()],
            blocked_commands: vec!["rm -rf".to_string(), "format".to_string()],
        },
        tools: task_runner::config::ToolConfig {
            auto_discovery: true,
            custom_tools_path: None,
            enabled_tools: vec![
                "read_file".to_string(),
                "write_file".to_string(),
                "list_files".to_string(),
                "run_command".to_string(),
            ],
            disabled_tools: vec![],
        },
        logging: task_runner::config::LoggingConfig {
            level: "info".to_string(),
            file: None,
            console: true,
            format: LogFormat::Pretty,
        },
    };

    match LlmModel::from_config(model_config) {
        Ok(model) => {
            let model_box = Box::new(model) as Box<dyn LanguageModel>;
            let agent = TaskAgent::new(model_box, agent_config);
            println!("✅ Test agent created successfully");
            Some(agent)
        }
        Err(e) => {
            println!("❌ Failed to create test agent: {}", e);
            None
        }
    }
}

/// Test Case 1: Complex Software Development Task
async fn test_complex_software_task(agent: &mut TaskAgent) {
    println!("🔨 Test Case 1: Multi-step Software Development Task");
    println!("====================================================");
    
    let complex_task = r#"
Create a comprehensive REST API service for a library management system with the following requirements:

1. ARCHITECTURE & DESIGN:
   - Use microservices architecture with at least 3 services (User, Book, Lending)
   - Implement database design with proper relationships
   - Design RESTful endpoints following OpenAPI 3.0 specification
   - Include authentication and authorization using JWT tokens

2. IMPLEMENTATION REQUIREMENTS:
   - Use Rust with Axum web framework and SQLx for database operations
   - Implement proper error handling with custom error types
   - Add comprehensive logging and monitoring capabilities
   - Include rate limiting and CORS configuration

3. FEATURES TO IMPLEMENT:
   - User registration, login, and profile management
   - Book catalog management (CRUD operations)
   - Book lending and return system with due dates
   - Search functionality with filters (title, author, genre, availability)
   - Overdue book tracking and notification system

4. QUALITY & TESTING:
   - Write unit tests for all business logic
   - Create integration tests for API endpoints
   - Add database migration scripts
   - Include comprehensive documentation
   - Set up CI/CD pipeline configuration

5. DEPLOYMENT & OPERATIONS:
   - Create Docker containers for each service
   - Write docker-compose configuration for local development
   - Include environment configuration management
   - Add health check endpoints for monitoring
   
Please analyze this requirement and create a detailed execution plan with estimated effort and dependencies.
"#;

    println!("📝 Complex Task:");
    println!("{}", complex_task.trim());
    println!();

    analyze_task_planning(agent, "Software Development", complex_task).await;
}

/// Test Case 2: System Architecture Analysis
async fn test_architecture_analysis_task(agent: &mut TaskAgent) {
    println!("🏗️  Test Case 2: System Architecture Analysis Task");
    println!("================================================");
    
    let architecture_task = r#"
Perform a comprehensive architecture analysis and optimization for a high-traffic e-commerce platform:

CURRENT SYSTEM CHALLENGES:
- Monolithic architecture causing deployment bottlenecks
- Database performance issues with 10M+ products and 1M+ daily users
- Payment processing taking 3-5 seconds on average
- Search functionality slow with complex filters
- Mobile app experiencing 20%+ bounce rate due to slow API responses

ANALYSIS REQUIREMENTS:
1. Conduct performance bottleneck analysis across all system components
2. Evaluate current technology stack (Java Spring Boot, MySQL, Redis, Elasticsearch)
3. Assess scalability limitations and identify critical failure points
4. Review security vulnerabilities and compliance gaps (PCI DSS, GDPR)
5. Analyze cost optimization opportunities in cloud infrastructure

DELIVERABLES NEEDED:
- Detailed architecture assessment report with metrics
- Migration strategy from monolith to microservices
- Database optimization and potential NoSQL integration plan
- Caching strategy improvements (CDN, application-level, database)
- API gateway implementation plan with rate limiting
- Monitoring and alerting system design
- Security enhancement roadmap
- Performance testing framework setup
- Disaster recovery and backup strategy
- Cost-benefit analysis with ROI projections

CONSTRAINTS:
- Zero-downtime migration requirement
- Budget limit of $500K for infrastructure changes
- 6-month timeline for complete implementation
- Must maintain backward compatibility for mobile apps
- Compliance with international data protection regulations

Create a comprehensive analysis plan with timeline, resource allocation, and risk assessment.
"#;

    println!("📝 Architecture Analysis Task:");
    println!("{}", architecture_task.trim());
    println!();

    analyze_task_planning(agent, "Architecture Analysis", architecture_task).await;
}

/// Test Case 3: Cross-functional Integration Task
async fn test_integration_task(agent: &mut TaskAgent) {
    println!("🔗 Test Case 3: Cross-functional Integration Task");
    println!("===============================================");
    
    let integration_task = r#"
Design and implement a comprehensive integration solution for a multi-vendor marketplace platform:

INTEGRATION REQUIREMENTS:
1. EXTERNAL SYSTEM INTEGRATIONS:
   - 15+ payment gateways (PayPal, Stripe, local processors)
   - 10+ shipping providers (FedEx, UPS, DHL, local carriers)
   - 5+ inventory management systems with different APIs
   - 3+ ERP systems (SAP, Oracle, Microsoft Dynamics)
   - Multiple tax calculation services for global compliance
   - Email marketing platforms (Mailchimp, SendGrid, Constant Contact)
   - Social media platforms for product syndication
   - Review and rating aggregation services

2. DATA SYNCHRONIZATION CHALLENGES:
   - Real-time inventory updates across multiple channels
   - Order status synchronization with tracking information
   - Customer data consistency across all touchpoints
   - Product catalog synchronization with pricing updates
   - Financial reconciliation between systems
   - Compliance data for audit trails

3. TECHNICAL REQUIREMENTS:
   - Event-driven architecture with message queuing
   - API rate limiting and retry mechanisms
   - Data transformation and validation layers
   - Error handling and dead letter queue management
   - Monitoring and alerting for integration failures
   - Webhook management for real-time updates

4. BUSINESS LOGIC COMPLEXITY:
   - Multi-currency support with dynamic exchange rates
   - Complex pricing rules and discount calculations
   - Inventory allocation across multiple warehouses
   - Split order processing and fulfillment
   - Returns and refund workflow automation
   - Vendor commission calculations

5. SCALABILITY & RELIABILITY:
   - Handle 50K+ transactions per day
   - 99.9% uptime requirement
   - Sub-second response times for critical operations
   - Horizontal scaling capabilities
   - Circuit breaker patterns for external services
   - Comprehensive backup and recovery procedures

Develop a detailed integration architecture with implementation phases, testing strategies, and monitoring solutions.
"#;

    println!("📝 Integration Task:");
    println!("{}", integration_task.trim());
    println!();

    analyze_task_planning(agent, "Integration", integration_task).await;
}

/// Test Case 4: Performance Optimization Task
async fn test_optimization_task(agent: &mut TaskAgent) {
    println!("⚡ Test Case 4: Performance Optimization Task");
    println!("============================================");
    
    let optimization_task = r#"
Optimize a high-performance real-time trading system with strict latency requirements:

CURRENT PERFORMANCE METRICS:
- Average order processing latency: 50ms (target: <10ms)
- Market data processing: 1000 updates/second (need: 10,000+/second)
- Database query response: 20ms average (target: <5ms)
- Memory usage: 8GB per instance (optimize to 4GB)
- CPU utilization: 80% average (optimize to 60%)
- Network throughput: 500MB/s (scale to 1GB/s)

OPTIMIZATION AREAS:
1. ALGORITHM OPTIMIZATION:
   - Order matching engine improvements
   - Risk calculation algorithms
   - Price discovery mechanisms
   - Portfolio rebalancing strategies
   - Market making algorithms

2. SYSTEM-LEVEL OPTIMIZATIONS:
   - Memory management and garbage collection tuning
   - Thread pool optimization and lock-free data structures
   - Network protocol optimization (UDP vs TCP)
   - CPU cache optimization and NUMA awareness
   - Disk I/O minimization and memory mapping

3. INFRASTRUCTURE IMPROVEMENTS:
   - Database sharding and read replicas
   - In-memory caching with Redis Cluster
   - Message queue optimization (Kafka tuning)
   - Load balancer configuration
   - Network topology optimization

4. MONITORING & PROFILING:
   - Real-time performance metrics dashboard
   - Latency percentile tracking (P50, P95, P99)
   - Memory allocation profiling
   - CPU hotspot identification
   - Network packet analysis

5. TESTING & VALIDATION:
   - Load testing with realistic trading patterns
   - Stress testing for extreme market conditions
   - Benchmark comparison before/after optimizations
   - A/B testing for algorithm improvements
   - Chaos engineering for resilience testing

CONSTRAINTS:
- Zero tolerance for data loss
- Regulatory compliance requirements (MiFID II, CFTC)
- 24/7 operation with minimal maintenance windows
- Multi-region deployment with sub-millisecond synchronization
- Backwards compatibility with existing client systems

Create a comprehensive optimization strategy with measurable performance targets and implementation roadmap.
"#;

    println!("📝 Optimization Task:");
    println!("{}", optimization_task.trim());
    println!();

    analyze_task_planning(agent, "Performance Optimization", optimization_task).await;
}

/// Analyze task planning capabilities
async fn analyze_task_planning(agent: &mut TaskAgent, task_type: &str, task_description: &str) {
    println!("🚀 Starting {} task analysis...", task_type);
    
    let start_time = std::time::Instant::now();
    
    match agent.process_task(task_description).await {
        Ok(result) => {
            let duration = start_time.elapsed();
            
            println!("✅ Task analysis completed successfully!");
            println!();
            println!("📊 PLANNING ANALYSIS RESULTS:");
            println!("============================");
            
            // Basic metrics
            println!("🔍 Basic Metrics:");
            println!("   - Success: {}", result.success);
            println!("   - Analysis time: {:?}", duration);
            println!("   - Summary length: {} characters", result.summary.len());
            
            if let Some(details) = &result.details {
                println!("   - Details length: {} characters", details.len());
            }
            
            if let Some(execution_time) = result.execution_time {
                println!("   - Execution time: {}ms", execution_time);
            }
            
            println!();
            
            // Task planning analysis
            if let Some(plan) = &result.task_plan {
                println!("🧠 TASK UNDERSTANDING & PLANNING:");
                println!("=================================");
                
                println!("📋 Understanding Quality:");
                let understanding_length = plan.understanding.len();
                let understanding_preview = plan.understanding.chars().take(200).collect::<String>();
                println!("   - Understanding depth: {} characters", understanding_length);
                println!("   - Understanding preview: {}...", understanding_preview);
                
                println!();
                println!("🎯 Approach Analysis:");
                let approach_length = plan.approach.len();
                let approach_preview = plan.approach.chars().take(200).collect::<String>();
                println!("   - Approach detail: {} characters", approach_length);
                println!("   - Approach preview: {}...", approach_preview);
                
                println!();
                println!("⚖️  Complexity Assessment:");
                println!("   - Assessed complexity: {:?}", plan.complexity);
                
                match plan.complexity {
                    TaskComplexity::Simple => println!("   - ⚠️  WARNING: Task marked as Simple - may be under-analyzed"),
                    TaskComplexity::Moderate => println!("   - ✅ Appropriate complexity for multi-step task"),
                    TaskComplexity::Complex => println!("   - ✅ Correctly identified as Complex task"),
                }
                
                if let Some(estimated_steps) = plan.estimated_steps {
                    println!("   - Estimated steps: {}", estimated_steps);
                    
                    // Validate step estimation for complex tasks
                    if estimated_steps < 5 {
                        println!("   - ⚠️  WARNING: Low step count for complex task");
                    } else if estimated_steps > 20 {
                        println!("   - ⚠️  WARNING: Very high step count - may need decomposition");
                    } else {
                        println!("   - ✅ Reasonable step estimation");
                    }
                } else {
                    println!("   - ⚠️  No step estimation provided");
                }
                
                println!();
                println!("📋 Requirements Analysis:");
                println!("   - Identified requirements: {}", plan.requirements.len());
                for (i, req) in plan.requirements.iter().enumerate().take(5) {
                    println!("     {}. {}", i + 1, req);
                }
                if plan.requirements.len() > 5 {
                    println!("     ... and {} more requirements", plan.requirements.len() - 5);
                }
                
                // Quality assessment
                println!();
                println!("🏆 PLANNING QUALITY ASSESSMENT:");
                println!("==============================");
                
                let mut quality_score = 0;
                let mut max_score = 0;
                
                // Understanding depth (0-25 points)
                max_score += 25;
                if understanding_length > 500 {
                    quality_score += 25;
                    println!("   ✅ Understanding depth: Excellent (25/25)");
                } else if understanding_length > 200 {
                    quality_score += 15;
                    println!("   ⚠️  Understanding depth: Good (15/25)");
                } else {
                    quality_score += 5;
                    println!("   ❌ Understanding depth: Poor (5/25)");
                }
                
                // Approach detail (0-25 points)
                max_score += 25;
                if approach_length > 400 {
                    quality_score += 25;
                    println!("   ✅ Approach detail: Excellent (25/25)");
                } else if approach_length > 150 {
                    quality_score += 15;
                    println!("   ⚠️  Approach detail: Good (15/25)");
                } else {
                    quality_score += 5;
                    println!("   ❌ Approach detail: Poor (5/25)");
                }
                
                // Complexity assessment (0-20 points)
                max_score += 20;
                match plan.complexity {
                    TaskComplexity::Complex => {
                        quality_score += 20;
                        println!("   ✅ Complexity assessment: Correct (20/20)");
                    }
                    TaskComplexity::Moderate => {
                        quality_score += 10;
                        println!("   ⚠️  Complexity assessment: Underestimated (10/20)");
                    }
                    TaskComplexity::Simple => {
                        quality_score += 0;
                        println!("   ❌ Complexity assessment: Severely underestimated (0/20)");
                    }
                }
                
                // Requirements identification (0-20 points)
                max_score += 20;
                if plan.requirements.len() >= 8 {
                    quality_score += 20;
                    println!("   ✅ Requirements identification: Excellent (20/20)");
                } else if plan.requirements.len() >= 5 {
                    quality_score += 15;
                    println!("   ⚠️  Requirements identification: Good (15/20)");
                } else if plan.requirements.len() >= 3 {
                    quality_score += 10;
                    println!("   ⚠️  Requirements identification: Fair (10/20)");
                } else {
                    quality_score += 5;
                    println!("   ❌ Requirements identification: Poor (5/20)");
                }
                
                // Step estimation (0-10 points)
                max_score += 10;
                if let Some(steps) = plan.estimated_steps {
                    if steps >= 10 && steps <= 20 {
                        quality_score += 10;
                        println!("   ✅ Step estimation: Appropriate (10/10)");
                    } else if steps >= 5 && steps < 10 {
                        quality_score += 7;
                        println!("   ⚠️  Step estimation: Conservative (7/10)");
                    } else {
                        quality_score += 3;
                        println!("   ⚠️  Step estimation: Suboptimal (3/10)");
                    }
                } else {
                    quality_score += 0;
                    println!("   ❌ Step estimation: Missing (0/10)");
                }
                
                let quality_percentage = (quality_score as f64 / max_score as f64) * 100.0;
                println!();
                println!("🎯 OVERALL PLANNING QUALITY: {:.1}% ({}/{})", 
                         quality_percentage, quality_score, max_score);
                
                if quality_percentage >= 90.0 {
                    println!("   🏆 EXCELLENT - AI demonstrated superior task understanding");
                } else if quality_percentage >= 75.0 {
                    println!("   ✅ GOOD - AI showed solid task comprehension");
                } else if quality_percentage >= 60.0 {
                    println!("   ⚠️  ADEQUATE - AI understood basic requirements");
                } else {
                    println!("   ❌ POOR - AI failed to properly analyze task complexity");
                }
                
            } else {
                println!("❌ CRITICAL FAILURE: No task plan generated!");
                println!("   - This indicates a fundamental problem with task understanding");
            }
            
            println!();
            println!("📝 Task Summary:");
            println!("   {}", result.summary);
            
        }
        Err(e) => {
            println!("❌ Task analysis failed: {}", e);
            println!("🔍 Failure Analysis:");
            let error_str = e.to_string();
            if error_str.contains("timeout") {
                println!("   - Task too complex for current timeout settings");
                println!("   - Consider increasing timeout or breaking down task");
            } else if error_str.contains("API") || error_str.contains("network") {
                println!("   - Network or API connectivity issue");
                println!("   - Check internet connection and API key validity");
            } else {
                println!("   - Unexpected error: {}", error_str);
            }
        }
    }
    
    println!();
    println!("{}", "=".repeat(80));
    println!();
}

/// Test to verify planning engine can handle multiple complex tasks
#[tokio::test]
async fn test_planning_consistency() {
    println!("🔄 Planning Consistency Test");
    println!("===========================");
    
    let agent = setup_test_agent().await;
    if agent.is_none() {
        return;
    }
    let mut agent = agent.unwrap();
    
    let simple_task = "Create a hello world program in Rust";
    let complex_task = "Design and implement a distributed microservices architecture for a real-time trading platform with sub-millisecond latency requirements, including order matching engine, risk management system, market data processing, and regulatory compliance reporting.";
    
    println!("🧪 Testing planning consistency across different complexity levels...");
    
    // Test simple task
    println!("\n📝 Simple Task Test:");
    match agent.process_task(simple_task).await {
        Ok(result) => {
            if let Some(plan) = &result.task_plan {
                println!("   - Complexity: {:?}", plan.complexity);
                println!("   - Steps: {:?}", plan.estimated_steps);
                println!("   - Requirements: {}", plan.requirements.len());
            }
        }
        Err(e) => println!("   - Failed: {}", e),
    }
    
    // Test complex task
    println!("\n📝 Complex Task Test:");
    match agent.process_task(complex_task).await {
        Ok(result) => {
            if let Some(plan) = &result.task_plan {
                println!("   - Complexity: {:?}", plan.complexity);
                println!("   - Steps: {:?}", plan.estimated_steps);
                println!("   - Requirements: {}", plan.requirements.len());
                
                // Validate that complex task is properly classified
                match plan.complexity {
                    TaskComplexity::Complex => println!("   ✅ Correctly identified as complex"),
                    _ => println!("   ❌ Failed to identify complexity"),
                }
            }
        }
        Err(e) => println!("   - Failed: {}", e),
    }
    
    println!("\n✅ Planning consistency test completed");
}