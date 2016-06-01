
.. |mosek| replace:: MOSEK
.. |null| replace:: ``None``

Constants
=========

.. index:: accmode
.. index:: ACC_...
.. _enum_accmode:
.. _accmode_con:
.. _accmode_var:

``accmode``
-----------

Constraint or variable access modes

``const MSK_ACC_CON : i32 = 1``
    Access data by rows (constraint oriented)
``const MSK_ACC_VAR : i32 = 0``
    Access data by columns (variable oriented)
.. index:: basindtype
.. index:: BI_...
.. _enum_basindtype:
.. _basindtype_always:
.. _basindtype_if_feasible:
.. _basindtype_never:
.. _basindtype_no_error:
.. _basindtype_reservered:

``basindtype``
--------------

Basis identification

``const MSK_BI_ALWAYS      : i32 = 1``
    Basis identification is always performed even if the interior-point optimizer terminates
                        abnormally.
``const MSK_BI_IF_FEASIBLE : i32 = 3``
    Basis identification is not performed if the interior-point optimizer terminates
                        with a problem status saying that the problem is primal or dual infeasible.
``const MSK_BI_NEVER       : i32 = 0``
    Never do basis identification.
``const MSK_BI_NO_ERROR    : i32 = 2``
    Basis identification is performed if the interior-point optimizer terminates without an error.
``const MSK_BI_RESERVERED  : i32 = 4``
    Not currently in use.
.. index:: boundkey
.. index:: BK_...
.. _enum_boundkey:
.. _boundkey_fr:
.. _boundkey_fx:
.. _boundkey_lo:
.. _boundkey_ra:
.. _boundkey_up:

``boundkey``
------------

Bound keys

``const MSK_BK_FR : i32 = 3``
    The constraint or variable is free.
``const MSK_BK_FX : i32 = 2``
    The constraint or variable is fixed.
``const MSK_BK_LO : i32 = 0``
    The constraint or variable has a finite
                        lower bound and an infinite upper bound.
``const MSK_BK_RA : i32 = 4``
    The constraint or variable is ranged.
``const MSK_BK_UP : i32 = 1``
    The constraint or variable has an infinite
                        lower bound and an finite upper bound.
.. index:: branchdir
.. index:: BRANCH_DIR_...
.. _enum_branchdir:
.. _branchdir_down:
.. _branchdir_far:
.. _branchdir_free:
.. _branchdir_guided:
.. _branchdir_near:
.. _branchdir_pseudocost:
.. _branchdir_root_lp:
.. _branchdir_up:

``branchdir``
-------------

Specifies the branching direction.

``const MSK_BRANCH_DIR_DOWN       : i32 = 2``
    The mixed-integer optimizer always chooses the down branch first.
``const MSK_BRANCH_DIR_FAR        : i32 = 4``
    Branch in direction farthest from selected fractional variable.
``const MSK_BRANCH_DIR_FREE       : i32 = 0``
    The mixed-integer optimizer decides which branch to choose.
``const MSK_BRANCH_DIR_GUIDED     : i32 = 6``
    Branch in direction of current incumbent.
``const MSK_BRANCH_DIR_NEAR       : i32 = 3``
    Branch in direction nearest to selected fractional variable.
``const MSK_BRANCH_DIR_PSEUDOCOST : i32 = 7``
    Branch based on the pseudocost of the variable.
``const MSK_BRANCH_DIR_ROOT_LP    : i32 = 5``
    Chose direction based on root lp value of selected variable.
``const MSK_BRANCH_DIR_UP         : i32 = 1``
    The mixed-integer optimizer always chooses the up branch first.
.. index:: callbackcode
.. index:: CALLBACK_...
.. _enum_callbackcode:
.. _callbackcode_begin_bi:
.. _callbackcode_begin_concurrent:
.. _callbackcode_begin_conic:
.. _callbackcode_begin_dual_bi:
.. _callbackcode_begin_dual_sensitivity:
.. _callbackcode_begin_dual_setup_bi:
.. _callbackcode_begin_dual_simplex:
.. _callbackcode_begin_dual_simplex_bi:
.. _callbackcode_begin_full_convexity_check:
.. _callbackcode_begin_infeas_ana:
.. _callbackcode_begin_intpnt:
.. _callbackcode_begin_license_wait:
.. _callbackcode_begin_mio:
.. _callbackcode_begin_network_dual_simplex:
.. _callbackcode_begin_network_primal_simplex:
.. _callbackcode_begin_network_simplex:
.. _callbackcode_begin_optimizer:
.. _callbackcode_begin_presolve:
.. _callbackcode_begin_primal_bi:
.. _callbackcode_begin_primal_dual_simplex:
.. _callbackcode_begin_primal_dual_simplex_bi:
.. _callbackcode_begin_primal_repair:
.. _callbackcode_begin_primal_sensitivity:
.. _callbackcode_begin_primal_setup_bi:
.. _callbackcode_begin_primal_simplex:
.. _callbackcode_begin_primal_simplex_bi:
.. _callbackcode_begin_qcqo_reformulate:
.. _callbackcode_begin_read:
.. _callbackcode_begin_root_cutgen:
.. _callbackcode_begin_simplex:
.. _callbackcode_begin_simplex_bi:
.. _callbackcode_begin_simplex_network_detect:
.. _callbackcode_begin_to_conic:
.. _callbackcode_begin_write:
.. _callbackcode_conic:
.. _callbackcode_dual_simplex:
.. _callbackcode_end_bi:
.. _callbackcode_end_concurrent:
.. _callbackcode_end_conic:
.. _callbackcode_end_dual_bi:
.. _callbackcode_end_dual_sensitivity:
.. _callbackcode_end_dual_setup_bi:
.. _callbackcode_end_dual_simplex:
.. _callbackcode_end_dual_simplex_bi:
.. _callbackcode_end_full_convexity_check:
.. _callbackcode_end_infeas_ana:
.. _callbackcode_end_intpnt:
.. _callbackcode_end_license_wait:
.. _callbackcode_end_mio:
.. _callbackcode_end_network_dual_simplex:
.. _callbackcode_end_network_primal_simplex:
.. _callbackcode_end_network_simplex:
.. _callbackcode_end_optimizer:
.. _callbackcode_end_presolve:
.. _callbackcode_end_primal_bi:
.. _callbackcode_end_primal_dual_simplex:
.. _callbackcode_end_primal_dual_simplex_bi:
.. _callbackcode_end_primal_repair:
.. _callbackcode_end_primal_sensitivity:
.. _callbackcode_end_primal_setup_bi:
.. _callbackcode_end_primal_simplex:
.. _callbackcode_end_primal_simplex_bi:
.. _callbackcode_end_qcqo_reformulate:
.. _callbackcode_end_read:
.. _callbackcode_end_root_cutgen:
.. _callbackcode_end_simplex:
.. _callbackcode_end_simplex_bi:
.. _callbackcode_end_simplex_network_detect:
.. _callbackcode_end_to_conic:
.. _callbackcode_end_write:
.. _callbackcode_im_bi:
.. _callbackcode_im_conic:
.. _callbackcode_im_dual_bi:
.. _callbackcode_im_dual_sensivity:
.. _callbackcode_im_dual_simplex:
.. _callbackcode_im_full_convexity_check:
.. _callbackcode_im_intpnt:
.. _callbackcode_im_license_wait:
.. _callbackcode_im_lu:
.. _callbackcode_im_mio:
.. _callbackcode_im_mio_dual_simplex:
.. _callbackcode_im_mio_intpnt:
.. _callbackcode_im_mio_primal_simplex:
.. _callbackcode_im_network_dual_simplex:
.. _callbackcode_im_network_primal_simplex:
.. _callbackcode_im_order:
.. _callbackcode_im_presolve:
.. _callbackcode_im_primal_bi:
.. _callbackcode_im_primal_dual_simplex:
.. _callbackcode_im_primal_sensivity:
.. _callbackcode_im_primal_simplex:
.. _callbackcode_im_qo_reformulate:
.. _callbackcode_im_read:
.. _callbackcode_im_root_cutgen:
.. _callbackcode_im_simplex:
.. _callbackcode_im_simplex_bi:
.. _callbackcode_intpnt:
.. _callbackcode_new_int_mio:
.. _callbackcode_primal_simplex:
.. _callbackcode_read_opf:
.. _callbackcode_read_opf_section:
.. _callbackcode_solving_remote:
.. _callbackcode_update_dual_bi:
.. _callbackcode_update_dual_simplex:
.. _callbackcode_update_dual_simplex_bi:
.. _callbackcode_update_network_dual_simplex:
.. _callbackcode_update_network_primal_simplex:
.. _callbackcode_update_presolve:
.. _callbackcode_update_primal_bi:
.. _callbackcode_update_primal_dual_simplex:
.. _callbackcode_update_primal_dual_simplex_bi:
.. _callbackcode_update_primal_simplex:
.. _callbackcode_update_primal_simplex_bi:
.. _callbackcode_write_opf:

``callbackcode``
----------------

Progress call-back codes

``const MSK_CALLBACK_BEGIN_BI                      : i32 = 0``
    The basis identification procedure
                        has been started.
``const MSK_CALLBACK_BEGIN_CONCURRENT              : i32 = 1``
    Concurrent optimizer is started.
``const MSK_CALLBACK_BEGIN_CONIC                   : i32 = 2``
    The call-back function is called
                        when the conic optimizer is started.
``const MSK_CALLBACK_BEGIN_DUAL_BI                 : i32 = 3``
    The call-back function is called
                        from within the basis identification procedure
                        when the dual phase is started.
``const MSK_CALLBACK_BEGIN_DUAL_SENSITIVITY        : i32 = 4``
    Dual sensitivity analysis is started.
``const MSK_CALLBACK_BEGIN_DUAL_SETUP_BI           : i32 = 5``
    The call-back function is called when the dual BI phase is started.
``const MSK_CALLBACK_BEGIN_DUAL_SIMPLEX            : i32 = 6``
    The call-back function is called when the dual simplex optimizer started.
``const MSK_CALLBACK_BEGIN_DUAL_SIMPLEX_BI         : i32 = 7``
    The call-back function is called 
                        from within the basis identification procedure
                        when the dual simplex clean-up phase is started.
``const MSK_CALLBACK_BEGIN_FULL_CONVEXITY_CHECK    : i32 = 8``
    Begin full convexity check.
``const MSK_CALLBACK_BEGIN_INFEAS_ANA              : i32 = 9``
    The call-back function is called when the infeasibility analyzer is started.
``const MSK_CALLBACK_BEGIN_INTPNT                  : i32 = 10``
    The call-back function is called
                        when the interior-point optimizer is started.
``const MSK_CALLBACK_BEGIN_LICENSE_WAIT            : i32 = 11``
    Begin waiting for license.
``const MSK_CALLBACK_BEGIN_MIO                     : i32 = 12``
    The call-back function is called when the mixed-integer optimizer is started.
``const MSK_CALLBACK_BEGIN_NETWORK_DUAL_SIMPLEX    : i32 = 13``
    The call-back function is called when the dual network simplex optimizer is started.
``const MSK_CALLBACK_BEGIN_NETWORK_PRIMAL_SIMPLEX  : i32 = 14``
    The call-back function is called when the primal network simplex optimizer is started.
``const MSK_CALLBACK_BEGIN_NETWORK_SIMPLEX         : i32 = 15``
    The call-back function is called when the simplex network optimizer is started.
``const MSK_CALLBACK_BEGIN_OPTIMIZER               : i32 = 16``
    The call-back function is called when the optimizer is started.
``const MSK_CALLBACK_BEGIN_PRESOLVE                : i32 = 17``
    The call-back function is called
                        when the presolve is started.
``const MSK_CALLBACK_BEGIN_PRIMAL_BI               : i32 = 18``
    The call-back function is called
                        from within the basis identification procedure
                        when the primal phase is started.
``const MSK_CALLBACK_BEGIN_PRIMAL_DUAL_SIMPLEX     : i32 = 19``
    The call-back function is called when the primal-dual simplex optimizer is started.
``const MSK_CALLBACK_BEGIN_PRIMAL_DUAL_SIMPLEX_BI  : i32 = 20``
    The call-back function is called
                        from within the basis identification procedure
                        when the primal-dual simplex clean-up phase is started.
``const MSK_CALLBACK_BEGIN_PRIMAL_REPAIR           : i32 = 21``
    Begin primal feasibility repair.
``const MSK_CALLBACK_BEGIN_PRIMAL_SENSITIVITY      : i32 = 22``
    Primal sensitivity analysis is started.
``const MSK_CALLBACK_BEGIN_PRIMAL_SETUP_BI         : i32 = 23``
    The call-back function is called when the primal BI setup is started.
``const MSK_CALLBACK_BEGIN_PRIMAL_SIMPLEX          : i32 = 24``
    The call-back function is called when the primal simplex optimizer is started.
``const MSK_CALLBACK_BEGIN_PRIMAL_SIMPLEX_BI       : i32 = 25``
    The call-back function is called
                        from within the basis identification procedure
                        when the primal simplex clean-up phase is started.
``const MSK_CALLBACK_BEGIN_QCQO_REFORMULATE        : i32 = 26``
    Begin QCQO reformulation.
``const MSK_CALLBACK_BEGIN_READ                    : i32 = 27``
    |mosek| has started reading a problem file.
``const MSK_CALLBACK_BEGIN_ROOT_CUTGEN             : i32 = 28``
    The call-back function is called when root cut generation is started.
``const MSK_CALLBACK_BEGIN_SIMPLEX                 : i32 = 29``
    The call-back function is called when the simplex optimizer is started.
``const MSK_CALLBACK_BEGIN_SIMPLEX_BI              : i32 = 30``
    The call-back function is called
                        from within the basis identification procedure
                        when the simplex clean-up phase is started.
``const MSK_CALLBACK_BEGIN_SIMPLEX_NETWORK_DETECT  : i32 = 31``
    The call-back function is called when the network detection procedure is started.
``const MSK_CALLBACK_BEGIN_TO_CONIC                : i32 = 32``
    Begin conic reformulation.
``const MSK_CALLBACK_BEGIN_WRITE                   : i32 = 33``
    |mosek| has started writing a problem file.
``const MSK_CALLBACK_CONIC                         : i32 = 34``
    The call-back function is called from within the
                        conic optimizer after the information database has been updated.
``const MSK_CALLBACK_DUAL_SIMPLEX                  : i32 = 35``
    The call-back function is called
                        from within the dual simplex optimizer.
``const MSK_CALLBACK_END_BI                        : i32 = 36``
    The call-back function is called
                        when the basis identification procedure
                        is terminated.
``const MSK_CALLBACK_END_CONCURRENT                : i32 = 37``
    Concurrent optimizer is terminated.
``const MSK_CALLBACK_END_CONIC                     : i32 = 38``
    The call-back function is called
                        when the conic optimizer is terminated.
``const MSK_CALLBACK_END_DUAL_BI                   : i32 = 39``
    The call-back function is called
                        from within the basis identification procedure
                        when the dual phase is terminated.
``const MSK_CALLBACK_END_DUAL_SENSITIVITY          : i32 = 40``
    Dual sensitivity analysis is terminated.
``const MSK_CALLBACK_END_DUAL_SETUP_BI             : i32 = 41``
    The call-back function is called when the dual BI phase is terminated.
``const MSK_CALLBACK_END_DUAL_SIMPLEX              : i32 = 42``
    The call-back function is called when the dual simplex optimizer is terminated.
``const MSK_CALLBACK_END_DUAL_SIMPLEX_BI           : i32 = 43``
    The call-back function is called
                        from within the basis identification procedure
                        when the dual clean-up phase is terminated.
``const MSK_CALLBACK_END_FULL_CONVEXITY_CHECK      : i32 = 44``
    End full convexity check.
``const MSK_CALLBACK_END_INFEAS_ANA                : i32 = 45``
    The call-back function is called when the infeasibility analyzer is terminated.
``const MSK_CALLBACK_END_INTPNT                    : i32 = 46``
    The call-back function is called
                        when the interior-point optimizer is terminated.
``const MSK_CALLBACK_END_LICENSE_WAIT              : i32 = 47``
    End waiting for license.
``const MSK_CALLBACK_END_MIO                       : i32 = 48``
    The call-back function is called when the mixed-integer optimizer is terminated.
``const MSK_CALLBACK_END_NETWORK_DUAL_SIMPLEX      : i32 = 49``
    The call-back function is called when the dual network simplex optimizer is terminated.
``const MSK_CALLBACK_END_NETWORK_PRIMAL_SIMPLEX    : i32 = 50``
    The call-back function is called when the primal network simplex optimizer is terminated.
``const MSK_CALLBACK_END_NETWORK_SIMPLEX           : i32 = 51``
    The call-back function is called when the simplex network optimizer is terminated.
``const MSK_CALLBACK_END_OPTIMIZER                 : i32 = 52``
    The call-back function is called when the optimizer is terminated.
``const MSK_CALLBACK_END_PRESOLVE                  : i32 = 53``
    The call-back function is called
                        when the presolve is completed.
``const MSK_CALLBACK_END_PRIMAL_BI                 : i32 = 54``
    The call-back function is called
                        from within the basis identification procedure
                        when the primal phase is terminated.
``const MSK_CALLBACK_END_PRIMAL_DUAL_SIMPLEX       : i32 = 55``
    The call-back function is called when the primal-dual simplex optimizer is terminated.
``const MSK_CALLBACK_END_PRIMAL_DUAL_SIMPLEX_BI    : i32 = 56``
    The call-back function is called
                        from within the basis identification procedure
                        when the primal-dual clean-up phase is terminated.
``const MSK_CALLBACK_END_PRIMAL_REPAIR             : i32 = 57``
    End primal feasibility repair.
``const MSK_CALLBACK_END_PRIMAL_SENSITIVITY        : i32 = 58``
    Primal sensitivity analysis is terminated.
``const MSK_CALLBACK_END_PRIMAL_SETUP_BI           : i32 = 59``
    The call-back function is called when the primal BI setup is terminated.
``const MSK_CALLBACK_END_PRIMAL_SIMPLEX            : i32 = 60``
    The call-back function is called when the primal simplex optimizer is terminated.
``const MSK_CALLBACK_END_PRIMAL_SIMPLEX_BI         : i32 = 61``
    The call-back function is called
                        from within the basis identification procedure
                        when the primal clean-up phase is terminated.
``const MSK_CALLBACK_END_QCQO_REFORMULATE          : i32 = 62``
    End QCQO reformulation.
``const MSK_CALLBACK_END_READ                      : i32 = 63``
    |mosek| has finished reading a problem file.
``const MSK_CALLBACK_END_ROOT_CUTGEN               : i32 = 64``
    The call-back function is called when root cut generation is is terminated.
``const MSK_CALLBACK_END_SIMPLEX                   : i32 = 65``
    The call-back function is called when the simplex optimizer is terminated.
``const MSK_CALLBACK_END_SIMPLEX_BI                : i32 = 66``
    The call-back function is called
                        from within the basis identification procedure
                        when the simplex clean-up phase is terminated.
``const MSK_CALLBACK_END_SIMPLEX_NETWORK_DETECT    : i32 = 67``
    The call-back function is called when the network detection procedure is terminated.
``const MSK_CALLBACK_END_TO_CONIC                  : i32 = 68``
    End conic reformulation.
``const MSK_CALLBACK_END_WRITE                     : i32 = 69``
    |mosek| has finished writing a problem file.
``const MSK_CALLBACK_IM_BI                         : i32 = 70``
    The call-back function is called
                        from within the basis identification procedure
                        at an intermediate point.
``const MSK_CALLBACK_IM_CONIC                      : i32 = 71``
    The call-back function is called
                        at an intermediate stage within the conic optimizer where
                        the information database has not been updated.
``const MSK_CALLBACK_IM_DUAL_BI                    : i32 = 72``
    The call-back function is called
                        from within the basis identification procedure
                        at an intermediate point in the dual phase.
``const MSK_CALLBACK_IM_DUAL_SENSIVITY             : i32 = 73``
    The call-back function is called at an intermediate stage of the dual sensitivity analysis.
``const MSK_CALLBACK_IM_DUAL_SIMPLEX               : i32 = 74``
    The call-back function is called at an intermediate point in the dual simplex optimizer.
``const MSK_CALLBACK_IM_FULL_CONVEXITY_CHECK       : i32 = 75``
    The call-back function is called at an intermediate stage of the full convexity check.
``const MSK_CALLBACK_IM_INTPNT                     : i32 = 76``
    The call-back function is called
                        at an intermediate stage within the interior-point optimizer where
                        the information database has not been updated.
``const MSK_CALLBACK_IM_LICENSE_WAIT               : i32 = 77``
    |mosek| is waiting for a license.
``const MSK_CALLBACK_IM_LU                         : i32 = 78``
    The call-back function is called
                        from within the LU factorization procedure at an intermediate point.
``const MSK_CALLBACK_IM_MIO                        : i32 = 79``
    The call-back function is called at an intermediate point in the mixed-integer optimizer.
``const MSK_CALLBACK_IM_MIO_DUAL_SIMPLEX           : i32 = 80``
    The call-back function is called at an intermediate point in the mixed-integer optimizer while running the
                        dual simplex optimizer.
``const MSK_CALLBACK_IM_MIO_INTPNT                 : i32 = 81``
    The call-back function is called at an intermediate point in the mixed-integer optimizer while running the
                        interior-point optimizer.
``const MSK_CALLBACK_IM_MIO_PRIMAL_SIMPLEX         : i32 = 82``
    The call-back function is called at an intermediate point in the mixed-integer optimizer while running the
                        primal simplex optimizer.
``const MSK_CALLBACK_IM_NETWORK_DUAL_SIMPLEX       : i32 = 83``
    The call-back function is called at an intermediate point in the dual network simplex optimizer.
``const MSK_CALLBACK_IM_NETWORK_PRIMAL_SIMPLEX     : i32 = 84``
    The call-back function is called at an intermediate point in the primal network simplex optimizer.
``const MSK_CALLBACK_IM_ORDER                      : i32 = 85``
    The call-back function is called
                        from within the matrix ordering procedure at an intermediate point.
``const MSK_CALLBACK_IM_PRESOLVE                   : i32 = 86``
    The call-back function is called
                        from within the presolve procedure
                        at an intermediate stage.
``const MSK_CALLBACK_IM_PRIMAL_BI                  : i32 = 87``
    The call-back function is called
                        from within the basis identification procedure
                        at an intermediate point in the primal phase.
``const MSK_CALLBACK_IM_PRIMAL_DUAL_SIMPLEX        : i32 = 88``
    The call-back function is called at an intermediate point in the primal-dual simplex optimizer.
``const MSK_CALLBACK_IM_PRIMAL_SENSIVITY           : i32 = 89``
    The call-back function is called at an intermediate stage of the primal sensitivity analysis.
``const MSK_CALLBACK_IM_PRIMAL_SIMPLEX             : i32 = 90``
    The call-back function is called at an intermediate point in the primal simplex optimizer.
``const MSK_CALLBACK_IM_QO_REFORMULATE             : i32 = 91``
    The call-back function is called at an intermediate stage of the conic quadratic reformulation.
``const MSK_CALLBACK_IM_READ                       : i32 = 92``
    Intermediate stage in reading.
``const MSK_CALLBACK_IM_ROOT_CUTGEN                : i32 = 93``
    The call-back is called from within root cut generation at an intermediate stage.
``const MSK_CALLBACK_IM_SIMPLEX                    : i32 = 94``
    The call-back function is called from within the
                        simplex optimizer at an intermediate point.
``const MSK_CALLBACK_IM_SIMPLEX_BI                 : i32 = 95``
    The call-back function is called
                        from within the basis identification procedure
                        at an intermediate point in the simplex clean-up phase.
``const MSK_CALLBACK_INTPNT                        : i32 = 96``
    The call-back function is called from within the
                        interior-point optimizer after the information database has been updated.
``const MSK_CALLBACK_NEW_INT_MIO                   : i32 = 97``
    The call-back function is called after a new integer solution
                        has been located by the mixed-integer optimizer.
``const MSK_CALLBACK_PRIMAL_SIMPLEX                : i32 = 98``
    The call-back function is called
                        from within the primal simplex optimizer.
``const MSK_CALLBACK_READ_OPF                      : i32 = 99``
    The call-back function is called from the OPF
                        reader.
``const MSK_CALLBACK_READ_OPF_SECTION              : i32 = 100``
    A chunk of Q non-zeros has been read from a problem file.
``const MSK_CALLBACK_SOLVING_REMOTE                : i32 = 101``
    The call-back function is called while the task is being solved on a remote server.
``const MSK_CALLBACK_UPDATE_DUAL_BI                : i32 = 102``
    The call-back function is called
                        from within the basis identification procedure
                        at an intermediate point in the dual phase.
``const MSK_CALLBACK_UPDATE_DUAL_SIMPLEX           : i32 = 103``
    The call-back function is called in the dual simplex optimizer.
``const MSK_CALLBACK_UPDATE_DUAL_SIMPLEX_BI        : i32 = 104``
    The call-back function is called
                        from within the basis identification procedure
                        at an intermediate point in the dual simplex clean-up phase.
``const MSK_CALLBACK_UPDATE_NETWORK_DUAL_SIMPLEX   : i32 = 105``
    The call-back function is called in the dual network simplex optimizer.
``const MSK_CALLBACK_UPDATE_NETWORK_PRIMAL_SIMPLEX : i32 = 106``
    The call-back function is called in the primal network simplex optimizer.
``const MSK_CALLBACK_UPDATE_PRESOLVE               : i32 = 107``
    The call-back function is called
                        from within the presolve procedure.
``const MSK_CALLBACK_UPDATE_PRIMAL_BI              : i32 = 108``
    The call-back function is called
                        from within the basis identification procedure
                        at an intermediate point in the primal phase.
``const MSK_CALLBACK_UPDATE_PRIMAL_DUAL_SIMPLEX    : i32 = 109``
    The call-back function is called  in the primal-dual simplex optimizer.
``const MSK_CALLBACK_UPDATE_PRIMAL_DUAL_SIMPLEX_BI : i32 = 110``
    The call-back function is called
                        from within the basis identification procedure
                        at an intermediate point in the primal simplex clean-up phase.
``const MSK_CALLBACK_UPDATE_PRIMAL_SIMPLEX         : i32 = 111``
    The call-back function is called  in the primal simplex optimizer.
``const MSK_CALLBACK_UPDATE_PRIMAL_SIMPLEX_BI      : i32 = 112``
    The call-back function is called from within the basis identification procedure at an intermediate point in the primal simplex clean-up phase.
``const MSK_CALLBACK_WRITE_OPF                     : i32 = 113``
    The call-back function is called from the OPF
                        writer.
.. index:: checkconvexitytype
.. index:: CHECK_CONVEXITY_...
.. _enum_checkconvexitytype:
.. _checkconvexitytype_full:
.. _checkconvexitytype_none:
.. _checkconvexitytype_simple:

``checkconvexitytype``
----------------------

Types of convexity checks.

``const MSK_CHECK_CONVEXITY_FULL   : i32 = 2``
    Perform a full convexity check.
``const MSK_CHECK_CONVEXITY_NONE   : i32 = 0``
    No convexity check.
``const MSK_CHECK_CONVEXITY_SIMPLE : i32 = 1``
    Perform simple and fast convexity check.
.. index:: compresstype
.. index:: COMPRESS_...
.. _enum_compresstype:
.. _compresstype_free:
.. _compresstype_gzip:
.. _compresstype_none:

``compresstype``
----------------

Compression types

``const MSK_COMPRESS_FREE : i32 = 1``
    The type of compression used is chosen automatically.
``const MSK_COMPRESS_GZIP : i32 = 2``
    The type of compression used is gzip compatible.
``const MSK_COMPRESS_NONE : i32 = 0``
    No compression is used.
.. index:: conetype
.. index:: CT_...
.. _enum_conetype:
.. _conetype_quad:
.. _conetype_rquad:

``conetype``
------------

Cone types

``const MSK_CT_QUAD  : i32 = 0``
    The cone is a quadratic cone.
``const MSK_CT_RQUAD : i32 = 1``
    The cone is a rotated quadratic cone.
.. index:: dataformat
.. index:: DATA_FORMAT_...
.. _enum_dataformat:
.. _dataformat_cb:
.. _dataformat_extension:
.. _dataformat_free_mps:
.. _dataformat_json_task:
.. _dataformat_lp:
.. _dataformat_mps:
.. _dataformat_op:
.. _dataformat_task:
.. _dataformat_xml:

``dataformat``
--------------

Data format types

``const MSK_DATA_FORMAT_CB        : i32 = 7``
    Conic benchmark format,
``const MSK_DATA_FORMAT_EXTENSION : i32 = 0``
    The file extension is used to determine the data file format.
``const MSK_DATA_FORMAT_FREE_MPS  : i32 = 5``
    The data a free MPS formatted file.
``const MSK_DATA_FORMAT_JSON_TASK : i32 = 8``
    JSON based task format.
``const MSK_DATA_FORMAT_LP        : i32 = 2``
    The data file is LP formatted.
``const MSK_DATA_FORMAT_MPS       : i32 = 1``
    The data file is MPS formatted.
``const MSK_DATA_FORMAT_OP        : i32 = 3``
    The data file is an optimization problem formatted file.
``const MSK_DATA_FORMAT_TASK      : i32 = 6``
    Generic task dump file.
``const MSK_DATA_FORMAT_XML       : i32 = 4``
    The data file is an XML formatted file.
