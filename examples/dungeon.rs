use orbgame::prelude::*;
use orbgame::theme::DEFAULT_THEME_CSS;
use std::{cell::Cell, rc::Rc};

static DUNGEON_THEME: &'static str = include_str!("res/dungeon/theme.css");

fn get_theme() -> ThemeValue {
    ThemeValue::create_from_css(DEFAULT_THEME_CSS)
        .extension_css(DUNGEON_THEME)
        .build()
}

widget!(MapView {});

impl Template for MapView {
    fn template(self, _: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MapView").child(
            Container::create()
                .child(TextBlock::create().text("Dungeon").build(ctx))
                .build(ctx),
        )
    }
}

#[derive(Copy, Clone)]
pub enum MenuAction {
    Start,
    Quit,
}

#[derive(Default)]
pub struct MenuViewState {
    action: Cell<Option<MenuAction>>,
}

impl MenuViewState {
    fn action(&self, action: MenuAction) {
        self.action.set(Some(action));
    }
}

impl State for MenuViewState {
    fn update(&self, ctx: &mut Context<'_>) {
        if let Some(action) = self.action.get() {
            match action {
                MenuAction::Start => {
                    ctx.widget()
                        .set::<Visibility>(Visibility::from("collapsed"));
                }
                MenuAction::Quit => {
                    ctx.push_event(SystemEvent::Quit);
                }
            }

            self.action.set(None);
        }
    }
}

widget!(
    MenuView<MenuViewState> {
        selector: Selector
    }
);

impl Template for MenuView {
    fn template(self, _: Entity, ctx: &mut BuildContext) -> Self {
        let state = self.clone_state();
        let ng_state = state.clone();
        let q_state = state.clone();

        self.name("MenuView")
            .selector(Selector::default().id("menu_view"))
            .child(
                Grid::create()
                    .selector(Selector::from("grid").class("start"))
                    .child(
                        Container::create()
                            .padding(16.0)
                            .selector(Selector::from("container").class("menu"))
                            .vertical_alignment("Center")
                            .horizontal_alignment("Center")
                            .child(
                                Stack::create()
                                    .child(
                                        TextBlock::create()
                                            .selector(Selector::from("textblock").class("h1"))
                                            .text("Dungeon")
                                            .horizontal_alignment("Center")
                                            .build(ctx),
                                    )
                                    .child(
                                        Button::create()
                                            .margin((0.0, 16.0, 0.0, 0.0))
                                            .text("Start Game")
                                            .on_click(move |_| {
                                                ng_state.action(MenuAction::Start);
                                                true
                                            })
                                            .build(ctx),
                                    )
                                    .child(
                                        Button::create()
                                            .margin((0.0, 8.0, 0.0, 0.0))
                                            .text("Quit")
                                            .on_click(move |_| {
                                                q_state.action(MenuAction::Quit);
                                                true
                                            })
                                            .build(ctx),
                                    )
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}

#[derive(Default)]
pub struct GameViewState {}

impl State for GameViewState {
    fn receive_messages(&self, ctx: &mut Context<'_>, messages: &Vec<MessageBox>) {
        for message in messages {
            if let Ok(message) = message.downcast_ref::<StringMessage>() {
                match message.0.as_str() {
                    "start" => {
                        if let Some(menu_view) = &mut ctx.child_by_id("menu_view") {
                            menu_view.set::<Visibility>(Visibility::from("Collapsed"));
                        }
                    }
                    "quit" => {
                        ctx.push_event(SystemEvent::Quit);
                    }
                    _ => {}
                }
            }
        }
    }
}

widget!(
    GameView<GameViewState> {
        selector: Selector
    }
);

impl Template for GameView {
    fn template(self, _: Entity, ctx: &mut BuildContext) -> Self {
        self.name("GameView")
            .selector(Selector::default().id("game_view"))
            .child(
                Grid::create()
                    .child(
                        MapView::create()
                            // .visibility(Visibility::from("collapsed"))
                            .build(ctx),
                    )
                    .child(MenuView::create().build(ctx))
                    .build(ctx),
            )
    }
}

fn main() {
    Game::new()
        .window(|ctx| {
            Window::create()
                .title("OrbGame - dungeon example")
                .position((100.0, 100.0))
                .size(800.0, 600.0)
                .theme(get_theme())
                .child(GameView::create().build(ctx))
                .build(ctx)
        })
        .run();
}
