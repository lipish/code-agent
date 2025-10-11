//! Performance benchmarks for Arc optimization
//!
//! This benchmark compares the performance before and after Arc optimization.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Runtime;
use dashmap::DashMap;

// Simulate the old structure (before optimization)
#[derive(Clone, Debug)]
struct OldTaskContext {
    task_id: String,
    status: String,
    data: Vec<u8>,
}

// Simulate the new structure (after optimization)
#[derive(Clone, Debug)]
struct NewTaskContext {
    task_id: String,
    status: String,
    data: Vec<u8>,
}

// Helper functions to use all fields and avoid dead code warnings
impl OldTaskContext {
    fn simulate_work(&self) -> usize {
        // Use all fields to avoid dead code warnings
        self.task_id.len() + self.status.len() + self.data.len()
    }
}

impl NewTaskContext {
    fn simulate_work(&self) -> usize {
        // Use all fields to avoid dead code warnings
        self.task_id.len() + self.status.len() + self.data.len()
    }
}

fn bench_concurrent_reads(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_reads");
    
    for num_tasks in [10, 50, 100, 500].iter() {
        // Old approach: Arc<RwLock<HashMap<..., Arc<RwLock<...>>>>>
        group.bench_with_input(
            BenchmarkId::new("old_arc_rwlock", num_tasks),
            num_tasks,
            |b, &num_tasks| {
                let rt = Runtime::new().unwrap();
                let map = Arc::new(tokio::sync::RwLock::new(
                    std::collections::HashMap::<String, Arc<tokio::sync::RwLock<OldTaskContext>>>::new()
                ));
                
                // Populate
                rt.block_on(async {
                    let mut m = map.write().await;
                    for i in 0..num_tasks {
                        let ctx = OldTaskContext {
                            task_id: format!("task-{}", i),
                            status: "running".to_string(),
                            data: vec![0u8; 1024],
                        };
                        m.insert(format!("task-{}", i), Arc::new(tokio::sync::RwLock::new(ctx)));
                    }
                });
                
                b.to_async(&rt).iter(|| async {
                    let map = map.clone();
                    let tasks = map.read().await;
                    if let Some(task) = tasks.get("task-0") {
                        let ctx = task.read().await;
                        black_box(&ctx.status);
                        // Use other fields to avoid dead code warnings
                        black_box(ctx.simulate_work());
                    }
                });
            },
        );
        
        // New approach: Arc<DashMap<String, TaskContext>>
        group.bench_with_input(
            BenchmarkId::new("new_dashmap", num_tasks),
            num_tasks,
            |b, &num_tasks| {
                let rt = Runtime::new().unwrap();
                let map = Arc::new(DashMap::<String, NewTaskContext>::new());
                
                // Populate
                for i in 0..num_tasks {
                    let ctx = NewTaskContext {
                        task_id: format!("task-{}", i),
                        status: "running".to_string(),
                        data: vec![0u8; 1024],
                    };
                    map.insert(format!("task-{}", i), ctx);
                }
                
                b.to_async(&rt).iter(|| async {
                    if let Some(ctx) = map.get("task-0") {
                        black_box(&ctx.status);
                        // Use other fields to avoid dead code warnings
                        black_box(ctx.simulate_work());
                    }
                });
            },
        );
    }
    
    group.finish();
}

fn bench_concurrent_writes(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_writes");
    
    for num_tasks in [10, 50, 100].iter() {
        // Old approach
        group.bench_with_input(
            BenchmarkId::new("old_arc_rwlock", num_tasks),
            num_tasks,
            |b, &num_tasks| {
                let rt = Runtime::new().unwrap();
                let map = Arc::new(tokio::sync::RwLock::new(
                    std::collections::HashMap::<String, Arc<tokio::sync::RwLock<OldTaskContext>>>::new()
                ));
                
                // Populate
                rt.block_on(async {
                    let mut m = map.write().await;
                    for i in 0..num_tasks {
                        let ctx = OldTaskContext {
                            task_id: format!("task-{}", i),
                            status: "running".to_string(),
                            data: vec![0u8; 1024],
                        };
                        m.insert(format!("task-{}", i), Arc::new(tokio::sync::RwLock::new(ctx)));
                    }
                });
                
                b.to_async(&rt).iter(|| async {
                    let map = map.clone();
                    let tasks = map.write().await;
                    if let Some(task) = tasks.get("task-0") {
                        let mut ctx = task.write().await;
                        ctx.status = "completed".to_string();
                        // Use other fields to avoid dead code warnings
                        black_box(ctx.simulate_work());
                    }
                });
            },
        );
        
        // New approach
        group.bench_with_input(
            BenchmarkId::new("new_dashmap", num_tasks),
            num_tasks,
            |b, &num_tasks| {
                let rt = Runtime::new().unwrap();
                let map = Arc::new(DashMap::<String, NewTaskContext>::new());
                
                // Populate
                for i in 0..num_tasks {
                    let ctx = NewTaskContext {
                        task_id: format!("task-{}", i),
                        status: "running".to_string(),
                        data: vec![0u8; 1024],
                    };
                    map.insert(format!("task-{}", i), ctx);
                }
                
                b.to_async(&rt).iter(|| async {
                    if let Some(mut ctx) = map.get_mut("task-0") {
                        ctx.status = "completed".to_string();
                        // Use other fields to avoid dead code warnings
                        black_box(ctx.simulate_work());
                    }
                });
            },
        );
    }
    
    group.finish();
}