.. index:: dinfitem
.. index:: DINF_...
.. _enum_dinfitem:
.. _dinfitem_bi_clean_dual_time:
.. _dinfitem_bi_clean_primal_dual_time:
.. _dinfitem_bi_clean_primal_time:
.. _dinfitem_bi_clean_time:
.. _dinfitem_bi_dual_time:
.. _dinfitem_bi_primal_time:
.. _dinfitem_bi_time:
.. _dinfitem_concurrent_time:
.. _dinfitem_intpnt_dual_feas:
.. _dinfitem_intpnt_dual_obj:
.. _dinfitem_intpnt_factor_num_flops:
.. _dinfitem_intpnt_opt_status:
.. _dinfitem_intpnt_order_time:
.. _dinfitem_intpnt_primal_feas:
.. _dinfitem_intpnt_primal_obj:
.. _dinfitem_intpnt_time:
.. _dinfitem_mio_clique_separation_time:
.. _dinfitem_mio_cmir_separation_time:
.. _dinfitem_mio_construct_solution_obj:
.. _dinfitem_mio_dual_bound_after_presolve:
.. _dinfitem_mio_gmi_separation_time:
.. _dinfitem_mio_heuristic_time:
.. _dinfitem_mio_implied_bound_time:
.. _dinfitem_mio_knapsack_cover_separation_time:
.. _dinfitem_mio_obj_abs_gap:
.. _dinfitem_mio_obj_bound:
.. _dinfitem_mio_obj_int:
.. _dinfitem_mio_obj_rel_gap:
.. _dinfitem_mio_optimizer_time:
.. _dinfitem_mio_probing_time:
.. _dinfitem_mio_root_cutgen_time:
.. _dinfitem_mio_root_optimizer_time:
.. _dinfitem_mio_root_presolve_time:
.. _dinfitem_mio_time:
.. _dinfitem_mio_user_obj_cut:
.. _dinfitem_optimizer_time:
.. _dinfitem_presolve_eli_time:
.. _dinfitem_presolve_lindep_time:
.. _dinfitem_presolve_time:
.. _dinfitem_primal_repair_penalty_obj:
.. _dinfitem_qcqo_reformulate_max_perturbation:
.. _dinfitem_qcqo_reformulate_time:
.. _dinfitem_qcqo_reformulate_worst_cholesky_column_scaling:
.. _dinfitem_qcqo_reformulate_worst_cholesky_diag_scaling:
.. _dinfitem_rd_time:
.. _dinfitem_sim_dual_time:
.. _dinfitem_sim_feas:
.. _dinfitem_sim_network_dual_time:
.. _dinfitem_sim_network_primal_time:
.. _dinfitem_sim_network_time:
.. _dinfitem_sim_obj:
.. _dinfitem_sim_primal_dual_time:
.. _dinfitem_sim_primal_time:
.. _dinfitem_sim_time:
.. _dinfitem_sol_bas_dual_obj:
.. _dinfitem_sol_bas_dviolcon:
.. _dinfitem_sol_bas_dviolvar:
.. _dinfitem_sol_bas_nrm_barx:
.. _dinfitem_sol_bas_nrm_slc:
.. _dinfitem_sol_bas_nrm_slx:
.. _dinfitem_sol_bas_nrm_suc:
.. _dinfitem_sol_bas_nrm_sux:
.. _dinfitem_sol_bas_nrm_xc:
.. _dinfitem_sol_bas_nrm_xx:
.. _dinfitem_sol_bas_nrm_y:
.. _dinfitem_sol_bas_primal_obj:
.. _dinfitem_sol_bas_pviolcon:
.. _dinfitem_sol_bas_pviolvar:
.. _dinfitem_sol_itg_nrm_barx:
.. _dinfitem_sol_itg_nrm_xc:
.. _dinfitem_sol_itg_nrm_xx:
.. _dinfitem_sol_itg_primal_obj:
.. _dinfitem_sol_itg_pviolbarvar:
.. _dinfitem_sol_itg_pviolcon:
.. _dinfitem_sol_itg_pviolcones:
.. _dinfitem_sol_itg_pviolitg:
.. _dinfitem_sol_itg_pviolvar:
.. _dinfitem_sol_itr_dual_obj:
.. _dinfitem_sol_itr_dviolbarvar:
.. _dinfitem_sol_itr_dviolcon:
.. _dinfitem_sol_itr_dviolcones:
.. _dinfitem_sol_itr_dviolvar:
.. _dinfitem_sol_itr_nrm_bars:
.. _dinfitem_sol_itr_nrm_barx:
.. _dinfitem_sol_itr_nrm_slc:
.. _dinfitem_sol_itr_nrm_slx:
.. _dinfitem_sol_itr_nrm_snx:
.. _dinfitem_sol_itr_nrm_suc:
.. _dinfitem_sol_itr_nrm_sux:
.. _dinfitem_sol_itr_nrm_xc:
.. _dinfitem_sol_itr_nrm_xx:
.. _dinfitem_sol_itr_nrm_y:
.. _dinfitem_sol_itr_primal_obj:
.. _dinfitem_sol_itr_pviolbarvar:
.. _dinfitem_sol_itr_pviolcon:
.. _dinfitem_sol_itr_pviolcones:
.. _dinfitem_sol_itr_pviolvar:
.. _dinfitem_to_conic_time:

``dinfitem``
------------

Double information items

``const MSK_DINF_BI_CLEAN_DUAL_TIME                             : i32 = 0``
    Time  spent within the dual clean-up optimizer of the basis identification
                        procedure since its invocation.
``const MSK_DINF_BI_CLEAN_PRIMAL_DUAL_TIME                      : i32 = 1``
    Time spent within the primal-dual clean-up optimizer of the basis identification
                        procedure since its invocation.
``const MSK_DINF_BI_CLEAN_PRIMAL_TIME                           : i32 = 2``
    Time spent within the primal clean-up optimizer of the basis identification
                        procedure since its invocation.
``const MSK_DINF_BI_CLEAN_TIME                                  : i32 = 3``
    Time spent within the clean-up phase of the basis identification
                        procedure since its invocation.
``const MSK_DINF_BI_DUAL_TIME                                   : i32 = 4``
    Time spent within the dual phase basis identification
                        procedure since its invocation.
``const MSK_DINF_BI_PRIMAL_TIME                                 : i32 = 5``
    Time  spent within the primal phase of the basis identification
                    procedure since its invocation.
``const MSK_DINF_BI_TIME                                        : i32 = 6``
    Time spent within the basis identification
                    procedure since its invocation.
``const MSK_DINF_CONCURRENT_TIME                                : i32 = 7``
    Time spent within the concurrent optimizer since its invocation.
``const MSK_DINF_INTPNT_DUAL_FEAS                               : i32 = 8``
    Dual feasibility measure reported by the
                        interior-point optimizer. (For the
                        interior-point optimizer this measure does not
                        directly related to the original problem because
                        a homogeneous model is employed.)
``const MSK_DINF_INTPNT_DUAL_OBJ                                : i32 = 9``
    Dual objective value reported by the
                        interior-point optimizer.
``const MSK_DINF_INTPNT_FACTOR_NUM_FLOPS                        : i32 = 10``
    An estimate of the number of flops used in the factorization.
``const MSK_DINF_INTPNT_OPT_STATUS                              : i32 = 11``
    This measure should converge to :math:`+1` if the problem
                        has a primal-dual optimal solution, and converge to :math:`-1`
                        if problem is (strictly) primal or dual infeasible. Furthermore, if the measure converges to 0
                        the problem is usually ill-posed.
``const MSK_DINF_INTPNT_ORDER_TIME                              : i32 = 12``
    Order time (in seconds).
``const MSK_DINF_INTPNT_PRIMAL_FEAS                             : i32 = 13``
    Primal feasibility measure reported by the
                        interior-point optimizers. (For the interior-point
                        optimizer this measure does not directly related
                        to the original problem because a homogeneous
                        model is employed).
``const MSK_DINF_INTPNT_PRIMAL_OBJ                              : i32 = 14``
    Primal objective value reported by the interior-point optimizer.
``const MSK_DINF_INTPNT_TIME                                    : i32 = 15``
    Time spent within the interior-point optimizer
                        since its invocation.
``const MSK_DINF_MIO_CLIQUE_SEPARATION_TIME                     : i32 = 16``
    Seperation time for clique cuts.
``const MSK_DINF_MIO_CMIR_SEPARATION_TIME                       : i32 = 17``
    Seperation time for CMIR cuts.
``const MSK_DINF_MIO_CONSTRUCT_SOLUTION_OBJ                     : i32 = 18``
    If |mosek| has successfully constructed an integer feasible solution, then this item
                        contains the optimal objective value corresponding to the feasible solution.
``const MSK_DINF_MIO_DUAL_BOUND_AFTER_PRESOLVE                  : i32 = 19``
    Value of the dual bound after presolve but before cut generation.
``const MSK_DINF_MIO_GMI_SEPARATION_TIME                        : i32 = 20``
    Seperation time for GMI cuts.
``const MSK_DINF_MIO_HEURISTIC_TIME                             : i32 = 21``
    Total time spent in the optimizer.
``const MSK_DINF_MIO_IMPLIED_BOUND_TIME                         : i32 = 22``
    Seperation time for implied bound cuts.
``const MSK_DINF_MIO_KNAPSACK_COVER_SEPARATION_TIME             : i32 = 23``
    Seperation time for knapsack cover.
``const MSK_DINF_MIO_OBJ_ABS_GAP                                : i32 = 24``
    If the mixed-integer optimizer has computed a feasible solution and a bound, this contains the absolute gap.
``const MSK_DINF_MIO_OBJ_BOUND                                  : i32 = 25``
    The best bound on the objective value known.
``const MSK_DINF_MIO_OBJ_INT                                    : i32 = 26``
    The primal objective value corresponding to the best integer feasible solution.
``const MSK_DINF_MIO_OBJ_REL_GAP                                : i32 = 27``
    If the mixed-integer optimizer has computed a feasible solution and a bound, this contains the relative gap.
``const MSK_DINF_MIO_OPTIMIZER_TIME                             : i32 = 28``
    Total time spent in the optimizer.
``const MSK_DINF_MIO_PROBING_TIME                               : i32 = 29``
    Total time for probing.
``const MSK_DINF_MIO_ROOT_CUTGEN_TIME                           : i32 = 30``
    Total time for cut generation.
``const MSK_DINF_MIO_ROOT_OPTIMIZER_TIME                        : i32 = 31``
    Time spent in the optimizer while solving the root relaxation.
``const MSK_DINF_MIO_ROOT_PRESOLVE_TIME                         : i32 = 32``
    Time spent in while presolving the root relaxation.
``const MSK_DINF_MIO_TIME                                       : i32 = 33``
    Time spent in the mixed-integer optimizer.
``const MSK_DINF_MIO_USER_OBJ_CUT                               : i32 = 34``
    If the objective cut is used, then this information item has the value of the cut.
``const MSK_DINF_OPTIMIZER_TIME                                 : i32 = 35``
    Total time spent in the optimizer since it was invoked.
``const MSK_DINF_PRESOLVE_ELI_TIME                              : i32 = 36``
    Total time spent in the eliminator
                        since the presolve was invoked.
``const MSK_DINF_PRESOLVE_LINDEP_TIME                           : i32 = 37``
    Total time spent  in the linear dependency checker
                        since the presolve was invoked.
``const MSK_DINF_PRESOLVE_TIME                                  : i32 = 38``
    Total time (in seconds) spent in the presolve
                        since it was invoked.
``const MSK_DINF_PRIMAL_REPAIR_PENALTY_OBJ                      : i32 = 39``
    The optimal objective value of the penalty function.
``const MSK_DINF_QCQO_REFORMULATE_MAX_PERTURBATION              : i32 = 40``
    Maximum absolute diagonal perturbation occuring during the QCQO reformulation.
``const MSK_DINF_QCQO_REFORMULATE_TIME                          : i32 = 41``
    Time spent with conic quadratic reformulation.
``const MSK_DINF_QCQO_REFORMULATE_WORST_CHOLESKY_COLUMN_SCALING : i32 = 42``
    Worst Cholesky column scaling.
``const MSK_DINF_QCQO_REFORMULATE_WORST_CHOLESKY_DIAG_SCALING   : i32 = 43``
    Worst Cholesky diagonal scaling.
``const MSK_DINF_RD_TIME                                        : i32 = 44``
    Time spent reading the data file.
``const MSK_DINF_SIM_DUAL_TIME                                  : i32 = 45``
    Time spent in the dual simplex
                        optimizer since invoking it.
``const MSK_DINF_SIM_FEAS                                       : i32 = 46``
    Feasibility measure reported by the
                        simplex optimizer.
``const MSK_DINF_SIM_NETWORK_DUAL_TIME                          : i32 = 47``
    Time spent in the dual network simplex
                        optimizer since invoking it.
``const MSK_DINF_SIM_NETWORK_PRIMAL_TIME                        : i32 = 48``
    Time spent in the primal network simplex
                        optimizer since invoking it.
``const MSK_DINF_SIM_NETWORK_TIME                               : i32 = 49``
    Time spent in the network simplex
                        optimizer since invoking it.
``const MSK_DINF_SIM_OBJ                                        : i32 = 50``
    Objective value reported by the
                        simplex optimizer.
``const MSK_DINF_SIM_PRIMAL_DUAL_TIME                           : i32 = 51``
    Time spent in the primal-dual simplex optimizer
                        since invoking it.
``const MSK_DINF_SIM_PRIMAL_TIME                                : i32 = 52``
    Time spent in the primal simplex
                        optimizer since invoking it.
``const MSK_DINF_SIM_TIME                                       : i32 = 53``
    Time spent in the simplex
                        optimizer since invoking it.
``const MSK_DINF_SOL_BAS_DUAL_OBJ                               : i32 = 54``
    Dual objective value of the basic solution. Updated by the function updatesolutioninfo.
``const MSK_DINF_SOL_BAS_DVIOLCON                               : i32 = 55``
    Maximal dual bound violation for xx in the basic solution. Updated by the function updatesolutioninfo.
``const MSK_DINF_SOL_BAS_DVIOLVAR                               : i32 = 56``
    Maximal dual bound violation for xx in the basic solution. Updated by the function updatesolutioninfo.
``const MSK_DINF_SOL_BAS_NRM_BARX                               : i32 = 57``
    Infinity norm of barx in the basic solution.
``const MSK_DINF_SOL_BAS_NRM_SLC                                : i32 = 58``
    Infinity norm of slc in the basic solution.
``const MSK_DINF_SOL_BAS_NRM_SLX                                : i32 = 59``
    Infinity norm of slx in the basic solution.
``const MSK_DINF_SOL_BAS_NRM_SUC                                : i32 = 60``
    Infinity norm of suc in the basic solution.
``const MSK_DINF_SOL_BAS_NRM_SUX                                : i32 = 61``
    Infinity norm of sux in the basic solution.
``const MSK_DINF_SOL_BAS_NRM_XC                                 : i32 = 62``
    Infinity norm of xc in the basic solution.
``const MSK_DINF_SOL_BAS_NRM_XX                                 : i32 = 63``
    Infinity norm of xx in the basic solution.
``const MSK_DINF_SOL_BAS_NRM_Y                                  : i32 = 64``
    Infinity norm of Y in the basic solution.
``const MSK_DINF_SOL_BAS_PRIMAL_OBJ                             : i32 = 65``
    Primal objective value of the basic solution. Updated by the function updatesolutioninfo.
``const MSK_DINF_SOL_BAS_PVIOLCON                               : i32 = 66``
    Maximal primal bound violation for xx in the basic solution. Updated by the function updatesolutioninfo.
``const MSK_DINF_SOL_BAS_PVIOLVAR                               : i32 = 67``
    Maximal primal bound violation for xx in the basic solution. Updated by the function updatesolutioninfo.
``const MSK_DINF_SOL_ITG_NRM_BARX                               : i32 = 68``
    Infinity norm of barx in the integer solution.
``const MSK_DINF_SOL_ITG_NRM_XC                                 : i32 = 69``
    Infinity norm of xc in the integer solution.
``const MSK_DINF_SOL_ITG_NRM_XX                                 : i32 = 70``
    Infinity norm of xx in the integer solution.
``const MSK_DINF_SOL_ITG_PRIMAL_OBJ                             : i32 = 71``
    Primal objective value of the integer solution. Updated by the function updatesolutioninfo.
``const MSK_DINF_SOL_ITG_PVIOLBARVAR                            : i32 = 72``
    Maximal primal bound violation for barx in the integer solution. Updated by the function updatesolutioninfo.
``const MSK_DINF_SOL_ITG_PVIOLCON                               : i32 = 73``
    Maximal primal bound violation for xx in the integer solution. Updated by the function updatesolutioninfo.
``const MSK_DINF_SOL_ITG_PVIOLCONES                             : i32 = 74``
    Maximal primal violation for primal conic constraints in the integer solution. Updated by the function updatesolutioninfo.
``const MSK_DINF_SOL_ITG_PVIOLITG                               : i32 = 75``
    Maximal violation for the integer constraints in the integer solution. Updated by the function updatesolutioninfo.
``const MSK_DINF_SOL_ITG_PVIOLVAR                               : i32 = 76``
    Maximal primal bound violation for xx in the integer solution. Updated by the function updatesolutioninfo.
``const MSK_DINF_SOL_ITR_DUAL_OBJ                               : i32 = 77``
    Dual objective value of the interior-point solution. Updated by the function updatesolutioninfo.
``const MSK_DINF_SOL_ITR_DVIOLBARVAR                            : i32 = 78``
    Maximal dual bound violation for barx in the interior-point solution. Updated by the function updatesolutioninfo.
``const MSK_DINF_SOL_ITR_DVIOLCON                               : i32 = 79``
    Maximal dual bound violation for xx in the interior-point solution. Updated by the function updatesolutioninfo.
``const MSK_DINF_SOL_ITR_DVIOLCONES                             : i32 = 80``
    Maximal dual violation for dual conic constraints in the interior-point solution. Updated by the function updatesolutioninfo.
``const MSK_DINF_SOL_ITR_DVIOLVAR                               : i32 = 81``
    Maximal dual bound violation for xx in the interior-point solution. Updated by the function updatesolutioninfo.
``const MSK_DINF_SOL_ITR_NRM_BARS                               : i32 = 82``
    Infinity norm of bars in the interior-point solution.
``const MSK_DINF_SOL_ITR_NRM_BARX                               : i32 = 83``
    Infinity norm of barx in the interior-point solution.
``const MSK_DINF_SOL_ITR_NRM_SLC                                : i32 = 84``
    Infinity norm of slc in the interior-point solution.
``const MSK_DINF_SOL_ITR_NRM_SLX                                : i32 = 85``
    Infinity norm of slx in the interior-point solution.
``const MSK_DINF_SOL_ITR_NRM_SNX                                : i32 = 86``
    Infinity norm of snx in the interior-point solution.
``const MSK_DINF_SOL_ITR_NRM_SUC                                : i32 = 87``
    Infinity norm of suc in the interior-point solution.
``const MSK_DINF_SOL_ITR_NRM_SUX                                : i32 = 88``
    Infinity norm of sux in the interior-point solution.
``const MSK_DINF_SOL_ITR_NRM_XC                                 : i32 = 89``
    Infinity norm of xc in the interior-point solution.
``const MSK_DINF_SOL_ITR_NRM_XX                                 : i32 = 90``
    Infinity norm of xx in the interior-point solution.
``const MSK_DINF_SOL_ITR_NRM_Y                                  : i32 = 91``
    Infinity norm of Y in the interior-point solution.
``const MSK_DINF_SOL_ITR_PRIMAL_OBJ                             : i32 = 92``
    Primal objective value of the interior-point solution. Updated by the function updatesolutioninfo.
``const MSK_DINF_SOL_ITR_PVIOLBARVAR                            : i32 = 93``
    Maximal primal bound violation for barx in the interior-point solution. Updated by the function updatesolutioninfo.
``const MSK_DINF_SOL_ITR_PVIOLCON                               : i32 = 94``
    Maximal primal bound violation for xx in the interior-point solution. Updated by the function updatesolutioninfo.
``const MSK_DINF_SOL_ITR_PVIOLCONES                             : i32 = 95``
    Maximal primal violation for primal conic constraints in the interior-point solution. Updated by the function updatesolutioninfo.
``const MSK_DINF_SOL_ITR_PVIOLVAR                               : i32 = 96``
    Maximal primal bound violation for xx in the interior-point solution. Updated by the function updatesolutioninfo.
``const MSK_DINF_TO_CONIC_TIME                                  : i32 = 97``
    Time spent in the last to conic reformulation.
.. index:: dparam
.. index:: DPAR_...
.. _enum_dparam:
.. _dparam_ana_sol_infeas_tol:
.. _dparam_basis_rel_tol_s:
.. _dparam_basis_tol_s:
.. _dparam_basis_tol_x:
.. _dparam_check_convexity_rel_tol:
.. _dparam_data_tol_aij:
.. _dparam_data_tol_aij_huge:
.. _dparam_data_tol_aij_large:
.. _dparam_data_tol_bound_inf:
.. _dparam_data_tol_bound_wrn:
.. _dparam_data_tol_c_huge:
.. _dparam_data_tol_cj_large:
.. _dparam_data_tol_qij:
.. _dparam_data_tol_x:
.. _dparam_feasrepair_tol:
.. _dparam_intpnt_co_tol_dfeas:
.. _dparam_intpnt_co_tol_infeas:
.. _dparam_intpnt_co_tol_mu_red:
.. _dparam_intpnt_co_tol_near_rel:
.. _dparam_intpnt_co_tol_pfeas:
.. _dparam_intpnt_co_tol_rel_gap:
.. _dparam_intpnt_nl_merit_bal:
.. _dparam_intpnt_nl_tol_dfeas:
.. _dparam_intpnt_nl_tol_mu_red:
.. _dparam_intpnt_nl_tol_near_rel:
.. _dparam_intpnt_nl_tol_pfeas:
.. _dparam_intpnt_nl_tol_rel_gap:
.. _dparam_intpnt_nl_tol_rel_step:
.. _dparam_intpnt_qo_tol_dfeas:
.. _dparam_intpnt_qo_tol_infeas:
.. _dparam_intpnt_qo_tol_mu_red:
.. _dparam_intpnt_qo_tol_near_rel:
.. _dparam_intpnt_qo_tol_pfeas:
.. _dparam_intpnt_qo_tol_rel_gap:
.. _dparam_intpnt_tol_dfeas:
.. _dparam_intpnt_tol_dsafe:
.. _dparam_intpnt_tol_infeas:
.. _dparam_intpnt_tol_mu_red:
.. _dparam_intpnt_tol_path:
.. _dparam_intpnt_tol_pfeas:
.. _dparam_intpnt_tol_psafe:
.. _dparam_intpnt_tol_rel_gap:
.. _dparam_intpnt_tol_rel_step:
.. _dparam_intpnt_tol_step_size:
.. _dparam_lower_obj_cut:
.. _dparam_lower_obj_cut_finite_trh:
.. _dparam_mio_disable_term_time:
.. _dparam_mio_max_time:
.. _dparam_mio_near_tol_abs_gap:
.. _dparam_mio_near_tol_rel_gap:
.. _dparam_mio_rel_gap_const:
.. _dparam_mio_tol_abs_gap:
.. _dparam_mio_tol_abs_relax_int:
.. _dparam_mio_tol_feas:
.. _dparam_mio_tol_rel_dual_bound_improvement:
.. _dparam_mio_tol_rel_gap:
.. _dparam_optimizer_max_time:
.. _dparam_presolve_tol_abs_lindep:
.. _dparam_presolve_tol_aij:
.. _dparam_presolve_tol_rel_lindep:
.. _dparam_presolve_tol_s:
.. _dparam_presolve_tol_x:
.. _dparam_qcqo_reformulate_rel_drop_tol:
.. _dparam_semidefinite_tol_approx:
.. _dparam_sim_lu_tol_rel_piv:
.. _dparam_simplex_abs_tol_piv:
.. _dparam_upper_obj_cut:
.. _dparam_upper_obj_cut_finite_trh:

``dparam``
----------

Double parameters

``const MSK_DPAR_ANA_SOL_INFEAS_TOL                 : i32 = 0``
    If a constraint violates its bound with an amount larger than this value,
                       the constraint name, index and violation will be printed by the solution analyzer.
``const MSK_DPAR_BASIS_REL_TOL_S                    : i32 = 1``
    Maximum relative dual bound violation allowed in an optimal
                        basic solution.
``const MSK_DPAR_BASIS_TOL_S                        : i32 = 2``
    Maximum absolute dual bound violation in
                        an optimal basic solution.
``const MSK_DPAR_BASIS_TOL_X                        : i32 = 3``
    Maximum absolute primal bound violation allowed
                        in an optimal basic solution.
``const MSK_DPAR_CHECK_CONVEXITY_REL_TOL            : i32 = 4``
    Convexity check tolerance.
``const MSK_DPAR_DATA_TOL_AIJ                       : i32 = 5``
    Data tolerance threshold.
``const MSK_DPAR_DATA_TOL_AIJ_HUGE                  : i32 = 6``
    Data tolerance threshold.
``const MSK_DPAR_DATA_TOL_AIJ_LARGE                 : i32 = 7``
    Data tolerance threshold.
``const MSK_DPAR_DATA_TOL_BOUND_INF                 : i32 = 8``
    Data tolerance threshold.
``const MSK_DPAR_DATA_TOL_BOUND_WRN                 : i32 = 9``
    Data tolerance threshold.
``const MSK_DPAR_DATA_TOL_C_HUGE                    : i32 = 10``
    Data tolerance threshold.
``const MSK_DPAR_DATA_TOL_CJ_LARGE                  : i32 = 11``
    Data tolerance threshold.
``const MSK_DPAR_DATA_TOL_QIJ                       : i32 = 12``
    Data tolerance threshold.
``const MSK_DPAR_DATA_TOL_X                         : i32 = 13``
    Data tolerance threshold.
``const MSK_DPAR_FEASREPAIR_TOL                     : i32 = 14``
    Tolerance for constraint enforcing upper bound on
                        sum of weighted violations in feasibility repair.
``const MSK_DPAR_INTPNT_CO_TOL_DFEAS                : i32 = 15``
    Dual feasibility tolerance used by the conic interior-point optimizer.
``const MSK_DPAR_INTPNT_CO_TOL_INFEAS               : i32 = 16``
    Infeasibility tolerance for the conic solver.
``const MSK_DPAR_INTPNT_CO_TOL_MU_RED               : i32 = 17``
    Optimality tolerance for the conic solver.
``const MSK_DPAR_INTPNT_CO_TOL_NEAR_REL             : i32 = 18``
    Optimality tolerance for the conic solver.
``const MSK_DPAR_INTPNT_CO_TOL_PFEAS                : i32 = 19``
    Primal feasibility tolerance used by the conic interior-point optimizer.
``const MSK_DPAR_INTPNT_CO_TOL_REL_GAP              : i32 = 20``
    Relative gap termination tolerance used by the
                        conic interior-point optimizer.
``const MSK_DPAR_INTPNT_NL_MERIT_BAL                : i32 = 21``
    Controls if the complementarity and infeasibility is converging to zero
                        at about equal rates.
``const MSK_DPAR_INTPNT_NL_TOL_DFEAS                : i32 = 22``
    Dual feasibility tolerance used when a nonlinear
                        model is solved.
``const MSK_DPAR_INTPNT_NL_TOL_MU_RED               : i32 = 23``
    Relative complementarity gap tolerance.
``const MSK_DPAR_INTPNT_NL_TOL_NEAR_REL             : i32 = 24``
    Nonlinear solver optimality tolerance parameter.
``const MSK_DPAR_INTPNT_NL_TOL_PFEAS                : i32 = 25``
    Primal feasibility tolerance used when a nonlinear
                        model is solved.
``const MSK_DPAR_INTPNT_NL_TOL_REL_GAP              : i32 = 26``
    Relative gap termination tolerance for nonlinear problems.
``const MSK_DPAR_INTPNT_NL_TOL_REL_STEP             : i32 = 27``
    Relative step size to the boundary
                        for general nonlinear optimization problems.
``const MSK_DPAR_INTPNT_QO_TOL_DFEAS                : i32 = 28``
    Dual feasibility tolerance used when the interior-point optimizer is applied to a quadratic optimization problem..
``const MSK_DPAR_INTPNT_QO_TOL_INFEAS               : i32 = 29``
    Infeasibility tolerance employed when a quadratic optimization problem is solved.
``const MSK_DPAR_INTPNT_QO_TOL_MU_RED               : i32 = 30``
    Optimality tolerance employed when a quadratic optimization problem is solved.
``const MSK_DPAR_INTPNT_QO_TOL_NEAR_REL             : i32 = 31``
    Optimality tolerance employed when a quadratic optimization problem is solved.
``const MSK_DPAR_INTPNT_QO_TOL_PFEAS                : i32 = 32``
    Primal feasibility tolerance used when the interior-point optimizer is applied to a quadratic optimization problem.
``const MSK_DPAR_INTPNT_QO_TOL_REL_GAP              : i32 = 33``
    Relative gap termination tolerance used when the
                      interior-point optimizer is applied to a quadratic
                      optimization problem.
``const MSK_DPAR_INTPNT_TOL_DFEAS                   : i32 = 34``
    Dual feasibility tolerance used for
                        linear and quadratic optimization problems.
``const MSK_DPAR_INTPNT_TOL_DSAFE                   : i32 = 35``
    Controls the interior-point dual starting point.
``const MSK_DPAR_INTPNT_TOL_INFEAS                  : i32 = 36``
    Nonlinear solver infeasibility tolerance parameter.
``const MSK_DPAR_INTPNT_TOL_MU_RED                  : i32 = 37``
    Relative complementarity gap tolerance.
