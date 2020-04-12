fn spawn() {
    return #{
    };
}

fn on_kb(self, key, is_down) {
    if key == "Escape" {
        game.camera_smoothstep_lookat(vec3(0.0, 1.0, -2.6), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0), 0.7);
        game.set_active_space("mainmenu");
    }
    return self;
}