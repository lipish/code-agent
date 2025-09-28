# Code Agent (Rust CLI, Roo‑Code inspired)

A terminal‑first code agent implemented in Rust, extracting the core "tool‑driven agent" architecture from Roo‑Code but without any VS Code dependency.

## Features
- Agent loop: plan → execute tool → report (LLM planning stubbed for now)
- Tool registry with clear schemas and safe execution
- Built‑in tools:
  - list_files, read_file, search_files, write_file, run_command
- Sandbox & safety
  - Working directory sandbox; path escape prevention
  - .gitignore and optional .rooignore respect
  - Approval gates for write/command actions (`--yes` to auto‑approve)

## Requirements
- Rust toolchain (stable)

## Build
- cargo build

## Run
- Start interactive REPL:
  - cargo run -- run
- One‑shot execution (will plan then execute one tool):
  - cargo run -- exec "your goal here"
- Preview plan only (prints a tool call JSON):
  - cargo run -- plan "your goal here"

Note: LLM planning is currently stubbed, so the agent will ask for a manual JSON plan. You can paste a JSON of the form:

```
{"tool":"list_files","args":{"glob":"**/*.rs","max":50}}
```

## CLI flags
- --model <MODEL>
  - LLM model name (planning is stubbed now)
- -y, --yes
  - Auto‑approve write/command actions
- --working-dir <PATH>
  - Sandbox root (defaults to current directory)

## Tool reference
- list_files
  - args: { glob?: string, max?: number }
  - Lists files under sandbox respecting ignores
- read_file
  - args: { path: string, start_line?: number, end_line?: number }
  - Reads UTF‑8 file content (1‑based optional line range)
- search_files
  - args: { pattern: string, max_results?: number }
  - Regex search across UTF‑8 files, honoring ignores
- write_file
  - args: { path: string, content: string }
  - Writes full content with backup .bak if file exists (requires approval)
- run_command
  - args: { cmd: string, timeout_secs?: number }
  - Executes shell command in sandbox root (requires approval)

## Examples (manual plan JSON to paste in REPL)
- List Rust files:
```
{"tool":"list_files","args":{"glob":"**/*.rs","max":200}}
```
- Read first 50 lines of src/main.rs:
```
{"tool":"read_file","args":{"path":"src/main.rs","start_line":1,"end_line":50}}
```
- Search for TODOs:
```
{"tool":"search_files","args":{"pattern":"TODO","max_results":100}}
```
- Write a new file (will ask for approval unless --yes):
```
{"tool":"write_file","args":{"path":"NOTES.md","content":"Hello"}}
```
- Run a shell command (will ask for approval unless --yes):
```
{"tool":"run_command","args":{"cmd":"ls -la","timeout_secs":10}}
```

## Safety & sandboxing
- All file and command operations are restricted to the configured working directory
- .gitignore and .rooignore are respected by file walkers
- write_file and run_command require explicit approval unless `--yes` is set

## Roadmap (short‑term)
- LLM planning: chat call returning strict JSON tool plans
- Apply diff/patch tool and batch file ops
- Repetition detection/backoff and basic heuristics against loops
- Richer search: ripgrep integration and tree‑sitter symbols for smarter context
- Tests: unit tests for tools and end‑to‑end REPL tests
- Config presets and examples for common tasks

---

If you want me to wire up the LLM support or add more tools/tests, please say the word.

