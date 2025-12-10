# Documentation: https://github.com/casey/just
# Cheat sheet: https://cheatography.com/linux-china/cheat-sheets/justfile/

# List all available scripts
[private]
default:
  @just --list --unsorted

# Format all crates in the workspace
[group("maintenance")]
format *ARGS:
  cargo +nightly fmt --all {{ARGS}}

# Lint all crates in the workspace
[group("maintenance")]
lint *ARGS:
  cargo clippy --all-targets {{ARGS}}
  cargo clippy -p gb-eframe --target wasm32-unknown-unknown {{ARGS}}

# Lint and fix all crates in the workspace, then format
[group("maintenance")]
fix *ARGS: (lint "--fix" ARGS) format

# Update all project dependencies (cargo and vcpkg)
[group("maintenance")]
update:
  cargo upgrade -i
  cargo update

generate year day:
  echo "{{year}} day{{day}}"
  cp -r "./_template" "./src/{{year}}/day{{day}}"
  #cargo generate --path ./_template --name day{{day}} --destination {{year}}

generate-next-day year:
  echo "next day: $(python3 next-day.py ./src/{{year}})"
  just --justfile {{justfile()}} generate {{year}} `python3 next-day.py ./src/{{year}}`
