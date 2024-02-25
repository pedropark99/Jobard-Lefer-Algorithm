// C Math Library
#include <cwchar>
#include <math.h>

// C++ STD Libraries
#include <vector>
#include <iostream>

#define FNL_IMPL
#include "FastNoiseLite.h"

#define FLOW_FIELD_WIDTH 120
#define FLOW_FIELD_HEIGHT 120
#define N_STEPS 30
#define MIN_STEPS_ALLOWED 5
#define N_CURVES 1500
#define STEP_LENGTH 0.01 * FLOW_FIELD_WIDTH
#define D_SEP 0.8
#define DENSITY_GRID_WIDTH ((int) (FLOW_FIELD_WIDTH / D_SEP))
#define DENSITY_GRID_HEIGHT ((int) (FLOW_FIELD_HEIGHT / D_SEP))





double distance (double x1, double y1, double x2, double y2) {
	double s1 = pow(x2 - x1, 2.0);
	double s2 = pow(y2 - y1, 2.0);
	return sqrt(s1 + s2);
}



class FlowField {
private:
	double _flow_field[FLOW_FIELD_WIDTH][FLOW_FIELD_HEIGHT];

public:
	FlowField () {
		// Create and configure noise state
		fnl_state noise = fnlCreateState();
		noise.seed = 50;
		noise.noise_type = FNL_NOISE_PERLIN;
		int index = 0;
		for (int y = 0; y < FLOW_FIELD_HEIGHT; y++) {
			for (int x = 0; x < FLOW_FIELD_WIDTH; x++) {
				_flow_field[x][y] = fnlGetNoise2D(&noise, x, y) * 2 * M_PI;
			}
		}
	}

	int get_flow_field_col(double x) {
		return (int) x;
	}

	int get_flow_field_row(double y) {
		return (int) y;
	}

	bool off_boundaries(double x, double y) {
		return (
			x <= 0 ||
			y <= 0 ||
			x >= FLOW_FIELD_WIDTH ||
			y >= FLOW_FIELD_HEIGHT
		);
	}

	double get_angle(double x, double y) {
		int xi = get_flow_field_col(x);
		int yi = get_flow_field_row(y);
		return _flow_field[xi][yi];
	}
};


struct Point {
	double x;
	double y;
};


class Curve {
public:
	int _curve_id;
	std::vector<double> _x;
	std::vector<double> _y;
	std::vector<int> _direction;
	std::vector<int> _step_id;
	int _steps_taken;
public:
	Curve(int id, int n_steps) {
		_curve_id = id;
		_steps_taken = 0;
		_x.reserve(n_steps);
		_y.reserve(n_steps);
		_direction.reserve(n_steps);
		_step_id.reserve(n_steps);
	}

	void insert_step(double x_coord, double y_coord, int direction_id) {
		_x[_steps_taken] = x_coord;
		_y[_steps_taken] = y_coord;
		_direction[_steps_taken] = direction_id;
		_step_id[_steps_taken] = _steps_taken;
		_steps_taken++;
	}
};


struct DensityCell {
	std::vector<double> x;
	std::vector<double> y;
	int capacity;
	int space_used;
};


class DensityGrid {
private:
	DensityCell _grid[DENSITY_GRID_WIDTH][DENSITY_GRID_HEIGHT];
	int _width;
	int _height;
	double _d_sep;
public:
	DensityGrid(double d_sep, int cell_capacity) {
		_d_sep = d_sep;
		_width = DENSITY_GRID_WIDTH;
		_height = DENSITY_GRID_HEIGHT;
		for (int r = 0; r < _height; r++) {
			for (int c = 0; c < _width; c++) {
				_grid[c][r].x.reserve(cell_capacity);
				_grid[c][r].y.reserve(cell_capacity);
				_grid[c][r].capacity = cell_capacity;
				_grid[c][r].space_used = 0;
			}
		}
	}

	int get_density_col (double x) {
		double c = (x / _d_sep) + 1;
		return (int) c;
	}

	int get_density_row (double y) {
		double r = (y / _d_sep) + 1;
		return (int) r;
	}

	bool off_boundaries(double x, double y) {
		int c = get_density_col(x);
		int r = get_density_row(y);
		return (
			x <= 0 ||
			y <= 0 ||
			x >= _width ||
			y >= _height
		);
	}

	void insert_coord(double x, double y) {
		if (off_boundaries(x, y)) {
			return;
		}

		int density_col = get_density_col(x);
		int density_row = get_density_row(y);

		int space_used = _grid[density_col][density_row].space_used;
		int capacity = _grid[density_col][density_row].capacity;

		if ((space_used + 1) < capacity) {
			_grid[density_col][density_row].x[space_used] = x;
			_grid[density_col][density_row].y[space_used] = y;
			_grid[density_col][density_row].space_used++;
		} else {
			std::cout << "[ERROR]: Attempt to add coordinate in density cell that is out of capacity!" << std::endl;
		}
	}

	void insert_curve_coords(Curve* curve) {
		int steps_taken = curve->_steps_taken;
		for (int i = 0; i < steps_taken; i++) {
			insert_coord(curve->_x[i], curve->_y[i]);
		}
	}