``const MSK_DPAR_INTPNT_TOL_PATH                    : i32 = 38``
    interior-point centering aggressiveness.
``const MSK_DPAR_INTPNT_TOL_PFEAS                   : i32 = 39``
    Primal feasibility tolerance used for
                        linear and quadratic optimization problems.
``const MSK_DPAR_INTPNT_TOL_PSAFE                   : i32 = 40``
    Controls the interior-point primal starting point.
``const MSK_DPAR_INTPNT_TOL_REL_GAP                 : i32 = 41``
    Relative gap termination tolerance.
``const MSK_DPAR_INTPNT_TOL_REL_STEP                : i32 = 42``
    Relative step size to the boundary
                        for linear and quadratic optimization problems.
``const MSK_DPAR_INTPNT_TOL_STEP_SIZE               : i32 = 43``
    If the step size falls below the value of this
                        parameter, then the interior-point optimizer
                        assumes that it is stalled. In other words the
                        interior-point optimizer does not make any
                        progress and therefore it is better stop.
``const MSK_DPAR_LOWER_OBJ_CUT                      : i32 = 44``
    Objective bound.
``const MSK_DPAR_LOWER_OBJ_CUT_FINITE_TRH           : i32 = 45``
    Objective bound.
``const MSK_DPAR_MIO_DISABLE_TERM_TIME              : i32 = 46``
    Certain termination criteria is disabled within the mixed-integer optimizer for period time specified by the parameter.
``const MSK_DPAR_MIO_MAX_TIME                       : i32 = 47``
    Time limit for the mixed-integer optimizer.
``const MSK_DPAR_MIO_NEAR_TOL_ABS_GAP               : i32 = 48``
    Relaxed absolute optimality tolerance employed by the mixed-integer optimizer.
``const MSK_DPAR_MIO_NEAR_TOL_REL_GAP               : i32 = 49``
    The mixed-integer optimizer is terminated when this tolerance is satisfied.
``const MSK_DPAR_MIO_REL_GAP_CONST                  : i32 = 50``
    This value is used to compute the relative gap for the solution to an integer optimization problem.
``const MSK_DPAR_MIO_TOL_ABS_GAP                    : i32 = 51``
    Absolute optimality tolerance employed by the mixed-integer optimizer.
``const MSK_DPAR_MIO_TOL_ABS_RELAX_INT              : i32 = 52``
    Integer constraint tolerance.
``const MSK_DPAR_MIO_TOL_FEAS                       : i32 = 53``
    Feasibility tolerance for mixed integer solver.
``const MSK_DPAR_MIO_TOL_REL_DUAL_BOUND_IMPROVEMENT : i32 = 54``
    Controls cut generation for mixed-integer optimizer.
``const MSK_DPAR_MIO_TOL_REL_GAP                    : i32 = 55``
    Relative optimality tolerance employed by the mixed-integer optimizer.
``const MSK_DPAR_OPTIMIZER_MAX_TIME                 : i32 = 56``
    Solver time limit.
``const MSK_DPAR_PRESOLVE_TOL_ABS_LINDEP            : i32 = 57``
    Absolute tolerance employed by the 
                        linear dependency checker.
``const MSK_DPAR_PRESOLVE_TOL_AIJ                   : i32 = 58``
    Absolute zero tolerance employed for constraint coefficients in the presolve.
``const MSK_DPAR_PRESOLVE_TOL_REL_LINDEP            : i32 = 59``
    Relative tolerance employed by the 
                        linear dependency checker.
``const MSK_DPAR_PRESOLVE_TOL_S                     : i32 = 60``
    Absolute zero tolerance employed for slack variables in the presolve.
``const MSK_DPAR_PRESOLVE_TOL_X                     : i32 = 61``
    Absolute zero tolerance employed for variables in the presolve.
``const MSK_DPAR_QCQO_REFORMULATE_REL_DROP_TOL      : i32 = 62``
    This parameter determines when columns are dropped in incomplete Cholesky factorization doing reformulation of quadratic problems.
``const MSK_DPAR_SEMIDEFINITE_TOL_APPROX            : i32 = 63``
    TBD
``const MSK_DPAR_SIM_LU_TOL_REL_PIV                 : i32 = 64``
    Relative pivot tolerance employed when computing the LU factorization of the basis matrix.
``const MSK_DPAR_SIMPLEX_ABS_TOL_PIV                : i32 = 65``
    Absolute pivot tolerance employed by the simplex optimizers.
``const MSK_DPAR_UPPER_OBJ_CUT                      : i32 = 66``
    Objective bound.
``const MSK_DPAR_UPPER_OBJ_CUT_FINITE_TRH           : i32 = 67``
    Objective bound.
.. index:: feasrepairtype
.. index:: FEASREPAIR_...
.. _enum_feasrepairtype:
.. _feasrepairtype_optimize_combined:
.. _feasrepairtype_optimize_none:
.. _feasrepairtype_optimize_penalty:

``feasrepairtype``
------------------

Feasibility repair types

``const MSK_FEASREPAIR_OPTIMIZE_COMBINED : i32 = 2``
    Minimize with original objective subject to minimal weighted violation of bounds.
``const MSK_FEASREPAIR_OPTIMIZE_NONE     : i32 = 0``
    Do not optimize the feasibility repair problem.
``const MSK_FEASREPAIR_OPTIMIZE_PENALTY  : i32 = 1``
    Minimize weighted sum of violations.
.. index:: feature
.. index:: FEATURE_...
.. _enum_feature:
.. _feature_pton:
.. _feature_pts:

``feature``
-----------

License feature

``const MSK_FEATURE_PTON : i32 = 1``
    Nonlinear extension.
``const MSK_FEATURE_PTS  : i32 = 0``
    Base system.
.. index:: iinfitem
.. index:: IINF_...
.. _enum_iinfitem:
.. _iinfitem_ana_pro_num_con:
.. _iinfitem_ana_pro_num_con_eq:
.. _iinfitem_ana_pro_num_con_fr:
.. _iinfitem_ana_pro_num_con_lo:
.. _iinfitem_ana_pro_num_con_ra:
.. _iinfitem_ana_pro_num_con_up:
.. _iinfitem_ana_pro_num_var:
.. _iinfitem_ana_pro_num_var_bin:
.. _iinfitem_ana_pro_num_var_cont:
.. _iinfitem_ana_pro_num_var_eq:
.. _iinfitem_ana_pro_num_var_fr:
.. _iinfitem_ana_pro_num_var_int:
.. _iinfitem_ana_pro_num_var_lo:
.. _iinfitem_ana_pro_num_var_ra:
.. _iinfitem_ana_pro_num_var_up:
.. _iinfitem_concurrent_winning_optimizer:
.. _iinfitem_intpnt_factor_dim_dense:
.. _iinfitem_intpnt_iter:
.. _iinfitem_intpnt_num_threads:
.. _iinfitem_intpnt_solve_dual:
.. _iinfitem_mio_absgap_satisfied:
.. _iinfitem_mio_clique_table_size:
.. _iinfitem_mio_construct_num_roundings:
.. _iinfitem_mio_construct_solution:
.. _iinfitem_mio_initial_solution:
.. _iinfitem_mio_near_absgap_satisfied:
.. _iinfitem_mio_near_relgap_satisfied:
.. _iinfitem_mio_node_depth:
.. _iinfitem_mio_num_active_nodes:
.. _iinfitem_mio_num_branch:
.. _iinfitem_mio_num_clique_cuts:
.. _iinfitem_mio_num_cmir_cuts:
.. _iinfitem_mio_num_gomory_cuts:
.. _iinfitem_mio_num_implied_bound_cuts:
.. _iinfitem_mio_num_int_solutions:
.. _iinfitem_mio_num_knapsack_cover_cuts:
.. _iinfitem_mio_num_relax:
.. _iinfitem_mio_num_repeated_presolve:
.. _iinfitem_mio_numcon:
.. _iinfitem_mio_numint:
.. _iinfitem_mio_numvar:
.. _iinfitem_mio_obj_bound_defined:
.. _iinfitem_mio_presolved_numbin:
.. _iinfitem_mio_presolved_numcon:
.. _iinfitem_mio_presolved_numcont:
.. _iinfitem_mio_presolved_numint:
.. _iinfitem_mio_presolved_numvar:
.. _iinfitem_mio_relgap_satisfied:
.. _iinfitem_mio_total_num_cuts:
.. _iinfitem_mio_user_obj_cut:
.. _iinfitem_opt_numcon:
.. _iinfitem_opt_numvar:
.. _iinfitem_optimize_response:
.. _iinfitem_rd_numbarvar:
.. _iinfitem_rd_numcon:
.. _iinfitem_rd_numcone:
.. _iinfitem_rd_numintvar:
.. _iinfitem_rd_numq:
.. _iinfitem_rd_numvar:
.. _iinfitem_rd_protype:
.. _iinfitem_sim_dual_deg_iter:
.. _iinfitem_sim_dual_hotstart:
.. _iinfitem_sim_dual_hotstart_lu:
.. _iinfitem_sim_dual_inf_iter:
.. _iinfitem_sim_dual_iter:
.. _iinfitem_sim_network_dual_deg_iter:
.. _iinfitem_sim_network_dual_hotstart:
.. _iinfitem_sim_network_dual_hotstart_lu:
.. _iinfitem_sim_network_dual_inf_iter:
.. _iinfitem_sim_network_dual_iter:
.. _iinfitem_sim_network_primal_deg_iter:
.. _iinfitem_sim_network_primal_hotstart:
.. _iinfitem_sim_network_primal_hotstart_lu:
.. _iinfitem_sim_network_primal_inf_iter:
.. _iinfitem_sim_network_primal_iter:
.. _iinfitem_sim_numcon:
.. _iinfitem_sim_numvar:
.. _iinfitem_sim_primal_deg_iter:
.. _iinfitem_sim_primal_dual_deg_iter:
.. _iinfitem_sim_primal_dual_hotstart:
.. _iinfitem_sim_primal_dual_hotstart_lu:
.. _iinfitem_sim_primal_dual_inf_iter:
.. _iinfitem_sim_primal_dual_iter:
.. _iinfitem_sim_primal_hotstart:
.. _iinfitem_sim_primal_hotstart_lu:
.. _iinfitem_sim_primal_inf_iter:
.. _iinfitem_sim_primal_iter:
.. _iinfitem_sim_solve_dual:
.. _iinfitem_sol_bas_prosta:
.. _iinfitem_sol_bas_solsta:
.. _iinfitem_sol_itg_prosta:
.. _iinfitem_sol_itg_solsta:
.. _iinfitem_sol_itr_prosta:
.. _iinfitem_sol_itr_solsta:
.. _iinfitem_sto_num_a_realloc:

``iinfitem``
------------

Integer information items.

``const MSK_IINF_ANA_PRO_NUM_CON                : i32 = 0``
    Number of constraints in the problem.
``const MSK_IINF_ANA_PRO_NUM_CON_EQ             : i32 = 1``
    Number of equality constraints.
``const MSK_IINF_ANA_PRO_NUM_CON_FR             : i32 = 2``
    Number of unbounded constraints.
``const MSK_IINF_ANA_PRO_NUM_CON_LO             : i32 = 3``
    Number of constraints with a lower bound and an
                      infinite upper bound.
``const MSK_IINF_ANA_PRO_NUM_CON_RA             : i32 = 4``
    Number of constraints with finite lower and upper bounds.
``const MSK_IINF_ANA_PRO_NUM_CON_UP             : i32 = 5``
    Number of constraints with an upper bound and an infinite lower bound.
``const MSK_IINF_ANA_PRO_NUM_VAR                : i32 = 6``
    Number of variables in the problem.
``const MSK_IINF_ANA_PRO_NUM_VAR_BIN            : i32 = 7``
    Number of binary variables.
``const MSK_IINF_ANA_PRO_NUM_VAR_CONT           : i32 = 8``
    Number of continuous variables.
``const MSK_IINF_ANA_PRO_NUM_VAR_EQ             : i32 = 9``
    Number of fixed variables.
``const MSK_IINF_ANA_PRO_NUM_VAR_FR             : i32 = 10``
    Number of unbounded constraints.
``const MSK_IINF_ANA_PRO_NUM_VAR_INT            : i32 = 11``
    Number of general integer variables.
``const MSK_IINF_ANA_PRO_NUM_VAR_LO             : i32 = 12``
    Number of variables with a lower bound and an
                      infinite upper bound.
``const MSK_IINF_ANA_PRO_NUM_VAR_RA             : i32 = 13``
    Number of variables with finite lower and upper bounds.
``const MSK_IINF_ANA_PRO_NUM_VAR_UP             : i32 = 14``
    Number of variables with an upper bound and an infinite lower bound.
``const MSK_IINF_CONCURRENT_WINNING_OPTIMIZER   : i32 = 15``
    The type of the optimizer that finished first in a concurrent optimization.
``const MSK_IINF_INTPNT_FACTOR_DIM_DENSE        : i32 = 16``
    Dimension of the dense sub system in factorization.
``const MSK_IINF_INTPNT_ITER                    : i32 = 17``
    Number of interior-point iterations since invoking the interior-point optimizer.
``const MSK_IINF_INTPNT_NUM_THREADS             : i32 = 18``
    Number of threads that the interior-point optimizer is using.
``const MSK_IINF_INTPNT_SOLVE_DUAL              : i32 = 19``
    Non-zero if the interior-point optimizer is solving the dual problem.
``const MSK_IINF_MIO_ABSGAP_SATISFIED           : i32 = 20``
    Non-zero if absolute gap is within tolerances.
``const MSK_IINF_MIO_CLIQUE_TABLE_SIZE          : i32 = 21``
    Size of the clique table.
``const MSK_IINF_MIO_CONSTRUCT_NUM_ROUNDINGS    : i32 = 22``
    Number of values in the integer solution that is rounded to an integer value.
``const MSK_IINF_MIO_CONSTRUCT_SOLUTION         : i32 = 23``
    If this item is positive, then |mosek| successfully constructed an initial integer feasible solution.
``const MSK_IINF_MIO_INITIAL_SOLUTION           : i32 = 24``
    Is non-zero if an initial integer solution is specified.
``const MSK_IINF_MIO_NEAR_ABSGAP_SATISFIED      : i32 = 25``
    Non-zero if absolute gap is within relaxed tolerances.
``const MSK_IINF_MIO_NEAR_RELGAP_SATISFIED      : i32 = 26``
    Non-zero if relative gap is within relaxed tolerances.
``const MSK_IINF_MIO_NODE_DEPTH                 : i32 = 27``
    Depth of the last node solved.
``const MSK_IINF_MIO_NUM_ACTIVE_NODES           : i32 = 28``
    Number of active branch bound nodes.
``const MSK_IINF_MIO_NUM_BRANCH                 : i32 = 29``
    Number of branches performed during the optimization.
``const MSK_IINF_MIO_NUM_CLIQUE_CUTS            : i32 = 30``
    Number of clique cuts.
``const MSK_IINF_MIO_NUM_CMIR_CUTS              : i32 = 31``
    Number of Complemented Mixed Integer Rounding (CMIR) cuts.
``const MSK_IINF_MIO_NUM_GOMORY_CUTS            : i32 = 32``
    Number of Gomory cuts.
``const MSK_IINF_MIO_NUM_IMPLIED_BOUND_CUTS     : i32 = 33``
    Number of implied bound cuts.
``const MSK_IINF_MIO_NUM_INT_SOLUTIONS          : i32 = 34``
    Number of integer feasible solutions that has been found.
``const MSK_IINF_MIO_NUM_KNAPSACK_COVER_CUTS    : i32 = 35``
    Number of clique cuts.
``const MSK_IINF_MIO_NUM_RELAX                  : i32 = 36``
    Number of relaxations solved during the optimization.
``const MSK_IINF_MIO_NUM_REPEATED_PRESOLVE      : i32 = 37``
    Number of times presolve was repeated at root.
``const MSK_IINF_MIO_NUMCON                     : i32 = 38``
    Number of constraints in the problem solved by the mixed-integer optimizer.
``const MSK_IINF_MIO_NUMINT                     : i32 = 39``
    Number of integer variables in the problem solved be the mixed-integer optimizer.
``const MSK_IINF_MIO_NUMVAR                     : i32 = 40``
    Number of variables in the problem solved by the mixed-integer optimizer.
``const MSK_IINF_MIO_OBJ_BOUND_DEFINED          : i32 = 41``
    Non-zero if a valid objective bound has been found, otherwise zero.
``const MSK_IINF_MIO_PRESOLVED_NUMBIN           : i32 = 42``
    Number of binary variables in the problem solved be the mixed-integer optimizer.
``const MSK_IINF_MIO_PRESOLVED_NUMCON           : i32 = 43``
    Number of constraints in the presolved problem.
``const MSK_IINF_MIO_PRESOLVED_NUMCONT          : i32 = 44``
    Number of continuous variables in the problem solved be the mixed-integer optimizer.
``const MSK_IINF_MIO_PRESOLVED_NUMINT           : i32 = 45``
    Number of integer variables in the presolved problem.
``const MSK_IINF_MIO_PRESOLVED_NUMVAR           : i32 = 46``
    Number of variables in the presolved problem.
``const MSK_IINF_MIO_RELGAP_SATISFIED           : i32 = 47``
    Non-zero if relative gap is within tolerances.
``const MSK_IINF_MIO_TOTAL_NUM_CUTS             : i32 = 48``
    Total number of cuts generated by the mixed-integer optimizer.
``const MSK_IINF_MIO_USER_OBJ_CUT               : i32 = 49``
    If it is non-zero, then the objective cut is used.
``const MSK_IINF_OPT_NUMCON                     : i32 = 50``
    Number of constraints in the problem solved when the optimizer is called.
``const MSK_IINF_OPT_NUMVAR                     : i32 = 51``
    Number of variables in the problem solved when the optimizer is called
``const MSK_IINF_OPTIMIZE_RESPONSE              : i32 = 52``
    The response code returned by optimize.
``const MSK_IINF_RD_NUMBARVAR                   : i32 = 53``
    Number of variables read.
``const MSK_IINF_RD_NUMCON                      : i32 = 54``
    Number of constraints read.
``const MSK_IINF_RD_NUMCONE                     : i32 = 55``
    Number of conic constraints read.
``const MSK_IINF_RD_NUMINTVAR                   : i32 = 56``
    Number of integer-constrained variables read.
``const MSK_IINF_RD_NUMQ                        : i32 = 57``
    Number of nonempty Q matrices read.
``const MSK_IINF_RD_NUMVAR                      : i32 = 58``
    Number of variables read.
``const MSK_IINF_RD_PROTYPE                     : i32 = 59``
    Problem type.
``const MSK_IINF_SIM_DUAL_DEG_ITER              : i32 = 60``
    The number of dual degenerate iterations.
``const MSK_IINF_SIM_DUAL_HOTSTART              : i32 = 61``
    If 1 then the dual simplex algorithm is solving from an advanced basis.
``const MSK_IINF_SIM_DUAL_HOTSTART_LU           : i32 = 62``
    If 1 then a valid basis factorization of full rank was located and used by the dual simplex algorithm.
``const MSK_IINF_SIM_DUAL_INF_ITER              : i32 = 63``
    The number of iterations taken with dual infeasibility.
``const MSK_IINF_SIM_DUAL_ITER                  : i32 = 64``
    Number of dual simplex iterations during the last optimization.
``const MSK_IINF_SIM_NETWORK_DUAL_DEG_ITER      : i32 = 65``
    The number of dual network degenerate iterations.
``const MSK_IINF_SIM_NETWORK_DUAL_HOTSTART      : i32 = 66``
    If 1 then the dual network simplex algorithm is solving from an advanced basis.
``const MSK_IINF_SIM_NETWORK_DUAL_HOTSTART_LU   : i32 = 67``
    If 1 then a valid basis factorization of full rank was located and used by the dual network simplex algorithm.
``const MSK_IINF_SIM_NETWORK_DUAL_INF_ITER      : i32 = 68``
    The number of iterations taken with dual infeasibility in the network optimizer.
``const MSK_IINF_SIM_NETWORK_DUAL_ITER          : i32 = 69``
    Number of dual network simplex iterations during the last optimization.
``const MSK_IINF_SIM_NETWORK_PRIMAL_DEG_ITER    : i32 = 70``
    The number of primal network degenerate iterations.
``const MSK_IINF_SIM_NETWORK_PRIMAL_HOTSTART    : i32 = 71``
    If 1 then the primal network simplex algorithm is solving from an advanced basis.
``const MSK_IINF_SIM_NETWORK_PRIMAL_HOTSTART_LU : i32 = 72``
    If 1 then a valid basis factorization of full rank was located and used by the primal network simplex algorithm.
``const MSK_IINF_SIM_NETWORK_PRIMAL_INF_ITER    : i32 = 73``
    The number of iterations taken with primal infeasibility in the network optimizer.
``const MSK_IINF_SIM_NETWORK_PRIMAL_ITER        : i32 = 74``
    Number of primal network simplex iterations during the last optimization.
``const MSK_IINF_SIM_NUMCON                     : i32 = 75``
    Number of constraints in the problem solved by the simplex optimizer.
``const MSK_IINF_SIM_NUMVAR                     : i32 = 76``
    Number of variables in the problem solved by the simplex optimizer.
``const MSK_IINF_SIM_PRIMAL_DEG_ITER            : i32 = 77``
    The number of primal degenerate iterations.
``const MSK_IINF_SIM_PRIMAL_DUAL_DEG_ITER       : i32 = 78``
    The number of degenerate major iterations taken by the primal dual simplex algorithm.
``const MSK_IINF_SIM_PRIMAL_DUAL_HOTSTART       : i32 = 79``
    If 1 then the primal dual simplex algorithm is solving from an advanced basis.
``const MSK_IINF_SIM_PRIMAL_DUAL_HOTSTART_LU    : i32 = 80``
    If 1 then a valid basis factorization of full rank was located and used by the primal dual simplex algorithm.
``const MSK_IINF_SIM_PRIMAL_DUAL_INF_ITER       : i32 = 81``
    The number of master iterations with dual infeasibility taken by the primal dual simplex algorithm.
``const MSK_IINF_SIM_PRIMAL_DUAL_ITER           : i32 = 82``
    Number of primal dual simplex iterations during the last optimization.
``const MSK_IINF_SIM_PRIMAL_HOTSTART            : i32 = 83``
    If 1 then the primal simplex algorithm is solving from an advanced basis.
``const MSK_IINF_SIM_PRIMAL_HOTSTART_LU         : i32 = 84``
    If 1 then a valid basis factorization of full rank was located and used by the primal simplex algorithm.
``const MSK_IINF_SIM_PRIMAL_INF_ITER            : i32 = 85``
    The number of iterations taken with primal infeasibility.
``const MSK_IINF_SIM_PRIMAL_ITER                : i32 = 86``
    Number of primal simplex iterations during the last optimization.
``const MSK_IINF_SIM_SOLVE_DUAL                 : i32 = 87``
    Is non-zero if dual problem is solved.
``const MSK_IINF_SOL_BAS_PROSTA                 : i32 = 88``
    Problem status of the basic solution. Updated after each optimization.
``const MSK_IINF_SOL_BAS_SOLSTA                 : i32 = 89``
    Solution status of the basic solution. Updated after each optimization.
``const MSK_IINF_SOL_ITG_PROSTA                 : i32 = 90``
    Problem status of the integer solution. Updated after each optimization.
``const MSK_IINF_SOL_ITG_SOLSTA                 : i32 = 91``
    Solution status of the integer solution. Updated after each optimization.
``const MSK_IINF_SOL_ITR_PROSTA                 : i32 = 92``
    Problem status of the interior-point solution. Updated after each optimization.
``const MSK_IINF_SOL_ITR_SOLSTA                 : i32 = 93``
    Solution status of the interior-point solution. Updated after each optimization.
``const MSK_IINF_STO_NUM_A_REALLOC              : i32 = 94``
    Number of times the storage for storing the linear coefficient matrix has been changed.
.. index:: inftype
.. index:: INF_...
.. _enum_inftype:
.. _inftype_dou_type:
.. _inftype_int_type:
.. _inftype_lint_type:

``inftype``
-----------

Information item types

``const MSK_INF_DOU_TYPE  : i32 = 0``
    Is a double information type.
``const MSK_INF_INT_TYPE  : i32 = 1``
    Is an integer.
``const MSK_INF_LINT_TYPE : i32 = 2``
    Is a long integer.
.. index:: intpnthotstart
.. index:: INTPNT_HOTSTART_...
.. _enum_intpnthotstart:
.. _intpnthotstart_dual:
.. _intpnthotstart_none:
.. _intpnthotstart_primal:
.. _intpnthotstart_primal_dual:

``intpnthotstart``
------------------

Hot-start type employed by the interior-point optimizers.

``const MSK_INTPNT_HOTSTART_DUAL        : i32 = 2``
    The interior-point optimizer exploits the dual solution only.
``const MSK_INTPNT_HOTSTART_NONE        : i32 = 0``
    The interior-point optimizer performs a coldstart.
``const MSK_INTPNT_HOTSTART_PRIMAL      : i32 = 1``
    The interior-point optimizer exploits the primal solution only.
``const MSK_INTPNT_HOTSTART_PRIMAL_DUAL : i32 = 3``
    The interior-point optimizer exploits both the primal and dual solution.
.. index:: iomode
.. index:: IOMODE_...
.. _enum_iomode:
.. _iomode_read:
.. _iomode_readwrite:
.. _iomode_write:

``iomode``
----------

Input/output modes

``const MSK_IOMODE_READ      : i32 = 0``
    The file is read-only.
``const MSK_IOMODE_READWRITE : i32 = 2``
    The file is to read and written.
``const MSK_IOMODE_WRITE     : i32 = 1``
    The file is write-only. If the file exists then it is
                        truncated when it is opened. Otherwise it is created when it is opened.
