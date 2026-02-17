use crate::conversion::AsSliceF64;
use crate::utils::get_color;
use eframe::App;
pub use egui::Color32;
use egui_plot::{Line, LineStyle, Plot, PlotPoints};

pub mod conversion;
mod utils;

struct PlotData {
    pub name: String,
    pub points: Vec<[f64; 2]>,
    pub color: Color32,
    pub dashed: bool,
    pub dotted: bool,
    pub pointed: bool,
}

impl Default for PlotData {
    fn default() -> Self {
        Self {
            name: String::new(),
            points: Vec::new(),
            color: Color32::TRANSPARENT,
            dashed: false,
            dotted: false,
            pointed: false,
        }
    }
}

struct Options {
    window_name: String,
    pub legend: bool,
    xlabel: Option<String>,
    ylabel: Option<String>,
}

pub struct MiniPlot {
    data: Vec<PlotData>,
    options: Options,
    color_index: usize,
}

impl MiniPlot {
    pub fn new(window_name: &str) -> Self {
        Self {
            data: Vec::new(),
            options: Options {
                window_name: window_name.to_string(),
                legend: false,
                xlabel: None,
                ylabel: None,
            },
            color_index: 0,
        }
    }

    pub fn get_color(&mut self) -> Color32 {
        self.color_index += 1;
        get_color(self.color_index)
    }

    pub fn xlabel(mut self, label: &str) -> Self {
        self.options.xlabel = Some(label.to_string());
        self
    }

    pub fn ylabel(mut self, label: &str) -> Self {
        self.options.ylabel = Some(label.to_string());
        self
    }

    /// Adds multiple lines to the plot, one for each row of the matrix. The x values are given by the x slice and the y values are given by the rows of the matrix. The lines are colored differently for each row.
    /// # Arguments
    /// * `x` - A slice of x values that is the same length as the number of columns in the matrix. These values are shared across all rows.
    /// * `y` - A matrix of y values where each row corresponds to a line to be plotted. The number of columns must be the same as the length of the x slice.
    /// # Example
    /// ```
    /// use nalgebra::DMatrix;
    /// use miniplot::MiniPlot;
    /// let time: Vec<f64> = (0..1000).map(|i| i as f64 * 0.01).collect();
    /// let theta = DMatrix::<f64>::from_fn(3, 1000, |r, c| ((c as f64) * 0.01 + r as f64).sin());
    /// MiniPlot::new("DMatrix Rows")
    ///     .xlabel("Time")
    ///     .ylabel("Angle [rad]")
    ///     .matrix_rows(&time, &theta)
    ///     .show();
    /// ```
    pub fn matrix_rows(mut self, x: &[f64], y: &nalgebra::DMatrix<f64>) -> Self {
        for i in 0..y.nrows() {
            let points: Vec<[f64; 2]> = x.iter().zip(y.row(i)).map(|(&x, &y)| [x, y]).collect();
            let color = self.get_color();
            self.data.push(PlotData {
                name: format!("Row {}", i),
                points,
                color,
                ..Default::default()
            });
        }
        self
    }

    /// Adds multiple lines to the plot, one for each row of the matrix. The x values are given by the x slice and the y values are given by the rows of the matrix. The lines are colored differently for each row.
    /// # Arguments
    /// * `x` - A slice of x values that is the same length as the number of columns in the matrix. These values are shared across all rows.
    /// * `y` - A matrix of y values where each row corresponds to a line to be plotted. The number of columns must be the same as the length of the x slice.
    /// # Example
    /// ```
    /// use nalgebra::SMatrix;
    /// use miniplot::MiniPlot;
    /// const N: usize = 1000;
    /// const R: usize = 3;
    /// let time: Vec<f64> = (0..N).map(|i| i as f64 * 0.01).collect();
    /// let theta = SMatrix::<f64, R, N>::from_fn(|r, c| ((c as f64) * 0.01 + r as f64).sin());
    /// MiniPlot::new("SMatrix Rows")
    ///     .xlabel("Time")
    ///     .ylabel("Angle [rad]")
    ///     .smatrix_rows(&time, &theta)
    ///     .show();
    /// ```
    pub fn smatrix_rows<const R: usize, const C: usize>(
        mut self,
        x: &[f64],
        y: &nalgebra::SMatrix<f64, R, C>,
    ) -> Self {
        for i in 0..y.nrows() {
            let points: Vec<[f64; 2]> = x.iter().zip(y.row(i)).map(|(&x, &y)| [x, y]).collect();
            let color = self.get_color();
            self.data.push(PlotData {
                name: format!("Row {}", i),
                points,
                color,
                ..Default::default()
            });
        }
        self
    }