fn bench_memory_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_overhead");
    group.measurement_time(Duration::from_secs(10));
    
    // Old approach memory usage
    group.bench_function("old_arc_rwlock_1000", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            let map = Arc::new(tokio::sync::RwLock::new(
                std::collections::HashMap::<String, Arc<tokio::sync::RwLock<OldTaskContext>>>::new()
            ));
            
            rt.block_on(async {
                let mut m = map.write().await;
                for i in 0..1000 {
                    let ctx = OldTaskContext {
                        task_id: format!("task-{}", i),
                        status: "running".to_string(),
                        data: vec![0u8; 1024],
                    };
                    m.insert(format!("task-{}", i), Arc::new(tokio::sync::RwLock::new(ctx)));
                }
            });
            
            black_box(map);
        });
    });
    
    // New approach memory usage
    group.bench_function("new_dashmap_1000", |b| {
        b.iter(|| {
            let map = Arc::new(DashMap::<String, NewTaskContext>::new());
            
            for i in 0..1000 {
                let ctx = NewTaskContext {
                    task_id: format!("task-{}", i),
                    status: "running".to_string(),
                    data: vec![0u8; 1024],
                };
                map.insert(format!("task-{}", i), ctx);
            }
            
            black_box(map);
        });
    });
    
    group.finish();
}

fn bench_parallel_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_access");
    
    // Old approach
    group.bench_function("old_arc_rwlock_parallel", |b| {
        let rt = Runtime::new().unwrap();
        let map = Arc::new(tokio::sync::RwLock::new(
            std::collections::HashMap::<String, Arc<tokio::sync::RwLock<OldTaskContext>>>::new()
        ));
        
        // Populate
        rt.block_on(async {
            let mut m = map.write().await;
            for i in 0..100 {
                let ctx = OldTaskContext {
                    task_id: format!("task-{}", i),
                    status: "running".to_string(),
                    data: vec![0u8; 1024],
                };
                m.insert(format!("task-{}", i), Arc::new(tokio::sync::RwLock::new(ctx)));
            }
        });
        
        b.to_async(&rt).iter(|| async {
            let mut handles = vec![];
            for i in 0..10 {
                let map = map.clone();
                let handle = tokio::spawn(async move {
                    let tasks = map.read().await;
                    if let Some(task) = tasks.get(&format!("task-{}", i)) {
                        let ctx = task.read().await;
                        black_box(&ctx.status);
                        // Use other fields to avoid dead code warnings
                        black_box(ctx.simulate_work());
                    }
                });
                handles.push(handle);
            }
            for handle in handles {
                handle.await.unwrap();
            }
        });
    });
    
    // New approach
    group.bench_function("new_dashmap_parallel", |b| {
        let rt = Runtime::new().unwrap();
        let map = Arc::new(DashMap::<String, NewTaskContext>::new());
        
        // Populate
        for i in 0..100 {
            let ctx = NewTaskContext {
                task_id: format!("task-{}", i),
                status: "running".to_string(),
                data: vec![0u8; 1024],
            };
            map.insert(format!("task-{}", i), ctx);
        }
        
        b.to_async(&rt).iter(|| async {
            let mut handles = vec![];
            for i in 0..10 {
                let map = map.clone();
                let handle = tokio::spawn(async move {
                    if let Some(ctx) = map.get(&format!("task-{}", i)) {
                        black_box(&ctx.status);
                        // Use other fields to avoid dead code warnings
                        black_box(ctx.simulate_work());
                    }
                });
                handles.push(handle);
            }
            for handle in handles {
                handle.await.unwrap();
            }
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_concurrent_reads,
    bench_concurrent_writes,
    bench_memory_overhead,
    bench_parallel_access
);
criterion_main!(benches);

