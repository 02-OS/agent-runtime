# 02 Agent Runtime

02 Agent Runtime is a local Agent Runtime that coding agents can query over MCP.
It builds a Tree-sitter evidence graph of symbols, callers, and tests, tracks Git diffs, detects stale context, and serves dense context packages inside a token budget. Agents pull exact context instead of raw files.
Every context item is backed by graph evidence and checked for staleness against the current Git diff before it is returned.
Currently supports Rust, Python, C/C++, Bash, JS/TS. Ships as a Rust workspace with systemd daemon, CLI, and MCP server.

## Workspace

```
crates/agent-core       identifiers, config, XDG paths
crates/agent-parser     Tree-sitter symbols, callers, and tests
crates/agent-git        repository discovery and diffs
crates/agent-index      SQLite evidence graph
crates/agent-memory     staleness against the current diff
crates/agent-context    budgeted context packages
crates/agent-mcp        MCP server
crates/agent-daemon     02-agentd user service
crates/agent-cli        `02` command (alias `02agent`)
packaging/02-agentd.service
```

Indexes live under `$XDG_DATA_HOME/02-agent/repos/<repo-id>/`. The daemon listens on `$XDG_RUNTIME_DIR/02agent.sock`.

## Build

```bash
cargo test --offline --workspace
cargo build --release --workspace
```

Release binaries are `target/release/02` and `target/release/02-agentd`. An 02_OS image build copies those binaries and `packaging/02-agentd.service` into the live root. Guides for the indexer, the security model, and the roadmap are in `docs/`.
