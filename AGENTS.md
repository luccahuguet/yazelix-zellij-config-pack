# Agent Guidelines

Shared Yazelix agent workflow and release policy live in the main repo:

- https://github.com/luccahuguet/yazelix/blob/main/AGENTS.md
- In sibling local checkouts, read `../yazelix/AGENTS.md` first

Only Zellij config-pack-specific guidance belongs here.

## Local Scope

- This repo owns deterministic Zellij config/layout rendering from explicit input data
- Main Yazelix owns settings normalization, runtime paths, plugin artifact resolution, filesystem writes, doctor/repair, and workspace/session behavior
- Keep the renderer free of `~/.config/yazelix`, generated state roots, live Zellij state, Home Manager state, and adjacent checkout discovery
- Preserve bundled layout templates, fragments, and layout-family metadata as package artifacts

## Local Commands

- `cargo fmt --all -- --check`
- `cargo test`
- `cargo run --bin yazelix_zellij_config_pack -- --schema-version`
- `nix build .#yazelix_zellij_config_pack --no-link`

## Integration Notes

Main Yazelix must consume a published child revision before deleting its local config-pack ownership. Local overrides are only smoke tests.
