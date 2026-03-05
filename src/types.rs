use pyo3::buffer::PyBuffer;
use pyo3::{Bound, FromPyObject, PyAny, PyResult};

pub struct Samples {
    pub source: Vec<i32>,
}

impl Samples {
    fn from_buffer<T: pyo3::buffer::Element + Into<i32>>(
        buffer: &PyBuffer<T>,
        ob: &Bound<'_, PyAny>,
    ) -> Option<Self> {
        let mut source = Vec::with_capacity(buffer.item_count());
        let buffer = buffer.as_slice(ob.py())?;

        source.extend(buffer.iter().map(|s| s.get().into()));
        Some(Self { source })
    }

    pub fn normalized_source(&self, max_amplitude: f64) -> Vec<f64> {
        self.source
            .iter()
            .map(|s| *s as f64 / max_amplitude)
            .collect()
    }
}

impl<'py> FromPyObject<'py> for Samples {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        macro_rules! try_extract_buffer {
            ($type:ty) => {
                if let Ok(buffer) = PyBuffer::<$type>::get(ob) {
                    if let Some(samples) = Samples::from_buffer(&buffer, ob) {
                        return Ok(samples);
                    }
                }
            };
        }

        try_extract_buffer!(i8);
        try_extract_buffer!(i16);
        try_extract_buffer!(i32);

        ob.extract::<Vec<i32>>().map(|v| Samples { source: v })
    }
}
