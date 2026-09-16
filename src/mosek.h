#ifndef MOSEK_H
#define MOSEK_H

/******************************************************************************
 ** Module : mosek.h
 **
 ** Generated 2026
 **
 ** Copyright (c) MOSEK ApS, Denmark.
 **
 ** All rights reserved
 **
 ******************************************************************************/
/*
 The content of this file is subject to copyright. However, it may free
 of charge be redistributed in identical form --- i.e. with no changes of
 the wording --- for any legitimate purpose.
*/


#include <stdlib.h>
#include <stddef.h>
#include <stdio.h>
#include <stdint.h>

#define MSK_VERSION_MAJOR    12
#define MSK_VERSION_MINOR    0
#define MSK_VERSION_REVISION 0
#define MSK_VERSION_STATE    "ALPHA"

#define MSK_INFINITY 1.0e30

#if _WIN32
#define MSKAPI __cdecl
#define MSKAPIVA __stdcall
#else
#define MSKAPI
#define MSKAPIVA
#endif

/* Enums and constants */

enum MSKbasindtype_enum {
  /** Never do basis identification. */
  MSK_BI_NEVER       = 0,
  /** Basis identification is always performed even if the interior-point optimizer terminates abnormally. */
  MSK_BI_ALWAYS      = 1,
  /** Basis identification is performed if the interior-point optimizer terminates without an error. */
  MSK_BI_NO_ERROR    = 2,
  /** Basis identification is not performed if the interior-point optimizer terminates with a problem status saying that the problem is primal or dual infeasible. */
  MSK_BI_IF_FEASIBLE = 3
}; /* MSKbasindtype_enum */
#define MSK_BI_BEGIN MSK_BI_NEVER
#define MSK_BI_END   (1+MSK_BI_IF_FEASIBLE)
#ifdef MSK_NO_ENUMS
typedef int MSKbasindtypee;
#else
typedef int MSKbasindtypee;
#endif

enum MSKboundkey_enum {
  /** The constraint or variable has a finite lower bound and an infinite upper bound. */
  MSK_BK_LO = 0,
  /** The constraint or variable has an infinite lower bound and an finite upper bound. */
  MSK_BK_UP = 1,
  /** The constraint or variable is fixed. */
  MSK_BK_FX = 2,
  /** The constraint or variable is free. */
  MSK_BK_FR = 3,
  /** The constraint or variable is ranged. */
  MSK_BK_RA = 4
}; /* MSKboundkey_enum */
#define MSK_BK_BEGIN MSK_BK_LO
#define MSK_BK_END   (1+MSK_BK_RA)
#ifdef MSK_NO_ENUMS
typedef int MSKboundkeye;
#else
typedef enum MSKboundkey_enum MSKboundkeye;
#endif

enum MSKmark_enum {
  /** The lower bound is selected for sensitivity analysis. */
  MSK_MARK_LO = 0,
  /** The upper bound is selected for sensitivity analysis. */
  MSK_MARK_UP = 1
}; /* MSKmark_enum */
#define MSK_MARK_BEGIN MSK_MARK_LO
#define MSK_MARK_END   (1+MSK_MARK_UP)
#ifdef MSK_NO_ENUMS
typedef int MSKmarke;
#else
typedef enum MSKmark_enum MSKmarke;
#endif

enum MSKsimprecision_enum {
  /** Experimental. Usage not recommended. */
  MSK_SIM_PRECISION_NORMAL   = 0,
  /** Experimental. Usage not recommended. */
  MSK_SIM_PRECISION_EXTENDED = 1
}; /* MSKsimprecision_enum */
#define MSK_SIM_PRECISION_BEGIN MSK_SIM_PRECISION_NORMAL
#define MSK_SIM_PRECISION_END   (1+MSK_SIM_PRECISION_EXTENDED)
#ifdef MSK_NO_ENUMS
typedef int MSKsimprecisione;
#else
typedef enum MSKsimprecision_enum MSKsimprecisione;
#endif

enum MSKsimdegen_enum {
  /** The simplex optimizer should use no degeneration strategy. */
  MSK_SIM_DEGEN_NONE       = 0,
  /** The simplex optimizer chooses the degeneration strategy. */
  MSK_SIM_DEGEN_FREE       = 1,
  /** The simplex optimizer should use an aggressive degeneration strategy. */
  MSK_SIM_DEGEN_AGGRESSIVE = 2,
  /** The simplex optimizer should use a moderate degeneration strategy. */
  MSK_SIM_DEGEN_MODERATE   = 3,
  /** The simplex optimizer should use a minimum degeneration strategy. */
  MSK_SIM_DEGEN_MINIMUM    = 4
}; /* MSKsimdegen_enum */
#define MSK_SIM_DEGEN_BEGIN MSK_SIM_DEGEN_NONE
#define MSK_SIM_DEGEN_END   (1+MSK_SIM_DEGEN_MINIMUM)
#ifdef MSK_NO_ENUMS
typedef int MSKsimdegene;
#else
typedef enum MSKsimdegen_enum MSKsimdegene;
#endif

enum MSKtranspose_enum {
  /** No transpose is applied. */
  MSK_TRANSPOSE_NO  = 0,
  /** A transpose is applied. */
  MSK_TRANSPOSE_YES = 1
}; /* MSKtranspose_enum */
#define MSK_TRANSPOSE_BEGIN MSK_TRANSPOSE_NO
#define MSK_TRANSPOSE_END   (1+MSK_TRANSPOSE_YES)
#ifdef MSK_NO_ENUMS
typedef int MSKtransposee;
#else
typedef enum MSKtranspose_enum MSKtransposee;
#endif

enum MSKuplo_enum {
  /** Lower part. */
  MSK_UPLO_LO = 0,
  /** Upper part. */
  MSK_UPLO_UP = 1
}; /* MSKuplo_enum */
#define MSK_UPLO_BEGIN MSK_UPLO_LO
#define MSK_UPLO_END   (1+MSK_UPLO_UP)
#ifdef MSK_NO_ENUMS
typedef int MSKuploe;
#else
typedef enum MSKuplo_enum MSKuploe;
#endif

enum MSKsimreform_enum {
  /** Disallow the simplex optimizer to reformulate the problem. */
  MSK_SIM_REFORMULATION_OFF        = 0,
  /** Allow the simplex optimizer to reformulate the problem. */
  MSK_SIM_REFORMULATION_ON         = 1,
  /** The simplex optimizer can choose freely. */
  MSK_SIM_REFORMULATION_FREE       = 2,
  /** The simplex optimizer should use an aggressive reformulation strategy. */
  MSK_SIM_REFORMULATION_AGGRESSIVE = 3
}; /* MSKsimreform_enum */
#define MSK_SIM_REFORMULATION_BEGIN MSK_SIM_REFORMULATION_OFF
#define MSK_SIM_REFORMULATION_END   (1+MSK_SIM_REFORMULATION_AGGRESSIVE)
#ifdef MSK_NO_ENUMS
typedef int MSKsimreforme;
#else
typedef enum MSKsimreform_enum MSKsimreforme;
#endif

enum MSKsimdupvec_enum {
  /** Disallow the simplex optimizer to exploit duplicated columns. */
  MSK_SIM_EXPLOIT_DUPVEC_OFF  = 0,
  /** Allow the simplex optimizer to exploit duplicated columns. */
  MSK_SIM_EXPLOIT_DUPVEC_ON   = 1,
  /** The simplex optimizer can choose freely. */
  MSK_SIM_EXPLOIT_DUPVEC_FREE = 2
}; /* MSKsimdupvec_enum */
#define MSK_SIM_EXPLOIT_DUPVEC_BEGIN MSK_SIM_EXPLOIT_DUPVEC_OFF
#define MSK_SIM_EXPLOIT_DUPVEC_END   (1+MSK_SIM_EXPLOIT_DUPVEC_FREE)
#ifdef MSK_NO_ENUMS
typedef int MSKsimdupvece;
#else
typedef enum MSKsimdupvec_enum MSKsimdupvece;
#endif

enum MSKsimhotstart_enum {
  /** The simplex optimizer performs a coldstart. */
  MSK_SIM_HOTSTART_NONE        = 0,
  /** The simplex optimize chooses the hot-start type. */
  MSK_SIM_HOTSTART_FREE        = 1,
  /** Only the status keys of the constraints and variables are used to choose the type of hot-start. */
  MSK_SIM_HOTSTART_STATUS_KEYS = 2
}; /* MSKsimhotstart_enum */
#define MSK_SIM_HOTSTART_BEGIN MSK_SIM_HOTSTART_NONE
#define MSK_SIM_HOTSTART_END   (1+MSK_SIM_HOTSTART_STATUS_KEYS)
#ifdef MSK_NO_ENUMS
typedef int MSKsimhotstarte;
#else
typedef enum MSKsimhotstart_enum MSKsimhotstarte;
#endif

enum MSKintpnthotstart_enum {
  /** The interior-point optimizer performs a coldstart. */
  MSK_INTPNT_HOTSTART_NONE        = 0,
  /** The interior-point optimizer exploits the primal solution only. */
  MSK_INTPNT_HOTSTART_PRIMAL      = 1,
  /** The interior-point optimizer exploits the dual solution only. */
  MSK_INTPNT_HOTSTART_DUAL        = 2,
  /** The interior-point optimizer exploits both the primal and dual solution. */
  MSK_INTPNT_HOTSTART_PRIMAL_DUAL = 3
}; /* MSKintpnthotstart_enum */
#define MSK_INTPNT_HOTSTART_BEGIN MSK_INTPNT_HOTSTART_NONE
#define MSK_INTPNT_HOTSTART_END   (1+MSK_INTPNT_HOTSTART_PRIMAL_DUAL)
#ifdef MSK_NO_ENUMS
typedef int MSKintpnthotstarte;
#else
typedef enum MSKintpnthotstart_enum MSKintpnthotstarte;
#endif

enum MSKcallbackcode_enum {
  /** The basis identification procedure has been started. */
  MSK_CALLBACK_BEGIN_BI                                         = 0,
  /** The callback function is called from within the basis identification procedure when the dual phase is started. */
  MSK_CALLBACK_BEGIN_BI_DUAL                                    = 1,
  /** The callback function is called from within the basis identification procedure when the initialization phase is started. */
  MSK_CALLBACK_BEGIN_BI_INITIALIZE                              = 2,
  /** Starting of reoptimizing the basic solution. */
  MSK_CALLBACK_BEGIN_BI_OPTIMIZER                               = 3,
  /** Reoptimizing the basic solution. */
  MSK_CALLBACK_BEGIN_BI_OPTIMIZER_PRIMAL_DUAL_SIMPLEX           = 4,
  /** The callback function is called from within the basis identification procedure when the primal phase is started. */
  MSK_CALLBACK_BEGIN_BI_PRIMAL                                  = 5,
  /** The callback function is called from within the basis identification procedure when the primal simplex clean-up phase is started. */
  MSK_CALLBACK_BEGIN_BI_PRIMAL_SIMPLEX                          = 6,
  /** The callback function is called when the concurrent optimizer is started. */
  MSK_CALLBACK_BEGIN_CONCURRENT                                 = 7,
  /** The callback function is called when the conic optimizer is started. */
  MSK_CALLBACK_BEGIN_CONIC                                      = 8,
  /** Dual sensitivity analysis is started. */
  MSK_CALLBACK_BEGIN_DUAL_SENSITIVITY                           = 9,
  /** The callback function is called when the dual BI phase is started. */
  MSK_CALLBACK_BEGIN_DUAL_SETUP_BI                              = 10,
  /** The callback function is called when the dual simplex optimizer started. */
  MSK_CALLBACK_BEGIN_DUAL_SIMPLEX                               = 11,
  /** The callback function is called from within the basis identification procedure when the dual simplex clean-up phase is started. */
  MSK_CALLBACK_BEGIN_DUAL_SIMPLEX_BI                            = 12,
  /** The callback function is called when the dualizer is started. */
  MSK_CALLBACK_BEGIN_DUALIZER                                   = 13,
  /** The calback function is called at the beginning of folding. */
  MSK_CALLBACK_BEGIN_FOLDING                                    = 14,
  /** The callback function is called from within the matrix ordering procedure at an intermediate point. */
  MSK_CALLBACK_BEGIN_GP_ORDER                                   = 15,
  /** The callback function is called when the infeasibility analyzer is started. */
  MSK_CALLBACK_BEGIN_INFEAS_ANA                                 = 16,
  /** The callback function is called when the interior-point optimizer is started. */
  MSK_CALLBACK_BEGIN_INTPNT                                     = 17,
  /** The callback function is called when the interior-point optimizer setup is started. */
  MSK_CALLBACK_BEGIN_INTPNT_SETUP                               = 18,
  /** Begin waiting for license. */
  MSK_CALLBACK_BEGIN_LICENSE_WAIT                               = 19,
  /** The callback function is called from within the matrix ordering procedure at an intermediate point. */
  MSK_CALLBACK_BEGIN_LOCAL_ORDER                                = 20,
  /** The callback function is called when the mixed-integer optimizer is started. */
  MSK_CALLBACK_BEGIN_MIO                                        = 21,
  /** The callback function is called when the optimizer is started. */
  MSK_CALLBACK_BEGIN_OPTIMIZER                                  = 22,
  /** The callback function is called from within the matrix ordering procedure at an intermediate point. */
  MSK_CALLBACK_BEGIN_ORDER                                      = 23,
  /** The callback function is called when the presolve is started. */
  MSK_CALLBACK_BEGIN_PRESOLVE                                   = 24,
  /** The presolve elimination phase is started. */
  MSK_CALLBACK_BEGIN_PRESOLVE_ELIMINATOR                        = 25,
  /** The presolve linear depdency check phase is started. */
  MSK_CALLBACK_BEGIN_PRESOLVE_LINEAR_DEPENDENCIES               = 26,
  /** The callback function is called when the primal-dual simplex optimizer started. */
  MSK_CALLBACK_BEGIN_PRIMAL_DUAL_SIMPLEX                        = 27,
  /** Begin primal feasibility repair. */
  MSK_CALLBACK_BEGIN_PRIMAL_REPAIR                              = 28,
  /** Primal sensitivity analysis is started. */
  MSK_CALLBACK_BEGIN_PRIMAL_SENSITIVITY                         = 29,
  /** The callback function is called when the primal BI setup is started. */
  MSK_CALLBACK_BEGIN_PRIMAL_SETUP_BI                            = 30,
  /** The callback function is called when the primal simplex optimizer is started. */
  MSK_CALLBACK_BEGIN_PRIMAL_SIMPLEX                             = 31,
  /** Begin QCQO reformulation. */
  MSK_CALLBACK_BEGIN_QCQO_REFORMULATE                           = 32,
  /** MOSEK has started reading a problem file. */
  MSK_CALLBACK_BEGIN_READ                                       = 33,
  /** The callback function is called when root cut generation is started. */
  MSK_CALLBACK_BEGIN_ROOT_CUTGEN                                = 34,
  /** The callback function is called when the simplex optimizer is started. */
  MSK_CALLBACK_BEGIN_SIMPLEX                                    = 35,
  /** The callback function is called when solution of root relaxation is started. */
  MSK_CALLBACK_BEGIN_SOLVE_ROOT_RELAX                           = 36,
  /** Begin conic reformulation. */
  MSK_CALLBACK_BEGIN_TO_CONIC                                   = 37,
  /** Basis identification after undualizing is started. */
  MSK_CALLBACK_BEGIN_UNDUALIZE_BI                               = 38,
  /** TBD */
  MSK_CALLBACK_BEGIN_UNDUALIZE_BI_DUAL                          = 39,
  /** TBD */
  MSK_CALLBACK_BEGIN_UNDUALIZE_BI_INITIALIZE                    = 40,
  /** TBD */
  MSK_CALLBACK_BEGIN_UNDUALIZE_BI_OPTIMIZE                      = 41,
  /** TBD */
  MSK_CALLBACK_BEGIN_UNDUALIZE_BI_OPTIMIZE_128BIT               = 42,
  /** TBD */
  MSK_CALLBACK_BEGIN_UNDUALIZE_BI_OPTIMIZER_PRIMAL_DUAL_SIMPLEX = 43,
  /** TBD */
  MSK_CALLBACK_BEGIN_UNDUALIZE_BI_PRIMAL                        = 44,
  /** The callback function is called when undualizing is started. */
  MSK_CALLBACK_BEGIN_UNDUALIZING                                = 45,
  /** Basis identification after unfolding is started. */
  MSK_CALLBACK_BEGIN_UNFOLD_BI                                  = 46,
  /** Basis identification dual phase after unfolding is started. */
  MSK_CALLBACK_BEGIN_UNFOLD_BI_DUAL                             = 47,
  /** Basis identification initialization after unfolding is started. */
  MSK_CALLBACK_BEGIN_UNFOLD_BI_INITIALIZE                       = 48,
  /** Basis identification optimizer phase after unfolding is started. */
  MSK_CALLBACK_BEGIN_UNFOLD_BI_OPTIMIZER                        = 49,
  /** Basis identification primal-dual-simplex phase after unfolding is started.. */
  MSK_CALLBACK_BEGIN_UNFOLD_BI_OPTIMIZER_PRIMAL_DUAL_SIMPLEX    = 50,
  /** Basis identification primal phase after unfolding is started. */
  MSK_CALLBACK_BEGIN_UNFOLD_BI_PRIMAL                           = 51,
  /** The calback function is called at the beginning of unfolding. */
  MSK_CALLBACK_BEGIN_UNFOLDING                                  = 52,
  /** MOSEK has started writing a problem file. */
  MSK_CALLBACK_BEGIN_WRITE                                      = 53,
  /** Reoptimizing the basic solution. */
  MSK_CALLBACK_BI_OPTIMIZER_PRIMAL_DUAL_SIMPLEX                 = 54,
  /** The callback function is called from within the conic optimizer after the information database has been updated. */
  MSK_CALLBACK_CONIC                                            = 55,
  /** The callback function is called when the dedicated algorithm for independent blocks inside the mixed-integer solver is started. */
  MSK_CALLBACK_DECOMP_MIO                                       = 56,
  /** The callback function is called from within the dual simplex optimizer. */
  MSK_CALLBACK_DUAL_SIMPLEX                                     = 57,
  /** The callback function is called when the basis identification procedure is terminated. */
  MSK_CALLBACK_END_BI                                           = 58,
  /** The callback function is called from within the basis identification procedure when the dual phase is terminated. */
  MSK_CALLBACK_END_BI_DUAL                                      = 59,
  /** The callback function is called from within the basis identification procedure when the initialization phase is terminated. */
  MSK_CALLBACK_END_BI_INITIALIZE                                = 60,
  /** Terminating of reoptimizing the basic solution. */
  MSK_CALLBACK_END_BI_OPTIMIZER                                 = 61,
  /** Reoptimizing the basic solution. */
  MSK_CALLBACK_END_BI_OPTIMIZER_PRIMAL_DUAL_SIMPLEX             = 62,
  /** The callback function is called from within the basis identification procedure when the primal phase is terminated. */
  MSK_CALLBACK_END_BI_PRIMAL                                    = 63,
  /** The callback function is called when the concurrent optimizer is terminated. */
  MSK_CALLBACK_END_CONCURRENT                                   = 64,
  /** The callback function is called when the conic optimizer is terminated. */
  MSK_CALLBACK_END_CONIC                                        = 65,
  /** Dual sensitivity analysis is terminated. */
  MSK_CALLBACK_END_DUAL_SENSITIVITY                             = 66,
  /** The callback function is called when the dual BI phase is terminated. */
  MSK_CALLBACK_END_DUAL_SETUP_BI                                = 67,
  /** The callback function is called when the dual simplex optimizer is terminated. */
  MSK_CALLBACK_END_DUAL_SIMPLEX                                 = 68,
  /** The callback function is called from within the basis identification procedure when the dual clean-up phase is terminated. */
  MSK_CALLBACK_END_DUAL_SIMPLEX_BI                              = 69,
  /** The callback function is called when the dualizer is terminated. */
  MSK_CALLBACK_END_DUALIZER                                     = 70,
  /** The calback function is called at the end of folding. */
  MSK_CALLBACK_END_FOLDING                                      = 71,
  /** The callback function is called from within the matrix ordering procedure at an intermediate point. */
  MSK_CALLBACK_END_GP_ORDER                                     = 72,
  /** The callback function is called when the infeasibility analyzer is terminated. */
  MSK_CALLBACK_END_INFEAS_ANA                                   = 73,
  /** The callback function is called when the interior-point optimizer is terminated. */
  MSK_CALLBACK_END_INTPNT                                       = 74,
  /** The callback function is called when the interior-point optimizer setup is terminated. */
  MSK_CALLBACK_END_INTPNT_SETUP                                 = 75,
  /** End waiting for license. */
  MSK_CALLBACK_END_LICENSE_WAIT                                 = 76,
  /** The callback function is called from within the matrix ordering procedure at an intermediate point. */
  MSK_CALLBACK_END_LOCAL_ORDER                                  = 77,
  /** The callback function is called when the mixed-integer optimizer is terminated. */
  MSK_CALLBACK_END_MIO                                          = 78,
  /** The callback function is called when the optimizer is terminated. */
  MSK_CALLBACK_END_OPTIMIZER                                    = 79,
  /** The callback function is called from within the matrix ordering procedure at an intermediate point. */
  MSK_CALLBACK_END_ORDER                                        = 80,
  /** The callback function is called when the presolve is completed. */
  MSK_CALLBACK_END_PRESOLVE                                     = 81,
  /** The presolve elimination phase is terminated. */
  MSK_CALLBACK_END_PRESOLVE_ELIMINATOR                          = 82,
  /** The presolve linear depdency check phase is terminated. */
  MSK_CALLBACK_END_PRESOLVE_LINEAR_DEPENDENCIES                 = 83,
  /** The callback function is called when the primal-dual optimizer is terminated. */
  MSK_CALLBACK_END_PRIMAL_DUAL_SIMPLEX                          = 84,
  /** End primal feasibility repair. */
  MSK_CALLBACK_END_PRIMAL_REPAIR                                = 85,
  /** Primal sensitivity analysis is terminated. */
  MSK_CALLBACK_END_PRIMAL_SENSITIVITY                           = 86,
  /** The callback function is called when the primal BI setup is terminated. */
  MSK_CALLBACK_END_PRIMAL_SETUP_BI                              = 87,
  /** The callback function is called when the primal simplex optimizer is terminated. */
  MSK_CALLBACK_END_PRIMAL_SIMPLEX                               = 88,
  /** The callback function is called from within the basis identification procedure when the primal clean-up phase is terminated. */
  MSK_CALLBACK_END_PRIMAL_SIMPLEX_BI                            = 89,
  /** End QCQO reformulation. */
  MSK_CALLBACK_END_QCQO_REFORMULATE                             = 90,
  /** MOSEK has finished reading a problem file. */
  MSK_CALLBACK_END_READ                                         = 91,
  /** The callback function is called when root cut generation is terminated. */
  MSK_CALLBACK_END_ROOT_CUTGEN                                  = 92,
  /** The callback function is called when the simplex optimizer is terminated. */
  MSK_CALLBACK_END_SIMPLEX                                      = 93,
  /** The callback function is called from within the basis identification procedure when the simplex clean-up phase is terminated. */
  MSK_CALLBACK_END_SIMPLEX_BI                                   = 94,
  /** The callback function is called when solution of root relaxation is terminated. */
  MSK_CALLBACK_END_SOLVE_ROOT_RELAX                             = 95,
  /** End conic reformulation. */
  MSK_CALLBACK_END_TO_CONIC                                     = 96,
  /** Basis identification after undualizing is terminated. */
  MSK_CALLBACK_END_UNDUALIZE_BI                                 = 97,
  /** TBD */
  MSK_CALLBACK_END_UNDUALIZE_BI_DUAL                            = 98,
  /** TBD */
  MSK_CALLBACK_END_UNDUALIZE_BI_INITIALIZE                      = 99,
  /** TBD */
  MSK_CALLBACK_END_UNDUALIZE_BI_OPIMIZER_PRIMAL_DUAL_SIMPLEX    = 100,
  /** TBD */
  MSK_CALLBACK_END_UNDUALIZE_BI_OPTIMIZE                        = 101,
  /** TBD */
  MSK_CALLBACK_END_UNDUALIZE_BI_OPTIMIZE_128BIT                 = 102,
  /** TBD */
  MSK_CALLBACK_END_UNDUALIZE_BI_PRIMAL                          = 103,
  /** The callback function is called when undualizing is terminated. */
  MSK_CALLBACK_END_UNDUALIZING                                  = 104,
  /** Basis identification after unfolding is terminated. */
  MSK_CALLBACK_END_UNFOLD_BI                                    = 105,
  /** Basis identification dual phase after unfolding is started. */
  MSK_CALLBACK_END_UNFOLD_BI_DUAL                               = 106,
  /** Basis identification initialization after unfolding is terminated. */
  MSK_CALLBACK_END_UNFOLD_BI_INITIALIZE                         = 107,
  /** Basis identification optimizer phase after unfolding is terminated. */
  MSK_CALLBACK_END_UNFOLD_BI_OPTIMIZER                          = 108,
  /** Basis identification primal-dual simplex phase after unfolding is terminated. */
  MSK_CALLBACK_END_UNFOLD_BI_OPTIMIZER_PRIMAL_DUAL_SIMPLEX      = 109,
  /** Basis identification primal phase after unfolding is terminated. */
  MSK_CALLBACK_END_UNFOLD_BI_PRIMAL                             = 110,
  /** The calback function is called at the end of unfolding. */
  MSK_CALLBACK_END_UNFOLDING                                    = 111,
  /** MOSEK has finished writing a problem file. */
  MSK_CALLBACK_END_WRITE                                        = 112,
  /** The callback function is called from within the matrix ordering procedure at an intermediate point. */
  MSK_CALLBACK_GP_ORDER                                         = 113,
  /** A heartbeat callback. */
  MSK_CALLBACK_HEARTBEAT                                        = 114,
  /** The callback function is called at an intermediate stage of the dual sensitivity analysis. */
  MSK_CALLBACK_IM_DUAL_SENSIVITY                                = 115,
  /** The callback function is called at an intermediate point in the dual simplex optimizer. */
  MSK_CALLBACK_IM_DUAL_SIMPLEX                                  = 116,
  /** MOSEK is waiting for a license. */
  MSK_CALLBACK_IM_LICENSE_WAIT                                  = 117,
  /** The callback function is called from within the LU factorization procedure at an intermediate point. */
  MSK_CALLBACK_IM_LU                                            = 118,
  /** The callback function is called at an intermediate point in the mixed-integer optimizer. */
  MSK_CALLBACK_IM_MIO                                           = 119,
  /** The callback function is called at an intermediate point in the mixed-integer optimizer while running the dual simplex optimizer. */
  MSK_CALLBACK_IM_MIO_DUAL_SIMPLEX                              = 120,
  /** The callback function is called at an intermediate point in the mixed-integer optimizer while running the interior-point optimizer. */
  MSK_CALLBACK_IM_MIO_INTPNT                                    = 121,
  /** The callback function is called at an intermediate point in the mixed-integer optimizer while running the primal simplex optimizer. */
  MSK_CALLBACK_IM_MIO_PRIMAL_SIMPLEX                            = 122,
  /** The callback function is called at an intermediate stage of the primal sensitivity analysis. */
  MSK_CALLBACK_IM_PRIMAL_SENSIVITY                              = 123,
  /** The callback function is called at an intermediate point in the primal simplex optimizer. */
  MSK_CALLBACK_IM_PRIMAL_SIMPLEX                                = 124,
  /** Intermediate stage in reading. */
  MSK_CALLBACK_IM_READ                                          = 125,
  /** The callback is called from within root cut generation at an intermediate stage. */
  MSK_CALLBACK_IM_ROOT_CUTGEN                                   = 126,
  /** The callback function is called from within the simplex optimizer at an intermediate point. */
  MSK_CALLBACK_IM_SIMPLEX                                       = 127,
  /** The callback function is called from within the interior-point optimizer after the information database has been updated. */
  MSK_CALLBACK_INTPNT                                           = 128,
  /** The callback function is called from within the matrix ordering procedure at an intermediate point. */
  MSK_CALLBACK_LOCAL_ORDER                                      = 129,
  /** The callback function is called after a new integer solution has been located by the mixed-integer optimizer. */
  MSK_CALLBACK_NEW_INT_MIO                                      = 130,
  /** The callback function is called from within the matrix ordering procedure at an intermediate point. */
  MSK_CALLBACK_ORDER                                            = 131,
  /** The callback function is called in the primal-dual simplex optimizer. */
  MSK_CALLBACK_PRIMAL_DUAL_SIMPLEX                              = 132,
  /** The callback function is called from within the primal simplex optimizer. */
  MSK_CALLBACK_PRIMAL_SIMPLEX                                   = 133,
  /** The callback function is called at an intermediate stage of the conic quadratic reformulation. */
  MSK_CALLBACK_QO_REFORMULATE                                   = 134,
  /** The callback function is called from the OPF reader. */
  MSK_CALLBACK_READ_OPF                                         = 135,
  /** A chunk of Q non-zeros has been read from a problem file. */
  MSK_CALLBACK_READ_OPF_SECTION                                 = 136,
  /** The callback function is called when the mixed-integer optimizer is restarted. */
  MSK_CALLBACK_RESTART_MIO                                      = 137,
  /** The callback function is called while the task is being solved on a remote server. */
  MSK_CALLBACK_SOLVING_REMOTE                                   = 138,
  /** TBD */
  MSK_CALLBACK_UNDUALIZE_BI_DUAL                                = 139,
  /** TBD */
  MSK_CALLBACK_UNDUALIZE_BI_OPTIMIZE_128BIT                     = 140,
  /** TBD */
  MSK_CALLBACK_UNDUALIZE_BI_OPTIMIZER_PRIMAL_DUAL_SIMPLEX       = 141,
  /** TBD */
  MSK_CALLBACK_UNDUALIZE_BI_PRIMAL                              = 142,
  /** Basis indentification dual phase after unfolding. */
  MSK_CALLBACK_UNFOLD_BI_DUAL                                   = 143,
  /** Reoptimizing the basic solution. */
  MSK_CALLBACK_UNFOLD_BI_OPTIMIZER_PRIMAL_DUAL_SIMPLEX          = 144,
  /** Basis indentification primal phase after unfolding. */
  MSK_CALLBACK_UNFOLD_BI_PRIMAL                                 = 145,
  /** The callback function is called from within the basis identification procedure at an intermediate point in the dual phase. */
  MSK_CALLBACK_UPDATE_BI_DUAL                                   = 146,
  /** The callback function is called from within the basis identification procedure at an intermediate point in the primal phase. */
  MSK_CALLBACK_UPDATE_BI_PRIMAL                                 = 147,
  /** The callback function is called from within the basis identification procedure at an intermediate point in the primal simplex clean-up phase. */
  MSK_CALLBACK_UPDATE_BI_PRIMAL_SIMPLEX                         = 148,
  /** The callback function is called in the dual simplex optimizer. */
  MSK_CALLBACK_UPDATE_DUAL_SIMPLEX                              = 149,
  /** The callback function is called from within the basis identification procedure at an intermediate point in the dual simplex clean-up phase. */
  MSK_CALLBACK_UPDATE_DUAL_SIMPLEX_BI                           = 150,
  /** The callback function is called from within the presolve procedure. */
  MSK_CALLBACK_UPDATE_PRESOLVE                                  = 151,
  /** The callback function is called  in the primal simplex optimizer. */
  MSK_CALLBACK_UPDATE_PRIMAL_SIMPLEX                            = 152,
  /** The callback function is called from simplex optimizer. */
  MSK_CALLBACK_UPDATE_SIMPLEX                                   = 153,
  /** The callback function is called from the OPF writer. */
  MSK_CALLBACK_WRITE_OPF                                        = 154
}; /* MSKcallbackcode_enum */
#define MSK_CALLBACK_BEGIN MSK_CALLBACK_BEGIN_BI
#define MSK_CALLBACK_END   (1+MSK_CALLBACK_WRITE_OPF)
#ifdef MSK_NO_ENUMS
typedef int MSKcallbackcodee;
#else
typedef enum MSKcallbackcode_enum MSKcallbackcodee;
#endif

enum MSKcompresstype_enum {
  /** No compression is used. */
  MSK_COMPRESS_NONE = 0,
  /** The type of compression used is chosen automatically. */
  MSK_COMPRESS_FREE = 1,
  /** The type of compression used is gzip compatible. */
  MSK_COMPRESS_GZIP = 2,
  /** The type of compression used is zstd compatible. */
  MSK_COMPRESS_ZSTD = 3
}; /* MSKcompresstype_enum */
#define MSK_COMPRESS_BEGIN MSK_COMPRESS_NONE
#define MSK_COMPRESS_END   (1+MSK_COMPRESS_ZSTD)
#ifdef MSK_NO_ENUMS
typedef int MSKcompresstypee;
#else
typedef enum MSKcompresstype_enum MSKcompresstypee;
#endif

enum MSKconetype_enum {
  /** The cone is a quadratic cone. */
  MSK_CT_QUAD  = 0,
  /** The cone is a rotated quadratic cone. */
  MSK_CT_RQUAD = 1,
  /** A primal exponential cone. */
  MSK_CT_PEXP  = 2,
  /** A dual exponential cone. */
  MSK_CT_DEXP  = 3,
  /** A primal power cone. */
  MSK_CT_PPOW  = 4,
  /** A dual power cone. */
  MSK_CT_DPOW  = 5,
  /** The zero cone. */
  MSK_CT_ZERO  = 6
}; /* MSKconetype_enum */
#define MSK_CT_BEGIN MSK_CT_QUAD
#define MSK_CT_END   (1+MSK_CT_ZERO)
#ifdef MSK_NO_ENUMS
typedef int MSKconetypee;
#else
typedef enum MSKconetype_enum MSKconetypee;
#endif

enum MSKdomaintype_enum {
  /** R. */
  MSK_DOMAIN_R                    = 0,
  /** The zero vector. */
  MSK_DOMAIN_RZERO                = 1,
  /** The positive orthant. */
  MSK_DOMAIN_RPLUS                = 2,
  /** The negative orthant. */
  MSK_DOMAIN_RMINUS               = 3,
  /** The quadratic cone. */
  MSK_DOMAIN_QUADRATIC_CONE       = 4,
  /** The rotated quadratic cone. */
  MSK_DOMAIN_RQUADRATIC_CONE      = 5,
  /** The primal exponential cone. */
  MSK_DOMAIN_PRIMAL_EXP_CONE      = 6,
  /** The dual exponential cone. */
  MSK_DOMAIN_DUAL_EXP_CONE        = 7,
  /** The primal power cone. */
  MSK_DOMAIN_PRIMAL_POWER_CONE    = 8,
  /** The dual power cone. */
  MSK_DOMAIN_DUAL_POWER_CONE      = 9,
  /** The primal geometric mean cone. */
  MSK_DOMAIN_PRIMAL_GEO_MEAN_CONE = 10,
  /** The dual geometric mean cone. */
  MSK_DOMAIN_DUAL_GEO_MEAN_CONE   = 11,
  /** The vectorized positive semidefinite cone. */
  MSK_DOMAIN_SVEC_PSD_CONE        = 12
}; /* MSKdomaintype_enum */
#define MSK_DOMAIN_BEGIN MSK_DOMAIN_R
#define MSK_DOMAIN_END   (1+MSK_DOMAIN_SVEC_PSD_CONE)
#ifdef MSK_NO_ENUMS
typedef int MSKdomaintypee;
#else
typedef enum MSKdomaintype_enum MSKdomaintypee;
#endif

enum MSKnametype_enum {
  /** General names. However, no duplicate and blank names are allowed. */
  MSK_NAME_TYPE_GEN = 0,
  /** MPS type names. */
  MSK_NAME_TYPE_MPS = 1,
  /** LP type names. */
  MSK_NAME_TYPE_LP  = 2
}; /* MSKnametype_enum */
#define MSK_NAME_TYPE_BEGIN MSK_NAME_TYPE_GEN
#define MSK_NAME_TYPE_END   (1+MSK_NAME_TYPE_LP)
#ifdef MSK_NO_ENUMS
typedef int MSKnametypee;
#else
typedef enum MSKnametype_enum MSKnametypee;
#endif

enum MSKsymmattype_enum {
  /** Sparse symmetric matrix. */
  MSK_SYMMAT_TYPE_SPARSE = 0
}; /* MSKsymmattype_enum */
#define MSK_SYMMAT_TYPE_BEGIN MSK_SYMMAT_TYPE_SPARSE
#define MSK_SYMMAT_TYPE_END   (1+MSK_SYMMAT_TYPE_SPARSE)
#ifdef MSK_NO_ENUMS
typedef int MSKsymmattypee;
#else
typedef enum MSKsymmattype_enum MSKsymmattypee;
#endif

enum MSKdataformat_enum {
  /** The file extension is used to determine the data file format. */
  MSK_DATA_FORMAT_EXTENSION = 0,
  /** The data file is MPS formatted. */
  MSK_DATA_FORMAT_MPS       = 1,
  /** The data file is LP formatted. */
  MSK_DATA_FORMAT_LP        = 2,
  /** The data file is an optimization problem formatted file. */
  MSK_DATA_FORMAT_OP        = 3,
  /** The data a free MPS formatted file. */
  MSK_DATA_FORMAT_FREE_MPS  = 4,
  /** Generic task dump file. */
  MSK_DATA_FORMAT_TASK      = 5,
  /** (P)retty (T)ext (F)format. */
  MSK_DATA_FORMAT_PTF       = 6,
  /** Conic benchmark format, */
  MSK_DATA_FORMAT_CB        = 7,
  /** JSON based task format. */
  MSK_DATA_FORMAT_JSON_TASK = 8
}; /* MSKdataformat_enum */
#define MSK_DATA_FORMAT_BEGIN MSK_DATA_FORMAT_EXTENSION
#define MSK_DATA_FORMAT_END   (1+MSK_DATA_FORMAT_JSON_TASK)
#ifdef MSK_NO_ENUMS
typedef int MSKdataformate;
#else
typedef enum MSKdataformat_enum MSKdataformate;
#endif

enum MSKsolformat_enum {
  /** The file extension is used to determine the data file format. */
  MSK_SOL_FORMAT_EXTENSION = 0,
  /** Simple binary format */
  MSK_SOL_FORMAT_B         = 1,
  /** Tar based format. */
  MSK_SOL_FORMAT_TASK      = 2,
  /** JSON based format. */
  MSK_SOL_FORMAT_JSON_TASK = 3
}; /* MSKsolformat_enum */
#define MSK_SOL_FORMAT_BEGIN MSK_SOL_FORMAT_EXTENSION
#define MSK_SOL_FORMAT_END   (1+MSK_SOL_FORMAT_JSON_TASK)
#ifdef MSK_NO_ENUMS
typedef int MSKsolformate;
#else
typedef enum MSKsolformat_enum MSKsolformate;
#endif

enum MSKdinfitem_enum {
  /** Density percentage of the scalarized constraint matrix. */
  MSK_DINF_ANA_PRO_SCALARIZED_CONSTRAINT_MATRIX_DENSITY   = 0,
  /** Time spent within the clean-up phase of the basis identification procedure since its invocation (in seconds). */
  MSK_DINF_BI_CLEAN_TIME                                  = 1,
  /** Time spent within the dual phase basis identification procedure since its invocation (in seconds). */
  MSK_DINF_BI_DUAL_TIME                                   = 2,
  /** Time spent reoptimizing in the BI procedure using 128 bit precision. */
  MSK_DINF_BI_OPTIMIZE_128BIT_TIME                        = 3,
  /** Time spent reoptimizing in the BI procedure using 64 bit precision. */
  MSK_DINF_BI_OPTIMIZE_TIME                               = 4,
  /** Time spent within the primal phase of the basis identification procedure since its invocation (in seconds). */
  MSK_DINF_BI_PRIMAL_TIME                                 = 5,
  /** Time spent within the basis identification procedure since its invocation (in seconds). */
  MSK_DINF_BI_TIME                                        = 6,
  /** Time spent within the concurrent optimizer. */
  MSK_DINF_CONCURRENT_TIME                                = 7,
  /** Maximal individual perturbation when fixing a mixed-integer problem. */
  MSK_DINF_FIXING_MAX_PERTURBATION                        = 8,
  /** Sum of perturbations when fixing a mixed-integer problem. */
  MSK_DINF_FIXING_TOTAL_PERTURBATION                      = 9,
  /** Problem size after folding as a fraction of the original size. */
  MSK_DINF_FOLDING_FACTOR                                 = 10,
  /** Total time spent in folding for continuous problems (in seconds). */
  MSK_DINF_FOLDING_TIME                                   = 11,
  /** Dual feasibility measure reported by the interior-point optimizer. */
  MSK_DINF_INTPNT_DUAL_FEAS                               = 12,
  /** Dual objective value reported by the interior-point optimizer. */
  MSK_DINF_INTPNT_DUAL_OBJ                                = 13,
  /** An estimate of the number of flops used in the factorization. */
  MSK_DINF_INTPNT_FACTOR_NUM_FLOPS                        = 14,
  /** A measure of optimality of the solution. */
  MSK_DINF_INTPNT_OPT_STATUS                              = 15,
  /** Order time (in seconds). */
  MSK_DINF_INTPNT_ORDER_TIME                              = 16,
  /** Primal feasibility measure reported by the interior-point optimizer. */
  MSK_DINF_INTPNT_PRIMAL_FEAS                             = 17,
  /** Primal objective value reported by the interior-point optimizer. */
  MSK_DINF_INTPNT_PRIMAL_OBJ                              = 18,
  /** Interior-point optimizer setup time. */
  MSK_DINF_INTPNT_SETUP_TIME                              = 19,
  /** Time spent within the interior-point optimizer since its invocation (in seconds). */
  MSK_DINF_INTPNT_TIME                                    = 20,
  /** Selection time for clique cuts (in seconds). */
  MSK_DINF_MIO_CLIQUE_SELECTION_TIME                      = 21,
  /** Separation time for clique cuts (in seconds). */
  MSK_DINF_MIO_CLIQUE_SEPARATION_TIME                     = 22,
  /** Selection time for CMIR cuts (in seconds). */
  MSK_DINF_MIO_CMIR_SELECTION_TIME                        = 23,
  /** Separation time for CMIR cuts (in seconds). */
  MSK_DINF_MIO_CMIR_SEPARATION_TIME                       = 24,
  /** Optimal objective value corresponding to the feasible solution. */
  MSK_DINF_MIO_CONSTRUCT_SOLUTION_OBJ                     = 25,
  /** Value of the dual bound after presolve but before cut generation. */
  MSK_DINF_MIO_DUAL_BOUND_AFTER_PRESOLVE                  = 26,
  /** Selection time for GMI cuts (in seconds). */
  MSK_DINF_MIO_GMI_SELECTION_TIME                         = 27,
  /** Separation time for GMI cuts (in seconds). */
  MSK_DINF_MIO_GMI_SEPARATION_TIME                        = 28,
  /** Selection time for implied bound cuts (in seconds). */
  MSK_DINF_MIO_IMPLIED_BOUND_SELECTION_TIME               = 29,
  /** Separation time for implied bound cuts (in seconds). */
  MSK_DINF_MIO_IMPLIED_BOUND_SEPARATION_TIME              = 30,
  /** Optimal objective value corresponding to the user provided initial solution. */
  MSK_DINF_MIO_INITIAL_FEASIBLE_SOLUTION_OBJ              = 31,
  /** Selection time for knapsack cover (in seconds). */
  MSK_DINF_MIO_KNAPSACK_COVER_SELECTION_TIME              = 32,
  /** Separation time for knapsack cover (in seconds). */
  MSK_DINF_MIO_KNAPSACK_COVER_SEPARATION_TIME             = 33,
  /** Selection time for lift-and-project cuts (in seconds). */
  MSK_DINF_MIO_LIPRO_SELECTION_TIME                       = 34,
  /** Separation time for lift-and-project cuts (in seconds). */
  MSK_DINF_MIO_LIPRO_SEPARATION_TIME                      = 35,
  /** If the mixed-integer optimizer has computed a feasible solution and a bound, this contains the absolute gap. */
  MSK_DINF_MIO_OBJ_ABS_GAP                                = 36,
  /** The best bound on the objective value known. */
  MSK_DINF_MIO_OBJ_BOUND                                  = 37,
  /** The primal objective value corresponding to the best integer feasible solution. */
  MSK_DINF_MIO_OBJ_INT                                    = 38,
  /** If the mixed-integer optimizer has computed a feasible solution and a bound, this contains the relative gap. */
  MSK_DINF_MIO_OBJ_REL_GAP                                = 39,
  /** Total time for probing (in seconds). */
  MSK_DINF_MIO_PROBING_TIME                               = 40,
  /** Total time for cut selection (in seconds). */
  MSK_DINF_MIO_ROOT_CUT_SELECTION_TIME                    = 41,
  /** Total time for cut separation (in seconds). */
  MSK_DINF_MIO_ROOT_CUT_SEPARATION_TIME                   = 42,
  /** Time spent in the contiuous optimizer while processing the root node relaxation (in seconds). */
  MSK_DINF_MIO_ROOT_OPTIMIZER_TIME                        = 43,
  /** Time spent presolving the problem at the root node (in seconds). */
  MSK_DINF_MIO_ROOT_PRESOLVE_TIME                         = 44,
  /** Time spent processing the root node (in seconds). */
  MSK_DINF_MIO_ROOT_TIME                                  = 45,
  /** Total time for symmetry detection (in seconds). */
  MSK_DINF_MIO_SYMMETRY_DETECTION_TIME                    = 46,
  /** Degree to which the problem is affected by detected symmetry. */
  MSK_DINF_MIO_SYMMETRY_FACTOR                            = 47,
  /** Time spent in the mixed-integer optimizer (in seconds). */
  MSK_DINF_MIO_TIME                                       = 48,
  /** If the objective cut is used, then this information item has the value of the cut. */
  MSK_DINF_MIO_USER_OBJ_CUT                               = 49,
  /** Total number of ticks spent in the optimizer since it was invoked. It is strictly negative if it is not available. */
  MSK_DINF_OPTIMIZER_TICKS                                = 50,
  /** Total time spent in the optimizer since it was invoked (in seconds). */
  MSK_DINF_OPTIMIZER_TIME                                 = 51,
  /** Total time spent in the eliminator since the presolve was invoked (in seconds). */
  MSK_DINF_PRESOLVE_ELI_TIME                              = 52,
  /** Total time spent  in the linear dependency checker since the presolve was invoked (in seconds). */
  MSK_DINF_PRESOLVE_LINDEP_TIME                           = 53,
  /** Total time spent in the presolve since it was invoked (in seconds). */
  MSK_DINF_PRESOLVE_TIME                                  = 54,
  /** Total perturbation of the bounds of the primal problem. */
  MSK_DINF_PRESOLVE_TOTAL_PRIMAL_PERTURBATION             = 55,
  /** The optimal objective value of the penalty function. */
  MSK_DINF_PRIMAL_REPAIR_PENALTY_OBJ                      = 56,
  /** Maximum absolute diagonal perturbation occurring during the QCQO reformulation. */
  MSK_DINF_QCQO_REFORMULATE_MAX_PERTURBATION              = 57,
  /** Time spent with conic quadratic reformulation (in seconds). */
  MSK_DINF_QCQO_REFORMULATE_TIME                          = 58,
  /** Worst Cholesky column scaling. */
  MSK_DINF_QCQO_REFORMULATE_WORST_CHOLESKY_COLUMN_SCALING = 59,
  /** Worst Cholesky diagonal scaling. */
  MSK_DINF_QCQO_REFORMULATE_WORST_CHOLESKY_DIAG_SCALING   = 60,
  /** Time spent reading the data file (in seconds). */
  MSK_DINF_READ_DATA_TIME                                 = 61,
  /** The total real time in seconds spent when optimizing on a server by the process performing the optimization on the server (in seconds). */
  MSK_DINF_REMOTE_TIME                                    = 62,
  /** Time spent in the dual simplex optimizer since invoking it (in seconds). */
  MSK_DINF_SIM_DUAL_TIME                                  = 63,
  /** Feasibility measure reported by the simplex optimizer. */
  MSK_DINF_SIM_FEAS                                       = 64,
  /** Objective value reported by the simplex optimizer. */
  MSK_DINF_SIM_OBJ                                        = 65,
  /** Time spent in the primal simplex optimizer since invoking it (in seconds). */
  MSK_DINF_SIM_PRIMAL_TIME                                = 66,
  /** Time spent in the simplex optimizer since invoking it (in seconds). */
  MSK_DINF_SIM_TIME                                       = 67,
  /** Dual objective value of the basic solution. Updated by the function updatesolutioninfo. */
  MSK_DINF_SOL_BAS_DUAL_OBJ                               = 68,
  /** Maximal dual bound violation for xx in the basic solution. Updated by the function updatesolutioninfo. */
  MSK_DINF_SOL_BAS_DVIOLCON                               = 69,
  /** Maximal dual bound violation for xx in the basic solution. Updated by the function updatesolutioninfo. */
  MSK_DINF_SOL_BAS_DVIOLVAR                               = 70,
  /** Infinity norm of barx in the basic solution. */
  MSK_DINF_SOL_BAS_NRM_BARX                               = 71,
  /** Infinity norm of slc in the basic solution. */
  MSK_DINF_SOL_BAS_NRM_SLC                                = 72,
  /** Infinity norm of slx in the basic solution. */
  MSK_DINF_SOL_BAS_NRM_SLX                                = 73,
  /** Infinity norm of suc in the basic solution. */
  MSK_DINF_SOL_BAS_NRM_SUC                                = 74,
  /** Infinity norm of sux in the basic solution. */
  MSK_DINF_SOL_BAS_NRM_SUX                                = 75,
  /** Infinity norm of xc in the basic solution. */
  MSK_DINF_SOL_BAS_NRM_XC                                 = 76,
  /** Infinity norm of xx in the basic solution. */
  MSK_DINF_SOL_BAS_NRM_XX                                 = 77,
  /** Infinity norm of Y in the basic solution. */
  MSK_DINF_SOL_BAS_NRM_Y                                  = 78,
  /** Primal objective value of the basic solution. Updated by the function updatesolutioninfo. */
  MSK_DINF_SOL_BAS_PRIMAL_OBJ                             = 79,
  /** Maximal primal bound violation for xc in the basic solution. Updated by the function updatesolutioninfo. */
  MSK_DINF_SOL_BAS_PVIOLCON                               = 80,
  /** Maximal primal bound violation for xx in the basic solution. Updated by the function updatesolutioninfo. */
  MSK_DINF_SOL_BAS_PVIOLVAR                               = 81,
  /** Infinity norm of barx in the integer solution. */
  MSK_DINF_SOL_ITG_NRM_BARX                               = 82,
  /** Infinity norm of xc in the integer solution. */
  MSK_DINF_SOL_ITG_NRM_XC                                 = 83,
  /** Infinity norm of xx in the integer solution. */
  MSK_DINF_SOL_ITG_NRM_XX                                 = 84,
  /** Primal objective value of the integer solution. Updated by the function updatesolutioninfo. */
  MSK_DINF_SOL_ITG_PRIMAL_OBJ                             = 85,
  /** Maximal primal violation for affine conic constraints in the integer solution. Updated by the function updatesolutioninfo. */
  MSK_DINF_SOL_ITG_PVIOLACC                               = 86,
  /** Maximal primal bound violation for barx in the integer solution. Updated by the function updatesolutioninfo. */
  MSK_DINF_SOL_ITG_PVIOLBARVAR                            = 87,
  /** Maximal primal bound violation for xc in the integer solution. Updated by the function updatesolutioninfo. */
  MSK_DINF_SOL_ITG_PVIOLCON                               = 88,
  /** Maximal primal violation for primal conic constraints in the integer solution. Updated by the function updatesolutioninfo. */
  MSK_DINF_SOL_ITG_PVIOLCONES                             = 89,
  /** Maximal primal violation for disjunctive constraints in the integer solution. Updated by the function updatesolutioninfo. */
  MSK_DINF_SOL_ITG_PVIOLDJC                               = 90,
  /** Maximal violation for the integer constraints in the integer solution. Updated by the function updatesolutioninfo. */
  MSK_DINF_SOL_ITG_PVIOLITG                               = 91,
  /** Maximal primal bound violation for xx in the integer solution. Updated by the function updatesolutioninfo. */
  MSK_DINF_SOL_ITG_PVIOLVAR                               = 92,
  /** Dual objective value of the interior-point solution. Updated by the function updatesolutioninfo. */
  MSK_DINF_SOL_ITR_DUAL_OBJ                               = 93,
  /** Maximal dual violation for affine conic constraints in the interior-point solution. Updated by the function updatesolutioninfo. */
  MSK_DINF_SOL_ITR_DVIOLACC                               = 94,
  /** Maximal dual bound violation for barx in the interior-point solution. Updated by the function updatesolutioninfo. */
  MSK_DINF_SOL_ITR_DVIOLBARVAR                            = 95,
  /** Maximal dual bound violation for xc in the interior-point solution. Updated by the function updatesolutioninfo. */
  MSK_DINF_SOL_ITR_DVIOLCON                               = 96,
  /** Maximal dual violation for conic constraints in the interior-point solution. Updated by the function updatesolutioninfo. */
  MSK_DINF_SOL_ITR_DVIOLCONES                             = 97,
  /** Maximal dual bound violation for xx in the interior-point solution. Updated by the function updatesolutioninfo. */
  MSK_DINF_SOL_ITR_DVIOLVAR                               = 98,
  /** Infinity norm of bars in the interior-point solution. */
  MSK_DINF_SOL_ITR_NRM_BARS                               = 99,
  /** Infinity norm of barx in the interior-point solution. */
  MSK_DINF_SOL_ITR_NRM_BARX                               = 100,
  /** Infinity norm of slc in the interior-point solution. */
  MSK_DINF_SOL_ITR_NRM_SLC                                = 101,
  /** Infinity norm of slx in the interior-point solution. */
  MSK_DINF_SOL_ITR_NRM_SLX                                = 102,
  /** Infinity norm of snx in the interior-point solution. */
  MSK_DINF_SOL_ITR_NRM_SNX                                = 103,
  /** Infinity norm of suc in the interior-point solution. */
  MSK_DINF_SOL_ITR_NRM_SUC                                = 104,
  /** Infinity norm of sux in the interior-point solution. */
  MSK_DINF_SOL_ITR_NRM_SUX                                = 105,
  /** Infinity norm of xc in the interior-point solution. */
  MSK_DINF_SOL_ITR_NRM_XC                                 = 106,
  /** Infinity norm of xx in the interior-point solution. */
  MSK_DINF_SOL_ITR_NRM_XX                                 = 107,
  /** Infinity norm of Y in the interior-point solution. */
  MSK_DINF_SOL_ITR_NRM_Y                                  = 108,
  /** Primal objective value of the interior-point solution. */
  MSK_DINF_SOL_ITR_PRIMAL_OBJ                             = 109,
  /** Maximal primal violation for affine conic constraints in the interior-point solution. Updated by the function updatesolutioninfo. */
  MSK_DINF_SOL_ITR_PVIOLACC                               = 110,
  /** Maximal primal bound violation for barx in the interior-point solution. Updated by the function updatesolutioninfo. */
  MSK_DINF_SOL_ITR_PVIOLBARVAR                            = 111,
  /** Maximal primal bound violation for xc in the interior-point solution. Updated by the function updatesolutioninfo. */
  MSK_DINF_SOL_ITR_PVIOLCON                               = 112,
  /** Maximal primal violation for conic constraints in the interior-point solution. Updated by the function updatesolutioninfo. */
  MSK_DINF_SOL_ITR_PVIOLCONES                             = 113,
  /** Maximal primal bound violation for xx in the interior-point solution. Updated by the function updatesolutioninfo. */
  MSK_DINF_SOL_ITR_PVIOLVAR                               = 114,
  /** Time spent in the last to conic reformulation (in seconds). */
  MSK_DINF_TO_CONIC_TIME                                  = 115,
  /** Time spent in basis identification dual phase after undualizing. */
  MSK_DINF_UNDUALIZE_BI_DUAL_TIME                         = 116,
  /** Time spent in basis identification initialization after undualizing. */
  MSK_DINF_UNDUALIZE_BI_INITIALIZE_TIME                   = 117,
  /** ime spent in basis identification optimizer using 128bit floating point precision undualizing. */
  MSK_DINF_UNDUALIZE_BI_OPTIMIZE_128BIT_TIME              = 118,
  /** Time spent in basis identification optimizer using 64bit floating point precision undualizing. */
  MSK_DINF_UNDUALIZE_BI_OPTIMIZE_TIME                     = 119,
  /** Time spent in basis identification primal phase after undualizing. */
  MSK_DINF_UNDUALIZE_BI_PRIMAL_TIME                       = 120,
  /** Time spent in basis identification after undualizing. */
  MSK_DINF_UNDUALIZE_BI_TIME                              = 121,
  /** Time spent in basis identification dual phase after unfolding. */
  MSK_DINF_UNFOLD_BI_DUAL_TIME                            = 122,
  /** Time spent in basis identification initialization after unfolding. */
  MSK_DINF_UNFOLD_BI_INITIALIZE_TIME                      = 123,
  /** Time spent in basis identification optimizer phase using 128bit floating point precision after unfolding. */
  MSK_DINF_UNFOLD_BI_OPTIMIZE_128BIT_TIME                 = 124,
  /** Time spent in basis identification optimizer phase using 64bit floating point precision after unfolding. */
  MSK_DINF_UNFOLD_BI_OPTIMIZE_TIME                        = 125,
  /** Time spent in basis identification primal phase after unfolding. */
  MSK_DINF_UNFOLD_BI_PRIMAL_TIME                          = 126,
  /** Time spent in basis identification after unfolding. */
  MSK_DINF_UNFOLD_BI_TIME                                 = 127,
  /** Time spent writing the data file (in seconds). */
  MSK_DINF_WRITE_DATA_TIME                                = 128
}; /* MSKdinfitem_enum */
#define MSK_DINF_BEGIN MSK_DINF_ANA_PRO_SCALARIZED_CONSTRAINT_MATRIX_DENSITY
#define MSK_DINF_END   (1+MSK_DINF_WRITE_DATA_TIME)
#ifdef MSK_NO_ENUMS
typedef int MSKdinfiteme;
#else
typedef enum MSKdinfitem_enum MSKdinfiteme;
#endif

enum MSKfeature_enum {
  /** Base system. */
  MSK_FEATURE_PTS  = 0,
  /** Conic extension. */
  MSK_FEATURE_PTON = 1
}; /* MSKfeature_enum */
#define MSK_FEATURE_BEGIN MSK_FEATURE_PTS
#define MSK_FEATURE_END   (1+MSK_FEATURE_PTON)
#ifdef MSK_NO_ENUMS
typedef int MSKfeaturee;
#else
typedef enum MSKfeature_enum MSKfeaturee;
#endif

enum MSKdparam_enum {
  /** If a constraint violates its bound with an amount larger than this value, the constraint name, index and violation will be printed by the solution analyzer. */
  MSK_DPAR_ANA_SOL_INFEAS_TOL                      = 0,
  /** Maximum relative dual bound violation allowed in an optimal basic solution. */
  MSK_DPAR_BASIS_REL_TOL_S                         = 1,
  /** Maximum absolute dual bound violation in an optimal basic solution. */
  MSK_DPAR_BASIS_TOL_S                             = 2,
  /** Maximum absolute primal bound violation allowed in an optimal basic solution. */
  MSK_DPAR_BASIS_TOL_X                             = 3,
  /** Zero tolerance threshold for symmetric matrices. */
  MSK_DPAR_DATA_SYM_MAT_TOL                        = 4,
  /** Data tolerance threshold. */
  MSK_DPAR_DATA_SYM_MAT_TOL_HUGE                   = 5,
  /** Data tolerance threshold. */
  MSK_DPAR_DATA_SYM_MAT_TOL_LARGE                  = 6,
  /** Data tolerance threshold. */
  MSK_DPAR_DATA_TOL_AIJ_HUGE                       = 7,
  /** Data tolerance threshold. */
  MSK_DPAR_DATA_TOL_AIJ_LARGE                      = 8,
  /** Data tolerance threshold. */
  MSK_DPAR_DATA_TOL_BOUND_INF                      = 9,
  /** Data tolerance threshold. */
  MSK_DPAR_DATA_TOL_BOUND_WRN                      = 10,
  /** Data tolerance threshold. */
  MSK_DPAR_DATA_TOL_C_HUGE                         = 11,
  /** Data tolerance threshold. */
  MSK_DPAR_DATA_TOL_CJ_LARGE                       = 12,
  /** Data tolerance threshold. */
  MSK_DPAR_DATA_TOL_QIJ                            = 13,
  /** Data tolerance threshold. */
  MSK_DPAR_DATA_TOL_X                              = 14,
  /** Tolerance for coefficient equality during folding. */
  MSK_DPAR_FOLDING_TOL_EQ                          = 15,
  /** Controls heartbeat frequency for the new simplex optimizers. */
  MSK_DPAR_HEARTBEAT_SIM_FREQ_TICKS                = 16,
  /** Dual feasibility tolerance used by the interior-point optimizer for conic problems. */
  MSK_DPAR_INTPNT_CO_TOL_DFEAS                     = 17,
  /** Infeasibility tolerance used by the interior-point optimizer for conic problems. */
  MSK_DPAR_INTPNT_CO_TOL_INFEAS                    = 18,
  /** Relative complementarity gap tolerance used by the interior-point optimizer for conic problems. */
  MSK_DPAR_INTPNT_CO_TOL_MU_RED                    = 19,
  /** Optimality tolerance used by the interior-point optimizer for conic problems. */
  MSK_DPAR_INTPNT_CO_TOL_NEAR_REL                  = 20,
  /** Primal feasibility tolerance used by the interior-point optimizer for conic problems. */
  MSK_DPAR_INTPNT_CO_TOL_PFEAS                     = 21,
  /** Relative gap termination tolerance used by the interior-point optimizer for conic problems. */
  MSK_DPAR_INTPNT_CO_TOL_REL_GAP                   = 22,
  /** Dual feasibility tolerance used by the interior-point optimizer for quadratic problems. */
  MSK_DPAR_INTPNT_QO_TOL_DFEAS                     = 23,
  /** Infeasibility tolerance used by the interior-point optimizer for quadratic problems. */
  MSK_DPAR_INTPNT_QO_TOL_INFEAS                    = 24,
  /** Relative complementarity gap tolerance used by the interior-point optimizer for quadratic problems. */
  MSK_DPAR_INTPNT_QO_TOL_MU_RED                    = 25,
  /** Optimality tolerance used by the interior-point optimizer for quadratic problems. */
  MSK_DPAR_INTPNT_QO_TOL_NEAR_REL                  = 26,
  /** Primal feasibility tolerance used by the interior-point optimizer for quadratic problems. */
  MSK_DPAR_INTPNT_QO_TOL_PFEAS                     = 27,
  /** Relative gap termination tolerance used by the interior-point optimizer for quadratic problems. */
  MSK_DPAR_INTPNT_QO_TOL_REL_GAP                   = 28,
  /** Dual feasibility tolerance used by the interior-point optimizer for linear problems. */
  MSK_DPAR_INTPNT_TOL_DFEAS                        = 29,
  /** Controls the interior-point dual starting point. */
  MSK_DPAR_INTPNT_TOL_DSAFE                        = 30,
  /** Infeasibility tolerance used by the interior-point optimizer for linear problems. */
  MSK_DPAR_INTPNT_TOL_INFEAS                       = 31,
  /** Relative complementarity gap tolerance used by the interior-point optimizer for linear problems. */
  MSK_DPAR_INTPNT_TOL_MU_RED                       = 32,
  /** Interior-point centering aggressiveness. */
  MSK_DPAR_INTPNT_TOL_PATH                         = 33,
  /** Primal feasibility tolerance used by the interior-point optimizer for linear problems. */
  MSK_DPAR_INTPNT_TOL_PFEAS                        = 34,
  /** Controls the interior-point primal starting point. */
  MSK_DPAR_INTPNT_TOL_PSAFE                        = 35,
  /** Relative gap termination tolerance used by the interior-point optimizer for linear problems. */
  MSK_DPAR_INTPNT_TOL_REL_GAP                      = 36,
  /** Relative step size to the boundary for linear and quadratic optimization problems. */
  MSK_DPAR_INTPNT_TOL_REL_STEP                     = 37,
  /** Minimal step size tolerance for the interior-point optimizer. */
  MSK_DPAR_INTPNT_TOL_STEP_SIZE                    = 38,
  /** Controls logging frequency for the new simplex optimizers. */
  MSK_DPAR_LOG_SIM_FREQ_TICKS                      = 39,
  /** Objective bound. */
  MSK_DPAR_LOWER_OBJ_CUT                           = 40,
  /** Objective bound. */
  MSK_DPAR_LOWER_OBJ_CUT_FINITE_TRH                = 41,
  /** Controlls the maximum size of the clique table as a factor of the number of nonzeros in the A matrix. */
  MSK_DPAR_MIO_CLIQUE_TABLE_SIZE_FACTOR            = 42,
  /** Maximum allowed big-M value when reformulating disjunctive constraints to linear constraints. */
  MSK_DPAR_MIO_DJC_MAX_BIGM                        = 43,
  /** Time limit for the mixed-integer optimizer. */
  MSK_DPAR_MIO_MAX_TIME                            = 44,
  /** This value is used to compute the relative gap for the solution to a mixed-integer optimization problem. */
  MSK_DPAR_MIO_REL_GAP_CONST                       = 45,
  /** Absolute optimality tolerance employed by the mixed-integer optimizer. */
  MSK_DPAR_MIO_TOL_ABS_GAP                         = 46,
  /** Integer feasibility tolerance. */
  MSK_DPAR_MIO_TOL_ABS_RELAX_INT                   = 47,
  /** Feasibility tolerance for mixed integer solver. */
  MSK_DPAR_MIO_TOL_FEAS                            = 48,
  /** Controls cut generation for mixed-integer optimizer. */
  MSK_DPAR_MIO_TOL_REL_DUAL_BOUND_IMPROVEMENT      = 49,
  /** Relative optimality tolerance employed by the mixed-integer optimizer. */
  MSK_DPAR_MIO_TOL_REL_GAP                         = 50,
  /** Solver ticks limit. */
  MSK_DPAR_OPTIMIZER_MAX_TICKS                     = 51,
  /** Solver time limit. */
  MSK_DPAR_OPTIMIZER_MAX_TIME                      = 52,
  /** Absolute tolerance employed by the linear dependency checker. */
  MSK_DPAR_PRESOLVE_TOL_ABS_LINDEP                 = 53,
  /** The presolve is allowed to perturb the objective by this amount if it removes a dual infeasibility. */
  MSK_DPAR_PRESOLVE_TOL_DUAL_INFEAS_PERTURBATION   = 54,
  /** The presolve is allowed to perturb a bound on a constraint or variable by this amount if it removes an infeasibility. */
  MSK_DPAR_PRESOLVE_TOL_PRIMAL_INFEAS_PERTURBATION = 55,
  /** Relative tolerance employed by the linear dependency checker. */
  MSK_DPAR_PRESOLVE_TOL_REL_LINDEP                 = 56,
  /** Absolute zero tolerance employed for slack variables in the presolve. */
  MSK_DPAR_PRESOLVE_TOL_S                          = 57,
  /** Absolute zero tolerance employed for variables in the presolve. */
  MSK_DPAR_PRESOLVE_TOL_X                          = 58,
  /** This parameter determines when columns are dropped in incomplete Cholesky factorization during reformulation of quadratic problems. */
  MSK_DPAR_QCQO_REFORMULATE_REL_DROP_TOL           = 59,
  /** Tolerance to define a matrix to be positive semidefinite. */
  MSK_DPAR_SEMIDEFINITE_TOL_APPROX                 = 60,
  /** Relative pivot tolerance employed when computing the LU factorization of the basis matrix. */
  MSK_DPAR_SIM_LU_TOL_REL_PIV                      = 61,
  /** Experimental. Usage not recommended. */
  MSK_DPAR_SIM_PRECISION_SCALING_EXTENDED          = 62,
  /** Experimental. Usage not recommended. */
  MSK_DPAR_SIM_PRECISION_SCALING_NORMAL            = 63,
  /** Absolute pivot tolerance employed by the simplex optimizers. */
  MSK_DPAR_SIMPLEX_ABS_TOL_PIV                     = 64,
  /** Objective bound. */
  MSK_DPAR_UPPER_OBJ_CUT                           = 65,
  /** Objective bound. */
  MSK_DPAR_UPPER_OBJ_CUT_FINITE_TRH                = 66
}; /* MSKdparam_enum */
#define MSK_DPAR_BEGIN MSK_DPAR_ANA_SOL_INFEAS_TOL
#define MSK_DPAR_END   (1+MSK_DPAR_UPPER_OBJ_CUT_FINITE_TRH)
#ifdef MSK_NO_ENUMS
typedef int MSKdparame;
#else
typedef enum MSKdparam_enum MSKdparame;
#endif

#define MSK_DPAR_ANA_SOL_INFEAS_TOL_                 "MSK_DPAR_ANA_SOL_INFEAS_TOL"
#define MSK_DPAR_BASIS_REL_TOL_S_                    "MSK_DPAR_BASIS_REL_TOL_S"
#define MSK_DPAR_BASIS_TOL_S_                        "MSK_DPAR_BASIS_TOL_S"
#define MSK_DPAR_BASIS_TOL_X_                        "MSK_DPAR_BASIS_TOL_X"
#define MSK_DPAR_DATA_SYM_MAT_TOL_                   "MSK_DPAR_DATA_SYM_MAT_TOL"
#define MSK_DPAR_DATA_SYM_MAT_TOL_HUGE_              "MSK_DPAR_DATA_SYM_MAT_TOL_HUGE"
#define MSK_DPAR_DATA_SYM_MAT_TOL_LARGE_             "MSK_DPAR_DATA_SYM_MAT_TOL_LARGE"
#define MSK_DPAR_DATA_TOL_AIJ_HUGE_                  "MSK_DPAR_DATA_TOL_AIJ_HUGE"
#define MSK_DPAR_DATA_TOL_AIJ_LARGE_                 "MSK_DPAR_DATA_TOL_AIJ_LARGE"
#define MSK_DPAR_DATA_TOL_BOUND_INF_                 "MSK_DPAR_DATA_TOL_BOUND_INF"
#define MSK_DPAR_DATA_TOL_BOUND_WRN_                 "MSK_DPAR_DATA_TOL_BOUND_WRN"
#define MSK_DPAR_DATA_TOL_C_HUGE_                    "MSK_DPAR_DATA_TOL_C_HUGE"
#define MSK_DPAR_DATA_TOL_CJ_LARGE_                  "MSK_DPAR_DATA_TOL_CJ_LARGE"
#define MSK_DPAR_DATA_TOL_QIJ_                       "MSK_DPAR_DATA_TOL_QIJ"
#define MSK_DPAR_DATA_TOL_X_                         "MSK_DPAR_DATA_TOL_X"
#define MSK_DPAR_FOLDING_TOL_EQ_                     "MSK_DPAR_FOLDING_TOL_EQ"
#define MSK_DPAR_HEARTBEAT_SIM_FREQ_TICKS_           "MSK_DPAR_HEARTBEAT_SIM_FREQ_TICKS"
#define MSK_DPAR_INTPNT_CO_TOL_DFEAS_                "MSK_DPAR_INTPNT_CO_TOL_DFEAS"
#define MSK_DPAR_INTPNT_CO_TOL_INFEAS_               "MSK_DPAR_INTPNT_CO_TOL_INFEAS"
#define MSK_DPAR_INTPNT_CO_TOL_MU_RED_               "MSK_DPAR_INTPNT_CO_TOL_MU_RED"
#define MSK_DPAR_INTPNT_CO_TOL_NEAR_REL_             "MSK_DPAR_INTPNT_CO_TOL_NEAR_REL"
#define MSK_DPAR_INTPNT_CO_TOL_PFEAS_                "MSK_DPAR_INTPNT_CO_TOL_PFEAS"
#define MSK_DPAR_INTPNT_CO_TOL_REL_GAP_              "MSK_DPAR_INTPNT_CO_TOL_REL_GAP"
#define MSK_DPAR_INTPNT_QO_TOL_DFEAS_                "MSK_DPAR_INTPNT_QO_TOL_DFEAS"
#define MSK_DPAR_INTPNT_QO_TOL_INFEAS_               "MSK_DPAR_INTPNT_QO_TOL_INFEAS"
#define MSK_DPAR_INTPNT_QO_TOL_MU_RED_               "MSK_DPAR_INTPNT_QO_TOL_MU_RED"
#define MSK_DPAR_INTPNT_QO_TOL_NEAR_REL_             "MSK_DPAR_INTPNT_QO_TOL_NEAR_REL"
#define MSK_DPAR_INTPNT_QO_TOL_PFEAS_                "MSK_DPAR_INTPNT_QO_TOL_PFEAS"
#define MSK_DPAR_INTPNT_QO_TOL_REL_GAP_              "MSK_DPAR_INTPNT_QO_TOL_REL_GAP"
#define MSK_DPAR_INTPNT_TOL_DFEAS_                   "MSK_DPAR_INTPNT_TOL_DFEAS"
#define MSK_DPAR_INTPNT_TOL_DSAFE_                   "MSK_DPAR_INTPNT_TOL_DSAFE"
#define MSK_DPAR_INTPNT_TOL_INFEAS_                  "MSK_DPAR_INTPNT_TOL_INFEAS"
#define MSK_DPAR_INTPNT_TOL_MU_RED_                  "MSK_DPAR_INTPNT_TOL_MU_RED"
#define MSK_DPAR_INTPNT_TOL_PATH_                    "MSK_DPAR_INTPNT_TOL_PATH"
#define MSK_DPAR_INTPNT_TOL_PFEAS_                   "MSK_DPAR_INTPNT_TOL_PFEAS"
#define MSK_DPAR_INTPNT_TOL_PSAFE_                   "MSK_DPAR_INTPNT_TOL_PSAFE"
#define MSK_DPAR_INTPNT_TOL_REL_GAP_                 "MSK_DPAR_INTPNT_TOL_REL_GAP"
#define MSK_DPAR_INTPNT_TOL_REL_STEP_                "MSK_DPAR_INTPNT_TOL_REL_STEP"
#define MSK_DPAR_INTPNT_TOL_STEP_SIZE_               "MSK_DPAR_INTPNT_TOL_STEP_SIZE"
#define MSK_DPAR_LOG_SIM_FREQ_TICKS_                 "MSK_DPAR_LOG_SIM_FREQ_TICKS"
#define MSK_DPAR_LOWER_OBJ_CUT_                      "MSK_DPAR_LOWER_OBJ_CUT"
#define MSK_DPAR_LOWER_OBJ_CUT_FINITE_TRH_           "MSK_DPAR_LOWER_OBJ_CUT_FINITE_TRH"
#define MSK_DPAR_MIO_CLIQUE_TABLE_SIZE_FACTOR_       "MSK_DPAR_MIO_CLIQUE_TABLE_SIZE_FACTOR"
#define MSK_DPAR_MIO_DJC_MAX_BIGM_                   "MSK_DPAR_MIO_DJC_MAX_BIGM"
#define MSK_DPAR_MIO_MAX_TIME_                       "MSK_DPAR_MIO_MAX_TIME"
#define MSK_DPAR_MIO_REL_GAP_CONST_                  "MSK_DPAR_MIO_REL_GAP_CONST"
#define MSK_DPAR_MIO_TOL_ABS_GAP_                    "MSK_DPAR_MIO_TOL_ABS_GAP"
#define MSK_DPAR_MIO_TOL_ABS_RELAX_INT_              "MSK_DPAR_MIO_TOL_ABS_RELAX_INT"
#define MSK_DPAR_MIO_TOL_FEAS_                       "MSK_DPAR_MIO_TOL_FEAS"
#define MSK_DPAR_MIO_TOL_REL_DUAL_BOUND_IMPROVEMENT_ "MSK_DPAR_MIO_TOL_REL_DUAL_BOUND_IMPROVEMENT"
#define MSK_DPAR_MIO_TOL_REL_GAP_                    "MSK_DPAR_MIO_TOL_REL_GAP"
#define MSK_DPAR_OPTIMIZER_MAX_TICKS_                "MSK_DPAR_OPTIMIZER_MAX_TICKS"
#define MSK_DPAR_OPTIMIZER_MAX_TIME_                 "MSK_DPAR_OPTIMIZER_MAX_TIME"
#define MSK_DPAR_PRESOLVE_TOL_ABS_LINDEP_            "MSK_DPAR_PRESOLVE_TOL_ABS_LINDEP"
#define MSK_DPAR_PRESOLVE_TOL_DUAL_INFEAS_PERTURBATION_ "MSK_DPAR_PRESOLVE_TOL_DUAL_INFEAS_PERTURBATION"
#define MSK_DPAR_PRESOLVE_TOL_PRIMAL_INFEAS_PERTURBATION_ "MSK_DPAR_PRESOLVE_TOL_PRIMAL_INFEAS_PERTURBATION"
#define MSK_DPAR_PRESOLVE_TOL_REL_LINDEP_            "MSK_DPAR_PRESOLVE_TOL_REL_LINDEP"
#define MSK_DPAR_PRESOLVE_TOL_S_                     "MSK_DPAR_PRESOLVE_TOL_S"
#define MSK_DPAR_PRESOLVE_TOL_X_                     "MSK_DPAR_PRESOLVE_TOL_X"
#define MSK_DPAR_QCQO_REFORMULATE_REL_DROP_TOL_      "MSK_DPAR_QCQO_REFORMULATE_REL_DROP_TOL"
#define MSK_DPAR_SEMIDEFINITE_TOL_APPROX_            "MSK_DPAR_SEMIDEFINITE_TOL_APPROX"
#define MSK_DPAR_SIM_LU_TOL_REL_PIV_                 "MSK_DPAR_SIM_LU_TOL_REL_PIV"
#define MSK_DPAR_SIM_PRECISION_SCALING_EXTENDED_     "MSK_DPAR_SIM_PRECISION_SCALING_EXTENDED"
#define MSK_DPAR_SIM_PRECISION_SCALING_NORMAL_       "MSK_DPAR_SIM_PRECISION_SCALING_NORMAL"
#define MSK_DPAR_SIMPLEX_ABS_TOL_PIV_                "MSK_DPAR_SIMPLEX_ABS_TOL_PIV"
#define MSK_DPAR_UPPER_OBJ_CUT_                      "MSK_DPAR_UPPER_OBJ_CUT"
#define MSK_DPAR_UPPER_OBJ_CUT_FINITE_TRH_           "MSK_DPAR_UPPER_OBJ_CUT_FINITE_TRH"

enum MSKliinfitem_enum {
  /** Number of columns in the scalarized constraint matrix. */
  MSK_LIINF_ANA_PRO_SCALARIZED_CONSTRAINT_MATRIX_NUM_COLUMNS = 0,
  /** Number of non-zero entries in the scalarized constraint matrix. */
  MSK_LIINF_ANA_PRO_SCALARIZED_CONSTRAINT_MATRIX_NUM_NZ      = 1,
  /** Number of rows in the scalarized constraint matrix. */
  MSK_LIINF_ANA_PRO_SCALARIZED_CONSTRAINT_MATRIX_NUM_ROWS    = 2,
  /** Number of clean iterations performed in the basis identification. */
  MSK_LIINF_BI_CLEAN_ITER                                    = 3,
  /** Number of dual pivots performed in the basis identification. */
  MSK_LIINF_BI_DUAL_ITER                                     = 4,
  /** Number of primal pivots performed in the basis identification. */
  MSK_LIINF_BI_PRIMAL_ITER                                   = 5,
  /** Number of non-zeros in factorization. */
  MSK_LIINF_INTPNT_FACTOR_NUM_NZ                             = 6,
  /** Number of non-zero entries in the constraint matrix of the problem to be solved by the mixed-integer optimizer. */
  MSK_LIINF_MIO_ANZ                                          = 7,
  /** Number of non-zero entries in the constraint matrix of the mixed-integer optimizer's final problem. */
  MSK_LIINF_MIO_FINAL_ANZ                                    = 8,
  /** Number of interior-point iterations performed by the mixed-integer optimizer. */
  MSK_LIINF_MIO_INTPNT_ITER                                  = 9,
  /** Number of dual illposed certificates encountered by the mixed-integer optimizer. */
  MSK_LIINF_MIO_NUM_DUAL_ILLPOSED_CER                        = 10,
  /** Number of primal illposed certificates encountered by the mixed-integer optimizer. */
  MSK_LIINF_MIO_NUM_PRIM_ILLPOSED_CER                        = 11,
  /** Number of non-zero entries in the constraint matrix of the problem after the mixed-integer optimizer's presolve. */
  MSK_LIINF_MIO_PRESOLVED_ANZ                                = 12,
  /** Number of simplex iterations performed by the mixed-integer optimizer. */
  MSK_LIINF_MIO_SIMPLEX_ITER                                 = 13,
  /** Number of affince conic constraints. */
  MSK_LIINF_RD_NUMACC                                        = 14,
  /** Number of non-zeros in A that is read. */
  MSK_LIINF_RD_NUMANZ                                        = 15,
  /** Number of disjuncive constraints. */
  MSK_LIINF_RD_NUMDJC                                        = 16,
  /** Number of Q non-zeros. */
  MSK_LIINF_RD_NUMQNZ                                        = 17,
  /** Number of iterations performed by the simplex optimizer. */
  MSK_LIINF_SIMPLEX_ITER                                     = 18,
  /** Number of dual pivots performed in the basis identification. */
  MSK_LIINF_UNDUALIZE_BI_DUAL_ITER                           = 19,
  /** Number of clean iterations performed in the basis identification. */
  MSK_LIINF_UNDUALIZE_BI_OPTIMIZE_ITER                       = 20,
  /** Number of clean iterations performed in the basis identification. */
  MSK_LIINF_UNDUALIZE_BI_OPTIMIZE_ITER_128BIT                = 21,
  /** Number of primal pivots performed in the basis identification. */
  MSK_LIINF_UNDUALIZE_BI_PRIMAL_ITER                         = 22,
  /** Number of dual iterations pivots performed in the basis identification after unfolding. */
  MSK_LIINF_UNFOLD_BI_DUAL_ITER                              = 23,
  /** Number of clean iterations performed in the basis identification. */
  MSK_LIINF_UNFOLD_BI_OPTIMIZE_128BIT_ITER                   = 24,
  /** Number of clean iterations performed in the basis identification. */
  MSK_LIINF_UNFOLD_BI_OPTIMIZE_ITER                          = 25,
  /** Number of reoptimization iterations performed in the basis identification after unfolding. */
  MSK_LIINF_UNFOLD_BI_OPTIMIZER_ITER                         = 26,
  /** Number of primal iterations performed in the basis identification after unfolding. */
  MSK_LIINF_UNFOLD_BI_PRIMAL_ITER                            = 27
}; /* MSKliinfitem_enum */
#define MSK_LIINF_BEGIN MSK_LIINF_ANA_PRO_SCALARIZED_CONSTRAINT_MATRIX_NUM_COLUMNS
#define MSK_LIINF_END   (1+MSK_LIINF_UNFOLD_BI_PRIMAL_ITER)
#ifdef MSK_NO_ENUMS
typedef int MSKliinfiteme;
#else
typedef enum MSKliinfitem_enum MSKliinfiteme;
#endif

enum MSKiinfitem_enum {
  /** Number of constraints in the problem. */
  MSK_IINF_ANA_PRO_NUM_CON                       = 0,
  /** Number of equality constraints. */
  MSK_IINF_ANA_PRO_NUM_CON_EQ                    = 1,
  /** Number of unbounded constraints. */
  MSK_IINF_ANA_PRO_NUM_CON_FR                    = 2,
  /** Number of constraints with a lower bound and an infinite upper bound. */
  MSK_IINF_ANA_PRO_NUM_CON_LO                    = 3,
  /** Number of constraints with finite lower and upper bounds. */
  MSK_IINF_ANA_PRO_NUM_CON_RA                    = 4,
  /** Number of constraints with an upper bound and an infinite lower bound. */
  MSK_IINF_ANA_PRO_NUM_CON_UP                    = 5,
  /** Number of variables in the problem. */
  MSK_IINF_ANA_PRO_NUM_VAR                       = 6,
  /** Number of binary variables. */
  MSK_IINF_ANA_PRO_NUM_VAR_BIN                   = 7,
  /** Number of continuous variables. */
  MSK_IINF_ANA_PRO_NUM_VAR_CONT                  = 8,
  /** Number of fixed variables. */
  MSK_IINF_ANA_PRO_NUM_VAR_EQ                    = 9,
  /** Number of unbounded constraints. */
  MSK_IINF_ANA_PRO_NUM_VAR_FR                    = 10,
  /** Number of general integer variables. */
  MSK_IINF_ANA_PRO_NUM_VAR_INT                   = 11,
  /** Number of variables with a lower bound and an infinite upper bound. */
  MSK_IINF_ANA_PRO_NUM_VAR_LO                    = 12,
  /** Number of variables with finite lower and upper bounds. */
  MSK_IINF_ANA_PRO_NUM_VAR_RA                    = 13,
  /** Number of variables with an upper bound and an infinite lower bound. */
  MSK_IINF_ANA_PRO_NUM_VAR_UP                    = 14,
  /** Non-zero if folding was exploited. */
  MSK_IINF_FOLDING_APPLIED                       = 15,
  /** Dimension of the dense sub system in factorization. */
  MSK_IINF_INTPNT_FACTOR_DIM_DENSE               = 16,
  /** Number of interior-point iterations since invoking the interior-point optimizer. */
  MSK_IINF_INTPNT_ITER                           = 17,
  /** Number of threads that the interior-point optimizer is using. */
  MSK_IINF_INTPNT_NUM_THREADS                    = 18,
  /** Non-zero if the interior-point optimizer is solving the dual problem. */
  MSK_IINF_INTPNT_SOLVE_DUAL                     = 19,
  /** Non-zero if absolute gap is within tolerances. */
  MSK_IINF_MIO_ABSGAP_SATISFIED                  = 20,
  /** Size of the clique table. */
  MSK_IINF_MIO_CLIQUE_TABLE_SIZE                 = 21,
  /** Informs if MOSEK successfully constructed an initial integer feasible solution. */
  MSK_IINF_MIO_CONSTRUCT_SOLUTION                = 22,
  /** Number of binary variables in the mixed-integer optimizer's final problem. */
  MSK_IINF_MIO_FINAL_NUMBIN                      = 23,
  /** Number of binary cone variables in the mixed-integer optimizer's final problem. */
  MSK_IINF_MIO_FINAL_NUMBINCONEVAR               = 24,
  /** Number of constraints in the mixed-integer optimizer's final problem. */
  MSK_IINF_MIO_FINAL_NUMCON                      = 25,
  /** Number of cones in the mixed-integer optimizer's final problem. */
  MSK_IINF_MIO_FINAL_NUMCONE                     = 26,
  /** Number of cone variables in the mixed-integer optimizer's final problem. */
  MSK_IINF_MIO_FINAL_NUMCONEVAR                  = 27,
  /** Number of continuous variables in the mixed-integer optimizer's final problem. */
  MSK_IINF_MIO_FINAL_NUMCONT                     = 28,
  /** Number of continuous cone variables in the mixed-integer optimizer's final problem. */
  MSK_IINF_MIO_FINAL_NUMCONTCONEVAR              = 29,
  /** Number of dual exponential cones in the mixed-integer optimizer's final problem. */
  MSK_IINF_MIO_FINAL_NUMDEXPCONES                = 30,
  /** Number of disjunctive constraints in the mixed-integer optimizer's final problem. */
  MSK_IINF_MIO_FINAL_NUMDJC                      = 31,
  /** Number of dual power cones in the mixed-integer optimizer's final problem. */
  MSK_IINF_MIO_FINAL_NUMDPOWCONES                = 32,
  /** Number of integer variables in the mixed-integer optimizer's final problem. */
  MSK_IINF_MIO_FINAL_NUMINT                      = 33,
  /** Number of integer cone variables in the mixed-integer optimizer's final problem. */
  MSK_IINF_MIO_FINAL_NUMINTCONEVAR               = 34,
  /** Number of primal exponential cones in the mixed-integer optimizer's final problem. */
  MSK_IINF_MIO_FINAL_NUMPEXPCONES                = 35,
  /** Number of primal power cones in the mixed-integer optimizer's final problem. */
  MSK_IINF_MIO_FINAL_NUMPPOWCONES                = 36,
  /** Number of quadratic cones in the mixed-integer optimizer's final problem. */
  MSK_IINF_MIO_FINAL_NUMQCONES                   = 37,
  /** Number of rotated quadratic cones in the mixed-integer optimizer's final problem. */
  MSK_IINF_MIO_FINAL_NUMRQCONES                  = 38,
  /** Number of variables in the mixed-integer optimizer's final problem. */
  MSK_IINF_MIO_FINAL_NUMVAR                      = 39,
  /** Informs if MOSEK found the solution provided by the user to be feasible */
  MSK_IINF_MIO_INITIAL_FEASIBLE_SOLUTION         = 40,
  /** Depth of the last node solved. */
  MSK_IINF_MIO_NODE_DEPTH                        = 41,
  /** Number of active branch and bound nodes. */
  MSK_IINF_MIO_NUM_ACTIVE_NODES                  = 42,
  /** Number of active cuts in the final relaxation after the mixed-integer optimizer's root cut generation. */
  MSK_IINF_MIO_NUM_ACTIVE_ROOT_CUTS              = 43,
  /** Number of independent decomposition blocks solved though a dedicated algorithm. */
  MSK_IINF_MIO_NUM_BLOCKS_SOLVED_IN_BB           = 44,
  /** Number of independent decomposition blocks solved during presolve. */
  MSK_IINF_MIO_NUM_BLOCKS_SOLVED_IN_PRESOLVE     = 45,
  /** Number of branches performed during the optimization. */
  MSK_IINF_MIO_NUM_BRANCH                        = 46,
  /** Number of integer feasible solutions that have been found. */
  MSK_IINF_MIO_NUM_INT_SOLUTIONS                 = 47,
  /** Number of relaxations solved during the optimization. */
  MSK_IINF_MIO_NUM_RELAX                         = 48,
  /** Number of times presolve was repeated at root. */
  MSK_IINF_MIO_NUM_REPEATED_PRESOLVE             = 49,
  /** Number of restarts performed during the optimization. */
  MSK_IINF_MIO_NUM_RESTARTS                      = 50,
  /** Number of cut separation rounds at the root node of the mixed-integer optimizer. */
  MSK_IINF_MIO_NUM_ROOT_CUT_ROUNDS               = 51,
  /** Number of clique cuts selected to be included in the relaxation. */
  MSK_IINF_MIO_NUM_SELECTED_CLIQUE_CUTS          = 52,
  /** Number of Complemented Mixed Integer Rounding (CMIR) cuts selected to be included in the relaxation. */
  MSK_IINF_MIO_NUM_SELECTED_CMIR_CUTS            = 53,
  /** Number of Gomory cuts selected to be included in the relaxation. */
  MSK_IINF_MIO_NUM_SELECTED_GOMORY_CUTS          = 54,
  /** Number of implied bound cuts selected to be included in the relaxation. */
  MSK_IINF_MIO_NUM_SELECTED_IMPLIED_BOUND_CUTS   = 55,
  /** Number of clique cuts selected to be included in the relaxation. */
  MSK_IINF_MIO_NUM_SELECTED_KNAPSACK_COVER_CUTS  = 56,
  /** Number of lift-and-project cuts selected to be included in the relaxation. */
  MSK_IINF_MIO_NUM_SELECTED_LIPRO_CUTS           = 57,
  /** Number of separated clique cuts. */
  MSK_IINF_MIO_NUM_SEPARATED_CLIQUE_CUTS         = 58,
  /** Number of separated Complemented Mixed Integer Rounding (CMIR) cuts. */
  MSK_IINF_MIO_NUM_SEPARATED_CMIR_CUTS           = 59,
  /** Number of separated Gomory cuts. */
  MSK_IINF_MIO_NUM_SEPARATED_GOMORY_CUTS         = 60,
  /** Number of separated implied bound cuts. */
  MSK_IINF_MIO_NUM_SEPARATED_IMPLIED_BOUND_CUTS  = 61,
  /** Number of separated clique cuts. */
  MSK_IINF_MIO_NUM_SEPARATED_KNAPSACK_COVER_CUTS = 62,
  /** Number of separated lift-and-project cuts. */
  MSK_IINF_MIO_NUM_SEPARATED_LIPRO_CUTS          = 63,
  /** Number of branch and bounds nodes solved in the main branch and bound tree. */
  MSK_IINF_MIO_NUM_SOLVED_NODES                  = 64,
  /** Number of binary variables in the problem to be solved by the mixed-integer optimizer. */
  MSK_IINF_MIO_NUMBIN                            = 65,
  /** Number of binary cone variables in the problem to be solved by the mixed-integer optimizer. */
  MSK_IINF_MIO_NUMBINCONEVAR                     = 66,
  /** Number of constraints in the problem to be solved by the mixed-integer optimizer. */
  MSK_IINF_MIO_NUMCON                            = 67,
  /** Number of cones in the problem to be solved by the mixed-integer optimizer. */
  MSK_IINF_MIO_NUMCONE                           = 68,
  /** Number of cone variables in the problem to be solved by the mixed-integer optimizer. */
  MSK_IINF_MIO_NUMCONEVAR                        = 69,
  /** Number of continuous variables in the problem to be solved by the mixed-integer optimizer. */
  MSK_IINF_MIO_NUMCONT                           = 70,
  /** Number of continuous cone variables in the problem to be solved by the mixed-integer optimizer. */
  MSK_IINF_MIO_NUMCONTCONEVAR                    = 71,
  /** Number of dual exponential cones in the problem to be solved by the mixed-integer optimizer. */
  MSK_IINF_MIO_NUMDEXPCONES                      = 72,
  /** Number of disjunctive constraints in the problem to be solved by the mixed-integer optimizer. */
  MSK_IINF_MIO_NUMDJC                            = 73,
  /** Number of dual power cones in the problem to be solved by the mixed-integer optimizer. */
  MSK_IINF_MIO_NUMDPOWCONES                      = 74,
  /** Number of integer variables in the problem to be solved by the mixed-integer optimizer. */
  MSK_IINF_MIO_NUMINT                            = 75,
  /** Number of integer cone variables in the problem to be solved by the mixed-integer optimizer. */
  MSK_IINF_MIO_NUMINTCONEVAR                     = 76,
  /** Number of primal exponential cones in the problem to be solved by the mixed-integer optimizer. */
  MSK_IINF_MIO_NUMPEXPCONES                      = 77,
  /** Number of primal power cones in the problem to be solved by the mixed-integer optimizer. */
  MSK_IINF_MIO_NUMPPOWCONES                      = 78,
  /** Number of quadratic cones in the problem to be solved by the mixed-integer optimizer. */
  MSK_IINF_MIO_NUMQCONES                         = 79,
  /** Number of rotated quadratic cones in the problem to be solved by the mixed-integer optimizer. */
  MSK_IINF_MIO_NUMRQCONES                        = 80,
  /** Number of variables in the problem to be solved by the mixed-integer optimizer. */
  MSK_IINF_MIO_NUMVAR                            = 81,
  /** Non-zero if a valid objective bound has been found, otherwise zero. */
  MSK_IINF_MIO_OBJ_BOUND_DEFINED                 = 82,
  /** Number of binary variables in the problem after the mixed-integer optimizer's presolve. */
  MSK_IINF_MIO_PRESOLVED_NUMBIN                  = 83,
  /** Number of binary cone variables in the problem after the mixed-integer optimizer's presolve. */
  MSK_IINF_MIO_PRESOLVED_NUMBINCONEVAR           = 84,
  /** Number of constraints in the problem after the mixed-integer optimizer's presolve. */
  MSK_IINF_MIO_PRESOLVED_NUMCON                  = 85,
  /** Number of cones in the problem after the mixed-integer optimizer's presolve. */
  MSK_IINF_MIO_PRESOLVED_NUMCONE                 = 86,
  /** Number of cone variables in the problem after the mixed-integer optimizer's presolve. */
  MSK_IINF_MIO_PRESOLVED_NUMCONEVAR              = 87,
  /** Number of continuous variables in the problem after the mixed-integer optimizer's presolve. */
  MSK_IINF_MIO_PRESOLVED_NUMCONT                 = 88,
  /** Number of continuous cone variables in the problem after the mixed-integer optimizer's presolve. */
  MSK_IINF_MIO_PRESOLVED_NUMCONTCONEVAR          = 89,
  /** Number of dual exponential cones in the problem after the mixed-integer optimizer's presolve. */
  MSK_IINF_MIO_PRESOLVED_NUMDEXPCONES            = 90,
  /** Number of disjunctive constraints in the problem after the mixed-integer optimizer's presolve. */
  MSK_IINF_MIO_PRESOLVED_NUMDJC                  = 91,
  /** Number of dual power cones in the problem after the mixed-integer optimizer's presolve. */
  MSK_IINF_MIO_PRESOLVED_NUMDPOWCONES            = 92,
  /** Number of integer variables in the problem after the mixed-integer optimizer's presolve. */
  MSK_IINF_MIO_PRESOLVED_NUMINT                  = 93,
  /** Number of integer cone variables in the problem after the mixed-integer optimizer's presolve. */
  MSK_IINF_MIO_PRESOLVED_NUMINTCONEVAR           = 94,
  /** Number of primal exponential cones in the problem after the mixed-integer optimizer's presolve. */
  MSK_IINF_MIO_PRESOLVED_NUMPEXPCONES            = 95,
  /** Number of primal power cones in the problem after the mixed-integer optimizer's presolve. */
  MSK_IINF_MIO_PRESOLVED_NUMPPOWCONES            = 96,
  /** Number of quadratic cones in the problem after the mixed-integer optimizer's presolve. */
  MSK_IINF_MIO_PRESOLVED_NUMQCONES               = 97,
  /** Number of rotated quadratic cones in the problem after the mixed-integer optimizer's presolve. */
  MSK_IINF_MIO_PRESOLVED_NUMRQCONES              = 98,
  /** Number of variables in the problem after the mixed-integer optimizer's presolve. */
  MSK_IINF_MIO_PRESOLVED_NUMVAR                  = 99,
  /** Non-zero if relative gap is within tolerances. */
  MSK_IINF_MIO_RELGAP_SATISFIED                  = 100,
  /** Total number of cuts selected to be included in the relaxation by the mixed-integer optimizer. */
  MSK_IINF_MIO_TOTAL_NUM_SELECTED_CUTS           = 101,
  /** Total number of cuts separated by the mixed-integer optimizer. */
  MSK_IINF_MIO_TOTAL_NUM_SEPARATED_CUTS          = 102,
  /** If it is non-zero, then the objective cut is used. */
  MSK_IINF_MIO_USER_OBJ_CUT                      = 103,
  /** Number of constraints in the problem solved when the optimizer is called. */
  MSK_IINF_OPT_NUMCON                            = 104,
  /** Number of variables in the problem solved when the optimizer is called */
  MSK_IINF_OPT_NUMVAR                            = 105,
  /** The response code returned by optimize. */
  MSK_IINF_OPTIMIZE_RESPONSE                     = 106,
  /** Number perturbations to thhe bounds of the primal problem. */
  MSK_IINF_PRESOLVE_NUM_PRIMAL_PERTURBATIONS     = 107,
  /** Is nonzero if the dual solution is purified. */
  MSK_IINF_PURIFY_DUAL_SUCCESS                   = 108,
  /** Is nonzero if the primal solution is purified. */
  MSK_IINF_PURIFY_PRIMAL_SUCCESS                 = 109,
  /** Number of symmetric variables read. */
  MSK_IINF_RD_NUMBARVAR                          = 110,
  /** Number of constraints read. */
  MSK_IINF_RD_NUMCON                             = 111,
  /** Number of conic constraints read. */
  MSK_IINF_RD_NUMCONE                            = 112,
  /** Number of integer-constrained variables read. */
  MSK_IINF_RD_NUMINTVAR                          = 113,
  /** Number of nonempty Q matrices read. */
  MSK_IINF_RD_NUMQ                               = 114,
  /** Number of variables read. */
  MSK_IINF_RD_NUMVAR                             = 115,
  /** Problem type. */
  MSK_IINF_RD_PROTYPE                            = 116,
  /** The number of dual degenerate iterations. */
  MSK_IINF_SIM_DUAL_DEG_ITER                     = 117,
  /** If 1 then the dual simplex algorithm is solving from an advanced basis. */
  MSK_IINF_SIM_DUAL_HOTSTART                     = 118,
  /** If 1 then a valid basis factorization of full rank was located and used by the dual simplex algorithm. */
  MSK_IINF_SIM_DUAL_HOTSTART_LU                  = 119,
  /** The number of iterations taken with dual infeasibility. */
  MSK_IINF_SIM_DUAL_INF_ITER                     = 120,
  /** Number of dual simplex iterations during the last optimization. */
  MSK_IINF_SIM_DUAL_ITER                         = 121,
  /** Number of constraints in the problem solved by the simplex optimizer. */
  MSK_IINF_SIM_NUMCON                            = 122,
  /** Number of variables in the problem solved by the simplex optimizer. */
  MSK_IINF_SIM_NUMVAR                            = 123,
  /** The number of primal degenerate iterations. */
  MSK_IINF_SIM_PRIMAL_DEG_ITER                   = 124,
  /** If 1 then the primal simplex algorithm is solving from an advanced basis. */
  MSK_IINF_SIM_PRIMAL_HOTSTART                   = 125,
  /** If 1 then a valid basis factorization of full rank was located and used by the primal simplex algorithm. */
  MSK_IINF_SIM_PRIMAL_HOTSTART_LU                = 126,
  /** The number of iterations taken with primal infeasibility. */
  MSK_IINF_SIM_PRIMAL_INF_ITER                   = 127,
  /** Number of primal simplex iterations during the last optimization. */
  MSK_IINF_SIM_PRIMAL_ITER                       = 128,
  /** Is non-zero if dual problem is solved. */
  MSK_IINF_SIM_SOLVE_DUAL                        = 129,
  /** Problem status of the basic solution. Updated after each optimization. */
  MSK_IINF_SOL_BAS_PROSTA                        = 130,
  /** Solution status of the basic solution. Updated after each optimization. */
  MSK_IINF_SOL_BAS_SOLSTA                        = 131,
  /** Problem status of the integer solution. Updated after each optimization. */
  MSK_IINF_SOL_ITG_PROSTA                        = 132,
  /** Solution status of the integer solution. Updated after each optimization. */
  MSK_IINF_SOL_ITG_SOLSTA                        = 133,
  /** Problem status of the interior-point solution. Updated after each optimization. */
  MSK_IINF_SOL_ITR_PROSTA                        = 134,
  /** Solution status of the interior-point solution. Updated after each optimization. */
  MSK_IINF_SOL_ITR_SOLSTA                        = 135,
  /** Number of times the storage for storing the linear coefficient matrix has been changed. */
  MSK_IINF_STO_NUM_A_REALLOC                     = 136
}; /* MSKiinfitem_enum */
#define MSK_IINF_BEGIN MSK_IINF_ANA_PRO_NUM_CON
#define MSK_IINF_END   (1+MSK_IINF_STO_NUM_A_REALLOC)
#ifdef MSK_NO_ENUMS
typedef int MSKiinfiteme;
#else
typedef enum MSKiinfitem_enum MSKiinfiteme;
#endif

enum MSKinftype_enum {
  /** Is a double information type. */
  MSK_INF_DOU_TYPE  = 0,
  /** Is an integer. */
  MSK_INF_INT_TYPE  = 1,
  /** Is a long integer. */
  MSK_INF_LINT_TYPE = 2
}; /* MSKinftype_enum */
#define MSK_INF_BEGIN MSK_INF_DOU_TYPE
#define MSK_INF_END   (1+MSK_INF_LINT_TYPE)
#ifdef MSK_NO_ENUMS
typedef int MSKinftypee;
#else
typedef enum MSKinftype_enum MSKinftypee;
#endif

enum MSKiomode_enum {
  /** The file is read-only. */
  MSK_IOMODE_READ      = 0,
  /** The file is write-only. If the file exists then it is truncated when it is opened. Otherwise it is created when it is opened. */
  MSK_IOMODE_WRITE     = 1,
  /** The file is to read and write. */
  MSK_IOMODE_READWRITE = 2
}; /* MSKiomode_enum */
#define MSK_IOMODE_BEGIN MSK_IOMODE_READ
#define MSK_IOMODE_END   (1+MSK_IOMODE_READWRITE)
#ifdef MSK_NO_ENUMS
typedef int MSKiomodee;
#else
typedef int MSKiomodee;
#endif

enum MSKiparam_enum {
  /** Controls whether the basis matrix is analyzed in solution analyzer. */
  MSK_IPAR_ANA_SOL_BASIS                      = 0,
  /** Controls whether a list of violated constraints is printed. */
  MSK_IPAR_ANA_SOL_PRINT_VIOLATED             = 1,
  /** Controls whether the elements in each column of A are sorted before an optimization is performed. */
  MSK_IPAR_AUTO_SORT_A_BEFORE_OPT             = 2,
  /** Controls whether the solution information items are automatically updated after an optimization is performed. */
  MSK_IPAR_AUTO_UPDATE_SOL_INFO               = 3,
  /** Controls the sign of the columns in the basis matrix corresponding to slack variables. */
  MSK_IPAR_BASIS_SOLVE_USE_PLUS_ONE           = 4,
  /** Controls which simplex optimizer is used in the clean-up phase. */
  MSK_IPAR_BI_CLEAN_OPTIMIZER                 = 5,
  /** Basis identification is performed even if the interior-point optimizer is terminated due to maximum number of iterations. */
  MSK_IPAR_BI_IGNORE_MAX_ITER                 = 6,
  /** Turns on basis identification in case the interior-point optimizer is terminated due to a numerical problem. */
  MSK_IPAR_BI_IGNORE_NUM_ERROR                = 7,
  /** Maximum number of iterations after basis identification. */
  MSK_IPAR_BI_MAX_ITERATIONS                  = 8,
  /** Control license caching. */
  MSK_IPAR_CACHE_LICENSE                      = 9,
  /** Control compression of stat files. */
  MSK_IPAR_COMPRESS_STATFILE                  = 10,
  /** Controls whether the concurrent optimizer is deterministic. */
  MSK_IPAR_CONCURRENT_DETERMINISTIC           = 11,
  /** Controls how to construct the fixed model. */
  MSK_IPAR_FIXING_METHOD                      = 12,
  /** Whether integer variable fixing requires feasible status. */
  MSK_IPAR_FIXING_REQUIRE_FEAS                = 13,
  /** Controls how to use folding. */
  MSK_IPAR_FOLDING_USE                        = 14,
  /** Detect LMIs and optimize their dualization. */
  MSK_IPAR_GETDUAL_CONVERT_LMIS               = 15,
  /** Controls the contents of the infeasibility report. */
  MSK_IPAR_INFEAS_GENERIC_NAMES               = 16,
  /** Turns the feasibility report on or off. */
  MSK_IPAR_INFEAS_REPORT_AUTO                 = 17,
  /** Controls the contents of the infeasibility report. */
  MSK_IPAR_INFEAS_REPORT_LEVEL                = 18,
  /** Controls whether basis identification is performed. */
  MSK_IPAR_INTPNT_BASIS                       = 19,
  /** Currently not in use. */
  MSK_IPAR_INTPNT_HOTSTART                    = 20,
  /** Controls the maximum number of iterations allowed in the interior-point optimizer. */
  MSK_IPAR_INTPNT_MAX_ITERATIONS              = 21,
  /** Maximum number of correction steps. */
  MSK_IPAR_INTPNT_MAX_NUM_COR                 = 22,
  /** Currently not in use. */
  MSK_IPAR_INTPNT_NOT_IN_USE                  = 23,
  /** Controls the aggressiveness of the offending column detection. */
  MSK_IPAR_INTPNT_OFF_COL_TRH                 = 24,
  /** This parameter controls the number of random seeds tried. */
  MSK_IPAR_INTPNT_ORDER_GP_NUM_SEEDS          = 25,
  /** Controls the ordering strategy. */
  MSK_IPAR_INTPNT_ORDER_METHOD                = 26,
  /** Controls whether regularization is allowed. */
  MSK_IPAR_INTPNT_REGULARIZATION_USE          = 27,
  /** Controls how the problem is scaled before the interior-point optimizer is used. */
  MSK_IPAR_INTPNT_SCALING                     = 28,
  /** Starting point used by the interior-point optimizer. */
  MSK_IPAR_INTPNT_STARTING_POINT              = 29,
  /** Controls the license manager client debugging behavior. */
  MSK_IPAR_LICENSE_DEBUG                      = 30,
  /** Controls license manager client behavior. */
  MSK_IPAR_LICENSE_PAUSE_TIME                 = 31,
  /** Controls license manager client behavior. */
  MSK_IPAR_LICENSE_SUPPRESS_EXPIRE_WRNS       = 32,
  /** Controls when expiry warnings are issued. */
  MSK_IPAR_LICENSE_TRH_EXPIRY_WRN             = 33,
  /** Controls if MOSEK should queue for a license if none is available. */
  MSK_IPAR_LICENSE_WAIT                       = 34,
  /** Controls the amount of log information. */
  MSK_IPAR_LOG                                = 35,
  /** Controls amount of output from the problem analyzer. */
  MSK_IPAR_LOG_ANA_PRO                        = 36,
  /** Controls the amount of output printed by the basis identification procedure. A higher level implies that more information is logged. */
  MSK_IPAR_LOG_BI                             = 37,
  /** Controls the logging frequency. */
  MSK_IPAR_LOG_BI_FREQ                        = 38,
  /** Controls the amount of log information from the concurrent optimizer. */
  MSK_IPAR_LOG_CONCURRENT                     = 39,
  /** Controls the reduction in the log levels for the second and any subsequent optimizations. */
  MSK_IPAR_LOG_CUT_SECOND_OPT                 = 40,
  /** Controls the amount of logging when a data item such as the maximum number constrains is expanded. */
  MSK_IPAR_LOG_EXPAND                         = 41,
  /** Controls the amount of output printed when performing feasibility repair. A value higher than one means extensive logging. */
  MSK_IPAR_LOG_FEAS_REPAIR                    = 42,
  /** If turned on, then some log info is printed when a file is written or read. */
  MSK_IPAR_LOG_FILE                           = 43,
  /** Controls whether solution summary should be printed by the optimizer. */
  MSK_IPAR_LOG_INCLUDE_SUMMARY                = 44,
  /** Controls log level for the infeasibility analyzer. */
  MSK_IPAR_LOG_INFEAS_ANA                     = 45,
  /** Controls the amount of log information from the interior-point optimizers. */
  MSK_IPAR_LOG_INTPNT                         = 46,
  /** Control whether local identifying information is printed to the log. */
  MSK_IPAR_LOG_LOCAL_INFO                     = 47,
  /** Controls the amount of log information from the mixed-integer optimizers. */
  MSK_IPAR_LOG_MIO                            = 48,
  /** The mixed-integer optimizer logging frequency. */
  MSK_IPAR_LOG_MIO_FREQ                       = 49,
  /** If turned on, then factor lines are added to the log. */
  MSK_IPAR_LOG_ORDER                          = 50,
  /** Controls amount of output printed by the presolve procedure. A higher level implies that more information is logged. */
  MSK_IPAR_LOG_PRESOLVE                       = 51,
  /** Control logging in sensitivity analyzer. */
  MSK_IPAR_LOG_SENSITIVITY                    = 52,
  /** Control logging in sensitivity analyzer. */
  MSK_IPAR_LOG_SENSITIVITY_OPT                = 53,
  /** Controls the amount of log information from the simplex optimizers. */
  MSK_IPAR_LOG_SIM                            = 54,
  /** Controls simplex logging frequency. */
  MSK_IPAR_LOG_SIM_FREQ                       = 55,
  /** Controls the memory related log information. */
  MSK_IPAR_LOG_STORAGE                        = 56,
  /** Each warning is shown a limited number of times controlled by this parameter. A negative value is identical to infinite number of times. */
  MSK_IPAR_MAX_NUM_WARNINGS                   = 57,
  /** Controls whether the mixed-integer optimizer is branching up or down by default. */
  MSK_IPAR_MIO_BRANCH_DIR                     = 58,
  /** Controls the amount of conflict analysis employed by the mixed-integer optimizer. */
  MSK_IPAR_MIO_CONFLICT_ANALYSIS_LEVEL        = 59,
  /** Toggles outer approximation for conic problems. */
  MSK_IPAR_MIO_CONIC_OUTER_APPROXIMATION      = 60,
  /** Controls if an initial mixed integer solution should be constructed from the values of the integer variables. */
  MSK_IPAR_MIO_CONSTRUCT_SOL                  = 61,
  /** Maximum number of nodes in each call to Crossover. */
  MSK_IPAR_MIO_CROSSOVER_MAX_NODES            = 62,
  /** Controls whether clique cuts should be generated. */
  MSK_IPAR_MIO_CUT_CLIQUE                     = 63,
  /** Controls whether mixed integer rounding cuts should be generated. */
  MSK_IPAR_MIO_CUT_CMIR                       = 64,
  /** Controls whether GMI cuts should be generated. */
  MSK_IPAR_MIO_CUT_GMI                        = 65,
  /** Controls whether implied bound cuts should be generated. */
  MSK_IPAR_MIO_CUT_IMPLIED_BOUND              = 66,
  /** Controls whether knapsack cover cuts should be generated. */
  MSK_IPAR_MIO_CUT_KNAPSACK_COVER             = 67,
  /** Controls whether lift-and-project cuts should be generated. */
  MSK_IPAR_MIO_CUT_LIPRO                      = 68,
  /** Controls how aggressively generated cuts are selected to be included in the relaxation. */
  MSK_IPAR_MIO_CUT_SELECTION_LEVEL            = 69,
  /** Controls what problem data permutation method is appplied to mixed-integer problems. */
  MSK_IPAR_MIO_DATA_PERMUTATION_METHOD        = 70,
  /** Controls the amount of dual ray analysis employed by the mixed-integer optimizer. */
  MSK_IPAR_MIO_DUAL_RAY_ANALYSIS_LEVEL        = 71,
  /** Controls the way the Feasibility Pump heuristic is employed by the mixed-integer optimizer. */
  MSK_IPAR_MIO_FEASPUMP_LEVEL                 = 72,
  /** Controls the heuristic employed by the mixed-integer optimizer to locate an initial integer feasible solution. */
  MSK_IPAR_MIO_HEURISTIC_LEVEL                = 73,
  /** Controls the way the mixed-integer optimizer exploits independent-block structure in the problem. */
  MSK_IPAR_MIO_INDEPENDENT_BLOCK_LEVEL        = 74,
  /** Maximum number of branches allowed during the branch and bound search. */
  MSK_IPAR_MIO_MAX_NUM_BRANCHES               = 75,
  /** Maximum number of relaxations in branch and bound search. */
  MSK_IPAR_MIO_MAX_NUM_RELAXS                 = 76,
  /** Maximum number of restarts allowed during the branch and bound search. */
  MSK_IPAR_MIO_MAX_NUM_RESTARTS               = 77,
  /** Maximum number of cut separation rounds at the root node. */
  MSK_IPAR_MIO_MAX_NUM_ROOT_CUT_ROUNDS        = 78,
  /** Controls how many feasible solutions the mixed-integer optimizer investigates. */
  MSK_IPAR_MIO_MAX_NUM_SOLUTIONS              = 79,
  /** Controls how much emphasis is put on reducing memory usage. */
  MSK_IPAR_MIO_MEMORY_EMPHASIS_LEVEL          = 80,
  /** Number of times a variable must have been branched on for its pseudocost to be considered reliable. */
  MSK_IPAR_MIO_MIN_REL                        = 81,
  /** Turns on/off the mixed-integer mode. */
  MSK_IPAR_MIO_MODE                           = 82,
  /** Controls which optimizer is employed at the non-root nodes in the mixed-integer optimizer. */
  MSK_IPAR_MIO_NODE_OPTIMIZER                 = 83,
  /** Controls the node selection strategy employed by the mixed-integer optimizer. */
  MSK_IPAR_MIO_NODE_SELECTION                 = 84,
  /** Controls how much emphasis is put on reducing numerical problems */
  MSK_IPAR_MIO_NUMERICAL_EMPHASIS_LEVEL       = 85,
  /** Maximum number of nodes in each call to RINS. */
  MSK_IPAR_MIO_OPT_FACE_MAX_NODES             = 86,
  /** Enables or disables perspective reformulation in presolve. */
  MSK_IPAR_MIO_PERSPECTIVE_REFORMULATE        = 87,
  /** Controls if the aggregator should be used. */
  MSK_IPAR_MIO_PRESOLVE_AGGREGATOR_USE        = 88,
  /** Controls the amount of probing employed by the mixed-integer optimizer in presolve. */
  MSK_IPAR_MIO_PROBING_LEVEL                  = 89,
  /** Use objective domain propagation. */
  MSK_IPAR_MIO_PROPAGATE_OBJECTIVE_CONSTRAINT = 90,
  /** Controls what reformulation method is applied to mixed-integer quadratic problems. */
  MSK_IPAR_MIO_QCQO_REFORMULATION_METHOD      = 91,
  /** Maximum number of nodes in each call to RENS. */
  MSK_IPAR_MIO_RENS_MAX_NODES                 = 92,
  /** Maximum number of nodes in each call to RINS. */
  MSK_IPAR_MIO_RINS_MAX_NODES                 = 93,
  /** Controls which optimizer is employed at the root node in the mixed-integer optimizer. */
  MSK_IPAR_MIO_ROOT_OPTIMIZER                 = 94,
  /** Sets the random seed used for randomization in the mixed integer optimizer. */
  MSK_IPAR_MIO_SEED                           = 95,
  /** Controls the amount of symmetry detection and handling employed by the mixed-integer optimizer in presolve. */
  MSK_IPAR_MIO_SYMMETRY_LEVEL                 = 96,
  /** Controls the variable selection strategy employed by the mixed-integer optimizer. */
  MSK_IPAR_MIO_VAR_SELECTION                  = 97,
  /** Controls how much effort is put into detecting variable bounds. */
  MSK_IPAR_MIO_VB_DETECTION_LEVEL             = 98,
  /** Set the number of iterations to spin before sleeping. */
  MSK_IPAR_MT_SPINCOUNT                       = 99,
  /** Not in use */
  MSK_IPAR_NG                                 = 100,
  /** The number of threads employed by the optimizer. */
  MSK_IPAR_NUM_THREADS                        = 101,
  /** Write a text header with date and MOSEK version in an OPF file. */
  MSK_IPAR_OPF_WRITE_HEADER                   = 102,
  /** Write a hint section with problem dimensions in the beginning of an OPF file. */
  MSK_IPAR_OPF_WRITE_HINTS                    = 103,
  /** Aim to keep lines in OPF files not much longer than this. */
  MSK_IPAR_OPF_WRITE_LINE_LENGTH              = 104,
  /** Write a parameter section in an OPF file. */
  MSK_IPAR_OPF_WRITE_PARAMETERS               = 105,
  /** Write objective, constraints, bounds etc. to an OPF file. */
  MSK_IPAR_OPF_WRITE_PROBLEM                  = 106,
  /** Controls what is written to the OPF files. */
  MSK_IPAR_OPF_WRITE_SOL_BAS                  = 107,
  /** Controls what is written to the OPF files. */
  MSK_IPAR_OPF_WRITE_SOL_ITG                  = 108,
  /** Controls what is written to the OPF files. */
  MSK_IPAR_OPF_WRITE_SOL_ITR                  = 109,
  /** Enable inclusion of solutions in the OPF files. */
  MSK_IPAR_OPF_WRITE_SOLUTIONS                = 110,
  /** Controls which optimizer is used to optimize the task. */
  MSK_IPAR_OPTIMIZER                          = 111,
  /** If turned on, then names in the parameter file are case sensitive. */
  MSK_IPAR_PARAM_READ_CASE_NAME               = 112,
  /** If turned on, then errors in parameter settings is ignored. */
  MSK_IPAR_PARAM_READ_IGN_ERROR               = 113,
  /** Maximum amount of fill-in created in one pivot during the elimination phase. */
  MSK_IPAR_PRESOLVE_ELIMINATOR_MAX_FILL       = 114,
  /** Control the maximum number of times the eliminator is tried. */
  MSK_IPAR_PRESOLVE_ELIMINATOR_MAX_NUM_TRIES  = 115,
  /** Controls linear dependency check in presolve. */
  MSK_IPAR_PRESOLVE_LINDEP_ABS_WORK_TRH       = 116,
  /** Controls whether a new experimental linear dependency checker is employed. */
  MSK_IPAR_PRESOLVE_LINDEP_NEW                = 117,
  /** Controls linear dependency check in presolve. */
  MSK_IPAR_PRESOLVE_LINDEP_REL_WORK_TRH       = 118,
  /** Controls whether the linear constraints are checked for linear dependencies. */
  MSK_IPAR_PRESOLVE_LINDEP_USE                = 119,
  /** Control the maximum number of times presolve passes over the problem. */
  MSK_IPAR_PRESOLVE_MAX_NUM_PASS              = 120,
  /** Controls the maximum number of reductions performed by the presolve. */
  MSK_IPAR_PRESOLVE_MAX_NUM_REDUCTIONS        = 121,
  /** Controls whether the presolve is applied to a problem before it is optimized. */
  MSK_IPAR_PRESOLVE_USE                       = 122,
  /** Controls which optimizer that is used to find the optimal repair. */
  MSK_IPAR_PRIMAL_REPAIR_OPTIMIZER            = 123,
  /** Controls whether parameters section is written in PTF files. */
  MSK_IPAR_PTF_WRITE_PARAMETERS               = 124,
  /** Controls whether PSD terms with a coefficient matrix of just one non-zero are written as a single term instead of as a matrix term. */
  MSK_IPAR_PTF_WRITE_SINGLE_PSD_TERMS         = 125,
  /** Controls whether solution section is written in PTF files. */
  MSK_IPAR_PTF_WRITE_SOLUTIONS                = 126,
  /** Controls whether files are read using synchronous or asynchronous reader. */
  MSK_IPAR_READ_ASYNC                         = 127,
  /** Turns on additional debugging information when reading files. */
  MSK_IPAR_READ_DEBUG                         = 128,
  /** Controls whether the free constraints are included in the problem. Applies to MPS files. */
  MSK_IPAR_READ_KEEP_FREE_CON                 = 129,
  /** Controls how strictly the MPS file reader interprets the MPS format. */
  MSK_IPAR_READ_MPS_FORMAT                    = 130,
  /** Controls the maximal number of characters allowed in one line of the MPS file. */
  MSK_IPAR_READ_MPS_WIDTH                     = 131,
  /** Controls what information is used from the task files. */
  MSK_IPAR_READ_TASK_IGNORE_PARAM             = 132,
  /** Use compression when sending data to an optimization server */
  MSK_IPAR_REMOTE_USE_COMPRESSION             = 133,
  /** Removes unused solutions before the optimization is performed. */
  MSK_IPAR_REMOVE_UNUSED_SOLUTIONS            = 134,
  /** If turned on, then an interior-point solution is available even if a basic solution is available. */
  MSK_IPAR_REQUEST_INTPNT                     = 135,
  /** Controls sensitivity report behavior. */
  MSK_IPAR_SENSITIVITY_ALL                    = 136,
  /** Controls which type of sensitivity analysis is to be performed. */
  MSK_IPAR_SENSITIVITY_TYPE                   = 137,
  /** Controls whether an LU factorization of the basis is used in a hot-start. */
  MSK_IPAR_SIM_BASIS_FACTOR_USE               = 138,
  /** TBD */
  MSK_IPAR_SIM_CACHE                          = 139,
  /** Controls how aggressively degeneration is handled. */
  MSK_IPAR_SIM_DEGEN                          = 140,
  /** Not in use. */
  MSK_IPAR_SIM_DETECT_PWL                     = 141,
  /** Controls whether crashing is performed in the dual simplex optimizer. */
  MSK_IPAR_SIM_DUAL_CRASH                     = 142,
  /** An experimental feature. */
  MSK_IPAR_SIM_DUAL_PHASEONE_METHOD           = 143,
  /** Controls how aggressively restricted selection is used. */
  MSK_IPAR_SIM_DUAL_RESTRICT_SELECTION        = 144,
  /** Controls the dual simplex strategy. */
  MSK_IPAR_SIM_DUAL_SELECTION                 = 145,
  /** Controls the type of hot-start that the simplex optimizer perform. */
  MSK_IPAR_SIM_HOTSTART                       = 146,
  /** Determines if the simplex optimizer should exploit the initial factorization. */
  MSK_IPAR_SIM_HOTSTART_LU                    = 147,
  /** Maximum number of iterations that can be used by a simplex optimizer. */
  MSK_IPAR_SIM_MAX_ITERATIONS                 = 148,
  /** Controls how many set-backs that are allowed within a simplex optimizer. */
  MSK_IPAR_SIM_MAX_NUM_SETBACKS               = 149,
  /** Controls if the simplex optimizer ensures a non-singular basis, if possible. */
  MSK_IPAR_SIM_NON_SINGULAR                   = 150,
  /** Experimental. Usage not recommended. */
  MSK_IPAR_SIM_PRECISION                      = 151,
  /** Controls whether the simplex optimizer is allowed to boost the precision. */
  MSK_IPAR_SIM_PRECISION_BOOST                = 152,
  /** Controls the simplex crash. */
  MSK_IPAR_SIM_PRIMAL_CRASH                   = 153,
  /** An experimental feature. */
  MSK_IPAR_SIM_PRIMAL_PHASEONE_METHOD         = 154,
  /** Controls how aggressively restricted selection is used. */
  MSK_IPAR_SIM_PRIMAL_RESTRICT_SELECTION      = 155,
  /** Controls the primal simplex strategy. */
  MSK_IPAR_SIM_PRIMAL_SELECTION               = 156,
  /** Controls the basis refactoring frequency. */
  MSK_IPAR_SIM_REFACTOR_FREQ                  = 157,
  /** Controls if the simplex optimizers are allowed to reformulate the problem. */
  MSK_IPAR_SIM_REFORMULATION                  = 158,
  /** Controls if the LU factorization stored should be replaced with the LU factorization corresponding to the initial basis. */
  MSK_IPAR_SIM_SAVE_LU                        = 159,
  /** Controls how much effort is used in scaling the problem before a simplex optimizer is used. */
  MSK_IPAR_SIM_SCALING                        = 160,
  /** Controls how the problem is scaled before a simplex optimizer is used. */
  MSK_IPAR_SIM_SCALING_METHOD                 = 161,
  /** Sets the random seed used for randomization in the simplex optimizers. */
  MSK_IPAR_SIM_SEED                           = 162,
  /** Experimental. Usage not recommended. */
  MSK_IPAR_SIM_SOLUTION_REFINEMENT            = 163,
  /** Controls the simplex behavior. */
  MSK_IPAR_SIM_SWITCH_OPTIMIZER               = 164,
  /** Control the contents of the solution files. */
  MSK_IPAR_SOL_FILTER_KEEP_BASIC              = 165,
  /** Controls the input solution file format. */
  MSK_IPAR_SOL_READ_NAME_WIDTH                = 166,
  /** Controls the input solution file format. */
  MSK_IPAR_SOL_READ_WIDTH                     = 167,
  /** Controls whether the primal or the dual problem is solved by the continous optimizers. */
  MSK_IPAR_SOLVE_FORM                         = 168,
  /** Controls the amount of timing performed inside MOSEK. */
  MSK_IPAR_TIMING_LEVEL                       = 169,
  /** Controls whether files are read using synchronous or asynchronous writer. */
  MSK_IPAR_WRITE_ASYNC                        = 170,
  /** Controls the basic solution file format. */
  MSK_IPAR_WRITE_BAS_CONSTRAINTS              = 171,
  /** Controls the basic solution file format. */
  MSK_IPAR_WRITE_BAS_HEAD                     = 172,
  /** Controls the basic solution file format. */
  MSK_IPAR_WRITE_BAS_VARIABLES                = 173,
  /** Controls output file compression. */
  MSK_IPAR_WRITE_COMPRESSION                  = 174,
  /** Controls the output file data. */
  MSK_IPAR_WRITE_FREE_CON                     = 175,
  /** Controls the output file data. */
  MSK_IPAR_WRITE_GENERIC_NAMES                = 176,
  /** Controls if the writer ignores incompatible problem items when writing files. */
  MSK_IPAR_WRITE_IGNORE_INCOMPATIBLE_ITEMS    = 177,
  /** Controls the integer solution file format. */
  MSK_IPAR_WRITE_INT_CONSTRAINTS              = 178,
  /** Controls the integer solution file format. */
  MSK_IPAR_WRITE_INT_HEAD                     = 179,
  /** Controls the integer solution file format. */
  MSK_IPAR_WRITE_INT_VARIABLES                = 180,
  /** When set, the JSON task and solution files are written with indentation for better readability. */
  MSK_IPAR_WRITE_JSON_INDENTATION             = 181,
  /** Write full linear objective. */
  MSK_IPAR_WRITE_LP_FULL_OBJ                  = 182,
  /** Ignore free constraints while writing a LP formatted file. */
  MSK_IPAR_WRITE_LP_IGNORE_FREE_CONSTRAINTS   = 183,
  /** Controls the LP output file format. */
  MSK_IPAR_WRITE_LP_LINE_WIDTH                = 184,
  /** Controls in which format the MPS file is written. */
  MSK_IPAR_WRITE_MPS_FORMAT                   = 185,
  /** Controls the output file data. */
  MSK_IPAR_WRITE_MPS_INT                      = 186,
  /** Controls the solution file format. */
  MSK_IPAR_WRITE_SOL_BARVARIABLES             = 187,
  /** Controls the solution file format. */
  MSK_IPAR_WRITE_SOL_CONSTRAINTS              = 188,
  /** Controls solution file format. */
  MSK_IPAR_WRITE_SOL_HEAD                     = 189,
  /** Controls whether the user specified names are employed even if they are invalid names. */
  MSK_IPAR_WRITE_SOL_IGNORE_INVALID_NAMES     = 190,
  /** Controls the solution file format. */
  MSK_IPAR_WRITE_SOL_VARIABLES                = 191
}; /* MSKiparam_enum */
#define MSK_IPAR_BEGIN MSK_IPAR_ANA_SOL_BASIS
#define MSK_IPAR_END   (1+MSK_IPAR_WRITE_SOL_VARIABLES)
#ifdef MSK_NO_ENUMS
typedef int MSKiparame;
#else
typedef enum MSKiparam_enum MSKiparame;
#endif

#define MSK_IPAR_ANA_SOL_BASIS_                 "MSK_IPAR_ANA_SOL_BASIS"
#define MSK_IPAR_ANA_SOL_PRINT_VIOLATED_        "MSK_IPAR_ANA_SOL_PRINT_VIOLATED"
#define MSK_IPAR_AUTO_SORT_A_BEFORE_OPT_        "MSK_IPAR_AUTO_SORT_A_BEFORE_OPT"
#define MSK_IPAR_AUTO_UPDATE_SOL_INFO_          "MSK_IPAR_AUTO_UPDATE_SOL_INFO"
#define MSK_IPAR_BASIS_SOLVE_USE_PLUS_ONE_      "MSK_IPAR_BASIS_SOLVE_USE_PLUS_ONE"
#define MSK_IPAR_BI_CLEAN_OPTIMIZER_            "MSK_IPAR_BI_CLEAN_OPTIMIZER"
#define MSK_IPAR_BI_IGNORE_MAX_ITER_            "MSK_IPAR_BI_IGNORE_MAX_ITER"
#define MSK_IPAR_BI_IGNORE_NUM_ERROR_           "MSK_IPAR_BI_IGNORE_NUM_ERROR"
#define MSK_IPAR_BI_MAX_ITERATIONS_             "MSK_IPAR_BI_MAX_ITERATIONS"
#define MSK_IPAR_CACHE_LICENSE_                 "MSK_IPAR_CACHE_LICENSE"
#define MSK_IPAR_COMPRESS_STATFILE_             "MSK_IPAR_COMPRESS_STATFILE"
#define MSK_IPAR_CONCURRENT_DETERMINISTIC_      "MSK_IPAR_CONCURRENT_DETERMINISTIC"
#define MSK_IPAR_FIXING_METHOD_                 "MSK_IPAR_FIXING_METHOD"
#define MSK_IPAR_FIXING_REQUIRE_FEAS_           "MSK_IPAR_FIXING_REQUIRE_FEAS"
#define MSK_IPAR_FOLDING_USE_                   "MSK_IPAR_FOLDING_USE"
#define MSK_IPAR_GETDUAL_CONVERT_LMIS_          "MSK_IPAR_GETDUAL_CONVERT_LMIS"
#define MSK_IPAR_INFEAS_GENERIC_NAMES_          "MSK_IPAR_INFEAS_GENERIC_NAMES"
#define MSK_IPAR_INFEAS_REPORT_AUTO_            "MSK_IPAR_INFEAS_REPORT_AUTO"
#define MSK_IPAR_INFEAS_REPORT_LEVEL_           "MSK_IPAR_INFEAS_REPORT_LEVEL"
#define MSK_IPAR_INTPNT_BASIS_                  "MSK_IPAR_INTPNT_BASIS"
#define MSK_IPAR_INTPNT_HOTSTART_               "MSK_IPAR_INTPNT_HOTSTART"
#define MSK_IPAR_INTPNT_MAX_ITERATIONS_         "MSK_IPAR_INTPNT_MAX_ITERATIONS"
#define MSK_IPAR_INTPNT_MAX_NUM_COR_            "MSK_IPAR_INTPNT_MAX_NUM_COR"
#define MSK_IPAR_INTPNT_NOT_IN_USE_             "MSK_IPAR_INTPNT_NOT_IN_USE"
#define MSK_IPAR_INTPNT_OFF_COL_TRH_            "MSK_IPAR_INTPNT_OFF_COL_TRH"
#define MSK_IPAR_INTPNT_ORDER_GP_NUM_SEEDS_     "MSK_IPAR_INTPNT_ORDER_GP_NUM_SEEDS"
#define MSK_IPAR_INTPNT_ORDER_METHOD_           "MSK_IPAR_INTPNT_ORDER_METHOD"
#define MSK_IPAR_INTPNT_REGULARIZATION_USE_     "MSK_IPAR_INTPNT_REGULARIZATION_USE"
#define MSK_IPAR_INTPNT_SCALING_                "MSK_IPAR_INTPNT_SCALING"
#define MSK_IPAR_INTPNT_STARTING_POINT_         "MSK_IPAR_INTPNT_STARTING_POINT"
#define MSK_IPAR_LICENSE_DEBUG_                 "MSK_IPAR_LICENSE_DEBUG"
#define MSK_IPAR_LICENSE_PAUSE_TIME_            "MSK_IPAR_LICENSE_PAUSE_TIME"
#define MSK_IPAR_LICENSE_SUPPRESS_EXPIRE_WRNS_  "MSK_IPAR_LICENSE_SUPPRESS_EXPIRE_WRNS"
#define MSK_IPAR_LICENSE_TRH_EXPIRY_WRN_        "MSK_IPAR_LICENSE_TRH_EXPIRY_WRN"
#define MSK_IPAR_LICENSE_WAIT_                  "MSK_IPAR_LICENSE_WAIT"
#define MSK_IPAR_LOG_                           "MSK_IPAR_LOG"
#define MSK_IPAR_LOG_ANA_PRO_                   "MSK_IPAR_LOG_ANA_PRO"
#define MSK_IPAR_LOG_BI_                        "MSK_IPAR_LOG_BI"
#define MSK_IPAR_LOG_BI_FREQ_                   "MSK_IPAR_LOG_BI_FREQ"
#define MSK_IPAR_LOG_CONCURRENT_                "MSK_IPAR_LOG_CONCURRENT"
#define MSK_IPAR_LOG_CUT_SECOND_OPT_            "MSK_IPAR_LOG_CUT_SECOND_OPT"
#define MSK_IPAR_LOG_EXPAND_                    "MSK_IPAR_LOG_EXPAND"
#define MSK_IPAR_LOG_FEAS_REPAIR_               "MSK_IPAR_LOG_FEAS_REPAIR"
#define MSK_IPAR_LOG_FILE_                      "MSK_IPAR_LOG_FILE"
#define MSK_IPAR_LOG_INCLUDE_SUMMARY_           "MSK_IPAR_LOG_INCLUDE_SUMMARY"
#define MSK_IPAR_LOG_INFEAS_ANA_                "MSK_IPAR_LOG_INFEAS_ANA"
#define MSK_IPAR_LOG_INTPNT_                    "MSK_IPAR_LOG_INTPNT"
#define MSK_IPAR_LOG_LOCAL_INFO_                "MSK_IPAR_LOG_LOCAL_INFO"
#define MSK_IPAR_LOG_MIO_                       "MSK_IPAR_LOG_MIO"
#define MSK_IPAR_LOG_MIO_FREQ_                  "MSK_IPAR_LOG_MIO_FREQ"
#define MSK_IPAR_LOG_ORDER_                     "MSK_IPAR_LOG_ORDER"
#define MSK_IPAR_LOG_PRESOLVE_                  "MSK_IPAR_LOG_PRESOLVE"
#define MSK_IPAR_LOG_SENSITIVITY_               "MSK_IPAR_LOG_SENSITIVITY"
#define MSK_IPAR_LOG_SENSITIVITY_OPT_           "MSK_IPAR_LOG_SENSITIVITY_OPT"
#define MSK_IPAR_LOG_SIM_                       "MSK_IPAR_LOG_SIM"
#define MSK_IPAR_LOG_SIM_FREQ_                  "MSK_IPAR_LOG_SIM_FREQ"
#define MSK_IPAR_LOG_STORAGE_                   "MSK_IPAR_LOG_STORAGE"
#define MSK_IPAR_MAX_NUM_WARNINGS_              "MSK_IPAR_MAX_NUM_WARNINGS"
#define MSK_IPAR_MIO_BRANCH_DIR_                "MSK_IPAR_MIO_BRANCH_DIR"
#define MSK_IPAR_MIO_CONFLICT_ANALYSIS_LEVEL_   "MSK_IPAR_MIO_CONFLICT_ANALYSIS_LEVEL"
#define MSK_IPAR_MIO_CONIC_OUTER_APPROXIMATION_ "MSK_IPAR_MIO_CONIC_OUTER_APPROXIMATION"
#define MSK_IPAR_MIO_CONSTRUCT_SOL_             "MSK_IPAR_MIO_CONSTRUCT_SOL"
#define MSK_IPAR_MIO_CROSSOVER_MAX_NODES_       "MSK_IPAR_MIO_CROSSOVER_MAX_NODES"
#define MSK_IPAR_MIO_CUT_CLIQUE_                "MSK_IPAR_MIO_CUT_CLIQUE"
#define MSK_IPAR_MIO_CUT_CMIR_                  "MSK_IPAR_MIO_CUT_CMIR"
#define MSK_IPAR_MIO_CUT_GMI_                   "MSK_IPAR_MIO_CUT_GMI"
#define MSK_IPAR_MIO_CUT_IMPLIED_BOUND_         "MSK_IPAR_MIO_CUT_IMPLIED_BOUND"
#define MSK_IPAR_MIO_CUT_KNAPSACK_COVER_        "MSK_IPAR_MIO_CUT_KNAPSACK_COVER"
#define MSK_IPAR_MIO_CUT_LIPRO_                 "MSK_IPAR_MIO_CUT_LIPRO"
#define MSK_IPAR_MIO_CUT_SELECTION_LEVEL_       "MSK_IPAR_MIO_CUT_SELECTION_LEVEL"
#define MSK_IPAR_MIO_DATA_PERMUTATION_METHOD_   "MSK_IPAR_MIO_DATA_PERMUTATION_METHOD"
#define MSK_IPAR_MIO_DUAL_RAY_ANALYSIS_LEVEL_   "MSK_IPAR_MIO_DUAL_RAY_ANALYSIS_LEVEL"
#define MSK_IPAR_MIO_FEASPUMP_LEVEL_            "MSK_IPAR_MIO_FEASPUMP_LEVEL"
#define MSK_IPAR_MIO_HEURISTIC_LEVEL_           "MSK_IPAR_MIO_HEURISTIC_LEVEL"
#define MSK_IPAR_MIO_INDEPENDENT_BLOCK_LEVEL_   "MSK_IPAR_MIO_INDEPENDENT_BLOCK_LEVEL"
#define MSK_IPAR_MIO_MAX_NUM_BRANCHES_          "MSK_IPAR_MIO_MAX_NUM_BRANCHES"
#define MSK_IPAR_MIO_MAX_NUM_RELAXS_            "MSK_IPAR_MIO_MAX_NUM_RELAXS"
#define MSK_IPAR_MIO_MAX_NUM_RESTARTS_          "MSK_IPAR_MIO_MAX_NUM_RESTARTS"
#define MSK_IPAR_MIO_MAX_NUM_ROOT_CUT_ROUNDS_   "MSK_IPAR_MIO_MAX_NUM_ROOT_CUT_ROUNDS"
#define MSK_IPAR_MIO_MAX_NUM_SOLUTIONS_         "MSK_IPAR_MIO_MAX_NUM_SOLUTIONS"
#define MSK_IPAR_MIO_MEMORY_EMPHASIS_LEVEL_     "MSK_IPAR_MIO_MEMORY_EMPHASIS_LEVEL"
#define MSK_IPAR_MIO_MIN_REL_                   "MSK_IPAR_MIO_MIN_REL"
#define MSK_IPAR_MIO_MODE_                      "MSK_IPAR_MIO_MODE"
#define MSK_IPAR_MIO_NODE_OPTIMIZER_            "MSK_IPAR_MIO_NODE_OPTIMIZER"
#define MSK_IPAR_MIO_NODE_SELECTION_            "MSK_IPAR_MIO_NODE_SELECTION"
#define MSK_IPAR_MIO_NUMERICAL_EMPHASIS_LEVEL_  "MSK_IPAR_MIO_NUMERICAL_EMPHASIS_LEVEL"
#define MSK_IPAR_MIO_OPT_FACE_MAX_NODES_        "MSK_IPAR_MIO_OPT_FACE_MAX_NODES"
#define MSK_IPAR_MIO_PERSPECTIVE_REFORMULATE_   "MSK_IPAR_MIO_PERSPECTIVE_REFORMULATE"
#define MSK_IPAR_MIO_PRESOLVE_AGGREGATOR_USE_   "MSK_IPAR_MIO_PRESOLVE_AGGREGATOR_USE"
#define MSK_IPAR_MIO_PROBING_LEVEL_             "MSK_IPAR_MIO_PROBING_LEVEL"
#define MSK_IPAR_MIO_PROPAGATE_OBJECTIVE_CONSTRAINT_ "MSK_IPAR_MIO_PROPAGATE_OBJECTIVE_CONSTRAINT"
#define MSK_IPAR_MIO_QCQO_REFORMULATION_METHOD_ "MSK_IPAR_MIO_QCQO_REFORMULATION_METHOD"
#define MSK_IPAR_MIO_RENS_MAX_NODES_            "MSK_IPAR_MIO_RENS_MAX_NODES"
#define MSK_IPAR_MIO_RINS_MAX_NODES_            "MSK_IPAR_MIO_RINS_MAX_NODES"
#define MSK_IPAR_MIO_ROOT_OPTIMIZER_            "MSK_IPAR_MIO_ROOT_OPTIMIZER"
#define MSK_IPAR_MIO_SEED_                      "MSK_IPAR_MIO_SEED"
#define MSK_IPAR_MIO_SYMMETRY_LEVEL_            "MSK_IPAR_MIO_SYMMETRY_LEVEL"
#define MSK_IPAR_MIO_VAR_SELECTION_             "MSK_IPAR_MIO_VAR_SELECTION"
#define MSK_IPAR_MIO_VB_DETECTION_LEVEL_        "MSK_IPAR_MIO_VB_DETECTION_LEVEL"
#define MSK_IPAR_MT_SPINCOUNT_                  "MSK_IPAR_MT_SPINCOUNT"
#define MSK_IPAR_NG_                            "MSK_IPAR_NG"
#define MSK_IPAR_NUM_THREADS_                   "MSK_IPAR_NUM_THREADS"
#define MSK_IPAR_OPF_WRITE_HEADER_              "MSK_IPAR_OPF_WRITE_HEADER"
#define MSK_IPAR_OPF_WRITE_HINTS_               "MSK_IPAR_OPF_WRITE_HINTS"
#define MSK_IPAR_OPF_WRITE_LINE_LENGTH_         "MSK_IPAR_OPF_WRITE_LINE_LENGTH"
#define MSK_IPAR_OPF_WRITE_PARAMETERS_          "MSK_IPAR_OPF_WRITE_PARAMETERS"
#define MSK_IPAR_OPF_WRITE_PROBLEM_             "MSK_IPAR_OPF_WRITE_PROBLEM"
#define MSK_IPAR_OPF_WRITE_SOL_BAS_             "MSK_IPAR_OPF_WRITE_SOL_BAS"
#define MSK_IPAR_OPF_WRITE_SOL_ITG_             "MSK_IPAR_OPF_WRITE_SOL_ITG"
#define MSK_IPAR_OPF_WRITE_SOL_ITR_             "MSK_IPAR_OPF_WRITE_SOL_ITR"
#define MSK_IPAR_OPF_WRITE_SOLUTIONS_           "MSK_IPAR_OPF_WRITE_SOLUTIONS"
#define MSK_IPAR_OPTIMIZER_                     "MSK_IPAR_OPTIMIZER"
#define MSK_IPAR_PARAM_READ_CASE_NAME_          "MSK_IPAR_PARAM_READ_CASE_NAME"
#define MSK_IPAR_PARAM_READ_IGN_ERROR_          "MSK_IPAR_PARAM_READ_IGN_ERROR"
#define MSK_IPAR_PRESOLVE_ELIMINATOR_MAX_FILL_  "MSK_IPAR_PRESOLVE_ELIMINATOR_MAX_FILL"
#define MSK_IPAR_PRESOLVE_ELIMINATOR_MAX_NUM_TRIES_ "MSK_IPAR_PRESOLVE_ELIMINATOR_MAX_NUM_TRIES"
#define MSK_IPAR_PRESOLVE_LINDEP_ABS_WORK_TRH_  "MSK_IPAR_PRESOLVE_LINDEP_ABS_WORK_TRH"
#define MSK_IPAR_PRESOLVE_LINDEP_NEW_           "MSK_IPAR_PRESOLVE_LINDEP_NEW"
#define MSK_IPAR_PRESOLVE_LINDEP_REL_WORK_TRH_  "MSK_IPAR_PRESOLVE_LINDEP_REL_WORK_TRH"
#define MSK_IPAR_PRESOLVE_LINDEP_USE_           "MSK_IPAR_PRESOLVE_LINDEP_USE"
#define MSK_IPAR_PRESOLVE_MAX_NUM_PASS_         "MSK_IPAR_PRESOLVE_MAX_NUM_PASS"
#define MSK_IPAR_PRESOLVE_MAX_NUM_REDUCTIONS_   "MSK_IPAR_PRESOLVE_MAX_NUM_REDUCTIONS"
#define MSK_IPAR_PRESOLVE_USE_                  "MSK_IPAR_PRESOLVE_USE"
#define MSK_IPAR_PRIMAL_REPAIR_OPTIMIZER_       "MSK_IPAR_PRIMAL_REPAIR_OPTIMIZER"
#define MSK_IPAR_PTF_WRITE_PARAMETERS_          "MSK_IPAR_PTF_WRITE_PARAMETERS"
#define MSK_IPAR_PTF_WRITE_SINGLE_PSD_TERMS_    "MSK_IPAR_PTF_WRITE_SINGLE_PSD_TERMS"
#define MSK_IPAR_PTF_WRITE_SOLUTIONS_           "MSK_IPAR_PTF_WRITE_SOLUTIONS"
#define MSK_IPAR_READ_ASYNC_                    "MSK_IPAR_READ_ASYNC"
#define MSK_IPAR_READ_DEBUG_                    "MSK_IPAR_READ_DEBUG"
#define MSK_IPAR_READ_KEEP_FREE_CON_            "MSK_IPAR_READ_KEEP_FREE_CON"
#define MSK_IPAR_READ_MPS_FORMAT_               "MSK_IPAR_READ_MPS_FORMAT"
#define MSK_IPAR_READ_MPS_WIDTH_                "MSK_IPAR_READ_MPS_WIDTH"
#define MSK_IPAR_READ_TASK_IGNORE_PARAM_        "MSK_IPAR_READ_TASK_IGNORE_PARAM"
#define MSK_IPAR_REMOTE_USE_COMPRESSION_        "MSK_IPAR_REMOTE_USE_COMPRESSION"
#define MSK_IPAR_REMOVE_UNUSED_SOLUTIONS_       "MSK_IPAR_REMOVE_UNUSED_SOLUTIONS"
#define MSK_IPAR_REQUEST_INTPNT_                "MSK_IPAR_REQUEST_INTPNT"
#define MSK_IPAR_SENSITIVITY_ALL_               "MSK_IPAR_SENSITIVITY_ALL"
#define MSK_IPAR_SENSITIVITY_TYPE_              "MSK_IPAR_SENSITIVITY_TYPE"
#define MSK_IPAR_SIM_BASIS_FACTOR_USE_          "MSK_IPAR_SIM_BASIS_FACTOR_USE"
#define MSK_IPAR_SIM_CACHE_                     "MSK_IPAR_SIM_CACHE"
#define MSK_IPAR_SIM_DEGEN_                     "MSK_IPAR_SIM_DEGEN"
#define MSK_IPAR_SIM_DETECT_PWL_                "MSK_IPAR_SIM_DETECT_PWL"
#define MSK_IPAR_SIM_DUAL_CRASH_                "MSK_IPAR_SIM_DUAL_CRASH"
#define MSK_IPAR_SIM_DUAL_PHASEONE_METHOD_      "MSK_IPAR_SIM_DUAL_PHASEONE_METHOD"
#define MSK_IPAR_SIM_DUAL_RESTRICT_SELECTION_   "MSK_IPAR_SIM_DUAL_RESTRICT_SELECTION"
#define MSK_IPAR_SIM_DUAL_SELECTION_            "MSK_IPAR_SIM_DUAL_SELECTION"
#define MSK_IPAR_SIM_HOTSTART_                  "MSK_IPAR_SIM_HOTSTART"
#define MSK_IPAR_SIM_HOTSTART_LU_               "MSK_IPAR_SIM_HOTSTART_LU"
#define MSK_IPAR_SIM_MAX_ITERATIONS_            "MSK_IPAR_SIM_MAX_ITERATIONS"
#define MSK_IPAR_SIM_MAX_NUM_SETBACKS_          "MSK_IPAR_SIM_MAX_NUM_SETBACKS"
#define MSK_IPAR_SIM_NON_SINGULAR_              "MSK_IPAR_SIM_NON_SINGULAR"
#define MSK_IPAR_SIM_PRECISION_                 "MSK_IPAR_SIM_PRECISION"
#define MSK_IPAR_SIM_PRECISION_BOOST_           "MSK_IPAR_SIM_PRECISION_BOOST"
#define MSK_IPAR_SIM_PRIMAL_CRASH_              "MSK_IPAR_SIM_PRIMAL_CRASH"
#define MSK_IPAR_SIM_PRIMAL_PHASEONE_METHOD_    "MSK_IPAR_SIM_PRIMAL_PHASEONE_METHOD"
#define MSK_IPAR_SIM_PRIMAL_RESTRICT_SELECTION_ "MSK_IPAR_SIM_PRIMAL_RESTRICT_SELECTION"
#define MSK_IPAR_SIM_PRIMAL_SELECTION_          "MSK_IPAR_SIM_PRIMAL_SELECTION"
#define MSK_IPAR_SIM_REFACTOR_FREQ_             "MSK_IPAR_SIM_REFACTOR_FREQ"
#define MSK_IPAR_SIM_REFORMULATION_             "MSK_IPAR_SIM_REFORMULATION"
#define MSK_IPAR_SIM_SAVE_LU_                   "MSK_IPAR_SIM_SAVE_LU"
#define MSK_IPAR_SIM_SCALING_                   "MSK_IPAR_SIM_SCALING"
#define MSK_IPAR_SIM_SCALING_METHOD_            "MSK_IPAR_SIM_SCALING_METHOD"
#define MSK_IPAR_SIM_SEED_                      "MSK_IPAR_SIM_SEED"
#define MSK_IPAR_SIM_SOLUTION_REFINEMENT_       "MSK_IPAR_SIM_SOLUTION_REFINEMENT"
#define MSK_IPAR_SIM_SWITCH_OPTIMIZER_          "MSK_IPAR_SIM_SWITCH_OPTIMIZER"
#define MSK_IPAR_SOL_FILTER_KEEP_BASIC_         "MSK_IPAR_SOL_FILTER_KEEP_BASIC"
#define MSK_IPAR_SOL_READ_NAME_WIDTH_           "MSK_IPAR_SOL_READ_NAME_WIDTH"
#define MSK_IPAR_SOL_READ_WIDTH_                "MSK_IPAR_SOL_READ_WIDTH"
#define MSK_IPAR_SOLVE_FORM_                    "MSK_IPAR_SOLVE_FORM"
#define MSK_IPAR_TIMING_LEVEL_                  "MSK_IPAR_TIMING_LEVEL"
#define MSK_IPAR_WRITE_ASYNC_                   "MSK_IPAR_WRITE_ASYNC"
#define MSK_IPAR_WRITE_BAS_CONSTRAINTS_         "MSK_IPAR_WRITE_BAS_CONSTRAINTS"
#define MSK_IPAR_WRITE_BAS_HEAD_                "MSK_IPAR_WRITE_BAS_HEAD"
#define MSK_IPAR_WRITE_BAS_VARIABLES_           "MSK_IPAR_WRITE_BAS_VARIABLES"
#define MSK_IPAR_WRITE_COMPRESSION_             "MSK_IPAR_WRITE_COMPRESSION"
#define MSK_IPAR_WRITE_FREE_CON_                "MSK_IPAR_WRITE_FREE_CON"
#define MSK_IPAR_WRITE_GENERIC_NAMES_           "MSK_IPAR_WRITE_GENERIC_NAMES"
#define MSK_IPAR_WRITE_IGNORE_INCOMPATIBLE_ITEMS_ "MSK_IPAR_WRITE_IGNORE_INCOMPATIBLE_ITEMS"
#define MSK_IPAR_WRITE_INT_CONSTRAINTS_         "MSK_IPAR_WRITE_INT_CONSTRAINTS"
#define MSK_IPAR_WRITE_INT_HEAD_                "MSK_IPAR_WRITE_INT_HEAD"
#define MSK_IPAR_WRITE_INT_VARIABLES_           "MSK_IPAR_WRITE_INT_VARIABLES"
#define MSK_IPAR_WRITE_JSON_INDENTATION_        "MSK_IPAR_WRITE_JSON_INDENTATION"
#define MSK_IPAR_WRITE_LP_FULL_OBJ_             "MSK_IPAR_WRITE_LP_FULL_OBJ"
#define MSK_IPAR_WRITE_LP_IGNORE_FREE_CONSTRAINTS_ "MSK_IPAR_WRITE_LP_IGNORE_FREE_CONSTRAINTS"
#define MSK_IPAR_WRITE_LP_LINE_WIDTH_           "MSK_IPAR_WRITE_LP_LINE_WIDTH"
#define MSK_IPAR_WRITE_MPS_FORMAT_              "MSK_IPAR_WRITE_MPS_FORMAT"
#define MSK_IPAR_WRITE_MPS_INT_                 "MSK_IPAR_WRITE_MPS_INT"
#define MSK_IPAR_WRITE_SOL_BARVARIABLES_        "MSK_IPAR_WRITE_SOL_BARVARIABLES"
#define MSK_IPAR_WRITE_SOL_CONSTRAINTS_         "MSK_IPAR_WRITE_SOL_CONSTRAINTS"
#define MSK_IPAR_WRITE_SOL_HEAD_                "MSK_IPAR_WRITE_SOL_HEAD"
#define MSK_IPAR_WRITE_SOL_IGNORE_INVALID_NAMES_ "MSK_IPAR_WRITE_SOL_IGNORE_INVALID_NAMES"
#define MSK_IPAR_WRITE_SOL_VARIABLES_           "MSK_IPAR_WRITE_SOL_VARIABLES"

enum MSKbranchdir_enum {
  /** The mixed-integer optimizer decides which branch to choose. */
  MSK_BRANCH_DIR_FREE       = 0,
  /** The mixed-integer optimizer always chooses the up branch first. */
  MSK_BRANCH_DIR_UP         = 1,
  /** The mixed-integer optimizer always chooses the down branch first. */
  MSK_BRANCH_DIR_DOWN       = 2,
  /** Branch in direction nearest to selected fractional variable. */
  MSK_BRANCH_DIR_NEAR       = 3,
  /** Branch in direction farthest from selected fractional variable. */
  MSK_BRANCH_DIR_FAR        = 4,
  /** Chose direction based on root lp value of selected variable. */
  MSK_BRANCH_DIR_ROOT_LP    = 5,
  /** Branch in direction of current incumbent. */
  MSK_BRANCH_DIR_GUIDED     = 6,
  /** Branch based on the pseudocost of the variable. */
  MSK_BRANCH_DIR_PSEUDOCOST = 7
}; /* MSKbranchdir_enum */
#define MSK_BRANCH_DIR_BEGIN MSK_BRANCH_DIR_FREE
#define MSK_BRANCH_DIR_END   (1+MSK_BRANCH_DIR_PSEUDOCOST)
#ifdef MSK_NO_ENUMS
typedef int MSKbranchdire;
#else
typedef int MSKbranchdire;
#endif

enum MSKmiqcqoreformmethod_enum {
  /** The mixed-integer optimizer decides which reformulation method to apply. */
  MSK_MIO_QCQO_REFORMULATION_METHOD_FREE             = 0,
  /** No reformulation method is applied. */
  MSK_MIO_QCQO_REFORMULATION_METHOD_NONE             = 1,
  /** A reformulation via linearization is applied. */
  MSK_MIO_QCQO_REFORMULATION_METHOD_LINEARIZATION    = 2,
  /** The eigenvalue method is applied. */
  MSK_MIO_QCQO_REFORMULATION_METHOD_EIGEN_VAL_METHOD = 3,
  /** A perturbation of matrix diagonals via the solution of SDPs is applied. */
  MSK_MIO_QCQO_REFORMULATION_METHOD_DIAG_SDP         = 4,
  /** A Reformulation based on the solution of an SDP-relaxation of the problem is applied. */
  MSK_MIO_QCQO_REFORMULATION_METHOD_RELAX_SDP        = 5
}; /* MSKmiqcqoreformmethod_enum */
#define MSK_MIO_QCQO_REFORMULATION_METHOD_BEGIN MSK_MIO_QCQO_REFORMULATION_METHOD_FREE
#define MSK_MIO_QCQO_REFORMULATION_METHOD_END   (1+MSK_MIO_QCQO_REFORMULATION_METHOD_RELAX_SDP)
#ifdef MSK_NO_ENUMS
typedef int MSKmiqcqoreformmethode;
#else
typedef int MSKmiqcqoreformmethode;
#endif

enum MSKmiodatapermmethod_enum {
  /** No problem data permutation is applied. */
  MSK_MIO_DATA_PERMUTATION_METHOD_NONE         = 0,
  /** A random cyclic shift is applied to permute the problem data. */
  MSK_MIO_DATA_PERMUTATION_METHOD_CYCLIC_SHIFT = 1,
  /** A random permutation is applied to the problem data. */
  MSK_MIO_DATA_PERMUTATION_METHOD_RANDOM       = 2
}; /* MSKmiodatapermmethod_enum */
#define MSK_MIO_DATA_PERMUTATION_METHOD_BEGIN MSK_MIO_DATA_PERMUTATION_METHOD_NONE
#define MSK_MIO_DATA_PERMUTATION_METHOD_END   (1+MSK_MIO_DATA_PERMUTATION_METHOD_RANDOM)
#ifdef MSK_NO_ENUMS
typedef int MSKmiodatapermmethode;
#else
typedef int MSKmiodatapermmethode;
#endif

enum MSKmiocontsoltype_enum {
  /** No interior-point or basic solution. */
  MSK_MIO_CONT_SOL_NONE    = 0,
  /** Solutions to the root node problem. */
  MSK_MIO_CONT_SOL_ROOT    = 1,
  /** A feasible primal solution. */
  MSK_MIO_CONT_SOL_ITG     = 2,
  /** A feasible primal solution or a root node solution if the problem is infeasible. */
  MSK_MIO_CONT_SOL_ITG_REL = 3
}; /* MSKmiocontsoltype_enum */
#define MSK_MIO_CONT_SOL_BEGIN MSK_MIO_CONT_SOL_NONE
#define MSK_MIO_CONT_SOL_END   (1+MSK_MIO_CONT_SOL_ITG_REL)
#ifdef MSK_NO_ENUMS
typedef int MSKmiocontsoltypee;
#else
typedef int MSKmiocontsoltypee;
#endif

enum MSKmiomode_enum {
  /** The integer constraints are ignored and the problem is solved as a continuous problem. */
  MSK_MIO_MODE_IGNORED   = 0,
  /** Integer restrictions should be satisfied. */
  MSK_MIO_MODE_SATISFIED = 1
}; /* MSKmiomode_enum */
#define MSK_MIO_MODE_BEGIN MSK_MIO_MODE_IGNORED
#define MSK_MIO_MODE_END   (1+MSK_MIO_MODE_SATISFIED)
#ifdef MSK_NO_ENUMS
typedef int MSKmiomodee;
#else
typedef int MSKmiomodee;
#endif

enum MSKmionodeseltype_enum {
  /** The optimizer decides the node selection strategy. */
  MSK_MIO_NODE_SELECTION_FREE   = 0,
  /** The optimizer employs a depth first node selection strategy. */
  MSK_MIO_NODE_SELECTION_FIRST  = 1,
  /** The optimizer employs a best bound node selection strategy. */
  MSK_MIO_NODE_SELECTION_BEST   = 2,
  /** The optimizer employs selects the node based on a pseudo cost estimate. */
  MSK_MIO_NODE_SELECTION_PSEUDO = 3
}; /* MSKmionodeseltype_enum */
#define MSK_MIO_NODE_SELECTION_BEGIN MSK_MIO_NODE_SELECTION_FREE
#define MSK_MIO_NODE_SELECTION_END   (1+MSK_MIO_NODE_SELECTION_PSEUDO)
#ifdef MSK_NO_ENUMS
typedef int MSKmionodeseltypee;
#else
typedef int MSKmionodeseltypee;
#endif

enum MSKmiovarseltype_enum {
  /** The optimizer decides the variable selection strategy. */
  MSK_MIO_VAR_SELECTION_FREE       = 0,
  /** The optimizer employs pseudocost variable selection. */
  MSK_MIO_VAR_SELECTION_PSEUDOCOST = 1,
  /** The optimizer employs strong branching varfiable selection */
  MSK_MIO_VAR_SELECTION_STRONG     = 2
}; /* MSKmiovarseltype_enum */
#define MSK_MIO_VAR_SELECTION_BEGIN MSK_MIO_VAR_SELECTION_FREE
#define MSK_MIO_VAR_SELECTION_END   (1+MSK_MIO_VAR_SELECTION_STRONG)
#ifdef MSK_NO_ENUMS
typedef int MSKmiovarseltypee;
#else
typedef int MSKmiovarseltypee;
#endif

enum MSKmpsformat_enum {
  /** It is assumed that the input file satisfies the MPS format strictly. */
  MSK_MPS_FORMAT_STRICT  = 0,
  /** It is assumed that the input file satisfies a slightly relaxed version of the MPS format. */
  MSK_MPS_FORMAT_RELAXED = 1,
  /** It is assumed that the input file satisfies the free MPS format. This implies that spaces are not allowed in names. Otherwise the format is free. */
  MSK_MPS_FORMAT_FREE    = 2,
  /** The CPLEX compatible version of the MPS format is employed. */
  MSK_MPS_FORMAT_CPLEX   = 3
}; /* MSKmpsformat_enum */
#define MSK_MPS_FORMAT_BEGIN MSK_MPS_FORMAT_STRICT
#define MSK_MPS_FORMAT_END   (1+MSK_MPS_FORMAT_CPLEX)
#ifdef MSK_NO_ENUMS
typedef int MSKmpsformate;
#else
typedef int MSKmpsformate;
#endif

enum MSKobjsense_enum {
  /** The problem should be minimized. */
  MSK_OBJECTIVE_SENSE_MINIMIZE = 0,
  /** The problem should be maximized. */
  MSK_OBJECTIVE_SENSE_MAXIMIZE = 1
}; /* MSKobjsense_enum */
#define MSK_OBJECTIVE_SENSE_BEGIN MSK_OBJECTIVE_SENSE_MINIMIZE
#define MSK_OBJECTIVE_SENSE_END   (1+MSK_OBJECTIVE_SENSE_MAXIMIZE)
#ifdef MSK_NO_ENUMS
typedef int MSKobjsensee;
#else
typedef enum MSKobjsense_enum MSKobjsensee;
#endif

enum MSKonoffkey_enum {
  /** Switch the option off. */
  MSK_OFF = 0,
  /** Switch the option on. */
  MSK_ON  = 1
}; /* MSKonoffkey_enum */
#define MSK_BEGIN MSK_OFF
#define MSK_END   (1+MSK_ON)
#ifdef MSK_NO_ENUMS
typedef int MSKonoffkeye;
#else
typedef int MSKonoffkeye;
#endif

enum MSKoptimizertype_enum {
  /** An experimental concurrent optimizer for continous linear optimization problems. */
  MSK_OPTIMIZER_CONCURRENT          = 0,
  /** The optimizer for problems having conic constraints. */
  MSK_OPTIMIZER_CONIC               = 1,
  /** The deprecated interior-point optimizer. */
  MSK_OPTIMIZER_DEPRECATED_INTPNT   = 2,
  /** The dual simplex optimizer. */
  MSK_OPTIMIZER_DUAL_SIMPLEX        = 3,
  /** The optimizer is chosen automatically. */
  MSK_OPTIMIZER_FREE                = 4,
  /** The primal and dual simplex optimizer ran concurrently or a simplex optimizer chosen automatically. */
  MSK_OPTIMIZER_FREE_SIMPLEX        = 5,
  /** The interior-point optimizer. */
  MSK_OPTIMIZER_INTPNT              = 6,
  /** The mixed-integer optimizer. */
  MSK_OPTIMIZER_MIXED_INT           = 7,
  /** The primal-dual simplex optimizer. It is an experimental implementation of the parametric primal-dual simplex algorithm. */
  MSK_OPTIMIZER_PRIMAL_DUAL_SIMPLEX = 8,
  /** The primal simplex optimizer. */
  MSK_OPTIMIZER_PRIMAL_SIMPLEX      = 9
}; /* MSKoptimizertype_enum */
#define MSK_OPTIMIZER_BEGIN MSK_OPTIMIZER_CONCURRENT
#define MSK_OPTIMIZER_END   (1+MSK_OPTIMIZER_PRIMAL_SIMPLEX)
#ifdef MSK_NO_ENUMS
typedef int MSKoptimizertypee;
#else
typedef int MSKoptimizertypee;
#endif

enum MSKorderingtype_enum {
  /** The ordering method is chosen automatically. */
  MSK_ORDER_METHOD_FREE           = 0,
  /** Approximate minimum local fill-in ordering is employed. */
  MSK_ORDER_METHOD_APPMINLOC      = 1,
  /** This option should not be used. */
  MSK_ORDER_METHOD_EXPERIMENTAL   = 2,
  /** Always try the graph partitioning based ordering. */
  MSK_ORDER_METHOD_TRY_GRAPHPAR   = 3,
  /** Always use the graph partitioning based ordering even if it is worse than the approximate minimum local fill ordering. */
  MSK_ORDER_METHOD_FORCE_GRAPHPAR = 4,
  /** No ordering is used. Note using this value almost always leads to a significantly slow down. */
  MSK_ORDER_METHOD_NONE           = 5
}; /* MSKorderingtype_enum */
#define MSK_ORDER_METHOD_BEGIN MSK_ORDER_METHOD_FREE
#define MSK_ORDER_METHOD_END   (1+MSK_ORDER_METHOD_NONE)
#ifdef MSK_NO_ENUMS
typedef int MSKorderingtypee;
#else
typedef int MSKorderingtypee;
#endif

enum MSKpresolvemode_enum {
  /** The problem is not presolved before it is optimized. */
  MSK_PRESOLVE_MODE_OFF  = 0,
  /** The problem is presolved before it is optimized. */
  MSK_PRESOLVE_MODE_ON   = 1,
  /** It is decided automatically whether to presolve before the problem is optimized. */
  MSK_PRESOLVE_MODE_FREE = 2
}; /* MSKpresolvemode_enum */
#define MSK_PRESOLVE_MODE_BEGIN MSK_PRESOLVE_MODE_OFF
#define MSK_PRESOLVE_MODE_END   (1+MSK_PRESOLVE_MODE_FREE)
#ifdef MSK_NO_ENUMS
typedef int MSKpresolvemodee;
#else
typedef int MSKpresolvemodee;
#endif

enum MSKfoldingmode_enum {
  /** Disabled. */
  MSK_FOLDING_MODE_OFF   = 0,
  /** The solver decides on the usage and amount of folding. */
  MSK_FOLDING_MODE_FREE  = 1,
  /** Full folding is always performed regardless of workload. */
  MSK_FOLDING_MODE_FORCE = 2
}; /* MSKfoldingmode_enum */
#define MSK_FOLDING_MODE_BEGIN MSK_FOLDING_MODE_OFF
#define MSK_FOLDING_MODE_END   (1+MSK_FOLDING_MODE_FORCE)
#ifdef MSK_NO_ENUMS
typedef int MSKfoldingmodee;
#else
typedef int MSKfoldingmodee;
#endif

enum MSKfixingmethod_enum {
  /** Allow perturbing the bounds to make solution strictly feasible. */
  MSK_FIXING_PERTURB           = 0,
  /** Round integer variables to exact integers before fixing them. */
  MSK_FIXING_ROUND             = 1,
  /** Round integer variables to exact integers before fixing them and allow perturbing the bounds to make the solution strictly feasible. */
  MSK_FIXING_ROUND_AND_PERTURB = 2,
  /** Only fix the variables to the solution values with neither rounding nor perturbation. */
  MSK_FIXING_DIRECT            = 3
}; /* MSKfixingmethod_enum */
#define MSK_FIXING_BEGIN MSK_FIXING_PERTURB
#define MSK_FIXING_END   (1+MSK_FIXING_DIRECT)
#ifdef MSK_NO_ENUMS
typedef int MSKfixingmethode;
#else
typedef int MSKfixingmethode;
#endif

enum MSKparametertype_enum {
  /** Not a valid parameter. */
  MSK_PAR_INVALID_TYPE = 0,
  /** Is a double parameter. */
  MSK_PAR_DOU_TYPE     = 1,
  /** Is an integer parameter. */
  MSK_PAR_INT_TYPE     = 2,
  /** Is a string parameter. */
  MSK_PAR_STR_TYPE     = 3
}; /* MSKparametertype_enum */
#define MSK_PAR_BEGIN MSK_PAR_INVALID_TYPE
#define MSK_PAR_END   (1+MSK_PAR_STR_TYPE)
#ifdef MSK_NO_ENUMS
typedef int MSKparametertypee;
#else
typedef enum MSKparametertype_enum MSKparametertypee;
#endif

enum MSKproblemitem_enum {
  /** Item is a variable. */
  MSK_PI_VAR  = 0,
  /** Item is a constraint. */
  MSK_PI_CON  = 1,
  /** Item is a cone. */
  MSK_PI_CONE = 2
}; /* MSKproblemitem_enum */
#define MSK_PI_BEGIN MSK_PI_VAR
#define MSK_PI_END   (1+MSK_PI_CONE)
#ifdef MSK_NO_ENUMS
typedef int MSKproblemiteme;
#else
typedef enum MSKproblemitem_enum MSKproblemiteme;
#endif

enum MSKproblemtype_enum {
  /** The problem is a linear optimization problem. */
  MSK_PROBTYPE_LO    = 0,
  /** The problem is a quadratic optimization problem. */
  MSK_PROBTYPE_QO    = 1,
  /** The problem is a quadratically constrained optimization problem. */
  MSK_PROBTYPE_QCQO  = 2,
  /** A conic optimization. */
  MSK_PROBTYPE_CONIC = 3,
  /** General nonlinear constraints and conic constraints. This combination can not be solved by MOSEK. */
  MSK_PROBTYPE_MIXED = 4
}; /* MSKproblemtype_enum */
#define MSK_PROBTYPE_BEGIN MSK_PROBTYPE_LO
#define MSK_PROBTYPE_END   (1+MSK_PROBTYPE_MIXED)
#ifdef MSK_NO_ENUMS
typedef int MSKproblemtypee;
#else
typedef enum MSKproblemtype_enum MSKproblemtypee;
#endif

enum MSKprosta_enum {
  /** Unknown problem status. */
  MSK_PRO_STA_UNKNOWN                  = 0,
  /** The problem is primal and dual feasible. */
  MSK_PRO_STA_PRIM_AND_DUAL_FEAS       = 1,
  /** The problem is primal feasible. */
  MSK_PRO_STA_PRIM_FEAS                = 2,
  /** The problem is dual feasible. */
  MSK_PRO_STA_DUAL_FEAS                = 3,
  /** The problem is primal infeasible. */
  MSK_PRO_STA_PRIM_INFEAS              = 4,
  /** The problem is dual infeasible. */
  MSK_PRO_STA_DUAL_INFEAS              = 5,
  /** The problem is primal and dual infeasible. */
  MSK_PRO_STA_PRIM_AND_DUAL_INFEAS     = 6,
  /** The problem is ill-posed. For example, it may be primal and dual feasible but have a positive duality gap. */
  MSK_PRO_STA_ILL_POSED                = 7,
  /** The problem is either primal infeasible or unbounded. This may occur for mixed-integer problems. */
  MSK_PRO_STA_PRIM_INFEAS_OR_UNBOUNDED = 8
}; /* MSKprosta_enum */
#define MSK_PRO_STA_BEGIN MSK_PRO_STA_UNKNOWN
#define MSK_PRO_STA_END   (1+MSK_PRO_STA_PRIM_INFEAS_OR_UNBOUNDED)
#ifdef MSK_NO_ENUMS
typedef int MSKprostae;
#else
typedef enum MSKprosta_enum MSKprostae;
#endif

enum MSKrescode_enum {
  /** No error occurred. */
  MSK_RES_OK                                                   = 0,
  /** Simplex precision boosting is unavailable on the platform. */
  MSK_RES_WRN_SIM_PRECISION_BOOSTING_IS_UNVAILABLE             = 40,
  /** The parameter file could not be opened. */
  MSK_RES_WRN_OPEN_PARAM_FILE                                  = 50,
  /** A numerically large bound value is specified. */
  MSK_RES_WRN_LARGE_BOUND                                      = 51,
  /** A numerically large lower bound value is specified. */
  MSK_RES_WRN_LARGE_LO_BOUND                                   = 52,
  /** A numerically large upper bound value is specified. */
  MSK_RES_WRN_LARGE_UP_BOUND                                   = 53,
  /** A equality constraint is fixed to numerically large value. */
  MSK_RES_WRN_LARGE_CON_FX                                     = 54,
  /** A numerically large value is specified for one element in c. */
  MSK_RES_WRN_LARGE_CJ                                         = 57,
  /** A numerically large value is specified for an element in A. */
  MSK_RES_WRN_LARGE_AIJ                                        = 62,
  /** One or more zero elements are specified in A. */
  MSK_RES_WRN_ZERO_AIJ                                         = 63,
  /** A name is longer than the buffer that is supposed to hold it. */
  MSK_RES_WRN_NAME_MAX_LEN                                     = 65,
  /** A value for a string parameter is longer than the buffer that is supposed to hold it. */
  MSK_RES_WRN_SPAR_MAX_LEN                                     = 66,
  /** An RHS vector is split into several nonadjacent parts. */
  MSK_RES_WRN_MPS_SPLIT_RHS_VECTOR                             = 70,
  /** A RANGE vector is split into several nonadjacent parts in an MPS file. */
  MSK_RES_WRN_MPS_SPLIT_RAN_VECTOR                             = 71,
  /** A BOUNDS vector is split into several nonadjacent parts in an MPS file. */
  MSK_RES_WRN_MPS_SPLIT_BOU_VECTOR                             = 72,
  /** Missing '/2' after quadratic expressions in bound or objective. */
  MSK_RES_WRN_LP_OLD_QUAD_FORMAT                               = 80,
  /** Ignore a variable because the variable was not previously defined. */
  MSK_RES_WRN_LP_DROP_VARIABLE                                 = 85,
  /** Non-zero elements specified in the upper triangle of a matrix were ignored. */
  MSK_RES_WRN_NZ_IN_UPR_TRI                                    = 200,
  /** One or more non-zero elements were dropped in the Q matrix in the objective. */
  MSK_RES_WRN_DROPPED_NZ_QOBJ                                  = 201,
  /** Ignored integer constraints. */
  MSK_RES_WRN_IGNORE_INTEGER                                   = 250,
  /** The final mixed-integer problem with all the integer variables fixed at their optimal values is infeasible. */
  MSK_RES_WRN_MIO_INFEASIBLE_FINAL                             = 270,
  /** Invalid solution filter is specified. */
  MSK_RES_WRN_SOL_FILTER                                       = 300,
  /** Undefined name occurred in a solution. */
  MSK_RES_WRN_UNDEF_SOL_FILE_NAME                              = 350,
  /** One or more lines in the constraint section were ignored when reading a solution file. */
  MSK_RES_WRN_SOL_FILE_IGNORED_CON                             = 351,
  /** One or more lines in the variable section were ignored when reading a solution file. */
  MSK_RES_WRN_SOL_FILE_IGNORED_VAR                             = 352,
  /** An incomplete basis is specified. */
  MSK_RES_WRN_TOO_FEW_BASIS_VARS                               = 400,
  /** A basis with too many variables is specified. */
  MSK_RES_WRN_TOO_MANY_BASIS_VARS                              = 405,
  /** The license expires. */
  MSK_RES_WRN_LICENSE_EXPIRE                                   = 500,
  /** The license server is not responding. */
  MSK_RES_WRN_LICENSE_SERVER                                   = 501,
  /** A variable or constraint name is empty. The output file may be invalid. */
  MSK_RES_WRN_EMPTY_NAME                                       = 502,
  /** Generic names are used because a name is invalid for requested format. */
  MSK_RES_WRN_USING_GENERIC_NAMES                              = 503,
  /** A name e.g. a row name is not a valid MPS name. */
  MSK_RES_WRN_INVALID_MPS_NAME                                 = 504,
  /** The objective name is not a valid MPS name. */
  MSK_RES_WRN_INVALID_MPS_OBJ_NAME                             = 505,
  /** The license expires. */
  MSK_RES_WRN_LICENSE_FEATURE_EXPIRE                           = 509,
  /** Parameter name not recognized. */
  MSK_RES_WRN_PARAM_NAME_DOU                                   = 510,
  /** Parameter name not recognized. */
  MSK_RES_WRN_PARAM_NAME_INT                                   = 511,
  /** Parameter name not recognized. */
  MSK_RES_WRN_PARAM_NAME_STR                                   = 512,
  /** A parameter value is not correct. */
  MSK_RES_WRN_PARAM_STR_VALUE                                  = 515,
  /** A parameter was ignored by the conic mixed integer optimizer. */
  MSK_RES_WRN_PARAM_IGNORED_CMIO                               = 516,
  /** One or more (near) zero elements are specified in a sparse row of a matrix. */
  MSK_RES_WRN_ZEROS_IN_SPARSE_ROW                              = 705,
  /** One or more (near) zero elements are specified in a sparse column of a matrix. */
  MSK_RES_WRN_ZEROS_IN_SPARSE_COL                              = 710,
  /** The linear dependency check(s) is incomplete. */
  MSK_RES_WRN_INCOMPLETE_LINEAR_DEPENDENCY_CHECK               = 800,
  /** The eliminator is skipped at least once due to lack of space. */
  MSK_RES_WRN_ELIMINATOR_SPACE                                 = 801,
  /** The presolve is incomplete due to lack of space. */
  MSK_RES_WRN_PRESOLVE_OUTOFSPACE                              = 802,
  /** The presolve perturbed the bounds of the primal problem. This is an indication that the problem is nearly infeasible. */
  MSK_RES_WRN_PRESOLVE_PRIMAL_PERTURBATIONS                    = 803,
  /** Some names were changed because they were invalid for the output file format. */
  MSK_RES_WRN_WRITE_CHANGED_NAMES                              = 830,
  /** The fixed objective term was discarded in the output file. */
  MSK_RES_WRN_WRITE_DISCARDED_CFIX                             = 831,
  /** Two constraint names are identical. */
  MSK_RES_WRN_DUPLICATE_CONSTRAINT_NAMES                       = 850,
  /** Two variable names are identical. */
  MSK_RES_WRN_DUPLICATE_VARIABLE_NAMES                         = 851,
  /** Two barvariable names are identical. */
  MSK_RES_WRN_DUPLICATE_BARVARIABLE_NAMES                      = 852,
  /** Two cone names are identical. */
  MSK_RES_WRN_DUPLICATE_CONE_NAMES                             = 853,
  /** Warn against very large bounds. */
  MSK_RES_WRN_ANA_LARGE_BOUNDS                                 = 900,
  /** Warn against all objective coefficients being zero. */
  MSK_RES_WRN_ANA_C_ZERO                                       = 901,
  /** Warn against empty columns. */
  MSK_RES_WRN_ANA_EMPTY_COLS                                   = 902,
  /** Warn against close bounds. */
  MSK_RES_WRN_ANA_CLOSE_BOUNDS                                 = 903,
  /** Warn against almost integral bounds. */
  MSK_RES_WRN_ANA_ALMOST_INT_BOUNDS                            = 904,
  /** An infeasibility report is not available when the problem contains matrix variables. */
  MSK_RES_WRN_NO_INFEASIBILITY_REPORT_WHEN_MATRIX_VARIABLES    = 930,
  /** Dualizer ignores integer variables and disjunctive constraints. */
  MSK_RES_WRN_GETDUAL_IGNORES_INTEGRALITY                      = 940,
  /** No automatic dualizer is available for the specified problem. */
  MSK_RES_WRN_NO_DUALIZER                                      = 950,
  /** A numerically large value is specified for an element in E. */
  MSK_RES_WRN_SYM_MAT_LARGE                                    = 960,
  /** A double parameter related to solver tolerances has a non-default value. */
  MSK_RES_WRN_MODIFIED_DOUBLE_PARAMETER                        = 970,
  /** A numerically large value is specified for an element in F. */
  MSK_RES_WRN_LARGE_FIJ                                        = 980,
  /** Unexpected section in PTF file */
  MSK_RES_WRN_PTF_UNKNOWN_SECTION                              = 981,
  /** Invalid license. */
  MSK_RES_ERR_LICENSE                                          = 1000,
  /** The license has expired. */
  MSK_RES_ERR_LICENSE_EXPIRED                                  = 1001,
  /** Invalid license version. */
  MSK_RES_ERR_LICENSE_VERSION                                  = 1002,
  /** The license server version is too old. */
  MSK_RES_ERR_LICENSE_OLD_SERVER_VERSION                       = 1003,
  /** The problem is bigger than the license. */
  MSK_RES_ERR_SIZE_LICENSE                                     = 1005,
  /** The software is not licensed to solve the problem. */
  MSK_RES_ERR_PROB_LICENSE                                     = 1006,
  /** Invalid license file. */
  MSK_RES_ERR_FILE_LICENSE                                     = 1007,
  /** A license cannot be located. */
  MSK_RES_ERR_MISSING_LICENSE_FILE                             = 1008,
  /** The problem has too many constraints. */
  MSK_RES_ERR_SIZE_LICENSE_CON                                 = 1010,
  /** The problem has too many variables. */
  MSK_RES_ERR_SIZE_LICENSE_VAR                                 = 1011,
  /** The problem contains too many integer variables. */
  MSK_RES_ERR_SIZE_LICENSE_INTVAR                              = 1012,
  /** The optimizer required is not licensed. */
  MSK_RES_ERR_OPTIMIZER_LICENSE                                = 1013,
  /** The license manager reported an error. */
  MSK_RES_ERR_FLEXLM                                           = 1014,
  /** The license server is not responding. */
  MSK_RES_ERR_LICENSE_SERVER                                   = 1015,
  /** Maximum number of licenses is reached. */
  MSK_RES_ERR_LICENSE_MAX                                      = 1016,
  /** The MOSEKLM license manager daemon is not up and running. */
  MSK_RES_ERR_LICENSE_MOSEKLM_DAEMON                           = 1017,
  /** A requested feature is not available in the license file(s). */
  MSK_RES_ERR_LICENSE_FEATURE                                  = 1018,
  /** A requested license feature is not available for the required platform. */
  MSK_RES_ERR_PLATFORM_NOT_LICENSED                            = 1019,
  /** The license system cannot allocate the memory required. */
  MSK_RES_ERR_LICENSE_CANNOT_ALLOCATE                          = 1020,
  /** MOSEK cannot connect to the license server. */
  MSK_RES_ERR_LICENSE_CANNOT_CONNECT                           = 1021,
  /** The host ID specified in the license file does not match the host ID of the computer. */
  MSK_RES_ERR_LICENSE_INVALID_HOSTID                           = 1025,
  /** The version specified in the checkout request is greater than the highest version number the daemon supports. */
  MSK_RES_ERR_LICENSE_SERVER_VERSION                           = 1026,
  /** The license server does not support the requested feature. */
  MSK_RES_ERR_LICENSE_NO_SERVER_SUPPORT                        = 1027,
  /** No SERVER lines in license file. */
  MSK_RES_ERR_LICENSE_NO_SERVER_LINE                           = 1028,
  /** The dynamic link library is older than the specified version. */
  MSK_RES_ERR_OLDER_DLL                                        = 1035,
  /** The dynamic link library is newer than the specified version. */
  MSK_RES_ERR_NEWER_DLL                                        = 1036,
  /** A file cannot be linked to a stream in the DLL version. */
  MSK_RES_ERR_LINK_FILE_DLL                                    = 1040,
  /** Could not initialize a mutex. */
  MSK_RES_ERR_THREAD_MUTEX_INIT                                = 1045,
  /** Could not lock a mutex. */
  MSK_RES_ERR_THREAD_MUTEX_LOCK                                = 1046,
  /** Could not unlock a mutex. */
  MSK_RES_ERR_THREAD_MUTEX_UNLOCK                              = 1047,
  /** Could not create a thread. */
  MSK_RES_ERR_THREAD_CREATE                                    = 1048,
  /** Could not initialize a condition. */
  MSK_RES_ERR_THREAD_COND_INIT                                 = 1049,
  /** Unknown error. */
  MSK_RES_ERR_UNKNOWN                                          = 1050,
  /** Out of space. */
  MSK_RES_ERR_SPACE                                            = 1051,
  /** An error occurred while opening a file. */
  MSK_RES_ERR_FILE_OPEN                                        = 1052,
  /** An error occurred while reading file. */
  MSK_RES_ERR_FILE_READ                                        = 1053,
  /** An error occurred while writing to a file. */
  MSK_RES_ERR_FILE_WRITE                                       = 1054,
  /** The data file format cannot be determined from the file name. */
  MSK_RES_ERR_DATA_FILE_EXT                                    = 1055,
  /** An invalid file name has been specified. */
  MSK_RES_ERR_INVALID_FILE_NAME                                = 1056,
  /** An invalid file name has been specified. */
  MSK_RES_ERR_INVALID_SOL_FILE_NAME                            = 1057,
  /** End of file has been reached unexpectedly. */
  MSK_RES_ERR_END_OF_FILE                                      = 1059,
  /** env is a null pointer. */
  MSK_RES_ERR_NULL_ENV                                         = 1060,
  /** task is a null pointer. */
  MSK_RES_ERR_NULL_TASK                                        = 1061,
  /** An invalid stream is referenced. */
  MSK_RES_ERR_INVALID_STREAM                                   = 1062,
  /** Environment is not initialized. */
  MSK_RES_ERR_NO_INIT_ENV                                      = 1063,
  /** The task is invalid. */
  MSK_RES_ERR_INVALID_TASK                                     = 1064,
  /** An argument to a function is unexpectedly a null pointer. */
  MSK_RES_ERR_NULL_POINTER                                     = 1065,
  /** Not all tasks associated with the environment have been deleted. */
  MSK_RES_ERR_LIVING_TASKS                                     = 1066,
  /** Error encountered in GZIP stream. */
  MSK_RES_ERR_READ_GZIP                                        = 1067,
  /** Error encountered in ZSTD stream. */
  MSK_RES_ERR_READ_ZSTD                                        = 1068,
  /** Error encountered in async stream. */
  MSK_RES_ERR_READ_ASYNC                                       = 1069,
  /** An all blank name has been specified. */
  MSK_RES_ERR_BLANK_NAME                                       = 1070,
  /** Duplicate names specified. */
  MSK_RES_ERR_DUP_NAME                                         = 1071,
  /** The name format string is invalid. */
  MSK_RES_ERR_FORMAT_STRING                                    = 1072,
  /** The sparsity included an index that was out of bounds of the shape. */
  MSK_RES_ERR_SPARSITY_SPECIFICATION                           = 1073,
  /** Mismatching dimensions specified in arguments */
  MSK_RES_ERR_MISMATCHING_DIMENSION                            = 1074,
  /** An invalid objective name is specified. */
  MSK_RES_ERR_INVALID_OBJ_NAME                                 = 1075,
  /** An invalid constraint name is used. */
  MSK_RES_ERR_INVALID_CON_NAME                                 = 1076,
  /** An invalid variable name is used. */
  MSK_RES_ERR_INVALID_VAR_NAME                                 = 1077,
  /** An invalid cone name is used. */
  MSK_RES_ERR_INVALID_CONE_NAME                                = 1078,
  /** An invalid symmetric matrix variable name is used. */
  MSK_RES_ERR_INVALID_BARVAR_NAME                              = 1079,
  /** MOSEK is leaking memory. */
  MSK_RES_ERR_SPACE_LEAKING                                    = 1080,
  /** No available information about the space usage. */
  MSK_RES_ERR_SPACE_NO_INFO                                    = 1081,
  /** Invalid dimension specification */
  MSK_RES_ERR_DIMENSION_SPECIFICATION                          = 1082,
  /** Invalid axis names specification */
  MSK_RES_ERR_AXIS_NAME_SPECIFICATION                          = 1083,
  /** Encountered premature end-of-file in input stream. */
  MSK_RES_ERR_READ_PREMATURE_EOF                               = 1089,
  /** The specified format cannot be read. */
  MSK_RES_ERR_READ_FORMAT                                      = 1090,
  /** Invalid variable name. Cannot write valid LP file. */
  MSK_RES_ERR_WRITE_LP_INVALID_VAR_NAMES                       = 1091,
  /** Duplicate variable names. Cannot write valid LP file. */
  MSK_RES_ERR_WRITE_LP_DUPLICATE_VAR_NAMES                     = 1092,
  /** Invalid constraint name. Cannot write valid LP file. */
  MSK_RES_ERR_WRITE_LP_INVALID_CON_NAMES                       = 1093,
  /** Duplicate constraint names. Cannot write valid LP file. */
  MSK_RES_ERR_WRITE_LP_DUPLICATE_CON_NAMES                     = 1094,
  /** An error occurred while reading an MPS file. */
  MSK_RES_ERR_MPS_FILE                                         = 1100,
  /** Invalid field occurred while reading an MPS file. */
  MSK_RES_ERR_MPS_INV_FIELD                                    = 1101,
  /** An invalid marker has been specified in the MPS file. */
  MSK_RES_ERR_MPS_INV_MARKER                                   = 1102,
  /** An empty constraint name is used in an MPS file. */
  MSK_RES_ERR_MPS_NULL_CON_NAME                                = 1103,
  /** An empty variable name is used in an MPS file. */
  MSK_RES_ERR_MPS_NULL_VAR_NAME                                = 1104,
  /** An undefined constraint name occurred in an MPS file. */
  MSK_RES_ERR_MPS_UNDEF_CON_NAME                               = 1105,
  /** An undefined variable name occurred in an MPS file. */
  MSK_RES_ERR_MPS_UNDEF_VAR_NAME                               = 1106,
  /** An invalid constraint key occurred in an MPS file. */
  MSK_RES_ERR_MPS_INVALID_CON_KEY                              = 1107,
  /** An invalid bound key occurred in an MPS file. */
  MSK_RES_ERR_MPS_INVALID_BOUND_KEY                            = 1108,
  /** An invalid section name occurred in an MPS file. */
  MSK_RES_ERR_MPS_INVALID_SEC_NAME                             = 1109,
  /** No objective is defined in an MPS file. */
  MSK_RES_ERR_MPS_NO_OBJECTIVE                                 = 1110,
  /** The non-zero elements in A corresponding to a variable in an MPS file must be specified consecutively. */
  MSK_RES_ERR_MPS_SPLITTED_VAR                                 = 1111,
  /** A constraint name is specified multiple times in the ROWS section in an MPS file. */
  MSK_RES_ERR_MPS_MUL_CON_NAME                                 = 1112,
  /** Multiple QSECTIONs are specified for a constraint. */
  MSK_RES_ERR_MPS_MUL_QSEC                                     = 1113,
  /** The Q term in the objective is specified multiple times. */
  MSK_RES_ERR_MPS_MUL_QOBJ                                     = 1114,
  /** The sections in an MPS file is not in the correct order. */
  MSK_RES_ERR_MPS_INV_SEC_ORDER                                = 1115,
  /** Multiple CSECTIONs are given the same name. */
  MSK_RES_ERR_MPS_MUL_CSEC                                     = 1116,
  /** Invalid cone type specified in a  CSECTION. */
  MSK_RES_ERR_MPS_CONE_TYPE                                    = 1117,
  /** A variable is specified to be a member of several cones. */
  MSK_RES_ERR_MPS_CONE_OVERLAP                                 = 1118,
  /** A variable is repeated within the CSECTION. */
  MSK_RES_ERR_MPS_CONE_REPEAT                                  = 1119,
  /** A non symmetric matrix has been speciefied. */
  MSK_RES_ERR_MPS_NON_SYMMETRIC_Q                              = 1120,
  /** Duplicate elements is specified in a Q matrix. */
  MSK_RES_ERR_MPS_DUPLICATE_Q_ELEMENT                          = 1121,
  /** An invalid objective sense is specified. */
  MSK_RES_ERR_MPS_INVALID_OBJSENSE                             = 1122,
  /** A tab char occurred in field 2. */
  MSK_RES_ERR_MPS_TAB_IN_FIELD2                                = 1125,
  /** A tab char occurred in field 3. */
  MSK_RES_ERR_MPS_TAB_IN_FIELD3                                = 1126,
  /** A tab char occurred in field 5. */
  MSK_RES_ERR_MPS_TAB_IN_FIELD5                                = 1127,
  /** An invalid objective name is specified. */
  MSK_RES_ERR_MPS_INVALID_OBJ_NAME                             = 1128,
  /** An invalid indicator key occurred in an MPS file. */
  MSK_RES_ERR_MPS_INVALID_KEY                                  = 1129,
  /** An invalid indicator constraint is used. It must not be a ranged constraint. */
  MSK_RES_ERR_MPS_INVALID_INDICATOR_CONSTRAINT                 = 1130,
  /** An invalid indicator variable is specfied. It must be a binary variable. */
  MSK_RES_ERR_MPS_INVALID_INDICATOR_VARIABLE                   = 1131,
  /** An invalid indicator value is specfied. It must be either 0 or 1. */
  MSK_RES_ERR_MPS_INVALID_INDICATOR_VALUE                      = 1132,
  /** A quadratic constraint can be be an indicator constraint. */
  MSK_RES_ERR_MPS_INVALID_INDICATOR_QUADRATIC_CONSTRAINT       = 1133,
  /** Syntax error in an OPF file */
  MSK_RES_ERR_OPF_SYNTAX                                       = 1134,
  /** Premature end of file in an OPF file. */
  MSK_RES_ERR_OPF_PREMATURE_EOF                                = 1136,
  /** Mismatched end-tag in OPF file */
  MSK_RES_ERR_OPF_MISMATCHED_TAG                               = 1137,
  /** Either upper or lower bound was specified twice in OPF file */
  MSK_RES_ERR_OPF_DUPLICATE_BOUND                              = 1138,
  /** Duplicate constraint name in OPF File */
  MSK_RES_ERR_OPF_DUPLICATE_CONSTRAINT_NAME                    = 1139,
  /** Invalid cone type in OPF File */
  MSK_RES_ERR_OPF_INVALID_CONE_TYPE                            = 1140,
  /** Invalid number of parameters in start-tag in OPF File */
  MSK_RES_ERR_OPF_INCORRECT_TAG_PARAM                          = 1141,
  /** Invalid start-tag in OPF File */
  MSK_RES_ERR_OPF_INVALID_TAG                                  = 1142,
  /** Same variable appears in multiple cones in OPF File */
  MSK_RES_ERR_OPF_DUPLICATE_CONE_ENTRY                         = 1143,
  /** The problem is too large to be correctly loaded */
  MSK_RES_ERR_OPF_TOO_LARGE                                    = 1144,
  /** Dual solution values are not allowed in OPF File */
  MSK_RES_ERR_OPF_DUAL_INTEGER_SOLUTION                        = 1146,
  /** The problem cannot be written to an LP formatted file. */
  MSK_RES_ERR_LP_EMPTY                                         = 1151,
  /** An invalid name is created while writing an MPS file. */
  MSK_RES_ERR_WRITE_MPS_INVALID_NAME                           = 1153,
  /** A variable name is invalid when used in an LP formatted file. */
  MSK_RES_ERR_LP_INVALID_VAR_NAME                              = 1154,
  /** Empty variable names cannot be written to OPF files. */
  MSK_RES_ERR_WRITE_OPF_INVALID_VAR_NAME                       = 1156,
  /** Syntax error in an LP file. */
  MSK_RES_ERR_LP_FILE_FORMAT                                   = 1157,
  /** Expected a number in LP file */
  MSK_RES_ERR_LP_EXPECTED_NUMBER                               = 1158,
  /** Syntax error in LP fil. Possibly missing End tag. */
  MSK_RES_ERR_READ_LP_MISSING_END_TAG                          = 1159,
  /** An indicator variable was not declared binary */
  MSK_RES_ERR_LP_INDICATOR_VAR                                 = 1160,
  /** Expected an objective section in LP file */
  MSK_RES_ERR_LP_EXPECTED_OBJECTIVE                            = 1161,
  /** Expected constraint relation */
  MSK_RES_ERR_LP_EXPECTED_CONSTRAINT_RELATION                  = 1162,
  /** Constraint has ambiguous or invalid bound */
  MSK_RES_ERR_LP_AMBIGUOUS_CONSTRAINT_BOUND                    = 1163,
  /** Duplicate section */
  MSK_RES_ERR_LP_DUPLICATE_SECTION                             = 1164,
  /** Duplicate section */
  MSK_RES_ERR_READ_LP_DELAYED_ROWS_NOT_SUPPORTED               = 1165,
  /** An error occurred while writing file */
  MSK_RES_ERR_WRITING_FILE                                     = 1166,
  /** An error occurred while performing asynchronous writing */
  MSK_RES_ERR_WRITE_ASYNC                                      = 1167,
  /** An invalid name occurred in a solution file. */
  MSK_RES_ERR_INVALID_NAME_IN_SOL_FILE                         = 1170,
  /** Syntax error in an JSON data */
  MSK_RES_ERR_JSON_SYNTAX                                      = 1175,
  /** Error in JSON string. */
  MSK_RES_ERR_JSON_STRING                                      = 1176,
  /** Invalid number entry - wrong type or value overflow. */
  MSK_RES_ERR_JSON_NUMBER_OVERFLOW                             = 1177,
  /** Error in an JSON Task file */
  MSK_RES_ERR_JSON_FORMAT                                      = 1178,
  /** Inconsistent data in JSON Task file */
  MSK_RES_ERR_JSON_DATA                                        = 1179,
  /** Missing data section in JSON task file. */
  MSK_RES_ERR_JSON_MISSING_DATA                                = 1180,
  /** Incompatible item */
  MSK_RES_ERR_PTF_INCOMPATIBILITY                              = 1181,
  /** Undefined symbol referenced */
  MSK_RES_ERR_PTF_UNDEFINED_ITEM                               = 1182,
  /** Inconsistent size of item */
  MSK_RES_ERR_PTF_INCONSISTENCY                                = 1183,
  /** Syntax error in an PTF file */
  MSK_RES_ERR_PTF_FORMAT                                       = 1184,
  /** Inconsistency in serialized update. */
  MSK_RES_ERR_INCONSISTENT_UPDATE                              = 1189,
  /** Syntax error in an B file. */
  MSK_RES_ERR_B_FORMAT                                         = 1190,
  /** Incorrect length of arguments. */
  MSK_RES_ERR_ARGUMENT_LENNEQ                                  = 1197,
  /** Incorrect argument type. */
  MSK_RES_ERR_ARGUMENT_TYPE                                    = 1198,
  /** Incorrect number of function arguments. */
  MSK_RES_ERR_NUM_ARGUMENTS                                    = 1199,
  /** A function argument is incorrect. */
  MSK_RES_ERR_IN_ARGUMENT                                      = 1200,
  /** A function argument is of incorrect dimension. */
  MSK_RES_ERR_ARGUMENT_DIMENSION                               = 1201,
  /** The size of the n-dimensional shape is too large. */
  MSK_RES_ERR_SHAPE_IS_TOO_LARGE                               = 1202,
  /** An index in an argument is too small. */
  MSK_RES_ERR_INDEX_IS_TOO_SMALL                               = 1203,
  /** An index in an argument is too large. */
  MSK_RES_ERR_INDEX_IS_TOO_LARGE                               = 1204,
  /** An index in an argument is not unique. */
  MSK_RES_ERR_INDEX_IS_NOT_UNIQUE                              = 1205,
  /** A parameter name is not correct. */
  MSK_RES_ERR_PARAM_NAME                                       = 1206,
  /** A parameter name is not correct. */
  MSK_RES_ERR_PARAM_NAME_DOU                                   = 1207,
  /** A parameter name is not correct. */
  MSK_RES_ERR_PARAM_NAME_INT                                   = 1208,
  /** A parameter name is not correct. */
  MSK_RES_ERR_PARAM_NAME_STR                                   = 1209,
  /** Parameter index is out of range. */
  MSK_RES_ERR_PARAM_INDEX                                      = 1210,
  /** A parameter value is too large. */
  MSK_RES_ERR_PARAM_IS_TOO_LARGE                               = 1215,
  /** A parameter value is too small. */
  MSK_RES_ERR_PARAM_IS_TOO_SMALL                               = 1216,
  /** A parameter value string is incorrect. */
  MSK_RES_ERR_PARAM_VALUE_STR                                  = 1217,
  /** A parameter type is invalid. */
  MSK_RES_ERR_PARAM_TYPE                                       = 1218,
  /** A double information index is out of range for the specified type. */
  MSK_RES_ERR_INF_DOU_INDEX                                    = 1219,
  /** An integer information index is out of range for the specified type. */
  MSK_RES_ERR_INF_INT_INDEX                                    = 1220,
  /** An index in an array argument is too small. */
  MSK_RES_ERR_INDEX_ARR_IS_TOO_SMALL                           = 1221,
  /** An index in an array argument is too large. */
  MSK_RES_ERR_INDEX_ARR_IS_TOO_LARGE                           = 1222,
  /** A long integer information index is out of range for the specified type. */
  MSK_RES_ERR_INF_LINT_INDEX                                   = 1225,
  /** The value of a argument is too small. */
  MSK_RES_ERR_ARG_IS_TOO_SMALL                                 = 1226,
  /** The value of a argument is too large. */
  MSK_RES_ERR_ARG_IS_TOO_LARGE                                 = 1227,
  /** whichsol is invalid. */
  MSK_RES_ERR_INVALID_WHICHSOL                                 = 1228,
  /** A double information name is invalid. */
  MSK_RES_ERR_INF_DOU_NAME                                     = 1230,
  /** An integer information name is invalid. */
  MSK_RES_ERR_INF_INT_NAME                                     = 1231,
  /** The information type is invalid. */
  MSK_RES_ERR_INF_TYPE                                         = 1232,
  /** A long integer information name is invalid. */
  MSK_RES_ERR_INF_LINT_NAME                                    = 1234,
  /** An index is out of range. */
  MSK_RES_ERR_INDEX                                            = 1235,
  /** The solution defined by whichsol does not exists. */
  MSK_RES_ERR_WHICHSOL                                         = 1236,
  /** The solution number  solemn does not exists. */
  MSK_RES_ERR_SOLITEM                                          = 1237,
  /** whichitem is unacceptable. */
  MSK_RES_ERR_WHICHITEM_NOT_ALLOWED                            = 1238,
  /** Invalid maximum number of constraints specified. */
  MSK_RES_ERR_MAXNUMCON                                        = 1240,
  /** The maximum number of variables limit is too small. */
  MSK_RES_ERR_MAXNUMVAR                                        = 1241,
  /** The maximum number of semidefinite variables limit is too small. */
  MSK_RES_ERR_MAXNUMBARVAR                                     = 1242,
  /** Too small maximum number of non-zeros for the Q matrices is specified. */
  MSK_RES_ERR_MAXNUMQNZ                                        = 1243,
  /** The maximum number of non-zeros specified is too small. */
  MSK_RES_ERR_TOO_SMALL_MAX_NUM_NZ                             = 1245,
  /** A specified index is invalid. */
  MSK_RES_ERR_INVALID_IDX                                      = 1246,
  /** A specified index is invalid. */
  MSK_RES_ERR_INVALID_MAX_NUM                                  = 1247,
  /** The value of whichsol is not allowed. */
  MSK_RES_ERR_UNALLOWED_WHICHSOL                               = 1248,
  /** Maximum number of constraints is exceeded. */
  MSK_RES_ERR_MAX_NUM_CON_EXCEEDED                             = 1250,
  /** Maximum number of variables is exceeded. */
  MSK_RES_ERR_MAX_NUM_VAR_EXCEEDED                             = 1251,
  /** Too small maximum number of non-zeros in A specified. */
  MSK_RES_ERR_TOO_SMALL_MAXNUMANZ                              = 1252,
  /** aptre[j] is strictly smaller than aptrb[j] for some j. */
  MSK_RES_ERR_INV_APTRE                                        = 1253,
  /** An element in A is defined multiple times. */
  MSK_RES_ERR_MUL_A_ELEMENT                                    = 1254,
  /** Invalid bound key. */
  MSK_RES_ERR_INV_BK                                           = 1255,
  /** Invalid bound key is specified for a constraint. */
  MSK_RES_ERR_INV_BKC                                          = 1256,
  /** An invalid bound key is specified for a variable. */
  MSK_RES_ERR_INV_BKX                                          = 1257,
  /** An invalid variable type is specified for a variable. */
  MSK_RES_ERR_INV_VAR_TYPE                                     = 1258,
  /** Problem type does not match the chosen optimizer. */
  MSK_RES_ERR_SOLVER_PROBTYPE                                  = 1259,
  /** Empty objective range. */
  MSK_RES_ERR_OBJECTIVE_RANGE                                  = 1260,
  /** Invalid response code. */
  MSK_RES_ERR_INV_RESCODE                                      = 1261,
  /** Invalid integer information item. */
  MSK_RES_ERR_INV_IINF                                         = 1262,
  /** Invalid long integer information item. */
  MSK_RES_ERR_INV_LIINF                                        = 1263,
  /** Invalid double information item. */
  MSK_RES_ERR_INV_DINF                                         = 1264,
  /** Invalid basis is specified. */
  MSK_RES_ERR_BASIS                                            = 1266,
  /** Invalid value in skc encountered. */
  MSK_RES_ERR_INV_SKC                                          = 1267,
  /** Invalid value in skx encountered. */
  MSK_RES_ERR_INV_SKX                                          = 1268,
  /** Invalid status key string encountered. */
  MSK_RES_ERR_INV_SK_STR                                       = 1269,
  /** Invalid status key code encountered. */
  MSK_RES_ERR_INV_SK                                           = 1270,
  /** Invalid cone type string encountered. */
  MSK_RES_ERR_INV_CONE_TYPE_STR                                = 1271,
  /** Invalid cone type code encountered. */
  MSK_RES_ERR_INV_CONE_TYPE                                    = 1272,
  /** Invalid value in skn encountered. */
  MSK_RES_ERR_INV_SKN                                          = 1274,
  /** Invalid surplus. */
  MSK_RES_ERR_INVALID_SURPLUS                                  = 1275,
  /** An invalid name item code is used. */
  MSK_RES_ERR_INV_NAME_ITEM                                    = 1280,
  /** An invalid problem item is used. */
  MSK_RES_ERR_PRO_ITEM                                         = 1281,
  /** Invalid format type. */
  MSK_RES_ERR_INVALID_FORMAT_TYPE                              = 1283,
  /** Invalid firsti. */
  MSK_RES_ERR_FIRSTI                                           = 1285,
  /** Invalid lasti. */
  MSK_RES_ERR_LASTI                                            = 1286,
  /** Invalid firstj. */
  MSK_RES_ERR_FIRSTJ                                           = 1287,
  /** Invalid lastj. */
  MSK_RES_ERR_LASTJ                                            = 1288,
  /** A maximum length that is too small has been specified. */
  MSK_RES_ERR_MAX_LEN_IS_TOO_SMALL                             = 1289,
  /** The model contains a nonlinear equality. */
  MSK_RES_ERR_NONLINEAR_EQUALITY                               = 1290,
  /** The optimization problem is nonconvex. */
  MSK_RES_ERR_NONCONVEX                                        = 1291,
  /** The problem contains a nonlinear constraint with inite lower and upper bound. */
  MSK_RES_ERR_NONLINEAR_RANGED                                 = 1292,
  /** The quadratic constraint matrix is not PSD. */
  MSK_RES_ERR_CON_Q_NOT_PSD                                    = 1293,
  /** The quadratic constraint matrix is not NSD. */
  MSK_RES_ERR_CON_Q_NOT_NSD                                    = 1294,
  /** The quadratic coefficient matrix in the objective is not PSD. */
  MSK_RES_ERR_OBJ_Q_NOT_PSD                                    = 1295,
  /** The quadratic coefficient matrix in the objective is not NSD. */
  MSK_RES_ERR_OBJ_Q_NOT_NSD                                    = 1296,
  /** An invalid permutation array is specified. */
  MSK_RES_ERR_ARGUMENT_PERM_ARRAY                              = 1299,
  /** An index of a non-existing cone has been specified. */
  MSK_RES_ERR_CONE_INDEX                                       = 1300,
  /** A cone with incorrect number of members is specified. */
  MSK_RES_ERR_CONE_SIZE                                        = 1301,
  /** One or more of variables in the cone to be added is already member of another cone. */
  MSK_RES_ERR_CONE_OVERLAP                                     = 1302,
  /** A variable is included multiple times in the cone. */
  MSK_RES_ERR_CONE_REP_VAR                                     = 1303,
  /** The value specified for maxnumcone is too small. */
  MSK_RES_ERR_MAXNUMCONE                                       = 1304,
  /** Invalid cone type specified. */
  MSK_RES_ERR_CONE_TYPE                                        = 1305,
  /** Invalid cone type specified. */
  MSK_RES_ERR_CONE_TYPE_STR                                    = 1306,
  /** The cone to be appended has one variable which is already member of another cone. */
  MSK_RES_ERR_CONE_OVERLAP_APPEND                              = 1307,
  /** A variable cannot be removed because it will make a cone invalid. */
  MSK_RES_ERR_REMOVE_CONE_VARIABLE                             = 1310,
  /** Trying to append a too big cone. */
  MSK_RES_ERR_APPENDING_TOO_BIG_CONE                           = 1311,
  /** An invalid cone parameter. */
  MSK_RES_ERR_CONE_PARAMETER                                   = 1320,
  /** An invalid number is specified in a solution file. */
  MSK_RES_ERR_SOL_FILE_INVALID_NUMBER                          = 1350,
  /** A huge value in absolute size is specified for an objective coefficient. */
  MSK_RES_ERR_HUGE_C                                           = 1375,
  /** A numerically huge value is specified for an element in A. */
  MSK_RES_ERR_HUGE_AIJ                                         = 1380,
  /** An element in the A matrix is specified twice. */
  MSK_RES_ERR_DUPLICATE_AIJ                                    = 1385,
  /** The lower bound specified is not a number (nan) or is not finite. */
  MSK_RES_ERR_LOWER_BOUND_IS_A_NAN                             = 1390,
  /** The upper bound specified is not a number (nan) or is not finite. */
  MSK_RES_ERR_UPPER_BOUND_IS_A_NAN                             = 1391,
  /** A numerically huge bound value is specified. */
  MSK_RES_ERR_INFINITE_BOUND                                   = 1400,
  /** Invalid value %d at qosubi. */
  MSK_RES_ERR_INV_QOBJ_SUBI                                    = 1401,
  /** Invalid value in qosubj. */
  MSK_RES_ERR_INV_QOBJ_SUBJ                                    = 1402,
  /** Invalid value in qoval. */
  MSK_RES_ERR_INV_QOBJ_VAL                                     = 1403,
  /** Invalid value in qcsubk. */
  MSK_RES_ERR_INV_QCON_SUBK                                    = 1404,
  /** Invalid value in qcsubi. */
  MSK_RES_ERR_INV_QCON_SUBI                                    = 1405,
  /** Invalid value in qcsubj. */
  MSK_RES_ERR_INV_QCON_SUBJ                                    = 1406,
  /** Invalid value in qcval. */
  MSK_RES_ERR_INV_QCON_VAL                                     = 1407,
  /** Invalid value in qcsubi. */
  MSK_RES_ERR_QCON_SUBI_TOO_SMALL                              = 1408,
  /** Invalid value in qcsubi. */
  MSK_RES_ERR_QCON_SUBI_TOO_LARGE                              = 1409,
  /** An element in the upper triangle of the quadratic term in the objective is specified. */
  MSK_RES_ERR_QOBJ_UPPER_TRIANGLE                              = 1415,
  /** An element in the upper triangle of the quadratic term in a constraint. */
  MSK_RES_ERR_QCON_UPPER_TRIANGLE                              = 1417,
  /** A fixed constraint/variable has been specified using the bound keys but the numerical bounds are different. */
  MSK_RES_ERR_FIXED_BOUND_VALUES                               = 1420,
  /** A too small value for the A trucation value is specified. */
  MSK_RES_ERR_TOO_SMALL_A_TRUNCATION_VALUE                     = 1421,
  /** An invalid objective sense is specified. */
  MSK_RES_ERR_INVALID_OBJECTIVE_SENSE                          = 1445,
  /** The objective sense has not been specified before the optimization. */
  MSK_RES_ERR_UNDEFINED_OBJECTIVE_SENSE                        = 1446,
  /** The solution item y is undefined. */
  MSK_RES_ERR_Y_IS_UNDEFINED                                   = 1449,
  /** An invalid floating value was used in some double data. */
  MSK_RES_ERR_NAN_IN_DOUBLE_DATA                               = 1450,
  /** An infinite floating value was used in some double data. */
  MSK_RES_ERR_INF_IN_DOUBLE_DATA                               = 1451,
  /** blc contains an invalid floating point value, i.e. a NaN or Infinity */
  MSK_RES_ERR_NAN_IN_BLC                                       = 1461,
  /** buc contains an invalid floating point value, i.e. a NaN. or Infinity */
  MSK_RES_ERR_NAN_IN_BUC                                       = 1462,
  /** An invalid fixed term in the objective is speficied. */
  MSK_RES_ERR_INVALID_CFIX                                     = 1469,
  /** c contains an invalid floating point value, i.e. a NaN or Infinity. */
  MSK_RES_ERR_NAN_IN_C                                         = 1470,
  /** blx contains an invalid floating point value, i.e. a NaN or Infinity. */
  MSK_RES_ERR_NAN_IN_BLX                                       = 1471,
  /** bux contains an invalid floating point value, i.e. a NaN or Infinity. */
  MSK_RES_ERR_NAN_IN_BUX                                       = 1472,
  /** a[i,j] contains an invalid floating point value, i.e. a NaN or an infinite value. */
  MSK_RES_ERR_INVALID_AIJ                                      = 1473,
  /** c[j] contains an invalid floating point value, i.e. a NaN or an infinite value. */
  MSK_RES_ERR_INVALID_CJ                                       = 1474,
  /** A symmetric matrix contains an invalid floating point value, i.e. a NaN or an infinite value. */
  MSK_RES_ERR_SYM_MAT_INVALID                                  = 1480,
  /** A numerically huge value is specified for an element in E. */
  MSK_RES_ERR_SYM_MAT_HUGE                                     = 1482,
  /** Invalid problem type. */
  MSK_RES_ERR_INV_PROBLEM                                      = 1500,
  /** The problem contains both conic and nonlinear constraints. */
  MSK_RES_ERR_MIXED_CONIC_AND_NL                               = 1501,
  /** The global optimizer can only be applied to problems without semidefinite variables. */
  MSK_RES_ERR_GLOBAL_INV_CONIC_PROBLEM                         = 1503,
  /** An invalid optimizer has been chosen for the problem. */
  MSK_RES_ERR_INV_OPTIMIZER                                    = 1550,
  /** No optimizer is available for the current class of integer optimization problems. */
  MSK_RES_ERR_MIO_NO_OPTIMIZER                                 = 1551,
  /** No optimizer is available for this class of optimization problems. */
  MSK_RES_ERR_NO_OPTIMIZER_VAR_TYPE                            = 1552,
  /** An error occurred during the solution finalization. */
  MSK_RES_ERR_FINAL_SOLUTION                                   = 1560,
  /** Invalid first. */
  MSK_RES_ERR_FIRST                                            = 1570,
  /** Invalid last. */
  MSK_RES_ERR_LAST                                             = 1571,
  /** Invalid slice size specified. */
  MSK_RES_ERR_SLICE_SIZE                                       = 1572,
  /** Negative surplus. */
  MSK_RES_ERR_NEGATIVE_SURPLUS                                 = 1573,
  /** Cannot append a negative number. */
  MSK_RES_ERR_NEGATIVE_APPEND                                  = 1578,
  /** An error occurred during the postsolve. */
  MSK_RES_ERR_POSTSOLVE                                        = 1580,
  /** A computation produced an overflow. */
  MSK_RES_ERR_OVERFLOW                                         = 1590,
  /** No basic solution is defined. */
  MSK_RES_ERR_NO_BASIS_SOL                                     = 1600,
  /** The factorization of the basis is invalid. */
  MSK_RES_ERR_BASIS_FACTOR                                     = 1610,
  /** The basis is singular. */
  MSK_RES_ERR_BASIS_SINGULAR                                   = 1615,
  /** An error occurred while factorizing a matrix. */
  MSK_RES_ERR_FACTOR                                           = 1650,
  /** An optimization problem cannot be relaxed. */
  MSK_RES_ERR_FEASREPAIR_CANNOT_RELAX                          = 1700,
  /** The relaxed problem could not be solved to optimality. */
  MSK_RES_ERR_FEASREPAIR_SOLVING_RELAXED                       = 1701,
  /** The upper bound is less than the lower bound for a variable or a constraint. */
  MSK_RES_ERR_FEASREPAIR_INCONSISTENT_BOUND                    = 1702,
  /** The feasibility repair does not support the specified problem type. */
  MSK_RES_ERR_REPAIR_INVALID_PROBLEM                           = 1710,
  /** Computation the optimal relaxation failed. */
  MSK_RES_ERR_REPAIR_OPTIMIZATION_FAILED                       = 1711,
  /** A name is longer than the buffer that is supposed to hold it. */
  MSK_RES_ERR_NAME_MAX_LEN                                     = 1750,
  /** The name buffer is a null pointer. */
  MSK_RES_ERR_NAME_IS_NULL                                     = 1760,
  /** Invalid compression type. */
  MSK_RES_ERR_INVALID_COMPRESSION                              = 1800,
  /** Invalid io mode. */
  MSK_RES_ERR_INVALID_IOMODE                                   = 1801,
  /** A certificate of primal infeasibility is not available. */
  MSK_RES_ERR_NO_PRIMAL_INFEAS_CER                             = 2000,
  /** A certificate of dual infeasibility is not available. */
  MSK_RES_ERR_NO_DUAL_INFEAS_CER                               = 2001,
  /** The required solution is not available. */
  MSK_RES_ERR_NO_SOLUTION_IN_CALLBACK                          = 2500,
  /** Invalid value in marki. */
  MSK_RES_ERR_INV_MARKI                                        = 2501,
  /** Invalid value in markj. */
  MSK_RES_ERR_INV_MARKJ                                        = 2502,
  /** Invalid numi. */
  MSK_RES_ERR_INV_NUMI                                         = 2503,
  /** Invalid numj. */
  MSK_RES_ERR_INV_NUMJ                                         = 2504,
  /** The Task file is incompatible with this platform. */
  MSK_RES_ERR_TASK_INCOMPATIBLE                                = 2560,
  /** The Task file is invalid. */
  MSK_RES_ERR_TASK_INVALID                                     = 2561,
  /** Failed to write the task file. */
  MSK_RES_ERR_TASK_WRITE                                       = 2562,
  /** Failed to read or write due to an I/O error. */
  MSK_RES_ERR_READ_WRITE                                       = 2563,
  /** The Task file ended prematurely. */
  MSK_RES_ERR_TASK_PREMATURE_EOF                               = 2564,
  /** Could not compute the LU factors of the matrix within the maximum number of allowed tries. */
  MSK_RES_ERR_LU_MAX_NUM_TRIES                                 = 2800,
  /** An invalid UTF8 string is encountered. */
  MSK_RES_ERR_INVALID_UTF8                                     = 2900,
  /** An invalid wchar string is encountered. */
  MSK_RES_ERR_INVALID_WCHAR                                    = 2901,
  /** No dual information is available for the integer solution. */
  MSK_RES_ERR_NO_DUAL_FOR_ITG_SOL                              = 2950,
  /** snx is not available for the basis solution. */
  MSK_RES_ERR_NO_SNX_FOR_BAS_SOL                               = 2953,
  /** An internal error occurred. */
  MSK_RES_ERR_INTERNAL                                         = 3000,
  /** An input array was too short. */
  MSK_RES_ERR_API_ARRAY_TOO_SMALL                              = 3001,
  /** Failed to connect a callback object. */
  MSK_RES_ERR_API_CB_CONNECT                                   = 3002,
  /** An internal error occurred in the API. Please report this problem. */
  MSK_RES_ERR_API_FATAL_ERROR                                  = 3005,
  /** A numerical failure occurred. */
  MSK_RES_ERR_NUMERICAL_FAILURE                                = 3040,
  /** Syntax error in sensitivity analysis file. */
  MSK_RES_ERR_SEN_FORMAT                                       = 3050,
  /** An undefined name was encountered in the sensitivity analysis file. */
  MSK_RES_ERR_SEN_UNDEF_NAME                                   = 3051,
  /** Index out of range in the sensitivity analysis file. */
  MSK_RES_ERR_SEN_INDEX_RANGE                                  = 3052,
  /** Analysis of upper bound requested for an index, where no upper bound exists. */
  MSK_RES_ERR_SEN_BOUND_INVALID_UP                             = 3053,
  /** Analysis of lower bound requested for an index, where no lower bound exists. */
  MSK_RES_ERR_SEN_BOUND_INVALID_LO                             = 3054,
  /** Invalid range given in the sensitivity file. */
  MSK_RES_ERR_SEN_INDEX_INVALID                                = 3055,
  /** Syntax error in regexp or regexp longer than 1024. */
  MSK_RES_ERR_SEN_INVALID_REGEXP                               = 3056,
  /** No optimal solution found to the original problem given for sensitivity analysis. */
  MSK_RES_ERR_SEN_SOLUTION_STATUS                              = 3057,
  /** Numerical difficulties encountered performing the sensitivity analysis. */
  MSK_RES_ERR_SEN_NUMERICAL                                    = 3058,
  /** Sensitivity analysis cannot be performed for the specified problem. */
  MSK_RES_ERR_SEN_UNHANDLED_PROBLEM_TYPE                       = 3080,
  /** A step-size in an optimizer was unexpectedly unbounded. */
  MSK_RES_ERR_UNB_STEP_SIZE                                    = 3100,
  /** Some tasks related to this function call were identical. Unique tasks were expected. */
  MSK_RES_ERR_IDENTICAL_TASKS                                  = 3101,
  /** The code list data was invalid. */
  MSK_RES_ERR_AD_INVALID_CODELIST                              = 3102,
  /** An internal unit test function failed. */
  MSK_RES_ERR_INTERNAL_TEST_FAILED                             = 3500,
  /** A 64 bit integer could not be cast to a 32 bit integer. */
  MSK_RES_ERR_INT64_TO_INT32_CAST                              = 3800,
  /** The requested value is not defined for this solution type. */
  MSK_RES_ERR_INFEAS_UNDEFINED                                 = 3910,
  /** There is no barx available for the solution specified. */
  MSK_RES_ERR_NO_BARX_FOR_SOLUTION                             = 3915,
  /** There is no bars available for the solution specified. */
  MSK_RES_ERR_NO_BARS_FOR_SOLUTION                             = 3916,
  /** The dimension of a symmetric matrix variable has to be greater than 0. */
  MSK_RES_ERR_BAR_VAR_DIM                                      = 3920,
  /** A row index specified for sparse symmetric matrix is invalid. */
  MSK_RES_ERR_SYM_MAT_INVALID_ROW_INDEX                        = 3940,
  /** A column index specified for sparse symmetric matrix is invalid. */
  MSK_RES_ERR_SYM_MAT_INVALID_COL_INDEX                        = 3941,
  /** Only the lower triangular part of sparse symmetric matrix should be specified. */
  MSK_RES_ERR_SYM_MAT_NOT_LOWER_TRINGULAR                      = 3942,
  /** The numerical value specified in a sparse symmetric matrix is not a floating point value. */
  MSK_RES_ERR_SYM_MAT_INVALID_VALUE                            = 3943,
  /** A value in a symmetric matric as been specified more than once. */
  MSK_RES_ERR_SYM_MAT_DUPLICATE                                = 3944,
  /** A sparse symmetric matrix of invalid dimension is specified. */
  MSK_RES_ERR_INVALID_SYM_MAT_DIM                              = 3950,
  /** An internal fatal error occurred in an interface function. */
  MSK_RES_ERR_API_INTERNAL                                     = 3999,
  /** The file format does not support a problem with symmetric matrix variables. */
  MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_SYM_MAT                  = 4000,
  /** The file format does not support a problem with nonzero fixed term in c. */
  MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_CFIX                     = 4001,
  /** The file format does not support a problem with ranged constraints. */
  MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_RANGED_CONSTRAINTS       = 4002,
  /** The file format does not support a problem with free constraints. */
  MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_FREE_CONSTRAINTS         = 4003,
  /** The file format does not support a problem with the simple cones (deprecated). */
  MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_CONES                    = 4005,
  /** The file format does not support a problem with quadratic terms. */
  MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_QUADRATIC_TERMS          = 4006,
  /** The file format does not support a problem with nonlinear terms. */
  MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_NONLINEAR                = 4010,
  /** The file format does not support a problem with disjunctive constraints. */
  MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_DISJUNCTIVE_CONSTRAINTS  = 4011,
  /** The file format does not support a problem with affine conic constraints. */
  MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_AFFINE_CONIC_CONSTRAINTS = 4012,
  /** Two constraint names are identical. */
  MSK_RES_ERR_DUPLICATE_CONSTRAINT_NAMES                       = 4500,
  /** Two variable names are identical. */
  MSK_RES_ERR_DUPLICATE_VARIABLE_NAMES                         = 4501,
  /** Two barvariable names are identical. */
  MSK_RES_ERR_DUPLICATE_BARVARIABLE_NAMES                      = 4502,
  /** Two cone names are identical. */
  MSK_RES_ERR_DUPLICATE_CONE_NAMES                             = 4503,
  /** Two domain names are identical. */
  MSK_RES_ERR_DUPLICATE_DOMAIN_NAMES                           = 4504,
  /** Two disjunctive constraint names are identical. */
  MSK_RES_ERR_DUPLICATE_DJC_NAMES                              = 4505,
  /** An array does not contain unique elements. */
  MSK_RES_ERR_NON_UNIQUE_ARRAY                                 = 5000,
  /** The value of a function argument is too small. */
  MSK_RES_ERR_ARGUMENT_IS_TOO_SMALL                            = 5004,
  /** The value of a function argument is too large. */
  MSK_RES_ERR_ARGUMENT_IS_TOO_LARGE                            = 5005,
  /** A fatal error occurred in the mixed integer optimizer.  Please contact MOSEK support. */
  MSK_RES_ERR_MIO_INTERNAL                                     = 5010,
  /** An invalid problem type. */
  MSK_RES_ERR_INVALID_PROBLEM_TYPE                             = 6000,
  /** Unhandled solution status. */
  MSK_RES_ERR_UNHANDLED_SOLUTION_STATUS                        = 6010,
  /** An element in the upper triangle of a lower triangular matrix is specified. */
  MSK_RES_ERR_UPPER_TRIANGLE                                   = 6020,
  /** A matrix is singular. */
  MSK_RES_ERR_LAU_SINGULAR_MATRIX                              = 7000,
  /** A matrix is not positive definite. */
  MSK_RES_ERR_LAU_NOT_POSITIVE_DEFINITE                        = 7001,
  /** An invalid lower triangular matrix. */
  MSK_RES_ERR_LAU_INVALID_LOWER_TRIANGULAR_MATRIX              = 7002,
  /** An unknown error. */
  MSK_RES_ERR_LAU_UNKNOWN                                      = 7005,
  /** Invalid argument m. */
  MSK_RES_ERR_LAU_ARG_M                                        = 7010,
  /** Invalid argument n. */
  MSK_RES_ERR_LAU_ARG_N                                        = 7011,
  /** Invalid argument k. */
  MSK_RES_ERR_LAU_ARG_K                                        = 7012,
  /** Invalid argument transa. */
  MSK_RES_ERR_LAU_ARG_TRANSA                                   = 7015,
  /** Invalid argument transb. */
  MSK_RES_ERR_LAU_ARG_TRANSB                                   = 7016,
  /** Invalid argument uplo. */
  MSK_RES_ERR_LAU_ARG_UPLO                                     = 7017,
  /** Invalid argument trans. */
  MSK_RES_ERR_LAU_ARG_TRANS                                    = 7018,
  /** An invalid sparse symmetric matrix is specfified. */
  MSK_RES_ERR_LAU_INVALID_SPARSE_SYMMETRIC_MATRIX              = 7019,
  /** An error occurred while parsing an CBF file. */
  MSK_RES_ERR_CBF_PARSE                                        = 7100,
  /** An invalid objective sense is specified. */
  MSK_RES_ERR_CBF_OBJ_SENSE                                    = 7101,
  /** An invalid objective sense is specified. */
  MSK_RES_ERR_CBF_NO_VARIABLES                                 = 7102,
  /** Too many constraints specified. */
  MSK_RES_ERR_CBF_TOO_MANY_CONSTRAINTS                         = 7103,
  /** Too many variables specified. */
  MSK_RES_ERR_CBF_TOO_MANY_VARIABLES                           = 7104,
  /** No version specified. */
  MSK_RES_ERR_CBF_NO_VERSION_SPECIFIED                         = 7105,
  /** Invalid syntax. */
  MSK_RES_ERR_CBF_SYNTAX                                       = 7106,
  /** Duplicate OBJ keyword. */
  MSK_RES_ERR_CBF_DUPLICATE_OBJ                                = 7107,
  /** Duplicate CON keyword. */
  MSK_RES_ERR_CBF_DUPLICATE_CON                                = 7108,
  /** Duplicate VAR keyword. */
  MSK_RES_ERR_CBF_DUPLICATE_VAR                                = 7110,
  /** Duplicate INT keyword. */
  MSK_RES_ERR_CBF_DUPLICATE_INT                                = 7111,
  /** Invalid variable type. */
  MSK_RES_ERR_CBF_INVALID_VAR_TYPE                             = 7112,
  /** Invalid constraint type. */
  MSK_RES_ERR_CBF_INVALID_CON_TYPE                             = 7113,
  /** Invalid domain dimension. */
  MSK_RES_ERR_CBF_INVALID_DOMAIN_DIMENSION                     = 7114,
  /** Duplicate index in OBJCOORD. */
  MSK_RES_ERR_CBF_DUPLICATE_OBJACOORD                          = 7115,
  /** Duplicate index in BCOORD. */
  MSK_RES_ERR_CBF_DUPLICATE_BCOORD                             = 7116,
  /** Duplicate index in ACOORD. */
  MSK_RES_ERR_CBF_DUPLICATE_ACOORD                             = 7117,
  /** Too few variables defined. */
  MSK_RES_ERR_CBF_TOO_FEW_VARIABLES                            = 7118,
  /** Too few constraints defined. */
  MSK_RES_ERR_CBF_TOO_FEW_CONSTRAINTS                          = 7119,
  /** Too ints specified. */
  MSK_RES_ERR_CBF_TOO_FEW_INTS                                 = 7120,
  /** Too ints specified. */
  MSK_RES_ERR_CBF_TOO_MANY_INTS                                = 7121,
  /** Invalid INT index. */
  MSK_RES_ERR_CBF_INVALID_INT_INDEX                            = 7122,
  /** Unsupported feature is present. */
  MSK_RES_ERR_CBF_UNSUPPORTED                                  = 7123,
  /** Duplicate PSDVAR keyword. */
  MSK_RES_ERR_CBF_DUPLICATE_PSDVAR                             = 7124,
  /** Invalid PSDVAR dimension. */
  MSK_RES_ERR_CBF_INVALID_PSDVAR_DIMENSION                     = 7125,
  /** Too few variables defined. */
  MSK_RES_ERR_CBF_TOO_FEW_PSDVAR                               = 7126,
  /** Invalid dimension of a exponential cone. */
  MSK_RES_ERR_CBF_INVALID_EXP_DIMENSION                        = 7127,
  /** Multiple POWCONES specified. */
  MSK_RES_ERR_CBF_DUPLICATE_POW_CONES                          = 7130,
  /** Multiple POW*CONES specified. */
  MSK_RES_ERR_CBF_DUPLICATE_POW_STAR_CONES                     = 7131,
  /** Invalid power specified. */
  MSK_RES_ERR_CBF_INVALID_POWER                                = 7132,
  /** Power cone is too long. */
  MSK_RES_ERR_CBF_POWER_CONE_IS_TOO_LONG                       = 7133,
  /** Invalid power cone index. */
  MSK_RES_ERR_CBF_INVALID_POWER_CONE_INDEX                     = 7134,
  /** Invalid power star cone index. */
  MSK_RES_ERR_CBF_INVALID_POWER_STAR_CONE_INDEX                = 7135,
  /** An unhandled power cone type. */
  MSK_RES_ERR_CBF_UNHANDLED_POWER_CONE_TYPE                    = 7136,
  /** An unhandled power star cone type. */
  MSK_RES_ERR_CBF_UNHANDLED_POWER_STAR_CONE_TYPE               = 7137,
  /** The power cone does not match with it definition. */
  MSK_RES_ERR_CBF_POWER_CONE_MISMATCH                          = 7138,
  /** The power star cone does not match with it definition. */
  MSK_RES_ERR_CBF_POWER_STAR_CONE_MISMATCH                     = 7139,
  /** Invalid number of cones. */
  MSK_RES_ERR_CBF_INVALID_NUMBER_OF_CONES                      = 7140,
  /** Invalid number of cones. */
  MSK_RES_ERR_CBF_INVALID_DIMENSION_OF_CONES                   = 7141,
  /** Invalid number of OBJACOORD. */
  MSK_RES_ERR_CBF_INVALID_NUM_OBJACOORD                        = 7150,
  /** Invalid number of OBJFCOORD. */
  MSK_RES_ERR_CBF_INVALID_NUM_OBJFCOORD                        = 7151,
  /** Invalid number of ACOORD. */
  MSK_RES_ERR_CBF_INVALID_NUM_ACOORD                           = 7152,
  /** Invalid number of BCOORD. */
  MSK_RES_ERR_CBF_INVALID_NUM_BCOORD                           = 7153,
  /** Invalid number of FCOORD. */
  MSK_RES_ERR_CBF_INVALID_NUM_FCOORD                           = 7155,
  /** Invalid number of HCOORD. */
  MSK_RES_ERR_CBF_INVALID_NUM_HCOORD                           = 7156,
  /** Invalid number of DCOORD. */
  MSK_RES_ERR_CBF_INVALID_NUM_DCOORD                           = 7157,
  /** Expected a key word. */
  MSK_RES_ERR_CBF_EXPECTED_A_KEYWORD                           = 7158,
  /** Invalid number of PSDCON. */
  MSK_RES_ERR_CBF_INVALID_NUM_PSDCON                           = 7200,
  /** Duplicate CON keyword. */
  MSK_RES_ERR_CBF_DUPLICATE_PSDCON                             = 7201,
  /** Invalid PSDCON dimension. */
  MSK_RES_ERR_CBF_INVALID_DIMENSION_OF_PSDCON                  = 7202,
  /** Invalid PSDCON index. */
  MSK_RES_ERR_CBF_INVALID_PSDCON_INDEX                         = 7203,
  /** Invalid PSDCON index. */
  MSK_RES_ERR_CBF_INVALID_PSDCON_VARIABLE_INDEX                = 7204,
  /** Invalid PSDCON index. */
  MSK_RES_ERR_CBF_INVALID_PSDCON_BLOCK_INDEX                   = 7205,
  /** Invalid index. */
  MSK_RES_ERR_CBF_INVALID_INDEX                                = 7206,
  /** The CHANGE section is not supported. */
  MSK_RES_ERR_CBF_UNSUPPORTED_CHANGE                           = 7210,
  /** An invalid root optimizer was selected for the problem type. */
  MSK_RES_ERR_MIO_INVALID_ROOT_OPTIMIZER                       = 7700,
  /** An invalid node optimizer was selected for the problem type. */
  MSK_RES_ERR_MIO_INVALID_NODE_OPTIMIZER                       = 7701,
  /** An invalid cone type occurs when writing a CPLEX formatted MPS file. */
  MSK_RES_ERR_MPS_WRITE_CPLEX_INVALID_CONE_TYPE                = 7750,
  /** The matrix defining the quadratric part of constraint is not positive semidefinite. */
  MSK_RES_ERR_TOCONIC_CONSTR_Q_NOT_PSD                         = 7800,
  /** The quadratic constraint is an equality, thus not convex. */
  MSK_RES_ERR_TOCONIC_CONSTRAINT_FX                            = 7801,
  /** The quadratic constraint has finite lower and upper bound, and therefore it is not convex. */
  MSK_RES_ERR_TOCONIC_CONSTRAINT_RA                            = 7802,
  /** The constraint is not conic representable. */
  MSK_RES_ERR_TOCONIC_CONSTR_NOT_CONIC                         = 7803,
  /** The matrix defining the quadratric part of the objective function is not positive semidefinite. */
  MSK_RES_ERR_TOCONIC_OBJECTIVE_NOT_PSD                        = 7804,
  /** The simple dualizer is not available for this problem class. */
  MSK_RES_ERR_GETDUAL_NOT_AVAILABLE                            = 7820,
  /** Constructing a problem with fixed integer variables not possible. */
  MSK_RES_ERR_FIXING_NOT_POSSIBLE                              = 7821,
  /** Failed to connect to remote solver server. */
  MSK_RES_ERR_SERVER_CONNECT                                   = 8000,
  /** Unexpected message or data from solver server. */
  MSK_RES_ERR_SERVER_PROTOCOL                                  = 8001,
  /** Server returned non-ok status code */
  MSK_RES_ERR_SERVER_STATUS                                    = 8002,
  /** Invalid job ID */
  MSK_RES_ERR_SERVER_TOKEN                                     = 8003,
  /** Invalid address */
  MSK_RES_ERR_SERVER_ADDRESS                                   = 8004,
  /** Invalid TLS certificate format or path */
  MSK_RES_ERR_SERVER_CERTIFICATE                               = 8005,
  /** Failed to create TLS client */
  MSK_RES_ERR_SERVER_TLS_CLIENT                                = 8006,
  /** TLS initialization failed */
  MSK_RES_ERR_TLS_FAIL                                         = 8007,
  /** TLS configuration failed */
  MSK_RES_ERR_TLS_CONFIGURATION                                = 8008,
  /** TLS handshake failed */
  MSK_RES_ERR_SERVER_TLS_HANDSHAKE                             = 8009,
  /** Invalid access token */
  MSK_RES_ERR_SERVER_ACCESS_TOKEN                              = 8020,
  /** The problem is too large. */
  MSK_RES_ERR_SERVER_PROBLEM_SIZE                              = 8021,
  /** The hard timeout limit was reached on solver server */
  MSK_RES_ERR_SERVER_HARD_TIMEOUT                              = 8022,
  /** Solution file version from server not supported. */
  MSK_RES_ERR_SERVER_VERSION_MISMATCH                          = 8023,
  /** An element in a sparse matrix is specified twice. */
  MSK_RES_ERR_DUPLICATE_INDEX_IN_A_SPARSE_MATRIX               = 20050,
  /** An index is specified twice in an affine expression list. */
  MSK_RES_ERR_DUPLICATE_INDEX_IN_AFEIDX_LIST                   = 20060,
  /** An element in the F matrix is specified twice. */
  MSK_RES_ERR_DUPLICATE_FIJ                                    = 20100,
  /** f[i,j] contains an invalid floating point value, i.e. a NaN or an infinite value. */
  MSK_RES_ERR_INVALID_FIJ                                      = 20101,
  /** A numerically huge value is specified for an element in F. */
  MSK_RES_ERR_HUGE_FIJ                                         = 20102,
  /** g contains an invalid floating point value, i.e. a NaN or an infinite value. */
  MSK_RES_ERR_INVALID_G                                        = 20103,
  /** b contains an invalid floating point value, i.e. a NaN or an infinite value. */
  MSK_RES_ERR_INVALID_B                                        = 20150,
  /** A domain index is invalid. */
  MSK_RES_ERR_DOMAIN_INVALID_INDEX                             = 20400,
  /** A domain dimension is invalid. */
  MSK_RES_ERR_DOMAIN_DIMENSION                                 = 20401,
  /** A PSD domain dimension is invalid. */
  MSK_RES_ERR_DOMAIN_DIMENSION_PSD                             = 20402,
  /** The function is only applicable to primal and dual power cone domains. */
  MSK_RES_ERR_NOT_POWER_DOMAIN                                 = 20403,
  /** Alpha contains an invalid floating point value, i.e. a NaN or an infinite value. */
  MSK_RES_ERR_DOMAIN_POWER_INVALID_ALPHA                       = 20404,
  /** Alpha contains a negative value or zero. */
  MSK_RES_ERR_DOMAIN_POWER_NEGATIVE_ALPHA                      = 20405,
  /** The value of nleft is too small or too large. */
  MSK_RES_ERR_DOMAIN_POWER_NLEFT                               = 20406,
  /** An affine expression index is invalid. */
  MSK_RES_ERR_AFE_INVALID_INDEX                                = 20500,
  /** A affine conic constraint index is invalid. */
  MSK_RES_ERR_ACC_INVALID_INDEX                                = 20600,
  /** The index of an element in an affine conic constraint is invalid. */
  MSK_RES_ERR_ACC_INVALID_ENTRY_INDEX                          = 20601,
  /** There is a mismatch between between the number of affine expressions and total dimension of the domain(s). */
  MSK_RES_ERR_ACC_AFE_DOMAIN_MISMATCH                          = 20602,
  /** A disjunctive constraint index is invalid. */
  MSK_RES_ERR_DJC_INVALID_INDEX                                = 20700,
  /** An unsupported domain type has been used in a disjunctive constraint. */
  MSK_RES_ERR_DJC_UNSUPPORTED_DOMAIN_TYPE                      = 20701,
  /** There is a mismatch between the number of affine expressions and total dimension of the domain(s). */
  MSK_RES_ERR_DJC_AFE_DOMAIN_MISMATCH                          = 20702,
  /** A termize is invalid. */
  MSK_RES_ERR_DJC_INVALID_TERM_SIZE                            = 20703,
  /** There is a mismatch between the number of domains and the term sizes. */
  MSK_RES_ERR_DJC_DOMAIN_TERMSIZE_MISMATCH                     = 20704,
  /** There total number of terms in all domains does not match. */
  MSK_RES_ERR_DJC_TOTAL_NUM_TERMS_MISMATCH                     = 20705,
  /** The required solution is not defined. */
  MSK_RES_ERR_UNDEF_SOLUTION                                   = 22000,
  /** No doty is available. */
  MSK_RES_ERR_NO_DOTY                                          = 22010,
  /** The hardware does not fused multi add hardware instruction which is required by the simplex precision boosting feature. */
  MSK_RES_ERR_SIM_PRECISION_BOOSTING_IS_UNVAILABLE             = 30000,
  /** A floating point precision is not available. */
  MSK_RES_ERR_PRECISION_UNVAILABLE                             = 30001,
  /** The optimizer terminated at the maximum number of iterations. */
  MSK_RES_TRM_MAX_ITERATIONS                                   = 100000,
  /** The optimizer terminated at the maximum amount of time. */
  MSK_RES_TRM_MAX_TIME                                         = 100001,
  /** The optimizer terminated with an objective value outside the objective range. */
  MSK_RES_TRM_OBJECTIVE_RANGE                                  = 100002,
  /** The optimizer is terminated due to slow progress. */
  MSK_RES_TRM_STALL                                            = 100006,
  /** The user-defined progress callback function terminated the optimization. */
  MSK_RES_TRM_USER_CALLBACK                                    = 100007,
  /** The mixed-integer optimizer terminated as the maximum number of relaxations was reached. */
  MSK_RES_TRM_MIO_NUM_RELAXS                                   = 100008,
  /** The mixed-integer optimizer terminated as the maximum number of branches was reached. */
  MSK_RES_TRM_MIO_NUM_BRANCHES                                 = 100009,
  /** The mixed-integer optimizer terminated as the maximum number of feasible solutions was reached. */
  MSK_RES_TRM_NUM_MAX_NUM_INT_SOLUTIONS                        = 100015,
  /** The optimizer terminated as the maximum number of set-backs was reached. */
  MSK_RES_TRM_MAX_NUM_SETBACKS                                 = 100020,
  /** The optimizer terminated due to a numerical problem. */
  MSK_RES_TRM_NUMERICAL_PROBLEM                                = 100025,
  /** The optimizer is terminated due to an extremely ill-conditioned basis matrix is encountered. */
  MSK_RES_TRM_ILL_CONDITIONED_BASIS                            = 100026,
  /** Lost a race. */
  MSK_RES_TRM_LOST_RACE                                        = 100027,
  /** The optimizer terminated at the maximum amount of ticks. */
  MSK_RES_TRM_MAX_TICKS                                        = 100028,
  /** The optimizer terminated due to some internal reason. */
  MSK_RES_TRM_INTERNAL                                         = 100030,
  /** The optimizer terminated for internal reasons. */
  MSK_RES_TRM_INTERNAL_STOP                                    = 100031,
  /** remote server terminated mosek on time limit criteria. */
  MSK_RES_TRM_SERVER_MAX_TIME                                  = 100032,
  /** remote server terminated mosek on memory limit criteria. */
  MSK_RES_TRM_SERVER_MAX_MEMORY                                = 100033
}; /* MSKrescode_enum */
#ifdef MSK_NO_ENUMS
typedef int MSKrescodee;
#else
typedef enum MSKrescode_enum MSKrescodee;
#endif

enum MSKrescodetype_enum {
  /** The response code is OK. */
  MSK_RESPONSE_OK  = 0,
  /** The response code is a warning. */
  MSK_RESPONSE_WRN = 1,
  /** The response code is an optimizer termination status. */
  MSK_RESPONSE_TRM = 2,
  /** The response code is an error. */
  MSK_RESPONSE_ERR = 3,
  /** The response code does not belong to any class. */
  MSK_RESPONSE_UNK = 4
}; /* MSKrescodetype_enum */
#define MSK_RESPONSE_BEGIN MSK_RESPONSE_OK
#define MSK_RESPONSE_END   (1+MSK_RESPONSE_UNK)
#ifdef MSK_NO_ENUMS
typedef int MSKrescodetypee;
#else
typedef enum MSKrescodetype_enum MSKrescodetypee;
#endif

enum MSKscalingtype_enum {
  /** The optimizer chooses the scaling heuristic. */
  MSK_SCALING_FREE = 0,
  /** No scaling is performed. */
  MSK_SCALING_NONE = 1
}; /* MSKscalingtype_enum */
#define MSK_SCALING_BEGIN MSK_SCALING_FREE
#define MSK_SCALING_END   (1+MSK_SCALING_NONE)
#ifdef MSK_NO_ENUMS
typedef int MSKscalingtypee;
#else
typedef int MSKscalingtypee;
#endif

enum MSKscalingmethod_enum {
  /** Scales only with power of 2 leaving the mantissa untouched. */
  MSK_SCALING_METHOD_POW2 = 0,
  /** The optimizer chooses the scaling heuristic. */
  MSK_SCALING_METHOD_FREE = 1
}; /* MSKscalingmethod_enum */
#define MSK_SCALING_METHOD_BEGIN MSK_SCALING_METHOD_POW2
#define MSK_SCALING_METHOD_END   (1+MSK_SCALING_METHOD_FREE)
#ifdef MSK_NO_ENUMS
typedef int MSKscalingmethode;
#else
typedef int MSKscalingmethode;
#endif

enum MSKsensitivitytype_enum {
  /** Basis sensitivity analysis is performed. */
  MSK_SENSITIVITY_TYPE_BASIS = 0
}; /* MSKsensitivitytype_enum */
#define MSK_SENSITIVITY_TYPE_BEGIN MSK_SENSITIVITY_TYPE_BASIS
#define MSK_SENSITIVITY_TYPE_END   (1+MSK_SENSITIVITY_TYPE_BASIS)
#ifdef MSK_NO_ENUMS
typedef int MSKsensitivitytypee;
#else
typedef int MSKsensitivitytypee;
#endif

enum MSKsimseltype_enum {
  /** The optimizer chooses the pricing strategy. */
  MSK_SIM_SELECTION_FREE    = 0,
  /** The optimizer uses full pricing. */
  MSK_SIM_SELECTION_FULL    = 1,
  /** The optimizer uses approximate steepest-edge pricing. */
  MSK_SIM_SELECTION_ASE     = 2,
  /** The optimizer uses devex steepest-edge pricing. */
  MSK_SIM_SELECTION_DEVEX   = 3,
  /** The optimizer uses steepest-edge selection. */
  MSK_SIM_SELECTION_SE      = 4,
  /** The optimizer uses a partial selection approach. */
  MSK_SIM_SELECTION_PARTIAL = 5
}; /* MSKsimseltype_enum */
#define MSK_SIM_SELECTION_BEGIN MSK_SIM_SELECTION_FREE
#define MSK_SIM_SELECTION_END   (1+MSK_SIM_SELECTION_PARTIAL)
#ifdef MSK_NO_ENUMS
typedef int MSKsimseltypee;
#else
typedef int MSKsimseltypee;
#endif

enum MSKsolitem_enum {
  /** Solution for the constraints. */
  MSK_SOL_ITEM_XC  = 0,
  /** Variable solution. */
  MSK_SOL_ITEM_XX  = 1,
  /** Lagrange multipliers for equations. */
  MSK_SOL_ITEM_Y   = 2,
  /** Lagrange multipliers for lower bounds on the constraints. */
  MSK_SOL_ITEM_SLC = 3,
  /** Lagrange multipliers for upper bounds on the constraints. */
  MSK_SOL_ITEM_SUC = 4,
  /** Lagrange multipliers for lower bounds on the variables. */
  MSK_SOL_ITEM_SLX = 5,
  /** Lagrange multipliers for upper bounds on the variables. */
  MSK_SOL_ITEM_SUX = 6,
  /** Lagrange multipliers corresponding to the conic constraints on the variables. */
  MSK_SOL_ITEM_SNX = 7
}; /* MSKsolitem_enum */
#define MSK_SOL_ITEM_BEGIN MSK_SOL_ITEM_XC
#define MSK_SOL_ITEM_END   (1+MSK_SOL_ITEM_SNX)
#ifdef MSK_NO_ENUMS
typedef int MSKsoliteme;
#else
typedef enum MSKsolitem_enum MSKsoliteme;
#endif

enum MSKsolsta_enum {
  /** Status of the solution is unknown. */
  MSK_SOL_STA_UNKNOWN            = 0,
  /** The solution is optimal. */
  MSK_SOL_STA_OPTIMAL            = 1,
  /** The solution is primal feasible. */
  MSK_SOL_STA_PRIM_FEAS          = 2,
  /** The solution is dual feasible. */
  MSK_SOL_STA_DUAL_FEAS          = 3,
  /** The solution is both primal and dual feasible. */
  MSK_SOL_STA_PRIM_AND_DUAL_FEAS = 4,
  /** The solution is a certificate of primal infeasibility. */
  MSK_SOL_STA_PRIM_INFEAS_CER    = 5,
  /** The solution is a certificate of dual infeasibility. */
  MSK_SOL_STA_DUAL_INFEAS_CER    = 6,
  /** The solution is a certificate that the primal problem is illposed. */
  MSK_SOL_STA_PRIM_ILLPOSED_CER  = 7,
  /** The solution is a certificate that the dual problem is illposed. */
  MSK_SOL_STA_DUAL_ILLPOSED_CER  = 8,
  /** The primal solution is integer optimal. */
  MSK_SOL_STA_INTEGER_OPTIMAL    = 9
}; /* MSKsolsta_enum */
#define MSK_SOL_STA_BEGIN MSK_SOL_STA_UNKNOWN
#define MSK_SOL_STA_END   (1+MSK_SOL_STA_INTEGER_OPTIMAL)
#ifdef MSK_NO_ENUMS
typedef int MSKsolstae;
#else
typedef enum MSKsolsta_enum MSKsolstae;
#endif

enum MSKsoltype_enum {
  /** The interior solution. */
  MSK_SOL_ITR = 0,
  /** The basic solution. */
  MSK_SOL_BAS = 1,
  /** The integer solution. */
  MSK_SOL_ITG = 2
}; /* MSKsoltype_enum */
#define MSK_SOL_BEGIN MSK_SOL_ITR
#define MSK_SOL_END   (1+MSK_SOL_ITG)
#ifdef MSK_NO_ENUMS
typedef int MSKsoltypee;
#else
typedef enum MSKsoltype_enum MSKsoltypee;
#endif

enum MSKsolveform_enum {
  /** The optimizer is free to solve either the primal or the dual problem. */
  MSK_SOLVE_FREE   = 0,
  /** The optimizer should solve the primal problem. */
  MSK_SOLVE_PRIMAL = 1,
  /** The optimizer should solve the dual problem. */
  MSK_SOLVE_DUAL   = 2
}; /* MSKsolveform_enum */
#define MSK_SOLVE_BEGIN MSK_SOLVE_FREE
#define MSK_SOLVE_END   (1+MSK_SOLVE_DUAL)
#ifdef MSK_NO_ENUMS
typedef int MSKsolveforme;
#else
typedef int MSKsolveforme;
#endif

enum MSKsparam_enum {
  /** Name of the bas solution file. */
  MSK_SPAR_BAS_SOL_FILE_NAME         = 0,
  /** Data are read and written to this file. */
  MSK_SPAR_DATA_FILE_NAME            = 1,
  /** MOSEK debug file. */
  MSK_SPAR_DEBUG_FILE_NAME           = 2,
  /** Name of the int solution file. */
  MSK_SPAR_INT_SOL_FILE_NAME         = 3,
  /** Name of the itr solution file. */
  MSK_SPAR_ITR_SOL_FILE_NAME         = 4,
  /** For internal debugging purposes. */
  MSK_SPAR_MIO_DEBUG_STRING          = 5,
  /** Parameter file comment character. */
  MSK_SPAR_PARAM_COMMENT_SIGN        = 6,
  /** Modifications to the parameter database is read from this file. */
  MSK_SPAR_PARAM_READ_FILE_NAME      = 7,
  /** The parameter database is written to this file. */
  MSK_SPAR_PARAM_WRITE_FILE_NAME     = 8,
  /** Name of the BOUNDS vector used. An empty name means that the first BOUNDS vector is used. */
  MSK_SPAR_READ_MPS_BOU_NAME         = 9,
  /** Objective name in the MPS file. */
  MSK_SPAR_READ_MPS_OBJ_NAME         = 10,
  /** Name of the RANGE vector  used. An empty name means that the first RANGE vector is used. */
  MSK_SPAR_READ_MPS_RAN_NAME         = 11,
  /** Name of the RHS used. An empty name means that the first RHS vector is used. */
  MSK_SPAR_READ_MPS_RHS_NAME         = 12,
  /** URL of the remote optimization server. */
  MSK_SPAR_REMOTE_OPTSERVER_HOST     = 13,
  /** Known server certificates in PEM format */
  MSK_SPAR_REMOTE_TLS_CERT           = 14,
  /** Path to known server certificates in PEM format */
  MSK_SPAR_REMOTE_TLS_CERT_PATH      = 15,
  /** Sensitivity report file name. */
  MSK_SPAR_SENSITIVITY_FILE_NAME     = 16,
  /** Name of the sensitivity report output file. */
  MSK_SPAR_SENSITIVITY_RES_FILE_NAME = 17,
  /** Solution file filter. */
  MSK_SPAR_SOL_FILTER_XC_LOW         = 18,
  /** Solution file filter. */
  MSK_SPAR_SOL_FILTER_XC_UPR         = 19,
  /** Solution file filter. */
  MSK_SPAR_SOL_FILTER_XX_LOW         = 20,
  /** Solution file filter. */
  MSK_SPAR_SOL_FILTER_XX_UPR         = 21,
  /** Key used when writing the summary file. */
  MSK_SPAR_STAT_KEY                  = 22,
  /** Name used when writing the statistics file. */
  MSK_SPAR_STAT_NAME                 = 23
}; /* MSKsparam_enum */
#define MSK_SPAR_BEGIN MSK_SPAR_BAS_SOL_FILE_NAME
#define MSK_SPAR_END   (1+MSK_SPAR_STAT_NAME)
#ifdef MSK_NO_ENUMS
typedef int MSKsparame;
#else
typedef enum MSKsparam_enum MSKsparame;
#endif

#define MSK_SPAR_BAS_SOL_FILE_NAME_    "MSK_SPAR_BAS_SOL_FILE_NAME"
#define MSK_SPAR_DATA_FILE_NAME_       "MSK_SPAR_DATA_FILE_NAME"
#define MSK_SPAR_DEBUG_FILE_NAME_      "MSK_SPAR_DEBUG_FILE_NAME"
#define MSK_SPAR_INT_SOL_FILE_NAME_    "MSK_SPAR_INT_SOL_FILE_NAME"
#define MSK_SPAR_ITR_SOL_FILE_NAME_    "MSK_SPAR_ITR_SOL_FILE_NAME"
#define MSK_SPAR_MIO_DEBUG_STRING_     "MSK_SPAR_MIO_DEBUG_STRING"
#define MSK_SPAR_PARAM_COMMENT_SIGN_   "MSK_SPAR_PARAM_COMMENT_SIGN"
#define MSK_SPAR_PARAM_READ_FILE_NAME_ "MSK_SPAR_PARAM_READ_FILE_NAME"
#define MSK_SPAR_PARAM_WRITE_FILE_NAME_ "MSK_SPAR_PARAM_WRITE_FILE_NAME"
#define MSK_SPAR_READ_MPS_BOU_NAME_    "MSK_SPAR_READ_MPS_BOU_NAME"
#define MSK_SPAR_READ_MPS_OBJ_NAME_    "MSK_SPAR_READ_MPS_OBJ_NAME"
#define MSK_SPAR_READ_MPS_RAN_NAME_    "MSK_SPAR_READ_MPS_RAN_NAME"
#define MSK_SPAR_READ_MPS_RHS_NAME_    "MSK_SPAR_READ_MPS_RHS_NAME"
#define MSK_SPAR_REMOTE_OPTSERVER_HOST_ "MSK_SPAR_REMOTE_OPTSERVER_HOST"
#define MSK_SPAR_REMOTE_TLS_CERT_      "MSK_SPAR_REMOTE_TLS_CERT"
#define MSK_SPAR_REMOTE_TLS_CERT_PATH_ "MSK_SPAR_REMOTE_TLS_CERT_PATH"
#define MSK_SPAR_SENSITIVITY_FILE_NAME_ "MSK_SPAR_SENSITIVITY_FILE_NAME"
#define MSK_SPAR_SENSITIVITY_RES_FILE_NAME_ "MSK_SPAR_SENSITIVITY_RES_FILE_NAME"
#define MSK_SPAR_SOL_FILTER_XC_LOW_    "MSK_SPAR_SOL_FILTER_XC_LOW"
#define MSK_SPAR_SOL_FILTER_XC_UPR_    "MSK_SPAR_SOL_FILTER_XC_UPR"
#define MSK_SPAR_SOL_FILTER_XX_LOW_    "MSK_SPAR_SOL_FILTER_XX_LOW"
#define MSK_SPAR_SOL_FILTER_XX_UPR_    "MSK_SPAR_SOL_FILTER_XX_UPR"
#define MSK_SPAR_STAT_KEY_             "MSK_SPAR_STAT_KEY"
#define MSK_SPAR_STAT_NAME_            "MSK_SPAR_STAT_NAME"

enum MSKstakey_enum {
  /** The status for the constraint or variable is unknown. */
  MSK_SK_UNK    = 0,
  /** The constraint or variable is in the basis. */
  MSK_SK_BAS    = 1,
  /** The constraint or variable is super basic. */
  MSK_SK_SUPBAS = 2,
  /** The constraint or variable is at its lower bound. */
  MSK_SK_LOW    = 3,
  /** The constraint or variable is at its upper bound. */
  MSK_SK_UPR    = 4,
  /** The constraint or variable is fixed. */
  MSK_SK_FIX    = 5,
  /** The constraint or variable is infeasible in the bounds. */
  MSK_SK_INF    = 6
}; /* MSKstakey_enum */
#define MSK_SK_BEGIN MSK_SK_UNK
#define MSK_SK_END   (1+MSK_SK_INF)
#ifdef MSK_NO_ENUMS
typedef int MSKstakeye;
#else
typedef enum MSKstakey_enum MSKstakeye;
#endif

enum MSKstartpointtype_enum {
  /** The starting point is chosen automatically. */
  MSK_STARTING_POINT_FREE     = 0,
  /** The optimizer guesses a starting point. */
  MSK_STARTING_POINT_GUESS    = 1,
  /** The optimizer constructs a starting point by assigning a constant value to all primal and dual variables. This starting point is normally robust. */
  MSK_STARTING_POINT_CONSTANT = 2
}; /* MSKstartpointtype_enum */
#define MSK_STARTING_POINT_BEGIN MSK_STARTING_POINT_FREE
#define MSK_STARTING_POINT_END   (1+MSK_STARTING_POINT_CONSTANT)
#ifdef MSK_NO_ENUMS
typedef int MSKstartpointtypee;
#else
typedef int MSKstartpointtypee;
#endif

enum MSKstreamtype_enum {
  /** Log stream. Contains the aggregated contents of all other streams. This means that a message written to any other stream will also be written to this stream. */
  MSK_STREAM_LOG = 0,
  /** Message stream. Log information relating to performance and progress of the optimization is written to this stream. */
  MSK_STREAM_MSG = 1,
  /** Error stream. Error messages are written to this stream. */
  MSK_STREAM_ERR = 2,
  /** Warning stream. Warning messages are written to this stream. */
  MSK_STREAM_WRN = 3
}; /* MSKstreamtype_enum */
#define MSK_STREAM_BEGIN MSK_STREAM_LOG
#define MSK_STREAM_END   (1+MSK_STREAM_WRN)
#ifdef MSK_NO_ENUMS
typedef int MSKstreamtypee;
#else
typedef enum MSKstreamtype_enum MSKstreamtypee;
#endif

enum MSKvalue_enum {
  /** The length of a license key buffer. */
  MSK_LICENSE_BUFFER_LENGTH = 21,
  /** Maximum string length allowed in MOSEK. */
  MSK_MAX_STR_LEN           = 1024
}; /* MSKvalue_enum */
#ifdef MSK_NO_ENUMS
typedef int MSKvaluee;
#else
typedef int MSKvaluee;
#endif

enum MSKvariabletype_enum {
  /** Is a continuous variable. */
  MSK_VAR_TYPE_CONT = 0,
  /** Is an integer variable. */
  MSK_VAR_TYPE_INT  = 1
}; /* MSKvariabletype_enum */
#define MSK_VAR_BEGIN MSK_VAR_TYPE_CONT
#define MSK_VAR_END   (1+MSK_VAR_TYPE_INT)
#ifdef MSK_NO_ENUMS
typedef int MSKvariabletypee;
#else
typedef enum MSKvariabletype_enum MSKvariabletypee;
#endif
typedef enum MSK_whichenum_enum {
  MSK_WHICHENUM_BASINDTYPE,
  MSK_WHICHENUM_BOUNDKEY,
  MSK_WHICHENUM_BRANCHDIR,
  MSK_WHICHENUM_CALLBACKCODE,
  MSK_WHICHENUM_COMPRESSTYPE,
  MSK_WHICHENUM_CONETYPE,
  MSK_WHICHENUM_DATAFORMAT,
  MSK_WHICHENUM_DINFITEM,
  MSK_WHICHENUM_DOMAINTYPE,
  MSK_WHICHENUM_DPARAM,
  MSK_WHICHENUM_FEATURE,
  MSK_WHICHENUM_FIXINGMETHOD,
  MSK_WHICHENUM_FOLDINGMODE,
  MSK_WHICHENUM_IINFITEM,
  MSK_WHICHENUM_INFTYPE,
  MSK_WHICHENUM_INTERNAL_DINF,
  MSK_WHICHENUM_INTERNAL_IINF,
  MSK_WHICHENUM_INTERNAL_LIINF,
  MSK_WHICHENUM_INTPNTHOTSTART,
  MSK_WHICHENUM_IOMODE,
  MSK_WHICHENUM_IPARAM,
  MSK_WHICHENUM_LANGUAGE,
  MSK_WHICHENUM_LIINFITEM,
  MSK_WHICHENUM_MARK,
  MSK_WHICHENUM_MIOCONTSOLTYPE,
  MSK_WHICHENUM_MIODATAPERMMETHOD,
  MSK_WHICHENUM_MIOMODE,
  MSK_WHICHENUM_MIONODESELTYPE,
  MSK_WHICHENUM_MIOVARSELTYPE,
  MSK_WHICHENUM_MIQCQOREFORMMETHOD,
  MSK_WHICHENUM_MPSFORMAT,
  MSK_WHICHENUM_NAMETYPE,
  MSK_WHICHENUM_OBJSENSE,
  MSK_WHICHENUM_ONOFFKEY,
  MSK_WHICHENUM_OPTIMIZERTYPE,
  MSK_WHICHENUM_ORDERINGTYPE,
  MSK_WHICHENUM_PARAMETERTYPE,
  MSK_WHICHENUM_PRESOLVEMODE,
  MSK_WHICHENUM_PROBLEMITEM,
  MSK_WHICHENUM_PROBLEMTYPE,
  MSK_WHICHENUM_PROSTA,
  MSK_WHICHENUM_RESCODE,
  MSK_WHICHENUM_RESCODETYPE,
  MSK_WHICHENUM_SCALINGMETHOD,
  MSK_WHICHENUM_SCALINGTYPE,
  MSK_WHICHENUM_SENSITIVITYTYPE,
  MSK_WHICHENUM_SIMDEGEN,
  MSK_WHICHENUM_SIMDUPVEC,
  MSK_WHICHENUM_SIMHOTSTART,
  MSK_WHICHENUM_SIMPRECISION,
  MSK_WHICHENUM_SIMREFORM,
  MSK_WHICHENUM_SIMSELTYPE,
  MSK_WHICHENUM_SOLFORMAT,
  MSK_WHICHENUM_SOLITEM,
  MSK_WHICHENUM_SOLSTA,
  MSK_WHICHENUM_SOLTYPE,
  MSK_WHICHENUM_SOLVEFORM,
  MSK_WHICHENUM_SPARAM,
  MSK_WHICHENUM_STAKEY,
  MSK_WHICHENUM_STARTPOINTTYPE,
  MSK_WHICHENUM_STREAMTYPE,
  MSK_WHICHENUM_SYMMATTYPE,
  MSK_WHICHENUM_TRANSPOSE,
  MSK_WHICHENUM_UPLO,
  MSK_WHICHENUM_VARIABLETYPE,
  MSK_WHICHENUM_LAST
}
MSKwhichenume;


#define MSK_FIRST_ERR_CODE 1000
#define MSK_LAST_ERR_CODE  9999



/* Typedefs */
typedef char       MSKchart;
typedef void     * MSKvoid_t;
struct mskenvt;
struct msktaskt;
typedef struct mskenvt  MSKenv;
typedef struct msktaskt MSKtask;


#ifdef  MSKINT64
typedef MSKINT64 __mskint64;
#else
typedef long long __mskint64;
#endif

#if defined(LLONG_MAX) && LLONG_MAX <= INT_MAX
#warning "Expected (long long) to be a 64bit type. MOSEK API functions may not work."
#endif
typedef int          __mskint32;

/*
typedef unsigned int       __mskuint32;
typedef signed   int       __mskint32;
typedef unsigned long long __mskuint64;
typedef signed   long long __mskint64;
*/

typedef MSKenv * MSKenv_t;
typedef MSKtask * MSKtask_t;
typedef void * MSKuserhandle_t;
typedef int MSKbooleant;
typedef int32_t MSKint32t;
typedef int64_t MSKint64t;
typedef wchar_t MSKwchart;
typedef double MSKrealt;
typedef char * MSKstring_t;
typedef MSKint32t (MSKAPI * MSKcallbackfunc)(
	MSKtask_t task,
	MSKuserhandle_t usrptr,
	MSKcallbackcodee caller,
	const MSKrealt* douinf,
	const MSKint32t* intinf,
	const MSKint64t* lintinf);
typedef void (MSKAPI * MSKexitfunc)(
	MSKuserhandle_t usrptr,
	const  char * file,
	MSKint32t line,
	const  char * msg);
typedef void (MSKAPI * MSKstreamfunc)(
	MSKuserhandle_t handle,
	const  char * str);
typedef MSKrescodee (MSKAPI * MSKresponsefunc)(
	MSKuserhandle_t handle,
	MSKrescodee r,
	const  char * msg);
typedef size_t (MSKAPI * MSKhreadfunc)(
	MSKuserhandle_t handle,
	void * dest,
	size_t count);
typedef size_t (MSKAPI * MSKhwritefunc)(
	MSKuserhandle_t handle,
	const void * src,
	size_t count);



/* Functions */

/* using __cplusplus */
#ifdef __cplusplus
extern "C" {
#endif


/** Analyze the names and issue an error for the first invalid name.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichstream` Index of the stream.
  * - `nametype` The type of names e.g. valid in MPS or LP files.
  */
MSKrescodee MSKAPI MSK_analyzenames(
	MSKtask_t task,
	MSKstreamtypee whichstream,
	MSKnametypee nametype);

/** Analyze the data of a task.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichstream` Index of the stream.
  */
MSKrescodee MSKAPI MSK_analyzeproblem(
	MSKtask_t task,
	MSKstreamtypee whichstream);

/** Print information related to the quality of the solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichstream` Index of the stream.
  * - `whichsol` Selects a solution.
  */
MSKrescodee MSKAPI MSK_analyzesolution(
	MSKtask_t task,
	MSKstreamtypee whichstream,
	MSKsoltypee whichsol);

/** Appends an affine conic constraint to the task.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `domidx` Domain index.
  * - `numafeidx` Number of affine expressions in the affine expression list (must equal the dimension of the domain).
  * - `afeidxlist` List of affine expression indexes.
  * - `b` The vector of constant terms modifying affine expressions. Optional.
  */
MSKrescodee MSKAPI MSK_appendacc(
	MSKtask_t task,
	MSKint64t domidx,
	MSKint64t numafeidx,
	const MSKint64t * afeidxlist,
	const MSKrealt * b);

/** Appends a number of affine conic constraint to the task.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numaccs` The number of affine conic constraints to append.
  * - `domidxs` Domain indices.
  * - `numafeidx` Number of affine expressions in the affine expression list (must equal the sum of dimensions of the domains).
  * - `afeidxlist` List of affine expression indexes.
  * - `b` The vector of constant terms modifying affine expressions. Optional.
  */
MSKrescodee MSKAPI MSK_appendaccs(
	MSKtask_t task,
	MSKint64t numaccs,
	const MSKint64t * domidxs,
	MSKint64t numafeidx,
	const MSKint64t * afeidxlist,
	const MSKrealt * b);

/** Appends an affine conic constraint to the task.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `domidx` Domain index.
  * - `numafeidx` Number of affine expressions in the affine expression list (must equal the dimension of the domain).
  * - `afeidxfirst` Index of the first affine expression.
  * - `b` The vector of constant terms modifying affine expressions. Optional.
  */
MSKrescodee MSKAPI MSK_appendaccseq(
	MSKtask_t task,
	MSKint64t domidx,
	MSKint64t numafeidx,
	MSKint64t afeidxfirst,
	const MSKrealt * b);

/** Appends a number of affine conic constraint to the task.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numaccs` The number of affine conic constraints to append.
  * - `domidxs` Domain indices.
  * - `numafeidx` Number of affine expressions in the affine expression list (must equal the sum of dimensions of the domains).
  * - `afeidxfirst` Index of the first affine expression.
  * - `b` The vector of constant terms modifying affine expressions. Optional.
  */
MSKrescodee MSKAPI MSK_appendaccsseq(
	MSKtask_t task,
	MSKint64t numaccs,
	const MSKint64t * domidxs,
	MSKint64t numafeidx,
	MSKint64t afeidxfirst,
	const MSKrealt * b);

/** Appends a number of empty affine expressions to the optimization task.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of empty affine expressions which should be appended.
  */
MSKrescodee MSKAPI MSK_appendafes(
	MSKtask_t task,
	MSKint64t num);

/** Appends semidefinite variables to the problem.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of symmetric matrix variables to be appended.
  * - `dim` Dimensions of symmetric matrix variables to be added.
  */
MSKrescodee MSKAPI MSK_appendbarvars(
	MSKtask_t task,
	MSKint32t num,
	const MSKint32t * dim);

/** Appends a new conic constraint to the problem.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `ct` Specifies the type of the cone.
  * - `conepar` For the power cone it denotes the exponent alpha. For other cone types it is unused and can be set to 0.
  * - `nummem` Number of member variables in the cone.
  * - `submem` Variable subscripts of the members in the cone.
  */
MSKrescodee MSKAPI MSK_appendcone(
	MSKtask_t task,
	MSKconetypee ct,
	MSKrealt conepar,
	MSKint32t nummem,
	const MSKint32t * submem);

/** Appends a new conic constraint to the problem.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `ct` Specifies the type of the cone.
  * - `conepar` For the power cone it denotes the exponent alpha. For other cone types it is unused and can be set to 0.
  * - `nummem` Number of member variables in the cone.
  * - `j` Index of the first variable in the conic constraint.
  */
MSKrescodee MSKAPI MSK_appendconeseq(
	MSKtask_t task,
	MSKconetypee ct,
	MSKrealt conepar,
	MSKint32t nummem,
	MSKint32t j);

/** Appends multiple conic constraints to the problem.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of cones to be added.
  * - `ct` Specifies the type of the cone.
  * - `conepar` For the power cone it denotes the exponent alpha. For other cone types it is unused and can be set to 0.
  * - `nummem` Numbers of member variables in the cones.
  * - `j` Index of the first variable in the first cone to be appended.
  */
MSKrescodee MSKAPI MSK_appendconesseq(
	MSKtask_t task,
	MSKint32t num,
	const MSKconetypee * ct,
	const MSKrealt * conepar,
	const MSKint32t * nummem,
	MSKint32t j);

/** Appends a number of constraints to the optimization task.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of constraints which should be appended.
  */
MSKrescodee MSKAPI MSK_appendcons(
	MSKtask_t task,
	MSKint32t num);

/** Appends a number of empty disjunctive constraints to the task.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of empty disjunctive constraints which should be appended.
  */
MSKrescodee MSKAPI MSK_appenddjcs(
	MSKtask_t task,
	MSKint64t num);

/** Appends the dual exponential cone domain.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `domidx` Index of the domain.
  */
MSKrescodee MSKAPI MSK_appenddualexpconedomain(
	MSKtask_t task,
	MSKint64t * domidx);

/** Appends the dual geometric mean cone domain.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `n` Dimension of the domain.
  * - `domidx` Index of the domain.
  */
MSKrescodee MSKAPI MSK_appenddualgeomeanconedomain(
	MSKtask_t task,
	MSKint64t n,
	MSKint64t * domidx);

/** Appends the dual power cone domain.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `n` Dimension of the domain.
  * - `nleft` Number of variables on the left hand side.
  * - `alpha` The sequence proportional to exponents. Must be positive.
  * - `domidx` Index of the domain.
  */
MSKrescodee MSKAPI MSK_appenddualpowerconedomain(
	MSKtask_t task,
	MSKint64t n,
	MSKint64t nleft,
	const MSKrealt * alpha,
	MSKint64t * domidx);

/** Appends a sequence of dual power cone domains.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of domains to append.
  * - `n` Dimensions of the domains.
  * - `nleft` Number of variables on the left hand sides.
  * - `alpha` The sequences proportional to exponents, concatenated for all domains. Must be positive.
  * - `domidxlist` Indexes of the domains.
  */
MSKrescodee MSKAPI MSK_appenddualpowerconedomainseq(
	MSKtask_t task,
	MSKint64t num,
	const MSKint64t * n,
	const MSKint64t * nleft,
	const MSKrealt * alpha,
	MSKint64t * domidxlist);

/** Appends the primal exponential cone domain.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `domidx` Index of the domain.
  */
MSKrescodee MSKAPI MSK_appendprimalexpconedomain(
	MSKtask_t task,
	MSKint64t * domidx);

/** Appends the primal geometric mean cone domain.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `n` Dimension of the domain.
  * - `domidx` Index of the domain.
  */
MSKrescodee MSKAPI MSK_appendprimalgeomeanconedomain(
	MSKtask_t task,
	MSKint64t n,
	MSKint64t * domidx);

/** Appends the primal power cone domain.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `n` Dimension of the domain.
  * - `nleft` Number of variables on the left hand side.
  * - `alpha` The sequence proportional to exponents. Must be positive.
  * - `domidx` Index of the domain.
  */
MSKrescodee MSKAPI MSK_appendprimalpowerconedomain(
	MSKtask_t task,
	MSKint64t n,
	MSKint64t nleft,
	const MSKrealt * alpha,
	MSKint64t * domidx);

/** Appends a sequence of primal power cone domains.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of domains to append.
  * - `n` Dimensions of the domains.
  * - `nleft` Number of variables on the left hand sides.
  * - `alpha` The sequences proportional to exponents, concatenated for all domains. Must be positive.
  * - `domidxlist` Indexes of the domains.
  */
MSKrescodee MSKAPI MSK_appendprimalpowerconedomainseq(
	MSKtask_t task,
	MSKint64t num,
	const MSKint64t * n,
	const MSKint64t * nleft,
	const MSKrealt * alpha,
	MSKint64t * domidxlist);

/** Appends the n dimensional quadratic cone domain.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `n` Dimension of the domain.
  * - `domidx` Index of the domain.
  */
MSKrescodee MSKAPI MSK_appendquadraticconedomain(
	MSKtask_t task,
	MSKint64t n,
	MSKint64t * domidx);

/** Appends the n dimensional real number domain.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `n` Dimension of the domain.
  * - `domidx` Index of the domain.
  */
MSKrescodee MSKAPI MSK_appendrdomain(
	MSKtask_t task,
	MSKint64t n,
	MSKint64t * domidx);

/** Appends the n dimensional negative orthant to the list of domains.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `n` Dimension of the domain.
  * - `domidx` Index of the domain.
  */
MSKrescodee MSKAPI MSK_appendrminusdomain(
	MSKtask_t task,
	MSKint64t n,
	MSKint64t * domidx);

/** Appends the n dimensional positive orthant to the list of domains.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `n` Dimension of the domain.
  * - `domidx` Index of the domain.
  */
MSKrescodee MSKAPI MSK_appendrplusdomain(
	MSKtask_t task,
	MSKint64t n,
	MSKint64t * domidx);

/** Appends the n dimensional rotated quadratic cone domain.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `n` Dimension of the domain.
  * - `domidx` Index of the domain.
  */
MSKrescodee MSKAPI MSK_appendrquadraticconedomain(
	MSKtask_t task,
	MSKint64t n,
	MSKint64t * domidx);

/** Appends the n dimensional 0 domain.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `n` Dimension of the domain.
  * - `domidx` Index of the domain.
  */
MSKrescodee MSKAPI MSK_appendrzerodomain(
	MSKtask_t task,
	MSKint64t n,
	MSKint64t * domidx);

/** Appends a general sparse symmetric matrix to the storage of symmetric matrices.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `dim` Dimension of the symmetric matrix that is appended.
  * - `nz` Number of triplets.
  * - `subi` Row subscript in the triplets.
  * - `subj` Column subscripts in the triplets.
  * - `valij` Values of each triplet.
  * - `idx` Unique index assigned to the inputted matrix.
  */
MSKrescodee MSKAPI MSK_appendsparsesymmat(
	MSKtask_t task,
	MSKint32t dim,
	MSKint64t nz,
	const MSKint32t * subi,
	const MSKint32t * subj,
	const MSKrealt * valij,
	MSKint64t * idx);

/** Appends a general sparse symmetric matrix to the storage of symmetric matrices.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of matrixes to append.
  * - `dims` Dimensions of the symmetric matrixes.
  * - `nz` Number of nonzeros for each matrix.
  * - `subi` Row subscript in the triplets.
  * - `subj` Column subscripts in the triplets.
  * - `valij` Values of each triplet.
  * - `idx` Unique index assigned to the inputted matrix.
  */
MSKrescodee MSKAPI MSK_appendsparsesymmatlist(
	MSKtask_t task,
	MSKint32t num,
	const MSKint32t * dims,
	const MSKint64t * nz,
	const MSKint32t * subi,
	const MSKint32t * subj,
	const MSKrealt * valij,
	MSKint64t * idx);

/** Appends the vectorized SVEC PSD cone domain.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `n` Dimension of the domain.
  * - `domidx` Index of the domain.
  */
MSKrescodee MSKAPI MSK_appendsvecpsdconedomain(
	MSKtask_t task,
	MSKint64t n,
	MSKint64t * domidx);

/** Appends a number of variables to the optimization task.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of variables which should be appended.
  */
MSKrescodee MSKAPI MSK_appendvars(
	MSKtask_t task,
	MSKint32t num);

/** Get the optimizer log from a remote job.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `addr` Address of the solver server
  * - `accesstoken` Access token string.
  * - `token` Job token
  */
MSKrescodee MSKAPI MSK_asyncgetlog(
	MSKtask_t task,
	const char * addr,
	const char * accesstoken,
	const char * token);

/** Request a solution from a remote job.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `address` Address of the OptServer.
  * - `accesstoken` Access token.
  * - `token` The task token.
  * - `respavailable` Indicates if a remote response is available.
  * - `resp` Is the response code from the remote solver.
  * - `trm` Is either OK or a termination response code.
  */
MSKrescodee MSKAPI MSK_asyncgetresult(
	MSKtask_t task,
	const char * address,
	const char * accesstoken,
	const char * token,
	MSKbooleant * respavailable,
	MSKrescodee * resp,
	MSKrescodee * trm);

/** Offload the optimization task to a solver server in asynchronous mode.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `address` Address of the OptServer.
  * - `accesstoken` Access token.
  * - `token` Returns the task token.
  */
MSKrescodee MSKAPI MSK_asyncoptimize(
	MSKtask_t task,
	const char * address,
	const char * accesstoken,
	char * token);

/** Requests information about the status of the remote job.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `address` Address of the OptServer.
  * - `accesstoken` Access token.
  * - `token` The task token.
  * - `respavailable` Indicates if a remote response is available.
  * - `resp` Is the response code from the remote solver.
  * - `trm` Is either OK or a termination response code.
  */
MSKrescodee MSKAPI MSK_asyncpoll(
	MSKtask_t task,
	const char * address,
	const char * accesstoken,
	const char * token,
	MSKbooleant * respavailable,
	MSKrescodee * resp,
	MSKrescodee * trm);

/** Request that the job identified by the token is terminated.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `address` Address of the OptServer.
  * - `accesstoken` Access token.
  * - `token` The task token.
  */
MSKrescodee MSKAPI MSK_asyncstop(
	MSKtask_t task,
	const char * address,
	const char * accesstoken,
	const char * token);

/** Computes conditioning information for the basis matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `nrmbasis` An estimate for the 1-norm of the basis.
  * - `nrminvbasis` An estimate for the 1-norm of the inverse of the basis.
  */
MSKrescodee MSKAPI MSK_basiscond(
	MSKtask_t task,
	MSKrealt * nrmbasis,
	MSKrealt * nrminvbasis);

/** Obtains a bound key string identifier.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `bk` Bound key.
  * - `str` String corresponding to the bound key.
  */
MSKrescodee MSKAPI MSK_bktostr(
	MSKtask_t task,
	MSKboundkeye bk,
	char * str);

/** Debug version of the task calloc function.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `number` Number of elements.
  * - `size` Size of each individual element.
  * - `file` File from which the function is called.
  * - `line` Line in the file from which the function is called.
  */
void * MSKAPI MSK_callocdbgtask(
	MSKtask_t task,
	size_t number,
	size_t size,
	const char * file,
	unsigned line);

/** A replacement for the system calloc function.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `number` Number of elements.
  * - `size` Size of each individual element.
  */
void * MSKAPI MSK_calloctask(
	MSKtask_t task,
	size_t number,
	size_t size);

/** Checks the memory allocated by the task.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `file` File from which the function is called.
  * - `line` Line in the file from which the function is called.
  */
MSKrescodee MSKAPI MSK_checkmemtask(
	MSKtask_t task,
	const char * file,
	MSKint32t line);

/** Changes the bounds for one constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `i` Index of the constraint for which the bounds should be changed.
  * - `lower` If non-zero, then the lower bound is changed, otherwise the upper bound is changed.
  * - `finite` If non-zero, then the given value is assumed to be finite.
  * - `value` New value for the bound.
  */
MSKrescodee MSKAPI MSK_chgconbound(
	MSKtask_t task,
	MSKint32t i,
	MSKint32t lower,
	MSKint32t finite,
	MSKrealt value);

/** Changes the bounds for one variable.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `j` Index of the variable for which the bounds should be changed.
  * - `lower` If non-zero, then the lower bound is changed, otherwise the upper bound is changed.
  * - `finite` If non-zero, then the given value is assumed to be finite.
  * - `value` New value for the bound.
  */
MSKrescodee MSKAPI MSK_chgvarbound(
	MSKtask_t task,
	MSKint32t j,
	MSKint32t lower,
	MSKint32t finite,
	MSKrealt value);

/** Creates a clone of an existing task.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `clonedtask` The cloned task.
  */
MSKrescodee MSKAPI MSK_clonetask(
	MSKtask_t task,
	MSKtask_t * clonedtask);

/** Commits all cached problem changes.
  *
  * #Arguments
  * - `task` An optimization task.
  */
MSKrescodee MSKAPI MSK_commitchanges(
	MSKtask_t task);

/** Obtains a cone type string identifier.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `ct` Specifies the type of the cone.
  * - `str` String corresponding to the cone type.
  */
MSKrescodee MSKAPI MSK_conetypetostr(
	MSKtask_t task,
	MSKconetypee ct,
	char * str);

/** Undefine a solution and free the memory it uses.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  */
MSKrescodee MSKAPI MSK_deletesolution(
	MSKtask_t task,
	MSKsoltypee whichsol);

/** Deletes a task.
  *
  * #Arguments
  * - `task` An optimization task.
  */
MSKrescodee MSKAPI MSK_deletetask(
	MSKtask_t * task);

/** Performs sensitivity analysis on objective coefficients.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numj` Length of the subscripts array.
  * - `subj` Indexes of objective coefficients to analyze.
  * - `leftpricej` Left shadow prices for requested coefficients.
  * - `rightpricej` Right shadow prices for requested coefficients.
  * - `leftrangej` Left range for requested coefficients.
  * - `rightrangej` Right range for requested coefficients.
  */
MSKrescodee MSKAPI MSK_dualsensitivity(
	MSKtask_t task,
	MSKint32t numj,
	const MSKint32t * subj,
	MSKrealt * leftpricej,
	MSKrealt * rightpricej,
	MSKrealt * leftrangej,
	MSKrealt * rightrangej);

/** Prints a formatted string to a task stream.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichstream` Index of the stream.
  * - `format` A valid printf-compatible format string
  * - `...` 
  */
MSKrescodee MSKAPIVA MSK_echotask(
	MSKtask_t task,
	MSKstreamtypee whichstream,
	const char * format,
	...);

/** Clears a row in barF
  *
  * #Arguments
  * - `task` An optimization task.
  * - `afeidx` Row index of barF.
  */
MSKrescodee MSKAPI MSK_emptyafebarfrow(
	MSKtask_t task,
	MSKint64t afeidx);

/** Clears rows in barF.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numafeidx` The number of rows to zero.
  * - `afeidxlist` Indices of rows in barF to clear.
  */
MSKrescodee MSKAPI MSK_emptyafebarfrowlist(
	MSKtask_t task,
	MSKint64t numafeidx,
	const MSKint64t * afeidxlist);

/** Clears a column in F.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `varidx` Variable index.
  */
MSKrescodee MSKAPI MSK_emptyafefcol(
	MSKtask_t task,
	MSKint32t varidx);

/** Clears columns in F.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numvaridx` The number of columns to zero.
  * - `varidx` Indices of variables in F to clear.
  */
MSKrescodee MSKAPI MSK_emptyafefcollist(
	MSKtask_t task,
	MSKint64t numvaridx,
	const MSKint32t * varidx);

/** Clears a row in F.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `afeidx` Row index.
  */
MSKrescodee MSKAPI MSK_emptyafefrow(
	MSKtask_t task,
	MSKint64t afeidx);

/** Clears rows in F.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numafeidx` The number of rows to zero.
  * - `afeidx` Indices of rows in F to clear.
  */
MSKrescodee MSKAPI MSK_emptyafefrowlist(
	MSKtask_t task,
	MSKint64t numafeidx,
	const MSKint64t * afeidx);

/** Evaluates the activity of an affine conic constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `accidx` The index of the affine conic constraint.
  * - `activity` The activity of the affine conic constraint. The array should have length equal to the dimension of the constraint.
  */
MSKrescodee MSKAPI MSK_evaluateacc(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint64t accidx,
	MSKrealt * activity);

/** Evaluates the activities of all affine conic constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `activity` The activity of affine conic constraints. The array should have length equal to the sum of dimensions of all affine conic constraints.
  */
MSKrescodee MSKAPI MSK_evaluateaccs(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKrealt * activity);

/** Frees space allocated by MOSEK.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `buffer` A pointer.
  * - `file` File from which the function is called.
  * - `line` Line in the file from which the function is called.
  */
void MSKAPI MSK_freedbgtask(
	MSKtask_t task,
	void * buffer,
	const char * file,
	unsigned line);

/** Frees space allocated by MOSEK.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `buffer` A pointer.
  */
void MSKAPI MSK_freetask(
	MSKtask_t task,
	void * buffer);

/** Generates systematic names for affine conic constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of variable indexes.
  * - `sub` Indexes of the affine conic constraints.
  * - `fmt` The variable name formatting string.
  * - `ndims` Number of dimensions in the shape.
  * - `dims` Dimensions in the shape.
  * - `sp` Items that should be named.
  * - `numnamedaxis` Number of named axes
  * - `namedaxisidxs` List if named index axes
  * - `numnames` Total number of names.
  * - `names` All axis names.
  */
MSKrescodee MSKAPI MSK_generateaccnames(
	MSKtask_t task,
	MSKint64t num,
	const MSKint64t * sub,
	const char * fmt,
	MSKint32t ndims,
	const MSKint32t * dims,
	const MSKint64t * sp,
	MSKint32t numnamedaxis,
	const MSKint32t * namedaxisidxs,
	MSKint64t numnames,
	const char * names[]);

/** Internal.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of variable indexes.
  * - `subj` Indexes of the variables.
  * - `fmt` The variable name formatting string.
  * - `ndims` Number of dimensions in the shape.
  * - `dims` Dimensions in the shape.
  * - `sp` Items that should be named.
  * - `numnamedaxis` Number of named axes
  * - `namedaxisidxs` List if named index axes
  * - `numnames` Total number of names.
  * - `names` All axis names.
  */
MSKrescodee MSKAPI MSK_generatebarvarnames(
	MSKtask_t task,
	MSKint32t num,
	const MSKint32t * subj,
	const char * fmt,
	MSKint32t ndims,
	const MSKint32t * dims,
	const MSKint64t * sp,
	MSKint32t numnamedaxis,
	const MSKint32t * namedaxisidxs,
	MSKint64t numnames,
	const char * names[]);

/** Generates systematic names for cone.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of cone indexes.
  * - `subk` Indexes of the cone.
  * - `fmt` The cone name formatting string.
  * - `ndims` Number of dimensions in the shape.
  * - `dims` Dimensions in the shape.
  * - `sp` Items that should be named.
  * - `numnamedaxis` Number of named axes
  * - `namedaxisidxs` List if named index axes
  * - `numnames` Total number of names.
  * - `names` All axis names.
  */
MSKrescodee MSKAPI MSK_generateconenames(
	MSKtask_t task,
	MSKint32t num,
	const MSKint32t * subk,
	const char * fmt,
	MSKint32t ndims,
	const MSKint32t * dims,
	const MSKint64t * sp,
	MSKint32t numnamedaxis,
	const MSKint32t * namedaxisidxs,
	MSKint64t numnames,
	const char * names[]);

/** Internal.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of constraint indexes.
  * - `subi` Indexes of the constraints.
  * - `fmt` The constraint name formatting string.
  * - `ndims` Number of dimensions in the shape.
  * - `dims` Dimensions in the shape.
  * - `sp` Items that should be named.
  * - `numnamedaxis` Number of named axes
  * - `namedaxisidxs` List if named index axes
  * - `numnames` Total number of names.
  * - `names` All axis names.
  */
MSKrescodee MSKAPI MSK_generateconnames(
	MSKtask_t task,
	MSKint32t num,
	const MSKint32t * subi,
	const char * fmt,
	MSKint32t ndims,
	const MSKint32t * dims,
	const MSKint64t * sp,
	MSKint32t numnamedaxis,
	const MSKint32t * namedaxisidxs,
	MSKint64t numnames,
	const char * names[]);

/** Generates systematic names for affine conic constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of variable indexes.
  * - `sub` Indexes of the disjunctive constraints.
  * - `fmt` The variable name formatting string.
  * - `ndims` Number of dimensions in the shape.
  * - `dims` Dimensions in the shape.
  * - `sp` Items that should be named.
  * - `numnamedaxis` Number of named axes
  * - `namedaxisidxs` List if named index axes
  * - `numnames` Total number of names.
  * - `names` All axis names.
  */
MSKrescodee MSKAPI MSK_generatedjcnames(
	MSKtask_t task,
	MSKint64t num,
	const MSKint64t * sub,
	const char * fmt,
	MSKint32t ndims,
	const MSKint32t * dims,
	const MSKint64t * sp,
	MSKint32t numnamedaxis,
	const MSKint32t * namedaxisidxs,
	MSKint64t numnames,
	const char * names[]);

/** Internal.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of variable indexes.
  * - `subj` Indexes of the variables.
  * - `fmt` The variable name formatting string.
  * - `ndims` Number of dimensions in the shape.
  * - `dims` Dimensions in the shape.
  * - `sp` Items that should be named.
  * - `numnamedaxis` Number of named axes
  * - `namedaxisidxs` List if named index axes
  * - `numnames` Total number of names.
  * - `names` All axis names.
  */
MSKrescodee MSKAPI MSK_generatevarnames(
	MSKtask_t task,
	MSKint32t num,
	const MSKint32t * subj,
	const char * fmt,
	MSKint32t ndims,
	const MSKint32t * dims,
	const MSKint64t * sp,
	MSKint32t numnamedaxis,
	const MSKint32t * namedaxisidxs,
	MSKint64t numnames,
	const char * names[]);

/** Obtains the list of affine expressions appearing in the affine conic constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `accidx` Index of the affine conic constraint.
  * - `afeidxlist` List of indexes of affine expressions appearing in the constraint.
  */
MSKrescodee MSKAPI MSK_getaccafeidxlist(
	MSKtask_t task,
	MSKint64t accidx,
	MSKint64t * afeidxlist);

/** Obtains the additional constant term vector appearing in the affine conic constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `accidx` Index of the affine conic constraint.
  * - `b` The vector b appearing in the constraint.
  */
MSKrescodee MSKAPI MSK_getaccb(
	MSKtask_t task,
	MSKint64t accidx,
	MSKrealt * b);

/** Obtains barF, implied by the ACCs, in block triplet form.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumtrip` acc_afe, bar_var, blk_row, blk_col and blk_val must be numtrip long.
  * - `numtrip` Number of elements in the block triplet form.
  * - `acc_afe` Index of the AFE within the concatenated list of AFEs in ACCs.
  * - `bar_var` Symmetric matrix variable index.
  * - `blk_row` Block row index.
  * - `blk_col` Block column index.
  * - `blk_val` The numerical value associated with each block triplet.
  */
MSKrescodee MSKAPI MSK_getaccbarfblocktriplet(
	MSKtask_t task,
	MSKint64t maxnumtrip,
	MSKint64t * numtrip,
	MSKint64t * acc_afe,
	MSKint32t * bar_var,
	MSKint32t * blk_row,
	MSKint32t * blk_col,
	MSKrealt * blk_val);

/** Obtains an upper bound on the number of elements in the block triplet form of barf, as used within the ACCs.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numtrip` An upper bound on the number of elements in the block triplet form of barf, as used within the ACCs.
  */
MSKrescodee MSKAPI MSK_getaccbarfnumblocktriplets(
	MSKtask_t task,
	MSKint64t * numtrip);

/** Obtains the domain appearing in the affine conic constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `accidx` The index of the affine conic constraint.
  * - `domidx` The index of domain in the affine conic constraint.
  */
MSKrescodee MSKAPI MSK_getaccdomain(
	MSKtask_t task,
	MSKint64t accidx,
	MSKint64t * domidx);

/** Obtains the doty vector for an affine conic constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `accidx` The index of the affine conic constraint.
  * - `doty` The dual values for this affine conic constraint. The array should have length equal to the dimension of the constraint.
  */
MSKrescodee MSKAPI MSK_getaccdoty(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint64t accidx,
	MSKrealt * doty);

/** Obtains the doty vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `doty` The dual values of affine conic constraints. The array should have length equal to the sum of dimensions of all affine conic constraints.
  */
MSKrescodee MSKAPI MSK_getaccdotys(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKrealt * doty);

/** Obtains the total number of nonzeros in the ACC implied F matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `accfnnz` Number of nonzeros in the F matrix implied by ACCs.
  */
MSKrescodee MSKAPI MSK_getaccfnumnz(
	MSKtask_t task,
	MSKint64t * accfnnz);

/** Obtains the F matrix (implied by the AFE ordering within the ACCs) in triplet format.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `frow` Row indices of nonzeros in the implied F matrix.
  * - `fcol` Column indices of nonzeros in the implied F matrix.
  * - `fval` Values of nonzero entries in the implied F matrix.
  */
MSKrescodee MSKAPI MSK_getaccftrip(
	MSKtask_t task,
	MSKint64t * frow,
	MSKint32t * fcol,
	MSKrealt * fval);

/** The g vector as used within the ACCs.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `g` The g vector as used within the ACCs.
  */
MSKrescodee MSKAPI MSK_getaccgvector(
	MSKtask_t task,
	MSKrealt * g);

/** Obtains the dimension of the affine conic constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `accidx` The index of the affine conic constraint.
  * - `n` The dimension of the affine conic constraint (equal to the dimension of its domain).
  */
MSKrescodee MSKAPI MSK_getaccn(
	MSKtask_t task,
	MSKint64t accidx,
	MSKint64t * n);

/** Obtains the name of an affine conic constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `accidx` Index of an affine conic constraint.
  * - `sizename` The length of the name buffer.
  * - `name` Returns the required name.
  */
MSKrescodee MSKAPI MSK_getaccname(
	MSKtask_t task,
	MSKint64t accidx,
	MSKint32t sizename,
	char * name);

/** Obtains the length of the name of an affine conic constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `accidx` Index of an affine conic constraint.
  * - `len` Returns the length of the indicated name.
  */
MSKrescodee MSKAPI MSK_getaccnamelen(
	MSKtask_t task,
	MSKint64t accidx,
	MSKint32t * len);

/** Obtains the total dimension of all affine conic constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `n` The total dimension of all affine conic constraints.
  */
MSKrescodee MSKAPI MSK_getaccntot(
	MSKtask_t task,
	MSKint64t * n);

/** Obtains full data of all affine conic constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `domidxlist` The list of domains appearing in all affine conic constraints.
  * - `afeidxlist` The concatenation of index lists of affine expressions appearing in all affine conic constraints.
  * - `b` The concatenation of vectors b appearing in all affine conic constraints.
  */
MSKrescodee MSKAPI MSK_getaccs(
	MSKtask_t task,
	MSKint64t * domidxlist,
	MSKint64t * afeidxlist,
	MSKrealt * b);

/** Obtains one column of the linear constraint matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `j` Index of the column.
  * - `nzj` Number of non-zeros in the column obtained.
  * - `subj` Row indices of the non-zeros in the column obtained.
  * - `valj` Numerical values in the column obtained.
  */
MSKrescodee MSKAPI MSK_getacol(
	MSKtask_t task,
	MSKint32t j,
	MSKint32t * nzj,
	MSKint32t * subj,
	MSKrealt * valj);

/** Obtains the number of non-zero elements in one column of the linear constraint matrix
  *
  * #Arguments
  * - `task` An optimization task.
  * - `i` Index of the column.
  * - `nzj` Number of non-zeros in the j'th column of (A).
  */
MSKrescodee MSKAPI MSK_getacolnumnz(
	MSKtask_t task,
	MSKint32t i,
	MSKint32t * nzj);

/** Obtains a sequence of columns from the coefficient matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `first` Index of the first column in the sequence.
  * - `last` Index of the last column in the sequence plus one.
  * - `maxnumnz` Denotes the length of the subscript and coefficient arrays.
  * - `ptrb` Column start pointers.
  * - `ptre` Column end pointers.
  * - `sub` Contains the row subscripts.
  * - `val` Contains the coefficient values.
  */
MSKrescodee MSKAPI MSK_getacolslice(
	MSKtask_t task,
	MSKint32t first,
	MSKint32t last,
	MSKint32t maxnumnz,
	MSKint32t * ptrb,
	MSKint32t * ptre,
	MSKint32t * sub,
	MSKrealt * val);

/** Obtains a sequence of columns from the coefficient matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `first` Index of the first column in the sequence.
  * - `last` Index of the last column in the sequence plus one.
  * - `maxnumnz` Denotes the length of the subscript and coefficient arrays.
  * - `ptrb` Column start pointers.
  * - `ptre` Column end pointers.
  * - `sub` Contains the row subscripts.
  * - `val` Contains the coefficient values.
  */
MSKrescodee MSKAPI MSK_getacolslice64(
	MSKtask_t task,
	MSKint32t first,
	MSKint32t last,
	MSKint64t maxnumnz,
	MSKint64t * ptrb,
	MSKint64t * ptre,
	MSKint32t * sub,
	MSKrealt * val);

/** Obtains the number of non-zeros in a slice of columns of the coefficient matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `first` Index of the first column in the sequence.
  * - `last` Index of the last column plus one in the sequence.
  * - `numnz` Number of non-zeros in the slice.
  */
MSKrescodee MSKAPI MSK_getacolslicenumnz(
	MSKtask_t task,
	MSKint32t first,
	MSKint32t last,
	MSKint32t * numnz);

/** Obtains the number of non-zeros in a slice of columns of the coefficient matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `first` Index of the first column in the sequence.
  * - `last` Index of the last column plus one in the sequence.
  * - `numnz` Number of non-zeros in the slice.
  */
MSKrescodee MSKAPI MSK_getacolslicenumnz64(
	MSKtask_t task,
	MSKint32t first,
	MSKint32t last,
	MSKint64t * numnz);

/** Obtains a sequence of columns from the coefficient matrix in triplet format.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `first` Index of the first column in the sequence.
  * - `last` Index of the last column in the sequence plus one.
  * - `maxnumnz` Denotes the length of the subscript and coefficient arrays.
  * - `subi` Constraint subscripts.
  * - `subj` Column subscripts.
  * - `val` Values.
  */
MSKrescodee MSKAPI MSK_getacolslicetrip(
	MSKtask_t task,
	MSKint32t first,
	MSKint32t last,
	MSKint64t maxnumnz,
	MSKint32t * subi,
	MSKint32t * subj,
	MSKrealt * val);

/** Obtains barF in block triplet form.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumtrip` afeidx, barvaridx, subk, subl and valijkl must be maxnumtrip long.
  * - `numtrip` Number of elements in the block triplet form.
  * - `afeidx` Constraint index.
  * - `barvaridx` Symmetric matrix variable index.
  * - `subk` Block row index.
  * - `subl` Block column index.
  * - `valkl` The numerical value associated with each block triplet.
  */
MSKrescodee MSKAPI MSK_getafebarfblocktriplet(
	MSKtask_t task,
	MSKint64t maxnumtrip,
	MSKint64t * numtrip,
	MSKint64t * afeidx,
	MSKint32t * barvaridx,
	MSKint32t * subk,
	MSKint32t * subl,
	MSKrealt * valkl);

/** Obtains an upper bound on the number of elements in the block triplet form of barf.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numtrip` An upper bound on the number of elements in the block triplet form of barf.
  */
MSKrescodee MSKAPI MSK_getafebarfnumblocktriplets(
	MSKtask_t task,
	MSKint64t * numtrip);

/** Obtains the number of nonzero entries in a row of barF.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `afeidx` Row index of barF.
  * - `numentr` Number of nonzero entries in a row of barF.
  */
MSKrescodee MSKAPI MSK_getafebarfnumrowentries(
	MSKtask_t task,
	MSKint64t afeidx,
	MSKint32t * numentr);

/** Obtains nonzero entries in one row of barF.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `afeidx` Row index of barF.
  * - `barvaridx` Semidefinite variable indices.
  * - `ptrterm` Pointers to the description of entries.
  * - `numterm` Number of terms in each entry.
  * - `termidx` Indices of semidefinite matrices from E.
  * - `termweight` Weights appearing in the weighted sum representation.
  */
MSKrescodee MSKAPI MSK_getafebarfrow(
	MSKtask_t task,
	MSKint64t afeidx,
	MSKint32t * barvaridx,
	MSKint64t * ptrterm,
	MSKint64t * numterm,
	MSKint64t * termidx,
	MSKrealt * termweight);

/** Obtains information about one row of barF.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `afeidx` Row index of barF.
  * - `numentr` Number of nonzero entries in a row of barF.
  * - `numterm` Number of terms in the weighted sums representation of the row of barF.
  */
MSKrescodee MSKAPI MSK_getafebarfrowinfo(
	MSKtask_t task,
	MSKint64t afeidx,
	MSKint32t * numentr,
	MSKint64t * numterm);

/** Obtains the total number of nonzeros in F.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numnz` Number of nonzeros in F.
  */
MSKrescodee MSKAPI MSK_getafefnumnz(
	MSKtask_t task,
	MSKint64t * numnz);

/** Obtains one row of F in sparse format.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `afeidx` Row index.
  * - `numnz` Number of non-zeros in the row obtained.
  * - `varidx` Column indices of the non-zeros in the row obtained.
  * - `val` Values of the non-zeros in the row obtained.
  */
MSKrescodee MSKAPI MSK_getafefrow(
	MSKtask_t task,
	MSKint64t afeidx,
	MSKint32t * numnz,
	MSKint32t * varidx,
	MSKrealt * val);

/** Obtains the number of nonzeros in a row of F.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `afeidx` Row index.
  * - `numnz` Number of non-zeros in the row.
  */
MSKrescodee MSKAPI MSK_getafefrownumnz(
	MSKtask_t task,
	MSKint64t afeidx,
	MSKint32t * numnz);

/** Obtains the F matrix in triplet format.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `afeidx` Row indices of nonzeros.
  * - `varidx` Column indices of nonzeros.
  * - `val` Values of nonzero entries.
  */
MSKrescodee MSKAPI MSK_getafeftrip(
	MSKtask_t task,
	MSKint64t * afeidx,
	MSKint32t * varidx,
	MSKrealt * val);

/** Obtains a single coefficient in g.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `afeidx` Element index.
  * - `g` The entry in g.
  */
MSKrescodee MSKAPI MSK_getafeg(
	MSKtask_t task,
	MSKint64t afeidx,
	MSKrealt * g);

/** Obtains a sequence of coefficients from the vector g.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `g` The slice of g as a dense vector.
  */
MSKrescodee MSKAPI MSK_getafegslice(
	MSKtask_t task,
	MSKint64t first,
	MSKint64t last,
	MSKrealt * g);

/** Obtains a single coefficient in linear constraint matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `i` Row index of the coefficient to be returned.
  * - `j` Column index of the coefficient to be returned.
  * - `aij` Returns the requested coefficient.
  */
MSKrescodee MSKAPI MSK_getaij(
	MSKtask_t task,
	MSKint32t i,
	MSKint32t j,
	MSKrealt * aij);

/** Obtains the number non-zeros in a rectangular piece of the linear constraint matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `firsti` Index of the first row in the rectangular piece.
  * - `lasti` Index of the last row plus one in the rectangular piece.
  * - `firstj` Index of the first column in the rectangular piece.
  * - `lastj` Index of the last column plus one in the rectangular piece.
  * - `numnz` Number of non-zero elements in the rectangular piece of the linear constraint matrix.
  */
MSKrescodee MSKAPI MSK_getapiecenumnz(
	MSKtask_t task,
	MSKint32t firsti,
	MSKint32t lasti,
	MSKint32t firstj,
	MSKint32t lastj,
	MSKint32t * numnz);

/** Obtains one row of the linear constraint matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `i` Index of the row.
  * - `nzi` Number of non-zeros in the row obtained.
  * - `subi` Column indices of the non-zeros in the row obtained.
  * - `vali` Numerical values of the row obtained.
  */
MSKrescodee MSKAPI MSK_getarow(
	MSKtask_t task,
	MSKint32t i,
	MSKint32t * nzi,
	MSKint32t * subi,
	MSKrealt * vali);

/** Obtains the number of non-zero elements in one row of the linear constraint matrix
  *
  * #Arguments
  * - `task` An optimization task.
  * - `i` Index of the row.
  * - `nzi` Number of non-zeros in the i'th row of `A`.
  */
MSKrescodee MSKAPI MSK_getarownumnz(
	MSKtask_t task,
	MSKint32t i,
	MSKint32t * nzi);

/** Obtains a sequence of rows from the coefficient matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `first` Index of the first row in the sequence.
  * - `last` Index of the last row in the sequence plus one.
  * - `maxnumnz` Denotes the length of the subscript and coefficient arrays.
  * - `ptrb` Row start pointers.
  * - `ptre` Row end pointers.
  * - `sub` Contains the column subscripts.
  * - `val` Contains the coefficient values.
  */
MSKrescodee MSKAPI MSK_getarowslice(
	MSKtask_t task,
	MSKint32t first,
	MSKint32t last,
	MSKint32t maxnumnz,
	MSKint32t * ptrb,
	MSKint32t * ptre,
	MSKint32t * sub,
	MSKrealt * val);

/** Obtains a sequence of rows from the coefficient matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `first` Index of the first row in the sequence.
  * - `last` Index of the last row in the sequence plus one.
  * - `maxnumnz` Denotes the length of the subscript and coefficient arrays.
  * - `ptrb` Row start pointers.
  * - `ptre` Row end pointers.
  * - `sub` Contains the column subscripts.
  * - `val` Contains the coefficient values.
  */
MSKrescodee MSKAPI MSK_getarowslice64(
	MSKtask_t task,
	MSKint32t first,
	MSKint32t last,
	MSKint64t maxnumnz,
	MSKint64t * ptrb,
	MSKint64t * ptre,
	MSKint32t * sub,
	MSKrealt * val);

/** Obtains the number of non-zeros in a slice of rows of the coefficient matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `first` Index of the first row in the sequence.
  * - `last` Index of the last row plus one in the sequence.
  * - `numnz` Number of non-zeros in the slice.
  */
MSKrescodee MSKAPI MSK_getarowslicenumnz(
	MSKtask_t task,
	MSKint32t first,
	MSKint32t last,
	MSKint32t * numnz);

/** Obtains the number of non-zeros in a slice of rows of the coefficient matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `first` Index of the first row in the sequence.
  * - `last` Index of the last row plus one in the sequence.
  * - `numnz` Number of non-zeros in the slice.
  */
MSKrescodee MSKAPI MSK_getarowslicenumnz64(
	MSKtask_t task,
	MSKint32t first,
	MSKint32t last,
	MSKint64t * numnz);

/** Obtains a sequence of rows from the coefficient matrix in sparse triplet format.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `first` Index of the first row in the sequence.
  * - `last` Index of the last row in the sequence plus one.
  * - `maxnumnz` Denotes the length of the subscript and coefficient arrays.
  * - `subi` Constraint subscripts.
  * - `subj` Column subscripts.
  * - `val` Values.
  */
MSKrescodee MSKAPI MSK_getarowslicetrip(
	MSKtask_t task,
	MSKint32t first,
	MSKint32t last,
	MSKint64t maxnumnz,
	MSKint32t * subi,
	MSKint32t * subj,
	MSKrealt * val);

/** Obtains the A matrix in sparse triplet format.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumnz` Denotes the length of the subscript and coefficient arrays.
  * - `subi` Constraint subscripts.
  * - `subj` Column subscripts.
  * - `val` Values.
  */
MSKrescodee MSKAPI MSK_getatrip(
	MSKtask_t task,
	MSKint64t maxnumnz,
	MSKint32t * subi,
	MSKint32t * subj,
	MSKrealt * val);

/** Gets the current A matrix truncation threshold.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `tolzero` Truncation tolerance.
  */
MSKrescodee MSKAPI MSK_getatruncatetol(
	MSKtask_t task,
	MSKrealt * tolzero);

/** Obtains barA in block triplet form.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnum` subi, subj, subk, subl and valijkl must be maxnum long.
  * - `num` Number of elements in the block triplet form.
  * - `subi` Constraint index.
  * - `subj` Symmetric matrix variable index.
  * - `subk` Block row index.
  * - `subl` Block column index.
  * - `valijkl` The numerical value associated with each block triplet.
  */
MSKrescodee MSKAPI MSK_getbarablocktriplet(
	MSKtask_t task,
	MSKint64t maxnum,
	MSKint64t * num,
	MSKint32t * subi,
	MSKint32t * subj,
	MSKint32t * subk,
	MSKint32t * subl,
	MSKrealt * valijkl);

/** Obtains information about an element in barA.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `idx` Position of the element in the vectorized form.
  * - `maxnum` sub and weights must be at least maxnum long.
  * - `i` Row index of the element at position idx.
  * - `j` Column index of the element at position idx.
  * - `num` Number of terms in weighted sum that forms the element.
  * - `sub` A list indexes of the elements from symmetric matrix storage that appear in the weighted sum.
  * - `weights` The weights associated with each term in the weighted sum.
  */
MSKrescodee MSKAPI MSK_getbaraidx(
	MSKtask_t task,
	MSKint64t idx,
	MSKint64t maxnum,
	MSKint32t * i,
	MSKint32t * j,
	MSKint64t * num,
	MSKint64t * sub,
	MSKrealt * weights);

/** Obtains information about an element in barA.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `idx` Position of the element in the vectorized form.
  * - `i` Row index of the element at position idx.
  * - `j` Column index of the element at position idx.
  */
MSKrescodee MSKAPI MSK_getbaraidxij(
	MSKtask_t task,
	MSKint64t idx,
	MSKint32t * i,
	MSKint32t * j);

/** Obtains the number of terms in the weighted sum that form a particular element in barA.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `idx` The internal position of the element for which information should be obtained.
  * - `num` Number of terms in the weighted sum that form the specified element in barA.
  */
MSKrescodee MSKAPI MSK_getbaraidxinfo(
	MSKtask_t task,
	MSKint64t idx,
	MSKint64t * num);

/** Obtains the sparsity pattern of the barA matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumnz` The array idxij must be at least maxnumnz long.
  * - `numnz` Number of nonzero elements in barA.
  * - `idxij` Position of each nonzero element in the vector representation of barA.
  */
MSKrescodee MSKAPI MSK_getbarasparsity(
	MSKtask_t task,
	MSKint64t maxnumnz,
	MSKint64t * numnz,
	MSKint64t * idxij);

/** Obtains barC in block triplet form.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnum` subj, subk, subl and valjkl must be maxnum long.
  * - `num` Number of elements in the block triplet form.
  * - `subj` Symmetric matrix variable index.
  * - `subk` Block row index.
  * - `subl` Block column index.
  * - `valjkl` The numerical value associated with each block triplet.
  */
MSKrescodee MSKAPI MSK_getbarcblocktriplet(
	MSKtask_t task,
	MSKint64t maxnum,
	MSKint64t * num,
	MSKint32t * subj,
	MSKint32t * subk,
	MSKint32t * subl,
	MSKrealt * valjkl);

/** Obtains information about an element in barc.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `idx` Index of the element for which information should be obtained.
  * - `maxnum` sub and weights must be at least maxnum long.
  * - `j` Row index in barc.
  * - `num` Number of terms in the weighted sum.
  * - `sub` Elements appearing the weighted sum.
  * - `weights` Weights of terms in the weighted sum.
  */
MSKrescodee MSKAPI MSK_getbarcidx(
	MSKtask_t task,
	MSKint64t idx,
	MSKint64t maxnum,
	MSKint32t * j,
	MSKint64t * num,
	MSKint64t * sub,
	MSKrealt * weights);

/** Obtains information about an element in barc.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `idx` Index of the element for which information should be obtained. The value is an index of a symmetric sparse variable.
  * - `num` Number of terms that appear in the weighted sum that forms the requested element.
  */
MSKrescodee MSKAPI MSK_getbarcidxinfo(
	MSKtask_t task,
	MSKint64t idx,
	MSKint64t * num);

/** Obtains the row index of an element in barc.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `idx` Index of the element for which information should be obtained.
  * - `j` Row index in barc.
  */
MSKrescodee MSKAPI MSK_getbarcidxj(
	MSKtask_t task,
	MSKint64t idx,
	MSKint32t * j);

/** Get the positions of the nonzero elements in barc.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumnz` idxj must be at least maxnumnz long.
  * - `numnz` Number of nonzero elements in barc.
  * - `idxj` Internal positions of the nonzeros elements in barc.
  */
MSKrescodee MSKAPI MSK_getbarcsparsity(
	MSKtask_t task,
	MSKint64t maxnumnz,
	MSKint64t * numnz,
	MSKint64t * idxj);

/** Obtains the dual solution for a semidefinite variable.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `j` Index of the semidefinite variable.
  * - `barsj` Value of the j'th dual variable of barx.
  */
MSKrescodee MSKAPI MSK_getbarsj(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t j,
	MSKrealt * barsj);

/** Obtains the dual solution for a sequence of semidefinite variables.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `first` Index of the first semidefinite variable in the slice.
  * - `last` Index of the last semidefinite variable in the slice plus one.
  * - `slicesize` Denotes the length of the array barsslice.
  * - `barsslice` Dual solution values of symmetric matrix variables in the slice, stored sequentially.
  */
MSKrescodee MSKAPI MSK_getbarsslice(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t first,
	MSKint32t last,
	MSKint64t slicesize,
	MSKrealt * barsslice);

/** Obtains the name of a semidefinite variable.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `i` Index of the variable.
  * - `sizename` Length of the name buffer.
  * - `name` The requested name is copied to this buffer.
  */
MSKrescodee MSKAPI MSK_getbarvarname(
	MSKtask_t task,
	MSKint32t i,
	MSKint32t sizename,
	char * name);

/** Obtains the index of semidefinite variable from its name.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `somename` The name of the variable.
  * - `asgn` Non-zero if the name somename is assigned to some semidefinite variable.
  * - `index` The index of a semidefinite variable with the name somename (if one exists).
  */
MSKrescodee MSKAPI MSK_getbarvarnameindex(
	MSKtask_t task,
	const char * somename,
	MSKint32t * asgn,
	MSKint32t * index);

/** Obtains the length of the name of a semidefinite variable.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `i` Index of the variable.
  * - `len` Returns the length of the indicated name.
  */
MSKrescodee MSKAPI MSK_getbarvarnamelen(
	MSKtask_t task,
	MSKint32t i,
	MSKint32t * len);

/** Obtains the primal solution for a semidefinite variable.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `j` Index of the semidefinite variable.
  * - `barxj` Value of the j'th variable of barx.
  */
MSKrescodee MSKAPI MSK_getbarxj(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t j,
	MSKrealt * barxj);

/** Obtains the primal solution for a sequence of semidefinite variables.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `first` Index of the first semidefinite variable in the slice.
  * - `last` Index of the last semidefinite variable in the slice plus one.
  * - `slicesize` Denotes the length of the array barxslice.
  * - `barxslice` Solution values of symmetric matrix variables in the slice, stored sequentially.
  */
MSKrescodee MSKAPI MSK_getbarxslice(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t first,
	MSKint32t last,
	MSKint64t slicesize,
	MSKrealt * barxslice);

/** Obtains all objective coefficients.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `c` Linear terms of the objective as a dense vector. The length is the number of variables.
  */
MSKrescodee MSKAPI MSK_getc(
	MSKtask_t task,
	MSKrealt * c);

/** Obtains the callback function and the associated user handle.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `func` Returns the current user-defined callback function.
  * - `handle` The user-defined pointer associated with the user-defined callback function.
  */
MSKrescodee MSKAPI MSK_getcallbackfunc(
	MSKtask_t task,
	MSKcallbackfunc * func,
	MSKuserhandle_t * handle);

/** Obtains the fixed term in the objective.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `cfix` Fixed term in the objective.
  */
MSKrescodee MSKAPI MSK_getcfix(
	MSKtask_t task,
	MSKrealt * cfix);

/** Obtains one objective coefficient.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `j` Index of the variable for which the c coefficient should be obtained.
  * - `cj` The c coefficient value.
  */
MSKrescodee MSKAPI MSK_getcj(
	MSKtask_t task,
	MSKint32t j,
	MSKrealt * cj);

/** Obtains a sequence of coefficients from the objective.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of variables for which the c values should be obtained.
  * - `subj` A list of variable indexes.
  * - `c` Linear terms of the requested list of the objective as a dense vector.
  */
MSKrescodee MSKAPI MSK_getclist(
	MSKtask_t task,
	MSKint32t num,
	const MSKint32t * subj,
	MSKrealt * c);

/** Obtains bound information for one constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `i` Index of the constraint for which the bound information should be obtained.
  * - `bk` Bound keys.
  * - `bl` Values for lower bounds.
  * - `bu` Values for upper bounds.
  */
MSKrescodee MSKAPI MSK_getconbound(
	MSKtask_t task,
	MSKint32t i,
	MSKboundkeye * bk,
	MSKrealt * bl,
	MSKrealt * bu);

/** Obtains bounds information for a slice of the constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `bk` Bound keys.
  * - `bl` Values for lower bounds.
  * - `bu` Values for upper bounds.
  */
MSKrescodee MSKAPI MSK_getconboundslice(
	MSKtask_t task,
	MSKint32t first,
	MSKint32t last,
	MSKboundkeye * bk,
	MSKrealt * bl,
	MSKrealt * bu);

/** Obtains a cone.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `k` Index of the cone.
  * - `ct` Specifies the type of the cone.
  * - `conepar` For the power cone it denotes the exponent alpha. For other cone types it is unused and can be set to 0.
  * - `nummem` Number of member variables in the cone.
  * - `submem` Variable subscripts of the members in the cone.
  */
MSKrescodee MSKAPI MSK_getcone(
	MSKtask_t task,
	MSKint32t k,
	MSKconetypee * ct,
	MSKrealt * conepar,
	MSKint32t * nummem,
	MSKint32t * submem);

/** Obtains information about a cone.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `k` Index of the cone.
  * - `ct` Specifies the type of the cone.
  * - `conepar` For the power cone it denotes the exponent alpha. For other cone types it is unused and can be set to 0.
  * - `nummem` Number of member variables in the cone.
  */
MSKrescodee MSKAPI MSK_getconeinfo(
	MSKtask_t task,
	MSKint32t k,
	MSKconetypee * ct,
	MSKrealt * conepar,
	MSKint32t * nummem);

/** Obtains the name of a cone.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `i` Index of the cone.
  * - `sizename` Length of the name buffer.
  * - `name` The required name.
  */
MSKrescodee MSKAPI MSK_getconename(
	MSKtask_t task,
	MSKint32t i,
	MSKint32t sizename,
	char * name);

/** Checks whether the name has been assigned to any cone.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `somename` The name which should be checked.
  * - `asgn` Is non-zero if the name somename is assigned to some cone.
  * - `index` If the name somename is assigned to some cone, this is the index of the cone.
  */
MSKrescodee MSKAPI MSK_getconenameindex(
	MSKtask_t task,
	const char * somename,
	MSKint32t * asgn,
	MSKint32t * index);

/** Obtains the length of the name of a cone.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `i` Index of the cone.
  * - `len` Returns the length of the indicated name.
  */
MSKrescodee MSKAPI MSK_getconenamelen(
	MSKtask_t task,
	MSKint32t i,
	MSKint32t * len);

/** Obtains the name of a constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `i` Index of the constraint.
  * - `sizename` Length of the name buffer.
  * - `name` The required name.
  */
MSKrescodee MSKAPI MSK_getconname(
	MSKtask_t task,
	MSKint32t i,
	MSKint32t sizename,
	char * name);

/** Checks whether the name has been assigned to any constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `somename` The name which should be checked.
  * - `asgn` Is non-zero if the name somename is assigned to some constraint.
  * - `index` If the name somename is assigned to a constraint, then return the index of the constraint.
  */
MSKrescodee MSKAPI MSK_getconnameindex(
	MSKtask_t task,
	const char * somename,
	MSKint32t * asgn,
	MSKint32t * index);

/** Obtains the length of the name of a constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `i` Index of the constraint.
  * - `len` Returns the length of the indicated name.
  */
MSKrescodee MSKAPI MSK_getconnamelen(
	MSKtask_t task,
	MSKint32t i,
	MSKint32t * len);

/** Obtains a sequence of coefficients from the objective.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `c` Linear terms of the requested slice of the objective as a dense vector.
  */
MSKrescodee MSKAPI MSK_getcslice(
	MSKtask_t task,
	MSKint32t first,
	MSKint32t last,
	MSKrealt * c);

/** Obtains the dimension of a symmetric matrix variable.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `j` Index of the semidefinite variable whose dimension is requested.
  * - `dimbarvarj` The dimension of the j'th semidefinite variable.
  */
MSKrescodee MSKAPI MSK_getdimbarvarj(
	MSKtask_t task,
	MSKint32t j,
	MSKint32t * dimbarvarj);

/** Obtains the list of affine expression indexes in a disjunctive constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `djcidx` Index of the disjunctive constraint.
  * - `afeidxlist` List of affine expression indexes.
  */
MSKrescodee MSKAPI MSK_getdjcafeidxlist(
	MSKtask_t task,
	MSKint64t djcidx,
	MSKint64t * afeidxlist);

/** Obtains the optional constant term vector of a disjunctive constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `djcidx` Index of the disjunctive constraint.
  * - `b` The vector b.
  */
MSKrescodee MSKAPI MSK_getdjcb(
	MSKtask_t task,
	MSKint64t djcidx,
	MSKrealt * b);

/** Obtains the list of domain indexes in a disjunctive constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `djcidx` Index of the disjunctive constraint.
  * - `domidxlist` List of term sizes.
  */
MSKrescodee MSKAPI MSK_getdjcdomainidxlist(
	MSKtask_t task,
	MSKint64t djcidx,
	MSKint64t * domidxlist);

/** Obtains the name of a disjunctive constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `djcidx` Index of a disjunctive constraint.
  * - `sizename` The length of the name buffer.
  * - `name` Returns the required name.
  */
MSKrescodee MSKAPI MSK_getdjcname(
	MSKtask_t task,
	MSKint64t djcidx,
	MSKint32t sizename,
	char * name);

/** Obtains the length of the name of a disjunctive constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `djcidx` Index of a disjunctive constraint.
  * - `len` Returns the length of the indicated name.
  */
MSKrescodee MSKAPI MSK_getdjcnamelen(
	MSKtask_t task,
	MSKint64t djcidx,
	MSKint32t * len);

/** Obtains the number of affine expressions in the disjunctive constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `djcidx` Index of the disjunctive constraint.
  * - `numafe` Number of affine expressions in the disjunctive constraint.
  */
MSKrescodee MSKAPI MSK_getdjcnumafe(
	MSKtask_t task,
	MSKint64t djcidx,
	MSKint64t * numafe);

/** Obtains the number of affine expressions in all disjunctive constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numafetot` Number of affine expressions in all disjunctive constraints.
  */
MSKrescodee MSKAPI MSK_getdjcnumafetot(
	MSKtask_t task,
	MSKint64t * numafetot);

/** Obtains the number of domains in the disjunctive constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `djcidx` Index of the disjunctive constraint.
  * - `numdomain` Number of domains in the disjunctive constraint.
  */
MSKrescodee MSKAPI MSK_getdjcnumdomain(
	MSKtask_t task,
	MSKint64t djcidx,
	MSKint64t * numdomain);

/** Obtains the number of domains in all disjunctive constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numdomaintot` Number of domains in all disjunctive constraints.
  */
MSKrescodee MSKAPI MSK_getdjcnumdomaintot(
	MSKtask_t task,
	MSKint64t * numdomaintot);

/** Obtains the number terms in the disjunctive constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `djcidx` Index of the disjunctive constraint.
  * - `numterm` Number of terms in the disjunctive constraint.
  */
MSKrescodee MSKAPI MSK_getdjcnumterm(
	MSKtask_t task,
	MSKint64t djcidx,
	MSKint64t * numterm);

/** Obtains the number of terms in all disjunctive constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numtermtot` Total number of terms in all disjunctive constraints.
  */
MSKrescodee MSKAPI MSK_getdjcnumtermtot(
	MSKtask_t task,
	MSKint64t * numtermtot);

/** Obtains full data of all disjunctive constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `domidxlist` The concatenation of index lists of domains appearing in all disjunctive constraints.
  * - `afeidxlist` The concatenation of index lists of affine expressions appearing in all disjunctive constraints.
  * - `b` The concatenation of vectors b appearing in all disjunctive constraints.
  * - `termsizelist` The concatenation of lists of term sizes appearing in all disjunctive constraints.
  * - `numterms` The number of terms in each of the disjunctive constraints.
  */
MSKrescodee MSKAPI MSK_getdjcs(
	MSKtask_t task,
	MSKint64t * domidxlist,
	MSKint64t * afeidxlist,
	MSKrealt * b,
	MSKint64t * termsizelist,
	MSKint64t * numterms);

/** Obtains the list of term sizes in a disjunctive constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `djcidx` Index of the disjunctive constraint.
  * - `termsizelist` List of term sizes.
  */
MSKrescodee MSKAPI MSK_getdjctermsizelist(
	MSKtask_t task,
	MSKint64t djcidx,
	MSKint64t * termsizelist);

/** Obtains the dimension of the domain.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `domidx` Index of the domain.
  * - `n` Dimension of the domain.
  */
MSKrescodee MSKAPI MSK_getdomainn(
	MSKtask_t task,
	MSKint64t domidx,
	MSKint64t * n);

/** Obtains the name of a domain.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `domidx` Index of a domain.
  * - `sizename` The length of the name buffer.
  * - `name` Returns the required name.
  */
MSKrescodee MSKAPI MSK_getdomainname(
	MSKtask_t task,
	MSKint64t domidx,
	MSKint32t sizename,
	char * name);

/** Obtains the length of the name of a domain.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `domidx` Index of a domain.
  * - `len` Returns the length of the indicated name.
  */
MSKrescodee MSKAPI MSK_getdomainnamelen(
	MSKtask_t task,
	MSKint64t domidx,
	MSKint32t * len);

/** Returns the type of the domain.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `domidx` Index of the domain.
  * - `domtype` The type of the domain.
  */
MSKrescodee MSKAPI MSK_getdomaintype(
	MSKtask_t task,
	MSKint64t domidx,
	MSKdomaintypee * domtype);

/** Obtains a double information item.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichdinf` Specifies a double information item.
  * - `dvalue` The value of the required double information item.
  */
MSKrescodee MSKAPI MSK_getdouinf(
	MSKtask_t task,
	MSKdinfiteme whichdinf,
	MSKrealt * dvalue);

/** Obtains a double parameter.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `param` Which parameter.
  * - `parvalue` Parameter value.
  */
MSKrescodee MSKAPI MSK_getdouparam(
	MSKtask_t task,
	MSKdparame param,
	MSKrealt * parvalue);

/** Computes the dual objective value associated with the solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `dualobj` Objective value corresponding to the dual solution.
  */
MSKrescodee MSKAPI MSK_getdualobj(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKrealt * dualobj);

/** Obtains the dual problem.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `dualtask` A new task containing the dualized problem.
  */
MSKrescodee MSKAPI MSK_getdualproblem(
	MSKtask_t task,
	MSKtask_t * dualtask);

/** Compute norms of the dual solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `nrmy` The norm of the y vector.
  * - `nrmslc` The norm of the slc vector.
  * - `nrmsuc` The norm of the suc vector.
  * - `nrmslx` The norm of the slx vector.
  * - `nrmsux` The norm of the sux vector.
  * - `nrmsnx` The norm of the snx vector.
  * - `nrmbars` The norm of the bars vector.
  */
MSKrescodee MSKAPI MSK_getdualsolutionnorms(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKrealt * nrmy,
	MSKrealt * nrmslc,
	MSKrealt * nrmsuc,
	MSKrealt * nrmslx,
	MSKrealt * nrmsux,
	MSKrealt * nrmsnx,
	MSKrealt * nrmbars);

/** Computes the violation of the dual solution for set of affine conic constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `numaccidx` Length of sub and viol.
  * - `accidxlist` An array of indexes of conic constraints.
  * - `viol` List of violations corresponding to sub.
  */
MSKrescodee MSKAPI MSK_getdviolacc(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint64t numaccidx,
	const MSKint64t * accidxlist,
	MSKrealt * viol);

/** Computes the violation of dual solution for a set of semidefinite variables.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `num` Length of sub and viol.
  * - `sub` An array of indexes of barx variables.
  * - `viol` List of violations corresponding to sub.
  */
MSKrescodee MSKAPI MSK_getdviolbarvar(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t num,
	const MSKint32t * sub,
	MSKrealt * viol);

/** Computes the violation of a dual solution associated with a set of constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `num` Length of sub and viol.
  * - `sub` An array of indexes of constraints.
  * - `viol` List of violations corresponding to sub.
  */
MSKrescodee MSKAPI MSK_getdviolcon(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t num,
	const MSKint32t * sub,
	MSKrealt * viol);

/** Computes the violation of a solution for set of dual conic constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `num` Length of sub and viol.
  * - `sub` An array of indexes of conic constraints.
  * - `viol` List of violations corresponding to sub.
  */
MSKrescodee MSKAPI MSK_getdviolcones(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t num,
	const MSKint32t * sub,
	MSKrealt * viol);

/** Computes the violation of a dual solution associated with a set of scalar variables.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `num` Length of sub and viol.
  * - `sub` An array of indexes of x variables.
  * - `viol` List of violations corresponding to sub.
  */
MSKrescodee MSKAPI MSK_getdviolvar(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t num,
	const MSKint32t * sub,
	MSKrealt * viol);

/** Obtains the environment used to create the task.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `env` The MOSEK environment.
  */
MSKrescodee MSKAPI MSK_getenv(
	MSKtask_t task,
	MSKenv_t * env);

/** Obtains the problem with integer variables fixed.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `fixedtask` A new task containing the problem with fixed variables.
  */
MSKrescodee MSKAPI MSK_getfixedproblem(
	MSKtask_t task,
	MSKtask_t * fixedtask);

/** Obtains an infeasible subproblem.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Which solution to use when determining the infeasible subproblem.
  * - `inftask` A new task containing the infeasible subproblem.
  */
MSKrescodee MSKAPI MSK_getinfeasiblesubproblem(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKtask_t * inftask);

/** Obtains the index of a named information item.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `inftype` Type of the information item.
  * - `infname` Name of the information item.
  * - `infindex` The item index.
  */
MSKrescodee MSKAPI MSK_getinfindex(
	MSKtask_t task,
	MSKinftypee inftype,
	const char * infname,
	MSKint32t * infindex);

/** Obtains the maximum index of an information item of a given type.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `inftype` Type of the information item.
  * - `infmax` The maximum index (plus 1) requested.
  */
MSKrescodee MSKAPI MSK_getinfmax(
	MSKtask_t task,
	MSKinftypee inftype,
	MSKint32t * infmax);

/** Obtains the name of an information item.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `inftype` Type of the information item.
  * - `whichinf` An information item.
  * - `infname` Name of the information item.
  */
MSKrescodee MSKAPI MSK_getinfname(
	MSKtask_t task,
	MSKinftypee inftype,
	MSKint32t whichinf,
	char * infname);

/** Obtains an integer information item.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichiinf` Specifies an integer information item.
  * - `ivalue` The value of the required integer information item.
  */
MSKrescodee MSKAPI MSK_getintinf(
	MSKtask_t task,
	MSKiinfiteme whichiinf,
	MSKint32t * ivalue);

/** Obtains an integer parameter.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `param` Which parameter.
  * - `parvalue` Parameter value.
  */
MSKrescodee MSKAPI MSK_getintparam(
	MSKtask_t task,
	MSKiparame param,
	MSKint32t * parvalue);

/** Obtains the last error code and error message reported in MOSEK.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `lastrescode` Returns the last error code reported in the task.
  * - `sizelastmsg` The length of the lastmsg buffer.
  * - `lastmsglen` Returns the length of the last error message reported in the task.
  * - `lastmsg` Returns the last error message reported in the task.
  */
MSKrescodee MSKAPI MSK_getlasterror(
	MSKtask_t task,
	MSKrescodee * lastrescode,
	MSKint32t sizelastmsg,
	MSKint32t * lastmsglen,
	char * lastmsg);

/** Obtains the last error code and error message reported in MOSEK.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `lastrescode` Returns the last error code reported in the task.
  * - `sizelastmsg` The length of the lastmsg buffer.
  * - `lastmsglen` Returns the length of the last error message reported in the task.
  * - `lastmsg` Returns the last error message reported in the task.
  */
MSKrescodee MSKAPI MSK_getlasterror64(
	MSKtask_t task,
	MSKrescodee * lastrescode,
	MSKint64t sizelastmsg,
	MSKint64t * lastmsglen,
	char * lastmsg);

/** Obtains the length of one semidefinite variable.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `j` Index of the semidefinite variable whose length if requested.
  * - `lenbarvarj` Number of scalar elements in the lower triangular part of the semidefinite variable.
  */
MSKrescodee MSKAPI MSK_getlenbarvarj(
	MSKtask_t task,
	MSKint32t j,
	MSKint64t * lenbarvarj);

/** Obtains a long integer information item.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichliinf` Specifies a long information item.
  * - `ivalue` The value of the required long integer information item.
  */
MSKrescodee MSKAPI MSK_getlintinf(
	MSKtask_t task,
	MSKliinfiteme whichliinf,
	MSKint64t * ivalue);

/** Obtains an integer parameter.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `param` Which parameter.
  * - `parvalue` Parameter value.
  */
MSKrescodee MSKAPI MSK_getlintparam(
	MSKtask_t task,
	MSKiparame param,
	MSKint64t * parvalue);

/** Obtains the maximum length (not including terminating zero character) of any objective, constraint, variable, domain or cone name.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxlen` The maximum length of any name.
  */
MSKrescodee MSKAPI MSK_getmaxnamelen(
	MSKtask_t task,
	MSKint32t * maxlen);

/** Obtains number of preallocated non-zeros in the linear constraint matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumanz` Number of preallocated non-zero linear matrix elements.
  */
MSKrescodee MSKAPI MSK_getmaxnumanz(
	MSKtask_t task,
	MSKint32t * maxnumanz);

/** Obtains number of preallocated non-zeros in the linear constraint matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumanz` Number of preallocated non-zero linear matrix elements.
  */
MSKrescodee MSKAPI MSK_getmaxnumanz64(
	MSKtask_t task,
	MSKint64t * maxnumanz);

/** Obtains maximum number of symmetric matrix variables for which space is currently preallocated.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumbarvar` Maximum number of symmetric matrix variables for which space is currently preallocated.
  */
MSKrescodee MSKAPI MSK_getmaxnumbarvar(
	MSKtask_t task,
	MSKint32t * maxnumbarvar);

/** Obtains the number of preallocated constraints in the optimization task.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumcon` Number of preallocated constraints in the optimization task.
  */
MSKrescodee MSKAPI MSK_getmaxnumcon(
	MSKtask_t task,
	MSKint32t * maxnumcon);

/** Obtains the number of preallocated cones in the optimization task.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumcone` Number of preallocated conic constraints in the optimization task.
  */
MSKrescodee MSKAPI MSK_getmaxnumcone(
	MSKtask_t task,
	MSKint32t * maxnumcone);

/** Obtains the number of preallocated non-zeros for all quadratic terms in objective and constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumqnz` Number of non-zero elements preallocated in quadratic coefficient matrices.
  */
MSKrescodee MSKAPI MSK_getmaxnumqnz(
	MSKtask_t task,
	MSKint32t * maxnumqnz);

/** Obtains the number of preallocated non-zeros for all quadratic terms in objective and constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumqnz` Number of non-zero elements preallocated in quadratic coefficient matrices.
  */
MSKrescodee MSKAPI MSK_getmaxnumqnz64(
	MSKtask_t task,
	MSKint64t * maxnumqnz);

/** Obtains the maximum number variables allowed.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumvar` Number of preallocated variables in the optimization task.
  */
MSKrescodee MSKAPI MSK_getmaxnumvar(
	MSKtask_t task,
	MSKint32t * maxnumvar);

/** Obtains information about the amount of memory used by a task.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `meminuse` Amount of memory currently used by the task.
  * - `maxmemuse` Maximum amount of memory used by the task until now.
  */
MSKrescodee MSKAPI MSK_getmemusagetask(
	MSKtask_t task,
	MSKint64t * meminuse,
	MSKint64t * maxmemuse);

/** Obtains a named double information item.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `infitemname` The name of a double information item.
  * - `dvalue` The value of the required double information item.
  */
MSKrescodee MSKAPI MSK_getnadouinf(
	MSKtask_t task,
	const char * infitemname,
	MSKrealt * dvalue);

/** Obtains a double parameter.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `paramname` Name of a parameter.
  * - `parvalue` Parameter value.
  */
MSKrescodee MSKAPI MSK_getnadouparam(
	MSKtask_t task,
	const char * paramname,
	MSKrealt * parvalue);

/** Obtains a named integer information item.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `infitemname` The name of an integer information item.
  * - `ivalue` The value of the required integer information item.
  */
MSKrescodee MSKAPI MSK_getnaintinf(
	MSKtask_t task,
	const char * infitemname,
	MSKint32t * ivalue);

/** Obtains an integer parameter.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `paramname` Name of a parameter.
  * - `parvalue` Parameter value.
  */
MSKrescodee MSKAPI MSK_getnaintparam(
	MSKtask_t task,
	const char * paramname,
	MSKint32t * parvalue);

/** Obtains a string parameter.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `paramname` Name of a parameter.
  * - `sizeparamname` Size of the name buffer.
  * - `len` Returns the length of the parameter value.
  * - `parvalue` Parameter value.
  */
MSKrescodee MSKAPI MSK_getnastrparam(
	MSKtask_t task,
	const char * paramname,
	MSKint32t sizeparamname,
	MSKint32t * len,
	char * parvalue);

/** Obtains the value of a named string parameter.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `paramname` Name of a parameter.
  * - `numaddchr` Extra capacity of the return string buffer.
  * - `value` Parameter value.
  */
MSKrescodee MSKAPI MSK_getnastrparamal(
	MSKtask_t task,
	const char * paramname,
	MSKint32t numaddchr,
	MSKstring_t * value);

/** Obtains the number of affine conic constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` The number of affine conic constraints.
  */
MSKrescodee MSKAPI MSK_getnumacc(
	MSKtask_t task,
	MSKint64t * num);

/** Obtains the number of affine expressions.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numafe` Number of affine expressions.
  */
MSKrescodee MSKAPI MSK_getnumafe(
	MSKtask_t task,
	MSKint64t * numafe);

/** Obtains the number of non-zeros in the coefficient matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numanz` Number of non-zero elements in the linear constraint matrix.
  */
MSKrescodee MSKAPI MSK_getnumanz(
	MSKtask_t task,
	MSKint32t * numanz);

/** Obtains the number of non-zeros in the coefficient matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numanz` Number of non-zero elements in the linear constraint matrix.
  */
MSKrescodee MSKAPI MSK_getnumanz64(
	MSKtask_t task,
	MSKint64t * numanz);

/** Obtains an upper bound on the number of scalar elements in the block triplet form of bara.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` An upper bound on the number of elements in the block triplet form of bara.
  */
MSKrescodee MSKAPI MSK_getnumbarablocktriplets(
	MSKtask_t task,
	MSKint64t * num);

/** Get the number of nonzero elements in barA.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `nz` The number of nonzero block elements in barA.
  */
MSKrescodee MSKAPI MSK_getnumbaranz(
	MSKtask_t task,
	MSKint64t * nz);

/** Obtains an upper bound on the number of elements in the block triplet form of barc.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` An upper bound on the number of elements in the block triplet form of barc.
  */
MSKrescodee MSKAPI MSK_getnumbarcblocktriplets(
	MSKtask_t task,
	MSKint64t * num);

/** Obtains the number of nonzero elements in barc.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `nz` The number of nonzero elements in barc.
  */
MSKrescodee MSKAPI MSK_getnumbarcnz(
	MSKtask_t task,
	MSKint64t * nz);

/** Obtains the number of semidefinite variables.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numbarvar` Number of semidefinite variables in the problem.
  */
MSKrescodee MSKAPI MSK_getnumbarvar(
	MSKtask_t task,
	MSKint32t * numbarvar);

/** Obtains the number of constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numcon` Number of constraints.
  */
MSKrescodee MSKAPI MSK_getnumcon(
	MSKtask_t task,
	MSKint32t * numcon);

/** Obtains the number of cones.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numcone` Number of conic constraints.
  */
MSKrescodee MSKAPI MSK_getnumcone(
	MSKtask_t task,
	MSKint32t * numcone);

/** Obtains the number of members in a cone.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `k` Index of the cone.
  * - `nummem` Number of member variables in the cone.
  */
MSKrescodee MSKAPI MSK_getnumconemem(
	MSKtask_t task,
	MSKint32t k,
	MSKint32t * nummem);

/** Obtains the number of disjunctive constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` The number of disjunctive constraints.
  */
MSKrescodee MSKAPI MSK_getnumdjc(
	MSKtask_t task,
	MSKint64t * num);

/** Obtain the number of domains defined.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numdomain` Number of domains in the task.
  */
MSKrescodee MSKAPI MSK_getnumdomain(
	MSKtask_t task,
	MSKint64t * numdomain);

/** Obtains the number of integer-constrained variables.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numintvar` Number of integer variables.
  */
MSKrescodee MSKAPI MSK_getnumintvar(
	MSKtask_t task,
	MSKint32t * numintvar);

/** Obtains the number of parameters of a given type.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `partype` Parameter type.
  * - `numparam` Returns the number of parameters of the requested type.
  */
MSKrescodee MSKAPI MSK_getnumparam(
	MSKtask_t task,
	MSKparametertypee partype,
	MSKint32t * numparam);

/** Obtains the number of non-zero quadratic terms in a constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `k` Index of the constraint for which the number of non-zero quadratic terms should be obtained.
  * - `numqcnz` Number of quadratic terms.
  */
MSKrescodee MSKAPI MSK_getnumqconknz(
	MSKtask_t task,
	MSKint32t k,
	MSKint32t * numqcnz);

/** Obtains the number of non-zero quadratic terms in a constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `k` Index of the constraint for which the number quadratic terms should be obtained.
  * - `numqcnz` Number of quadratic terms.
  */
MSKrescodee MSKAPI MSK_getnumqconknz64(
	MSKtask_t task,
	MSKint32t k,
	MSKint64t * numqcnz);

/** Obtains the number of non-zero quadratic terms in the objective.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numqonz` Number of non-zero elements in the quadratic objective terms.
  */
MSKrescodee MSKAPI MSK_getnumqobjnz(
	MSKtask_t task,
	MSKint32t * numqonz);

/** Obtains the number of non-zero quadratic terms in the objective.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numqonz` Number of non-zero elements in the quadratic objective terms.
  */
MSKrescodee MSKAPI MSK_getnumqobjnz64(
	MSKtask_t task,
	MSKint64t * numqonz);

/** Obtains the number of symmetric matrices stored.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` The number of symmetric sparse matrices.
  */
MSKrescodee MSKAPI MSK_getnumsymmat(
	MSKtask_t task,
	MSKint64t * num);

/** Obtains the number of variables.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numvar` Number of variables.
  */
MSKrescodee MSKAPI MSK_getnumvar(
	MSKtask_t task,
	MSKint32t * numvar);

/** Obtains the name assigned to the objective function.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `sizeobjname` Length of the objname buffer.
  * - `objname` Assigned the objective name.
  */
MSKrescodee MSKAPI MSK_getobjname(
	MSKtask_t task,
	MSKint32t sizeobjname,
	char * objname);

/** Obtains the length of the name assigned to the objective function.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `len` Assigned the length of the objective name.
  */
MSKrescodee MSKAPI MSK_getobjnamelen(
	MSKtask_t task,
	MSKint32t * len);

/** Gets the objective sense.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `sense` The returned objective sense.
  */
MSKrescodee MSKAPI MSK_getobjsense(
	MSKtask_t task,
	MSKobjsensee * sense);

/** Obtains the maximum index of a parameter of a given type.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `partype` Parameter type.
  * - `parammax` The maximum index (plus 1) of the given parameter type.
  */
MSKrescodee MSKAPI MSK_getparammax(
	MSKtask_t task,
	MSKparametertypee partype,
	MSKint32t * parammax);

/** Obtains the name of a parameter.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `partype` Parameter type.
  * - `param` Which parameter.
  * - `parname` Parameter name.
  */
MSKrescodee MSKAPI MSK_getparamname(
	MSKtask_t task,
	MSKparametertypee partype,
	MSKint32t param,
	char * parname);

/** Obtains the exponent vector of a power domain.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `domidx` Index of the domain.
  * - `alpha` The exponent vector of the domain.
  */
MSKrescodee MSKAPI MSK_getpowerdomainalpha(
	MSKtask_t task,
	MSKint64t domidx,
	MSKrealt * alpha);

/** Obtains structural information about a power domain.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `domidx` Index of the domain.
  * - `n` Dimension of the domain.
  * - `nleft` Number of variables on the left hand side.
  */
MSKrescodee MSKAPI MSK_getpowerdomaininfo(
	MSKtask_t task,
	MSKint64t domidx,
	MSKint64t * n,
	MSKint64t * nleft);

/** Computes the primal objective value for the desired solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `primalobj` Objective value corresponding to the primal solution.
  */
MSKrescodee MSKAPI MSK_getprimalobj(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKrealt * primalobj);

/** Compute norms of the primal solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `nrmxc` The norm of the xc vector.
  * - `nrmxx` The norm of the xx vector.
  * - `nrmbarx` The norm of the barX vector.
  */
MSKrescodee MSKAPI MSK_getprimalsolutionnorms(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKrealt * nrmxc,
	MSKrealt * nrmxx,
	MSKrealt * nrmbarx);

/** Obtains the problem type.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `probtype` The problem type.
  */
MSKrescodee MSKAPI MSK_getprobtype(
	MSKtask_t task,
	MSKproblemtypee * probtype);

/** Obtains the problem status.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `problemsta` Problem status.
  */
MSKrescodee MSKAPI MSK_getprosta(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKprostae * problemsta);

/** Computes the violation of a solution for set of affine conic constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `numaccidx` Length of sub and viol.
  * - `accidxlist` An array of indexes of conic constraints.
  * - `viol` List of violations corresponding to sub.
  */
MSKrescodee MSKAPI MSK_getpviolacc(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint64t numaccidx,
	const MSKint64t * accidxlist,
	MSKrealt * viol);

/** Computes the violation of a primal solution for a list of semidefinite variables.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `num` Length of sub and viol.
  * - `sub` An array of indexes of barX variables.
  * - `viol` List of violations corresponding to sub.
  */
MSKrescodee MSKAPI MSK_getpviolbarvar(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t num,
	const MSKint32t * sub,
	MSKrealt * viol);

/** Computes the violation of a primal solution associated to a constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `num` Length of sub and viol.
  * - `sub` An array of indexes of constraints.
  * - `viol` List of violations corresponding to sub.
  */
MSKrescodee MSKAPI MSK_getpviolcon(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t num,
	const MSKint32t * sub,
	MSKrealt * viol);

/** Computes the violation of a solution for set of conic constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `num` Length of sub and viol.
  * - `sub` An array of indexes of conic constraints.
  * - `viol` List of violations corresponding to sub.
  */
MSKrescodee MSKAPI MSK_getpviolcones(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t num,
	const MSKint32t * sub,
	MSKrealt * viol);

/** Computes the violation of a solution for set of disjunctive constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `numdjcidx` Length of sub and viol.
  * - `djcidxlist` An array of indexes of disjunctive constraints.
  * - `viol` List of violations corresponding to sub.
  */
MSKrescodee MSKAPI MSK_getpvioldjc(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint64t numdjcidx,
	const MSKint64t * djcidxlist,
	MSKrealt * viol);

/** Computes the violation of a primal solution for a list of scalar variables.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `num` Length of sub and viol.
  * - `sub` An array of indexes of x variables.
  * - `viol` List of violations corresponding to sub.
  */
MSKrescodee MSKAPI MSK_getpviolvar(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t num,
	const MSKint32t * sub,
	MSKrealt * viol);

/** Obtains all the quadratic terms in a constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `k` Which constraint.
  * - `maxnumqcnz` Length of the subscript and coefficient buffers.
  * - `numqcnz` Number of quadratic terms.
  * - `qcsubi` Row subscripts for quadratic constraint matrix.
  * - `qcsubj` Column subscripts for quadratic constraint matrix.
  * - `qcval` Quadratic constraint coefficient values.
  */
MSKrescodee MSKAPI MSK_getqconk(
	MSKtask_t task,
	MSKint32t k,
	MSKint32t maxnumqcnz,
	MSKint32t * numqcnz,
	MSKint32t * qcsubi,
	MSKint32t * qcsubj,
	MSKrealt * qcval);

/** Obtains all the quadratic terms in a constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `k` Which constraint.
  * - `maxnumqcnz` Length of the subscript and coefficient buffers.
  * - `numqcnz` Number of quadratic terms.
  * - `qcsubi` Row subscripts for quadratic constraint matrix.
  * - `qcsubj` Column subscripts for quadratic constraint matrix.
  * - `qcval` Quadratic constraint coefficient values.
  */
MSKrescodee MSKAPI MSK_getqconk64(
	MSKtask_t task,
	MSKint32t k,
	MSKint64t maxnumqcnz,
	MSKint64t * numqcnz,
	MSKint32t * qcsubi,
	MSKint32t * qcsubj,
	MSKrealt * qcval);

/** Obtains all the quadratic terms in the objective.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumqonz` Length of the subscript and coefficient arrays.
  * - `numqonz` Number of non-zero elements in the quadratic objective terms.
  * - `qosubi` Row subscripts for quadratic objective coefficients.
  * - `qosubj` Column subscripts for quadratic objective coefficients.
  * - `qoval` Quadratic objective coefficient values.
  */
MSKrescodee MSKAPI MSK_getqobj(
	MSKtask_t task,
	MSKint32t maxnumqonz,
	MSKint32t * numqonz,
	MSKint32t * qosubi,
	MSKint32t * qosubj,
	MSKrealt * qoval);

/** Obtains all the quadratic terms in the objective.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumqonz` Length of the subscript and coefficient arrays.
  * - `numqonz` Number of non-zero elements in the quadratic objective terms.
  * - `qosubi` Row subscripts for quadratic objective coefficients.
  * - `qosubj` Column subscripts for quadratic objective coefficients.
  * - `qoval` Quadratic objective coefficient values.
  */
MSKrescodee MSKAPI MSK_getqobj64(
	MSKtask_t task,
	MSKint64t maxnumqonz,
	MSKint64t * numqonz,
	MSKint32t * qosubi,
	MSKint32t * qosubj,
	MSKrealt * qoval);

/** Obtains one coefficient from the quadratic term of the objective
  *
  * #Arguments
  * - `task` An optimization task.
  * - `i` Row index of the coefficient.
  * - `j` Column index of coefficient.
  * - `qoij` The required coefficient.
  */
MSKrescodee MSKAPI MSK_getqobjij(
	MSKtask_t task,
	MSKint32t i,
	MSKint32t j,
	MSKrealt * qoij);

/** Obtains the reduced costs for a sequence of variables.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `first` The index of the first variable in the sequence.
  * - `last` The index of the last variable in the sequence plus 1.
  * - `redcosts` Returns the requested reduced costs.
  */
MSKrescodee MSKAPI MSK_getreducedcosts(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t first,
	MSKint32t last,
	MSKrealt * redcosts);

/** Obtains the status keys for the constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `skc` Status keys for the constraints.
  */
MSKrescodee MSKAPI MSK_getskc(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKstakeye * skc);

/** Obtains the status keys for a slice of the constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `skc` Status keys for the constraints.
  */
MSKrescodee MSKAPI MSK_getskcslice(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t first,
	MSKint32t last,
	MSKstakeye * skc);

/** Obtains the status keys for the conic constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `skn` Status keys for the conic constraints.
  */
MSKrescodee MSKAPI MSK_getskn(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKstakeye * skn);

/** Obtains the status keys for the scalar variables.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `skx` Status keys for the variables.
  */
MSKrescodee MSKAPI MSK_getskx(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKstakeye * skx);

/** Obtains the status keys for a slice of the scalar variables.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `skx` Status keys for the variables.
  */
MSKrescodee MSKAPI MSK_getskxslice(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t first,
	MSKint32t last,
	MSKstakeye * skx);

/** Obtains the slc vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `slc` Dual variables corresponding to the lower bounds on the constraints.
  */
MSKrescodee MSKAPI MSK_getslc(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKrealt * slc);

/** Obtains a slice of the slc vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `slc` Dual variables corresponding to the lower bounds on the constraints.
  */
MSKrescodee MSKAPI MSK_getslcslice(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t first,
	MSKint32t last,
	MSKrealt * slc);

/** Obtains the slx vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `slx` Dual variables corresponding to the lower bounds on the variables.
  */
MSKrescodee MSKAPI MSK_getslx(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKrealt * slx);

/** Obtains a slice of the slx vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `slx` Dual variables corresponding to the lower bounds on the variables.
  */
MSKrescodee MSKAPI MSK_getslxslice(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t first,
	MSKint32t last,
	MSKrealt * slx);

/** Obtains the snx vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `snx` Dual variables corresponding to the conic constraints on the variables.
  */
MSKrescodee MSKAPI MSK_getsnx(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKrealt * snx);

/** Obtains a slice of the snx vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `snx` Dual variables corresponding to the conic constraints on the variables.
  */
MSKrescodee MSKAPI MSK_getsnxslice(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t first,
	MSKint32t last,
	MSKrealt * snx);

/** Obtains the solution status.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `solutionsta` Solution status.
  */
MSKrescodee MSKAPI MSK_getsolsta(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKsolstae * solutionsta);

/** Obtains the complete solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `problemsta` Problem status.
  * - `solutionsta` Solution status.
  * - `skc` Status keys for the constraints.
  * - `skx` Status keys for the variables.
  * - `skn` Status keys for the conic constraints.
  * - `xc` Primal constraint solution.
  * - `xx` Primal variable solution.
  * - `y` Vector of dual variables corresponding to the constraints.
  * - `slc` Dual variables corresponding to the lower bounds on the constraints.
  * - `suc` Dual variables corresponding to the upper bounds on the constraints.
  * - `slx` Dual variables corresponding to the lower bounds on the variables.
  * - `sux` Dual variables corresponding to the upper bounds on the variables.
  * - `snx` Dual variables corresponding to the conic constraints on the variables.
  */
MSKrescodee MSKAPI MSK_getsolution(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKprostae * problemsta,
	MSKsolstae * solutionsta,
	MSKstakeye * skc,
	MSKstakeye * skx,
	MSKstakeye * skn,
	MSKrealt * xc,
	MSKrealt * xx,
	MSKrealt * y,
	MSKrealt * slc,
	MSKrealt * suc,
	MSKrealt * slx,
	MSKrealt * sux,
	MSKrealt * snx);

/** Obtains information about of a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `pobj` The primal objective value.
  * - `pviolcon` Maximal primal bound violation for a xc variable.
  * - `pviolvar` Maximal primal bound violation for a xx variable.
  * - `pviolbarvar` Maximal primal bound violation for a barx variable.
  * - `pviolcone` Maximal primal violation of the solution with respect to the conic constraints.
  * - `pviolitg` Maximal violation in the integer constraints.
  * - `dobj` Dual objective value.
  * - `dviolcon` Maximal dual bound violation for a xc variable.
  * - `dviolvar` Maximal dual bound violation for a xx variable.
  * - `dviolbarvar` Maximal dual bound violation for a bars variable.
  * - `dviolcone` Maximum violation of the dual solution in the dual conic constraints.
  */
MSKrescodee MSKAPI MSK_getsolutioninfo(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKrealt * pobj,
	MSKrealt * pviolcon,
	MSKrealt * pviolvar,
	MSKrealt * pviolbarvar,
	MSKrealt * pviolcone,
	MSKrealt * pviolitg,
	MSKrealt * dobj,
	MSKrealt * dviolcon,
	MSKrealt * dviolvar,
	MSKrealt * dviolbarvar,
	MSKrealt * dviolcone);

/** Obtains information about of a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `pobj` The primal objective value.
  * - `pviolcon` Maximal primal bound violation for a xc variable.
  * - `pviolvar` Maximal primal bound violation for a xx variable.
  * - `pviolbarvar` Maximal primal bound violation for a barx variable.
  * - `pviolcone` Maximal primal violation of the solution with respect to the conic constraints.
  * - `pviolacc` Maximal primal violation of the solution with respect to the affine conic constraints.
  * - `pvioldjc` Maximal primal violation of the solution with respect to the disjunctive constraints.
  * - `pviolitg` Maximal violation in the integer constraints.
  * - `dobj` Dual objective value.
  * - `dviolcon` Maximal dual bound violation for a xc variable.
  * - `dviolvar` Maximal dual bound violation for a xx variable.
  * - `dviolbarvar` Maximal dual bound violation for a bars variable.
  * - `dviolcone` Maximum violation of the dual solution in the dual conic constraints.
  * - `dviolacc` Maximum violation of the dual solution in the dual affine conic constraints.
  */
MSKrescodee MSKAPI MSK_getsolutioninfonew(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKrealt * pobj,
	MSKrealt * pviolcon,
	MSKrealt * pviolvar,
	MSKrealt * pviolbarvar,
	MSKrealt * pviolcone,
	MSKrealt * pviolacc,
	MSKrealt * pvioldjc,
	MSKrealt * pviolitg,
	MSKrealt * dobj,
	MSKrealt * dviolcon,
	MSKrealt * dviolvar,
	MSKrealt * dviolbarvar,
	MSKrealt * dviolcone,
	MSKrealt * dviolacc);

/** Obtains the complete solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `problemsta` Problem status.
  * - `solutionsta` Solution status.
  * - `skc` Status keys for the constraints.
  * - `skx` Status keys for the variables.
  * - `skn` Status keys for the conic constraints.
  * - `xc` Primal constraint solution.
  * - `xx` Primal variable solution.
  * - `y` Vector of dual variables corresponding to the constraints.
  * - `slc` Dual variables corresponding to the lower bounds on the constraints.
  * - `suc` Dual variables corresponding to the upper bounds on the constraints.
  * - `slx` Dual variables corresponding to the lower bounds on the variables.
  * - `sux` Dual variables corresponding to the upper bounds on the variables.
  * - `snx` Dual variables corresponding to the conic constraints on the variables.
  * - `doty` Dual variables corresponding to affine conic constraints.
  */
MSKrescodee MSKAPI MSK_getsolutionnew(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKprostae * problemsta,
	MSKsolstae * solutionsta,
	MSKstakeye * skc,
	MSKstakeye * skx,
	MSKstakeye * skn,
	MSKrealt * xc,
	MSKrealt * xx,
	MSKrealt * y,
	MSKrealt * slc,
	MSKrealt * suc,
	MSKrealt * slx,
	MSKrealt * sux,
	MSKrealt * snx,
	MSKrealt * doty);

/** Obtains a slice of the solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `solitem` Which part of the solution is required.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `values` The values of the requested solution elements.
  */
MSKrescodee MSKAPI MSK_getsolutionslice(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKsoliteme solitem,
	MSKint32t first,
	MSKint32t last,
	MSKrealt * values);

/** Gets a single symmetric matrix from the matrix store.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `idx` Index of the matrix to retrieve.
  * - `maxlen` Length of the output arrays subi, subj and valij.
  * - `subi` Row subscripts of the matrix non-zero elements.
  * - `subj` Column subscripts of the matrix non-zero elements.
  * - `valij` Coefficients of the matrix non-zero elements.
  */
MSKrescodee MSKAPI MSK_getsparsesymmat(
	MSKtask_t task,
	MSKint64t idx,
	MSKint64t maxlen,
	MSKint32t * subi,
	MSKint32t * subj,
	MSKrealt * valij);

/** Obtains the value of a string parameter.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `param` Which parameter.
  * - `maxlen` Length of the parvalue buffer.
  * - `len` The length of the parameter value.
  * - `parvalue` If this is not a null pointer, the parameter value is stored here.
  */
MSKrescodee MSKAPI MSK_getstrparam(
	MSKtask_t task,
	MSKsparame param,
	MSKint32t maxlen,
	MSKint32t * len,
	char * parvalue);

/** Obtains the value of a string parameter.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `param` Which parameter.
  * - `numaddchr` Extra capacity of the return string buffer.
  * - `value` Parameter value.
  */
MSKrescodee MSKAPI MSK_getstrparamal(
	MSKtask_t task,
	MSKsparame param,
	MSKint32t numaddchr,
	MSKstring_t * value);

/** Obtains the length of a string parameter.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `param` Which parameter.
  * - `len` The length of the parameter value.
  */
MSKrescodee MSKAPI MSK_getstrparamlen(
	MSKtask_t task,
	MSKsparame param,
	MSKint32t * len);

/** Obtains the suc vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `suc` Dual variables corresponding to the upper bounds on the constraints.
  */
MSKrescodee MSKAPI MSK_getsuc(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKrealt * suc);

/** Obtains a slice of the suc vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `suc` Dual variables corresponding to the upper bounds on the constraints.
  */
MSKrescodee MSKAPI MSK_getsucslice(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t first,
	MSKint32t last,
	MSKrealt * suc);

/** Obtains the sux vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `sux` Dual variables corresponding to the upper bounds on the variables.
  */
MSKrescodee MSKAPI MSK_getsux(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKrealt * sux);

/** Obtains a slice of the sux vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `sux` Dual variables corresponding to the upper bounds on the variables.
  */
MSKrescodee MSKAPI MSK_getsuxslice(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t first,
	MSKint32t last,
	MSKrealt * sux);

/** Obtains a cone type string identifier.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `i` Index.
  * - `sizevalue` The length of the value buffer.
  * - `name` Name of the i'th symbolic constant.
  * - `value` The corresponding value.
  */
MSKrescodee MSKAPI MSK_getsymbcon(
	MSKtask_t task,
	MSKint32t i,
	MSKint32t sizevalue,
	char * name,
	MSKint32t * value);

/** Obtains information about a matrix from the symmetric matrix storage.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `idx` Index of the matrix for which information is requested.
  * - `dim` Returns the dimension of the requested matrix.
  * - `nz` Returns the number of non-zeros in the requested matrix.
  * - `mattype` Returns the type of the requested matrix.
  */
MSKrescodee MSKAPI MSK_getsymmatinfo(
	MSKtask_t task,
	MSKint64t idx,
	MSKint32t * dim,
	MSKint64t * nz,
	MSKsymmattypee * mattype);

/** Obtains the task name.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `sizetaskname` Length of the taskname buffer.
  * - `taskname` Returns the task name.
  */
MSKrescodee MSKAPI MSK_gettaskname(
	MSKtask_t task,
	MSKint32t sizetaskname,
	char * taskname);

/** Obtains the length the task name.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `len` Returns the length of the task name.
  */
MSKrescodee MSKAPI MSK_gettasknamelen(
	MSKtask_t task,
	MSKint32t * len);

/** Obtains bound information for one variable.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `i` Index of the variable for which the bound information should be obtained.
  * - `bk` Bound keys.
  * - `bl` Values for lower bounds.
  * - `bu` Values for upper bounds.
  */
MSKrescodee MSKAPI MSK_getvarbound(
	MSKtask_t task,
	MSKint32t i,
	MSKboundkeye * bk,
	MSKrealt * bl,
	MSKrealt * bu);

/** Obtains bounds information for a slice of the variables.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `bk` Bound keys.
  * - `bl` Values for lower bounds.
  * - `bu` Values for upper bounds.
  */
MSKrescodee MSKAPI MSK_getvarboundslice(
	MSKtask_t task,
	MSKint32t first,
	MSKint32t last,
	MSKboundkeye * bk,
	MSKrealt * bl,
	MSKrealt * bu);

/** Obtains the name of a variable.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `j` Index of a variable.
  * - `sizename` The length of the name buffer.
  * - `name` Returns the required name.
  */
MSKrescodee MSKAPI MSK_getvarname(
	MSKtask_t task,
	MSKint32t j,
	MSKint32t sizename,
	char * name);

/** Checks whether the name has been assigned to any variable.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `somename` The name which should be checked.
  * - `asgn` Is non-zero if the name somename is assigned to a variable.
  * - `index` If the name somename is assigned to a variable, then return the index of the variable.
  */
MSKrescodee MSKAPI MSK_getvarnameindex(
	MSKtask_t task,
	const char * somename,
	MSKint32t * asgn,
	MSKint32t * index);

/** Obtains the length of the name of a variable.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `i` Index of a variable.
  * - `len` Returns the length of the indicated name.
  */
MSKrescodee MSKAPI MSK_getvarnamelen(
	MSKtask_t task,
	MSKint32t i,
	MSKint32t * len);

/** Gets the variable type of one variable.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `j` Index of the variable.
  * - `vartype` Variable type of variable index j.
  */
MSKrescodee MSKAPI MSK_getvartype(
	MSKtask_t task,
	MSKint32t j,
	MSKvariabletypee * vartype);

/** Obtains the variable type for one or more variables.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of variables for which the variable type should be obtained.
  * - `subj` A list of variable indexes.
  * - `vartype` Returns the variables types corresponding the variable indexes requested.
  */
MSKrescodee MSKAPI MSK_getvartypelist(
	MSKtask_t task,
	MSKint32t num,
	const MSKint32t * subj,
	MSKvariabletypee * vartype);

/** Obtains the xc vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `xc` Primal constraint solution.
  */
MSKrescodee MSKAPI MSK_getxc(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKrealt * xc);

/** Obtains a slice of the xc vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `xc` Primal constraint solution.
  */
MSKrescodee MSKAPI MSK_getxcslice(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t first,
	MSKint32t last,
	MSKrealt * xc);

/** Obtains the xx vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `xx` Primal variable solution.
  */
MSKrescodee MSKAPI MSK_getxx(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKrealt * xx);

/** Obtains a slice of the xx vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `xx` Primal variable solution.
  */
MSKrescodee MSKAPI MSK_getxxslice(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t first,
	MSKint32t last,
	MSKrealt * xx);

/** Obtains the y vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `y` Vector of dual variables corresponding to the constraints.
  */
MSKrescodee MSKAPI MSK_gety(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKrealt * y);

/** Obtains a slice of the y vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `y` Vector of dual variables corresponding to the constraints.
  */
MSKrescodee MSKAPI MSK_getyslice(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t first,
	MSKint32t last,
	MSKrealt * y);

/** Prints the infeasibility report to an output stream.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichstream` Index of the stream.
  * - `whichsol` Selects a solution.
  */
MSKrescodee MSKAPI MSK_infeasibilityreport(
	MSKtask_t task,
	MSKstreamtypee whichstream,
	MSKsoltypee whichsol);

/** Prepare a task for basis solver.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `basis` Returns the array of basis indexes.
  */
MSKrescodee MSKAPI MSK_initbasissolve(
	MSKtask_t task,
	MSKint32t * basis);

/** Input the linear part of an optimization task in one function call.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumcon` Number of preallocated constraints in the optimization task.
  * - `maxnumvar` Number of preallocated variables in the optimization task.
  * - `numcon` Number of constraints.
  * - `numvar` Number of variables.
  * - `c` Linear terms of the objective as a dense vector. The length is the number of variables.
  * - `cfix` Fixed term in the objective.
  * - `aptrb` Row or column start pointers.
  * - `aptre` Row or column end pointers.
  * - `asub` Coefficient subscripts.
  * - `aval` Coefficient values.
  * - `bkc` Bound keys for the constraints.
  * - `blc` Lower bounds for the constraints.
  * - `buc` Upper bounds for the constraints.
  * - `bkx` Bound keys for the variables.
  * - `blx` Lower bounds for the variables.
  * - `bux` Upper bounds for the variables.
  */
MSKrescodee MSKAPI MSK_inputdata(
	MSKtask_t task,
	MSKint32t maxnumcon,
	MSKint32t maxnumvar,
	MSKint32t numcon,
	MSKint32t numvar,
	const MSKrealt * c,
	MSKrealt cfix,
	const MSKint32t * aptrb,
	const MSKint32t * aptre,
	const MSKint32t * asub,
	const MSKrealt * aval,
	const MSKboundkeye * bkc,
	const MSKrealt * blc,
	const MSKrealt * buc,
	const MSKboundkeye * bkx,
	const MSKrealt * blx,
	const MSKrealt * bux);

/** Input the linear part of an optimization task in one function call.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumcon` Number of preallocated constraints in the optimization task.
  * - `maxnumvar` Number of preallocated variables in the optimization task.
  * - `numcon` Number of constraints.
  * - `numvar` Number of variables.
  * - `c` Linear terms of the objective as a dense vector. The length is the number of variables.
  * - `cfix` Fixed term in the objective.
  * - `aptrb` Row or column start pointers.
  * - `aptre` Row or column end pointers.
  * - `asub` Coefficient subscripts.
  * - `aval` Coefficient values.
  * - `bkc` Bound keys for the constraints.
  * - `blc` Lower bounds for the constraints.
  * - `buc` Upper bounds for the constraints.
  * - `bkx` Bound keys for the variables.
  * - `blx` Lower bounds for the variables.
  * - `bux` Upper bounds for the variables.
  */
MSKrescodee MSKAPI MSK_inputdata64(
	MSKtask_t task,
	MSKint32t maxnumcon,
	MSKint32t maxnumvar,
	MSKint32t numcon,
	MSKint32t numvar,
	const MSKrealt * c,
	MSKrealt cfix,
	const MSKint64t * aptrb,
	const MSKint64t * aptre,
	const MSKint32t * asub,
	const MSKrealt * aval,
	const MSKboundkeye * bkc,
	const MSKrealt * blc,
	const MSKrealt * buc,
	const MSKboundkeye * bkx,
	const MSKrealt * blx,
	const MSKrealt * bux);

/** Checks a double parameter name.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `parname` Parameter name.
  * - `param` Returns the parameter corresponding to the name, if one exists.
  */
MSKrescodee MSKAPI MSK_isdouparname(
	MSKtask_t task,
	const char * parname,
	MSKdparame * param);

/** Checks an integer parameter name.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `parname` Parameter name.
  * - `param` Returns the parameter corresponding to the name, if one exists.
  */
MSKrescodee MSKAPI MSK_isintparname(
	MSKtask_t task,
	const char * parname,
	MSKiparame * param);

/** Checks a string parameter name.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `parname` Parameter name.
  * - `param` Returns the parameter corresponding to the name, if one exists.
  */
MSKrescodee MSKAPI MSK_isstrparname(
	MSKtask_t task,
	const char * parname,
	MSKsparame * param);

/** Directs all output from a task stream to a file.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichstream` Index of the stream.
  * - `filename` A valid file name.
  * - `append` If this argument is 0 the output file will be overwritten, otherwise it will be appended to.
  */
MSKrescodee MSKAPI MSK_linkfiletotaskstream(
	MSKtask_t task,
	MSKstreamtypee whichstream,
	const char * filename,
	MSKint32t append);

/** Connects a user-defined function to a task stream.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichstream` Index of the stream.
  * - `handle` Pointer to a user-defined structure.
  */
MSKrescodee MSKAPI MSK_linkfunctotaskstream(
	MSKtask_t task,
	MSKstreamtypee whichstream,
	MSKuserhandle_t handle,
	MSKstreamfunc func);

/** Prints a short summary of a specified solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichstream` Index of the stream.
  * - `whichsol` Selects a solution.
  */
MSKrescodee MSKAPI MSK_onesolutionsummary(
	MSKtask_t task,
	MSKstreamtypee whichstream,
	MSKsoltypee whichsol);

/** Optimizes the problem.
  *
  * #Arguments
  * - `task` An optimization task.
  */
MSKrescodee MSKAPI MSK_optimize(
	MSKtask_t task);

/** Offload the optimization task to a solver server and wait for the solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `address` Address of the OptServer.
  * - `accesstoken` Access token.
  * - `trmcode` Is either OK or a termination response code.
  */
MSKrescodee MSKAPI MSK_optimizermt(
	MSKtask_t task,
	const char * address,
	const char * accesstoken,
	MSKrescodee * trmcode);

/** Prints a short summary with optimizer statistics from last optimization.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichstream` Index of the stream.
  */
MSKrescodee MSKAPI MSK_optimizersummary(
	MSKtask_t task,
	MSKstreamtypee whichstream);

/** Optimizes the problem.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `trmcode` Is either OK or a termination response code.
  */
MSKrescodee MSKAPI MSK_optimizetrm(
	MSKtask_t task,
	MSKrescodee * trmcode);

/** Repairs a primal infeasible optimization problem by adjusting the bounds on the constraints and variables.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `wlc` Weights associated with relaxing lower bounds on the constraints.
  * - `wuc` Weights associated with relaxing the upper bound on the constraints.
  * - `wlx` Weights associated with relaxing the lower bounds of the variables.
  * - `wux` Weights associated with relaxing the upper bounds of variables.
  */
MSKrescodee MSKAPI MSK_primalrepair(
	MSKtask_t task,
	const MSKrealt * wlc,
	const MSKrealt * wuc,
	const MSKrealt * wlx,
	const MSKrealt * wux);

/** Perform sensitivity analysis on bounds.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numi` Number constraint bounds to analyze.
  * - `subi` Indexes of constraints to analyze.
  * - `marki` Mark which constraint bounds to analyze.
  * - `numj` Number of variable bounds to analyze.
  * - `subj` Indexes of variables to analyze.
  * - `markj` Mark which variable bounds to analyze.
  * - `leftpricei` Left shadow price for constraints.
  * - `rightpricei` Right shadow price for constraints.
  * - `leftrangei` Left range for constraints.
  * - `rightrangei` Right range for constraints.
  * - `leftpricej` Left shadow price for variables.
  * - `rightpricej` Right shadow price for variables.
  * - `leftrangej` Left range for variables.
  * - `rightrangej` Right range for variables.
  */
MSKrescodee MSKAPI MSK_primalsensitivity(
	MSKtask_t task,
	MSKint32t numi,
	const MSKint32t * subi,
	const MSKmarke * marki,
	MSKint32t numj,
	const MSKint32t * subj,
	const MSKmarke * markj,
	MSKrealt * leftpricei,
	MSKrealt * rightpricei,
	MSKrealt * leftrangei,
	MSKrealt * rightrangei,
	MSKrealt * leftpricej,
	MSKrealt * rightpricej,
	MSKrealt * leftrangej,
	MSKrealt * rightrangej);

/** Prints the current parameter settings.
  *
  * #Arguments
  * - `task` An optimization task.
  */
MSKrescodee MSKAPI MSK_printparam(
	MSKtask_t task);

/** Obtains a string containing the name of a given problem type.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `probtype` Problem type.
  * - `str` String corresponding to the problem type key.
  */
MSKrescodee MSKAPI MSK_probtypetostr(
	MSKtask_t task,
	MSKproblemtypee probtype,
	char * str);

/** Obtains a string containing the name of a given problem status.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `problemsta` Problem status.
  * - `str` String corresponding to the status key.
  */
MSKrescodee MSKAPI MSK_prostatostr(
	MSKtask_t task,
	MSKprostae problemsta,
	char * str);

/** Puts an affine conic constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `accidx` Affine conic constraint index.
  * - `domidx` Domain index.
  * - `numafeidx` Number of affine expressions in the affine expression list (must equal the dimension of the domain).
  * - `afeidxlist` List of affine expression indexes.
  * - `b` The vector of constant terms modifying affine expressions. Optional.
  */
MSKrescodee MSKAPI MSK_putacc(
	MSKtask_t task,
	MSKint64t accidx,
	MSKint64t domidx,
	MSKint64t numafeidx,
	const MSKint64t * afeidxlist,
	const MSKrealt * b);

/** Puts the constant vector b in an affine conic constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `accidx` Affine conic constraint index.
  * - `lengthb` Length of the vector (must equal the dimension of this constraint).
  * - `b` The vector of constant terms modifying affine expressions. Optional.
  */
MSKrescodee MSKAPI MSK_putaccb(
	MSKtask_t task,
	MSKint64t accidx,
	MSKint64t lengthb,
	const MSKrealt * b);

/** Sets one element in the b vector of an affine conic constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `accidx` Affine conic constraint index.
  * - `j` The index of an element in b to change.
  * - `bj` The new value of b[j].
  */
MSKrescodee MSKAPI MSK_putaccbj(
	MSKtask_t task,
	MSKint64t accidx,
	MSKint64t j,
	MSKrealt bj);

/** Puts the doty vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `accidx` The index of the affine conic constraint.
  * - `doty` The dual values for this affine conic constraint. The array should have length equal to the dimension of the constraint.
  */
MSKrescodee MSKAPI MSK_putaccdoty(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint64t accidx,
	MSKrealt * doty);

/** Puts a number of affine conic constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numaccs` The number of affine conic constraints to append.
  * - `accidxs` Affine conic constraint indices.
  * - `domidxs` Domain indices.
  * - `numafeidx` Number of affine expressions in the affine expression list (must equal the sum of dimensions of the domains).
  * - `afeidxlist` List of affine expression indexes.
  * - `b` The vector of constant terms modifying affine expressions. Optional.
  */
MSKrescodee MSKAPI MSK_putacclist(
	MSKtask_t task,
	MSKint64t numaccs,
	const MSKint64t * accidxs,
	const MSKint64t * domidxs,
	MSKint64t numafeidx,
	const MSKint64t * afeidxlist,
	const MSKrealt * b);

/** Sets the name of an affine conic constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `accidx` Index of the affine conic constraint.
  * - `name` The name of the affine conic constraint.
  */
MSKrescodee MSKAPI MSK_putaccname(
	MSKtask_t task,
	MSKint64t accidx,
	const char * name);

/** Replaces all elements in one column of the linear constraint matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `j` Column index.
  * - `nzj` Number of non-zeros in column.
  * - `subj` Row indexes of non-zero values in column.
  * - `valj` New non-zero values of column.
  */
MSKrescodee MSKAPI MSK_putacol(
	MSKtask_t task,
	MSKint32t j,
	MSKint32t nzj,
	const MSKint32t * subj,
	const MSKrealt * valj);

/** Replaces all elements in several columns the linear constraint matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of columns of A matrix to replace.
  * - `sub` Indexes of columns that should be replaced.
  * - `ptrb` Array of pointers to the first element in the columns.
  * - `ptre` Array of pointers to the last element plus one in the columns.
  * - `asub` Row indexes
  * - `aval` Coefficient values.
  */
MSKrescodee MSKAPI MSK_putacollist(
	MSKtask_t task,
	MSKint32t num,
	const MSKint32t * sub,
	const MSKint32t * ptrb,
	const MSKint32t * ptre,
	const MSKint32t * asub,
	const MSKrealt * aval);

/** Replaces all elements in several columns the linear constraint matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of columns of A matrix to replace.
  * - `sub` Indexes of columns that should be replaced.
  * - `ptrb` Array of pointers to the first element in the columns.
  * - `ptre` Array of pointers to the last element plus one in the columns.
  * - `asub` Row indexes
  * - `aval` Coefficient values.
  */
MSKrescodee MSKAPI MSK_putacollist64(
	MSKtask_t task,
	MSKint32t num,
	const MSKint32t * sub,
	const MSKint64t * ptrb,
	const MSKint64t * ptre,
	const MSKint32t * asub,
	const MSKrealt * aval);

/** Replaces all elements in a sequence of columns the linear constraint matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `first` First column in the slice.
  * - `last` Last column plus one in the slice.
  * - `ptrb` Array of pointers to the first element in the columns.
  * - `ptre` Array of pointers to the last element plus one in the columns.
  * - `asub` Row indexes
  * - `aval` Coefficient values.
  */
MSKrescodee MSKAPI MSK_putacolslice(
	MSKtask_t task,
	MSKint32t first,
	MSKint32t last,
	const MSKint32t * ptrb,
	const MSKint32t * ptre,
	const MSKint32t * asub,
	const MSKrealt * aval);

/** Replaces all elements in a sequence of columns the linear constraint matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `first` First column in the slice.
  * - `last` Last column plus one in the slice.
  * - `ptrb` Array of pointers to the first element in the columns.
  * - `ptre` Array of pointers to the last element plus one in the columns.
  * - `asub` Row indexes
  * - `aval` Coefficient values.
  */
MSKrescodee MSKAPI MSK_putacolslice64(
	MSKtask_t task,
	MSKint32t first,
	MSKint32t last,
	const MSKint64t * ptrb,
	const MSKint64t * ptre,
	const MSKint32t * asub,
	const MSKrealt * aval);

/** Inputs barF in block triplet form.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numtrip` Number of elements in the block triplet form.
  * - `afeidx` Constraint index.
  * - `barvaridx` Symmetric matrix variable index.
  * - `subk` Block row index.
  * - `subl` Block column index.
  * - `valkl` The numerical value associated with each block triplet.
  */
MSKrescodee MSKAPI MSK_putafebarfblocktriplet(
	MSKtask_t task,
	MSKint64t numtrip,
	const MSKint64t * afeidx,
	const MSKint32t * barvaridx,
	const MSKint32t * subk,
	const MSKint32t * subl,
	const MSKrealt * valkl);

/** Inputs one entry in barF.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `afeidx` Row index of barF.
  * - `barvaridx` Semidefinite variable index.
  * - `numterm` Number of terms in the weighted sum.
  * - `termidx` Element indices in matrix storage.
  * - `termweight` Weights in the weighted sum.
  */
MSKrescodee MSKAPI MSK_putafebarfentry(
	MSKtask_t task,
	MSKint64t afeidx,
	MSKint32t barvaridx,
	MSKint64t numterm,
	const MSKint64t * termidx,
	const MSKrealt * termweight);

/** Inputs a list of entries in barF.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numafeidx` Number of elements in the list.
  * - `afeidx` Row indexes of barF.
  * - `barvaridx` Semidefinite variable indexes.
  * - `numterm` Number of terms in the weighted sums.
  * - `ptrterm` Pointer to the terms forming each entry.
  * - `lenterm` Length of the index and weight lists.
  * - `termidx` Concatenated element indexes in matrix storage.
  * - `termweight` Concatenated weights in the weighted sum.
  */
MSKrescodee MSKAPI MSK_putafebarfentrylist(
	MSKtask_t task,
	MSKint64t numafeidx,
	const MSKint64t * afeidx,
	const MSKint32t * barvaridx,
	const MSKint64t * numterm,
	const MSKint64t * ptrterm,
	MSKint64t lenterm,
	const MSKint64t * termidx,
	const MSKrealt * termweight);

/** Inputs a row of barF.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `afeidx` Row index of barF.
  * - `numentr` Number of entries in the row of F.
  * - `barvaridx` Semidefinite variable indexes.
  * - `numterm` Number of terms in the weighted sums.
  * - `ptrterm` Pointer to the terms forming each entry.
  * - `lenterm` Length of the index and weight lists.
  * - `termidx` Concatenated element indexes in matrix storage.
  * - `termweight` Concatenated weights in the weighted sum.
  */
MSKrescodee MSKAPI MSK_putafebarfrow(
	MSKtask_t task,
	MSKint64t afeidx,
	MSKint32t numentr,
	const MSKint32t * barvaridx,
	const MSKint64t * numterm,
	const MSKint64t * ptrterm,
	MSKint64t lenterm,
	const MSKint64t * termidx,
	const MSKrealt * termweight);

/** Replaces all elements in one column of the F matrix in the affine expressions.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `varidx` Column index.
  * - `numnz` Number of non-zeros in the column.
  * - `afeidx` Row indexes of non-zero values in the column.
  * - `val` New non-zero values in the column.
  */
MSKrescodee MSKAPI MSK_putafefcol(
	MSKtask_t task,
	MSKint32t varidx,
	MSKint64t numnz,
	const MSKint64t * afeidx,
	const MSKrealt * val);

/** Replaces one entry in F.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `afeidx` Row index in F.
  * - `varidx` Column index in F.
  * - `value` Value of the entry.
  */
MSKrescodee MSKAPI MSK_putafefentry(
	MSKtask_t task,
	MSKint64t afeidx,
	MSKint32t varidx,
	MSKrealt value);

/** Replaces a list of entries in F.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numentr` Number of entries.
  * - `afeidx` Row indices in F.
  * - `varidx` Column indices in F.
  * - `val` Values of the entries in F.
  */
MSKrescodee MSKAPI MSK_putafefentrylist(
	MSKtask_t task,
	MSKint64t numentr,
	const MSKint64t * afeidx,
	const MSKint32t * varidx,
	const MSKrealt * val);

/** Replaces all elements in one row of the F matrix in the affine expressions.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `afeidx` Row index.
  * - `numnz` Number of non-zeros in the row.
  * - `varidx` Column indexes of non-zero values in the row.
  * - `val` New non-zero values in the row.
  */
MSKrescodee MSKAPI MSK_putafefrow(
	MSKtask_t task,
	MSKint64t afeidx,
	MSKint32t numnz,
	const MSKint32t * varidx,
	const MSKrealt * val);

/** Replaces all elements in a number of rows of the F matrix in the affine expressions.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numafeidx` The number of rows.
  * - `afeidx` Row indices.
  * - `numnzrow` Number of non-zeros in each row.
  * - `ptrrow` Pointer to the first nonzero in each row.
  * - `lenidxval` Length of the arrays with nonzero indexes and values.
  * - `varidx` Column indexes of non-zero values.
  * - `val` New non-zero values in the rows.
  */
MSKrescodee MSKAPI MSK_putafefrowlist(
	MSKtask_t task,
	MSKint64t numafeidx,
	const MSKint64t * afeidx,
	const MSKint32t * numnzrow,
	const MSKint64t * ptrrow,
	MSKint64t lenidxval,
	const MSKint32t * varidx,
	const MSKrealt * val);

/** Replaces one element in the g vector in the affine expressions.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `afeidx` Row index.
  * - `g` New value for the element of g.
  */
MSKrescodee MSKAPI MSK_putafeg(
	MSKtask_t task,
	MSKint64t afeidx,
	MSKrealt g);

/** Replaces a list of elements in the g vector in the affine expressions.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numafeidx` Number of entries.
  * - `afeidx` Indices of entries in g.
  * - `g` New values for the elements of g.
  */
MSKrescodee MSKAPI MSK_putafeglist(
	MSKtask_t task,
	MSKint64t numafeidx,
	const MSKint64t * afeidx,
	const MSKrealt * g);

/** Modifies a slice of the vector g.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `slice` The slice of g as a dense vector.
  */
MSKrescodee MSKAPI MSK_putafegslice(
	MSKtask_t task,
	MSKint64t first,
	MSKint64t last,
	const MSKrealt * slice);

/** Changes a single value in the linear coefficient matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `i` Constraint (row) index.
  * - `j` Variable (column) index.
  * - `aij` New coefficient.
  */
MSKrescodee MSKAPI MSK_putaij(
	MSKtask_t task,
	MSKint32t i,
	MSKint32t j,
	MSKrealt aij);

/** Changes one or more coefficients in the linear constraint matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of coefficients that should be changed.
  * - `subi` Constraint (row) indices.
  * - `subj` Variable (column) indices.
  * - `valij` New coefficient values.
  */
MSKrescodee MSKAPI MSK_putaijlist(
	MSKtask_t task,
	MSKint32t num,
	const MSKint32t * subi,
	const MSKint32t * subj,
	const MSKrealt * valij);

/** Changes one or more coefficients in the linear constraint matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of coefficients that should be changed.
  * - `subi` Constraint (row) indices.
  * - `subj` Variable (column) indices.
  * - `valij` New coefficient values.
  */
MSKrescodee MSKAPI MSK_putaijlist64(
	MSKtask_t task,
	MSKint64t num,
	const MSKint32t * subi,
	const MSKint32t * subj,
	const MSKrealt * valij);

/** Replaces all elements in one row of the linear constraint matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `i` Row index.
  * - `nzi` Number of non-zeros in row.
  * - `subi` Column indexes of non-zero values in row.
  * - `vali` New non-zero values of row.
  */
MSKrescodee MSKAPI MSK_putarow(
	MSKtask_t task,
	MSKint32t i,
	MSKint32t nzi,
	const MSKint32t * subi,
	const MSKrealt * vali);

/** Replaces all elements in several rows of the linear constraint matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of rows of A matrix to replace.
  * - `sub` Indexes of rows or columns that should be replaced.
  * - `ptrb` Array of pointers to the first element in the rows.
  * - `ptre` Array of pointers to the last element plus one in the rows.
  * - `asub` Variable indexes.
  * - `aval` Coefficient values.
  */
MSKrescodee MSKAPI MSK_putarowlist(
	MSKtask_t task,
	MSKint32t num,
	const MSKint32t * sub,
	const MSKint32t * ptrb,
	const MSKint32t * ptre,
	const MSKint32t * asub,
	const MSKrealt * aval);

/** Replaces all elements in several rows of the linear constraint matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of rows of A matrix to replace.
  * - `sub` Indexes of rows or columns that should be replaced.
  * - `ptrb` Array of pointers to the first element in the rows.
  * - `ptre` Array of pointers to the last element plus one in the rows.
  * - `asub` Variable indexes.
  * - `aval` Coefficient values.
  */
MSKrescodee MSKAPI MSK_putarowlist64(
	MSKtask_t task,
	MSKint32t num,
	const MSKint32t * sub,
	const MSKint64t * ptrb,
	const MSKint64t * ptre,
	const MSKint32t * asub,
	const MSKrealt * aval);

/** Replaces all elements in several rows the linear constraint matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `first` First row in the slice.
  * - `last` Last row plus one in the slice.
  * - `ptrb` Array of pointers to the first element in the rows.
  * - `ptre` Array of pointers to the last element plus one in the rows.
  * - `asub` Column indexes of new elements.
  * - `aval` Coefficient values.
  */
MSKrescodee MSKAPI MSK_putarowslice(
	MSKtask_t task,
	MSKint32t first,
	MSKint32t last,
	const MSKint32t * ptrb,
	const MSKint32t * ptre,
	const MSKint32t * asub,
	const MSKrealt * aval);

/** Replaces all elements in several rows the linear constraint matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `first` First row in the slice.
  * - `last` Last row plus one in the slice.
  * - `ptrb` Array of pointers to the first element in the rows.
  * - `ptre` Array of pointers to the last element plus one in the rows.
  * - `asub` Column indexes of new elements.
  * - `aval` Coefficient values.
  */
MSKrescodee MSKAPI MSK_putarowslice64(
	MSKtask_t task,
	MSKint32t first,
	MSKint32t last,
	const MSKint64t * ptrb,
	const MSKint64t * ptre,
	const MSKint32t * asub,
	const MSKrealt * aval);

/** Truncates all elements in A below a certain tolerance to zero.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `tolzero` Truncation tolerance.
  */
MSKrescodee MSKAPI MSK_putatruncatetol(
	MSKtask_t task,
	MSKrealt tolzero);

/** Inputs barA in block triplet form.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of elements in the block triplet form.
  * - `subi` Constraint index.
  * - `subj` Symmetric matrix variable index.
  * - `subk` Block row index.
  * - `subl` Block column index.
  * - `valijkl` The numerical value associated with each block triplet.
  */
MSKrescodee MSKAPI MSK_putbarablocktriplet(
	MSKtask_t task,
	MSKint64t num,
	const MSKint32t * subi,
	const MSKint32t * subj,
	const MSKint32t * subk,
	const MSKint32t * subl,
	const MSKrealt * valijkl);

/** Inputs an element of barA.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `i` Row index of barA.
  * - `j` Column index of barA.
  * - `num` Number terms in the weighted sum.
  * - `sub` Element indexes in matrix storage.
  * - `weights` Weights in the weighted sum.
  */
MSKrescodee MSKAPI MSK_putbaraij(
	MSKtask_t task,
	MSKint32t i,
	MSKint32t j,
	MSKint64t num,
	const MSKint64t * sub,
	const MSKrealt * weights);

/** Inputs list of elements of barA.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of barA entries to add.
  * - `subi` Row index of barA.
  * - `subj` Column index of barA.
  * - `alphaptrb` Start entries for terms in the weighted sum.
  * - `alphaptre` End entries for terms in the weighted sum.
  * - `matidx` Element indexes in matrix storage.
  * - `weights` Weights in the weighted sum.
  */
MSKrescodee MSKAPI MSK_putbaraijlist(
	MSKtask_t task,
	MSKint32t num,
	const MSKint32t * subi,
	const MSKint32t * subj,
	const MSKint64t * alphaptrb,
	const MSKint64t * alphaptre,
	const MSKint64t * matidx,
	const MSKrealt * weights);

/** Replace a set of rows of barA
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of barA entries to add.
  * - `subi` Row indexes of barA.
  * - `ptrb` Start of rows in barA.
  * - `ptre` End of rows in barA.
  * - `subj` Column index of barA.
  * - `nummat` Number of entries in weighted sum of matrixes.
  * - `matidx` Matrix indexes for weighted sum of matrixes.
  * - `weights` Weights for weighted sum of matrixes.
  */
MSKrescodee MSKAPI MSK_putbararowlist(
	MSKtask_t task,
	MSKint32t num,
	const MSKint32t * subi,
	const MSKint64t * ptrb,
	const MSKint64t * ptre,
	const MSKint32t * subj,
	const MSKint64t * nummat,
	const MSKint64t * matidx,
	const MSKrealt * weights);

/** Inputs barC in block triplet form.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of elements in the block triplet form.
  * - `subj` Symmetric matrix variable index.
  * - `subk` Block row index.
  * - `subl` Block column index.
  * - `valjkl` The numerical value associated with each block triplet.
  */
MSKrescodee MSKAPI MSK_putbarcblocktriplet(
	MSKtask_t task,
	MSKint64t num,
	const MSKint32t * subj,
	const MSKint32t * subk,
	const MSKint32t * subl,
	const MSKrealt * valjkl);

/** Changes one element in barc.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `j` Index of the element in barc` that should be changed.
  * - `num` The number elements appearing in the sum that forms the j'th element of barc.
  * - `sub` sub is list of indexes of those symmetric matrices appearing in sum.
  * - `weights` The weights of the terms in the weighted sum.
  */
MSKrescodee MSKAPI MSK_putbarcj(
	MSKtask_t task,
	MSKint32t j,
	MSKint64t num,
	const MSKint64t * sub,
	const MSKrealt * weights);

/** Sets the dual solution for a semidefinite variable.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `j` Index of the semidefinite variable.
  * - `barsj` Value of the j'th variable of barx.
  */
MSKrescodee MSKAPI MSK_putbarsj(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t j,
	const MSKrealt * barsj);

/** Sets the name of a semidefinite variable.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `j` Index of the variable.
  * - `name` The variable name.
  */
MSKrescodee MSKAPI MSK_putbarvarname(
	MSKtask_t task,
	MSKint32t j,
	const char * name);

/** Sets the primal solution for a semidefinite variable.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `j` Index of the semidefinite variable.
  * - `barxj` Value of the j'th variable of barx.
  */
MSKrescodee MSKAPI MSK_putbarxj(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t j,
	const MSKrealt * barxj);

/** Input the progress callback function.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `handle` A pointer to user-defined structure.
  */
MSKrescodee MSKAPI MSK_putcallbackfunc(
	MSKtask_t task,
	MSKcallbackfunc func,
	MSKuserhandle_t handle);

/** Replaces the fixed term in the objective.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `cfix` Fixed term in the objective.
  */
MSKrescodee MSKAPI MSK_putcfix(
	MSKtask_t task,
	MSKrealt cfix);

/** Modifies one linear coefficient in the objective.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `j` Index of the variable whose objective coefficient should be changed.
  * - `cj` New coefficient value.
  */
MSKrescodee MSKAPI MSK_putcj(
	MSKtask_t task,
	MSKint32t j,
	MSKrealt cj);

/** Modifies a part of the linear objective coefficients.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of coefficients that should be changed.
  * - `subj` Indices of variables for which objective coefficients should be changed.
  * - `val` New numerical values for the objective coefficients that should be modified.
  */
MSKrescodee MSKAPI MSK_putclist(
	MSKtask_t task,
	MSKint32t num,
	const MSKint32t * subj,
	const MSKrealt * val);

/** Changes the bound for one constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `i` Index of the constraint.
  * - `bkc` New bound key.
  * - `blc` New lower bound.
  * - `buc` New upper bound.
  */
MSKrescodee MSKAPI MSK_putconbound(
	MSKtask_t task,
	MSKint32t i,
	MSKboundkeye bkc,
	MSKrealt blc,
	MSKrealt buc);

/** Changes the bounds of a list of constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of bounds that should be changed.
  * - `sub` List of constraint indexes.
  * - `bkc` Bound keys for the constraints.
  * - `blc` Lower bounds for the constraints.
  * - `buc` Upper bounds for the constraints.
  */
MSKrescodee MSKAPI MSK_putconboundlist(
	MSKtask_t task,
	MSKint32t num,
	const MSKint32t * sub,
	const MSKboundkeye * bkc,
	const MSKrealt * blc,
	const MSKrealt * buc);

/** Changes the bounds of a list of constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of bounds that should be changed.
  * - `sub` List of constraint indexes.
  * - `bkc` New bound key for all constraints in the list.
  * - `blc` New lower bound for all constraints in the list.
  * - `buc` New upper bound for all constraints in the list.
  */
MSKrescodee MSKAPI MSK_putconboundlistconst(
	MSKtask_t task,
	MSKint32t num,
	const MSKint32t * sub,
	MSKboundkeye bkc,
	MSKrealt blc,
	MSKrealt buc);

/** Changes the bounds for a slice of the constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `bkc` Bound keys for the constraints.
  * - `blc` Lower bounds for the constraints.
  * - `buc` Upper bounds for the constraints.
  */
MSKrescodee MSKAPI MSK_putconboundslice(
	MSKtask_t task,
	MSKint32t first,
	MSKint32t last,
	const MSKboundkeye * bkc,
	const MSKrealt * blc,
	const MSKrealt * buc);

/** Changes the bounds for a slice of the constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `bkc` New bound key for all constraints in the slice.
  * - `blc` New lower bound for all constraints in the slice.
  * - `buc` New upper bound for all constraints in the slice.
  */
MSKrescodee MSKAPI MSK_putconboundsliceconst(
	MSKtask_t task,
	MSKint32t first,
	MSKint32t last,
	MSKboundkeye bkc,
	MSKrealt blc,
	MSKrealt buc);

/** Replaces a conic constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `k` Index of the cone.
  * - `ct` Specifies the type of the cone.
  * - `conepar` For the power cone it denotes the exponent alpha. For other cone types it is unused and can be set to 0.
  * - `nummem` Number of member variables in the cone.
  * - `submem` Variable subscripts of the members in the cone.
  */
MSKrescodee MSKAPI MSK_putcone(
	MSKtask_t task,
	MSKint32t k,
	MSKconetypee ct,
	MSKrealt conepar,
	MSKint32t nummem,
	const MSKint32t * submem);

/** Sets the name of a cone.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `j` Index of the cone.
  * - `name` The name of the cone.
  */
MSKrescodee MSKAPI MSK_putconename(
	MSKtask_t task,
	MSKint32t j,
	const char * name);

/** Sets the name of a constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `i` Index of the constraint.
  * - `name` The name of the constraint.
  */
MSKrescodee MSKAPI MSK_putconname(
	MSKtask_t task,
	MSKint32t i,
	const char * name);

/** Sets the primal and dual solution information for a single constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `i` Index of the constraint.
  * - `whichsol` Selects a solution.
  * - `sk` Status key of the constraint.
  * - `x` Primal solution value of the constraint.
  * - `sl` Solution value of the dual variable associated with the lower bound.
  * - `su` Solution value of the dual variable associated with the upper bound.
  */
MSKrescodee MSKAPI MSK_putconsolutioni(
	MSKtask_t task,
	MSKint32t i,
	MSKsoltypee whichsol,
	MSKstakeye sk,
	MSKrealt x,
	MSKrealt sl,
	MSKrealt su);

/** Modifies a slice of the linear objective coefficients.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `first` First element in the slice of c.
  * - `last` Last element plus 1 of the slice in c to be changed.
  * - `slice` New numerical values for the objective coefficients that should be modified.
  */
MSKrescodee MSKAPI MSK_putcslice(
	MSKtask_t task,
	MSKint32t first,
	MSKint32t last,
	const MSKrealt * slice);

/** Inputs a disjunctive constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `djcidx` Index of the disjunctive constraint.
  * - `numdomidx` Number of domains.
  * - `domidxlist` List of domain indexes.
  * - `numafeidx` Number of affine expressions.
  * - `afeidxlist` List of affine expression indexes.
  * - `b` The vector of constant terms modifying affine expressions.
  * - `numterms` Number of terms in disjunctive constraint.
  * - `termsizelist` List of term sizes.
  */
MSKrescodee MSKAPI MSK_putdjc(
	MSKtask_t task,
	MSKint64t djcidx,
	MSKint64t numdomidx,
	const MSKint64t * domidxlist,
	MSKint64t numafeidx,
	const MSKint64t * afeidxlist,
	const MSKrealt * b,
	MSKint64t numterms,
	const MSKint64t * termsizelist);

/** Sets the name of a disjunctive constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `djcidx` Index of the disjunctive constraint.
  * - `name` The name of the disjunctive constraint.
  */
MSKrescodee MSKAPI MSK_putdjcname(
	MSKtask_t task,
	MSKint64t djcidx,
	const char * name);

/** Inputs a slice of disjunctive constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `idxfirst` Index of the first disjunctive constraint in the slice.
  * - `idxlast` Index of the last disjunctive constraint in the slice plus 1.
  * - `numdomidx` Number of domains.
  * - `domidxlist` List of domain indexes.
  * - `numafeidx` Number of affine expressions.
  * - `afeidxlist` List of affine expression indexes.
  * - `b` The vector of constant terms modifying affine expressions. Optional.
  * - `numterms` Number of terms in the disjunctive constraints.
  * - `termsizelist` List of term sizes.
  * - `termsindjc` Number of terms in each of the disjunctive constraints in the slice.
  */
MSKrescodee MSKAPI MSK_putdjcslice(
	MSKtask_t task,
	MSKint64t idxfirst,
	MSKint64t idxlast,
	MSKint64t numdomidx,
	const MSKint64t * domidxlist,
	MSKint64t numafeidx,
	const MSKint64t * afeidxlist,
	const MSKrealt * b,
	MSKint64t numterms,
	const MSKint64t * termsizelist,
	const MSKint64t * termsindjc);

/** Sets the name of a domain.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `domidx` Index of the domain.
  * - `name` The name of the domain.
  */
MSKrescodee MSKAPI MSK_putdomainname(
	MSKtask_t task,
	MSKint64t domidx,
	const char * name);

/** Sets a double parameter.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `param` Which parameter.
  * - `parvalue` Parameter value.
  */
MSKrescodee MSKAPI MSK_putdouparam(
	MSKtask_t task,
	MSKdparame param,
	MSKrealt parvalue);

/** Sets an integer parameter.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `param` Which parameter.
  * - `parvalue` Parameter value.
  */
MSKrescodee MSKAPI MSK_putintparam(
	MSKtask_t task,
	MSKiparame param,
	MSKint32t parvalue);

/** Sets an integer parameter.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `param` Which parameter.
  * - `parvalue` Parameter value.
  */
MSKrescodee MSKAPI MSK_putlintparam(
	MSKtask_t task,
	MSKiparame param,
	MSKint64t parvalue);

/** Sets the number of preallocated affine conic constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumacc` Number of preallocated affine conic constraints.
  */
MSKrescodee MSKAPI MSK_putmaxnumacc(
	MSKtask_t task,
	MSKint64t maxnumacc);

/** Sets the number of preallocated affine expressions in the optimization task.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumafe` Number of preallocated affine expressions.
  */
MSKrescodee MSKAPI MSK_putmaxnumafe(
	MSKtask_t task,
	MSKint64t maxnumafe);

/** Sets the number of preallocated non-zero entries in the linear coefficient matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumanz` New size of the storage reserved for storing the linear coefficient matrix.
  */
MSKrescodee MSKAPI MSK_putmaxnumanz(
	MSKtask_t task,
	MSKint64t maxnumanz);

/** Sets the number of preallocated symmetric matrix variables.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumbarvar` Number of preallocated symmetric matrix variables.
  */
MSKrescodee MSKAPI MSK_putmaxnumbarvar(
	MSKtask_t task,
	MSKint32t maxnumbarvar);

/** Sets the number of preallocated constraints in the optimization task.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumcon` Number of preallocated constraints in the optimization task.
  */
MSKrescodee MSKAPI MSK_putmaxnumcon(
	MSKtask_t task,
	MSKint32t maxnumcon);

/** Sets the number of preallocated constraints in the optimization task.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumcon` Number of preallocated constraints in the optimization task.
  */
MSKrescodee MSKAPI MSK_putmaxnumcon64(
	MSKtask_t task,
	MSKint64t maxnumcon);

/** Sets the number of preallocated conic constraints in the optimization task.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumcone` Number of preallocated conic constraints in the optimization task.
  */
MSKrescodee MSKAPI MSK_putmaxnumcone(
	MSKtask_t task,
	MSKint32t maxnumcone);

/** Sets the number of preallocated disjunctive constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumdjc` Number of preallocated disjunctive constraints in the task.
  */
MSKrescodee MSKAPI MSK_putmaxnumdjc(
	MSKtask_t task,
	MSKint64t maxnumdjc);

/** Sets the number of preallocated domains in the optimization task.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumdomain` Number of preallocated domains.
  */
MSKrescodee MSKAPI MSK_putmaxnumdomain(
	MSKtask_t task,
	MSKint64t maxnumdomain);

/** Sets the number of preallocated non-zero entries in quadratic terms.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumqnz` Number of non-zero elements preallocated in quadratic coefficient matrices.
  */
MSKrescodee MSKAPI MSK_putmaxnumqnz(
	MSKtask_t task,
	MSKint64t maxnumqnz);

/** Sets the number of preallocated variables in the optimization task.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumvar` Number of preallocated variables in the optimization task.
  */
MSKrescodee MSKAPI MSK_putmaxnumvar(
	MSKtask_t task,
	MSKint32t maxnumvar);

/** Sets the number of preallocated variables in the optimization task.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumvar` Number of preallocated variables in the optimization task.
  */
MSKrescodee MSKAPI MSK_putmaxnumvar64(
	MSKtask_t task,
	MSKint64t maxnumvar);

/** Sets a double parameter.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `paramname` Name of a parameter.
  * - `parvalue` Parameter value.
  */
MSKrescodee MSKAPI MSK_putnadouparam(
	MSKtask_t task,
	const char * paramname,
	MSKrealt parvalue);

/** Sets an integer parameter.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `paramname` Name of a parameter.
  * - `parvalue` Parameter value.
  */
MSKrescodee MSKAPI MSK_putnaintparam(
	MSKtask_t task,
	const char * paramname,
	MSKint32t parvalue);

/** Sets a string parameter.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `paramname` Name of a parameter.
  * - `parvalue` Parameter value.
  */
MSKrescodee MSKAPI MSK_putnastrparam(
	MSKtask_t task,
	const char * paramname,
	const char * parvalue);

/** Assigns a new name to the objective.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `objname` Name of the objective.
  */
MSKrescodee MSKAPI MSK_putobjname(
	MSKtask_t task,
	const char * objname);

/** Sets the objective sense.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `sense` The objective sense of the task
  */
MSKrescodee MSKAPI MSK_putobjsense(
	MSKtask_t task,
	MSKobjsensee sense);

/** Specify an OptServer for remote calls.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `host` A URL specifying the optimization server to be used.
  */
MSKrescodee MSKAPI MSK_putoptserverhost(
	MSKtask_t task,
	const char * host);

/** Modifies the value of parameter.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `parname` Parameter name.
  * - `parvalue` Parameter value.
  */
MSKrescodee MSKAPI MSK_putparam(
	MSKtask_t task,
	const char * parname,
	const char * parvalue);

/** Replaces all quadratic terms in constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numqcnz` Number of quadratic terms.
  * - `qcsubk` Constraint subscripts for quadratic coefficients.
  * - `qcsubi` Row subscripts for quadratic constraint matrix.
  * - `qcsubj` Column subscripts for quadratic constraint matrix.
  * - `qcval` Quadratic constraint coefficient values.
  */
MSKrescodee MSKAPI MSK_putqcon(
	MSKtask_t task,
	MSKint32t numqcnz,
	const MSKint32t * qcsubk,
	const MSKint32t * qcsubi,
	const MSKint32t * qcsubj,
	const MSKrealt * qcval);

/** Replaces all quadratic terms in a single constraint.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `k` The constraint in which the new quadratic elements are inserted.
  * - `numqcnz` Number of quadratic terms.
  * - `qcsubi` Row subscripts for quadratic constraint matrix.
  * - `qcsubj` Column subscripts for quadratic constraint matrix.
  * - `qcval` Quadratic constraint coefficient values.
  */
MSKrescodee MSKAPI MSK_putqconk(
	MSKtask_t task,
	MSKint32t k,
	MSKint32t numqcnz,
	const MSKint32t * qcsubi,
	const MSKint32t * qcsubj,
	const MSKrealt * qcval);

/** Replaces all quadratic terms in the objective.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `numqonz` Number of non-zero elements in the quadratic objective terms.
  * - `qosubi` Row subscripts for quadratic objective coefficients.
  * - `qosubj` Column subscripts for quadratic objective coefficients.
  * - `qoval` Quadratic objective coefficient values.
  */
MSKrescodee MSKAPI MSK_putqobj(
	MSKtask_t task,
	MSKint32t numqonz,
	const MSKint32t * qosubi,
	const MSKint32t * qosubj,
	const MSKrealt * qoval);

/** Replaces one coefficient in the quadratic term in the objective.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `i` Row index for the coefficient to be replaced.
  * - `j` Column index for the coefficient to be replaced.
  * - `qoij` The new coefficient value.
  */
MSKrescodee MSKAPI MSK_putqobjij(
	MSKtask_t task,
	MSKint32t i,
	MSKint32t j,
	MSKrealt qoij);

/** Inputs a user-defined error callback function.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `handle` Pointer to a user-defined data structure.
  */
MSKrescodee MSKAPI MSK_putresponsefunc(
	MSKtask_t task,
	MSKresponsefunc responsefunc,
	MSKuserhandle_t handle);

/** Sets the status keys for the constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `skc` Status keys for the constraints.
  */
MSKrescodee MSKAPI MSK_putskc(
	MSKtask_t task,
	MSKsoltypee whichsol,
	const MSKstakeye * skc);

/** Sets the status keys for a slice of the constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `skc` Status keys for the constraints.
  */
MSKrescodee MSKAPI MSK_putskcslice(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t first,
	MSKint32t last,
	const MSKstakeye * skc);

/** Sets the status keys for the scalar variables.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `skx` Status keys for the variables.
  */
MSKrescodee MSKAPI MSK_putskx(
	MSKtask_t task,
	MSKsoltypee whichsol,
	const MSKstakeye * skx);

/** Sets the status keys for a slice of the variables.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `skx` Status keys for the variables.
  */
MSKrescodee MSKAPI MSK_putskxslice(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t first,
	MSKint32t last,
	const MSKstakeye * skx);

/** Sets the slc vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `slc` Dual variables corresponding to the lower bounds on the constraints.
  */
MSKrescodee MSKAPI MSK_putslc(
	MSKtask_t task,
	MSKsoltypee whichsol,
	const MSKrealt * slc);

/** Sets a slice of the slc vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `slc` Dual variables corresponding to the lower bounds on the constraints.
  */
MSKrescodee MSKAPI MSK_putslcslice(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t first,
	MSKint32t last,
	const MSKrealt * slc);

/** Sets the slx vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `slx` Dual variables corresponding to the lower bounds on the variables.
  */
MSKrescodee MSKAPI MSK_putslx(
	MSKtask_t task,
	MSKsoltypee whichsol,
	const MSKrealt * slx);

/** Sets a slice of the slx vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `slx` Dual variables corresponding to the lower bounds on the variables.
  */
MSKrescodee MSKAPI MSK_putslxslice(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t first,
	MSKint32t last,
	const MSKrealt * slx);

/** Sets the snx vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `sux` Dual variables corresponding to the upper bounds on the variables.
  */
MSKrescodee MSKAPI MSK_putsnx(
	MSKtask_t task,
	MSKsoltypee whichsol,
	const MSKrealt * sux);

/** Sets a slice of the snx vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `snx` Dual variables corresponding to the conic constraints on the variables.
  */
MSKrescodee MSKAPI MSK_putsnxslice(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t first,
	MSKint32t last,
	const MSKrealt * snx);

/** Inserts a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `skc` Status keys for the constraints.
  * - `skx` Status keys for the variables.
  * - `skn` Status keys for the conic constraints.
  * - `xc` Primal constraint solution.
  * - `xx` Primal variable solution.
  * - `y` Vector of dual variables corresponding to the constraints.
  * - `slc` Dual variables corresponding to the lower bounds on the constraints.
  * - `suc` Dual variables corresponding to the upper bounds on the constraints.
  * - `slx` Dual variables corresponding to the lower bounds on the variables.
  * - `sux` Dual variables corresponding to the upper bounds on the variables.
  * - `snx` Dual variables corresponding to the conic constraints on the variables.
  */
MSKrescodee MSKAPI MSK_putsolution(
	MSKtask_t task,
	MSKsoltypee whichsol,
	const MSKstakeye * skc,
	const MSKstakeye * skx,
	const MSKstakeye * skn,
	const MSKrealt * xc,
	const MSKrealt * xx,
	const MSKrealt * y,
	const MSKrealt * slc,
	const MSKrealt * suc,
	const MSKrealt * slx,
	const MSKrealt * sux,
	const MSKrealt * snx);

/** Inserts a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `skc` Status keys for the constraints.
  * - `skx` Status keys for the variables.
  * - `skn` Status keys for the conic constraints.
  * - `xc` Primal constraint solution.
  * - `xx` Primal variable solution.
  * - `y` Vector of dual variables corresponding to the constraints.
  * - `slc` Dual variables corresponding to the lower bounds on the constraints.
  * - `suc` Dual variables corresponding to the upper bounds on the constraints.
  * - `slx` Dual variables corresponding to the lower bounds on the variables.
  * - `sux` Dual variables corresponding to the upper bounds on the variables.
  * - `snx` Dual variables corresponding to the conic constraints on the variables.
  * - `doty` Dual variables corresponding to affine conic constraints.
  */
MSKrescodee MSKAPI MSK_putsolutionnew(
	MSKtask_t task,
	MSKsoltypee whichsol,
	const MSKstakeye * skc,
	const MSKstakeye * skx,
	const MSKstakeye * skn,
	const MSKrealt * xc,
	const MSKrealt * xx,
	const MSKrealt * y,
	const MSKrealt * slc,
	const MSKrealt * suc,
	const MSKrealt * slx,
	const MSKrealt * sux,
	const MSKrealt * snx,
	const MSKrealt * doty);

/** Inputs the dual variable of a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `i` Index of the dual variable.
  * - `whichsol` Selects a solution.
  * - `y` Solution value of the dual variable.
  */
MSKrescodee MSKAPI MSK_putsolutionyi(
	MSKtask_t task,
	MSKint32t i,
	MSKsoltypee whichsol,
	MSKrealt y);

/** Sets a string parameter.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `param` Which parameter.
  * - `parvalue` Parameter value.
  */
MSKrescodee MSKAPI MSK_putstrparam(
	MSKtask_t task,
	MSKsparame param,
	const char * parvalue);

/** Sets the suc vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `suc` Dual variables corresponding to the upper bounds on the constraints.
  */
MSKrescodee MSKAPI MSK_putsuc(
	MSKtask_t task,
	MSKsoltypee whichsol,
	const MSKrealt * suc);

/** Sets a slice of the suc vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `suc` Dual variables corresponding to the upper bounds on the constraints.
  */
MSKrescodee MSKAPI MSK_putsucslice(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t first,
	MSKint32t last,
	const MSKrealt * suc);

/** Sets the sux vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `sux` Dual variables corresponding to the upper bounds on the variables.
  */
MSKrescodee MSKAPI MSK_putsux(
	MSKtask_t task,
	MSKsoltypee whichsol,
	const MSKrealt * sux);

/** Sets a slice of the sux vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `sux` Dual variables corresponding to the upper bounds on the variables.
  */
MSKrescodee MSKAPI MSK_putsuxslice(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t first,
	MSKint32t last,
	const MSKrealt * sux);

/** Assigns a new name to the task.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `taskname` Name assigned to the task.
  */
MSKrescodee MSKAPI MSK_puttaskname(
	MSKtask_t task,
	const char * taskname);

/** Changes the bounds for one variable.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `j` Index of the variable.
  * - `bkx` New bound key.
  * - `blx` New lower bound.
  * - `bux` New upper bound.
  */
MSKrescodee MSKAPI MSK_putvarbound(
	MSKtask_t task,
	MSKint32t j,
	MSKboundkeye bkx,
	MSKrealt blx,
	MSKrealt bux);

/** Changes the bounds of a list of variables.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of bounds that should be changed.
  * - `sub` List of variable indexes.
  * - `bkx` Bound keys for the variables.
  * - `blx` Lower bounds for the variables.
  * - `bux` Upper bounds for the variables.
  */
MSKrescodee MSKAPI MSK_putvarboundlist(
	MSKtask_t task,
	MSKint32t num,
	const MSKint32t * sub,
	const MSKboundkeye * bkx,
	const MSKrealt * blx,
	const MSKrealt * bux);

/** Changes the bounds of a list of variables.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of bounds that should be changed.
  * - `sub` List of variable indexes.
  * - `bkx` New bound key for all variables in the list.
  * - `blx` New lower bound for all variables in the list.
  * - `bux` New upper bound for all variables in the list.
  */
MSKrescodee MSKAPI MSK_putvarboundlistconst(
	MSKtask_t task,
	MSKint32t num,
	const MSKint32t * sub,
	MSKboundkeye bkx,
	MSKrealt blx,
	MSKrealt bux);

/** Changes the bounds for a slice of the variables.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `bkx` Bound keys for the variables.
  * - `blx` Lower bounds for the variables.
  * - `bux` Upper bounds for the variables.
  */
MSKrescodee MSKAPI MSK_putvarboundslice(
	MSKtask_t task,
	MSKint32t first,
	MSKint32t last,
	const MSKboundkeye * bkx,
	const MSKrealt * blx,
	const MSKrealt * bux);

/** Changes the bounds for a slice of the variables.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `bkx` New bound key for all variables in the slice.
  * - `blx` New lower bound for all variables in the slice.
  * - `bux` New upper bound for all variables in the slice.
  */
MSKrescodee MSKAPI MSK_putvarboundsliceconst(
	MSKtask_t task,
	MSKint32t first,
	MSKint32t last,
	MSKboundkeye bkx,
	MSKrealt blx,
	MSKrealt bux);

/** Sets the name of a variable.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `j` Index of the variable.
  * - `name` The variable name.
  */
MSKrescodee MSKAPI MSK_putvarname(
	MSKtask_t task,
	MSKint32t j,
	const char * name);

/** Sets the primal and dual solution information for a single variable.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `j` Index of the variable.
  * - `whichsol` Selects a solution.
  * - `sk` Status key of the variable.
  * - `x` Primal solution value of the variable.
  * - `sl` Solution value of the dual variable associated with the lower bound.
  * - `su` Solution value of the dual variable associated with the upper bound.
  * - `sn` Solution value of the dual variable associated with the conic constraint.
  */
MSKrescodee MSKAPI MSK_putvarsolutionj(
	MSKtask_t task,
	MSKint32t j,
	MSKsoltypee whichsol,
	MSKstakeye sk,
	MSKrealt x,
	MSKrealt sl,
	MSKrealt su,
	MSKrealt sn);

/** Sets the variable type of one variable.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `j` Index of the variable.
  * - `vartype` The new variable type.
  */
MSKrescodee MSKAPI MSK_putvartype(
	MSKtask_t task,
	MSKint32t j,
	MSKvariabletypee vartype);

/** Sets the variable type for one or more variables.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of variables for which the variable type should be set.
  * - `subj` A list of variable indexes for which the variable type should be changed.
  * - `vartype` A list of variable types.
  */
MSKrescodee MSKAPI MSK_putvartypelist(
	MSKtask_t task,
	MSKint32t num,
	const MSKint32t * subj,
	const MSKvariabletypee * vartype);

/** Sets the xc vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `xc` Primal constraint solution.
  */
MSKrescodee MSKAPI MSK_putxc(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKrealt * xc);

/** Sets a slice of the xc vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `xc` Primal constraint solution.
  */
MSKrescodee MSKAPI MSK_putxcslice(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t first,
	MSKint32t last,
	const MSKrealt * xc);

/** Sets the xx vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `xx` Primal variable solution.
  */
MSKrescodee MSKAPI MSK_putxx(
	MSKtask_t task,
	MSKsoltypee whichsol,
	const MSKrealt * xx);

/** Sets a slice of the xx vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `xx` Primal variable solution.
  */
MSKrescodee MSKAPI MSK_putxxslice(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t first,
	MSKint32t last,
	const MSKrealt * xx);

/** Sets the y vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `y` Vector of dual variables corresponding to the constraints.
  */
MSKrescodee MSKAPI MSK_puty(
	MSKtask_t task,
	MSKsoltypee whichsol,
	const MSKrealt * y);

/** Sets a slice of the y vector for a solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `first` First index in the sequence.
  * - `last` Last index plus 1 in the sequence.
  * - `y` Vector of dual variables corresponding to the constraints.
  */
MSKrescodee MSKAPI MSK_putyslice(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKint32t first,
	MSKint32t last,
	const MSKrealt * y);

/** Read a binary dump of the task solution and information items.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `filename` A valid file name.
  * - `compress` Data compression type.
  */
MSKrescodee MSKAPI MSK_readbsolution(
	MSKtask_t task,
	const char * filename,
	MSKcompresstypee compress);

/** Read a serialized task update from a file.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `filename` A valid file name.
  * - `compress` Data compression type.
  */
MSKrescodee MSKAPI MSK_readbupdate(
	MSKtask_t task,
	const char * filename,
	MSKcompresstypee compress);

/** Read a serialized task update from a handle using a reader function.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `handle` Handle for reading.
  * - `compress` Data compression type.
  */
MSKrescodee MSKAPI MSK_readbupdatehandle(
	MSKtask_t task,
	MSKhreadfunc hread,
	MSKuserhandle_t handle,
	MSKcompresstypee compress);

/** Reads problem data from a file.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `filename` A valid file name.
  */
MSKrescodee MSKAPI MSK_readdata(
	MSKtask_t task,
	const char * filename);

/** Reads problem data from a file.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `filename` A valid file name.
  */
MSKrescodee MSKAPI MSK_readdataautoformat(
	MSKtask_t task,
	const char * filename);

/** Reads problem data from a file.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `filename` A valid file name.
  * - `format` File data format.
  * - `compress` File compression type.
  */
MSKrescodee MSKAPI MSK_readdataformat(
	MSKtask_t task,
	const char * filename,
	MSKdataformate format,
	MSKcompresstypee compress);

/** Reads problem data from a handle.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `h` Handle for reading.
  * - `format` File data dormat.
  * - `compress` Data compression type.
  * - `path` 
  */
MSKrescodee MSKAPI MSK_readdatahandle(
	MSKtask_t task,
	MSKhreadfunc hread,
	MSKuserhandle_t h,
	MSKdataformate format,
	MSKcompresstypee compress,
	const char * path);

/** Reads a solution from a JSOL file.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `filename` A valid file name.
  */
MSKrescodee MSKAPI MSK_readjsonsol(
	MSKtask_t task,
	const char * filename);

/** Load task data from a string in JSON format.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `data` Problem data in text format.
  */
MSKrescodee MSKAPI MSK_readjsonstring(
	MSKtask_t task,
	const char * data);

/** Load task data from a string in LP format.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `data` Problem data in text format.
  */
MSKrescodee MSKAPI MSK_readlpstring(
	MSKtask_t task,
	const char * data);

/** Load task data from a string in OPF format.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `data` Problem data in text format.
  */
MSKrescodee MSKAPI MSK_readopfstring(
	MSKtask_t task,
	const char * data);

/** Reads a parameter file.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `filename` A valid file name.
  */
MSKrescodee MSKAPI MSK_readparamfile(
	MSKtask_t task,
	const char * filename);

/** Load task data from a string in PTF format.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `data` Problem data in text format.
  */
MSKrescodee MSKAPI MSK_readptfstring(
	MSKtask_t task,
	const char * data);

/** Reads a solution from a file.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `filename` A valid file name.
  */
MSKrescodee MSKAPI MSK_readsolution(
	MSKtask_t task,
	MSKsoltypee whichsol,
	const char * filename);

/** Read solution file in format determined by the filename
  *
  * #Arguments
  * - `task` An optimization task.
  * - `filename` A valid file name.
  */
MSKrescodee MSKAPI MSK_readsolutionfile(
	MSKtask_t task,
	const char * filename);

/** Prints information about last file read.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichstream` Index of the stream.
  */
MSKrescodee MSKAPI MSK_readsummary(
	MSKtask_t task,
	MSKstreamtypee whichstream);

/** Load task data from a file.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `filename` A valid file name.
  */
MSKrescodee MSKAPI MSK_readtask(
	MSKtask_t task,
	const char * filename);

/** Removes a number of symmetric matrices.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of symmetric matrices which should be removed.
  * - `subset` Indexes of symmetric matrices which should be removed.
  */
MSKrescodee MSKAPI MSK_removebarvars(
	MSKtask_t task,
	MSKint32t num,
	const MSKint32t * subset);

/** Removes a number of conic constraints from the problem.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of cones which should be removed.
  * - `subset` Indexes of cones which should be removed.
  */
MSKrescodee MSKAPI MSK_removecones(
	MSKtask_t task,
	MSKint32t num,
	const MSKint32t * subset);

/** Removes a number of constraints.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of constraints which should be removed.
  * - `subset` Indexes of constraints which should be removed.
  */
MSKrescodee MSKAPI MSK_removecons(
	MSKtask_t task,
	MSKint32t num,
	const MSKint32t * subset);

/** Removes a number of variables.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `num` Number of variables which should be removed.
  * - `subset` Indexes of variables which should be removed.
  */
MSKrescodee MSKAPI MSK_removevars(
	MSKtask_t task,
	MSKint32t num,
	const MSKint32t * subset);

/** Resets a double parameter to its default value.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `param` Which parameter.
  */
MSKrescodee MSKAPI MSK_resetdouparam(
	MSKtask_t task,
	MSKdparame param);

/** Resets an integer parameter to its default value.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `param` Which parameter.
  */
MSKrescodee MSKAPI MSK_resetintparam(
	MSKtask_t task,
	MSKiparame param);

/** Resets all parameter values.
  *
  * #Arguments
  * - `task` An optimization task.
  */
MSKrescodee MSKAPI MSK_resetparameters(
	MSKtask_t task);

/** Resets a string parameter to its defalt value.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `param` Which parameter.
  */
MSKrescodee MSKAPI MSK_resetstrparam(
	MSKtask_t task,
	MSKsparame param);

/** Resizes an optimization task.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `maxnumcon` New maximum number of constraints.
  * - `maxnumvar` New maximum number of variables.
  * - `maxnumcone` New maximum number of cones.
  * - `maxnumanz` New maximum number of linear non-zero elements.
  * - `maxnumqnz` New maximum number of quadratic non-zeros elements.
  */
MSKrescodee MSKAPI MSK_resizetask(
	MSKtask_t task,
	MSKint32t maxnumcon,
	MSKint32t maxnumvar,
	MSKint32t maxnumcone,
	MSKint64t maxnumanz,
	MSKint64t maxnumqnz);

/** Creates a sensitivity report.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichstream` Index of the stream.
  */
MSKrescodee MSKAPI MSK_sensitivityreport(
	MSKtask_t task,
	MSKstreamtypee whichstream);

/** Obtains a status key abbreviation.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `sk` A valid status key.
  * - `str` Abbreviation string corresponding to the status key.
  */
MSKrescodee MSKAPI MSK_sktostr(
	MSKtask_t task,
	MSKstakeye sk,
	char * str);

/** Obtains a solution status string.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `solutionsta` Solution status.
  * - `str` String corresponding to the solution status.
  */
MSKrescodee MSKAPI MSK_solstatostr(
	MSKtask_t task,
	MSKsolstae solutionsta,
	char * str);

/** Checks whether a solution is defined.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `isdef` Is non-zero if the requested solution is defined.
  */
MSKrescodee MSKAPI MSK_solutiondef(
	MSKtask_t task,
	MSKsoltypee whichsol,
	MSKbooleant * isdef);

/** Prints a short summary of the current solutions.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichstream` Index of the stream.
  */
MSKrescodee MSKAPI MSK_solutionsummary(
	MSKtask_t task,
	MSKstreamtypee whichstream);

/** Solve a linear equation system involving a basis matrix.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `transp` Controls which problem formulation is solved.
  * - `numnz` Input (number of non-zeros in right-hand side).
  * - `sub` Input (indexes of non-zeros in right-hand side) and output (indexes of non-zeros in solution vector).
  * - `val` Input (right-hand side values) and output (solution vector values).
  * - `numnzout` Output (number of non-zeros in solution vector).
  */
MSKrescodee MSKAPI MSK_solvewithbasis(
	MSKtask_t task,
	MSKbooleant transp,
	MSKint32t numnz,
	MSKint32t * sub,
	MSKrealt * val,
	MSKint32t * numnzout);

/** Obtains a cone type code.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `str` String corresponding to the cone type code.
  * - `conetype` The cone type corresponding to str.
  */
MSKrescodee MSKAPI MSK_strtoconetype(
	MSKtask_t task,
	const char * str,
	MSKconetypee * conetype);

/** Obtains a status key.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `str` A status key abbreviation string.
  * - `sk` Status key corresponding to the string.
  */
MSKrescodee MSKAPI MSK_strtosk(
	MSKtask_t task,
	const char * str,
	MSKstakeye * sk);

/** In-place reformulation of a QCQO to a conic quadratic problem.
  *
  * #Arguments
  * - `task` An optimization task.
  */
MSKrescodee MSKAPI MSK_toconic(
	MSKtask_t task);

/** In-place reformulation into a problem with integer variables fixed.
  *
  * #Arguments
  * - `task` An optimization task.
  */
MSKrescodee MSKAPI MSK_tofixedproblem(
	MSKtask_t task);

/** Disconnects a user-defined function from a task stream.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichstream` Index of the stream.
  */
MSKrescodee MSKAPI MSK_unlinkfuncfromtaskstream(
	MSKtask_t task,
	MSKstreamtypee whichstream);

/** Update the information items related to the solution.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  */
MSKrescodee MSKAPI MSK_updatesolutioninfo(
	MSKtask_t task,
	MSKsoltypee whichsol);

/** Checks a parameter name.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `parname` Parameter name.
  * - `partype` Parameter type.
  * - `param` Which parameter.
  */
MSKrescodee MSKAPI MSK_whichparam(
	MSKtask_t task,
	const char * parname,
	MSKparametertypee * partype,
	MSKint32t * param);

/** Write a binary dump of the task.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `filename` A valid file name.
  * - `compress` Data compression type.
  */
MSKrescodee MSKAPI MSK_writeb(
	MSKtask_t task,
	const char * filename,
	MSKcompresstypee compress);

/** Write a binary dump of the task solution and information items.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `filename` A valid file name.
  * - `compress` Data compression type.
  */
MSKrescodee MSKAPI MSK_writebsolution(
	MSKtask_t task,
	const char * filename,
	MSKcompresstypee compress);

/** Write a binary dump of the task solution and information items.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `handle` A user-defined handle.
  * - `compress` Data compression type.
  */
MSKrescodee MSKAPI MSK_writebsolutionhandle(
	MSKtask_t task,
	MSKhwritefunc func,
	MSKuserhandle_t handle,
	MSKcompresstypee compress);

/** Writes problem data to a file.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `filename` A valid file name.
  */
MSKrescodee MSKAPI MSK_writedata(
	MSKtask_t task,
	const char * filename);

/** Writes problem data to a user-defined handle.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `handle` A user-defined handle.
  * - `format` Selects data format.
  * - `compress` Selects compression type.
  */
MSKrescodee MSKAPI MSK_writedatahandle(
	MSKtask_t task,
	MSKhwritefunc func,
	MSKuserhandle_t handle,
	MSKdataformate format,
	MSKcompresstypee compress);

/** Writes a solution to a JSON file.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `filename` A valid file name.
  */
MSKrescodee MSKAPI MSK_writejsonsol(
	MSKtask_t task,
	const char * filename);

/** Writes all the parameters to a parameter file.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `filename` A valid file name.
  */
MSKrescodee MSKAPI MSK_writeparamfile(
	MSKtask_t task,
	const char * filename);

/** Write a solution to a file.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `whichsol` Selects a solution.
  * - `filename` A valid file name.
  */
MSKrescodee MSKAPI MSK_writesolution(
	MSKtask_t task,
	MSKsoltypee whichsol,
	const char * filename);

/** Write solution file in format determined by the filename
  *
  * #Arguments
  * - `task` An optimization task.
  * - `filename` A valid file name.
  */
MSKrescodee MSKAPI MSK_writesolutionfile(
	MSKtask_t task,
	const char * filename);

/** Write a complete binary dump of the task data.
  *
  * #Arguments
  * - `task` An optimization task.
  * - `filename` A valid file name.
  */
MSKrescodee MSKAPI MSK_writetask(
	MSKtask_t task,
	const char * filename);

/** Computes vector addition and multiplication by a scalar.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `n` Length of the vectors.
  * - `alpha` The scalar that multiplies x.
  * - `x` The x vector.
  * - `y` The y vector.
  */
MSKrescodee MSKAPI MSK_axpy(
	MSKenv_t env,
	MSKint32t n,
	MSKrealt alpha,
	const MSKrealt * x,
	MSKrealt * y);

/** Obtains a callback code string identifier.
  *
  * #Arguments
  * - `code` A callback code.
  * - `callbackcodestr` String corresponding to the callback code.
  */
MSKrescodee MSKAPI MSK_callbackcodetostr(
	MSKcallbackcodee code,
	char * callbackcodestr);

/** Debug version of the system calloc function.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `number` Number of elements.
  * - `size` Size of each individual element.
  * - `file` File from which the function is called.
  * - `line` Line in the file from which the function is called.
  */
void * MSKAPI MSK_callocdbgenv(
	MSKenv_t env,
	size_t number,
	size_t size,
	const char * file,
	unsigned line);

/** A replacement for the system calloc function.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `number` Number of elements.
  * - `size` Size of each individual element.
  */
void * MSKAPI MSK_callocenv(
	MSKenv_t env,
	size_t number,
	size_t size);

/** Check in all unused license features to the license token server.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  */
MSKrescodee MSKAPI MSK_checkinall(
	MSKenv_t env);

/** Check in a license feature back to the license server ahead of time.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `feature` Feature to check in to the license system.
  */
MSKrescodee MSKAPI MSK_checkinlicense(
	MSKenv_t env,
	MSKfeaturee feature);

/** Checks the memory allocated by the environment.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `file` File from which the function is called.
  * - `line` Line in the file from which the function is called.
  */
MSKrescodee MSKAPI MSK_checkmemenv(
	MSKenv_t env,
	const char * file,
	MSKint32t line);

/** Check out a license feature from the license server ahead of time.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `feature` Feature to check out from the license system.
  */
MSKrescodee MSKAPI MSK_checkoutlicense(
	MSKenv_t env,
	MSKfeaturee feature);

/** Compares a version of the MOSEK DLL with a specified version.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `major` Major version number.
  * - `minor` Minor version number.
  * - `revision` Revision number.
  */
MSKrescodee MSKAPI MSK_checkversion(
	MSKenv_t env,
	MSKint32t major,
	MSKint32t minor,
	MSKint32t revision);

/** Computes a Cholesky factorization of sparse matrix.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `numthreads` The number threads that can be used to do the computation. 0 means the code makes the choice.
  * - `ordermethod` If nonzero, then a sparsity preserving ordering will be employed.
  * - `tolsingular` A positive parameter controlling when a pivot is declared zero.
  * - `n` Specifies the order of A.
  * - `anzc` anzc[j] is the number of nonzeros in the jth column of A.
  * - `aptrc` aptrc[j] is a pointer to the first element in column j.
  * - `asubc` Row indexes for each column stored in increasing order.
  * - `avalc` The value corresponding to row indexed stored in asubc.
  * - `perm` Permutation array used to specify the permutation matrix P computed by the function.
  * - `diag` The diagonal elements of matrix D.
  * - `lnzc` lnzc[j] is the number of non zero elements in column j.
  * - `lptrc` lptrc[j] is a pointer to the first row index and value in column j.
  * - `lensubnval` Number of elements in lsubc and lvalc.
  * - `lsubc` Row indexes for each column stored in increasing order.
  * - `lvalc` The values corresponding to row indexed stored in lsubc.
  */
MSKrescodee MSKAPI MSK_computesparsecholesky(
	MSKenv_t env,
	MSKint32t numthreads,
	MSKint32t ordermethod,
	MSKrealt tolsingular,
	MSKint32t n,
	const MSKint32t * anzc,
	const MSKint64t * aptrc,
	const MSKint32t * asubc,
	const MSKrealt * avalc,
	MSKint32t * perm[],
	MSKrealt * diag[],
	MSKint32t * lnzc[],
	MSKint64t * lptrc[],
	MSKint64t * lensubnval,
	MSKint32t * lsubc[],
	MSKrealt * lvalc[]);

/** Delete a MOSEK environment.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  */
MSKrescodee MSKAPI MSK_deleteenv(
	MSKenv_t * env);

/** Obtains a information item string identifier.
  *
  * #Arguments
  * - `item` Information item.
  * - `str` String corresponding to the information item.
  */
MSKrescodee MSKAPI MSK_dinfitemtostr(
	MSKdinfiteme item,
	char * str);

/** Computes the inner product of two vectors.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `n` Length of the vectors.
  * - `x` The x vector.
  * - `y` The y vector.
  * - `xty` The result of the inner product.
  */
MSKrescodee MSKAPI MSK_dot(
	MSKenv_t env,
	MSKint32t n,
	const MSKrealt * x,
	const MSKrealt * y,
	MSKrealt * xty);

/** Prints a formatted message to the environment stream.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `whichstream` Index of the stream.
  * - `format` A valid printf-compatible format string
  * - `...` 
  */
MSKrescodee MSKAPIVA MSK_echoenv(
	MSKenv_t env,
	MSKstreamtypee whichstream,
	const char * format,
	...);

/** Prints an intro to message stream.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `longver` If non-zero, then the intro is slightly longer.
  */
MSKrescodee MSKAPI MSK_echointro(
	MSKenv_t env,
	MSKint32t longver);

/** Reports when the first license feature expires.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `expiry` If nonnegative, then it is the minimum number days to expiry of any feature that has been checked out.
  */
MSKrescodee MSKAPI MSK_expirylicenses(
	MSKenv_t env,
	MSKint64t * expiry);

/** Frees space allocated by MOSEK.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `buffer` A pointer.
  * - `file` File from which the function is called.
  * - `line` Line in the file from which the function is called.
  */
void MSKAPI MSK_freedbgenv(
	MSKenv_t env,
	void * buffer,
	const char * file,
	unsigned line);

/** Frees space allocated by MOSEK.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `buffer` A pointer.
  */
void MSKAPI MSK_freeenv(
	MSKenv_t env,
	void * buffer);

/** Performs a dense matrix multiplication.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `transa` Indicates whether the matrix A must be transposed.
  * - `transb` Indicates whether the matrix B must be transposed.
  * - `m` Indicates the number of rows of matrix C.
  * - `n` Indicates the number of columns of matrix C.
  * - `k` Specifies the common dimension along which op(A) and op(B) are multiplied.
  * - `alpha` A scalar value multiplying the result of the matrix multiplication.
  * - `a` The pointer to the array storing matrix A in a column-major format.
  * - `b` The pointer to the array storing matrix B in a column-major format.
  * - `beta` A scalar value that multiplies C.
  * - `c` The pointer to the array storing matrix C in a column-major format.
  */
MSKrescodee MSKAPI MSK_gemm(
	MSKenv_t env,
	MSKtransposee transa,
	MSKtransposee transb,
	MSKint32t m,
	MSKint32t n,
	MSKint32t k,
	MSKrealt alpha,
	const MSKrealt * a,
	const MSKrealt * b,
	MSKrealt beta,
	MSKrealt * c);

/** Computes dense matrix times a dense vector product.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `transa` Indicates whether the matrix A must be transposed.
  * - `m` Specifies the number of rows of the matrix A.
  * - `n` Specifies the number of columns of the matrix A.
  * - `alpha` A scalar value multiplying the matrix A.
  * - `a` A pointer to the array storing matrix A in a column-major format.
  * - `x` A pointer to the array storing the vector x.
  * - `beta` A scalar value multiplying the vector y.
  * - `y` A pointer to the array storing the vector y.
  */
MSKrescodee MSKAPI MSK_gemv(
	MSKenv_t env,
	MSKtransposee transa,
	MSKint32t m,
	MSKint32t n,
	MSKrealt alpha,
	const MSKrealt * a,
	const MSKrealt * x,
	MSKrealt beta,
	MSKrealt * y);

/** Obtains build information.
  *
  * #Arguments
  * - `buildstate` State of binaries, i.e. a debug, release candidate or final release.
  * - `builddate` Date when the binaries were built.
  */
MSKrescodee MSKAPI MSK_getbuildinfo(
	char * buildstate,
	char * builddate);

/** Obtains a short description of a response code.
  *
  * #Arguments
  * - `code` A valid response code.
  * - `symname` Symbolic name corresponding to the code.
  * - `str` Obtains a short description of a response code.
  */
MSKrescodee MSKAPI MSK_getcodedesc(
	MSKrescodee code,
	char * symname,
	char * str);

/** Obtain the class of a response code.
  *
  * #Arguments
  * - `r` A response code indicating the result of function call.
  * - `rc` The response class.
  */
MSKrescodee MSKAPI MSK_getresponseclass(
	MSKrescodee r,
	MSKrescodetypee * rc);

/** Obtains dimensional information for the defined symbolic constants.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `num` Returns the number of symbolic constants.
  * - `maxlen` Maximum length of the name of any symbolic constants.
  */
MSKrescodee MSKAPI MSK_getsymbcondim(
	MSKenv_t env,
	MSKint32t * num,
	size_t * maxlen);

/** Obtains MOSEK version information.
  *
  * #Arguments
  * - `major` Major version number.
  * - `minor` Minor version number.
  * - `revision` Revision number.
  */
MSKrescodee MSKAPI MSK_getversion(
	MSKint32t * major,
	MSKint32t * minor,
	MSKint32t * revision);

/** Finalize global env.
  */
MSKrescodee MSKAPI MSK_globalenvfinalize();

/** Initialize global env.
  *
  * #Arguments
  * - `maxnumalloc` If it is nonnegative then it is the maximum number of alloacations allowed.
  * - `dbgfile` A user-defined file debug file.
  */
MSKrescodee MSKAPI MSK_globalenvinitialize(
	MSKint64t maxnumalloc,
	const char * dbgfile);

/** Obtains a information item string identifier.
  *
  * #Arguments
  * - `item` Information item.
  * - `str` String corresponding to the information item.
  */
MSKrescodee MSKAPI MSK_iinfitemtostr(
	MSKiinfiteme item,
	char * str);

/** Obtains the symbolic name corresponding to a value that can be assigned to an integer parameter.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `whichparam` Which parameter.
  * - `whichvalue` Which value.
  * - `symbolicname` The symbolic name corresponding to the whichvalue argument.
  */
MSKrescodee MSKAPI MSK_iparvaltosymnam(
	MSKenv_t env,
	MSKiparame whichparam,
	MSKint32t whichvalue,
	char * symbolicname);

/** Return true if value is considered infinity by MOSEK.
  *
  * #Arguments
  * - `value` The value to be checked
  */
int MSKAPI MSK_isinfinity(
	MSKrealt value);

/** Stops all threads and delete all handles used by the license system.
  */
MSKrescodee MSKAPI MSK_licensecleanup();

/** Obtains a information item string identifier.
  *
  * #Arguments
  * - `item` Information item.
  * - `str` String corresponding to the information item.
  */
MSKrescodee MSKAPI MSK_liinfitemtostr(
	MSKliinfiteme item,
	char * str);

/** Directs all output from a stream to a file.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `whichstream` Index of the stream.
  * - `filename` A valid file name.
  * - `append` If this argument is 0 the file will be overwritten, otherwise it will be appended to.
  */
MSKrescodee MSKAPI MSK_linkfiletoenvstream(
	MSKenv_t env,
	MSKstreamtypee whichstream,
	const char * filename,
	MSKint32t append);

/** Connects a user-defined function to a stream.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `whichstream` Index of the stream.
  * - `handle` Pointer to a user-defined structure.
  */
MSKrescodee MSKAPI MSK_linkfunctoenvstream(
	MSKenv_t env,
	MSKstreamtypee whichstream,
	MSKuserhandle_t handle,
	MSKstreamfunc func);

/** Creates a new and empty optimization task.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `task` An optimization task.
  */
MSKrescodee MSKAPI MSK_makeemptytask(
	MSKenv_t env,
	MSKtask_t * task);

/** Creates a new MOSEK environment.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `dbgfile` A user-defined memory debug file.
  */
MSKrescodee MSKAPI MSK_makeenv(
	MSKenv_t * env,
	const char * dbgfile);

/** Creates a new task.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `maxnumcon` An optional estimate on the maximum number of constraints in the task.
  * - `maxnumvar` An optional estimate on the maximum number of variables in the task.
  * - `task` An optimization task.
  */
MSKrescodee MSKAPI MSK_maketask(
	MSKenv_t env,
	MSKint32t maxnumcon,
	MSKint32t maxnumvar,
	MSKtask_t * task);

/** Optimize a number of tasks in parallel using a specified number of threads.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `israce` If nonzero, then the function is terminated after the first task has been completed.
  * - `maxtime` Time limit for the function.
  * - `numthreads` Number of threads to be employed.
  * - `numtask` Number of tasks to optimize.
  * - `task` An array of tasks to optimize in parallel.
  * - `trmcode` The termination code for each task.
  * - `rcode` The response code for each task.
  */
MSKrescodee MSKAPI MSK_optimizebatch(
	MSKenv_t env,
	MSKbooleant israce,
	MSKrealt maxtime,
	MSKint32t numthreads,
	MSKint64t numtask,
	const MSKtask_t * task,
	MSKrescodee * trmcode,
	MSKrescodee * rcode);

/** Computes a Cholesky factorization of a dense matrix.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `uplo` Indicates whether the upper or lower triangular part of the matrix is stored.
  * - `n` Dimension of the symmetric matrix.
  * - `a` A symmetric matrix stored in column-major order.
  */
MSKrescodee MSKAPI MSK_potrf(
	MSKenv_t env,
	MSKuploe uplo,
	MSKint32t n,
	MSKrealt * a);

/** Inputs a user-defined exit function which is called in case of fatal errors.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `handle` A pointer to a user-defined data structure.
  */
MSKrescodee MSKAPI MSK_putexitfunc(
	MSKenv_t env,
	MSKexitfunc exitfunc,
	MSKuserhandle_t handle);

/** Input a runtime license code.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `code` A license key string.
  */
MSKrescodee MSKAPI MSK_putlicensecode(
	MSKenv_t env,
	const MSKint32t * code);

/** Enables debug information for the license system.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `licdebug` Enable output of license check-out debug information.
  */
MSKrescodee MSKAPI MSK_putlicensedebug(
	MSKenv_t env,
	MSKint32t licdebug);

/** Set the path to the license file.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `licensepath` A path specifying where to search for the license.
  */
MSKrescodee MSKAPI MSK_putlicensepath(
	MSKenv_t env,
	const char * licensepath);

/** Control whether mosek should wait for an available license if no license is available.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `licwait` Enable waiting for a license.
  */
MSKrescodee MSKAPI MSK_putlicensewait(
	MSKenv_t env,
	MSKint32t licwait);

/** Obtains a response code string identifier.
  *
  * #Arguments
  * - `res` Response code.
  * - `str` String corresponding to the response code.
  */
MSKrescodee MSKAPI MSK_rescodetostr(
	MSKrescodee res,
	char * str);

/** Reset the license expiry reporting startpoint.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  */
MSKrescodee MSKAPI MSK_resetexpirylicenses(
	MSKenv_t env);

/** Solves a sparse triangular system of linear equations.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `transposed` Controls whether the solve is with L or the transposed L.
  * - `n` Specifies the dimension of L.
  * - `lnzc` lnzc[j] is the number of nonzeros in column j.
  * - `lptrc` lptrc[j] is a pointer to the first row index and value in column j.
  * - `lensubnval` Number of elements in lsubc and lvalc.
  * - `lsubc` Row indexes for each column stored sequentially.
  * - `lvalc` The value corresponding to row indexed stored lsubc.
  * - `b` The right-hand side of linear equation system to be solved as a dense vector.
  */
MSKrescodee MSKAPI MSK_sparsetriangularsolvedense(
	MSKenv_t env,
	MSKtransposee transposed,
	MSKint32t n,
	const MSKint32t * lnzc,
	const MSKint64t * lptrc,
	MSKint64t lensubnval,
	const MSKint32t * lsubc,
	const MSKrealt * lvalc,
	MSKrealt * b);

/** Computes all eigenvalues of a symmetric dense matrix.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `uplo` Indicates whether the upper or lower triangular part is used.
  * - `n` Dimension of the symmetric input matrix.
  * - `a` Input matrix A.
  * - `w` Array of length at least n containing the eigenvalues of A.
  */
MSKrescodee MSKAPI MSK_syeig(
	MSKenv_t env,
	MSKuploe uplo,
	MSKint32t n,
	const MSKrealt * a,
	MSKrealt * w);

/** Computes all the eigenvalues and eigenvectors of a symmetric dense matrix, and thus its eigenvalue decomposition.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `uplo` Indicates whether the upper or lower triangular part is used.
  * - `n` Dimension of the symmetric input matrix.
  * - `a` Input matrix A.
  * - `w` Array of length at least n containing the eigenvalues of A.
  */
MSKrescodee MSKAPI MSK_syevd(
	MSKenv_t env,
	MSKuploe uplo,
	MSKint32t n,
	MSKrealt * a,
	MSKrealt * w);

/** Obtains the value corresponding to a symbolic name defined by MOSEK.
  *
  * #Arguments
  * - `name` Symbolic name.
  * - `value` The corresponding value.
  */
int MSKAPI MSK_symnamtovalue(
	const char * name,
	char * value);

/** Performs a rank-k update of a symmetric matrix.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `uplo` Indicates whether the upper or lower triangular part of C is used.
  * - `trans` Indicates whether the matrix A must be transposed.
  * - `n` Specifies the order of C.
  * - `k` Indicates the number of rows or columns of A, and its rank.
  * - `alpha` A scalar value multiplying the result of the matrix multiplication.
  * - `a` The pointer to the array storing matrix A in a column-major format.
  * - `beta` A scalar value that multiplies C.
  * - `c` The pointer to the array storing matrix C in a column-major format.
  */
MSKrescodee MSKAPI MSK_syrk(
	MSKenv_t env,
	MSKuploe uplo,
	MSKtransposee trans,
	MSKint32t n,
	MSKint32t k,
	MSKrealt alpha,
	const MSKrealt * a,
	MSKrealt beta,
	MSKrealt * c);

/** Disconnects a user-defined function from a stream.
  *
  * #Arguments
  * - `env` The MOSEK environment.
  * - `whichstream` Index of the stream.
  */
MSKrescodee MSKAPI MSK_unlinkfuncfromenvstream(
	MSKenv_t env,
	MSKstreamtypee whichstream);

/** Converts an UTF8 string to a wchar string.
  *
  * #Arguments
  * - `outputlen` The length of the output buffer.
  * - `len` The length of the string contained in the output buffer.
  * - `conv` Returns the number of characters converted.
  * - `output` The input string converted to a wchar string.
  * - `input` The UTF8 input string.
  */
MSKrescodee MSKAPI MSK_utf8towchar(
	size_t outputlen,
	size_t * len,
	size_t * conv,
	MSKwchart * output,
	const char * input);

/** Converts a wchar string to an UTF8 string.
  *
  * #Arguments
  * - `outputlen` The length of the output buffer.
  * - `len` The length of the string contained in the output buffer.
  * - `conv` Returns the number of characters converted.
  * - `output` The input string converted to a UTF8 string.
  * - `input` The wchar input string.
  */
MSKrescodee MSKAPI MSK_wchartoutf8(
	size_t outputlen,
	size_t * len,
	size_t * conv,
	char * output,
	const MSKwchart * input);


#ifdef __cplusplus
}
#endif


#endif