.. index:: iparam
.. index:: IPAR_...
.. _enum_iparam:
.. _iparam_ana_sol_basis:
.. _iparam_ana_sol_print_violated:
.. _iparam_auto_sort_a_before_opt:
.. _iparam_auto_update_sol_info:
.. _iparam_basis_solve_use_plus_one:
.. _iparam_bi_clean_optimizer:
.. _iparam_bi_ignore_max_iter:
.. _iparam_bi_ignore_num_error:
.. _iparam_bi_max_iterations:
.. _iparam_cache_license:
.. _iparam_check_convexity:
.. _iparam_compress_statfile:
.. _iparam_concurrent_num_optimizers:
.. _iparam_concurrent_priority_dual_simplex:
.. _iparam_concurrent_priority_free_simplex:
.. _iparam_concurrent_priority_intpnt:
.. _iparam_concurrent_priority_primal_simplex:
.. _iparam_feasrepair_optimize:
.. _iparam_infeas_generic_names:
.. _iparam_infeas_prefer_primal:
.. _iparam_infeas_report_auto:
.. _iparam_infeas_report_level:
.. _iparam_intpnt_basis:
.. _iparam_intpnt_diff_step:
.. _iparam_intpnt_hotstart:
.. _iparam_intpnt_max_iterations:
.. _iparam_intpnt_max_num_cor:
.. _iparam_intpnt_max_num_refinement_steps:
.. _iparam_intpnt_multi_thread:
.. _iparam_intpnt_off_col_trh:
.. _iparam_intpnt_order_method:
.. _iparam_intpnt_regularization_use:
.. _iparam_intpnt_scaling:
.. _iparam_intpnt_solve_form:
.. _iparam_intpnt_starting_point:
.. _iparam_license_debug:
.. _iparam_license_pause_time:
.. _iparam_license_suppress_expire_wrns:
.. _iparam_license_trh_expiry_wrn:
.. _iparam_license_wait:
.. _iparam_log:
.. _iparam_log_ana_pro:
.. _iparam_log_bi:
.. _iparam_log_bi_freq:
.. _iparam_log_check_convexity:
.. _iparam_log_concurrent:
.. _iparam_log_cut_second_opt:
.. _iparam_log_expand:
.. _iparam_log_factor:
.. _iparam_log_feas_repair:
.. _iparam_log_file:
.. _iparam_log_head:
.. _iparam_log_infeas_ana:
.. _iparam_log_intpnt:
.. _iparam_log_mio:
.. _iparam_log_mio_freq:
.. _iparam_log_optimizer:
.. _iparam_log_order:
.. _iparam_log_presolve:
.. _iparam_log_response:
.. _iparam_log_sensitivity:
.. _iparam_log_sensitivity_opt:
.. _iparam_log_sim:
.. _iparam_log_sim_freq:
.. _iparam_log_sim_minor:
.. _iparam_log_storage:
.. _iparam_max_num_warnings:
.. _iparam_mio_branch_dir:
.. _iparam_mio_construct_sol:
.. _iparam_mio_cut_clique:
.. _iparam_mio_cut_cmir:
.. _iparam_mio_cut_gmi:
.. _iparam_mio_cut_implied_bound:
.. _iparam_mio_cut_knapsack_cover:
.. _iparam_mio_cut_selection_level:
.. _iparam_mio_heuristic_level:
.. _iparam_mio_max_num_branches:
.. _iparam_mio_max_num_relaxs:
.. _iparam_mio_max_num_solutions:
.. _iparam_mio_mode:
.. _iparam_mio_mt_user_cb:
.. _iparam_mio_node_optimizer:
.. _iparam_mio_node_selection:
.. _iparam_mio_perspective_reformulate:
.. _iparam_mio_probing_level:
.. _iparam_mio_rins_max_nodes:
.. _iparam_mio_root_optimizer:
.. _iparam_mio_root_repeat_presolve_level:
.. _iparam_mio_vb_detection_level:
.. _iparam_mt_spincount:
.. _iparam_num_threads:
.. _iparam_opf_max_terms_per_line:
.. _iparam_opf_write_header:
.. _iparam_opf_write_hints:
.. _iparam_opf_write_parameters:
.. _iparam_opf_write_problem:
.. _iparam_opf_write_sol_bas:
.. _iparam_opf_write_sol_itg:
.. _iparam_opf_write_sol_itr:
.. _iparam_opf_write_solutions:
.. _iparam_optimizer:
.. _iparam_param_read_case_name:
.. _iparam_param_read_ign_error:
.. _iparam_presolve_eliminator_max_fill:
.. _iparam_presolve_eliminator_max_num_tries:
.. _iparam_presolve_level:
.. _iparam_presolve_lindep_abs_work_trh:
.. _iparam_presolve_lindep_rel_work_trh:
.. _iparam_presolve_lindep_use:
.. _iparam_presolve_max_num_reductions:
.. _iparam_presolve_use:
.. _iparam_primal_repair_optimizer:
.. _iparam_read_data_compressed:
.. _iparam_read_data_format:
.. _iparam_read_debug:
.. _iparam_read_keep_free_con:
.. _iparam_read_lp_drop_new_vars_in_bou:
.. _iparam_read_lp_quoted_names:
.. _iparam_read_mps_format:
.. _iparam_read_mps_width:
.. _iparam_read_task_ignore_param:
.. _iparam_sensitivity_all:
.. _iparam_sensitivity_optimizer:
.. _iparam_sensitivity_type:
.. _iparam_sim_basis_factor_use:
.. _iparam_sim_degen:
.. _iparam_sim_dual_crash:
.. _iparam_sim_dual_phaseone_method:
.. _iparam_sim_dual_restrict_selection:
.. _iparam_sim_dual_selection:
.. _iparam_sim_exploit_dupvec:
.. _iparam_sim_hotstart:
.. _iparam_sim_hotstart_lu:
.. _iparam_sim_integer:
.. _iparam_sim_max_iterations:
.. _iparam_sim_max_num_setbacks:
.. _iparam_sim_non_singular:
.. _iparam_sim_primal_crash:
.. _iparam_sim_primal_phaseone_method:
.. _iparam_sim_primal_restrict_selection:
.. _iparam_sim_primal_selection:
.. _iparam_sim_refactor_freq:
.. _iparam_sim_reformulation:
.. _iparam_sim_save_lu:
.. _iparam_sim_scaling:
.. _iparam_sim_scaling_method:
.. _iparam_sim_solve_form:
.. _iparam_sim_stability_priority:
.. _iparam_sim_switch_optimizer:
.. _iparam_sol_filter_keep_basic:
.. _iparam_sol_filter_keep_ranged:
.. _iparam_sol_read_name_width:
.. _iparam_sol_read_width:
.. _iparam_solution_callback:
.. _iparam_timing_level:
.. _iparam_write_bas_constraints:
.. _iparam_write_bas_head:
.. _iparam_write_bas_variables:
.. _iparam_write_data_compressed:
.. _iparam_write_data_format:
.. _iparam_write_data_param:
.. _iparam_write_free_con:
.. _iparam_write_generic_names:
.. _iparam_write_generic_names_io:
.. _iparam_write_ignore_incompatible_conic_items:
.. _iparam_write_ignore_incompatible_items:
.. _iparam_write_ignore_incompatible_nl_items:
.. _iparam_write_ignore_incompatible_psd_items:
.. _iparam_write_int_constraints:
.. _iparam_write_int_head:
.. _iparam_write_int_variables:
.. _iparam_write_lp_full_obj:
.. _iparam_write_lp_line_width:
.. _iparam_write_lp_quoted_names:
.. _iparam_write_lp_strict_format:
.. _iparam_write_lp_terms_per_line:
.. _iparam_write_mps_format:
.. _iparam_write_mps_int:
.. _iparam_write_precision:
.. _iparam_write_sol_barvariables:
.. _iparam_write_sol_constraints:
.. _iparam_write_sol_head:
.. _iparam_write_sol_ignore_invalid_names:
.. _iparam_write_sol_variables:
.. _iparam_write_task_inc_sol:
.. _iparam_write_xml_mode:

``iparam``
----------

Integer parameters

``const MSK_IPAR_ANA_SOL_BASIS                         : i32 = 0``
    Controls whether the basis matrix is analyzed in solution analyzer.
``const MSK_IPAR_ANA_SOL_PRINT_VIOLATED                : i32 = 1``
    Controls whether a list of violated constraints is printed.
``const MSK_IPAR_AUTO_SORT_A_BEFORE_OPT                : i32 = 2``
    Controls whether the elements in each column of A are sorted before an optimization is performed.
``const MSK_IPAR_AUTO_UPDATE_SOL_INFO                  : i32 = 3``
    Controls whether the solution information items are automatically updated after an optimization is performed.
``const MSK_IPAR_BASIS_SOLVE_USE_PLUS_ONE              : i32 = 4``
    Controls the sign of the columns in the basis matrix corresponding to slack variables.
``const MSK_IPAR_BI_CLEAN_OPTIMIZER                    : i32 = 5``
    Controls which simplex optimizer is used in the clean-up phase.
``const MSK_IPAR_BI_IGNORE_MAX_ITER                    : i32 = 6``
    Turns on basis identification in case the interior-point optimizer is terminated due to maximum number of iterations.
``const MSK_IPAR_BI_IGNORE_NUM_ERROR                   : i32 = 7``
    Turns on basis identification in case the interior-point optimizer is terminated due to a numerical problem.
``const MSK_IPAR_BI_MAX_ITERATIONS                     : i32 = 8``
    Maximum number of iterations after basis identification.
``const MSK_IPAR_CACHE_LICENSE                         : i32 = 9``
    Control license caching.
``const MSK_IPAR_CHECK_CONVEXITY                       : i32 = 10``
    Specify the level of convexity check on quadratic problems
``const MSK_IPAR_COMPRESS_STATFILE                     : i32 = 11``
    Control compression of stat files.
``const MSK_IPAR_CONCURRENT_NUM_OPTIMIZERS             : i32 = 12``
    The maximum number of simultaneous optimizations that will be started
                        by the concurrent optimizer.
``const MSK_IPAR_CONCURRENT_PRIORITY_DUAL_SIMPLEX      : i32 = 13``
    Priority of the dual simplex algorithm when selecting solvers for
                        concurrent optimization.
``const MSK_IPAR_CONCURRENT_PRIORITY_FREE_SIMPLEX      : i32 = 14``
    Priority of the free simplex optimizer when selecting solvers for
                        concurrent optimization.
``const MSK_IPAR_CONCURRENT_PRIORITY_INTPNT            : i32 = 15``
    Priority of the interior-point algorithm when selecting solvers for
                        concurrent optimization.
``const MSK_IPAR_CONCURRENT_PRIORITY_PRIMAL_SIMPLEX    : i32 = 16``
    Priority of the primal simplex algorithm when selecting solvers for
                        concurrent optimization.
``const MSK_IPAR_FEASREPAIR_OPTIMIZE                   : i32 = 17``
    Controls which type of feasibility analysis is to be performed.
``const MSK_IPAR_INFEAS_GENERIC_NAMES                  : i32 = 18``
    Controls the contents of the infeasibility report.
``const MSK_IPAR_INFEAS_PREFER_PRIMAL                  : i32 = 19``
    Controls which certificate is used if both primal- and dual- certificate of infeasibility is available.
``const MSK_IPAR_INFEAS_REPORT_AUTO                    : i32 = 20``
    Turns the feasibility report on or off.
``const MSK_IPAR_INFEAS_REPORT_LEVEL                   : i32 = 21``
    Controls the contents of the infeasibility report.
``const MSK_IPAR_INTPNT_BASIS                          : i32 = 22``
    Controls whether basis identification is performed.
``const MSK_IPAR_INTPNT_DIFF_STEP                      : i32 = 23``
    Controls whether different step sizes
                        are allowed in the primal and dual space.
``const MSK_IPAR_INTPNT_HOTSTART                       : i32 = 24``
    Currently not in use.
``const MSK_IPAR_INTPNT_MAX_ITERATIONS                 : i32 = 25``
    Controls the maximum number of iterations
                        allowed in the interior-point optimizer.
``const MSK_IPAR_INTPNT_MAX_NUM_COR                    : i32 = 26``
    Maximum number of correction steps.
``const MSK_IPAR_INTPNT_MAX_NUM_REFINEMENT_STEPS       : i32 = 27``
    Maximum number of steps to be used by the iterative
                      search direction refinement.
``const MSK_IPAR_INTPNT_MULTI_THREAD                   : i32 = 28``
    Controls whether the interior-point optimizers are allowed to employ multiple threads if more threads is available.
``const MSK_IPAR_INTPNT_OFF_COL_TRH                    : i32 = 29``
    Controls the aggressiveness of the offending column detection.
``const MSK_IPAR_INTPNT_ORDER_METHOD                   : i32 = 30``
    Controls the ordering strategy.
``const MSK_IPAR_INTPNT_REGULARIZATION_USE             : i32 = 31``
    Controls whether regularization is allowed.
``const MSK_IPAR_INTPNT_SCALING                        : i32 = 32``
    Controls how the problem is scaled
                        before the interior-point optimizer
                        is used.
``const MSK_IPAR_INTPNT_SOLVE_FORM                     : i32 = 33``
    Controls whether the primal
                        or the dual problem is solved.
``const MSK_IPAR_INTPNT_STARTING_POINT                 : i32 = 34``
    Starting point used by the interior-point optimizer.
``const MSK_IPAR_LICENSE_DEBUG                         : i32 = 35``
    Controls the license manager client debugging behavior.
``const MSK_IPAR_LICENSE_PAUSE_TIME                    : i32 = 36``
    Controls license manager client behavior.
``const MSK_IPAR_LICENSE_SUPPRESS_EXPIRE_WRNS          : i32 = 37``
    Controls license manager client behavior.
``const MSK_IPAR_LICENSE_TRH_EXPIRY_WRN                : i32 = 38``
    Controls when expiry warnings are issued.
``const MSK_IPAR_LICENSE_WAIT                          : i32 = 39``
    Controls if |mosek| should queue for a license if none is available.
``const MSK_IPAR_LOG                                   : i32 = 40``
    Controls the amount of log information.
``const MSK_IPAR_LOG_ANA_PRO                           : i32 = 41``
    Controls amount of outputfrom the problem analyzer.
``const MSK_IPAR_LOG_BI                                : i32 = 42``
    Controls the amount of output printed
                        by the basis identification procedure. A higher level implies that more information is logged.
``const MSK_IPAR_LOG_BI_FREQ                           : i32 = 43``
    Controls the logging frequency.
``const MSK_IPAR_LOG_CHECK_CONVEXITY                   : i32 = 44``
    Controls logging in convexity check on quadratic problems.
                        Set to a positive value to turn logging on.
    
                        If a quadratic coefficient matrix is found to violate the requirement of PSD (NSD)
                        then a list of negative (positive) pivot elements is printed. The absolute value of the pivot elements
                        is also shown.
``const MSK_IPAR_LOG_CONCURRENT                        : i32 = 45``
    Controls amount of output printed
                        by the concurrent optimizer.
``const MSK_IPAR_LOG_CUT_SECOND_OPT                    : i32 = 46``
    Controls the reduction in the log levels for the second and any subsequent optimizations.
``const MSK_IPAR_LOG_EXPAND                            : i32 = 47``
    Controls the amount of logging when a data item such as the maximum number constrains is expanded.
``const MSK_IPAR_LOG_FACTOR                            : i32 = 48``
    If turned on, then the factor log lines are added to the log.
``const MSK_IPAR_LOG_FEAS_REPAIR                       : i32 = 49``
    Controls the amount of output printed when performing feasibility repair. A value higher than one means extensive logging.
``const MSK_IPAR_LOG_FILE                              : i32 = 50``
    If turned on, then some log info is printed when a file is written or read.
``const MSK_IPAR_LOG_HEAD                              : i32 = 51``
    If turned on, then a header line is added to the log.
``const MSK_IPAR_LOG_INFEAS_ANA                        : i32 = 52``
    Controls log level for the infeasibility analyzer.
``const MSK_IPAR_LOG_INTPNT                            : i32 = 53``
    Controls the amount of log information from the interior-point optimizers.
``const MSK_IPAR_LOG_MIO                               : i32 = 54``
    Controls the amount of log information from the mixed-integer optimizers.
``const MSK_IPAR_LOG_MIO_FREQ                          : i32 = 55``
    The mixed-integer optimizer logging frequency.
``const MSK_IPAR_LOG_OPTIMIZER                         : i32 = 56``
    Controls the amount of general optimizer information that is logged.
``const MSK_IPAR_LOG_ORDER                             : i32 = 57``
    If turned on, then factor lines are added to the log.
``const MSK_IPAR_LOG_PRESOLVE                          : i32 = 58``
    Controls amount of output printed by the presolve procedure. 
                        A higher level implies that more information is logged.
``const MSK_IPAR_LOG_RESPONSE                          : i32 = 59``
    Controls amount of output printed when response codes are reported. A higher level implies that more information is logged.
``const MSK_IPAR_LOG_SENSITIVITY                       : i32 = 60``
    Control logging in sensitivity analyzer.
``const MSK_IPAR_LOG_SENSITIVITY_OPT                   : i32 = 61``
    Control logging in sensitivity analyzer.
``const MSK_IPAR_LOG_SIM                               : i32 = 62``
    Controls the amount of log information from the simplex optimizers.
``const MSK_IPAR_LOG_SIM_FREQ                          : i32 = 63``
    Controls simplex logging frequency.
``const MSK_IPAR_LOG_SIM_MINOR                         : i32 = 64``
    Currently not in use.
``const MSK_IPAR_LOG_STORAGE                           : i32 = 65``
    Controls the memory related log information.
``const MSK_IPAR_MAX_NUM_WARNINGS                      : i32 = 66``
    Each warning is shown a limit number times controlled by this parameter.
                    A negative value is identical to infinite number of times.
``const MSK_IPAR_MIO_BRANCH_DIR                        : i32 = 67``
    Controls whether the mixed-integer optimizer is branching up or down by default.
``const MSK_IPAR_MIO_CONSTRUCT_SOL                     : i32 = 68``
    Controls if an initial mixed integer solution should be constructed from the values of the integer variables.
``const MSK_IPAR_MIO_CUT_CLIQUE                        : i32 = 69``
    Controls whether mixed integer rounding cuts should be generated.
``const MSK_IPAR_MIO_CUT_CMIR                          : i32 = 70``
    Controls whether mixed integer rounding cuts should be generated.
``const MSK_IPAR_MIO_CUT_GMI                           : i32 = 71``
    Controls whether GMI cuts should be generated.
``const MSK_IPAR_MIO_CUT_IMPLIED_BOUND                 : i32 = 72``
    Controls whether implied bound cuts should be generated.
``const MSK_IPAR_MIO_CUT_KNAPSACK_COVER                : i32 = 73``
    Controls whether knapsack cover cuts should be generated.
``const MSK_IPAR_MIO_CUT_SELECTION_LEVEL               : i32 = 74``
    Controlls how aggresively generated cuts are selected to be inluded in the relaxation.
``const MSK_IPAR_MIO_HEURISTIC_LEVEL                   : i32 = 75``
    Controls the heuristic employed by the mixed-integer
                      optimizer to locate an initial integer feasible
                      solution.
``const MSK_IPAR_MIO_MAX_NUM_BRANCHES                  : i32 = 76``
    Maximum number of branches allowed during the branch and bound search.
``const MSK_IPAR_MIO_MAX_NUM_RELAXS                    : i32 = 77``
    Maximum number of relaxations in branch and bound search.
``const MSK_IPAR_MIO_MAX_NUM_SOLUTIONS                 : i32 = 78``
    Controls how many feasible solutions the mixed-integer optimizer investigates.
``const MSK_IPAR_MIO_MODE                              : i32 = 79``
    Turns on/off the mixed-integer mode.
``const MSK_IPAR_MIO_MT_USER_CB                        : i32 = 80``
    It true user callbacks are called from each thread used by this optimizer. If false the user callback is only called from a single thread.
``const MSK_IPAR_MIO_NODE_OPTIMIZER                    : i32 = 81``
    Controls which optimizer is employed at the non-root nodes in the mixed-integer optimizer.
``const MSK_IPAR_MIO_NODE_SELECTION                    : i32 = 82``
    Controls the node selection strategy employed by the
                        mixed-integer optimizer.
``const MSK_IPAR_MIO_PERSPECTIVE_REFORMULATE           : i32 = 83``
    Enables or disables perspective reformulation in presolve.
``const MSK_IPAR_MIO_PROBING_LEVEL                     : i32 = 84``
    Controls the amount of probing employed by the mixed-integer
                        optimizer in presolve.
``const MSK_IPAR_MIO_RINS_MAX_NODES                    : i32 = 85``
    Maximum number of nodes in each call to RINS.
``const MSK_IPAR_MIO_ROOT_OPTIMIZER                    : i32 = 86``
    Controls which optimizer is employed at the root node in the mixed-integer optimizer.
``const MSK_IPAR_MIO_ROOT_REPEAT_PRESOLVE_LEVEL        : i32 = 87``
    Controls whether presolve can be repeated at root node.
``const MSK_IPAR_MIO_VB_DETECTION_LEVEL                : i32 = 88``
    Controls how much effort is put into detecting variable bounds.
``const MSK_IPAR_MT_SPINCOUNT                          : i32 = 89``
    Set the number of iterations to spin before sleeping.
``const MSK_IPAR_NUM_THREADS                           : i32 = 90``
    Controls the number of threads employed by the optimizer. If set to 0 the number of threads used will
                        be equal to the number of cores detected on the machine.
``const MSK_IPAR_OPF_MAX_TERMS_PER_LINE                : i32 = 91``
    The maximum number of terms (linear and quadratic) per line when an OPF file is written.
``const MSK_IPAR_OPF_WRITE_HEADER                      : i32 = 92``
    Write a text header with date and |mosek| version in an OPF file.
``const MSK_IPAR_OPF_WRITE_HINTS                       : i32 = 93``
    Write a hint section with problem dimensions in the beginning of an OPF file.
``const MSK_IPAR_OPF_WRITE_PARAMETERS                  : i32 = 94``
    Write a parameter section in an OPF file.
``const MSK_IPAR_OPF_WRITE_PROBLEM                     : i32 = 95``
    Write objective, constraints, bounds etc. to an OPF file.
``const MSK_IPAR_OPF_WRITE_SOL_BAS                     : i32 = 96``
    Controls what is written to the OPF files.
``const MSK_IPAR_OPF_WRITE_SOL_ITG                     : i32 = 97``
    Controls what is written to the OPF files.
``const MSK_IPAR_OPF_WRITE_SOL_ITR                     : i32 = 98``
    Controls what is written to the OPF files.
``const MSK_IPAR_OPF_WRITE_SOLUTIONS                   : i32 = 99``
    Enable inclusion of solutions in the OPF files.
``const MSK_IPAR_OPTIMIZER                             : i32 = 100``
    Controls which optimizer is used to optimize the task.
``const MSK_IPAR_PARAM_READ_CASE_NAME                  : i32 = 101``
    If turned on, then names in the parameter file are case sensitive.
``const MSK_IPAR_PARAM_READ_IGN_ERROR                  : i32 = 102``
    If turned on, then errors in parameter settings is ignored.
``const MSK_IPAR_PRESOLVE_ELIMINATOR_MAX_FILL          : i32 = 103``
    Maximum amount of fill-in created in one pivot during the elimination phase.
``const MSK_IPAR_PRESOLVE_ELIMINATOR_MAX_NUM_TRIES     : i32 = 104``
    Control the maximum number of times the eliminator is tried.
``const MSK_IPAR_PRESOLVE_LEVEL                        : i32 = 105``
    Currently not used.
``const MSK_IPAR_PRESOLVE_LINDEP_ABS_WORK_TRH          : i32 = 106``
    Controls linear dependency check in presolve.
``const MSK_IPAR_PRESOLVE_LINDEP_REL_WORK_TRH          : i32 = 107``
    Controls linear dependency check in presolve.
``const MSK_IPAR_PRESOLVE_LINDEP_USE                   : i32 = 108``
    Controls whether the linear constraints are checked for linear dependencies.
``const MSK_IPAR_PRESOLVE_MAX_NUM_REDUCTIONS           : i32 = 109``
    Controls the maximum number reductions performed by the presolve.
``const MSK_IPAR_PRESOLVE_USE                          : i32 = 110``
    Controls whether the presolve is applied to a problem before it is optimized.
``const MSK_IPAR_PRIMAL_REPAIR_OPTIMIZER               : i32 = 111``
    Controls which optimizer that is used to find the optimal repair.
``const MSK_IPAR_READ_DATA_COMPRESSED                  : i32 = 112``
    Controls the input file decompression.
``const MSK_IPAR_READ_DATA_FORMAT                      : i32 = 113``
    Format of the data file to be read.
``const MSK_IPAR_READ_DEBUG                            : i32 = 114``
    Turns on additional debugging information when reading files.
``const MSK_IPAR_READ_KEEP_FREE_CON                    : i32 = 115``
    Controls whether the free constraints are included in the problem.
``const MSK_IPAR_READ_LP_DROP_NEW_VARS_IN_BOU          : i32 = 116``
    Controls how the LP files are interpreted.
``const MSK_IPAR_READ_LP_QUOTED_NAMES                  : i32 = 117``
    If a name is in quotes when reading an LP file, the quotes will be removed.
``const MSK_IPAR_READ_MPS_FORMAT                       : i32 = 118``
    Controls how strictly the MPS file reader interprets the MPS format.
``const MSK_IPAR_READ_MPS_WIDTH                        : i32 = 119``
    Controls the maximal number of characters allowed in one line of the MPS file.
``const MSK_IPAR_READ_TASK_IGNORE_PARAM                : i32 = 120``
    Controls what information is used from the task files.
``const MSK_IPAR_SENSITIVITY_ALL                       : i32 = 121``
    Controls sensitivity report behavior.
``const MSK_IPAR_SENSITIVITY_OPTIMIZER                 : i32 = 122``
    Controls which optimizer is used for optimal partition sensitivity analysis.
``const MSK_IPAR_SENSITIVITY_TYPE                      : i32 = 123``
    Controls which type of sensitivity analysis is to be performed.
``const MSK_IPAR_SIM_BASIS_FACTOR_USE                  : i32 = 124``
    Controls whether a (LU) factorization of the basis is used in a hot-start.
                        Forcing a refactorization sometimes improves the stability of the simplex optimizers, but in most cases
                        there is a performance penalty.
``const MSK_IPAR_SIM_DEGEN                             : i32 = 125``
    Controls how aggressively degeneration is handled.
``const MSK_IPAR_SIM_DUAL_CRASH                        : i32 = 126``
    Controls whether crashing is performed in the dual simplex optimizer.
``const MSK_IPAR_SIM_DUAL_PHASEONE_METHOD              : i32 = 127``
    An experimental feature.
``const MSK_IPAR_SIM_DUAL_RESTRICT_SELECTION           : i32 = 128``
    Controls how aggressively restricted selection is used.
``const MSK_IPAR_SIM_DUAL_SELECTION                    : i32 = 129``
    Controls the dual simplex strategy.
``const MSK_IPAR_SIM_EXPLOIT_DUPVEC                    : i32 = 130``
    Controls if the simplex optimizers are allowed to exploit duplicated columns.
``const MSK_IPAR_SIM_HOTSTART                          : i32 = 131``
    Controls the type of hot-start that the simplex optimizer perform.
``const MSK_IPAR_SIM_HOTSTART_LU                       : i32 = 132``
    Determines if the simplex optimizer should exploit the initial factorization.
``const MSK_IPAR_SIM_INTEGER                           : i32 = 133``
    An experimental feature.
``const MSK_IPAR_SIM_MAX_ITERATIONS                    : i32 = 134``
    Maximum number of iterations that can be used by a
                        simplex optimizer.
``const MSK_IPAR_SIM_MAX_NUM_SETBACKS                  : i32 = 135``
    Controls how many set-backs that are allowed within a
                      simplex optimizer.
``const MSK_IPAR_SIM_NON_SINGULAR                      : i32 = 136``
    Controls if the simplex optimizer ensures a non-singular basis, if possible.
``const MSK_IPAR_SIM_PRIMAL_CRASH                      : i32 = 137``
    Controls the simplex crash.
``const MSK_IPAR_SIM_PRIMAL_PHASEONE_METHOD            : i32 = 138``
    An experimental feature.
``const MSK_IPAR_SIM_PRIMAL_RESTRICT_SELECTION         : i32 = 139``
    Controls how aggressively restricted selection is used.
``const MSK_IPAR_SIM_PRIMAL_SELECTION                  : i32 = 140``
    Controls the primal simplex strategy.
``const MSK_IPAR_SIM_REFACTOR_FREQ                     : i32 = 141``
    Controls the basis refactoring frequency.
``const MSK_IPAR_SIM_REFORMULATION                     : i32 = 142``
    Controls if the simplex optimizers are allowed to reformulate the problem.
``const MSK_IPAR_SIM_SAVE_LU                           : i32 = 143``
    Controls if the LU factorization stored should be replaced with the LU factorization
                        corresponding to the initial basis.
``const MSK_IPAR_SIM_SCALING                           : i32 = 144``
    Controls how much effort is used in scaling the problem
                        before a simplex optimizer is used.
``const MSK_IPAR_SIM_SCALING_METHOD                    : i32 = 145``
    Controls how the problem is scaled
                        before a simplex optimizer is used.
``const MSK_IPAR_SIM_SOLVE_FORM                        : i32 = 146``
    Controls whether the primal or the dual problem is solved by the primal-/dual-simplex optimizer.
``const MSK_IPAR_SIM_STABILITY_PRIORITY                : i32 = 147``
    Controls how high priority the numerical stability should be given.
``const MSK_IPAR_SIM_SWITCH_OPTIMIZER                  : i32 = 148``
    Controls the simplex behavior.
``const MSK_IPAR_SOL_FILTER_KEEP_BASIC                 : i32 = 149``
    Controls the license manager client behavior.
``const MSK_IPAR_SOL_FILTER_KEEP_RANGED                : i32 = 150``
    Control the contents of the solution files.
``const MSK_IPAR_SOL_READ_NAME_WIDTH                   : i32 = 151``
    Controls the input solution file format.
``const MSK_IPAR_SOL_READ_WIDTH                        : i32 = 152``
    Controls the input solution file format.
``const MSK_IPAR_SOLUTION_CALLBACK                     : i32 = 153``
    Indicates whether solution call-backs will be
                        performed during the optimization.
``const MSK_IPAR_TIMING_LEVEL                          : i32 = 154``
    Controls the a amount of timing performed inside |mosek|.
``const MSK_IPAR_WRITE_BAS_CONSTRAINTS                 : i32 = 155``
    Controls the basic solution file format.
``const MSK_IPAR_WRITE_BAS_HEAD                        : i32 = 156``
    Controls the basic solution file format.
``const MSK_IPAR_WRITE_BAS_VARIABLES                   : i32 = 157``
    Controls the basic solution file format.
``const MSK_IPAR_WRITE_DATA_COMPRESSED                 : i32 = 158``
    Controls output file compression.
``const MSK_IPAR_WRITE_DATA_FORMAT                     : i32 = 159``
    Controls the output file format.
``const MSK_IPAR_WRITE_DATA_PARAM                      : i32 = 160``
    Controls output file data.
``const MSK_IPAR_WRITE_FREE_CON                        : i32 = 161``
    Controls the output file data.
``const MSK_IPAR_WRITE_GENERIC_NAMES                   : i32 = 162``
    Controls the output file data.
``const MSK_IPAR_WRITE_GENERIC_NAMES_IO                : i32 = 163``
    Index origin used in  generic names.
``const MSK_IPAR_WRITE_IGNORE_INCOMPATIBLE_CONIC_ITEMS : i32 = 164``
    If the output format is not compatible with conic quadratic problems this parameter controls if the writer ignores the conic parts or produces an error.
``const MSK_IPAR_WRITE_IGNORE_INCOMPATIBLE_ITEMS       : i32 = 165``
    Controls if the writer ignores incompatible problem items when writing files.
``const MSK_IPAR_WRITE_IGNORE_INCOMPATIBLE_NL_ITEMS    : i32 = 166``
    Controls if the writer ignores general non-linear terms or produces an error.
``const MSK_IPAR_WRITE_IGNORE_INCOMPATIBLE_PSD_ITEMS   : i32 = 167``
    If the output format is not compatible with semidefinite problems this parameter controls if the writer ignores the conic parts or produces an error.
``const MSK_IPAR_WRITE_INT_CONSTRAINTS                 : i32 = 168``
    Controls the integer solution file format.
``const MSK_IPAR_WRITE_INT_HEAD                        : i32 = 169``
    Controls the integer solution file format.
``const MSK_IPAR_WRITE_INT_VARIABLES                   : i32 = 170``
    Controls the integer solution file format.
``const MSK_IPAR_WRITE_LP_FULL_OBJ                     : i32 = 171``
    Write full linear objective
``const MSK_IPAR_WRITE_LP_LINE_WIDTH                   : i32 = 172``
    Controls the LP output file format.
``const MSK_IPAR_WRITE_LP_QUOTED_NAMES                 : i32 = 173``
    Controls LP output file format.
``const MSK_IPAR_WRITE_LP_STRICT_FORMAT                : i32 = 174``
    Controls whether LP  output files satisfy the LP format strictly.
