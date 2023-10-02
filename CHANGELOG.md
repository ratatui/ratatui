# Changelog

All notable changes to this project will be documented in this file.

## [v0.23.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.23.0) - 2023-08-28

We are thrilled to release the new version of `ratatui` 🐭, the official successor[\*](https://github.com/fdehau/tui-rs/commit/335f5a4563342f9a4ee19e2462059e1159dcbf25) of [`tui-rs`](https://github.com/fdehau/tui-rs).

In this version, we improved the existing widgets such as `Barchart` and `Scrollbar`. We also made improvmements in the testing/internal APIs to provide a smoother testing/development experience. Additionally, we have addressed various bugs and implemented enhancements.

Here is a blog post that highlights the new features and breaking changes along with a retrospective about the project: [https://blog.orhun.dev/ratatui-0-23-0](https://blog.orhun.dev/ratatui-0-23-0)

### Features

- *(barchart)* Add direction attribute. (horizontal bars support) ([#325](https://github.com/ratatui-org/ratatui/issues/325))
([0dca6a6](https://github.com/ratatui-org/ratatui/commit/0dca6a689a7af640c5de8f7c87c2f1e03f0adf25))

  ````text
  * feat(barchart): Add direction attribute

  Enable rendering the bars horizontally. In some cases this allow us to
  make more efficient use of the available space.
  ````

- *(cell)* Add voluntary skipping capability for sixel ([#215](https://github.com/ratatui-org/ratatui/issues/215))
([e4bcf78](https://github.com/ratatui-org/ratatui/commit/e4bcf78afabe6b06970c51b4284246e345002cf5))

  ````text
  > Sixel is a bitmap graphics format supported by terminals.
  > "Sixel mode" is entered by sending the sequence ESC+Pq.
  > The "String Terminator" sequence ESC+\ exits the mode.

  The graphics are then rendered with the top left positioned at the
  cursor position.

  It is actually possible to render sixels in ratatui with just
  `buf.get_mut(x, y).set_symbol("^[Pq ... ^[\")`. But any buffer covering
  the "image area" will overwrite the graphics. This is most likely the same
  buffer, even though it consists of empty characters `' '`, except for
  the top-left character that starts the sequence.

  Thus, either the buffer or cells must be specialized to avoid drawing
  over the graphics. This patch specializes the `Cell` with a
  `set_skip(bool)` method, based on James' patch:
  https://github.com/TurtleTheSeaHobo/tui-rs/tree/sixel-support
  I unsuccessfully tried specializing the `Buffer`, but as far as I can tell
  buffers get merged all the way "up" and thus skipping must be set on the
  Cells. Otherwise some kind of "skipping area" state would be required,
  which I think is too complicated.

  Having access to the buffer now it is possible to skip all cells but the
  first one which can then `set_symbol(sixel)`. It is up to the user to
  deal with the graphics size and buffer area size. It is possible to get
  the terminal's font size in pixels with a syscall.

  An image widget for ratatui that uses this `skip` flag is available at
  https://github.com/benjajaja/ratatu-image.
  ````

- *(list)* Add option to always allocate the "selection" column width ([#394](https://github.com/ratatui-org/ratatui/issues/394))
([4d70169](https://github.com/ratatui-org/ratatui/commit/4d70169bef86898d331f46013ff72ef6d1c275ed))

  ````text
  * feat(list): add option to always allocate the "selection" column width

  Before this option was available, selecting a item in a list when nothing was selected
  previously made the row layout change (the same applies to unselecting) by adding the width
  of the "highlight symbol" in the front of the list, this option allows to configure this
  behavior.

  * style: change "highlight_spacing" doc comment to use inline code-block for reference
  ````

- *(release)* Add automated nightly releases ([#359](https://github.com/ratatui-org/ratatui/issues/359))
([aad164a](https://github.com/ratatui-org/ratatui/commit/aad164a5311b0a6d6d3f752a87ed385d5f0c1962))

  ````text
  * feat(release): add automated nightly releases

  * refactor(release): rename the alpha workflow

  * refactor(release): simplify the release calculation
  ````

- *(scrollbar)* Add optional track symbol ([#360](https://github.com/ratatui-org/ratatui/issues/360))
([1727fa5](https://github.com/ratatui-org/ratatui/commit/1727fa5120fa4bfcddd57484e532b2d5da88bc73)) [**breaking**]

  ````text
  The track symbol is now optional, simplifying composition with other
  widgets.
  ````

- *(table)* Add support for line alignment in the table widget ([#392](https://github.com/ratatui-org/ratatui/issues/392))
([7748720](https://github.com/ratatui-org/ratatui/commit/77487209634f26da32bc59d9280769d80cc7c25c))

  ````text
  * feat(table): enforce line alignment in table render

  * test(table): add table alignment render test
  ````

- *(widgets::table)* Add option to always allocate the "selection" constraint ([#375](https://github.com/ratatui-org/ratatui/issues/375))
([f63ac72](https://github.com/ratatui-org/ratatui/commit/f63ac72305f80062727d81996f9bdb523e666099))

  ````text
  * feat(table): add option to configure selection layout changes

  Before this option was available, selecting a row in the table when no row was selected
  previously made the tables layout change (the same applies to unselecting) by adding the width
  of the "highlight symbol" in the front of the first column, this option allows to configure this
  behavior.

  * refactor(table): refactor "get_columns_widths" to return (x, width)

  and "render" to make use of that

  * refactor(table): refactor "get_columns_widths" to take in a selection_width instead of a boolean

  also refactor "render" to make use of this change

  * fix(table): rename "highlight_set_selection_space" to "highlight_spacing"

  * style(table): apply doc-comment suggestions from code review
  ````

- *(uncategorized)* Expand serde attributes for `TestBuffer` ([#389](https://github.com/ratatui-org/ratatui/issues/389))
([57ea871](https://github.com/ratatui-org/ratatui/commit/57ea871753a5b23f302c6f0a83d98f6a1988abfb))

- *(uncategorized)* Add weak constraints to make rects closer to each other in size ✨ ([#395](https://github.com/ratatui-org/ratatui/issues/395))
([6153371](https://github.com/ratatui-org/ratatui/commit/61533712be57f3921217a905618b319975f90330))

  ````text
  Also make `Max` and `Min` constraints MEDIUM strength for higher priority over equal chunks
  ````

- *(uncategorized)* Simplify split function ✨ ([#411](https://github.com/ratatui-org/ratatui/issues/411))
([b090101](https://github.com/ratatui-org/ratatui/commit/b090101b231a467628c910f05a73715809cb8d73))

### Bug Fixes

- *(barchart)* Empty groups causes panic ([#333](https://github.com/ratatui-org/ratatui/issues/333))
([9c95673](https://github.com/ratatui-org/ratatui/commit/9c956733f740b18616974e2c7d786ca761666f79))

  ````text
  This unlikely to happen, since nobody wants to add an empty group.
  Even we fix the panic, things will not render correctly.
  So it is better to just not add them to the BarChart.
  ````

- *(block)* Fixed title_style not rendered ([#349](https://github.com/ratatui-org/ratatui/issues/349)) ([#363](https://github.com/ratatui-org/ratatui/issues/363))
([49a82e0](https://github.com/ratatui-org/ratatui/commit/49a82e062f2c46dc3060cdfdb230b65d9dbfb2d9))

- *(cargo)* Adjust minimum paste version ([#348](https://github.com/ratatui-org/ratatui/issues/348))
([8db9fb4](https://github.com/ratatui-org/ratatui/commit/8db9fb4aebd01e5ddc4edd68482361928f7e9c97))

  ````text
  ratatui is using features that are currently only available in paste 1.0.2; specifying the minimum version to be 1.0 will consequently cause a compilation error if cargo is only able to use a version less than 1.0.2.
  ````

- *(example)* Fix typo ([#337](https://github.com/ratatui-org/ratatui/issues/337))
([daf5890](https://github.com/ratatui-org/ratatui/commit/daf589015290ac8b379389d29ef90a1af15e3f75))

  ````text
  the existential feels
  ````

- *(layout)* Don't leave gaps between chunks ([#408](https://github.com/ratatui-org/ratatui/issues/408))
([56455e0](https://github.com/ratatui-org/ratatui/commit/56455e0fee57616f87ea43872fb7d5d9bb14aff5))

  ````text
  Previously the layout used the floor of the calculated start and width
  as the value to use for the split Rects. This resulted in gaps between
  the split rects.

  This change modifies the layout to round to the nearest column instead
  of taking the floor of the start and width. This results in the start
  and end of each rect being rounded the same way and being strictly
  adjacent without gaps.

  Because there is a required constraint that ensures that the last end is
  equal to the area end, there is no longer the need to fixup the last
  item width when the fill (as e.g. width = x.99 now rounds to x+1 not x).

  The colors example has been updated to use Ratio(1, 8) instead of
  Percentage(13), as this now renders without gaps for all possible sizes,
  whereas previously it would have left odd gaps between columns.
  ````

- *(layout)* Ensure left <= right ([#410](https://github.com/ratatui-org/ratatui/issues/410))
([f4ed3b7](https://github.com/ratatui-org/ratatui/commit/f4ed3b758450ef9c257705f3a1ea937329a968b4))

  ````text
  The recent refactor missed the positive width constraint
  ````

- *(readme)* Fix typo in readme ([#344](https://github.com/ratatui-org/ratatui/issues/344))
([d05ab6f](https://github.com/ratatui-org/ratatui/commit/d05ab6fb700527f0e062f334c7a5319c07099b04))

- *(readme)* Fix incorrect template link ([#338](https://github.com/ratatui-org/ratatui/issues/338))
([b9290b3](https://github.com/ratatui-org/ratatui/commit/b9290b35d13df57726d65a16d3c8bb18ce43e8c2))

- *(readme)* Fix typo in readme ([#336](https://github.com/ratatui-org/ratatui/issues/336))
([7e37a96](https://github.com/ratatui-org/ratatui/commit/7e37a96678440bc62cce52de840fef82eed58dd8))

- *(release)* Fix the last tag retrieval for alpha releases ([#416](https://github.com/ratatui-org/ratatui/issues/416))
([b6b2da5](https://github.com/ratatui-org/ratatui/commit/b6b2da5eb761ac5894cc7a2ee67f422312b63cfc))

- *(release)* Set the correct permissions for creating alpha releases ([#400](https://github.com/ratatui-org/ratatui/issues/400))
([778c320](https://github.com/ratatui-org/ratatui/commit/778c32000815b9abb0246c73997b1800256aade2))

- *(scrollbar)* Move symbols to symbols module ([#330](https://github.com/ratatui-org/ratatui/issues/330))
([7539f77](https://github.com/ratatui-org/ratatui/commit/7539f775fef4d816495e1e06732f6500cf08c126)) [**breaking**]

  ````text
  The symbols and sets are moved from `widgets::scrollbar` to
  `symbols::scrollbar`. This makes it consistent with the other symbol
  sets and allows us to make the scrollbar module private rather than
  re-exporting it.
  ````

- *(table)* Fix unit tests broken due to rounding ([#419](https://github.com/ratatui-org/ratatui/issues/419))
([dc55211](https://github.com/ratatui-org/ratatui/commit/dc552116cf5e83c7ffcc2f5299c00d2315490c1d))

  ````text
  The merge of the table unit tests after the rounding layout fix was not
  rebased correctly, this addresses the broken tests, makes them more
  concise while adding comments to help clarify that the rounding behavior
  is working as expected.
  ````

- *(uncategorized)* Correct minor typos in documentation ([#331](https://github.com/ratatui-org/ratatui/issues/331))
([13fb11a](https://github.com/ratatui-org/ratatui/commit/13fb11a62c826da412045d498a03673d130ec057))

### Refactor

- *(barchart)* Reduce some calculations ([#430](https://github.com/ratatui-org/ratatui/issues/430))
([fc727df](https://github.com/ratatui-org/ratatui/commit/fc727df7d2d8347434a7d3a4e19465b29d7a0ed8))

  ````text
  Calculating the label_offset is unnecessary, if we just render the
  group label after rendering the bars. We can just reuse bar_y.
  ````

- *(layout)* Simplify and doc split() ([#405](https://github.com/ratatui-org/ratatui/issues/405))
([de25de0](https://github.com/ratatui-org/ratatui/commit/de25de0a9506e53df1378929251594bccf63d932))

  ````text
  * test(layout): add tests for split()

  * refactor(layout): simplify and doc split()

  This is mainly a reduction in density of the code with a goal of
  improving mainatainability so that the algorithm is clear.
  ````

- *(layout)* Simplify split() function ([#396](https://github.com/ratatui-org/ratatui/issues/396))
([5195099](https://github.com/ratatui-org/ratatui/commit/519509945be866c3b2f6a4230ee317262266f894))

  ````text
  Removes some unnecessary code and makes the function more readable.
  Instead of creating a temporary result and mutating it, we just create
  the result directly from the list of changes.
  ````

### Documentation

- *(examples)* Fix the instructions for generating demo GIF ([#442](https://github.com/ratatui-org/ratatui/issues/442))
([7a70602](https://github.com/ratatui-org/ratatui/commit/7a70602ec6bfcfec51bafd3bdbd35ff68b64340c))

- *(examples)* Show layout constraints ([#393](https://github.com/ratatui-org/ratatui/issues/393))
([10dbd6f](https://github.com/ratatui-org/ratatui/commit/10dbd6f2075285473ef47c4c898ef2f643180cd1))

  ````text
  Shows the way that layout constraints interact visually

  ![example](https://vhs.charm.sh/vhs-1ZNoNLNlLtkJXpgg9nCV5e.gif)
  ````

- *(examples)* Add color and modifiers examples ([#345](https://github.com/ratatui-org/ratatui/issues/345))
([6ad4bd4](https://github.com/ratatui-org/ratatui/commit/6ad4bd4cf2e7ea7548e49e64f92114c30d61ebb2))

  ````text
  The intent of these examples is to show the available colors and
  modifiers.

  - added impl Display for Color

  ![colors](https://vhs.charm.sh/vhs-2ZCqYbTbXAaASncUeWkt1z.gif)
  ![modifiers](https://vhs.charm.sh/vhs-2ovGBz5l3tfRGdZ7FCw0am.gif)
  ````

- *(examples)* Regen block.gif in readme ([#365](https://github.com/ratatui-org/ratatui/issues/365))
([e82521e](https://github.com/ratatui-org/ratatui/commit/e82521ea798d1385f671e1849c48de42857bf87a))

- *(examples)* Update block example ([#351](https://github.com/ratatui-org/ratatui/issues/351))
([554805d](https://github.com/ratatui-org/ratatui/commit/554805d6cbbf140c6da474daa891e9e754a5d281))

  ````text
  ![Block example](https://vhs.charm.sh/vhs-5X6hpReuDBKjD6hLxmDQ6F.gif)
  ````

- *(examples)* Add examples readme with gifs ([#303](https://github.com/ratatui-org/ratatui/issues/303))
([add578a](https://github.com/ratatui-org/ratatui/commit/add578a7d6d342e3ebaa26e69452a2ab5b08b0c7))

  ````text
  This commit adds a readme to the examples directory with gifs of each
  example. This should make it easier to see what each example does
  without having to run it.

  I modified the examples to fit better in the gifs. Mostly this was just
  removing the margins, but for the block example I cleaned up the code a
  bit to make it more readable and changed it so the background bug is not
  triggered.

  For the table example, the combination of Min, Length, and Percent
  constraints was causing the table to panic when the terminal was too
  small. I changed the example to use the Max constraint instead of the
  Length constraint.

  The layout example now shows information about how the layout is
  constrained on each block (which is now a paragraph with a block).
  ````

- *(layout)* Add doc comments ([#403](https://github.com/ratatui-org/ratatui/issues/403))
([418ed20](https://github.com/ratatui-org/ratatui/commit/418ed20479e060c1bd2f430ae127eae19a013afc))

- *(layout::Constraint)* Add doc-comments for all variants ([#371](https://github.com/ratatui-org/ratatui/issues/371))
([c8ddc16](https://github.com/ratatui-org/ratatui/commit/c8ddc164c7941c31b1b5fa82345e452923ec56e7))

- *(lib)* Extract feature documentation from Cargo.toml ([#438](https://github.com/ratatui-org/ratatui/issues/438))
([8b36683](https://github.com/ratatui-org/ratatui/commit/8b36683571e078792b20d6f693b817522cf6e992))

  ````text
  * docs(lib): extract feature documentation from Cargo.toml

  * chore(deps): make `document-features` optional dependency

  * docs(lib): document the serde feature from features section
  ````

- *(paragraph)* Add more docs ([#428](https://github.com/ratatui-org/ratatui/issues/428))
([6d6ecee](https://github.com/ratatui-org/ratatui/commit/6d6eceeb88b4da593c63dad258d2724cd583f9e0))

- *(project)* Make the project description cooler ([#441](https://github.com/ratatui-org/ratatui/issues/441))
([47fe4ad](https://github.com/ratatui-org/ratatui/commit/47fe4ad69f527fcbf879e9fec2a4d3702badc76b))

  ````text
  * docs(project): make the project description cooler

  * docs(lib): simplify description
  ````

- *(readme)* Use the correct version for MSRV ([#369](https://github.com/ratatui-org/ratatui/issues/369))
([3a37d2f](https://github.com/ratatui-org/ratatui/commit/3a37d2f6ede02fdde9ddffbb996059d6b95f98e7))

- *(readme)* Fix widget docs links ([#346](https://github.com/ratatui-org/ratatui/issues/346))
([2920e04](https://github.com/ratatui-org/ratatui/commit/2920e045ba23aa2eb3a4049625cd256ff37076c9))

  ````text
  Add scrollbar, clear. Fix Block link. Sort
  ````

- *(span)* Update docs and tests for `Span` ([#427](https://github.com/ratatui-org/ratatui/issues/427))
([d0ee04a](https://github.com/ratatui-org/ratatui/commit/d0ee04a69f30506fae706b429f15fe63b056b79e))

- *(uncategorized)* Improve scrollbar doc comment ([#329](https://github.com/ratatui-org/ratatui/issues/329))
([c3f87f2](https://github.com/ratatui-org/ratatui/commit/c3f87f245a5a2fc180d4c8f64557bcff716d09a9))

### Performance

- *(bench)* Used `iter_batched` to clone widgets in setup function ([#383](https://github.com/ratatui-org/ratatui/issues/383))
([149d489](https://github.com/ratatui-org/ratatui/commit/149d48919d870e29a7f104664db11eb77fb951a8))

  ````text
  Replaced `Bencher::iter` by `Bencher::iter_batched` to clone the widget in the setup function instead of in the benchmark timing.
  ````

### Styling

- *(paragraph)* Add documentation for "scroll"'s "offset" ([#355](https://github.com/ratatui-org/ratatui/issues/355))
([ab5e616](https://github.com/ratatui-org/ratatui/commit/ab5e6166358b2e6f0e9601a1ec5480760b91ca8e))

  ````text
  * style(paragraph): add documentation for "scroll"'s "offset"

  * style(paragraph): add more text to the scroll doc-comment
  ````

### Testing

- *(block)* Test all block methods ([#431](https://github.com/ratatui-org/ratatui/issues/431))
([a890f2a](https://github.com/ratatui-org/ratatui/commit/a890f2ac004b0e45db40de222fe3560fe0fdf94b))

- *(block)* Add benchmarks ([#368](https://github.com/ratatui-org/ratatui/issues/368))
([e18393d](https://github.com/ratatui-org/ratatui/commit/e18393dbc6781a8b1266906e8ba7da019a0a5d82))

  ````text
  Added benchmarks to the block widget to uncover eventual performance issues
  ````

- *(canvas)* Add unit tests for line ([#437](https://github.com/ratatui-org/ratatui/issues/437))
([ad3413e](https://github.com/ratatui-org/ratatui/commit/ad3413eeec9aab1568f8519caaf5efb951b2800c))

  ````text
  Also add constructor to simplify creating lines
  ````

- *(canvas)* Add tests for rectangle ([#429](https://github.com/ratatui-org/ratatui/issues/429))
([ad4d6e7](https://github.com/ratatui-org/ratatui/commit/ad4d6e7dec0f7e4c4e2e5624ccec54eb71c3f5ca))

- *(clear)* Test Clear rendering ([#432](https://github.com/ratatui-org/ratatui/issues/432))
([e9bd736](https://github.com/ratatui-org/ratatui/commit/e9bd736b1a680204fa801a7208cddc477f208680))

- *(list)* Added benchmarks ([#377](https://github.com/ratatui-org/ratatui/issues/377))
([664fb4c](https://github.com/ratatui-org/ratatui/commit/664fb4cffd71c85da87545cb4258165c1a44afa6))

  ````text
  Added benchmarks for the list widget (render and render half scrolled)
  ````

- *(map)* Add unit tests ([#436](https://github.com/ratatui-org/ratatui/issues/436))
([f0716ed](https://github.com/ratatui-org/ratatui/commit/f0716edbcfd33d50e4e74eaf51fe5ad945dab6b3))

- *(sparkline)* Added benchmark ([#384](https://github.com/ratatui-org/ratatui/issues/384))
([3293c6b](https://github.com/ratatui-org/ratatui/commit/3293c6b80b0505f9ed031fc8d9678e3db627b7ad))

  ````text
  Added benchmark for the `sparkline` widget testing a basic render with different amount of data
  ````

- *(styled_grapheme)* Test StyledGrapheme methods ([#433](https://github.com/ratatui-org/ratatui/issues/433))
([292a11d](https://github.com/ratatui-org/ratatui/commit/292a11d81e2f8c7676cc897f3493b75903025766))

- *(table)* Add test for consistent table-column-width ([#404](https://github.com/ratatui-org/ratatui/issues/404))
([4cd843e](https://github.com/ratatui-org/ratatui/commit/4cd843eda97abbc8fa7af85a03c2fffafce3c676))

- *(tabs)* Add unit tests ([#439](https://github.com/ratatui-org/ratatui/issues/439))
([14eb6b6](https://github.com/ratatui-org/ratatui/commit/14eb6b69796550648f7d0d0427384b64c31e36d8))

- *(test_backend)* Add tests for TestBackend coverage ([#434](https://github.com/ratatui-org/ratatui/issues/434))
([b35f19e](https://github.com/ratatui-org/ratatui/commit/b35f19ec442d3eb4810f6181e03ba0d4c077b768))

  ````text
  These are mostly to catch any future bugs introduced in the test backend
  ````

- *(text)* Add unit tests ([#435](https://github.com/ratatui-org/ratatui/issues/435))
([fc9f637](https://github.com/ratatui-org/ratatui/commit/fc9f637fb08fdc2959a52ed3eb12643565c634d9))

### Miscellaneous Tasks

- *(changelog)* Ignore alpha tags ([#440](https://github.com/ratatui-org/ratatui/issues/440))
([6009844](https://github.com/ratatui-org/ratatui/commit/6009844e256cf926039fa969b9ad8896e2289213))

- *(changelog)* Show full commit message ([#423](https://github.com/ratatui-org/ratatui/issues/423))
([a937500](https://github.com/ratatui-org/ratatui/commit/a937500ae4ac0a60fc5db82f6ce105a1154215f6))

  ````text
  This allows someone reading the changelog to search for information
  about breaking changes or implementation of new functionality.

  - refactored the commit template part to a macro instead of repeating it
  - added a link to the commit and to the release
  - updated the current changelog for the alpha and unreleased changes
  - Automatically changed the existing * lists to - lists
  ````

- *(ci)* Update the name of the CI workflow ([#417](https://github.com/ratatui-org/ratatui/issues/417))
([89ef0e2](https://github.com/ratatui-org/ratatui/commit/89ef0e29f56078ed0629f2dce89656c1131ebda1))

- *(codecov)* Fix yaml syntax ([#407](https://github.com/ratatui-org/ratatui/issues/407))
([ea48af1](https://github.com/ratatui-org/ratatui/commit/ea48af1c9abac7012e3bf79e78c6179f889a6321))

  ````text
  a yaml file cannot contain tabs outside of strings
  ````

- *(docs)* Add doc comment bump to release documentation ([#382](https://github.com/ratatui-org/ratatui/issues/382))
([8b28672](https://github.com/ratatui-org/ratatui/commit/8b286721314142dc7078354015db909e6938068c))

- *(github)* Add kdheepak as a maintainer ([#343](https://github.com/ratatui-org/ratatui/issues/343))
([60a4131](https://github.com/ratatui-org/ratatui/commit/60a4131384e6c0b38b6a6e933e62646b5265ca60))

- *(github)* Rename `tui-rs-revival` references to `ratatui-org` ([#340](https://github.com/ratatui-org/ratatui/issues/340))
([964190a](https://github.com/ratatui-org/ratatui/commit/964190a859e6479f22c6ccae8305192f548fbcc3))

- *(make)* Add task descriptions to Makefile.toml ([#398](https://github.com/ratatui-org/ratatui/issues/398))
([268bbed](https://github.com/ratatui-org/ratatui/commit/268bbed17e0ebc18b39f3253c9beb92c21946c80))

- *(toolchain)* Bump msrv to 1.67 ([#361](https://github.com/ratatui-org/ratatui/issues/361))
([8cd3205](https://github.com/ratatui-org/ratatui/commit/8cd3205d70a1395d2c60fc26d76c300a2a463c9e)) [**breaking**]

  ````text
  * chore(toolchain)!: bump msrv to 1.67
  ````

- *(traits)* Add Display and FromStr traits ([#425](https://github.com/ratatui-org/ratatui/issues/425))
([98155dc](https://github.com/ratatui-org/ratatui/commit/98155dce25bbc0e8fe271735024a1f6bf2279d67))

  ````text
  Use strum for most of these, with a couple of manual implementations,
  and related tests
  ````

- *(uncategorized)* Create rust-toolchain.toml ([#415](https://github.com/ratatui-org/ratatui/issues/415))
([d2429bc](https://github.com/ratatui-org/ratatui/commit/d2429bc3e44a34197511192dbd215dd32fdf2d9c))

- *(uncategorized)* Use vhs to create demo.gif ([#390](https://github.com/ratatui-org/ratatui/issues/390))
([8c55158](https://github.com/ratatui-org/ratatui/commit/8c551588224ca97ee07948b445aa2ac9d05f997d))

  ````text
  The bug that prevented braille rendering is fixed, so switch to VHS for
  rendering the demo gif

  ![Demo of Ratatui](https://vhs.charm.sh/vhs-tF0QbuPbtHgUeG0sTVgFr.gif)
  ````

- *(uncategorized)* Implement `Hash` common traits ([#381](https://github.com/ratatui-org/ratatui/issues/381))
([8c4a2e0](https://github.com/ratatui-org/ratatui/commit/8c4a2e0fbfd021f1e087bb7256d9c6457742ea39))

  ````text
  Reorder the derive fields to be more consistent:

      Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash

  Hash trait won't be impl in this PR due to rust std design.
  If we need hash trait for f64 related structs in the future,
  we should consider wrap f64 into a new type.
  ````

- *(uncategorized)* Implement `Eq & PartialEq` common traits ([#357](https://github.com/ratatui-org/ratatui/issues/357))
([181706c](https://github.com/ratatui-org/ratatui/commit/181706c564d86e02991f89ec674b1af1d7f393fe))

  ````text
  Reorder the derive fields to be more consistent:

      Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash
  ````

- *(uncategorized)* Implement `Clone & Copy` common traits ([#350](https://github.com/ratatui-org/ratatui/issues/350))
([440f62f](https://github.com/ratatui-org/ratatui/commit/440f62ff5435af9536c55d17707a9bc48dae92cc))

  ````text
  Implement `Clone & Copy` common traits for most structs in src.

  Only implement `Copy` for structs that are simple and trivial to copy.

  Reorder the derive fields to be more consistent:

      Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash
  ````

- *(uncategorized)* Implement `Debug & Default` common traits ([#339](https://github.com/ratatui-org/ratatui/issues/339))
([bf49446](https://github.com/ratatui-org/ratatui/commit/bf4944683d6afb6f42bec80a1bd308ecdac50cbc))

  ````text
  Implement `Debug & Default` common traits for most structs in src.

  Reorder the derive fields to be more consistent:

      Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash
  ````

### Build

- *(deps)* Upgrade crossterm to 0.27 ([#380](https://github.com/ratatui-org/ratatui/issues/380))
([37fa6ab](https://github.com/ratatui-org/ratatui/commit/37fa6abe9d5dc459dc9855ea10f06afa72717c98))

- *(examples)* Fix cargo make run-examples ([#327](https://github.com/ratatui-org/ratatui/issues/327))
([e2cb11c](https://github.com/ratatui-org/ratatui/commit/e2cb11cc30072d90b20e04270c1fa97c18ab6f3f))

  ````text
  Enables the all-widgets feature so that the calendar example runs correctly
  ````

- *(uncategorized)* Forbid unsafe code ([#332](https://github.com/ratatui-org/ratatui/issues/332))
([0fb1ed8](https://github.com/ratatui-org/ratatui/commit/0fb1ed85c6232966ab25c8b3cab0fc277e9b69a6))

  ````text
  This indicates good (high level) code and is used by tools like cargo-geiger.
  ````

### Continuous Integration

- *(coverage)* Exclude examples directory from coverage ([#373](https://github.com/ratatui-org/ratatui/issues/373))
([de9f52f](https://github.com/ratatui-org/ratatui/commit/de9f52ff2cc606e1bf6b6bd8b97907afd73860fe))

- *(uncategorized)* Don't fail fast ([#364](https://github.com/ratatui-org/ratatui/issues/364))
([9191ad6](https://github.com/ratatui-org/ratatui/commit/9191ad60fd4fc3ddf8650a8f5eed87216a0e5c6f))

  ````text
  Run all the tests rather than canceling when one test fails. This allows
  us to see all the failures, rather than just the first one if there are
  multiple. Specifically this is useful when we have an issue in one
  toolchain or backend.
  ````

- *(uncategorized)* Add coverage token ([#352](https://github.com/ratatui-org/ratatui/issues/352))
([6f659cf](https://github.com/ratatui-org/ratatui/commit/6f659cfb07aad5ad2524f32fe46c45b84c8e9e34))

### Contributors

Thank you so much to everyone that contributed to this release!

Here is the list of contributors who have contributed to `ratatui` for the first time!

- @[EdJoPaTo](https://github.com/EdJoPaTo)
- @[mhovd](https://github.com/mhovd)
- @[joshrotenberg](https://github.com/joshrotenberg)
- @[t-nil](https://github.com/t-nil)
- @[ndd7xv](https://github.com/ndd7xv)
- @[TieWay59](https://github.com/TieWay59)
- @[Valentin271](https://github.com/Valentin271)
- @[hasezoey](https://github.com/hasezoey)
- @[jkcdarunday](https://github.com/jkcdarunday)
- @[stappersg](https://github.com/stappersg)
- @[benjajaja](https://github.com/benjajaja)

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

- Bump `crossterm` to `0.25`

## v0.18.0 - 2022-04-24

### Features

- Update `crossterm` to `0.23`

## v0.17.0 - 2022-01-22

### Features

- Add option to `widgets::List` to repeat the highlight symbol for each line of multi-line items (#533).
- Add option to control the alignment of `Axis` labels in the `Chart` widget (#568).

### Breaking changes

- The minimum supported rust version is now `1.56.1`.

#### New default backend and consolidated backend options (#553)

- `crossterm` is now the default backend.
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

- [curses](https://github.com/fdehau/tui-rs/blob/v0.16.0/src/backend/curses.rs)
- [rustbox](https://github.com/fdehau/tui-rs/blob/v0.16.0/src/backend/rustbox.rs)

#### Canvas labels (#543)

- Labels of the `Canvas` widget are now `text::Spans`.
The signature of `widgets::canvas::Context::print` has thus been updated:

```diff
- ctx.print(x, y, "Some text", Color::Yellow);
+ ctx.print(x, y, Span::styled("Some text", Style::default().fg(Color::Yellow)))
```

## v0.16.0 - 2021-08-01

### Features

- Update `crossterm` to `0.20`.
- Add `From<Cow<str>>` implementation for `text::Text` (#471).
- Add option to right or center align the title of a `widgets::Block` (#462).

### Fixes

- Apply label style in `widgets::Gauge` and avoid panics because of overflows with long labels (#494).
- Avoid panics because of overflows with long axis labels in `widgets::Chart` (#512).
- Fix computation of column widths in `widgets::Table` (#514).
- Fix panics because of invalid offset when input changes between two frames in `widgets::List` and
  `widgets::Chart` (#516).

## v0.15.0 - 2021-05-02

### Features

- Update `crossterm` to `0.19`.
- Update `rand` to `0.8`.
- Add a read-only view of the terminal state after the draw call (#440).

### Fixes

- Remove compile warning in `TestBackend::assert_buffer` (#466).

## v0.14.0 - 2021-01-01

### Breaking changes

#### New API for the Table widget

The `Table` widget got a lot of improvements that should make it easier to work with:

- It should not longer panic when rendered on small areas.
- `Row`s are now a collection of `Cell`s, themselves wrapping a `Text`. This means you can style
the entire `Table`, an entire `Row`, an entire `Cell` and rely on the styling capabilities of
`Text` to get full control over the look of your `Table`.
- `Row`s can have multiple lines.
- The header is now optional and is just another `Row` always visible at the top.
- `Row`s can have a bottom margin.
- The header alignment is no longer off when an item is selected.

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

- Add `LineGauge` widget which is a more compact variant of the existing `Gauge`.
- Bump `crossterm` to 0.18

### Fixes

- Take into account the borders of the `Table` widget when the widths of columns is controlled by
`Percentage` and `Ratio` constraints.

## v0.12.0 - 2020-09-27

### Features

- Make it easier to work with string with multiple lines in `Text` (#361).

### Fixes

- Fix a style leak in `Graph` so components drawn on top of the plotted data (i.e legend and axis
titles) are not affected by the style of the `Dataset`s (#388).
- Make sure `BarChart` shows bars with the max height only when the plotted data is actually equal
to the max (#383).

## v0.11.0 - 2020-09-20

### Features

- Add the dot character as a new type of canvas marker (#350).
- Support more style modifiers on Windows (#368).

### Fixes

- Clearing the terminal through `Terminal::clear` will cause the whole UI to be redrawn (#380).
- Fix incorrect output when the first diff to draw is on the second cell of the terminal (#347).

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

- Fix out of bounds panic in `widgets::Tabs` when the widget is rendered on
small areas.

## v0.9.4 - 2020-05-12

### Bug Fixes

- Ignore zero-width graphemes in `Buffer::set_stringn`.

## v0.9.3 - 2020-05-11

### Bug Fixes

- Fix usize overflows in `widgets::Chart` when a dataset is empty.

## v0.9.2 - 2020-05-10

### Bug Fixes

- Fix usize overflows in `widgets::canvas::Line` drawing algorithm.

## v0.9.1 - 2020-04-16

### Bug Fixes

- The `List` widget now takes into account the width of the `highlight_symbol`
when calculating the total width of its items. It prevents items to overflow
outside of the widget area.

## v0.9.0 - 2020-04-14

### Features

- Introduce stateful widgets, i.e widgets that can take advantage of keeping
some state around between two draw calls (#210 goes a bit more into the
details).
- Allow a `Table` row to be selected.

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

- Add a way to choose the type of border used to draw a block. You can now
choose from plain, rounded, double and thick lines.

- Add a `graph_type` property on the `Dataset` of a `Chart` widget. By
default it will be `Scatter` where the points are drawn as is. An other
option is `Line` where a line will be draw between each consecutive points
of the dataset.
- Style methods are now const, allowing you to initialize const `Style`
objects.
- Improve control over whether the legend in the `Chart` widget is shown or
not. You can now set custom constraints using
`Chart::hidden_legend_constraints`.
- Add `Table::header_gap` to add some space between the header and the first
row.
- Remove `log` from the dependencies
- Add a way to use a restricted set of unicode symbols in several widgets to
improve portability in exchange of a degraded output. (see `BarChart::bar_set`,
`Sparkline::bar_set` and `Canvas::marker`). You can check how the
`--enhanced-graphics` flag is used in the demos.

### Breaking Changes

- `Widget::render` has been deleted. You should now use `Frame::render_widget`
to render a widget on the corresponding `Frame`. This makes the `Widget`
implementation totally decoupled from the `Frame`.

```rust
// Before
Block::default().render(&mut f, size);

// After
let block = Block::default();
f.render_widget(block, size);
```

- `Widget::draw` has been renamed to `Widget::render` and the signature has
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

- `Widget::background` has been replaced by `Buffer::set_background`

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

- Update the `Shape` trait for objects that can be draw on a `Canvas` widgets.
Instead of returning an iterator over its points, a `Shape` is given a
`Painter` object that provides a `paint` as well as a `get_point` method. This
gives the `Shape` more information about the surface it will be drawn to. In
particular, this change allows the `Line` shape to use a more precise and
efficient drawing algorithm (Bresenham's line algorithm).

- `SelectableList` has been deleted. You can now take advantage of the
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

- `widgets::Marker` has been moved to `symbols::Marker`

## v0.8.0 - 2019-12-15

### Breaking Changes

- Bump crossterm to 0.14.
- Add cross symbol to the symbols list.

### Bug Fixes

- Use the value of `title_style` to style the title of `Axis`.

## v0.7.0 - 2019-11-29

### Breaking Changes

- Use `Constraint` instead of integers to specify the widths of the `Table`
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

- Bump crossterm to 0.13.
- Use Github Actions for CI (Travis and Azure Pipelines integrations have been deleted).

### Features

- Add support for horizontal and vertical margins in `Layout`.

## v0.6.2 - 2019-07-16

### Features

- `Text` implements PartialEq

### Bug Fixes

- Avoid overflow errors in canvas

## v0.6.1 - 2019-06-16

### Bug Fixes

- Avoid a division by zero when all values in a barchart are equal to 0.
- Fix the inverted cursor position in the curses backend.
- Ensure that the correct terminal size is returned when using the crossterm
backend.
- Avoid highlighting the separator after the selected item in the Tabs widget.

## v0.6.0 - 2019-05-18

### Breaking Changes

- Update crossterm backend

## v0.5.1 - 2019-04-14

### Bug Fixes

- Fix a panic in the Sparkline widget

## v0.5.0 - 2019-03-10

### Features

- Add a new curses backend (with Windows support thanks to `pancurses`).
- Add `Backend::get_cursor` and `Backend::set_cursor` methods to query and
set the position of the cursor.
- Add more constructors to the `Crossterm` backend.
- Add a demo for all backends using a shared UI and application state.
- Add `Ratio` as a new variant of layout `Constraint`. It can be used to define
exact ratios constraints.

### Breaking Changes

- Add support for multiple modifiers on the same `Style` by changing `Modifier`
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

- Ensure correct behavior of the alternate screens with the `Crossterm` backend.
- Fix out of bounds panic when two `Buffer` are merged.

## v0.4.0 - 2019-02-03

### Features

- Add a new canvas shape: `Rectangle`.
- Official support of `Crossterm` backend.
- Make it possible to choose the divider between `Tabs`.
- Add word wrapping on Paragraph.
- The gauge widget accepts a ratio (f64 between 0 and 1) in addition of a
percentage.

### Breaking Changes

- Upgrade to Rust 2018 edition.

### Bug Fixes

- Fix rendering of double-width characters.
- Fix race condition on the size of the terminal and expose a size that is
safe to use when drawing through `Frame::size`.
- Prevent unsigned int overflow on large screens.

## v0.3.0 - 2018-11-04

### Features

- Add experimental test backend

## v0.3.0-beta.3 - 2018-09-24

### Features

- `show_cursor` is called when `Terminal` is dropped if the cursor is hidden.

## v0.3.0-beta.2 - 2018-09-23

### Breaking Changes

- Remove custom `termion` backends. This is motivated by the fact that
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

- Replace `Item` by a generic and flexible `Text` that can be used in both
`Paragraph` and `List` widgets.
- Remove unnecessary borrows on `Style`.

## v0.3.0-beta.0 - 2018-09-04

### Features

- Add a basic `Crossterm` backend

### Breaking Changes

- Remove `Group` and introduce `Layout` in its place
  - `Terminal` is no longer required to compute a layout
  - `Size` has been renamed `Constraint`
- Widgets are rendered on a `Frame` instead of a `Terminal` in order to
avoid mixing `draw` and `render` calls
- `draw` on `Terminal` expects a closure where the UI is built by rendering
widgets on the given `Frame`
- Update `Widget` trait
  - `draw` takes area by value
  - `render` takes a `Frame` instead of a `Terminal`
- All widgets use the consumable builder pattern
- `SelectableList` can have no selected item and the highlight symbol is hidden
in this case
- Remove markup language inside `Paragraph`. `Paragraph` now expects an iterator
of `Text` items

## v0.2.3 - 2018-06-09

### Features

- Add `start_corner` option for `List`
- Add more text alignment options for `Paragraph`

## v0.2.2 - 2018-05-06

### Features

- `Terminal` implements `Debug`

### Breaking Changes

- Use `FnOnce` instead of `FnMut` in Group::render

## v0.2.1 - 2018-04-01

### Features

- Add `AlternateScreenBackend` in `termion` backend
- Add `TermionBackend::with_stdout` in order to let an user of the library
provides its own termion struct
- Add tests and documentation for `Buffer::pos_of`
- Remove leading whitespaces when wrapping text

### Bug Fixes

- Fix `debug_assert` in `Buffer::pos_of`
- Pass the style of `SelectableList` to the underlying `List`
- Fix missing character when wrapping text
- Fix panic when specifying layout constraints

## v0.2.0 - 2017-12-26

### Features

- Add `MouseBackend` in `termion` backend to handle scroll and mouse events
- Add generic `Item` for items in a `List`
- Drop `log4rs` as a dev-dependencies in favor of `stderrlog`

### Breaking Changes

- Rename `TermionBackend` to `RawBackend` (to distinguish it from the `MouseBackend`)
- Generic parameters for `List` to allow passing iterators as items
- Generic parameters for `Table` to allow using iterators as rows and header
- Generic parameters for `Tabs`
- Rename `border` bitflags to `Borders`
