
// Euler method to solve dy/dt = f(t, y)
pub fn euler_method(
    f: impl Fn(f64, f64) -> f64,
    y0: f64,
    t0: f64,
    t_end: f64,
    n: usize,
) -> Vec<(f64, f64)> {
    let h = (t_end - t0) / n as f64;
    let mut result = Vec::new();
    let mut t = t0;
    let mut y = y0;

    result.push((t, y));
    for _ in 0..n {
        y += h * f(t, y);
        t += h;
        result.push((t, y));
    }

    result
}
