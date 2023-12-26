use itertools::Itertools;
use std::ops::RangeInclusive;
use z3::{
    ast::{Ast, Int},
    Config, Context, Solver,
};

#[derive(Debug)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn from_str(s: &str) -> Self {
        let (x, y, z) = s
            .trim()
            .split(',')
            .flat_map(|s| s.trim().parse())
            .collect_tuple()
            .expect("{s} is not a vec");
        Self { x, y, z }
    }
}

#[derive(Debug)]
struct Hailstone {
    pos: Vec3,
    vel: Vec3,
}

impl Hailstone {
    fn from_str(s: &str) -> Self {
        let (loc, dir) = s
            .trim()
            .split_once("@")
            .map(|(l, d)| (Vec3::from_str(l), Vec3::from_str(d)))
            .unwrap();
        Self { pos: loc, vel: dir }
    }

    fn linear_consts(&self) -> (f64, f64, f64) {
        // (x,y) + t(vx, vy)
        //
        // Px = x + tVx
        // Px - x = tVx
        // (Px - x)/Vx = t
        // (Py - y)/Vy = t
        // (Px - x)/Vx = (Py - y)/Vy
        //
        // Vy(Px - x) = Vx(Py -y)
        // VyPx - Vy*x = VxPy - Vx*y
        //
        // VyPx = VxPy - Vx*y + Vy*x
        // VyPx - VxPy = -Vx*y + Vy*x
        //
        // linear equation: ax + bx = c

        (
            self.vel.y,
            -self.vel.x,
            -self.vel.x * self.pos.y + self.vel.y * self.pos.x,
        )
    }

    fn point_in_future(&self, point: (f64, f64)) -> bool {
        self.vel.x * (point.0 - self.pos.x) >= 0.0 && self.vel.y * (point.1 - self.pos.y) >= 0.0
    }

    fn intersect2d(&self, other: &Self) -> Option<(f64, f64)> {
        // a1x + b1y = c1
        // a2x + b2y = c2
        //
        // solve for x,y
        //
        // x:
        // b2a1x + b2b1y = b2c1
        // b1a2x + b1b2y = b1c2
        //
        // b2a1x - b1a2x = b2c1 - b1c2
        // (b2a1 - b1a2)x = b2c1 - b1c2
        // x = (b2c1 - b1c2) / (b2a1 - b1a2)
        //
        // y:
        // a2a1x + a2b1y = a2c1
        // a1a2x + a1b2y = a1c2
        //
        // a2b1y - a1b2y = a2c1 - a1c2
        // (a2b1 - a1b2)y = a2c1 - a1c2
        // y = (a2c1 - a1c2) / (a2b1 - a1b2)

        let (a1, b1, c1) = self.linear_consts();
        let (a2, b2, c2) = other.linear_consts();

        // Parallel
        if a1 * b2 == a2 * b1 {
            return None;
        }

        let x = (b2 * c1 - b1 * c2) / (b2 * a1 - b1 * a2);
        let y = (a2 * c1 - a1 * c2) / (a2 * b1 - a1 * b2);

        if self.point_in_future((x, y)) && other.point_in_future((x, y)) {
            Some((x, y))
        } else {
            None
        }
    }
}

fn parse(input: &str) -> Vec<Hailstone> {
    input.trim().lines().map(Hailstone::from_str).collect()
}

fn solve_1(hailstones: &[Hailstone], range: RangeInclusive<f64>) -> usize {
    hailstones
        .iter()
        .combinations(2)
        .flat_map(|hs| hs[0].intersect2d(hs[1]))
        .filter(|(x, y)| range.contains(x) && range.contains(y))
        .count()
}

fn solve_2(hailstones: &[Hailstone]) -> usize {
    // we need to find t s.t.
    // x_rock + t * vx_rock = x_hail + t * vx_hail
    // y_rock + t * vy_rock = y_hail + t * vy_hail
    // z_rock + t * vz_rock = z_hail + t * vz_hail
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let x_rock = Int::new_const(&ctx, "x");
    let y_rock = Int::new_const(&ctx, "y");
    let z_rock = Int::new_const(&ctx, "z");
    let vx_rock = Int::new_const(&ctx, "vx");
    let vy_rock = Int::new_const(&ctx, "vy");
    let vz_rock = Int::new_const(&ctx, "vz");

    // 3 iterations are enough to find my solution
    hailstones.iter().take(3).for_each(|h| {
        let x_hail = Int::from_i64(&ctx, h.pos.x as i64);
        let y_hail = Int::from_i64(&ctx, h.pos.y as i64);
        let z_hail = Int::from_i64(&ctx, h.pos.z as i64);
        let vx_hail = Int::from_i64(&ctx, h.vel.x as i64);
        let vy_hail = Int::from_i64(&ctx, h.vel.y as i64);
        let vz_hail = Int::from_i64(&ctx, h.vel.z as i64);

        let t = Int::fresh_const(&ctx, "t");

        solver.assert(&(&x_hail + &t * &vx_hail)._eq(&(&x_rock + &t * &vx_rock)));
        solver.assert(&(&y_hail + &t * &vy_hail)._eq(&(&y_rock + &t * &vy_rock)));
        solver.assert(&(&z_hail + &t * &vz_hail)._eq(&(&z_rock + &t * &vz_rock)));
    });
    solver.check();
    if let Some(model) = solver.get_model() {
        let (x, y, z) = (
            model
                .get_const_interp(&x_rock)
                .and_then(|x| x.as_i64())
                .unwrap(),
            model
                .get_const_interp(&y_rock)
                .and_then(|y| y.as_i64())
                .unwrap(),
            model
                .get_const_interp(&z_rock)
                .and_then(|z| z.as_i64())
                .unwrap(),
        );
        return (x + y + z) as usize;
    }
    panic!()
}
pub fn solve(input: &str) -> (usize, usize) {
    let hailstones = parse(input);
    // (solve_1(&hailstones, 7.0..=27.0), solve_2(&hailstones)) -- for test input
    (
        solve_1(&hailstones, 200000000000000.0..=400000000000000.0),
        solve_2(&hailstones),
    )
}
