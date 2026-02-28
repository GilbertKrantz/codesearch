# CodeSearch Capability Redesign

## 1. The Differentiator Question

**What can CodeSearch do that nothing else does as well?**

| Competitor | Strength | CodeSearch Position |
|------------|----------|---------------------|
| **ripgrep/grep** | Raw speed, minimal deps | Can't win on pure grep—ripgrep is faster |
| **IDE (VS Code, JetBrains)** | Deep refactoring, debugging | Can't be an IDE—different use case |
| **SonarQube/CodeClimate** | CI dashboards, PR integration | No cloud—but CLI fits pipelines better |
| **Sourcegraph/bloop** | Semantic/AI search | Local-first, no cloud dependency |
| **ctags/gtags** | Symbol indexing | Overlap—but CodeSearch has more |

**The gap:** No tool combines *local*, *CLI-first*, *structure-aware search*, and *tech-debt detection* in one package that works offline, in CI, in SSH sessions, and for AI agents.

### Differentiation from Joern and CodeQL

| | **CodeSearch** | **Joern** | **CodeQL** |
|--|----------------|-----------|------------|
| **Primary use** | Daily grep + tech-debt, AI agents | Security research, vulnerability hunting | Security audits, GitHub-native analysis |
| **Model** | Lightweight graphs (AST, CFG, DFG) as separate views | Single Code Property Graph (CPG) in graph DB | Full semantic model + QL logic queries |
| **Query** | CLI commands, regex, grep-like; no DSL | Scala/Gremlin DSL, custom query language | QL logic language, path queries |
| **Setup** | Zero config; works on raw source immediately | Import → build CPG → query; graph DB required | Compile with extractors; GitHub/codebase integration |
| **Output** | Instant text/JSON; human and CI friendly | Query results from graph traversal | Alerts, path explanations, SARIF |
| **Focus** | Dead code, duplicates, complexity, find symbol | Taint analysis, vuln patterns, data flow to sink | Security bugs, code smells, GitHub PR checks |
| **Agent-ready** | MCP first-class; AI tools as primary consumers | API/REST; not agent-centric | GitHub Actions; cloud-oriented |
| **When to use** | Terminal exploration, CI health gate, AI coding assistant | Deep security research, custom vuln queries | GitHub PRs, org-wide security policy |

**Summary:** Joern and CodeQL are *deep semantic* tools for security and correctness—they require indexing, a query language, and heavy setup. CodeSearch is *lightweight and immediate*—grep that understands structure, plus one-command health and AI-agent integration. Use CodeSearch for daily discovery and debt; use Joern/CodeQL when you need taint analysis, path queries, or formal vulnerability patterns.

---

## 2. Proposed Differentiator

### **"The local code lens for terminal users and AI agents"**

**Core thesis:** CodeSearch is the tool you reach for when:
- You're in the terminal (not an IDE) and need to find/understand code
- Your CI pipeline needs structure-aware quality checks
- An AI agent (via MCP) needs to explore a codebase
- You want grep-like speed with code-structure awareness

**Three pillars:**

1. **Search++** — Grep that understands functions, classes, calls. Not just text—*structure*.
2. **Health scan** — One command for dead code + duplicates + complexity. CI-friendly.
3. **Agent-ready** — MCP as first-class; AI tools use CodeSearch as their "eyes" on a repo.

---

## 3. Capability Tiers

### Tier 1: Core (The Differentiator)

| Capability | Current | Proposed | Rationale |
|------------|---------|----------|-----------|
| **Search** | `codesearch "query"` | Keep, enhance | Primary use case. Add `--symbol` for structural-only. |
| **Find** | (scattered) | **NEW** `codesearch find <symbol>` | Structural find: definition, references, callers. The "grep that understands code" moment. |
| **Health** | deadcode, duplicates, complexity (3 commands) | **NEW** `codesearch health` | Single "debt scan" = dead + dupes + complexity. Exit code + JSON for CI. |
| **MCP** | Optional feature | First-class, documented | AI agents are a unique use case. Make it prominent. |

### Tier 2: Supporting (Keep, Simplify)

| Capability | Current | Proposed | Rationale |
|------------|---------|----------|-----------|
| **Analyze** | `analyze` | Keep | Overview metrics—useful for "what's in this repo?" |
| **Call graph** | `callgraph` | Keep | Critical for "who calls X?" |
| **Circular** | `circular` | Keep | Part of health, but also useful standalone |
| **Graphs** | cfg, dfg, pdg, depgraph, ast | Consolidate to `graph <type>` | Niche but valuable. One entry point. |
| **Interactive** | `interactive` | Keep | Good for exploration |
| **Index/Watch** | `index`, `watch` | Keep | Needed for large codebases |

### Tier 3: Deprecate or De-emphasize

