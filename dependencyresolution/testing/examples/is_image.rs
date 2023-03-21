
use analyser::analyse_image;


fn main() {

    let r  = analyse_image("tests/data/pepe1.jpg");
    println!("is_image: {:?}",r);
}
