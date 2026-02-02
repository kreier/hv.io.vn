fn main() {
    let width = 80;
    let height = 40;
    let max_iter = 100;

    for y in 0..height {
        for x in 0..width {
            // Map pixel coordinates to the complex plane
            // Real part (cx) and Imaginary part (cy)
            let cx = (x as f64 / width as f64) * 3.5 - 2.5;
            let cy = (y as f64 / height as f64) * 2.0 - 1.0;

            let mut zx = 0.0;
            let mut zy = 0.0;
            let mut i = 0;

            // Iteration loop: z = z^2 + c
            while zx * zx + zy * zy <= 4.0 && i < max_iter {
                let next_zx = zx * zx - zy * zy + cx;
                zy = 2.0 * zx * zy + cy;
                zx = next_zx;
                i += 1;
            }

            // Print character based on whether the point escaped
            if i == max_iter {
                print!("#");
            } else if i > 10 {
                print!("*");
            } else if i > 5 {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!(); // New line after each row
    }
}