	bool is_valid_next_step(double x, double y) {
		if (off_boundaries(x, y)) {
			return 0;
		}

		int density_col = get_density_col(x);
		int density_row = get_density_row(y);

		int start_row = (density_row - 1) > 0 ? density_row - 1 : 0;
		int end_row = (density_row + 1) < _width ? density_row + 1 : density_row; 
		int start_col = (density_col - 1) > 0 ? density_col - 1 : 0;
		int end_col = (density_col + 1) < _height ? density_col + 1 : density_col;

		// Subtracting a very small amount from D_TEST, just to account for the lost of float precision
		// that happens during the calculations below, specially in the distance calc
		double d_test = _d_sep - (0.01 * _d_sep);
		for (int c = start_col; c <= end_col; c++) {
			for (int r = start_row; r <= end_row; r++) {
				int n_elements = _grid[c][r].space_used;
				if (n_elements == 0) {
					continue;
				}

				for (int i = 0; i < n_elements; i++) {
					double x2 = _grid[c][r].x[i];
					double y2 = _grid[c][r].y[i];
					double dist = distance(x, y, x2, y2);
					if (dist <= d_test) {
						return 0;
					}
				}
			}
		}

		return 1;
	}
};


class SeedPointsQueue {
public:
	std::vector<Point> _points;
	int _capacity;
	int _space_used;

public:
	SeedPointsQueue(int n_steps) {
		_capacity = n_steps * 2;
		_space_used = 0;
		_points.reserve(n_steps * 2);
	}

	bool is_empty() {
		return _space_used == 0;
	}

	void insert_coord(double x, double y) {
		Point p = {x, y};
		_points[_space_used] = p;
		_space_used++;
	}

	void insert_point(Point p) {
		_points[_space_used] = p;
		_space_used++;
	}
};



SeedPointsQueue collect_seedpoints (Curve* curve) {
	int steps_taken = curve->_steps_taken;
	SeedPointsQueue queue = SeedPointsQueue(steps_taken);
	if (steps_taken == 0) {
		return queue;
	}

	for (int i = 0; i < steps_taken - 1; i++) {
		double x = curve->_x[i];
		double y = curve->_y[i];
			
		int ff_column_index = (int) floor(x);
		int ff_row_index = (int) floor(y);
		double angle = atan2(curve->_y[i + 1] - y, curve->_x[i + 1] - x);

		double angle_left = angle + (M_PI / 2);
		double angle_right = angle - (M_PI / 2);

		Point left_point = {
			x + (D_SEP * cos(angle_left)),
			y + (D_SEP * sin(angle_left))
		};
		Point right_point = {
			x + (D_SEP * cos(angle_right)),
			y + (D_SEP * sin(angle_right))
		};

		queue.insert_point(left_point);	
		queue.insert_point(right_point);	
	}

	return queue;
}




Curve draw_curve(int curve_id,
		 double x_start,
		 double y_start,
		 int n_steps,
		 double step_length,
		 double d_sep,
		 FlowField* flow_field,
		 DensityGrid* density_grid) {

	Curve curve = Curve(curve_id, n_steps);
	curve.insert_step(x_start, y_start, 0);
	double x = x_start;
	double y = y_start;
	int i = 1;
	// Draw curve from right to left
	while (i < (n_steps / 2)) {
		if (flow_field->off_boundaries(x, y)) {
			break;
		}
		
		double angle = flow_field->get_angle(x, y);
		double x_step = step_length * cos(angle);
		double y_step = step_length * sin(angle);
		x = x - x_step;
		y = y - y_step;

		if (!density_grid->is_valid_next_step(x, y)) {
			break;
		}

		curve.insert_step(x, y, 0);
		i++;
	}

	x = x_start;
	y = y_start;
	// Draw curve from left to right
	while (i < n_steps) {
		if (flow_field->off_boundaries(x, y)) {
			break;
		}
		
		double angle = flow_field->get_angle(x, y);
		double x_step = step_length * cos(angle);
		double y_step = step_length * sin(angle);
		x = x + x_step;
		y = y + y_step;

		if (!density_grid->is_valid_next_step(x, y)) {
			break;
		}

		curve.insert_step(x, y, 1);
		i++;
	}

	return curve;
}


std::vector<Curve> even_spaced_curves(double x_start,
				      double y_start,
				      int n_curves,
				      int n_steps,
				      int min_steps_allowed,
				      double step_length,
				      double d_sep,
				      FlowField* flow_field,
			              DensityGrid* density_grid) {
 
	std::vector<Curve> curves;
	curves.reserve(N_CURVES);
	double x = x_start;
	double y = y_start;
	int curve_array_index = 0;
	int curve_id = 0;
	curves[curve_array_index] = draw_curve(
		curve_id,
		x, y,
		n_steps,
		step_length,
		d_sep,
		flow_field,
		density_grid
	);

	density_grid->insert_curve_coords(&curves[curve_array_index]);
	curve_array_index++;


	while (curve_id < n_curves && curve_array_index < n_curves) {
		SeedPointsQueue queue = SeedPointsQueue(n_steps);
		queue = collect_seedpoints(&curves[curve_id]);
		for (Point p : queue._points) {
			// check if it is valid given the current state
			if (density_grid->is_valid_next_step(p.x, p.y)) {
				// if it is, draw the curve from it
				Curve curve = draw_curve(
					curve_array_index,
					p.x, p.y,
					n_steps,
					step_length,
					d_sep,
					flow_field,
					density_grid
				);

				if (curve._steps_taken < min_steps_allowed) {
					continue;
				}

				curves[curve_array_index] = curve;
				// insert this new curve into the density grid
				density_grid->insert_curve_coords(&curve);
				curve_array_index++;
			}
		}

		curve_id++;
	}



	return curves;
}





int main() {
	FlowField flow_field = FlowField();
	DensityGrid density_grid = DensityGrid(D_SEP, 5000);
	std::vector<Curve> curves = even_spaced_curves(
		45.0, 24.0,
		N_CURVES,
		N_STEPS,
		MIN_STEPS_ALLOWED,
		STEP_LENGTH,
		D_SEP,
		&flow_field,
		&density_grid
	);


	return 1;
}
