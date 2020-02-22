pub struct Checkbox {
    is_checked: bool,
}

impl super::UIElement for Checkbox {
    fn get_verts(&self) -> Vec<super::UIVertex> {
        Vec::new()
    }

    fn on_click(&mut self) {
        self.is_checked = !self.is_checked;
    }
}
