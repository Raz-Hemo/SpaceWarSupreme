fn spawn() {
    info("spawned menu");
    self = game.new_entity();
    game.subscribe_event("lclick", self.id);
    return self;
}

fn lclick(self) {
    return self;
}