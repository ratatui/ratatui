# AGENT PLAYBOOK

Guidance for automated coding assistants working on Ratatui. Human contributors should continue to
follow `CONTRIBUTING.md`.

## Snapshot

- Workspace: confirm the correct crate(s) in `ARCHITECTURE.md` before changing code.
- Tooling: run project tasks through `cargo xtask`; mirror CI expectations when practical.
- Quality gate: produce documentation, tests, and summaries that keep maintainers ready for
  changelog-friendly commits.

## Operating Principles

- **Understand the assignment**: clarify goals, deliverables, acceptance criteria, affected crates,
  and target platforms before writing code.
- **Plan and focus**: map out work, tackle one objective at a time, and split large or cross-cutting
  changes into follow-up PRs.
- **Keep a visible worklog**: record commands, observations, blockers, and dead ends in the
  conversation so reviewers know what was attempted.
- **Quality priorities (highest first)**: preserve user trust, avoid regressions, keep docs and tests
  accurate, maintain code health, and only then optimize efficiency.
- **Prefer minimal patch sets**: avoid mixing refactors with behavioral changes and stage risky
  restructures separately.
- **Escalate early**: ask for human input if work requires `unsafe`, licensing changes, sweeping
  architecture shifts, or you cannot reproduce failing CI locally.

## Implementation Standards

- Run `cargo xtask format` (nightly rustfmt, grouped imports, 100 character comment width) before
  hand-off; do not edit formatting or lint configs without approval.
- Maintain idiomatic Ratatui style: use field init shorthand, prefer `Stylize` helpers, and keep
  imports in std/external/local order.
- Exclude `unsafe` code. If an optimization seems to require it, pause and flag the work.
- Keep refactors, formatting sweeps, or mass renames separate from behavioral changes.
- Avoid workspace configuration or dependency changes unless explicitly requested; respect MSRV and
  license checks enforced by `cargo deny`.
- Leave version bumps, changelog entries, and release tooling alone—automation handles them.
- Reuse existing patterns by searching the repository (`rg`, examples, tests) before inventing new
  abstractions.
- Write docs and comments in US English. Run `cargo xtask typos` (typos-cli) or the typos step in
  `cargo xtask lint` to keep spelling consistent.

## Documentation Standards

- Keep code and docs in lockstep: improve documentation anywhere you touch behavior, APIs, or
  workflows.
- Prefer short, factual prose that explains intent, invariants, and observable behavior rather than
  translating code line by line.
- Update repo guides (`README.md`, `ARCHITECTURE.md`, crate docs) when flows or guidance change; remove
  stale content instead of layering caveats.
- Maintain runnable examples and tutorials; prefer `TestBackend` assertions where visual output
  matters, and add inline comments only when constraints are non-obvious.
- Add dedicated `# Examples` sections with runnable snippets for non-trivial APIs; link to Example
  programs or recipes when they illustrate broader usage.

### Confidentiality & Privacy

- Never include personally identifiable information, secrets, or other sensitive data in docs or
  examples.
- Use fictional data when demonstrating behavior and scrub environment-specific details before
  committing documentation changes.

### Core Principles

- Document intent and invariants rather than restating implementation details.
- Capture non-obvious behavior such as I/O, concurrency, retries, performance, or safety
  constraints.
- Help newcomers orient quickly by linking related concepts and highlighting entry points.
- Preserve crate-level orientation narratives (for example, why `ratatui-core` exists within the
  workspace) when revising top-level docs; expand or clarify that context rather than trimming it.
- Retain or enhance guidance that explains when consumers should depend on `ratatui-core` directly
  versus higher-level crates like `ratatui`, so new readers understand how to choose the right
  entry point.
- Mirror the tone of Rust std docs: plain, direct, and technical, relying on
  concise examples to show how pieces fit together.
- Improve documentation incrementally—the file you touch should leave in a better state than you
  found it.
- Begin each doc comment with a short narrative overview that explains purpose, context, and common
  usage before introducing lists or sections.
- Lead with consumer workflows; mention authoring guidance only when readers must adapt the API.
- Link to built-in helpers or vetted third-party crates when they solve the problem better than
  reimplementing it in the current module.
