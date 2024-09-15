use std::cmp::Ordering;

use fyrox::{
    core::algebra::{Point3, Vector3},
    scene::graph::physics::{FeatureId, Intersection, QueryResultsStorage, RayCastOptions},
};

use crate::{lite_node::LiteNode, script_context::with_script_context};

#[derive(Debug)]
pub struct LitePhysics;

impl LitePhysics {
    pub fn cast_ray(opts: RayCastOptions, results: &mut Vec<LiteIntersection>) {
        with_script_context(|ctx| {
            ctx.scene.graph.physics.cast_ray(opts, &mut QueryResultsStorageWrapper(results));
        });
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
        self.0.sort_by(|a, b| cmp(&a.into(), &b.into()))
    }
}

impl From<&LiteIntersection> for Intersection {
    fn from(value: &LiteIntersection) -> Self {
        Self {
            collider: value.collider.into(),
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
            collider: value.collider.into(),
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
