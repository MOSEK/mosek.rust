#ifndef _MOSEK_DYNAMIC_H_
#define _MOSEK_DYNAMIC_H_

#include "mosek.h"

int MSK_isinitialized();

/** Initialize dynamically loaded library. Search the given paths for the MOSEK library, and fall back to loading from
 *  default system paths. Once the library has been successfully initialized any subsequent calls to this function will
 *  do nothing and always succeed.
 */
MSKrescodee MSK_initializedynamicwithpaths(int num_paths, const char * paths[]);

#endif
