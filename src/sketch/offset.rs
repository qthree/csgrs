use crate::float_types::Real;
use crate::sketch::Sketch;
use geo::buffer::{BufferStyle, LineCap, LineJoin};
use geo::Buffer;
use std::fmt::Debug;
use std::sync::OnceLock;

fn buffer_style_square(d: Real) -> BufferStyle<Real> {
    BufferStyle::new(d).line_cap(LineCap::Square).line_join(LineJoin::Miter(0.20f32.into()))
}

fn buffer_style_rounded(d: Real) -> BufferStyle<Real> {
    BufferStyle::new(d)
}

impl<S: Clone + Debug + Send + Sync> Sketch<S> {
    fn offset_with_style(&self, style: BufferStyle<Real>) -> Sketch<S> {
        let geometry = self
            .geometry
            .iter()
            .map(|geom| geom.buffer_with_style(style.clone()))
            .collect();

        // Return a new Sketch using the offset geometry collection and the old metadata
        Sketch {
            geometry,
            bounding_box: OnceLock::new(),
            metadata: self.metadata.clone(),
        }
    }
    pub fn offset(&self, distance: Real) -> Sketch<S> {
        self.offset_with_style(buffer_style_square(distance))
    }

    pub fn offset_rounded(&self, distance: Real) -> Sketch<S> {
        self.offset_with_style(buffer_style_rounded(distance))
    }
}
