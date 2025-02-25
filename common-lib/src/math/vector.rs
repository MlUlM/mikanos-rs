use core::cmp::{max, min};
use core::fmt::Debug;

mod add;
mod add_assign;
mod mul;
mod mul_assign;
mod partial_eq;
mod sub;

#[derive(Debug, Copy, Clone)]
pub struct Vector2D<T> {
    x: T,
    y: T,
}


impl<T: Copy> Vector2D<T> {
    #[inline(always)]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }


    #[inline(always)]
    pub const fn x(&self) -> T {
        self.x
    }


    #[inline(always)]
    pub const fn y(&self) -> T {
        self.y
    }
}


impl<T: PartialOrd> Vector2D<T> {
    #[inline(always)]
    pub fn is_over_x(&self, other: &Vector2D<T>) -> bool {
        self.x < other.x
    }


    #[inline(always)]
    pub fn is_over_y(&self, other: &Vector2D<T>) -> bool {
        self.y < other.y
    }


    #[inline(always)]
    pub fn is_over(&self, other: &Vector2D<T>) -> bool {
        self.is_over_x(other) || self.is_over_y(other)
    }
}


impl Vector2D<usize> {
    #[inline(always)]
    pub const fn unit() -> Self {
        Self::new(1, 1)
    }


    #[inline(always)]
    pub const fn zeros() -> Vector2D<usize> {
        Self::new(0, 0)
    }


    #[inline(always)]
    pub fn relative(&self, pos: Vector2D<usize>) -> Vector2D<isize> {
        let x = self.x() as isize - pos.x() as isize;
        let y = self.y() as isize - pos.y() as isize;

        Vector2D::new(x, y)
    }
}


impl Default for Vector2D<usize> {
    #[inline(always)]
    fn default() -> Self {
        Self::zeros()
    }
}


#[inline(always)]
pub fn min_vector2d<T: Ord + Copy>(lhs: &Vector2D<T>, rhs: &Vector2D<T>) -> Vector2D<T> {
    Vector2D::new(min(lhs.x(), rhs.x()), min(lhs.y(), rhs.y()))
}


#[inline(always)]
pub fn max_vector2d<T: Ord + Copy>(lhs: &Vector2D<T>, rhs: &Vector2D<T>) -> Vector2D<T> {
    Vector2D::new(max(lhs.x(), rhs.x()), max(lhs.y(), rhs.y()))
}


#[cfg(test)]
mod tests {
    use crate::math::vector::{max_vector2d, min_vector2d, Vector2D};

    #[test]
    fn it_relative_move_left() {
        let origin = Vector2D::<usize>::new(5, 5);
        let moved = Vector2D::<usize>::new(3, 5);

        let relative = moved.relative(origin);

        assert_eq!(relative, Vector2D::new(-2, 0));
    }


    #[test]
    fn it_min_vector2d() {
        let origin = Vector2D::<usize>::new(5, 5);
        let moved = Vector2D::<usize>::new(10, 0);

        assert_eq!(min_vector2d(&origin, &moved), Vector2D::new(5, 0));
    }


    #[test]
    fn it_max_vector2d() {
        let origin = Vector2D::<usize>::new(5, 5);
        let moved = Vector2D::<usize>::new(10, 0);

        assert_eq!(max_vector2d(&origin, &moved), Vector2D::new(10, 5));
    }
}
