fn spawn() {
    game.camera_smoothstep_lookat(vec3(0.214, 2.046, -0.746), vec3(0.0, 0.0, 0.3), vec3(0.0, 1.0, 0.0), 0.0);
    return #{
    };
}

fn on_kb(self, key, is_down) {
    if key == "Escape" {
        game.camera_smoothstep_lookat(vec3(0.214, -2.0, -0.746), vec3(0.0, -5.0, 0.0), vec3(0.0, 1.0, 0.0), 0.7);
        game.set_active_space("galaxymap");
    }
    return self;
}