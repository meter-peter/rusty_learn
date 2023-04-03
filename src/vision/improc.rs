fn integral_image(image: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let rows = image.len();
    let cols = image[0].len();
    let mut integral = vec![vec![0; cols]; rows];

    for y in 0..rows {
        for x in 0..cols {
            let top = if y > 0 { integral[y - 1][x] } else { 0 };
            let left = if x > 0 { integral[y][x - 1] } else { 0 };
            let top_left = if x > 0 && y > 0 { integral[y - 1][x - 1] } else { 0 };
            integral[y][x] = image[y][x] + top + left - top_left;
        }
    }

    integral
}

