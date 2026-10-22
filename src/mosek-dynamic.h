#ifndef _MOSEK_DYNAMIC_H_
#define _MOSEK_DYNAMIC_H_

#include "mosek.h"

int MSK_isinitialized();

/** Initialize dynamically loaded library. Search the given paths for the MOSEK library, and fall back to loading from
 *  default system paths. Once the library has been successfully initialized any subsequent calls to this function will
 *  do nothing and always succeed.
 *
 *  # Arguments
 *  - `num_paths` Length of the `paths` argument
 *  - `paths[num_paths]` Array of paths to search for the library. If all fails, attempt to load without path, i.e. letting the system loader locate it.
 */
MSKrescodee MSK_initializedynamicwithpaths(int num_paths, const char * paths[]);

/** Search standard locations for the library, and fall back on letting the system loader find it.
 *
 *  This will search in the following locations:
 *  - `$USERPROFILE/` (Windows only)
 *  - `$USERPROFILE/.local` (Windows only)
 *  - `$LOCALAPPDATA/` (Windows only)
 *  - `$PROGRAMFILES/` (Windows only)
 *  - `$HOME/` (Non-Windows only)
 *  - `$HOME/.local` (Non-Windows only)
 *  - `/opt` (Non-Windows only)
 *  - Finally, if all the above fail, load without a path
 */
MSKrescodee MSK_initializedynamicwithdefaults();

#endif
