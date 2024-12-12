use std::hint::black_box;

mod day11;

fn main() {
    let r = day11::part1("125 17");
    // for _ in 0..3_000 {
    //     let r = day11::part1(black_box("92 0 286041 8034 34394 795 8 2051489"));
    //     black_box(r);
    // }
    // let r = day11::part1("92 0 286041 8034 34394 795 8 2051489");
    // let r = day11::part1("1");
    // let r = day11::part1("125 17");
    // let r = day11::part1("0 1 10 99 999");
    println!("RES: {r}");
}
