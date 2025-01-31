use crate::engine::prelude::*;
use crate::gameplay::types::{StarSystem, Star};
use rhai::{RegisterFn, Scope};
use rand::Rng;

fn distance_sq(p1: &(f64, f64), p2: &(f64, f64)) -> f64 {
    ((p1.0 - p2.0) * (p1.0 - p2.0)) + ((p1.1 - p2.1) * (p1.1 - p2.1))
}

// Returns a set of poisson distributed points in the space (-1 <= x <= 1, -1 <= y <= 1)
pub fn poisson_distribution(num_cells: usize) -> Vec<(f64, f64)> {
    // some useful constants
    use std::f64::consts::PI;
    let cell_size = 2.0f64 / (num_cells as f64);
    let r = 2.0f64.sqrt() * cell_size;

    // Only need to check these nearby cells for collision
    let collision_check_offsets: Vec<(i64, i64)> = vec![
                 (-1, 2), (0, 2), (1, 2),
        (-2, 1), (-1, 1), (0, 1), (1, 1), (2, 1),
        (-2, 0), (-1, 0), (0, 0), (1, 0), (2, 0), 
        (-2, -1), (-1, -1), (0, -1), (1, -1), (2, -1),
                 (-1, -2), (0, -2), (1, -2)
    ];

    let mut samples: Vec<(f64, f64)> = Vec::new();
    let mut rng = rand::thread_rng();

    // The grid can have a single reference to a sample for each cell
    let mut grid: Vec<Vec<Option<usize>>> = vec![vec![None; num_cells]; num_cells];

    // List of points which might still have a valid spot for a neighbor
    let mut active_list: Vec<usize> = Vec::new();

    // Start with a central point
    let mut random_point_pos: (f64, f64) = (1.0, 1.0);
    grid[(random_point_pos.0 / cell_size) as usize]
        [(random_point_pos.1 / cell_size) as usize] = Some(samples.len());
    active_list.push(samples.len());
    samples.push(random_point_pos);

    // 2N-1 (N=cell count) is the amount of iterations needed to fill every cell
    while !active_list.is_empty() {
        let mut should_place_point = false;
        let mut random_point_idx: (usize, usize) = (0, 0);

        // Pick a random existing point i from active list
        let i: usize = rng.gen_range(0..active_list.len());

        // Try to find a valid point around i (30 tries)
        for _ in 0..30 {
            let mut collision_found = false;

            // Generate something in bounds
            let dist: f64 = rng.gen_range(r..2.0 * r);
            let theta: f64 = rng.gen_range(0.0..2.0 * PI);
            random_point_pos = (samples[active_list[i]].0 + dist * theta.cos(), 
                                samples[active_list[i]].1 + dist * theta.sin());
            if random_point_pos.0 < 0.0 || random_point_pos.1 < 0.0 ||
               random_point_pos.0 >= 2.0 || random_point_pos.1 >= 2.0 {
                continue;
            }
            random_point_idx = ((random_point_pos.0 / cell_size) as usize,
                                (random_point_pos.1 / cell_size) as usize);
            
            // Check surroundings
            for &(xoff, yoff) in collision_check_offsets.iter() {
                // Don't try to check out of bounds
                let tested_point_idx = (xoff + random_point_idx.0 as i64, 
                                        yoff + random_point_idx.1 as i64);
                if tested_point_idx.0 < 0 || tested_point_idx.1 < 0 ||
                   tested_point_idx.0 >= num_cells as i64 || tested_point_idx.1 >= num_cells as i64 {
                    continue;
                }

                // If there is a collision, stop iterating
                if let &Some(p) = &grid[tested_point_idx.0 as usize]
                                       [tested_point_idx.1 as usize] {
                    if distance_sq(&samples[p], &random_point_pos) < r * r {
                        collision_found = true;
                        break;
                    }
                }
            }
            if !collision_found {
                should_place_point = true;
                break;
            }
        }

        if should_place_point {
            active_list.push(samples.len());
            grid[random_point_idx.0][random_point_idx.1] = Some(samples.len());
            samples.push(random_point_pos);
        } else {
            // This point has no valid space left. Don't try it anymore.
            active_list.remove(i);
        }
    }

    // shift coords from [0,2] to [-1, 1]
    samples.iter().map(|&(x, y)| (x - 1.0, y - 1.0)).collect()
}


pub fn apply_mask<P: AsRef<std::path::Path>>(points: Vec<(f64, f64)>, mask_path: P) -> anyhow::Result<Vec<(f64, f64)>> {
    use image::GenericImageView;
    let img = utils::load_image(mask_path)?;
    let mut points = points;

    let x_factor = img.dimensions().0 as f64 / 2.0;
    let y_factor = img.dimensions().1 as f64 / 2.0;
    points.retain(|&(x, y)| img.get_pixel(((x + 1.0) * x_factor) as u32, ((y + 1.0) * y_factor) as u32)[0] == 255);
    
    Ok(points)
}

fn generate_star_system(x: f32, y: f32) -> StarSystem {
    StarSystem {
        pos: (x, y),
        name: String::from("test"),
        stars: vec![Star {name: String::from("teststar"), radius:1.0, temperature:5000.0}]
    }
}

pub fn execute_map_generator<P: AsRef<std::path::Path>>(
mapgen_path: P, 
stargen_path: P,
namegen_path: P)
-> anyhow::Result<Vec<StarSystem>> {
    use anyhow::Context;
    let mapgen_script: String = std::fs::read_to_string(mapgen_path.as_ref())
    .context(format!("Failed to open {:?}", mapgen_path.as_ref()))?;
    let _stargen_script: String = std::fs::read_to_string(stargen_path.as_ref())
    .context(format!("Failed to open {:?}", stargen_path.as_ref()))?;
    let namegen_script: String = std::fs::read_to_string(namegen_path.as_ref())
    .context(format!("Failed to open {:?}", namegen_path.as_ref()))?;

    let mut engine = crate::engine::scripting::new_engine();
    let mut scope = Scope::new();
    engine.eval_with_scope::<()>(&mut scope, &namegen_script).context(
        "Namegen script failed"
    )?;

    engine.register_fn("make_star_vector", Vec::new as fn()->Vec<Star>);
    engine.register_fn("push", Vec::push as fn(&mut Vec<Star>, Star));
    engine.register_fn("generate_star_system", generate_star_system);

    engine.eval::<Vec<StarSystem>>(&mapgen_script).context("Mapgen script failed")
}
