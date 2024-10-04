#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;
use polygonical::point::Point;

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct Controller {}

impl Controller {
    pub fn init() -> Result<Self, RenderError> {
        let ctrl = Self::default();
        use_context_provider(|| ctrl);

        tracing::debug!("Home data loaded");

        Ok(ctrl)
    }

    pub fn use_controller() -> Self {
        use_context()
    }

    pub fn points_by_polygon(num_points: usize, center: (f32, f32), radius: f32) -> Vec<Point> {
        if num_points == 0 {
            return vec![];
        }
        let center = Point::new(center.0, center.1);
        let num_points = if num_points < 3 { 3 } else { num_points };

        (0..360)
            .step_by(360 / num_points)
            .map(|a| {
                Point::new(radius, 0.0)
                    .rotate((a as f64).to_radians())
                    .translate(&center)
            })
            .collect::<Vec<_>>()[..num_points - 1]
            .to_vec()
    }
}
