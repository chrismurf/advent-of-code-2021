struct Pose {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32
}

impl Pose {
    fn step(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
        self.dy -= 1;
        if self.dx > 0 { self.dx -= 1; }
        else if self.dx < 0 { self.dx += 1; }
    }
}

fn main() {
    // target area: x=79..137, y=-176..-117
    const MIN_X: i32 = 79;
    const MAX_X: i32 = 137;
    const MIN_Y: i32 = -176;
    const MAX_Y: i32 = -117;

    let mut best_peak_y  : i32 = i32::MIN;
    let mut best_dx : i32 = 0;
    let mut best_dy : i32 = 0;
    let mut in_box : u32 = 0;

    for dx in 0..1000 {
        for dy in -1000..10000 {
            let mut pose = Pose {x:0, y:0, dx, dy };
            let mut peak_y = i32::MIN;
            let mut was_in_box = false;

            while pose.y >= MIN_Y && pose.x <= MAX_X {
                if pose.y > peak_y { peak_y = pose.y; }

                if pose.y >= MIN_Y &&
                   pose.y <= MAX_Y &&
                   pose.x >= MIN_X &&
                   pose.x <= MAX_X {
                    was_in_box = true;

                    if peak_y > best_peak_y {
                        best_peak_y = peak_y;
                        best_dx = dx;
                        best_dy = dy;
                        break;
                    }
                }
                pose.step();
            }
            if was_in_box { in_box += 1; }
        }
    }

    println!("Best dx: {}, dy: {}, peak: {}", best_dx, best_dy, best_peak_y);
    println!("Total in box: {}", in_box);
}
