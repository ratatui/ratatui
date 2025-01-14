# Examples

This folder contains examples that are more app focused. There are also other examples in example
folders under each crate folder e.g. [ratatui examples], [ratatui-widgets examples].

[ratatui examples]: ../ratatui/examples
[ratatui-widgets examples]: ../ratatui-widgets/examples

You can run these examples using:

```shell
cargo run -p example-name
```

This folder might use unreleased code. Consider viewing the examples in the `latest` branch instead
of the `main` branch for code which is guaranteed to work with the released ratatui version.

> [!WARNING]
>
> There may be backwards incompatible changes in these examples, as they are designed to compile
> against the `main` branch.
>
> There are a few workaround for this problem:
>
> - View the examples as they were when the latest version was release by selecting the tag that
>   matches that version. E.g. <https://github.com/ratatui/ratatui/tree/v0.26.1/examples>.
> - If you're viewing this file on GitHub, there is a combo box at the top of this page which
>   allows you to select any previous tagged version.
> - To view the code locally, checkout the tag. E.g. `git switch --detach v0.26.1`.
> - Use the latest [alpha version of Ratatui] in your app. These are released weekly on Saturdays.
> - Compile your code against the main branch either locally by adding e.g. `path = "../ratatui"` to
>   the dependency, or remotely by adding `git = "https://github.com/ratatui/ratatui"`
>
> For a list of unreleased breaking changes, see [BREAKING-CHANGES.md].
>
> We don't keep the CHANGELOG updated with unreleased changes, check the git commit history or run
> `git-cliff -u` against a cloned version of this repository.

## Demo

This is the original demo example from the main README. It is available for each of the backends.
[Source](./apps/demo/).

![Demo](https://github.com/ratatui/ratatui/blob/images/examples/demo.gif?raw=true)

## Demo2

This is the demo example from the main README and crate page. [Source](./apps/demo2/).

![Demo2](https://github.com/ratatui/ratatui/blob/images/examples/demo2.gif?raw=true)

## Async GitHub demo

Shows how to fetch data from GitHub API asynchronously. [Source](./apps/async-github/).

## Calendar explorer demo

Shows how to render a calendar with different styles. [Source](./apps/calendar-explorer/).

## Canvas demo

Shows how to render a canvas with different shapes. [Source](./apps/canvas/).

## Colors-RGB demo

This example shows the full range of RGB colors in an animation. [Source](./apps/colors-rgb/).

## Input form

Shows how to render a form with input fields. [Source](./apps/input-form/).

## Mouse Drawing demo

Shows how to handle mouse events. [Source](./apps/mouse-drawing/).

## Weather demo

Shows how to render weather data using barchart widget. [Source](./apps/weather/).
