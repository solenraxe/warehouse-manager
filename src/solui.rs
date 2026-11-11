#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use tetra::graphics::mesh::{GeometryBuilder, Mesh, ShapeStyle, BorderRadii};
use tetra::graphics::{self, Color, Rectangle, Texture, DrawParams};
use tetra::input::{self, Key, MouseButton};
use tetra::math::Vec2;
use tetra::window;
use tetra::{Context, ContextBuilder, State};

#[derive(Clone)]
pub struct Coords {x: f32, y: f32, w: f32, h: f32,}
impl Coords {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Coords { Coords {x: x, y: y, w: w, h: h} }
}

#[derive(Clone)]
struct Element {
    pos: Vec2<f32>,
    mesh: Mesh,
    rect: Rectangle,
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
            rect: rect,
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
    ($ctx:expr, $coords:expr $(, $key:ident = $val:expr)*) => {{
        let mut hollow: Option<f32> = None;
        let mut radius: Option<f32> = None;
        let mut color = Color::rgb8(255, 255, 255);
        let mut hid = false;
        $(
            match $key {
                hollow => hollow = Some($val),
                radius => radius = Some($val),
                color => color = $val,
                hidden => hid = $val,
                _ => compile_error!("oopsie"),
            }
        )*
        Element::new($ctx, $coords, hollow, radius, color, hid)
    }};
}

trait HasElement {
    fn element(&self) -> &Element;
}
trait Drawable: HasElement {
    fn draw(&self, ctx: &mut Context) {
        self.element().draw(ctx);
    }
}
trait Interactable: HasElement {
    fn checkClick(&self, ctx: &mut Context) -> bool {
        let m = input::get_mouse_position(ctx).round();
        let r = self.element().rect;
        if (r.x <= m.x) && (r.x+r.width >= m.x) && (r.y <= m.y) && (r.y+r.height >= m.y) {
            return true;
        }
        false
    }
}
trait ButtonLike: HasElement {
    fn func(&self) -> fn(&mut Context);
    fn run(&self, ctx: &mut Context) {
        (self.func())(ctx);
    }
}

#[derive(Clone)]
struct Rect {
    base: Element,
}
impl Rect { fn new(ctx: &mut Context, crds: Coords) -> Rect {Rect {base: newElement!(ctx, crds)}} }
impl HasElement for Rect { fn element(&self) -> &Element { &self.base }}
impl Drawable for Rect {}

#[derive(Clone)]
struct Button {
    base: Element,
    func: fn(&mut Context),
}
impl HasElement for Button { fn element(&self) -> &Element { &self.base }}
impl Drawable for Button {}
impl Interactable for Button {}
impl ButtonLike for Button { fn func(&self) -> fn(&mut Context) { self.func }}

#[derive(Clone)]
enum AnyElement {
    Rect(Rect),
    Button(Button),
}
impl HasElement for AnyElement {
    fn element(&self) -> &Element {
        match self {
            AnyElement::Rect(r) => &r.base,
            AnyElement::Button(b) => &b.base,
        }
    }
}
impl Drawable for AnyElement {
    fn draw(&self, ctx: &mut Context) {
        match self {
            AnyElement::Rect(r) => r.draw(ctx),
            AnyElement::Button(b) => b.draw(ctx),
        }
    }
}

#[derive(Clone)]
pub struct GameState {
    elements: Vec<AnyElement>,
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
            self.elements.push(AnyElement::Rect(Rect::new(ctx, coords)));
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