- Point to relevant sections on <https://ratatui.rs> (Guides, Showcase, Recipes) when they reinforce
  how to use, extend, or troubleshoot the API.
- Use the standard Ratatui section headings to reduce cognitive load, placing a blank line after
  each heading:
  - `# Constructors` for creation helpers.
  - `# Conversions` for type transforms.
  - `# Fluent setters` for chaining APIs (use `# Configuration` only when setters include
    configuration beyond chaining).
  - `# Overview` or `# Usage` if extended narrative organization is needed.
  - `# Examples` (plural) for runnable snippets.
  - `# See also` for related types and cross-links.
  - `# Notes`, `# Panics`, `# Errors`, `# Safety`, `# Performance` for the corresponding method
    sections when needed.
- When touching a file that uses different headings, move it toward this standard rather than
  preserving the older wording.

**Docs should:**

- Explain what an item does, when to use it, and relevant assumptions or side effects.
- Note invariants, error behavior, and cross-crate interactions.
- Use backticked intra-doc links (for example, [`WidgetRef`], [`crate::layout::Layout`]) to connect
  related APIs.
- Prefer reference-style link definitions (`[WidgetRef]: crate::widgets::WidgetRef`) placed at the
  end of the doc block instead of inline `[text](url)` syntax so docs stay readable in source form.
- Provide minimal, meaningful examples or doctests that compile.

**Docs should NOT:**

- Paraphrase code, repeat type signatures, or add commentary that obscures flow.
- Include speculative ideas, stale TODOs, or dead links—delete outdated notes instead of preserving
  them.
- Introduce noise, marketing language, or philosophical digressions.

### When to Update Docs

Update or add documentation when changes involve:

- New or modified public APIs (modules, structs, enums, functions, traits).
- Behavioral shifts, invariants, side effects, or error and retry logic updates.
- Configuration or environment variables, serialization, persistence, or cross-crate boundaries.
- Concurrency, async behavior, `unsafe` code, or performance-sensitive logic.
- Module purpose or architectural responsibilities.

Summarize relevant discussions (issues, PRs, chats) in docs or comments so the reasoning stays with
the code.

### Module-Level Docs (`//!`)

- Start with a short overview that explains what the module offers and when to reach for it.
- Map responsibilities, data flow, and main types so consumers see how pieces fit together.
- Call out assumptions, invariants, and I/O or async behavior that affects end users.
- Link to workspace examples or ratatui.rs Guides, Recipes, or Showcase entries that demonstrate the
  module in real code.
- Keep the tone direct and example-driven in the style of Rust std docs.
- Add a `# Types` section that enumerates each export with a one-line intent blurb.
- Use goal-based headings (`# Usage Patterns`, `# Design Notes`, `# Extending`) to separate
  consumption guidance from customization notes.
- Make it explicit when the built-ins are sufficient versus when readers should reach for examples,
  recipes, the ratatui.rs Showcase, or third-party crates like Awesome Ratatui.
- Leave a blank line after each heading to match rustdoc expectations.
- Blend narrative context with scannable structure: introduce orientation paragraphs first, then use
  focused lists or sections (for example, “Use crate X when...”) to reinforce the decision points
  without overwhelming the reader.
- When documenting a module, list every public type exported from that module (structs, enums,
  type aliases, macros) so readers can see the full surface area at a glance.
- Add a `# Usage` section with a minimal example that composes the primary types, and call out any
  feature flags or configuration knobs that affect availability.

```rust
//! Brief overview: what this module provides and when to reach for it.
//!
//! # Usage
//!
//! ```rust
//! let widget = FooWidget::new(...);
//! frame.render_widget(widget, area);
//! ```
//!
//! # Types
//!
//! - [`FooWidget`]: primary entry point for rendering.
//! - [`FooState`]: optional state to preserve selections or scroll.
//!
//! # Extending
//!
//! Explain how to customize or swap parts, linking to ratatui.rs guides or recipes.
```

### Struct-Level Docs

- Explain what the type represents, major behaviors, and how related methods group together.
- Start with a narrative summary that helps readers understand when and why to use the type.
- Link to supporting types and emphasize invariants or lifecycle expectations.
- Use short sections (for example, `# Constructors`, `# Conversions`, `# Methods`, `# Examples`) or
  bullet lists to orient readers toward the most common entry points.
