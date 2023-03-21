#[cfg(feature = "dep_new")]
extern crate dep_new as dep;
#[cfg(feature = "dep_old")]
extern crate dep_old as dep;


use dep::image as image;


use image::io::DynamicImage;
use image::io::Reader;
use std::error::Error;

pub fn analyse_image(name: &str) -> Result<(), Box<dyn Error>> {
    println!("opening: {}", name);
    let r_open = Reader::open(name)?;
    let r_decoded: DynamicImage = r_open.decode()?;

    if r_decoded.height() > 0 {
        return Ok(());
    }
    Result::Err("no height")?
}

#[cfg(test)]
mod tests {
    // use super::*;
    // #[test]    fn it_works() {        let result = add(2, 2);        assert_eq!(result, 4);    }
}