``const MSK_IPAR_WRITE_LP_TERMS_PER_LINE               : i32 = 175``
    Controls the LP output file format.
``const MSK_IPAR_WRITE_MPS_FORMAT                      : i32 = 176``
    Controls in which format the MPS is written.
``const MSK_IPAR_WRITE_MPS_INT                         : i32 = 177``
    Controls the output file data.
``const MSK_IPAR_WRITE_PRECISION                       : i32 = 178``
    Controls data precision employed in when writing an MPS file.
``const MSK_IPAR_WRITE_SOL_BARVARIABLES                : i32 = 179``
    Controls the solution file format.
``const MSK_IPAR_WRITE_SOL_CONSTRAINTS                 : i32 = 180``
    Controls the solution file format.
``const MSK_IPAR_WRITE_SOL_HEAD                        : i32 = 181``
    Controls solution file format.
``const MSK_IPAR_WRITE_SOL_IGNORE_INVALID_NAMES        : i32 = 182``
    Controls whether the user specified names are employed even if they are invalid names.
``const MSK_IPAR_WRITE_SOL_VARIABLES                   : i32 = 183``
    Controls the solution file format.
``const MSK_IPAR_WRITE_TASK_INC_SOL                    : i32 = 184``
    Controls whether the solutions are  stored in the task file too.
``const MSK_IPAR_WRITE_XML_MODE                        : i32 = 185``
    Controls if linear coefficients should be written by row or column when writing in the XML file format.
.. index:: language
.. index:: LANG_...
.. _enum_language:
.. _language_dan:
.. _language_eng:

``language``
------------

Language selection constants

``const MSK_LANG_DAN : i32 = 1``
    Danish language selection
``const MSK_LANG_ENG : i32 = 0``
    English language selection
.. index:: liinfitem
.. index:: LIINF_...
.. _enum_liinfitem:
.. _liinfitem_bi_clean_dual_deg_iter:
.. _liinfitem_bi_clean_dual_iter:
.. _liinfitem_bi_clean_primal_deg_iter:
.. _liinfitem_bi_clean_primal_dual_deg_iter:
.. _liinfitem_bi_clean_primal_dual_iter:
.. _liinfitem_bi_clean_primal_dual_sub_iter:
.. _liinfitem_bi_clean_primal_iter:
.. _liinfitem_bi_dual_iter:
.. _liinfitem_bi_primal_iter:
.. _liinfitem_intpnt_factor_num_nz:
.. _liinfitem_mio_intpnt_iter:
.. _liinfitem_mio_presolved_anz:
.. _liinfitem_mio_sim_maxiter_setbacks:
.. _liinfitem_mio_simplex_iter:
.. _liinfitem_rd_numanz:
.. _liinfitem_rd_numqnz:

``liinfitem``
-------------

Long integer information items.

``const MSK_LIINF_BI_CLEAN_DUAL_DEG_ITER        : i32 = 0``
    Number of dual degenerate clean iterations performed in the basis identification.
``const MSK_LIINF_BI_CLEAN_DUAL_ITER            : i32 = 1``
    Number of dual clean iterations performed in the basis identification.
``const MSK_LIINF_BI_CLEAN_PRIMAL_DEG_ITER      : i32 = 2``
    Number of primal degenerate clean iterations performed in the basis identification.
``const MSK_LIINF_BI_CLEAN_PRIMAL_DUAL_DEG_ITER : i32 = 3``
    Number of primal-dual degenerate clean iterations performed in the basis identification.
``const MSK_LIINF_BI_CLEAN_PRIMAL_DUAL_ITER     : i32 = 4``
    Number of primal-dual clean iterations performed in the basis identification.
``const MSK_LIINF_BI_CLEAN_PRIMAL_DUAL_SUB_ITER : i32 = 5``
    Number of primal-dual subproblem clean iterations performed in the basis identification.
``const MSK_LIINF_BI_CLEAN_PRIMAL_ITER          : i32 = 6``
    Number of primal clean iterations performed in the basis identification.
``const MSK_LIINF_BI_DUAL_ITER                  : i32 = 7``
    Number of dual pivots performed in the basis identification.
``const MSK_LIINF_BI_PRIMAL_ITER                : i32 = 8``
    Number of primal pivots performed in the basis identification.
``const MSK_LIINF_INTPNT_FACTOR_NUM_NZ          : i32 = 9``
    Number of non-zeros in factorization.
``const MSK_LIINF_MIO_INTPNT_ITER               : i32 = 10``
    Number of interior-point iterations performed by the mixed-integer optimizer.
``const MSK_LIINF_MIO_PRESOLVED_ANZ             : i32 = 11``
    Number of  non-zero entries in the constraint matrix of presolved problem.
``const MSK_LIINF_MIO_SIM_MAXITER_SETBACKS      : i32 = 12``
    Number of times the the simplex optimizer has hit the maximum iteration limit when re-optimizing.
``const MSK_LIINF_MIO_SIMPLEX_ITER              : i32 = 13``
    Number of simplex iterations performed by the mixed-integer optimizer.
``const MSK_LIINF_RD_NUMANZ                     : i32 = 14``
    Number of non-zeros in A that is read.
``const MSK_LIINF_RD_NUMQNZ                     : i32 = 15``
    Number of Q non-zeros.
.. index:: mark
.. index:: MARK_...
.. _enum_mark:
.. _mark_lo:
.. _mark_up:

``mark``
--------

Mark

``const MSK_MARK_LO : i32 = 0``
    The lower bound is selected for sensitivity analysis.
``const MSK_MARK_UP : i32 = 1``
    The upper bound is selected for sensitivity analysis.
.. index:: miocontsoltype
.. index:: MIO_CONT_SOL_...
.. _enum_miocontsoltype:
.. _miocontsoltype_itg:
.. _miocontsoltype_itg_rel:
.. _miocontsoltype_none:
.. _miocontsoltype_root:

``miocontsoltype``
------------------

Continuous mixed-integer solution type

``const MSK_MIO_CONT_SOL_ITG     : i32 = 2``
    The reported interior-point and basic solutions are
                        a solution to the problem with all integer variables
                        fixed at the value they have in the integer solution.
                        A solution is only reported in case the
                        problem has a primal feasible solution.
``const MSK_MIO_CONT_SOL_ITG_REL : i32 = 3``
    In case the problem is primal feasible
                        then the reported interior-point and basic solutions
                        are a solution to the problem with all integer variables
                        fixed at the value they have in the integer solution.
                        If the problem is primal infeasible, then the solution to the root node problem is reported.
``const MSK_MIO_CONT_SOL_NONE    : i32 = 0``
    No interior-point or basic solution are reported when the mixed-integer optimizer is used.
``const MSK_MIO_CONT_SOL_ROOT    : i32 = 1``
    The reported interior-point and basic solutions are a solution to the root node problem
                        when mixed-integer optimizer is used.
.. index:: miomode
.. index:: MIO_MODE_...
.. _enum_miomode:
.. _miomode_ignored:
.. _miomode_satisfied:

``miomode``
-----------

Integer restrictions

``const MSK_MIO_MODE_IGNORED   : i32 = 0``
    The integer constraints are ignored and the problem is solved as a continuous problem.
``const MSK_MIO_MODE_SATISFIED : i32 = 1``
    Integer restrictions should be satisfied.
.. index:: mionodeseltype
.. index:: MIO_NODE_SELECTION_...
.. _enum_mionodeseltype:
.. _mionodeseltype_best:
.. _mionodeseltype_first:
.. _mionodeseltype_free:
.. _mionodeseltype_hybrid:
.. _mionodeseltype_pseudo:
.. _mionodeseltype_worst:

``mionodeseltype``
------------------

Mixed-integer node selection types

``const MSK_MIO_NODE_SELECTION_BEST   : i32 = 2``
    The optimizer employs a best bound node selection strategy.
``const MSK_MIO_NODE_SELECTION_FIRST  : i32 = 1``
    The optimizer employs a depth first node selection strategy.
``const MSK_MIO_NODE_SELECTION_FREE   : i32 = 0``
    The optimizer decides the node selection strategy.
``const MSK_MIO_NODE_SELECTION_HYBRID : i32 = 4``
    The optimizer employs a hybrid strategy.
``const MSK_MIO_NODE_SELECTION_PSEUDO : i32 = 5``
    The optimizer employs selects the node based on a pseudo cost estimate.
``const MSK_MIO_NODE_SELECTION_WORST  : i32 = 3``
    The optimizer employs a worst bound node selection strategy.
.. index:: mpsformat
.. index:: MPS_FORMAT_...
.. _enum_mpsformat:
.. _mpsformat_cplex:
.. _mpsformat_free:
.. _mpsformat_relaxed:
.. _mpsformat_strict:

``mpsformat``
-------------

MPS file format type

``const MSK_MPS_FORMAT_CPLEX   : i32 = 3``
    The CPLEX compatible version of the MPS format is employed.
``const MSK_MPS_FORMAT_FREE    : i32 = 2``
    It is assumed that the input file satisfies the free
                        MPS format. This implies that spaces
                        are not allowed in names. Otherwise
                        the format is free.
``const MSK_MPS_FORMAT_RELAXED : i32 = 1``
    It is assumed that the input file satisfies
                        a slightly relaxed version of the MPS format.
``const MSK_MPS_FORMAT_STRICT  : i32 = 0``
    It is assumed that the input file satisfies
                        the MPS format strictly.
.. index:: msgkey
.. index:: MSG_...
.. _enum_msgkey:
.. _msgkey_mps_selected:
.. _msgkey_reading_file:
.. _msgkey_writing_file:

``msgkey``
----------

Message keys

``const MSK_MSG_MPS_SELECTED : i32 = 1100``
    
``const MSK_MSG_READING_FILE : i32 = 1000``
    
``const MSK_MSG_WRITING_FILE : i32 = 1001``
    
.. index:: nametype
.. index:: NAME_TYPE_...
.. _enum_nametype:
.. _nametype_gen:
.. _nametype_lp:
.. _nametype_mps:

``nametype``
------------

Name types

``const MSK_NAME_TYPE_GEN : i32 = 0``
    General names. However, no duplicate and blank names are allowed.
``const MSK_NAME_TYPE_LP  : i32 = 2``
    LP type names.
``const MSK_NAME_TYPE_MPS : i32 = 1``
    MPS type names.
.. index:: objsense
.. index:: OBJECTIVE_SENSE_...
.. _enum_objsense:
.. _objsense_maximize:
.. _objsense_minimize:

``objsense``
------------

Objective sense types

``const MSK_OBJECTIVE_SENSE_MAXIMIZE : i32 = 1``
    The problem should be maximized.
``const MSK_OBJECTIVE_SENSE_MINIMIZE : i32 = 0``
    The problem should be minimized.
.. index:: onoffkey
.. index:: ...
.. _enum_onoffkey:
.. _onoffkey_off:
.. _onoffkey_on:

``onoffkey``
------------

On/off

``const MSK_OFF : i32 = 0``
    Switch the option off.
``const MSK_ON  : i32 = 1``
    Switch the option on.
.. index:: optimizertype
.. index:: OPTIMIZER_...
.. _enum_optimizertype:
.. _optimizertype_conic:
.. _optimizertype_dual_simplex:
.. _optimizertype_free:
.. _optimizertype_free_simplex:
.. _optimizertype_intpnt:
.. _optimizertype_mixed_int:
.. _optimizertype_primal_simplex:

``optimizertype``
-----------------

Optimizer types

``const MSK_OPTIMIZER_CONIC          : i32 = 0``
    The optimizer for problems having conic constraints.
``const MSK_OPTIMIZER_DUAL_SIMPLEX   : i32 = 1``
    The dual simplex optimizer is used.
``const MSK_OPTIMIZER_FREE           : i32 = 2``
    The optimizer is chosen automatically.
``const MSK_OPTIMIZER_FREE_SIMPLEX   : i32 = 3``
    One of the simplex optimizers is used.
``const MSK_OPTIMIZER_INTPNT         : i32 = 4``
    The interior-point optimizer is used.
``const MSK_OPTIMIZER_MIXED_INT      : i32 = 5``
    The mixed-integer optimizer.
``const MSK_OPTIMIZER_PRIMAL_SIMPLEX : i32 = 6``
    The primal simplex optimizer is used.
.. index:: orderingtype
.. index:: ORDER_METHOD_...
.. _enum_orderingtype:
.. _orderingtype_appminloc:
.. _orderingtype_experimental:
.. _orderingtype_force_graphpar:
.. _orderingtype_free:
.. _orderingtype_none:
.. _orderingtype_try_graphpar:

``orderingtype``
----------------

Ordering strategies

``const MSK_ORDER_METHOD_APPMINLOC      : i32 = 1``
    Approximate minimum local fill-in ordering is employed.
``const MSK_ORDER_METHOD_EXPERIMENTAL   : i32 = 2``
    This option should not be used.
``const MSK_ORDER_METHOD_FORCE_GRAPHPAR : i32 = 4``
    Always use the graph partitioning based ordering even if it is worse that the approximate minimum local fill ordering.
``const MSK_ORDER_METHOD_FREE           : i32 = 0``
    The ordering method is chosen automatically.
``const MSK_ORDER_METHOD_NONE           : i32 = 5``
    No ordering is used.
``const MSK_ORDER_METHOD_TRY_GRAPHPAR   : i32 = 3``
    Always try the graph partitioning based ordering.
.. index:: parametertype
.. index:: PAR_...
.. _enum_parametertype:
.. _parametertype_dou_type:
.. _parametertype_int_type:
.. _parametertype_invalid_type:
.. _parametertype_str_type:

``parametertype``
-----------------

Parameter type

``const MSK_PAR_DOU_TYPE     : i32 = 1``
    Is a double parameter.
``const MSK_PAR_INT_TYPE     : i32 = 2``
    Is an integer parameter.
``const MSK_PAR_INVALID_TYPE : i32 = 0``
    Not a valid parameter.
``const MSK_PAR_STR_TYPE     : i32 = 3``
    Is a string parameter.
.. index:: presolvemode
.. index:: PRESOLVE_MODE_...
.. _enum_presolvemode:
.. _presolvemode_free:
.. _presolvemode_off:
.. _presolvemode_on:

``presolvemode``
----------------

Presolve method.

``const MSK_PRESOLVE_MODE_FREE : i32 = 2``
    It is decided automatically whether to presolve before the problem is optimized.
``const MSK_PRESOLVE_MODE_OFF  : i32 = 0``
    The problem is not presolved before it is optimized.
``const MSK_PRESOLVE_MODE_ON   : i32 = 1``
    The problem is presolved before it is optimized.
.. index:: problemitem
.. index:: PI_...
.. _enum_problemitem:
.. _problemitem_con:
.. _problemitem_cone:
.. _problemitem_var:

``problemitem``
---------------

Problem data items

``const MSK_PI_CON  : i32 = 1``
    Item is a constraint.
``const MSK_PI_CONE : i32 = 2``
    Item is a cone.
``const MSK_PI_VAR  : i32 = 0``
    Item is a variable.
.. index:: problemtype
.. index:: PROBTYPE_...
.. _enum_problemtype:
.. _problemtype_conic:
.. _problemtype_geco:
.. _problemtype_lo:
.. _problemtype_mixed:
.. _problemtype_qcqo:
.. _problemtype_qo:

``problemtype``
---------------

Problem types

``const MSK_PROBTYPE_CONIC : i32 = 4``
    A conic optimization.
``const MSK_PROBTYPE_GECO  : i32 = 3``
    General convex optimization.
``const MSK_PROBTYPE_LO    : i32 = 0``
    The problem is a linear optimization problem.
``const MSK_PROBTYPE_MIXED : i32 = 5``
    General nonlinear constraints and conic constraints. This combination can not be solved by |mosek|.
``const MSK_PROBTYPE_QCQO  : i32 = 2``
    The problem is a quadratically constrained optimization problem.
``const MSK_PROBTYPE_QO    : i32 = 1``
    The problem is a quadratic optimization problem.
.. index:: prosta
.. index:: PRO_STA_...
.. _enum_prosta:
.. _prosta_dual_feas:
.. _prosta_dual_infeas:
.. _prosta_ill_posed:
.. _prosta_near_dual_feas:
.. _prosta_near_prim_and_dual_feas:
.. _prosta_near_prim_feas:
.. _prosta_prim_and_dual_feas:
.. _prosta_prim_and_dual_infeas:
.. _prosta_prim_feas:
.. _prosta_prim_infeas:
.. _prosta_prim_infeas_or_unbounded:
.. _prosta_unknown:

``prosta``
----------

Problem status keys

``const MSK_PRO_STA_DUAL_FEAS                : i32 = 3``
    The problem is dual feasible.
``const MSK_PRO_STA_DUAL_INFEAS              : i32 = 5``
    The problem is dual infeasible.
``const MSK_PRO_STA_ILL_POSED                : i32 = 7``
    The problem is ill-posed. For example,
                        it may be primal and dual feasible but
                        have a positive duality gap.
``const MSK_PRO_STA_NEAR_DUAL_FEAS           : i32 = 10``
    The problem is at least nearly dual feasible.
``const MSK_PRO_STA_NEAR_PRIM_AND_DUAL_FEAS  : i32 = 8``
    The problem is at least nearly primal and dual feasible.
``const MSK_PRO_STA_NEAR_PRIM_FEAS           : i32 = 9``
    The problem is at least nearly primal feasible.
``const MSK_PRO_STA_PRIM_AND_DUAL_FEAS       : i32 = 1``
    The problem is primal and dual feasible.
``const MSK_PRO_STA_PRIM_AND_DUAL_INFEAS     : i32 = 6``
    The problem is primal and dual infeasible.
``const MSK_PRO_STA_PRIM_FEAS                : i32 = 2``
    The problem is primal feasible.
``const MSK_PRO_STA_PRIM_INFEAS              : i32 = 4``
    The problem is primal infeasible.
``const MSK_PRO_STA_PRIM_INFEAS_OR_UNBOUNDED : i32 = 11``
    The problem is either primal infeasible or unbounded. This may occur for
                        mixed-integer problems.
``const MSK_PRO_STA_UNKNOWN                  : i32 = 0``
    Unknown problem status.
.. index:: rescode
.. index:: RES_...
.. _enum_rescode:
.. _rescode_err_ad_invalid_codelist:
.. _rescode_err_api_array_too_small:
.. _rescode_err_api_cb_connect:
.. _rescode_err_api_fatal_error:
.. _rescode_err_api_internal:
.. _rescode_err_arg_is_too_large:
.. _rescode_err_arg_is_too_small:
.. _rescode_err_argument_dimension:
.. _rescode_err_argument_is_too_large:
.. _rescode_err_argument_lenneq:
.. _rescode_err_argument_perm_array:
.. _rescode_err_argument_type:
.. _rescode_err_bar_var_dim:
.. _rescode_err_basis:
.. _rescode_err_basis_factor:
.. _rescode_err_basis_singular:
.. _rescode_err_blank_name:
.. _rescode_err_cannot_clone_nl:
.. _rescode_err_cannot_handle_nl:
.. _rescode_err_cbf_duplicate_acoord:
.. _rescode_err_cbf_duplicate_bcoord:
.. _rescode_err_cbf_duplicate_con:
.. _rescode_err_cbf_duplicate_int:
.. _rescode_err_cbf_duplicate_obj:
.. _rescode_err_cbf_duplicate_objacoord:
.. _rescode_err_cbf_duplicate_var:
.. _rescode_err_cbf_invalid_con_type:
.. _rescode_err_cbf_invalid_domain_dimension:
.. _rescode_err_cbf_invalid_int_index:
.. _rescode_err_cbf_invalid_var_type:
.. _rescode_err_cbf_no_variables:
.. _rescode_err_cbf_no_version_specified:
.. _rescode_err_cbf_obj_sense:
.. _rescode_err_cbf_parse:
.. _rescode_err_cbf_syntax:
.. _rescode_err_cbf_too_few_constraints:
.. _rescode_err_cbf_too_few_ints:
.. _rescode_err_cbf_too_few_variables:
.. _rescode_err_cbf_too_many_constraints:
.. _rescode_err_cbf_too_many_ints:
.. _rescode_err_cbf_too_many_variables:
.. _rescode_err_cbf_unsupported:
.. _rescode_err_con_q_not_nsd:
.. _rescode_err_con_q_not_psd:
.. _rescode_err_concurrent_optimizer:
.. _rescode_err_cone_index:
.. _rescode_err_cone_overlap:
.. _rescode_err_cone_overlap_append:
.. _rescode_err_cone_rep_var:
.. _rescode_err_cone_size:
.. _rescode_err_cone_type:
.. _rescode_err_cone_type_str:
.. _rescode_err_data_file_ext:
.. _rescode_err_dup_name:
.. _rescode_err_duplicate_aij:
.. _rescode_err_duplicate_barvariable_names:
.. _rescode_err_duplicate_cone_names:
.. _rescode_err_duplicate_constraint_names:
.. _rescode_err_duplicate_variable_names:
.. _rescode_err_end_of_file:
.. _rescode_err_factor:
.. _rescode_err_feasrepair_cannot_relax:
.. _rescode_err_feasrepair_inconsistent_bound:
.. _rescode_err_feasrepair_solving_relaxed:
.. _rescode_err_file_license:
.. _rescode_err_file_open:
.. _rescode_err_file_read:
.. _rescode_err_file_write:
.. _rescode_err_first:
.. _rescode_err_firsti:
.. _rescode_err_firstj:
.. _rescode_err_fixed_bound_values:
.. _rescode_err_flexlm:
.. _rescode_err_global_inv_conic_problem:
.. _rescode_err_huge_aij:
.. _rescode_err_huge_c:
.. _rescode_err_identical_tasks:
.. _rescode_err_in_argument:
.. _rescode_err_index:
.. _rescode_err_index_arr_is_too_large:
.. _rescode_err_index_arr_is_too_small:
.. _rescode_err_index_is_too_large:
.. _rescode_err_index_is_too_small:
.. _rescode_err_inf_dou_index:
.. _rescode_err_inf_dou_name:
.. _rescode_err_inf_int_index:
.. _rescode_err_inf_int_name:
.. _rescode_err_inf_lint_index:
.. _rescode_err_inf_lint_name:
.. _rescode_err_inf_type:
.. _rescode_err_infeas_undefined:
.. _rescode_err_infinite_bound:
.. _rescode_err_int64_to_int32_cast:
.. _rescode_err_internal:
.. _rescode_err_internal_test_failed:
.. _rescode_err_inv_aptre:
.. _rescode_err_inv_bk:
.. _rescode_err_inv_bkc:
.. _rescode_err_inv_bkx:
.. _rescode_err_inv_cone_type:
.. _rescode_err_inv_cone_type_str:
.. _rescode_err_inv_marki:
.. _rescode_err_inv_markj:
.. _rescode_err_inv_name_item:
.. _rescode_err_inv_numi:
.. _rescode_err_inv_numj:
.. _rescode_err_inv_optimizer:
.. _rescode_err_inv_problem:
.. _rescode_err_inv_qcon_subi:
.. _rescode_err_inv_qcon_subj:
.. _rescode_err_inv_qcon_subk:
.. _rescode_err_inv_qcon_val:
.. _rescode_err_inv_qobj_subi:
.. _rescode_err_inv_qobj_subj:
.. _rescode_err_inv_qobj_val:
.. _rescode_err_inv_sk:
.. _rescode_err_inv_sk_str:
.. _rescode_err_inv_skc:
.. _rescode_err_inv_skn:
.. _rescode_err_inv_skx:
.. _rescode_err_inv_var_type:
.. _rescode_err_invalid_accmode:
.. _rescode_err_invalid_aij:
.. _rescode_err_invalid_ampl_stub:
.. _rescode_err_invalid_barvar_name:
.. _rescode_err_invalid_compression:
.. _rescode_err_invalid_con_name:
.. _rescode_err_invalid_cone_name:
.. _rescode_err_invalid_file_format_for_cones:
.. _rescode_err_invalid_file_format_for_general_nl:
.. _rescode_err_invalid_file_format_for_sym_mat:
.. _rescode_err_invalid_file_name:
.. _rescode_err_invalid_format_type:
.. _rescode_err_invalid_idx:
.. _rescode_err_invalid_iomode:
.. _rescode_err_invalid_max_num:
.. _rescode_err_invalid_name_in_sol_file:
.. _rescode_err_invalid_network_problem:
.. _rescode_err_invalid_obj_name:
.. _rescode_err_invalid_objective_sense:
.. _rescode_err_invalid_problem_type:
.. _rescode_err_invalid_sol_file_name:
.. _rescode_err_invalid_stream:
.. _rescode_err_invalid_surplus:
.. _rescode_err_invalid_sym_mat_dim:
.. _rescode_err_invalid_task:
.. _rescode_err_invalid_utf8:
.. _rescode_err_invalid_var_name:
.. _rescode_err_invalid_wchar:
.. _rescode_err_invalid_whichsol:
.. _rescode_err_json_data:
.. _rescode_err_json_format:
.. _rescode_err_json_missing_data:
.. _rescode_err_json_number_overflow:
.. _rescode_err_json_string:
.. _rescode_err_json_syntax:
.. _rescode_err_last:
.. _rescode_err_lasti:
.. _rescode_err_lastj:
.. _rescode_err_lau_arg_k:
.. _rescode_err_lau_arg_m:
.. _rescode_err_lau_arg_n:
.. _rescode_err_lau_arg_trans:
.. _rescode_err_lau_arg_transa:
.. _rescode_err_lau_arg_transb:
.. _rescode_err_lau_arg_uplo:
.. _rescode_err_lau_invalid_lower_triangular_matrix:
.. _rescode_err_lau_invalid_sparse_symmetric_matrix:
.. _rescode_err_lau_not_positive_definite:
.. _rescode_err_lau_singular_matrix:
.. _rescode_err_lau_unknown:
.. _rescode_err_license:
.. _rescode_err_license_cannot_allocate:
.. _rescode_err_license_cannot_connect:
.. _rescode_err_license_expired:
.. _rescode_err_license_feature:
.. _rescode_err_license_invalid_hostid:
.. _rescode_err_license_max:
.. _rescode_err_license_moseklm_daemon:
.. _rescode_err_license_no_server_line:
.. _rescode_err_license_no_server_support:
.. _rescode_err_license_server:
.. _rescode_err_license_server_version:
.. _rescode_err_license_version:
.. _rescode_err_link_file_dll:
.. _rescode_err_living_tasks:
.. _rescode_err_lower_bound_is_a_nan:
.. _rescode_err_lp_dup_slack_name:
.. _rescode_err_lp_empty:
.. _rescode_err_lp_file_format:
.. _rescode_err_lp_format:
.. _rescode_err_lp_free_constraint:
.. _rescode_err_lp_incompatible:
.. _rescode_err_lp_invalid_con_name:
.. _rescode_err_lp_invalid_var_name:
.. _rescode_err_lp_write_conic_problem:
.. _rescode_err_lp_write_geco_problem:
.. _rescode_err_lu_max_num_tries:
.. _rescode_err_max_len_is_too_small:
.. _rescode_err_maxnumbarvar:
.. _rescode_err_maxnumcon:
.. _rescode_err_maxnumcone:
.. _rescode_err_maxnumqnz:
.. _rescode_err_maxnumvar:
.. _rescode_err_mio_internal:
.. _rescode_err_mio_invalid_node_optimizer:
.. _rescode_err_mio_invalid_root_optimizer:
.. _rescode_err_mio_no_optimizer:
.. _rescode_err_mio_not_loaded:
.. _rescode_err_missing_license_file:
.. _rescode_err_mixed_conic_and_nl:
.. _rescode_err_mps_cone_overlap:
.. _rescode_err_mps_cone_repeat:
.. _rescode_err_mps_cone_type:
.. _rescode_err_mps_duplicate_q_element:
.. _rescode_err_mps_file:
.. _rescode_err_mps_inv_bound_key:
.. _rescode_err_mps_inv_con_key:
.. _rescode_err_mps_inv_field:
.. _rescode_err_mps_inv_marker:
.. _rescode_err_mps_inv_sec_name:
.. _rescode_err_mps_inv_sec_order:
.. _rescode_err_mps_invalid_obj_name:
.. _rescode_err_mps_invalid_objsense:
.. _rescode_err_mps_mul_con_name:
.. _rescode_err_mps_mul_csec:
.. _rescode_err_mps_mul_qobj:
.. _rescode_err_mps_mul_qsec:
.. _rescode_err_mps_no_objective:
.. _rescode_err_mps_non_symmetric_q:
.. _rescode_err_mps_null_con_name:
.. _rescode_err_mps_null_var_name:
.. _rescode_err_mps_splitted_var:
.. _rescode_err_mps_tab_in_field2:
.. _rescode_err_mps_tab_in_field3:
.. _rescode_err_mps_tab_in_field5:
.. _rescode_err_mps_undef_con_name:
.. _rescode_err_mps_undef_var_name:
.. _rescode_err_mul_a_element:
.. _rescode_err_name_is_null:
.. _rescode_err_name_max_len:
.. _rescode_err_nan_in_blc:
.. _rescode_err_nan_in_blx:
.. _rescode_err_nan_in_buc:
.. _rescode_err_nan_in_bux:
.. _rescode_err_nan_in_c:
.. _rescode_err_nan_in_double_data:
.. _rescode_err_negative_append:
.. _rescode_err_negative_surplus:
.. _rescode_err_newer_dll:
.. _rescode_err_no_bars_for_solution:
.. _rescode_err_no_barx_for_solution:
.. _rescode_err_no_basis_sol:
.. _rescode_err_no_dual_for_itg_sol:
.. _rescode_err_no_dual_infeas_cer:
.. _rescode_err_no_init_env:
.. _rescode_err_no_optimizer_var_type:
.. _rescode_err_no_primal_infeas_cer:
.. _rescode_err_no_snx_for_bas_sol:
.. _rescode_err_no_solution_in_callback:
.. _rescode_err_non_unique_array:
.. _rescode_err_nonconvex:
.. _rescode_err_nonlinear_equality:
.. _rescode_err_nonlinear_functions_not_allowed:
.. _rescode_err_nonlinear_ranged:
.. _rescode_err_nr_arguments:
.. _rescode_err_null_env:
.. _rescode_err_null_pointer:
.. _rescode_err_null_task:
.. _rescode_err_numconlim:
.. _rescode_err_numvarlim:
.. _rescode_err_obj_q_not_nsd:
.. _rescode_err_obj_q_not_psd:
.. _rescode_err_objective_range:
.. _rescode_err_older_dll:
.. _rescode_err_open_dl:
.. _rescode_err_opf_format:
.. _rescode_err_opf_new_variable:
.. _rescode_err_opf_premature_eof:
.. _rescode_err_optimizer_license:
.. _rescode_err_overflow:
.. _rescode_err_param_index:
.. _rescode_err_param_is_too_large:
.. _rescode_err_param_is_too_small:
.. _rescode_err_param_name:
.. _rescode_err_param_name_dou:
.. _rescode_err_param_name_int:
.. _rescode_err_param_name_str:
.. _rescode_err_param_type:
.. _rescode_err_param_value_str:
.. _rescode_err_platform_not_licensed:
.. _rescode_err_postsolve:
.. _rescode_err_pro_item:
.. _rescode_err_prob_license:
.. _rescode_err_qcon_subi_too_large:
.. _rescode_err_qcon_subi_too_small:
.. _rescode_err_qcon_upper_triangle:
.. _rescode_err_qobj_upper_triangle:
.. _rescode_err_read_format:
.. _rescode_err_read_lp_missing_end_tag:
.. _rescode_err_read_lp_nonexisting_name:
.. _rescode_err_remove_cone_variable:
.. _rescode_err_repair_invalid_problem:
.. _rescode_err_repair_optimization_failed:
.. _rescode_err_sen_bound_invalid_lo:
.. _rescode_err_sen_bound_invalid_up:
.. _rescode_err_sen_format:
.. _rescode_err_sen_index_invalid:
.. _rescode_err_sen_index_range:
.. _rescode_err_sen_invalid_regexp:
.. _rescode_err_sen_numerical:
.. _rescode_err_sen_solution_status:
.. _rescode_err_sen_undef_name:
.. _rescode_err_sen_unhandled_problem_type:
.. _rescode_err_server_connect:
.. _rescode_err_server_protocol:
.. _rescode_err_server_status:
.. _rescode_err_server_token:
.. _rescode_err_size_license:
.. _rescode_err_size_license_con:
.. _rescode_err_size_license_intvar:
.. _rescode_err_size_license_numcores:
.. _rescode_err_size_license_var:
.. _rescode_err_sol_file_invalid_number:
.. _rescode_err_solitem:
.. _rescode_err_solver_probtype:
.. _rescode_err_space:
.. _rescode_err_space_leaking:
.. _rescode_err_space_no_info:
.. _rescode_err_sym_mat_duplicate:
.. _rescode_err_sym_mat_invalid_col_index:
.. _rescode_err_sym_mat_invalid_row_index:
.. _rescode_err_sym_mat_invalid_value:
.. _rescode_err_sym_mat_not_lower_tringular:
.. _rescode_err_task_incompatible:
.. _rescode_err_task_invalid:
.. _rescode_err_thread_cond_init:
.. _rescode_err_thread_create:
.. _rescode_err_thread_mutex_init:
.. _rescode_err_thread_mutex_lock:
.. _rescode_err_thread_mutex_unlock:
.. _rescode_err_toconic_constr_not_conic:
.. _rescode_err_toconic_constr_q_not_psd:
.. _rescode_err_toconic_constraint_fx:
.. _rescode_err_toconic_constraint_ra:
.. _rescode_err_toconic_objective_not_psd:
.. _rescode_err_too_many_concurrent_tasks:
.. _rescode_err_too_small_max_num_nz:
.. _rescode_err_too_small_maxnumanz:
.. _rescode_err_unb_step_size:
.. _rescode_err_undef_solution:
.. _rescode_err_undefined_objective_sense:
.. _rescode_err_unhandled_solution_status:
.. _rescode_err_unknown:
.. _rescode_err_upper_bound_is_a_nan:
.. _rescode_err_upper_triangle:
.. _rescode_err_user_func_ret:
.. _rescode_err_user_func_ret_data:
.. _rescode_err_user_nlo_eval:
.. _rescode_err_user_nlo_eval_hessubi:
.. _rescode_err_user_nlo_eval_hessubj:
.. _rescode_err_user_nlo_func:
.. _rescode_err_whichitem_not_allowed:
.. _rescode_err_whichsol:
.. _rescode_err_write_lp_format:
.. _rescode_err_write_lp_non_unique_name:
.. _rescode_err_write_mps_invalid_name:
.. _rescode_err_write_opf_invalid_var_name:
.. _rescode_err_writing_file:
.. _rescode_err_xml_invalid_problem_type:
.. _rescode_err_y_is_undefined:
.. _rescode_ok:
.. _rescode_trm_internal:
.. _rescode_trm_internal_stop:
.. _rescode_trm_max_iterations:
.. _rescode_trm_max_num_setbacks:
.. _rescode_trm_max_time:
.. _rescode_trm_mio_near_abs_gap:
.. _rescode_trm_mio_near_rel_gap:
.. _rescode_trm_mio_num_branches:
.. _rescode_trm_mio_num_relaxs:
.. _rescode_trm_num_max_num_int_solutions:
.. _rescode_trm_numerical_problem:
.. _rescode_trm_objective_range:
.. _rescode_trm_stall:
.. _rescode_trm_user_callback:
.. _rescode_wrn_ana_almost_int_bounds:
.. _rescode_wrn_ana_c_zero:
.. _rescode_wrn_ana_close_bounds:
.. _rescode_wrn_ana_empty_cols:
.. _rescode_wrn_ana_large_bounds:
.. _rescode_wrn_construct_invalid_sol_itg:
.. _rescode_wrn_construct_no_sol_itg:
.. _rescode_wrn_construct_solution_infeas:
.. _rescode_wrn_dropped_nz_qobj:
.. _rescode_wrn_duplicate_barvariable_names:
.. _rescode_wrn_duplicate_cone_names:
.. _rescode_wrn_duplicate_constraint_names:
.. _rescode_wrn_duplicate_variable_names:
.. _rescode_wrn_eliminator_space:
.. _rescode_wrn_empty_name:
.. _rescode_wrn_ignore_integer:
.. _rescode_wrn_incomplete_linear_dependency_check:
.. _rescode_wrn_large_aij:
.. _rescode_wrn_large_bound:
.. _rescode_wrn_large_cj:
.. _rescode_wrn_large_con_fx:
.. _rescode_wrn_large_lo_bound:
.. _rescode_wrn_large_up_bound:
.. _rescode_wrn_license_expire:
.. _rescode_wrn_license_feature_expire:
.. _rescode_wrn_license_server:
.. _rescode_wrn_lp_drop_variable:
.. _rescode_wrn_lp_old_quad_format:
.. _rescode_wrn_mio_infeasible_final:
.. _rescode_wrn_mps_split_bou_vector:
.. _rescode_wrn_mps_split_ran_vector:
.. _rescode_wrn_mps_split_rhs_vector:
.. _rescode_wrn_name_max_len:
.. _rescode_wrn_no_dualizer:
.. _rescode_wrn_no_global_optimizer:
.. _rescode_wrn_no_nonlinear_function_write:
.. _rescode_wrn_nz_in_upr_tri:
.. _rescode_wrn_open_param_file:
.. _rescode_wrn_param_ignored_cmio:
.. _rescode_wrn_param_name_dou:
.. _rescode_wrn_param_name_int:
.. _rescode_wrn_param_name_str:
.. _rescode_wrn_param_str_value:
.. _rescode_wrn_presolve_outofspace:
.. _rescode_wrn_quad_cones_with_root_fixed_at_zero:
.. _rescode_wrn_rquad_cones_with_root_fixed_at_zero:
.. _rescode_wrn_sol_file_ignored_con:
.. _rescode_wrn_sol_file_ignored_var:
.. _rescode_wrn_sol_filter:
.. _rescode_wrn_spar_max_len:
.. _rescode_wrn_too_few_basis_vars:
.. _rescode_wrn_too_many_basis_vars:
.. _rescode_wrn_too_many_threads_concurrent:
.. _rescode_wrn_undef_sol_file_name:
.. _rescode_wrn_using_generic_names:
.. _rescode_wrn_write_changed_names:
.. _rescode_wrn_write_discarded_cfix:
.. _rescode_wrn_zero_aij:
.. _rescode_wrn_zeros_in_sparse_col:
.. _rescode_wrn_zeros_in_sparse_row:

