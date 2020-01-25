#[macro_use]
mod log;
mod gameplay;
mod scripting;
mod utils;

fn main()
{
    log::logger().info("Starting Space War Supreme!");
    use gameplay::mapgen::{poisson_distribution, apply_mask};
    println!("{:?}", apply_mask(poisson_distribution(100), "./resources/spiral_mask.png"));
/*
    let mut rng = thread_rng();
    let mut galaxy: Galaxy = Galaxy::new();
    let mut stars: Vec<String> = Vec::new();
    for _ in 0..100
    {
        galaxy.stars.push(MainSequenceStar {
            pos: (rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0)),
            color: (rng.gen_range(0, 255), rng.gen_range(0, 255), rng.gen_range(0, 255)),
            name:,
            radius:,
        });
    }

    let mut args: Vec<String> = vec![
        "C:\\Users\\Raz\\Documents\\source dump\\rust\\visualize_stars.py".to_string(), 
        "-d".to_string(),
    ];

    args.append(&mut stars);
    
    let output = Command::new("python").args(args).output().expect("Failed to execute command");
    println!("{:?}", std::str::from_utf8(&output.stderr.as_slice()));*/
}