- Point to related modules or traits (`See the [`layout`](crate::layout) module`) when a struct is
  part of a larger subsystem.
- Document every public field with comments that explain how it should be used and any invariants.
- When fluent builders or setters exist, group them under `# Constructors` and `# Fluent setters`
  (or `# Configuration` when appropriate) with bullet lists that link to each API.
- Present sections in a predictable order: start with constructors, then fluent setters/configuration,
  follow with examples, and end with a `# See also` section that calls out related types.
- Highlight state and companion types (`See [`ListState`]` or `Uses [`StatefulWidget`]`) so readers
  understand how the struct fits into common workflows.
- Insert a blank line between every heading and the content that follows, as shown in the examples.

### Enum-Level Docs

- Summarize what the enum models and how consumers should select between variants.
- Ensure every variant has a short doc comment that explains when it applies and any constraints.
- Reference companion APIs that produce or consume the enum, especially default behavior and
  ordering (`#[default]` variants, fallbacks, or rendering differences).
- Maintain a blank line after each heading for readability.

```rust
/// Builder for composing multi-panel layouts.
///
/// Handles constraint-driven splits and margin helpers to arrange widgets across terminal regions.
/// Layouts can be reused by cloning or adjusting constraints incrementally.
///
/// # Constructors
///
/// - [`LayoutEngine::new`] builds a layout with constraint defaults.
///
/// # Fluent setters
///
/// - [`with_constraints`] updates constraint lists.
///
/// # Examples
///
/// ```rust
/// let engine = LayoutEngine::new([...]).with_margin(...);
/// ```
///
/// # See also
///
/// - [`Constraint`]
/// - [`Rect`]
pub struct LayoutEngine { /* ... */ }
```

### Function & Method Docs (`///`)

- Structure: one-sentence summary, subtle behavior and invariants, then optional sections (`# Errors`,
  `# Panics`, `# Safety`, `# Examples`, and similar).
- Prefer narrative sentences to describe behavior; reserve extra headings only when a section adds
  clarity beyond the summary.
- Highlight input expectations, side effects, and error semantics.
- Provide minimal examples that compile; mark examples `no_run` for I/O heavy cases or `compile_fail`
  for invariant demonstrations.
- Reference other APIs that help readers build context, and explain any noteworthy performance or
  layout implications observed in practice.
- Ensure every heading is followed by a blank line before content or lists.

```rust
/// Render a list into the provided area.
///
/// Preserves the previously selected row stored in [`ListState`] and applies the configured
/// highlight symbol to the active item.
///
/// # Errors
///
/// Returns an error if the backend cannot queue drawing commands.
///
/// # Examples
///
/// ```rust
/// # use ratatui::{Frame, layout::Rect, widgets::{List, ListState}};
/// fn ui(frame: &mut Frame, area: Rect, items: List<'_>, state: &mut ListState) {
///     frame.render_stateful_widget(items, area, state);
/// }
/// ```
```

### Examples & Doctests

- Keep examples concise and runnable; prefer doctests that validate behavior.
- Use `no_run` for operations needing external resources and `compile_fail` to illustrate prohibited
  usage.
- Ensure examples demonstrate the documented invariants rather than repeating trivial boilerplate.
- When possible, show how the item composes with surrounding APIs (e.g., widgets with `Frame`,
  layouts with multiple constraints).

### Intra-Doc Link Conventions

- Always wrap references in backticked intra-doc links, for example, [`Buffer`] or
  [`crate::widgets::Block`].
- Prefer short, unambiguous paths and include the `crate::` prefix only when necessary.
- When linking to workspace docs or sites, use descriptive reference labels that match the target
  name (for example, [`ARCHITECTURE.md`], [`Ratatui Website`]) and define them in a reference
  section at the end of the doc block.
- Reference canonical sources (such as the facade crate docs or `ARCHITECTURE.md`) for workspace
  maps instead of duplicating long crate lists; keep crate-level docs focused on their own surface.
- Use `crate::module::Type` style references so `cargo-rdme` can rewrite links for README output.
  (Re-exported items may still fail to resolve; upstream fixes are pending, so avoid elaborate
  workarounds and accept the limitation.)
- When referencing sibling crates from within workspace docs (for example, linking to `ratatui`
  from `ratatui-core`), prefer stable URLs (crates.io, docs.rs, repository anchors) instead of
  direct intra-doc links that would require cross-crate rustdoc wiring and introduce circular
  dependencies.
- Document feature flags with the `document-features` generated section (`#![cfg_attr(feature =
  "document-features", doc = ...)]`) and keep any supplemental commentary close to each `[features]`
  entry in `Cargo.toml` so crate-level docs and metadata stay in sync. The `document-features`
  crate uses `##` comments immediately above a feature to populate the generated list, so write
  those summaries with end-user docs in mind.

