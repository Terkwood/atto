use atto::parse;

fn main() {
    let prog = parse::code(r#"
        def ['' |items| |t|
            __cat __wrap items t

        def ,'' |x| |y|
            __cat __wrap x y

        def ]
            let foo 5
            let |bar foobar| [1, 3]
            __tail __wrap null

        def test
            [1, 2, 3, 4]
    "#).unwrap();

    println!("{:?}", prog);
}
