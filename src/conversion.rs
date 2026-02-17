pub trait AsSliceF64 {
    fn as_slice_f64(&self) -> &[f64];
}

impl AsSliceF64 for Vec<f64> {
    fn as_slice_f64(&self) -> &[f64] {
        self.as_slice()
    }
}

impl AsSliceF64 for [f64] {
    fn as_slice_f64(&self) -> &[f64] {
        self
    }
}

impl AsSliceF64 for nalgebra::DVector<f64> {
    fn as_slice_f64(&self) -> &[f64] {
        self.as_slice()
    }
}

impl<const R: usize, const C: usize> AsSliceF64 for nalgebra::SMatrix<f64, R, C> {
    fn as_slice_f64(&self) -> &[f64] {
        self.as_slice()
    }
}
