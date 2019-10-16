use orbgame::prelude::*;
use orbgame::theme::DEFAULT_THEME_CSS;
use std::{cell::{Cell, RefCell}, rc::Rc};

static DUNGEON_THEME: &'static str = include_str!("res/dungeon/theme.css");

fn get_theme() -> ThemeValue {
    ThemeValue::create_from_css(DEFAULT_THEME_CSS)
        .extension_css(DUNGEON_THEME)
        .build()
}

#[derive(Copy, Clone)]
pub enum MapViewAction {
    Start,
}

#[derive(Default, Clone)]
pub struct MapViewState {
    action: Cell<Option<MapViewAction>>,
}

impl MapViewState {
    pub fn action(&self, action: MapViewAction) {
        self.action.set(Some(action));
    }
}

impl State for MapViewState {
    fn update(&self, ctx: &mut Context<'_>) {
        if let Some(action) = self.action.get() {
            match action {
                MapViewAction::Start => {
                    ctx.widget().set::<Visibility>(Visibility::from("Visible"));
                }
            }

            self.action.set(None);
        }
    }
}

widget!(MapView<MapViewState> {

});

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
    map_view_state: RefCell<Option<Rc<MapViewState>>>,
}

impl MenuViewState {
    fn action(&self, action: MenuAction) {
        self.action.set(Some(action));
    }

    fn set_map_view_state(&self, state: Rc<MapViewState>) {
        *self.map_view_state.borrow_mut() = Some(state);
    }
}

impl State for MenuViewState {
    fn update(&self, ctx: &mut Context<'_>) {
        if let Some(action) = self.action.get() {
            match action {
                MenuAction::Start => {
                    ctx.widget()
                        .set::<Visibility>(Visibility::from("collapsed"));
                    if let Some(state) = &*self.map_view_state.borrow() {
                        state.action(MapViewAction::Start);
                    }
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
widget!(
    GameView {
        selector: Selector
    }
);

impl Template for GameView {
    fn template(self, _: Entity, ctx: &mut BuildContext) -> Self {
        let map_view = MapView::create();
        let map_view_state = map_view.clone_state();
        let menu_view = MenuView::create();
        let menu_view_state = menu_view.clone_state();
        menu_view_state.set_map_view_state(map_view_state);

        self.name("GameView")
            .selector(Selector::default().id("game_view"))
            .child(
                Grid::create()
                    .child(
                        map_view
                            .visibility(Visibility::from("collapsed"))
                            .build(ctx),
                    )
                    .child(menu_view.build(ctx))
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
