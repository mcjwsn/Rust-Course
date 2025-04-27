use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};

/// Struktura reprezentująca dwuwymiarowy wektor v = [x, y], gdzie x, y ∈ R
#[derive(Debug, Copy, Clone)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64,
}

impl Vec2D {
    /// Konstruktor tworzący nowy wektor o podanych współrzędnych
    pub fn new(x: f64, y: f64) -> Self {
        Vec2D { x, y }
    }

    /// Metoda fabryczna tworząca wektor jednostkowy o podanym kącie (w radianach)
    pub fn unit(angle: f64) -> Self {
        Vec2D {
            x: angle.cos(),
            y: angle.sin(),
        }
    }

    /// Oblicza długość (moduł) wektora
    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /// Oblicza iloczyn skalarny dwóch wektorów
    pub fn dot(&self, other: &Vec2D) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Sprawdza, czy dwa wektory są równe z zadaną precyzją
    pub fn equals(&self, other: &Vec2D, epsilon: f64) -> bool {
        (self.x - other.x).abs() < epsilon && (self.y - other.y).abs() < epsilon
    }
}

// Implementacja formatu wyświetlania dla struktury Vec2D
impl fmt::Display for Vec2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:.6}, {:.6}]", self.x, self.y)
    }
}

// Implementacja operatora równości dla struktury Vec2D
impl PartialEq for Vec2D {
    fn eq(&self, other: &Self) -> bool {
        // Domyślne porównanie z małą tolerancją na błędy zaokrągleń
        self.equals(other, 1e-10)
    }
}

// Implementacja operatora dodawania wektorów
impl Add for Vec2D {
    type Output = Self;

    fn add(self, other: Vec2D) -> Self {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Vec2D {x,y}
    }
}

// Implementacja operatora odejmowania wektorów
impl Sub for Vec2D {
    type Output = Self;

    fn sub(self, other: Vec2D) -> Self {
        Vec2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// Implementacja operatora mnożenia wektora przez skalar (Vec2D * f64)
impl Mul<f64> for Vec2D {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Vec2D {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

// Implementacja operatora mnożenia skalara przez wektor (f64 * Vec2D)
impl Mul<Vec2D> for f64 {
    type Output = Vec2D;

    fn mul(self, vector: Vec2D) -> Vec2D {
        Vec2D {
            x: self * vector.x,
            y: self * vector.y,
        }
    }
}

// Implementacja operatora dzielenia wektora przez skalar
impl Div<f64> for Vec2D {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        if scalar == 0.0 {
            panic!("Dzielenie przez zero!");
        }
        Vec2D {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

// Implementacja operatora negacji wektora
impl Neg for Vec2D {
    type Output = Self;

    fn neg(self) -> Self {
        Vec2D {
            x: -self.x,
            y: -self.y,
        }
    }
}

// Kod demonstracyjny działania operatorów na wektorach
fn main() {
    // Tworzenie wektorów
    let v1 = Vec2D::new(3.0, 4.0);
    let v2 = Vec2D::new(1.0, 2.0);
    let v3 = Vec2D::unit(std::f64::consts::PI / 4.0); // Wektor jednostkowy pod kątem 45 stopni

    println!("Wektor v1: {}", v1);
    println!("Wektor v2: {}", v2);
    println!("Wektor jednostkowy pod kątem 45 stopni: {}", v3);
    println!("Długość v1: {:.6}", v1.length());

    // Demonstracja operacji na wektorach
    let sum = v1 + v2;
    let diff = v1 - v2;
    let scaled1 = v1 * 2.0;
    let scaled2 = 3.0 * v2;
    let divided = v1 / 2.0;
    let negated = -v1;

    println!("\nOperacje na wektorach:");
    println!("v1 + v2 = {}", sum);
    println!("v1 - v2 = {}", diff);
    println!("v1 * 2.0 = {}", scaled1);
    println!("3.0 * v2 = {}", scaled2);
    println!("v1 / 2.0 = {}", divided);
    println!("-v1 = {}", negated);

    // Iloczyn skalarny
    let dot_product = v1.dot(&v2);
    println!("\nIloczyn skalarny v1·v2 = {:.6}", dot_product);

    // Kąt między wektorami
    let angle = (v1.dot(&v2) / (v1.length() * v2.length())).acos();
    println!("Kąt między v1 i v2: {:.6} rad ({:.2}°)", angle, angle * 180.0 / std::f64::consts::PI);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vec2D::new(3.0, 4.0);
        let v2 = Vec2D::new(1.0, 2.0);
        let result = v1 + v2;
        assert_eq!(result, Vec2D::new(4.0, 6.0));
    }

    #[test]
    fn test_sub() {
        let v1 = Vec2D::new(3.0, 4.0);
        let v2 = Vec2D::new(1.0, 2.0);
        let result = v1 - v2;
        assert_eq!(result, Vec2D::new(2.0, 2.0));
    }

    #[test]
    fn test_mul_scalar() {
        let v = Vec2D::new(3.0, 4.0);
        let result = v * 2.0;
        assert_eq!(result, Vec2D::new(6.0, 8.0));

        let result2 = 3.0 * v;
        assert_eq!(result2, Vec2D::new(9.0, 12.0));
    }

    #[test]
    fn test_div_scalar() {
        let v = Vec2D::new(6.0, 8.0);
        let result = v / 2.0;
        assert_eq!(result, Vec2D::new(3.0, 4.0));
    }

    #[test]
    #[should_panic(expected = "Dzielenie przez zero!")]
    fn test_div_by_zero() {
        let v = Vec2D::new(3.0, 4.0);
        let _result = v / 0.0;
    }

    #[test]
    fn test_neg() {
        let v = Vec2D::new(3.0, 4.0);
        let result = -v;
        assert_eq!(result, Vec2D::new(-3.0, -4.0));
    }

    #[test]
    fn test_dot_product() {
        let v1 = Vec2D::new(3.0, 4.0);
        let v2 = Vec2D::new(1.0, 2.0);
        assert_eq!(v1.dot(&v2), 11.0);

        // Wektory prostopadłe powinny mieć iloczyn skalarny równy zero
        let v3 = Vec2D::new(1.0, 0.0);
        let v4 = Vec2D::new(0.0, 1.0);
        assert_eq!(v3.dot(&v4), 0.0);
    }

    #[test]
    fn test_unit_vector() {
        let v = Vec2D::unit(0.0);
        assert!((v.length() - 1.0).abs() < 1e-10);

        let v2 = Vec2D::unit(std::f64::consts::PI / 4.0);
        assert!((v2.length() - 1.0).abs() < 1e-10);
        assert!((v2.x - 1.0 / 2.0_f64.sqrt()).abs() < 1e-10);
        assert!((v2.y - 1.0 / 2.0_f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_complex_operations() {
        let v1 = Vec2D::new(3.0, 4.0);
        let v2 = Vec2D::new(1.0, 2.0);

        // (v1 + v2) * 2.0 = (3+1, 4+2) * 2 = (8, 12)
        let result = (v1 + v2) * 2.0;
        assert_eq!(result, Vec2D::new(8.0, 12.0));

        // v1 * 2.0 - v2 / 0.5 = (6, 8) - (2, 4) = (4, 4)
        let result2 = v1 * 2.0 - v2 / 0.5;
        assert_eq!(result2, Vec2D::new(4.0, 4.0));
    }
}