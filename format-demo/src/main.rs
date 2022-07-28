fn main() {
    println!("{} days", 31);

    println!("{0},{1},{2},{1}", "a", "b", "c");

    println!("{x}+{y}={z}", x = "x", y = "y", z = "z");

    println!("base 10 repr: {}",69420);
    println!("base 2 (binary) repr: {:b}",69420);
    println!("base 8 (octal) repr:  {:o}",69420);
    println!("base 16 (hexadecimal) repr:   {:x}",69420);
    println!("base 16 (hexadecimal) repr:   {:X}",69420);

    println!("{number:>5}",number=1);
    println!("{number:0>5}",number=1);
    println!("{number:0>width$}", number=1, width=6);

    println!("my name is {0},{1} {0}", "bond","james");

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);
    println!("this struct `{:?}` won't print...", Structure(3));

    let number: f64 = 1.0;
    let width: usize = 6;
    println!("{number:>width$}");
}
