# mew phases

## Phase 0 — Lock, Name, Repo, Rules

Status: done

- [x] lock project name: mew
- [x] lock CLI binary: mew
- [x] lock default display name: mew
- [x] lock CLI-first direction
- [x] lock Rust stack
- [x] lock Termux/low-resource priority
- [x] lock default providers: OpenAI/Codex, Gemini, OpenRouter
- [x] lock rich Crush-like CLI aesthetic
- [x] lock rename/persona system
- [x] lock caveman skills direction
- [x] lock advanced token system direction
- [x] lock native tools + guard direction
- [x] lock agent-to-agent council direction
- [x] add README
- [x] add LICENSE
- [x] add SECURITY.md
- [x] add CONTRIBUTING.md

## Phase 1 — Rust Workspace + Beautiful CLI Shell

Status: done

- [x] create Cargo workspace
- [x] create `mew-cli`
- [x] create `mew-common`
- [x] create `mew-ui`
- [x] add `mew --help`
- [x] add startup greet
- [x] clear terminal before greet
- [x] add config paths
- [x] add config load/save
- [x] add name commands
- [x] add style commands
- [x] add doctor command
- [x] add `mew init --dry-run`
- [x] add phrase bank
- [x] add basic tables/cards/banner/code/diff preview
- [x] polish terminal capability detection
- [x] add more rich themes
- [x] add markdown/code/diff renderer
- [x] add spinner animation helpers
- [x] add responsive terminal wrapper
- [x] add tiny/narrow/normal/wide layout classes
- [x] add tests
- [x] add CI

## Phase 1 Learnings

- [x] terminal UI must be more attractive and minimal
- [x] banner should feel vectorized and Claude Code-like
- [x] greet should clear terminal
- [x] blocks must wrap and adapt to narrow/wide terminals
- [x] rich output should not depend on TUI
- [x] normal spacing + responsive blocks > fixed giant boxes

## Phase 2 — Provider Brain v0

Status: next

- [ ] create `mew-provider`
- [ ] provider trait
- [ ] OpenAI-compatible adapter
- [ ] OpenAI/Codex preset
- [ ] OpenRouter preset
- [ ] Gemini provider
- [ ] config env key loading
- [ ] `mew provider list`
- [ ] `mew provider test`
- [ ] `mew model list`
- [ ] `mew model use`
- [ ] `mew ask`
- [ ] `mew chat`
- [ ] streaming renderer

## Phase 3 — Project Init + Context Sniffer

Status: queued

- [ ] create `mew-index`
- [ ] full `mew init`
- [ ] `.mew/project.toml`
- [ ] `.mew/repo-map.md`
- [ ] `.mew/memory.md`
- [ ] AGENT.md / CLAUDE.md / GEMINI.md detection

## Phase 4 — Native Tools + Guard

Status: queued

- [ ] create `mew-tools`
- [ ] create `mew-guard`
- [ ] fs tools
- [ ] git tools
- [ ] http tools
- [ ] package tools
- [ ] machine guard

## Phase 5 — Agent Loop v1

Status: queued

- [ ] tool call loop
- [ ] observations
- [ ] permissions
- [ ] read-only ask mode

## Phase 6 — Edit/Fix/Review

Status: queued

- [ ] patch-first editing
- [ ] diff preview
- [ ] fix mode
- [ ] review mode

## Phase 7 — Caveman Skills + Token Paw

Status: queued

- [ ] caveman skills
- [ ] token budgeting
- [ ] compact context
- [ ] token usage report

## Phase 8 — Mew Council

Status: queued

- [ ] agent-to-agent talks
- [ ] planner/coder/reviewer roles
- [ ] transcript export

## Phase 9 — MCP

Status: queued

- [ ] MCP client
- [ ] MCP server
- [ ] native tools bridge

## Phase 10 — TUI/GUI Later

Status: queued

- [ ] TUI
- [ ] GUI
