// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100; 
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
    //可变引用和不可变引用只能同时存在一种
    //不可变引用可以同时有多个
    //可变引用只能同时有一个
}
