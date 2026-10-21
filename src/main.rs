struct Point<T> {
    // Один обобщённый вид
    x: T,
    y: T,
}

struct Point_2<T, U> {
    // Два обобщённых вида
    x: T,
    y: U,
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
    let integer_and_float = Point_2 { x: 5, y: 4.0 }; // <- А так код скомпилируется
}
