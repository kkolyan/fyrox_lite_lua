use std::cmp::Ordering;

use fyrox::{
    core::{
        algebra::{Point3, Vector3},
        pool::Handle,
    },
    scene::{
        collider,
        graph::physics::{FeatureId, Intersection, QueryResultsStorage, RayCastOptions},
        node::Node,
    },
};

use crate::{lite_math::LiteVector3, lite_node::LiteNode, script_context::with_script_context};

#[derive(Debug)]
pub struct LitePhysics;

impl LitePhysics {
    pub fn cast_ray(opts: LiteRayCastOptions, results: &mut Vec<LiteIntersection>) {
        with_script_context(|ctx| {
            ctx.scene
                .as_mut()
                .expect("scene unavailable")
                .graph
                .physics
                .cast_ray(opts.into(), &mut QueryResultsStorageWrapper(results));
        });
    }
}

struct QueryResultsStorageWrapper<'a>(&'a mut Vec<LiteIntersection>);

impl<'a> QueryResultsStorage for QueryResultsStorageWrapper<'a> {
    fn push(&mut self, intersection: Intersection) -> bool {
        self.0.push(LiteIntersection::from(&intersection));
        true
    }

    fn clear(&mut self) {
        self.0.clear();
    }

    fn sort_intersections_by<C: FnMut(&Intersection, &Intersection) -> Ordering>(
        &mut self,
        mut cmp: C,
    ) {
        self.0.sort_by(|a, b| cmp(&a.into(), &b.into()))
    }
}

impl From<&LiteIntersection> for Intersection {
    fn from(value: &LiteIntersection) -> Self {
        Self {
            collider: value.collider.inner(),
            normal: value.normal,
            position: value.position,
            feature: value.feature,
            toi: value.toi,
        }
    }
}

impl From<&Intersection> for LiteIntersection {
    fn from(value: &Intersection) -> Self {
        Self {
            collider: LiteNode::new(value.collider),
            normal: value.normal,
            position: value.position,
            feature: value.feature,
            toi: value.toi,
        }
    }
}

/// A trait for ray cast results storage. It has two implementations: Vec and ArrayVec.
/// Latter is needed for the cases where you need to avoid runtime memory allocations
/// and do everything on stack.
pub trait LiteQueryResultsStorage {
    /// Pushes new intersection in the storage. Returns true if intersection was
    /// successfully inserted, false otherwise.
    fn push(&mut self, intersection: LiteIntersection) -> bool;

    /// Clears the storage.
    fn clear(&mut self);

    /// Sorts intersections by given compare function.
    fn sort_intersections_by<C: FnMut(&LiteIntersection, &LiteIntersection) -> Ordering>(
        &mut self,
        cmp: C,
    );
}

/// A ray intersection result.
#[derive(Debug, Clone, PartialEq)]
pub struct LiteIntersection {
    /// A handle of the collider with which intersection was detected.
    pub collider: LiteNode,

    /// A normal at the intersection position.
    pub normal: Vector3<f32>,

    /// A position of the intersection in world coordinates.
    pub position: Point3<f32>,

    /// Additional data that contains a kind of the feature with which
    /// intersection was detected as well as its index.
    ///
    /// # Important notes.
    ///
    /// FeatureId::Face might have index that is greater than amount of triangles in
    /// a triangle mesh, this means that intersection was detected from "back" side of
    /// a face. To "fix" that index, simply subtract amount of triangles of a triangle
    /// mesh from the value.
    pub feature: FeatureId,

    /// Distance from the ray origin.
    pub toi: f32,
}
pub struct LiteRayCastOptions {
    /// A ray origin.
    pub ray_origin: LiteVector3,

    /// A ray direction. Can be non-normalized.
    pub ray_direction: LiteVector3,

    /// Maximum distance of cast.
    pub max_len: f32,

    /// Groups to check.
    pub groups: collider::InteractionGroups,

    /// Whether to sort intersections from closest to farthest.
    pub sort_results: bool,
}

impl From<LiteRayCastOptions> for RayCastOptions {
    fn from(
        LiteRayCastOptions {
            ray_origin,
            ray_direction,
            max_len,
            groups,
            sort_results,
        }: LiteRayCastOptions,
    ) -> Self {
        RayCastOptions {
            ray_origin: Point3::from(Vector3::from(ray_origin)),
            ray_direction: ray_direction.into(),
            max_len,
            groups,
            sort_results,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LiteRigidBody {
    pub handle: Handle<Node>,
}

impl LiteRigidBody {
    pub fn apply_force(&mut self, force: LiteVector3) {
        with_script_context(|ctx| {
            ctx.scene.as_mut().expect("scene unavailable").graph[self.handle]
                .as_rigid_body_mut()
                .apply_force(force.into())
        })
    }
}
