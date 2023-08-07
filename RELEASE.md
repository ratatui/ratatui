# Creating a Release

[crates.io](https://crates.io/crates/ratatui) releases are automated via [GitHub
actions](.github/workflows/cd.yml) and triggered by pushing a tag.

1. Record a new demo gif. The preferred tool for this is [ttyrec](http://0xcc.net/ttyrec/) and
   [ttygif](https://github.com/icholy/ttygif). [Asciinema](https://asciinema.org/) handles block
   character height poorly, [termanilizer](https://www.terminalizer.com/) takes forever to render,
   [vhs](https://github.com/charmbracelet/vhs) handles braille
   characters poorly (though if <https://github.com/charmbracelet/vhs/issues/322> is fixed, then
   it's probably the best option).

   ```shell
   cargo build --example demo
   ttyrec -e 'cargo --quiet run --release --example demo -- --tick-rate 100' demo.rec
   ttygif demo.rec
   ```

   Then upload it somewhere (e.g. use `vhs publish tty.gif` to publish it or upload it to a GitHub
   wiki page as an attachment). Avoid adding the gif to the git repo as binary files tend to bloat
   repositories.

1. Bump the version in [Cargo.toml](Cargo.toml).
1. Bump versions in the doc comments of [lib.rs](src/lib.rs).
1. Ensure [CHANGELOG.md](CHANGELOG.md) is updated. [git-cliff](https://github.com/orhun/git-cliff)
   can be used for generating the entries.
1. Commit and push the changes.
1. Create a new tag: `git tag -a v[X.Y.Z]`
1. Push the tag: `git push --tags`
1. Wait for [Continuous Deployment](https://github.com/ratatui-org/ratatui/actions) workflow to
   finish.
