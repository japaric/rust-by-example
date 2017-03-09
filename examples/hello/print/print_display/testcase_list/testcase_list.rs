use std::fmt; // Импортируем модуль `fmt`.

// Определим структуру с именим `List`, которая хранит в себе `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Разыменуем `self` и создадим связь с `vec`
        // с помощью ссылки.
        let List(ref vec) = *self;

        try!(write!(f, "["));

        // Пройдемся по каждому `v` в `vec`.
        // Номер итерации хранится в `count`.
        for (count, v) in vec.iter().enumerate() {
            // Для каждого элемента, кроме первого, добавим запятую
            // до вызова `write!`. Используем `try!`, чтобы вернуться при наличие ошибок.
            if count != 0 { try!(write!(f, ", ")); }
            try!(write!(f, "{}", v));
        }

        // Закроем открытую скобку и вернем значение `fmt::Result` 
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
