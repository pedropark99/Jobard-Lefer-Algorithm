use noise::{NoiseFn, Perlin, Seedable};
use array2d::{Array2D, Error};
use libm;


struct FlowField {
    field:Array2D<f64>,
    height:u8,
    width:u8
}

impl FlowField {
    pub fn new(seed:u8, field_width:u8, field_height:u8) -> FlowField {
        let noise_gen = Perlin::new(seed.into());
        let mut flow_field = Array2D::filled_with(
            0 as f64,
            field_height.into(),
            field_width.into()
        );

        let width_as_double = f64::from(field_width);
        for y in 0..field_height {
            for x in 0..field_width {
                let x_as_double = f64::from(x);
                let y_as_double = f64::from(y);
                let xp = x_as_double / width_as_double;
                let yp = y_as_double / width_as_double;
                flow_field[(x as usize, y as usize)] = noise_gen.get([xp, yp]);
            }
        }

        FlowField {
            field: flow_field,
            height: field_height,
            width: field_width
        }
    }

    pub fn get_angle(&self, x:f64, y:f64) -> f64 {
        let x = FlowField::get_flow_field_col(x);
        let y = FlowField::get_flow_field_row(y);
        self.field[(x as usize, y as usize)]
    }

    pub fn get_flow_field_col(x:f64) -> u8 {
        x as u8
    }

    pub fn get_flow_field_row(y:f64) -> u8 {
        y as u8
    }

    pub fn off_boundaries(&self, x:f64, y:f64) -> bool {
        x <= 0.0 ||
        y <= 0.0 ||
        x >= self.width.into() ||
        y >= self.height.into()
    }
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

// fn vector_with(size: u32) -> Vec<u32> {
//     let mut vecc: Vec<u32> = vec::with_capacity(size as usize);
//     vecc
// }


struct Curve {
    pub _curve_id:u32,
    pub _x:Vec<f64>,
    pub _y:Vec<f64>,
    pub _direction:Vec<u8>,
    pub _step_id:Vec<u32>,
    pub _steps_taken:u32
}

impl Curve {
    pub fn new(id:u32, n_steps:u32) -> Curve {
        Curve {
            _curve_id: id,
            _steps_taken: 0,
            _x: Vec::with_capacity(n_steps as usize),
            _y: Vec::with_capacity(n_steps as usize),
            _direction: Vec::with_capacity(n_steps as usize),
            _step_id: Vec::with_capacity(n_steps as usize)
        }
    }

    pub fn insert_step(&mut self, x_coord:f64, y_coord:f64, direction_id:u8) {
        self._x.push(x_coord);
        self._y.push(y_coord);
        self._direction.push(direction_id);
        self._step_id.push(self._steps_taken);
        self._steps_taken += 1;
    }

}

#[derive(Clone)]
struct DensityCell {
    x:Vec<f64>,
    y:Vec<f64>,
    capacity:u32,
    space_used:u32
}

pub fn empty_cell(cell_capacity:u32) -> DensityCell {
    DensityCell {
        x: Vec::with_capacity(cell_capacity as usize),
        y: Vec::with_capacity(cell_capacity as usize),
        capacity: cell_capacity,
        space_used: 0
    }
}

struct DensityGrid {
    _grid:Array2D<DensityCell>,
    _width:u8,
    _height:u8,
    _d_sep:f64
}


impl DensityGrid {
    pub fn new(d_sep:f64, width:u8, height:u8, cell_capacity:u32) -> DensityGrid {
        DensityGrid {
            _d_sep: d_sep,
            _width: width,
            _height: height,
            _grid: Array2D::fill_with(empty_cell(cell_capacity), width.into(), height.into()),
        }
    }


    pub fn get_density_col(&self, x:f64) -> usize {
        let c = x / self._d_sep;
        c as usize
    }

    pub fn get_density_row(&self, y:f64) -> usize {
        let r = y / self._d_sep;
        r as usize
    }

