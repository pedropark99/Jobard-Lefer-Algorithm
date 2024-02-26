use noise::{NoiseFn, Perlin, Seedable};
use array2d::{Array2D, Error};

fn main() {

    let flow_field_width:u8 = 120;
    let flow_field_height:u8 = 120;
    let n_steps:u8 = 30;
    let min_steps_allowed:u8 = 5;
    let n_curves:u32 = 1500;
    let step_length:f64 = 0.01 * f64::from(flow_field_width);
    let d_sep:f64 = 0.8;
    let density_grid_width = calc_density_dim(flow_field_width, d_sep);
    let density_grid_height = calc_density_dim(flow_field_height, d_sep);

    let noise_gen = Perlin::new(50);
    let mut flow_field = Array2D::filled_with(
        0 as f64,
        flow_field_height.into(),
        flow_field_width.into()
    );

    for y in 0..flow_field_height {
        for x in 0..flow_field_width {
            let xa = f64::from(x / flow_field_width);
            lef ya = f64::from(y / flow_field_height);
            println!("x:{} | y:{} | v:{}", x, y, noise_gen.get([xa, ya]));
            // flow_field[(x as usize, y as usize)] = noise_gen.get([x as f64, y as f64]);
        }
    }

    // for row_iter in flow_field.row_iter(0) {
    //     for element in row_iter {
    //         println!("{}", element);
    //     }
    // }


    println!("Hello, world!");
}


fn calc_density_dim(x:u8, d_sep:f64) -> u8 {
    let as_float = f64::from(x);
    let mut div = as_float / d_sep;
    div = div.floor();
    let as_int = div as u8;
    as_int 
}

fn distance(x1:f64, y1:f64, x2:f64, y2:f64) -> f64 {
	let s1 = (x2 - x1).powi(2);
	let s2 = (y2 - y1).powi(2);
	let result = (s1 + s2).sqrt();
    result
}
