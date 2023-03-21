#[cfg(feature = "a_new")]
extern crate dep_new as dep;

#[cfg(feature = "a_old")]
extern crate dep_old as dep;

use dep::image::DynamicImage;
use dep::image::io::Reader;
use std::error::Error;

pub fn analyse_image(name: &str) -> Result<(), Box<dyn Error>> {
    let r_open = Reader::open(name)?;
    let r_decoded: DynamicImage = r_open.decode()?;
    if r_decoded.height() > 0 {
        return Ok(());
    }
    Result::Err("no height")?
}
