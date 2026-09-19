# Creating a Release

Our release strategy is:

> Release major versions with detailed summaries when necessary, while releasing minor versions
> weekly or as needed without extensive announcements.
>
> Versioning scheme being `0.x.y`, where `x` is the major version and `y` is the minor version.

[crates.io](https://crates.io/crates/ratatui) releases are automated by
[release-plz](https://release-plz.dev/), configured in
[release-plz.toml](./release-plz.toml) and run by the
[Release-plz](https://github.com/ratatui/ratatui/actions/workflows/release-plz.yml) workflow.

1. Record a new demo gif if necessary. The preferred tool for this is
[vhs](https://github.com/charmbracelet/vhs) (installation instructions in README).

   ```shell
   cargo build --example demo2
   vhs examples/demo2.tape
   ```

1. Switch branches to the images branch and copy demo2.gif to examples/, commit, and push.
1. Grab the permalink from <https://github.com/ratatui/ratatui/blob/images/examples/demo2.gif> and
   append `?raw=true` to redirect to the actual image url. Then update the link in the main README.
   Avoid adding the gif to the git repo as binary files tend to bloat repositories.

1. Ensure that any breaking changes are documented in [BREAKING-CHANGES.md](./BREAKING-CHANGES.md)
1. Commit and push the changes to `main`.
1. On every push to `main`, release-plz opens (or updates) a release PR that bumps the version in
   [Cargo.toml](Cargo.toml) and updates [CHANGELOG.md](CHANGELOG.md) (generated via
   [git-cliff](https://github.com/orhun/git-cliff), configured in [cliff.toml](./cliff.toml)).
1. Merging that release PR into `main` triggers the release job: it tags the release (e.g.
   `ratatui-v0.30.2`), publishes to [crates.io](https://crates.io/crates/ratatui), and creates a
   GitHub Release. Watch the [Release-plz
   workflow](https://github.com/ratatui/ratatui/actions/workflows/release-plz.yml) run to
   completion.
