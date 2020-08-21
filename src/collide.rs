use crate::math::*;

/// A projection is a segment on an axis, represented by two numbers.
///
/// This gaurentees that `start` is less than `end`.
pub struct Projection {
    start: FLOAT,
    end:   FLOAT,
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
}

/// Geometry is any shape that can collide.
///
/// Geometry in this context MUST BE CONVEX. Concave shapes will mess with the
/// collision and ruin your life.
///
/// Geometry does not mean that the shape can be translated, rotated or scaled.
pub trait Geometry {
    /// Project this geometry onto an axis.
    fn project(&self, axis: Vector2) -> Projection;

    /// Get the shape's vertices.
    fn vertices(&self) -> &[Vector2];

    /// Get the shape's axes.
    ///
    /// These vectors should be normalized.
    fn axis<T>(&self, other: &T) -> Vec<Vector2>
    where T: Geometry;
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
    where T: Geometry {
        other.vertices().iter().map(|v| (self.center - v).normalize()).collect()
    }
}

/// A closed polygon with `N` vertices.
pub struct Polygon<const N: usize>([Vector2; N]);

impl<const N: usize> Geometry for Polygon<N> {
    fn project(&self, axis: Vector2) -> Projection {
        let first = axis.dot(self.0[0]);
        let mut proj = Projection::new(first, first);

        for v in self.0[1..].iter() {
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
    where T: Geometry {
        let mut vec = Vec::<Vector2>::with_capacity(self.0.len());

        for i in 0..self.0.len() {
            let this = self.0[i];
            let next = self.0[(i + 1) % self.0.len()];

            let edge = next - this;
            let normal = Vector2::new(-edge.y, edge.x);

            vec.push(normal.normalize());
        }

        vec
    }
}
