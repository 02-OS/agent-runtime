# 02 Agent Runtime

02 Agent Runtime is a local Agent Runtime that coding agents can query over MCP.
It builds a Tree-sitter evidence graph of symbols, callers, and tests, tracks Git diffs, detects stale context, and serves dense context packages inside a token budget. Agents pull exact context instead of raw files.
Every context item is backed by graph evidence and checked for staleness against the current Git diff before it is returned.
Currently supports Rust, Python, C/C++, Bash, JS/TS, Go, and Dart. Ships as a Rust workspace with systemd daemon, CLI, desktop status app, and MCP server.

## Workspace

```
crates/agent-core       identifiers, config, containment, XDG paths
crates/agent-parser     Tree-sitter symbols, callers, and tests (Rust, Python, C/C++, Bash, JS/TS, Go, Dart)
crates/agent-git        repository discovery and diffs
crates/agent-index      SQLite evidence graph, FTS5 search, approvals, and evidence ledger
crates/agent-memory     staleness against the current diff and git-anchored facts
crates/agent-context    budgeted context packages and multi-signal ranking
crates/agent-skills     SKILL.toml parsing, tool allowlists, and session scopes
crates/agent-mcp        MCP server (~40 tools across context, navigation, git, facts, and graph)
crates/agent-daemon     02-agentd user service and repository watcher
crates/agent-cli        `02` command (alias `02agent`: init, index, status, context, search, symbol, deps, tests, memory, watch, unwatch, dashboard, skill, approval, evidence, connect, mcp, doctor, security)
crates/agent-desktop    `02-desktop` local runtime status viewer
packaging/02-agentd.service
packaging/prepare-storage.sh
packaging/02-agent-storage
```

Indexes live under `$XDG_DATA_HOME/02-agent/repos/<repo-id>/`. The daemon listens on `$XDG_RUNTIME_DIR/02agent.sock`.

## Build

```bash
cargo test --locked --workspace
cargo build --release --locked --workspace
```

Release binaries are `target/release/02`, `target/release/02-agentd`, and `target/release/02-desktop`. An 02_OS image build copies those binaries and `packaging/` into the live root. Guides for the indexer, the security model, and the roadmap are in `docs/`.
