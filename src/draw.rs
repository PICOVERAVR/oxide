#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/*

plotLine(int x0, int y0, int x1, int y1)
    dx =  abs(x1-x0);
    sx = x0<x1 ? 1 : -1;
    dy = -abs(y1-y0);
    sy = y0<y1 ? 1 : -1;
    err = dx+dy;  /* error value e_xy */
    while (true)   /* loop */
        plot(x0, y0);
        if (x0 == x1 && y0 == y1) break;
        e2 = 2*err;
        if (e2 >= dy) /* e_xy+e_x > 0 */
            err += dy;
            x0 += sx;
        end if
        if (e2 <= dx) /* e_xy+e_y < 0 */
            err += dx;
            y0 += sy;
        end if
    end while

*/

// draw a line from start to end using Bresenham's line algorithm
pub fn draw_line(buf: &mut Vec<Pixel>, rlen: usize, start: (i32, i32), end: (i32, i32), color: Pixel) {
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

        if cidx >= buf.len() {
            break; // corner case (literally) where end is the end of the buffer
        }

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