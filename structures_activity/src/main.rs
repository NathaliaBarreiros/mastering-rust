// 1. Add a function rect_area which calculates the area of a Rectangle (try using nested destructuring).
// 2. Add a function square which takes a Point and a f32 as arguments, and returns a Rectangle with its top left corner on the point, and a width and height corresponding to the f32.
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)] // Added Debug derivation for Rectangle
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rectangle: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = rectangle;
    (x2 - x1) * (y1 - y2).abs()
}

fn square(top_left: Point, size: f32) -> Rectangle {
    let bottom_right = Point {
        x: top_left.x + size,
        y: top_left.y - size,
    };
    Rectangle {
        top_left,
        bottom_right,
    }
}

fn main() {
    let rect = Rectangle {
        top_left: Point { x: 0.0, y: 2.0 },
        bottom_right: Point { x: 3.0, y: 0.0 },
    };
    println!("Rectangle area: {}", rect_area(rect)); // Should print 6.0

    let sq = square(Point { x: 1.0, y: 2.0 }, 2.0);
    println!("Square: {:?}", sq); // Now works with Debug
}
