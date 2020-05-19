pub fn get_line(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<(i32, i32)> {
    if (y1 - y0).abs() < (x1 - x0).abs() {
        if x0 > x1 {
            get_line_low(x1, y1, x0, y0, true)
        } else {
            get_line_low(x0, y0, x1, y1, false)
        }
    } else {
        if y0 > y1 {
            get_line_high(x1, y1, x0, y0, true)
        } else {
            get_line_high(x0, y0, x1, y1, false)
        }
    }
}

fn get_line_low(x0: i32, y0: i32, x1: i32, y1: i32, flip: bool) -> Vec<(i32, i32)> {
    let dx = x1 - x0;
    let (dy, yi) = if y1 < y0 { (y0 - y1, -1) } else { (y1 - y0, 1) };

    let mut d = 2*dy - dx;
    let mut y = y0;

    let mut line = vec![];
    for x in x0..x1+1 {
        if flip {
            line.insert(0, (x, y));
        } else {
            line.push((x, y));
        }
        if d > 0 {
            y += yi;
            d -= 2*dx;
        }
        d += 2*dy;
    }

    line
}

fn get_line_high(x0: i32, y0: i32, x1: i32, y1: i32, flip: bool) -> Vec<(i32, i32)> {
    let dy = y1 - y0;
    let (dx, xi) = if x1 < x0 { (x0 - x1, -1) } else { (x1 - x0, 1) };

    let mut d = 2*dx - dy;
    let mut x = x0;

    let mut line = vec![];
    for y in y0..y1+1 {
        if flip {
            line.insert(0, (x, y));
        } else {
            line.push((x, y));
        }
        if d > 0 {
            x += xi;
            d -= 2*dy;
        }
        d += 2*dx;
    }

    line
}

