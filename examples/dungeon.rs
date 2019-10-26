use orbgame::theme::DEFAULT_THEME_CSS;
use orbgame::{prelude::*, shell::KeyEvent};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

static DUNGEON_THEME: &'static str = include_str!("../res/dungeon/theme.css");

fn get_theme() -> ThemeValue {
    ThemeValue::create_from_css(DEFAULT_THEME_CSS)
        .extension_css(DUNGEON_THEME)
        .build()
}

#[derive(Copy, Clone)]
pub enum MapViewAction {
    OpenMenu,
}

#[derive(Default, Clone)]
pub struct MapViewState {
    action: Cell<Option<MapViewAction>>,
}

impl MapViewState {
    fn action(&self, action: MapViewAction) {
        self.action.set(Some(action));
    }
}

impl State for MapViewState {
    fn init(&self, ctx: &mut Context<'_>) {
        // workaround
        ctx.window().get_mut::<Global>("global").focused_widget = Some(ctx.entity);
    }

    fn update(&self, ctx: &mut Context<'_>) {
        if let Some(action) = self.action.get() {
            if let Some(window_id) = ctx.parent_entity_by_element("window") {
                match action {
                    MapViewAction::OpenMenu => {
                        ctx.push_event_by_entity(GameEvent::OpenMenu, window_id);
                    }
                }
            }

            self.action.set(None);
        }
    }
}

widget!(MapView<MapViewState> : KeyDownHandler {
    selector: Selector
});

impl Template for MapView {
    fn template(self, _: Entity, ctx: &mut BuildContext) -> Self {
        let state = self.clone_state();

        self.name("MapView")
            .child(
                Container::create()
                    .selector("container")
                    .child(
                        Grid::create()
                            .child(
                                TileMap::create()
                                    .camera(
                                        CameraBuilder::new()
                                            .x(0.0)
                                            .y(0.0)
                                            .width(352.0)
                                            .height(352.0)
                                            .max_width(352.0)
                                            .max_height(352.0)
                                            .build(),
                                    )
                                    .map("res/dungeon/dungeon.ron")
                                    .image("res/dungeon/tile_set.png")
                                    .build(ctx),
                            )
                            .child(
                                TextBlock::create()
                                    .text("Press ESC to open menu")
                                    .vertical_alignment("bottom")
                                    .margin(4.0)
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .build(ctx),
            )
            .on_key_down(move |event: KeyEvent| -> bool {
                if event.key == Key::Escape {
                    state.action(MapViewAction::OpenMenu);
                }
                true
            })
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
            if let Some(window_id) = ctx.parent_entity_by_element("window") {
                match action {
                    MenuAction::Start => {
                        ctx.push_event_by_entity(GameEvent::StartGame, window_id);
                    }
                    MenuAction::Quit => {
                        ctx.push_event_by_entity(GameEvent::Quit, window_id);
                    }
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

        self.name("MenuView").child(
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

#[derive(Copy, Clone)]
pub enum GameEvent {
    OpenMenu,
    StartGame,
    Quit,
}

pub type GameHandlerFn = dyn Fn(&GameEvent) -> bool + 'static;

pub struct GameEventHandler {
    handler: Rc<GameHandlerFn>,
}

impl Into<Rc<dyn EventHandler>> for GameEventHandler {
    fn into(self) -> Rc<dyn EventHandler> {
        Rc::new(self)
    }
}

impl EventHandler for GameEventHandler {
    fn handle_event(&self, event: &EventBox) -> bool {
        if let Ok(event) = event.downcast_ref::<GameEvent>() {
            return (self.handler)(event);
        }

        return false;
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<GameEvent>()
    }
}

impl Event for GameEvent {}

#[derive(Default, Clone)]
pub struct GameViewState {
    event: Cell<Option<GameEvent>>,
}

impl GameViewState {
    fn game_event(&self, event: &GameEvent) {
        self.event.set(Some(*event));
    }
}

impl State for GameViewState {
    fn update(&self, ctx: &mut Context<'_>) {
        if let Some(event) = self.event.get() {
            match event {
                GameEvent::OpenMenu => {
                    ctx.child_by_id("map_view")
                        .unwrap()
                        .set("visibility", Visibility::from("collapsed"));
                    ctx.child_by_id("menu_view")
                        .unwrap()
                        .set("visibility", Visibility::from("visible"));
                }
                GameEvent::StartGame => {
                    ctx.child_by_id("menu_view")
                        .unwrap()
                        .set("visibility", Visibility::from("collapsed"));
                    ctx.child_by_id("map_view")
                        .unwrap()
                        .set("visibility", Visibility::from("visible"));
                }
                GameEvent::Quit => {
                    ctx.push_event(SystemEvent::Quit);
                }
            }

            self.event.set(None);
        }
    }
}

widget!(GameView<GameViewState> { selector: Selector });

impl GameView {
    fn on_game_event<H: Fn(&GameEvent) -> bool + 'static>(self, handler: H) -> Self {
        self.insert_handler(GameEventHandler {
            handler: Rc::new(handler),
        })
    }
}

impl Template for GameView {
    fn template(self, _: Entity, ctx: &mut BuildContext) -> Self {
        let state = self.clone_state();
        self.name("GameView")
            .selector(Selector::default().id("game_view"))
            .child(
                Grid::create()
                    .child(
                        MapView::create()
                            .selector(Selector::default().id("map_view"))
                            .visibility(Visibility::from("collapsed"))
                            .build(ctx),
                    )
                    .child(
                        MenuView::create()
                            .selector(Selector::default().id("menu_view"))
                            .build(ctx),
                    )
                    .build(ctx),
            )
            .on_game_event(move |e| {
                state.game_event(e);
                true
            })
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