| Capability | Current | Proposed | Rationale |
|------------|---------|----------|-----------|
| **Design metrics** | `design-metrics` | Merge into `health` or drop | Overlaps with complexity; low usage |
| **Metrics** | `metrics` | Merge with `analyze` | Halstead etc.—consolidate |
| **Remote** | `remote --github` | Deprecate or move to separate tool | GitHub has its own search; maintenance burden |
| **Git history** | `git-history` | Optional/plugin | Niche; `git log -S` exists |
| **Graph all** | `graph-all` | Remove | Redundant with `graph <type>` |

---

## 4. Proposed CLI Shape

### Primary commands (daily use)

```
codesearch [query] [path]           # Search (default behavior)
codesearch find <symbol> [path]     # Structural: defs, refs, callers
codesearch health [path]            # Unified tech-debt scan
codesearch analyze [path]           # Codebase overview
```

### Secondary commands (when needed)

```
codesearch callgraph [path]         # Who calls what
codesearch circular [path]         # Circular dependencies
codesearch graph <cfg|dfg|dep> [path]  # Other graphs
codesearch interactive [path]
codesearch index [path]
codesearch watch [path]
```

### Utility

```
codesearch files [path]
codesearch languages
codesearch mcp-server               # For AI agents
```

### Deprecated / Removed

```
codesearch design-metrics   → merge into health
codesearch metrics          → merge into analyze
codesearch remote           → deprecate
codesearch git-history      → optional
codesearch ast/cfg/dfg/pdg/depgraph → codesearch graph <type>
codesearch graph-all        → remove
```

---

## 5. "Find" Command (The Structural Differentiator)

**Problem:** Grep finds text. But "where is `authenticate` defined?" and "who calls `authenticate`?" need structure.

**Proposal:**

```bash
codesearch find authenticate ./src
# Output:
# DEFINITION  src/auth.rs:42    pub fn authenticate(user: &User) -> Result<Session>
# CALLS       src/login.rs:18  authenticate(&user)
# CALLS       src/api.rs:92    auth::authenticate(credentials)
# REFERENCES  src/main.rs:5    use auth::authenticate
```

```bash
codesearch find authenticate --type definition   # Only definitions
codesearch find authenticate --type callers     # Only call sites
codesearch find authenticate --type references  # All references
```

Uses: `extract` + `callgraph` + `search`. Combines existing modules into one UX.

---

## 6. "Health" Command (The CI Differentiator)

**Problem:** CI needs one command: "Is this codebase healthy?" Not three separate tools.

**Proposal:**

```bash
codesearch health ./src
# Output:
# HEALTH SCORE: 72/100
# ├─ Dead code:    8 items (12 pts)
# ├─ Duplicates:   3 blocks (15 pts)
# └─ Complexity:   2 files > 15 CC (13 pts)
# 
# Run with --format json for machine output.
```

```bash
codesearch health ./src --format json
# {"score": 72, "dead_code": 8, "duplicates": 3, "complex_files": 2, ...}
```

```bash
codesearch health ./src --fail-under 70
# Exit code 1 if score < 70 (CI gates)
```

Combines: `deadcode` + `duplicates` + `complexity` with a unified scoring model.

---

## 7. MCP as First-Class

**Current:** MCP is an optional feature, documented briefly.

**Proposed:**
- Document MCP in README: "CodeSearch is the recommended code lens for AI coding assistants"
- Add MCP tools: `find_symbol`, `get_callers`, `get_health` (not just search/analyze)
- Ensure all Tier 1 capabilities are exposed via MCP

---

## 8. Implementation Phases

### Phase 1: Consolidation (Low risk)
- Add `codesearch health` that calls deadcode + duplicates + complexity, aggregates output
- Add `codesearch graph <type>` as alias for cfg/dfg/depgraph
- Merge `metrics` into `analyze`
- Deprecate `remote`, `design-metrics`, `graph-all`

### Phase 2: Structural Find (Medium)
- Implement `codesearch find <symbol>` using extract + callgraph + search
- Add `--type definition|callers|references`
- Pipe-friendly, JSON output

### Phase 3: Health Scoring (Medium)
- Define health score formula (dead code + dupes + complexity → 0-100)
- Add `--fail-under` for CI
- Structured JSON output

### Phase 4: MCP Expansion (Low)
- Add `find_symbol` MCP tool
- Add `get_health` MCP tool
- Update docs

---

## 9. Summary

| Before | After |
|--------|-------|
| 20+ commands, unclear focus | 3 core commands + supporting set |
| Search vs. analysis vs. graphs (fragmented) | Search → Find → Health (narrative) |
| MCP optional | MCP first-class for AI agents |
| Compete with grep (lose) | Own "structure-aware grep + health" (win) |

**Tagline:** *"CodeSearch: grep that understands code, and a one-command health check—local, fast, agent-ready."*
