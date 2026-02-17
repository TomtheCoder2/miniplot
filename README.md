# MiniPlot
A simple plotting library that makes it easy to plot data from vectors or `nalgebra` types without boilerplate.
### Example - Plot rust vector

```rust
use miniplot::MiniPlot;
fn main() {
    let data: Vec<f64> = (0..1000).map(|i| (i as f64 / 50.).sin()).collect();
    MiniPlot::new("Sine Wave - Rust Vec")
        .plot(data)
        .show();
}
```
### Example - Plot nalgebra vector

```rust
use miniplot::MiniPlot;
use nalgebra::DVector;
fn main() {
    let data: DVector<f64> = DVector::from_fn(1000, |i, _| (i as f64 / 50.).sin());
    MiniPlot::new("Sine Wave - nalgebra DVector")
        .plot(data)
        .show();
}
```

### Example - Plot multiple series

```rust
use miniplot::MiniPlot;
fn main() {
    let data1: Vec<f64> = (0..1000).map(|i| (i as f64 / 50.).sin()).collect();
    let data2: Vec<f64> = (0..1000).map(|i| (i as f64 / 50.).cos()).collect();
    MiniPlot::new("Sine and Cosine Waves")
        .plot(data1)
        .plot(data2)
        .show();
}
```