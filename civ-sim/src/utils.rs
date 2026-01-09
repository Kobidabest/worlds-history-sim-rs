use rand::{rngs::SmallRng, Rng};

/// Calculate distance between two points on a wrapped world
pub fn wrapped_distance(
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
    world_width: u32,
) -> f32 {
    let dx = {
        let direct = (x1 as f32 - x2 as f32).abs();
        let wrapped = world_width as f32 - direct;
        direct.min(wrapped)
    };
    let dy = (y1 as f32 - y2 as f32).abs();
    (dx * dx + dy * dy).sqrt()
}

/// Weighted random selection
pub fn weighted_choice<'a, T>(options: &'a [(T, f32)], rng: &mut SmallRng) -> Option<&'a T> {
    if options.is_empty() {
        return None;
    }

    let total_weight: f32 = options.iter().map(|(_, w)| *w).sum();
    if total_weight <= 0.0 {
        return Some(&options[0].0);
    }

    let mut roll = rng.gen_range(0.0..total_weight);

    for (item, weight) in options {
        roll -= weight;
        if roll <= 0.0 {
            return Some(item);
        }
    }

    Some(&options[options.len() - 1].0)
}

/// Linear interpolation
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Clamp a value between min and max
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Convert HSV to RGB (for generating distinct colors)
pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

/// Generate a distinct color for an ID
pub fn color_from_id(id: u64) -> (u8, u8, u8) {
    // Use golden ratio to generate well-distributed hues
    let golden_ratio = 0.618033988749895;
    let hue = ((id as f32 * golden_ratio) % 1.0) * 360.0;
    let saturation = 0.5 + (id % 3) as f32 * 0.15;
    let value = 0.7 + (id % 5) as f32 * 0.06;

    hsv_to_rgb(hue, saturation, value)
}

/// Format a year for display
pub fn format_year(year: i32) -> String {
    if year < 0 {
        format!("{} BCE", -year)
    } else {
        format!("{} CE", year)
    }
}

/// Format a large number with suffixes
pub fn format_number(n: u64) -> String {
    if n >= 1_000_000_000 {
        format!("{:.1}B", n as f64 / 1_000_000_000.0)
    } else if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

/// Simple pathfinding (A* or similar would be better for real use)
pub fn find_path(
    start: (u32, u32),
    end: (u32, u32),
    world_width: u32,
    world_height: u32,
    is_passable: impl Fn(u32, u32) -> bool,
    max_steps: u32,
) -> Option<Vec<(u32, u32)>> {
    use std::collections::{BinaryHeap, HashMap, HashSet};

    #[derive(Clone, Eq, PartialEq)]
    struct Node {
        pos: (u32, u32),
        cost: u32,
        heuristic: u32,
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic))
        }
    }

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    if start == end {
        return Some(vec![start]);
    }

    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<(u32, u32), (u32, u32)> = HashMap::new();
    let mut g_score: HashMap<(u32, u32), u32> = HashMap::new();
    let mut closed: HashSet<(u32, u32)> = HashSet::new();

    g_score.insert(start, 0);
    open_set.push(Node {
        pos: start,
        cost: 0,
        heuristic: wrapped_distance(start.0, start.1, end.0, end.1, world_width) as u32,
    });

    let directions: [(i32, i32); 8] = [
        (-1, 0), (1, 0), (0, -1), (0, 1),
        (-1, -1), (1, -1), (-1, 1), (1, 1),
    ];

    let mut steps = 0;

    while let Some(current) = open_set.pop() {
        if current.pos == end {
            // Reconstruct path
            let mut path = vec![end];
            let mut pos = end;
            while let Some(&prev) = came_from.get(&pos) {
                path.push(prev);
                pos = prev;
            }
            path.reverse();
            return Some(path);
        }

        if closed.contains(&current.pos) {
            continue;
        }
        closed.insert(current.pos);

        steps += 1;
        if steps > max_steps {
            return None;
        }

        for (dx, dy) in &directions {
            let nx = ((current.pos.0 as i32 + dx + world_width as i32) % world_width as i32) as u32;
            let ny = (current.pos.1 as i32 + dy).clamp(0, world_height as i32 - 1) as u32;

            if closed.contains(&(nx, ny)) || !is_passable(nx, ny) {
                continue;
            }

            let move_cost = if *dx != 0 && *dy != 0 { 14 } else { 10 };
            let tentative_g = g_score.get(&current.pos).unwrap_or(&u32::MAX).saturating_add(move_cost);

            if tentative_g < *g_score.get(&(nx, ny)).unwrap_or(&u32::MAX) {
                came_from.insert((nx, ny), current.pos);
                g_score.insert((nx, ny), tentative_g);
                open_set.push(Node {
                    pos: (nx, ny),
                    cost: tentative_g,
                    heuristic: wrapped_distance(nx, ny, end.0, end.1, world_width) as u32,
                });
            }
        }
    }

    None
}
