use orbgame::prelude::*;

struct MainView;

impl Widget for MainView {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Single)
            .with_debug_name("MainView")
            .with_child(
                Container::create()
                    .as_parent_type(ParentType::Single)
                    .with_child(TextBlock::create().with_property(Label::from("Adventure"))),
            )
    }
}

fn main() {
    let mut game = Game::default();
    game
        .create_window()
        .with_bounds(Bounds::new(0, 0, 800, 600))
        .with_title("OrbGame - Adventure example")
        .with_root(MainView::create())
        .with_debug_flag(true)
        .build();
    game.run();
}
