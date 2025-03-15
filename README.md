# Cargo Template

* Template cargo project
* Pre-installed:
  * `tokio_test` for testing async code
  * `cargo_make` for running scripts (defined in [Makefile.toml](./Makefile.toml))

# Using [cargo make](https://github.com/sagiegurari/cargo-make)

* Add new entries to [Makefile.toml](./Makefile.toml)

# Publishing releases to crates

1. Update version number in Cargo.toml
1. Commit changes and update `main`
1. Create a new tag with the version: `git tag -a x.y.z -m "Note for this version"`
1. Push the tag `git push origin tag x.y.z`

This will run a workflow defined in [.github/workflows/publish.yml](./.github/workflows/publish.yml) that will publish the package.

The tagged version must match the version in Cargo.toml or the worfklow will abort.

# Docs

`cargo make docs`

# Tests

`cargo make tests`