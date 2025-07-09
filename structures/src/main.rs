// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct -> useful for generics
#[derive(Debug)]
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

// Struct sin Copy trait para demostrar borrowing
#[derive(Debug)]
struct PointNoCopy {
    x: f32,
    y: f32,
}

// Alternative: Rectangle que usa borrowing (pedir prestado) en lugar de ownership
struct RectangleWithBorrowedPoints<'a> {
    // Esta versión "pide prestado" los PointNoCopy en lugar de tomarlos
    top_left_r: &'a PointNoCopy,
    bottom_right_r: &'a PointNoCopy,
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point {
        x: 10.5,
        ..another_point
    };

    // `bottom_right.y` will be the same as `another_point.y` because we used that field
    // from `another_point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    println!(
        "This point contains the x values at {} and the y values at {}",
        left_edge, top_edge
    );

    // ================================
    // EJEMPLO CON COPY TRAIT
    // ================================

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right, // Con COPY trait -> se copia automáticamente
                                    // bottom_right sigue siendo válido
    };

    println!(
        "The top left corner is at ({}, {}) and the bottom right corner is at ({}, {})",
        left_edge, top_edge, bottom_right.x, bottom_right.y
    );

    // ================================
    // EJEMPLO CON BORROWING (sin Copy)
    // ================================

    // Usando el struct PointNoCopy que NO tiene Copy trait
    let punto_no_copy = PointNoCopy { x: 1.0, y: 2.0 };
    let otro_punto_no_copy = PointNoCopy { x: 3.0, y: 4.0 };

    // PASO 1: Desestructurar los puntos primero
    let PointNoCopy { x: x1, y: y1 } = punto_no_copy;
    let PointNoCopy { x: x2, y: y2 } = otro_punto_no_copy;

    println!(
        "Puntos desestructurados: ({}, {}) y ({}, {})",
        x1, y1, x2, y2
    );

    // PASO 2: Crear el rectangle usando las referencias originales
    let rectangle_con_borrowing = RectangleWithBorrowedPoints {
        top_left_r: &punto_no_copy,          // ← Borrowing: no se mueve el valor
        bottom_right_r: &otro_punto_no_copy, // ← Borrowing: no se mueve el valor
    };

    // PASO 3: Desestructurar el rectangle
    let RectangleWithBorrowedPoints {
        top_left_r,
        bottom_right_r,
    } = rectangle_con_borrowing;

    // Los valores originales siguen siendo válidos después del borrowing
    println!("Punto original después del borrowing: {:?}", punto_no_copy);
    println!(
        "Otro punto original después del borrowing: {:?}",
        otro_punto_no_copy
    );

    // PASO 4: Usar directamente en el println
    println!("Usando desestructuración completa:");
    println!("  top_left_r.x = {}", top_left_r.x); // ← ¡Directo!
    println!("  top_left_r.y = {}", top_left_r.y); // ← ¡Directo!
    println!("  bottom_right_r.x = {}", bottom_right_r.x); // ← ¡Directo!
    println!("  bottom_right_r.y = {}", bottom_right_r.y); // ← ¡Directo!

    println!("--------------------------------");
    println!("Instantiating a unit struct");
    // Instantiate a unit struct
    let _unit = Unit;

    // Print the unit struct
    println!("Unit struct: {:?}", _unit);
    println!("Unit struct (alternative): {:#?}", _unit);

    println!("--------------------------------");
    println!("Instantiating a tuple struct");
    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // ================================
    // RESUMEN DE DIFERENCIAS
    // ================================

    println!("\n=== RESUMEN DE DIFERENCIAS ===");
    println!("1. COPY TRAIT:");
    println!("   - Point se copia automáticamente");
    println!("   - Los valores originales siguen siendo válidos");
    println!("   - Más simple de usar");
    println!("   - Solo funciona con tipos pequeños y simples");

    println!("\n2. BORROWING:");
    println!("   - Se \"pide prestado\" el valor con &");
    println!("   - No se hace copia, solo se comparte la referencia");
    println!("   - Los valores originales siguen siendo válidos");
    println!("   - Funciona con cualquier tipo (incluso grandes y complejos)");
    println!("   - Requiere lifetimes ('a)");
}
