use std::cmp::Ordering;

use fyrox::{
    core::{
        algebra::{Point3, Vector3},
        pool::Handle,
    },
    scene::{
        collider::{BitMask, InteractionGroups}, graph::physics::{FeatureId, Intersection, QueryResultsStorage, RayCastOptions}, node::Node
    },
};
use fyrox_lite_macro::{fyrox_lite};

use crate::{lite_math::PodVector3, lite_node::LiteNode, script_context::with_script_context};

#[derive(Debug)]
pub struct LitePhysics;

#[fyrox_lite]
impl LitePhysics {
    /// Exclude from the query any collider attached to a fixed rigid-body and colliders with no rigid-body attached.
    const EXCLUDE_FIXED: i32 = 1 << 1;
    /// Exclude from the query any collider attached to a kinematic rigid-body.
    const EXCLUDE_KINEMATIC: i32 = 1 << 2;
    /// Exclude from the query any collider attached to a dynamic rigid-body.
    const EXCLUDE_DYNAMIC: i32 = 1 << 3;
    /// Exclude from the query any collider that is a sensor.
    const EXCLUDE_SENSORS: i32 = 1 << 4;
    /// Exclude from the query any collider that is not a sensor.
    const EXCLUDE_SOLIDS: i32 = 1 << 5;
    /// Excludes all colliders not attached to a dynamic rigid-body.
    const ONLY_DYNAMIC: i32 = LitePhysics::EXCLUDE_FIXED | LitePhysics::EXCLUDE_KINEMATIC;
    /// Excludes all colliders not attached to a kinematic rigid-body.
    const ONLY_KINEMATIC: i32 = LitePhysics::EXCLUDE_DYNAMIC | LitePhysics::EXCLUDE_FIXED;
    /// Exclude all colliders attached to a non-fixed rigid-body
    /// (this will not exclude colliders not attached to any rigid-body).
    const ONLY_FIXED: i32 = LitePhysics::EXCLUDE_DYNAMIC | LitePhysics::EXCLUDE_KINEMATIC;

    pub fn cast_ray(opts: LiteRayCastOptions) -> Vec<LiteIntersection>{
        with_script_context(|ctx| {
            let mut results = Vec::new();
            ctx.scene
                .as_mut()
                .expect("scene unavailable")
                .graph
                .physics
                .cast_ray(opts.into(), &mut QueryResultsStorageWrapper(&mut results));
            results
        })
    }
}

struct QueryResultsStorageWrapper<'a>(&'a mut Vec<LiteIntersection>);

impl <'a> QueryResultsStorage for QueryResultsStorageWrapper<'a> {
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
        self.0.sort_by(&mut |a: &LiteIntersection, b: &LiteIntersection| cmp(&a.into(), &b.into()))
    }
}
impl From<&LiteIntersection> for Intersection {
    fn from(value: &LiteIntersection) -> Self {
        Self {
            collider: value.collider.inner(),
            normal: value.normal.into(),
            position: Vector3::from(value.position).into(),
            feature: match value.feature {
                LiteFeatureId::Vertex(it) => FeatureId::Vertex(it as u32),
                LiteFeatureId::Edge(it) => FeatureId::Edge(it as u32),
                LiteFeatureId::Face(it) => FeatureId::Face(it as u32),
                LiteFeatureId::Unknown => FeatureId::Unknown,
            },
            toi: value.toi,
        }
    }
}

impl From<&Intersection> for LiteIntersection {
    fn from(value: &Intersection) -> Self {
        Self {
            collider: LiteNode::new(value.collider),
            normal: value.normal.into(),
            position: PodVector3::from(Vector3::new(value.position.x, value.position.y, value.position.z)),
            toi: value.toi,
            feature: match value.feature {
                FeatureId::Vertex(it) => LiteFeatureId::Vertex(it as i32),
                FeatureId::Edge(it) => LiteFeatureId::Edge(it as i32),
                FeatureId::Face(it) => LiteFeatureId::Face(it as i32),
                FeatureId::Unknown => LiteFeatureId::Unknown,
            },
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
#[fyrox_lite(Intersection)]
pub struct LiteIntersection {
    /// A handle of the collider with which intersection was detected.
    pub collider: LiteNode,

    /// A normal at the intersection position.
    pub normal: PodVector3,

    /// A position of the intersection in world coordinates.
    pub position: PodVector3,

    /// Additional data that contains a kind of the feature with which
    /// intersection was detected as well as its index.
    ///
    /// # Important notes.
    ///
    /// FeatureId::Face might have index that is greater than amount of triangles in
    /// a triangle mesh, this means that intersection was detected from "back" side of
    /// a face. To "fix" that index, simply subtract amount of triangles of a triangle
    /// mesh from the value.
    pub feature: LiteFeatureId,

    /// Distance from the ray origin.
    pub toi: f32,
}


/// Shape-dependent identifier.
#[derive(Copy, Hash, PartialEq, Eq)]
#[fyrox_lite(FeatureId)]
pub enum LiteFeatureId {
    /// Shape-dependent identifier of a vertex.
    Vertex(i32),
    /// Shape-dependent identifier of an edge.
    Edge(i32),
    /// Shape-dependent identifier of a face.
    Face(i32),
    /// Unknown identifier.
    Unknown,
}

#[fyrox_lite(RayCastOptions)]
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

#[derive(Copy, PartialEq, Eq, Default)]
#[fyrox_lite(InteractionGroups)]
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
            groups: InteractionGroups {
                memberships: BitMask(groups.memberships as u32),
                filter: BitMask(groups.filter as u32),
            },
            sort_results,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LiteRigidBody {
    pub handle: Handle<Node>,
}

#[fyrox_lite(RigidBody)]
impl LiteRigidBody {
    pub fn apply_force(&mut self, force: PodVector3) {
        with_script_context(|ctx| {
            ctx.scene.as_mut().expect("scene unavailable").graph[self.handle]
                .as_rigid_body_mut()
                .apply_force(force.into())
        })
    }
}
