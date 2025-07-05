
mod euler;

use plotters::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};

fn ode(t: f64, y: f64) -> f64 {
    t.cos() - y
}

fn y_exact(t: f64) -> f64 {
    0.5 * (t.cos() + t.sin()) + 0.5 * (-t).exp()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let t0 = 0.0;
    let t_end = 5.0;
    let y0 = 1.0;
    let steps_list = [20, 100, 1000];
    let colors = [&RED, &BLUE, &GREEN];

    let file = File::create("euler_solutions.csv")?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "n,t,y")?;

    let root = BitMapBackend::new("euler_plot.png", (900, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Euler vs Analytical Solution", ("sans-serif", 25))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(t0..t_end, -1.5..1.5)?;
    chart.configure_mesh().draw()?;

    for (i, &n) in steps_list.iter().enumerate() {
        let result = euler::euler_method(ode, y0, t0, t_end, n);
        for (t, y) in &result {
            writeln!(writer, "{},{:.4},{:.6}", n, t, y)?;
        }
        chart.draw_series(LineSeries::new(result.clone(), colors[i]))?
            .label(format!("Euler n = {}", n))
            .legend(move |(x, y)| PathElement::new([(x, y), (x+20, y)], colors[i]));
    }

    let exact: Vec<(f64, f64)> = (0..=1000)
        .map(|i| {
            let t = t0 + i as f64 * (t_end - t0) / 1000.0;
            (t, y_exact(t))
        })
        .collect();

    chart.draw_series(LineSeries::new(exact, &BLACK))?
    .label("Analytical Solution")
    .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &BLACK));

    chart.configure_series_labels().background_style(&WHITE).draw()?;
    println!("✅ Plot and CSV created.");
    Ok(())
}
