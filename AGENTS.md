# Repository Guidelines

## Project Structure & Module Organization
- `src/main.rs` is the CLI entrypoint; add helper modules under `src/` and wire them through `main.rs` to keep command wiring in one place.
- `common/` stores reusable configs, rc scripts, menu items, and systemd units referenced via the `common:` token inside `setup.json`.
- Each `setups/<name>/` folder must include a `setup.json` plus any local scripts or assets; machine bundles in `nests/` compose setups through `dependencies`.
- Validate new nests or setups with `cargo run -- nest info <name>` to preview links and dependencies before committing.

## Build, Test, and Development Commands
- `cargo check` – fast compile validation while iterating.
- `cargo fmt` – enforce standard Rust formatting; run before staging changes.
- `cargo clippy --all-targets -- -D warnings` – lint with warnings treated as errors.
- `cargo test` – execute unit/integration tests; create them alongside code in `src/` or under `tests/`.
- `cargo run -- <command>` – exercise CLI flows locally (e.g., `cargo run -- setup git link`).
- `./setups/owl/setup.sh` – installer invoked by the bootstrap script; keep it updated when provisioning logic changes.

## Coding Style & Naming Conventions
- Rely on default `rustfmt` (4-space indentation, trailing commas); avoid manual alignment.
- Use lowercase snake_case for Rust items and kebab-case for new CLI subcommands.
- Share filesystem helpers instead of duplicating path expansion or token parsing logic.

## Testing Guidelines
- Prefer focused unit tests in `#[cfg(test)]` modules; name cases after the behavior under test (`loads_config_with_defaults`).
- Add integration tests when covering multi-command flows; the harness can shell out via `assert_cmd` once introduced.
- Always run `cargo run -- setups-validate` and `cargo run -- nest info <affected>` after editing `setup.json` files.

## Commit & Pull Request Guidelines
- Follow the repository pattern of short, present-tense commit subjects (`Add sway service link`), keeping the first line under 72 characters.
- PRs should outline motivation, affected nests/setups, and manual validation steps (commands run, machines verified).
- Link issues or tickets when relevant and attach output snippets or screenshots for user-facing changes.
- Request review only after `cargo fmt`, `clippy`, and required validation commands succeed locally.

## Security & Configuration Tips
- Do not commit machine-specific secrets; point to them with `local:` tokens and describe manual setup steps in the PR.
- Flag any new `root: true` links or system services so reviewers can confirm the privilege footprint.
