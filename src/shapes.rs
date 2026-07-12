use crate::shape::Shape;
use std::sync::atomic::{AtomicI32, Ordering};

// Pour les IDs
static NEXT_ID: AtomicI32 = AtomicI32::new(1);

fn generate_id() -> i32 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

pub struct Circle {
    pub radius: f64,
    id: i32,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn info(&self) -> String {
        format!("Cercle, avec un rayon de {}", self.radius)
    }

    fn id(&self) -> i32 {
        self.id
    }
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Self {
            radius,
            id: generate_id(),
        }
    }
}

pub struct Square {
    pub side: f64,
    id: i32,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }

    fn info(&self) -> String {
        format!("Carré, avec un côté de {}", self.side)
    }

    fn id(&self) -> i32 {
        self.id
    }
}

impl Square {
    pub fn new(side: f64) -> Self {
        Self {
            side,
            id: generate_id(),
        }
    }
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
    id: i32,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn info(&self) -> String {
        format!("Rectangle, avec une longueur de {} et une largeur de {}", self.width, self.height)
    }

    fn id(&self) -> i32 {
        self.id
    }
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            height,
            id: generate_id(),
        }
    }
}

pub enum TriangleType {
    Equilateral,
    Isosceles,
    Rectangle,
    RectangleIsosceles,
    Scalene,
}

pub struct Triangle {
    pub side1: f64,
    pub side2: f64,
    pub side3: f64,
    triangle_type: TriangleType,
    id: i32,
}

impl Triangle {
    pub fn new(side1: f64, side2: f64, side3: f64) -> Self {
        // Vérification qu'un triangle est possible
        assert!(
            side1 + side2 > side3
                && side1 + side3 > side2
                && side2 + side3 > side1,
            "Triangle impossible"
        );

        let mut sides = [side1, side2, side3];
        sides.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let a = sides[0];
        let b = sides[1];
        let c = sides[2];

        // Magie trouvée sur le web
        let rectangle = (a * a + b * b - c * c).abs() < 1e-10;
        let equilateral = a == b && b == c;
        let isosceles = a == b || b == c || a == c;

        let triangle_type = if equilateral {
            TriangleType::Equilateral
        } else if rectangle && isosceles {
            TriangleType::RectangleIsosceles
        } else if rectangle {
            TriangleType::Rectangle
        } else if isosceles {
            TriangleType::Isosceles
        } else {
            TriangleType::Scalene
        };

        Self {
            side1,
            side2,
            side3,
            triangle_type,
            id: generate_id(),
        }
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        let p = (self.side1 + self.side2 + self.side3) / 2.0;
        (p * (p - self.side1) * (p - self.side2) * (p - self.side3)).sqrt()
    }

    fn info(&self) -> String {
        let triangle_type = match self.triangle_type {
            TriangleType::Equilateral => "équilatéral",
            TriangleType::Isosceles => "isocèle",
            TriangleType::Rectangle => "rectangle",
            TriangleType::RectangleIsosceles => "rectangle isocèle",
            TriangleType::Scalene => "quelconque",
        };

        format!(
            "Triangle {}, avec des côtés de {}, {} et {}",
            triangle_type,
            self.side1,
            self.side2,
            self.side3
        )
    }

    fn id(&self) -> i32 {
        self.id
    }
}
