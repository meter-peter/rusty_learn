enum HaarFeatureType {
    VerticalEdge,
    HorizontalEdge,
    Diagonal,
}

fn haar_feature(integral_image: &[Vec<u32>], feature_type: HaarFeatureType, x: usize, y: usize, width: usize, height: usize) -> i32 {
    let get_pixel = |x: isize, y: isize| -> u32 {
        if x < 0 || y < 0 {
            0
        } else {
            integral_image[y as usize][x as usize]
        }
    };

    let (half_width, half_height) = (width / 2, height / 2);
    match feature_type {
        HaarFeatureType::VerticalEdge => {
            let white_area = get_pixel(x as isize - 1, y as isize + height as isize - 1)
                - get_pixel(x as isize - 1, y as isize - 1)
                - get_pixel(x as isize + half_width as isize - 1, y as isize + height as isize - 1)
                + get_pixel(x as isize + half_width as isize - 1, y as isize - 1);

            let black_area = get_pixel(x as isize + width as isize - 1, y as isize + height as isize - 1)
                - get_pixel(x as isize + width as isize - 1, y as isize - 1)
                - get_pixel(x as isize + half_width as isize - 1, y as isize + height as isize - 1)
                + get_pixel(x as isize + half_width as isize - 1, y as isize - 1);

            black_area as i32 - white_area as i32
        },
        HaarFeatureType::HorizontalEdge => {
            let white_area = get_pixel(x as isize + width as isize - 1, y as isize - 1)
                - get_pixel(x as isize - 1, y as isize - 1)
                - get_pixel(x as isize + width as isize - 1, y as isize + half_height as isize - 1)
                + get_pixel(x as isize - 1, y as isize + half_height as isize - 1);

            let black_area = get_pixel(x as isize + width as isize - 1, y as isize + height as isize - 1)
                - get_pixel(x as isize - 1, y as isize + height as isize - 1)
                - get_pixel(x as isize + width as isize - 1, y as isize + half_height as isize - 1)
                + get_pixel(x as isize - 1, y as isize + half_height as isize - 1);

            black_area as i32 - white_area as i32
        },
        HaarFeatureType::Diagonal => {
            let black_area = get_pixel(x as isize + half_width as isize - 1, y as isize + half_height as isize - 1)
                - get_pixel(x as isize - 1, y as isize + half_height as isize - 1)
                - get_pixel(x as isize + half_width as isize - 1, y as isize - 1)
                + get_pixel(x as isize - 1, y as isize - 1);

            let white_area = get_pixel(x as isize + width as isize - 1, y as isize + height as isize - 1)
            - get_pixel(x as isize + width as isize - 1, y as isize + half_height as isize - 1)
            - get_pixel(x as isize + half_width as isize - 1, y as isize + height as isize - 1)
            + get_pixel(x as isize + half_width as isize - 1, y as isize + half_height as isize - 1);

        let white_area2 = get_pixel(x as isize + half_width as isize - 1, y as isize - 1)
            - get_pixel(x as isize - 1, y as isize - 1)
            - get_pixel(x as isize + half_width as isize - 1, y as isize + half_height as isize - 1)
            + get_pixel(x as isize - 1, y as isize + half_height as isize - 1);

        let black_area2 = get_pixel(x as isize + width as isize - 1, y as isize + half_height as isize - 1)
            - get_pixel(x as isize + half_width as isize - 1, y as isize + half_height as isize - 1)
            - get_pixel(x as isize + width as isize - 1, y as isize - 1)
            + get_pixel(x as isize + half_width as isize - 1, y as isize - 1);

        (black_area + white_area) as i32 - (black_area2 + white_area2) as i32
    },
}

}

fn compute_all_haar_features(integral_image: &[Vec<u32>], feature_types: &[HaarFeatureType]) -> Vec<Vec<Vec<i32>>> {
let rows = integral_image.len();
let cols = integral_image[0].len();

rust

let mut features = vec![vec![vec![0; cols]; rows]; feature_types.len()];

for (feature_index, &feature_type) in feature_types.iter().enumerate() {
    for y in 0..rows {
        for x in 0..cols {
            let mut max_feature_value = std::i32::MIN;
            for height in 1..=rows - y {
                for width in 1..=cols - x {
                    let feature_value = haar_feature(integral_image, feature_type, x, y, width, height);
                    max_feature_value = max_feature_value.max(feature_value);
                }
            }
            features[feature_index][y][x] = max_feature_value;
        }
    }
}

features

}