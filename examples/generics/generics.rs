// Конкретный тип `A`.
struct A;

// В определении типа `Single` первому использованию `A` не предшествует `<A>`.
// Поэтому `Single` имеет конкретный тип, и `A` определена выше.
struct Single(A);
//            ^ Здесь `A` в первый раз используется в `Single`.

// В данном примере, `<T>` предшествует первому использованию `T`,
// поэтому `SingleGen` является обобщённым типом.
// Поскольку тип параметра `T` является обобщённым, он может быть чем угодно, включая
// конкретный тип `A`, определённый выше.
struct SingleGen<T>(T);

fn main() {
    // `Single` имеет конкретный тип и явно принимает параметр `A`.
    let _s = Single(A);

    // Создаём переменную `_char` типа `SingleGen<char>`
    // и присваиваем ей значение `SingleGen('a')`.
    // В примере ниже, тип параметра `SingleGen` явно определён.
    let _char: SingleGen<char> = SingleGen('a');

    // Здесь, `SingleGen` также может иметь неявно определённый параметр типа:
    let _t    = SingleGen(A); // Используется структура `A`, объявленная выше.
    let _i32  = SingleGen(6); // Используется `i32`.
    let _char = SingleGen('a'); // Используется `char`.
}
