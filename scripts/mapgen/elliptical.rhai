// This creates an empty vector for you to work with.
let stars = make_star_vector();

// Generate the stars
let i = 500;
while i > 0 {
    // TODO generate random angle and a radius between a min and max (user configurable)
    stars.push(make_random_star());
    i -= 1;
}

// Return your star vector
stars