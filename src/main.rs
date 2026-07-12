mod shape;
mod shapes;

use shape::Shape;
use shapes::*;
use std::io;

fn total_area(shapes: &Vec<Box<dyn Shape>>) -> f64 {
    let mut total_area: f64 = 0.0;
    for shape in shapes {
        total_area += shape.area();
    }
    total_area
}

fn add_shape(shapes: &mut Vec<Box<dyn Shape>>)  {
    println!();
    println!("=== Ajout de forme ===");
    println!("1. Carré; 2. Rectangle; 3. Cercle; 4. Triangle");
    let shape_number: i32 = read_number() as i32;
    match shape_number {
        1 => {
            println!("Sélectionné: Carré. Veuillez taper la taille du côté");
            let side: f64 = read_number();
            let square = Square::new(side);
            println!("{} ajouté", square.info());
            shapes.push(Box::new(square));
        },
        2 => {
            println!("Sélectionné: Rectangle. Veuillez taper la longueur puis la largeur");

            let width = read_number();
            let height = read_number();

            if width == height {
                println!("Les valeurs sont identiques, création d'un carré");

                let square = Square::new(width);
                println!("{} ajouté", square.info());
                shapes.push(Box::new(square));
            } else {
                let rectangle = Rectangle::new(width, height);
                println!("{} ajouté", rectangle.info());
                shapes.push(Box::new(rectangle));
            }
        },
        3 => {
            println!("Sélectionné: Cercle. Veuillez taper le rayon");
            let radius: f64 = read_number();
            let circle = Circle::new(radius);
            println!("{} ajouté", circle.info());
            shapes.push(Box::new(circle));
        },
        4 => {
            println!("Sélectionné: Triangle. Veuillez taper la taille des trois côtés");
            let side1 = read_number();
            let side2 = read_number();
            let side3 = read_number();
            let triangle = Triangle::new(side1, side2, side3);
            println!("{} ajouté", triangle.info());
            shapes.push(Box::new(triangle));
        }
        _ => println!("Erreur de sélection"),
    }
}

fn list_shapes(shapes: &Vec<Box<dyn Shape>>) {
    println!("\n=== Liste des formes ===");
    if shapes.is_empty() {
        println!("C'est bien vide ici.");
        return;
    }
    for shape in shapes {
        println!("{}- {} - Aire : {:.2}", shape.id(), shape.info(), shape.area());
    }
}

fn get_shape_index(shapes: &Vec<Box<dyn Shape>>, shape_id: i32) -> usize {
    if let Some(index) = shapes.iter().position(|shape| shape.id() == shape_id) {
        return index;
    } else {
        println!("Aucune forme avec cet identifiant.");
        return 0;
    }
}

fn remove_shape(shapes: &mut Vec<Box<dyn Shape>>) {
    list_shapes(shapes);

    println!("\nQuelle forme souhaitez-vous supprimer ? Tapez son identifiant :");
    let shape_id = read_number() as i32;

    let shape = shapes.remove(get_shape_index(&shapes, shape_id));
    println!("{}", shape.deleted_message());
}

fn calculate(shapes: &mut Vec<Box<dyn Shape>>) {
    println!("\nQue souhaitez vous calculer?");
    println!("1. Additionner des formes");
    match read_number() as i32 {
        1 => {
            list_shapes(shapes);
            println!("Tapez le nombre de forme à additionner, puis leur IDs repectives, ligne par ligne");
            let shape_number = read_number() as i32;
            let mut total_area: f64 = 0.0;
            for _ in 0..shape_number {
                let shape_id = read_number() as i32;
                let shape_index = get_shape_index(shapes, shape_id);
                total_area += shapes[shape_index].area();
            }
            println!("Aire des formes: {}", total_area);
        },
        _ => println!("Erreur de sélection"),
    }
}

fn main() {
    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();
    loop {
        println!("\n\nQue souhaitez vous faire ?");
        println!("1. Ajouter une forme; 2. Lister les formes; 3. Calculer l'aire totale; 4. Effectuer une opération; 5. Enlever une forme; 10. Quitter");
        match read_number() as i32 {
            1 => add_shape(&mut shapes),
            2 => list_shapes(&shapes),
            3 => println!("\nAire totale: {}", total_area(&shapes)),
            4 => calculate(&mut shapes),
            5 => remove_shape(&mut shapes),
            10 => break,
            _ => println!("Erreur de sélection"),
        }
    }

    println!("Merci et à bientôt");
}

fn read_number() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or(0);
    input.trim().parse().unwrap_or(0.0)
}
