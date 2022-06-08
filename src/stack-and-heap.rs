fn main() {
    // stack-frame #1
    let x = 3;
    let y = successor(x);
    dbg!(y);
}

fn successor(x: u32) -> u32 {
    // stack-frame #2
    x + *one_heap()
    // deallocation #1
}

fn one_heap() -> Box<u32> {
    // stack-frame #3
    Box::new(1) // allocation #1
}