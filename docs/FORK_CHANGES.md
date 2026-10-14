# Fork changes

This document records the changes maintained in [zuobaoquan/RaTeX](https://github.com/zuobaoquan/RaTeX) on top of [erweixin/RaTeX](https://github.com/erweixin/RaTeX). It is a maintenance record for the fork, not the upstream release changelog. Add or update an entry whenever a fork-specific change is added, revised, or removed.

## Fallback glyph metrics

Status: implemented in the fork; not present in the upstream base listed below.

- Implementation commit: `f1cdac7962b68b6910857b54ff4bced5c1b4b6e3` (`feat(layout): support explicit fallback glyph metrics`).
- Upstream base: `fad2f6c61ae6f2334394522f1079cee901f20372` (2026-10-12, upstream version 0.1.14).
- Purpose: let a caller using fixed fallback fonts supply the same glyph dimensions used by its renderer, avoiding layout based on estimated Unicode dimensions.
- API: `FallbackGlyphMetrics = fn(char) -> Option<(f64, f64, f64)>` and `LayoutOptions::fallback_metrics`.
- Metrics: advance width, height, and depth in unscaled em units. The callback should measure the renderer's selected font for the character.
- Behavior: only the existing missing-metrics fallback path consults the callback. Default `None`, or a callback result of `None`, retains the built-in estimates. Existing KaTeX metrics take precedence. The callback survives derived style, color, sizing, and tracking options. The layout hook does not load or discover fonts.
- Affected files: `crates/ratex-layout/src/{layout_options.rs,engine.rs,lib.rs}`, `crates/ratex-layout/tests/fallback_metrics.rs`, and the English/Chinese README descriptions.
- Validation: `cargo fmt --all -- --check`, `cargo clippy -p ratex-layout --all-targets -- -D warnings`, and `cargo test -p ratex-layout` passed (145 tests, including 4 new callback regressions). The downstream fixed-font renderer also passed its 6 compatibility tests against this fork.
- Removal condition: upstream provides an equivalent explicit metrics hook, and compatibility tests confirm the same fixed-font behavior. At that point remove the fork-specific implementation, retain this entry as historical with the upstream implementation reference, and let consumers adopt a validated upstream revision or release.

## Dependency and upstream synchronization policy

The fork's `main` branch is the integration branch for upstream synchronization. Downstream builds should depend on a full, immutable Git commit via Cargo's `rev` key, and commit their `Cargo.lock`. A branch dependency is also locked by `Cargo.lock`, but a fixed manifest revision makes an upgrade explicit even after lockfile regeneration or `cargo update`.

Directly used `ratex-*` crates that exchange parser, layout, or display-list types should use the same repository and revision. Mixing registry and fork copies of those crates can produce distinct Rust types even when their version numbers match.

For each upstream synchronization:

1. Merge the chosen upstream revision into the fork's `main`, preserving commits already used by downstream consumers.
2. Review each active entry in this document. Preserve its behavior, resolve conflicts, or mark it superseded when upstream supplies an equivalent implementation.
3. Run formatting, focused layout tests and Clippy, then the downstream fixed-font rendering and output integration checks. Record any changes to the extension and its validation here.
4. Make the validated fork commit available on the remote, then update downstream manifests to that exact revision and regenerate their lockfiles in a separate reviewed change.

Do not use upstream synchronization as an implicit downstream dependency upgrade. The downstream manifest and lockfile remain the authority for the version actually used by a build.
