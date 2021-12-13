#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// maps a float vector to a concrete color type
pub fn map_color(v: &[f32]) -> Color {
    Color {
        r: (v[0] * u8::max_value() as f32) as u8,
        g: (v[1] * u8::max_value() as f32) as u8,
        b: (v[2] * u8::max_value() as f32) as u8,
    }
}

// draw a line from start to end using Bresenham's line algorithm
pub fn draw_line(buf: &mut Vec<Color>, rlen: usize, start: (i32, i32), end: (i32, i32), color: Color) {
    let dx = i32::abs(end.0 - start.0);
    let dy = -i32::abs(end.1 - start.1);

    let sx = if start.0 < end.0 {
        1
    } else {
        -1
    };

    let sy = if start.1 < end.1 {
        1
    } else {
        -1
    };

    let mut err = dx + dy;
    let mut x = start.0;
    let mut y = start.1;

    loop {
        let cidx = y as usize * rlen + x as usize;

        buf[cidx] = color;

        if x == end.0 && y == end.1 {
            break;
        }

        let e2 = 2 * err;
        
        if e2 >= dy {
            err += dy;
            x += sx;
        }

        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}