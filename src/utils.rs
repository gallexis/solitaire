pub fn coordinates_by_index(i: i16, width: i16, height: i16) -> (i16, i16) {
    let x = i % width;
    let y = i / height;
    (x, y)
}

pub fn index_by_coordinates(x: i16, y: i16, width: i16) -> usize {
    (width * y + x) as usize
}

pub fn coordinates_within_boundaries(x: i16, y: i16, width: i16, height: i16) -> bool {
    x >= 0 && x < width && y >= 0 && y < height
}

/*
pub fn index_within_boundaries(i: i16, width: i16, height: i16) -> bool {
    let (x, y) = coordinates_by_index(i, width, height);

    coordinates_within_boundaries(x, y, width, height)
}
*/