``rescode``
-----------

Response codes

``const MSK_RES_ERR_AD_INVALID_CODELIST                 : i32 = 3102``
    The code list data was invalid.
``const MSK_RES_ERR_API_ARRAY_TOO_SMALL                 : i32 = 3001``
    An input array was too short.
``const MSK_RES_ERR_API_CB_CONNECT                      : i32 = 3002``
    Failed to connect a callback object.
``const MSK_RES_ERR_API_FATAL_ERROR                     : i32 = 3005``
    An internal error occurred in the API. Please report this problem.
``const MSK_RES_ERR_API_INTERNAL                        : i32 = 3999``
    An internal fatal error occurred in an interface function.:w
``const MSK_RES_ERR_ARG_IS_TOO_LARGE                    : i32 = 1227``
    The value of a argument is too small.
``const MSK_RES_ERR_ARG_IS_TOO_SMALL                    : i32 = 1226``
    The value of a argument is too small.
``const MSK_RES_ERR_ARGUMENT_DIMENSION                  : i32 = 1201``
    A function argument is of incorrect dimension.
``const MSK_RES_ERR_ARGUMENT_IS_TOO_LARGE               : i32 = 5005``
    The value of a function argument is too large.
``const MSK_RES_ERR_ARGUMENT_LENNEQ                     : i32 = 1197``
    Incorrect length of arguments.
``const MSK_RES_ERR_ARGUMENT_PERM_ARRAY                 : i32 = 1299``
    An invalid permutation array is specified.
``const MSK_RES_ERR_ARGUMENT_TYPE                       : i32 = 1198``
    Incorrect argument type.
``const MSK_RES_ERR_BAR_VAR_DIM                         : i32 = 3920``
    The dimension of a symmetric matrix variable has to greater than 0.
``const MSK_RES_ERR_BASIS                               : i32 = 1266``
    Invalid basis is specified.
``const MSK_RES_ERR_BASIS_FACTOR                        : i32 = 1610``
    The factorization of the basis is invalid.
``const MSK_RES_ERR_BASIS_SINGULAR                      : i32 = 1615``
    The basis is singular.
``const MSK_RES_ERR_BLANK_NAME                          : i32 = 1070``
    An all blank name has been specified.
``const MSK_RES_ERR_CANNOT_CLONE_NL                     : i32 = 2505``
    A task with a nonlinear function call-back cannot be cloned.
``const MSK_RES_ERR_CANNOT_HANDLE_NL                    : i32 = 2506``
    A function cannot handle a task with nonlinear function call-backs.
``const MSK_RES_ERR_CBF_DUPLICATE_ACOORD                : i32 = 7116``
    Duplicate index in ACOORD.
``const MSK_RES_ERR_CBF_DUPLICATE_BCOORD                : i32 = 7115``
    Duplicate index in BCOORD.
``const MSK_RES_ERR_CBF_DUPLICATE_CON                   : i32 = 7108``
    Duplicate CON keyword.
``const MSK_RES_ERR_CBF_DUPLICATE_INT                   : i32 = 7110``
    Duplicate INT keyword.
``const MSK_RES_ERR_CBF_DUPLICATE_OBJ                   : i32 = 7107``
    Duplicate OBJ keyword.
``const MSK_RES_ERR_CBF_DUPLICATE_OBJACOORD             : i32 = 7114``
    Duplicate index in OBJCOORD.
``const MSK_RES_ERR_CBF_DUPLICATE_VAR                   : i32 = 7109``
    Duplicate VAR keyword.
``const MSK_RES_ERR_CBF_INVALID_CON_TYPE                : i32 = 7112``
    Invalid constraint type.
``const MSK_RES_ERR_CBF_INVALID_DOMAIN_DIMENSION        : i32 = 7113``
    Invalid domain dimension.
``const MSK_RES_ERR_CBF_INVALID_INT_INDEX               : i32 = 7121``
    Invalid INT index.
``const MSK_RES_ERR_CBF_INVALID_VAR_TYPE                : i32 = 7111``
    Invalid variable type.
``const MSK_RES_ERR_CBF_NO_VARIABLES                    : i32 = 7102``
    An invalid objective sense is specified.
``const MSK_RES_ERR_CBF_NO_VERSION_SPECIFIED            : i32 = 7105``
    No version specified.
``const MSK_RES_ERR_CBF_OBJ_SENSE                       : i32 = 7101``
    An invalid objective sense is specified.
``const MSK_RES_ERR_CBF_PARSE                           : i32 = 7100``
    An error occurred while parsing an CBF file.
``const MSK_RES_ERR_CBF_SYNTAX                          : i32 = 7106``
    Invalid syntax.
``const MSK_RES_ERR_CBF_TOO_FEW_CONSTRAINTS             : i32 = 7118``
    Too few constraints defined.
``const MSK_RES_ERR_CBF_TOO_FEW_INTS                    : i32 = 7119``
    Too ints specified.
``const MSK_RES_ERR_CBF_TOO_FEW_VARIABLES               : i32 = 7117``
    Too few variables defined.
``const MSK_RES_ERR_CBF_TOO_MANY_CONSTRAINTS            : i32 = 7103``
    Too many constraints specified.
``const MSK_RES_ERR_CBF_TOO_MANY_INTS                   : i32 = 7120``
    Too ints specified.
``const MSK_RES_ERR_CBF_TOO_MANY_VARIABLES              : i32 = 7104``
    Too many variables specified.
``const MSK_RES_ERR_CBF_UNSUPPORTED                     : i32 = 7122``
    Unsupported feature is present.
``const MSK_RES_ERR_CON_Q_NOT_NSD                       : i32 = 1294``
    The quadratic constraint matrix is not NSD.
``const MSK_RES_ERR_CON_Q_NOT_PSD                       : i32 = 1293``
    The quadratic constraint matrix is not PSD.
``const MSK_RES_ERR_CONCURRENT_OPTIMIZER                : i32 = 3059``
    An unsupported optimizer was chosen for use with the concurrent optimizer.
``const MSK_RES_ERR_CONE_INDEX                          : i32 = 1300``
    An index of a non-existing cone has been specified.
``const MSK_RES_ERR_CONE_OVERLAP                        : i32 = 1302``
    One or more of variables in the cone to be added is already member of another cone.
``const MSK_RES_ERR_CONE_OVERLAP_APPEND                 : i32 = 1307``
    The cone to be appended has one variable which is already member of another cone.
``const MSK_RES_ERR_CONE_REP_VAR                        : i32 = 1303``
    A variable is included multiple times in the cone.
``const MSK_RES_ERR_CONE_SIZE                           : i32 = 1301``
    A cone with too few members is specified.
``const MSK_RES_ERR_CONE_TYPE                           : i32 = 1305``
    Invalid cone type specified.
``const MSK_RES_ERR_CONE_TYPE_STR                       : i32 = 1306``
    Invalid cone type specified.
``const MSK_RES_ERR_DATA_FILE_EXT                       : i32 = 1055``
    The data file format cannot be determined from the file name.
``const MSK_RES_ERR_DUP_NAME                            : i32 = 1071``
    Duplicate names specified.
``const MSK_RES_ERR_DUPLICATE_AIJ                       : i32 = 1385``
    An element in the A matrix is specified twice.
``const MSK_RES_ERR_DUPLICATE_BARVARIABLE_NAMES         : i32 = 4502``
    Two barvariable names are identical.
``const MSK_RES_ERR_DUPLICATE_CONE_NAMES                : i32 = 4503``
    Two cone names are identical.
``const MSK_RES_ERR_DUPLICATE_CONSTRAINT_NAMES          : i32 = 4500``
    Two constraint names are identical.
``const MSK_RES_ERR_DUPLICATE_VARIABLE_NAMES            : i32 = 4501``
    Two variable names are identical.
``const MSK_RES_ERR_END_OF_FILE                         : i32 = 1059``
    End of file reached.
``const MSK_RES_ERR_FACTOR                              : i32 = 1650``
    An error occurred while factorizing a matrix.
``const MSK_RES_ERR_FEASREPAIR_CANNOT_RELAX             : i32 = 1700``
    An optimization problem cannot be relaxed.
``const MSK_RES_ERR_FEASREPAIR_INCONSISTENT_BOUND       : i32 = 1702``
    The upper bound is less than the lower bound for a variable or a constraint.
``const MSK_RES_ERR_FEASREPAIR_SOLVING_RELAXED          : i32 = 1701``
    The relaxed problem could not be solved to optimality.
``const MSK_RES_ERR_FILE_LICENSE                        : i32 = 1007``
    Invalid license file.
``const MSK_RES_ERR_FILE_OPEN                           : i32 = 1052``
    An error occurred while opening a file.
``const MSK_RES_ERR_FILE_READ                           : i32 = 1053``
    An error occurred while reading file.
``const MSK_RES_ERR_FILE_WRITE                          : i32 = 1054``
    An error occurred while writing to a file.
``const MSK_RES_ERR_FIRST                               : i32 = 1261``
    Invalid first.
``const MSK_RES_ERR_FIRSTI                              : i32 = 1285``
    Invalid firsti.
``const MSK_RES_ERR_FIRSTJ                              : i32 = 1287``
    Invalid firstj.
``const MSK_RES_ERR_FIXED_BOUND_VALUES                  : i32 = 1425``
    A fixed constraint/variable has been specified using the bound keys but the numerical bounds are different.
``const MSK_RES_ERR_FLEXLM                              : i32 = 1014``
    The |flexlm| license manager reported an error.
``const MSK_RES_ERR_GLOBAL_INV_CONIC_PROBLEM            : i32 = 1503``
    The global optimizer can only be applied to problems without semidefinite variables.
``const MSK_RES_ERR_HUGE_AIJ                            : i32 = 1380``
    A numerically huge value is specified for an element in A.
``const MSK_RES_ERR_HUGE_C                              : i32 = 1375``
    A huge value in absolute size is specified for one an objective coefficient.
``const MSK_RES_ERR_IDENTICAL_TASKS                     : i32 = 3101``
    Some tasks related to this function call were identical. Unique tasks were expected.
``const MSK_RES_ERR_IN_ARGUMENT                         : i32 = 1200``
    A function argument is incorrect.
``const MSK_RES_ERR_INDEX                               : i32 = 1235``
    An index is out of range.
``const MSK_RES_ERR_INDEX_ARR_IS_TOO_LARGE              : i32 = 1222``
    An index in an array argument is too large.
``const MSK_RES_ERR_INDEX_ARR_IS_TOO_SMALL              : i32 = 1221``
    An index in an array argument is too small.
``const MSK_RES_ERR_INDEX_IS_TOO_LARGE                  : i32 = 1204``
    An index in an argument is too large.
``const MSK_RES_ERR_INDEX_IS_TOO_SMALL                  : i32 = 1203``
    An index in an argument is too small.
``const MSK_RES_ERR_INF_DOU_INDEX                       : i32 = 1219``
    A double information index is out of range for the specified type.
``const MSK_RES_ERR_INF_DOU_NAME                        : i32 = 1230``
    A double information name is invalid.
``const MSK_RES_ERR_INF_INT_INDEX                       : i32 = 1220``
    An integer information index is out of range for the specified type.
``const MSK_RES_ERR_INF_INT_NAME                        : i32 = 1231``
    An integer information name is invalid.
``const MSK_RES_ERR_INF_LINT_INDEX                      : i32 = 1225``
    A long integer information index is out of range for the specified type.
``const MSK_RES_ERR_INF_LINT_NAME                       : i32 = 1234``
    A long integer information name is invalid.
``const MSK_RES_ERR_INF_TYPE                            : i32 = 1232``
    The information type is invalid.
``const MSK_RES_ERR_INFEAS_UNDEFINED                    : i32 = 3910``
    The requested value is not defined for this solution type.
``const MSK_RES_ERR_INFINITE_BOUND                      : i32 = 1400``
    A numerically huge bound value is specified.
``const MSK_RES_ERR_INT64_TO_INT32_CAST                 : i32 = 3800``
    An 32 bit integer could not cast to a 64 bit integer.
``const MSK_RES_ERR_INTERNAL                            : i32 = 3000``
    An internal error occurred.
``const MSK_RES_ERR_INTERNAL_TEST_FAILED                : i32 = 3500``
    An internal unit test function failed.
``const MSK_RES_ERR_INV_APTRE                           : i32 = 1253``
    aptre[j] is strictly smaller than aptrb[j] for some j.
``const MSK_RES_ERR_INV_BK                              : i32 = 1255``
    Invalid bound key.
``const MSK_RES_ERR_INV_BKC                             : i32 = 1256``
    Invalid bound key is specified for a constraint.
``const MSK_RES_ERR_INV_BKX                             : i32 = 1257``
    An invalid bound key is specified for a variable.
``const MSK_RES_ERR_INV_CONE_TYPE                       : i32 = 1272``
    Invalid cone type code encountered.
``const MSK_RES_ERR_INV_CONE_TYPE_STR                   : i32 = 1271``
    Invalid cone type string encountered.
``const MSK_RES_ERR_INV_MARKI                           : i32 = 2501``
    Invalid value in marki.
``const MSK_RES_ERR_INV_MARKJ                           : i32 = 2502``
    Invalid value in markj.
``const MSK_RES_ERR_INV_NAME_ITEM                       : i32 = 1280``
    An invalid name item code is used.
``const MSK_RES_ERR_INV_NUMI                            : i32 = 2503``
    Invalid numi.
``const MSK_RES_ERR_INV_NUMJ                            : i32 = 2504``
    Invalid numj.
``const MSK_RES_ERR_INV_OPTIMIZER                       : i32 = 1550``
    An invalid optimizer has been chosen for the problem.
``const MSK_RES_ERR_INV_PROBLEM                         : i32 = 1500``
    Invalid problem type.
``const MSK_RES_ERR_INV_QCON_SUBI                       : i32 = 1405``
    Invalid value in qcsubi.
``const MSK_RES_ERR_INV_QCON_SUBJ                       : i32 = 1406``
    Invalid value in qcsubj.
``const MSK_RES_ERR_INV_QCON_SUBK                       : i32 = 1404``
    Invalid value in qcsubk.
``const MSK_RES_ERR_INV_QCON_VAL                        : i32 = 1407``
    Invalid value in qcval.
``const MSK_RES_ERR_INV_QOBJ_SUBI                       : i32 = 1401``
    Invalid value %d at qosubi.
``const MSK_RES_ERR_INV_QOBJ_SUBJ                       : i32 = 1402``
    Invalid value in qosubj.
``const MSK_RES_ERR_INV_QOBJ_VAL                        : i32 = 1403``
    Invalid value in qoval.
``const MSK_RES_ERR_INV_SK                              : i32 = 1270``
    Invalid status key code encountered.
``const MSK_RES_ERR_INV_SK_STR                          : i32 = 1269``
    Invalid status key string encountered.
``const MSK_RES_ERR_INV_SKC                             : i32 = 1267``
    Invalid value in skc encountered.
``const MSK_RES_ERR_INV_SKN                             : i32 = 1274``
    Invalid value in skn encountered.
``const MSK_RES_ERR_INV_SKX                             : i32 = 1268``
    Invalid value in skx encountered.
``const MSK_RES_ERR_INV_VAR_TYPE                        : i32 = 1258``
    An invalid variable type is specified for a variable.
``const MSK_RES_ERR_INVALID_ACCMODE                     : i32 = 2520``
    An invalid access mode is specified.
``const MSK_RES_ERR_INVALID_AIJ                         : i32 = 1473``
    a[i,j] contains an invalid floating point value, i.e. a NaN or an infinite value.
``const MSK_RES_ERR_INVALID_AMPL_STUB                   : i32 = 3700``
    Invalid AMPL stub.
``const MSK_RES_ERR_INVALID_BARVAR_NAME                 : i32 = 1079``
    An invalid symmetric matrix variable name is used.
``const MSK_RES_ERR_INVALID_COMPRESSION                 : i32 = 1800``
    Invalid compression type.
``const MSK_RES_ERR_INVALID_CON_NAME                    : i32 = 1076``
    An invalid constraint name is used.
``const MSK_RES_ERR_INVALID_CONE_NAME                   : i32 = 1078``
    An invalid cone name is used.
``const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_CONES       : i32 = 4005``
    The file format does not support a problem with conic constraints.
``const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_GENERAL_NL  : i32 = 4010``
    The file format does not support a problem with general nonlinear terms.
``const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_SYM_MAT     : i32 = 4000``
    The file format does not support a problem with symmetric matrix variables.
``const MSK_RES_ERR_INVALID_FILE_NAME                   : i32 = 1056``
    An invalid file name has been specified.
``const MSK_RES_ERR_INVALID_FORMAT_TYPE                 : i32 = 1283``
    Invalid format type.
``const MSK_RES_ERR_INVALID_IDX                         : i32 = 1246``
    A specified index is invalid.
``const MSK_RES_ERR_INVALID_IOMODE                      : i32 = 1801``
    Invalid io mode.
``const MSK_RES_ERR_INVALID_MAX_NUM                     : i32 = 1247``
    A specified index is invalid.
``const MSK_RES_ERR_INVALID_NAME_IN_SOL_FILE            : i32 = 1170``
    An invalid name occurred in a solution file.
``const MSK_RES_ERR_INVALID_NETWORK_PROBLEM             : i32 = 1504``
    The problem is not a network problem as expected.
``const MSK_RES_ERR_INVALID_OBJ_NAME                    : i32 = 1075``
    An invalid objective name is specified.
``const MSK_RES_ERR_INVALID_OBJECTIVE_SENSE             : i32 = 1445``
    An invalid objective sense is specified.
``const MSK_RES_ERR_INVALID_PROBLEM_TYPE                : i32 = 6000``
    An invalid problem type.
``const MSK_RES_ERR_INVALID_SOL_FILE_NAME               : i32 = 1057``
    An invalid file name has been specified.
``const MSK_RES_ERR_INVALID_STREAM                      : i32 = 1062``
    An invalid stream is referenced.
``const MSK_RES_ERR_INVALID_SURPLUS                     : i32 = 1275``
    Invalid surplus.
``const MSK_RES_ERR_INVALID_SYM_MAT_DIM                 : i32 = 3950``
    A sparse symmetric matrix of invalid dimension is specified.
``const MSK_RES_ERR_INVALID_TASK                        : i32 = 1064``
    The task is invalid.
``const MSK_RES_ERR_INVALID_UTF8                        : i32 = 2900``
    An invalid UTF8 string is encountered.
``const MSK_RES_ERR_INVALID_VAR_NAME                    : i32 = 1077``
    An invalid variable name is used.
``const MSK_RES_ERR_INVALID_WCHAR                       : i32 = 2901``
    An invalid wchar string is encountered.
``const MSK_RES_ERR_INVALID_WHICHSOL                    : i32 = 1228``
    whichsol is invalid.
``const MSK_RES_ERR_JSON_DATA                           : i32 = 1179``
    Inconsistent data in JSON Task file
``const MSK_RES_ERR_JSON_FORMAT                         : i32 = 1178``
    Error in an JSON Task file
``const MSK_RES_ERR_JSON_MISSING_DATA                   : i32 = 1180``
    Missing data section in JSON task file.
``const MSK_RES_ERR_JSON_NUMBER_OVERFLOW                : i32 = 1177``
    Invalid number entry - wrong type or value overflow.
``const MSK_RES_ERR_JSON_STRING                         : i32 = 1176``
    Error in JSON string.
``const MSK_RES_ERR_JSON_SYNTAX                         : i32 = 1175``
    Syntax error in an JSON data
``const MSK_RES_ERR_LAST                                : i32 = 1262``
    Invalid last.
``const MSK_RES_ERR_LASTI                               : i32 = 1286``
    Invalid lasti.
``const MSK_RES_ERR_LASTJ                               : i32 = 1288``
    Invalid lastj.
``const MSK_RES_ERR_LAU_ARG_K                           : i32 = 7012``
    Invalid argument k.
``const MSK_RES_ERR_LAU_ARG_M                           : i32 = 7010``
    Invalid argument m.
``const MSK_RES_ERR_LAU_ARG_N                           : i32 = 7011``
    Invalid argument n.
``const MSK_RES_ERR_LAU_ARG_TRANS                       : i32 = 7018``
    Invalid argument trans.
``const MSK_RES_ERR_LAU_ARG_TRANSA                      : i32 = 7015``
    Invalid argument transa.
``const MSK_RES_ERR_LAU_ARG_TRANSB                      : i32 = 7016``
    Invalid argument transb.
``const MSK_RES_ERR_LAU_ARG_UPLO                        : i32 = 7017``
    Invalid argument uplo.
``const MSK_RES_ERR_LAU_INVALID_LOWER_TRIANGULAR_MATRIX : i32 = 7002``
    An invalid lower triangular matrix.