### Style Rules

- Start docs with a present-tense sentence and keep prose tight.
- Use bullet lists for behavior instead of dense paragraphs.
- Target roughly 100 character line width to match formatting.
- Delete outdated documentation instead of leaving commented or TODO markers.

### Platform, Performance, and Error Notes

- Call out OS-specific behavior, retry and backoff semantics, async versus blocking differences, and
  resource constraints such as memory, threads, and streaming.
- Document performance caveats when complexity or buffering strategies matter to downstream users.

### Documentation Ratchet and Responsibilities

- Apply the ratchet: whenever you modify a file, raise its documentation standard by improving docs,
  examples, or invariants.
- Introduce stricter linting (for example, `#![warn(missing_docs)]` or
  `#![deny(rustdoc::broken_intra_doc_links)]`) module by module when documentation is ready.
- As an agent:
  - Add or update docs for every changed public API.
  - Refresh module or struct “maps” when responsibilities shift.
  - Fix or add backticked links and doctests as you touch them.
  - Summarise complex reasoning in concise bullet lists where readers benefit.
  - Skip trivial boilerplate docs unless behavior changes need a note.
- In review scenarios, flag missing or outdated documentation, broken links, absent safety notes, or
  examples that no longer compile.

## Tests and Validation

| Scope            | Expectations                                                                 |
| ---------------- | ----------------------------------------------------------------------------- |
| Docs-only        | Run relevant linting (`cargo xtask docs`, `cargo xtask readme --check`, or    |
|                  | `cargo xtask test-docs`) and confirm formatting.                              |
| Library code     | Prefer `cargo xtask lint`, `cargo xtask test`, or focused subsets such as     |
|                  | `cargo xtask clippy` and `cargo xtask test-libs`. State what ran and why      |
|                  | anything was skipped.                                                         |
| Rendering cases  | Exercise `cargo xtask test-backend <backend>` and adjust `TestBackend`        |
|                  | coverage when output changes.                                                 |
| Broad changes    | Mirror CI via `cargo xtask ci` and consider `cargo xtask coverage` when       |
|                  | rendering logic shifts significantly.                                         |

Prefer lighter-weight checks when they give equivalent assurance, but list executed commands and
rationale for omissions.

## Hand-off Expectations

- Summaries should lead with what changed and why, include a conventional-commit style title
  suggestion, list tests or checks, and mention open questions or skipped validations.
- Capture difficulties, trade-offs, and abandoned approaches in the final message; add code comments
  only when alternative implementations would otherwise be unclear.
- Note any follow-up work (docs, debt, refactors) that deserves a separate PR rather than bundling it
  in.

## Escalation and Safety

- Stop work and request direction when encountering security-sensitive changes, potential semver
  breaks without migration paths, missing licenses, or CI failures you cannot diagnose with available
  tooling.
- Use an emergency stop (report immediately) if you discover behavior that risks data loss or
  security issues.

## Quick Reference

- Prefer `cargo xtask --help` over raw `cargo` commands to stay aligned with CI.
- CI covers formatting, typos, license compliance, clippy (stable and beta), docs, README sync,
  no-std builds, coverage, and backend matrices—match these expectations as your change warrants.
- Keep PRs under roughly 500 changed lines; if growth is unavoidable, plan incremental submissions and
  explain your strategy.
- Update this file when policies evolve so future agents inherit the latest expectations.
