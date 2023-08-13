# Creating a Release

[crates.io](https://crates.io/crates/ratatui) releases are automated via [GitHub
actions](.github/workflows/cd.yml) and triggered by pushing a tag.

1. Record a new demo gif if necessary. The preferred tool for this is
[vhs](https://github.com/charmbracelet/vhs) (installation instructions in README).

   ```shell
   cargo build --example demo
   vhs examples/demo.tape --publish --quiet
   ```

   Then update the link in the [examples README](./examples/README) and the main README. Avoid
   adding the gif to the git repo as binary files tend to bloat repositories.

1. Bump the version in [Cargo.toml](Cargo.toml).
1. Bump versions in the doc comments of [lib.rs](src/lib.rs).
1. Ensure [CHANGELOG.md](CHANGELOG.md) is updated. [git-cliff](https://github.com/orhun/git-cliff)
   can be used for generating the entries.
1. Commit and push the changes.
1. Create a new tag: `git tag -a v[X.Y.Z]`
1. Push the tag: `git push --tags`
1. Wait for [Continuous Deployment](https://github.com/ratatui-org/ratatui/actions) workflow to
   finish.
