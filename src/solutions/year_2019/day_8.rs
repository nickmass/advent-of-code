pub fn part_one(input: &str) -> usize {
    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;
    let layers = input.trim().as_bytes().chunks(WIDTH * HEIGHT);

    let mut result = (std::usize::MAX, 0);
    for layer in layers {
        let mut counts = [0; 3];

        layer
            .iter()
            .map(|p| p - 48)
            .map(usize::from)
            .for_each(|i| counts[i] += 1);

        if counts[0] < result.0 {
            result = (counts[0], counts[1] * counts[2]);
        }
    }

    result.1
}

pub fn part_two(input: &str) -> String {
    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;
    let layers = input.trim().as_bytes().chunks(WIDTH * HEIGHT).rev();

    let mut canvas = vec![false; WIDTH * HEIGHT];

    for layer in layers {
        for (index, pixel) in layer.into_iter().enumerate() {
            canvas[index] = match pixel {
                b'0' => false,
                b'1' => true,
                _ => canvas[index],
            };
        }
    }

    let mut image = String::from("\n");
    for row in canvas.chunks(WIDTH) {
        for &pixel in row {
            image.push_str(if pixel { "##" } else { "  " });
        }
        image.push('\n');
    }

    image
}