    pub fn dashed(mut self) -> Self {
        if let Some(last) = self.data.last_mut() {
            last.dashed = true;
        }
        self
    }

    pub fn dotted(mut self) -> Self {
        if let Some(last) = self.data.last_mut() {
            last.dotted = true;
        }
        self
    }

    /// Pointed means that the points of the line are also shown, which can be useful for sparse data
    pub fn pointed(mut self) -> Self {
        if let Some(last) = self.data.last_mut() {
            last.pointed = true;
        }
        self
    }

    pub fn legend(mut self) -> Self {
        self.options.legend = true;
        self
    }

    /// changes the color of the last line added and does nothing if no line has been added yet
    pub fn color(mut self, color: Color32) -> Self {
        if let Some(last) = self.data.last_mut() {
            last.color = color;
        }
        self
    }

    /// Changes the name of the last line added and does nothing if no line has been added yet
    pub fn name(mut self, name: &str) -> Self {
        if let Some(last) = self.data.last_mut() {
            last.name = name.to_string();
        }
        self
    }


    /// Makes a line plot of the given data, where the x values are the indices of the y values. The line is colored differently for each call to this method.
    /// # Arguments
    /// * `line` - A slice of y values to be plotted. Must be convertible to a slice of f64 using the AsSliceF64 trait. The x values are automatically generated as the indices of the y values.
    /// # Example - normal rust vector
    /// ```
    /// use miniplot::MiniPlot;
    /// let data: Vec<f64> = (0..1000).map(|i| (i as f64 / 50.).sin()).collect();
    /// MiniPlot::new("Sine Wave - Rust Vec")
    ///     .plot(data)
    ///     .show();
    /// ```
    /// # Example - nalgebra DVector
    /// ```
    /// use nalgebra::DVector;
    /// use miniplot::MiniPlot;
    /// let data: DVector<f64> = DVector::from_fn(1000, |i, _| (i as f64 / 50.).sin());
    /// MiniPlot::new("Sine Wave - DVector")
    ///     .plot(data)
    ///     .show();
    /// ```
    /// # Example - nalgebra SVector
    /// ```
    /// use nalgebra::SVector;
    /// use miniplot::MiniPlot;
    /// const N: usize = 1000;
    /// let data: SVector<f64, N> = SVector::from_fn(|i, _| (i as f64 / 50.).sin());
    /// MiniPlot::new("Sine Wave - SVector")
    ///     .plot(data)
    ///     .show();
    /// ```
    pub fn plot(mut self, line: impl AsSliceF64) -> Self {
        let points: Vec<[f64; 2]> = line
            .as_slice_f64()
            .iter()
            .enumerate()
            .map(|(i, &y)| [i as f64, y])
            .collect();
        let color = self.get_color();
        self.data.push(PlotData {
            name: format!("Line {}", self.data.len()),
            points,
            color,
            ..Default::default()
        });
        self
    }

    pub fn show(self) {
        let options = eframe::NativeOptions::default();
        eframe::run_native(
            &self.options.window_name.clone(),
            options,
            Box::new(|_cc| Ok(Box::new(PlotWindow::new(self)))),
        )
        .expect("Failed to run eframe");
    }

    fn show_plot(&self, ui: &mut egui::Ui) {
        Plot::new(&self.options.window_name)
            .x_axis_label(self.options.xlabel.clone().unwrap_or_default())
            .y_axis_label(self.options.ylabel.clone().unwrap_or_default())
            .legend(if self.options.legend {
                egui_plot::Legend::default()
            } else {
                egui_plot::Legend::default()
            })
            .show(ui, |plot_ui| {
                for data in &self.data {
                    let plot_points = PlotPoints::from_iter(data.points.clone().into_iter());
                    plot_ui.line(Line::new(&data.name, plot_points).color(data.color).style(
                        if data.dashed {
                            LineStyle::dashed_loose()
                        } else if data.dotted {
                            LineStyle::dotted_loose()
                        } else {
                            LineStyle::Solid
                        },
                    ));
                    if data.pointed {
                        let plot_points = PlotPoints::from_iter(data.points.clone().into_iter());
                        plot_ui.points(
                            egui_plot::Points::new(&data.name, plot_points)
                                .color(data.color)
                                .radius(4.),
                        );
                    }
                }
            });
    }
}

struct PlotWindow {
    mini_plot: MiniPlot,
}

impl PlotWindow {
    fn new(mini_plot: MiniPlot) -> PlotWindow {
        PlotWindow { mini_plot }
    }
}

impl App for PlotWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.mini_plot.show_plot(ui);
        });
    }
}
