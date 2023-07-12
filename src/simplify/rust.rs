use crate::geometry::{FarthestPoint, Point};
use crate::surface::rust::SurfaceMapping;
use std::collections::HashSet;

struct SurfaceGroupIter<'a, 'b> {
    points: &'a [Point],
    mapping: &'b SurfaceMapping,
    group: Option<&'b String>,
}

impl<'a, 'b> SurfaceGroupIter<'a, 'b> {
    fn new(points: &'a [Point], mapping: &'b SurfaceMapping) -> Self {
        Self {
            points,
            mapping,
            group: points.first().and_then(|point| mapping.get_surface_group(point)),
        }
    }
}

impl<'a, 'b> Iterator for SurfaceGroupIter<'a, 'b> {
    type Item = &'a [Point];

    fn next(&mut self) -> Option<Self::Item> {
        let mut partition_len = 0;
        for point in self.points.iter() {
            let group = self.mapping.get_surface_group(point);

            if self.group == group {
                partition_len += 1;
            } else {
                self.group = group;
                break;
            }
        }

        if partition_len > 0 {
            let (partition, new_points) = self.points.split_at(partition_len);
            self.points = new_points;
            Some(partition)
        } else {
            None
        }
    }
}

pub(crate) fn simplify_points(points: &[Point], mapping: &SurfaceMapping, tolerance: f64) -> HashSet<usize> {
    fn stack_rdp(points: &[Point], tolerance_sq: f64) -> HashSet<usize> {
        let mut anchors = HashSet::new();
        let mut stack = Vec::new();
        stack.push(points);

        while let Some(slice) = stack.pop() {
            let (farthest_index, farthest_dist) = slice.farthest_point();

            if farthest_dist > tolerance_sq {
                stack.push(&slice[..=farthest_index]);
                stack.push(&slice[farthest_index..]);
            } else {
                anchors.insert(slice.first().unwrap().index());
                anchors.insert(slice.last().unwrap().index());
            }
        }

        anchors
    }

    let tolerance_sq = tolerance * tolerance;
    SurfaceGroupIter::new(points, mapping)
        .map(|points| stack_rdp(points, tolerance_sq))
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_group_iterator_all_points_missing_surface_info() {
        let mut mapping = SurfaceMapping::new(99);
        mapping.add_surface(0, "0".to_string());
        mapping.add_surface(1, "1".to_string());
        mapping.add_surface(2, "2".to_string());

        let points = vec![
            Point::new(0, 0.0, 0.0, 0.0, 0.0, None, None),
            Point::new(1, 0.0, 0.0, 0.0, 0.0, None, None),
            Point::new(2, 0.0, 0.0, 0.0, 0.0, None, None),
        ];

        let groups = SurfaceGroupIter::new(&points, &mapping).collect::<Vec<_>>();

        assert_eq!(groups, vec![points.as_slice()]);
    }

    #[test]
    fn test_surface_group_iterator_all_points_different_surface() {
        let mut mapping = SurfaceMapping::new(99);
        mapping.add_surface(0, "0".to_string());
        mapping.add_surface(1, "1".to_string());
        mapping.add_surface(2, "2".to_string());

        let points = vec![
            Point::new(0, 0.0, 0.0, 0.0, 0.0, Some(1), None),
            Point::new(1, 0.0, 0.0, 0.0, 0.0, Some(2), None),
            Point::new(2, 0.0, 0.0, 0.0, 0.0, Some(3), None),
        ];

        let groups = SurfaceGroupIter::new(&points, &mapping).collect::<Vec<_>>();

        assert_eq!(
            groups,
            vec![
                &[points[0].clone()][..],
                &[points[1].clone()][..],
                &[points[2].clone()][..]
            ]
        );
    }

    #[test]
    fn test_surface_group_iterator_normal_track() {
        let mut mapping = SurfaceMapping::new(99);
        mapping.add_surface(0, "0".to_string());
        mapping.add_surface(1, "1".to_string());
        mapping.add_surface(2, "2".to_string());

        let points = vec![
            Point::new(0, 0.0, 0.0, 0.0, 0.0, None, None),
            Point::new(1, 0.0, 0.0, 0.0, 0.0, Some(1), None),
            Point::new(2, 0.0, 0.0, 0.0, 0.0, Some(1), None),
            Point::new(3, 0.0, 0.0, 0.0, 0.0, Some(1), None),
            Point::new(4, 0.0, 0.0, 0.0, 0.0, Some(2), None),
            Point::new(5, 0.0, 0.0, 0.0, 0.0, Some(2), None),
            Point::new(6, 0.0, 0.0, 0.0, 0.0, None, None),
        ];

        let groups = SurfaceGroupIter::new(&points, &mapping).collect::<Vec<_>>();

        assert_eq!(
            groups,
            vec![
                &[points[0].clone()][..],
                &[points[1].clone(), points[2].clone(), points[3].clone()][..],
                &[points[4].clone(), points[5].clone()][..],
                &[points[6].clone()][..]
            ]
        );
    }

    #[test]
    fn test_simplifying_zero_points() {
        let mapping = SurfaceMapping::new(0);
        assert_eq!(simplify_points(&[], &mapping, 0.0), HashSet::new());
    }

    #[test]
    fn test_simplifying_one_point() {
        let mapping = SurfaceMapping::new(0);
        assert_eq!(
            simplify_points(&[Point::default()], &mapping, 0.0),
            HashSet::from_iter([0])
        );
    }

    #[test]
    fn test_simplifying_two_points() {
        let mapping = SurfaceMapping::new(0);
        assert_eq!(
            simplify_points(
                &[
                    Point::new(0, 0.0, 0.0, 0.0, 0.0, None, None),
                    Point::new(1, 1.0, 0.0, 0.0, 0.0, None, None),
                ],
                &mapping,
                0.0
            ),
            HashSet::from_iter([0, 1])
        );
    }

    #[test]
    fn test_simplifying_three_points() {
        let mapping = SurfaceMapping::new(0);
        assert_eq!(
            simplify_points(
                &[
                    Point::new(0, 0.0, 0.0, 0.0, 0.0, None, None),
                    Point::new(1, 1.0, 0.0, 0.0, 0.0, None, None),
                    Point::new(2, 2.0, 2.0, 0.0, 0.0, None, None),
                ],
                &mapping,
                0.0
            ),
            HashSet::from_iter([0, 1, 2])
        );
    }
}
