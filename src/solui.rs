#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

use tetra::graphics::mesh::{GeometryBuilder, Mesh, ShapeStyle};
use tetra::graphics::{self, Color, Rectangle, Texture};
use tetra::input::{self, Key, MouseButton};
use tetra::math::Vec2;
use tetra::window;
use tetra::{Context, ContextBuilder, State};

struct Coords {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

pub struct Element {
    pos: Vec2<f32>,
    mesh: Mesh,
    hidden: bool,
}

impl Element {
    fn new(ctx: &mut Context, coords: Coords) -> Element {
        let rect = Rectangle {x: coords.x, y: coords.y, width: coords.w, height: coords.h};
        Element {
            pos: Vec2::new(coords.x, coords.y),
            mesh: Mesh::rectangle(ctx, ShapeStyle::Fill, rect).expect("REASON"),
            hidden: false,
        }
    }

    fn draw(&self, ctx: &mut Context) {
        self.mesh.draw(ctx, self.pos);
    }
}

struct GameState {
    mouse: Element,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameState {
        let coords = Coords {x: -10.0, y: -10.0, w: 20.0, h: 20.0};
        GameState {
            mouse: Element::new(ctx, coords),
        }
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<()> {
        self.mouse.pos = input::get_mouse_position(ctx).round();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result<()> {
        graphics::clear(ctx, Color::rgb(0.769, 0.812, 0.631));
        self.mouse.draw(ctx);
        graphics::flush(ctx);
        Ok(())
    }
}

pub fn start() -> tetra::Result<()> {
    ContextBuilder::new("Warehouse Manager", 1280, 720)
        .quit_on_escape(true)
        .build()?
        .run(|ctx| Ok(GameState::new(ctx)))
}