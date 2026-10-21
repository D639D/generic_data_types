struct Point<T> {
    // Один обобщённый вид
    x: T,
    y: T,
}

struct Point2<T, U> {
    // Два обобщённых вида
    x: T,
    y: U,
}

struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<T> Point<T> {
    // Метод с обобщённым типом
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    // Тип Point<f32> будет иметь метод с именем distance_from_origin
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}

fn largest<T>(list: &[T]) -> &T {
    // Один обобщённый вид для функции
    let mut largest = &list[0];

    for item in list {
        // Код не компилируетя так как нужно нужно сравнивать только те значения
        // ..которые можно упорядочить
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    // ------------------------------ //

    let integer = Point { x: 5, y: 10 }; // <- Один обобщённый вид
    let float = Point { x: 12.3, y: 3.1 }; // <- Один обобщённый вид

    // let wont_work = Point { x: 5, y: 4.0 }; <-- Так код не скомпилируется
    let integer_and_float = Point2 { x: 5, y: 4.0 }; // <- А так код скомпилируется

    // ----------------------------- //

    let p = Point { x: 5, y: 10 };
    println!("p.x - {}", p.x());

    // ----------------------------- //

    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
