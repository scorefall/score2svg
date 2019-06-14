use score2svg::*;

fn main() {
    let string = test_svg(DEFAULT);

    std::fs::write("test.svg", string).unwrap(); // _string
}
