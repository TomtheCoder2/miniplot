use nalgebra::{DMatrix, Vector6};
use miniplot::MiniPlot;

fn main() {
    let n = 1000;
    let dt = 0.01;
    let time: Vec<f64> = (0..n).map(|i| i as f64 * dt).collect();

    let theta = DMatrix::<f64>::from_fn(3, n, |r, c| ((c as f64) * dt + r as f64).sin());

    let theta_d = DMatrix::<f64>::from_fn(3, n, |r, c| ((c as f64) * dt + r as f64).cos());

    let line = Vector6::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);

    MiniPlot::new("Joint Angles")
        .xlabel("Time")
        .ylabel("Angle [rad]")
        .matrix_rows(&time, &theta)
        .pointed()
        .matrix_rows(&time, &theta_d)
        .color(miniplot::Color32::RED)
        .dashed()
        .plot(line)
        .name("Line")
        .pointed()
        .legend()
        .show();
}
