#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused)]

use tetra::graphics::text::{Font, Text};
use tetra::graphics::mesh::{GeometryBuilder, Mesh, ShapeStyle, BorderRadii};
use tetra::graphics::{self, Color, Rectangle, Texture, DrawParams};
use tetra::input::{self, Key, MouseButton};
use tetra::math::Vec2;
use tetra::window;
use tetra::{Context, ContextBuilder, State};

fn noFunctionWarning(ctx: &mut Context) {
    println!("No function was set for this button!");
}

#[derive(Clone)]
pub struct Coords {x: f32, y: f32, w: f32, h: f32,}
impl Coords {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Coords { Coords {x: x, y: y, w: w, h: h} }
}

pub enum ItemTypes {
    Rect,
    Button,
    Text,
}
struct ElementConstr {
    coords: Coords,
    color: Color,
    radius: Option<f32>,
    hollow: Option<f32>,
    itemType: ItemTypes,
    hidden: bool,
    text: Option<String>,
    func: Option<fn(&mut Context)>,
}

struct Element {
    pos: Vec2<f32>,
    mesh: Mesh,
    rect: Rectangle,
    color: Color,
    hidden: bool,
    clickable: bool,
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
            clickable: false,
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
    fn draw(&mut self, ctx: &mut Context) {
        if self.element().hidden {return;}
        self.element().draw(ctx);
    }
    fn checkClick(&self, ctx: &mut Context) -> bool {
        if !self.element().clickable {return false;}
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

struct TextElement {
    base: Element,
    text: Text,
    textPos: Vec2<f32>,
}
impl TextElement {
    fn new(ctx: &mut Context, coords: Coords, hid: bool, text: String) -> TextElement {
        let e = Element::new(ctx, coords, None, None, Color::rgb8(255, 255, 255), hid);
        let mut t = Text::new(text, Font::vector(ctx, "./resources/ComicSans.ttf", 24.0).expect("REASON"));
        let b = t.get_bounds(ctx).unwrap();
        let textPos = Vec2::new(e.rect.x-b.width/2.0, e.rect.y-b.height/2.0);
        TextElement {
            base: e,
            text: t,
            textPos: textPos,
        }
    }
}
impl UiElement for TextElement { 
    fn element(&self) -> &Element { &self.base }
    fn draw(&mut self, ctx: &mut Context) {
        self.text.draw(ctx, self.textPos);
    }
 }

struct Button {
    base: Element,
    text: TextElement,
    func: fn(&mut Context),
}
impl Button {
    fn new(ctx: &mut Context, coords: Coords, hid: bool, text: String, func: Option<fn(&mut Context)>) -> Button {
        let mut e = Element::new(ctx, coords.clone(), Some(2.0), Some(5.0), Color::rgb8(255, 255, 255), hid);
        e.clickable = true;
        let tPos = Coords {x: coords.x+coords.w/2.0, y: coords.y+coords.h/2.0, w: coords.w, h: coords.h};
        let t = TextElement::new(ctx, tPos, hid, text);
        Button {
            base: e,
            text: t,
            func: func.unwrap_or(noFunctionWarning),
        }
    }
}
impl UiElement for Button { 
    fn element(&self) -> &Element { &self.base }
    fn draw(&mut self, ctx: &mut Context) {
        self.element().draw(ctx);
        self.text.draw(ctx);
    }
    fn runFunc(&self, ctx: &mut Context) { (&self.func)(ctx); }
}

pub struct GameState {
    elements: Vec<Box<dyn UiElement>>,
    pending: Vec<ElementConstr>,
    title: String,
    size: Vec<i32>,
}
impl GameState {
    pub fn new() -> GameState {
        GameState {
            elements: Vec::new(),
            pending: Vec::new(),
            title: String::from("My Solui Window"),
            size: vec![800, 600],
        }
    }
    pub fn setTitle(mut self, title: String) -> GameState {
        self.title = title;
        self
    }
    pub fn setSize(mut self, width: i32, height: i32) -> GameState {
        self.size = vec![width, height];
        self
    }

    pub fn requestAdd(&mut self, crds: Vec<f32>, hollow: Option<f32>, radius: Option<f32>, color: Vec<u8>, hid: bool, iType: ItemTypes, text: Option<String>, func: Option<fn(&mut Context)>) {
        let constructor = ElementConstr {
            coords: Coords{x: crds[0], y: crds[1], w: crds[2], h: crds[3]},
            color: Color::rgb8(color[0], color[1], color[2]),
            hollow: hollow,
            radius: radius,
            hidden: hid,
            itemType: iType,
            func: func,
            text: text,
        };
        self.pending.push(constructor);
    }

    fn addElement(&mut self, ctx: &mut Context) {
        for constr in self.pending.drain(..) {
            self.elements.push(
                match constr.itemType {
                    ItemTypes::Rect => Box::new(Rect::new(ctx, constr.coords, constr.hollow, constr.radius, constr.color, constr.hidden)),
                    ItemTypes::Button => Box::new(Button::new(ctx, constr.coords, constr.hidden, constr.text.unwrap_or(String::from("")), constr.func)),
                    ItemTypes::Text => Box::new(TextElement::new(ctx, constr.coords, constr.hidden, constr.text.unwrap_or(String::from("")))),
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
                    elem.runFunc(ctx);
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result<()> {
        graphics::clear(ctx, Color::rgb8(0, 0, 0));
        for elem in &mut self.elements {
            elem.draw(ctx);
        }
        graphics::flush(ctx);
        Ok(())
    }
}

macro_rules! setField {
    ($hollow:ident, $radius:ident, $color:ident, $hid:ident, $iType:ident, $text: ident, $func: ident; hollow = $v:expr) => {
        $hollow = Some($v);
    };
    ($hollow:ident, $radius:ident, $color:ident, $hid:ident, $iType:ident, $text: ident, $func: ident; radius = $v:expr) => {
        $radius = Some($v);
    };
    ($hollow:ident, $radius:ident, $color:ident, $hid:ident, $iType:ident, $text: ident, $func: ident; color = $v:expr) => {
        $color = $v;
    };
    ($hollow:ident, $radius:ident, $color:ident, $hid:ident, $iType:ident, $text: ident, $func: ident; hidden = $v:expr) => {
        $hid = $v;
    };
    ($hollow:ident, $radius:ident, $color:ident, $hid:ident, $iType:ident, $text: ident, $func: ident; itemType = $v:expr) => {
        $iType = $v;
    };
    ($hollow:ident, $radius:ident, $color:ident, $hid:ident, $iType:ident, $text: ident, $func: ident; text = $v:expr) => {
        $text = Some($v);
    };
    ($hollow:ident, $radius:ident, $color:ident, $hid:ident, $iType:ident, $text: ident, $func: ident; func = $v:expr) => {
        $func = Some($v);
    };
}
pub(crate) use setField;

macro_rules! newElement {
    ($gState:expr, $coords:expr $(, $key:ident = $val:expr)*) => {{
        let mut hollow: Option<f32> = None;
        let mut radius: Option<f32> = None;
        let mut text: Option<String> = None;
        let mut function: Option<fn(&mut tetra::Context)> = None;
        let mut color = vec![255, 255, 255];
        let mut hid = false;
        let mut iType = solui::ItemTypes::Rect;

        $(
            solui::setField!(hollow, radius, color, hid, iType, text, function; $key = $val);
        )*

        $gState.requestAdd($coords, hollow, radius, color, hid, iType, text, function)
    }};
}
pub(crate) use newElement;

pub fn start(gState: GameState) -> tetra::Result {
    let mut binding = ContextBuilder::new(&gState.title, gState.size[0], gState.size[1]);
    let ctxBuild = binding.quit_on_escape(true).show_mouse(true);
    let mut ctx = ctxBuild.build().expect("REASON");
    ctx.run(|_| Ok(gState))
}