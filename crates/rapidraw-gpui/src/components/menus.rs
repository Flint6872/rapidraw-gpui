use crate::actions;
use gpui_kit::*;

pub fn setup_menus(cx: &mut App) {
    cx.set_menus(vec![Menu {
        name: "File".into(),
        items: vec![
            MenuItem::action("Open Image...", actions::OpenImage),
            MenuItem::separator(),
            MenuItem::action("Quit", actions::FileQuit),
        ],
        disabled: false,
    }]);
}
