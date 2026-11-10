#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

use tetra::graphics::mesh::{GeometryBuilder, Mesh, ShapeStyle, BorderRadii};
use tetra::graphics::{self, Color, Rectangle, Texture, DrawParams};
use tetra::input::{self, Key, MouseButton};
use tetra::math::Vec2;
use tetra::window;
use tetra::{Context, ContextBuilder, State};

#[derive(Clone)]
pub struct Coords {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Coords {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Coords {
        Coords {
            x: x,
            y: y,
            w: w,
            h: h,
        }
    }
}

#[derive(Clone)]
pub struct Element {
    pos: Vec2<f32>,
    mesh: Mesh,
    color: Color,
    hidden: bool,
}

impl Element {
    fn new(ctx: &mut Context, coords: Coords, hollow: Option<f32>, radius: Option<f32>, color: Color, hid: bool) -> Element {
        let rect = Rectangle {x: coords.x, y: coords.y, width: coords.w, height: coords.h};
        let style = match hollow {
            Some(width) => ShapeStyle::Stroke(width),
            None => ShapeStyle::Fill,
        };
        let mesh = match radius {
            Some(rad) => Mesh::rounded_rectangle(ctx, style, rect, BorderRadii::new(rad)),
            None => Mesh::rectangle(ctx, style, rect),
        };
        Element {
            pos: Vec2::new(coords.x, coords.y),
            mesh: mesh.expect("REASON"),
            color: color,
            hidden: hid,
        }
    }

    fn draw(&self, ctx: &mut Context) {
        if self.hidden {return;}
        let mut params = DrawParams::new();
        params.position = self.pos;
        params.color = self.color;
        self.mesh.draw(ctx, params);
    }
}
macro_rules! newElement {
    ($ctx: expr, $coords: expr) => {Element::new($ctx, $coords, None, None, Color::rgb8(255, 255, 255), false)};
    ($ctx: expr, $coords: expr, $hollow: expr) => {Element::new($ctx, $coords, $hollow, None, Color::rgb8(255, 255, 255), false)};
    ($ctx: expr, $coords: expr, $hollow: expr, $radius: expr) => {Element::new($ctx, $coords, $hollow, $radius, Color::rgb8(255, 255, 255), false)};
    ($ctx: expr, $coords: expr, $hollow: expr, $radius: expr, $color: expr) => {Element::new($ctx, $coords, $hollow, $radius, $color, false)};
    ($ctx: expr, $coords: expr, $hollow: expr, $radius: expr, $color: expr, $hid: expr) => {Element::new($ctx, $coords, $hollow, $radius, $color, $hid)};
}

#[derive(Clone)]
pub struct GameState {
    elements: Vec<Element>,
    pending: Vec<Coords>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            elements: Vec::new(),
            pending: Vec::new(),
        }
    }

    pub fn requestAdd(&mut self, coords: Coords) {
        self.pending.push(coords);
    }

    fn addElement(&mut self, ctx: &mut Context) {
        for coords in self.pending.drain(..) {
            self.elements.push(newElement!(ctx, coords));
        }
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<()> {
        if !(self.pending.len() == 0) {
            self.addElement(ctx);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result<()> {
        graphics::clear(ctx, Color::rgb8(0, 0, 0));
        for elem in &self.elements {
            elem.draw(ctx);
        }
        graphics::flush(ctx);
        Ok(())
    }
}

pub fn start(gState: GameState) -> tetra::Result {
    let mut binding = ContextBuilder::new("Warehouse Manager", 800, 600);
    let ctxBuild = binding.quit_on_escape(true).show_mouse(true);
    let mut ctx = ctxBuild.build().expect("REASON");
    ctx.run(|_| Ok(gState.clone()))
}