    pub fn off_boundaries(&self, x:f64, y:f64) -> bool {
        let c = self.get_density_col(x);
        let r = self.get_density_row(y);
        c <= 0 ||
        r <= 0 ||
        c >= self._width.into() ||
        r >= self._height.into()

    }

    pub fn insert_coord(&mut self, x:f64, y:f64) {
        if (self.off_boundaries(x, y)) {
            return;
        }

        let density_col = self.get_density_col(x);
        let density_row = self.get_density_row(y);

        let space_used = self._grid[(density_col, density_row)].space_used;
        let capacity = self._grid[(density_col, density_row)].capacity;

        if ((space_used + 1) < capacity) {
            self._grid[(density_col, density_row)].x.push(x);
            self._grid[(density_col, density_row)].y.push(y);
            self._grid[(density_col, density_row)].space_used += 1;
        } else {
            print!("[ERROR]: Attempt to add coordinate in density cell that is out of capacity!\n");
        }
    }

    fn insert_curve_coords(&mut self, curve:Curve){
        let steps_taken = curve._steps_taken;
        for i in 0..steps_taken {
            let aus = i as usize;
            self.insert_coord(curve._x[aus], curve._y[aus]);
        }
    }


    pub fn is_valid_next_step(&self, x:f64, y:f64) -> bool {
        if (self.off_boundaries(x, y)) {
            return false
        }

        let density_col = self.get_density_col(x);
        let density_row = self.get_density_row(y);

        let mut start_row = 0;
        let mut end_row = 0;
        let mut start_col = 0;
        let mut end_col = 0;
        if ((density_row - 1) > 0) {
            start_row =  density_row - 1;
        } else {
            start_row = 0;
        }

        if ((density_row + 1) < self._width.into()) {
            end_row = density_row + 1;
        } else {
            end_row = density_row; 
        }

        if ((density_col - 1) > 0) {	
            start_col = density_col - 1;
        } else {
            start_col = 0;
        }

        if ((density_col + 1) < self._height.into()) {
            end_col = density_col + 1;
        } else {
            end_col = density_col;
        }

        // Subtracting a very small amount from D_TEST, just to account for the lost of float precision
        // that happens during the calculations below, specially in the distance calc
        let d_test = self._d_sep - (0.01 * self._d_sep);
        for c in start_col..= end_col {
            for r in start_row..=end_row {
                let n_elements = self._grid[(c, r)].space_used;
                if (n_elements == 0) {
                    continue;
                }

                for i in 0..n_elements {
                    let x2 = self._grid[(c, r)].x[i as usize];
                    let y2 = self._grid[(c, r)].y[i as usize];
                    let dist = distance(x, y, x2, y2);
                    if (dist <= d_test) {
                        return false;
                    }
                }
            }
        }

        true
    }
}


struct Point {
    x:f64,
    y:f64,
}

struct SeedPointsQueue {
    _points:Vec<Point>,
    _capacity:u32,
    _space_used:u32,
}


impl SeedPointsQueue {
    pub fn new(n_steps:u32) -> SeedPointsQueue {
        let capacity = n_steps * 2;
        SeedPointsQueue {
            _capacity: capacity,
            _space_used: 0,
            _points: Vec::with_capacity(capacity as usize),
        }
    }

    pub fn is_empty(&self) -> bool {
        self._space_used == 0
    }

    pub fn insert_coord(&mut self, x:f64, y:f64) {
        let p = Point {x, y};
        self._points.push(p);
        self._space_used += 1;
    }

