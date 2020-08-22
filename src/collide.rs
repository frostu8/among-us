use crate::math::*;

/// A projection is a segment on an axis, represented by two numbers.
///
/// This gaurentees that `start` is less than `end`.
pub struct Projection {
    start: FLOAT,
    end: FLOAT,
}

impl Projection {
    /// Create a new projection from a start and end value.
    pub fn new(mut start: FLOAT, mut end: FLOAT) -> Projection {
        if start > end {
            std::mem::swap(&mut start, &mut end);
        }

        Projection { start, end }
    }

    /// The start point.
    pub fn start(&self) -> FLOAT {
        self.start
    }

    /// The end point.
    pub fn end(&self) -> FLOAT {
        self.end
    }

    /// Set the start point.
    pub fn set_start(&mut self, start: FLOAT) {
        self.start = start;

        if self.start > self.end {
            std::mem::swap(&mut self.start, &mut self.end);
        }
    }

    /// Set the end point.
    pub fn set_end(&mut self, end: FLOAT) {
        self.end = end;

        if self.end < self.start {
            std::mem::swap(&mut self.start, &mut self.end);
        }
    }

    /// Check if there is a overlap between two projections.
    pub fn overlap(&self, other: &Projection) -> bool {
        !(self.start >= other.end || self.end <= other.start)
    }

    /// Check if `self` contains another projection.
    pub fn contains(&self, other: &Projection) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

/// Geometry is any shape that can collide.
///
/// Geometry in this context MUST BE CONVEX. Concave shapes will mess with the
/// collision and ruin your life.
///
/// Geometry does not mean that the shape can be translated, rotated or scaled.
pub trait Geometry: Sized {
    /// Project this geometry onto an axis.
    fn project(&self, axis: Vector2) -> Projection;

    /// Get the shape's vertices.
    fn vertices(&self) -> &[Vector2];

    /// Get the shape's axes.
    ///
    /// These vectors should be normalized.
    fn axis<T>(&self, other: &T) -> Vec<Vector2>
    where
        T: Geometry;

    /// Collide two objects together, returning true if they collide
    fn collide<T>(&self, other: &T) -> bool
    where
        T: Geometry,
    {
        self.axis(other).into_iter()
            .chain(other.axis(self).into_iter())
            .all(|p| self.project(p).overlap(&other.project(p)))
    }

    /// Contain one object within the other, returning true if the shape is
    /// contained within the container shape.
    fn contain<T>(&self, other: &T) -> bool
    where
        T: Geometry,
    {
        self.axis(other).into_iter()
            .chain(other.axis(self).into_iter())
            .all(|p| self.project(p).contains(&other.project(p)))
    }
}

/// A circle.
///
/// *Circles are geometry too!*
pub struct Circle {
    center: Vector2,
    radius: FLOAT,
}

impl Circle {
    /// Create a new circle.
    pub fn new(center: Vector2, radius: FLOAT) -> Circle {
        Circle { center, radius }
    }
}

impl Geometry for Circle {
    fn project(&self, axis: Vector2) -> Projection {
        let proj = axis.dot(self.center);

        Projection::new(proj - self.radius, proj + self.radius)
    }

    fn vertices(&self) -> &[Vector2] {
        std::slice::from_ref(&self.center)
    }

    fn axis<T>(&self, other: &T) -> Vec<Vector2>
    where
        T: Geometry,
    {
        other
            .vertices()
            .iter()
            .map(|v| (self.center - v).normalize())
            .collect()
    }
}

/// A closed polygon with `N` vertices.
#[derive(Clone)]
pub struct Polygon(Vec<Vector2>);

impl Polygon {
    /// Create a new polygon.
    pub fn new() -> Polygon {
        Polygon(Vec::new())
    }

    /// Push a vertex to the polygon.
    pub fn push(&mut self, vertex: Vector2) {
        self.0.push(vertex)
    }
}

impl From<Vec<Vector2>> for Polygon {
    fn from(vec: Vec<Vector2>) -> Polygon {
        Polygon(vec)
    }
}

impl Geometry for Polygon {
    fn project(&self, axis: Vector2) -> Projection {
        let mut iter = self.0.iter();

        let first = axis.dot(
            *iter
                .next()
                .expect("polygons with zero points are not supported"),
        );
        let mut proj = Projection::new(first, first);

        for v in iter {
            let p = axis.dot(*v);

            if p < proj.start() {
                proj.set_start(p);
            } else if p > proj.end() {
                proj.set_end(p);
            }
        }

        proj
    }

    fn vertices(&self) -> &[Vector2] {
        &self.0
    }

    fn axis<T>(&self, _other: &T) -> Vec<Vector2>
    where
        T: Geometry,
    {
        self.0
            .iter()
            .zip(self.0.iter().skip(1))
            .map(|v| {
                let edge = v.1 - v.0;
                Vector2::new(-edge.y, edge.x).normalize()
            })
            .collect()
    }
}
