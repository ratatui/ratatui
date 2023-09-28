# Creating a Release

[crates.io](https://crates.io/crates/ratatui) releases are automated via [GitHub
actions](.github/workflows/cd.yml) and triggered by pushing a tag.

1. Record a new demo gif if necessary. The preferred tool for this is
[vhs](https://github.com/charmbracelet/vhs) (installation instructions in README).

   ```shell
   cargo build --example demo2
   vhs examples/demo2.tape
   ```

1. Switch branches to the images branch and copy demo2.gif to examples/, commit, and push.
1. Grab the permalink from <https://github.com/ratatui-org/ratatui/blob/images/examples/demo2.gif> and
   append `?raw=true` to redirect to the actual image url. Then update the link in the main README.
   Avoid adding the gif to the git repo as binary files tend to bloat repositories.

1. Bump the version in [Cargo.toml](Cargo.toml).
1. Bump versions in the doc comments of [lib.rs](src/lib.rs).
1. Ensure [CHANGELOG.md](CHANGELOG.md) is updated. [git-cliff](https://github.com/orhun/git-cliff)
   can be used for generating the entries.
1. Ensure that any breaking changes are documented in [BREAKING-CHANGES.md](./BREAKING-CHANGES.md)
1. Commit and push the changes.
1. Create a new tag: `git tag -a v[X.Y.Z]`
1. Push the tag: `git push --tags`
1. Wait for [Continuous Deployment](https://github.com/ratatui-org/ratatui/actions) workflow to
   finish.
