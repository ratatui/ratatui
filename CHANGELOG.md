# Changelog

## v0.22.0 - 2023-07-17

### Features

- *(barchart)* Set custom text value in the bar ([#309](https://github.com/ratatui-org/ratatui/issues/309))
- *(barchart)* Enable barchart groups ([#288](https://github.com/ratatui-org/ratatui/issues/288))
- *(block)* Support for having more than one title ([#232](https://github.com/ratatui-org/ratatui/issues/232))
- *(examples)* User_input example cursor movement ([#302](https://github.com/ratatui-org/ratatui/issues/302))
- *(misc)* Make builder fn const ([#275](https://github.com/ratatui-org/ratatui/issues/275)) ([#275](https://github.com/ratatui-org/ratatui/issues/275))
- *(prelude)* Add a prelude ([#304](https://github.com/ratatui-org/ratatui/issues/304))
- *(style)* Enable setting the underline color for crossterm ([#308](https://github.com/ratatui-org/ratatui/issues/308)) ([#310](https://github.com/ratatui-org/ratatui/issues/310))
- *(style)* Allow Modifiers add/remove in const ([#287](https://github.com/ratatui-org/ratatui/issues/287))
- *(stylize)* Allow all widgets to be styled ([#289](https://github.com/ratatui-org/ratatui/issues/289))
- *(terminal)* Expose 'swap_buffers' method
- *(uncategorized)* Stylization shorthands ([#283](https://github.com/ratatui-org/ratatui/issues/283))
- *(uncategorized)* Add scrollbar widget ([#228](https://github.com/ratatui-org/ratatui/issues/228))

### Bug Fixes

- *(clippy)* Unused_mut lint for layout ([#285](https://github.com/ratatui-org/ratatui/issues/285))
- *(examples)* Correct progress label in gague example ([#263](https://github.com/ratatui-org/ratatui/issues/263))
- *(layout)* Cap Constraint::apply to 100% length ([#264](https://github.com/ratatui-org/ratatui/issues/264))
- *(lint)* Suspicious_double_ref_op is new in 1.71 ([#311](https://github.com/ratatui-org/ratatui/issues/311))
- *(prelude)* Remove widgets module from prelude ([#317](https://github.com/ratatui-org/ratatui/issues/317))
- *(title)* Remove default alignment and position ([#323](https://github.com/ratatui-org/ratatui/issues/323))
- *(typos)* Configure typos linter ([#233](https://github.com/ratatui-org/ratatui/issues/233))
- *(uncategorized)* Rust-tui-template became a revival project ([#320](https://github.com/ratatui-org/ratatui/issues/320))
- *(uncategorized)* Revert removal of WTFPL from deny.toml ([#266](https://github.com/ratatui-org/ratatui/issues/266))

### Refactor

- *(ci)* Simplify cargo-make installation ([#240](https://github.com/ratatui-org/ratatui/issues/240))
- *(text)* Simplify reflow implementation ([#290](https://github.com/ratatui-org/ratatui/issues/290))

### Documentation

- *(color)* Parse more color formats and add docs ([#306](https://github.com/ratatui-org/ratatui/issues/306))
- *(lib)* Add `tui-term` a pseudoterminal library ([#268](https://github.com/ratatui-org/ratatui/issues/268))
- *(lib)* Fixup tui refs in widgets/mod.rs ([#216](https://github.com/ratatui-org/ratatui/issues/216))
- *(lib)* Add backend docs ([#213](https://github.com/ratatui-org/ratatui/issues/213))
- *(readme)* Remove duplicated mention of tui-rs-tree-widgets ([#223](https://github.com/ratatui-org/ratatui/issues/223))
- *(uncategorized)* Improve CONTRIBUTING.md ([#277](https://github.com/ratatui-org/ratatui/issues/277))
- *(uncategorized)* Fix scrollbar ascii illustrations and calendar doc paths ([#272](https://github.com/ratatui-org/ratatui/issues/272))
- *(uncategorized)* README tweaks ([#225](https://github.com/ratatui-org/ratatui/issues/225))
- *(uncategorized)* Add CODEOWNERS file ([#212](https://github.com/ratatui-org/ratatui/issues/212))
- *(uncategorized)* Update README.md and add hello_world example ([#204](https://github.com/ratatui-org/ratatui/issues/204))

### Styling

- *(comments)* Set comment length to wrap at 100 chars ([#218](https://github.com/ratatui-org/ratatui/issues/218))
- *(config)* Apply formatting to config files ([#238](https://github.com/ratatui-org/ratatui/issues/238))
- *(manifest)* Apply formatting to Cargo.toml ([#237](https://github.com/ratatui-org/ratatui/issues/237))
- *(readme)* Update the style of badges in README.md ([#299](https://github.com/ratatui-org/ratatui/issues/299))
- *(widget)* Inline format arguments ([#279](https://github.com/ratatui-org/ratatui/issues/279))
- *(uncategorized)* Fix formatting ([#292](https://github.com/ratatui-org/ratatui/issues/292))
- *(uncategorized)* Reformat imports ([#219](https://github.com/ratatui-org/ratatui/issues/219))

### Testing

- *(barchart)* Add unit tests ([#301](https://github.com/ratatui-org/ratatui/issues/301))
- *(paragraph)* Simplify paragraph benchmarks ([#282](https://github.com/ratatui-org/ratatui/issues/282))
- *(uncategorized)* Add benchmarks for paragraph ([#262](https://github.com/ratatui-org/ratatui/issues/262))

### Miscellaneous Tasks

- *(ci)* Bump cargo-make version ([#239](https://github.com/ratatui-org/ratatui/issues/239))
- *(ci)* Enable merge queue for builds ([#235](https://github.com/ratatui-org/ratatui/issues/235))
- *(ci)* Integrate cargo-deny for linting dependencies ([#221](https://github.com/ratatui-org/ratatui/issues/221))
- *(commitizen)* Add commitizen config ([#222](https://github.com/ratatui-org/ratatui/issues/222))
- *(demo)* Update demo gif ([#234](https://github.com/ratatui-org/ratatui/issues/234))
- *(demo)* Update demo gif with a fixed unicode gauge ([#227](https://github.com/ratatui-org/ratatui/issues/227))
- *(features)* Enable building with all-features ([#286](https://github.com/ratatui-org/ratatui/issues/286))
- *(github)* Add EditorConfig config ([#300](https://github.com/ratatui-org/ratatui/issues/300))
- *(github)* Simplify the CODEOWNERS file ([#271](https://github.com/ratatui-org/ratatui/issues/271))
- *(github)* Add pull request template ([#269](https://github.com/ratatui-org/ratatui/issues/269))
- *(github)* Fix the syntax in CODEOWNERS file ([#236](https://github.com/ratatui-org/ratatui/issues/236))
- *(license)* Add Ratatui developers to license ([#297](https://github.com/ratatui-org/ratatui/issues/297))
- *(tests)* Add coverage job to bacon ([#312](https://github.com/ratatui-org/ratatui/issues/312))
- *(uncategorized)* Lint and doc cleanup ([#191](https://github.com/ratatui-org/ratatui/issues/191))

### Build

- *(deps)* Upgrade bitflags to 2.3 ([#205](https://github.com/ratatui-org/ratatui/issues/205)) [**breaking**]
- *(uncategorized)* Add git pre-push hooks using cargo-husky ([#274](https://github.com/ratatui-org/ratatui/issues/274))

### Continuous Integration

- *(makefile)* Split CI jobs ([#278](https://github.com/ratatui-org/ratatui/issues/278))
- *(uncategorized)* Parallelize CI jobs ([#318](https://github.com/ratatui-org/ratatui/issues/318))
- *(uncategorized)* Add feat-wrapping on push and on pull request ci triggers ([#267](https://github.com/ratatui-org/ratatui/issues/267))
- *(uncategorized)* Add code coverage action ([#209](https://github.com/ratatui-org/ratatui/issues/209))

### Contributors

Thank you so much to everyone that contributed to this release!

Here is the list of contributors who have contributed to `ratatui` for the first time!

- [@Nydragon](https://github.com/Nydragon)
- [@snpefk](https://github.com/snpefk)
- [@Philipp-M](https://github.com/Philipp-M)
- [@mrbcmorris](https://github.com/mrbcmorris)
- [@endepointe](https://github.com/endepointe)
- [@kdheepak](https://github.com/kdheepak)
- [@samyosm](https://github.com/samyosm)
- [@SLASHLogin](https://github.com/SLASHLogin)
- [@karthago1](https://github.com/karthago1)
- [@BoolPurist](https://github.com/BoolPurist)
- [@Nogesma](https://github.com/Nogesma)

## v0.21.0 - 2023-05-28

### Features

- *(backend)* Add termwiz backend and example ([#5](https://github.com/ratatui-org/ratatui/issues/5))
- *(block)* Support placing the title on bottom ([#36](https://github.com/ratatui-org/ratatui/issues/36))
- *(border)* Add border! macro for easy bitflag manipulation ([#11](https://github.com/ratatui-org/ratatui/issues/11))
- *(calendar)* Add calendar widget ([#138](https://github.com/ratatui-org/ratatui/issues/138))
- *(color)* Add `FromStr` implementation for `Color` ([#180](https://github.com/ratatui-org/ratatui/issues/180))
- *(list)* Add len() to List ([#24](https://github.com/ratatui-org/ratatui/pull/24))
- *(paragraph)* Allow Lines to be individually aligned ([#149](https://github.com/ratatui-org/ratatui/issues/149))
- *(sparkline)* Finish #1 Sparkline directions PR ([#134](https://github.com/ratatui-org/ratatui/issues/134))
- *(terminal)* Add inline viewport ([#114](https://github.com/ratatui-org/ratatui/issues/114)) [**breaking**]
- *(test)* Expose test buffer ([#160](https://github.com/ratatui-org/ratatui/issues/160))
- *(text)* Add `Masked` to display secure data ([#168](https://github.com/ratatui-org/ratatui/issues/168)) [**breaking**]
- *(widget)* Add circle widget ([#159](https://github.com/ratatui-org/ratatui/issues/159))
- *(widget)* Add style methods to Span, Spans, Text ([#148](https://github.com/ratatui-org/ratatui/issues/148))
- *(widget)* Support adding padding to Block ([#20](https://github.com/ratatui-org/ratatui/issues/20))
- *(widget)* Add offset() and offset_mut() for table and list state ([#12](https://github.com/ratatui-org/ratatui/issues/12))

### Bug Fixes

- *(canvas)* Use full block for Marker::Block ([#133](https://github.com/ratatui-org/ratatui/issues/133)) [**breaking**]
- *(example)* Update input in examples to only use press events ([#129](https://github.com/ratatui-org/ratatui/issues/129))
- *(uncategorized)* Cleanup doc example ([#145](https://github.com/ratatui-org/ratatui/issues/145))
- *(reflow)* Remove debug macro call ([#198](https://github.com/ratatui-org/ratatui/issues/198))

### Refactor

- *(example)* Remove redundant `vec![]` in `user_input` example ([#26](https://github.com/ratatui-org/ratatui/issues/26))
- *(example)* Refactor paragraph example ([#152](https://github.com/ratatui-org/ratatui/issues/152))
- *(style)* Mark some Style fns const so they can be defined globally ([#115](https://github.com/ratatui-org/ratatui/issues/115))
- *(text)* Replace `Spans` with `Line` ([#178](https://github.com/ratatui-org/ratatui/issues/178))

### Documentation

- *(apps)* Fix rsadsb/adsb_deku radar link ([#140](https://github.com/ratatui-org/ratatui/issues/140))
- *(apps)* Add tenere ([#141](https://github.com/ratatui-org/ratatui/issues/141))
- *(apps)* Add twitch-tui ([#124](https://github.com/ratatui-org/ratatui/issues/124))
- *(apps)* Add oxycards ([#113](https://github.com/ratatui-org/ratatui/issues/113))
- *(apps)* Re-add trippy to APPS.md ([#117](https://github.com/ratatui-org/ratatui/issues/117))
- *(block)* Add example for block.inner ([#158](https://github.com/ratatui-org/ratatui/issues/158))
- *(changelog)* Update the empty profile link in contributors ([#112](https://github.com/ratatui-org/ratatui/issues/112))
- *(readme)* Fix small typo in readme ([#186](https://github.com/ratatui-org/ratatui/issues/186))
- *(readme)* Add termwiz demo to examples ([#183](https://github.com/ratatui-org/ratatui/issues/183))
- *(readme)* Add acknowledgement section ([#154](https://github.com/ratatui-org/ratatui/issues/154))
- *(readme)* Update project description ([#127](https://github.com/ratatui-org/ratatui/issues/127))
- *(uncategorized)* Scrape example code from examples/* ([#195](https://github.com/ratatui-org/ratatui/issues/195))

### Styling

- *(apps)* Update the style of application list ([#184](https://github.com/ratatui-org/ratatui/issues/184))
- *(readme)* Update project introduction in README.md ([#153](https://github.com/ratatui-org/ratatui/issues/153))
- *(uncategorized)* Clippy's variable inlining in format macros

### Testing

- *(buffer)* Add `assert_buffer_eq!` and Debug implementation ([#161](https://github.com/ratatui-org/ratatui/issues/161))
- *(list)* Add characterization tests for list ([#167](https://github.com/ratatui-org/ratatui/issues/167))
- *(widget)* Add unit tests for Paragraph ([#156](https://github.com/ratatui-org/ratatui/issues/156))

### Miscellaneous Tasks

- *(uncategorized)* Inline format args ([#190](https://github.com/ratatui-org/ratatui/issues/190))
- *(uncategorized)* Minor lints, making Clippy happier ([#189](https://github.com/ratatui-org/ratatui/issues/189))

### Build

- *(uncategorized)* Bump MSRV to 1.65.0 ([#171](https://github.com/ratatui-org/ratatui/issues/171))

### Continuous Integration

- *(uncategorized)* Add ci, build, and revert to allowed commit types

### Contributors

Thank you so much to everyone that contributed to this release!

Here is the list of contributors who have contributed to `ratatui` for the first time!

- [@kpcyrd](https://github.com/kpcyrd)
- [@fujiapple852](https://github.com/fujiapple852)
- [@BrookJeynes](https://github.com/BrookJeynes)
- [@Ziqi-Yang](https://github.com/Ziqi-Yang)
- [@Xithrius](https://github.com/Xithrius)
- [@lesleyrs](https://github.com/lesleyrs)
- [@pythops](https://github.com/pythops)
- [@wcampbell0x2a](https://github.com/wcampbell0x2a)
- [@sophacles](https://github.com/sophacles)
- [@Eyesonjune18](https://github.com/Eyesonjune18)
- [@a-kenji](https://github.com/a-kenji)
- [@TimerErTim](https://github.com/TimerErTim)
- [@Mehrbod2002](https://github.com/Mehrbod2002)
- [@thomas-mauran](https://github.com/thomas-mauran)
- [@nyurik](https://github.com/nyurik)

## v0.20.1 - 2023-03-19

### Bug Fixes

- *(style)* Bold needs a bit ([#104](https://github.com/ratatui-org/ratatui/issues/104))

### Documentation

- *(apps)* Add "logss" to apps ([#105](https://github.com/ratatui-org/ratatui/issues/105))
- *(uncategorized)* Fixup remaining tui references ([#106](https://github.com/ratatui-org/ratatui/issues/106))

### Contributors

Thank you so much to everyone that contributed to this release!

- [@joshka](https://github.com/joshka)
- [@todoesverso](https://github.com/todoesverso)
- [@UncleScientist](https://github.com/UncleScientist)

## v0.20.0 - 2023-03-19

This marks the first release of `ratatui`, a community-maintained fork of [tui](https://github.com/fdehau/tui-rs).

The purpose of this release is to include **bug fixes** and **small changes** into the repository thus **no new features** are added. We have transferred all the pull requests from the original repository and worked on the low hanging ones to incorporate them in this "maintenance" release.

Here is a list of changes:

### Features

- *(cd)* Add continuous deployment workflow ([#93](https://github.com/ratatui-org/ratatui/issues/93))
- *(ci)* Add MacOS to CI ([#60](https://github.com/ratatui-org/ratatui/issues/60))
- *(widget)* Add `offset()` to `TableState` ([#10](https://github.com/ratatui-org/ratatui/issues/10))
- *(widget)* Add `width()` to ListItem ([#17](https://github.com/ratatui-org/ratatui/issues/17))

### Bug Fixes

- *(ci)* Test MSRV compatibility on CI ([#85](https://github.com/ratatui-org/ratatui/issues/85))
- *(ci)* Bump Rust version to 1.63.0 ([#80](https://github.com/ratatui-org/ratatui/issues/80))
- *(ci)* Use env for the cargo-make version ([#76](https://github.com/ratatui-org/ratatui/issues/76))
- *(ci)* Fix deprecation warnings on CI ([#58](https://github.com/ratatui-org/ratatui/issues/58))
- *(doc)* Add 3rd party libraries accidentally removed at #21 ([#61](https://github.com/ratatui-org/ratatui/issues/61))
- *(widget)* List should not ignore empty string items ([#42](https://github.com/ratatui-org/ratatui/issues/42)) [**breaking**]
- *(uncategorized)* Cassowary/layouts: add extra constraints for fixing Min(v)/Max(v) combination. ([#31](https://github.com/ratatui-org/ratatui/issues/31))
- *(uncategorized)* Fix user_input example double key press registered on windows
- *(uncategorized)* Ignore zero-width symbol on rendering `Paragraph`
- *(uncategorized)* Fix typos ([#45](https://github.com/ratatui-org/ratatui/issues/45))
- *(uncategorized)* Fix typos ([#47](https://github.com/ratatui-org/ratatui/issues/47))

### Refactor

- *(style)* Make bitflags smaller ([#13](https://github.com/ratatui-org/ratatui/issues/13))

### Documentation

- *(apps)* Move 'apps using ratatui' to dedicated file ([#98](https://github.com/ratatui-org/ratatui/issues/98)) ([#99](https://github.com/ratatui-org/ratatui/issues/99))
- *(canvas)* Add documentation for x_bounds, y_bounds ([#35](https://github.com/ratatui-org/ratatui/issues/35))
- *(contributing)* Specify the use of unsafe for optimization ([#67](https://github.com/ratatui-org/ratatui/issues/67))
- *(github)* Remove pull request template ([#68](https://github.com/ratatui-org/ratatui/issues/68))
- *(readme)* Update crate status badge ([#102](https://github.com/ratatui-org/ratatui/issues/102))
- *(readme)* Small edits before first release ([#101](https://github.com/ratatui-org/ratatui/issues/101))
- *(readme)* Add install instruction and update title ([#100](https://github.com/ratatui-org/ratatui/issues/100))
- *(readme)* Add systeroid to application list ([#92](https://github.com/ratatui-org/ratatui/issues/92))
- *(readme)* Add glicol-cli to showcase list ([#95](https://github.com/ratatui-org/ratatui/issues/95))
- *(readme)* Add oxker to application list ([#74](https://github.com/ratatui-org/ratatui/issues/74))
- *(readme)* Add app kubectl-watch which uses tui ([#73](https://github.com/ratatui-org/ratatui/issues/73))
- *(readme)* Add poketex to 'apps using tui' in README ([#64](https://github.com/ratatui-org/ratatui/issues/64))
- *(readme)* Update README.md ([#39](https://github.com/ratatui-org/ratatui/issues/39))
- *(readme)* Update README.md ([#40](https://github.com/ratatui-org/ratatui/issues/40))
- *(readme)* Clarify README.md fork status update
- *(uncategorized)* Fix: fix typos ([#90](https://github.com/ratatui-org/ratatui/issues/90))
- *(uncategorized)* Update to build more backends ([#81](https://github.com/ratatui-org/ratatui/issues/81))
- *(uncategorized)* Expand "Apps" and "Third-party" sections ([#21](https://github.com/ratatui-org/ratatui/issues/21))
- *(uncategorized)* Add tui-input and update xplr in README.md
- *(uncategorized)* Add hncli to list of applications made with tui-rs ([#41](https://github.com/ratatui-org/ratatui/issues/41))
- *(uncategorized)* Updated readme and contributing guide with updates about the fork ([#46](https://github.com/ratatui-org/ratatui/issues/46))

### Performance

- *(layout)* Better safe shared layout cache ([#62](https://github.com/ratatui-org/ratatui/issues/62))

### Miscellaneous Tasks

- *(cargo)* Update project metadata ([#94](https://github.com/ratatui-org/ratatui/issues/94))
- *(ci)* Integrate `typos` for checking typos ([#91](https://github.com/ratatui-org/ratatui/issues/91))
- *(ci)* Change the target branch to main ([#79](https://github.com/ratatui-org/ratatui/issues/79))
- *(ci)* Re-enable clippy on CI ([#59](https://github.com/ratatui-org/ratatui/issues/59))
- *(uncategorized)* Integrate `committed` for checking conventional commits ([#77](https://github.com/ratatui-org/ratatui/issues/77))
- *(uncategorized)* Update `rust-version` to 1.59 in Cargo.toml ([#57](https://github.com/ratatui-org/ratatui/issues/57))
- *(uncategorized)* Update deps ([#51](https://github.com/ratatui-org/ratatui/issues/51))
- *(uncategorized)* Fix typo in layout.rs ([#619](https://github.com/ratatui-org/ratatui/issues/619))
- *(uncategorized)* Add apps using `tui`

### Contributors

Thank you so much to everyone that contributed to this release!

- [@orhun](https://github.com/orhun)
- [@mindoodoo](https://github.com/mindoodoo)
- [@sayanarijit](https://github.com/sayanarijit)
- [@Owletti](https://github.com/Owletti)
- [@UncleScientist](https://github.com/UncleScientist)
- [@rhysd](https://github.com/rhysd)
- [@ckaznable](https://github.com/ckaznable)
- [@imuxin](https://github.com/imuxin)
- [@mrjackwills](https://github.com/mrjackwills)
- [@conradludgate](https://github.com/conradludgate)
- [@kianmeng](https://github.com/kianmeng)
- [@chaosprint](https://github.com/chaosprint)

And most importantly, special thanks to [Florian Dehau](https://github.com/fdehau) for creating this awesome library 💖 We look forward to building on the strong foundations that the original crate laid out.

## v0.19.0 - 2022-08-14

### Features

* Bump `crossterm` to `0.25`

## v0.18.0 - 2022-04-24

### Features

* Update `crossterm` to `0.23`

## v0.17.0 - 2022-01-22

### Features

* Add option to `widgets::List` to repeat the highlight symbol for each line of multi-line items (#533).
* Add option to control the alignment of `Axis` labels in the `Chart` widget (#568).

### Breaking changes

* The minimum supported rust version is now `1.56.1`.

#### New default backend and consolidated backend options (#553)

* `crossterm` is now the default backend.
If you are already using the `crossterm` backend, you can simplify your dependency specification in `Cargo.toml`:
```diff
- tui = { version = "0.16", default-features = false, features = ["crossterm"] }
+ tui = "0.17"
```
If you are using the `termion` backend, your `Cargo` is now a bit more verbose:
```diff
- tui = "0.16"
+ tui = { version = "0.17", default-features = false, features = ["termion"] }
```

`crossterm` has also been bumped to version `0.22`.

Because of their apparent low usage, `curses` and `rustbox` backends have been removed.
If you are using one of them, you can import their last implementation in your own project:
* [curses](https://github.com/fdehau/tui-rs/blob/v0.16.0/src/backend/curses.rs)
* [rustbox](https://github.com/fdehau/tui-rs/blob/v0.16.0/src/backend/rustbox.rs)

#### Canvas labels (#543)

* Labels of the `Canvas` widget are now `text::Spans`.
The signature of `widgets::canvas::Context::print` has thus been updated:
```diff
- ctx.print(x, y, "Some text", Color::Yellow);
+ ctx.print(x, y, Span::styled("Some text", Style::default().fg(Color::Yellow)))
```

## v0.16.0 - 2021-08-01

### Features

* Update `crossterm` to `0.20`.
* Add `From<Cow<str>>` implementation for `text::Text` (#471).
* Add option to right or center align the title of a `widgets::Block` (#462).

### Fixes

* Apply label style in `widgets::Gauge` and avoid panics because of overflows with long labels (#494).
* Avoid panics because of overflows with long axis labels in `widgets::Chart` (#512).
* Fix computation of column widths in `widgets::Table` (#514).
* Fix panics because of invalid offset when input changes between two frames in `widgets::List` and
  `widgets::Chart` (#516).

## v0.15.0 - 2021-05-02

### Features

* Update `crossterm` to `0.19`.
* Update `rand` to `0.8`.
* Add a read-only view of the terminal state after the draw call (#440).

### Fixes

* Remove compile warning in `TestBackend::assert_buffer` (#466).

## v0.14.0 - 2021-01-01

### Breaking changes

#### New API for the Table widget

The `Table` widget got a lot of improvements that should make it easier to work with:
* It should not longer panic when rendered on small areas.
* `Row`s are now a collection of `Cell`s, themselves wrapping a `Text`. This means you can style
the entire `Table`, an entire `Row`, an entire `Cell` and rely on the styling capabilities of
`Text` to get full control over the look of your `Table`.
* `Row`s can have multiple lines.
* The header is now optional and is just another `Row` always visible at the top.
* `Row`s can have a bottom margin.
* The header alignment is no longer off when an item is selected.

Taking the example of the code in `examples/demo/ui.rs`, this is what you may have to change:
```diff
     let failure_style = Style::default()
         .fg(Color::Red)
         .add_modifier(Modifier::RAPID_BLINK | Modifier::CROSSED_OUT);
-    let header = ["Server", "Location", "Status"];
     let rows = app.servers.iter().map(|s| {
         let style = if s.status == "Up" {
             up_style
         } else {
             failure_style
         };
-        Row::StyledData(vec![s.name, s.location, s.status].into_iter(), style)
+        Row::new(vec![s.name, s.location, s.status]).style(style)
     });
-    let table = Table::new(header.iter(), rows)
+    let table = Table::new(rows)
+        .header(
+            Row::new(vec!["Server", "Location", "Status"])
+                .style(Style::default().fg(Color::Yellow))
+                .bottom_margin(1),
+        )
         .block(Block::default().title("Servers").borders(Borders::ALL))
-        .header_style(Style::default().fg(Color::Yellow))
         .widths(&[
             Constraint::Length(15),
             Constraint::Length(15),
```
Here, we had to:
- Change the way we construct [`Row`](https://docs.rs/tui/*/tui/widgets/struct.Row.html) which is no
longer an `enum` but a `struct`. It accepts anything that can be converted to an iterator of things
that can be converted to a [`Cell`](https://docs.rs/tui/*/tui/widgets/struct.Cell.html)
- The header is no longer a required parameter so we use
[`Table::header`](https://docs.rs/tui/*/tui/widgets/struct.Table.html#method.header) to set it.
`Table::header_style` has been removed since the style can be directly set using
[`Row::style`](https://docs.rs/tui/*/tui/widgets/struct.Row.html#method.style). In addition, we want
to preserve the old margin between the header and the rest of the rows so we add a bottom margin to
the header using
[`Row::bottom_margin`](https://docs.rs/tui/*/tui/widgets/struct.Row.html#method.bottom_margin).

You may want to look at the documentation of the different types to get a better understanding:
- [`Table`](https://docs.rs/tui/*/tui/widgets/struct.Table.html)
- [`Row`](https://docs.rs/tui/*/tui/widgets/struct.Row.html)
- [`Cell`](https://docs.rs/tui/*/tui/widgets/struct.Cell.html)

### Fixes

- Fix handling of Non Breaking Space (NBSP) in wrapped text in `Paragraph` widget.

### Features

- Add `Style::reset` to create a `Style` resetting all styling properties when applied.
- Add an option to render the `Gauge` widget with unicode blocks.
- Manage common project tasks with `cargo-make` rather than `make` for easier on-boarding.

## v0.13.0 - 2020-11-14

### Features

* Add `LineGauge` widget which is a more compact variant of the existing `Gauge`.
* Bump `crossterm` to 0.18

### Fixes

* Take into account the borders of the `Table` widget when the widths of columns is controlled by
`Percentage` and `Ratio` constraints.

## v0.12.0 - 2020-09-27

### Features

* Make it easier to work with string with multiple lines in `Text` (#361).

### Fixes

* Fix a style leak in `Graph` so components drawn on top of the plotted data (i.e legend and axis
titles) are not affected by the style of the `Dataset`s (#388).
* Make sure `BarChart` shows bars with the max height only when the plotted data is actually equal
to the max (#383).

## v0.11.0 - 2020-09-20

### Features

* Add the dot character as a new type of canvas marker (#350).
* Support more style modifiers on Windows (#368).

### Fixes

* Clearing the terminal through `Terminal::clear` will cause the whole UI to be redrawn (#380).
* Fix incorrect output when the first diff to draw is on the second cell of the terminal (#347).

## v0.10.0 - 2020-07-17

### Breaking changes

#### Easier cursor management

A new method has been added to `Frame` called `set_cursor`. It lets you specify where the cursor
should be placed after the draw call. Furthermore like any other widgets, if you do not set a cursor
position during a draw call, the cursor is automatically hidden.

For example:

```rust
fn draw_input(f: &mut Frame, app: &App) {
  if app.editing {
    let input_width = app.input.width() as u16;
    // The cursor will be placed just after the last character of the input
    f.set_cursor((input_width + 1, 0));
  } else {
    // We are no longer editing, the cursor does not have to be shown, set_cursor is not called and
    // thus automatically hidden.
  }
}
```

In order to make this possible, the draw closure takes in input `&mut Frame` instead of `mut Frame`.

#### Advanced text styling

It has been reported several times that the text styling capabilities were somewhat limited in many
places of the crate. To solve the issue, this release includes a new set of text primitives that are
now used by a majority of widgets to provide flexible text styling.

`Text` is replaced by the following types:
- `Span`: a string with a unique style.
- `Spans`: a string with multiple styles.
- `Text`: a multi-lines string with multiple styles.

However, you do not always need this complexity so the crate provides `From` implementations to
let you use simple strings as a default and switch to the previous primitives when you need
additional styling capabilities.

For example, the title of a `Block` can be set in the following ways:

```rust
// A title with no styling
Block::default().title("My title");
// A yellow title
Block::default().title(Span::styled("My title", Style::default().fg(Color::Yellow)));
// A title where "My" is bold and "title" is a simple string
Block::default().title(vec![
    Span::styled("My", Style::default().add_modifier(Modifier::BOLD)),
    Span::from("title")
]);
```

- `Buffer::set_spans` and `Buffer::set_span` were added.
- `Paragraph::new` expects an input that can be converted to a `Text`.
- `Block::title_style` is deprecated.
- `Block::title` expects a `Spans`.
- `Tabs` expects a list of `Spans`.
- `Gauge` custom label is now a `Span`.
- `Axis` title and labels are `Spans` (as a consequence `Chart` no longer has generic bounds).

#### Incremental styling

Previously `Style` was used to represent an exhaustive set of style rules to be applied to an UI
element. It implied that whenever you wanted to change even only one property you had to provide the
complete style. For example, if you had a `Block` where you wanted to have a green background and
a title in bold, you had to do the following:

```rust
let style = Style::default().bg(Color::Green);
Block::default()
  .style(style)
  .title("My title")
  // Here we reused the style otherwise the background color would have been reset
  .title_style(style.modifier(Modifier::BOLD));
```

In this new release, you may now write this as:

```rust
Block::default()
    .style(Style::default().bg(Color::Green))
    // The style is not overridden anymore, we simply add new style rule for the title.
    .title(Span::styled("My title", Style::default().add_modifier(Modifier::BOLD)))
```

In addition, the crate now provides a method `patch` to combine two styles into a new set of style
rules:

```rust
let style = Style::default().modifier(Modifier::BOLD);
let style = style.patch(Style::default().add_modifier(Modifier::ITALIC));
// style.modifier == Modifier::BOLD | Modifier::ITALIC, the modifier has been enriched not overridden
```

- `Style::modifier` has been removed in favor of `Style::add_modifier` and `Style::remove_modifier`.
- `Buffer::set_style` has been added. `Buffer::set_background` is deprecated.
- `BarChart::style` no longer set the style of the bars. Use `BarChart::bar_style` in replacement.
- `Gauge::style` no longer set the style of the gauge. Use `Gauge::gauge_style` in replacement.

#### List with item on multiple lines

The `List` widget has been refactored once again to support items with variable heights and complex
styling.

- `List::new` expects an input that can be converted to a `Vec<ListItem>` where `ListItem` is a
wrapper around the item content to provide additional styling capabilities. `ListItem` contains a
`Text`.
- `List::items` has been removed.

```rust
// Before
let items = vec![
  "Item1",
  "Item2",
  "Item3"
];
List::default().items(items.iters());

// After
let items = vec![
  ListItem::new("Item1"),
  ListItem::new("Item2"),
  ListItem::new("Item3"),
];
List::new(items);
```

See the examples for more advanced usages.

#### More wrapping options

`Paragraph::wrap` expects `Wrap` instead of `bool` to let users decided whether they want to trim
whitespaces when the text is wrapped.

```rust
// before
Paragraph::new(text).wrap(true)
// after
Paragraph::new(text).wrap(Wrap { trim: true }) // to have the same behavior
Paragraph::new(text).wrap(Wrap { trim: false }) // to use the new behavior
```

#### Horizontal scrolling in paragraph

You can now scroll horizontally in `Paragraph`. The argument of `Paragraph::scroll` has thus be
changed from `u16` to `(u16, u16)`.

### Features

#### Serialization of style

You can now serialize and de-serialize `Style` using the optional `serde` feature.

## v0.9.5 - 2020-05-21

### Bug Fixes

* Fix out of bounds panic in `widgets::Tabs` when the widget is rendered on
small areas.

## v0.9.4 - 2020-05-12

### Bug Fixes

* Ignore zero-width graphemes in `Buffer::set_stringn`.

## v0.9.3 - 2020-05-11

### Bug Fixes

* Fix usize overflows in `widgets::Chart` when a dataset is empty.

## v0.9.2 - 2020-05-10

### Bug Fixes

* Fix usize overflows in `widgets::canvas::Line` drawing algorithm.

## v0.9.1 - 2020-04-16

### Bug Fixes

* The `List` widget now takes into account the width of the `highlight_symbol`
when calculating the total width of its items. It prevents items to overflow
outside of the widget area.

## v0.9.0 - 2020-04-14

### Features

* Introduce stateful widgets, i.e widgets that can take advantage of keeping
some state around between two draw calls (#210 goes a bit more into the
details).
* Allow a `Table` row to be selected.
```rust
// State initialization
let mut state = TableState::default();

// In the terminal.draw closure
let header = ["Col1", "Col2", "Col"];
let rows = [
  Row::Data(["Row11", "Row12", "Row13"].into_iter())
];
let table = Table::new(header.into_iter(), rows.into_iter());
f.render_stateful_widget(table, area, &mut state);

// In response to some event:
state.select(Some(1));
```
* Add a way to choose the type of border used to draw a block. You can now
choose from plain, rounded, double and thick lines.
* Add a `graph_type` property on the `Dataset` of a `Chart` widget. By
default it will be `Scatter` where the points are drawn as is. An other
option is `Line` where a line will be draw between each consecutive points
of the dataset.
* Style methods are now const, allowing you to initialize const `Style`
objects.
* Improve control over whether the legend in the `Chart` widget is shown or
not. You can now set custom constraints using
`Chart::hidden_legend_constraints`.
* Add `Table::header_gap` to add some space between the header and the first
row.
* Remove `log` from the dependencies
* Add a way to use a restricted set of unicode symbols in several widgets to
improve portability in exchange of a degraded output. (see `BarChart::bar_set`,
`Sparkline::bar_set` and `Canvas::marker`). You can check how the
`--enhanced-graphics` flag is used in the demos.

### Breaking Changes

* `Widget::render` has been deleted. You should now use `Frame::render_widget`
to render a widget on the corresponding `Frame`. This makes the `Widget`
implementation totally decoupled from the `Frame`.
```rust
// Before
Block::default().render(&mut f, size);

// After
let block = Block::default();
f.render_widget(block, size);
```
* `Widget::draw` has been renamed to `Widget::render` and the signature has
been updated to reflect that widgets are consumable objects. Thus the method
takes `self` instead of `&mut self`.
```rust
// Before
impl Widget for MyWidget {
  fn draw(&mut self, area: Rect, buf: &mut Buffer) {
  }
}

/// After
impl Widget for MyWidget {
  fn render(self, arera: Rect, buf: &mut Buffer) {
  }
}
```
* `Widget::background` has been replaced by `Buffer::set_background`
```rust
// Before
impl Widget for MyWidget {
  fn render(self, arera: Rect, buf: &mut Buffer) {
    self.background(area, buf, self.style.bg);
  }
}

// After
impl Widget for MyWidget {
  fn render(self, arera: Rect, buf: &mut Buffer) {
    buf.set_background(area, self.style.bg);
  }
}
```
* Update the `Shape` trait for objects that can be draw on a `Canvas` widgets.
Instead of returning an iterator over its points, a `Shape` is given a
`Painter` object that provides a `paint` as well as a `get_point` method. This
gives the `Shape` more information about the surface it will be drawn to. In
particular, this change allows the `Line` shape to use a more precise and
efficient drawing algorithm (Bresenham's line algorithm).
* `SelectableList` has been deleted. You can now take advantage of the
associated `ListState` of the `List` widget to select an item.
```rust
// Before
List::new(&["Item1", "Item2", "Item3"])
  .select(Some(1))
  .render(&mut f, area);

// After

// State initialization
let mut state = ListState::default();

// In the terminal.draw closure
let list = List::new(&["Item1", "Item2", "Item3"]);
f.render_stateful_widget(list, area, &mut state);

// In response to some events
state.select(Some(1));
```
* `widgets::Marker` has been moved to `symbols::Marker`

## v0.8.0 - 2019-12-15

### Breaking Changes

* Bump crossterm to 0.14.
* Add cross symbol to the symbols list.

### Bug Fixes

* Use the value of `title_style` to style the title of `Axis`.

## v0.7.0 - 2019-11-29

### Breaking Changes

* Use `Constraint` instead of integers to specify the widths of the `Table`
widget's columns. This will allow more responsive tables.

```rust
Table::new(header, row)
  .widths(&[15, 15, 10])
  .render(f, chunk);
```

becomes:

```rust
Table::new(header, row)
  .widths(&[
    Constraint::Length(15),
    Constraint::Length(15),
    Constraint::Length(10),
  ])
  .render(f, chunk);
```

* Bump crossterm to 0.13.
* Use Github Actions for CI (Travis and Azure Pipelines integrations have been deleted).

### Features

* Add support for horizontal and vertical margins in `Layout`.

## v0.6.2 - 2019-07-16

### Features

* `Text` implements PartialEq

### Bug Fixes

* Avoid overflow errors in canvas

## v0.6.1 - 2019-06-16

### Bug Fixes

* Avoid a division by zero when all values in a barchart are equal to 0.
* Fix the inverted cursor position in the curses backend.
* Ensure that the correct terminal size is returned when using the crossterm
backend.
* Avoid highlighting the separator after the selected item in the Tabs widget.

## v0.6.0 - 2019-05-18

### Breaking Changes

* Update crossterm backend

## v0.5.1 - 2019-04-14

### Bug Fixes

* Fix a panic in the Sparkline widget

## v0.5.0 - 2019-03-10

### Features

* Add a new curses backend (with Windows support thanks to `pancurses`).
* Add `Backend::get_cursor` and `Backend::set_cursor` methods to query and
set the position of the cursor.
* Add more constructors to the `Crossterm` backend.
* Add a demo for all backends using a shared UI and application state.
* Add `Ratio` as a new variant of layout `Constraint`. It can be used to define
exact ratios constraints.

### Breaking Changes

* Add support for multiple modifiers on the same `Style` by changing `Modifier`
from an enum to a bitflags struct.

So instead of writing:

```rust
let style = Style::default().add_modifier(Modifier::Italic);
```

one should use:

```rust
let style = Style::default().add_modifier(Modifier::ITALIC);
// or
let style = Style::default().add_modifier(Modifier::ITALIC | Modifier::BOLD);
```

### Bug Fixes

* Ensure correct behavior of the alternate screens with the `Crossterm` backend.
* Fix out of bounds panic when two `Buffer` are merged.

## v0.4.0 - 2019-02-03

### Features

* Add a new canvas shape: `Rectangle`.
* Official support of `Crossterm` backend.
* Make it possible to choose the divider between `Tabs`.
* Add word wrapping on Paragraph.
* The gauge widget accepts a ratio (f64 between 0 and 1) in addition of a
percentage.

### Breaking Changes

* Upgrade to Rust 2018 edition.

### Bug Fixes

* Fix rendering of double-width characters.
* Fix race condition on the size of the terminal and expose a size that is
safe to use when drawing through `Frame::size`.
* Prevent unsigned int overflow on large screens.

## v0.3.0 - 2018-11-04

### Features

* Add experimental test backend

## v0.3.0-beta.3 - 2018-09-24

### Features

* `show_cursor` is called when `Terminal` is dropped if the cursor is hidden.

## v0.3.0-beta.2 - 2018-09-23

### Breaking Changes

* Remove custom `termion` backends. This is motivated by the fact that
`termion` structs are meant to be combined/wrapped to provide additional
functionalities to the terminal (e.g AlternateScreen, Mouse support, ...).
Thus providing exclusive types do not make a lot of sense and give a false
hint that additional features cannot be used together. The recommended
approach is now to create your own version of `stdout`:

```rust
let stdout = io::stdout().into_raw_mode()?;
let stdout = MouseTerminal::from(stdout);
let stdout = AlternateScreen::from(stdout);
```

and then to create the corresponding `termion` backend:

```rust
let backend = TermionBackend::new(stdout);
```

The resulting code is more verbose but it works with all combinations of
additional `termion` features.

## v0.3.0-beta.1 - 2018-09-08

### Breaking Changes

* Replace `Item` by a generic and flexible `Text` that can be used in both
`Paragraph` and `List` widgets.
* Remove unnecessary borrows on `Style`.

## v0.3.0-beta.0 - 2018-09-04

### Features

* Add a basic `Crossterm` backend

### Breaking Changes

* Remove `Group` and introduce `Layout` in its place
  - `Terminal` is no longer required to compute a layout
  - `Size` has been renamed `Constraint`
* Widgets are rendered on a `Frame` instead of a `Terminal` in order to
avoid mixing `draw` and `render` calls
* `draw` on `Terminal` expects a closure where the UI is built by rendering
widgets on the given `Frame`
* Update `Widget` trait
  - `draw` takes area by value
  - `render` takes a `Frame` instead of a `Terminal`
* All widgets use the consumable builder pattern
* `SelectableList` can have no selected item and the highlight symbol is hidden
in this case
* Remove markup language inside `Paragraph`. `Paragraph` now expects an iterator
of `Text` items

## v0.2.3 - 2018-06-09

### Features

* Add `start_corner` option for `List`
* Add more text alignment options for `Paragraph`

## v0.2.2 - 2018-05-06

### Features

* `Terminal` implements `Debug`

### Breaking Changes

* Use `FnOnce` instead of `FnMut` in Group::render

## v0.2.1 - 2018-04-01

### Features

* Add `AlternateScreenBackend` in `termion` backend
* Add `TermionBackend::with_stdout` in order to let an user of the library
provides its own termion struct
* Add tests and documentation for `Buffer::pos_of`
* Remove leading whitespaces when wrapping text

### Bug Fixes

* Fix `debug_assert` in `Buffer::pos_of`
* Pass the style of `SelectableList` to the underlying `List`
* Fix missing character when wrapping text
* Fix panic when specifying layout constraints

## v0.2.0 - 2017-12-26

### Features

* Add `MouseBackend` in `termion` backend to handle scroll and mouse events
* Add generic `Item` for items in a `List`
* Drop `log4rs` as a dev-dependencies in favor of `stderrlog`

### Breaking Changes

* Rename `TermionBackend` to `RawBackend` (to distinguish it from the `MouseBackend`)
* Generic parameters for `List` to allow passing iterators as items
* Generic parameters for `Table` to allow using iterators as rows and header
* Generic parameters for `Tabs`
* Rename `border` bitflags to `Borders`
