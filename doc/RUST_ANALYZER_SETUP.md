# Rust Analyzer 配置说明

## 问题描述

在 examples 目录中使用 `use code_agent::service::*` 时，rust-analyzer 可能会报错找不到 `service` 模块。这是因为 `service` 模块在 `lib.rs` 中被 `#[cfg(feature = "service")]` 条件编译保护，需要启用 `service` feature 才能使用。

## 解决方案

我们提供了多种配置方式来让 rust-analyzer 识别 `service` feature：

### 方案 1：使用 `.zed/settings.json`（推荐，适用于 Zed 用户）✨

已创建 `.zed/settings.json` 文件：

```json
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "cargo": {
          "features": ["service"]
        }
      }
    }
  }
}
```

**优点**：
- Zed 用户开箱即用
- 配置清晰，易于理解
- 不影响其他编辑器用户

**重启方法**：
- 按 `Cmd+Shift+P` (macOS) 或 `Ctrl+Shift+P` (Windows/Linux)
- 输入 "zed: reload" 或 "editor: reload window"
- 或者直接重启 Zed

### 方案 2：使用 `.vscode/settings.json`（推荐，适用于 VSCode 用户）

已创建 `.vscode/settings.json` 文件：

```json
{
  "rust-analyzer.cargo.features": ["service"],
  "rust-analyzer.cargo.loadOutDirsFromCheck": true,
  "rust-analyzer.procMacro.enable": true
}
```

**优点**：
- VSCode 用户开箱即用
- 配置清晰，易于理解
- 不影响其他编辑器用户

**重启方法**：
- 按 `Cmd+Shift+P` (macOS) 或 `Ctrl+Shift+P` (Windows/Linux)
- 输入 "Rust Analyzer: Restart Server"
- 回车执行

### 方案 3：使用 `rust-analyzer.toml`（适用于其他编辑器）

已创建/更新 `rust-analyzer.toml` 文件：

```toml
# rust-analyzer configuration for Zed and other editors
# This file configures rust-analyzer to enable the "service" feature by default
# so that it can properly analyze code that depends on service modules

# Enable the "service" feature for all crates in the workspace
cargo.features = "all"
```

**优点**：
- 适用于支持 rust-analyzer 的编辑器（Vim, Emacs, Helix, etc.）
- 项目级配置，所有开发者共享

**注意**：
- Zed 和 VSCode 应该使用各自的配置文件（`.zed/settings.json` 或 `.vscode/settings.json`）
- `rust-analyzer.toml` 的配置选项在不同编辑器中支持程度不同
- Zed 不支持 `cargo.loadOutDirsFromCheck` 和 `procMacro.enable` 等选项

### 方案 4：在 `examples/Cargo.toml` 中配置（已完成）

`examples/Cargo.toml` 已经正确配置：

```toml
[dependencies]
code-agent = { path = "..", features = ["service"] }
```

这确保了在编译 examples 时会启用 `service` feature。

## 验证配置

### 1. 重启 rust-analyzer

配置文件修改后，需要重启 rust-analyzer：

**Zed**：
- 按 `Cmd+Shift+P` (macOS) 或 `Ctrl+Shift+P` (Windows/Linux)
- 输入 "zed: reload" 或 "editor: reload window"
- 或者直接重启 Zed

**VSCode**：
- 按 `Cmd+Shift+P` (macOS) 或 `Ctrl+Shift+P` (Windows/Linux)
- 输入 "Rust Analyzer: Restart Server"
- 回车执行

**Vim/Neovim**：
```vim
:LspRestart
```

### 2. 验证编译

运行以下命令验证配置是否正确：

```bash
# 检查主项目（带 service feature）
cargo check --features service

# 检查 examples
cd examples
cargo check --example rust_client
cargo check --example http_client
cargo check --example in_process_service
```

### 3. 运行 examples

```bash
# 运行 Rust 客户端示例
cd examples
cargo run --example rust_client

# 运行 HTTP 客户端示例
cargo run --example http_client

# 运行进程内服务示例
cargo run --example in_process_service
```

## 常见问题

### Q1: 修改配置后 rust-analyzer 仍然报错？

**A**: 尝试以下步骤：
1. 重启 rust-analyzer（见上文）
2. 清理并重新构建：`cargo clean && cargo check --features service`
3. 重启编辑器
4. 删除 `target` 目录后重新构建

### Q2: Zed 报错 "unexpected field"？

**A**: Zed 的 rust-analyzer 配置格式与其他编辑器不同：
- ✅ 使用 `.zed/settings.json` 配置（推荐）
- ❌ 不要在 `rust-analyzer.toml` 中使用 `cargo.loadOutDirsFromCheck` 等 Zed 不支持的选项
- 如果仍有问题，删除 `rust-analyzer.toml`，只使用 `.zed/settings.json`

### Q3: 为什么需要 `service` feature？

**A**: `service` 模块包含 HTTP 服务相关的功能，依赖于 `axum`、`tower` 等较重的依赖。通过 feature 控制，可以让不需要服务功能的用户避免编译这些依赖，加快编译速度。

### Q4: 如何在自己的项目中使用 code-agent？

**A**: 在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
# 只使用核心功能
code-agent = "0.2"

# 或者启用 service 功能
code-agent = { version = "0.2", features = ["service"] }

# 或者启用所有功能
code-agent = { version = "0.2", features = ["full"] }
```

## 编辑器特定配置

### Zed 配置

Zed 使用 `.zed/settings.json` 配置 LSP：

```json
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "cargo": {
          "features": ["service"]
        }
      }
    }
  }
}
```

### VSCode 配置

VSCode 使用 `.vscode/settings.json` 配置：

```json
{
  "rust-analyzer.cargo.features": ["service"]
}
```

### Vim/Neovim 配置

对于使用 `nvim-lspconfig` 的用户：

```lua
require('lspconfig').rust_analyzer.setup({
  settings = {
    ['rust-analyzer'] = {
      cargo = {
        features = {"service"}
      }
    }
  }
})
```

### Helix 配置

在 `.helix/languages.toml` 中：

```toml
[[language]]
name = "rust"

[language.config.rust-analyzer]
cargo.features = ["service"]
```

## 配置文件优先级

当多个配置文件存在时，优先级为：
1. 编辑器特定配置（`.zed/settings.json`, `.vscode/settings.json` 等）
2. `rust-analyzer.toml`（项目根目录）
3. 用户全局配置

## 参考资料

- [rust-analyzer 配置文档](https://rust-analyzer.github.io/manual.html#configuration)
- [Zed rust-analyzer 配置](https://zed.dev/docs/languages/rust)
- [Cargo Features 文档](https://doc.rust-lang.org/cargo/reference/features.html)
- [code-agent 项目文档](../README.md)

