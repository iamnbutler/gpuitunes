# gpuitunes

Created with [create-gpui-app](https://crates.io/crates/create-gpui-app).

- [`gpui`](https://www.gpui.rs/)
- [GPUI documentation](https://github.com/zed-industries/zed/tree/main/crates/gpui/docs)
- [GPUI examples](https://github.com/zed-industries/zed/tree/main/crates/gpui/examples)

Important note: `gpui` is still currently a part of [Zed](https://github.com/zed-industries/zed). It's unlikely it can be used in any sort of production application at this time.

## Usage

- Ensure Rust is installed - [Rustup](https://rustup.rs/)
- Run your app with `cargo run`

---

## gpui TODOs:

- [ ] Add radial gradient to gpui
- [ ] BoxShadow can't have a blur radius of 0
- [ ] We don't seem to render divs with no children

## Maybe useful to share?

### Create an app without a titlebar

You can create an app without a titlebar by setting the `titlebar` option to `None` in the `WindowOptions` struct.

```rust
/// Minimal Example
fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(
            WindowOptions {
                titlebar: None,
                ..Default::default()
            },
            |cx| {
                cx.new_view(|_cx| SomeView {
                    text: "World".into(),
                })
            },
        )
        .unwrap();

        cx.activate(true);
    });
}
```
