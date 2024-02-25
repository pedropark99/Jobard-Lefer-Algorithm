// C Math Library
#include <math.h>

// C++ STD Libraries
#include <vector>

#define FNL_IMPL
#include "FastNoiseLite.h"

#define FLOW_FIELD_WIDTH 120
#define FLOW_FIELD_HEIGHT 120
#define N_STEPS 30
#define N_CURVES 1500
#define STEP_LENGTH 0.01 * FLOW_FIELD_WIDTH
#define D_SEP 0.8
#define DENSITY_GRID_WIDTH ((int) (FLOW_FIELD_WIDTH / D_SEP))



int get_density_col (double x, double d_sep) {
	double c = (x / d_sep) + 1;
	return (int) c;
}

int get_density_row (double y, double d_sep) {
	double r = (y / d_sep) + 1;
	return (int) r;
}

double distance (double x1, double y1, double x2, double y2) {
	double s1 = pow(x2 - x1, 2.0);
	double s2 = pow(y2 - y1, 2.0);
	return sqrt(s1 + s2);
}

bool off_boundaries (double x, double y, int limit) {
	return (
		x <= 0 ||
		y <= 0 ||
		x >= limit ||
		y >= limit
	);
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



int main() {

	FlowField f = FlowField();

	return 1;
}
