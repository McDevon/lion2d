mod dmath;
use dmath::fix::Fix;

fn main() {
    let one = Fix::ONE;
    println!("one is {}", one);
    dmath::fix::testfun();
}