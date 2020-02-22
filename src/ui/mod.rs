#[derive(Default, Debug, Clone)]
struct UIVertex {
    position: [f32; 2],
}
vulkano::impl_vertex!(UIVertex, position);

pub trait UIElement {
    // UI logic is decoupled from rendering. All it needs to know is how to export
    // itself as vertices.
    fn get_verts(&self) -> Vec<UIVertex>;

    fn on_click(&mut self);
}

pub struct UIElementCore<T: UIElement> {
    pos: (f32, f32),
    size: (f32, f32),
    start_hover: Option<std::time::Instant>,
    start_click: Option<std::time::Instant>,
    element: T,
}

impl<T: UIElement> UIElementCore<T> {
    pub fn is_inside(&self, pos: (f32, f32)) -> bool {
        pos.0 > self.pos.0 && pos.0 < self.pos.0 + self.size.0 &&
        pos.1 > self.pos.1 && pos.1 < self.pos.1 + self.size.1
    }
    fn mouse_move(&mut self, new_pos: (f32, f32)) {        
        if self.is_inside(new_pos) && self.start_hover.is_none() {
            self.start_hover = Some(std::time::Instant::now());
        }
        if !self.is_inside(new_pos) && self.start_hover.is_some() {
            self.start_hover = None;
            self.start_click = None;
        }
    }
    fn mouse_down(&mut self) {
        self.start_click = Some(std::time::Instant::now());
    }
    fn mouse_up(&mut self) {
        if self.start_click.is_some() {
            self.element.on_click();
        }
        self.start_click = None;
    }
}

mod checkbox;
pub use checkbox::Checkbox;
