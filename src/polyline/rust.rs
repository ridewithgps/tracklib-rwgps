use crate::geometry::Point;
use crate::surface::rust::{RoadClassId, SurfaceTypeId};

#[derive(Clone, Copy, Debug)]
pub(crate) enum PointField {
    Y,
    X,
    D,
    E,
    S { default: SurfaceTypeId },
    R { default: RoadClassId },
}

#[derive(Debug)]
pub(crate) struct PolylineOption {
    field: PointField,
    factor: f64,
}

impl PolylineOption {
    pub(crate) fn new(field: PointField, factor: f64) -> Self {
        Self { field, factor }
    }

    pub(crate) fn field(&self) -> PointField {
        self.field
    }

    pub(crate) fn factor(&self) -> f64 {
        self.factor
    }
}

fn scale(n: f64, factor: f64) -> i64 {
    (n * factor).round() as i64
}

fn encode(current: f64, previous: f64, factor: f64) -> String {
    let current_scaled = scale(current, factor);
    let previous_scaled = scale(previous, factor);
    let diff = current_scaled - previous_scaled;
    let mut v = diff << 1;
    if diff < 0 {
        v = !v;
    }

    let mut output = String::new();
    while v >= 0x20 {
        let from_char = char::from_u32(((0x20 | (v & 0x1f)) + 63) as u32).unwrap();
        output.push(from_char);
        v >>= 5;
    }
    let from_char = char::from_u32((v + 63) as u32).unwrap();
    output.push(from_char);
    output
}

pub(crate) fn polyline_encode(points: &[Point], fields: &[PolylineOption]) -> String {
    let mut output = String::new();
    let mut prev = &Point::default();

    for point in points {
        for field in fields {
            match field.field() {
                PointField::Y => output.push_str(&encode(point.y(), prev.y(), field.factor())),
                PointField::X => output.push_str(&encode(point.x(), prev.x(), field.factor())),
                PointField::D => output.push_str(&encode(point.d(), prev.d(), field.factor())),
                PointField::E => output.push_str(&encode(point.e(), prev.e(), field.factor())),
                PointField::S {
                    default: default_surface_id,
                } => output.push_str(&encode(
                    f64::from(i32::try_from(point.s().unwrap_or(default_surface_id)).unwrap_or(0)),
                    f64::from(i32::try_from(prev.s().unwrap_or(default_surface_id)).unwrap_or(0)),
                    field.factor(),
                )),
                PointField::R {
                    default: default_road_class_id,
                } => output.push_str(&encode(
                    f64::from(i32::try_from(point.r().unwrap_or(default_road_class_id)).unwrap_or(0)),
                    f64::from(i32::try_from(prev.r().unwrap_or(default_road_class_id)).unwrap_or(0)),
                    field.factor(),
                )),
            }
        }

        prev = point;
    }

    output
}
