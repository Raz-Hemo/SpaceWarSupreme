extern crate winres;

fn main() {
    println!("{:?}", std::env::current_dir());
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("./resources/icon.ico");
        res.compile().unwrap();
    }
}