``const MSK_RES_ERR_LAU_INVALID_SPARSE_SYMMETRIC_MATRIX : i32 = 7019``
    An invalid sparse symmetric matrix is specfified.
``const MSK_RES_ERR_LAU_NOT_POSITIVE_DEFINITE           : i32 = 7001``
    A matrix is not positive definite.
``const MSK_RES_ERR_LAU_SINGULAR_MATRIX                 : i32 = 7000``
    A matrix is singular.
``const MSK_RES_ERR_LAU_UNKNOWN                         : i32 = 7005``
    An unknown error.
``const MSK_RES_ERR_LICENSE                             : i32 = 1000``
    Invalid license.
``const MSK_RES_ERR_LICENSE_CANNOT_ALLOCATE             : i32 = 1020``
    The license system cannot allocate the memory required.
``const MSK_RES_ERR_LICENSE_CANNOT_CONNECT              : i32 = 1021``
    |mosek| cannot connect to the license server.
``const MSK_RES_ERR_LICENSE_EXPIRED                     : i32 = 1001``
    The license has expired.
``const MSK_RES_ERR_LICENSE_FEATURE                     : i32 = 1018``
    A requested feature is not available in the license file(s).
``const MSK_RES_ERR_LICENSE_INVALID_HOSTID              : i32 = 1025``
    The host ID specified in the license file does not match the host ID of the computer.
``const MSK_RES_ERR_LICENSE_MAX                         : i32 = 1016``
    Maximum number of licenses is reached.
``const MSK_RES_ERR_LICENSE_MOSEKLM_DAEMON              : i32 = 1017``
    The MOSEKLM license manager daemon is not up and running.
``const MSK_RES_ERR_LICENSE_NO_SERVER_LINE              : i32 = 1028``
    No SERVER lines in license file.
``const MSK_RES_ERR_LICENSE_NO_SERVER_SUPPORT           : i32 = 1027``
    The license server does not support the requested feature.
``const MSK_RES_ERR_LICENSE_SERVER                      : i32 = 1015``
    The license server is not responding.
``const MSK_RES_ERR_LICENSE_SERVER_VERSION              : i32 = 1026``
    The version specified in the checkout request is greater than the highest version number the daemon supports.
``const MSK_RES_ERR_LICENSE_VERSION                     : i32 = 1002``
    Invalid license version.
``const MSK_RES_ERR_LINK_FILE_DLL                       : i32 = 1040``
    A file cannot be linked to a stream in the DLL version.
``const MSK_RES_ERR_LIVING_TASKS                        : i32 = 1066``
    Not all tasks associated with the environment have been deleted.
``const MSK_RES_ERR_LOWER_BOUND_IS_A_NAN                : i32 = 1390``
    The lower bound specified is not a number (nan).
``const MSK_RES_ERR_LP_DUP_SLACK_NAME                   : i32 = 1152``
    The name of the slack variable added to a ranged constraint already exists.
``const MSK_RES_ERR_LP_EMPTY                            : i32 = 1151``
    The problem cannot be written to an LP formatted file.
``const MSK_RES_ERR_LP_FILE_FORMAT                      : i32 = 1157``
    Syntax error in an LP file.
``const MSK_RES_ERR_LP_FORMAT                           : i32 = 1160``
    Syntax error in an LP file.
``const MSK_RES_ERR_LP_FREE_CONSTRAINT                  : i32 = 1155``
    Free constraints cannot be written in LP file format.
``const MSK_RES_ERR_LP_INCOMPATIBLE                     : i32 = 1150``
    The problem cannot be written to an LP formatted file.
``const MSK_RES_ERR_LP_INVALID_CON_NAME                 : i32 = 1171``
    A constraint name is invalid when used in an LP formatted file.
``const MSK_RES_ERR_LP_INVALID_VAR_NAME                 : i32 = 1154``
    A variable name is invalid when used in an LP formatted file.
``const MSK_RES_ERR_LP_WRITE_CONIC_PROBLEM              : i32 = 1163``
    The problem contains cones that cannot be written to an LP formatted file.
``const MSK_RES_ERR_LP_WRITE_GECO_PROBLEM               : i32 = 1164``
    The problem contains general convex terms that cannot be written to an LP formatted file.
``const MSK_RES_ERR_LU_MAX_NUM_TRIES                    : i32 = 2800``
    Could not compute the LU factors of the matrix within the maximum number of allowed tries.
``const MSK_RES_ERR_MAX_LEN_IS_TOO_SMALL                : i32 = 1289``
    An maximum length that is too small has been specified.
``const MSK_RES_ERR_MAXNUMBARVAR                        : i32 = 1242``
    The maximum number of semidefinite variables limit is too small.
``const MSK_RES_ERR_MAXNUMCON                           : i32 = 1240``
    Invalid maximum number of constraints specified.
``const MSK_RES_ERR_MAXNUMCONE                          : i32 = 1304``
    The value specified for maxnumcone is too small.
``const MSK_RES_ERR_MAXNUMQNZ                           : i32 = 1243``
    Too small maximum number of non-zeros for the Q matrices is specified.
``const MSK_RES_ERR_MAXNUMVAR                           : i32 = 1241``
    The maximum number of variables limit is too small.
``const MSK_RES_ERR_MIO_INTERNAL                        : i32 = 5010``
    A fatal error occurred in the mixed integer optimizer.  Please contact |mosek| support.
``const MSK_RES_ERR_MIO_INVALID_NODE_OPTIMIZER          : i32 = 7131``
    An invalid node optimizer was selected for the problem type.
``const MSK_RES_ERR_MIO_INVALID_ROOT_OPTIMIZER          : i32 = 7130``
    An invalid root optimizer was selected for the problem type.
``const MSK_RES_ERR_MIO_NO_OPTIMIZER                    : i32 = 1551``
    No optimizer is available for the current class of integer optimization problems.
``const MSK_RES_ERR_MIO_NOT_LOADED                      : i32 = 1553``
    The mixed-integer optimizer is not loaded.
``const MSK_RES_ERR_MISSING_LICENSE_FILE                : i32 = 1008``
    |mosek| cannot locate a license.
``const MSK_RES_ERR_MIXED_CONIC_AND_NL                  : i32 = 1501``
    The problem contains both conic and nonlinear constraints.
``const MSK_RES_ERR_MPS_CONE_OVERLAP                    : i32 = 1118``
    A variable is specified to be a member of several cones.
``const MSK_RES_ERR_MPS_CONE_REPEAT                     : i32 = 1119``
    A variable is repeated within the CSECTION.
``const MSK_RES_ERR_MPS_CONE_TYPE                       : i32 = 1117``
    Invalid cone type specified in a  CSECTION.
``const MSK_RES_ERR_MPS_DUPLICATE_Q_ELEMENT             : i32 = 1121``
    Duplicate elements is specified in a Q matrix.
``const MSK_RES_ERR_MPS_FILE                            : i32 = 1100``
    An error occurred while reading an MPS file.
``const MSK_RES_ERR_MPS_INV_BOUND_KEY                   : i32 = 1108``
    An invalid bound key occurred in an MPS file.
``const MSK_RES_ERR_MPS_INV_CON_KEY                     : i32 = 1107``
    An invalid constraint key occurred in an MPS file.
``const MSK_RES_ERR_MPS_INV_FIELD                       : i32 = 1101``
    Invalid field occurred while reading an MPS file.
``const MSK_RES_ERR_MPS_INV_MARKER                      : i32 = 1102``
    An invalid marker has been specified in the MPS file.
``const MSK_RES_ERR_MPS_INV_SEC_NAME                    : i32 = 1109``
    An invalid section name occurred in an MPS file.
``const MSK_RES_ERR_MPS_INV_SEC_ORDER                   : i32 = 1115``
    The sections in an MPS file is not in the correct order.
``const MSK_RES_ERR_MPS_INVALID_OBJ_NAME                : i32 = 1128``
    An invalid objective name is specified.
``const MSK_RES_ERR_MPS_INVALID_OBJSENSE                : i32 = 1122``
    An invalid objective sense is specified.
``const MSK_RES_ERR_MPS_MUL_CON_NAME                    : i32 = 1112``
    A constraint name is specified multiple times in the ROWS section in an MPS file.
``const MSK_RES_ERR_MPS_MUL_CSEC                        : i32 = 1116``
    Multiple CSECTIONs are given the same name.
``const MSK_RES_ERR_MPS_MUL_QOBJ                        : i32 = 1114``
    The Q term in the objective is specified multiple times.
``const MSK_RES_ERR_MPS_MUL_QSEC                        : i32 = 1113``
    Multiple QSECTIONs are specified for a constraint.
``const MSK_RES_ERR_MPS_NO_OBJECTIVE                    : i32 = 1110``
    No objective is defined in an MPS file.
``const MSK_RES_ERR_MPS_NON_SYMMETRIC_Q                 : i32 = 1120``
    A non symmetric matrice has been speciefied.
``const MSK_RES_ERR_MPS_NULL_CON_NAME                   : i32 = 1103``
    An empty constraint name is used in an MPS file.
``const MSK_RES_ERR_MPS_NULL_VAR_NAME                   : i32 = 1104``
    An empty variable name is used in an MPS file.
``const MSK_RES_ERR_MPS_SPLITTED_VAR                    : i32 = 1111``
    The non-zero elements in A corresponding to a variable in an MPS file must be specified consecutively.
``const MSK_RES_ERR_MPS_TAB_IN_FIELD2                   : i32 = 1125``
    A tab char occurred in field 2.
``const MSK_RES_ERR_MPS_TAB_IN_FIELD3                   : i32 = 1126``
    A tab char occurred in field 3.
``const MSK_RES_ERR_MPS_TAB_IN_FIELD5                   : i32 = 1127``
    A tab char occurred in field 5.
``const MSK_RES_ERR_MPS_UNDEF_CON_NAME                  : i32 = 1105``
    An undefined constraint name occurred in an MPS file.
``const MSK_RES_ERR_MPS_UNDEF_VAR_NAME                  : i32 = 1106``
    An undefined variable name occurred in an MPS file.
``const MSK_RES_ERR_MUL_A_ELEMENT                       : i32 = 1254``
    An element in A is defined multiple times.
``const MSK_RES_ERR_NAME_IS_NULL                        : i32 = 1760``
    The name buffer is a |null| pointer.
``const MSK_RES_ERR_NAME_MAX_LEN                        : i32 = 1750``
    A name is longer than the buffer that is supposed to hold it.
``const MSK_RES_ERR_NAN_IN_BLC                          : i32 = 1461``
    blc contains an invalid floating point value, i.e. a NaN.
``const MSK_RES_ERR_NAN_IN_BLX                          : i32 = 1471``
    blx contains an invalid floating point value, i.e. a NaN.
``const MSK_RES_ERR_NAN_IN_BUC                          : i32 = 1462``
    buc contains an invalid floating point value, i.e. a NaN.
``const MSK_RES_ERR_NAN_IN_BUX                          : i32 = 1472``
    bux contains an invalid floating point value, i.e. a NaN.
``const MSK_RES_ERR_NAN_IN_C                            : i32 = 1470``
    c contains an invalid floating point value, i.e. a NaN.
``const MSK_RES_ERR_NAN_IN_DOUBLE_DATA                  : i32 = 1450``
    An invalid floating value was used in some double data.
``const MSK_RES_ERR_NEGATIVE_APPEND                     : i32 = 1264``
    Cannot append a negative number.
``const MSK_RES_ERR_NEGATIVE_SURPLUS                    : i32 = 1263``
    Negative surplus.
``const MSK_RES_ERR_NEWER_DLL                           : i32 = 1036``
    The dynamic link library is newer than the specified version.
``const MSK_RES_ERR_NO_BARS_FOR_SOLUTION                : i32 = 3916``
    There is no bars available for the solution specified.
``const MSK_RES_ERR_NO_BARX_FOR_SOLUTION                : i32 = 3915``
    There is no barx available for the solution specified.
``const MSK_RES_ERR_NO_BASIS_SOL                        : i32 = 1600``
    No basic solution is defined.
``const MSK_RES_ERR_NO_DUAL_FOR_ITG_SOL                 : i32 = 2950``
    No dual information is available for the integer solution.
``const MSK_RES_ERR_NO_DUAL_INFEAS_CER                  : i32 = 2001``
    A certificate of dual infeasibility is not available.
``const MSK_RES_ERR_NO_INIT_ENV                         : i32 = 1063``
    Environment is not initialized.
``const MSK_RES_ERR_NO_OPTIMIZER_VAR_TYPE               : i32 = 1552``
    No optimizer is available for this class of optimization problems.
``const MSK_RES_ERR_NO_PRIMAL_INFEAS_CER                : i32 = 2000``
    A certificate of primal infeasibility is not available.
``const MSK_RES_ERR_NO_SNX_FOR_BAS_SOL                  : i32 = 2953``
    snx is not available for the basis solution.
``const MSK_RES_ERR_NO_SOLUTION_IN_CALLBACK             : i32 = 2500``
    The required solution is not available.
``const MSK_RES_ERR_NON_UNIQUE_ARRAY                    : i32 = 5000``
    An array does not contain unique elements.
``const MSK_RES_ERR_NONCONVEX                           : i32 = 1291``
    The optimization problem is nonconvex.
``const MSK_RES_ERR_NONLINEAR_EQUALITY                  : i32 = 1290``
    The model contains a nonlinear equality.
``const MSK_RES_ERR_NONLINEAR_FUNCTIONS_NOT_ALLOWED     : i32 = 1428``
    An operation that is invalid for problems with nonlinear functions defined has been attempted.
``const MSK_RES_ERR_NONLINEAR_RANGED                    : i32 = 1292``
    The problem contains a nonlinear constraint with inite lower and upper bound.
``const MSK_RES_ERR_NR_ARGUMENTS                        : i32 = 1199``
    Incorrect number of function arguments.
``const MSK_RES_ERR_NULL_ENV                            : i32 = 1060``
    env is a |null| pointer.
``const MSK_RES_ERR_NULL_POINTER                        : i32 = 1065``
    An argument to a function is unexpectedly a |null| pointer.
``const MSK_RES_ERR_NULL_TASK                           : i32 = 1061``
    task is a |null| pointer.
``const MSK_RES_ERR_NUMCONLIM                           : i32 = 1250``
    Maximum number of constraints limit is exceeded.
``const MSK_RES_ERR_NUMVARLIM                           : i32 = 1251``
    Maximum number of variables limit is exceeded.
``const MSK_RES_ERR_OBJ_Q_NOT_NSD                       : i32 = 1296``
    The quadratic coefficient matrix in the objective is not NSD.
``const MSK_RES_ERR_OBJ_Q_NOT_PSD                       : i32 = 1295``
    The quadratic coefficient matrix in the objective is not PSD.
``const MSK_RES_ERR_OBJECTIVE_RANGE                     : i32 = 1260``
    Empty objective range.
``const MSK_RES_ERR_OLDER_DLL                           : i32 = 1035``
    The dynamic link library is older than the specified version.
``const MSK_RES_ERR_OPEN_DL                             : i32 = 1030``
    A dynamic link library could not be opened.
``const MSK_RES_ERR_OPF_FORMAT                          : i32 = 1168``
    Syntax error in an OPF file
``const MSK_RES_ERR_OPF_NEW_VARIABLE                    : i32 = 1169``
    Variable not previously defined.
``const MSK_RES_ERR_OPF_PREMATURE_EOF                   : i32 = 1172``
    Premature end of file in an OPF file.
``const MSK_RES_ERR_OPTIMIZER_LICENSE                   : i32 = 1013``
    The optimizer required is not licensed.
``const MSK_RES_ERR_OVERFLOW                            : i32 = 1590``
    A computation produced an overflow.
``const MSK_RES_ERR_PARAM_INDEX                         : i32 = 1210``
    Parameter index is out of range.
``const MSK_RES_ERR_PARAM_IS_TOO_LARGE                  : i32 = 1215``
    A parameter value is too large.
``const MSK_RES_ERR_PARAM_IS_TOO_SMALL                  : i32 = 1216``
    A parameter value is too small.
``const MSK_RES_ERR_PARAM_NAME                          : i32 = 1205``
    A parameter name is not correct.
``const MSK_RES_ERR_PARAM_NAME_DOU                      : i32 = 1206``
    A parameter name is not correct.
``const MSK_RES_ERR_PARAM_NAME_INT                      : i32 = 1207``
    A parameter name is not correct.
``const MSK_RES_ERR_PARAM_NAME_STR                      : i32 = 1208``
    A parameter name is not correct.
``const MSK_RES_ERR_PARAM_TYPE                          : i32 = 1218``
    A parameter type is invalid.
``const MSK_RES_ERR_PARAM_VALUE_STR                     : i32 = 1217``
    A parameter value string is incorrect.
``const MSK_RES_ERR_PLATFORM_NOT_LICENSED               : i32 = 1019``
    A requested license feature is not available for the required platform.
``const MSK_RES_ERR_POSTSOLVE                           : i32 = 1580``
    An error occurred during the postsolve.
``const MSK_RES_ERR_PRO_ITEM                            : i32 = 1281``
    An invalid problem item is used.
``const MSK_RES_ERR_PROB_LICENSE                        : i32 = 1006``
    The software is not licensed to solve the problem.
``const MSK_RES_ERR_QCON_SUBI_TOO_LARGE                 : i32 = 1409``
    Invalid value in qcsubi.
``const MSK_RES_ERR_QCON_SUBI_TOO_SMALL                 : i32 = 1408``
    Invalid value in qcsubi.
``const MSK_RES_ERR_QCON_UPPER_TRIANGLE                 : i32 = 1417``
    An element in the upper triangle of the quadratic term in a constraint.
``const MSK_RES_ERR_QOBJ_UPPER_TRIANGLE                 : i32 = 1415``
    An element in the upper triangle of the quadratic term in the objective is specified.
``const MSK_RES_ERR_READ_FORMAT                         : i32 = 1090``
    The specified format cannot be read.
``const MSK_RES_ERR_READ_LP_MISSING_END_TAG             : i32 = 1159``
    Syntax error in LP fil. Possibly missing End tag.
``const MSK_RES_ERR_READ_LP_NONEXISTING_NAME            : i32 = 1162``
    A variable never occurred in objective or constraints.
``const MSK_RES_ERR_REMOVE_CONE_VARIABLE                : i32 = 1310``
    A variable cannot be removed because it will make a cone invalid.
``const MSK_RES_ERR_REPAIR_INVALID_PROBLEM              : i32 = 1710``
    The feasibility repair does not support the specified problem type.
``const MSK_RES_ERR_REPAIR_OPTIMIZATION_FAILED          : i32 = 1711``
    Computation the optimal relaxation failed.
``const MSK_RES_ERR_SEN_BOUND_INVALID_LO                : i32 = 3054``
    Analysis of lower bound requested for an index, where no lower bound exists.
``const MSK_RES_ERR_SEN_BOUND_INVALID_UP                : i32 = 3053``
    Analysis of upper bound requested for an index, where no upper bound exists.
``const MSK_RES_ERR_SEN_FORMAT                          : i32 = 3050``
    Syntax error in sensitivity analysis file.
``const MSK_RES_ERR_SEN_INDEX_INVALID                   : i32 = 3055``
    Invalid range given in the sensitivity file.
``const MSK_RES_ERR_SEN_INDEX_RANGE                     : i32 = 3052``
    Index out of range in the sensitivity analysis file.
``const MSK_RES_ERR_SEN_INVALID_REGEXP                  : i32 = 3056``
    Syntax error in regexp or regexp longer than 1024.
``const MSK_RES_ERR_SEN_NUMERICAL                       : i32 = 3058``
    Numerical difficulties encountered performing the sensitivity analysis.
``const MSK_RES_ERR_SEN_SOLUTION_STATUS                 : i32 = 3057``
    No optimal solution found to the original problem given for sensitivity analysis.
``const MSK_RES_ERR_SEN_UNDEF_NAME                      : i32 = 3051``
    An undefined name was encountered in the sensitivity analysis file.
``const MSK_RES_ERR_SEN_UNHANDLED_PROBLEM_TYPE          : i32 = 3080``
    Sensitivity analysis cannot be performed for the specified problem.
``const MSK_RES_ERR_SERVER_CONNECT                      : i32 = 8000``
    Failed to connect to remote solver server.
``const MSK_RES_ERR_SERVER_PROTOCOL                     : i32 = 8001``
    Unexpected message or data from solver server.
``const MSK_RES_ERR_SERVER_STATUS                       : i32 = 8002``
    Server returned non-ok status code
``const MSK_RES_ERR_SERVER_TOKEN                        : i32 = 8003``
    Invalid job ID
``const MSK_RES_ERR_SIZE_LICENSE                        : i32 = 1005``
    The problem is bigger than the license.
``const MSK_RES_ERR_SIZE_LICENSE_CON                    : i32 = 1010``
    The problem has too many constraints.
``const MSK_RES_ERR_SIZE_LICENSE_INTVAR                 : i32 = 1012``
    The problem contains too many integer variables.
``const MSK_RES_ERR_SIZE_LICENSE_NUMCORES               : i32 = 3900``
    The computer contains more cpu cores than the license allows for.
``const MSK_RES_ERR_SIZE_LICENSE_VAR                    : i32 = 1011``
    The problem has too many variables.
``const MSK_RES_ERR_SOL_FILE_INVALID_NUMBER             : i32 = 1350``
    An invalid number is specified in a solution file.
``const MSK_RES_ERR_SOLITEM                             : i32 = 1237``
    The solution number  solemn does not exists.
``const MSK_RES_ERR_SOLVER_PROBTYPE                     : i32 = 1259``
    Problem type does not match the chosen optimizer.
``const MSK_RES_ERR_SPACE                               : i32 = 1051``
    Out of space.
``const MSK_RES_ERR_SPACE_LEAKING                       : i32 = 1080``
    |mosek| is leaking memory.
``const MSK_RES_ERR_SPACE_NO_INFO                       : i32 = 1081``
    No available information about the space usage.
``const MSK_RES_ERR_SYM_MAT_DUPLICATE                   : i32 = 3944``
    A value in a symmetric matric as been specified more than once.
``const MSK_RES_ERR_SYM_MAT_INVALID_COL_INDEX           : i32 = 3941``
    A column index specified for sparse symmetric matrix is invalid.
``const MSK_RES_ERR_SYM_MAT_INVALID_ROW_INDEX           : i32 = 3940``
    A row index specified for sparse symmetric matrix is invalid.
``const MSK_RES_ERR_SYM_MAT_INVALID_VALUE               : i32 = 3943``
    The numerical value specified in a sparse symmetric matrix is not a value floating value.
``const MSK_RES_ERR_SYM_MAT_NOT_LOWER_TRINGULAR         : i32 = 3942``
    Only the lower triangular part of sparse symmetric matrix should be specified.
``const MSK_RES_ERR_TASK_INCOMPATIBLE                   : i32 = 2560``
    The Task file is incompatible with  this platform.
``const MSK_RES_ERR_TASK_INVALID                        : i32 = 2561``
    The Task file is invalid.
``const MSK_RES_ERR_THREAD_COND_INIT                    : i32 = 1049``
    Could not initialize a condition.
``const MSK_RES_ERR_THREAD_CREATE                       : i32 = 1048``
    Could not create a thread.
``const MSK_RES_ERR_THREAD_MUTEX_INIT                   : i32 = 1045``
    Could not initialize a mutex.
``const MSK_RES_ERR_THREAD_MUTEX_LOCK                   : i32 = 1046``
    Could not lock a mutex.
``const MSK_RES_ERR_THREAD_MUTEX_UNLOCK                 : i32 = 1047``
    Could not unlock a mutex.
``const MSK_RES_ERR_TOCONIC_CONSTR_NOT_CONIC            : i32 = 7153``
    The constraint is not conic representable.
``const MSK_RES_ERR_TOCONIC_CONSTR_Q_NOT_PSD            : i32 = 7150``
    The matrix defining the quadratric part of constraint is not positive semidefinite.
``const MSK_RES_ERR_TOCONIC_CONSTRAINT_FX               : i32 = 7151``
    The quadratic constraint is an equality, thus not convex.
``const MSK_RES_ERR_TOCONIC_CONSTRAINT_RA               : i32 = 7152``
    The quadratic constraint has finite lower and upper bound, and therefore it is not convex.
``const MSK_RES_ERR_TOCONIC_OBJECTIVE_NOT_PSD           : i32 = 7155``
    The matrix defining the quadratric part of the objective function is not positive semidefinite.
``const MSK_RES_ERR_TOO_MANY_CONCURRENT_TASKS           : i32 = 3090``
    Too many concurrent tasks specified.
``const MSK_RES_ERR_TOO_SMALL_MAX_NUM_NZ                : i32 = 1245``
    The maximum number of non-zeros specified is too small.
``const MSK_RES_ERR_TOO_SMALL_MAXNUMANZ                 : i32 = 1252``
    Too small maximum number of non-zeros in A specified.
``const MSK_RES_ERR_UNB_STEP_SIZE                       : i32 = 3100``
    A step-size in an optimizer was unexpectedly unbounded.
``const MSK_RES_ERR_UNDEF_SOLUTION                      : i32 = 1265``
    The required solution is not defined.
``const MSK_RES_ERR_UNDEFINED_OBJECTIVE_SENSE           : i32 = 1446``
    The objective sense has not been specified before the optimization.
``const MSK_RES_ERR_UNHANDLED_SOLUTION_STATUS           : i32 = 6010``
    Unhandled solution status.
``const MSK_RES_ERR_UNKNOWN                             : i32 = 1050``
    Unknown error.
``const MSK_RES_ERR_UPPER_BOUND_IS_A_NAN                : i32 = 1391``
    The upper bound specified is not a number (nan).
``const MSK_RES_ERR_UPPER_TRIANGLE                      : i32 = 6020``
    An element in the upper triangle of a lower triangular matrix is specified.
``const MSK_RES_ERR_USER_FUNC_RET                       : i32 = 1430``
    An user function reported an error.
``const MSK_RES_ERR_USER_FUNC_RET_DATA                  : i32 = 1431``
    An user function returned invalid data.
``const MSK_RES_ERR_USER_NLO_EVAL                       : i32 = 1433``
    The user-defined nonlinear function reported an error.
``const MSK_RES_ERR_USER_NLO_EVAL_HESSUBI               : i32 = 1440``
    The user-defined nonlinear function reported an Hessian an invalid subscript.
``const MSK_RES_ERR_USER_NLO_EVAL_HESSUBJ               : i32 = 1441``
    The user-defined nonlinear function reported an invalid subscript in the Hessian.
``const MSK_RES_ERR_USER_NLO_FUNC                       : i32 = 1432``
    The user-defined nonlinear function reported an error.
``const MSK_RES_ERR_WHICHITEM_NOT_ALLOWED               : i32 = 1238``
    whichitem is unacceptable.
``const MSK_RES_ERR_WHICHSOL                            : i32 = 1236``
    The solution defined by whichsol does not exists.
``const MSK_RES_ERR_WRITE_LP_FORMAT                     : i32 = 1158``
    Problem cannot be written as an LP file.
``const MSK_RES_ERR_WRITE_LP_NON_UNIQUE_NAME            : i32 = 1161``
    An auto-generated name is not unique.
``const MSK_RES_ERR_WRITE_MPS_INVALID_NAME              : i32 = 1153``
    An invalid name is created while writing an MPS file.
``const MSK_RES_ERR_WRITE_OPF_INVALID_VAR_NAME          : i32 = 1156``
    Empty variable names cannot be written to OPF files.
``const MSK_RES_ERR_WRITING_FILE                        : i32 = 1166``
    An error occurred while writing file
``const MSK_RES_ERR_XML_INVALID_PROBLEM_TYPE            : i32 = 3600``
    The problem type is not supported by the XML format.
``const MSK_RES_ERR_Y_IS_UNDEFINED                      : i32 = 1449``
    The solution item y is undefined.
``const MSK_RES_OK                                      : i32 = 0``
    No error occurred.
``const MSK_RES_TRM_INTERNAL                            : i32 = 10030``
    The optimizer terminated due to some internal reason.
``const MSK_RES_TRM_INTERNAL_STOP                       : i32 = 10031``
    The optimizer terminated for internal reasons.
``const MSK_RES_TRM_MAX_ITERATIONS                      : i32 = 10000``
    The optimizer terminated at the maximum number of iterations.
``const MSK_RES_TRM_MAX_NUM_SETBACKS                    : i32 = 10020``
    The optimizer terminated as the maximum number of set-backs was reached.
``const MSK_RES_TRM_MAX_TIME                            : i32 = 10001``
    The optimizer terminated at the maximum amount of time.
``const MSK_RES_TRM_MIO_NEAR_ABS_GAP                    : i32 = 10004``
    The mixed-integer optimizer terminated because the near optimal absolute gap tolerance was satisfied.
``const MSK_RES_TRM_MIO_NEAR_REL_GAP                    : i32 = 10003``
    The mixed-integer optimizer terminated because the near optimal relative gap tolerance was satisfied.
``const MSK_RES_TRM_MIO_NUM_BRANCHES                    : i32 = 10009``
    The mixed-integer optimizer terminated as to the maximum number of branches was reached.
``const MSK_RES_TRM_MIO_NUM_RELAXS                      : i32 = 10008``
    The mixed-integer optimizer terminated as the maximum number of relaxations was reached.
``const MSK_RES_TRM_NUM_MAX_NUM_INT_SOLUTIONS           : i32 = 10015``
    The mixed-integer optimizer terminated as the maximum number of feasible solutions was reached.
``const MSK_RES_TRM_NUMERICAL_PROBLEM                   : i32 = 10025``
    The optimizer terminated due to a numerical problem.
