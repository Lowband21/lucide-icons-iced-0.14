# lucide-icons

Auto-generated rust icon definitions for [lucide icons](https://github.com/lucide-icons/lucide) [version 0.525.0](https://github.com/lucide-icons/lucide/releases/tag/0.525.0)

The library provides an `Icon` enum which contains all lucide icon variants:

```rust
use lucide_icons::Icon;

fn main() {
    let icon = Icon::Anvil;
    assert_eq!(format!("{icon}"), String::from("anvil"));
    println!("unicode = {}", icon.unicode());
}
```

With the `iced` feature the library also provides the icons as iced widgets:

```rust
use lucide_icons::lucide_font_bytes;
use lucide_icons::iced::icon_anvil;

fn setup_application() {
    // get font bytes for the bundled font
    let bytes = lucide_font_bytes();

    // add the font to iced
    let settings = iced::Settings {
        fonts: vec![bytes.into()],
        ..Default::default()
    };

    // run app with settings...
}

fn view() -> iced::Element<'_, Message, Theme, iced::Renderer> {
    iced::widget::column![
        icon_anvil()
    ].into()
}

```

For more details have a look at the [generator repository page](https://github.com/WhySoBad/lucide-icons-rs/)