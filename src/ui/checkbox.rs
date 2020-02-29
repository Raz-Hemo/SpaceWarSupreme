pub struct Checkbox {
    is_checked: bool,
}

impl super::UIElement for Checkbox {
    fn get_verts(&self) -> Vec<super::UIVertex> {
        vec![
            super::UIVertex {position: [0.0, 0.0]},
            super::UIVertex {position: [1.0, 0.0]},
            super::UIVertex {position: [1.0, 1.0]},
            super::UIVertex {position: [0.0, 0.0]},
            super::UIVertex {position: [1.0, 1.0]},
            super::UIVertex {position: [0.0, 1.0]},
        ]
    }

    fn on_click(&mut self) {
        self.is_checked = !self.is_checked;
    }
}
