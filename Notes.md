## TODO:

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
