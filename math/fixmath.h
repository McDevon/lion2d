//  Created by Jussi Enroos on 12/11/2018.
//  Copyright (c) 2018 Jussi Enroos

#ifndef fixmath_h
#define fixmath_h

#include <stdio.h>
#include <stdbool.h>

typedef int32_t dnum;

dnum d_add(dnum a, dnum b);
dnum d_sub(dnum a, dnum b);
dnum d_mul(dnum a, dnum b);
dnum d_div(dnum a, dnum b);

dnum d_sign(dnum a);
dnum d_abs(dnum a);
dnum d_floor(dnum a);
dnum d_ceil(dnum a);
dnum d_round(dnum a);

dnum d_mod(dnum a);
bool d_less_than(dnum a, dnum b);
bool d_less_than_or_equal(dnum a, dnum b);
bool d_greater_than(dnum a, dnum b);
bool d_greater_than_or_equal(dnum a, dnum b);
bool d_equals(dnum a, dnum b);

dnum d_sin(dnum a);
dnum d_cos(dnum a);
dnum d_tan(dnum a);
dnum d_atan2(dnum y, dnum x);

dnum d_to_degrees(dnum a);
dnum d_to_radians(dnum a);

dnum d_from_string(const char *string);
dnum d_from_int(int value);
dnum d_from_float(float value);
dnum d_from_double(double value);

char *d_to_string(dnum a);
int d_to_int(dnum a);
float d_to_float(dnum a);
double d_to_double(dnum a);

#endif
