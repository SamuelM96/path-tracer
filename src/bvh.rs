use crate::bounds::Bounds3;
use crate::intersectable::IntersectRecord;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::utils::partition;

#[allow(dead_code)]
#[derive(Default)]
pub struct BVHNode {
    pub left: Option<Box<BVHNode>>,
    pub right: Option<Box<BVHNode>>,
    pub offset: usize,
    pub count: usize,
    pub bounds: Bounds3,
}

#[allow(dead_code)]
impl BVHNode {
    pub fn create_leaf(offset: usize, count: usize, bounds: Bounds3) -> BVHNode {
        BVHNode {
            left: None,
            right: None,
            offset,
            count,
            bounds,
        }
    }

    pub fn create_parent(left: Box<BVHNode>, right: Box<BVHNode>, bounds: Bounds3) -> BVHNode {
        BVHNode {
            left: Some(left),
            right: Some(right),
            offset: 0,
            count: 0,
            bounds,
        }
    }

    pub fn construct(shapes: &mut Vec<Box<dyn Shape>>) -> BVHNode {
        let len = shapes.len();
        Self::construct_recurse(&mut shapes[..], 0, len)
    }

    fn construct_recurse(shapes: &mut [Box<dyn Shape>], start: usize, end: usize) -> BVHNode {
        let start_bounds = shapes[start].world_bounds();
        let mut bounds = start_bounds;
        for shape in shapes[start + 1..end].iter() {
            bounds = bounds.union(&shape.world_bounds());
        }

        let count = end - start;
        if count == 1 {
            BVHNode::create_leaf(start, count, bounds)
        } else {
            let mut centroid_bounds = Bounds3::new(start_bounds.centroid, start_bounds.centroid);
            for shape in shapes[start + 1..end].iter() {
                centroid_bounds = centroid_bounds.union_point(shape.world_bounds().centroid);
            }
            let dim = centroid_bounds.maximum_extent();

            if centroid_bounds.p_min[dim]
                .partial_cmp(&centroid_bounds.p_max[dim])
                .unwrap()
                == std::cmp::Ordering::Equal
            {
                BVHNode::create_leaf(start, count, bounds)
            } else {
                let p_mid = (centroid_bounds.p_min[dim] + centroid_bounds.p_max[dim]) / 2.0;
                partition(&mut shapes[start..end], |shape| {
                    shape.world_bounds().centroid[dim] < p_mid
                });
                let mid = start + count / 2;

                BVHNode::create_parent(
                    Box::new(BVHNode::construct_recurse(shapes, start, mid)),
                    Box::new(BVHNode::construct_recurse(shapes, mid, end)),
                    bounds,
                )
            }
        }
    }

    pub fn intersect(
        &self,
        shapes: &[Box<dyn Shape>],
        ray: &Ray,
        test_alpha_textures: bool,
    ) -> Option<(IntersectRecord, f32)> {
        if self.bounds.intersect(ray) {
            let mut distance = ray.t_max;
            let mut record = None;

            return if let Some(left) = &self.left {
                if let Some((rec, dist)) = left.intersect(shapes, ray, test_alpha_textures) {
                    distance = dist;
                    record = Some(rec);
                }

                if let Some(right) = &self.right {
                    let ray = Ray::new(ray.origin, ray.direction, ray.t_min, distance);
                    if let Some((rec, dist)) = right.intersect(shapes, &ray, test_alpha_textures) {
                        if dist < distance {
                            distance = dist;
                            record = Some(rec);
                        }
                    }
                }

                match record {
                    Some(rec) => Some((rec, distance)),
                    None => None,
                }
            } else {
                for shape in shapes[self.offset..self.offset + self.count].iter() {
                    if let Some((rec, dist)) = shape.intersect(ray, test_alpha_textures) {
                        if dist < distance {
                            distance = dist;
                            record = Some(rec);
                        }
                    }
                }

                match record {
                    Some(rec) => Some((rec, distance)),
                    None => None,
                }
            };
        }

        None
    }
}
