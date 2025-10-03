# Zed 配置说明

这个目录包含 Zed 编辑器的项目级配置。

## settings.json

配置 rust-analyzer 启用 `service` feature，使其能够识别 `code_agent::service` 模块。

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

## 重启 rust-analyzer

配置修改后，需要重启 rust-analyzer：

1. 按 `Cmd+Shift+P` (macOS)
2. 输入 `zed: reload`
3. 按回车

## 验证

打开 `examples/rust_client.rs`，检查是否能识别：

```rust
use code_agent::service::{...};
```

## 更多信息

查看项目根目录的 `ZED_SETUP.md` 文件。

