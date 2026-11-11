#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused)]

use tetra::graphics::mesh::{GeometryBuilder, Mesh, ShapeStyle, BorderRadii};
use tetra::graphics::{self, Color, Rectangle, Texture, DrawParams};
use tetra::input::{self, Key, MouseButton};
use tetra::math::Vec2;
use tetra::window;
use tetra::{Context, ContextBuilder, State};

pub struct Coords {x: f32, y: f32, w: f32, h: f32,}
impl Coords {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Coords { Coords {x: x, y: y, w: w, h: h} }
}

pub enum ItemTypes {
    Rect,
    Button,
}
struct ElementConstr {
    coords: Coords,
    color: Color,
    radius: Option<f32>,
    hollow: Option<f32>,
    itemType: ItemTypes,
    hidden: bool,
}

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
        params.color = self.color;
        self.mesh.draw(ctx, params);
    }
}

trait UiElement {
    fn element(&self) -> &Element;
    fn draw(&self, ctx: &mut Context) {
        self.element().draw(ctx);
    }
    fn checkClick(&self, ctx: &mut Context) -> bool {
        let m = input::get_mouse_position(ctx).round();
        let r = self.element().rect;
        if (r.x <= m.x) && (r.x+r.width >= m.x) && (r.y <= m.y) && (r.y+r.height >= m.y) {
            return true;
        }
        false
    }
    fn runFunc(&self, _ctx: &mut Context) {}
}

struct Rect {
    base: Element,
}
impl Rect { 
    fn new(ctx: &mut Context, crds: Coords, hollow: Option<f32>, radius: Option<f32>, color: Color, hid: bool) -> Rect {
        Rect {base: Element::new(ctx, crds, hollow, radius, color, hid)}
    } 
}
impl UiElement for Rect { fn element(&self) -> &Element { &self.base }}

struct Button {
    base: Element,
    func: fn(&mut Context),
}
impl UiElement for Button { 
    fn element(&self) -> &Element { &self.base }
    fn runFunc(&self, ctx: &mut Context) { (&self.func)(ctx); }
}

pub struct GameState {
    elements: Vec<Box<dyn UiElement>>,
    pending: Vec<ElementConstr>,
}
impl GameState {
    pub fn new() -> GameState {
        GameState {
            elements: Vec::new(),
            pending: Vec::new(),
        }
    }

    pub fn requestAdd(&mut self, crds: Vec<f32>, hollow: Option<f32>, radius: Option<f32>, color: Vec<u8>, hid: bool, iType: ItemTypes) {
        let constructor = ElementConstr {
            coords: Coords{x: crds[0], y: crds[1], w: crds[2], h: crds[3]},
            color: Color::rgb8(color[0], color[1], color[2]),
            hollow: hollow,
            radius: radius,
            hidden: hid,
            itemType: iType,
        };
        self.pending.push(constructor);
    }

    fn addElement(&mut self, ctx: &mut Context) {
        for constr in self.pending.drain(..) {
            self.elements.push(
                match constr.itemType {
                    ItemTypes::Rect => Box::new(Rect::new(ctx, constr.coords, constr.hollow, constr.radius, constr.color, constr.hidden)),
                    ItemTypes::Button => Box::new(Rect::new(ctx, constr.coords, constr.hollow, constr.radius, constr.color, constr.hidden)),
                    _ => Box::new(Rect::new(ctx, constr.coords, constr.hollow, constr.radius, constr.color, constr.hidden)),
                }
            );
        }
    }
}
impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<()> {
        if !(self.pending.len() == 0) {
            self.addElement(ctx);
        }
        if input::is_mouse_button_pressed(ctx, MouseButton::Left) {
            let m = input::get_mouse_position(ctx);
            for elem in &self.elements {
                if elem.checkClick(ctx) {
                    println!("Works");
                }
            }
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

macro_rules! setField {
    ($hollow:ident, $radius:ident, $color:ident, $hid:ident, $iType:ident; hollow = $v:expr) => {
        $hollow = Some($v);
    };
    ($hollow:ident, $radius:ident, $color:ident, $hid:ident, $iType:ident; radius = $v:expr) => {
        $radius = Some($v);
    };
    ($hollow:ident, $radius:ident, $color:ident, $hid:ident, $iType:ident; color = $v:expr) => {
        $color = $v;
    };
    ($hollow:ident, $radius:ident, $color:ident, $hid:ident, $iType:ident; hidden = $v:expr) => {
        $hid = $v;
    };
    ($hollow:ident, $radius:ident, $color:ident, $hid:ident, $iType:ident; itemType = $v:expr) => {
        $iType = $v;
    };
}
pub(crate) use setField;

macro_rules! newElement {
    ($gState:expr, $coords:expr $(, $key:ident = $val:expr)*) => {{
        let mut hollow: Option<f32> = None;
        let mut radius: Option<f32> = None;
        let mut color = vec![255, 255, 255];
        let mut hid = false;
        let mut iType = solui::ItemTypes::Rect;

        $(
            solui::setField!(hollow, radius, color, hid, iType; $key = $val);
        )*

        $gState.requestAdd($coords, hollow, radius, color, hid, iType)
    }};
}
pub(crate) use newElement;

pub fn start(gState: GameState) -> tetra::Result {
    let mut binding = ContextBuilder::new("Warehouse Manager", 800, 600);
    let ctxBuild = binding.quit_on_escape(true).show_mouse(true);
    let mut ctx = ctxBuild.build().expect("REASON");
    ctx.run(|_| Ok(gState))
}