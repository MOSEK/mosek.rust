#ifndef _MOSEK_DYNAMIC_H_
#define _MOSEK_DYNAMIC_H_

#include "mosek.h"

int MSK_isinitialized();
MSKrescodee MSK_initializedynamicwithpaths(int num_paths, const char * paths[]);

#endif