    pub fn insert_point(&mut self, p:Point) {
        self._points.push(p);
        self._space_used += 1;
    }
}


pub fn collect_seedpoints(curve:Curve, d_sep:f64) -> SeedPointsQueue {
    let steps_taken = curve._steps_taken;
    let m_pi = std::f64::consts::PI;
    let mut queue = SeedPointsQueue::new(steps_taken);
    if (steps_taken == 0) {
        return queue
    }

    for i in 0..(steps_taken - 1) {
        let aus = i as usize;
        let x = curve._x[aus];
        let y = curve._y[aus];

        let ff_column_index = x.floor() as u8;
        let ff_row_index = y.floor() as u8;
        let angle = libm::atan2(curve._y[aus + 1] - y, curve._x[aus + 1] - x);

        let angle_left = angle + (m_pi / 2.0);
        let angle_right = angle - (m_pi / 2.0);

        let left_point = Point {
            x: x + (d_sep * libm::cos(angle_left)),
            y: y + (d_sep * libm::sin(angle_left))
        };

        let right_point = Point {
            x: x + (d_sep * libm::cos(angle_right)),
            y: y + (d_sep * libm::sin(angle_right))
        };

        queue.insert_point(left_point);	
        queue.insert_point(right_point);	
    }

    queue
}



pub fn draw_curve(curve_id:u32,
    x_start:f64,
    y_start:f64,
    n_steps:u32,
    step_length:f64,
    flow_field:FlowField,
    density_grid:DensityGrid) -> Curve {

    let mut curve = Curve::new(curve_id, n_steps);
    curve.insert_step(x_start, y_start, 0);
    let mut x = x_start;
    let mut y = y_start;
    let mut i = 1;
    // Draw curve from right to left
    while i < (n_steps / 2) {
        if (flow_field.off_boundaries(x, y)) {
            break;
        }

        let angle = flow_field.get_angle(x, y);
        let x_step = step_length * libm::cos(angle);
        let y_step = step_length * libm::sin(angle);
        x = x - x_step;
        y = y - y_step;

        if (!density_grid.is_valid_next_step(x, y)) {
            break;
        }

        curve.insert_step(x, y, 0);
        i += 1;
    }

    x = x_start;
    y = y_start;
    // Draw curve from left to right
    while i < n_steps {
        if (flow_field.off_boundaries(x, y)) {
            break;
        }

        let angle = flow_field.get_angle(x, y);
        let x_step = step_length * libm::cos(angle);
        let y_step = step_length * libm::sin(angle);
        x = x + x_step;
        y = y + y_step;

        if (!density_grid.is_valid_next_step(x, y)) {
            break;
        }

        curve.insert_step(x, y, 1);
        i += 1;
    }

    curve
}



pub fn even_spaced_curves(x_start:f64,
    y_start:f64,
    n_curves:u32,
    n_steps:u32,
    min_steps_allowed:u8,
    step_length:f64,
    d_sep:f64,
    flow_field:FlowField,
    density_grid:DensityGrid) -> Vec<Curve> {

    let mut curves = Vec::with_capacity(n_curves as usize);
    let mut curve_array_index = 0;
    let mut curve_id = 0;
    let mut density_grid = density_grid;

    let x = x_start;
    let y = y_start;
    let curve = draw_curve(
        curve_id,
        x, y,
        n_steps,
        step_length,
        flow_field,
        density_grid
    );

    curves.push(curve);
    density_grid.insert_curve_coords(curve);
    curve_array_index += 1;


    while curve_id < n_curves && curve_array_index < n_curves {
        let mut queue = SeedPointsQueue::new(n_steps);
        if (curve_id >= curves.len() as u32) {
            // There is no more curves to be analyzed in the queue
            break;
        }
        let curve_usize = curve_id as usize;
        queue = collect_seedpoints(curves[curve_usize], d_sep);
        for p in queue._points {
            // check if it is valid given the current state
            if (density_grid.is_valid_next_step(p.x, p.y)) {
                // if it is, draw the curve from it
                let curve = draw_curve(
                    curve_array_index,
                    p.x, p.y,
                    n_steps,
                    step_length,
                    flow_field,
                    density_grid
                );

            if (curve._steps_taken < min_steps_allowed as u32) {
                    continue;
                }

                curves.push(curve);
                // insert this new curve into the density grid
                density_grid.insert_curve_coords(curve);
                curve_array_index += 1;
            }
        }

        curve_id += 1;
    }



    curves
}



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

    let flow_field = FlowField::new(50, flow_field_width, flow_field_height);




    println!("Hello, world!");
}


