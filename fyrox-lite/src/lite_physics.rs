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
use fyrox_lite_macro::fyrox_lite_pod;

use crate::{lite_math::PodVector3, lite_node::LiteNode, script_context::with_script_context, spi::{DynamicArray, LiteOrdering}};

#[derive(Debug)]
pub struct LitePhysics;

impl LitePhysics {
    pub fn cast_ray(opts: LiteRayCastOptions, results: &mut dyn DynamicArray<LiteIntersection>) {
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

struct QueryResultsStorageWrapper<'a>(&'a mut dyn DynamicArray<LiteIntersection>);

impl <'a> QueryResultsStorage for QueryResultsStorageWrapper<'a> {
    fn push(&mut self, intersection: Intersection) -> bool {
        self.0.add(LiteIntersection::from(&intersection));
        true
    }

    fn clear(&mut self) {
        self.0.clear();
    }

    fn sort_intersections_by<C: FnMut(&Intersection, &Intersection) -> Ordering>(
        &mut self,
        mut cmp: C,
    ) {
        self.0.sort(&mut |a, b| match cmp(&a.into(), &b.into()) {
            Ordering::Less => LiteOrdering::Less,
            Ordering::Equal => LiteOrdering::Equal,
            Ordering::Greater => LiteOrdering::Greater,
        })
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
#[derive(PartialEq)]
#[fyrox_lite_pod("Intersection")]
pub struct LiteIntersection {
    /// A handle of the collider with which intersection was detected.
    pub collider: LiteNode,

    /// A normal at the intersection position.
    pub normal: PodVector3,

    /// A position of the intersection in world coordinates.
    pub position: PodVector3,

    /// Distance from the ray origin.
    pub toi: f32,
}

#[fyrox_lite_pod("RayCastOptions")]
pub struct LiteRayCastOptions {
    /// A ray origin.
    pub ray_origin: PodVector3,

    /// A ray direction. Can be non-normalized.
    pub ray_direction: PodVector3,

    /// Maximum distance of cast.
    pub max_len: f32,

    /// Groups to check.
    pub groups: LiteInteractionGroups,

    /// Whether to sort intersections from closest to farthest.
    pub sort_results: bool,
}

#[derive(Copy, PartialEq, Eq)]
#[fyrox_lite_pod("InteractionGroups")]
pub struct LiteInteractionGroups {
    /// Groups memberships.
    pub memberships: i32,
    /// Groups filter.
    pub filter: i32,
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
    pub fn apply_force(&mut self, force: PodVector3) {
        with_script_context(|ctx| {
            ctx.scene.as_mut().expect("scene unavailable").graph[self.handle]
                .as_rigid_body_mut()
                .apply_force(force.into())
        })
    }
}
