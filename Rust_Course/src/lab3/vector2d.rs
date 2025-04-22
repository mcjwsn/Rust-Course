use std::fmt;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_vector() {
        let v = Vec2D::new(3.0, 4.0);
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 4.0);
    }

    #[test]
    fn test_length() {
        let v = Vec2D::new(3.0, 4.0);
        assert_eq!(v.length(), 5.0);
    }

    #[test]
    fn test_unit_vector() {
        let v = Vec2D::unit(0.0);
        assert!((v.x - 1.0).abs() < 1e-10);
        assert!(v.y.abs() < 1e-10);
        assert!((v.length() - 1.0).abs() < 1e-10);

        let v2 = Vec2D::unit(std::f64::consts::PI / 2.0);
        assert!(v2.x.abs() < 1e-10);
        assert!((v2.y - 1.0).abs() < 1e-10);
        assert!((v2.length() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_display() {
        let v = Vec2D::new(1.23456789, 2.34567891);
        assert_eq!(format!("{}", v), "[1.234568, 2.345679]");
    }

    #[test]
    fn test_equals() {
        let v1 = Vec2D::new(1.0, 2.0);
        let v2 = Vec2D::new(1.0, 2.0);
        let v3 = Vec2D::new(1.0000001, 2.0);

        assert!(v1.equals(&v2, 1e-10));
        assert!(!v1.equals(&v3, 1e-10));
        assert!(v1.equals(&v3, 1e-6));
    }

    #[test]
    fn test_eq_operator() {
        let v1 = Vec2D::new(1.0, 2.0);
        let v2 = Vec2D::new(1.0, 2.0);
        let v3 = Vec2D::new(1.1, 2.0);

        assert_eq!(v1, v2);
        assert_ne!(v1, v3);
    }
}