``const MSK_RES_TRM_OBJECTIVE_RANGE                     : i32 = 10002``
    The optimizer terminated on the bound of the objective range.
``const MSK_RES_TRM_STALL                               : i32 = 10006``
    The optimizer is terminated due to slow progress.
``const MSK_RES_TRM_USER_CALLBACK                       : i32 = 10007``
    The user-defined progress call-back function terminated the optimization.
``const MSK_RES_WRN_ANA_ALMOST_INT_BOUNDS               : i32 = 904``
    Warn against almost integral bounds.
``const MSK_RES_WRN_ANA_C_ZERO                          : i32 = 901``
    Warn against all objective coefficients being zero.
``const MSK_RES_WRN_ANA_CLOSE_BOUNDS                    : i32 = 903``
    Warn against close bounds.
``const MSK_RES_WRN_ANA_EMPTY_COLS                      : i32 = 902``
    Warn against empty columns.
``const MSK_RES_WRN_ANA_LARGE_BOUNDS                    : i32 = 900``
    Warn against very large bounds.
``const MSK_RES_WRN_CONSTRUCT_INVALID_SOL_ITG           : i32 = 807``
    The initial value for one or more  of the integer variables is not feasible.
``const MSK_RES_WRN_CONSTRUCT_NO_SOL_ITG                : i32 = 810``
    The construct solution requires an integer solution.
``const MSK_RES_WRN_CONSTRUCT_SOLUTION_INFEAS           : i32 = 805``
    After fixing the integer variables at the suggested values then the problem is infeasible.
``const MSK_RES_WRN_DROPPED_NZ_QOBJ                     : i32 = 201``
    One or more non-zero elements were dropped in the Q matrix in the objective.
``const MSK_RES_WRN_DUPLICATE_BARVARIABLE_NAMES         : i32 = 852``
    Two barvariable names are identical.
``const MSK_RES_WRN_DUPLICATE_CONE_NAMES                : i32 = 853``
    Two cone names are identical.
``const MSK_RES_WRN_DUPLICATE_CONSTRAINT_NAMES          : i32 = 850``
    Two constraint names are identical.
``const MSK_RES_WRN_DUPLICATE_VARIABLE_NAMES            : i32 = 851``
    Two variable names are identical.
``const MSK_RES_WRN_ELIMINATOR_SPACE                    : i32 = 801``
    The eliminator is skipped at least once due to lack of space.
``const MSK_RES_WRN_EMPTY_NAME                          : i32 = 502``
    A variable or constraint name is empty. The output file may be invalid.
``const MSK_RES_WRN_IGNORE_INTEGER                      : i32 = 250``
    Ignored integer constraints.
``const MSK_RES_WRN_INCOMPLETE_LINEAR_DEPENDENCY_CHECK  : i32 = 800``
    The linear dependency check(s) is incomplete.
``const MSK_RES_WRN_LARGE_AIJ                           : i32 = 62``
    A numerically large value is specified for an element in A.
``const MSK_RES_WRN_LARGE_BOUND                         : i32 = 51``
    A numerically large bound value is specified.
``const MSK_RES_WRN_LARGE_CJ                            : i32 = 57``
    A numerically large value is specified for one element in A.
``const MSK_RES_WRN_LARGE_CON_FX                        : i32 = 54``
    A equality constraint is fixed to numerically large value.
``const MSK_RES_WRN_LARGE_LO_BOUND                      : i32 = 52``
    A numerically large lower bound value is specified.
``const MSK_RES_WRN_LARGE_UP_BOUND                      : i32 = 53``
    A numerically large upper bound value is specified.
``const MSK_RES_WRN_LICENSE_EXPIRE                      : i32 = 500``
    The license expires.
``const MSK_RES_WRN_LICENSE_FEATURE_EXPIRE              : i32 = 505``
    The license expires.
``const MSK_RES_WRN_LICENSE_SERVER                      : i32 = 501``
    The license server is not responding.
``const MSK_RES_WRN_LP_DROP_VARIABLE                    : i32 = 85``
    Ignore a variable because the variable was not previously defined.
``const MSK_RES_WRN_LP_OLD_QUAD_FORMAT                  : i32 = 80``
    Missing '/2' after quadratic expressions in bound or objective.
``const MSK_RES_WRN_MIO_INFEASIBLE_FINAL                : i32 = 270``
    The final mixed-integer problem with all the integer variables fixed at their optimal values is infeasible.
``const MSK_RES_WRN_MPS_SPLIT_BOU_VECTOR                : i32 = 72``
    A BOUNDS vector is split into several nonadjacent parts in an MPS file.
``const MSK_RES_WRN_MPS_SPLIT_RAN_VECTOR                : i32 = 71``
    A RANGE vector is split into several nonadjacent parts in an MPS file.
``const MSK_RES_WRN_MPS_SPLIT_RHS_VECTOR                : i32 = 70``
    An RHS vector is split into several nonadjacent parts.
``const MSK_RES_WRN_NAME_MAX_LEN                        : i32 = 65``
    A name is longer than the buffer that is supposed to hold it.
``const MSK_RES_WRN_NO_DUALIZER                         : i32 = 950``
    No automatic dualizer is available for the specified problem.
``const MSK_RES_WRN_NO_GLOBAL_OPTIMIZER                 : i32 = 251``
    No global optimizer is available.
``const MSK_RES_WRN_NO_NONLINEAR_FUNCTION_WRITE         : i32 = 450``
    The problem contains a general nonlinear function that cannot be written to a disk file.
``const MSK_RES_WRN_NZ_IN_UPR_TRI                       : i32 = 200``
    Non-zero elements specified in the upper triangle of a matrix were ignored.
``const MSK_RES_WRN_OPEN_PARAM_FILE                     : i32 = 50``
    The parameter file could not be opened.
``const MSK_RES_WRN_PARAM_IGNORED_CMIO                  : i32 = 516``
    A parameter was ignored by the conic mixed integer optimizer.
``const MSK_RES_WRN_PARAM_NAME_DOU                      : i32 = 510``
    Parameter name not recognized.
``const MSK_RES_WRN_PARAM_NAME_INT                      : i32 = 511``
    Parameter name not recognized.
``const MSK_RES_WRN_PARAM_NAME_STR                      : i32 = 512``
    Parameter name not recognized.
``const MSK_RES_WRN_PARAM_STR_VALUE                     : i32 = 515``
    A parameter value is not correct.
``const MSK_RES_WRN_PRESOLVE_OUTOFSPACE                 : i32 = 802``
    The presolve is incomplete due to lack of space.
``const MSK_RES_WRN_QUAD_CONES_WITH_ROOT_FIXED_AT_ZERO  : i32 = 930``
    For at least one quadratic cone the root is fixed at (nearly) zero.
``const MSK_RES_WRN_RQUAD_CONES_WITH_ROOT_FIXED_AT_ZERO : i32 = 931``
    For at least one rotated quadratic cone the root is fixed at (nearly) zero.
``const MSK_RES_WRN_SOL_FILE_IGNORED_CON                : i32 = 351``
    One or more lines in the constraint section were ignored when reading a solution file.
``const MSK_RES_WRN_SOL_FILE_IGNORED_VAR                : i32 = 352``
    One or more lines in the variable section were ignored when reading a solution file.
``const MSK_RES_WRN_SOL_FILTER                          : i32 = 300``
    Invalid solution filter is specified.
``const MSK_RES_WRN_SPAR_MAX_LEN                        : i32 = 66``
    A value for a string parameter is longer than the buffer that is supposed to hold it.
``const MSK_RES_WRN_TOO_FEW_BASIS_VARS                  : i32 = 400``
    An incomplete basis is specified.
``const MSK_RES_WRN_TOO_MANY_BASIS_VARS                 : i32 = 405``
    A basis with too many variables is specified.
``const MSK_RES_WRN_TOO_MANY_THREADS_CONCURRENT         : i32 = 750``
    The concurrent optimizer employs more threads than available.
``const MSK_RES_WRN_UNDEF_SOL_FILE_NAME                 : i32 = 350``
    Undefined name occurred in a solution.
``const MSK_RES_WRN_USING_GENERIC_NAMES                 : i32 = 503``
    Generic names are used because a name is not valid.
``const MSK_RES_WRN_WRITE_CHANGED_NAMES                 : i32 = 803``
    Some names were changed because they were invalid for the output file format.
``const MSK_RES_WRN_WRITE_DISCARDED_CFIX                : i32 = 804``
    The fixed objective term was discarded in the output file.
``const MSK_RES_WRN_ZERO_AIJ                            : i32 = 63``
    One or more zero elements are specified in A.
``const MSK_RES_WRN_ZEROS_IN_SPARSE_COL                 : i32 = 710``
    One or more (near) zero elements are specified in a sparse column of a matrix.
``const MSK_RES_WRN_ZEROS_IN_SPARSE_ROW                 : i32 = 705``
    One or more (near) zero elements are specified in a sparse row of a matrix.
.. index:: rescodetype
.. index:: RESPONSE_...
.. _enum_rescodetype:
.. _rescodetype_err:
.. _rescodetype_ok:
.. _rescodetype_trm:
.. _rescodetype_unk:
.. _rescodetype_wrn:

``rescodetype``
---------------

Response code type

``const MSK_RESPONSE_ERR : i32 = 3``
    The response code is an error.
``const MSK_RESPONSE_OK  : i32 = 0``
    The response code is OK.
``const MSK_RESPONSE_TRM : i32 = 2``
    The response code is an optimizer termination status.
``const MSK_RESPONSE_UNK : i32 = 4``
    The response code does not belong to any class.
``const MSK_RESPONSE_WRN : i32 = 1``
    The response code is a warning.
.. index:: scalingmethod
.. index:: SCALING_METHOD_...
.. _enum_scalingmethod:
.. _scalingmethod_free:
.. _scalingmethod_pow2:

``scalingmethod``
-----------------

Scaling method

``const MSK_SCALING_METHOD_FREE : i32 = 1``
    The optimizer chooses the scaling heuristic.
``const MSK_SCALING_METHOD_POW2 : i32 = 0``
    Scales only with power of 2 leaving the mantissa untouched.
.. index:: scalingtype
.. index:: SCALING_...
.. _enum_scalingtype:
.. _scalingtype_aggressive:
.. _scalingtype_free:
.. _scalingtype_moderate:
.. _scalingtype_none:

``scalingtype``
---------------

Scaling type

``const MSK_SCALING_AGGRESSIVE : i32 = 3``
    A very aggressive scaling is performed.
``const MSK_SCALING_FREE       : i32 = 0``
    The optimizer chooses the scaling heuristic.
``const MSK_SCALING_MODERATE   : i32 = 2``
    A conservative scaling is performed.
``const MSK_SCALING_NONE       : i32 = 1``
    No scaling is performed.
.. index:: sensitivitytype
.. index:: SENSITIVITY_TYPE_...
.. _enum_sensitivitytype:
.. _sensitivitytype_basis:
.. _sensitivitytype_optimal_partition:

``sensitivitytype``
-------------------

Sensitivity types

``const MSK_SENSITIVITY_TYPE_BASIS             : i32 = 0``
    Basis sensitivity analysis is performed.
``const MSK_SENSITIVITY_TYPE_OPTIMAL_PARTITION : i32 = 1``
    Optimal partition sensitivity analysis is performed.
.. index:: simdegen
.. index:: SIM_DEGEN_...
.. _enum_simdegen:
.. _simdegen_aggressive:
.. _simdegen_free:
.. _simdegen_minimum:
.. _simdegen_moderate:
.. _simdegen_none:

``simdegen``
------------

Degeneracy strategies

``const MSK_SIM_DEGEN_AGGRESSIVE : i32 = 2``
    The simplex optimizer should use an aggressive degeneration strategy.
``const MSK_SIM_DEGEN_FREE       : i32 = 1``
    The simplex optimizer chooses the degeneration strategy.
``const MSK_SIM_DEGEN_MINIMUM    : i32 = 4``
    The simplex optimizer should use a minimum degeneration strategy.
``const MSK_SIM_DEGEN_MODERATE   : i32 = 3``
    The simplex optimizer should use a moderate degeneration strategy.
``const MSK_SIM_DEGEN_NONE       : i32 = 0``
    The simplex optimizer should use no degeneration strategy.
.. index:: simdupvec
.. index:: SIM_EXPLOIT_DUPVEC_...
.. _enum_simdupvec:
.. _simdupvec_free:
.. _simdupvec_off:
.. _simdupvec_on:

``simdupvec``
-------------

Exploit duplicate columns.

``const MSK_SIM_EXPLOIT_DUPVEC_FREE : i32 = 2``
    The simplex optimizer can choose freely.
``const MSK_SIM_EXPLOIT_DUPVEC_OFF  : i32 = 0``
    Disallow the simplex optimizer to exploit duplicated columns.
``const MSK_SIM_EXPLOIT_DUPVEC_ON   : i32 = 1``
    Allow the simplex optimizer to exploit duplicated columns.
.. index:: simhotstart
.. index:: SIM_HOTSTART_...
.. _enum_simhotstart:
.. _simhotstart_free:
.. _simhotstart_none:
.. _simhotstart_status_keys:

``simhotstart``
---------------

Hot-start type employed by the simplex optimizer

``const MSK_SIM_HOTSTART_FREE        : i32 = 1``
    The simplex optimize chooses the hot-start type.
``const MSK_SIM_HOTSTART_NONE        : i32 = 0``
    The simplex optimizer performs a coldstart.
``const MSK_SIM_HOTSTART_STATUS_KEYS : i32 = 2``
    Only the status keys of the constraints and variables are used
                       to choose the type of hot-start.
.. index:: simreform
.. index:: SIM_REFORMULATION_...
.. _enum_simreform:
.. _simreform_aggressive:
.. _simreform_free:
.. _simreform_off:
.. _simreform_on:

``simreform``
-------------

Problem reformulation.

``const MSK_SIM_REFORMULATION_AGGRESSIVE : i32 = 3``
    The simplex optimizer should use an aggressive reformulation strategy.
``const MSK_SIM_REFORMULATION_FREE       : i32 = 2``
    The simplex optimizer can choose freely.
``const MSK_SIM_REFORMULATION_OFF        : i32 = 0``
    Disallow the simplex optimizer to reformulate the problem.
``const MSK_SIM_REFORMULATION_ON         : i32 = 1``
    Allow the simplex optimizer to reformulate the problem.
.. index:: simseltype
.. index:: SIM_SELECTION_...
.. _enum_simseltype:
.. _simseltype_ase:
.. _simseltype_devex:
.. _simseltype_free:
.. _simseltype_full:
.. _simseltype_partial:
.. _simseltype_se:

``simseltype``
--------------

Simplex selection strategy

``const MSK_SIM_SELECTION_ASE     : i32 = 2``
    The optimizer uses approximate steepest-edge
                        pricing.
``const MSK_SIM_SELECTION_DEVEX   : i32 = 3``
    The optimizer uses devex steepest-edge pricing (or if it is not available an
                        approximate steep-edge selection).
``const MSK_SIM_SELECTION_FREE    : i32 = 0``
    The optimizer chooses the pricing strategy.
``const MSK_SIM_SELECTION_FULL    : i32 = 1``
    The optimizer uses full pricing.
``const MSK_SIM_SELECTION_PARTIAL : i32 = 5``
    The optimizer uses a partial selection approach. The approach is usually
                        beneficial if the number of variables is much larger than  the number of constraints.
``const MSK_SIM_SELECTION_SE      : i32 = 4``
    The optimizer uses steepest-edge selection (or if it is not available an
                        approximate steep-edge selection).
.. index:: solitem
.. index:: SOL_ITEM_...
.. _enum_solitem:
.. _solitem_slc:
.. _solitem_slx:
.. _solitem_snx:
.. _solitem_suc:
.. _solitem_sux:
.. _solitem_xc:
.. _solitem_xx:
.. _solitem_y:

``solitem``
-----------

Solution items

``const MSK_SOL_ITEM_SLC : i32 = 3``
    Lagrange multipliers for lower
                        bounds on the constraints.
``const MSK_SOL_ITEM_SLX : i32 = 5``
    Lagrange multipliers for lower
                        bounds on the variables.
``const MSK_SOL_ITEM_SNX : i32 = 7``
    Lagrange multipliers corresponding to the conic constraints on the variables.
``const MSK_SOL_ITEM_SUC : i32 = 4``
    Lagrange multipliers for upper
                        bounds on the constraints.
``const MSK_SOL_ITEM_SUX : i32 = 6``
    Lagrange multipliers for upper
                        bounds on the variables.
``const MSK_SOL_ITEM_XC  : i32 = 0``
    Solution for the constraints.
``const MSK_SOL_ITEM_XX  : i32 = 1``
    Variable solution.
``const MSK_SOL_ITEM_Y   : i32 = 2``
    Lagrange multipliers for equations.
.. index:: solsta
.. index:: SOL_STA_...
.. _enum_solsta:
.. _solsta_dual_feas:
.. _solsta_dual_illposed_cer:
.. _solsta_dual_infeas_cer:
.. _solsta_integer_optimal:
.. _solsta_near_dual_feas:
.. _solsta_near_dual_infeas_cer:
.. _solsta_near_integer_optimal:
.. _solsta_near_optimal:
.. _solsta_near_prim_and_dual_feas:
.. _solsta_near_prim_feas:
.. _solsta_near_prim_infeas_cer:
.. _solsta_optimal:
.. _solsta_prim_and_dual_feas:
.. _solsta_prim_feas:
.. _solsta_prim_illposed_cer:
.. _solsta_prim_infeas_cer:
.. _solsta_unknown:

``solsta``
----------

Solution status keys

``const MSK_SOL_STA_DUAL_FEAS               : i32 = 3``
    The solution is dual feasible.
``const MSK_SOL_STA_DUAL_ILLPOSED_CER       : i32 = 15``
    The solution is a certificate that the dual problem is illposed.
``const MSK_SOL_STA_DUAL_INFEAS_CER         : i32 = 6``
    The solution is a certificate of dual infeasibility.
``const MSK_SOL_STA_INTEGER_OPTIMAL         : i32 = 16``
    The primal solution is integer optimal.
``const MSK_SOL_STA_NEAR_DUAL_FEAS          : i32 = 10``
    The solution is nearly dual feasible.
``const MSK_SOL_STA_NEAR_DUAL_INFEAS_CER    : i32 = 13``
    The solution is almost a certificate of dual infeasibility.
``const MSK_SOL_STA_NEAR_INTEGER_OPTIMAL    : i32 = 17``
    The primal solution is near integer optimal.
``const MSK_SOL_STA_NEAR_OPTIMAL            : i32 = 8``
    The solution is nearly optimal.
``const MSK_SOL_STA_NEAR_PRIM_AND_DUAL_FEAS : i32 = 11``
    The solution is nearly both
                        primal and dual feasible.
``const MSK_SOL_STA_NEAR_PRIM_FEAS          : i32 = 9``
    The solution is nearly primal feasible.
``const MSK_SOL_STA_NEAR_PRIM_INFEAS_CER    : i32 = 12``
    The solution is almost a certificate
                        of primal infeasibility.
``const MSK_SOL_STA_OPTIMAL                 : i32 = 1``
    The solution is optimal.
``const MSK_SOL_STA_PRIM_AND_DUAL_FEAS      : i32 = 4``
    The solution is both primal and dual feasible.
``const MSK_SOL_STA_PRIM_FEAS               : i32 = 2``
    The solution is primal feasible.
``const MSK_SOL_STA_PRIM_ILLPOSED_CER       : i32 = 14``
    The solution is a certificate that the primal problem is illposed.
``const MSK_SOL_STA_PRIM_INFEAS_CER         : i32 = 5``
    The solution is a certificate of primal infeasibility.
``const MSK_SOL_STA_UNKNOWN                 : i32 = 0``
    Status of the solution is unknown.
.. index:: soltype
.. index:: SOL_...
.. _enum_soltype:
.. _soltype_bas:
.. _soltype_itg:
.. _soltype_itr:

``soltype``
-----------

Solution types

``const MSK_SOL_BAS : i32 = 1``
    The basic solution.
``const MSK_SOL_ITG : i32 = 2``
    The integer solution.
``const MSK_SOL_ITR : i32 = 0``
    The interior solution.
.. index:: solveform
.. index:: SOLVE_...
.. _enum_solveform:
.. _solveform_dual:
.. _solveform_free:
.. _solveform_primal:

``solveform``
-------------

Solve primal or dual form

``const MSK_SOLVE_DUAL   : i32 = 2``
    The optimizer should solve the dual problem.
``const MSK_SOLVE_FREE   : i32 = 0``
    The optimizer is free to solve either the primal or
                        the dual problem.
``const MSK_SOLVE_PRIMAL : i32 = 1``
    The optimizer should solve the primal problem.
.. index:: sparam
.. index:: SPAR_...
.. _enum_sparam:
.. _sparam_bas_sol_file_name:
.. _sparam_data_file_name:
.. _sparam_debug_file_name:
.. _sparam_int_sol_file_name:
.. _sparam_itr_sol_file_name:
.. _sparam_mio_debug_string:
.. _sparam_param_comment_sign:
.. _sparam_param_read_file_name:
.. _sparam_param_write_file_name:
.. _sparam_read_mps_bou_name:
.. _sparam_read_mps_obj_name:
.. _sparam_read_mps_ran_name:
.. _sparam_read_mps_rhs_name:
.. _sparam_remote_access_token:
.. _sparam_sensitivity_file_name:
.. _sparam_sensitivity_res_file_name:
.. _sparam_sol_filter_xc_low:
.. _sparam_sol_filter_xc_upr:
.. _sparam_sol_filter_xx_low:
.. _sparam_sol_filter_xx_upr:
.. _sparam_stat_file_name:
.. _sparam_stat_key:
.. _sparam_stat_name:
.. _sparam_write_lp_gen_var_name:

``sparam``
----------

String parameter types

``const MSK_SPAR_BAS_SOL_FILE_NAME         : i32 = 0``
    Name of the bas solution file.
``const MSK_SPAR_DATA_FILE_NAME            : i32 = 1``
    Data are read and written to this file.
``const MSK_SPAR_DEBUG_FILE_NAME           : i32 = 2``
    |mosek| debug file.
``const MSK_SPAR_INT_SOL_FILE_NAME         : i32 = 3``
    Name of the int solution file.
``const MSK_SPAR_ITR_SOL_FILE_NAME         : i32 = 4``
    Name of the itr solution file.
``const MSK_SPAR_MIO_DEBUG_STRING          : i32 = 5``
    For internal use only.
``const MSK_SPAR_PARAM_COMMENT_SIGN        : i32 = 6``
    Solution file comment character.
``const MSK_SPAR_PARAM_READ_FILE_NAME      : i32 = 7``
    Modifications to the parameter
                        database is read from this file.
``const MSK_SPAR_PARAM_WRITE_FILE_NAME     : i32 = 8``
    The parameter database is written to this file.
``const MSK_SPAR_READ_MPS_BOU_NAME         : i32 = 9``
    Name of the BOUNDS vector used.
                        An empty name means that the first BOUNDS vector is used.
``const MSK_SPAR_READ_MPS_OBJ_NAME         : i32 = 10``
    Objective name in the MPS file.
``const MSK_SPAR_READ_MPS_RAN_NAME         : i32 = 11``
    Name of the RANGE vector  used.
                        An empty name means that the first RANGE vector is used.
``const MSK_SPAR_READ_MPS_RHS_NAME         : i32 = 12``
    Name of the RHS used.
                        An empty name means that the first RHS vector is used.
``const MSK_SPAR_REMOTE_ACCESS_TOKEN       : i32 = 13``
    An access token used to submit tasks to a remote |mosek| server
``const MSK_SPAR_SENSITIVITY_FILE_NAME     : i32 = 14``
    Sensitivity report file name.
``const MSK_SPAR_SENSITIVITY_RES_FILE_NAME : i32 = 15``
    Name of the sensitivity report output file.
``const MSK_SPAR_SOL_FILTER_XC_LOW         : i32 = 16``
    Solution file filter.
``const MSK_SPAR_SOL_FILTER_XC_UPR         : i32 = 17``
    Solution file filter.
``const MSK_SPAR_SOL_FILTER_XX_LOW         : i32 = 18``
    Solution file filter.
``const MSK_SPAR_SOL_FILTER_XX_UPR         : i32 = 19``
    Solution file filter.
``const MSK_SPAR_STAT_FILE_NAME            : i32 = 20``
    Statistics file name.
``const MSK_SPAR_STAT_KEY                  : i32 = 21``
    Key used when writing the summary file.
``const MSK_SPAR_STAT_NAME                 : i32 = 22``
    Name used when writing the statistics file.
``const MSK_SPAR_WRITE_LP_GEN_VAR_NAME     : i32 = 23``
    Added variable names in the LP files.
.. index:: stakey
.. index:: SK_...
.. _enum_stakey:
.. _stakey_bas:
.. _stakey_fix:
.. _stakey_inf:
.. _stakey_low:
.. _stakey_supbas:
.. _stakey_unk:
.. _stakey_upr:

``stakey``
----------

Status keys

``const MSK_SK_BAS    : i32 = 1``
    The constraint or variable is in the basis.
``const MSK_SK_FIX    : i32 = 5``
    The constraint or variable is fixed.
``const MSK_SK_INF    : i32 = 6``
    The constraint or variable is infeasible in the bounds.
``const MSK_SK_LOW    : i32 = 3``
    The constraint or variable is at its lower bound.
``const MSK_SK_SUPBAS : i32 = 2``
    The constraint or variable is super basic.
``const MSK_SK_UNK    : i32 = 0``
    The status for the constraint or variable is unknown.
``const MSK_SK_UPR    : i32 = 4``
    The constraint or variable is at its upper bound.
.. index:: startpointtype
.. index:: STARTING_POINT_...
.. _enum_startpointtype:
.. _startpointtype_constant:
.. _startpointtype_free:
.. _startpointtype_guess:
.. _startpointtype_satisfy_bounds:

``startpointtype``
------------------

Starting point types

``const MSK_STARTING_POINT_CONSTANT       : i32 = 2``
    The optimizer constructs a starting point by assigning a constant value to all primal and dual variables.
                        This starting point is normally robust.
``const MSK_STARTING_POINT_FREE           : i32 = 0``
    The starting point is chosen automatically.
``const MSK_STARTING_POINT_GUESS          : i32 = 1``
    The optimizer guesses a starting point.
``const MSK_STARTING_POINT_SATISFY_BOUNDS : i32 = 3``
    The starting point is chosen to satisfy all the simple bounds on nonlinear variables. If this starting point is employed,
                        then more care than usual should employed when choosing the bounds on the nonlinear variables. In particular very tight bounds
                        should be avoided.
.. index:: streamtype
.. index:: STREAM_...
.. _enum_streamtype:
.. _streamtype_err:
.. _streamtype_log:
.. _streamtype_msg:
.. _streamtype_wrn:

``streamtype``
--------------

Stream types

``const MSK_STREAM_ERR : i32 = 2``
    Error stream. Error messages are written to this stream.
``const MSK_STREAM_LOG : i32 = 0``
    Log stream. Contains the aggregated contents of all other streams. This means that a message written to any other stream will also be written to this stream.
``const MSK_STREAM_MSG : i32 = 1``
    Message stream. Log information relating to performance and progress of the optimization is written to this stream.
``const MSK_STREAM_WRN : i32 = 3``
    Warning stream. Warning messages are written to this stream.
.. index:: symmattype
.. index:: SYMMAT_TYPE_...
.. _enum_symmattype:
.. _symmattype_sparse:

``symmattype``
--------------

Cone types

``const MSK_SYMMAT_TYPE_SPARSE : i32 = 0``
    Sparse symmetric matrix.
.. index:: transpose
.. index:: TRANSPOSE_...
.. _enum_transpose:
.. _transpose_no:
.. _transpose_yes:

``transpose``
-------------

Transposed matrix.

``const MSK_TRANSPOSE_NO  : i32 = 0``
    No transpose is applied.
``const MSK_TRANSPOSE_YES : i32 = 1``
    A transpose is applied.
.. index:: uplo
.. index:: UPLO_...
.. _enum_uplo:
.. _uplo_lo:
.. _uplo_up:

``uplo``
--------

Triangular part of a symmetric matrix.

``const MSK_UPLO_LO : i32 = 0``
    Lower part.
``const MSK_UPLO_UP : i32 = 1``
    Upper part
.. index:: value
.. index:: ...
.. _enum_value:
.. _value_license_buffer_length:
.. _value_max_str_len:

``value``
---------

Integer values

``const MSK_LICENSE_BUFFER_LENGTH : i32 = 20``
    The length of a license key buffer.
``const MSK_MAX_STR_LEN           : i32 = 1024``
    Maximum string length allowed in |mosek|.
.. index:: variabletype
.. index:: VAR_...
.. _enum_variabletype:
.. _variabletype_type_cont:
.. _variabletype_type_int:

``variabletype``
----------------

Variable types

``const MSK_VAR_TYPE_CONT : i32 = 0``
    Is a continuous variable.
``const MSK_VAR_TYPE_INT  : i32 = 1``
    Is an integer variable.
.. index:: xmlwriteroutputtype
.. index:: WRITE_XML_MODE_...
.. _enum_xmlwriteroutputtype:
.. _xmlwriteroutputtype_col:
.. _xmlwriteroutputtype_row:

``xmlwriteroutputtype``
-----------------------

XML writer output mode

``const MSK_WRITE_XML_MODE_COL : i32 = 1``
    Write in column order.
``const MSK_WRITE_XML_MODE_ROW : i32 = 0``
    Write in row order.
