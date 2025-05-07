#[derive(Debug)]
pub struct Vector2d{
    x: f32,
    y: f32
}

impl Vector2d{
    pub fn  new(x: f32, y: f32) -> Vector2d{
        Vector2d{x,y}
    }

    pub fn write(&self){
        println!("Vector x: {}, y: {}", self.x,self.y);
    }

    pub fn fabricate(&self) -> Vector2d{
        let module:f32 = (self.x.powi(2) + self.y.powi(2)).powf(0.5);
        Vector2d{x: self.x/module,y:self.y/module}
    }

    pub fn compare(&self,other: &Vector2d) -> bool{
        self.x == other.x && self.y == other.y
    }
}

use std::ops::{Add, Sub};

impl Add for Vector2d {
    type Output = Self; // Output describes the resulting type after applying the + operator.
    // Self describes the current type, here it means Vec2D

    fn add(self, other: Vector2d) -> Self {
        Self{x: self.x + other.x ,
        y : self.y + other.y}
    }
}

impl Sub for Vector2d {
    type Output = Self;

    fn sub(self, other: Vector2d) -> Self {
        Self{
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let v = Vector2d::new(3.0, 4.0);
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 4.0);
    }

    #[test]
    fn test_vector_addition() {
        let v1 = Vector2d::new(1.0, 2.0);
        let v2 = Vector2d::new(3.0, 4.0);
        let result = v1 + v2;
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 6.0);
    }

    #[test]
    fn test_vector_subtraction() {
        let v1 = Vector2d::new(5.0, 7.0);
        let v2 = Vector2d::new(2.0, 3.0);
        let result = v1 - v2;
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 4.0);
    }

    #[test]
    fn test_vector_comparison_equal() {
        let v1 = Vector2d::new(1.0, 1.0);
        let v2 = Vector2d::new(1.0, 1.0);
        assert!(v1.compare(&v2));
    }

    #[test]
    fn test_vector_comparison_not_equal() {
        let v1 = Vector2d::new(1.0, 1.0);
        let v2 = Vector2d::new(2.0, 1.0);
        assert!(!v1.compare(&v2));
    }

    #[test]
    fn test_vector_fabricate() {
        let v = Vector2d::new(3.0, 4.0);
        let normalized = v.fabricate();
        let expected = Vector2d::new(0.6, 0.8);
        // Używamy przybliżonego porównania ze względu na niedokładności f32
        let epsilon = 1e-5;
        assert!((normalized.x - expected.x).abs() < epsilon);
        assert!((normalized.y - expected.y).abs() < epsilon);
    }
}