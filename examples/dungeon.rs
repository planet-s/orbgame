use orbgame::{
    prelude::*,
    theme::{COLORS_RON, DARK_THEME_RON, FONTS_RON},
    theming::config::ThemeConfig,
};
use std::{cell::RefCell, rc::Rc};

static DUNGEON_EXT: &'static str = include_str!("../res/dungeon/dungeon_theme.ron");

fn theme() -> Theme {
    Theme::from_config(
        ThemeConfig::from(DARK_THEME_RON)
            .extend(ThemeConfig::from(DUNGEON_EXT))
            .extend(ThemeConfig::from(COLORS_RON))
            .extend(ThemeConfig::from(FONTS_RON)),
    )
}

#[derive(Copy, Clone)]
pub enum MapViewAction {
    OpenMenu,
}

#[derive(AsAny, Default, Clone)]
pub struct MapViewState {
    action: Option<MapViewAction>,
}

impl MapViewState {
    fn action(&mut self, action: MapViewAction) {
        self.action = Some(action);
    }
}

impl State for MapViewState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if let Some(action) = self.action {
            match action {
                MapViewAction::OpenMenu => {
                    ctx.push_event_by_window(GameEvent::OpenMenu);
                }
            }

            self.action = None;
        }
    }
}

widget!(MapView<MapViewState> : KeyDownHandler);

impl Template for MapView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MapView")
            .child(
                Container::new()
                    .style("container")
                    .child(
                        Grid::new()
                            .child(
                                TileMap::new()
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
                                TextBlock::new()
                                    .text("Press ESC to open menu")
                                    .v_align("bottom")
                                    .margin(4.0)
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .build(ctx),
            )
            .on_key_down(move |states, event| -> bool {
                if event.key == Key::Escape {
                    states
                        .get_mut::<MapViewState>(id)
                        .action(MapViewAction::OpenMenu);
                }
                true
            })
    }
}

#[derive(AsAny, Copy, Clone)]
pub enum MenuAction {
    Start,
    Quit,
}

#[derive(AsAny, Default)]
pub struct MenuViewState {
    action: Option<MenuAction>,
}

impl MenuViewState {
    fn action(&mut self, action: MenuAction) {
        self.action = Some(action);
    }
}

impl State for MenuViewState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if let Some(action) = self.action {
            match action {
                MenuAction::Start => {
                    ctx.push_event_by_window(GameEvent::StartGame);
                }
                MenuAction::Quit => {
                    ctx.push_event_by_window(GameEvent::Quit);
                }
            }

            self.action = None;
        }
    }
}

widget!(
    MenuView<MenuViewState> {}
);

impl Template for MenuView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MenuView").child(
            Grid::new()
                .style("start")
                .child(
                    Container::new()
                        .padding(16.0)
                        .min_width(120.0)
                        .style("menu")
                        .v_align("center")
                        .h_align("center")
                        .child(
                            Stack::new()
                                .child(
                                    TextBlock::new()
                                        .style("header")
                                        .text("Dungeon")
                                        .h_align("Center")
                                        .build(ctx),
                                )
                                .child(
                                    Button::new()
                                        .style("button_single_content")
                                        .margin((0.0, 16.0, 0.0, 0.0))
                                        .text("Start Game")
                                        .on_click(move |states, _| {
                                            states
                                                .get_mut::<MenuViewState>(id)
                                                .action(MenuAction::Start);
                                            true
                                        })
                                        .build(ctx),
                                )
                                .child(
                                    Button::new()
                                        .style("button_single_content")
                                        .margin((0.0, 8.0, 0.0, 0.0))
                                        .text("Quit")
                                        .on_click(move |states, _| {
                                            states
                                                .get_mut::<MenuViewState>(id)
                                                .action(MenuAction::Quit);
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

pub type GameHandlerFn = dyn Fn(&mut StatesContext, &GameEvent) -> bool + 'static;

pub struct GameEventHandler {
    handler: Rc<GameHandlerFn>,
}

impl Into<Rc<dyn EventHandler>> for GameEventHandler {
    fn into(self) -> Rc<dyn EventHandler> {
        Rc::new(self)
    }
}

impl EventHandler for GameEventHandler {
    fn handle_event(&self, states: &mut StatesContext, event: &EventBox) -> bool {
        if let Ok(event) = event.downcast_ref::<GameEvent>() {
            return (self.handler)(states, event);
        }

        return false;
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<GameEvent>()
    }
}

impl Event for GameEvent {}

#[derive(AsAny, Default, Clone)]
pub struct GameViewState {
    event: Option<GameEvent>,
    map_view: Entity,
    menu_view: Entity,
}

impl GameViewState {
    fn game_event(&mut self, event: &GameEvent) {
        self.event = Some(*event);
    }
}

impl State for GameViewState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.map_view = ctx
            .entity_of_child("map_view")
            .expect("GameViewState.init: map_view child could not be found.");
        self.menu_view = ctx
            .entity_of_child("menu_view")
            .expect("GameViewState.init: menu_view box could not be found.");
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if let Some(event) = self.event {
            match event {
                GameEvent::OpenMenu => {
                    ctx.get_widget(self.map_view)
                        .set("visibility", Visibility::Collapsed);
                    ctx.get_widget(self.menu_view)
                        .set("visibility", Visibility::Visible);
                }
                GameEvent::StartGame => {
                    ctx.get_widget(self.menu_view)
                        .set("visibility", Visibility::Collapsed);
                    ctx.get_widget(self.map_view)
                        .set("visibility", Visibility::Visible);
                }
                GameEvent::Quit => {
                    ctx.push_event(SystemEvent::Quit);
                }
            }

            self.event = None;
        }
    }
}

widget!(GameView<GameViewState>);

impl GameView {
    fn on_game_event<H: Fn(&mut StatesContext, &GameEvent) -> bool + 'static>(
        self,
        handler: H,
    ) -> Self {
        self.insert_handler(GameEventHandler {
            handler: Rc::new(handler),
        })
    }
}

impl Template for GameView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("GameView")
            .id("game_view")
            .child(
                Grid::new()
                    .child(
                        MapView::new()
                            .id("map_view")
                            .visibility("collapsed")
                            .build(ctx),
                    )
                    .child(MenuView::new().id("menu_view").build(ctx))
                    .build(ctx),
            )
            .on_game_event(move |states, e| {
                states.get_mut::<GameViewState>(id).game_event(e);
                true
            })
    }
}

fn main() {
    Game::new()
        .theme(theme())
        .window(|ctx| {
            Window::new()
                .title("OrbGame - dungeon example")
                .position((100.0, 100.0))
                .size(800.0, 600.0)
                .child(GameView::new().build(ctx))
                .build(ctx)
        })
        .run();
}
