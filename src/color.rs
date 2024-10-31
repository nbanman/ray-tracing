use std::{fs::File, io::{ BufWriter, Write }};

use crate::prelude::*;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color(writer: &mut BufWriter<File>, pixel_color: &Color) -> std::io::Result<()> {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);
    
    // Translate the [0,1] component values to the byte range [0,255].
    let intensity = Interval::new(0.0, 0.999);
    let rbyte = (256.0 * intensity.clamp(r)) as u32;
    let gbyte = (256.0 * intensity.clamp(g)) as u32;
    let bbyte = (256.0 * intensity.clamp(b)) as u32;

    // Write out the pixel color components.
    writeln!(writer, "{rbyte} {gbyte} {bbyte}")?;
    Ok(())
}