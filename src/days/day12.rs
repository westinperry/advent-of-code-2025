use std::{collections::HashSet, fs};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Shape {
    cells: Vec<(i32, i32)>,
    width: i32,
    height: i32
}

#[derive(Clone)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<bool>
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            height,
            cells: vec![false; width * height],
        }
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn is_free(&self, x: usize, y: usize) -> bool {
        !self.cells[self.idx(x, y)]
    }

    fn set(&mut self, x: usize, y: usize, value: bool) {
        let i = self.idx(x, y);
        self.cells[i] = value;
    }

    fn free_count(&self) -> usize {
        self.cells.iter().filter(|&&c| !c).count()
    }
}


pub fn day_12() {
    let input = fs::read_to_string("./inputs/day_12_input.txt").expect("Error");

    // Split input into shape section and region section
    let mut shape_section = String::new();
    let mut region_section = String::new();
    let mut in_regions = false;

    for line in input.lines() {
        // Region lines start with digit(s), then 'x', then digit(s), then ':'
        // Shape index lines start with just a digit and ':'
        // We detect regions by checking for the 'x' pattern like "4x4:" or "12x5:"
        if !in_regions {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                // Check if this looks like a region definition (e.g., "4x4:" or "12x5:")
                if let Some(colon_pos) = trimmed.find(':') {
                    let before_colon = &trimmed[..colon_pos];
                    if before_colon.contains('x') {
                        // This is a region line
                        in_regions = true;
                    }
                }
            }
        }

        if in_regions {
            region_section.push_str(line);
            region_section.push('\n');
        } else {
            shape_section.push_str(line);
            shape_section.push('\n');
        }
    }

    let shapes = parse_shapes(&shape_section);
    let mut all_orientations = Vec::new();
    for shape in &shapes {
        all_orientations.push(generate_orientations(shape));
    }

    let mut count = 0;

    for line in region_section.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let mut parts = line.split(':');
        let size = parts.next().unwrap().trim();
        let counts: Vec<usize> = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut size_parts = size.split('x');
        let width: usize = size_parts.next().unwrap().parse().unwrap();
        let height: usize = size_parts.next().unwrap().parse().unwrap();

        let mut shapes_to_place = Vec::new();
        let mut total_cells = 0;

        for (i, &count_i) in counts.iter().enumerate() {
            for _ in 0..count_i {
                shapes_to_place.push(all_orientations[i].clone());
                total_cells += all_orientations[i][0].cells.len();
            }
        }

        // Sort largest shapes first for better pruning
        shapes_to_place.sort_by_key(|s| -(s[0].cells.len() as i32));

        let mut grid = Grid::new(width, height);

        if solve(&mut grid, &shapes_to_place, 0, total_cells) {
            count += 1;
        }
    }

    println!("Regions that fit all presents: {}", count);
}

fn parse_shapes(input: &str) -> Vec<Shape> {
    let mut shapes = Vec::new();

    for block in input.split("\n\n") {
        if !block.contains(':') {
            continue;
        }

        let mut lines = block.lines();
        lines.next();

        let mut cells = Vec::new();
        for (y, line) in lines.enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    cells.push((x as i32, y as i32));
                }
            }
        }

        if cells.is_empty() {
            continue;
        }

        let norm = normalize(&cells);
        shapes.push(shape_from_cells(norm));

    }

    shapes
}

fn normalize(cells: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let min_x = cells.iter().map(|(x, _)| *x).min().unwrap();
    let min_y = cells.iter().map(|(_, y)| *y).min().unwrap();

    let mut norm: Vec<(i32, i32)> = cells
        .iter()
        .map(|(x, y)| (x - min_x, y - min_y))
        .collect();

    norm.sort();
    norm
}

fn shape_from_cells(cells: Vec<(i32, i32)>) -> Shape {
    let max_x = cells.iter().map(|(x, _)| *x).max().unwrap();
    let max_y = cells.iter().map(|(_, y)| *y).max().unwrap();


    Shape {
        cells,
        width: max_x + 1,
        height: max_y + 1,
    }
}

fn rotate_90(cells: &Vec<(i32, i32)>, height: i32) -> Vec<(i32, i32)> {
    cells.iter().map(|(x, y)| (height - 1 - y, *x)).collect()
}

fn flip_horizontal(cells: &Vec<(i32, i32)>, width: i32) -> Vec<(i32, i32)> {
    cells.iter().map(|(x, y)| (width - 1 - x, *y)).collect()
}

fn generate_orientations(shape: &Shape) -> Vec<Shape> {
    let mut seen = HashSet::new();
    let mut results = Vec::new();

    let mut cells = shape.cells.clone();
    let mut w = shape.width;
    let mut h = shape.height;

    for _ in 0..4 {
        // no flip
        let norm = normalize(&cells);
        if seen.insert(norm.clone()) {
            results.push(shape_from_cells(norm.clone()));
        }

        // flipped
        let flipped = flip_horizontal(&cells, w);
        let norm_flip = normalize(&flipped);
        if seen.insert(norm_flip.clone()) {
            results.push(shape_from_cells(norm_flip.clone()));
        }

        // rotate
        let rotated = rotate_90(&cells,  h);
        cells = rotated;
        std::mem::swap(&mut w, &mut h);
    }

    results
}

fn can_place(grid: &Grid, shape: &Shape, ox: i32, oy: i32) -> bool {
    for (dx, dy) in &shape.cells {
        let x = ox + dx;
        let y = oy + dy;

        if x < 0 || y < 0 {
            return false;
        }

        let ux = x as usize;
        let uy = y as usize;

        if ux >= grid.width || uy >= grid.height {
            return false;
        }

        if !grid.is_free(ux, uy) {
            return false;
        }
    }
    true
}

fn place(grid: &mut Grid, shape: &Shape, ox: i32, oy: i32, value: bool) {
    for (dx, dy) in &shape.cells {
        let x = (ox + dx) as usize;
        let y = (oy + dy) as usize;
        grid.set(x, y, value);
    }
}

fn solve(
    grid: &mut Grid,
    shapes: &Vec<Vec<Shape>>,
    index: usize,
    remaining_cells: usize,
) -> bool {
    if index == shapes.len() {
        return true;
    }

    if grid.free_count() < remaining_cells {
        return false;
    }

    for shape in &shapes[index] {
        for y in 0..=(grid.height as i32 - shape.height) {
            for x in 0..=(grid.width as i32 - shape.width) {
                if can_place(grid, shape, x, y) {
                    place(grid, shape, x, y, true);

                    if solve(
                        grid,
                        shapes,
                        index + 1,
                        remaining_cells - shape.cells.len(),
                    ) {
                        return true;
                    }

                    place(grid, shape, x, y, false);
                }
            }
        }
    }

    false
}

