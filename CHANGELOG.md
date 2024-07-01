# Changelog

All notable changes to this project will be documented in this file.


## [unreleased]

### Features

- [36d49e5](https://github.com/ratatui-org/ratatui/commit/36d49e549b9e19e87e71c23afaee274fa7415fde) *(table)* Select first, last, etc to table state by @robertpsoane in [#1198](https://github.com/ratatui-org/ratatui/pull/1198)

  > Add select_previous, select_next, select_first & select_last to
  > TableState
  >
  > Used equivalent API as in ListState

### Refactor

- [32a0b26](https://github.com/ratatui-org/ratatui/commit/32a0b265253cb83cbf1d51784abf150fed7ef82f) *(uncategorized)* Simplify WordWrapper implementation by @tranzystorekk in [#1193](https://github.com/ratatui-org/ratatui/pull/1193)

### Documentation

- [55e0880](https://github.com/ratatui-org/ratatui/commit/55e0880d2fef35b328901b4194d39101fe26a9e9) *(block)* Update block documentation by @leohscl in [#1206](https://github.com/ratatui-org/ratatui/pull/1206)

  > Update block documentation with constructor methods and setter methods
  > in the main doc comment Added an example for using it to surround
  > widgets
  >
  > Fixes:https://github.com/ratatui-org/ratatui/issues/914

### Miscellaneous Tasks

- [3e7458f](https://github.com/ratatui-org/ratatui/commit/3e7458fdb8051b9a62aac551372d5592e7f59eb7) *(github)* Add forums and faqs to the issue template by @joshka in [#1201](https://github.com/ratatui-org/ratatui/pull/1201)

- [ccf83e6](https://github.com/ratatui-org/ratatui/commit/ccf83e6d7610bf74f4ab02e0b1e2fe0e55ad9e78) *(uncategorized)* Update labels in issue templates by @joshka in [#1212](https://github.com/ratatui-org/ratatui/pull/1212)



### New Contributors
* @leohscl made their first contribution in [#1206](https://github.com/ratatui-org/ratatui/pull/1206)
* @robertpsoane made their first contribution in [#1198](https://github.com/ratatui-org/ratatui/pull/1198)


## [v0.27.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.27.0) - 2024-06-24

### Features

- [eef1afe](https://github.com/ratatui-org/ratatui/commit/eef1afe9155089dca489a9159c368a5ac07e7585) *(linegauge)* Allow LineGauge background styles by @nowNick in [#565](https://github.com/ratatui-org/ratatui/pull/565)

  > This PR deprecates `gauge_style` in favor of `filled_style` and
  > `unfilled_style` which can have it's foreground and background styled.
  >
  > `cargo run --example=line_gauge --features=crossterm`
  >
  > https://github.com/ratatui-org/ratatui/assets/5149215/5fb2ce65-8607-478f-8be4-092e08612f5b
  >
  > Implements:<https://github.com/ratatui-org/ratatui/issues/424>

- [1365620](https://github.com/ratatui-org/ratatui/commit/13656206064b53c7f86f179b570c7769399212a3) *(borders)* Add FULL and EMPTY border sets by @joshka in [#1182](https://github.com/ratatui-org/ratatui/pull/1182)
  >
  > `border::FULL` uses a full block symbol, while `border::EMPTY` uses an
  > empty space. This is useful for when you need to allocate space for the
  > border and apply the border style to a block without actually drawing a
  > border. This makes it possible to style the entire title area or a block
  > rather than just the title content.
  >
  > ```rust
  > use ratatui::{symbols::border, widgets::Block};
  > let block = Block::bordered().title("Title").border_set(border::FULL);
  > let block = Block::bordered().title("Title").border_set(border::EMPTY);
  > ```

- [7a48c5b](https://github.com/ratatui-org/ratatui/commit/7a48c5b11b3d51b915ccc187d0499b6e0e88b89d) *(cell)* Add EMPTY and (const) new method by @EdJoPaTo in [#1143](https://github.com/ratatui-org/ratatui/pull/1143)

  > This simplifies calls to `Buffer::filled` in tests.

- [3f2f2cd](https://github.com/ratatui-org/ratatui/commit/3f2f2cd6abf67a04809ff314025a462a3c2e2446) *(docs)* Add tracing example by @joshka in [#1192](https://github.com/ratatui-org/ratatui/pull/1192)

  > Add an example that demonstrates logging to a file for:
  >
  > <https://forum.ratatui.rs/t/how-do-you-println-debug-your-tui-programs/66>
  >
  > ```shell
  > cargo run --example tracing
  > RUST_LOG=trace cargo run --example=tracing
  > cat tracing.log
  > ```
  >
  > ![Made with VHS](https://vhs.charm.sh/vhs-21jgJCedh2YnFDONw0JW7l.gif)

- [1520ed9](https://github.com/ratatui-org/ratatui/commit/1520ed9d106f99580a9e529212e43dac06a2f6d2) *(layout)* Impl Display for Position and Size by @joshka in [#1162](https://github.com/ratatui-org/ratatui/pull/1162)

- [46977d8](https://github.com/ratatui-org/ratatui/commit/46977d88519d28ccac1c94e171af0c9cca071dbc) *(list)* Add list navigation methods (first, last, previous, next) by @joshka in [#1159](https://github.com/ratatui-org/ratatui/pull/1159) [**breaking**]

  > Also cleans up the list example significantly (see also
  > <https://github.com/ratatui-org/ratatui/issues/1157>)
  >
  > Fixes:<https://github.com/ratatui-org/ratatui/pull/1159>
  >
  > BREAKING CHANGE:The `List` widget now clamps the selected index to the
  > bounds of the list when navigating with `first`, `last`, `previous`, and
  > `next`, as well as when setting the index directly with `select`.

- [10d7788](https://github.com/ratatui-org/ratatui/commit/10d778866edea55207ff3f03d063c9fec619b9c9) *(style)* Add conversions from the palette crate colors by @joshka in [#1172](https://github.com/ratatui-org/ratatui/pull/1172)

  > This is behind the "palette" feature flag.
  >
  > ```rust
  > use palette::{LinSrgb, Srgb};
  > use ratatui::style::Color;
  >
  > let color = Color::from(Srgb::new(1.0f32, 0.0, 0.0));
  > let color = Color::from(LinSrgb::new(1.0f32, 0.0, 0.0));
  > ```

- [7ef2dae](https://github.com/ratatui-org/ratatui/commit/7ef2daee060a7fe964a8de64eafcb6062228e035) *(text)* Support constructing `Line` and `Text` from `usize` by @orhun in [#1167](https://github.com/ratatui-org/ratatui/pull/1167)

  > Now you can create `Line` and `Text` from numbers like so:
  >
  > ```rust
  > let line = Line::from(42);
  > let text = Text::from(666);
  > ```
  >
  > (I was doing little testing for my TUI app and saw that this isn't
  > supported - then I was like WHA and decided to make it happen :tm:)

- [74a32af](https://github.com/ratatui-org/ratatui/commit/74a32afbaef8851f9462b27094d88d518e56addf) *(uncategorized)* Re-export backends from the ratatui crate by @joshka in [#1151](https://github.com/ratatui-org/ratatui/pull/1151)

  > `crossterm`, `termion`, and `termwiz` can now be accessed as
  > `ratatui::{crossterm, termion, termwiz}` respectively. This makes it
  > possible to just add the Ratatui crate as a dependency and use the
  > backend of choice without having to add the backend crates as
  > dependencies.
  >
  > To update existing code, replace all instances of `crossterm::` with
  > `ratatui::crossterm::`, `termion::` with `ratatui::termion::`, and
  > `termwiz::` with `ratatui::termwiz::`.

- [3594180](https://github.com/ratatui-org/ratatui/commit/35941809e11ab43309dd83a8f67bb375f5e7ff2b) *(uncategorized)* Make Stylize's `.bg(color)` generic by @kdheepak in [#1103](https://github.com/ratatui-org/ratatui/pull/1103) [**breaking**]

- [0b5fd6b](https://github.com/ratatui-org/ratatui/commit/0b5fd6bf8eb64662df96900faea3608d4cbb3984) *(uncategorized)* Add writer() and writer_mut() to termion and crossterm backends by @enricozb in [#991](https://github.com/ratatui-org/ratatui/pull/991)

  > It is sometimes useful to obtain access to the writer if we want to see
  > what has been written so far. For example, when using &mut [u8] as a
  > writer.

### Bug Fixes

- [efa965e](https://github.com/ratatui-org/ratatui/commit/efa965e1e806c60cb1bdb2d1715f960db0857704) *(line)* Remove newlines when converting strings to Lines by @joshka in [#1191](https://github.com/ratatui-org/ratatui/pull/1191)
  >
  > `Line::from("a\nb")` now returns a line with two `Span`s instead of 1
  >
  > Fixes:https://github.com/ratatui-org/ratatui/issues/1111

- [d370aa7](https://github.com/ratatui-org/ratatui/commit/d370aa75af99da3e0c41ceb28e2d02ee81cd2538) *(span)* Ensure that zero-width characters are rendered correctly by @joshka in [#1165](https://github.com/ratatui-org/ratatui/pull/1165)

- [127d706](https://github.com/ratatui-org/ratatui/commit/127d706ee4876a58230f42f4a730b18671eae167) *(table)* Ensure render offset without selection properly by @joshka in [#1187](https://github.com/ratatui-org/ratatui/pull/1187)
  >
  > Fixes:<https://github.com/ratatui-org/ratatui/issues/1179>

- [4bfdc15](https://github.com/ratatui-org/ratatui/commit/4bfdc15b80ba14489d359ab1f88564c3bd016c19) *(uncategorized)* Render of &str and String doesn't respect area.width by @thscharler in [#1177](https://github.com/ratatui-org/ratatui/pull/1177)

- [e6871b9](https://github.com/ratatui-org/ratatui/commit/e6871b9e21c25acf1e203f4860198c37aa9429a1) *(uncategorized)* Avoid unicode-width breaking change in tests by @joshka in [#1171](https://github.com/ratatui-org/ratatui/pull/1171)

  > unicode-width 0.1.13 changed the width of \u{1} from 0 to 1.
  > Our tests assumed that \u{1} had a width of 0, so this change replaces
  > the \u{1} character with \u{200B} (zero width space) in the tests.
  >
  > Upstream issue (closed as won't fix):
  > https://github.com/unicode-rs/unicode-width/issues/55

- [7f3efb0](https://github.com/ratatui-org/ratatui/commit/7f3efb02e6f846fc72079f0921abd2cee09d2d83) *(uncategorized)* Pin unicode-width crate to 0.1.13 by @joshka in [#1170](https://github.com/ratatui-org/ratatui/pull/1170)

  > semver breaking change in 0.1.13
  > <https://github.com/unicode-rs/unicode-width/issues/55>
  >
  >

- [42cda6d](https://github.com/ratatui-org/ratatui/commit/42cda6d28706bf83308787ca784f374f6c286a02) *(uncategorized)* Prevent panic from string_slice by @EdJoPaTo in [#1140](https://github.com/ratatui-org/ratatui/pull/1140)
  >
  > <https://rust-lang.github.io/rust-clippy/master/index.html#string_slice>

### Refactor

- [73fd367](https://github.com/ratatui-org/ratatui/commit/73fd367a740924ce80ef7a0cd13a66b5094f7a54) *(block)* Group builder pattern methods by @EdJoPaTo in [#1134](https://github.com/ratatui-org/ratatui/pull/1134)

- [257db62](https://github.com/ratatui-org/ratatui/commit/257db6257f392a07ee238b439344d91566beb740) *(cell)* Must_use and simplify style() by @EdJoPaTo in [#1124](https://github.com/ratatui-org/ratatui/pull/1124)

  >

- [bf20369](https://github.com/ratatui-org/ratatui/commit/bf2036987f04d83f4f2b8338fab1b4fd7f4cc81d) *(cell)* Reset instead of applying default by @EdJoPaTo in [#1127](https://github.com/ratatui-org/ratatui/pull/1127)

  > Using reset is clearer to me what actually happens. On the other case a
  > struct is created to override the old one completely which basically
  > does the same in a less clear way.

- [7d175f8](https://github.com/ratatui-org/ratatui/commit/7d175f85c1905c08adf547dd37cc89c63039f480) *(lint)* Fix new lint warnings by @EdJoPaTo in [#1178](https://github.com/ratatui-org/ratatui/pull/1178)

- [cf67ed9](https://github.com/ratatui-org/ratatui/commit/cf67ed9b884347cef034b09e0e9f9d4aff74ab0a) *(lint)* Use clippy::or_fun_call by @EdJoPaTo in [#1138](https://github.com/ratatui-org/ratatui/pull/1138)
  >
  > <https://rust-lang.github.io/rust-clippy/master/index.html#or_fun_call>

- [4770e71](https://github.com/ratatui-org/ratatui/commit/4770e715819475cdca2f2ccdbac00cba203cd6d2) *(list)* Remove deprecated `start_corner` and `Corner` by @Valentin271 in [#759](https://github.com/ratatui-org/ratatui/pull/759) [**breaking**]
  >
  > `List::start_corner` was deprecated in v0.25. Use `List::direction` and
  > `ListDirection` instead.
  >
  > ```diff
  > - list.start_corner(Corner::TopLeft);
  > - list.start_corner(Corner::TopRight);
  > // This is not an error, BottomRight rendered top to bottom previously
  > - list.start_corner(Corner::BottomRight);
  > // all becomes
  > + list.direction(ListDirection::TopToBottom);
  > ```
  >
  > ```diff
  > - list.start_corner(Corner::BottomLeft);
  > // becomes
  > + list.direction(ListDirection::BottomToTop);
  > ```
  >
  > `layout::Corner` is removed entirely.

- [4f77910](https://github.com/ratatui-org/ratatui/commit/4f7791079edd16b54dc8cdfc95bb72b282a09576) *(padding)* Add Padding::ZERO as a constant by @EdJoPaTo in [#1133](https://github.com/ratatui-org/ratatui/pull/1133)

  > Deprecate Padding::zero()

- [8061813](https://github.com/ratatui-org/ratatui/commit/8061813f324c08e11196e62fca22c2f6b9216b7e) *(uncategorized)* Expand glob imports by @joshka in [#1152](https://github.com/ratatui-org/ratatui/pull/1152)

  > Consensus is that explicit imports make it easier to understand the
  > example code. This commit removes the prelude import from all examples
  > and replaces it with the necessary imports, and expands other glob
  > imports (widget::*, Constraint::*, KeyCode::*, etc.) everywhere else.
  > Prelude glob imports not in examples are not covered by this PR.
  >
  > See https://github.com/ratatui-org/ratatui/issues/1150 for more details.

- [d929971](https://github.com/ratatui-org/ratatui/commit/d92997105bde15a1fd43829466ec8cc46bffe121) *(uncategorized)* Dont manually impl Default for defaults by @EdJoPaTo in [#1142](https://github.com/ratatui-org/ratatui/pull/1142)

  > Replace `impl Default` by `#[derive(Default)]` when its implementation
  > equals.

- [8a60a56](https://github.com/ratatui-org/ratatui/commit/8a60a561c95691912cbf41d55866abafcba0127d) *(uncategorized)* Needless_pass_by_ref_mut by @EdJoPaTo in [#1137](https://github.com/ratatui-org/ratatui/pull/1137)
  >
  > <https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_ref_mut>

- [1de9a82](https://github.com/ratatui-org/ratatui/commit/1de9a82b7a871a83995d224785cae139c6f4787b) *(uncategorized)* Simplify if let by @EdJoPaTo in [#1135](https://github.com/ratatui-org/ratatui/pull/1135)

  > While looking through lints
  > [`clippy::option_if_let_else`](https://rust-lang.github.io/rust-clippy/master/index.html#option_if_let_else)
  > found these. Other findings are more complex so I skipped them.

### Documentation

- [1908b06](https://github.com/ratatui-org/ratatui/commit/1908b06b4a497ff1cfb2c8d8c165d2a241ee1864) *(borders)* Add missing closing code blocks by @orhun in [#1195](https://github.com/ratatui-org/ratatui/pull/1195)

- [38bb196](https://github.com/ratatui-org/ratatui/commit/38bb19640449c7a3eee3a2fba6450071395e5e06) *(breaking-changes)* Mention `LineGauge::gauge_style` by @orhun in [#1194](https://github.com/ratatui-org/ratatui/pull/1194)
  >
  > see #565

- [07efde5](https://github.com/ratatui-org/ratatui/commit/07efde5233752e1bcb7ae94a91b9e36b7fadb01b) *(examples)* Add hyperlink example by @joshka in [#1063](https://github.com/ratatui-org/ratatui/pull/1063)

- [7fdccaf](https://github.com/ratatui-org/ratatui/commit/7fdccafd52f4ddad1a3c9dda59fada59af21ecfa) *(examples)* Add vhs tapes for constraint-explorer and minimal examples by @joshka in [#1164](https://github.com/ratatui-org/ratatui/pull/1164)

- [4f307e6](https://github.com/ratatui-org/ratatui/commit/4f307e69db058891675d0f12d75ef49006c511d6) *(examples)* Simplify paragraph example by @joshka in [#1169](https://github.com/ratatui-org/ratatui/pull/1169)
  >
  > Related:https://github.com/ratatui-org/ratatui/issues/1157

- [f429f68](https://github.com/ratatui-org/ratatui/commit/f429f688da536a52266144e63a1a7897ec6b7f26) *(examples)* Remove lifetimes from the List example by @matta in [#1132](https://github.com/ratatui-org/ratatui/pull/1132)

  > Simplify the List example by removing lifetimes not strictly necessary
  > to demonstrate how Ratatui lists work. Instead, the sample strings are
  > copied into each `TodoItem`. To further simplify, I changed the code to
  > use a new TodoItem::new function, rather than an implementation of the
  > `From` trait.

- [308c1df](https://github.com/ratatui-org/ratatui/commit/308c1df6495ee4373f808007a1566ca7e9279933) *(readme)* Add links to forum by @joshka in [#1188](https://github.com/ratatui-org/ratatui/pull/1188)

- [2f8a936](https://github.com/ratatui-org/ratatui/commit/2f8a9363fc6c54fe2b10792c9f57fbb40b06bc0f) *(uncategorized)* Fix links on docs.rs by @EdJoPaTo in [#1144](https://github.com/ratatui-org/ratatui/pull/1144)

  > This also results in a more readable Cargo.toml as the locations of the
  > things are more obvious now.
  >
  > Includes rewording of the underline-color feature.
  >
  > Logs of the errors: https://docs.rs/crate/ratatui/0.26.3/builds/1224962
  > Also see #989

### Performance

- [4ce67fc](https://github.com/ratatui-org/ratatui/commit/4ce67fc84e3bc472e9ae97aece85f8ffae091834) *(buffer)* Filled moves the cell to be filled by @EdJoPaTo in [#1148](https://github.com/ratatui-org/ratatui/pull/1148) [**breaking**]

- [8b447ec](https://github.com/ratatui-org/ratatui/commit/8b447ec4d6276c3110285e663417487ff18dafc1) *(rect)* `Rect::inner` takes `Margin` directly instead of reference by @EdJoPaTo in [#1008](https://github.com/ratatui-org/ratatui/pull/1008) [**breaking**]
  >
  > BREAKING CHANGE:Margin needs to be passed without reference now.
  >
  > ```diff
  > -let area = area.inner(&Margin {
  > +let area = area.inner(Margin {
  >      vertical: 0,
  >      horizontal: 2,
  >  });
  > ```

### Styling

- [df4b706](https://github.com/ratatui-org/ratatui/commit/df4b706674de806bdf2a1fb8c04d0654b6b0b891) *(uncategorized)* Enable more rustfmt settings by @EdJoPaTo in [#1125](https://github.com/ratatui-org/ratatui/pull/1125)

### Testing

- [d6587bc](https://github.com/ratatui-org/ratatui/commit/d6587bc6b0db955aeac6af167e1b8ef81a3afcc9) *(style)* Use rstest by @EdJoPaTo in [#1136](https://github.com/ratatui-org/ratatui/pull/1136)

  >

### Miscellaneous Tasks

- [7b45f74](https://github.com/ratatui-org/ratatui/commit/7b45f74b719ff18329ddbf9f05a9ac53bf06f71d) *(prelude)* Add / remove items by @joshka in [#1149](https://github.com/ratatui-org/ratatui/pull/1149) [**breaking**]

  > his PR removes the items from the prelude that don't form a coherent
  > common vocabulary and adds the missing items that do.
  >
  > Based on a comment at
  > <https://www.reddit.com/r/rust/comments/1cle18j/comment/l2uuuh7/>
  >
  > BREAKING CHANGE:The following items have been removed from the prelude:
  > - `style::Styled` - this trait is useful for widgets that want to
  >   support the Stylize trait, but it adds complexity as widgets have two
  >   `style` methods and a `set_style` method.
  > - `symbols::Marker` - this item is used by code that needs to draw to
  >   the `Canvas` widget, but it's not a common item that would be used by
  >   most users of the library.
  > - `terminal::{CompletedFrame, TerminalOptions, Viewport}` - these items
  >   are rarely used by code that needs to interact with the terminal, and
  >   they're generally only ever used once in any app.
  >
  > The following items have been added to the prelude:
  > - `layout::{Position, Size}` - these items are used by code that needs
  >   to interact with the layout system. These are newer items that were
  >   added in the last few releases, which should be used more liberally.

- [cd64367](https://github.com/ratatui-org/ratatui/commit/cd64367e244a1588206f653fd79678ce62a86a2f) *(symbols)* Add tests for line symbols by @joshka in [#1186](https://github.com/ratatui-org/ratatui/pull/1186)

- [8cfc316](https://github.com/ratatui-org/ratatui/commit/8cfc316bccb48e88660d14cd18c0df2264c4d6ce) *(uncategorized)* Alphabetize examples in Cargo.toml by @joshka in [#1145](https://github.com/ratatui-org/ratatui/pull/1145)

### Build

- [70df102](https://github.com/ratatui-org/ratatui/commit/70df102de0154cdfbd6508659cf6ed649f820bc8) *(bench)* Improve benchmark consistency by @EdJoPaTo in [#1126](https://github.com/ratatui-org/ratatui/pull/1126)

  > Codegen units are optimized on their own. Per default bench / release
  > have 16 codegen units. What ends up in a codeget unit is rather random
  > and can influence a benchmark result as a code change can move stuff
  > into a different codegen unit → prevent / allow LLVM optimizations
  > unrelated to the actual change.
  >
  > More details: https://doc.rust-lang.org/cargo/reference/profiles.html



### New Contributors
* @thscharler made their first contribution in [#1177](https://github.com/ratatui-org/ratatui/pull/1177)
* @matta made their first contribution in [#1132](https://github.com/ratatui-org/ratatui/pull/1132)
* @nowNick made their first contribution in [#565](https://github.com/ratatui-org/ratatui/pull/565)
* @enricozb made their first contribution in [#991](https://github.com/ratatui-org/ratatui/pull/991)

**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.26.3...v0.27.0


## [v0.26.3](https://github.com/ratatui-org/ratatui/releases/tag/v0.26.3) - 2024-05-20

### Features

- [97ee102](https://github.com/ratatui-org/ratatui/commit/97ee102f179eed4f309d575495f0e4c8359b4f04) *(buffer)* Track_caller for index_of by @EdJoPaTo in [#1046](https://github.com/ratatui-org/ratatui/pull/1046)

  > The caller put in the wrong x/y -> the caller is the cause.

- [bf09234](https://github.com/ratatui-org/ratatui/commit/bf0923473c5cb7f2cff24b010f0072b5ce2f8cf2) *(table)* Make TableState::new const by @EdJoPaTo in [#1040](https://github.com/ratatui-org/ratatui/pull/1040)

- [eb281df](https://github.com/ratatui-org/ratatui/commit/eb281df97482c2aab66875dc27a49a316a4d7fd7) *(uncategorized)* Use inner Display implementation by @EdJoPaTo in [#1097](https://github.com/ratatui-org/ratatui/pull/1097)

- [ec763af](https://github.com/ratatui-org/ratatui/commit/ec763af8512df731799c8f30c38c37252068a4c4) *(uncategorized)* Make Stylize's `.bg(color)` generic by @kdheepak in [#1099](https://github.com/ratatui-org/ratatui/pull/1099)

  > This PR makes `.bg(color)` generic accepting anything that can be
  > converted into `Color`; similar to the `.fg(color)` method on the same
  > trait

- [4d1784f](https://github.com/ratatui-org/ratatui/commit/4d1784f2de104b88e998216addaae96ab018f44f) *(uncategorized)* Re-export ParseColorError as style::ParseColorError by @joshka in [#1086](https://github.com/ratatui-org/ratatui/pull/1086)
  >
  > Fixes:https://github.com/ratatui-org/ratatui/issues/1085

### Bug Fixes

- [366cbae](https://github.com/ratatui-org/ratatui/commit/366cbae09fb2bf5b5d7f489de1ff15f930569f05) *(buffer)* Fix Debug panic and fix formatting of overridden parts by @EdJoPaTo in [#1098](https://github.com/ratatui-org/ratatui/pull/1098)

  > Fix panic in `Debug for Buffer` when `width == 0`.
  > Also corrects the output when symbols are overridden.

- [4392759](https://github.com/ratatui-org/ratatui/commit/43927595012254b33a3901e0d2e5d28164ad04f0) *(examples)* Changed user_input example to work with multi-byte unicode chars by @OkieOth in [#1069](https://github.com/ratatui-org/ratatui/pull/1069)

  > This is the proposed solution for issue #1068. It solves the bug in the
  > user_input example with multi-byte UTF-8 characters as input.
  >
  > Fixes:#1068
  >
  > ---------

- [20fc0dd](https://github.com/ratatui-org/ratatui/commit/20fc0ddfca97a863c9ec7537bcf283d3d49baab4) *(examples)* Fix key handling in constraints by @psobolik in [#1066](https://github.com/ratatui-org/ratatui/pull/1066)

  > Add check for `KeyEventKind::Press` to constraints example's event
  > handler to eliminate double keys
  > on Windows.
  >
  > Fixes:#1062
  >
  > ---------

- [f4637d4](https://github.com/ratatui-org/ratatui/commit/f4637d40c35e068fd60d17c9a42b9114667c9861) *(reflow)* Allow wrapping at zero width whitespace by @kxxt in [#1074](https://github.com/ratatui-org/ratatui/pull/1074)

- [fcb5d58](https://github.com/ratatui-org/ratatui/commit/fcb5d589bbf71b1204302465d2754ec6ed810090) *(uncategorized)* Make cargo test --doc work with unstable-widget-ref examples by @joshka in [#1117](https://github.com/ratatui-org/ratatui/pull/1117)

- [699c2d7](https://github.com/ratatui-org/ratatui/commit/699c2d7c8d0e8c2023cf75350b66535a7b48a102) *(uncategorized)* Unicode truncation bug by @joshka in [#1089](https://github.com/ratatui-org/ratatui/pull/1089)

  > - Rewrote the line / span rendering code to take into account how
  > multi-byte / wide emoji characters are truncated when rendering into
  > areas that cannot accommodate them in the available space
  > - Added comprehensive coverage over the edge cases
  > - Adds a benchmark to ensure perf
  >
  > Fixes:https://github.com/ratatui-org/ratatui/issues/1032

- [b30411d](https://github.com/ratatui-org/ratatui/commit/b30411d1c71cb7b43b7232226514caa54a56c25f) *(uncategorized)* Termwiz underline color test by @joshka in [#1094](https://github.com/ratatui-org/ratatui/pull/1094)

  > Fixes code that doesn't compile in the termwiz tests when
  > underline-color feature is enabled.

- [5f1e119](https://github.com/ratatui-org/ratatui/commit/5f1e119563043e97e5c2c5e7dd48ccd75e17791e) *(uncategorized)* Correct feature flag typo for termwiz by @joshka in [#1088](https://github.com/ratatui-org/ratatui/pull/1088)

  > underline-color was incorrectly spelt as underline_color

- [0a16496](https://github.com/ratatui-org/ratatui/commit/0a164965ea2b163433871717cee4fd774a23ee5a) *(uncategorized)* Use `to_string` to serialize Color by @SleepySwords in [#934](https://github.com/ratatui-org/ratatui/pull/934)

  > Since deserialize now uses `FromStr` to deserialize color, serializing
  > `Color` RGB values, as well as index values, would produce an output
  > that would no longer be able to be deserialized without causing an
  > error.
  >
  > Color::Rgb will now be serialized as the hex representation of their
  > value.
  > For example, with serde_json, `Color::Rgb(255, 0, 255)` would be
  > serialized as `"#FF00FF"` rather than `{"Rgb": [255, 0, 255]}`.
  >
  > Color::Indexed will now be serialized as just the string of the index.
  > For example, with serde_json, `Color::Indexed(10)` would be serialized
  > as `"10"` rather than `{"Indexed": 10}`.
  >
  > Other color variants remain the same.

### Refactor

- [2cfe82a](https://github.com/ratatui-org/ratatui/commit/2cfe82a47eb34baa25f474db7be364de7b95374a) *(buffer)* Deprecate assert_buffer_eq! in favor of assert_eq! by @EdJoPaTo in [#1007](https://github.com/ratatui-org/ratatui/pull/1007)

  > - Simplify `assert_buffer_eq!` logic.
  > - Deprecate `assert_buffer_eq!`.
  > - Introduce `TestBackend::assert_buffer_lines`.
  >
  > Also simplify many tests involving buffer comparisons.
  >
  > For the deprecation, just use `assert_eq` instead of `assert_buffer_eq`:
  >
  > ```diff
  > -assert_buffer_eq!(actual, expected);
  > +assert_eq!(actual, expected);
  > ```
  >
  > ---
  >
  > I noticed `assert_buffer_eq!` creating no test coverage reports and
  > looked into this macro. First I simplified it. Then I noticed a bunch of
  > `assert_eq!(buffer, …)` and other indirect usages of this macro (like
  > `TestBackend::assert_buffer`).
  >
  > The good thing here is that it's mainly used in tests so not many
  > changes to the library code.

- [baedc39](https://github.com/ratatui-org/ratatui/commit/baedc39494ea70292b1d247934420a20d0544b7e) *(buffer)* Simplify set_stringn logic by @EdJoPaTo in [#1083](https://github.com/ratatui-org/ratatui/pull/1083)

- [9bd89c2](https://github.com/ratatui-org/ratatui/commit/9bd89c218afb1f3999dce1bfe6edea5b7442966d) *(clippy)* Enable breaking lint checks by @EdJoPaTo in [#988](https://github.com/ratatui-org/ratatui/pull/988)

  > We need to make sure to not change existing methods without a notice.
  > But at the same time this also finds public additions with mistakes
  > before they are even released which is what I would like to have.
  >
  > This renames a method and deprecated the old name hinting to a new name.
  > Should this be mentioned somewhere, so it's added to the release notes?
  > It's not breaking because the old method is still there.

- [bef5bcf](https://github.com/ratatui-org/ratatui/commit/bef5bcf750375a78b11ae06f217091b2463e842f) *(example)* Remove pointless new method by @EdJoPaTo in [#1038](https://github.com/ratatui-org/ratatui/pull/1038)

  > Use `App::default()` directly.

- [f3172c5](https://github.com/ratatui-org/ratatui/commit/f3172c59d4dae6ce4909251976a39c21d88f1907) *(gauge)* Fix internal typo by @EdJoPaTo in [#1048](https://github.com/ratatui-org/ratatui/pull/1048)

### Documentation

- [da1ade7](https://github.com/ratatui-org/ratatui/commit/da1ade7b2e4d8909ea0001483780d2c907349fd6) *(github)* Update code owners about past maintainers by @orhun in [#1073](https://github.com/ratatui-org/ratatui/pull/1073)

  > As per suggestion in
  > https://github.com/ratatui-org/ratatui/pull/1067#issuecomment-2079766990
  >
  > It's good for historical purposes!

- [3687f78](https://github.com/ratatui-org/ratatui/commit/3687f78f6a06bd175eda3e19819f6dc68012fb59) *(github)* Update code owners by @orhun in [#1067](https://github.com/ratatui-org/ratatui/pull/1067)

  > Removes the team members that are not able to review PRs recently (with
  > their approval ofc)

- [839cca2](https://github.com/ratatui-org/ratatui/commit/839cca20bf3f109352ea43f1119e13c879e04b95) *(table)* Fix typo in docs for highlight_symbol by @kdheepak in [#1108](https://github.com/ratatui-org/ratatui/pull/1108)

- [f945a0b](https://github.com/ratatui-org/ratatui/commit/f945a0bcff644c1fa2ad3caaa87cf2b640beaf46) *(test)* Fix typo in TestBackend documentation by @orhun in [#1107](https://github.com/ratatui-org/ratatui/pull/1107)

- [828d17a](https://github.com/ratatui-org/ratatui/commit/828d17a3f5f449255d7981bb462bf48382c7cb2e) *(uncategorized)* Add minimal example by @joshka in [#1114](https://github.com/ratatui-org/ratatui/pull/1114)

- [e95230b](https://github.com/ratatui-org/ratatui/commit/e95230beda9f86dfb7a9bc1c1167e5a91a2748c3) *(uncategorized)* Add note about scrollbar state content length by @Utagai in [#1077](https://github.com/ratatui-org/ratatui/pull/1077)

### Performance

- [366c2a0](https://github.com/ratatui-org/ratatui/commit/366c2a0e6d17810b26ba37918e72c2f784176d2c) *(block)* Use Block::bordered by @EdJoPaTo in [#1041](https://github.com/ratatui-org/ratatui/pull/1041)
  >
  > `Block::bordered()` is shorter than
  >
  > `Block::new().borders(Borders::ALL)`, requires one less import
  > (`Borders`) and in case `Block::default()` was used before can even be
  > `const`.

- [2e71c18](https://github.com/ratatui-org/ratatui/commit/2e71c1874e2de6d9f2bd21622246e55484a9fc62) *(buffer)* Simplify Buffer::filled with macro by @EdJoPaTo in [#1036](https://github.com/ratatui-org/ratatui/pull/1036)

  > The `vec![]` macro is highly optimized by the Rust team and shorter.
  > Don't do it manually.
  >
  > This change is mainly cleaner code. The only production code that uses
  > this is `Terminal::with_options` and `Terminal::insert_before` so it's
  > not performance relevant on every render.

- [81b9633](https://github.com/ratatui-org/ratatui/commit/81b96338ea41f9e5fbb0868808a0b450f31eef41) *(calendar)* Use const fn by @EdJoPaTo in [#1039](https://github.com/ratatui-org/ratatui/pull/1039)

  > Also, do the comparison without `as u8`. Stays the same at runtime and
  > is cleaner code.

- [c442dfd](https://github.com/ratatui-org/ratatui/commit/c442dfd1ad4896e7abeeaac1754b94bae1f8d014) *(canvas)* Change map data to const instead of static by @EdJoPaTo in [#1037](https://github.com/ratatui-org/ratatui/pull/1037)

- [1706b0a](https://github.com/ratatui-org/ratatui/commit/1706b0a3e434c51dfed9af88470f47162b615c33) *(crossterm)* Speed up combined fg and bg color changes by up to 20% by @joshka in [#1072](https://github.com/ratatui-org/ratatui/pull/1072)

- [1a4bb1c](https://github.com/ratatui-org/ratatui/commit/1a4bb1cbb8dc98ab3c9ecfce225a591b0f7a36bc) *(layout)* Avoid allocating memory when using split ergonomic utils by @tranzystorekk in [#1105](https://github.com/ratatui-org/ratatui/pull/1105)

  > Don't create intermediate vec in `Layout::areas` and
  > `Layout::spacers` when there's no need for one.

### Styling

- [aa4260f](https://github.com/ratatui-org/ratatui/commit/aa4260f92c869ed77123fab700f9f20b059bbe07) *(uncategorized)* Use std::fmt instead of importing Debug and Display by @joshka in [#1087](https://github.com/ratatui-org/ratatui/pull/1087)

  > This is a small universal style change to avoid making this change a
  > part of other PRs.
  >
  > [rationale](https://github.com/ratatui-org/ratatui/pull/1083#discussion_r1588466060)

### Testing

- [3cc29bd](https://github.com/ratatui-org/ratatui/commit/3cc29bdada096283f1fa89d0a610fa6fd5425f9b) *(block)* Use rstest to simplify test cases by @EdJoPaTo in [#1095](https://github.com/ratatui-org/ratatui/pull/1095)

### Miscellaneous Tasks

- [5fbb77a](https://github.com/ratatui-org/ratatui/commit/5fbb77ad205ccff763d71899c2f5a34560d25b92) *(readme)* Use terminal theme for badges by @TadoTheMiner in [#1026](https://github.com/ratatui-org/ratatui/pull/1026)

  > The badges in the readme were all the default theme. Giving them
  > prettier colors that match the terminal gif is better. I've used the
  > colors from the VHS repo.

- [bef2bc1](https://github.com/ratatui-org/ratatui/commit/bef2bc1e7c012ecbf357ac54a5262304646b292d) *(cargo)* Add homepage to Cargo.toml by @joshka in [#1080](https://github.com/ratatui-org/ratatui/pull/1080)

- [76e5fe5](https://github.com/ratatui-org/ratatui/commit/76e5fe5a9a1934aa7cce8f0d48c1c9035ac0bf41) *(uncategorized)* Revert "Make Stylize's `.bg(color)` generic" by @kdheepak in [#1102](https://github.com/ratatui-org/ratatui/pull/1102)

  > This reverts commit ec763af8512df731799c8f30c38c37252068a4c4 from #1099

- [64eb391](https://github.com/ratatui-org/ratatui/commit/64eb3913a4776db290baeb4179e00d2686d42934) *(uncategorized)* Fixup cargo lint for windows targets by @joshka in [#1071](https://github.com/ratatui-org/ratatui/pull/1071)

  > Crossterm brings in multiple versions of the same dep

- [326a461](https://github.com/ratatui-org/ratatui/commit/326a461f9a345ba853d57afefc8d77ba0b0b5a14) *(uncategorized)* Add package categories field by @mcskware in [#1035](https://github.com/ratatui-org/ratatui/pull/1035)

  > Add the package categories field in Cargo.toml, with value
  > `["command-line-interface"]`. This fixes the (currently non-default)
  > clippy cargo group lint
  > [`clippy::cargo_common_metadata`](https://rust-lang.github.io/rust-clippy/master/index.html#/cargo_common_metadata).
  >
  > As per discussion in [Cargo package categories
  > suggestions](https://github.com/ratatui-org/ratatui/discussions/1034),
  > this lint is not suggested to be run by default in CI, but rather as an
  > occasional one-off as part of the larger
  > [`clippy::cargo`](https://doc.rust-lang.org/stable/clippy/lints.html#cargo)
  > lint group.

### Build

- [4955380](https://github.com/ratatui-org/ratatui/commit/4955380932ab4d657be15dd6c65f48334795c785) *(uncategorized)* Remove pre-push hooks by @joshka in [#1115](https://github.com/ratatui-org/ratatui/pull/1115)

- [28e81c0](https://github.com/ratatui-org/ratatui/commit/28e81c0714d55f0103d9f075609bcf7e5f551fb1) *(uncategorized)* Add underline-color to all features flag in makefile by @joshka in [#1100](https://github.com/ratatui-org/ratatui/pull/1100)

- [c75aa19](https://github.com/ratatui-org/ratatui/commit/c75aa1990f5c1e7e86de0fafc9ce0c1b1dcac3ea) *(uncategorized)* Add clippy::cargo lint by @joshka in [#1053](https://github.com/ratatui-org/ratatui/pull/1053)

  > Followup to https://github.com/ratatui-org/ratatui/pull/1035 and
  > https://github.com/ratatui-org/ratatui/discussions/1034
  >
  > It's reasonable to enable this and deal with breakage by fixing any
  > specific issues that arise.



### New Contributors
* @Utagai made their first contribution in [#1077](https://github.com/ratatui-org/ratatui/pull/1077)
* @kxxt made their first contribution in [#1074](https://github.com/ratatui-org/ratatui/pull/1074)
* @OkieOth made their first contribution in [#1069](https://github.com/ratatui-org/ratatui/pull/1069)
* @psobolik made their first contribution in [#1066](https://github.com/ratatui-org/ratatui/pull/1066)
* @SleepySwords made their first contribution in [#934](https://github.com/ratatui-org/ratatui/pull/934)
* @mcskware made their first contribution in [#1035](https://github.com/ratatui-org/ratatui/pull/1035)

**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.26.2...v0.26.3


## [v0.26.2](https://github.com/ratatui-org/ratatui/releases/tag/v0.26.2) - 2024-04-15

### Features

- [11b452d](https://github.com/ratatui-org/ratatui/commit/11b452d56fe590188ee7a53fa2dde95513b1a4c7) *(layout)* Mark various functions as const by @EdJoPaTo in [#951](https://github.com/ratatui-org/ratatui/pull/951)

- [1cff511](https://github.com/ratatui-org/ratatui/commit/1cff51193466f5a94d202b6233d56889eccf6d7b) *(line)* Impl Styled for Line by @joshka in [#968](https://github.com/ratatui-org/ratatui/pull/968)

  > This adds `FromIterator` impls for `Line` and `Text` that allow creating
  > `Line` and `Text` instances from iterators of `Span` and `Line`
  > instances, respectively.
  >
  > ```rust
  > let line = Line::from_iter(vec!["Hello".blue(), " world!".green()]);
  > let line: Line = iter::once("Hello".blue())
  >     .chain(iter::once(" world!".green()))
  >     .collect();
  > let text = Text::from_iter(vec!["The first line", "The second line"]);
  > let text: Text = iter::once("The first line")
  >     .chain(iter::once("The second line"))
  >     .collect();
  > ```

- [654949b](https://github.com/ratatui-org/ratatui/commit/654949bb00b4522130642f9ad50ab4d9095d921b) *(list)* Add Scroll Padding to Lists by @CameronBarnes in [#958](https://github.com/ratatui-org/ratatui/pull/958)

  > Introduces scroll padding, which allows the api user to request that a certain number of ListItems be kept visible above and below the currently selected item while scrolling.
  >
  > ```rust
  > let list = List::new(items).scroll_padding(1);
  > ```
  >
  > Fixes:https://github.com/ratatui-org/ratatui/pull/955

- [26af650](https://github.com/ratatui-org/ratatui/commit/26af65043ee9f165459dec228d12eaeed9997d92) *(text)* Add push methods for text and line by @joshka in [#998](https://github.com/ratatui-org/ratatui/pull/998)

  > Adds the following methods to the `Text` and `Line` structs:
  > - Text::push_line
  > - Text::push_span
  > - Line::push_span
  >
  > This allows for adding lines and spans to a text object without having
  > to call methods on the fields directly, which is usefult for incremental
  > construction of text objects.

- [b5bdde0](https://github.com/ratatui-org/ratatui/commit/b5bdde079e0e1eda98b9b1bbbba011b770e5b167) *(text)* Add `FromIterator` impls for `Line` and `Text` by @joshka in [#967](https://github.com/ratatui-org/ratatui/pull/967)

  > This adds `FromIterator` impls for `Line` and `Text` that allow creating
  > `Line` and `Text` instances from iterators of `Span` and `Line`
  > instances, respectively.
  >
  > ```rust
  > let line = Line::from_iter(vec!["Hello".blue(), " world!".green()]);
  > let line: Line = iter::once("Hello".blue())
  >     .chain(iter::once(" world!".green()))
  >     .collect();
  > let text = Text::from_iter(vec!["The first line", "The second line"]);
  > let text: Text = iter::once("The first line")
  >     .chain(iter::once("The second line"))
  >     .collect();
  > ```

- [12f67e8](https://github.com/ratatui-org/ratatui/commit/12f67e810fad0f907546408192a2380b590ff7bd) *(uncategorized)* Impl Widget for `&str` and `String` by @kdheepak in [#952](https://github.com/ratatui-org/ratatui/pull/952)

  > Currently, `f.render_widget("hello world".bold(), area)` works but
  > `f.render_widget("hello world", area)` doesn't. This PR changes that my
  > implementing `Widget` for `&str` and `String`. This makes it easier to
  > render strings with no styles as widgets.
  >
  > Example usage:
  >
  > ```rust
  > terminal.draw(|f| f.render_widget("Hello World!", f.size()))?;
  > ```
  >
  > ---------

### Bug Fixes

- [0207160](https://github.com/ratatui-org/ratatui/commit/02071607848c51250b4663722c52e19c8ce1c5e2) *(line)* Line truncation respects alignment by @TadoTheMiner in [#987](https://github.com/ratatui-org/ratatui/pull/987)

  > When rendering a `Line`, the line will be truncated:
  > - on the right for left aligned lines
  > - on the left for right aligned lines
  > - on bot sides for centered lines
  >
  > E.g. "Hello World" will be rendered as "Hello", "World", "lo wo" for
  > left, right, centered lines respectively.
  >
  > Fixes:https://github.com/ratatui-org/ratatui/issues/932

- [c56f49b](https://github.com/ratatui-org/ratatui/commit/c56f49b9fb1c7f1c8c97749119e85f81882ca9a9) *(list)* Saturating_sub to fix highlight_symbol overflow by @mrjackwills in [#949](https://github.com/ratatui-org/ratatui/pull/949)

  > An overflow (pedantically an underflow) can occur if the
  > highlight_symbol is a multi-byte char, and area is reduced to a size
  > less than that char length.

- [b7778e5](https://github.com/ratatui-org/ratatui/commit/b7778e5cd15d0d4b28f7bbb8b3c62950748e333a) *(paragraph)* Unit test typo by @joshka in [#1022](https://github.com/ratatui-org/ratatui/pull/1022)

- [943c043](https://github.com/ratatui-org/ratatui/commit/943c0431d968a82b23a2f31527f32e57f86f8a7c) *(scrollbar)* Dont render on 0 length track by @EdJoPaTo in [#964](https://github.com/ratatui-org/ratatui/pull/964)

  > Fixes a panic when `track_length - 1` is used. (clamp panics on `-1.0`
  > being smaller than `0.0`)

- [742a5ea](https://github.com/ratatui-org/ratatui/commit/742a5ead066bec14047f6ab7ffa3ac8307eea715) *(text)* Fix panic when rendering out of bounds by @joshka in [#997](https://github.com/ratatui-org/ratatui/pull/997)

  > Previously it was possible to cause a panic when rendering to an area
  > outside of the buffer bounds. Instead this now correctly renders nothing
  > to the buffer.

- [f6c4e44](https://github.com/ratatui-org/ratatui/commit/f6c4e447e65fe10f4fc7fcc9e9c4312acad41096) *(uncategorized)* Ensure that paragraph correctly renders styled text by @joshka in [#992](https://github.com/ratatui-org/ratatui/pull/992)

  > Paragraph was ignoring the new `Text::style` field added in 0.26.0
  >
  > Fixes:https://github.com/ratatui-org/ratatui/issues/990

- [35e971f](https://github.com/ratatui-org/ratatui/commit/35e971f7ebb0deadc613b561b15511abd48bdb54) *(uncategorized)* Scrollbar thumb not visible on long lists by @ThomasMiz in [#959](https://github.com/ratatui-org/ratatui/pull/959)

  > When displaying somewhat-long lists, the `Scrollbar` widget sometimes did not display a thumb character, and only the track will be visible.

### Refactor

- [6fd5f63](https://github.com/ratatui-org/ratatui/commit/6fd5f631bbd58156d9fcae196040bb0248097819) *(lint)* Prefer idiomatic for loops by @EdJoPaTo

- [37b957c](https://github.com/ratatui-org/ratatui/commit/37b957c7e167a7ecda07b8a60cee5de71efcc55e) *(lints)* Add lints to scrollbar by @EdJoPaTo

- [c12bcfe](https://github.com/ratatui-org/ratatui/commit/c12bcfefa26529610886040bd96f2b6762436b15) *(non-src)* Apply pedantic lints by @EdJoPaTo in [#976](https://github.com/ratatui-org/ratatui/pull/976)

  > Fixes many not yet enabled lints (mostly pedantic) on everything that is
  > not the lib (examples, benchs, tests). Therefore, this is not containing
  > anything that can be a breaking change.
  >
  > Lints are not enabled as that should be the job of #974. I created this
  > as a separate PR as its mostly independent and would only clutter up the
  > diff of #974 even more.
  >
  > Also see
  > https://github.com/ratatui-org/ratatui/pull/974#discussion_r1506458743
  >
  > ---------

- [8719608](https://github.com/ratatui-org/ratatui/commit/8719608bdaf32ba92bdfdd60569cf73f7070a618) *(span)* Rename to_aligned_line into into_aligned_line by @EdJoPaTo in [#993](https://github.com/ratatui-org/ratatui/pull/993)

  > With the Rust method naming conventions these methods are into methods
  > consuming the Span. Therefore, it's more consistent to use `into_`
  > instead of `to_`.
  >
  > ```rust
  > Span::to_centered_line
  > Span::to_left_aligned_line
  > Span::to_right_aligned_line
  > ```
  >
  > Are marked deprecated and replaced with the following
  >
  > ```rust
  > Span::into_centered_line
  > Span::into_left_aligned_line
  > Span::into_right_aligned_line
  > ```

- [b831c56](https://github.com/ratatui-org/ratatui/commit/b831c5688c6f1fbfa6ae2bcd70d803a54fcf0196) *(widget-ref)* Clippy::needless_pass_by_value by @EdJoPaTo

- [359204c](https://github.com/ratatui-org/ratatui/commit/359204c9298cc26ea21807d886d596de0329bacc) *(uncategorized)* Simplify to io::Result by @EdJoPaTo in [#1016](https://github.com/ratatui-org/ratatui/pull/1016)

  > Simplifies the code, logic stays exactly the same.

- [8e68db9](https://github.com/ratatui-org/ratatui/commit/8e68db9e2f57fcbf7cb5140006bbbd4dd80bf907) *(uncategorized)* Remove pointless default on internal structs by @EdJoPaTo in [#980](https://github.com/ratatui-org/ratatui/pull/980)
  >
  > See #978
  >
  > Also remove other derives. They are unused and just slow down
  > compilation.

- [3be189e](https://github.com/ratatui-org/ratatui/commit/3be189e3c6ebd418d13138ff32bc4a749dc840cf) *(uncategorized)* Clippy::thread_local_initializer_can_be_made_const by @EdJoPaTo

  > enabled by default on nightly

- [5c4efac](https://github.com/ratatui-org/ratatui/commit/5c4efacd1d70bb295d90ffaa73853dc206c187fb) *(uncategorized)* Clippy::map_err_ignore by @EdJoPaTo

- [bbb6d65](https://github.com/ratatui-org/ratatui/commit/bbb6d65e063df9a74ab6487b2216183c1fdd7230) *(uncategorized)* Clippy::else_if_without_else by @EdJoPaTo

- [fdb14dc](https://github.com/ratatui-org/ratatui/commit/fdb14dc7cd69788e2ed20709e767f7631b11ffa2) *(uncategorized)* Clippy::redundant_type_annotations by @EdJoPaTo

- [9b3b23a](https://github.com/ratatui-org/ratatui/commit/9b3b23ac14518a1ef23065d4a5da0fb047b18213) *(uncategorized)* Remove literal suffix by @EdJoPaTo

  > its not needed and can just be assumed
  >
  > related:clippy::(un)separated_literal_suffix

- [58b6e0b](https://github.com/ratatui-org/ratatui/commit/58b6e0be0f4db3d90005e130e4b84cd865179785) *(uncategorized)* Clippy::should_panic_without_expect by @EdJoPaTo

- [c870a41](https://github.com/ratatui-org/ratatui/commit/c870a41057ac0c14c2e72e762b37689dc32e7b23) *(uncategorized)* Clippy::many_single_char_names by @EdJoPaTo

- [a6036ad](https://github.com/ratatui-org/ratatui/commit/a6036ad78911653407f607f5efa556a055d3dce9) *(uncategorized)* Clippy::similar_names by @EdJoPaTo

- [060d26b](https://github.com/ratatui-org/ratatui/commit/060d26b6dc6e1027dbf46ae98b0ebba83701f941) *(uncategorized)* Clippy::match_same_arms by @EdJoPaTo

- [fcbea9e](https://github.com/ratatui-org/ratatui/commit/fcbea9ee68591344a29a7b2e83f1c8c878857aeb) *(uncategorized)* Clippy::uninlined_format_args by @EdJoPaTo

- [14b24e7](https://github.com/ratatui-org/ratatui/commit/14b24e75858af48f39d5880e7f6c9adeac1b1da9) *(uncategorized)* Clippy::if_not_else by @EdJoPaTo

- [5ed1f43](https://github.com/ratatui-org/ratatui/commit/5ed1f43c627053f25d9ee711677ebec6cb8fcd85) *(uncategorized)* Clippy::redundant_closure_for_method_calls by @EdJoPaTo

- [c8c7924](https://github.com/ratatui-org/ratatui/commit/c8c7924e0ca84351f5ed5c54e79611ce16d4dc37) *(uncategorized)* Clippy::too_many_lines by @EdJoPaTo

- [e3afe7c](https://github.com/ratatui-org/ratatui/commit/e3afe7c8a14c1cffd7de50782a7acf0f95f41673) *(uncategorized)* Clippy::unreadable_literal by @EdJoPaTo

- [a1f54de](https://github.com/ratatui-org/ratatui/commit/a1f54de7d60fa6c57be29bf8f02a675e58b7b9c2) *(uncategorized)* Clippy::bool_to_int_with_if by @EdJoPaTo

- [b8ea190](https://github.com/ratatui-org/ratatui/commit/b8ea190bf2cde8c18e2ac8276d2eb57d219db263) *(uncategorized)* Clippy::cast_lossless by @EdJoPaTo

- [0de5238](https://github.com/ratatui-org/ratatui/commit/0de5238ed3613f2d663f5e9628ca7b2aa205ed02) *(uncategorized)* Dead_code by @EdJoPaTo

  > enabled by default, only detected by nightly yet

- [df5dddf](https://github.com/ratatui-org/ratatui/commit/df5dddfbc9c679d15a5a90ea79bb1f8946d5cb9c) *(uncategorized)* Unused_imports by @EdJoPaTo

  > enabled by default, only detected on nightly yet

- [f1398ae](https://github.com/ratatui-org/ratatui/commit/f1398ae6cb1abd32106923d64844b482c7ba6f82) *(uncategorized)* Clippy::useless_vec by @EdJoPaTo

  > Lint enabled by default but only nightly finds this yet

- [525848f](https://github.com/ratatui-org/ratatui/commit/525848ff4e066526d402fecf1d5b9c63cff1f22a) *(uncategorized)* Manually apply clippy::use_self for impl with lifetimes by @EdJoPaTo

- [660c718](https://github.com/ratatui-org/ratatui/commit/660c7183c7a10dc453d80dfb651d9534536960b9) *(uncategorized)* Clippy::empty_line_after_doc_comments by @EdJoPaTo

- [ab951fa](https://github.com/ratatui-org/ratatui/commit/ab951fae8166c9321728ba942b48552dfe4d9c55) *(uncategorized)* Clippy::return_self_not_must_use by @EdJoPaTo

- [3cd4369](https://github.com/ratatui-org/ratatui/commit/3cd436917649a93b4b80d0c4a0343284e0585522) *(uncategorized)* Clippy::doc_markdown by @EdJoPaTo

- [9bc014d](https://github.com/ratatui-org/ratatui/commit/9bc014d7f16efdb70fcd6b6b786fe74eac7b9bdf) *(uncategorized)* Clippy::items_after_statements by @EdJoPaTo

- [36a0cd5](https://github.com/ratatui-org/ratatui/commit/36a0cd56e5645533a1d6c2720536fa10a56b0d40) *(uncategorized)* Clippy::deref_by_slicing by @EdJoPaTo

- [f7f6692](https://github.com/ratatui-org/ratatui/commit/f7f66928a8833532a3bc97292665640285e7aafa) *(uncategorized)* Clippy::equatable_if_let by @EdJoPaTo

- [01418eb](https://github.com/ratatui-org/ratatui/commit/01418eb7c2e1874cb4070828c485d81ea171b18d) *(uncategorized)* Clippy::default_trait_access by @EdJoPaTo

- [8536760](https://github.com/ratatui-org/ratatui/commit/8536760e7802a498f7c6d9fe8fb4c7920a1c6e71) *(uncategorized)* Clippy::inefficient_to_string by @EdJoPaTo

- [a558b19](https://github.com/ratatui-org/ratatui/commit/a558b19c9a7b90a1ed3f309301f49f0b483e02ec) *(uncategorized)* Clippy::implicit_clone by @EdJoPaTo

- [5b00e3a](https://github.com/ratatui-org/ratatui/commit/5b00e3aae98cb5c20c10bec944948a75ac83f956) *(uncategorized)* Clippy::use_self by @EdJoPaTo

- [27680c0](https://github.com/ratatui-org/ratatui/commit/27680c05ce1670f026ad23c446ada321c1c755f0) *(uncategorized)* Clippy::semicolon_if_nothing_returned by @EdJoPaTo

### Documentation

- [14461c3](https://github.com/ratatui-org/ratatui/commit/14461c3a3554c95905ebca433fc3d4dae1e1acda) *(breaking-changes)* Typos and markdownlint by @EdJoPaTo in [#1009](https://github.com/ratatui-org/ratatui/pull/1009)

- [d0067c8](https://github.com/ratatui-org/ratatui/commit/d0067c8815d5244d319934d58a9366c8ad36b3e5) *(license)* Update copyright years by @orhun in [#962](https://github.com/ratatui-org/ratatui/pull/962)

- [88bfb5a](https://github.com/ratatui-org/ratatui/commit/88bfb5a43027cf3410ad560772c5bfdbaa3d58b7) *(text)* Update Text and Line docs by @joshka in [#969](https://github.com/ratatui-org/ratatui/pull/969)

- [3b002fd](https://github.com/ratatui-org/ratatui/commit/3b002fdcab964ce3f65f55dc8053d9678ae247a3) *(uncategorized)* Update incompatible code warning in examples readme by @joshka in [#1013](https://github.com/ratatui-org/ratatui/pull/1013)

### Performance

- [e02f476](https://github.com/ratatui-org/ratatui/commit/e02f4768ce2ee30473200fe98e2687e42acb9c33) *(borders)* Allow border!() in const by @EdJoPaTo in [#977](https://github.com/ratatui-org/ratatui/pull/977)

  > This allows more compiler optimizations when the macro is used.

- [541f0f9](https://github.com/ratatui-org/ratatui/commit/541f0f99538762a07d68a71b2989ecc6ff6f71ef) *(cell)* Use const CompactString::new_inline by @EdJoPaTo in [#979](https://github.com/ratatui-org/ratatui/pull/979)

  > Some minor find when messing around trying to `const` all the things.
  >
  > While `reset()` and `default()` can not be `const` it's still a benefit
  > when their contents are.

- [65e7923](https://github.com/ratatui-org/ratatui/commit/65e792375396c3160d76964ef0dfc4fb1e53be41) *(scrollbar)* Const creation by @EdJoPaTo in [#963](https://github.com/ratatui-org/ratatui/pull/963)

  > A bunch of `const fn` allow for more performance and `Default` now uses the `const` new implementations.

- [8195f52](https://github.com/ratatui-org/ratatui/commit/8195f526cb4b321f337dcbe9e689cc7f6eb84065) *(uncategorized)* Clippy::needless_pass_by_value by @EdJoPaTo

- [183c07e](https://github.com/ratatui-org/ratatui/commit/183c07ef436cbb8fb0bec418042b44b4fedd836f) *(uncategorized)* Clippy::trivially_copy_pass_by_ref by @EdJoPaTo

- [a13867f](https://github.com/ratatui-org/ratatui/commit/a13867ffceb2f8f57f4540049754c2f916fd3efc) *(uncategorized)* Clippy::cloned_instead_of_copied by @EdJoPaTo

- [3834374](https://github.com/ratatui-org/ratatui/commit/3834374652b46c5ddbfedcf8dea2086fd762f884) *(uncategorized)* Clippy::missing_const_for_fn by @EdJoPaTo

### Miscellaneous Tasks

- [125ee92](https://github.com/ratatui-org/ratatui/commit/125ee929ee9009b97a270e2e105a3f1167ab13d7) *(docs)* Fix: fix typos in crate documentation by @orhun in [#1002](https://github.com/ratatui-org/ratatui/pull/1002)

- [38c17e0](https://github.com/ratatui-org/ratatui/commit/38c17e091cf3f4de2d196ecdd6a40129019eafc4) *(editorconfig)* Set and apply some defaults by @EdJoPaTo

- [07da90a](https://github.com/ratatui-org/ratatui/commit/07da90a7182035b24f870bcbf0a0ffaad75eb48b) *(funding)* Add eth address for receiving funds from drips.network by @BenJam in [#994](https://github.com/ratatui-org/ratatui/pull/994)

- [078e97e](https://github.com/ratatui-org/ratatui/commit/078e97e4ff65c02afa7c884914ecd38a6e959b58) *(github)* Add EdJoPaTo as a maintainer by @orhun in [#986](https://github.com/ratatui-org/ratatui/pull/986)

- [b0314c5](https://github.com/ratatui-org/ratatui/commit/b0314c5731b32f51f5b6ca71a5194c6d7f265972) *(uncategorized)* Remove conventional commit check for PR by @Valentin271 in [#950](https://github.com/ratatui-org/ratatui/pull/950)

  > This removes conventional commit check for PRs.
  >
  > Since we use the PR title and description this is useless. It fails a
  > lot of time and we ignore it.
  >
  > IMPORTANT NOTE: This does **not** mean Ratatui abandons conventional
  > commits. This only relates to commits in PRs.

### Build

- [6e6ba27](https://github.com/ratatui-org/ratatui/commit/6e6ba27a122560bcf47b0efd20b7095f1bfd8714) *(lint)* Warn on pedantic and allow the rest by @EdJoPaTo

- [c4ce7e8](https://github.com/ratatui-org/ratatui/commit/c4ce7e8ff6f00875e1ead5b68052f0db737bd44d) *(uncategorized)* Enable more satisfied lints by @EdJoPaTo

  > These lints dont generate warnings and therefore dont need refactoring.
  > I think they are useful in the future.

- [a4e84a6](https://github.com/ratatui-org/ratatui/commit/a4e84a6a7f6f5b80903799028f30e2a4438f2807) *(uncategorized)* Increase msrv to 1.74.0 by @EdJoPaTo [**breaking**]

  > configure lints in Cargo.toml requires 1.74.0
  >
  > BREAKING CHANGE:rust 1.74 is required now



### New Contributors
* @BenJam made their first contribution in [#994](https://github.com/ratatui-org/ratatui/pull/994)
* @CameronBarnes made their first contribution in [#958](https://github.com/ratatui-org/ratatui/pull/958)
* @ThomasMiz made their first contribution in [#959](https://github.com/ratatui-org/ratatui/pull/959)

**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.26.1...v0.26.2


## [v0.26.1](https://github.com/ratatui-org/ratatui/releases/tag/v0.26.1) - 2024-02-12

### Features

- [74a0511](https://github.com/ratatui-org/ratatui/commit/74a051147a4059990c31e08d96a8469d8220537b) *(rect)* Add Rect::positions iterator by @joshka in [#928](https://github.com/ratatui-org/ratatui/pull/928)

  > Useful for performing some action on all the cells in a particular area.
  > E.g.,
  >
  > ```rust
  > fn render(area: Rect, buf: &mut Buffer) {
  >    for position in area.positions() {
  >         buf.get_mut(position.x, position.y).set_symbol("x");
  >     }
  > }
  > ```

- [9182f47](https://github.com/ratatui-org/ratatui/commit/9182f47026d1630cb749163b6f8b8987474312ae) *(uncategorized)* Add Block::title_top and Block::title_top_bottom by @joshka in [#940](https://github.com/ratatui-org/ratatui/pull/940)

  > This adds the ability to add titles to the top and bottom of a block
  > without having to use the `Title` struct (which will be removed in a
  > future release - likely v0.28.0).
  >
  > Fixes a subtle bug if the title was created from a right aligned Line
  > and was also right aligned. The title would be rendered one cell too far
  > to the right.
  >
  > ```rust
  > Block::bordered()
  >     .title_top(Line::raw("A").left_aligned())
  >     .title_top(Line::raw("B").centered())
  >     .title_top(Line::raw("C").right_aligned())
  >     .title_bottom(Line::raw("D").left_aligned())
  >     .title_bottom(Line::raw("E").centered())
  >     .title_bottom(Line::raw("F").right_aligned())
  >     .render(buffer.area, &mut buffer);
  > // renders
  > "┌A─────B─────C┐",
  > "│             │",
  > "└D─────E─────F┘",
  > ```
  >
  > Addresses part of https://github.com/ratatui-org/ratatui/issues/738
  >
  >

### Bug Fixes

- [2202059](https://github.com/ratatui-org/ratatui/commit/220205925911ed4377358d2a28ffca9373f11bda) *(block)* Fix crash on empty right aligned title by @joshka in [#933](https://github.com/ratatui-org/ratatui/pull/933)

  > - Simplified implementation of the rendering for block.
  > - Introduces a subtle rendering change where centered titles that are
  >   odd in length will now be rendered one character to the left compared
  >   to before. This aligns with other places that we render centered text
  >   and is a more consistent behavior. See
  >   https://github.com/ratatui-org/ratatui/pull/807#discussion_r1455645954
  >   for another example of this.
  >
  > Fixes:https://github.com/ratatui-org/ratatui/pull/929

- [14c67fb](https://github.com/ratatui-org/ratatui/commit/14c67fbb52101d10b2d2e26898c408ab8dd3ec2d) *(list)* Highlight symbol when using a  multi-bytes char by @mrjackwills in [#924](https://github.com/ratatui-org/ratatui/pull/924)

  > ratatui v0.26.0 brought a regression in the List widget, in which the
  > highlight symbol width was incorrectly calculated - specifically when
  > the highlight symbol was a multi-char character, e.g. `▶`.

- [0dcdbea](https://github.com/ratatui-org/ratatui/commit/0dcdbea083aace6d531c0d505837e0911f400675) *(paragraph)* Render Line::styled correctly inside a paragraph by @m4rch3n1ng in [#930](https://github.com/ratatui-org/ratatui/pull/930)

  > Renders the styled graphemes of the line instead of the contained spans.

- [fae5862](https://github.com/ratatui-org/ratatui/commit/fae5862c6e0947ee1488a7e4775413dbead67c8b) *(uncategorized)* Ensure that buffer::set_line sets the line style by @joshka in [#926](https://github.com/ratatui-org/ratatui/pull/926)

  > Fixes a regression in 0.26 where buffer::set_line was no longer setting
  > the style. This was due to the new style field on Line instead of being
  > stored only in the spans.
  >
  > Also adds a configuration for just running unit tests to bacon.toml.

- [fbb5dfa](https://github.com/ratatui-org/ratatui/commit/fbb5dfaaa903efde0e63114c393dc3063d5f56fd) *(uncategorized)* Scrollbar rendering when no track symbols are provided by @kdheepak in [#911](https://github.com/ratatui-org/ratatui/pull/911)

### Refactor

- [c3fb258](https://github.com/ratatui-org/ratatui/commit/c3fb25898f3e3ffe485ee69631b680679874d2cb) *(rect)* Move iters to module and add docs by @joshka in [#927](https://github.com/ratatui-org/ratatui/pull/927)

- [e51ca6e](https://github.com/ratatui-org/ratatui/commit/e51ca6e0d2705e6e0a96aeee78f1e80fcaaf34fc) *(uncategorized)* Finish tidying up table by @joshka in [#942](https://github.com/ratatui-org/ratatui/pull/942)

- [91040c0](https://github.com/ratatui-org/ratatui/commit/91040c0865043b8d5e7387509523a41345ed5af3) *(uncategorized)* Rearrange block structure by @joshka in [#939](https://github.com/ratatui-org/ratatui/pull/939)

### Documentation

- [61a8278](https://github.com/ratatui-org/ratatui/commit/61a827821dff2bd733377cfc143266edce1dbeec) *(canvas)* Add documentation to canvas module by @Valentin271 in [#913](https://github.com/ratatui-org/ratatui/pull/913)

  > Document the whole `canvas` module. With this, the whole `widgets`
  > module is documented.

- [d2d91f7](https://github.com/ratatui-org/ratatui/commit/d2d91f754c87458c6d07863eca20f3ea8ae319ce) *(changelog)* Add sponsors section by @orhun in [#908](https://github.com/ratatui-org/ratatui/pull/908)

  >

- [410d08b](https://github.com/ratatui-org/ratatui/commit/410d08b2b5812d7e29302adc0e8ddf18eb7d1d26) *(uncategorized)* Add link to FOSDEM 2024 talk by @orhun in [#944](https://github.com/ratatui-org/ratatui/pull/944)

- [1f208ff](https://github.com/ratatui-org/ratatui/commit/1f208ffd0368b4d269854dc0c550686dcd2d1de0) *(uncategorized)* Add GitHub Sponsors badge by @orhun in [#943](https://github.com/ratatui-org/ratatui/pull/943)

### Performance

- [0963463](https://github.com/ratatui-org/ratatui/commit/096346350e19c5de9a4d74bba64796997e9f40da) *(uncategorized)* Use drain instead of remove in chart examples by @mo8it in [#922](https://github.com/ratatui-org/ratatui/pull/922)

### Miscellaneous Tasks

- [a4892ad](https://github.com/ratatui-org/ratatui/commit/a4892ad444739d7a760bc45bbd954e728c66b2d2) *(uncategorized)* Fix typo in docsrs example by @orhun in [#946](https://github.com/ratatui-org/ratatui/pull/946)

- [18870ce](https://github.com/ratatui-org/ratatui/commit/18870ce99063a492674de061441b2cce5dc54c60) *(uncategorized)* Fix the method name for setting the Line style by @orhun in [#947](https://github.com/ratatui-org/ratatui/pull/947)

- [8fb4630](https://github.com/ratatui-org/ratatui/commit/8fb46301a00b5d065f9b890496f914d3fdc17495) *(uncategorized)* Remove github action bot that makes comments nudging commit signing by @kdheepak in [#937](https://github.com/ratatui-org/ratatui/pull/937)

  > We can consider reverting this commit once this PR is merged:
  > https://github.com/1Password/check-signed-commits-action/pull/9



### New Contributors
* @m4rch3n1ng made their first contribution in [#930](https://github.com/ratatui-org/ratatui/pull/930)
* @mo8it made their first contribution in [#922](https://github.com/ratatui-org/ratatui/pull/922)

**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.26.0...v0.26.1


## [v0.26.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.26.0) - 2024-02-02

### Features

- [79ceb9f](https://github.com/ratatui-org/ratatui/commit/79ceb9f7b6ce7d7079fd7a1e1de8b160086206d0) *(line)* Add alignment convenience functions by @Eeelco in [#856](https://github.com/ratatui-org/ratatui/pull/856)

  > This adds convenience functions `left_aligned()`, `centered()` and
  > `right_aligned()` plus unit tests. Updated example code.

- [0df9354](https://github.com/ratatui-org/ratatui/commit/0df935473f59d9bcf16ea5092878e59ee129d876) *(padding)* Add new constructors for padding by @Emivvvvv in [#828](https://github.com/ratatui-org/ratatui/pull/828)

  > Adds `proportional`, `symmetric`, `left`, `right`, `top`, and `bottom`
  > constructors for Padding struct.
  >
  > Proportional is
  > ```
  > /// **NOTE**: Terminal cells are often taller than they are wide, so to make horizontal and vertical
  > /// padding seem equal, doubling the horizontal padding is usually pretty good.
  > ```
  >
  > Fixes:https://github.com/ratatui-org/ratatui/issues/798

- [d726e92](https://github.com/ratatui-org/ratatui/commit/d726e928d2004d2a99caeeb00b95ce27dbc04bc0) *(paragraph)* Add alignment convenience functions by @Eeelco in [#866](https://github.com/ratatui-org/ratatui/pull/866)

  > Added convenience functions left_aligned(), centered() and
  > right_aligned() plus unit tests. Updated example code.

- [c1ed5c3](https://github.com/ratatui-org/ratatui/commit/c1ed5c3637dc4574612ac2029249ba700e9192b5) *(span)* Add alignment functions by @Eeelco in [#873](https://github.com/ratatui-org/ratatui/pull/873)

  > Implemented functions that convert Span into a
  > left-/center-/right-aligned Line. Implemented unit tests.
  >
  > Closes #853
  > ---------

- [b80264d](https://github.com/ratatui-org/ratatui/commit/b80264de877e7ca240cea15716379622d822bc08) *(text)* Add alignment convenience functions by @Eeelco in [#862](https://github.com/ratatui-org/ratatui/pull/862)

  > Adds convenience functions `left_aligned()`, `centered()` and
  > `right_aligned()` plus unit tests.

- [23f6938](https://github.com/ratatui-org/ratatui/commit/23f6938498a7c31916a091d5b79c9d95a0575344) *(block)* Add `Block::bordered` by @Valentin271 in [#736](https://github.com/ratatui-org/ratatui/pull/736)

  > This avoid creating a block with no borders and then settings Borders::ALL. i.e.
  >
  > ```diff
  > - Block::default().borders(Borders::ALL);
  > + Block::bordered();
  > ```

- [ffd5fc7](https://github.com/ratatui-org/ratatui/commit/ffd5fc79fcaf8bfff1a49c55f8d4b503a9e6dfed) *(color)* Add Color::from_u32 constructor by @joshka in [#785](https://github.com/ratatui-org/ratatui/pull/785)

  > Convert a u32 in the format 0x00RRGGBB to a Color.
  >
  > ```rust
  > let white = Color::from_u32(0x00FFFFFF);
  > let black = Color::from_u32(0x00000000);
  > ```

- [4f2db82](https://github.com/ratatui-org/ratatui/commit/4f2db82a774a3faea7db9659f30684e9635c24b2) *(color)* Use the FromStr implementation for deserialization by @orhun in [#705](https://github.com/ratatui-org/ratatui/pull/705)

  > The deserialize implementation for Color used to support only the enum
  > names (e.g. Color, LightRed, etc.) With this change, you can use any of
  > the strings supported by the FromStr implementation (e.g. black,
  > light-red, #00ff00, etc.)

- [1cbe1f5](https://github.com/ratatui-org/ratatui/commit/1cbe1f52abb7ab1cd5bd05030e7857ee1762f44a) *(constraints)* Rename `Constraint::Proportional` to `Constraint::Fill` by @kdheepak in [#880](https://github.com/ratatui-org/ratatui/pull/880)
  >
  > `Constraint::Fill` is a more intuitive name for the behavior, and it is
  > shorter.
  >
  > Resolves #859

- [dfd6db9](https://github.com/ratatui-org/ratatui/commit/dfd6db988faa7a45cbe99b01024c086c4fcf7577) *(demo2)* Add destroy mode to celebrate commit 1000! by @joshka in [#809](https://github.com/ratatui-org/ratatui/pull/809)

  > ```shell
  > cargo run --example demo2 --features="crossterm widget-calendar"
  > ```
  >
  > Press `d` to activate destroy mode and Enjoy!
  >
  > ![Destroy
  > Demo2](https://github.com/ratatui-org/ratatui/blob/1d39444e3dea6f309cf9035be2417ac711c1abc9/examples/demo2-destroy.gif?raw=true)
  >
  > Vendors a copy of tui-big-text to allow us to use it in the demo.

- [540fd2d](https://github.com/ratatui-org/ratatui/commit/540fd2df036648674a2f6d37f7b12326d5978bbd) *(layout)* Change `Flex::default()` by @kdheepak in [#881](https://github.com/ratatui-org/ratatui/pull/881) [**breaking**]

  > This PR makes a number of simplifications to the layout and constraint
  > features that were added after v0.25.0.
  >
  > For users upgrading from v0.25.0, the net effect of this PR (along with
  > the other PRs) is the following:
  >
  > - New `Flex` modes have been added.
  >   - `Flex::Start` (new default)
  >   - `Flex::Center`
  >   - `Flex::End`
  >   - `Flex::SpaceAround`
  >   - `Flex::SpaceBetween`
  >   - `Flex::Legacy` (old default)
  > - `Min(v)` grows to allocate excess space in all `Flex` modes instead of
  > shrinking (except in `Flex::Legacy` where it retains old behavior).
  > - `Fill(1)` grows to allocate excess space, growing equally with
  > `Min(v)`.
  >
  > ---
  >
  > The following contains a summary of the changes in this PR and the
  > motivation behind them.
  >
  > **`Flex`**
  >
  > - Removes `Flex::Stretch`
  > - Renames `Flex::StretchLast` to `Flex::Legacy`
  >
  > **`Constraint`**
  >
  > - Removes `Fixed`
  > - Makes `Min(v)` grow as much as possible everywhere (except
  > `Flex::Legacy` where it retains the old behavior)
  > - Makes `Min(v)` grow equally as `Fill(1)` while respecting `Min` lower
  > bounds. When `Fill` and `Min` are used together, they both fill excess
  > space equally.
  >
  > Allowing `Min(v)` to grow still allows users to build the same layouts
  > as before with `Flex::Start` with no breaking changes to the behavior.
  >
  > This PR also removes the unstable feature `SegmentSize`.
  >
  > This is a breaking change to the behavior of constraints. If users want
  > old behavior, they can use `Flex::Legacy`.
  >
  > ```rust
  > Layout::vertical([Length(25), Length(25)]).flex(Flex::Legacy)
  > ```
  >
  > Users that have constraint that exceed the available space will probably
  > not see any difference or see an improvement in their layouts. Any
  > layout with `Min` will be identical in `Flex::Start` and `Flex::Legacy`
  > so any layout with `Min` will not be breaking.
  >
  > Previously, `Table` used `EvenDistribution` internally by default, but
  > with that gone the default is now `Flex::Start`. This changes the
  > behavior of `Table` (for the better in most cases). The only way for
  > users to get exactly the same as the old behavior is to change their
  > constraints. I imagine most users will be happier out of the box with
  > the new Table default.
  >
  > Resolves https://github.com/ratatui-org/ratatui/issues/843
  >
  > Thanks to @joshka for the direction

- [bbcfa55](https://github.com/ratatui-org/ratatui/commit/bbcfa55a88c1916598ea0442217ac7f6a99ea96f) *(layout)* Add Rect::contains method by @joshka in [#882](https://github.com/ratatui-org/ratatui/pull/882)

  > This is useful for performing hit tests (i.e. did the user click in an
  > area).

- [736605e](https://github.com/ratatui-org/ratatui/commit/736605ec88aac4877b19dd66ded97b26d933407f) *(layout)* Add default impl for Position by @joshka in [#869](https://github.com/ratatui-org/ratatui/pull/869)

- [1e75596](https://github.com/ratatui-org/ratatui/commit/1e755967c53e9a1803cc7fcc46ad0946c78f0eda) *(layout)* Increase default cache size to 500 by @joshka in [#850](https://github.com/ratatui-org/ratatui/pull/850)

  > This is a somewhat arbitrary size for the layout cache based on adding
  > the columns and rows on my laptop's terminal (171+51 = 222) and doubling
  > it for good measure and then adding a bit more to make it a round
  > number. This gives enough entries to store a layout for every row and
  > every column, twice over, which should be enough for most apps. For
  > those that need more, the cache size can be set with
  > `Layout::init_cache()`.
  >
  > Fixes:https://github.com/ratatui-org/ratatui/issues/820

- [2819eea](https://github.com/ratatui-org/ratatui/commit/2819eea82bfde48562b830b4ef1c998dacae8b69) *(layout)* Add Position struct by @joshka in [#790](https://github.com/ratatui-org/ratatui/pull/790)

  > This stores the x and y coordinates (columns and rows)
  >
  > - add conversions from Rect
  > - add conversion with Size to Rect
  > - add Rect::as_position

- [1561d64](https://github.com/ratatui-org/ratatui/commit/1561d64c80e6498f90807a1607d84a1405d3e0bb) *(layout)* Add Rect -> Size conversion methods by @joshka in [#789](https://github.com/ratatui-org/ratatui/pull/789)

  > - add Size::new() constructor
  > - add Rect::as_size()
  > - impl From<Rect> for Size
  > - document and add tests for Size

- [f13fd73](https://github.com/ratatui-org/ratatui/commit/f13fd73d9ec108af723a9cd11f4262f2b09c9d25) *(layout)* Add `Rect::clamp()` method by @joshka in [#749](https://github.com/ratatui-org/ratatui/pull/749)

  > * feat(layout): add a Rect::clamp() method
  >
  > This ensures a rectangle does not end up outside an area. This is useful
  > when you want to be able to dynamically move a rectangle around, but
  > keep it constrained to a certain area.
  >
  > For example, this can be used to implement a draggable window that can
  > be moved around, but not outside the terminal window.
  >
  > ```rust
  > let window_area = Rect::new(state.x, state.y, 20, 20).clamp(area);
  > state.x = rect.x;
  > state.y = rect.y;
  > ```
  >
  > * refactor: use rstest to simplify clamp test
  >
  > * fix: use rstest description instead of string
  >
  > test layout::rect::tests::clamp::case_01_inside ... ok
  > test layout::rect::tests::clamp::case_02_up_left ... ok
  > test layout::rect::tests::clamp::case_04_up_right ... ok
  > test layout::rect::tests::clamp::case_05_left ... ok
  > test layout::rect::tests::clamp::case_03_up ... ok
  > test layout::rect::tests::clamp::case_06_right ... ok
  > test layout::rect::tests::clamp::case_07_down_left ... ok
  > test layout::rect::tests::clamp::case_08_down ... ok
  > test layout::rect::tests::clamp::case_09_down_right ... ok
  > test layout::rect::tests::clamp::case_10_too_wide ... ok
  > test layout::rect::tests::clamp::case_11_too_tall ... ok
  > test layout::rect::tests::clamp::case_12_too_large ... ok
  >
  > * fix: less ambiguous docs for this / other rect
  >
  > * fix: move rstest to dev deps

- [98bcf1c](https://github.com/ratatui-org/ratatui/commit/98bcf1c0a57a340229684345497b2d378979de04) *(layout)* Add Rect::split method by @joshka in [#729](https://github.com/ratatui-org/ratatui/pull/729)

  > This method splits a Rect and returns a fixed-size array of the
  > resulting Rects. This allows the caller to use array destructuring
  > to get the individual Rects.
  >
  > ```rust
  > use Constraint::*;
  > let layout = &Layout::vertical([Length(1), Min(0)]);
  > let [top, main] = area.split(&layout);
  > ```

- [0494ee5](https://github.com/ratatui-org/ratatui/commit/0494ee52f1f0070f1ccf4532f7301fd59d4a5c10) *(layout)* Accept Into<Constraint> for constructors by @joshka in [#744](https://github.com/ratatui-org/ratatui/pull/744)

  > This allows Layout constructors to accept any type that implements
  > Into<Constraint> instead of just AsRef<Constraint>. This is useful when
  > you want to specify a fixed size for a layout, but don't want to
  > explicitly create a Constraint::Length yourself.
  >
  > ```rust
  > Layout::new(Direction::Vertical, [1, 2, 3]);
  > Layout::horizontal([1, 2, 3]);
  > Layout::vertical([1, 2, 3]);
  > Layout::default().constraints([1, 2, 3]);
  > ```

- [7ab12ed](https://github.com/ratatui-org/ratatui/commit/7ab12ed8ce8f6cdb0712d132b4dfc4cccfda08da) *(layout)* Add horizontal and vertical constructors by @joshka in [#728](https://github.com/ratatui-org/ratatui/pull/728)

  > * feat(layout): add vertical and horizontal constructors
  >
  > This commit adds two new constructors to the `Layout` struct, which
  > allow the user to create a vertical or horizontal layout with default
  > values.
  >
  > ```rust
  > let layout = Layout::vertical([
  >     Constraint::Length(10),
  >     Constraint::Min(5),
  >     Constraint::Length(10),
  > ]);
  >
  > let layout = Layout::horizontal([
  >     Constraint::Length(10),
  >     Constraint::Min(5),
  >     Constraint::Length(10),
  > ]);
  > ```

- [4278b40](https://github.com/ratatui-org/ratatui/commit/4278b4088d2ab1d94aa5d73d7a0c321a46dbd9de) *(line)* Implement iterators for Line by @joshka in [#896](https://github.com/ratatui-org/ratatui/pull/896)

  > This allows iterating over the `Span`s of a line using `for` loops and
  > other iterator methods.
  >
  > - add `iter` and `iter_mut` methods to `Line`
  > - implement `IntoIterator` for `Line`, `&Line`, and `&mut Line` traits
  > - update call sites to iterate over `Line` rather than `Line::spans`

- [5d410c6](https://github.com/ratatui-org/ratatui/commit/5d410c6895de49e77c7e0d1884be63d797724448) *(line)* Implement Widget for Line by @joshka in [#715](https://github.com/ratatui-org/ratatui/pull/715)

  > This allows us to use Line as a child of other widgets, and to use
  > Line::render() to render it rather than calling buffer.set_line().
  >
  > ```rust
  > frame.render_widget(Line::raw("Hello, world!"), area);
  > // or
  > Line::raw("Hello, world!").render(frame, area);
  > ```

- [c977293](https://github.com/ratatui-org/ratatui/commit/c977293f14b019ee520379bf5eaafb44cef04a01) *(line)* Add style field, setters and docs by @joshka in [#708](https://github.com/ratatui-org/ratatui/pull/708) [**breaking**]

  > - The `Line` struct now stores the style of the line rather than each
  >   `Span` storing it.
  > - Adds two new setters for style and spans
  > - Adds missing docs
  >
  > BREAKING CHANGE:`Line::style` is now a field of `Line` instead of being
  > stored in each `Span`.

- [bbf2f90](https://github.com/ratatui-org/ratatui/commit/bbf2f906fbe7e593fdeb5dd7530d3479788f77a5) *(rect.rs)* Implement Rows and Columns iterators in Rect by @BogdanPaul15 in [#765](https://github.com/ratatui-org/ratatui/pull/765)

  > This enables iterating over rows and columns of a Rect. In tern being able to use that with other iterators and simplify looping over cells.

- [fe06f0c](https://github.com/ratatui-org/ratatui/commit/fe06f0c7b06e50cd5d7916dab9ccb5e28f5a6511) *(serde)* Support TableState, ListState, and ScrollbarState by @MultisampledNight in [#723](https://github.com/ratatui-org/ratatui/pull/723)

  > TableState, ListState, and ScrollbarState can now be serialized and deserialized
  > using serde.
  >
  > ```rust
  > #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
  > struct AppState {
  >     list_state: ListState,
  >     table_state: TableState,
  >     scrollbar_state: ScrollbarState,
  > }
  >
  > let app_state = AppState::default();
  > let serialized = serde_json::to_string(app_state);
  >
  > let app_state = serde_json::from_str(serialized);
  > ```

- [37c1836](https://github.com/ratatui-org/ratatui/commit/37c183636b573e7637af5fbab9ae5c6f2d3fec6b) *(span)* Implement Widget on Span by @joshka in [#709](https://github.com/ratatui-org/ratatui/pull/709)

  > This allows us to use Span as a child of other widgets, and to use
  > Span::render() to render it rather than calling buffer.set_span().
  >
  > ```rust
  > frame.render_widget(Span::raw("Hello, world!"), area);
  > // or
  > Span::raw("Hello, world!").render(frame, area);
  > // or even
  > "Hello, world!".green().render(frame, area);
  > ```

- [e1e85aa](https://github.com/ratatui-org/ratatui/commit/e1e85aa7af2a7624b12a0ad7f0aa2413b409475d) *(style)* Add material design color palette by @joshka in [#786](https://github.com/ratatui-org/ratatui/pull/786)

  > The `ratatui::style::palette::material` module contains the Google 2014
  > Material Design palette.
  >
  > See https://m2.material.io/design/color/the-color-system.html#tools-for-picking-colors
  > for more information.
  >
  > ```rust
  > use ratatui::style::palette::material::BLUE_GRAY;
  > Line::styled("Hello", BLUE_GRAY.c500);
  > ```

- [bf67850](https://github.com/ratatui-org/ratatui/commit/bf678507395a528befcf5c5e3180368cb8f4b826) *(style)* Add tailwind color palette by @joshka in [#787](https://github.com/ratatui-org/ratatui/pull/787)

  > The `ratatui::style::palette::tailwind` module contains the default
  > Tailwind color palette. This is useful for styling components with
  > colors that match the Tailwind color palette.
  >
  > See https://tailwindcss.com/docs/customizing-colors for more information
  > on Tailwind.
  >
  > ```rust
  > use ratatui::style::palette::tailwind::SLATE;
  > Line::styled("Hello", SLATE.c500);
  > ```

- [27e9216](https://github.com/ratatui-org/ratatui/commit/27e9216cea7f25fcf172fe0a8f11e7cca222b055) *(table)* Remove allow deprecated attribute used previously for segment_size ✨ by @kdheepak in [#875](https://github.com/ratatui-org/ratatui/pull/875)

- [a489d85](https://github.com/ratatui-org/ratatui/commit/a489d85f2dda561ea18f1431f6e44f0335549eca) *(table)* Deprecate SegmentSize on table by @Emivvvvv in [#842](https://github.com/ratatui-org/ratatui/pull/842)

  > This adds for table:
  >
  > - Added new flex method with flex field
  > - Deprecated segment_size method and removed segment_size field
  > - Updated documentation
  > - Updated tests

- [c69ca47](https://github.com/ratatui-org/ratatui/commit/c69ca47922619332f76488f5d9e70541b496fe1c) *(table)* Collect iterator of `Row` into `Table` by @Lunderberg in [#774](https://github.com/ratatui-org/ratatui/pull/774) [**breaking**]

  > Any iterator whose item is convertible into `Row` can now be
  > collected into a `Table`.
  >
  > Where previously, `Table::new` accepted `IntoIterator<Item = Row>`, it
  > now accepts `IntoIterator<Item: Into<Row>>`.
  >
  > BREAKING CHANGE:The compiler can no longer infer the element type of the container
  > passed to `Table::new()`.  For example, `Table::new(vec![], widths)`
  > will no longer compile, as the type of `vec![]` can no longer be
  > inferred.

- [2faa879](https://github.com/ratatui-org/ratatui/commit/2faa879658a439d233edc4ac886fb42c17ff971a) *(table)* Accept Text for highlight_symbol by @joshka in [#781](https://github.com/ratatui-org/ratatui/pull/781)

  > This allows for multi-line symbols to be used as the highlight symbol.
  >
  > ```rust
  > let table = Table::new(rows, widths)
  >     .highlight_symbol(Text::from(vec![
  >         "".into(),
  >         " █ ".into(),
  >         " █ ".into(),
  >         "".into(),
  >     ]));
  > ```

- [e64e194](https://github.com/ratatui-org/ratatui/commit/e64e194b6bc5f89c68fe73d430e63c264af6ca4f) *(table)* Implement FromIterator for widgets::Row by @Lunderberg in [#755](https://github.com/ratatui-org/ratatui/pull/755)

  > The `Row::new` constructor accepts a single argument that implements
  > `IntoIterator`.  This commit adds an implementation of `FromIterator`,
  > as a thin wrapper around `Row::new`.  This allows `.collect::<Row>()`
  > to be used at the end of an iterator chain, rather than wrapping the
  > entire iterator chain in `Row::new`.

- [803a72d](https://github.com/ratatui-org/ratatui/commit/803a72df27190e273556e089e42036bfc001f003) *(table)* Accept Into<Constraint> for widths by @joshka in [#745](https://github.com/ratatui-org/ratatui/pull/745)

  > This allows Table constructors to accept any type that implements
  > Into<Constraint> instead of just AsRef<Constraint>. This is useful when
  > you want to specify a fixed size for a table columns, but don't want to
  > explicitly create a Constraint::Length yourself.
  >
  > ```rust
  > Table::new(rows, [1,2,3])
  > Table::default().widths([1,2,3])
  > ```

- [f025d2b](https://github.com/ratatui-org/ratatui/commit/f025d2bfa26eac11ef5c2a63943a4e177abfc800) *(table)* Add Table::footer and Row::top_margin methods by @yanganto in [#722](https://github.com/ratatui-org/ratatui/pull/722)

  > * feat(table): Add a Table::footer method

- [f29c73f](https://github.com/ratatui-org/ratatui/commit/f29c73fb1cf746aea0adfaed4a8b959e0466b830) *(tabs)* Accept Iterators of `Line` in constructors by @Lunderberg in [#776](https://github.com/ratatui-org/ratatui/pull/776) [**breaking**]

  > Any iterator whose item is convertible into `Line` can now be
  > collected into `Tabs`.
  >
  > In addition, where previously `Tabs::new` required a `Vec`, it can now
  > accept any object that implements `IntoIterator` with an item type
  > implementing `Into<Line>`.
  >
  > BREAKING CHANGE:Calls to `Tabs::new()` whose argument is collected from an iterator
  > will no longer compile.  For example,
  >
  > `Tabs::new(["a","b"].into_iter().collect())` will no longer compile,
  > because the return type of `.collect()` can no longer be inferred to
  > be a `Vec<_>`.

- [b459228](https://github.com/ratatui-org/ratatui/commit/b459228e26b9429b8a09084d76251361f7f5bfd3) *(termwiz)* Add `From` termwiz style impls by @Valentin271 in [#726](https://github.com/ratatui-org/ratatui/pull/726)

  > Important note: this also fixes a wrong mapping between ratatui's gray
  > and termwiz's grey. `ratatui::Color::Gray` now maps to
  > `termwiz::color::AnsiColor::Silver`

- [9ba7354](https://github.com/ratatui-org/ratatui/commit/9ba7354335a106607fe0670e1205a038ec54aa1b) *(text)* Implement iterators for Text by @joshka in [#900](https://github.com/ratatui-org/ratatui/pull/900)

  > This allows iterating over the `Lines`s of a text using `for` loops and
  > other iterator methods.
  >
  > - add `iter` and `iter_mut` methods to `Text`
  > - implement `IntoIterator` for `Text`, `&Text`, and `&mut Text` traits
  > - update call sites to iterate over `Text` rather than `Text::lines`

- [68d5783](https://github.com/ratatui-org/ratatui/commit/68d5783a6912c644b922b7030facff4b1172a434) *(text)* Add style and alignment by @Valentin271 in [#807](https://github.com/ratatui-org/ratatui/pull/807)
  >
  > Fixes #758, fixes #801
  >
  > This PR adds:
  >
  > - `style` and `alignment` to `Text`
  > - impl `Widget` for `Text`
  > - replace `Text` manual draw to call for Widget impl
  >
  > All places that use `Text` have been updated and support its new
  > features expect paragraph which still has a custom implementation.

- [815757f](https://github.com/ratatui-org/ratatui/commit/815757fcbbc147050f8ce9418a4e91fd871d011f) *(widgets)* Implement Widget for Widget refs by @joshka in [#833](https://github.com/ratatui-org/ratatui/pull/833)

  > Many widgets can be rendered without changing their state.
  >
  > This commit implements The `Widget` trait for references to
  > widgets and changes their implementations to be immutable.
  >
  > This allows us to render widgets without consuming them by passing a ref
  > to the widget when calling `Frame::render_widget()`.
  >
  > ```rust
  > // this might be stored in a struct
  > let paragraph = Paragraph::new("Hello world!");
  >
  > let [left, right] = area.split(&Layout::horizontal([20, 20]));
  > frame.render_widget(&paragraph, left);
  > frame.render_widget(&paragraph, right); // we can reuse the widget
  > ```
  >
  > Implemented for all widgets except BarChart (which has an implementation
  > that modifies the internal state and requires a rewrite to fix.
  >
  > Other widgets will be implemented in follow up commits.
  >
  > Fixes:https://github.com/ratatui-org/ratatui/discussions/164
  > Replaces PRs: https://github.com/ratatui-org/ratatui/pull/122 and
  >
  > https://github.com/ratatui-org/ratatui/pull/16
  >
  > Enables:https://github.com/ratatui-org/ratatui/issues/132
  > Validated as a viable working solution by:
  >
  > https://github.com/ratatui-org/ratatui/pull/836

- [eb79256](https://github.com/ratatui-org/ratatui/commit/eb79256ceea151130c6b80930b51098b9ad43f5b) *(widgets)* Collect iterator of `ListItem` into `List` by @Lunderberg in [#775](https://github.com/ratatui-org/ratatui/pull/775)

  > Any iterator whose item is convertible into `ListItem` can now be
  > collected into a `List`.
  >
  > ```rust
  > let list: List = (0..3).map(|i| format!("Item{i}")).collect();
  > ```

- [c8dd879](https://github.com/ratatui-org/ratatui/commit/c8dd87918d44fff6d4c3c78e1fc821a3275db1ae) *(uncategorized)* Add WidgetRef and StatefulWidgetRef traits by @joshka in [#903](https://github.com/ratatui-org/ratatui/pull/903)

  > The Widget trait consumes self, which makes it impossible to use in a
  > boxed context. Previously we implemented the Widget trait for &T, but
  > this was not enough to render a boxed widget. We now have a new trait
  > called `WidgetRef` that allows rendering a widget by reference. This
  > trait is useful when you want to store a reference to one or more
  > widgets and render them later. Additionaly this makes it possible to
  > render boxed widgets where the type is not known at compile time (e.g.
  > in a composite layout with multiple panes of different types).
  >
  > This change also adds a new trait called `StatefulWidgetRef` which is
  > the stateful equivalent of `WidgetRef`.
  >
  > Both new traits are gated behind the `unstable-widget-ref` feature flag
  > as we may change the exact name / approach a little on this based on
  > further discussion.
  >
  > Blanket implementation of `Widget` for `&W` where `W` implements
  > `WidgetRef` and `StatefulWidget` for `&W` where `W` implements
  > `StatefulWidgetRef` is provided. This allows you to render a widget by
  > reference and a stateful widget by reference.
  >
  > A blanket implementation of `WidgetRef` for `Option<W>` where `W`
  > implements `WidgetRef` is provided. This makes it easier to render
  > child widgets that are optional without the boilerplate of unwrapping
  > the option. Previously several widgets implemented this manually. This
  > commits expands the pattern to apply to all widgets.
  >
  > ```rust
  > struct Parent {
  >     child: Option<Child>,
  > }
  >
  > impl WidgetRef for Parent {
  >     fn render_ref(&self, area: Rect, buf: &mut Buffer) {
  >         self.child.render_ref(area, buf);
  >     }
  > }
  > ```
  >
  > ```rust
  > let widgets: Vec<Box<dyn WidgetRef>> = vec![Box::new(Greeting), Box::new(Farewell)];
  > for widget in widgets {
  >     widget.render_ref(buf.area, &mut buf);
  > }
  > assert_eq!(buf, Buffer::with_lines(["Hello        Goodbye"]));
  > ```

- [87bf1dd](https://github.com/ratatui-org/ratatui/commit/87bf1dd9dfb8bf2e6c08c488d4a38dac21e14304) *(uncategorized)* Replace Rect::split with Layout::areas and spacers by @joshka in [#904](https://github.com/ratatui-org/ratatui/pull/904)

  > In a recent commit we added Rec::split, but this feels more ergonomic as
  > Layout::areas. This also adds Layout::spacers to get the spacers between
  > the areas.

- [dab08b9](https://github.com/ratatui-org/ratatui/commit/dab08b99b6a2a4c8ced6f780af7a37a0f3c34f6b) *(uncategorized)* Show space constrained UIs conditionally by @kdheepak in [#895](https://github.com/ratatui-org/ratatui/pull/895)

  > With this PR the constraint explorer demo only shows space constrained
  > UIs instead:
  >
  > Smallest (15 row height):
  >
  > <img width="759" alt="image"
  > src="https://github.com/ratatui-org/ratatui/assets/1813121/37a4a027-6c6d-4feb-8104-d732aee298ac">
  >
  > Small (20 row height):
  >
  > <img width="759" alt="image"
  > src="https://github.com/ratatui-org/ratatui/assets/1813121/f76e025f-0061-4f09-9c91-2f7b00fcfb9e">
  >
  > Medium (30 row height):
  >
  > <img width="758" alt="image"
  > src="https://github.com/ratatui-org/ratatui/assets/1813121/81b070da-1bfb-40c5-9fbc-c1ab44ce422e">
  >
  > Full (40 row height):
  >
  > <img width="760" alt="image"
  > src="https://github.com/ratatui-org/ratatui/assets/1813121/7bb8a8c4-1a77-4bbc-a346-c8b5c198c6d3">

- [2a12f7b](https://github.com/ratatui-org/ratatui/commit/2a12f7bddf0b286e63439c2d1fa894dcfbfde6c0) *(uncategorized)* Impl Widget for &BarChart by @joshka in [#897](https://github.com/ratatui-org/ratatui/pull/897)

  > BarChart had some internal mutations that needed to be removed to
  > implement the Widget trait for &BarChart to bring it in line with the
  > other widgets.

- [9ec43ef](https://github.com/ratatui-org/ratatui/commit/9ec43eff1c7a62631fab99e4874ccd15fe7b210a) *(uncategorized)* Constraint Explorer example by @kdheepak in [#893](https://github.com/ratatui-org/ratatui/pull/893)

  > Here's a constraint explorer demo put together with @joshka
  >
  > https://github.com/ratatui-org/ratatui/assets/1813121/08d7d8f6-d013-44b4-8331-f4eee3589cce
  >
  > It allows users to interactive explore how the constraints behave with
  > respect to each other and compare that across flex modes. It allows
  > users to swap constraints out for other constraints, increment or
  > decrement the values, add and remove constraints, and add spacing
  >
  > It is also a good example for how to structure a simple TUI with several
  > Ratatui code patterns that are useful for refactoring.
  >
  > Fixes:https://github.com/ratatui-org/ratatui/issues/792
  >
  > ---------

- [4ee4e6d](https://github.com/ratatui-org/ratatui/commit/4ee4e6d78a136b5a1e4942f25b9afe34f7dd5d0c) *(uncategorized)* Make spacing work in `Flex::SpaceAround` and `Flex::SpaceBetween` by @kdheepak in [#892](https://github.com/ratatui-org/ratatui/pull/892)

  > This PR implements user provided spacing gaps for `SpaceAround` and
  > `SpaceBetween`.
  >
  > https://github.com/ratatui-org/ratatui/assets/1813121/2e260708-e8a7-48ef-aec7-9cf84b655e91
  >
  > Now user provided spacing gaps always take priority in all `Flex` modes.

- [dd5ca3a](https://github.com/ratatui-org/ratatui/commit/dd5ca3a0c83bc1efc281133707eec04864567e69) *(uncategorized)* Better weights for constraints by @kdheepak in [#889](https://github.com/ratatui-org/ratatui/pull/889)

  > This PR is a split of reworking the weights from #888
  >
  > This keeps the same ranking of weights, just uses a different numerical
  > value so that the lowest weight is `WEAK` (`1.0`).
  >
  > No tests are changed as a result of this change, and running the
  > following multiple times did not cause any errors for me:
  >
  > ```rust
  > for i in {0..100}
  > do
  >  cargo test --lib --
  >  if [ $? -ne 0 ]; then
  >  echo "Test failed. Exiting loop."
  >  break
  >  fi
  > done
  > ```

- [aeec163](https://github.com/ratatui-org/ratatui/commit/aeec16369bdf26dc96af46cc580df191078464ae) *(uncategorized)* Change rounding to make tests stable by @kdheepak in [#888](https://github.com/ratatui-org/ratatui/pull/888)

  > This fixes some unstable tests

- [be4fdaa](https://github.com/ratatui-org/ratatui/commit/be4fdaa0c7c863daa50c0109cd5f96005365029d) *(uncategorized)* Change priority of constraints and add `split_with_spacers` ✨ by @kdheepak in [#788](https://github.com/ratatui-org/ratatui/pull/788)

  > Follow up to https://github.com/ratatui-org/ratatui/pull/783
  >
  > This PR introduces different priorities for each kind of constraint.
  > This PR also adds tests that specifies this behavior. This PR resolves a
  > number of broken tests.
  >
  > Fixes https://github.com/ratatui-org/ratatui/issues/827
  >
  > With this PR, the layout algorithm will do the following in order:
  >
  > 1. Ensure that all the segments are within the user provided area and
  > ensure that all segments and spacers are aligned next to each other
  > 2. if a user provides a `layout.spacing`, it will enforce it.
  > 3. ensure proportional elements are all proportional to each other
  > 4. if a user provides a `Fixed(v)` constraint, it will enforce it.
  > 5. `Min` / `Max` binding inequality constraints
  > 6. `Length`
  > 7. `Percentage`
  > 8. `Ratio`
  > 9. collapse `Min` or collapse `Max`
  > 10. grow `Proportional` as much as possible
  > 11. grow spacers as much as possible
  >
  > This PR also returns the spacer areas as `Rects` to the user. Users can
  > then draw into the spacers as they see fit (thanks @joshka for the
  > idea). Here's a screenshot with the modified flex example:
  >
  > <img width="569" alt="image"
  > src="https://github.com/ratatui-org/ratatui/assets/1813121/46c8901d-882c-43b0-ba87-b1d455099d8f">
  >
  > This PR introduces a `strengths` module that has "default" weights that
  > give stable solutions as well as predictable behavior.

- [d713201](https://github.com/ratatui-org/ratatui/commit/d7132011f921cb87593914bd7d2e24ac676ec911) *(uncategorized)* Add `Color::from_hsl` ✨ by @kdheepak in [#772](https://github.com/ratatui-org/ratatui/pull/772)

  > This PR adds `Color::from_hsl` that returns a valid `Color::Rgb`.
  >
  > ```rust
  > let color: Color = Color::from_hsl(360.0, 100.0, 100.0);
  > assert_eq!(color, Color::Rgb(255, 255, 255));
  >
  > let color: Color = Color::from_hsl(0.0, 0.0, 0.0);
  > assert_eq!(color, Color::Rgb(0, 0, 0));
  > ```
  >
  > HSL stands for Hue (0-360 deg), Saturation (0-100%), and Lightness
  > (0-100%) and working with HSL the values can be more intuitive. For
  > example, if you want to make a red color more orange, you can change the
  > Hue closer toward yellow on the color wheel (i.e. increase the Hue).
  >
  > Related #763

- [405a125](https://github.com/ratatui-org/ratatui/commit/405a125c8235b983993e3774361821b67a340aa0) *(uncategorized)* Add wide and tall proportional border set by @Emivvvvv in [#848](https://github.com/ratatui-org/ratatui/pull/848)

  > Adds `PROPORTIONAL_WIDE` and `PROPORTIONAL_TALL` border sets.
  >
  > `symbols::border::PROPORTIONAL_WIDE`
  > ```
  > ▄▄▄▄
  > █xx█
  > █xx█
  > ▀▀▀▀
  > ```
  >
  > `symbols::border::PROPORTIONAL_TALL`
  > ```
  > █▀▀█
  > █xx█
  > █xx█
  > █▄▄█
  > ```
  >
  > Fixes:https://github.com/ratatui-org/ratatui/issues/834

- [9df6ceb](https://github.com/ratatui-org/ratatui/commit/9df6cebb58e97ac795868fa0af96a8aaf9c794c0) *(uncategorized)* Table column calculation uses layout spacing ✨ by @kdheepak in [#824](https://github.com/ratatui-org/ratatui/pull/824)

  > This uses the new `spacing` feature of the `Layout` struct to allocate
  > columns spacing in the `Table` widget.
  > This changes the behavior of the table column layout in the following
  > ways:
  >
  > 1. Selection width is always allocated.
  > - if a user does not want a selection width ever they should use
  > `HighlightSpacing::Never`
  > 2. Column spacing is prioritized over other constraints
  > - if a user does not want column spacing, they should use
  > `Table::new(...).column_spacing(0)`
  >
  > ---------

- [f299463](https://github.com/ratatui-org/ratatui/commit/f299463847e8aa4b61619e5a5c02c5855d8fdb7b) *(uncategorized)* Add one eighth wide and tall border sets ✨ by @kdheepak in [#831](https://github.com/ratatui-org/ratatui/pull/831)

  > This PR adds the
  > [`McGugan`](https://www.willmcgugan.com/blog/tech/post/ceo-just-wants-to-draw-boxes/)
  > border set, which allows for tighter borders.
  >
  > For example, with the `flex` example you can get this effect (top is
  > mcgugan wide, bottom is mcgugan tall):
  >
  > <img width="759" alt="image"
  > src="https://github.com/ratatui-org/ratatui/assets/1813121/756bb50e-f8c3-4eec-abe8-ce358058a526">
  >
  > <img width="759" alt="image"
  > src="https://github.com/ratatui-org/ratatui/assets/1813121/583485ef-9eb2-4b45-ab88-90bd7cb14c54">
  >
  > As of this PR, `MCGUGAN_WIDE` has to be styled manually, like so:
  >
  > ```rust
  >             let main_color = color_for_constraint(*constraint);
  >             let cell = buf.get_mut(block.x, block.y + 1);
  >             cell.set_style(Style::reset().fg(main_color).reversed());
  >             let cell = buf.get_mut(block.x, block.y + 2);
  >             cell.set_style(Style::reset().fg(main_color).reversed());
  >             let cell = buf.get_mut(block.x + block.width.saturating_sub(1), block.y + 1);
  >             cell.set_style(Style::reset().fg(main_color).reversed());
  >             let cell = buf.get_mut(block.x + block.width.saturating_sub(1), block.y + 2);
  >             cell.set_style(Style::reset().fg(main_color).reversed());
  >
  > ```
  >
  > `MCGUGAN_TALL` has to be styled manually, like so:
  >
  > ```rust
  >             let main_color = color_for_constraint(*constraint);
  >             for x in block.x + 1..(block.x + block.width).saturating_sub(1) {
  >                 let cell = buf.get_mut(x, block.y);
  >                 cell.set_style(Style::reset().fg(main_color).reversed());
  >                 let cell = buf.get_mut(x, block.y + block.height - 1);
  >                 cell.set_style(Style::reset().fg(main_color).reversed());
  >             }
  >
  > ```

- [ae6a2b0](https://github.com/ratatui-org/ratatui/commit/ae6a2b0007ee7195de14d36420e2e30853fbb2f4) *(uncategorized)* Add spacing feature to flex example ✨ by @kdheepak in [#830](https://github.com/ratatui-org/ratatui/pull/830)

  > This adds the `spacing` using `+` and `-` to the flex example

- [cddf4b2](https://github.com/ratatui-org/ratatui/commit/cddf4b2930f573fafad64a4ddd7fe5753f7540e2) *(uncategorized)* Implement Display for Text, Line, Span by @Emivvvvv in [#826](https://github.com/ratatui-org/ratatui/pull/826)
  >
  > Issue:https://github.com/ratatui-org/ratatui/issues/816
  >
  > This PR adds:
  >
  > `std::fmt::Display` for `Text`, `Line`, and `Span` structs.
  >
  > Display implementation displays actual content while ignoring style.

- [5131c81](https://github.com/ratatui-org/ratatui/commit/5131c813ce5de078be0458c9a067bca2d6b38921) *(uncategorized)* Add layout spacing ✨ by @kdheepak in [#821](https://github.com/ratatui-org/ratatui/pull/821)

  > This adds a `spacing` feature for layouts.
  >
  > Spacing can be added between items of a layout.

- [de97a1f](https://github.com/ratatui-org/ratatui/commit/de97a1f1da4fd146034f7c8f20264f4d558cc1a0) *(uncategorized)* Add flex to layout ✨ by @kdheepak

  > This PR adds a new way to space elements in a `Layout`.
  >
  > Loosely based on
  > [flexbox](https://css-tricks.com/snippets/css/a-guide-to-flexbox/), this
  > PR adds a `Flex` enum with the following variants:
  >
  > - Start
  > - Center
  > - End
  > - SpaceAround
  > - SpaceBetween
  >
  > <img width="380" alt="image" src="https://github.com/ratatui-org/ratatui/assets/1813121/b744518c-eae7-4e35-bbc4-fe3c95193cde">
  >
  > It also adds two more variants, to make this backward compatible and to
  > make it replace `SegmentSize`:
  >
  > - StretchLast (default in the `Flex` enum, also behavior matches old
  >   default `SegmentSize::LastTakesRemainder`)
  > - Stretch (behavior matches `SegmentSize::EvenDistribution`)
  >
  > The `Start` variant from above matches `SegmentSize::None`.
  >
  > This allows `Flex` to be a complete replacement for `SegmentSize`, hence
  > this PR also deprecates the `segment_size` constructor on `Layout`.
  > `SegmentSize` is still used in `Table` but under the hood `segment_size`
  > maps to `Flex` with all tests passing unchanged.
  >
  > I also put together a simple example for `Flex` layouts so that I could
  > test it visually, shared below:
  >
  > https://github.com/ratatui-org/ratatui/assets/1813121/c8716c59-493f-4631-add5-feecf4bd4e06

- [9a3815b](https://github.com/ratatui-org/ratatui/commit/9a3815b66d8b6e4ff9f6475666f5742701e256bb) *(uncategorized)* Add Constraint::Fixed and Constraint::Proportional ✨ by @kdheepak in [#783](https://github.com/ratatui-org/ratatui/pull/783)

- [425a651](https://github.com/ratatui-org/ratatui/commit/425a65140b61695169c996784974488ad2fd16ea) *(uncategorized)* Add comprehensive tests for Length interacting with other constraints ✨ by @kdheepak in [#802](https://github.com/ratatui-org/ratatui/pull/802)

- [c50ff08](https://github.com/ratatui-org/ratatui/commit/c50ff08a630ae59c9aac10f69fe3ce67c2db449c) *(uncategorized)* Add frame count ✨ by @kdheepak in [#766](https://github.com/ratatui-org/ratatui/pull/766)

- [8f56fab](https://github.com/ratatui-org/ratatui/commit/8f56fabcdd34cb3938736f3302902a7fead64ee5) *(uncategorized)* Accept Color and Modifier for all Styles by @joshka in [#720](https://github.com/ratatui-org/ratatui/pull/720) [**breaking**]

  > * feat: accept Color and Modifier for all Styles
  >
  > All style related methods now accept `S: Into<Style>` instead of
  > `Style`.
  > `Color` and `Modifier` implement `Into<Style>` so this is allows for
  > more ergonomic usage. E.g.:
  >
  > ```rust
  > Line::styled("hello", Style::new().red());
  > Line::styled("world", Style::new().bold());
  >
  > // can now be simplified to
  >
  > Line::styled("hello", Color::Red);
  >
  > Line::styled("world", Modifier::BOLD);
  > ```
  >
  > Fixes https://github.com/ratatui-org/ratatui/issues/694
  >
  > BREAKING CHANGE:All style related methods now accept `S: Into<Style>`
  > instead of `Style`. This means that if you are already passing an
  > ambiguous type that implements `Into<Style>` you will need to remove
  > the `.into()` call.
  >
  > `Block` style methods can no longer be called from a const context as
  > trait functions cannot (yet) be const.
  >
  > * feat: add tuple conversions to Style
  >
  > Adds conversions for various Color and Modifier combinations
  >
  > * chore: add unit tests

### Bug Fixes

- [ee54493](https://github.com/ratatui-org/ratatui/commit/ee544931633ada25d84daa95e4e3a0b17801cb8b) *(buffer)* Don't panic in set_style by @joshka in [#714](https://github.com/ratatui-org/ratatui/pull/714)

  > This fixes a panic in set_style when the area to be styled is
  > outside the buffer's bounds.

- [c959bd2](https://github.com/ratatui-org/ratatui/commit/c959bd2881244a4ad9609403d8a84860f290b859) *(calendar)* CalendarEventStore panic by @lxl66566 in [#822](https://github.com/ratatui-org/ratatui/pull/822)
  >
  > `CalendarEventStore::today()` panics if the system's UTC offset cannot
  > be determined. In this circumstance, it's better to use `now_utc`
  > instead.

- [0614190](https://github.com/ratatui-org/ratatui/commit/06141900b4f049dd2c76bfccb49b4d51ae854bb0) *(cd)* Fix grepping the last release by @Valentin271 in [#762](https://github.com/ratatui-org/ratatui/pull/762)

- [a67815e](https://github.com/ratatui-org/ratatui/commit/a67815e1388806d87d387ff17af0dfab48412011) *(chart)* Exclude unnamed datasets from legend by @Valentin271 in [#753](https://github.com/ratatui-org/ratatui/pull/753)

  > A dataset with no name won't display an empty line anymore in the legend.
  > If no dataset have name, then no legend is ever displayed.

- [3e7810a](https://github.com/ratatui-org/ratatui/commit/3e7810a2ab2bbd09027ecd832aa295c5e71d9eda) *(example)* Increase layout cache size by @Valentin271 in [#815](https://github.com/ratatui-org/ratatui/pull/815)

  > This was causing very bad performances especially on scrolling.
  > It's also a good usage demonstration.

- [50b81c9](https://github.com/ratatui-org/ratatui/commit/50b81c9d4ea6a357cc964baff0b267dcfe6087c6) *(examples/scrollbar)* Title wasn't displayed because of background reset by @Valentin271 in [#795](https://github.com/ratatui-org/ratatui/pull/795)

- [b3a57f3](https://github.com/ratatui-org/ratatui/commit/b3a57f3dff1e56fe431235b839c4bd0ee0fec594) *(list)* Modify List and List example to support saving offsets. by @bblsh in [#667](https://github.com/ratatui-org/ratatui/pull/667)

  > The current `List` example will unselect and reset the position of a
  > list.
  >
  > This PR will save the last selected item, and updates `List` to honor
  > its offset, preventing the list from resetting when the user
  > `unselect()`s a `StatefulList`.

- [6645d2e](https://github.com/ratatui-org/ratatui/commit/6645d2e0585a4e2d1d64fa730c09077b2d215545) *(table)* Ensure that default and new() match by @joshka in [#751](https://github.com/ratatui-org/ratatui/pull/751) [**breaking**]

  > In https://github.com/ratatui-org/ratatui/pull/660 we introduced the
  > segment_size field to the Table struct. However, we forgot to update
  > the default() implementation to match the new() implementation. This
  > meant that the default() implementation picked up SegmentSize::default()
  > instead of SegmentSize::None.
  >
  > Additionally the introduction of Table::default() in an earlier PR,
  > https://github.com/ratatui-org/ratatui/pull/339, was also missing the
  > default for the column_spacing field (1).
  >
  > This commit fixes the default() implementation to match the new()
  > implementation of these two fields by implementing the Default trait
  > manually.
  >
  > BREAKING CHANGE:The default() implementation of Table now sets the
  > column_spacing field to 1 and the segment_size field to
  >
  > SegmentSize::None. This will affect the rendering of a small amount of
  > apps.

- [b0ed658](https://github.com/ratatui-org/ratatui/commit/b0ed658970e8a94f25948c80d511102c197a8f6a) *(table)* Render missing widths as equal by @joshka in [#710](https://github.com/ratatui-org/ratatui/pull/710)

  > Previously, if `.widths` was not called before rendering a `Table`, no
  > content would render in the area of the table. This commit changes that
  > behaviour to default to equal widths for each column.
  >
  > Fixes #510.

- [f71bf18](https://github.com/ratatui-org/ratatui/commit/f71bf182975526aa2eca9ee710361f39db2d666d) *(uncategorized)* Bug with flex stretch with spacing and proportional constraints by @kdheepak in [#829](https://github.com/ratatui-org/ratatui/pull/829)

  > This PR fixes a bug with layouts when using spacing on proportional
  > constraints.

- [cc6737b](https://github.com/ratatui-org/ratatui/commit/cc6737b8bc09d254413adc1cbf2bc62d2f93792d) *(uncategorized)* Make SpaceBetween with one element Stretch 🐛 by @kdheepak in [#813](https://github.com/ratatui-org/ratatui/pull/813)

  > When there's just one element, `SpaceBetween` should do the same thing
  > as `Stretch`.

- [7a8af8d](https://github.com/ratatui-org/ratatui/commit/7a8af8da6ba83c7a3f31d03b29c51de6b03ced64) *(uncategorized)* Update templates links by @Valentin271 in [#808](https://github.com/ratatui-org/ratatui/pull/808)

- [f2eab71](https://github.com/ratatui-org/ratatui/commit/f2eab71ccf11a206c253bf4efeafc744f103b116) *(uncategorized)* Broken tests in table.rs by @kdheepak in [#784](https://github.com/ratatui-org/ratatui/pull/784)

  > * fix: broken tests in table.rs
  >
  > * fix: Use default instead of raw

- [8dd177a](https://github.com/ratatui-org/ratatui/commit/8dd177a0513230bfddc89aa315dfb49d1c7b070c) *(uncategorized)* Fix PR write permission to upload unsigned commit comment by @Valentin271 in [#770](https://github.com/ratatui-org/ratatui/pull/770)

### Refactor

- [cf86123](https://github.com/ratatui-org/ratatui/commit/cf861232c7c2369fa44010374432ba0a4814b6f8) *(scrollbar)* Rewrite scrollbar implementation by @kdheepak in [#847](https://github.com/ratatui-org/ratatui/pull/847)

  > Implementation was simplified and calculates the size of the thumb a
  > bit more proportionally to the content that is visible.

- [fd4703c](https://github.com/ratatui-org/ratatui/commit/fd4703c0869eca22a51d9a33f7bb54bfd051c565) *(block)* Move padding and title into separate files by @joshka in [#837](https://github.com/ratatui-org/ratatui/pull/837)

- [bc274e2](https://github.com/ratatui-org/ratatui/commit/bc274e2bd9cfee1133dfbcca3c95374560706537) *(block)* Remove deprecated `title_on_bottom` by @Valentin271 in [#757](https://github.com/ratatui-org/ratatui/pull/757) [**breaking**]
  >
  > `Block::title_on_bottom` was deprecated in v0.22. Use `Block::title` and `Title::position` instead.

- [a62632a](https://github.com/ratatui-org/ratatui/commit/a62632a947a950f7ab303e67eb910b01f4ee256d) *(buffer)* Split buffer module into files by @joshka in [#721](https://github.com/ratatui-org/ratatui/pull/721)

- [e0aa6c5](https://github.com/ratatui-org/ratatui/commit/e0aa6c5e1f254c7222afee7a8acf1652025b1949) *(chart)* Replace deprecated apply by @Valentin271 in [#812](https://github.com/ratatui-org/ratatui/pull/812)
  >
  > Fixes #793

- [7f42ec9](https://github.com/ratatui-org/ratatui/commit/7f42ec97139da1897583d1d04610fa24e3c53fa2) *(colors_rgb)* Impl widget on mutable refs by @joshka in [#865](https://github.com/ratatui-org/ratatui/pull/865)

  > This commit refactors the colors_rgb example to implement the Widget
  > trait on mutable references to the app and its sub-widgets. This allows
  > the app to update its state while it is being rendered.
  >
  > Additionally the main and run functions are refactored to be similar to
  > the other recent examples. This uses a pattern where the App struct has
  > a `run` method that takes a terminal as an argument, and the main
  > function is in control of initializing and restoring the terminal and
  > installing the error hooks.

- [813f707](https://github.com/ratatui-org/ratatui/commit/813f707892d77177b5f7bfe910ff0d312f17eb83) *(example)* Improve constraints and flex examples by @Valentin271 in [#817](https://github.com/ratatui-org/ratatui/pull/817)

  > This PR is a follow up to
  > https://github.com/ratatui-org/ratatui/pull/811.
  >
  > It improves the UI of the layouts by
  >
  > - thoughtful accessible color that represent priority in constraints
  > resolving
  > - using QUADRANT_OUTSIDE symbol set for block rendering
  > - adding a scrollbar
  > - panic handling
  > - refactoring for readability
  >
  > to name a few. Here are some example gifs of the outcome:
  >
  >
  > ![constraints](https://github.com/ratatui-org/ratatui/assets/381361/8eed34cf-e959-472f-961b-d439bfe3324e)
  >
  >
  > ![flex](https://github.com/ratatui-org/ratatui/assets/381361/3195a56c-9cb6-4525-bc1c-b969c0d6a812)
  >
  > ---------

- [bb5444f](https://github.com/ratatui-org/ratatui/commit/bb5444f618f8baf7be9c9ba9f0cad829160d9392) *(example)* Add scroll to flex example by @Valentin271 in [#811](https://github.com/ratatui-org/ratatui/pull/811)

  > This commit adds `scroll` to the flex example. It also adds more examples to showcase how constraints interact. It improves the UI to make it easier to understand and short terminal friendly.
  >
  > <img width="380" alt="image" src="https://github.com/ratatui-org/ratatui/assets/1813121/30541efc-ecbe-4e28-b4ef-4d5f1dc63fec"/>
  >
  > ---------

- [6d15b25](https://github.com/ratatui-org/ratatui/commit/6d15b2570ff1a7c5dc2f6888efb313fb38f55f2a) *(layout)* Move the remaining types by @joshka in [#743](https://github.com/ratatui-org/ratatui/pull/743)

  > - alignment -> layout/alignment.rs
  > - corner -> layout/corner.rs
  > - direction -> layout/direction.rs
  > - size -> layout/size.rs

- [659460e](https://github.com/ratatui-org/ratatui/commit/659460e19cc4109a36f416f79e583066730ca199) *(layout)* Move SegmentSize to layout/segment_size.rs by @joshka in [#742](https://github.com/ratatui-org/ratatui/pull/742)

- [ba036cd](https://github.com/ratatui-org/ratatui/commit/ba036cd57966ff9e7e2f871580095fda1df158ee) *(layout)* Move Layout to layout/layout.rs by @joshka in [#741](https://github.com/ratatui-org/ratatui/pull/741)

- [8724aeb](https://github.com/ratatui-org/ratatui/commit/8724aeb9e74f4756a15681740ce7825cb094b42a) *(layout)* Move Margin to margin.rs by @joshka in [#740](https://github.com/ratatui-org/ratatui/pull/740)

- [9574198](https://github.com/ratatui-org/ratatui/commit/95741989588547cec12aaa27fbb5bc7cf2600426) *(line)* Reorder methods for natural reading order by @joshka in [#713](https://github.com/ratatui-org/ratatui/pull/713)

- [6364533](https://github.com/ratatui-org/ratatui/commit/63645333d681c13502047e20d67612d9113d4375) *(table)* Split table into multiple files by @joshka in [#718](https://github.com/ratatui-org/ratatui/pull/718)

  > At close to 2000 lines of code, the table widget was getting a bit
  > unwieldy. This commit splits it into multiple files, one for each
  > struct, and one for the table itself.
  >
  > Also refactors the table rendering code to be easier to maintain.

- [5aba988](https://github.com/ratatui-org/ratatui/commit/5aba988fac6d0a2437192f5127c36bd272de5c78) *(terminal)* Extract types to files by @joshka in [#760](https://github.com/ratatui-org/ratatui/pull/760)

  > Fields on Frame that were private are now pub(crate).

- [4d262d2](https://github.com/ratatui-org/ratatui/commit/4d262d21cbfba12da92a754fad533403df20701d) *(widget)* Move borders to widgets/borders.rs by @joshka in [#832](https://github.com/ratatui-org/ratatui/pull/832)

- [5254795](https://github.com/ratatui-org/ratatui/commit/525479546acebff7faec165f45028001a01525fe) *(uncategorized)* Make layout tests a bit easier to understand by @joshka in [#890](https://github.com/ratatui-org/ratatui/pull/890)

- [bd6b91c](https://github.com/ratatui-org/ratatui/commit/bd6b91c958a8ac2eb5b0e62432d65294403e5af3) *(uncategorized)* Make `patch_style` & `reset_style` chainable by @Valentin271 in [#754](https://github.com/ratatui-org/ratatui/pull/754) [**breaking**]

  > Previously, `patch_style` and `reset_style` in `Text`, `Line` and `Span`
  >  were using a mutable reference to `Self`. To be more consistent with
  >  the rest of `ratatui`, which is using fluent setters, these now take
  >  ownership of `Self` and return it.

- [da6c299](https://github.com/ratatui-org/ratatui/commit/da6c299804850a1b7747ca1472c9a904bcd956ea) *(uncategorized)* Extract layout::Constraint to file by @joshka in [#739](https://github.com/ratatui-org/ratatui/pull/739)

### Documentation

- [6ecaeed](https://github.com/ratatui-org/ratatui/commit/6ecaeed5497b15c4fa12c15048776b884e46b985) *(text)* Add overview of the relevant methods by @Emivvvvv in [#857](https://github.com/ratatui-org/ratatui/pull/857)

  > Add an overview of the relevant methods under `Constructor Methods`, `Setter Methods`, and `Other Methods` subtitles.

- [50374b2](https://github.com/ratatui-org/ratatui/commit/50374b2456808af8e14715c86bd773d7cfee2627) *(backend)* Fix broken book link by @orhun in [#733](https://github.com/ratatui-org/ratatui/pull/733)

- [e1cc849](https://github.com/ratatui-org/ratatui/commit/e1cc8495544513bc0d9a26f8d2fe446d9b6b1091) *(breaking)* Fix typo by @a-kenji in [#702](https://github.com/ratatui-org/ratatui/pull/702)

- [49df5d4](https://github.com/ratatui-org/ratatui/commit/49df5d46263a3e2fab2e8bdb9379c507922e3aa1) *(example)* Fix markdown syntax for note by @akiomik in [#730](https://github.com/ratatui-org/ratatui/pull/730)

- [4b8e54e](https://github.com/ratatui-org/ratatui/commit/4b8e54e811bbd591f21ad8fe5b2467e4486aa6e9) *(examples)* Refactor Tabs example by @Emivvvvv in [#861](https://github.com/ratatui-org/ratatui/pull/861)

  > - Used a few new techniques from the 0.26 features (ref widgets, text rendering,
  >   dividers / padding etc.)
  > - Updated the app to a simpler application approach
  > - Use color_eyre
  > - Make it look pretty (colors, new proportional borders)
  >
  > ![Made with VHS](https://vhs.charm.sh/vhs-4WW21XTtepDhUSq4ZShO56.gif)
  >
  > ---------
  > Fixes https://github.com/ratatui-org/ratatui/issues/819
  > Co-authored-by: Josh McKinney <joshka@users.noreply.github.com>

- [5b7ad2a](https://github.com/ratatui-org/ratatui/commit/5b7ad2ad82f38af25d5f8d40ea5bdc454fbbbc60) *(examples)* Update gauge example by @Emivvvvv in [#863](https://github.com/ratatui-org/ratatui/pull/863)

  > - colored gauges
  > - removed box borders
  > - show the difference between ratio / percentage and unicode / no unicode better
  > - better application approach (consistent with newer examples)
  > - various changes for 0.26 featuers
  > - impl `Widget` for `&App`
  > - use color_eyre
  >
  > for gauge.tape
  >
  > - change to get better output from the new code
  >
  > ---------
  > Fixes: https://github.com/ratatui-org/ratatui/issues/846
  > Co-authored-by: Josh McKinney <joshka@users.noreply.github.com>

- [f383625](https://github.com/ratatui-org/ratatui/commit/f383625f0e1cae320ae56af615f3b05c59700f93) *(examples)* Add note about example versions to all examples by @joshka in [#871](https://github.com/ratatui-org/ratatui/pull/871)

- [847bacf](https://github.com/ratatui-org/ratatui/commit/847bacf32ee40e5af2207f8aefd2a0538beec693) *(examples)* Refactor demo2 by @joshka in [#836](https://github.com/ratatui-org/ratatui/pull/836)

  > Simplified a bunch of the logic in the demo2 example
  > - Moved destroy mode to its own file.
  > - Moved error handling to its own file.
  > - Removed AppContext
  > - Implemented Widget for &App. The app state is small enough that it
  >   doesn't matter here and we could just copy or clone the app state on
  >   every frame, but for larger apps this can be a significant performance
  >   improvement.
  > - Made the tabs stateful
  > - Made the term module just a collection of functions rather than a
  >   struct.
  > - Changed to use color_eyre for error handling.
  > - Changed keyboard shortcuts and rearranged the bottom bar.
  > - Use strum for the tabs enum.

- [804c841](https://github.com/ratatui-org/ratatui/commit/804c841fdc370049403282e0c6d140cbed85db7b) *(examples)* Update list example and list.tape by @Emivvvvv in [#864](https://github.com/ratatui-org/ratatui/pull/864)

  > This PR adds:
  >
  > - subjectively better-looking list example
  > - change list example to a todo list example
  > - status of a TODO can be changed, further info can be seen under the list.

- [eb1484b](https://github.com/ratatui-org/ratatui/commit/eb1484b6db5b21df6bda017fbe1a8f4888151ed3) *(examples)* Update tabs example and tabs.tape by @Emivvvvv in [#855](https://github.com/ratatui-org/ratatui/pull/855)

  > This PR adds:
  >
  > for tabs.rs
  >
  > - general refactoring on code
  > - subjectively better looking front
  > - add tailwind colors
  >
  > for tabs.tape
  >
  > - change to get better output from the new code
  >
  > Here is the new output:
  >
  > ![tabs](https://github.com/ratatui-org/ratatui/assets/30180366/0a9371a5-e90d-42ba-aba5-70cbf66afd1f)

- [330a899](https://github.com/ratatui-org/ratatui/commit/330a899eacb1f7d2d6dc19856f2bbb782e2c53b0) *(examples)* Update table example and table.tape by @Emivvvvv in [#840](https://github.com/ratatui-org/ratatui/pull/840)

  > In table.rs
  > - added scrollbar to the table
  > - colors changed to use style::palette::tailwind
  > - now colors can be changed with keys (l or →) for the next color, (h or
  > ←) for the previous color
  > - added a footer for key info
  >
  > For table.tape
  > - typing speed changed to 0.75s from 0.5s
  > - screen size changed to fit
  > - pushed keys changed to show the current example better
  >
  > Fixes:https://github.com/ratatui-org/ratatui/issues/800

- [41de884](https://github.com/ratatui-org/ratatui/commit/41de8846fda6b50dbd8288eb108037dd5b0a2acd) *(examples)* Document incompatible examples better by @joshka in [#844](https://github.com/ratatui-org/ratatui/pull/844)

  > Examples often take advantage of unreleased API changes, which makes
  > them not copy-paste friendly.

- [3464894](https://github.com/ratatui-org/ratatui/commit/34648941d447245cf7b1b6172fe84b1867b1bd5a) *(examples)* Add warning about examples matching the main branch by @joshka in [#778](https://github.com/ratatui-org/ratatui/pull/778)

- [fb93db0](https://github.com/ratatui-org/ratatui/commit/fb93db073029fc9bc6a365511706c1f60a64af1b) *(examples)* Simplify docs using new layout methods by @joshka in [#731](https://github.com/ratatui-org/ratatui/pull/731)

  > Use the new `Layout::horizontal` and `vertical` constructors and
  > `Rect::split_array` through all the examples.

- [d6b8513](https://github.com/ratatui-org/ratatui/commit/d6b851301e0edcc96274262c2351391c4d414481) *(examples)* Refactor chart example to showcase scatter by @Valentin271 in [#703](https://github.com/ratatui-org/ratatui/pull/703)

- [fe84141](https://github.com/ratatui-org/ratatui/commit/fe84141119d87f478478fa1570344aaa7fa5f417) *(layout)* Document the difference in the split methods by @joshka in [#750](https://github.com/ratatui-org/ratatui/pull/750)

  > * docs(layout): document the difference in the split methods
  >
  > * fix: doc suggestion

- [48b0380](https://github.com/ratatui-org/ratatui/commit/48b0380cb3c50b62fe347e27fed46b6c702d0e13) *(scrollbar)* Complete scrollbar documentation by @Valentin271 in [#823](https://github.com/ratatui-org/ratatui/pull/823)

- [e67d3c6](https://github.com/ratatui-org/ratatui/commit/e67d3c64e0192ac5a31ecb34cfb8a55c53ba7bdc) *(table)* Fix typo by @a-kenji in [#707](https://github.com/ratatui-org/ratatui/pull/707)

- [065b6b0](https://github.com/ratatui-org/ratatui/commit/065b6b05b7685d30cfccc9343ff5232fe67d5a7a) *(terminal)* Document buffer diffing better by @Valentin271 in [#852](https://github.com/ratatui-org/ratatui/pull/852)

- [bcf4368](https://github.com/ratatui-org/ratatui/commit/bcf43688ec4a13825307aef88f3cdcd007b32641) *(uncategorized)* Update BREAKING-CHANGES.md summary by @kdheepak in [#907](https://github.com/ratatui-org/ratatui/pull/907)

  > Update summary section

- [652dc46](https://github.com/ratatui-org/ratatui/commit/652dc469ea7ac71bbbc33a291f027f3a907fbbcf) *(uncategorized)* Update BREAKING-CHANGES.md with flex section by @kdheepak in [#906](https://github.com/ratatui-org/ratatui/pull/906)

  > Follow up to: https://github.com/ratatui-org/ratatui/pull/881

- [86168aa](https://github.com/ratatui-org/ratatui/commit/86168aa7117b4f4218bd658c861a0bd2bc03e7b5) *(uncategorized)* Fix docstring for `Max` constraints by @kdheepak in [#898](https://github.com/ratatui-org/ratatui/pull/898)

- [11e4f6a](https://github.com/ratatui-org/ratatui/commit/11e4f6a0ba71b7adad44af5866a2b0789175aafa) *(uncategorized)* Adds better documentation for constraints and flex 📚 by @kdheepak in [#818](https://github.com/ratatui-org/ratatui/pull/818)

- [1746a61](https://github.com/ratatui-org/ratatui/commit/1746a616595af019d52b8cd69bf08d5c49c0a968) *(uncategorized)* Update links to templates repository 📚 by @kdheepak in [#810](https://github.com/ratatui-org/ratatui/pull/810)

  > This PR updates links to the `templates` repository.

- [43b2b57](https://github.com/ratatui-org/ratatui/commit/43b2b57191ed9226c93cbef40b8e5b899ef81fdc) *(uncategorized)* Fix typo in Table widget description by @stchris in [#797](https://github.com/ratatui-org/ratatui/pull/797)

- [2b4aa46](https://github.com/ratatui-org/ratatui/commit/2b4aa46a6a225c6629778257a4548b7fa55f3ef9) *(uncategorized)* GitHub admonition syntax for examples README.md by @kdheepak in [#791](https://github.com/ratatui-org/ratatui/pull/791)

  > * docs: GitHub admonition syntax for examples README.md
  >
  > * docs: Add link to stable release

- [388aa46](https://github.com/ratatui-org/ratatui/commit/388aa467f17dd219ec8e99a177547eb03c6fa01d) *(uncategorized)* Update crate, lib and readme links by @joshka in [#771](https://github.com/ratatui-org/ratatui/pull/771)

  > Link to the contributing, changelog, and breaking changes docs at the
  > top of the page instead of just in in the main part of the doc. This
  > makes it easier to find them.
  >
  > Rearrange the links to be in a more logical order.
  >
  > Use link refs for all the links
  >
  > Fix up the CI link to point to the right workflow

### Performance

- [1d3fbc1](https://github.com/ratatui-org/ratatui/commit/1d3fbc1b15c619f571b9981b841986a7947a4195) *(buffer)* Apply SSO technique to text buffer in `buffer::Cell` by @rhysd in [#601](https://github.com/ratatui-org/ratatui/pull/601) [**breaking**]

  > Use CompactString instead of String to store the Cell::symbol field.
  > This saves reduces the size of memory allocations at runtime.

### Testing

- [663bbde](https://github.com/ratatui-org/ratatui/commit/663bbde9c39afc1ad15cc44228811ae1b62f4343) *(layout)* Convert layout tests to use rstest by @kdheepak in [#879](https://github.com/ratatui-org/ratatui/pull/879)

  > This PR makes all the letters test use `rstest`

- [f780be3](https://github.com/ratatui-org/ratatui/commit/f780be31f37f2305f514f4dba6f82dcae0ad3f9b) *(layout)* Parameterized tests 🚨 by @kdheepak in [#858](https://github.com/ratatui-org/ratatui/pull/858)

### Miscellaneous Tasks

- [ba20372](https://github.com/ratatui-org/ratatui/commit/ba20372c23c65122db055e202cfe68fcddafd342) *(contributing)* Remove part about squashing commits by @Valentin271 in [#874](https://github.com/ratatui-org/ratatui/pull/874)

  > Removes the part about squashing commits from the CONTRIBUTING file.
  >
  > We no longer require that because github squashes commits when merging.
  > This will cleanup the CONTRIBUTING file a bit which is already quite
  > dense.

- [d49bbb2](https://github.com/ratatui-org/ratatui/commit/d49bbb259091a7b061e0dec71ee06884b27e308a) *(ci)* Update the job description for installing cargo-nextest by @orhun in [#839](https://github.com/ratatui-org/ratatui/pull/839)

- [8d77b73](https://github.com/ratatui-org/ratatui/commit/8d77b734bb5d267114afffd4bb594695d8544dce) *(ci)* Use cargo-nextest for running tests by @orhun in [#717](https://github.com/ratatui-org/ratatui/pull/717)

  > * chore(ci): use cargo-nextest for running tests
  >
  > * refactor(make): run library tests before doc tests

- [b7a4793](https://github.com/ratatui-org/ratatui/commit/b7a479392ee71574e32b5aa797ef612cdd99498f) *(ci)* Bump alpha release for breaking changes by @joshka in [#495](https://github.com/ratatui-org/ratatui/pull/495)

  > Automatically detect breaking changes based on commit messages
  > and bump the alpha release number accordingly.
  >
  > E.g. v0.23.1-alpha.1 will be bumped to v0.24.0-alpha.0 if any commit
  > since v0.23.0 has a breaking change.

- [fab943b](https://github.com/ratatui-org/ratatui/commit/fab943b61afb1c5f79d03b1f3764067ac26945d0) *(contributing)* Add deprecation notice guideline by @Valentin271 in [#761](https://github.com/ratatui-org/ratatui/pull/761)

- [fc0879f](https://github.com/ratatui-org/ratatui/commit/fc0879f98dedf36699ebf77b5b1298f6f3fb3015) *(layout)* Comment tests that may fail on occasion by @kdheepak in [#814](https://github.com/ratatui-org/ratatui/pull/814)

  > These fails seem to fail on occasion, locally and on CI.
  >
  > This issue will be revisited in the PR on constraint weights:
  > https://github.com/ratatui-org/ratatui/pull/788

- [f8367fd](https://github.com/ratatui-org/ratatui/commit/f8367fdfdd1da0ae98705a0b23fc88d156425f4c) *(uncategorized)* Allow Buffer::with_lines to accept IntoIterator by @joshka in [#901](https://github.com/ratatui-org/ratatui/pull/901)

  > This can make it easier to use `Buffer::with_lines` with iterators that
  > don't necessarily produce a `Vec`. For example, this allows using
  > `Buffer::with_lines` with `&[&str]` directly, without having to call
  > `collect` on it first.

- [78f1c14](https://github.com/ratatui-org/ratatui/commit/78f1c1446b00824970449d9aff2d74ef875d2449) *(uncategorized)* Small fixes to constraint-explorer by @joshka in [#894](https://github.com/ratatui-org/ratatui/pull/894)

- [984afd5](https://github.com/ratatui-org/ratatui/commit/984afd580bff5be6f30622733e5a28db952c72fd) *(uncategorized)* Cache dependencies in the CI workflow to speed up builds by @joshka in [#883](https://github.com/ratatui-org/ratatui/pull/883)

- [6e76729](https://github.com/ratatui-org/ratatui/commit/6e76729ce899e2f32af8335aff530622d9a8dbe4) *(uncategorized)* Move example vhs tapes to a folder by @joshka in [#867](https://github.com/ratatui-org/ratatui/pull/867)

- [151db6a](https://github.com/ratatui-org/ratatui/commit/151db6ac7d93713b6212ce627e3b725879573aa9) *(uncategorized)* Add commit footers to git-cliff config by @joshka in [#805](https://github.com/ratatui-org/ratatui/pull/805)
  >
  > Fixes:https://github.com/orhun/git-cliff/issues/297

- [c24216c](https://github.com/ratatui-org/ratatui/commit/c24216cf307bba7d19ed579a10ef541e28dfd4bc) *(uncategorized)* Add comment on PRs with unsigned commits by @Valentin271 in [#768](https://github.com/ratatui-org/ratatui/pull/768)



### New Contributors
* @Eeelco made their first contribution in [#873](https://github.com/ratatui-org/ratatui/pull/873)
* @Emivvvvv made their first contribution in [#861](https://github.com/ratatui-org/ratatui/pull/861)
* @bblsh made their first contribution in [#667](https://github.com/ratatui-org/ratatui/pull/667)
* @lxl66566 made their first contribution in [#822](https://github.com/ratatui-org/ratatui/pull/822)
* @MultisampledNight made their first contribution in [#723](https://github.com/ratatui-org/ratatui/pull/723)
* @stchris made their first contribution in [#797](https://github.com/ratatui-org/ratatui/pull/797)
* @Lunderberg made their first contribution in [#774](https://github.com/ratatui-org/ratatui/pull/774)
* @BogdanPaul15 made their first contribution in [#765](https://github.com/ratatui-org/ratatui/pull/765)
* @akiomik made their first contribution in [#730](https://github.com/ratatui-org/ratatui/pull/730)
* @yanganto made their first contribution in [#722](https://github.com/ratatui-org/ratatui/pull/722)

**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.25.0...v0.26.0


## [v0.25.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.25.0) - 2023-12-18

### Features

- [aef4956](https://github.com/ratatui-org/ratatui/commit/aef495604c52e563fbacfb1a6e730cd441a99129) *(list)* `List::new` now accepts `IntoIterator<Item = Into<ListItem>>` by @Valentin271 in [#672](https://github.com/ratatui-org/ratatui/pull/672) [**breaking**]

  > This allows to build list like
  >
  > ```
  > List::new(["Item 1", "Item 2"])
  > ```
  >
  > BREAKING CHANGE:`List::new` parameter type changed from `Into<Vec<ListItem<'a>>>`
  > to `IntoIterator<Item = Into<ListItem<'a>>>`

- [8bfd666](https://github.com/ratatui-org/ratatui/commit/8bfd6661e251b6943f74bda626e4708b2e9f4b51) *(paragraph)* Add `line_count` and `line_width` unstable helper methods by @TylerBloom in [#668](https://github.com/ratatui-org/ratatui/pull/668)

  > This is an unstable feature that may be removed in the future

- [1229b96](https://github.com/ratatui-org/ratatui/commit/1229b96e428df880a951ef57f53ca73e74ef1ea2) *(rect)* Add `offset` method by @Valentin271 in [#533](https://github.com/ratatui-org/ratatui/pull/533)

  > The offset method creates a new Rect that is moved by the amount
  > specified in the x and y direction. These values can be positive or
  > negative. This is useful for manual layout tasks.
  >
  > ```rust
  > let rect = area.offset(Offset { x: 10, y -10 });
  > ```

- [edacaf7](https://github.com/ratatui-org/ratatui/commit/edacaf7ff4e4b14702f6361af5a6da713b7dc564) *(buffer)* Deprecate `Cell::symbol` field by @rhysd in [#624](https://github.com/ratatui-org/ratatui/pull/624)

  > The Cell::symbol field is now accessible via a getter method (`symbol()`). This will
  > allow us to make future changes to the Cell internals such as replacing `String` with
  > `compact_str`.

- [6b2efd0](https://github.com/ratatui-org/ratatui/commit/6b2efd0f6c3bf56dc06bbf042db40c0c66de577e) *(layout)* Accept IntoIterator for constraints by @joshka in [#663](https://github.com/ratatui-org/ratatui/pull/663)

  > Layout and Table now accept IntoIterator for constraints with an Item
  > that is AsRef<Constraint>. This allows pretty much any collection of
  > constraints to be passed to the layout functions including arrays,
  > vectors, slices, and iterators (without having to call collect() on
  > them).

- [753e246](https://github.com/ratatui-org/ratatui/commit/753e246531e1e9e2ea558911f8d03e738901d85f) *(layout)* Allow configuring layout fill by @joshka in [#633](https://github.com/ratatui-org/ratatui/pull/633)

  > The layout split will generally fill the remaining area when `split()`
  > is called. This change allows the caller to configure how any extra
  > space is allocated to the `Rect`s. This is useful for cases where the
  > caller wants to have a fixed size for one of the `Rect`s, and have the
  > other `Rect`s fill the remaining space.
  >
  > For now, the method and enum are marked as unstable because the exact
  > name is still being bikeshedded. To enable this functionality, add the
  > `unstable-segment-size` feature flag in your `Cargo.toml`.
  >
  > To configure the layout to fill the remaining space evenly, use
  > `Layout::segment_size(SegmentSize::EvenDistribution)`. The default
  > behavior is `SegmentSize::LastTakesRemainder`, which gives the last
  > segment the remaining space. `SegmentSize::None` will disable this
  > behavior. See the docs for `Layout::segment_size()` and
  > `layout::SegmentSize` for more information.
  >
  > Fixes https://github.com/ratatui-org/ratatui/issues/536

- [1e2f0be](https://github.com/ratatui-org/ratatui/commit/1e2f0be75ac3fb3d6500c1de291bd49972b808e4) *(layout)* Add parameters to Layout::new() by @joshka in [#557](https://github.com/ratatui-org/ratatui/pull/557) [**breaking**]

  > Adds a convenience function to create a layout with a direction and a
  > list of constraints which are the most common parameters that would be
  > generally configured using the builder pattern. The constraints can be
  > passed in as any iterator of constraints.
  >
  > ```rust
  > let layout = Layout::new(Direction::Horizontal, [
  >     Constraint::Percentage(50),
  >     Constraint::Percentage(50),
  > ]);
  > ```
  >
  > BREAKING CHANGE:Layout::new() now takes a direction and a list of constraints instead of
  > no arguments. This is a breaking change because it changes the signature
  > of the function. Layout::new() is also no longer const because it takes
  > an iterator of constraints.

- [c862aa5](https://github.com/ratatui-org/ratatui/commit/c862aa5e9ef4dbf494b5151214ac87f5c71e76d4) *(list)* Support line alignment by @orhun in [#599](https://github.com/ratatui-org/ratatui/pull/599)

  > The `List` widget now respects the alignment of `Line`s and renders them as expected.

- [4424637](https://github.com/ratatui-org/ratatui/commit/4424637af252dc2f227fe4956eac71135e60fb02) *(span)* Add setters for content and style by @joshka in [#647](https://github.com/ratatui-org/ratatui/pull/647)

- [ebf1f42](https://github.com/ratatui-org/ratatui/commit/ebf1f4294211d478b8633a06576ec269a50db588) *(style)* Implement `From` trait for crossterm to `Style` related structs by @Valentin271 in [#686](https://github.com/ratatui-org/ratatui/pull/686)

- [e49385b](https://github.com/ratatui-org/ratatui/commit/e49385b78c8e01fe6381b19d15137346bc6eb8a1) *(table)* Add a Table::segment_size method by @asomers in [#660](https://github.com/ratatui-org/ratatui/pull/660)

  > It controls how to distribute extra space to an underconstrained table.
  > The default, legacy behavior is to leave the extra space unused.  The
  > new options are LastTakesRemainder which gets all space to the rightmost
  > column that can used it, and EvenDistribution which divides it amongst
  > all columns.
  >
  > Fixes #370

- [b8f71c0](https://github.com/ratatui-org/ratatui/commit/b8f71c0d6eda3da272d29c7a9b3c47181049f76a) *(widgets/chart)* Add option to set the position of legend by @lyuha in [#378](https://github.com/ratatui-org/ratatui/pull/378)

- [5bf4f52](https://github.com/ratatui-org/ratatui/commit/5bf4f52119ab3e0e3a266af196058179dc1d18c3) *(uncategorized)* Implement `From` trait for termion to `Style` related structs by @Valentin271 in [#692](https://github.com/ratatui-org/ratatui/pull/692)

  > * feat(termion): implement from termion color
  >
  > * feat(termion): implement from termion style
  >
  > * feat(termion): implement from termion `Bg` and `Fg`

- [d19b266](https://github.com/ratatui-org/ratatui/commit/d19b266e0eabdb0fb00660439a1818239c94024b) *(uncategorized)* Add Constraint helpers (e.g. from_lengths) by @joshka in [#641](https://github.com/ratatui-org/ratatui/pull/641)

  > Adds helper methods that convert from iterators of u16 values to the
  > specific Constraint type. This makes it easy to create constraints like:
  >
  > ```rust
  > // a fixed layout
  > let constraints = Constraint::from_lengths([10, 20, 10]);
  >
  > // a centered layout
  > let constraints = Constraint::from_ratios([(1, 4), (1, 2), (1, 4)]);
  > let constraints = Constraint::from_percentages([25, 50, 25]);
  >
  > // a centered layout with a minimum size
  > let constraints = Constraint::from_mins([0, 100, 0]);
  >
  > // a sidebar / main layout with maximum sizes
  > let constraints = Constraint::from_maxes([30, 200]);
  > ```

### Bug Fixes

- [f69d57c](https://github.com/ratatui-org/ratatui/commit/f69d57c3b59e27b517a5ca1a002af808fee47970) *(rect)* Fix underflow in the `Rect::intersection` method by @Valentin271 in [#678](https://github.com/ratatui-org/ratatui/pull/678)

- [56fc410](https://github.com/ratatui-org/ratatui/commit/56fc4101056e0f631f563f8f2c07646063e650d3) *(block)* Make `inner` aware of title positions by @jan-ferdinand in [#657](https://github.com/ratatui-org/ratatui/pull/657)

  > Previously, when computing the inner rendering area of a block, all
  > titles were assumed to be positioned at the top, which caused the
  > height of the inner area to be miscalculated.

- [ec7b387](https://github.com/ratatui-org/ratatui/commit/ec7b3872b46c6828c88ce7f72308dc67731fca25) *(doc)* Do not access deprecated `Cell::symbol` field in doc example by @rhysd in [#626](https://github.com/ratatui-org/ratatui/pull/626)

- [37c70db](https://github.com/ratatui-org/ratatui/commit/37c70dbb8e19c0fb35ced16b29751933514a441e) *(table)* Add widths parameter to new() by @joshka in [#664](https://github.com/ratatui-org/ratatui/pull/664) [**breaking**]

  > This prevents creating a table that doesn't actually render anything.
  >
  > Fixes:https://github.com/ratatui-org/ratatui/issues/537
  >
  > BREAKING CHANGE:Table::new() now takes an additional widths parameter.

- [1f88da7](https://github.com/ratatui-org/ratatui/commit/1f88da75383f6de76e64e9258fbf38d02ec77af9) *(table)* Fix new clippy lint which triggers on table widths tests by @joshka in [#630](https://github.com/ratatui-org/ratatui/pull/630)

  > * fix(table): new clippy lint in 1.74.0 triggers on table widths tests
  >
  > https://rust-lang.github.io/rust-clippy/master/index.html\#/needless_borrows_for_generic_args
  >
  > * fix(clippy): fix beta lint for .get(0) -> .first()
  >
  > https://rust-lang.github.io/rust-clippy/master/index.html\#/get_first

- [36d8c53](https://github.com/ratatui-org/ratatui/commit/36d8c5364590a559913c40ee5f021b5d8e3466e6) *(table)* Widths() now accepts AsRef<[Constraint]> by @joshka in [#628](https://github.com/ratatui-org/ratatui/pull/628)

  > This allows passing an array, slice or Vec of constraints, which is more
  > ergonomic than requiring this to always be a slice.
  >
  > The following calls now all succeed:
  >
  > ```rust
  > Table::new(rows).widths([Constraint::Length(5), Constraint::Length(5)]);
  > Table::new(rows).widths(&[Constraint::Length(5), Constraint::Length(5)]);
  >
  > // widths could also be computed at runtime
  > let widths = vec![Constraint::Length(5), Constraint::Length(5)];
  > Table::new(rows).widths(widths.clone());
  > Table::new(rows).widths(&widths);
  > ```

- [34d099c](https://github.com/ratatui-org/ratatui/commit/34d099c99af27eacfdde71f9ced255c29e1e001a) *(tabs)* Fixup tests broken by semantic merge conflict by @joshka in [#665](https://github.com/ratatui-org/ratatui/pull/665)

  > Two changes without any line overlap caused the tabs tests to break

- [e4579f0](https://github.com/ratatui-org/ratatui/commit/e4579f0db2b70b59590cae02e994e3736b19a1b3) *(tabs)* Set the default highlight_style by @joshka in [#635](https://github.com/ratatui-org/ratatui/pull/635) [**breaking**]

  > Previously the default highlight_style was set to `Style::default()`,
  > which meant that the highlight style was the same as the normal style.
  > This change sets the default highlight_style to reversed text.
  >
  > BREAKING CHANGE:The `Tab` widget now renders the highlight style as
  > reversed text by default. This can be changed by setting the
  > `highlight_style` field of the `Tab` widget.

- [28ac55b](https://github.com/ratatui-org/ratatui/commit/28ac55bc62e4e14e3ace300633d56791a1d3dea0) *(tabs)* Tab widget now supports custom padding by @rhaskia in [#629](https://github.com/ratatui-org/ratatui/pull/629)

  > The Tab widget now contains padding_left and and padding_right
  > properties. Those values can be set with functions `padding_left()`,
  > `padding_right()`, and `padding()` whic all accept `Into<Line>`.
  >
  > Fixes issue https://github.com/ratatui-org/ratatui/issues/502

- [df0eb1f](https://github.com/ratatui-org/ratatui/commit/df0eb1f8e94752db542ff58e1453f4f8beab17e2) *(terminal)* Insert_before() now accepts lines > terminal height and doesn't add an extra blank line by @danny-burrows in [#596](https://github.com/ratatui-org/ratatui/pull/596)

  > Fixes issue with inserting content with height>viewport_area.height and adds
  > the ability to insert content of height>terminal_height
  >
  > - Adds TestBackend::append_lines() and TestBackend::clear_region() methods to
  >   support testing the changes

- [aaeba27](https://github.com/ratatui-org/ratatui/commit/aaeba2709c09b7373f3781ecd4b0a96b22fc2764) *(uncategorized)* Truncate table when overflow by @YeungKC in [#685](https://github.com/ratatui-org/ratatui/pull/685)

  > This prevents a panic when rendering an empty right aligned and rightmost table cell

- [ffa78aa](https://github.com/ratatui-org/ratatui/commit/ffa78aa67ccd79b9aa1af0d7ccf56a2059d0f519) *(uncategorized)* Add #[must_use] to Style-moving methods by @SOF3 in [#600](https://github.com/ratatui-org/ratatui/pull/600)

- [a2f2bd5](https://github.com/ratatui-org/ratatui/commit/a2f2bd5df53a796c0f2a57bb1b22151e52b5ef03) *(uncategorized)* MSRV is now `1.70.0` by @a-kenji in [#593](https://github.com/ratatui-org/ratatui/pull/593)

### Refactor

- [f767ea7](https://github.com/ratatui-org/ratatui/commit/f767ea7d3766887cb79145103b5aa92e0eabf8f6) *(list)* `start_corner` is now `direction` by @Valentin271 in [#673](https://github.com/ratatui-org/ratatui/pull/673)

  > The previous name `start_corner` did not communicate clearly the intent of the method.
  > A new method `direction` and a new enum `ListDirection` were added.
  >
  > `start_corner` is now deprecated

- [b82451f](https://github.com/ratatui-org/ratatui/commit/b82451fb33f35ae0323a56bb6f962404b076a262) *(examples)* Add vim binding by @Valentin271 in [#688](https://github.com/ratatui-org/ratatui/pull/688)

- [0576a8a](https://github.com/ratatui-org/ratatui/commit/0576a8aa3212c57d288c67592337a3870ae6dafc) *(layout)* To natural reading order by @joshka in [#681](https://github.com/ratatui-org/ratatui/pull/681)

  > Structs and enums at the top of the file helps show the interaction
  > between the types without having to find each type in between longer
  > impl sections.
  >
  > Also moved the try_split function into the Layout impl as an associated
  > function and inlined the `layout::split()` which just called try_split.
  > This makes the code a bit more contained.

- [4be18ab](https://github.com/ratatui-org/ratatui/commit/4be18aba8b535165f03d15450276b2e95a7970eb) *(readme)* Reference awesome-ratatui instead of wiki by @Valentin271 in [#689](https://github.com/ratatui-org/ratatui/pull/689)

  > * refactor(readme): link awesome-ratatui instead of wiki
  >
  > The apps wiki moved to awesome-ratatui
  >
  > * docs(readme): Update README.md

- [7ef0afc](https://github.com/ratatui-org/ratatui/commit/7ef0afcb62198f76321e84d9bb19a8a590a3b649) *(widgets)* Remove unnecessary dynamic dispatch and heap allocation by @rhysd in [#597](https://github.com/ratatui-org/ratatui/pull/597)

- [b282a06](https://github.com/ratatui-org/ratatui/commit/b282a0693289d9d2602b54b639d3701d8c8cc8a8) *(uncategorized)* Remove items deprecated since 0.10 by @Valentin271 in [#691](https://github.com/ratatui-org/ratatui/pull/691) [**breaking**]

  > Remove `Axis::title_style` and `Buffer::set_background` which are deprecated since 0.10

- [7ced7c0](https://github.com/ratatui-org/ratatui/commit/7ced7c0aa3acdaa63ed6add59711614993210ba3) *(uncategorized)* Define struct WrappedLine instead of anonymous tuple by @progval in [#608](https://github.com/ratatui-org/ratatui/pull/608)

  > It makes the type easier to document, and more obvious for users

### Documentation

- [fe632d7](https://github.com/ratatui-org/ratatui/commit/fe632d70cb150264d9af2f79145a1d14a3637f3e) *(sparkline)* Add documentation by @Valentin271 in [#648](https://github.com/ratatui-org/ratatui/pull/648)

- [f4c8de0](https://github.com/ratatui-org/ratatui/commit/f4c8de041d48cec5ea9b3e1f540f57af5a09d7a4) *(chart)* Document chart module by @Valentin271 in [#696](https://github.com/ratatui-org/ratatui/pull/696)

- [1b8b626](https://github.com/ratatui-org/ratatui/commit/1b8b6261e2de29a37b2cd7d6ee8659fb46d3beff) *(examples)* Add animation and FPS counter to colors_rgb by @joshka in [#583](https://github.com/ratatui-org/ratatui/pull/583)

- [2169a0d](https://github.com/ratatui-org/ratatui/commit/2169a0da01e3bd6272e33b9de26a033fcb5f55f2) *(examples)* Add example of half block rendering by @joshka in [#687](https://github.com/ratatui-org/ratatui/pull/687)

  > This is a fun example of how to render big text using half blocks

- [41c44a4](https://github.com/ratatui-org/ratatui/commit/41c44a4af66ba791959f3a298d1b544330b9a164) *(frame)* Add docs about resize events by @joshka in [#697](https://github.com/ratatui-org/ratatui/pull/697)

- [91c67eb](https://github.com/ratatui-org/ratatui/commit/91c67eb1009449e0dfdd29e6ef0132c5254cfbde) *(github)* Update code owners by @orhun in [#666](https://github.com/ratatui-org/ratatui/pull/666)

  > onboard @Valentin271 as maintainer

- [458fa90](https://github.com/ratatui-org/ratatui/commit/458fa9036281e0e6e88bd2ec90c633e499ce547c) *(lib)* Tweak the crate documentation by @orhun in [#659](https://github.com/ratatui-org/ratatui/pull/659)

- [3ec4e24](https://github.com/ratatui-org/ratatui/commit/3ec4e24d00e118a12c8fea888e16ce19b75cf45f) *(list)* Add documentation to the List widget by @Valentin271 in [#669](https://github.com/ratatui-org/ratatui/pull/669)

  > Adds documentation to the List widget and all its sub components like `ListState` and `ListItem`

- [9f37100](https://github.com/ratatui-org/ratatui/commit/9f371000968044e09545d66068c4ed4ea4b35d8a) *(readme)* Update README.md and fix the bug that demo2 cannot run by @rikonaka in [#595](https://github.com/ratatui-org/ratatui/pull/595)

  > Fixes https://github.com/ratatui-org/ratatui/issues/594

- [2a87251](https://github.com/ratatui-org/ratatui/commit/2a87251152432fd99c18864f32874fed2cab2f99) *(security)* Add security policy by @joshka in [#676](https://github.com/ratatui-org/ratatui/pull/676)

  > * docs: Create SECURITY.md
  >
  > * Update SECURITY.md

- [987f7ee](https://github.com/ratatui-org/ratatui/commit/987f7eed4c8bd09e319b504e587eb1f3667ee64b) *(website)* Rename book to website by @joshka in [#661](https://github.com/ratatui-org/ratatui/pull/661)

- [a15c3b2](https://github.com/ratatui-org/ratatui/commit/a15c3b2660bf4102bc881a5bc11959bc136f4a17) *(uncategorized)* Remove deprecated table constructor from breaking changes by @orhun in [#698](https://github.com/ratatui-org/ratatui/pull/698)

- [113b4b7](https://github.com/ratatui-org/ratatui/commit/113b4b7a4ea841fe2ca7b1c153243fec781c3cc0) *(uncategorized)* Rename template links to remove ratatui from name 📚 by @kdheepak in [#690](https://github.com/ratatui-org/ratatui/pull/690)

- [211160c](https://github.com/ratatui-org/ratatui/commit/211160ca165e2ad23b3d4cd9382c6e4869644a9c) *(uncategorized)* Remove simple-tui-rs by @joshka in [#651](https://github.com/ratatui-org/ratatui/pull/651)

  > This has not been recently and doesn't lead to good code

### Styling

- [6a6e9dd](https://github.com/ratatui-org/ratatui/commit/6a6e9dde9dc66ecb6f47f858fd0a67d7dc9eb7d1) *(tabs)* Fix doc formatting by @joshka in [#662](https://github.com/ratatui-org/ratatui/pull/662)

### Miscellaneous Tasks

- [910ad00](https://github.com/ratatui-org/ratatui/commit/910ad00059c3603ba6b1751c95783f974fde88a1) *(rustfmt)* Enable format_code_in_doc_comments by @Valentin271 in [#695](https://github.com/ratatui-org/ratatui/pull/695)

  > This enables more consistently formatted code in doc comments,
  > especially since ratatui heavily uses fluent setters.
  >
  > See https://rust-lang.github.io/rustfmt/?version=v1.6.0#format_code_in_doc_comments

- [d118565](https://github.com/ratatui-org/ratatui/commit/d118565ef60480fba8f2906ede81f875a562cb61) *(table)* Cleanup docs and builder methods by @joshka in [#638](https://github.com/ratatui-org/ratatui/pull/638)

  > - Refactor the `table` module for better top to bottom readability by
  > putting types first and arranging them in a logical order (Table, Row,
  > Cell, other).
  >
  > - Adds new methods for:
  >   - `Table::rows`
  >   - `Row::cells`
  >   - `Cell::new`
  >   - `Cell::content`
  >   - `TableState::new`
  >   - `TableState::selected_mut`
  >
  > - Makes `HighlightSpacing::should_add` pub(crate) since it's an internal
  >   detail.
  >
  > - Adds tests for all the new methods and simple property tests for all
  >   the other setter methods.

- [dd22e72](https://github.com/ratatui-org/ratatui/commit/dd22e721e3aed24538eb08e46e40339cec636bcb) *(uncategorized)* Correct "builder methods" in docs and add `must_use` on widgets setters by @TieWay59 in [#655](https://github.com/ratatui-org/ratatui/pull/655)
  >
  > Fixes #650
  >
  > This PR corrects the "builder methods" expressing to simple `setters`
  > (see #650 #655), and gives a clearer diagnostic notice on setters `must_use`.
  >
  > `#[must_use = "method moves the value of self and returns the modified value"]`
  >
  > Details:docs: Correct wording in docs from builder methods
  >
  >     Add `must_use` on layout setters
  >
  >     chore: add `must_use` on widgets fluent methods
  >
  >         This commit ignored `table.rs` because it is included in other PRs.
  >
  >     test(gauge): fix test

- [18e19f6](https://github.com/ratatui-org/ratatui/commit/18e19f6ce6ae3ce9bd52110ab6cbd4ed4bcca5e6) *(uncategorized)* Fix breaking changes doc versions by @joshka in [#639](https://github.com/ratatui-org/ratatui/pull/639)

  > Moves the layout::new change to unreleasedd section and adds the table change

- [a58cce2](https://github.com/ratatui-org/ratatui/commit/a58cce2dba404fe394bbb298645bf3c40518fe1f) *(uncategorized)* Disable default benchmarking by @mindoodoo in [#598](https://github.com/ratatui-org/ratatui/pull/598)

  > Disables the default benchmarking behaviour for the lib target to fix unrecognized
  > criterion benchmark arguments.
  >
  > See https://bheisler.github.io/criterion.rs/book/faq.html#cargo-bench-gives-unrecognized-option-errors-for-valid-command-line-options for details

### Continuous Integration

- [59b9c32](https://github.com/ratatui-org/ratatui/commit/59b9c32fbc2bc6725bdec42e63216024fab71493) *(codecov)* Adjust threshold and noise settings by @joshka in [#615](https://github.com/ratatui-org/ratatui/pull/615)

  > Fixes https://github.com/ratatui-org/ratatui/issues/612

- [03401cd](https://github.com/ratatui-org/ratatui/commit/03401cd46e6566af4d063bac11efc30f28b5358a) *(uncategorized)* Fix untrusted input in pr check workflow by @joshka in [#680](https://github.com/ratatui-org/ratatui/pull/680)



### New Contributors
* @lyuha made their first contribution in [#378](https://github.com/ratatui-org/ratatui/pull/378)
* @YeungKC made their first contribution in [#685](https://github.com/ratatui-org/ratatui/pull/685)
* @TylerBloom made their first contribution in [#668](https://github.com/ratatui-org/ratatui/pull/668)
* @progval made their first contribution in [#608](https://github.com/ratatui-org/ratatui/pull/608)
* @asomers made their first contribution in [#660](https://github.com/ratatui-org/ratatui/pull/660)
* @rhaskia made their first contribution in [#629](https://github.com/ratatui-org/ratatui/pull/629)
* @jan-ferdinand made their first contribution in [#657](https://github.com/ratatui-org/ratatui/pull/657)
* @SOF3 made their first contribution in [#600](https://github.com/ratatui-org/ratatui/pull/600)
* @danny-burrows made their first contribution in [#596](https://github.com/ratatui-org/ratatui/pull/596)
* @rikonaka made their first contribution in [#595](https://github.com/ratatui-org/ratatui/pull/595)

**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.24.0...v0.25.0


## [v0.24.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.24.0) - 2023-10-23

### Features

- [c6c3f88](https://github.com/ratatui-org/ratatui/commit/c6c3f88a79515a085fb8a96fe150843dab6dd5bc) *(backend)* Implement common traits for `WindowSize` by @orhun in [#586](https://github.com/ratatui-org/ratatui/pull/586)

- [d077903](https://github.com/ratatui-org/ratatui/commit/d0779034e741834aac36b5b7a87c54bd8c50b7f2) *(backend)* Backend provides window_size, add Size struct by @benjajaja in [#276](https://github.com/ratatui-org/ratatui/pull/276)

  > For image (sixel, iTerm2, Kitty...) support that handles graphics in
  > terms of `Rect` so that the image area can be included in layouts.
  >
  > For example: an image is loaded with a known pixel-size, and drawn, but
  > the image protocol has no mechanism of knowing the actual cell/character
  > area that been drawn on. It is then impossible to skip overdrawing the
  > area.
  >
  > Returning the window size in pixel-width / pixel-height, together with
  > colums / rows, it can be possible to account the pixel size of each cell
  > / character, and then known the `Rect` of a given image, and also resize
  > the image so that it fits exactly in a `Rect`.
  >
  > Crossterm and termwiz also both return both sizes from one syscall,
  > while termion does two.
  >
  > Add a `Size` struct for the cases where a `Rect`'s `x`/`y` is unused
  > (always zero).
  >
  > `Size` is not "clipped" for `area < u16::max_value()` like `Rect`. This
  > is why there are `From` implementations between the two.

- [301366c](https://github.com/ratatui-org/ratatui/commit/301366c4fa33524b0634bbd3dcf1abd1a1ebe7c6) *(barchart)* Render charts smaller than 3 lines by @karthago1 in [#532](https://github.com/ratatui-org/ratatui/pull/532)

  > The bar values are not shown if the value width is equal the bar width
  > and the bar is height is less than one line
  >
  > Add an internal structure `LabelInfo` which stores the reserved height
  > for the labels (0, 1 or 2) and also whether the labels will be shown.
  >
  > Fixes ratatui-org#513

- [32e4619](https://github.com/ratatui-org/ratatui/commit/32e461953c8c9231edeef65c410b295916f26f3e) *(block)* Allow custom symbols for borders by @joshka in [#529](https://github.com/ratatui-org/ratatui/pull/529) [**breaking**]

  > Adds a new `Block::border_set` method that allows the user to specify
  > the symbols used for the border.
  >
  > Added two new border types: `BorderType::QuadrantOutside` and
  > `BorderType::QuadrantInside`. These are used to draw borders using the
  > unicode quadrant characters (which look like half block "pixels").
  >
  > QuadrantOutside:```
  > ▛▀▀▜
  > ▌  ▐
  > ▙▄▄▟
  > ```
  >
  > QuadrantInside:```
  > ▗▄▄▖
  > ▐  ▌
  > ▝▀▀▘
  > ```
  >
  > Fixes:https://github.com/ratatui-org/ratatui/issues/528
  >
  > BREAKING CHANGES:
  > - BorderType::to_line_set is renamed to to_border_set
  > - BorderType::line_symbols is renamed to border_symbols

- [4541336](https://github.com/ratatui-org/ratatui/commit/45413365146ede5472dc28e0ee1970d245e2fa02) *(canvas)* Implement half block marker by @joshka in [#550](https://github.com/ratatui-org/ratatui/pull/550)

  > * feat(canvas): implement half block marker
  >
  > A useful technique for the terminal is to use half blocks to draw a grid
  > of "pixels" on the screen. Because we can set two colors per cell, and
  > because terminal cells are about twice as tall as they are wide, we can
  > draw a grid of half blocks that looks like a grid of square pixels.
  >
  > This commit adds a new `HalfBlock` marker that can be used in the Canvas
  > widget and the associated HalfBlockGrid.
  >
  > Also updated demo2 to use the new marker as it looks much nicer.
  >
  > Adds docs for many of the methods and structs on canvas.
  >
  > Changes the grid resolution method to return the pixel count
  > rather than the index of the last pixel.
  > This is an internal detail with no user impact.

- [be55a5f](https://github.com/ratatui-org/ratatui/commit/be55a5fbcdffc4fd6aeb7edffa32f6e6c942a41e) *(examples)* Add demo2 example by @joshka in [#500](https://github.com/ratatui-org/ratatui/pull/500)

- [082cbcb](https://github.com/ratatui-org/ratatui/commit/082cbcbc501d4284dc7e142227f9e04ef17da61d) *(frame)* Remove generic Backend parameter by @joshka in [#530](https://github.com/ratatui-org/ratatui/pull/530) [**breaking**]

  > This change simplifys UI code that uses the Frame type. E.g.:
  >
  > ```rust
  > fn draw<B: Backend>(frame: &mut Frame<B>) {
  >     // ...
  > }
  > ```
  >
  > Frame was generic over Backend because it stored a reference to the
  > terminal in the field. Instead it now directly stores the viewport area
  > and current buffer. These are provided at creation time and are valid
  > for the duration of the frame.
  >
  > BREAKING CHANGE:Frame is no longer generic over Backend. Code that
  > accepted `Frame<Backend>` will now need to accept `Frame`. To migrate
  > existing code, remove any generic parameters from code that uses an
  > instance of a Frame. E.g. the above code becomes:
  >
  > ```rust
  > fn draw(frame: &mut Frame) {
  >     // ...
  > }
  > ```

- [d67fa2c](https://github.com/ratatui-org/ratatui/commit/d67fa2c00d6d6125eeefa0eeeb032664dae9a4de) *(line)* Add `Line::raw` constructor by @orhun in [#511](https://github.com/ratatui-org/ratatui/pull/511)

  > * feat(line): add `Line::raw` constructor
  >
  > There is already `Span::raw` and `Text::raw` methods
  > and this commit simply adds `Line::raw` method for symmetry.
  >
  > Multi-line content is converted to multiple spans with the new line removed

- [cbf86da](https://github.com/ratatui-org/ratatui/commit/cbf86da0e7e4a2d99ace8df68854de74157a665a) *(rect)* Add is_empty() to simplify some common checks by @joshka in [#534](https://github.com/ratatui-org/ratatui/pull/534)

  > - add `Rect::is_empty()` that checks whether either height or width == 0
  > - refactored `Rect` into layout/rect.rs from layout.rs. No public API change as
  >    the module is private and the type is re-exported under the `layout` module.

- [15641c8](https://github.com/ratatui-org/ratatui/commit/15641c8475b7596c97a0affce0d6082c4b9586c2) *(uncategorized)* Add `buffer_mut` method on `Frame` ✨ by @kdheepak in [#548](https://github.com/ratatui-org/ratatui/pull/548)

### Bug Fixes

- [638d596](https://github.com/ratatui-org/ratatui/commit/638d596a3b7aec723a2354cf0e261b207ac412f8) *(layout)* Use LruCache for layout cache by @marianomarciello in [#487](https://github.com/ratatui-org/ratatui/pull/487)

  > The layout cache now uses a LruCache with default size set to 16 entries.
  > Previously the cache was backed by a HashMap, and was able to grow
  > without bounds as a new entry was added for every new combination of
  > layout parameters.
  >
  > - Added a new method (`layout::init_cache(usize)`) that allows the cache
  > size to be changed if necessary. This will only have an effect if it is called
  > prior to any calls to `layout::split()` as the cache is wrapped in a `OnceLock`

- [8d507c4](https://github.com/ratatui-org/ratatui/commit/8d507c43fa866ab4c0eda9fd169f307fba2a1109) *(backend)* Add feature flag for underline-color by @joshka in [#570](https://github.com/ratatui-org/ratatui/pull/570)

  > Windows 7 doesn't support the underline color attribute, so we need to
  > make it optional. This commit adds a feature flag for the underline
  > color attribute - it is enabled by default, but can be disabled by
  > passing `--no-default-features` to cargo.
  >
  > We could specically check for Windows 7 and disable the feature flag
  > automatically, but I think it's better for this check to be done by the
  > crossterm crate, since it's the one that actually knows about the
  > underlying terminal.
  >
  > To disable the feature flag in an application that supports Windows 7,
  > add the following to your Cargo.toml:
  >
  > ```toml
  > ratatui = { version = "0.24.0", default-features = false, features = ["crossterm"] }
  > ```
  >
  > Fixes https://github.com/ratatui-org/ratatui/issues/555

- [c3155a2](https://github.com/ratatui-org/ratatui/commit/c3155a24895ec4dfb1a8e580fb9ee3d31e9af139) *(barchart)* Add horizontal labels by @Valentin271 in [#518](https://github.com/ratatui-org/ratatui/pull/518)

  > Labels were missed in the initial implementation of the horizontal
  > mode for the BarChart widget. This adds them.
  >
  > Fixes https://github.com/ratatui-org/ratatui/issues/499

- [c5ea656](https://github.com/ratatui-org/ratatui/commit/c5ea656385843c880b3bef45dccbe8ea57431d10) *(barchart)* Avoid divide by zero in rendering by @joshka in [#525](https://github.com/ratatui-org/ratatui/pull/525)
  >
  > Fixes:https://github.com/ratatui-org/ratatui/issues/521

- [c9b8e7c](https://github.com/ratatui-org/ratatui/commit/c9b8e7cf412de235082f1fcd1698468c4b1b6171) *(barchart)* Render value labels with unicode correctly by @karthago1 in [#515](https://github.com/ratatui-org/ratatui/pull/515)

  > An earlier change introduced a bug where the width of value labels with
  > unicode characters was incorrectly using the string length in bytes
  > instead of the unicode character count. This reverts the earlier change.

- [c8ab2d5](https://github.com/ratatui-org/ratatui/commit/c8ab2d59087f5b475ecf6ffa31b89ce24b6b1d28) *(chart)* Use graph style for top line by @aatukaj in [#462](https://github.com/ratatui-org/ratatui/pull/462)

  > A bug in the rendering caused the top line of the chart to be rendered
  > using the style of the chart, instead of the dataset style. This is
  > fixed by only setting the style for the width of the text, and not the
  > entire row.
  >
  > Fixes:https://github.com/ratatui-org/ratatui/issues/379

- [0c7d547](https://github.com/ratatui-org/ratatui/commit/0c7d547db196a7cf65a6bf8cde74bd908407a3ff) *(docs)* Don't fail rustdoc due to termion by @joshka in [#503](https://github.com/ratatui-org/ratatui/pull/503)

  > Windows cannot compile termion, so it is not included in the docs.
  > Rustdoc will fail if it cannot find a link, so the docs fail to build
  > on windows.
  >
  > This replaces the link to TermionBackend with one that does not fail
  > during checks.
  >
  > Fixes https://github.com/ratatui-org/ratatui/issues/498

- [0c52ff4](https://github.com/ratatui-org/ratatui/commit/0c52ff431a1eedb0e38b5c8fb6623d4da17fa97e) *(gauge)* Fix gauge widget colors by @joshka in [#572](https://github.com/ratatui-org/ratatui/pull/572)

  > The background colors of the gauge had a workaround for the issue we had
  > with VHS / TTYD rendering the background color of the gauge. This
  > workaround is no longer necessary in the updated versions of VHS / TTYD.
  >
  > Fixes https://github.com/ratatui-org/ratatui/issues/501

- [11076d0](https://github.com/ratatui-org/ratatui/commit/11076d0af3a76229af579fb40684fdd37df172dd) *(rect)* Fix arithmetic overflow edge cases by @joshka in [#543](https://github.com/ratatui-org/ratatui/pull/543)

  > Fixes https://github.com/ratatui-org/ratatui/issues/258

- [21303f2](https://github.com/ratatui-org/ratatui/commit/21303f21672de1405135bb785497c30150644078) *(rect)* Prevent overflow in inner() and area() by @HeeillWang in [#523](https://github.com/ratatui-org/ratatui/pull/523)

- [ebd3680](https://github.com/ratatui-org/ratatui/commit/ebd3680a471d96ae1d8f52cd9e4a8a80c142d060) *(stylize)* Add Stylize impl for String by @joshka in [#466](https://github.com/ratatui-org/ratatui/pull/466) [**breaking**]

  > Although the `Stylize` trait is already implemented for `&str` which
  > extends to `String`, it is not implemented for `String` itself. This
  > commit adds an impl of Stylize that returns a Span<'static> for `String`
  > so that code can call Stylize methods on temporary `String`s.
  >
  > E.g. the following now compiles instead of failing with a compile error
  > about referencing a temporary value:
  >
  >     let s = format!("hello {name}!", "world").red();
  >
  > BREAKING CHANGE:This may break some code that expects to call Stylize
  > methods on `String` values and then use the String value later. This
  > will now fail to compile because the String is consumed by set_style
  > instead of a slice being created and consumed.
  >
  > This can be fixed by cloning the `String`. E.g.:
  >
  >     let s = String::from("hello world");
  >     let line = Line::from(vec![s.red(), s.green()]); // fails to compile
  >     let line = Line::from(vec![s.clone().red(), s.green()]); // works

### Refactor

- [2fd85af](https://github.com/ratatui-org/ratatui/commit/2fd85af33c5cb7c04286e4e4198a939b4857eadc) *(barchart)* Simplify internal implementation by @karthago1 in [#544](https://github.com/ratatui-org/ratatui/pull/544)

  > Replace `remove_invisible_groups_and_bars` with `group_ticks`
  > `group_ticks` calculates the visible bar length in ticks. (A cell contains 8 ticks).
  >
  > It is used for 2 purposes:
  > 1. to get the bar length in ticks for rendering
  > 2. since it delivers only the values of the visible bars, If we zip these values
  >    with the groups and bars, then we will filter out the invisible groups and bars

### Documentation

- [0c68ebe](https://github.com/ratatui-org/ratatui/commit/0c68ebed4f63a595811006e0af221b11a83780cf) *(block)* Add documentation to Block by @Valentin271 in [#469](https://github.com/ratatui-org/ratatui/pull/469)

- [0fe7385](https://github.com/ratatui-org/ratatui/commit/0fe738500cd461aeafa0a63b37ed6250777f3599) *(gauge)* Add docs for `Gauge` and `LineGauge` by @Valentin271 in [#514](https://github.com/ratatui-org/ratatui/pull/514)

- [27c5637](https://github.com/ratatui-org/ratatui/commit/27c56376756b854db6d2fd8939419bd8578f8a90) *(readme)* Fix links to CONTRIBUTING.md and BREAKING-CHANGES.md by @hueblu in [#577](https://github.com/ratatui-org/ratatui/pull/577)

- [1947c58](https://github.com/ratatui-org/ratatui/commit/1947c58c60127ee7d1a72bcd408ee23062b8c4ec) *(backend)* Improve backend module docs by @joshka in [#489](https://github.com/ratatui-org/ratatui/pull/489)

- [e098731](https://github.com/ratatui-org/ratatui/commit/e098731d6c1a68a0319d544301ac91cf2d05ccb2) *(barchart)* Add documentation to `BarChart` by @Valentin271 in [#449](https://github.com/ratatui-org/ratatui/pull/449)

  > Add documentation to the `BarChart` widgets and its sub-modules.

- [17797d8](https://github.com/ratatui-org/ratatui/commit/17797d83dab07dc6b76e7a3838e3e17fc3c94711) *(canvas)* Add support note for Braille marker by @joshka in [#472](https://github.com/ratatui-org/ratatui/pull/472)

- [3cf0b83](https://github.com/ratatui-org/ratatui/commit/3cf0b83bda5deee18b8a1233acec0a21fde1f5f4) *(color)* Document true color support by @joshka in [#477](https://github.com/ratatui-org/ratatui/pull/477)

  > * refactor(style): move Color to separate color mod
  >
  > * docs(color): document true color support

- [e5caf17](https://github.com/ratatui-org/ratatui/commit/e5caf170c8c304b952cbff7499fd4da17ab154ea) *(custom_widget)* Make button sticky when clicking with mouse by @kdheepak in [#561](https://github.com/ratatui-org/ratatui/pull/561)

- [ad2dc56](https://github.com/ratatui-org/ratatui/commit/ad2dc5646dae04fa5502e677182cdeb0c3630cce) *(examples)* Update examples readme by @joshka in [#576](https://github.com/ratatui-org/ratatui/pull/576)

  > remove VHS bug info, tweak colors_rgb image, update some of the instructions. add demo2

- [b61f65b](https://github.com/ratatui-org/ratatui/commit/b61f65bc20918380f2854253d4301ea804fc7437) *(examples)* Udpate theme to Aardvark Blue by @joshka in [#574](https://github.com/ratatui-org/ratatui/pull/574)

  > This is a nicer theme that makes the colors pop

- [61af0d9](https://github.com/ratatui-org/ratatui/commit/61af0d99069ec99b3075cd499ede13cc2143401f) *(examples)* Make custom widget example into a button by @joshka in [#539](https://github.com/ratatui-org/ratatui/pull/539)

  > The widget also now supports mouse

- [6b8725f](https://github.com/ratatui-org/ratatui/commit/6b8725f09173f418e9f17933d8ef8c943af444de) *(examples)* Add colors_rgb example by @joshka in [#476](https://github.com/ratatui-org/ratatui/pull/476)

- [5c785b2](https://github.com/ratatui-org/ratatui/commit/5c785b22709fb64a0982722e4f6d0021ccf621b2) *(examples)* Move example gifs to github by @joshka

  > - A new orphan branch named "images" is created to store the example
  >   images

- [ca9bcd3](https://github.com/ratatui-org/ratatui/commit/ca9bcd3156f55cd2df4edf003aa1401abbed9b12) *(examples)* Add descriptions and update theme by @joshka

  > - Use the OceanicMaterial consistently in examples

- [0e573cd](https://github.com/ratatui-org/ratatui/commit/0e573cd6c7305445a298ef596651db02212a9dcb) *(github)* Update code owners by @orhun in [#587](https://github.com/ratatui-org/ratatui/pull/587)

- [080a05b](https://github.com/ratatui-org/ratatui/commit/080a05bbd3357cde3f0a02721a0f7f1aa206206b) *(paragraph)* Add docs for alignment fn by @DreadedHippy

- [b070008](https://github.com/ratatui-org/ratatui/commit/b07000835fa1b5b1156583b4a0f0269f56fffa41) *(readme)* Fix link to demo2 image by @joshka in [#589](https://github.com/ratatui-org/ratatui/pull/589)

- [1e20475](https://github.com/ratatui-org/ratatui/commit/1e204750617acccf952b1845a3c7ce86e2b90cf7) *(stylize)* Improve docs for style shorthands by @joshka in [#491](https://github.com/ratatui-org/ratatui/pull/491)

  > The Stylize trait was introduced in 0.22 to make styling less verbose.
  > This adds a bunch of documentation comments to the style module and
  > types to make this easier to discover.

- [dd9a8df](https://github.com/ratatui-org/ratatui/commit/dd9a8df03ab09d2381ef5ddd0c2b6ef5517b44df) *(table)* Add documentation for `block` and `header` methods of the `Table` widget by @DreadedHippy in [#505](https://github.com/ratatui-org/ratatui/pull/505)

- [232be80](https://github.com/ratatui-org/ratatui/commit/232be80325cb899359ea1389516c421e57bc9cce) *(table)* Add documentation for `Table::new()` by @DreadedHippy in [#471](https://github.com/ratatui-org/ratatui/pull/471)

- [3bda372](https://github.com/ratatui-org/ratatui/commit/3bda37284781b62560cde2a7fa774211f651ec25) *(tabs)* Add documentation to `Tabs` by @Valentin271 in [#535](https://github.com/ratatui-org/ratatui/pull/535)

- [42f8169](https://github.com/ratatui-org/ratatui/commit/42f816999e2cd573c498c4885069a5523707663c) *(terminal)* Add docs for terminal module by @joshka in [#486](https://github.com/ratatui-org/ratatui/pull/486)

  > - moves the impl Terminal block up to be closer to the type definition

- [28e7fd4](https://github.com/ratatui-org/ratatui/commit/28e7fd4bc58edf537b66b69095691ae06872acd8) *(terminal)* Fix doc comment by @a-kenji in [#452](https://github.com/ratatui-org/ratatui/pull/452)

- [51fdcbe](https://github.com/ratatui-org/ratatui/commit/51fdcbe7e936b3af3ee6a8ae8fee43df31aab27c) *(title)* Add documentation to title by @Valentin271 in [#443](https://github.com/ratatui-org/ratatui/pull/443)

  > This adds documentation for Title and Position

- [d4976d4](https://github.com/ratatui-org/ratatui/commit/d4976d4b63d4a17adb31bbe853a82109e2caaf1b) *(widgets)* Update the list of available widgets by @joshka in [#496](https://github.com/ratatui-org/ratatui/pull/496)

- [6c7bef8](https://github.com/ratatui-org/ratatui/commit/6c7bef8d111bbc3ecfe228b14002c5db9634841c) *(uncategorized)* Replace colons with dashes in README.md for consistency in [#566](https://github.com/ratatui-org/ratatui/pull/566)

- [88ae348](https://github.com/ratatui-org/ratatui/commit/88ae3485c2c540b4ee630ab13e613e84efa7440a) *(uncategorized)* Update `Frame` docstring to remove reference to generic backend by @kdheepak in [#564](https://github.com/ratatui-org/ratatui/pull/564)

- [089f8ba](https://github.com/ratatui-org/ratatui/commit/089f8ba66a50847780c4416b9b8833778a95e558) *(uncategorized)* Add double quotes to instructions for features by @kdheepak in [#560](https://github.com/ratatui-org/ratatui/pull/560)

- [346e7b4](https://github.com/ratatui-org/ratatui/commit/346e7b4f4d53063ee13b04758b1b994e4f14e51c) *(uncategorized)* Add summary to breaking changes by @joshka in [#549](https://github.com/ratatui-org/ratatui/pull/549)

- [401a7a7](https://github.com/ratatui-org/ratatui/commit/401a7a7f7111989d7dda11524b211a488483e732) *(uncategorized)* Improve clarity in documentation for `Frame` and `Terminal` 📚 by @kdheepak in [#545](https://github.com/ratatui-org/ratatui/pull/545)

- [e35e413](https://github.com/ratatui-org/ratatui/commit/e35e4135c9080389baa99e13814aace7784d9cb3) *(uncategorized)* Fix terminal comment by @kdheepak in [#547](https://github.com/ratatui-org/ratatui/pull/547)

- [8ae4403](https://github.com/ratatui-org/ratatui/commit/8ae4403b63a82d353b224c898b15249f30215476) *(uncategorized)* Fix `Terminal` docstring by @kdheepak in [#546](https://github.com/ratatui-org/ratatui/pull/546)

- [9cfb133](https://github.com/ratatui-org/ratatui/commit/9cfb133a981c070a27342d78f4b9451673d8b349) *(uncategorized)* Document alpha release process by @joshka in [#542](https://github.com/ratatui-org/ratatui/pull/542)

  > Fixes https://github.com/ratatui-org/ratatui/issues/412

- [4548a9b](https://github.com/ratatui-org/ratatui/commit/4548a9b7e22b07c1bd6839280c44123b8679589d) *(uncategorized)* Add BREAKING-CHANGES.md by @joshka in [#538](https://github.com/ratatui-org/ratatui/pull/538)

  > Document the breaking changes in each version. This document is
  > manually curated by summarizing the breaking changes in the changelog.

- [c0991cc](https://github.com/ratatui-org/ratatui/commit/c0991cc576b3ade02494cb33fd7c290aba55bfb8) *(uncategorized)* Make library and README consistent by @joshka in [#526](https://github.com/ratatui-org/ratatui/pull/526)

  > * docs: make library and README consistent
  >
  > Generate the bulk of the README from the library documentation, so that
  > they are consistent using cargo-rdme.
  >
  > - Removed the Contributors section, as it is redundant with the github
  >   contributors list.
  > - Removed the info about the other backends and replaced it with a
  >   pointer to the documentation.
  > - add docsrs example, vhs tape and images that will end up in the README
  >
  > Fixes:https://github.com/ratatui-org/ratatui/issues/512

- [1414fbc](https://github.com/ratatui-org/ratatui/commit/1414fbcc05b4dfd7706cc68fcaba7d883e22f869) *(uncategorized)* Import prelude::* in doc examples by @joshka in [#490](https://github.com/ratatui-org/ratatui/pull/490)

  > This commit adds `prelude::*` all doc examples and widget::* to those
  > that need it. This is done to highlight the use of the prelude and
  > simplify the examples.
  >
  > - Examples in Type and module level comments show all imports and use
  >   `prelude::*` and `widget::*` where possible.
  > - Function level comments hide imports unless there are imports other
  >   than `prelude::*` and `widget::*`.

- [74c5244](https://github.com/ratatui-org/ratatui/commit/74c5244be12031e372797c3c7949914552293f5c) *(uncategorized)* Add logo and favicon to docs.rs page by @joshka in [#473](https://github.com/ratatui-org/ratatui/pull/473)

- [927a5d8](https://github.com/ratatui-org/ratatui/commit/927a5d8251a7947446100f4bb4d7a8e3ec2ad962) *(uncategorized)* Fix documentation lint warnings by @Valentin271 in [#450](https://github.com/ratatui-org/ratatui/pull/450)

- [eda2fb7](https://github.com/ratatui-org/ratatui/commit/eda2fb7077dcf0b158d1a69d2725aeb9464162be) *(uncategorized)* Use ratatui 📚 by @kdheepak in [#446](https://github.com/ratatui-org/ratatui/pull/446)

### Testing

- [ea70bff](https://github.com/ratatui-org/ratatui/commit/ea70bffe5d3ec68dcf9eff015437d2474c08f855) *(barchart)* Add benchmarks by @Valentin271 in [#455](https://github.com/ratatui-org/ratatui/pull/455)

- [94af2a2](https://github.com/ratatui-org/ratatui/commit/94af2a29e10248ed709bbc8a7bf2f569894abc62) *(buffer)* Allow with_lines to accept Vec<Into<Line>> by @joshka in [#494](https://github.com/ratatui-org/ratatui/pull/494)

  > This allows writing unit tests without having to call set_style on the
  > expected buffer.
  >
  > E.g.:```rust
  > use crate::style::Stylize;
  > let mut buf = Buffer::empty(Rect::new(0, 0, 10, 10));
  > buf.set_string(0, 0, "foo", Style::new().red());
  > buf.set_string(0, 1, "bar", Style::new().blue());
  > assert_eq!(buf, Buffer::with_lines(vec!["foo".red(), "bar".blue()]));
  > ```
  >
  > Inspired by https://github.com/ratatui-org/ratatui/issues/493#issuecomment-1714844468

### Miscellaneous Tasks

- [82b40be](https://github.com/ratatui-org/ratatui/commit/82b40be4ab8aa735070dff1681c3d711147792e1) *(ci)* Improve checking the PR title by @orhun in [#464](https://github.com/ratatui-org/ratatui/pull/464)

  > - Use [`action-semantic-pull-request`](https://github.com/amannn/action-semantic-pull-request)
  > - Allow only reading the PR contents
  > - Enable merge group

- [6cbdb06](https://github.com/ratatui-org/ratatui/commit/6cbdb06fd86858849d2454d09393a8e43c10741f) *(examples)* Refactor some examples by @rhysd in [#578](https://github.com/ratatui-org/ratatui/pull/578)

  > * chore(examples): Simplify timeout calculation with `Duration::saturating_sub`

- [12f9291](https://github.com/ratatui-org/ratatui/commit/12f92911c74211a22c6c142762ccb459d399763b) *(github)* Create dependabot.yml by @joshka in [#575](https://github.com/ratatui-org/ratatui/pull/575)

  > * chore: Create dependabot.yml
  >
  > * Update .github/dependabot.yml

- [3a57e76](https://github.com/ratatui-org/ratatui/commit/3a57e76ed18b93f0bcee264d818a469920ce70db) *(github)* Add contact links for issues by @orhun in [#567](https://github.com/ratatui-org/ratatui/pull/567)

- [5498a88](https://github.com/ratatui-org/ratatui/commit/5498a889ae8bd4ccb51b04d3a848dd2f58935906) *(spans)* Remove deprecated `Spans` type by @joshka in [#426](https://github.com/ratatui-org/ratatui/pull/426)

  > The `Spans` type (plural, not singular) was replaced with a more ergonomic `Line` type
  > in Ratatui v0.21.0 and marked deprecated byt left for backwards compatibility. This is now
  > removed.
  >
  > - `Line` replaces `Spans`
  > - `Buffer::set_line` replaces `Buffer::set_spans`

- [fbf1a45](https://github.com/ratatui-org/ratatui/commit/fbf1a451c85871db598cf1df2ad9a50edbe07cd2) *(uncategorized)* Simplify constraints by @joshka in [#556](https://github.com/ratatui-org/ratatui/pull/556)

  > Use bare arrays rather than array refs / Vecs for all constraint
  > examples.
  >
  > Ref:https://github.com/ratatui-org/ratatui-book/issues/94

- [a7bf4b3](https://github.com/ratatui-org/ratatui/commit/a7bf4b3f36f3281017d112ac1a67af7e82308261) *(uncategorized)* Use modern modules syntax by @joshka in [#492](https://github.com/ratatui-org/ratatui/pull/492)

  > Move xxx/mod.rs to xxx.rs

- [af36282](https://github.com/ratatui-org/ratatui/commit/af36282df5d8dd1b4e6b32bba0539dba3382c23c) *(uncategorized)* Only run check pr action on pull_request_target events by @kdheepak in [#485](https://github.com/ratatui-org/ratatui/pull/485)

- [322e46f](https://github.com/ratatui-org/ratatui/commit/322e46f15d8326d18c951be4c57e3b47005285bc) *(uncategorized)* Prevent PR merge with do not merge labels ♻️ by @kdheepak in [#484](https://github.com/ratatui-org/ratatui/pull/484)

- [983ea7f](https://github.com/ratatui-org/ratatui/commit/983ea7f7a5371dd608891a0e2a7444a16e9fdc54) *(uncategorized)* Fix check for if breaking change label should be added ♻️ by @kdheepak in [#483](https://github.com/ratatui-org/ratatui/pull/483)

- [384e616](https://github.com/ratatui-org/ratatui/commit/384e616231c1579328e7a4ba1a7130f624753ad1) *(uncategorized)* Add a check for if breaking change label should be added ♻️ by @kdheepak in [#481](https://github.com/ratatui-org/ratatui/pull/481)

- [5f6aa30](https://github.com/ratatui-org/ratatui/commit/5f6aa30be54ea5dfcef730d709707a814e64deee) *(uncategorized)* Check documentation lint by @Valentin271 in [#454](https://github.com/ratatui-org/ratatui/pull/454)

- [47ae602](https://github.com/ratatui-org/ratatui/commit/47ae602df43674928f10016e2edc97c550b01ba2) *(uncategorized)* Check that PR title matches conventional commit guidelines ♻️ by @kdheepak in [#459](https://github.com/ratatui-org/ratatui/pull/459)

- [28c6157](https://github.com/ratatui-org/ratatui/commit/28c61571e8a90345a866285a6f8459b24b70578a) *(uncategorized)* Add documentation guidelines by @Valentin271 in [#447](https://github.com/ratatui-org/ratatui/pull/447)

### Continuous Integration

- [343c6cd](https://github.com/ratatui-org/ratatui/commit/343c6cdc47c4fe38e64633d982aa413be356fb90) *(lint)* Move formatting and doc checks first by @joshka in [#465](https://github.com/ratatui-org/ratatui/pull/465)

  > Putting the formatting and doc checks first to ensure that more critical
  > errors are caught first (e.g. a conventional commit error or typo should
  > not prevent the formatting and doc checks from running).

- [c95a75c](https://github.com/ratatui-org/ratatui/commit/c95a75c5d5e0370c98a2a37bcbd65bde996b2306) *(makefile)* Remove termion dependency from doc lint by @joshka in [#470](https://github.com/ratatui-org/ratatui/pull/470)

  > Only build termion on non-windows targets

- [b996102](https://github.com/ratatui-org/ratatui/commit/b996102837dad7c77710bcbbc524c6e9691bd96f) *(makefile)* Add format target by @joshka in [#468](https://github.com/ratatui-org/ratatui/pull/468)

  > - add format target to Makefile.toml that actually fixes the formatting
  > - rename fmt target to lint-format
  > - rename style-check target to lint-style
  > - rename typos target to lint-typos
  > - rename check-docs target to lint-docs
  > - add section to CONTRIBUTING.md about formatting

- [572df75](https://github.com/ratatui-org/ratatui/commit/572df758ba1056759aa6f79c9e975854d27331db) *(uncategorized)* Put commit id first in changelog by @joshka in [#463](https://github.com/ratatui-org/ratatui/pull/463)

- [878b6fc](https://github.com/ratatui-org/ratatui/commit/878b6fc258110b41e85833c35150d7dfcedf31ca) *(uncategorized)* Ignore benches from code coverage by @joshka in [#461](https://github.com/ratatui-org/ratatui/pull/461)



### New Contributors
* @hueblu made their first contribution in [#577](https://github.com/ratatui-org/ratatui/pull/577)
* @HeeillWang made their first contribution in [#523](https://github.com/ratatui-org/ratatui/pull/523)
* @DreadedHippy made their first contribution in [#505](https://github.com/ratatui-org/ratatui/pull/505)
* @marianomarciello made their first contribution in [#487](https://github.com/ratatui-org/ratatui/pull/487)
* @aatukaj made their first contribution in [#462](https://github.com/ratatui-org/ratatui/pull/462)

**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.23.0...v0.24.0


## [v0.23.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.23.0) - 2023-08-28

### Features

- [0dca6a6](https://github.com/ratatui-org/ratatui/commit/0dca6a689a7af640c5de8f7c87c2f1e03f0adf25) *(barchart)* Add direction attribute. (horizontal bars support) by @karthago1 in [#325](https://github.com/ratatui-org/ratatui/pull/325)

  > * feat(barchart): Add direction attribute
  >
  > Enable rendring the bars horizontally. In some cases this allow us to
  > make more efficient use of the available space.

- [e4bcf78](https://github.com/ratatui-org/ratatui/commit/e4bcf78afabe6b06970c51b4284246e345002cf5) *(cell)* Add voluntary skipping capability for sixel by @benjajaja in [#215](https://github.com/ratatui-org/ratatui/pull/215)

  > > Sixel is a bitmap graphics format supported by terminals.
  > > "Sixel mode" is entered by sending the sequence ESC+Pq.
  > > The "String Terminator" sequence ESC+\ exits the mode.
  >
  > The graphics are then rendered with the top left positioned at the
  > cursor position.
  >
  > It is actually possible to render sixels in ratatui with just
  > `buf.get_mut(x, y).set_symbol("^[Pq ... ^[\")`. But any buffer covering
  > the "image area" will overwrite the graphics. This is most likely the same
  > buffer, even though it consists of empty characters `' '`, except for
  > the top-left character that starts the sequence.
  >
  > Thus, either the buffer or cells must be specialized to avoid drawing
  > over the graphics. This patch specializes the `Cell` with a
  > `set_skip(bool)` method, based on James' patch:
  > https://github.com/TurtleTheSeaHobo/tui-rs/tree/sixel-support
  > I unsuccessfully tried specializing the `Buffer`, but as far as I can tell
  > buffers get merged all the way "up" and thus skipping must be set on the
  > Cells. Otherwise some kind of "skipping area" state would be required,
  > which I think is too complicated.
  >
  > Having access to the buffer now it is possible to skipp all cells but the
  > first one which can then `set_symbol(sixel)`. It is up to the user to
  > deal with the graphics size and buffer area size. It is possible to get
  > the terminal's font size in pixels with a syscall.
  >
  > An image widget for ratatui that uses this `skip` flag is available at
  > https://github.com/benjajaja/ratatu-image.

- [4d70169](https://github.com/ratatui-org/ratatui/commit/4d70169bef86898d331f46013ff72ef6d1c275ed) *(list)* Add option to always allocate the "selection" column width by @hasezoey in [#394](https://github.com/ratatui-org/ratatui/pull/394)

  > * feat(list): add option to always allocate the "selection" column width
  >
  > Before this option was available, selecting a item in a list when nothing was selected
  > previously made the row layout change (the same applies to unselecting) by adding the width
  > of the "highlight symbol" in the front of the list, this option allows to configure this
  > behavior.
  >
  > * style: change "highlight_spacing" doc comment to use inline code-block for reference

- [aad164a](https://github.com/ratatui-org/ratatui/commit/aad164a5311b0a6d6d3f752a87ed385d5f0c1962) *(release)* Add automated nightly releases by @orhun in [#359](https://github.com/ratatui-org/ratatui/pull/359)

  > * feat(release): add automated nightly releases
  >
  > * refactor(release): rename the alpha workflow
  >
  > * refactor(release): simplify the release calculation

- [1727fa5](https://github.com/ratatui-org/ratatui/commit/1727fa5120fa4bfcddd57484e532b2d5da88bc73) *(scrollbar)* Add optional track symbol by @a-kenji in [#360](https://github.com/ratatui-org/ratatui/pull/360) [**breaking**]

  > The track symbol is now optional, simplifying composition with other
  > widgets.
  >
  > BREAKING_CHANGE:The `track_symbol` needs to be set in the following way
  >
  > now:```
  > let scrollbar = Scrollbar::default().track_symbol(Some("-"));
  > ```

- [7748720](https://github.com/ratatui-org/ratatui/commit/77487209634f26da32bc59d9280769d80cc7c25c) *(table)* Add support for line alignment in the table widget by @jkcdarunday in [#392](https://github.com/ratatui-org/ratatui/pull/392)

  > * feat(table): enforce line alignment in table render
  >
  > * test(table): add table alignment render test

- [f63ac72](https://github.com/ratatui-org/ratatui/commit/f63ac72305f80062727d81996f9bdb523e666099) *(widgets::table)* Add option to always allocate the "selection" constraint by @hasezoey in [#375](https://github.com/ratatui-org/ratatui/pull/375)

  > * feat(table): add option to configure selection layout changes
  >
  > Before this option was available, selecting a row in the table when no row was selected
  > previously made the tables layout change (the same applies to unselecting) by adding the width
  > of the "highlight symbol" in the front of the first column, this option allows to configure this
  > behavior.
  >
  > * refactor(table): refactor "get_columns_widths" to return (x, width)
  >
  > and "render" to make use of that
  >
  > * refactor(table): refactor "get_columns_widths" to take in a selection_width instead of a boolean
  >
  > also refactor "render" to make use of this change
  >
  > * fix(table): rename "highlight_set_selection_space" to "highlight_spacing"
  >
  > * style(table): apply doc-comment suggestions from code review

- [57ea871](https://github.com/ratatui-org/ratatui/commit/57ea871753a5b23f302c6f0a83d98f6a1988abfb) *(uncategorized)* Expand serde attributes for `TestBuffer` by @a-kenji in [#389](https://github.com/ratatui-org/ratatui/pull/389)

- [6153371](https://github.com/ratatui-org/ratatui/commit/61533712be57f3921217a905618b319975f90330) *(uncategorized)* Add weak constraints to make rects closer to each other in size ✨ by @kdheepak in [#395](https://github.com/ratatui-org/ratatui/pull/395)

  > Also make `Max` and `Min` constraints MEDIUM strength for higher priority over equal chunks

- [b090101](https://github.com/ratatui-org/ratatui/commit/b090101b231a467628c910f05a73715809cb8d73) *(uncategorized)* Simplify split function ✨ by @kdheepak in [#411](https://github.com/ratatui-org/ratatui/pull/411)

### Bug Fixes

- [9c95673](https://github.com/ratatui-org/ratatui/commit/9c956733f740b18616974e2c7d786ca761666f79) *(barchart)* Empty groups causes panic by @karthago1 in [#333](https://github.com/ratatui-org/ratatui/pull/333)

  > This unlikely to happen, since nobody wants to add an empty group.
  > Even we fix the panic, things will not render correctly.
  > So it is better to just not add them to the BarChart.

- [49a82e0](https://github.com/ratatui-org/ratatui/commit/49a82e062f2c46dc3060cdfdb230b65d9dbfb2d9) *(block)* Fixed title_style not rendered by @Valentin271 in [#363](https://github.com/ratatui-org/ratatui/pull/363)
  >
  > Fixes #349

- [8db9fb4](https://github.com/ratatui-org/ratatui/commit/8db9fb4aebd01e5ddc4edd68482361928f7e9c97) *(cargo)* Adjust minimum paste version by @ndd7xv in [#348](https://github.com/ratatui-org/ratatui/pull/348)

  > ratatui is using features that are currently only available in paste 1.0.2; specifying the minimum version to be 1.0 will consequently cause a compilation error if cargo is only able to use a version less than 1.0.2.

- [daf5890](https://github.com/ratatui-org/ratatui/commit/daf589015290ac8b379389d29ef90a1af15e3f75) *(example)* Fix typo by @joshrotenberg in [#337](https://github.com/ratatui-org/ratatui/pull/337)

  > the existential feels

- [56455e0](https://github.com/ratatui-org/ratatui/commit/56455e0fee57616f87ea43872fb7d5d9bb14aff5) *(layout)* Don't leave gaps between chunks by @joshka in [#408](https://github.com/ratatui-org/ratatui/pull/408)

  > Previously the layout used the floor of the calculated start and width
  > as the value to use for the split Rects. This resulted in gaps between
  > the split rects.
  >
  > This change modifies the layout to round to the nearest column instead
  > of taking the floor of the start and width. This results in the start
  > and end of each rect being rounded the same way and being strictly
  > adjacent without gaps.
  >
  > Because there is a required constraint that ensures that the last end is
  > equal to the area end, there is no longer the need to fixup the last
  > item width when the fill (as e.g. width = x.99 now rounds to x+1 not x).
  >
  > The colors example has been updated to use Ratio(1, 8) instead of
  > Percentage(13), as this now renders without gaps for all possible sizes,
  > whereas previously it would have left odd gaps between columns.

- [f4ed3b7](https://github.com/ratatui-org/ratatui/commit/f4ed3b758450ef9c257705f3a1ea937329a968b4) *(layout)* Ensure left <= right by @joshka in [#410](https://github.com/ratatui-org/ratatui/pull/410)

  > The recent refactor missed the positive width constraint

- [d05ab6f](https://github.com/ratatui-org/ratatui/commit/d05ab6fb700527f0e062f334c7a5319c07099b04) *(readme)* Fix typo in readme by @t-nil in [#344](https://github.com/ratatui-org/ratatui/pull/344)

- [b9290b3](https://github.com/ratatui-org/ratatui/commit/b9290b35d13df57726d65a16d3c8bb18ce43e8c2) *(readme)* Fix incorrect template link by @joshrotenberg in [#338](https://github.com/ratatui-org/ratatui/pull/338)

- [7e37a96](https://github.com/ratatui-org/ratatui/commit/7e37a96678440bc62cce52de840fef82eed58dd8) *(readme)* Fix typo in readme by @joshrotenberg in [#336](https://github.com/ratatui-org/ratatui/pull/336)

- [b6b2da5](https://github.com/ratatui-org/ratatui/commit/b6b2da5eb761ac5894cc7a2ee67f422312b63cfc) *(release)* Fix the last tag retrieval for alpha releases by @orhun in [#416](https://github.com/ratatui-org/ratatui/pull/416)

- [778c320](https://github.com/ratatui-org/ratatui/commit/778c32000815b9abb0246c73997b1800256aade2) *(release)* Set the correct permissions for creating alpha releases by @orhun in [#400](https://github.com/ratatui-org/ratatui/pull/400)

- [7539f77](https://github.com/ratatui-org/ratatui/commit/7539f775fef4d816495e1e06732f6500cf08c126) *(scrollbar)* Move symbols to symbols module by @joshka in [#330](https://github.com/ratatui-org/ratatui/pull/330) [**breaking**]

  > The symbols and sets are moved from `widgets::scrollbar` to
  > `symbols::scrollbar`. This makes it consistent with the other symbol
  > sets and allows us to make the scrollbar module private rather than
  > re-exporting it.
  >
  > BREAKING CHANGE:The symbols are now in the `symbols` module. To update
  > your code, add an import for `ratatui::symbols::scrollbar::*` (or the
  > specific symbols you need). The scrollbar module is no longer public. To
  > update your code, remove any `widgets::scrollbar` imports and replace it
  > with `ratatui::widgets::Scrollbar`.

- [dc55211](https://github.com/ratatui-org/ratatui/commit/dc552116cf5e83c7ffcc2f5299c00d2315490c1d) *(table)* Fix unit tests broken due to rounding by @joshka in [#419](https://github.com/ratatui-org/ratatui/pull/419)

  > The merge of the table unit tests after the rounding layout fix was not
  > rebased correctly, this addresses the broken tests, makes them more
  > concise while adding comments to help clarify that the rounding behavior
  > is working as expected.

- [13fb11a](https://github.com/ratatui-org/ratatui/commit/13fb11a62c826da412045d498a03673d130ec057) *(uncategorized)* Correct minor typos in documentation by @mhovd in [#331](https://github.com/ratatui-org/ratatui/pull/331)

### Refactor

- [fc727df](https://github.com/ratatui-org/ratatui/commit/fc727df7d2d8347434a7d3a4e19465b29d7a0ed8) *(barchart)* Reduce some calculations by @karthago1 in [#430](https://github.com/ratatui-org/ratatui/pull/430)

  > Calculating the label_offset is unnecessary, if we just render the
  > group label after rendering the bars. We can just reuse bar_y.

- [de25de0](https://github.com/ratatui-org/ratatui/commit/de25de0a9506e53df1378929251594bccf63d932) *(layout)* Simplify and doc split() by @joshka in [#405](https://github.com/ratatui-org/ratatui/pull/405)

  > * test(layout): add tests for split()
  >
  > * refactor(layout): simplify and doc split()
  >
  > This is mainly a reduction in density of the code with a goal of
  > improving mainatainability so that the algorithm is clear.

- [5195099](https://github.com/ratatui-org/ratatui/commit/519509945be866c3b2f6a4230ee317262266f894) *(layout)* Simplify split() function by @hasezoey in [#396](https://github.com/ratatui-org/ratatui/pull/396)

  > Removes some unnecessary code and makes the function more readable.
  > Instead of creating a temporary result and mutating it, we just create
  > the result directly from the list of changes.

### Documentation

- [7a70602](https://github.com/ratatui-org/ratatui/commit/7a70602ec6bfcfec51bafd3bdbd35ff68b64340c) *(examples)* Fix the instructions for generating demo GIF by @orhun in [#442](https://github.com/ratatui-org/ratatui/pull/442)

- [10dbd6f](https://github.com/ratatui-org/ratatui/commit/10dbd6f2075285473ef47c4c898ef2f643180cd1) *(examples)* Show layout constraints by @joshka in [#393](https://github.com/ratatui-org/ratatui/pull/393)

  > Shows the way that layout constraints interact visually
  >
  > ![example](https://vhs.charm.sh/vhs-1ZNoNLNlLtkJXpgg9nCV5e.gif)

- [6ad4bd4](https://github.com/ratatui-org/ratatui/commit/6ad4bd4cf2e7ea7548e49e64f92114c30d61ebb2) *(examples)* Add color and modifiers examples by @joshka in [#345](https://github.com/ratatui-org/ratatui/pull/345)

  > The intent of these examples is to show the available colors and
  > modifiers.
  >
  > - added impl Display for Color
  >
  > ![colors](https://vhs.charm.sh/vhs-2ZCqYbTbXAaASncUeWkt1z.gif)
  > ![modifiers](https://vhs.charm.sh/vhs-2ovGBz5l3tfRGdZ7FCw0am.gif)

- [e82521e](https://github.com/ratatui-org/ratatui/commit/e82521ea798d1385f671e1849c48de42857bf87a) *(examples)* Regen block.gif in readme by @joshka in [#365](https://github.com/ratatui-org/ratatui/pull/365)

- [554805d](https://github.com/ratatui-org/ratatui/commit/554805d6cbbf140c6da474daa891e9e754a5d281) *(examples)* Update block example by @joshka in [#351](https://github.com/ratatui-org/ratatui/pull/351)

  > ![Block example](https://vhs.charm.sh/vhs-5X6hpReuDBKjD6hLxmDQ6F.gif)

- [add578a](https://github.com/ratatui-org/ratatui/commit/add578a7d6d342e3ebaa26e69452a2ab5b08b0c7) *(examples)* Add examples readme with gifs by @joshka in [#303](https://github.com/ratatui-org/ratatui/pull/303)

  > This commit adds a readme to the examples directory with gifs of each
  > example. This should make it easier to see what each example does
  > without having to run it.
  >
  > I modified the examples to fit better in the gifs. Mostly this was just
  > removing the margins, but for the block example I cleaned up the code a
  > bit to make it more readable and changed it so the background bug is not
  > triggered.
  >
  > For the table example, the combination of Min, Length, and Percent
  > constraints was causing the table to panic when the terminal was too
  > small. I changed the example to use the Max constraint instead of the
  > Length constraint.
  >
  > The layout example now shows information about how the layout is
  > constrained on each block (which is now a paragraph with a block).

- [418ed20](https://github.com/ratatui-org/ratatui/commit/418ed20479e060c1bd2f430ae127eae19a013afc) *(layout)* Add doc comments by @joshka in [#403](https://github.com/ratatui-org/ratatui/pull/403)

- [c8ddc16](https://github.com/ratatui-org/ratatui/commit/c8ddc164c7941c31b1b5fa82345e452923ec56e7) *(layout::constraint)* Add doc-comments for all variants by @hasezoey in [#371](https://github.com/ratatui-org/ratatui/pull/371)
  >
  > fixes #354

- [8b36683](https://github.com/ratatui-org/ratatui/commit/8b36683571e078792b20d6f693b817522cf6e992) *(lib)* Extract feature documentation from Cargo.toml by @orhun in [#438](https://github.com/ratatui-org/ratatui/pull/438)

  > * docs(lib): extract feature documentation from Cargo.toml
  >
  > * chore(deps): make `document-features` optional dependency
  >
  > * docs(lib): document the serde feature from features section

- [6d6ecee](https://github.com/ratatui-org/ratatui/commit/6d6eceeb88b4da593c63dad258d2724cd583f9e0) *(paragraph)* Add more docs by @joshka in [#428](https://github.com/ratatui-org/ratatui/pull/428)

- [47fe4ad](https://github.com/ratatui-org/ratatui/commit/47fe4ad69f527fcbf879e9fec2a4d3702badc76b) *(project)* Make the project description cooler by @orhun in [#441](https://github.com/ratatui-org/ratatui/pull/441)

  > * docs(project): make the project description cooler
  >
  > * docs(lib): simplify description

- [3a37d2f](https://github.com/ratatui-org/ratatui/commit/3a37d2f6ede02fdde9ddffbb996059d6b95f98e7) *(readme)* Use the correct version for MSRV by @Valentin271 in [#369](https://github.com/ratatui-org/ratatui/pull/369)

- [2920e04](https://github.com/ratatui-org/ratatui/commit/2920e045ba23aa2eb3a4049625cd256ff37076c9) *(readme)* Fix widget docs links by @joshka in [#346](https://github.com/ratatui-org/ratatui/pull/346)

  > Add scrollbar, clear. Fix Block link. Sort

- [d0ee04a](https://github.com/ratatui-org/ratatui/commit/d0ee04a69f30506fae706b429f15fe63b056b79e) *(span)* Update docs and tests for `Span` by @joshka in [#427](https://github.com/ratatui-org/ratatui/pull/427)

- [c3f87f2](https://github.com/ratatui-org/ratatui/commit/c3f87f245a5a2fc180d4c8f64557bcff716d09a9) *(uncategorized)* Improve scrollbar doc comment by @a-kenji in [#329](https://github.com/ratatui-org/ratatui/pull/329)

### Performance

- [149d489](https://github.com/ratatui-org/ratatui/commit/149d48919d870e29a7f104664db11eb77fb951a8) *(bench)* Used `iter_batched` to clone widgets in setup function by @Valentin271 in [#383](https://github.com/ratatui-org/ratatui/pull/383)

  > Replaced `Bencher::iter` by `Bencher::iter_batched` to clone the widget in the setup function instead of in the benchmark timing.

### Styling

- [ab5e616](https://github.com/ratatui-org/ratatui/commit/ab5e6166358b2e6f0e9601a1ec5480760b91ca8e) *(paragraph)* Add documentation for "scroll"'s "offset" by @hasezoey in [#355](https://github.com/ratatui-org/ratatui/pull/355)

  > * style(paragraph): add documentation for "scroll"'s "offset"
  >
  > * style(paragraph): add more text to the scroll doc-comment

### Testing

- [a890f2a](https://github.com/ratatui-org/ratatui/commit/a890f2ac004b0e45db40de222fe3560fe0fdf94b) *(block)* Test all block methods by @joshka in [#431](https://github.com/ratatui-org/ratatui/pull/431)

- [e18393d](https://github.com/ratatui-org/ratatui/commit/e18393dbc6781a8b1266906e8ba7da019a0a5d82) *(block)* Add benchmarks by @Valentin271 in [#368](https://github.com/ratatui-org/ratatui/pull/368)

  > Added benchmarks to the block widget to uncover eventual performance issues

- [ad3413e](https://github.com/ratatui-org/ratatui/commit/ad3413eeec9aab1568f8519caaf5efb951b2800c) *(canvas)* Add unit tests for line by @joshka in [#437](https://github.com/ratatui-org/ratatui/pull/437)

  > Also add constructor to simplify creating lines

- [ad4d6e7](https://github.com/ratatui-org/ratatui/commit/ad4d6e7dec0f7e4c4e2e5624ccec54eb71c3f5ca) *(canvas)* Add tests for rectangle by @joshka in [#429](https://github.com/ratatui-org/ratatui/pull/429)

- [e9bd736](https://github.com/ratatui-org/ratatui/commit/e9bd736b1a680204fa801a7208cddc477f208680) *(clear)* Test Clear rendering by @joshka in [#432](https://github.com/ratatui-org/ratatui/pull/432)

- [664fb4c](https://github.com/ratatui-org/ratatui/commit/664fb4cffd71c85da87545cb4258165c1a44afa6) *(list)* Added benchmarks by @Valentin271 in [#377](https://github.com/ratatui-org/ratatui/pull/377)

  > Added benchmarks for the list widget (render and render half scrolled)

- [f0716ed](https://github.com/ratatui-org/ratatui/commit/f0716edbcfd33d50e4e74eaf51fe5ad945dab6b3) *(map)* Add unit tests by @joshka in [#436](https://github.com/ratatui-org/ratatui/pull/436)

- [3293c6b](https://github.com/ratatui-org/ratatui/commit/3293c6b80b0505f9ed031fc8d9678e3db627b7ad) *(sparkline)* Added benchmark by @Valentin271 in [#384](https://github.com/ratatui-org/ratatui/pull/384)

  > Added benchmark for the `sparkline` widget testing a basic render with different amout of data

- [292a11d](https://github.com/ratatui-org/ratatui/commit/292a11d81e2f8c7676cc897f3493b75903025766) *(styled_grapheme)* Test StyledGrapheme methods by @joshka in [#433](https://github.com/ratatui-org/ratatui/pull/433)

- [4cd843e](https://github.com/ratatui-org/ratatui/commit/4cd843eda97abbc8fa7af85a03c2fffafce3c676) *(table)* Add test for consistent table-column-width by @hasezoey in [#404](https://github.com/ratatui-org/ratatui/pull/404)

- [14eb6b6](https://github.com/ratatui-org/ratatui/commit/14eb6b69796550648f7d0d0427384b64c31e36d8) *(tabs)* Add unit tests by @joshka in [#439](https://github.com/ratatui-org/ratatui/pull/439)

- [b35f19e](https://github.com/ratatui-org/ratatui/commit/b35f19ec442d3eb4810f6181e03ba0d4c077b768) *(test_backend)* Add tests for TestBackend coverage by @joshka in [#434](https://github.com/ratatui-org/ratatui/pull/434)

  > These are mostly to catch any future bugs introduced in the test backend

- [fc9f637](https://github.com/ratatui-org/ratatui/commit/fc9f637fb08fdc2959a52ed3eb12643565c634d9) *(text)* Add unit tests by @joshka in [#435](https://github.com/ratatui-org/ratatui/pull/435)

### Miscellaneous Tasks

- [89ef0e2](https://github.com/ratatui-org/ratatui/commit/89ef0e29f56078ed0629f2dce89656c1131ebda1) *(ci)* Update the name of the CI workflow by @orhun in [#417](https://github.com/ratatui-org/ratatui/pull/417)

- [ea48af1](https://github.com/ratatui-org/ratatui/commit/ea48af1c9abac7012e3bf79e78c6179f889a6321) *(codecov)* Fix yaml syntax by @hasezoey in [#407](https://github.com/ratatui-org/ratatui/pull/407)

  > a yaml file cannot contain tabs outside of strings

- [8b28672](https://github.com/ratatui-org/ratatui/commit/8b286721314142dc7078354015db909e6938068c) *(docs)* Add doc comment bump to release documentation by @a-kenji in [#382](https://github.com/ratatui-org/ratatui/pull/382)

- [60a4131](https://github.com/ratatui-org/ratatui/commit/60a4131384e6c0b38b6a6e933e62646b5265ca60) *(github)* Add kdheepak as a maintainer by @orhun in [#343](https://github.com/ratatui-org/ratatui/pull/343)

- [964190a](https://github.com/ratatui-org/ratatui/commit/964190a859e6479f22c6ccae8305192f548fbcc3) *(github)* Rename `tui-rs-revival` references to `ratatui-org` by @orhun in [#340](https://github.com/ratatui-org/ratatui/pull/340)

- [268bbed](https://github.com/ratatui-org/ratatui/commit/268bbed17e0ebc18b39f3253c9beb92c21946c80) *(make)* Add task descriptions to Makefile.toml by @orhun in [#398](https://github.com/ratatui-org/ratatui/pull/398)

- [8cd3205](https://github.com/ratatui-org/ratatui/commit/8cd3205d70a1395d2c60fc26d76c300a2a463c9e) *(toolchain)* Bump msrv to 1.67 by @a-kenji in [#361](https://github.com/ratatui-org/ratatui/pull/361) [**breaking**]

  > * chore(toolchain)!: bump msrv to 1.67
  >
  > BREAKING_CHANGE:The msrv is now `1.67`
  >
  > * docs(readme): update the MSRV notice
  >
  > ---------

- [98155dc](https://github.com/ratatui-org/ratatui/commit/98155dce25bbc0e8fe271735024a1f6bf2279d67) *(traits)* Add Display and FromStr traits by @joshka in [#425](https://github.com/ratatui-org/ratatui/pull/425)

  > Use strum for most of these, with a couple of manual implementations,
  > and related tests

- [d2429bc](https://github.com/ratatui-org/ratatui/commit/d2429bc3e44a34197511192dbd215dd32fdf2d9c) *(uncategorized)* Create rust-toolchain.toml by @kdheepak in [#415](https://github.com/ratatui-org/ratatui/pull/415)

- [8c55158](https://github.com/ratatui-org/ratatui/commit/8c551588224ca97ee07948b445aa2ac9d05f997d) *(uncategorized)* Use vhs to create demo.gif by @joshka in [#390](https://github.com/ratatui-org/ratatui/pull/390)

  > The bug that prevented braille rendering is fixed, so switch to VHS for
  > rendering the demo gif
  >
  > ![Demo of Ratatui](https://vhs.charm.sh/vhs-tF0QbuPbtHgUeG0sTVgFr.gif)

- [8c4a2e0](https://github.com/ratatui-org/ratatui/commit/8c4a2e0fbfd021f1e087bb7256d9c6457742ea39) *(uncategorized)* Implement `Hash` common traits by @TieWay59 in [#381](https://github.com/ratatui-org/ratatui/pull/381)

  > Reorder the derive fields to be more consistent:
  >
  >     Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash
  >
  > Hash trait won't be impl in this PR due to rust std design.
  > If we need hash trait for f64 related structs in the future,
  > we should consider wrap f64 into a new type.
  >
  > see:https://github.com/ratatui-org/ratatui/issues/307

- [181706c](https://github.com/ratatui-org/ratatui/commit/181706c564d86e02991f89ec674b1af1d7f393fe) *(uncategorized)* Implement `Eq & PartialEq` common traits by @TieWay59 in [#357](https://github.com/ratatui-org/ratatui/pull/357)

  > Reorder the derive fields to be more consistent:
  >
  >     Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash
  >
  > see:https://github.com/ratatui-org/ratatui/issues/307

- [440f62f](https://github.com/ratatui-org/ratatui/commit/440f62ff5435af9536c55d17707a9bc48dae92cc) *(uncategorized)* Implement `Clone & Copy` common traits by @TieWay59 in [#350](https://github.com/ratatui-org/ratatui/pull/350)

  > Implement `Clone & Copy` common traits for most structs in src.
  >
  > Only implement `Copy` for structs that are simple and trivial to copy.
  >
  > Reorder the derive fields to be more consistent:
  >
  >     Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash
  >
  > see:https://github.com/ratatui-org/ratatui/issues/307

- [bf49446](https://github.com/ratatui-org/ratatui/commit/bf4944683d6afb6f42bec80a1bd308ecdac50cbc) *(uncategorized)* Implement `Debug & Default` common traits by @TieWay59 in [#339](https://github.com/ratatui-org/ratatui/pull/339)

  > Implement `Debug & Default` common traits for most structs in src.
  >
  > Reorder the derive fields to be more consistent:
  >
  >     Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash
  >
  > see:https://github.com/ratatui-org/ratatui/issues/307

### Build

- [37fa6ab](https://github.com/ratatui-org/ratatui/commit/37fa6abe9d5dc459dc9855ea10f06afa72717c98) *(deps)* Upgrade crossterm to 0.27 by @a-kenji in [#380](https://github.com/ratatui-org/ratatui/pull/380)

- [e2cb11c](https://github.com/ratatui-org/ratatui/commit/e2cb11cc30072d90b20e04270c1fa97c18ab6f3f) *(examples)* Fix cargo make run-examples by @joshka in [#327](https://github.com/ratatui-org/ratatui/pull/327)

  > Enables the all-widgets feature so that the calendar example runs correctly

- [0fb1ed8](https://github.com/ratatui-org/ratatui/commit/0fb1ed85c6232966ab25c8b3cab0fc277e9b69a6) *(uncategorized)* Forbid unsafe code by @EdJoPaTo in [#332](https://github.com/ratatui-org/ratatui/pull/332)

  > This indicates good (high level) code and is used by tools like cargo-geiger.

### Continuous Integration

- [de9f52f](https://github.com/ratatui-org/ratatui/commit/de9f52ff2cc606e1bf6b6bd8b97907afd73860fe) *(coverage)* Exclude examples directory from coverage by @a-kenji in [#373](https://github.com/ratatui-org/ratatui/pull/373)

- [9191ad6](https://github.com/ratatui-org/ratatui/commit/9191ad60fd4fc3ddf8650a8f5eed87216a0e5c6f) *(uncategorized)* Don't fail fast by @joshka in [#364](https://github.com/ratatui-org/ratatui/pull/364)

  > Run all the tests rather than canceling when one test fails. This allows
  > us to see all the failures, rather than just the first one if there are
  > multiple. Specifically this is useful when we have an issue in one
  > toolchain or backend.

- [6f659cf](https://github.com/ratatui-org/ratatui/commit/6f659cfb07aad5ad2524f32fe46c45b84c8e9e34) *(uncategorized)* Add coverage token by @joshka in [#352](https://github.com/ratatui-org/ratatui/pull/352)



### New Contributors
* @jkcdarunday made their first contribution in [#392](https://github.com/ratatui-org/ratatui/pull/392)
* @ndd7xv made their first contribution in [#348](https://github.com/ratatui-org/ratatui/pull/348)
* @t-nil made their first contribution in [#344](https://github.com/ratatui-org/ratatui/pull/344)
* @mhovd made their first contribution in [#331](https://github.com/ratatui-org/ratatui/pull/331)

**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.22.0...v0.23.0


## [v0.22.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.22.0) - 2023-07-17

### Features

- [60150f6](https://github.com/ratatui-org/ratatui/commit/60150f6236168cdfd526d7eab601a102582f965c) *(barchart)* Set custom text value in the bar by @karthago1 in [#309](https://github.com/ratatui-org/ratatui/pull/309)

  > for now the value is converted to a string and then printed. in many
  > cases the values are too wide or double values. so it make sense
  > to set a custom value text instead of the default behavior.
  >
  > this patch suggests to add a method
  > "fn text_value(mut self, text_value: String)"
  > to the Bar, which allows to override the value printed in the bar

- [ae8ed88](https://github.com/ratatui-org/ratatui/commit/ae8ed8867df13c1dccd921f6e77f63fc1fdc812c) *(barchart)* Enable barchart groups by @karthago1 in [#288](https://github.com/ratatui-org/ratatui/pull/288)

  > * feat(barchart): allow to add a group of bars
  >
  > Example:to show the revenue of different companies:
  > ┌────────────────────────┐
  > │             ████       │
  > │             ████       │
  > │      ████   ████       │
  > │ ▄▄▄▄ ████   ████ ████  │
  > │ ████ ████   ████ ████  │
  > │ ████ ████   ████ ████  │
  > │ █50█ █60█   █90█ █55█  │
  > │    Mars       April    │
  > └────────────────────────┘
  > new structs are introduced: Group and Bar.
  > the data function is modified to accept "impl Into<Group<'a>>".
  >
  > a new function "group_gap" is introduced to set the gap between each group
  >
  > unit test changed to allow the label to be in the center

- [a04b190](https://github.com/ratatui-org/ratatui/commit/a04b19025196362e5582da5fda2fc32cd1191f20) *(block)* Support for having more than one title by @samyosm in [#232](https://github.com/ratatui-org/ratatui/pull/232)

- [57678a5](https://github.com/ratatui-org/ratatui/commit/57678a5fe83e178ad46172b62025951e64d19354) *(examples)* User_input example cursor movement by @BoolPurist in [#302](https://github.com/ratatui-org/ratatui/pull/302)

  > The user_input example now responds to left/right and allows the
  > character at the cursor position to be deleted / inserted.

- [7a6c3d9](https://github.com/ratatui-org/ratatui/commit/7a6c3d9db1990d17d13293a716111193fc57b158) *(misc)* Make builder fn const by @a-kenji in [#275](https://github.com/ratatui-org/ratatui/pull/275)

  > This allows the following types to be used in a constant context:
  > - `Layout`
  > - `Rect`
  > - `Style`
  > - `BorderType`
  > - `Padding`
  > - `Block`
  >
  > Also adds several missing `new()` functions to the above types.
  >
  > Blocks can now be used in the following way:
  > ```
  > const DEFAULT_BLOCK: Block = Block::new()
  >     .title_style(Style::new())
  >     .title_alignment(Alignment::Left)
  >     .title_position(Position::Top)
  >     .borders(Borders::ALL)
  >     .border_style(Style::new())
  >     .style(Style::reset())
  >     .padding(Padding::uniform(1));
  >
  > ```
  >
  > Layouts can now be used in the following way:
  > ``
  > const DEFAULT_LAYOUT: Layout = Layout::new()
  >     .direction(Direction::Horizontal)
  >     .margin(1)
  >     .expand_to_fill(false);
  > ```
  >
  > Rects can now be used in the following way:
  > ```
  > const RECT: Rect = Rect {
  >     x: 0,
  >     y: 0,
  >     width: 10,
  >     height: 10,
  > };
  > ```

- [804115a](https://github.com/ratatui-org/ratatui/commit/804115ac6f55c40d41502f7d3b8dc352d7c70038) *(prelude)* Add a prelude by @joshka in [#304](https://github.com/ratatui-org/ratatui/pull/304)

  > This allows users of the library to easily use ratatui without a huge amount of imports

- [b347201](https://github.com/ratatui-org/ratatui/commit/b347201b9f1243f7e4f3fd1efe6ed3c20c56c65a) *(style)* Enable setting the underline color for crossterm by @Nogesma in [#310](https://github.com/ratatui-org/ratatui/pull/310)

  > This commit adds the underline_color() function to the Style and Cell
  > structs. This enables setting the underline color of text on the
  > crossterm backend. This is a no-op for the termion and termwiz backends
  > as they do not support this feature.

- [f7c4b44](https://github.com/ratatui-org/ratatui/commit/f7c4b449621f9138c658abc1475370a1dde9669c) *(style)* Allow Modifiers add/remove in const by @a-kenji in [#287](https://github.com/ratatui-org/ratatui/pull/287)

  > Allows Modifiers to be added or removed from `Style` in a const context.
  > This can be used in the following way:
  >
  > ```
  > const DEFAULT_MODIFIER: Modifier = Modifier::BOLD.union(Modifier::ITALIC);
  > const DEFAULT_STYLE: Style = Style::new()
  > .fg(Color::Red).bg(Color::Black).add_modifier(DEFAULT_MODIFIER);
  > ```

- [9f1f59a](https://github.com/ratatui-org/ratatui/commit/9f1f59a51c49da8f5d82187a42a3040c465f9799) *(stylize)* Allow all widgets to be styled by @joshka in [#289](https://github.com/ratatui-org/ratatui/pull/289)

  > * feat(stylize): allow all widgets to be styled
  >
  > - Add styled impl to:
  >   - Barchart
  >   - Chart (including Axis and Dataset),
  >   - Guage and LineGuage
  >   - List and ListItem
  >   - Sparkline
  >   - Table, Row, and Cell
  >   - Tabs
  >   - Style
  > - Allow modifiers to be removed (e.g. .not_italic())
  > - Allow .bg() to recieve Into<Color>
  > - Made shorthand methods consistent with modifier names (e.g. dim() not
  >   dimmed() and underlined() not underline())
  > - Simplify integration tests
  > - Add doc comments
  > - Simplified stylize macros with https://crates.io/crates/paste
  >
  > * build: run clippy before tests
  >
  > Runny clippy first means that we fail fast when there is an issue that
  > can easily be fixed rather than having to wait 30-40s for the failure

- [f84d97b](https://github.com/ratatui-org/ratatui/commit/f84d97b17bc359f4fc5c3ab6a704192b64960255) *(terminal)* Expose 'swap_buffers' method by @Philipp-M in [#230](https://github.com/ratatui-org/ratatui/pull/230)

- [2f4413b](https://github.com/ratatui-org/ratatui/commit/2f4413be6e32f27b3fb6adf0945f652bccd2c421) *(uncategorized)* Stylization shorthands by @samyosm in [#283](https://github.com/ratatui-org/ratatui/pull/283)

- [130bdf8](https://github.com/ratatui-org/ratatui/commit/130bdf833732cb707d524a3afb8085d32b61fcd9) *(uncategorized)* Add scrollbar widget by @kdheepak in [#228](https://github.com/ratatui-org/ratatui/pull/228)

  > Represents a scrollbar widget that renders a track, thumb and arrows
  > either horizontally or vertically. State is kept in ScrollbarState, and
  > passed as a parameter to the render function.

### Bug Fixes

- [83d3ec7](https://github.com/ratatui-org/ratatui/commit/83d3ec73e7b8328cc5be5da4cf818396246a2123) *(clippy)* Ununsed_mut lint for layout by @joshka in [#285](https://github.com/ratatui-org/ratatui/pull/285)

  > This lint is slightly more agressive in +nightly than it is in stable.

- [43bac80](https://github.com/ratatui-org/ratatui/commit/43bac80e4d57a2d1c98925a8485782cf4d9376ca) *(examples)* Correct progress label in gague example by @mrbcmorris in [#263](https://github.com/ratatui-org/ratatui/pull/263)

- [28a8435](https://github.com/ratatui-org/ratatui/commit/28a8435a52c902c6ccfd76500ea07de3fad68797) *(layout)* Cap Contstraint::apply to 100% length by @endepointe in [#264](https://github.com/ratatui-org/ratatui/pull/264)

  > This function is only currently used in by the chart widget for
  > constraining the width and height of the legend.

- [2889c7d](https://github.com/ratatui-org/ratatui/commit/2889c7d08448a5bd786329fdaeedbb37f77d55b9) *(lint)* Suspicious_double_ref_op is new in 1.71 by @joshka in [#311](https://github.com/ratatui-org/ratatui/pull/311)

  > Fixed tests and completed coverage for `Masked` type.

- [446efae](https://github.com/ratatui-org/ratatui/commit/446efae185a5bb02a9ab6481542d1dc3024b66a9) *(prelude)* Remove widgets module from prelude by @joshka in [#317](https://github.com/ratatui-org/ratatui/pull/317)

  > This helps to keep the prelude small and less likely to conflict with
  > other crates.
  >
  > - remove widgets module from prelude as the entire module can be just as
  >   easily imported with `use ratatui::widgets::*;`
  > - move prelude module into its own file
  > - update examples to import widgets module instead of just prelude
  > - added several modules to prelude to make it possible to qualify
  >   imports that collide with other types that have similar names

- [1ff8553](https://github.com/ratatui-org/ratatui/commit/1ff85535c834f6b7c002826cead811b4e73a8d9c) *(title)* Remove default alignment and position by @orhun in [#323](https://github.com/ratatui-org/ratatui/pull/323)

  > * fix(title): remove default alignment and position
  >
  > * test(block): add test cases for alignment
  >
  > * test(block): extend the unit tests for block title alignment

- [ef4d743](https://github.com/ratatui-org/ratatui/commit/ef4d743af3717a146a63fed692b916ce2459d640) *(typos)* Configure typos linter by @joshka in [#233](https://github.com/ratatui-org/ratatui/pull/233)

  > - Adds a new typos.toml
  > - Prevents ratatui from being marked as a typo of ratatouille
  > - Changes paragraph tests so that the truncated word is a valid word

- [33f3212](https://github.com/ratatui-org/ratatui/commit/33f3212cbf2b01960559c0035a910eaf96115597) *(uncategorized)* Rust-tui-template became a revival project by @joshka in [#320](https://github.com/ratatui-org/ratatui/pull/320)

  > Changed the URL https://github.com/orhun/rust-tui-template
  > into https://github.com/rust-tui-revival/rust-tui-template

- [dca9871](https://github.com/ratatui-org/ratatui/commit/dca987174494e3edb40f11659b23f3c1319c99d3) *(uncategorized)* Revert removal of WTFPL from deny.toml by @joshka in [#266](https://github.com/ratatui-org/ratatui/pull/266)

  > This is actually used for terminfo (transitively from termwiz

### Refactor

- [0bf6af1](https://github.com/ratatui-org/ratatui/commit/0bf6af17e7d72b959b2776ca66a9c9986388d2df) *(ci)* Simplify cargo-make installation by @orhun in [#240](https://github.com/ratatui-org/ratatui/pull/240)

  > * refactor(ci): simplify cargo-make installation
  >
  > * chore(ci): use the latest version of cargo-make
  >
  > * refactor(ci): remove unused triple values
  >
  > * chore(ci): list all steps before ci
  >
  > * fix(ci): checkout the repository
  >
  > * refactor(ci): remove unnecessary os variables
  >
  > * refactor(ci): use dtolnay/rust-toolchain action

- [fb6d4b2](https://github.com/ratatui-org/ratatui/commit/fb6d4b2f51ae372c8d919a969ef7734f18cdaf9c) *(text)* Simplify reflow implementation by @joshka in [#290](https://github.com/ratatui-org/ratatui/pull/290)

  > * refactor(text): split text::* into separate files
  >
  > * feat(text): expose graphemes on Line
  >
  > - add `Line::styled()`
  > - add `Line::styled_graphemes()`
  > - add `StyledGrapheme::new()`
  >
  > ---------

### Documentation

- [e66d5cd](https://github.com/ratatui-org/ratatui/commit/e66d5cdee0380c0a21e3fadbd27ed2967c186809) *(color)* Parse more color formats and add docs by @joshka in [#306](https://github.com/ratatui-org/ratatui/pull/306)

- [20c0051](https://github.com/ratatui-org/ratatui/commit/20c0051026fa67ab9f3de69af4c850ff9efede1b) *(lib)* Add `tui-term` a pseudoterminal library by @a-kenji in [#268](https://github.com/ratatui-org/ratatui/pull/268)

- [e95b512](https://github.com/ratatui-org/ratatui/commit/e95b5127caaf7841ccb45f692bcf13d7e015c0c6) *(lib)* Fixup tui refs in widgets/mod.rs by @joshka in [#216](https://github.com/ratatui-org/ratatui/pull/216)

- [5243aa0](https://github.com/ratatui-org/ratatui/commit/5243aa06280a9ec7b203690b5ae37db123b35044) *(lib)* Add backend docs by @joshka in [#213](https://github.com/ratatui-org/ratatui/pull/213)

- [e165025](https://github.com/ratatui-org/ratatui/commit/e165025c94a6bdfe17956c4e67d275b4d8c503dd) *(readme)* Remove duplicated mention of tui-rs-tree-widgets by @snpefk in [#223](https://github.com/ratatui-org/ratatui/pull/223)

- [0833c90](https://github.com/ratatui-org/ratatui/commit/0833c9018bc0de7ff5916743d851d46a1f3f2eb1) *(uncategorized)* Improve CONTRIBUTING.md by @joshka in [#277](https://github.com/ratatui-org/ratatui/pull/277)

- [b808305](https://github.com/ratatui-org/ratatui/commit/b8083055070c6dd5ebc7117dc4f9f832a4948eb5) *(uncategorized)* Fix scrollbar ascii illustrations and calendar doc paths by @SLASHLogin in [#272](https://github.com/ratatui-org/ratatui/pull/272)

  > * docs(src\widgets\scrollbar.rs): wrap scrollbar's visualisation in text block
  >
  > 'cargo doc' and 'rust-analyzer' removes many whitespaces thus making those parts render improperly
  >
  > * docs(src/widgets/calendar.rs): fix `no item named ...` for calendar.rs
  >
  > * style(src/widgets/block.rs): format `block.rs`

- [9ecc4a1](https://github.com/ratatui-org/ratatui/commit/9ecc4a15df5731fdf6956320cbb6737d7b01ec08) *(uncategorized)* README tweaks by @joshka in [#225](https://github.com/ratatui-org/ratatui/pull/225)

  > - Add contributors graph
  > - Add markdownlint config file
  > - Reformat README line width to 100
  > - Add a link to the CHANGELOG
  > - Remove APPS.md
  > - Change apps link to the Wiki instead of APPS.md

- [77067bd](https://github.com/ratatui-org/ratatui/commit/77067bdc587d6c315b1346c4f06224e8b8560b72) *(uncategorized)* Add CODEOWNERS file by @joshka in [#212](https://github.com/ratatui-org/ratatui/pull/212)

  > See https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners

- [b40ca44](https://github.com/ratatui-org/ratatui/commit/b40ca44e1a216207faf2b0070b30c97b91f0c1dd) *(uncategorized)* Update README.md and add hello_world example by @joshka in [#204](https://github.com/ratatui-org/ratatui/pull/204)

  > - Reformat summary info
  > - Add badges for dependencies, discord, license
  > - point existing badges to shields.io
  > - add Table of Contents
  > - tweaked installation instructions to show instructions for new and
  > existing crates
  > - moved fork status lower
  > - chop lines generally to 100 limit
  > - add a quickstart based on a simplified hello_world example
  > - added / updated some internal links to point locally
  > - removed some details to simplify the readme (e.g. tick-rate)
  > - reordered widgets and pointed these at the widget docs
  > - adds a hello_world example that has just the absolute basic code
  > necessary to run a ratatui app. This includes some comments that help
  > guide the user towards other approaches and considerations for a real
  > world app.

### Styling

- [40b3543](https://github.com/ratatui-org/ratatui/commit/40b3543c3f4dc9999b534aef699f801f28447429) *(comments)* Set comment length to wrap at 100 chars by @joshka in [#218](https://github.com/ratatui-org/ratatui/pull/218)

  > This is an opinionated default that helps avoid horizontal scrolling.
  > 100 is the most common width on github rust projects and works well for
  > displaying code on a 16in macbook pro.

- [231aae2](https://github.com/ratatui-org/ratatui/commit/231aae2920a81683c27bd5e4a901a7a363c26d8d) *(config)* Apply formatting to config files by @orhun in [#238](https://github.com/ratatui-org/ratatui/pull/238)

- [26dbb29](https://github.com/ratatui-org/ratatui/commit/26dbb29b3d4b5d5f9aaa96025c01ed134965f335) *(manifest)* Apply formatting to Cargo.toml by @orhun in [#237](https://github.com/ratatui-org/ratatui/pull/237)

- [860a40c](https://github.com/ratatui-org/ratatui/commit/860a40c13a2d159f93fc21b15adca367647186ee) *(readme)* Update the style of badges in README.md by @orhun in [#299](https://github.com/ratatui-org/ratatui/pull/299)

- [bfcc550](https://github.com/ratatui-org/ratatui/commit/bfcc5504bbd88f5e242b96ae9a0ab9104ab53cf0) *(widget)* Inline format arguments by @a-kenji in [#279](https://github.com/ratatui-org/ratatui/pull/279)

- [c5d387c](https://github.com/ratatui-org/ratatui/commit/c5d387cb53104ca3f1de9682bbeee1f8b0368625) *(uncategorized)* Fix formatting by @joshka in [#292](https://github.com/ratatui-org/ratatui/pull/292)

  > There are a couple of small formatting changes in the current nightly

- [f7af8a3](https://github.com/ratatui-org/ratatui/commit/f7af8a3863e8b357748d897d2c6e36aca2577286) *(uncategorized)* Reformat imports by @joshka in [#219](https://github.com/ratatui-org/ratatui/pull/219)

  > Order imports by std, external, crate and group them by crate

### Testing

- [a1813af](https://github.com/ratatui-org/ratatui/commit/a1813af297058a141011f03870aed132f2912754) *(barchart)* Add unit tests by @joshka in [#301](https://github.com/ratatui-org/ratatui/pull/301)

- [cf8eda0](https://github.com/ratatui-org/ratatui/commit/cf8eda04a19320cde7acd1414b42acd7bae04a12) *(paragraph)* Simplify paragraph benchmarks by @joshka in [#282](https://github.com/ratatui-org/ratatui/pull/282)

  > Reduce benchmarks from 60 calls to 18. Now 3 different line counts
  > (64, 2048, 65535) * 6 different tests (new, render, scroll half / full,
  > wrap, wrap and scroll)

- [6c2fbbf](https://github.com/ratatui-org/ratatui/commit/6c2fbbf275a0543ef75b9bd460336a87686c9a24) *(uncategorized)* Add benchmarks for paragraph by @joshka in [#262](https://github.com/ratatui-org/ratatui/pull/262)

  > To run the benchmarks:
  >
  >     cargo bench
  >
  > And then open the generated `target/criterion/report/index.html` in a
  > browser.
  >
  > - add the BSD 2 clause and ISC licenses to the `cargo deny` allowed
  > licenses list (as a transitive dependency of the `fakeit` crate).
  > - remove the WTFPL license from the `cargo deny` allowed licenses list
  > as it is unused and causes a warning when running the check.

### Miscellaneous Tasks

- [492af7a](https://github.com/ratatui-org/ratatui/commit/492af7a92d264a590ebbf4eb90b45657e30dfd30) *(ci)* Bump cargo-make version by @orhun in [#239](https://github.com/ratatui-org/ratatui/pull/239)

- [4a2ff20](https://github.com/ratatui-org/ratatui/commit/4a2ff204eca7cceee1510f1e54f1d53fac9b7d41) *(ci)* Enable merge queue for builds by @orhun in [#235](https://github.com/ratatui-org/ratatui/pull/235)

  > * chore(ci): enable merge queue for builds
  >
  > https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/managing-a-merge-queue
  >
  > * style(ci): format the ci workflow

- [d711f2a](https://github.com/ratatui-org/ratatui/commit/d711f2aef38013cb2ec6d0eca638372600d63bd8) *(ci)* Integrate cargo-deny for linting dependencies by @orhun in [#221](https://github.com/ratatui-org/ratatui/pull/221)

- [e724bec](https://github.com/ratatui-org/ratatui/commit/e724bec987839a013bb728ec9df0541fae9bef3e) *(commitizen)* Add commitizen config by @Nydragon in [#222](https://github.com/ratatui-org/ratatui/pull/222)

  > * style(commitizen): add commitizen
  >
  > add customized commitizen config to match repo needs
  >
  > implement request mentioned in [#214](https://github.com/tui-rs-revival/ratatui/issues/214) by @joshka
  >
  > * docs(CONTRIBUTING.md): add a section for the commitizen installation
  >
  > BREAKING_CHANGE:* style(commitizen): update breaking change default to false

- [de4f2b9](https://github.com/ratatui-org/ratatui/commit/de4f2b99900800d791ad50693f9146b9ba85726d) *(demo)* Update demo gif by @orhun in [#234](https://github.com/ratatui-org/ratatui/pull/234)

- [593fd29](https://github.com/ratatui-org/ratatui/commit/593fd29d00dde265be3dd83cc8c7ca132022cf87) *(demo)* Update demo gif with a fixed unicode gauge by @joshka in [#227](https://github.com/ratatui-org/ratatui/pull/227)

  > * fix(gauge): render gauge with unicode correctly
  >
  > Gauge now correctly renders a block rather than a space when in unicode mode.
  >
  > * docs: update demo.gif
  >
  > - remove existing gif
  > - upload using VHS (https://github.com/charmbracelet/vhs)
  > - add instructions to RELEASE.md
  > - link new gif in README

- [ad288f5](https://github.com/ratatui-org/ratatui/commit/ad288f5168e81f9c01758c1fdc30816855d73930) *(features)* Enable building with all-features by @joshka in [#286](https://github.com/ratatui-org/ratatui/pull/286)

  > Before this change, it wasn't possible to build all features and all
  > targets at the same time, which prevents rust-analyzer from working
  > for the whole project.
  >
  > Adds a bacon.toml file to the project, which is used by bacon
  > https://dystroy.org/bacon/
  >
  > Configures docs.rs to show the feature flags that are necessary to
  > make modules / types / functions available.

- [085fde7](https://github.com/ratatui-org/ratatui/commit/085fde7d4a61cbdd05fca92eedd79139109da639) *(github)* Add EditorConfig config by @orhun in [#300](https://github.com/ratatui-org/ratatui/pull/300)

- [284b0b8](https://github.com/ratatui-org/ratatui/commit/284b0b8de05bf094f49930739eaed6c273e07943) *(github)* Simplify the CODEOWNERS file by @orhun in [#271](https://github.com/ratatui-org/ratatui/pull/271)

- [8b7b788](https://github.com/ratatui-org/ratatui/commit/8b7b7881f53bcd36516c542419e356bf4dfa75a8) *(github)* Add pull request template by @orhun in [#269](https://github.com/ratatui-org/ratatui/pull/269)

- [4cc7380](https://github.com/ratatui-org/ratatui/commit/4cc7380a887561ca58048990c133b33662d15589) *(github)* Fix the syntax in CODEOWNERS file by @orhun in [#236](https://github.com/ratatui-org/ratatui/pull/236)

- [56e44a0](https://github.com/ratatui-org/ratatui/commit/56e44a0efa290b97d8d919819c5fec0755b8cc4e) *(license)* Add Ratatui developers to license by @joshka in [#297](https://github.com/ratatui-org/ratatui/pull/297)

- [6f6c355](https://github.com/ratatui-org/ratatui/commit/6f6c355c5c08e159092b253f127f9676d79201ed) *(tests)* Add coverage job to bacon by @joshka in [#312](https://github.com/ratatui-org/ratatui/pull/312)

  > - Add two jobs to bacon.toml (one for unit tests, one for all tests)
  > - Remove "run" job as it doesn't work well with bacon due to no stdin
  > - Document coverage tooling in CONTRIBUTING.md

- [509d185](https://github.com/ratatui-org/ratatui/commit/509d18501ccac27c484e50ed64938f6c04ab5177) *(uncategorized)* Lint and doc cleanup by @nyurik in [#191](https://github.com/ratatui-org/ratatui/pull/191)

  > * chore: Lint and doc cleanup
  >
  > A few more minor cleanups, mostly in documentation
  >
  > * Remove unused comment

### Build

- [358b50b](https://github.com/ratatui-org/ratatui/commit/358b50ba21dd924a6a05efa3f96f5ce7ea6b9467) *(deps)* Upgrade bitflags to 2.3 by @joshka in [#205](https://github.com/ratatui-org/ratatui/pull/205) [**breaking**]
  >
  > BREAKING CHANGE:The serde representation of bitflags has changed. Any
  > existing serialized types that have Borders or Modifiers will need to be
  > re-serialized. This is documented in the bitflags changelog.
  >
  > https://github.com/bitflags/bitflags/blob/main/CHANGELOG.md#200-rc2

- [669a4d5](https://github.com/ratatui-org/ratatui/commit/669a4d56529e42ce9b589bb511c8fb6d6cc74c83) *(uncategorized)* Add git pre-push hooks using cargo-husky by @SLASHLogin in [#274](https://github.com/ratatui-org/ratatui/pull/274)

  > Fixes https://github.com/tui-rs-revival/ratatui/issues/214
  > - add cargo-husky to dev-deps
  > - create hook
  > - update `CONTRIBUTING.md`
  > - ensure that the hook is not installed in CI

### Continuous Integration

- [6bdb97c](https://github.com/ratatui-org/ratatui/commit/6bdb97c55c7d57a38d5159d7cf2ac715aed60a66) *(makefile)* Split CI jobs by @SLASHLogin in [#278](https://github.com/ratatui-org/ratatui/pull/278)

  > - Split CI into build, clippy and test.
  > - Run format on nightly only due to the settings being unstable

- [bb061fd](https://github.com/ratatui-org/ratatui/commit/bb061fdab6fc069615191bc1203a749d53c42ec8) *(uncategorized)* Parallelize CI jobs by @joshka in [#318](https://github.com/ratatui-org/ratatui/pull/318)

  > * ci: parallelize CI jobs
  >
  > - remove the dependency on the lint job from all other jobs
  > - implement workflow concurrency
  > - reorder the workflow so that the lint, clippy and coverage jobs are
  >   scheduled before the test jobs
  > - run jobs which run for each backend in parallel by calling e.g.
  >   cargo make test-termion, instead of cargo make test
  > - add a coverage task to the makefile
  > - change "cargo-make check" to check all features valid for OS in
  >   parallel
  > - run clippy only on the ubuntu-latest runner and check all features
  >   valid in parallel
  > - tidy up the workflow file
  >
  > * ci: simplify Makefile OS detection
  >
  > Use platform overrides to significantly simplify the Makefile logic
  > See https://github.com/sagiegurari/cargo-make\#platform-override
  >
  > * fix(termwiz): skip doc test that requires stdout

- [e869869](https://github.com/ratatui-org/ratatui/commit/e869869462b4599b1401236243c493f515b7f588) *(uncategorized)* Add feat-wrapping on push and on pull request ci triggers by @mindoodoo in [#267](https://github.com/ratatui-org/ratatui/pull/267)

- [a68d621](https://github.com/ratatui-org/ratatui/commit/a68d621f2d563106594115fbbace76ff29f3e590) *(uncategorized)* Add code coverage action by @joshka in [#209](https://github.com/ratatui-org/ratatui/pull/209)

  > This runs the coverage and uploads the output to
  > https://app.codecov.io/gh/tui-rs-revival/ratatui/



### New Contributors
* @Nogesma made their first contribution in [#310](https://github.com/ratatui-org/ratatui/pull/310)
* @BoolPurist made their first contribution in [#302](https://github.com/ratatui-org/ratatui/pull/302)
* @samyosm made their first contribution in [#283](https://github.com/ratatui-org/ratatui/pull/283)
* @SLASHLogin made their first contribution in [#278](https://github.com/ratatui-org/ratatui/pull/278)
* @endepointe made their first contribution in [#264](https://github.com/ratatui-org/ratatui/pull/264)
* @mrbcmorris made their first contribution in [#263](https://github.com/ratatui-org/ratatui/pull/263)
* @Philipp-M made their first contribution in [#230](https://github.com/ratatui-org/ratatui/pull/230)
* @snpefk made their first contribution in [#223](https://github.com/ratatui-org/ratatui/pull/223)
* @Nydragon made their first contribution in [#222](https://github.com/ratatui-org/ratatui/pull/222)

**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.21.0...v0.22.0


## [v0.21.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.21.0) - 2023-05-29

### Features

- [4437835](https://github.com/ratatui-org/ratatui/commit/44378350574b5ebe6c75c80b72f13d653016113e) *(backend)* Add termwiz backend and example by @orhun in [#5](https://github.com/ratatui-org/ratatui/pull/5)

  > * build: bump MSRV to 1.65
  >
  > The latest version of the time crate requires Rust 1.65.0
  >
  > ```
  > cargo +1.64.0-x86_64-apple-darwin test --no-default-features \
  >   --features serde,crossterm,all-widgets --lib --tests --examples
  > error: package `time v0.3.21` cannot be built because it requires rustc
  > 1.65.0 or newer, while the currently active rustc version is 1.64.0
  > ```
  >
  > * feat(backend): add termwiz backend and demo
  >
  > * ci(termwiz): add termwiz to makefile.toml
  >
  > ---------

- [7adc3fe](https://github.com/ratatui-org/ratatui/commit/7adc3fe19b2ef332a4a47be3e053229b47c1c74a) *(block)* Support placing the title on bottom by @orhun in [#36](https://github.com/ratatui-org/ratatui/pull/36)

- [ef8bc7c](https://github.com/ratatui-org/ratatui/commit/ef8bc7c5a8d3a48c2b562d4dd59fbeb6d6122b03) *(border)* Add border! macro for easy bitflag manipulation by @orhun in [#11](https://github.com/ratatui-org/ratatui/pull/11)

  > Adds a `border!` macro that takes TOP, BOTTOM, LEFT, RIGHT, and ALL and
  > returns a Borders object. An empty `border!()` call returns
  > Borders::NONE
  >
  > This is gated behind a `macros` feature flag to ensure short build
  > times. To enable, add the following to your `Cargo.toml`:
  >
  > ```toml
  > ratatui = { version = 0.21.0, features = ["macros"] }
  > ```

- [f7ab4f0](https://github.com/ratatui-org/ratatui/commit/f7ab4f04ac0844af808e77097c1e6b2dfa735222) *(calendar)* Add calendar widget by @sophacles in [#138](https://github.com/ratatui-org/ratatui/pull/138)

- [a425102](https://github.com/ratatui-org/ratatui/commit/a425102cf3016020cfe4fad539f7a798ebc171b8) *(color)* Add `FromStr` implementation for `Color` by @Mehrbod2002 in [#180](https://github.com/ratatui-org/ratatui/pull/180)

  > * fix: FromStr in Color Struct
  >
  > * fix: rgb issue
  >
  > * fix: rgb issue
  >
  > * fix: add doc
  >
  > * fix: doctests
  >
  > * feat: move and add tests
  >
  > * refactor: rgb & invalid_color test

- [ef96109](https://github.com/ratatui-org/ratatui/commit/ef96109ea587d73d04da44b951d9c26618017cf3) *(list)* Add len() to List by @kyoto7250 in [#24](https://github.com/ratatui-org/ratatui/pull/24)

- [32e416e](https://github.com/ratatui-org/ratatui/commit/32e416ea95bc8964437345329a0870a390a84c11) *(paragraph)* Allow Lines to be individually aligned by @TimerErTim in [#149](https://github.com/ratatui-org/ratatui/pull/149)

  > `Paragraph` now supports rendering each line in the paragraph with a
  > different alignment (Center, Left and Right) rather than the entire
  > paragraph being aligned the same. Each line either overrides the
  > paragraph's alignment or inherits it if the line's alignment is
  > unspecified.
  >
  > - Adds `alignment` field to `Line` and builder methods on `Line` and
  > `Span`
  > - Updates reflow module types to be line oriented rather than symbol
  > oriented to take account of each lines alignment
  > - Adds unit tests to `Paragraph` to fully capture the existing and new
  > behavior
  >
  > ---------

- [b7bd305](https://github.com/ratatui-org/ratatui/commit/b7bd3051b105f0744ee9a8e62ec4a41a1eefc840) *(sparkline)* Finish #1 Sparkline directions PR by @joshka in [#134](https://github.com/ratatui-org/ratatui/pull/134)

- [6af75d6](https://github.com/ratatui-org/ratatui/commit/6af75d6d40b39b8769a311bb92e788f8d8e3618c) *(terminal)* Add inline viewport by @conradludgate in [#114](https://github.com/ratatui-org/ratatui/pull/114) [**breaking**]

- [86c3fc9](https://github.com/ratatui-org/ratatui/commit/86c3fc9fac6ac20744fbd8abcd5be543316815c2) *(test)* Expose test buffer by @a-kenji in [#160](https://github.com/ratatui-org/ratatui/pull/160)

  > Allow a way to expose the buffer of the `TestBackend`,
  > to easier support different testing methodologies.

- [2f0d549](https://github.com/ratatui-org/ratatui/commit/2f0d549a5024113820debef2e0cbbb46ba4f7295) *(text)* Add `Masked` to display secure data by @joshka in [#168](https://github.com/ratatui-org/ratatui/pull/168) [**breaking**]

  > Adds a new type Masked that can mask data with a mask character, and can
  > be used anywhere we expect Cow<'a, str> or Text<'a>. E.g. Paragraph,
  > ListItem, Table Cells etc.
  >
  > BREAKING CHANGE:Because Masked implements From for Text<'a>, code that binds
  > Into<Text<'a>> without type annotations may no longer compile
  > (e.g. `Paragraph::new("".as_ref())`)
  >
  > To fix this, annotate or call to_string() / to_owned() / as_str()

- [c7aca64](https://github.com/ratatui-org/ratatui/commit/c7aca64ba111f29b8159c832df23c1f1a34c63b0) *(widget)* Add circle widget by @fujiapple852 in [#159](https://github.com/ratatui-org/ratatui/pull/159)

- [33b3f4e](https://github.com/ratatui-org/ratatui/commit/33b3f4e3122cc59eaa789c684dd17dae3a716be1) *(widget)* Add style methods to Span, Spans, Text by @lthoerner in [#148](https://github.com/ratatui-org/ratatui/pull/148)

  > - add patch_style() and reset_style() to `Span` and `Spans`
  > - add reset_style() to `Text`
  > - updated doc for patch_style() on `Text` for clarity.

- [782820c](https://github.com/ratatui-org/ratatui/commit/782820c34ad9b9e216262768d346dcbf633b2edf) *(widget)* Support adding padding to Block by @orhun in [#20](https://github.com/ratatui-org/ratatui/pull/20)

- [bc66a27](https://github.com/ratatui-org/ratatui/commit/bc66a27baf79d9f02fc626c037a6f754c7614b5d) *(widgets)* Add offset() and offset_mut() for table and list state by @orhun in [#12](https://github.com/ratatui-org/ratatui/pull/12)

### Bug Fixes

- [2751f08](https://github.com/ratatui-org/ratatui/commit/2751f08bdcb9d621f16088fe84ec89057c9eb4ba) *(142)* Cleanup doc example by @sophacles in [#145](https://github.com/ratatui-org/ratatui/pull/145)

- [5f1a37f](https://github.com/ratatui-org/ratatui/commit/5f1a37f0db779cd4287ca7523b02e40e92a82543) *(canvas)* Use full block for Marker::Block by @joshka in [#133](https://github.com/ratatui-org/ratatui/pull/133) [**breaking**]

- [4842885](https://github.com/ratatui-org/ratatui/commit/4842885aa64680aac5f58856345ab0e1f399827c) *(examples)* Update input in examples to only use press events by @lesleyrs in [#129](https://github.com/ratatui-org/ratatui/pull/129)

- [769efc2](https://github.com/ratatui-org/ratatui/commit/769efc20d13d29cd8991ae7014026f88f9f85144) *(reflow)* Remove debug macro call by @joshka in [#198](https://github.com/ratatui-org/ratatui/pull/198)

  > This was accidentally left in the previous commit and causes all the
  > demos to fail.

### Refactor

- [62930f2](https://github.com/ratatui-org/ratatui/commit/62930f2821d5108cd0178b709e55da356bd6aeef) *(example)* Remove redundant `vec![]` in `user_input` example by @orhun in [#26](https://github.com/ratatui-org/ratatui/pull/26)

- [26cf1f7](https://github.com/ratatui-org/ratatui/commit/26cf1f7a89f5efd6558a97e17b8ded89a2ea8c9b) *(examples)* Refactor paragraph example by @mindoodoo in [#152](https://github.com/ratatui-org/ratatui/pull/152)

- [21a029f](https://github.com/ratatui-org/ratatui/commit/21a029f17eadd5a7914b6044d45c73f790ab357e) *(style)* Mark some Style fns const so they can be defined globally by @kpcyrd in [#115](https://github.com/ratatui-org/ratatui/pull/115)

- [728f82c](https://github.com/ratatui-org/ratatui/commit/728f82c0845f7381d72e43ff7fe092a4927864f3) *(text)* Replace `Spans` with `Line` by @joshka in [#178](https://github.com/ratatui-org/ratatui/pull/178)

  > * refactor: add Line type to replace Spans
  >
  > `Line` is a significantly better name over `Spans` as the plural causes
  > confusion and the type really is a representation of a line of text made
  > up of spans.
  >
  > This is a backwards compatible version of the approach from
  > https://github.com/tui-rs-revival/ratatui/pull/175
  >
  > There is a significant amount of code that uses the Spans type and
  > methods, so instead of just renaming it, we add a new type and replace
  > parameters that accepts a `Spans` with a parameter that accepts
  > `Into<Line>`.
  >
  > Note that the examples have been intentionally left using `Spans` in
  > this commit to demonstrate the compiler warnings that will be emitted in
  > existing code.
  >
  > Implementation notes:
  > - moves the Spans code to text::spans and publicly reexports on the text
  > module. This makes the test in that module only relevant to the Spans
  > type.
  > - adds a line module with a copy of the code and tests from Spans with a
  > single addition: `impl<'a> From<Spans<'a>> for Line<'a>`
  > - adds tests for `Spans` (created and checked before refactoring)
  > - adds the same tests for `Line`
  > - updates all widget methods that accept and store Spans to instead
  > store `Line` and accept `Into<Line>`
  >
  > * refactor: move text::Masked to text::masked::Masked
  >
  > Re-exports the Masked type at text::Masked
  >
  > * refactor: replace Spans with Line in tests/examples/docs

### Documentation

- [0904d44](https://github.com/ratatui-org/ratatui/commit/0904d448e0ff7cc56a7553bf227ff4b0e24fdb43) *(apps)* Ix rsadsb/adsb_deku radar link by @wcampbell0x2a in [#140](https://github.com/ratatui-org/ratatui/pull/140)

- [fab7952](https://github.com/ratatui-org/ratatui/commit/fab7952af6fbec9ed87b3c7a840d6185ea6ce142) *(apps)* Add tenere by @pythops in [#141](https://github.com/ratatui-org/ratatui/pull/141)

- [00e8c0d](https://github.com/ratatui-org/ratatui/commit/00e8c0d1b0cd5cae26ef2b11e9e2e1b202d3f57d) *(apps)* Add twitch-tui by @Xithrius in [#124](https://github.com/ratatui-org/ratatui/pull/124)

- [a9ba23b](https://github.com/ratatui-org/ratatui/commit/a9ba23bae8398b0add4ee647516b8e094dd29e3d) *(apps)* Add oxycards by @BrookJeynes in [#113](https://github.com/ratatui-org/ratatui/pull/113)

- [4334c71](https://github.com/ratatui-org/ratatui/commit/4334c71bc7c8ac2e0738cee03b4c7014cf683cac) *(apps)* Re-add trippy to APPS.md by @fujiapple852 in [#117](https://github.com/ratatui-org/ratatui/pull/117)

- [603ec3f](https://github.com/ratatui-org/ratatui/commit/603ec3f10a2f633fedc9e510b3d6fb3bd25499f3) *(block)* Add example for block.inner by @joshka in [#158](https://github.com/ratatui-org/ratatui/pull/158)

- [9d37c3b](https://github.com/ratatui-org/ratatui/commit/9d37c3bd451ffddb39c5459799147dfe753b2c67) *(changelog)* Update the empty profile link in contributors by @orhun in [#112](https://github.com/ratatui-org/ratatui/pull/112)

- [fa02cf0](https://github.com/ratatui-org/ratatui/commit/fa02cf0f2b465612061acc491ac33606af95baa3) *(readme)* Fix small typo in readme by @a-kenji in [#186](https://github.com/ratatui-org/ratatui/pull/186)

- [60cd280](https://github.com/ratatui-org/ratatui/commit/60cd28005f5b78a53d14903131d56db4ab871c2c) *(readme)* Add termwiz demo to examples by @a-kenji in [#183](https://github.com/ratatui-org/ratatui/pull/183)

- [37bb82e](https://github.com/ratatui-org/ratatui/commit/37bb82e87d12efe7fb9e93ec538b50ae06c41b10) *(readme)* Add acknowledgement section by @mindoodoo in [#154](https://github.com/ratatui-org/ratatui/pull/154)

- [239efa5](https://github.com/ratatui-org/ratatui/commit/239efa5fbd39027dcc88f64432985d8d11175d76) *(readme)* Update project description by @Ziqi-Yang in [#127](https://github.com/ratatui-org/ratatui/pull/127)

- [49e0f4e](https://github.com/ratatui-org/ratatui/commit/49e0f4e9833ddd93ea6453d1ad0f47cd03652c3d) *(uncategorized)* Scrape example code from examples/* by @joshka in [#195](https://github.com/ratatui-org/ratatui/pull/195)

  > see https://doc.rust-lang.org/nightly/rustdoc/scraped-examples.html

### Styling

- [3e54ac3](https://github.com/ratatui-org/ratatui/commit/3e54ac3acaf7fab6e18bfce7b42fdb7d37826ac2) *(apps)* Update the style of application list by @thomas-mauran in [#184](https://github.com/ratatui-org/ratatui/pull/184)

  > * style(APPS): adding categories
  >
  > * chore(APPS): remove dots at the end of titles
  >
  > * chore(APPS): rename to other
  >
  > * chore(APPS): add description and table of contents
  >
  > * chore(APPS): change table to list and remove authors
  >
  > * style(apps): apply formatting
  >
  > * docs(apps): add the description for termchat
  >
  > ---------

- [047e0b7](https://github.com/ratatui-org/ratatui/commit/047e0b7e8d6c9c3f2c07fecfafb407466bcbb5cb) *(readme)* Update project introduction in README.md by @orhun in [#153](https://github.com/ratatui-org/ratatui/pull/153)

- [b3072ce](https://github.com/ratatui-org/ratatui/commit/b3072ce354b529fe4f2360105cfe8058d2b90fbf) *(uncategorized)* Clippy's variable inlining in format macros by @TimerErTim in [#170](https://github.com/ratatui-org/ratatui/pull/170)

### Testing

- [98b6b19](https://github.com/ratatui-org/ratatui/commit/98b6b1911cfbddf64fefb7dc8fe495e345b1632e) *(buffer)* Add `assert_buffer_eq!` and Debug implementation by @joshka in [#161](https://github.com/ratatui-org/ratatui/pull/161)

  > - The implementation of Debug is customized to make it easy to use the
  > output (particularly the content) directly when writing tests (by
  > surrounding it with `Buffer::with_lines(vec![])`). The styles part of
  > the message shows the position of every style change, rather than the
  > style of each cell, which reduces the verbosity of the detail, while
  > still showing everything necessary to debug the buffer.
  >
  > ```rust
  > Buffer {
  >     area: Rect { x: 0, y: 0, width: 12, height: 2 },
  >     content: [
  >         "Hello World!",
  >         "G'day World!",
  >     ],
  >     styles: [
  >         x: 0, y: 0, fg: Reset, bg: Reset, modifier: (empty),
  >         x: 0, y: 1, fg: Green, bg: Yellow, modifier: BOLD,
  >     ]
  > }
  > ```
  >
  > - The assert_buffer_eq! macro shows debug view and diff of the two
  > buffers, which makes it easy to understand exactly where the difference
  > is.
  >
  > - Also adds a unit test for buffer_set_string_multi_width_overwrite
  > which was missing from the buffer tests

- [548961f](https://github.com/ratatui-org/ratatui/commit/548961f61009c97f804c33d0783fcca5d10906fc) *(list)* Add characterization tests for list by @joshka in [#167](https://github.com/ratatui-org/ratatui/pull/167)

  > - also adds builder methods on list state to make it easy to construct
  >   a list state with selected and offset as a one-liner. Uses `with_` as
  >   the prefix for these methods as the selected method currently acts as
  >   a getter rather than a builder.
  > - cargo tarpaulin suggests only two lines are not covered (the two
  >   match patterns of the self.start_corner match 223 and 227).
  >   the body of these lines is covered, so this is probably 100% coverage.

- [cf1a759](https://github.com/ratatui-org/ratatui/commit/cf1a759fa57d43476b312393bd391ee3971968a5) *(widget)* Add unit tests for Paragraph by @joshka in [#156](https://github.com/ratatui-org/ratatui/pull/156)

### Miscellaneous Tasks

- [e08b466](https://github.com/ratatui-org/ratatui/commit/e08b466166821d9fb2d1774d42a33184cef0c762) *(uncategorized)* Inline format args by @nyurik in [#190](https://github.com/ratatui-org/ratatui/pull/190)

- [3f9935b](https://github.com/ratatui-org/ratatui/commit/3f9935bbcc6ab5d8271ed9acf7968bdc8f9298f7) *(uncategorized)* Minor lints, making Clippy happier by @nyurik in [#189](https://github.com/ratatui-org/ratatui/pull/189)

  > - `Default::default` is hard to read
  > - a few `map` -> `map_or`
  > - simplified `match` -> `let-if`

### Build

- [1cc405d](https://github.com/ratatui-org/ratatui/commit/1cc405d2dccb276c2b34f3c3d6b3850dccb5b5b6) *(uncategorized)* Bump MSRV to 1.65.0 by @joshka in [#171](https://github.com/ratatui-org/ratatui/pull/171)

  > The latest version of the time crate requires Rust 1.65.0
  >
  > ```
  > cargo +1.64.0-x86_64-apple-darwin test --no-default-features \
  >   --features serde,crossterm,all-widgets --lib --tests --examples
  > error: package `time v0.3.21` cannot be built because it requires rustc
  > 1.65.0 or newer, while the currently active rustc version is 1.64.0
  > ```
  >
  > Also fixes several clippy warnings added in 1.63/1.65. Although these
  > have been since moved to nursery / pedantic, it doesn't hurt to fix
  > these issues as part of this change:
  >
  > - https://rust-lang.github.io/rust-clippy/master/index.html#derive_partial_eq_without_eq (nursery)
  > - https://rust-lang.github.io/rust-clippy/master/index.html#bool_to_int_with_if (pedantic)

### Continuous Integration

- [5f12f06](https://github.com/ratatui-org/ratatui/commit/5f12f06297b8e0c66f1378bd3085b177f40fd52a) *(uncategorized)* Add ci, build, and revert to allowed commit types by @joshka in [#165](https://github.com/ratatui-org/ratatui/pull/165)

  > This is the same list as https://github.com/conventional-changelog/commitlint/tree/master/%40commitlint/config-conventional\#rules



### New Contributors
* @TimerErTim made their first contribution in [#149](https://github.com/ratatui-org/ratatui/pull/149)
* @thomas-mauran made their first contribution in [#184](https://github.com/ratatui-org/ratatui/pull/184)
* @Mehrbod2002 made their first contribution in [#180](https://github.com/ratatui-org/ratatui/pull/180)
* @fujiapple852 made their first contribution in [#159](https://github.com/ratatui-org/ratatui/pull/159)
* @kyoto7250 made their first contribution in [#24](https://github.com/ratatui-org/ratatui/pull/24)
* @lthoerner made their first contribution in [#148](https://github.com/ratatui-org/ratatui/pull/148)
* @sophacles made their first contribution in [#138](https://github.com/ratatui-org/ratatui/pull/138)
* @pythops made their first contribution in [#141](https://github.com/ratatui-org/ratatui/pull/141)
* @lesleyrs made their first contribution in [#129](https://github.com/ratatui-org/ratatui/pull/129)
* @Xithrius made their first contribution in [#124](https://github.com/ratatui-org/ratatui/pull/124)
* @Ziqi-Yang made their first contribution in [#127](https://github.com/ratatui-org/ratatui/pull/127)
* @BrookJeynes made their first contribution in [#113](https://github.com/ratatui-org/ratatui/pull/113)
* @kpcyrd made their first contribution in [#115](https://github.com/ratatui-org/ratatui/pull/115)

**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.20.1...v0.21.0


## [v0.20.1](https://github.com/ratatui-org/ratatui/releases/tag/v0.20.1) - 2023-03-22

### Bug Fixes

- [26c7bcf](https://github.com/ratatui-org/ratatui/commit/26c7bcfdd5a87e3f6d7018cde13c8a237936a6e3) *(style)* Bold needs a bit by @UncleScientist in [#104](https://github.com/ratatui-org/ratatui/pull/104)

### Documentation

- [cf1b05d](https://github.com/ratatui-org/ratatui/commit/cf1b05d16eda0abe7fd68d02510d057e69590d81) *(apps)* Add "logss" to apps by @todoesverso in [#105](https://github.com/ratatui-org/ratatui/pull/105)

- [2da4c10](https://github.com/ratatui-org/ratatui/commit/2da4c103844d96aa686e8de844c81cc2f520a0bd) *(uncategorized)* Fixup remaining tui references by @joshka in [#106](https://github.com/ratatui-org/ratatui/pull/106)



### New Contributors
* @todoesverso made their first contribution in [#105](https://github.com/ratatui-org/ratatui/pull/105)

**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.20.0...v0.20.1


## [v0.20.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.20.0) - 2023-03-19

### Features

- [d38d185](https://github.com/ratatui-org/ratatui/commit/d38d185d1c3b9a7096a0baeaf707f1fbca44527e) *(cd)* Add continuous deployment workflow by @orhun in [#93](https://github.com/ratatui-org/ratatui/pull/93)

- [142bc57](https://github.com/ratatui-org/ratatui/commit/142bc5720e82123c5a136159f0e30fbdbc66c6f7) *(ci)* Add MacOS to CI by @rhysd in [#60](https://github.com/ratatui-org/ratatui/pull/60)

- [9b7a6ed](https://github.com/ratatui-org/ratatui/commit/9b7a6ed85d09a0b7147f357bc6e068ed02686373) *(widget)* Add `offset()` to `TableState` by @orhun in [#10](https://github.com/ratatui-org/ratatui/pull/10)

- [e15a614](https://github.com/ratatui-org/ratatui/commit/e15a6146f81b497370e22669c58d04dc8b54fc7c) *(widget)* Add `width()` to ListItem by @orhun in [#17](https://github.com/ratatui-org/ratatui/pull/17)

### Bug Fixes

- [33acfce](https://github.com/ratatui-org/ratatui/commit/33acfce083a7a15ab8d7dbb48f572fccd6718b9b) *(ci)* Test MSRV compatibility on CI by @rhysd in [#85](https://github.com/ratatui-org/ratatui/pull/85)

- [73f7f16](https://github.com/ratatui-org/ratatui/commit/73f7f16298b75e015e0e80c7c82208b5b420e78a) *(ci)* Bump Rust version to 1.63.0 by @orhun in [#80](https://github.com/ratatui-org/ratatui/pull/80)

- [1c0ed32](https://github.com/ratatui-org/ratatui/commit/1c0ed3268b1f75e351b8547e1c7d223429c1ebe1) *(ci)* Use env for the cargo-make version by @orhun in [#76](https://github.com/ratatui-org/ratatui/pull/76)

- [feaeb78](https://github.com/ratatui-org/ratatui/commit/feaeb7870fafaaa338158b904a5a1d090941ee6f) *(ci)* Fix deprecation warnings on CI by @rhysd in [#58](https://github.com/ratatui-org/ratatui/pull/58)

  > * fix(ci): fix deprecation warnings on CI
  >
  > * fix(ci): remove unnecessary step in CI workflow

- [e49cb11](https://github.com/ratatui-org/ratatui/commit/e49cb1126b3e097748bcf2b1f8338d3ef356fb55) *(doc)* Add 3rd party libraries accidentally removed at #21 by @rhysd in [#61](https://github.com/ratatui-org/ratatui/pull/61)

- [0456abb](https://github.com/ratatui-org/ratatui/commit/0456abb32725c29849e9b67b1baf436736ee7b39) *(widget)* List should not ignore empty string items by @orhun in [#42](https://github.com/ratatui-org/ratatui/pull/42) [**breaking**]

  > Fixes issue #680. Handles the case where a list item is created with an empty string, which is not split by the lines iterator.

- [0dc3943](https://github.com/ratatui-org/ratatui/commit/0dc39434c2527647b32b6f72f2d24f244fecabf4) *(uncategorized)* Cassowary/layouts: add extra constraints for fixing Min(v)/Max(v) combination. by @orhun in [#31](https://github.com/ratatui-org/ratatui/pull/31)

- [d3df8fe](https://github.com/ratatui-org/ratatui/commit/d3df8fe7ef7deea0fc00cfbe20be1f4dccd31fec) *(uncategorized)* Fix user_input example double key press registered on windows by @orhun in [#44](https://github.com/ratatui-org/ratatui/pull/44)

- [9534d53](https://github.com/ratatui-org/ratatui/commit/9534d533e371999d32ef00e5fe67ae0320145977) *(uncategorized)* Ignore zero-width symbol on rendering `Paragraph` by @orhun in [#30](https://github.com/ratatui-org/ratatui/pull/30)

  > This fixes out-of-bounds crash on rendering `Paragraph` when zero-width
  > character is at end of line.
  >
  > fix #642

- [33087e3](https://github.com/ratatui-org/ratatui/commit/33087e3a99d8cc2de01130a55bf9962990fff560) *(uncategorized)* Fix typos by @orhun in [#45](https://github.com/ratatui-org/ratatui/pull/45)

- [fe4eb5e](https://github.com/ratatui-org/ratatui/commit/fe4eb5e771302992fe94f741f63baf2d63f3937c) *(uncategorized)* Fix typos by @UncleScientist in [#47](https://github.com/ratatui-org/ratatui/pull/47)

### Refactor

- [7e31035](https://github.com/ratatui-org/ratatui/commit/7e310351144594566ba59e091589fcc1627aae7e) *(style)* Make bitflags smaller by @orhun in [#13](https://github.com/ratatui-org/ratatui/pull/13)

### Documentation

- [ccd142d](https://github.com/ratatui-org/ratatui/commit/ccd142df973a1c4b7181282b26bba03eced7a86f) *(apps)* Move 'apps using ratatui' to dedicated file by @orhun in [#99](https://github.com/ratatui-org/ratatui/pull/99)

- [33e67ab](https://github.com/ratatui-org/ratatui/commit/33e67abbe9adc16d2deb5f545545edd0fb49f91c) *(canvas)* Add documentation for x_bounds, y_bounds by @orhun in [#35](https://github.com/ratatui-org/ratatui/pull/35)

- [454c845](https://github.com/ratatui-org/ratatui/commit/454c8459f29d45117134c99062784994e9d8a1b9) *(contributing)* Specify the use of unsafe for optimization by @mindoodoo in [#67](https://github.com/ratatui-org/ratatui/pull/67)

- [ffb3de6](https://github.com/ratatui-org/ratatui/commit/ffb3de6c362cc169083937e341aceee207a6541d) *(github)* Remove pull request template by @mindoodoo in [#68](https://github.com/ratatui-org/ratatui/pull/68)

- [f0c0985](https://github.com/ratatui-org/ratatui/commit/f0c0985708423ff8b63bb05d7df1437d99563020) *(readme)* Update crate status badge by @orhun in [#102](https://github.com/ratatui-org/ratatui/pull/102)

- [73c937b](https://github.com/ratatui-org/ratatui/commit/73c937bc30a1303892091f9fe1e598ab39b0d3aa) *(readme)* Small edits before first release by @mindoodoo in [#101](https://github.com/ratatui-org/ratatui/pull/101)

- [c20fb72](https://github.com/ratatui-org/ratatui/commit/c20fb723ef7d9226cd6eda9e3f9abdc51f841226) *(readme)* Add install instruction and update title by @sayanarijit in [#100](https://github.com/ratatui-org/ratatui/pull/100)

- [f6dbd1c](https://github.com/ratatui-org/ratatui/commit/f6dbd1c0b5a680f4d6bb38cdf7b466c2524359f8) *(readme)* Add systeroid to application list by @orhun in [#92](https://github.com/ratatui-org/ratatui/pull/92)

- [ef79b72](https://github.com/ratatui-org/ratatui/commit/ef79b72471d102e07a642b2b63d205fc53de5f07) *(readme)* Add glicol-cli to showcase list by @chaosprint in [#95](https://github.com/ratatui-org/ratatui/pull/95)

- [ec50458](https://github.com/ratatui-org/ratatui/commit/ec504584914abbde8eda45b8b8a4069224d03556) *(readme)* Add oxker to application list by @mrjackwills in [#74](https://github.com/ratatui-org/ratatui/pull/74)

- [b834cca](https://github.com/ratatui-org/ratatui/commit/b834ccaa2f153b46da5c28e05dfc9154b763d0b9) *(readme)* Add app kubectl-watch which uses tui by @imuxin in [#73](https://github.com/ratatui-org/ratatui/pull/73)

- [94a0d09](https://github.com/ratatui-org/ratatui/commit/94a0d09591231084131d5a773a1493d01f14c2ed) *(readme)* Add poketex to 'apps using tui' in README by @ckaznable in [#64](https://github.com/ratatui-org/ratatui/pull/64)

- [bf9d502](https://github.com/ratatui-org/ratatui/commit/bf9d502742072fbc04598d27b3c4905d6f99342c) *(readme)* Update README.md by @orhun in [#39](https://github.com/ratatui-org/ratatui/pull/39)

- [85c0779](https://github.com/ratatui-org/ratatui/commit/85c0779ac09e2c073d3dd4f4f6c48df418e146aa) *(readme)* Update README.md by @orhun in [#40](https://github.com/ratatui-org/ratatui/pull/40)

- [2fead23](https://github.com/ratatui-org/ratatui/commit/2fead23556abb083cca267773564187b990fcffd) *(readme)* Clarify README.md fork status update by @Owletti in [#48](https://github.com/ratatui-org/ratatui/pull/48)

  > docs(readme): Clarify README.md

- [e10f626](https://github.com/ratatui-org/ratatui/commit/e10f62663edd50b065acc74e1e339dd4fe513981) *(uncategorized)* Fix: fix typos by @kianmeng in [#90](https://github.com/ratatui-org/ratatui/pull/90)

- [79d0ead](https://github.com/ratatui-org/ratatui/commit/79d0eadbd6cad65f9141d47bca8a78b190c49e6a) *(uncategorized)* Update to build more backends by @sayanarijit in [#81](https://github.com/ratatui-org/ratatui/pull/81)

- [85eefe1](https://github.com/ratatui-org/ratatui/commit/85eefe1d8b10a2c8e5d93e9869352dea2348d67c) *(uncategorized)* Expand "Apps" and "Third-party" sections by @orhun in [#21](https://github.com/ratatui-org/ratatui/pull/21)

  > * README.md/#Apps using tui: Sort lexically
  >
  > * README.md/#Apps using tui: Remove Hoard.
  >
  > Closes #569.
  >
  > * README.md/#Apps using tui: Add a description to each app.
  >
  > * README.md/#Apps using tui: Add some more apps.
  >
  > This is a curated addition.
  >
  > Here are the apps I chose not to add:
  >
  > ```md
  > * [Chatui](https://github.com/xaerru/chatui) — ChatApp made using the standard library net module and Tui-rs
  > * [Corona-rs](https://github.com/varjolintu/corona-rs) — Corona virus statistics with Tui-rs
  > * [HTTP Request Tool](https://github.com/Callum-Irving/http-request-tool) — HTTP request sending tool similar to Insomnia, but uses a text user interface (TUI)
  > * [KRTirtho/portfolio](https://github.com/KRTirtho/portfolio) — A TUI based personal portfolio created using Rust & Tui-rs
  > * [Picterm](https://github.com/ksk001100/picterm) — TUI image viewer
  > ```
  >
  > * README.md/#Third party: add more projects
  >
  > * Undo the capitalization
  >
  > ---------

- [3343270](https://github.com/ratatui-org/ratatui/commit/3343270680a56f14997f7328d5ad0417bf3a5f74) *(uncategorized)* Add tui-input and update xplr in README.md by @orhun in [#37](https://github.com/ratatui-org/ratatui/pull/37)

  > Also update xplr description.

- [070de44](https://github.com/ratatui-org/ratatui/commit/070de44069522e71b85e59c8566b88cd6989878d) *(uncategorized)* Add hncli to list of applications made with tui-rs by @orhun in [#41](https://github.com/ratatui-org/ratatui/pull/41)

- [bc5a9e4](https://github.com/ratatui-org/ratatui/commit/bc5a9e4c063bd1bc7e27c355f7d1af2a580899b4) *(uncategorized)* Updated readme and contributing guide with updates about the fork by @mindoodoo in [#46](https://github.com/ratatui-org/ratatui/pull/46)

  > * doc: Updated readme and contributing guide with updates about the fork
  >
  > * doc: added missing discord invite link

### Performance

- [02573b0](https://github.com/ratatui-org/ratatui/commit/02573b0ad2f0017838062d8719a9dc74d01b1dc9) *(layout)* Better safe shared layout cache by @conradludgate in [#62](https://github.com/ratatui-org/ratatui/pull/62)

### Miscellaneous Tasks

- [ed12ab1](https://github.com/ratatui-org/ratatui/commit/ed12ab16e0550298fe4ddee993287bf0ff4d7641) *(cargo)* Update project metadata by @orhun in [#94](https://github.com/ratatui-org/ratatui/pull/94)

- [24820cf](https://github.com/ratatui-org/ratatui/commit/24820cfcff95ab1f0308ab2e8b2857469e70c22f) *(ci)* Integrate `typos` for checking typos by @orhun in [#91](https://github.com/ratatui-org/ratatui/pull/91)

- [66eb0e4](https://github.com/ratatui-org/ratatui/commit/66eb0e42fe6b09a197a79b5d74b9644dd0b12aa7) *(ci)* Change the target branch to main by @orhun in [#79](https://github.com/ratatui-org/ratatui/pull/79)

- [9df0eef](https://github.com/ratatui-org/ratatui/commit/9df0eefe49fe63f2033e9c77d449a9e03c3c24d1) *(ci)* Re-enable clippy on CI by @rhysd in [#59](https://github.com/ratatui-org/ratatui/pull/59)

- [052ae53](https://github.com/ratatui-org/ratatui/commit/052ae53b6e4bd14fbf02fefbd912d01a55a76f34) *(uncategorized)* Integrate `committed` for checking conventional commits by @orhun in [#77](https://github.com/ratatui-org/ratatui/pull/77)
  >
  > Closes #50

- [8e89a93](https://github.com/ratatui-org/ratatui/commit/8e89a9377a7588832452b6f23788112e68f3d3b0) *(uncategorized)* Update `rust-version` to 1.59 in Cargo.toml by @rhysd in [#57](https://github.com/ratatui-org/ratatui/pull/57)

- [9feda98](https://github.com/ratatui-org/ratatui/commit/9feda988a51ec2bd97e83e0bbfd63af630059669) *(uncategorized)* Update deps by @sayanarijit in [#51](https://github.com/ratatui-org/ratatui/pull/51)

  > * Update deps
  >
  > Also, temporarily disabled clippy check. Can be discussed in #49.
  >
  > * Fix termion demo
  >
  > * chore: fix all clippy warnings
  >
  > * Call into_raw_mode()
  >
  > * Update min supported rust version
  >
  > ---------

- [fafad6c](https://github.com/ratatui-org/ratatui/commit/fafad6c96109610825aad89c4bba5253e01101ed) *(uncategorized)* Fix typo in layout.rs by @davidhelbig

- [a4de409](https://github.com/ratatui-org/ratatui/commit/a4de409235feba9f4e47f0fd39406adaf3862d70) *(uncategorized)* Add apps using `tui` by @fdehau



### New Contributors
* @chaosprint made their first contribution in [#95](https://github.com/ratatui-org/ratatui/pull/95)
* @kianmeng made their first contribution in [#90](https://github.com/ratatui-org/ratatui/pull/90)
* @imuxin made their first contribution in [#73](https://github.com/ratatui-org/ratatui/pull/73)
* @ckaznable made their first contribution in [#64](https://github.com/ratatui-org/ratatui/pull/64)
* @Owletti made their first contribution in [#48](https://github.com/ratatui-org/ratatui/pull/48)
* @davidhelbig made their first contribution

**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.19.0...v0.20.0


## [v0.19.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.19.0) - 2022-08-14

### Miscellaneous Tasks

- [a67706b](https://github.com/ratatui-org/ratatui/commit/a67706bea0b342a92f391bd34425c5b1a42ceff0) *(ci)* Bump cargo-make to v0.35.16 by @fdehau

- [24de2f8](https://github.com/ratatui-org/ratatui/commit/24de2f8a96970d871330597dd719cef8f482e343) *(uncategorized)* Bump crossterm to v0.25 by @fdehau

- [eee3701](https://github.com/ratatui-org/ratatui/commit/eee37011a543d2e1a9373f304819512ccf3e80d0) *(uncategorized)* Fix clippy warnings by @fdehau

- [faa69b6](https://github.com/ratatui-org/ratatui/commit/faa69b6cfe034ec302253d7b5c75c25cc2c376e8) *(uncategorized)* Explicitly set MSRV to 1.56.1 in Cargo.toml by @rhysd

- [ba5ea2d](https://github.com/ratatui-org/ratatui/commit/ba5ea2deff72fbfd568d4589bfd7ff627959e8cc) *(uncategorized)* Update README by @fdehau

- [a6b25a4](https://github.com/ratatui-org/ratatui/commit/a6b25a487786534205d818a76acb3989658ae58c) *(uncategorized)* Add panic hook example by @wookietreiber

  > Without a terminal-resetting panic hook there are two main problems when
  > an application panics:
  >
  > 1.  The report of the panic is distorted because the terminal has not
  >     properly left the alternate screen and is still in raw mode.
  >
  > 2.  The terminal needs to be manually reset with the `reset` command.
  >
  > To avoid this, the standard panic hook can be extended to first reset
  > the terminal.

- [90d8cb6](https://github.com/ratatui-org/ratatui/commit/90d8cb65261247d87fe5f096c32635c1b2049d9c) *(uncategorized)* Add more apps using `tui` to the README by @fdehau



### New Contributors
* @wookietreiber made their first contribution

**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.18.0...v0.19.0


## [v0.18.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.18.0) - 2022-04-24

### Miscellaneous Tasks

- [ed0ae81](https://github.com/ratatui-org/ratatui/commit/ed0ae81aaef466400e9616826d43bed0c2951b81) *(uncategorized)* Update crossterm to v0.23 by @alpha-tango-kilo

- [a61b078](https://github.com/ratatui-org/ratatui/commit/a61b078dea36f526a32c2b3738dc3462e8a15c15) *(uncategorized)* Fix clippy warning by @wendajiang



### New Contributors
* @alpha-tango-kilo made their first contribution
* @wendajiang made their first contribution

**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.17.0...v0.18.0


## [v0.17.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.17.0) - 2022-01-22

### Features

- [853d904](https://github.com/ratatui-org/ratatui/commit/853d9047b0f83e50dabbfc0359692e44bbbb5f1a) *(widgets/chart)* Add option to control alignment of axis labels by @theogilbert

  > * feat(chart): allow custom alignment of first X-Axis label
  >
  > * refactor(chart): rename ambiguous function parameter
  >
  > * feat(chart): allow custom alignment of Y-Axis labels
  >
  > * refactor(chart): refactor axis test cases
  >
  > * refactor(chart): rename minor variable
  >
  > * fix(chart): force centered x-axis label near Y-Axis
  >
  > * fix(chart): fix subtract overflow on small rendering area
  >
  > * refactor(chart): rename alignment property
  >
  > * refactor(chart): merge two nested conditions
  >
  > * refactor(chart): decompose x labels rendering loop

- [4845c03](https://github.com/ratatui-org/ratatui/commit/4845c03eec4beae395adca7b38cab0044c710465) *(widgets/list)* Repeat highlight symbol on multi-line items by @abusch

  > When this option is true, the hightlight symbol is repeated for each
  > line of the selected item, instead of just the first line.

- [cf2d9c2](https://github.com/ratatui-org/ratatui/commit/cf2d9c2c1d53212fb2bdd62939bbf8b50b68071b) *(uncategorized)* Bump MSRV to 1.56.1 and migrate to edition 2021 by @fdehau [**breaking**]

- [9806217](https://github.com/ratatui-org/ratatui/commit/9806217a6a4c240462bba3b32cb1bc59524f1bc2) *(uncategorized)* Use crossterm as default backend by @fdehau [**breaking**]

### Documentation

- [90c4da4](https://github.com/ratatui-org/ratatui/commit/90c4da4e687c365afeccd8dd8808a2d5d0ef6ba2) *(readme)* Update README.md by @Hyde46

### Miscellaneous Tasks

- [ef583ce](https://github.com/ratatui-org/ratatui/commit/ef583cead9395f6336d5fe00fc8991b188f22edd) *(examples)* Remove unused `demo/util.rs` by @JarvisCraft

  > This module is unused and is not imported by any other module.

- [6069d89](https://github.com/ratatui-org/ratatui/commit/6069d89dee8e43b99c3938c2f747c5274ce4aef0) *(uncategorized)* Fix all clippy warnings by @fdehau

- [d25e263](https://github.com/ratatui-org/ratatui/commit/d25e263b8e11e5e2bedc90ea3edb92202be58274) *(uncategorized)* Enable clippy on all targets by @fdehau

- [d05e696](https://github.com/ratatui-org/ratatui/commit/d05e696d456cc8af23fd7252acba2d3270a10b65) *(uncategorized)* Fix optional attribute for `serde` feature by @ljedrz

- [8032191](https://github.com/ratatui-org/ratatui/commit/803219136682f6f669d93634750c6f4ba4c37f9d) *(uncategorized)* Fix table example by @fdehau

  > Third column in table example was using the `Max` constraint.
  >
  > But since version 0.16, the layout system does not add a hidden constraint on the last column which would ensure that it fills the remaining available space (a change that was already mentioned in #525). In addition, `tui` does not support sizing based on content because of its immediate mode nature. Therefore, `Max` is now resolved to `0`. Replacing with `Min` fixes the issue.
  >
  > A new way of specifying constraints is being worked on at #519 which should for more deterministic and advanced layout.

- [c8c0329](https://github.com/ratatui-org/ratatui/commit/c8c03294e101e0005333553964dd15e3530f409a) *(uncategorized)* Self contained examples by @fdehau

- [e00df22](https://github.com/ratatui-org/ratatui/commit/e00df225889606e8aad4c10a49960a5b1b5125d3) *(uncategorized)* Add `adsb_deku/radar` to apps using `tui` by @wcampbell0x2a

  > My `adsb_deku/radar` application uses tui, using the Table and Canvas to show
  > information and plot airplanes on a latitude/longitude coordinates map.

- [1be5cf2](https://github.com/ratatui-org/ratatui/commit/1be5cf2d90e9e6de571ea937ebaa5c2e14d13086) *(uncategorized)* Add `joshuto` to the apps using `tui` by @fdehau

- [8c1f580](https://github.com/ratatui-org/ratatui/commit/8c1f58079f4917a75f6818ac6fda6aaf4ecc55d6) *(uncategorized)* Fix build by @fdehau

- [532a595](https://github.com/ratatui-org/ratatui/commit/532a595c4105f5787d5ee5e6e42bef8a91a680cf) *(uncategorized)* Pin bitflags version to 1.3 by @fdehau

- [25ce5bc](https://github.com/ratatui-org/ratatui/commit/25ce5bc90b7983336e35fe28ee6020c2ba752028) *(uncategorized)* Bump the minimum supported Rust version to 1.52.1 by @abusch

  > - `const_fn` usage in the `bitflags` crate.
  > - `unsafe_op_in_unsafe_fn` lint usage in `rust_info` despite pinned `cargo-make` version.

- [80a929c](https://github.com/ratatui-org/ratatui/commit/80a929ccc65c03fd751f150145807b32f26d60af) *(uncategorized)* Fix typo

- [3797863](https://github.com/ratatui-org/ratatui/commit/3797863e14ea0047168f52496682f04ecf73b656) *(uncategorized)* Add termscp to list of apps using tui by @veeso



### New Contributors
* @theogilbert made their first contribution
* @ljedrz made their first contribution
* @JarvisCraft made their first contribution
* @Hyde46 made their first contribution
* @veeso made their first contribution

**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.16.0...v0.17.0


## [v0.16.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.16.0) - 2021-08-01

### Features

- [703e41c](https://github.com/ratatui-org/ratatui/commit/703e41cd49079dcc3b135011651eeb2a39f5c684) *(text)* Add a From<Cow<str>> impl for Text by @eminence

- [a346704](https://github.com/ratatui-org/ratatui/commit/a346704cdc3c176bbfbe57bac7b004301e3a6dbf) *(block)* Add option to center and right align the title by @olekslitus

  > * Added ability to set title alignment, added tests, modified blocks example to show the feature
  >
  > * Added test for inner with title in block
  >
  > * Updated block example to show center alignment
  >
  > * Formatting fixed
  >
  > * Updated tests to use lamdas and be more concise. Updated title alignmnet code to be more straightforward and have correct behavior when placing title in the center without left border

- [e870e5d](https://github.com/ratatui-org/ratatui/commit/e870e5d8a51cede29d3e4b40adefd660d90d0029) *(layout)* Add private option to control last chunk expansion by @fdehau

### Bug Fixes

- [a68e38e](https://github.com/ratatui-org/ratatui/commit/a68e38e59e6735c0a99139303b1609669d2c38da) *(table)* Use `Layout` in table column widths computation by @fdehau

- [a7c21a9](https://github.com/ratatui-org/ratatui/commit/a7c21a972921d9a6008f59fc2bc8f7a08daec936) *(widgets)* Avoid offset panic in `Table` and `List` when input changes by @fdehau

- [34a2be6](https://github.com/ratatui-org/ratatui/commit/34a2be6458776631c3b9637c644dc2bb5ea1c53a) *(widgets/chart)* Remove panics with long axis labels by @fdehau

- [38dcddb](https://github.com/ratatui-org/ratatui/commit/38dcddb12664f8939c90fe8b652037027ad848d8) *(widgets/gauge)* Apply label style and avoid overflow on large labels by @fdehau

- [a1c3ba2](https://github.com/ratatui-org/ratatui/commit/a1c3ba20881f7e7075d518db0091c2ca1785f692) *(uncategorized)* Actually clear buffer in TestBackend::clear by @scvalex

- [d47565b](https://github.com/ratatui-org/ratatui/commit/d47565be5c8c48e20627ad074d934e70a4a774d5) *(uncategorized)* Actually clear buffer in TestBackend::clear by @scvalex

### Refactor

- [8da5f74](https://github.com/ratatui-org/ratatui/commit/8da5f740af08c769dd9567dd2283553563883da4) *(examples)* Show more use case in gauge example by @fdehau

- [23d5fbd](https://github.com/ratatui-org/ratatui/commit/23d5fbde56c1b4354ef7991ec1f98d9e2896be2c) *(examples)* Remove exit key from Events handler by @fdehau

  > The thread spawned by `Events` to listen for keyboard inputs had knowlegde of
  > the exit key to exit on its own when it was pressed. It is however a source of
  > confusion  because the exit behavior is wired in both the event handler
  > and the input handling performed by the app. In addition, this is not needed as
  > the thread will exit anyway when the main thread finishes as it is already the
  > case for the "tick" thread. Therefore, this commit removes both the option to
  > configure the exit key in the `Events` handler and the option to temporarily
  > ignore it.

- [8eb6336](https://github.com/ratatui-org/ratatui/commit/8eb6336f5e0f5d3d7158e929a167e693f211b991) *(widgets)* Remove iter::repeat for blank symbols by @fdehau

### Documentation

- [fbd8344](https://github.com/ratatui-org/ratatui/commit/fbd834469fd99c7408a20ed09c5e205fe994c678) *(widgets/clear)* Clarify usage of clear by @fdehau

- [a3a0a80](https://github.com/ratatui-org/ratatui/commit/a3a0a80a020f01a4138fc4722aebd15f862e0744) *(uncategorized)* Add gpg-tui to the "apps using tui" list by @orhun

- [a5f7019](https://github.com/ratatui-org/ratatui/commit/a5f7019b2a8eab1cd4267d1583467e9c8fe78d9a) *(uncategorized)* Fix minor grammatical errors by @jmrgibson

  > A missing "and" after "an" (which I do all the time) and some tense clarification.

- [e05b80c](https://github.com/ratatui-org/ratatui/commit/e05b80cec1f48beb022b3ec5e1c61cd5c3343be6) *(uncategorized)* Fix: fix typos in comments. by @mschulteg

- [24396d9](https://github.com/ratatui-org/ratatui/commit/24396d97edb75a2562ac6ffdb8f113d4362e26dc) *(uncategorized)* Add doctests that shows how Text can be created from Cow<str> by @eminence

### Miscellaneous Tasks

- [914d54e](https://github.com/ratatui-org/ratatui/commit/914d54e672e19d68a77f8bbbfd0fe1898d877cdd) *(uncategorized)* Bump crossterm to 0.20 by @fdehau

- [92948d2](https://github.com/ratatui-org/ratatui/commit/92948d23943d007ecbffd8e528f54e4e801b5a46) *(uncategorized)* Add minesweep to list of apps using tui-rs by @cpcloud

- [975c416](https://github.com/ratatui-org/ratatui/commit/975c4165d0d534c42c567c1d1051186bfe8a5792) *(uncategorized)* Fix clippy warnings by @fdehau

- [91a2519](https://github.com/ratatui-org/ratatui/commit/91a2519cc337ea3c3dd69d7dfbd49eb954be3a05) *(uncategorized)* Update links to examples in README by @fdehau

  > Links now include the fully qualified domain as well as the version.
  > This will make them work in docs.rs and make sure readers are looking at code which is consistent with the latest version available.

- [1028d39](https://github.com/ratatui-org/ratatui/commit/1028d39db0946736b3c11a7c641f425dbf269030) *(uncategorized)* Improve contributing guidelines by @fdehau

  > * Improve issue templates and make them mandatory.
  > * Improve CONTRIBUTING.md.
  > * Add template for pull requests.



### New Contributors
* @cpcloud made their first contribution
* @jmrgibson made their first contribution
* @mschulteg made their first contribution
* @olekslitus made their first contribution
* @eminence made their first contribution
* @scvalex made their first contribution

**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.15.0...v0.16.0


## [v0.15.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.15.0) - 2021-05-02

### Features

- [67e996c](https://github.com/ratatui-org/ratatui/commit/67e996c5f41397c3c5a162937b9875b45de9bcef) *(examples)* Add third tab to demo to show colors by @fdehau

- [853f3d9](https://github.com/ratatui-org/ratatui/commit/853f3d9200da1a76a2fba24af098ff037d11ad53) *(terminal)* Add a read-only view of the terminal state after the draw call by @fdehau

### Bug Fixes

- [3a843d5](https://github.com/ratatui-org/ratatui/commit/3a843d50744c22647a6c010535fb4adf8112b870) *(test)* Remove compile warning in TestBackend::assert_buffer by @jjpe

### Miscellaneous Tasks

- [414386e](https://github.com/ratatui-org/ratatui/commit/414386e7972be9c366962c7b96e636d5a2a1298c) *(uncategorized)* Update `rand` to 0.8 by @fdehau

- [4e76bfa](https://github.com/ratatui-org/ratatui/commit/4e76bfa2ca8eb51719d611bb8d3d4094ab8ba398) *(uncategorized)* Add Vector to list of apps using tui



### New Contributors
* @jjpe made their first contribution

**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.14.0...v0.15.0


## [v0.14.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.14.0) - 2021-01-01

### Features

- [a15ac88](https://github.com/ratatui-org/ratatui/commit/a15ac8870bc05bfa8bd35cb77c8ed00efaecfeaa) *(style)* Add a method to create a style that reset all properties until that point by @fdehau

- [efdd6bf](https://github.com/ratatui-org/ratatui/commit/efdd6bfb193dafcb5e3bdc75e7d2d314065da1d7) *(tests)* Add tests covering new table features by @fdehau

- [0a05579](https://github.com/ratatui-org/ratatui/commit/0a05579a1c429c71f7aded447649bf65f49c0a23) *(widgets/gauge)* Allow gauge to use unicode block for more descriptive progress by @marshoepial

  > * gauge now uses unicode blocks for more descriptive progress
  >
  > * removed unnecessary if
  >
  > * changed function name to better reflect unicode
  >
  > * standardized block symbols, added no unicode option, added tests
  >
  > * formatting
  >
  > * improved readability
  >
  > * gauge tests now check color
  >
  > * formatted

### Bug Fixes

- [0030eb4](https://github.com/ratatui-org/ratatui/commit/0030eb4a133276046591725d3eaf3cee6d520adb) *(tests)* Remove clippy warnings about single char push by @orf

- [eb1e3be](https://github.com/ratatui-org/ratatui/commit/eb1e3be7228509e42cbcbaef610e6bd5c5f64ba6) *(widgets/block)* Make Block::inner return more accurate results on small areas by @fdehau

- [8a27036](https://github.com/ratatui-org/ratatui/commit/8a27036a54212db3a8df70e2bf2d2675959866f9) *(widgets/block)* Allow Block to render on small areas by @fdehau

- [5bf4034](https://github.com/ratatui-org/ratatui/commit/5bf40343ebfb9a28bfcade235bbadafe5167d648) *(widgets/paragraph)* Handle trailing nbsp in wrapped text by @pm100

- [7424339](https://github.com/ratatui-org/ratatui/commit/74243394d90ea1316b6bedac6c9e4f26971c76b6) *(widgets/table)* Draw table header and border even if rows are empty by @vchekan

### Refactor

- [117098d](https://github.com/ratatui-org/ratatui/commit/117098d2d23394b84ba63bdf5d83c284d459acfd) *(examples)* Add missing margin at the bottom of the header of table in the demo by @fdehau

- [79e27b1](https://github.com/ratatui-org/ratatui/commit/79e27b1778b0c2ddd8eecc1e64f3b54c84c8c930) *(widgets/gauge)* Stop using unicode blocks by default by @fdehau

- [5ea5479](https://github.com/ratatui-org/ratatui/commit/5ea54792c0f61f7498101ee2eb425d22ab229758) *(widgets/table)* More flexible table by @fdehau

  > - control over the style of each cell and its content using the styling capabilities of Text.
  > - rows with multiple lines.
  > - fix panics on small areas.
  > - less generic type parameters.

### Documentation

- [77c6e10](https://github.com/ratatui-org/ratatui/commit/77c6e106e4b1335f5016f0d0df33f6cd9b7e6a3a) *(examples)* Add comments to "list" example and fix list direction by @Nukesor

  > * Add docs to list example and fix list direction
  >
  > * List example: review adjustments and typo fixes

- [1e35f98](https://github.com/ratatui-org/ratatui/commit/1e35f983c49b4766f95dc5fb649f1f899e09d287) *(style)* Improve documentation of Style by @fdehau

### Miscellaneous Tasks

- [e7f263e](https://github.com/ratatui-org/ratatui/commit/e7f263efa74ebab9b44d162e62e98d60f0ad7c00) *(ci)* Fix cargo-make cache on windows runner by @fdehau

- [0991145](https://github.com/ratatui-org/ratatui/commit/0991145c5822e340ca273a82670d2d13ca306e6e) *(ci)* Simplify ci workflow by @fdehau

  > * chore(ci): simplify ci workflow
  >
  > * use more up to date action
  > * restrict actions allowed to run
  > * cache cargo-make

- [01d2a85](https://github.com/ratatui-org/ratatui/commit/01d2a8588a10c68645ae1d8be9aad8c4b22c9314) *(ci)* Reduce the number of triggered jobs by @fdehau

- [4ec902b](https://github.com/ratatui-org/ratatui/commit/4ec902b96f850891b495ee610568d86d5ad58b04) *(uncategorized)* Make run-examples available on all platforms by @sagiegurari

  > * Make examples available for all platforms
  > * limit windows to crossterm_demo only and make q exit demos work

- [45431a2](https://github.com/ratatui-org/ratatui/commit/45431a264951219497a02c9c2a3f990ec912ff97) *(uncategorized)* Add first contributing guidelines by @fdehau

- [0b78fb9](https://github.com/ratatui-org/ratatui/commit/0b78fb9201fcde2ea26ab9178269ac61335cf967) *(uncategorized)* Use `cargo-make` in the CI as well by @fdehau

- [9cdff27](https://github.com/ratatui-org/ratatui/commit/9cdff275cbeeb4bb4942f147602018cb49ba88c7) *(uncategorized)* Replace `make` with `cargo-make` by @fdehau

  > `cargo-make` make it easier to provide developers of all platforms an unified build workflow.

- [f933d89](https://github.com/ratatui-org/ratatui/commit/f933d892aae0abbc2ec818fee0dfa95ca9e959d2) *(uncategorized)* Update CHANGELOG by @fdehau

- [23a9280](https://github.com/ratatui-org/ratatui/commit/23a9280db7064cb00fe6e52483f6f6c2c7e97efa) *(uncategorized)* Add gping to the lists of apps using tui by @orf

  > * Add gping to the lists of apps using tui



### New Contributors
* @sagiegurari made their first contribution
* @Nukesor made their first contribution
* @orf made their first contribution
* @marshoepial made their first contribution
* @pm100 made their first contribution

**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.13.0...v0.14.0


## [v0.13.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.13.0) - 2020-11-14

### Features

- [5050f1c](https://github.com/ratatui-org/ratatui/commit/5050f1ce1c4a7790ee29a750709950f8b123939a) *(widgets/gauge)* Add `LineGauge` variant of `Gauge` by @fdehau

- [5a9b598](https://github.com/ratatui-org/ratatui/commit/5a9b59866bbe93dad00309e4ffc449a21601972d) *(widgets/listitem)* Derive PartialEq by @acheronfail

### Bug Fixes

- [98fb5e4](https://github.com/ratatui-org/ratatui/commit/98fb5e4bbd243d5c418301e09fd9a26f16236b00) *(widgets/table)* Take borders into account when percentage and ration constraints are used by @Kemyt

  > * Fix percentage and ratio constraints for table to take borders into account
  >
  > Percentage and ratio constraints don't take borders into account, which causes
  > the table to not be correctly aligned. This is easily seen when using 50/50
  > percentage split with bordered table. However fixing this causes the last column
  > of table to not get printed, so there is probably another problem with columns
  > width determination.
  >
  > * Fix rounding of cassowary solved widths to eliminate imprecisions
  >
  > * Fix formatting to fit convention

### Miscellaneous Tasks

- [dc76956](https://github.com/ratatui-org/ratatui/commit/dc769562152e29091d04cf4904a1ebcde9336b46) *(uncategorized)* Add taskwarrior-tui to the list of apps using tui-rs by @kdheepak




**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.12.0...v0.13.0


## [v0.12.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.12.0) - 2020-09-27

### Features

- [4114273](https://github.com/ratatui-org/ratatui/commit/41142732ec2bb6490c18e68fa7da02823adb65d7) *(buffer)* Add a method to build a `Style` out of an existing `Cell` by @fdehau

- [d00184a](https://github.com/ratatui-org/ratatui/commit/d00184a7c387fb0c31453527aa7e54c52c088950) *(text)* Extend `Text` to be stylable and extendable by @TheLostLambda

  > * Extend `Text` to be extendable
  > * Add some documentation

### Bug Fixes

- [62495c3](https://github.com/ratatui-org/ratatui/commit/62495c3bd1afb46969640fb6307085b33e4a85cb) *(widgets/barchart)* Fix chart filled more than actual by @Kemyt

  > * Fix barchart incorrectly showing chart filled more than actual
  >
  > Determination of how filled the bar should be was incorrectly taking the
  > entire chart height into account, when in fact it should take height-1, because
  > of extra line with label. Because of that the chart appeared fuller than it
  > should and was full before reaching maximum value.
  >
  > * Add a test case for checking if barchart fills correctly up to max value

- [c4cd0a5](https://github.com/ratatui-org/ratatui/commit/c4cd0a5f31581518501a1ecda93875277ff943d0) *(widgets/chart)* Use the correct style to draw legend and axis titles by @fdehau

  > Before this change, the style of the points drawn in the graph area could reused to draw the
  > title of the axis and the legend. Now the style of these components put on top of the graph area
  > is solely based on the widget style.

### Miscellaneous Tasks

- [ce32d55](https://github.com/ratatui-org/ratatui/commit/ce32d5537ddfca2870bcab71736f61cd034b99bd) *(uncategorized)* Clippy fixes by @TheLostLambda

- [25921fa](https://github.com/ratatui-org/ratatui/commit/25921fa91af9446baeea2586e5131ae222d6b1a4) *(uncategorized)* Added termchat to "apps using tui" by @lemunozm

- [932a496](https://github.com/ratatui-org/ratatui/commit/932a496c3c6b0a26ba856bae77082ae3d585829a) *(uncategorized)* Add rkm to the list of apps using tui by @aryakaul



### New Contributors
* @lemunozm made their first contribution
* @aryakaul made their first contribution

**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.11.0...v0.12.0


## [v0.11.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.11.0) - 2020-09-20

### Features

- [90f3858](https://github.com/ratatui-org/ratatui/commit/90f3858effe0e252495a04469ebb94b307c7e95a) *(backend)* Keep the internal buffers in sync when the terminal is cleared. by @fdehau

- [641f391](https://github.com/ratatui-org/ratatui/commit/641f39113710b7b873e346fec6296afbb045eee0) *(terminal)* Add unstable api to use a fixed viewport by @fdehau

  > There was now way to avoid the autoresize behavior of `Terminal`. While it was fine for most users,
  > it made the testing experience painful as it was impossible to avoid the calls to `Backend::size()`.
  > Indeed they trigger the following error: "Inappropriate ioctl for device" since we are not running
  > the tests in a real terminal (at least in the CI).
  >
  > This commit introduces a new api to create a `Terminal` with a fixed viewport.

- [c35a1dd](https://github.com/ratatui-org/ratatui/commit/c35a1dd79fd59e37999a1a64b5faa7018f95e9dd) *(widgets/canvas)* Added type Block in canvas markers by @Amjad50

  > This allows for clearer colors than using Dot, especially when
  > decreasing the size of the terminal font in order to increase the
  > resolution of the canvas

### Bug Fixes

- [ecb482f](https://github.com/ratatui-org/ratatui/commit/ecb482f2978bfbfe396fa29968a4b72e01432c86) *(backend)* Move the cursor when first diff is on second cell by @fdehau

  > Both termion and crossterm backends were not moving the cursor if the first diff to draw was on the
  > second cell. The condition triggering the cursor move has been updated to fix this. In addition, two
  > tests have been added to avoid future regressions.

- [11df94d](https://github.com/ratatui-org/ratatui/commit/11df94d601df6a01441536ebc206df9b950abc9f) *(examples)* Avoid panic when computing event poll timeout in crossterm demo by @fdehau

### Refactor

- [e0b2572](https://github.com/ratatui-org/ratatui/commit/e0b2572eba7512d336db9e749d0a29cc00c39ed3) *(backend/crossterm)* Support more style modifiers on Windows and fix build with Crossterm 0.17.8 by @alvinhochun

  > * Support more style modifiers on Windows
  > * Change Crossterm backend to write directly to buffer instead of String
  >
  > Crossterm might actually do WinAPI calls instead of writing ANSI excape
  > codes so writing to an intermediate String may cause issues on older
  > versions of Windows. It also fails to compile with Crossterm 0.17.8 due
  > to Crossterm now expecting the writer to support `flush`, which String
  > doesn't.
  >
  > Fixes #373

- [0abaa20](https://github.com/ratatui-org/ratatui/commit/0abaa20de987849a6a09323af64aba02cfdab6a4) *(uncategorized)* Clean up some folds by @TheLostLambda

### Miscellaneous Tasks

- [aada695](https://github.com/ratatui-org/ratatui/commit/aada695b3f792f1c2d919bd45c1f3e2bc043ed72) *(uncategorized)* Add tickrs to apps using tui by @tarkah

- [dc26f7b](https://github.com/ratatui-org/ratatui/commit/dc26f7ba9f0965f3ae550573938d257636dce182) *(uncategorized)* Document rustc min version supported by @fdehau

  > - Add section to README
  > - Run ci tests with this min version in addition of stable to track changes that would require a min
  > rustc version bump.



### New Contributors
* @Amjad50 made their first contribution
* @alvinhochun made their first contribution

**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.10.0...v0.11.0


## [v0.10.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.10.0) - 2020-07-18

### Features

- [b59e4bb](https://github.com/ratatui-org/ratatui/commit/b59e4bb80824c81a11b8f4db16eae8ef740e1d48) *(examples)* Enable mouse capture to make crossterm demo on par with termion by @fdehau

- [7251186](https://github.com/ratatui-org/ratatui/commit/72511867624c9bc416e64a1b856026ced5c4e1eb) *(style)* Add StyleDiff by @fdehau

- [ac99104](https://github.com/ratatui-org/ratatui/commit/ac9910411417fb84e128ece18a3c530a2467bdc5) *(style)* Add support to serialize and deserialize Style using serde by @fdehau

  > * Add serde as an optional dependency.
  > * Add feature-gated derives to Color, Modifier and Style.

- [8c2ee0e](https://github.com/ratatui-org/ratatui/commit/8c2ee0ed8516be887f5f68355c325fadecb5d2dc) *(terminal)* Add after-draw() cursor control to Frame by @Minoru

- [88c4b19](https://github.com/ratatui-org/ratatui/commit/88c4b191fba7922a3ef3bf4205be2a685e80cb48) *(text)* Add new text primitives by @fdehau

- [112d2a6](https://github.com/ratatui-org/ratatui/commit/112d2a65f67e0305c43573ca0ae5cafbf882835b) *(widgets/paragraph)* Add option to preserve indentation when the text is wrapped by @TheLostLambda

- [d999c1b](https://github.com/ratatui-org/ratatui/commit/d999c1b434e10c41580a3f539f2cadccd9933bd4) *(widgets/paragraph)* Add horizontal scroll by @xiaopengli89

  > * `Paragraph:scroll` takes a tuple of offsets instead of a single vertical offset.
  > * `LineTruncator` takes this new horizontal offset into account to let the paragraph scroll horizontally.

### Bug Fixes

- [2b48409](https://github.com/ratatui-org/ratatui/commit/2b48409cfd21ce91e00e2a45fdfe24b3c825ef69) *(examples)* Remove typo in demo text by @Cokemonkey11

- [6204edd](https://github.com/ratatui-org/ratatui/commit/6204eddadef418136b0b960989c838db7d869a8a) *(readme)* Typo in demo section by @priime0

  > There was a very small typo in the README on line 40, which cited the
  > `examples` folder as `exmples`. This resolves that issue.

- [fdbea9e](https://github.com/ratatui-org/ratatui/commit/fdbea9e2ee98747f01341c9bf66cd6defaf3bcf4) *(widgets/canvas)* Avoid panic on zero-width bounds by @fdehau

### Refactor

- [72ba4ff](https://github.com/ratatui-org/ratatui/commit/72ba4ff2d45d1e436dbfde31b2791aa15a9097b5) *(examples)* Remove unecessary `terminal.hide_cursor` calls by @fdehau

- [4fe647d](https://github.com/ratatui-org/ratatui/commit/4fe647df0aabfffd4e8612f503bab3664e2b64eb) *(tests)* Rename integration tests to be able to call group of tests by @fdehau

- [a00350a](https://github.com/ratatui-org/ratatui/commit/a00350ab54a79842b6f381613be316a05361d379) *(tests)* Rename test files and use the new `TestBackend::assert_buffer` method by @fdehau

- [96c6b4e](https://github.com/ratatui-org/ratatui/commit/96c6b4efcbf7ec1e4ceb258f2eba05d2cb3c540d) *(tests)* Move test utilities to TestBackend by @fdehau

  > * Remove custom Debug implementation of Buffer
  > * Add `TestBackend::assert_buffer` to compare buffers in integration tests. When
  > the assertion fails, the output now show the list of differences in addition
  > of the views of the computed and expected buffers. This effectively replaces
  > the table of debug code for colors and modifiers as it is easier to read.

- [0ffea49](https://github.com/ratatui-org/ratatui/commit/0ffea495b1c64f4f443f81249d73e97964d1a0ab) *(uncategorized)* Implement cascading styles by @fdehau

  > - merge `Style` and `StyleDiff` together. `Style` now is used to activate or deactivate certain
  > style rules not to overidden all of them.
  > - update all impacted widgets, examples and tests.

### Documentation

- [e789c67](https://github.com/ratatui-org/ratatui/commit/e789c671b038d516c347874e899be815b15caf25) *(readme)* Update README.md by @imsnif

- [82fda4a](https://github.com/ratatui-org/ratatui/commit/82fda4ac0e91ed1995d80ee3b32c0d39ee49701f) *(style)* Improve documentation of Style by @fdehau

### Miscellaneous Tasks

- [6b52c91](https://github.com/ratatui-org/ratatui/commit/6b52c91257f9073e4fe913f15bee0716b45dfd0d) *(uncategorized)* Update CHANGELOG by @fdehau

### Layout

- [1d12ddb](https://github.com/ratatui-org/ratatui/commit/1d12ddbdfcfd235bf6ea136b9720aede53b9a3a2) *(uncategorized)* Add vertical split constraint test on height by @lithdew

- [f474c76](https://github.com/ratatui-org/ratatui/commit/f474c76e193d9087e3ede4b60aa14ca2729032c7) *(uncategorized)* Force constraint that width and height are non-negative by @lithdew



### New Contributors
* @xiaopengli89 made their first contribution
* @priime0 made their first contribution
* @Minoru made their first contribution
* @Cokemonkey11 made their first contribution
* @lithdew made their first contribution

**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.9.5...v0.10.0


## [v0.9.5](https://github.com/ratatui-org/ratatui/releases/tag/v0.9.5) - 2020-05-21

### Miscellaneous Tasks

- [5a590bc](https://github.com/ratatui-org/ratatui/commit/5a590bca74ecfe0dd7c99312d64742b270582b50) *(uncategorized)* Enable clippy on all targets and all features by @fdehau

  > - Remove deny warnings in lib.rs. This allows easier iteration when developing
  > new features. The warnings will make the CI fails anyway on the clippy CI
  > stage.
  > - Run clippy on all targets (including tests and examples) and all features.
  > - Fail CI on clippy warnings.




**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.9.4...v0.9.5


## [v0.9.4](https://github.com/ratatui-org/ratatui/releases/tag/v0.9.4) - 2020-05-12

### Bug Fixes

- [a7761fe](https://github.com/ratatui-org/ratatui/commit/a7761fe55d420fb68cfcdcd0c4eb017d3c091c37) *(buffer)* Ignore zero-width graphemes by @fdehau




**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.9.3...v0.9.4


## [v0.9.3](https://github.com/ratatui-org/ratatui/releases/tag/v0.9.3) - 2020-05-10

### Bug Fixes

- [b72ced4](https://github.com/ratatui-org/ratatui/commit/b72ced4511c55ab6623f8ab916dfffa87d901b87) *(widgets/chart)* Remove overflow when dataset if empty by @ClementTsang

  > * docs: Fix missing code block fence
  > * use slice::windows to deal with underflow issue
  > * add test for empty dataset and lines




**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.9.2...v0.9.3


## [v0.9.2](https://github.com/ratatui-org/ratatui/releases/tag/v0.9.2) - 2020-05-10

### Bug Fixes

- [359b7fe](https://github.com/ratatui-org/ratatui/commit/359b7feb8c80a3ebcf1269c098feac52df2c2c82) *(widgets/canvas)* Add bounds check when drawing line high/low by @DarrienG

  > * Add bounds check when drawing line high/low
  > * Add test to ensure codepath doesn't break

### Miscellaneous Tasks

- [6ffdede](https://github.com/ratatui-org/ratatui/commit/6ffdede95aa09f51ff61ddd42d577991d19db072) *(uncategorized)* Add documentation field in Cargo.toml



### New Contributors
* @DarrienG made their first contribution

**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.9.1...v0.9.2


## [v0.9.1](https://github.com/ratatui-org/ratatui/releases/tag/v0.9.1) - 2020-04-16

### Bug Fixes

- [8f9aa27](https://github.com/ratatui-org/ratatui/commit/8f9aa276e846f76efde27ac33a0e069ffe579c6d) *(widgets/list)* Fix line length calculation for selectable lists by @dotdash

  > The code that outputs the list elements uses the full inner width of its
  > block, without taking the width of the highlight symbol into
  > consideration. This allows the elements to overflow the box and draw
  > over the block's border. To fix that, we need to reduce the target width
  > for the list elements.

### Documentation

- [5d99b4a](https://github.com/ratatui-org/ratatui/commit/5d99b4af00c94e87da77a40aa6580469089c29f2) *(uncategorized)* Improve widgets documentation by @fdehau

### Testing

- [da4d4e1](https://github.com/ratatui-org/ratatui/commit/da4d4e167261884c497814490e67a33db18ea93f) *(uncategorized)* Assert items are correctly truncated in the `List` widget by @fdehau



### New Contributors
* @dotdash made their first contribution

**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.9.0...v0.9.1


## [v0.9.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.9.0) - 2020-04-14

### Features

- [28017f9](https://github.com/ratatui-org/ratatui/commit/28017f97eafda3d8f7c605b28e04499bc7576ee6) *(widgets/chart)* Add more control on the visibility of the legend by @fdehau

- [ae67709](https://github.com/ratatui-org/ratatui/commit/ae677099d6f1613a70995e07a7e1de7faf0750e4) *(widgets/table)* Allow one row to be selected by @fdehau

- [bc2a512](https://github.com/ratatui-org/ratatui/commit/bc2a5121018f33eee9347dab4c1cbdb3622654fa) *(uncategorized)* Add missing `Clone` and `Copy` on types by @fdehau

- [c98002e](https://github.com/ratatui-org/ratatui/commit/c98002eb76951c925c04b61e8103658b8ef8f4db) *(uncategorized)* Add an option to run the examples without all unicode symbols by @fdehau

- [cee65ed](https://github.com/ratatui-org/ratatui/commit/cee65ed2833dab66625f9f30593ae870f98e3ccf) *(uncategorized)* Allow BarChart and Sparkline to use a more portable set of symbols by @fdehau

  > Add `BarChart::bar_set` and `Sparkline::bar_set` methods to customize
  > the set of symbols used to display the data. The new set should give
  > a better looking output on terminal that do not support a wide range
  > of unicode symbols.

- [6cb57f5](https://github.com/ratatui-org/ratatui/commit/6cb57f5d2a231b409baa2c064d7bbd40ce118a85) *(uncategorized)* Add stateful widgets by @fdehau

  > Most widgets can be drawn directly based on the input parameters. However, some
  > features may require some kind of associated state to be implemented.
  >
  > For example, the `List` widget can highlight the item currently selected. This
  > can be translated in an offset, which is the number of elements to skip in
  > order to have the selected item within the viewport currently allocated to this
  > widget. The widget can therefore only provide the following behavior: whenever
  > the selected item is out of the viewport scroll to a predefined position (make
  > the selected item the last viewable item or the one in the middle).
  > Nonetheless, if the widget has access to the last computed offset then it can
  > implement a natural scrolling experience where the last offset is reused until
  > the selected item is out of the viewport.
  >
  > To allow such behavior within the widgets, this commit introduces the following
  > changes:
  > - Add a `StatefulWidget` trait with an associated `State` type. Widgets that
  > can take advantage of having a "memory" between two draw calls needs to
  > implement this trait.
  > - Add a `render_stateful_widget` method on `Frame` where the associated
  > state is given as a parameter.
  >
  > The chosen approach is thus to let the developers manage their widgets' states
  > themselves as they are already responsible for the lifecycle of the wigets
  > (given that the crate exposes an immediate mode api).
  >
  > The following changes were also introduced:
  >
  > - `Widget::render` has been deleted. Developers should use `Frame::render_widget`
  > instead.
  > - `Widget::background` has been deleted. Developers should use `Buffer::set_background`
  > instead.
  > - `SelectableList` has been deleted. Developers can directly use `List` where
  > `SelectableList` features have been back-ported.

### Bug Fixes

- [e81af75](https://github.com/ratatui-org/ratatui/commit/e81af75427d3d62466c33ef5471ebc6f1969a038) *(examples)* Improve input handling in crossterm demo by @fdehau

  > * avoid stacking events
  > * ensure tick events are sent at the given tick rate (and not everytime a key is pressed).

- [4f728d3](https://github.com/ratatui-org/ratatui/commit/4f728d363f0be75f844f39b2fa2705e7b4373953) *(widgets/list)* Stop highlighting blank placeholders by @fdehau

- [67dd1ac](https://github.com/ratatui-org/ratatui/commit/67dd1ac60863bd152602e5f4b602a936107843da) *(uncategorized)* Remove array_into_iter warnings by @fdehau

- [ea43413](https://github.com/ratatui-org/ratatui/commit/ea434135070dba635c3c4136c3263a8c3622f0c6) *(uncategorized)* Remove clippy warnings by @fdehau

### Refactor

- [140db9b](https://github.com/ratatui-org/ratatui/commit/140db9b2e251e1567656dc2bb29be1de81269077) *(canvas)* Update shape drawing strategy by @fdehau

  > * Update the `Shape` trait. Instead of returning an iterator of point, all
  > shapes are now aware of the surface they will be drawn to through a `Painter`.
  > In order to draw themselves, they paint points of the "braille grid".
  > * Rewrite how lines are drawn using a common line drawing algorithm (Bresenham).

- [e6ce0ab](https://github.com/ratatui-org/ratatui/commit/e6ce0ab9a7f4f71c42f53fc7c34eda0d78eb46aa) *(examples)* Add input modes to user input examples by @fdehau

- [584e1b0](https://github.com/ratatui-org/ratatui/commit/584e1b05007245fa333ad4bff5924f8eead744d1) *(widgets/canvas)* Allow canvas to render with a simple dot character instead of braille patterns by @fdehau

  > This change allows developers to gracefully degrade the output if the targeted
  > terminal does not support the full range of unicode symbols.

- [9085c81](https://github.com/ratatui-org/ratatui/commit/9085c81e768b488ea8e63e9dc6704f439d9b27a4) *(uncategorized)* Clean up border type for blocks by @fdehau

  > * Merge line symbols in a single module.
  > * Replace set_border_type with border_type to match other builder methods.
  > * Remove unecessary branching.

### Documentation

- [cf39de8](https://github.com/ratatui-org/ratatui/commit/cf39de882a47fd5992b116a7d2a3a9a545d65b3a) *(readme)* Add bandwhich to "apps using tui" by @imsnif

- [3e6211e](https://github.com/ratatui-org/ratatui/commit/3e6211e0a3c3077df207ec99b926319348149640) *(uncategorized)* Add kmon to 'apps using tui' in README by @orhun

### Styling

- [278c153](https://github.com/ratatui-org/ratatui/commit/278c153d31788575079a8c22da594deaf8e7a2f7) *(uncategorized)* Remove clippy warnings by @fdehau

- [d16db5e](https://github.com/ratatui-org/ratatui/commit/d16db5ed90c5d37f0db86acf40c41250049505f9) *(uncategorized)* Fix clippy warnings by @fdehau

- [6e24f9d](https://github.com/ratatui-org/ratatui/commit/6e24f9d47bc4bfa6b956e237d3a024b0c26e7980) *(uncategorized)* Run cargo fmt by @fdehau

- [d503275](https://github.com/ratatui-org/ratatui/commit/d50327548b558fb84dd0d35aee222a8e10982d87) *(uncategorized)* Run rustfmt by @fdehau

### Miscellaneous Tasks

- [d3f1669](https://github.com/ratatui-org/ratatui/commit/d3f1669234534e4f01afd3301aa911ebc4644ec8) *(makefile)* Add lint to stable and beta rules by @fdehau

- [8387b32](https://github.com/ratatui-org/ratatui/commit/8387b32bb87240cd617da3e7e7846a8b1d340cc8) *(uncategorized)* Update changelog by @fdehau

- [2fccee7](https://github.com/ratatui-org/ratatui/commit/2fccee740b1e8394751c7e95952dcb71fe6b3c4d) *(uncategorized)* Add command to README to run demos without all unicode symbols by @fdehau

- [8104b17](https://github.com/ratatui-org/ratatui/commit/8104b17ee6affa0d38f27b661e751d91df43cc8b) *(uncategorized)* Bump crossterm to 0.17

  > this fixes #250 because crossterm `0.17.3` has a fix for the resize/size issue
  >
  > Co-Authored-By:Florian Dehau <work@fdehau.com>

- [867ba1f](https://github.com/ratatui-org/ratatui/commit/867ba1fd8c1a3bc02457333a74a7023e5cd2a7e0) *(uncategorized)* Update changelog by @fdehau

- [503bdee](https://github.com/ratatui-org/ratatui/commit/503bdeeadbadf7deecd3bb5954d575de534ac4a8) *(uncategorized)* Bump itertools to 0.9 by @fdehau

- [3f62ce9](https://github.com/ratatui-org/ratatui/commit/3f62ce9c199bb0048996bbdeb236d6e5522ec9e0) *(uncategorized)* Remove unecessary dependencies by @fdehau

  > * Remove log, stderrlog, structopt
  > * Add argh

- [a6b3503](https://github.com/ratatui-org/ratatui/commit/a6b35031aebd4308b5ac3dd5dd80d62c1cbaa6f5) *(uncategorized)* Use master branch instead of latest release for crossterm

  > This will prevent [this](https://github.com/crossterm-rs/crossterm/pull/383)
  > descriptor leak bug when people use the crossterm backend with the current
  > master.

- [8c3db49](https://github.com/ratatui-org/ratatui/commit/8c3db49fba9598714d36f37f9b4ebe1f83266b3a) *(uncategorized)* Bump crossterm to 0.16 by @fdehau

- [02b1aac](https://github.com/ratatui-org/ratatui/commit/02b1aac0b050c10b3ba9728e50969073db65aa26) *(uncategorized)* Remove outdated badges by @fdehau




**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.8.0...v0.9.0


## [v0.8.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.8.0) - 2019-12-15

### Features

- [60b99cf](https://github.com/ratatui-org/ratatui/commit/60b99cfc66c1a3b1c150850b0da76a086e6e5d3d) *(uncategorized)* Bump crossterm to 0.14 by @TimonPost

### Miscellaneous Tasks

- [7cc4189](https://github.com/ratatui-org/ratatui/commit/7cc4189eb01c8b3504bd0fb09d7c07e1c991cad2) *(uncategorized)* Update issue templates by @fdehau

- [86d4a32](https://github.com/ratatui-org/ratatui/commit/86d4a32314e9b5b4ab03533471bcbd8d40cc809e) *(uncategorized)* Update issue templates by @fdehau

- [67c9c64](https://github.com/ratatui-org/ratatui/commit/67c9c64eabfb711a490e1c244f87270338fb5778) *(uncategorized)* Add spotify-tui to the list of apps using tui by @fdehau

- [e0083fb](https://github.com/ratatui-org/ratatui/commit/e0083fb8deb6b24aa8f1fd4131f43f97c2550494) *(uncategorized)* Make the onboarding easier for Windows users. by @fdehau

### Bugfix

- [bbd4363](https://github.com/ratatui-org/ratatui/commit/bbd4363fa97afbbbdef61f51215ec323bbbed6ee) *(uncategorized)* Title_style was not used to style the axis title by @wose




**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.7.0...v0.8.0


## [v0.7.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.7.0) - 2019-11-29

### Features

- [e4873e4](https://github.com/ratatui-org/ratatui/commit/e4873e4da9a213df7a7dc718e1f400b807870724) *(backend)* Bump crossterm to 0.13 by @TimonPost

  > * removed flush calls because execute already calls flush under the hood.
  > * moved some static functions into From traits
  > * removed useless clone in demo
  > * upgrade to crossterm 0.13
  > * map all errors

- [3747ddb](https://github.com/ratatui-org/ratatui/commit/3747ddbefb1d4c838c857b7e063031543d8064d2) *(backend)* Refactor crossterm backend by @fdehau

  > * Remove compilation warnings
  > * Fix rendering artifacts in the crossterm demo. In particular, the bold modifier
  > was leaking on most of the terminal screen because the old logic was not
  > properly unsetting the bold modifier after use (took inspiration of the termion
  > backend implementation)

### Bug Fixes

- [a82c82f](https://github.com/ratatui-org/ratatui/commit/a82c82fcd7ca323642afa4fbb6bff99aa8ecec4f) *(widgets)* Remove compilation warning in table widget by @fdehau

### Styling

- [816bc9b](https://github.com/ratatui-org/ratatui/commit/816bc9b5c81f90e1f80dea7e651b68eb19f79bf3) *(uncategorized)* Fix formatting and clippy issues by @fdehau

### Miscellaneous Tasks

- [055af0f](https://github.com/ratatui-org/ratatui/commit/055af0f78a89117e469530b0191beb774e06a75b) *(uncategorized)* Bump dev dependencies by @fdehau

  > * bump rand to 0.7
  > * bump structopt to 0.3

- [2233cdc](https://github.com/ratatui-org/ratatui/commit/2233cdc9cce34342eef778912d87e0121a734f4e) *(uncategorized)* Add CI based on github actions by @fdehau




**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.6.2...v0.7.0


## [v0.6.1](https://github.com/ratatui-org/ratatui/releases/tag/v0.6.1) - 2019-06-16

### Bug Fixes

- [25a0825](https://github.com/ratatui-org/ratatui/commit/25a0825ae4f534a6762b0fc94159b573abe9f3c1) *(uncategorized)* Curses backend cursor positions by @defiori

### Miscellaneous Tasks

- [2dfe9c1](https://github.com/ratatui-org/ratatui/commit/2dfe9c1663062cf69859fafa8c78f1863fec4267) *(uncategorized)* User_input] Assure the cursor responds immediatel when hitting backspace

  > This was discovered with the termion backend in alacritty on OSX.




**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.6.0...v0.6.1


## [v0.5.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.5.0) - 2019-03-10

### Features

- [4a1f3cd](https://github.com/ratatui-org/ratatui/commit/4a1f3cd61fb056ac34885985f56d2cc2add49d3f) *(uncategorized)* Curses instance can be passed to backend by @defiori

- [d75198a](https://github.com/ratatui-org/ratatui/commit/d75198a8ee0fc0c09f3462a98219770d9afba91b) *(uncategorized)* Add pancurses backend by @defiori

- [b30cae0](https://github.com/ratatui-org/ratatui/commit/b30cae0473a64d2a47b9441d59bdf822a4c40992) *(uncategorized)* Crossterm backend can use alternate screen by @defiori

- [f20512b](https://github.com/ratatui-org/ratatui/commit/f20512b599ff8609e265561e74f9bf76f66c547d) *(uncategorized)* Add rustbox and crossterm demo by @fdehau

### Bug Fixes

- [e037db0](https://github.com/ratatui-org/ratatui/commit/e037db076cd91065d240a597255c63b71d43516c) *(backend/curses)* Use chtype to achieve platform agnostic conversion of graphemes by @fdehau

- [3ef19f4](https://github.com/ratatui-org/ratatui/commit/3ef19f41e6c38573169033113d224f08a599967b) *(backend/curses)* Avoid platform specific conversion of graphemes by @fdehau

- [7c4a3d2](https://github.com/ratatui-org/ratatui/commit/7c4a3d2b020a606a67a54a25cad1de3b15497af0) *(examples)* Bring in line with demo organization by @defiori

- [da90ec1](https://github.com/ratatui-org/ratatui/commit/da90ec15fa4e3a5acb63a81209031a41c206e198) *(uncategorized)* Add missing get_cursor and set_cursor on CursesBackend by @fdehau

- [624e6ee](https://github.com/ratatui-org/ratatui/commit/624e6ee047f2b51d0fc24c50c757f63a9ee1cfde) *(uncategorized)* Filter out wide unicode characters on windows by @defiori

- [8db1bb5](https://github.com/ratatui-org/ratatui/commit/8db1bb56f23cb483920bcff24bf2bfbc7b85b1a3) *(uncategorized)* Curses demo required features by @defiori

- [cadb41c](https://github.com/ratatui-org/ratatui/commit/cadb41c9e3a4db6ed689c84e9357cf2fd3e1eda1) *(uncategorized)* Unified crossterm backend by @defiori

### Styling

- [7f5af46](https://github.com/ratatui-org/ratatui/commit/7f5af463005686c9399f320ab064bd5a2f6f93c9) *(uncategorized)* Fmt by @fdehau




**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.4.0...v0.5.0


## [v0.4.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.4.0) - 2019-02-03

### Features

- [ec6b463](https://github.com/ratatui-org/ratatui/commit/ec6b46324e59454f74d271b9149a84fc5e4821cd) *(examples)* Add cmd line args to the demo by @fdehau

- [97f764b](https://github.com/ratatui-org/ratatui/commit/97f764b45da359d1976d039c3751d8f9ea1d8d5b) *(uncategorized)* Handle crossterm errors by @fdehau

- [b3689ec](https://github.com/ratatui-org/ratatui/commit/b3689eceb7cd675d1e758ce2db0573b80fced5eb) *(uncategorized)* Update outdated dependencies by @fdehau

### Bug Fixes

- [09c09d2](https://github.com/ratatui-org/ratatui/commit/09c09d2fd1ee3b645e9ead1a1b81a62095cdc90d) *(examples)* Remove logging in layout example by @fdehau

- [52a40ec](https://github.com/ratatui-org/ratatui/commit/52a40ec99a00fb4bd044ae211f687e692eea99f8) *(uncategorized)* Remove undefined crossterm attributes in windows builds by @fdehau

### Styling

- [b669cf9](https://github.com/ratatui-org/ratatui/commit/b669cf9ce7c5f41f381edd1ff9e0bc25641e5059) *(uncategorized)* Fix clippy warnings by @fdehau

### Miscellaneous Tasks

- [22579b7](https://github.com/ratatui-org/ratatui/commit/22579b77cc9e926434db1341eca8ab4312c8fef4) *(makefile)* Make run-examples compile the examples in release mode by @fdehau

- [5bc617a](https://github.com/ratatui-org/ratatui/commit/5bc617a9a61d3b02e781068d9721cc9fd5aad3db) *(makefile)* Build and test using all features by @fdehau

- [2286d09](https://github.com/ratatui-org/ratatui/commit/2286d097dc6b6032584320cba25c3084da29c368) *(ci)* Add appveyor config by @fdehau

- [0168442](https://github.com/ratatui-org/ratatui/commit/0168442c224bd3cd23f1d2b6494dd236b556a124) *(uncategorized)* Remove typos by @fdehau

- [a75b811](https://github.com/ratatui-org/ratatui/commit/a75b811061e25a37100f470e629ac349bd27f8d0) *(uncategorized)* Bump itertools to 0.8 by @fdehau

- [7f31a55](https://github.com/ratatui-org/ratatui/commit/7f31a555062481cba71b34a589a6513adbf10c69) *(uncategorized)* Show appveyor build status by @fdehau

- [3fd9e23](https://github.com/ratatui-org/ratatui/commit/3fd9e23851a0077e2ab571c0f72f5be7ed67d960) *(uncategorized)* Correct diffing of buffers with multi-width characters by @karolinepauls
  >
  > Resolves #104

- [10642d0](https://github.com/ratatui-org/ratatui/commit/10642d0e04cfde214b93de1ef4f427c1fa35440d) *(uncategorized)* Word wrapping by @karolinepauls

- [228816f](https://github.com/ratatui-org/ratatui/commit/228816f5f8ec6f056a08562ffb03a2bb143ca813) *(uncategorized)* Provide consistent size for rendering by @karolinepauls

- [cc95c8c](https://github.com/ratatui-org/ratatui/commit/cc95c8cfb04172d37d7f5e845d34bc534d918e35) *(uncategorized)* Use f64 internally and allow to set any f64 between 0 and 1 by @karolinepauls

- [89dac9d](https://github.com/ratatui-org/ratatui/commit/89dac9d2a64d918178e23d2b2480b57f15f9afff) *(uncategorized)* Add quotes to fmt::Debug for better testing experience by @karolinepauls

### Feature

- [8cdfc88](https://github.com/ratatui-org/ratatui/commit/8cdfc883b9ee3de741d35fc2f4c958e96d54c1b7) *(uncategorized)* Autoresize by @karolinepauls

  > It basically never makes sense to render without syncing the size.
  >
  > Without resizing, if shrinking, we get artefacts. If growing, we may get
  > panics (before this change the Rustbox sample (the only one which didn't
  > handle resizing on its own) panicked because the widget would get an
  > updated size, while the terminal would not).




**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.3.0...v0.4.0


## [v0.3.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.3.0) - 2018-11-04

### Features

- [f6d2f8f](https://github.com/ratatui-org/ratatui/commit/f6d2f8f929b97ecb75c9cd1e7321b53f70fc390a) *(examples)* Use generic backend in draw functions by @fdehau

- [3294766](https://github.com/ratatui-org/ratatui/commit/32947669d51e2e65499bdc054119db82c8189917) *(examples)* Show how to move the cursor by @fdehau

- [fdf3015](https://github.com/ratatui-org/ratatui/commit/fdf3015ad072b2111d973c4cd089a9a8dede34d2) *(terminal)* Log error if failed to show cursor on drop by @fdehau

- [22e8fad](https://github.com/ratatui-org/ratatui/commit/22e8fade7e282b1c3edbf632a266a7fcbc7bba96) *(uncategorized)* Add experimental test backend by @fdehau

### Styling

- [37aa06f](https://github.com/ratatui-org/ratatui/commit/37aa06f508ea16f6aee9d4103d114d3ef9d99590) *(examples)* Rustfmt by @fdehau

### Miscellaneous Tasks

- [03bfcde](https://github.com/ratatui-org/ratatui/commit/03bfcde147c7477d8958eddf55b347e24a990546) *(uncategorized)* Truncate long lines when wrap is false by @karolinepauls




**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.3.0-beta.3...v0.3.0


## [v0.3.0-beta.3](https://github.com/ratatui-org/ratatui/releases/tag/v0.3.0-beta.3) - 2018-09-24

### Features

- [7b4d35d](https://github.com/ratatui-org/ratatui/commit/7b4d35d224c1b15d415be9979b4c5cde637dfacb) *(uncategorized)* Restore the cursor state on terminal drop by @fdehau




**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.3.0-beta.2...v0.3.0-beta.3


## [v0.3.0-beta.2](https://github.com/ratatui-org/ratatui/releases/tag/v0.3.0-beta.2) - 2018-09-23

### Bug Fixes

- [aa85e59](https://github.com/ratatui-org/ratatui/commit/aa85e597d953d7683a97931c18bbd19d11772e47) *(crossterm)* Fix goto coordinates by @fdehau

- [4ae9850](https://github.com/ratatui-org/ratatui/commit/4ae9850e13cd7f7c40c2e207e0c27b5cef3cb7f6) *(uncategorized)* Replace links to assets by @fdehau

- [e14190a](https://github.com/ratatui-org/ratatui/commit/e14190ae4b9f67727da70643c2b60003a290b6be) *(uncategorized)* Update crossterm example by @fdehau

### Refactor

- [08ab92d](https://github.com/ratatui-org/ratatui/commit/08ab92da8014711b3f3396f7773dd652cf9484f0) *(uncategorized)* Clean examples by @fdehau

  > * Introduce a common event handler in order to focus on the drawing part
  > * Remove deprecated custom termion backends

- [5d52fd2](https://github.com/ratatui-org/ratatui/commit/5d52fd2486d8373dbd9a9c18a548808d96e4924a) *(uncategorized)* Remove custom termion backends by @fdehau

### Styling

- [d8e5f57](https://github.com/ratatui-org/ratatui/commit/d8e5f57d53f9900892c649b3cf974540ee7c0adb) *(uncategorized)* Fmt by @fdehau

### Miscellaneous Tasks

- [ce445a8](https://github.com/ratatui-org/ratatui/commit/ce445a8096eab2fd23f0ffec156e0065f2beb7da) *(uncategorized)* Remove scripts by @fdehau




**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.3.0-beta.1...v0.3.0-beta.2


## [v0.3.0-beta.1](https://github.com/ratatui-org/ratatui/releases/tag/v0.3.0-beta.1) - 2018-09-08

### Features

- [6c69160](https://github.com/ratatui-org/ratatui/commit/6c69160d6b297a6963230fcc4d41a34f39898cc5) *(uncategorized)* Remove unecessary borrows of Style by @fdehau




**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.3.0-beta.0...v0.3.0-beta.1


## [v0.3.0-beta.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.3.0-beta.0) - 2018-09-04

### Features

- [40bad7a](https://github.com/ratatui-org/ratatui/commit/40bad7a7186462b34d391135fbb2b714cb2ae9e3) *(uncategorized)* Add initial support for crossterm by @fdehau

- [7181970](https://github.com/ratatui-org/ratatui/commit/7181970a32f8f7ac94c154d3ec61797aabb1eb49) *(uncategorized)* Split layout from rendering by @fdehau

  > * remove layout logic from Terminal
  > * replace Group with Layout
  > * add Frame intermediate object

### Bug Fixes

- [cfc90ab](https://github.com/ratatui-org/ratatui/commit/cfc90ab7f6ce18514baa12c20dd309f93f7d46c3) *(widgets)* Prevent chart legend from rendering when no dataset has a name by @z2oh

### Refactor

- [ad602a5](https://github.com/ratatui-org/ratatui/commit/ad602a54bf100df762b7f97e4f03598f4e9e869a) *(widgets)* Replace text rendering in Paragraph by @fdehau

  > * remove custom markup language
  > * add Text container for both raw and styled strings

- [bcd1e30](https://github.com/ratatui-org/ratatui/commit/bcd1e303768e0d642eabfabe2b4c68dcafc43cd3) *(uncategorized)* Update List select behavior by @fdehau

  > * allow a selectable list to have no selected item
  > * show highlight_symbol only when something is selected

- [13e194c](https://github.com/ratatui-org/ratatui/commit/13e194cd266faf27567da103a9cbd86c7907a4d2) *(uncategorized)* Update widgets by @fdehau

  > * all widgets use the consumable builder pattern
  > * `draw` on terminal expect a closure that take a frame as only arg

- [d601678](https://github.com/ratatui-org/ratatui/commit/d6016788ef29ec5895c5d9ff715e1caf556b2389) *(uncategorized)* Clippy + rustfmt by @fdehau

### Documentation

- [3d63f96](https://github.com/ratatui-org/ratatui/commit/3d63f9607f3307f2d637e1d4754f3bedf684d269) *(uncategorized)* Update main documentation by @fdehau

### Styling

- [cf169d1](https://github.com/ratatui-org/ratatui/commit/cf169d1582db3267456e765ed9581b83a2154bbe) *(uncategorized)* Run rustfmt and clippy by @fdehau

### Miscellaneous Tasks

- [ccebb56](https://github.com/ratatui-org/ratatui/commit/ccebb56a8375bf2905e4b4bded98fab8571570eb) *(cargo)* Update dependencies by @fdehau




**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.2.3...v0.3.0-beta.0


## [v0.2.3](https://github.com/ratatui-org/ratatui/releases/tag/v0.2.3) - 2018-06-09

### Features

- [62df7ba](https://github.com/ratatui-org/ratatui/commit/62df7badf3997f11b5eb8d222841e3b9f23a3211) *(layout)* Add Corner enum by @fdehau

- [5de571f](https://github.com/ratatui-org/ratatui/commit/5de571fb037fa02c2a67db1110c1838aea68000d) *(widgets)* Add start_corner option to List by @fdehau

### Bug Fixes

- [9a9f49f](https://github.com/ratatui-org/ratatui/commit/9a9f49f4673b39fd548b773d1f4f757d281cf0e9) *(backend)* Add missing color pattern by @fdehau

### Styling

- [df7493f](https://github.com/ratatui-org/ratatui/commit/df7493fd33cf2cd9f90726edd052eb136e768214) *(uncategorized)* Run rustfmt by @fdehau

### Miscellaneous Tasks

- [c552ae9](https://github.com/ratatui-org/ratatui/commit/c552ae98b40ecce4840e3932b0e896d31fcbe4ce) *(readme)* Add link to third-party widgets and other crates by @fdehau

### Travis

- [464ba4f](https://github.com/ratatui-org/ratatui/commit/464ba4f334fb694ec7f9634c136ee08a9211acd1) *(uncategorized)* Check style on stable only by @fdehau




**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.2.2...v0.2.3


## [v0.2.1](https://github.com/ratatui-org/ratatui/releases/tag/v0.2.1) - 2018-04-01

### BUG

- [d0d2f88](https://github.com/ratatui-org/ratatui/commit/d0d2f88346ac4e21156d5b603c68232c34d6afb8) *(uncategorized)* Buffer::pos_of panics on inside-bounds index by @Mange

  > - Add tests for this behavior.
  > - Extend documentation of Buffer::pos_of and Buffer::index_of
  >   - Clarify that the coordinates should be in global coordinate-space
  >   (rather than local).
  >   - Document panics.
  >   - Add examples.



### New Contributors
* @Mange made their first contribution

**Full Changelog**: https://github.com/ratatui-org/ratatui/compare/v0.2.0...v0.2.1


<!-- generated by git-cliff -->

