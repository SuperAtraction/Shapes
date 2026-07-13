mod shape;
mod shapes;

use shape::Shape;
use shapes::*;
use std::io::{self, Write};

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
    let shape_number: i32 = read_number("Forme:") as i32;

    match shape_number {
        1 => {
            println!("Sélectionné: Carré. Veuillez taper la taille du côté");
            let side: f64 = read_number("Taille du côté:");
            let square = Square::new(side);
            println!("{} ajouté", square.info());
            shapes.push(Box::new(square));
        },
        2 => {
            println!("Sélectionné: Rectangle");

            let width = read_number("Longueur:");
            let height = read_number("Largeur:");

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
            println!("Sélectionné: Cercle");
            let radius: f64 = read_number("Rayon:");
            let circle = Circle::new(radius);
            println!("{} ajouté", circle.info());
            shapes.push(Box::new(circle));
        },
        4 => {
            println!("Sélectionné: Triangle");
            let side1 = read_number("Côté 1:");
            let side2 = read_number("Côté 2:");
            let side3 = read_number("Côté 3:");
            match Triangle::new(side1, side2, side3) {
                Ok(triangle) => {
                    println!("{} ajouté", triangle.info());
                    shapes.push(Box::new(triangle));
                }
                Err(error) => {
                    println!("{error}");
                }
            }
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

fn get_shape_index(shapes: &[Box<dyn Shape>], shape_id: i32) -> Option<usize> {
    let index = shapes.iter().position(|shape| shape.id() == shape_id);

    if index.is_none() {
        println!("Aucune forme avec l'identifiant {shape_id}.");
    }
    index
}

fn remove_shape(shapes: &mut Vec<Box<dyn Shape>>) {
    list_shapes(shapes);
    println!("\nQuelle forme souhaitez-vous supprimer ?");
    let shape_id = read_number("Tapez son identifiant:") as i32;
    if let Some(index) = get_shape_index(shapes, shape_id) {
        let shape = shapes.remove(index);
        println!("{}", shape.deleted_message());
    }
}

fn calculate(shapes: &mut Vec<Box<dyn Shape>>) {
    println!("\nQue souhaitez vous calculer?");
    println!("1. Additionner des formes 2. Calculer le périmètre");
    match read_number("Sélection:") as i32 {
        1 => {
            list_shapes(shapes);
            println!("\nTapez les IDs repectives des formes à additionner, séparés par un espace");
            let shape_list = read_string("IDs des formes:");
            let mut total_area: f64 = 0.0;
            for shape_id in shape_list.split_whitespace() {
                let shape_id = match shape_id.parse() {
                    Ok(val) => val,
                    Err(e) => {
                        println!("Erreur de parsing: {e}");
                        continue;
                    }
                };
                if let Some(index) = get_shape_index(shapes, shape_id) {
                    total_area += shapes[index].area();
                }
            }
            println!("Aire des formes: {}", total_area);
        },
        2 => {
            list_shapes(shapes);
            println!();
            let shape_list = read_string("Tapez le/les IDs de la/des forme(s):");
            let mut total_perimeter: f64 = 0.0;
            for shape_id in shape_list.split_whitespace() {
                let shape_id = match shape_id.parse() {
                    Ok(val) => val,
                    Err(e) => {
                        println!("Erreur de parsing: {e}");
                        continue;
                    }
                };
                if let Some(index) = get_shape_index(shapes, shape_id) {
                    total_perimeter += shapes[index].perimeter();
                }
            }
            println!("Le périmètre total des formes est de: {total_perimeter}");
        },
        _ => println!("Erreur de sélection"),
    }
}

fn main() {
    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();
    loop {
        println!("\n\nQue souhaitez vous faire ?");
        println!("1. Ajouter une forme; 2. Lister les formes; 3. Calculer l'aire totale; 4. Effectuer une opération; 5. Enlever une forme; 10. Quitter");
        match read_number("Sélection:") as i32 {
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

fn read_number(prompt: &str) -> f64 {
    print!("{prompt} ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse() {
        Ok(val) => val,
        Err(e) => {
            println!("Erreur de lecture! {e}");
            -1.1
        },
    }
}

fn read_string(prompt: &str) -> String {
    print!("{prompt} ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
