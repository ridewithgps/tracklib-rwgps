use crate::surface::rust::{RoadClassId, SurfaceTypeId};
use rutie::{Class, VM};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Point {
    index: usize,
    x: f64,
    y: f64,
    d: f64,
    e: f64,
    s: Option<SurfaceTypeId>,
    r: Option<RoadClassId>,
}

impl Point {
    pub(crate) fn new(
        index: usize,
        x: f64,
        y: f64,
        d: f64,
        e: f64,
        s: Option<SurfaceTypeId>,
        r: Option<RoadClassId>,
    ) -> Self {
        Self {
            index,
            x,
            y,
            d,
            e,
            s,
            r,
        }
    }

    pub(crate) fn index(&self) -> usize {
        self.index
    }

    pub(crate) fn x(&self) -> f64 {
        self.x
    }

    pub(crate) fn y(&self) -> f64 {
        self.y
    }

    pub(crate) fn d(&self) -> f64 {
        self.d
    }

    pub(crate) fn e(&self) -> f64 {
        self.e
    }

    pub(crate) fn s(&self) -> Option<SurfaceTypeId> {
        self.s
    }

    pub(crate) fn r(&self) -> Option<RoadClassId> {
        self.r
    }
}

impl Default for Point {
    fn default() -> Self {
        Self {
            index: 0,
            x: 0.0,
            y: 0.0,
            d: 0.0,
            e: 0.0,
            s: Some(0),
            r: Some(0),
        }
    }
}

pub(crate) trait FarthestPoint {
    fn farthest_point(&self) -> (usize, f64);
}

impl FarthestPoint for &[Point] {
    fn farthest_point(&self) -> (usize, f64) {
        let line = Line::new(self.first().unwrap(), self.last().unwrap());

        self.iter()
            .enumerate()
            .take(self.len() - 1) // Don't include the last index
            .skip(1) // Don't include the first index
            .map(|(index, point)| (index, line.distance_2d(&point)))
            .fold(
                (0, 0.0),
                |(farthest_index, farthest_dist), (index, distance)| {
                    if distance > farthest_dist {
                        (index, distance)
                    } else {
                        (farthest_index, farthest_dist)
                    }
                },
            )
    }
}

struct Line<'a> {
    start: &'a Point,
    end: &'a Point,
}

impl<'a> Line<'a> {
    fn new(start: &'a Point, end: &'a Point) -> Self {
        Self { start, end }
    }

    fn distance_2d(&self, point: &Point) -> f64 {
        let mut x = self.start.x;
        let mut y = self.start.y;

        let mut dx = self.end.x - x;
        let mut dy = self.end.y - y;

        if dx != 0.0 || dy != 0.0 {
            let t = ((point.x - x) * dx + (point.y - y) * dy) / (dx * dx + dy * dy);

            if t > 1.0 {
                x = self.end.x;
                y = self.end.y;
            } else if t > 0.0 {
                x += dx * t;
                y += dy * t;
            }
        }

        dx = point.x - x;
        dy = point.y - y;

        return dx * dx + dy * dy;
    }
}

pub(crate) fn haversine_distance(prev: &Point, x: f64, y: f64) -> f64 {
    // lifted wholesale from https://github.com/georust/geo/blob/2cf153d59072d18054baf4da8bcaf3e0c088a7d8/geo/src/algorithm/haversine_distance.rs
    const MEAN_EARTH_RADIUS: f64 = 6_371_000.0;

    let theta1 = prev.y.to_radians();
    let theta2 = y.to_radians();
    let delta_theta = (y - prev.y).to_radians();
    let delta_lambda = (x - prev.x).to_radians();
    let a = (delta_theta / 2.0).sin().powi(2) + theta1.cos() * theta2.cos() * (delta_lambda / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    MEAN_EARTH_RADIUS * c
}

use std::collections::HashMap;

fn new_point<'a, T>(index: usize, prev: Option<&Point>, iter: T) -> Option<Point>
where
    T: IntoIterator<
        Item = (
            &'a tracklib::schema::FieldDefinition,
            Option<tracklib::types::FieldValue>,
        ),
    >,
{
    let fields = iter
        .into_iter()
        .map(|(field_def, field_value)| (field_def.name(), field_value))
        .collect::<HashMap<_, _>>();

    if let Some((x, y, e, d)) = match (fields.get("x"), fields.get("y"), fields.get("e")) {
        (
            Some(Some(tracklib::types::FieldValue::F64(x))),
            Some(Some(tracklib::types::FieldValue::F64(y))),
            Some(Some(tracklib::types::FieldValue::F64(e))),
        ) => {
            let d = if let Some(p) = prev {
                p.d() + haversine_distance(p, *x, *y)
            } else {
                0.0
            };

            Some((*x, *y, *e, d))
        }
        _ => None,
    } {
        let mut s = None;
        let mut r = None;

        match fields.get("S") {
            Some(Some(tracklib::types::FieldValue::U64(v))) => s = Some(*v),
            None | Some(None) => {}
            _ => {
                return None;
            }
        }

        match fields.get("R") {
            Some(Some(tracklib::types::FieldValue::U64(v))) => r = Some(*v),
            None | Some(None) => {}
            _ => {
                return None;
            }
        }

        Some(Point::new(index, x, y, d, e, s, r))
    } else {
        None
    }
}

#[derive(PartialEq)]
pub(crate) enum IrrelevantPointsBehavior {
    Count,
    Ignore,
}

pub(crate) fn reader_to_points(
    mut reader: tracklib::read::section::reader::SectionReader,
    irrelevant_points_behavior: IrrelevantPointsBehavior,
) -> Vec<Point> {
    let mut index = 0;
    let mut points = Vec::with_capacity(reader.rows_remaining());
    while let Some(columniter) = reader.open_column_iter() {
        if let Some(point) = new_point(
            index,
            points.last(),
            columniter.map(|row| {
                row.map_err(|e| VM::raise(Class::from_existing("Exception"), &format!("{}", e)))
                    .unwrap()
            }),
        ) {
            points.push(point);
            index += 1;
        } else {
            if irrelevant_points_behavior == IrrelevantPointsBehavior::Count {
                index += 1;
            }
        }
    }

    points
}
