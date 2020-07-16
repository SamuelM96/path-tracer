use crate::bounds::Bounds3;
use crate::intersectable::{IntersectRecord, Intersectable};
use crate::ray::Ray;
use crate::shape::Shape;
use crate::utils::partition;
use std::sync::Arc;

#[allow(dead_code)]
#[derive(Default)]
pub struct BVH {
    pub root: BVHNode,
    pub shapes: Vec<Arc<dyn Shape>>,
}

#[allow(dead_code)]
impl BVH {
    pub fn intersect(&self, ray: &Ray, test_texture_alpha: bool) -> Option<(IntersectRecord, f32)> {
        self.root.intersect(&self.shapes, ray, test_texture_alpha)
    }
}

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

    pub fn construct(shapes: &mut Vec<Arc<dyn Shape>>) -> BVH {
        let mut ordered_shapes = Vec::new();
        let len = shapes.len();
        let node = Self::construct_recurse(&mut shapes[0..len], &mut ordered_shapes);

        BVH {
            root: node,
            shapes: ordered_shapes,
        }
    }

    fn construct_recurse(
        shapes: &mut [Arc<dyn Shape>],
        ordered_shapes: &mut Vec<Arc<dyn Shape>>,
    ) -> BVHNode {
        let start_bounds = shapes[0].world_bounds();
        let mut bounds = start_bounds;
        for shape in shapes.iter().skip(1) {
            bounds = bounds.union(&shape.world_bounds());
        }

        let count = shapes.len();
        if count == 1 {
            let offset = ordered_shapes.len();
            ordered_shapes.push(shapes[0].clone());

            BVHNode::create_leaf(offset, count, bounds)
        } else {
            let mut centroid_bounds = Bounds3::new(start_bounds.centroid, start_bounds.centroid);
            for shape in shapes.iter().skip(1) {
                centroid_bounds = centroid_bounds.union_point(shape.world_bounds().centroid);
            }
            let dim = centroid_bounds.maximum_extent();

            if centroid_bounds.p_min[dim]
                .partial_cmp(&centroid_bounds.p_max[dim])
                .unwrap()
                == std::cmp::Ordering::Equal
            {
                let offset = ordered_shapes.len();
                for shape in shapes.iter() {
                    ordered_shapes.push(shape.clone());
                }

                BVHNode::create_leaf(offset, count, bounds)
            } else {
                let p_mid = (centroid_bounds.p_min[dim] + centroid_bounds.p_max[dim]) / 2.0;
                let (left, right) =
                    partition(shapes, |shape| shape.world_bounds().centroid[dim] < p_mid);

                BVHNode::create_parent(
                    Box::new(BVHNode::construct_recurse(left, ordered_shapes)),
                    Box::new(BVHNode::construct_recurse(right, ordered_shapes)),
                    bounds,
                )
            }
        }
    }

    pub fn intersect(
        &self,
        shapes: &[Arc<dyn Shape>],
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
                for i in self.offset..self.offset + self.count {
                    if let Some((rec, dist)) = shapes[i].intersect(ray, test_alpha_textures) {
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
