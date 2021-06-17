
.. |mosek| replace:: MOSEK
.. |null| replace:: ``None``

Constants
=========

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



``const MSK_BI_ALWAYS      : i32 = 1``
    
``const MSK_BI_IF_FEASIBLE : i32 = 3``
    
``const MSK_BI_NEVER       : i32 = 0``
    
``const MSK_BI_NO_ERROR    : i32 = 2``
    
``const MSK_BI_RESERVERED  : i32 = 4``
    
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



``const MSK_BK_FR : i32 = 3``
    
``const MSK_BK_FX : i32 = 2``
    
``const MSK_BK_LO : i32 = 0``
    
``const MSK_BK_RA : i32 = 4``
    
``const MSK_BK_UP : i32 = 1``
    
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



``const MSK_BRANCH_DIR_DOWN       : i32 = 2``
    
``const MSK_BRANCH_DIR_FAR        : i32 = 4``
    
``const MSK_BRANCH_DIR_FREE       : i32 = 0``
    
``const MSK_BRANCH_DIR_GUIDED     : i32 = 6``
    
``const MSK_BRANCH_DIR_NEAR       : i32 = 3``
    
``const MSK_BRANCH_DIR_PSEUDOCOST : i32 = 7``
    
``const MSK_BRANCH_DIR_ROOT_LP    : i32 = 5``
    
``const MSK_BRANCH_DIR_UP         : i32 = 1``
    
.. index:: callbackcode
.. index:: CALLBACK_...
.. _enum_callbackcode:
.. _callbackcode_begin_bi:
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
.. _callbackcode_begin_optimizer:
.. _callbackcode_begin_presolve:
.. _callbackcode_begin_primal_bi:
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
.. _callbackcode_begin_to_conic:
.. _callbackcode_begin_write:
.. _callbackcode_conic:
.. _callbackcode_dual_simplex:
.. _callbackcode_end_bi:
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
.. _callbackcode_end_optimizer:
.. _callbackcode_end_presolve:
.. _callbackcode_end_primal_bi:
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
.. _callbackcode_im_order:
.. _callbackcode_im_presolve:
.. _callbackcode_im_primal_bi:
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
.. _callbackcode_update_presolve:
.. _callbackcode_update_primal_bi:
.. _callbackcode_update_primal_simplex:
.. _callbackcode_update_primal_simplex_bi:
.. _callbackcode_write_opf:

``callbackcode``
----------------



``const MSK_CALLBACK_BEGIN_BI                   : i32 = 0``
    
``const MSK_CALLBACK_BEGIN_CONIC                : i32 = 1``
    
``const MSK_CALLBACK_BEGIN_DUAL_BI              : i32 = 2``
    
``const MSK_CALLBACK_BEGIN_DUAL_SENSITIVITY     : i32 = 3``
    
``const MSK_CALLBACK_BEGIN_DUAL_SETUP_BI        : i32 = 4``
    
``const MSK_CALLBACK_BEGIN_DUAL_SIMPLEX         : i32 = 5``
    
``const MSK_CALLBACK_BEGIN_DUAL_SIMPLEX_BI      : i32 = 6``
    
``const MSK_CALLBACK_BEGIN_FULL_CONVEXITY_CHECK : i32 = 7``
    
``const MSK_CALLBACK_BEGIN_INFEAS_ANA           : i32 = 8``
    
``const MSK_CALLBACK_BEGIN_INTPNT               : i32 = 9``
    
``const MSK_CALLBACK_BEGIN_LICENSE_WAIT         : i32 = 10``
    
``const MSK_CALLBACK_BEGIN_MIO                  : i32 = 11``
    
``const MSK_CALLBACK_BEGIN_OPTIMIZER            : i32 = 12``
    
``const MSK_CALLBACK_BEGIN_PRESOLVE             : i32 = 13``
    
``const MSK_CALLBACK_BEGIN_PRIMAL_BI            : i32 = 14``
    
``const MSK_CALLBACK_BEGIN_PRIMAL_REPAIR        : i32 = 15``
    
``const MSK_CALLBACK_BEGIN_PRIMAL_SENSITIVITY   : i32 = 16``
    
``const MSK_CALLBACK_BEGIN_PRIMAL_SETUP_BI      : i32 = 17``
    
``const MSK_CALLBACK_BEGIN_PRIMAL_SIMPLEX       : i32 = 18``
    
``const MSK_CALLBACK_BEGIN_PRIMAL_SIMPLEX_BI    : i32 = 19``
    
``const MSK_CALLBACK_BEGIN_QCQO_REFORMULATE     : i32 = 20``
    
``const MSK_CALLBACK_BEGIN_READ                 : i32 = 21``
    
``const MSK_CALLBACK_BEGIN_ROOT_CUTGEN          : i32 = 22``
    
``const MSK_CALLBACK_BEGIN_SIMPLEX              : i32 = 23``
    
``const MSK_CALLBACK_BEGIN_SIMPLEX_BI           : i32 = 24``
    
``const MSK_CALLBACK_BEGIN_TO_CONIC             : i32 = 25``
    
``const MSK_CALLBACK_BEGIN_WRITE                : i32 = 26``
    
``const MSK_CALLBACK_CONIC                      : i32 = 27``
    
``const MSK_CALLBACK_DUAL_SIMPLEX               : i32 = 28``
    
``const MSK_CALLBACK_END_BI                     : i32 = 29``
    
``const MSK_CALLBACK_END_CONIC                  : i32 = 30``
    
``const MSK_CALLBACK_END_DUAL_BI                : i32 = 31``
    
``const MSK_CALLBACK_END_DUAL_SENSITIVITY       : i32 = 32``
    
``const MSK_CALLBACK_END_DUAL_SETUP_BI          : i32 = 33``
    
``const MSK_CALLBACK_END_DUAL_SIMPLEX           : i32 = 34``
    
``const MSK_CALLBACK_END_DUAL_SIMPLEX_BI        : i32 = 35``
    
``const MSK_CALLBACK_END_FULL_CONVEXITY_CHECK   : i32 = 36``
    
``const MSK_CALLBACK_END_INFEAS_ANA             : i32 = 37``
    
``const MSK_CALLBACK_END_INTPNT                 : i32 = 38``
    
``const MSK_CALLBACK_END_LICENSE_WAIT           : i32 = 39``
    
``const MSK_CALLBACK_END_MIO                    : i32 = 40``
    
``const MSK_CALLBACK_END_OPTIMIZER              : i32 = 41``
    
``const MSK_CALLBACK_END_PRESOLVE               : i32 = 42``
    
``const MSK_CALLBACK_END_PRIMAL_BI              : i32 = 43``
    
``const MSK_CALLBACK_END_PRIMAL_REPAIR          : i32 = 44``
    
``const MSK_CALLBACK_END_PRIMAL_SENSITIVITY     : i32 = 45``
    
``const MSK_CALLBACK_END_PRIMAL_SETUP_BI        : i32 = 46``
    
``const MSK_CALLBACK_END_PRIMAL_SIMPLEX         : i32 = 47``
    
``const MSK_CALLBACK_END_PRIMAL_SIMPLEX_BI      : i32 = 48``
    
``const MSK_CALLBACK_END_QCQO_REFORMULATE       : i32 = 49``
    
``const MSK_CALLBACK_END_READ                   : i32 = 50``
    
``const MSK_CALLBACK_END_ROOT_CUTGEN            : i32 = 51``
    
``const MSK_CALLBACK_END_SIMPLEX                : i32 = 52``
    
``const MSK_CALLBACK_END_SIMPLEX_BI             : i32 = 53``
    
``const MSK_CALLBACK_END_TO_CONIC               : i32 = 54``
    
``const MSK_CALLBACK_END_WRITE                  : i32 = 55``
    
``const MSK_CALLBACK_IM_BI                      : i32 = 56``
    
``const MSK_CALLBACK_IM_CONIC                   : i32 = 57``
    
``const MSK_CALLBACK_IM_DUAL_BI                 : i32 = 58``
    
``const MSK_CALLBACK_IM_DUAL_SENSIVITY          : i32 = 59``
    
``const MSK_CALLBACK_IM_DUAL_SIMPLEX            : i32 = 60``
    
``const MSK_CALLBACK_IM_FULL_CONVEXITY_CHECK    : i32 = 61``
    
``const MSK_CALLBACK_IM_INTPNT                  : i32 = 62``
    
``const MSK_CALLBACK_IM_LICENSE_WAIT            : i32 = 63``
    
``const MSK_CALLBACK_IM_LU                      : i32 = 64``
    
``const MSK_CALLBACK_IM_MIO                     : i32 = 65``
    
``const MSK_CALLBACK_IM_MIO_DUAL_SIMPLEX        : i32 = 66``
    
``const MSK_CALLBACK_IM_MIO_INTPNT              : i32 = 67``
    
``const MSK_CALLBACK_IM_MIO_PRIMAL_SIMPLEX      : i32 = 68``
    
``const MSK_CALLBACK_IM_ORDER                   : i32 = 69``
    
``const MSK_CALLBACK_IM_PRESOLVE                : i32 = 70``
    
``const MSK_CALLBACK_IM_PRIMAL_BI               : i32 = 71``
    
``const MSK_CALLBACK_IM_PRIMAL_SENSIVITY        : i32 = 72``
    
``const MSK_CALLBACK_IM_PRIMAL_SIMPLEX          : i32 = 73``
    
``const MSK_CALLBACK_IM_QO_REFORMULATE          : i32 = 74``
    
``const MSK_CALLBACK_IM_READ                    : i32 = 75``
    
``const MSK_CALLBACK_IM_ROOT_CUTGEN             : i32 = 76``
    
``const MSK_CALLBACK_IM_SIMPLEX                 : i32 = 77``
    
``const MSK_CALLBACK_IM_SIMPLEX_BI              : i32 = 78``
    
``const MSK_CALLBACK_INTPNT                     : i32 = 79``
    
``const MSK_CALLBACK_NEW_INT_MIO                : i32 = 80``
    
``const MSK_CALLBACK_PRIMAL_SIMPLEX             : i32 = 81``
    
``const MSK_CALLBACK_READ_OPF                   : i32 = 82``
    
``const MSK_CALLBACK_READ_OPF_SECTION           : i32 = 83``
    
``const MSK_CALLBACK_SOLVING_REMOTE             : i32 = 84``
    
``const MSK_CALLBACK_UPDATE_DUAL_BI             : i32 = 85``
    
``const MSK_CALLBACK_UPDATE_DUAL_SIMPLEX        : i32 = 86``
    
``const MSK_CALLBACK_UPDATE_DUAL_SIMPLEX_BI     : i32 = 87``
    
``const MSK_CALLBACK_UPDATE_PRESOLVE            : i32 = 88``
    
``const MSK_CALLBACK_UPDATE_PRIMAL_BI           : i32 = 89``
    
``const MSK_CALLBACK_UPDATE_PRIMAL_SIMPLEX      : i32 = 90``
    
``const MSK_CALLBACK_UPDATE_PRIMAL_SIMPLEX_BI   : i32 = 91``
    
``const MSK_CALLBACK_WRITE_OPF                  : i32 = 92``
    
.. index:: checkconvexitytype
.. index:: CHECK_CONVEXITY_...
.. _enum_checkconvexitytype:
.. _checkconvexitytype_full:
.. _checkconvexitytype_none:
.. _checkconvexitytype_simple:

``checkconvexitytype``
----------------------



``const MSK_CHECK_CONVEXITY_FULL   : i32 = 2``
    
``const MSK_CHECK_CONVEXITY_NONE   : i32 = 0``
    
``const MSK_CHECK_CONVEXITY_SIMPLE : i32 = 1``
    
.. index:: compresstype
.. index:: COMPRESS_...
.. _enum_compresstype:
.. _compresstype_free:
.. _compresstype_gzip:
.. _compresstype_none:
.. _compresstype_zstd:

``compresstype``
----------------



``const MSK_COMPRESS_FREE : i32 = 1``
    
``const MSK_COMPRESS_GZIP : i32 = 2``
    
``const MSK_COMPRESS_NONE : i32 = 0``
    
``const MSK_COMPRESS_ZSTD : i32 = 3``
    
.. index:: conetype
.. index:: CT_...
.. _enum_conetype:
.. _conetype_dexp:
.. _conetype_dpow:
.. _conetype_pexp:
.. _conetype_ppow:
.. _conetype_quad:
.. _conetype_rquad:
.. _conetype_zero:

``conetype``
------------



``const MSK_CT_DEXP  : i32 = 3``
    
``const MSK_CT_DPOW  : i32 = 5``
    
``const MSK_CT_PEXP  : i32 = 2``
    
``const MSK_CT_PPOW  : i32 = 4``
    
``const MSK_CT_QUAD  : i32 = 0``
    
``const MSK_CT_RQUAD : i32 = 1``
    
``const MSK_CT_ZERO  : i32 = 6``
    
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
.. _dataformat_ptf:
.. _dataformat_task:

``dataformat``
--------------



``const MSK_DATA_FORMAT_CB        : i32 = 7``
    
``const MSK_DATA_FORMAT_EXTENSION : i32 = 0``
    
``const MSK_DATA_FORMAT_FREE_MPS  : i32 = 4``
    
``const MSK_DATA_FORMAT_JSON_TASK : i32 = 8``
    
``const MSK_DATA_FORMAT_LP        : i32 = 2``
    
``const MSK_DATA_FORMAT_MPS       : i32 = 1``
    
``const MSK_DATA_FORMAT_OP        : i32 = 3``
    
``const MSK_DATA_FORMAT_PTF       : i32 = 6``
    
``const MSK_DATA_FORMAT_TASK      : i32 = 5``
    
.. index:: dinfitem
.. index:: DINF_...
.. _enum_dinfitem:
.. _dinfitem_bi_clean_dual_time:
.. _dinfitem_bi_clean_primal_time:
.. _dinfitem_bi_clean_time:
.. _dinfitem_bi_dual_time:
.. _dinfitem_bi_primal_time:
.. _dinfitem_bi_time:
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
.. _dinfitem_mio_implied_bound_time:
.. _dinfitem_mio_knapsack_cover_separation_time:
.. _dinfitem_mio_obj_abs_gap:
.. _dinfitem_mio_obj_bound:
.. _dinfitem_mio_obj_int:
.. _dinfitem_mio_obj_rel_gap:
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
.. _dinfitem_sim_obj:
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



``const MSK_DINF_BI_CLEAN_DUAL_TIME                             : i32 = 0``
    
``const MSK_DINF_BI_CLEAN_PRIMAL_TIME                           : i32 = 1``
    
``const MSK_DINF_BI_CLEAN_TIME                                  : i32 = 2``
    
``const MSK_DINF_BI_DUAL_TIME                                   : i32 = 3``
    
``const MSK_DINF_BI_PRIMAL_TIME                                 : i32 = 4``
    
``const MSK_DINF_BI_TIME                                        : i32 = 5``
    
``const MSK_DINF_INTPNT_DUAL_FEAS                               : i32 = 6``
    
``const MSK_DINF_INTPNT_DUAL_OBJ                                : i32 = 7``
    
``const MSK_DINF_INTPNT_FACTOR_NUM_FLOPS                        : i32 = 8``
    
``const MSK_DINF_INTPNT_OPT_STATUS                              : i32 = 9``
    
``const MSK_DINF_INTPNT_ORDER_TIME                              : i32 = 10``
    
``const MSK_DINF_INTPNT_PRIMAL_FEAS                             : i32 = 11``
    
``const MSK_DINF_INTPNT_PRIMAL_OBJ                              : i32 = 12``
    
``const MSK_DINF_INTPNT_TIME                                    : i32 = 13``
    
``const MSK_DINF_MIO_CLIQUE_SEPARATION_TIME                     : i32 = 14``
    
``const MSK_DINF_MIO_CMIR_SEPARATION_TIME                       : i32 = 15``
    
``const MSK_DINF_MIO_CONSTRUCT_SOLUTION_OBJ                     : i32 = 16``
    
``const MSK_DINF_MIO_DUAL_BOUND_AFTER_PRESOLVE                  : i32 = 17``
    
``const MSK_DINF_MIO_GMI_SEPARATION_TIME                        : i32 = 18``
    
``const MSK_DINF_MIO_IMPLIED_BOUND_TIME                         : i32 = 19``
    
``const MSK_DINF_MIO_KNAPSACK_COVER_SEPARATION_TIME             : i32 = 20``
    
``const MSK_DINF_MIO_OBJ_ABS_GAP                                : i32 = 21``
    
``const MSK_DINF_MIO_OBJ_BOUND                                  : i32 = 22``
    
``const MSK_DINF_MIO_OBJ_INT                                    : i32 = 23``
    
``const MSK_DINF_MIO_OBJ_REL_GAP                                : i32 = 24``
    
``const MSK_DINF_MIO_PROBING_TIME                               : i32 = 25``
    
``const MSK_DINF_MIO_ROOT_CUTGEN_TIME                           : i32 = 26``
    
``const MSK_DINF_MIO_ROOT_OPTIMIZER_TIME                        : i32 = 27``
    
``const MSK_DINF_MIO_ROOT_PRESOLVE_TIME                         : i32 = 28``
    
``const MSK_DINF_MIO_TIME                                       : i32 = 29``
    
``const MSK_DINF_MIO_USER_OBJ_CUT                               : i32 = 30``
    
``const MSK_DINF_OPTIMIZER_TIME                                 : i32 = 31``
    
``const MSK_DINF_PRESOLVE_ELI_TIME                              : i32 = 32``
    
``const MSK_DINF_PRESOLVE_LINDEP_TIME                           : i32 = 33``
    
``const MSK_DINF_PRESOLVE_TIME                                  : i32 = 34``
    
``const MSK_DINF_PRIMAL_REPAIR_PENALTY_OBJ                      : i32 = 35``
    
``const MSK_DINF_QCQO_REFORMULATE_MAX_PERTURBATION              : i32 = 36``
    
``const MSK_DINF_QCQO_REFORMULATE_TIME                          : i32 = 37``
    
``const MSK_DINF_QCQO_REFORMULATE_WORST_CHOLESKY_COLUMN_SCALING : i32 = 38``
    
``const MSK_DINF_QCQO_REFORMULATE_WORST_CHOLESKY_DIAG_SCALING   : i32 = 39``
    
``const MSK_DINF_RD_TIME                                        : i32 = 40``
    
``const MSK_DINF_SIM_DUAL_TIME                                  : i32 = 41``
    
``const MSK_DINF_SIM_FEAS                                       : i32 = 42``
    
``const MSK_DINF_SIM_OBJ                                        : i32 = 43``
    
``const MSK_DINF_SIM_PRIMAL_TIME                                : i32 = 44``
    
``const MSK_DINF_SIM_TIME                                       : i32 = 45``
    
``const MSK_DINF_SOL_BAS_DUAL_OBJ                               : i32 = 46``
    
``const MSK_DINF_SOL_BAS_DVIOLCON                               : i32 = 47``
    
``const MSK_DINF_SOL_BAS_DVIOLVAR                               : i32 = 48``
    
``const MSK_DINF_SOL_BAS_NRM_BARX                               : i32 = 49``
    
``const MSK_DINF_SOL_BAS_NRM_SLC                                : i32 = 50``
    
``const MSK_DINF_SOL_BAS_NRM_SLX                                : i32 = 51``
    
``const MSK_DINF_SOL_BAS_NRM_SUC                                : i32 = 52``
    
``const MSK_DINF_SOL_BAS_NRM_SUX                                : i32 = 53``
    
``const MSK_DINF_SOL_BAS_NRM_XC                                 : i32 = 54``
    
``const MSK_DINF_SOL_BAS_NRM_XX                                 : i32 = 55``
    
``const MSK_DINF_SOL_BAS_NRM_Y                                  : i32 = 56``
    
``const MSK_DINF_SOL_BAS_PRIMAL_OBJ                             : i32 = 57``
    
``const MSK_DINF_SOL_BAS_PVIOLCON                               : i32 = 58``
    
``const MSK_DINF_SOL_BAS_PVIOLVAR                               : i32 = 59``
    
``const MSK_DINF_SOL_ITG_NRM_BARX                               : i32 = 60``
    
``const MSK_DINF_SOL_ITG_NRM_XC                                 : i32 = 61``
    
``const MSK_DINF_SOL_ITG_NRM_XX                                 : i32 = 62``
    
``const MSK_DINF_SOL_ITG_PRIMAL_OBJ                             : i32 = 63``
    
``const MSK_DINF_SOL_ITG_PVIOLBARVAR                            : i32 = 64``
    
``const MSK_DINF_SOL_ITG_PVIOLCON                               : i32 = 65``
    
``const MSK_DINF_SOL_ITG_PVIOLCONES                             : i32 = 66``
    
``const MSK_DINF_SOL_ITG_PVIOLITG                               : i32 = 67``
    
``const MSK_DINF_SOL_ITG_PVIOLVAR                               : i32 = 68``
    
``const MSK_DINF_SOL_ITR_DUAL_OBJ                               : i32 = 69``
    
``const MSK_DINF_SOL_ITR_DVIOLBARVAR                            : i32 = 70``
    
``const MSK_DINF_SOL_ITR_DVIOLCON                               : i32 = 71``
    
``const MSK_DINF_SOL_ITR_DVIOLCONES                             : i32 = 72``
    
``const MSK_DINF_SOL_ITR_DVIOLVAR                               : i32 = 73``
    
``const MSK_DINF_SOL_ITR_NRM_BARS                               : i32 = 74``
    
``const MSK_DINF_SOL_ITR_NRM_BARX                               : i32 = 75``
    
``const MSK_DINF_SOL_ITR_NRM_SLC                                : i32 = 76``
    
``const MSK_DINF_SOL_ITR_NRM_SLX                                : i32 = 77``
    
``const MSK_DINF_SOL_ITR_NRM_SNX                                : i32 = 78``
    
``const MSK_DINF_SOL_ITR_NRM_SUC                                : i32 = 79``
    
``const MSK_DINF_SOL_ITR_NRM_SUX                                : i32 = 80``
    
``const MSK_DINF_SOL_ITR_NRM_XC                                 : i32 = 81``
    
``const MSK_DINF_SOL_ITR_NRM_XX                                 : i32 = 82``
    
``const MSK_DINF_SOL_ITR_NRM_Y                                  : i32 = 83``
    
``const MSK_DINF_SOL_ITR_PRIMAL_OBJ                             : i32 = 84``
    
``const MSK_DINF_SOL_ITR_PVIOLBARVAR                            : i32 = 85``
    
``const MSK_DINF_SOL_ITR_PVIOLCON                               : i32 = 86``
    
``const MSK_DINF_SOL_ITR_PVIOLCONES                             : i32 = 87``
    
``const MSK_DINF_SOL_ITR_PVIOLVAR                               : i32 = 88``
    
``const MSK_DINF_TO_CONIC_TIME                                  : i32 = 89``
    
.. index:: dparam
.. index:: DPAR_...
.. _enum_dparam:
.. _dparam_ana_sol_infeas_tol:
.. _dparam_basis_rel_tol_s:
.. _dparam_basis_tol_s:
.. _dparam_basis_tol_x:
.. _dparam_check_convexity_rel_tol:
.. _dparam_data_sym_mat_tol:
.. _dparam_data_sym_mat_tol_huge:
.. _dparam_data_sym_mat_tol_large:
.. _dparam_data_tol_aij_huge:
.. _dparam_data_tol_aij_large:
.. _dparam_data_tol_bound_inf:
.. _dparam_data_tol_bound_wrn:
.. _dparam_data_tol_c_huge:
.. _dparam_data_tol_cj_large:
.. _dparam_data_tol_qij:
.. _dparam_data_tol_x:
.. _dparam_intpnt_co_tol_dfeas:
.. _dparam_intpnt_co_tol_infeas:
.. _dparam_intpnt_co_tol_mu_red:
.. _dparam_intpnt_co_tol_near_rel:
.. _dparam_intpnt_co_tol_pfeas:
.. _dparam_intpnt_co_tol_rel_gap:
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
.. _dparam_mio_max_time:
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



``const MSK_DPAR_ANA_SOL_INFEAS_TOL                 : i32 = 0``
    
``const MSK_DPAR_BASIS_REL_TOL_S                    : i32 = 1``
    
``const MSK_DPAR_BASIS_TOL_S                        : i32 = 2``
    
``const MSK_DPAR_BASIS_TOL_X                        : i32 = 3``
    
``const MSK_DPAR_CHECK_CONVEXITY_REL_TOL            : i32 = 4``
    
``const MSK_DPAR_DATA_SYM_MAT_TOL                   : i32 = 5``
    
``const MSK_DPAR_DATA_SYM_MAT_TOL_HUGE              : i32 = 6``
    
``const MSK_DPAR_DATA_SYM_MAT_TOL_LARGE             : i32 = 7``
    
``const MSK_DPAR_DATA_TOL_AIJ_HUGE                  : i32 = 8``
    
``const MSK_DPAR_DATA_TOL_AIJ_LARGE                 : i32 = 9``
    
``const MSK_DPAR_DATA_TOL_BOUND_INF                 : i32 = 10``
    
``const MSK_DPAR_DATA_TOL_BOUND_WRN                 : i32 = 11``
    
``const MSK_DPAR_DATA_TOL_C_HUGE                    : i32 = 12``
    
``const MSK_DPAR_DATA_TOL_CJ_LARGE                  : i32 = 13``
    
``const MSK_DPAR_DATA_TOL_QIJ                       : i32 = 14``
    
``const MSK_DPAR_DATA_TOL_X                         : i32 = 15``
    
``const MSK_DPAR_INTPNT_CO_TOL_DFEAS                : i32 = 16``
    
``const MSK_DPAR_INTPNT_CO_TOL_INFEAS               : i32 = 17``
    
``const MSK_DPAR_INTPNT_CO_TOL_MU_RED               : i32 = 18``
    
``const MSK_DPAR_INTPNT_CO_TOL_NEAR_REL             : i32 = 19``
    
``const MSK_DPAR_INTPNT_CO_TOL_PFEAS                : i32 = 20``
    
``const MSK_DPAR_INTPNT_CO_TOL_REL_GAP              : i32 = 21``
    
``const MSK_DPAR_INTPNT_QO_TOL_DFEAS                : i32 = 22``
    
``const MSK_DPAR_INTPNT_QO_TOL_INFEAS               : i32 = 23``
    
``const MSK_DPAR_INTPNT_QO_TOL_MU_RED               : i32 = 24``
    
``const MSK_DPAR_INTPNT_QO_TOL_NEAR_REL             : i32 = 25``
    
``const MSK_DPAR_INTPNT_QO_TOL_PFEAS                : i32 = 26``
    
``const MSK_DPAR_INTPNT_QO_TOL_REL_GAP              : i32 = 27``
    
``const MSK_DPAR_INTPNT_TOL_DFEAS                   : i32 = 28``
    
``const MSK_DPAR_INTPNT_TOL_DSAFE                   : i32 = 29``
    
``const MSK_DPAR_INTPNT_TOL_INFEAS                  : i32 = 30``
    
``const MSK_DPAR_INTPNT_TOL_MU_RED                  : i32 = 31``
    
``const MSK_DPAR_INTPNT_TOL_PATH                    : i32 = 32``
    
``const MSK_DPAR_INTPNT_TOL_PFEAS                   : i32 = 33``
    
``const MSK_DPAR_INTPNT_TOL_PSAFE                   : i32 = 34``
    
``const MSK_DPAR_INTPNT_TOL_REL_GAP                 : i32 = 35``
    
``const MSK_DPAR_INTPNT_TOL_REL_STEP                : i32 = 36``
    
``const MSK_DPAR_INTPNT_TOL_STEP_SIZE               : i32 = 37``
    
``const MSK_DPAR_LOWER_OBJ_CUT                      : i32 = 38``
    
``const MSK_DPAR_LOWER_OBJ_CUT_FINITE_TRH           : i32 = 39``
    
``const MSK_DPAR_MIO_MAX_TIME                       : i32 = 40``
    
``const MSK_DPAR_MIO_REL_GAP_CONST                  : i32 = 41``
    
``const MSK_DPAR_MIO_TOL_ABS_GAP                    : i32 = 42``
    
``const MSK_DPAR_MIO_TOL_ABS_RELAX_INT              : i32 = 43``
    
``const MSK_DPAR_MIO_TOL_FEAS                       : i32 = 44``
    
``const MSK_DPAR_MIO_TOL_REL_DUAL_BOUND_IMPROVEMENT : i32 = 45``
    
``const MSK_DPAR_MIO_TOL_REL_GAP                    : i32 = 46``
    
``const MSK_DPAR_OPTIMIZER_MAX_TIME                 : i32 = 47``
    
``const MSK_DPAR_PRESOLVE_TOL_ABS_LINDEP            : i32 = 48``
    
``const MSK_DPAR_PRESOLVE_TOL_AIJ                   : i32 = 49``
    
``const MSK_DPAR_PRESOLVE_TOL_REL_LINDEP            : i32 = 50``
    
``const MSK_DPAR_PRESOLVE_TOL_S                     : i32 = 51``
    
``const MSK_DPAR_PRESOLVE_TOL_X                     : i32 = 52``
    
``const MSK_DPAR_QCQO_REFORMULATE_REL_DROP_TOL      : i32 = 53``
    
``const MSK_DPAR_SEMIDEFINITE_TOL_APPROX            : i32 = 54``
    
``const MSK_DPAR_SIM_LU_TOL_REL_PIV                 : i32 = 55``
    
``const MSK_DPAR_SIMPLEX_ABS_TOL_PIV                : i32 = 56``
    
``const MSK_DPAR_UPPER_OBJ_CUT                      : i32 = 57``
    
``const MSK_DPAR_UPPER_OBJ_CUT_FINITE_TRH           : i32 = 58``
    
.. index:: feature
.. index:: FEATURE_...
.. _enum_feature:
.. _feature_pton:
.. _feature_pts:

``feature``
-----------



``const MSK_FEATURE_PTON : i32 = 1``
    
``const MSK_FEATURE_PTS  : i32 = 0``
    
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
.. _iinfitem_intpnt_factor_dim_dense:
.. _iinfitem_intpnt_iter:
.. _iinfitem_intpnt_num_threads:
.. _iinfitem_intpnt_solve_dual:
.. _iinfitem_mio_absgap_satisfied:
.. _iinfitem_mio_clique_table_size:
.. _iinfitem_mio_construct_solution:
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
.. _iinfitem_mio_numbin:
.. _iinfitem_mio_numbinconevar:
.. _iinfitem_mio_numcon:
.. _iinfitem_mio_numcone:
.. _iinfitem_mio_numconevar:
.. _iinfitem_mio_numcont:
.. _iinfitem_mio_numcontconevar:
.. _iinfitem_mio_numdexpcones:
.. _iinfitem_mio_numdpowcones:
.. _iinfitem_mio_numint:
.. _iinfitem_mio_numintconevar:
.. _iinfitem_mio_numpexpcones:
.. _iinfitem_mio_numppowcones:
.. _iinfitem_mio_numqcones:
.. _iinfitem_mio_numrqcones:
.. _iinfitem_mio_numvar:
.. _iinfitem_mio_obj_bound_defined:
.. _iinfitem_mio_presolved_numbin:
.. _iinfitem_mio_presolved_numbinconevar:
.. _iinfitem_mio_presolved_numcon:
.. _iinfitem_mio_presolved_numcone:
.. _iinfitem_mio_presolved_numconevar:
.. _iinfitem_mio_presolved_numcont:
.. _iinfitem_mio_presolved_numcontconevar:
.. _iinfitem_mio_presolved_numdexpcones:
.. _iinfitem_mio_presolved_numdpowcones:
.. _iinfitem_mio_presolved_numint:
.. _iinfitem_mio_presolved_numintconevar:
.. _iinfitem_mio_presolved_numpexpcones:
.. _iinfitem_mio_presolved_numppowcones:
.. _iinfitem_mio_presolved_numqcones:
.. _iinfitem_mio_presolved_numrqcones:
.. _iinfitem_mio_presolved_numvar:
.. _iinfitem_mio_relgap_satisfied:
.. _iinfitem_mio_total_num_cuts:
.. _iinfitem_mio_user_obj_cut:
.. _iinfitem_opt_numcon:
.. _iinfitem_opt_numvar:
.. _iinfitem_optimize_response:
.. _iinfitem_purify_dual_success:
.. _iinfitem_purify_primal_success:
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
.. _iinfitem_sim_numcon:
.. _iinfitem_sim_numvar:
.. _iinfitem_sim_primal_deg_iter:
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



``const MSK_IINF_ANA_PRO_NUM_CON              : i32 = 0``
    
``const MSK_IINF_ANA_PRO_NUM_CON_EQ           : i32 = 1``
    
``const MSK_IINF_ANA_PRO_NUM_CON_FR           : i32 = 2``
    
``const MSK_IINF_ANA_PRO_NUM_CON_LO           : i32 = 3``
    
``const MSK_IINF_ANA_PRO_NUM_CON_RA           : i32 = 4``
    
``const MSK_IINF_ANA_PRO_NUM_CON_UP           : i32 = 5``
    
``const MSK_IINF_ANA_PRO_NUM_VAR              : i32 = 6``
    
``const MSK_IINF_ANA_PRO_NUM_VAR_BIN          : i32 = 7``
    
``const MSK_IINF_ANA_PRO_NUM_VAR_CONT         : i32 = 8``
    
``const MSK_IINF_ANA_PRO_NUM_VAR_EQ           : i32 = 9``
    
``const MSK_IINF_ANA_PRO_NUM_VAR_FR           : i32 = 10``
    
``const MSK_IINF_ANA_PRO_NUM_VAR_INT          : i32 = 11``
    
``const MSK_IINF_ANA_PRO_NUM_VAR_LO           : i32 = 12``
    
``const MSK_IINF_ANA_PRO_NUM_VAR_RA           : i32 = 13``
    
``const MSK_IINF_ANA_PRO_NUM_VAR_UP           : i32 = 14``
    
``const MSK_IINF_INTPNT_FACTOR_DIM_DENSE      : i32 = 15``
    
``const MSK_IINF_INTPNT_ITER                  : i32 = 16``
    
``const MSK_IINF_INTPNT_NUM_THREADS           : i32 = 17``
    
``const MSK_IINF_INTPNT_SOLVE_DUAL            : i32 = 18``
    
``const MSK_IINF_MIO_ABSGAP_SATISFIED         : i32 = 19``
    
``const MSK_IINF_MIO_CLIQUE_TABLE_SIZE        : i32 = 20``
    
``const MSK_IINF_MIO_CONSTRUCT_SOLUTION       : i32 = 21``
    
``const MSK_IINF_MIO_NODE_DEPTH               : i32 = 22``
    
``const MSK_IINF_MIO_NUM_ACTIVE_NODES         : i32 = 23``
    
``const MSK_IINF_MIO_NUM_BRANCH               : i32 = 24``
    
``const MSK_IINF_MIO_NUM_CLIQUE_CUTS          : i32 = 25``
    
``const MSK_IINF_MIO_NUM_CMIR_CUTS            : i32 = 26``
    
``const MSK_IINF_MIO_NUM_GOMORY_CUTS          : i32 = 27``
    
``const MSK_IINF_MIO_NUM_IMPLIED_BOUND_CUTS   : i32 = 28``
    
``const MSK_IINF_MIO_NUM_INT_SOLUTIONS        : i32 = 29``
    
``const MSK_IINF_MIO_NUM_KNAPSACK_COVER_CUTS  : i32 = 30``
    
``const MSK_IINF_MIO_NUM_RELAX                : i32 = 31``
    
``const MSK_IINF_MIO_NUM_REPEATED_PRESOLVE    : i32 = 32``
    
``const MSK_IINF_MIO_NUMBIN                   : i32 = 33``
    
``const MSK_IINF_MIO_NUMBINCONEVAR            : i32 = 34``
    
``const MSK_IINF_MIO_NUMCON                   : i32 = 35``
    
``const MSK_IINF_MIO_NUMCONE                  : i32 = 36``
    
``const MSK_IINF_MIO_NUMCONEVAR               : i32 = 37``
    
``const MSK_IINF_MIO_NUMCONT                  : i32 = 38``
    
``const MSK_IINF_MIO_NUMCONTCONEVAR           : i32 = 39``
    
``const MSK_IINF_MIO_NUMDEXPCONES             : i32 = 40``
    
``const MSK_IINF_MIO_NUMDPOWCONES             : i32 = 41``
    
``const MSK_IINF_MIO_NUMINT                   : i32 = 42``
    
``const MSK_IINF_MIO_NUMINTCONEVAR            : i32 = 43``
    
``const MSK_IINF_MIO_NUMPEXPCONES             : i32 = 44``
    
``const MSK_IINF_MIO_NUMPPOWCONES             : i32 = 45``
    
``const MSK_IINF_MIO_NUMQCONES                : i32 = 46``
    
``const MSK_IINF_MIO_NUMRQCONES               : i32 = 47``
    
``const MSK_IINF_MIO_NUMVAR                   : i32 = 48``
    
``const MSK_IINF_MIO_OBJ_BOUND_DEFINED        : i32 = 49``
    
``const MSK_IINF_MIO_PRESOLVED_NUMBIN         : i32 = 50``
    
``const MSK_IINF_MIO_PRESOLVED_NUMBINCONEVAR  : i32 = 51``
    
``const MSK_IINF_MIO_PRESOLVED_NUMCON         : i32 = 52``
    
``const MSK_IINF_MIO_PRESOLVED_NUMCONE        : i32 = 53``
    
``const MSK_IINF_MIO_PRESOLVED_NUMCONEVAR     : i32 = 54``
    
``const MSK_IINF_MIO_PRESOLVED_NUMCONT        : i32 = 55``
    
``const MSK_IINF_MIO_PRESOLVED_NUMCONTCONEVAR : i32 = 56``
    
``const MSK_IINF_MIO_PRESOLVED_NUMDEXPCONES   : i32 = 57``
    
``const MSK_IINF_MIO_PRESOLVED_NUMDPOWCONES   : i32 = 58``
    
``const MSK_IINF_MIO_PRESOLVED_NUMINT         : i32 = 59``
    
``const MSK_IINF_MIO_PRESOLVED_NUMINTCONEVAR  : i32 = 60``
    
``const MSK_IINF_MIO_PRESOLVED_NUMPEXPCONES   : i32 = 61``
    
``const MSK_IINF_MIO_PRESOLVED_NUMPPOWCONES   : i32 = 62``
    
``const MSK_IINF_MIO_PRESOLVED_NUMQCONES      : i32 = 63``
    
``const MSK_IINF_MIO_PRESOLVED_NUMRQCONES     : i32 = 64``
    
``const MSK_IINF_MIO_PRESOLVED_NUMVAR         : i32 = 65``
    
``const MSK_IINF_MIO_RELGAP_SATISFIED         : i32 = 66``
    
``const MSK_IINF_MIO_TOTAL_NUM_CUTS           : i32 = 67``
    
``const MSK_IINF_MIO_USER_OBJ_CUT             : i32 = 68``
    
``const MSK_IINF_OPT_NUMCON                   : i32 = 69``
    
``const MSK_IINF_OPT_NUMVAR                   : i32 = 70``
    
``const MSK_IINF_OPTIMIZE_RESPONSE            : i32 = 71``
    
``const MSK_IINF_PURIFY_DUAL_SUCCESS          : i32 = 72``
    
``const MSK_IINF_PURIFY_PRIMAL_SUCCESS        : i32 = 73``
    
``const MSK_IINF_RD_NUMBARVAR                 : i32 = 74``
    
``const MSK_IINF_RD_NUMCON                    : i32 = 75``
    
``const MSK_IINF_RD_NUMCONE                   : i32 = 76``
    
``const MSK_IINF_RD_NUMINTVAR                 : i32 = 77``
    
``const MSK_IINF_RD_NUMQ                      : i32 = 78``
    
``const MSK_IINF_RD_NUMVAR                    : i32 = 79``
    
``const MSK_IINF_RD_PROTYPE                   : i32 = 80``
    
``const MSK_IINF_SIM_DUAL_DEG_ITER            : i32 = 81``
    
``const MSK_IINF_SIM_DUAL_HOTSTART            : i32 = 82``
    
``const MSK_IINF_SIM_DUAL_HOTSTART_LU         : i32 = 83``
    
``const MSK_IINF_SIM_DUAL_INF_ITER            : i32 = 84``
    
``const MSK_IINF_SIM_DUAL_ITER                : i32 = 85``
    
``const MSK_IINF_SIM_NUMCON                   : i32 = 86``
    
``const MSK_IINF_SIM_NUMVAR                   : i32 = 87``
    
``const MSK_IINF_SIM_PRIMAL_DEG_ITER          : i32 = 88``
    
``const MSK_IINF_SIM_PRIMAL_HOTSTART          : i32 = 89``
    
``const MSK_IINF_SIM_PRIMAL_HOTSTART_LU       : i32 = 90``
    
``const MSK_IINF_SIM_PRIMAL_INF_ITER          : i32 = 91``
    
``const MSK_IINF_SIM_PRIMAL_ITER              : i32 = 92``
    
``const MSK_IINF_SIM_SOLVE_DUAL               : i32 = 93``
    
``const MSK_IINF_SOL_BAS_PROSTA               : i32 = 94``
    
``const MSK_IINF_SOL_BAS_SOLSTA               : i32 = 95``
    
``const MSK_IINF_SOL_ITG_PROSTA               : i32 = 96``
    
``const MSK_IINF_SOL_ITG_SOLSTA               : i32 = 97``
    
``const MSK_IINF_SOL_ITR_PROSTA               : i32 = 98``
    
``const MSK_IINF_SOL_ITR_SOLSTA               : i32 = 99``
    
``const MSK_IINF_STO_NUM_A_REALLOC            : i32 = 100``
    
.. index:: inftype
.. index:: INF_...
.. _enum_inftype:
.. _inftype_dou_type:
.. _inftype_int_type:
.. _inftype_lint_type:

``inftype``
-----------



``const MSK_INF_DOU_TYPE  : i32 = 0``
    
``const MSK_INF_INT_TYPE  : i32 = 1``
    
``const MSK_INF_LINT_TYPE : i32 = 2``
    
.. index:: intpnthotstart
.. index:: INTPNT_HOTSTART_...
.. _enum_intpnthotstart:
.. _intpnthotstart_dual:
.. _intpnthotstart_none:
.. _intpnthotstart_primal:
.. _intpnthotstart_primal_dual:

``intpnthotstart``
------------------



``const MSK_INTPNT_HOTSTART_DUAL        : i32 = 2``
    
``const MSK_INTPNT_HOTSTART_NONE        : i32 = 0``
    
``const MSK_INTPNT_HOTSTART_PRIMAL      : i32 = 1``
    
``const MSK_INTPNT_HOTSTART_PRIMAL_DUAL : i32 = 3``
    
.. index:: iomode
.. index:: IOMODE_...
.. _enum_iomode:
.. _iomode_read:
.. _iomode_readwrite:
.. _iomode_write:

``iomode``
----------



``const MSK_IOMODE_READ      : i32 = 0``
    
``const MSK_IOMODE_READWRITE : i32 = 2``
    
``const MSK_IOMODE_WRITE     : i32 = 1``
    
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
.. _iparam_intpnt_order_gp_num_seeds:
.. _iparam_intpnt_order_method:
.. _iparam_intpnt_purify:
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
.. _iparam_log_cut_second_opt:
.. _iparam_log_expand:
.. _iparam_log_feas_repair:
.. _iparam_log_file:
.. _iparam_log_include_summary:
.. _iparam_log_infeas_ana:
.. _iparam_log_intpnt:
.. _iparam_log_local_info:
.. _iparam_log_mio:
.. _iparam_log_mio_freq:
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
.. _iparam_mio_conic_outer_approximation:
.. _iparam_mio_cut_clique:
.. _iparam_mio_cut_cmir:
.. _iparam_mio_cut_gmi:
.. _iparam_mio_cut_implied_bound:
.. _iparam_mio_cut_knapsack_cover:
.. _iparam_mio_cut_selection_level:
.. _iparam_mio_feaspump_level:
.. _iparam_mio_heuristic_level:
.. _iparam_mio_max_num_branches:
.. _iparam_mio_max_num_relaxs:
.. _iparam_mio_max_num_root_cut_rounds:
.. _iparam_mio_max_num_solutions:
.. _iparam_mio_mode:
.. _iparam_mio_node_optimizer:
.. _iparam_mio_node_selection:
.. _iparam_mio_perspective_reformulate:
.. _iparam_mio_probing_level:
.. _iparam_mio_propagate_objective_constraint:
.. _iparam_mio_rins_max_nodes:
.. _iparam_mio_root_optimizer:
.. _iparam_mio_root_repeat_presolve_level:
.. _iparam_mio_seed:
.. _iparam_mio_vb_detection_level:
.. _iparam_mt_spincount:
.. _iparam_num_threads:
.. _iparam_opf_write_header:
.. _iparam_opf_write_hints:
.. _iparam_opf_write_line_length:
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
.. _iparam_presolve_max_num_pass:
.. _iparam_presolve_max_num_reductions:
.. _iparam_presolve_use:
.. _iparam_primal_repair_optimizer:
.. _iparam_ptf_write_transform:
.. _iparam_read_debug:
.. _iparam_read_keep_free_con:
.. _iparam_read_lp_drop_new_vars_in_bou:
.. _iparam_read_lp_quoted_names:
.. _iparam_read_mps_format:
.. _iparam_read_mps_width:
.. _iparam_read_task_ignore_param:
.. _iparam_remove_unused_solutions:
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
.. _iparam_sim_seed:
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
.. _iparam_write_compression:
.. _iparam_write_data_param:
.. _iparam_write_free_con:
.. _iparam_write_generic_names:
.. _iparam_write_generic_names_io:
.. _iparam_write_ignore_incompatible_items:
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



``const MSK_IPAR_ANA_SOL_BASIS                      : i32 = 0``
    
``const MSK_IPAR_ANA_SOL_PRINT_VIOLATED             : i32 = 1``
    
``const MSK_IPAR_AUTO_SORT_A_BEFORE_OPT             : i32 = 2``
    
``const MSK_IPAR_AUTO_UPDATE_SOL_INFO               : i32 = 3``
    
``const MSK_IPAR_BASIS_SOLVE_USE_PLUS_ONE           : i32 = 4``
    
``const MSK_IPAR_BI_CLEAN_OPTIMIZER                 : i32 = 5``
    
``const MSK_IPAR_BI_IGNORE_MAX_ITER                 : i32 = 6``
    
``const MSK_IPAR_BI_IGNORE_NUM_ERROR                : i32 = 7``
    
``const MSK_IPAR_BI_MAX_ITERATIONS                  : i32 = 8``
    
``const MSK_IPAR_CACHE_LICENSE                      : i32 = 9``
    
``const MSK_IPAR_CHECK_CONVEXITY                    : i32 = 10``
    
``const MSK_IPAR_COMPRESS_STATFILE                  : i32 = 11``
    
``const MSK_IPAR_INFEAS_GENERIC_NAMES               : i32 = 12``
    
``const MSK_IPAR_INFEAS_PREFER_PRIMAL               : i32 = 13``
    
``const MSK_IPAR_INFEAS_REPORT_AUTO                 : i32 = 14``
    
``const MSK_IPAR_INFEAS_REPORT_LEVEL                : i32 = 15``
    
``const MSK_IPAR_INTPNT_BASIS                       : i32 = 16``
    
``const MSK_IPAR_INTPNT_DIFF_STEP                   : i32 = 17``
    
``const MSK_IPAR_INTPNT_HOTSTART                    : i32 = 18``
    
``const MSK_IPAR_INTPNT_MAX_ITERATIONS              : i32 = 19``
    
``const MSK_IPAR_INTPNT_MAX_NUM_COR                 : i32 = 20``
    
``const MSK_IPAR_INTPNT_MAX_NUM_REFINEMENT_STEPS    : i32 = 21``
    
``const MSK_IPAR_INTPNT_MULTI_THREAD                : i32 = 22``
    
``const MSK_IPAR_INTPNT_OFF_COL_TRH                 : i32 = 23``
    
``const MSK_IPAR_INTPNT_ORDER_GP_NUM_SEEDS          : i32 = 24``
    
``const MSK_IPAR_INTPNT_ORDER_METHOD                : i32 = 25``
    
``const MSK_IPAR_INTPNT_PURIFY                      : i32 = 26``
    
``const MSK_IPAR_INTPNT_REGULARIZATION_USE          : i32 = 27``
    
``const MSK_IPAR_INTPNT_SCALING                     : i32 = 28``
    
``const MSK_IPAR_INTPNT_SOLVE_FORM                  : i32 = 29``
    
``const MSK_IPAR_INTPNT_STARTING_POINT              : i32 = 30``
    
``const MSK_IPAR_LICENSE_DEBUG                      : i32 = 31``
    
``const MSK_IPAR_LICENSE_PAUSE_TIME                 : i32 = 32``
    
``const MSK_IPAR_LICENSE_SUPPRESS_EXPIRE_WRNS       : i32 = 33``
    
``const MSK_IPAR_LICENSE_TRH_EXPIRY_WRN             : i32 = 34``
    
``const MSK_IPAR_LICENSE_WAIT                       : i32 = 35``
    
``const MSK_IPAR_LOG                                : i32 = 36``
    
``const MSK_IPAR_LOG_ANA_PRO                        : i32 = 37``
    
``const MSK_IPAR_LOG_BI                             : i32 = 38``
    
``const MSK_IPAR_LOG_BI_FREQ                        : i32 = 39``
    
``const MSK_IPAR_LOG_CHECK_CONVEXITY                : i32 = 40``
    
``const MSK_IPAR_LOG_CUT_SECOND_OPT                 : i32 = 41``
    
``const MSK_IPAR_LOG_EXPAND                         : i32 = 42``
    
``const MSK_IPAR_LOG_FEAS_REPAIR                    : i32 = 43``
    
``const MSK_IPAR_LOG_FILE                           : i32 = 44``
    
``const MSK_IPAR_LOG_INCLUDE_SUMMARY                : i32 = 45``
    
``const MSK_IPAR_LOG_INFEAS_ANA                     : i32 = 46``
    
``const MSK_IPAR_LOG_INTPNT                         : i32 = 47``
    
``const MSK_IPAR_LOG_LOCAL_INFO                     : i32 = 48``
    
``const MSK_IPAR_LOG_MIO                            : i32 = 49``
    
``const MSK_IPAR_LOG_MIO_FREQ                       : i32 = 50``
    
``const MSK_IPAR_LOG_ORDER                          : i32 = 51``
    
``const MSK_IPAR_LOG_PRESOLVE                       : i32 = 52``
    
``const MSK_IPAR_LOG_RESPONSE                       : i32 = 53``
    
``const MSK_IPAR_LOG_SENSITIVITY                    : i32 = 54``
    
``const MSK_IPAR_LOG_SENSITIVITY_OPT                : i32 = 55``
    
``const MSK_IPAR_LOG_SIM                            : i32 = 56``
    
``const MSK_IPAR_LOG_SIM_FREQ                       : i32 = 57``
    
``const MSK_IPAR_LOG_SIM_MINOR                      : i32 = 58``
    
``const MSK_IPAR_LOG_STORAGE                        : i32 = 59``
    
``const MSK_IPAR_MAX_NUM_WARNINGS                   : i32 = 60``
    
``const MSK_IPAR_MIO_BRANCH_DIR                     : i32 = 61``
    
``const MSK_IPAR_MIO_CONIC_OUTER_APPROXIMATION      : i32 = 62``
    
``const MSK_IPAR_MIO_CUT_CLIQUE                     : i32 = 63``
    
``const MSK_IPAR_MIO_CUT_CMIR                       : i32 = 64``
    
``const MSK_IPAR_MIO_CUT_GMI                        : i32 = 65``
    
``const MSK_IPAR_MIO_CUT_IMPLIED_BOUND              : i32 = 66``
    
``const MSK_IPAR_MIO_CUT_KNAPSACK_COVER             : i32 = 67``
    
``const MSK_IPAR_MIO_CUT_SELECTION_LEVEL            : i32 = 68``
    
``const MSK_IPAR_MIO_FEASPUMP_LEVEL                 : i32 = 69``
    
``const MSK_IPAR_MIO_HEURISTIC_LEVEL                : i32 = 70``
    
``const MSK_IPAR_MIO_MAX_NUM_BRANCHES               : i32 = 71``
    
``const MSK_IPAR_MIO_MAX_NUM_RELAXS                 : i32 = 72``
    
``const MSK_IPAR_MIO_MAX_NUM_ROOT_CUT_ROUNDS        : i32 = 73``
    
``const MSK_IPAR_MIO_MAX_NUM_SOLUTIONS              : i32 = 74``
    
``const MSK_IPAR_MIO_MODE                           : i32 = 75``
    
``const MSK_IPAR_MIO_NODE_OPTIMIZER                 : i32 = 76``
    
``const MSK_IPAR_MIO_NODE_SELECTION                 : i32 = 77``
    
``const MSK_IPAR_MIO_PERSPECTIVE_REFORMULATE        : i32 = 78``
    
``const MSK_IPAR_MIO_PROBING_LEVEL                  : i32 = 79``
    
``const MSK_IPAR_MIO_PROPAGATE_OBJECTIVE_CONSTRAINT : i32 = 80``
    
``const MSK_IPAR_MIO_RINS_MAX_NODES                 : i32 = 81``
    
``const MSK_IPAR_MIO_ROOT_OPTIMIZER                 : i32 = 82``
    
``const MSK_IPAR_MIO_ROOT_REPEAT_PRESOLVE_LEVEL     : i32 = 83``
    
``const MSK_IPAR_MIO_SEED                           : i32 = 84``
    
``const MSK_IPAR_MIO_VB_DETECTION_LEVEL             : i32 = 85``
    
``const MSK_IPAR_MT_SPINCOUNT                       : i32 = 86``
    
``const MSK_IPAR_NUM_THREADS                        : i32 = 87``
    
``const MSK_IPAR_OPF_WRITE_HEADER                   : i32 = 88``
    
``const MSK_IPAR_OPF_WRITE_HINTS                    : i32 = 89``
    
``const MSK_IPAR_OPF_WRITE_LINE_LENGTH              : i32 = 90``
    
``const MSK_IPAR_OPF_WRITE_PARAMETERS               : i32 = 91``
    
``const MSK_IPAR_OPF_WRITE_PROBLEM                  : i32 = 92``
    
``const MSK_IPAR_OPF_WRITE_SOL_BAS                  : i32 = 93``
    
``const MSK_IPAR_OPF_WRITE_SOL_ITG                  : i32 = 94``
    
``const MSK_IPAR_OPF_WRITE_SOL_ITR                  : i32 = 95``
    
``const MSK_IPAR_OPF_WRITE_SOLUTIONS                : i32 = 96``
    
``const MSK_IPAR_OPTIMIZER                          : i32 = 97``
    
``const MSK_IPAR_PARAM_READ_CASE_NAME               : i32 = 98``
    
``const MSK_IPAR_PARAM_READ_IGN_ERROR               : i32 = 99``
    
``const MSK_IPAR_PRESOLVE_ELIMINATOR_MAX_FILL       : i32 = 100``
    
``const MSK_IPAR_PRESOLVE_ELIMINATOR_MAX_NUM_TRIES  : i32 = 101``
    
``const MSK_IPAR_PRESOLVE_LEVEL                     : i32 = 102``
    
``const MSK_IPAR_PRESOLVE_LINDEP_ABS_WORK_TRH       : i32 = 103``
    
``const MSK_IPAR_PRESOLVE_LINDEP_REL_WORK_TRH       : i32 = 104``
    
``const MSK_IPAR_PRESOLVE_LINDEP_USE                : i32 = 105``
    
``const MSK_IPAR_PRESOLVE_MAX_NUM_PASS              : i32 = 106``
    
``const MSK_IPAR_PRESOLVE_MAX_NUM_REDUCTIONS        : i32 = 107``
    
``const MSK_IPAR_PRESOLVE_USE                       : i32 = 108``
    
``const MSK_IPAR_PRIMAL_REPAIR_OPTIMIZER            : i32 = 109``
    
``const MSK_IPAR_PTF_WRITE_TRANSFORM                : i32 = 110``
    
``const MSK_IPAR_READ_DEBUG                         : i32 = 111``
    
``const MSK_IPAR_READ_KEEP_FREE_CON                 : i32 = 112``
    
``const MSK_IPAR_READ_LP_DROP_NEW_VARS_IN_BOU       : i32 = 113``
    
``const MSK_IPAR_READ_LP_QUOTED_NAMES               : i32 = 114``
    
``const MSK_IPAR_READ_MPS_FORMAT                    : i32 = 115``
    
``const MSK_IPAR_READ_MPS_WIDTH                     : i32 = 116``
    
``const MSK_IPAR_READ_TASK_IGNORE_PARAM             : i32 = 117``
    
``const MSK_IPAR_REMOVE_UNUSED_SOLUTIONS            : i32 = 118``
    
``const MSK_IPAR_SENSITIVITY_ALL                    : i32 = 119``
    
``const MSK_IPAR_SENSITIVITY_OPTIMIZER              : i32 = 120``
    
``const MSK_IPAR_SENSITIVITY_TYPE                   : i32 = 121``
    
``const MSK_IPAR_SIM_BASIS_FACTOR_USE               : i32 = 122``
    
``const MSK_IPAR_SIM_DEGEN                          : i32 = 123``
    
``const MSK_IPAR_SIM_DUAL_CRASH                     : i32 = 124``
    
``const MSK_IPAR_SIM_DUAL_PHASEONE_METHOD           : i32 = 125``
    
``const MSK_IPAR_SIM_DUAL_RESTRICT_SELECTION        : i32 = 126``
    
``const MSK_IPAR_SIM_DUAL_SELECTION                 : i32 = 127``
    
``const MSK_IPAR_SIM_EXPLOIT_DUPVEC                 : i32 = 128``
    
``const MSK_IPAR_SIM_HOTSTART                       : i32 = 129``
    
``const MSK_IPAR_SIM_HOTSTART_LU                    : i32 = 130``
    
``const MSK_IPAR_SIM_MAX_ITERATIONS                 : i32 = 131``
    
``const MSK_IPAR_SIM_MAX_NUM_SETBACKS               : i32 = 132``
    
``const MSK_IPAR_SIM_NON_SINGULAR                   : i32 = 133``
    
``const MSK_IPAR_SIM_PRIMAL_CRASH                   : i32 = 134``
    
``const MSK_IPAR_SIM_PRIMAL_PHASEONE_METHOD         : i32 = 135``
    
``const MSK_IPAR_SIM_PRIMAL_RESTRICT_SELECTION      : i32 = 136``
    
``const MSK_IPAR_SIM_PRIMAL_SELECTION               : i32 = 137``
    
``const MSK_IPAR_SIM_REFACTOR_FREQ                  : i32 = 138``
    
``const MSK_IPAR_SIM_REFORMULATION                  : i32 = 139``
    
``const MSK_IPAR_SIM_SAVE_LU                        : i32 = 140``
    
``const MSK_IPAR_SIM_SCALING                        : i32 = 141``
    
``const MSK_IPAR_SIM_SCALING_METHOD                 : i32 = 142``
    
``const MSK_IPAR_SIM_SEED                           : i32 = 143``
    
``const MSK_IPAR_SIM_SOLVE_FORM                     : i32 = 144``
    
``const MSK_IPAR_SIM_STABILITY_PRIORITY             : i32 = 145``
    
``const MSK_IPAR_SIM_SWITCH_OPTIMIZER               : i32 = 146``
    
``const MSK_IPAR_SOL_FILTER_KEEP_BASIC              : i32 = 147``
    
``const MSK_IPAR_SOL_FILTER_KEEP_RANGED             : i32 = 148``
    
``const MSK_IPAR_SOL_READ_NAME_WIDTH                : i32 = 149``
    
``const MSK_IPAR_SOL_READ_WIDTH                     : i32 = 150``
    
``const MSK_IPAR_SOLUTION_CALLBACK                  : i32 = 151``
    
``const MSK_IPAR_TIMING_LEVEL                       : i32 = 152``
    
``const MSK_IPAR_WRITE_BAS_CONSTRAINTS              : i32 = 153``
    
``const MSK_IPAR_WRITE_BAS_HEAD                     : i32 = 154``
    
``const MSK_IPAR_WRITE_BAS_VARIABLES                : i32 = 155``
    
``const MSK_IPAR_WRITE_COMPRESSION                  : i32 = 156``
    
``const MSK_IPAR_WRITE_DATA_PARAM                   : i32 = 157``
    
``const MSK_IPAR_WRITE_FREE_CON                     : i32 = 158``
    
``const MSK_IPAR_WRITE_GENERIC_NAMES                : i32 = 159``
    
``const MSK_IPAR_WRITE_GENERIC_NAMES_IO             : i32 = 160``
    
``const MSK_IPAR_WRITE_IGNORE_INCOMPATIBLE_ITEMS    : i32 = 161``
    
``const MSK_IPAR_WRITE_INT_CONSTRAINTS              : i32 = 162``
    
``const MSK_IPAR_WRITE_INT_HEAD                     : i32 = 163``
    
``const MSK_IPAR_WRITE_INT_VARIABLES                : i32 = 164``
    
``const MSK_IPAR_WRITE_LP_FULL_OBJ                  : i32 = 165``
    
``const MSK_IPAR_WRITE_LP_LINE_WIDTH                : i32 = 166``
    
``const MSK_IPAR_WRITE_LP_QUOTED_NAMES              : i32 = 167``
    
``const MSK_IPAR_WRITE_LP_STRICT_FORMAT             : i32 = 168``
    
``const MSK_IPAR_WRITE_LP_TERMS_PER_LINE            : i32 = 169``
    
``const MSK_IPAR_WRITE_MPS_FORMAT                   : i32 = 170``
    
``const MSK_IPAR_WRITE_MPS_INT                      : i32 = 171``
    
``const MSK_IPAR_WRITE_PRECISION                    : i32 = 172``
    
``const MSK_IPAR_WRITE_SOL_BARVARIABLES             : i32 = 173``
    
``const MSK_IPAR_WRITE_SOL_CONSTRAINTS              : i32 = 174``
    
``const MSK_IPAR_WRITE_SOL_HEAD                     : i32 = 175``
    
``const MSK_IPAR_WRITE_SOL_IGNORE_INVALID_NAMES     : i32 = 176``
    
``const MSK_IPAR_WRITE_SOL_VARIABLES                : i32 = 177``
    
``const MSK_IPAR_WRITE_TASK_INC_SOL                 : i32 = 178``
    
``const MSK_IPAR_WRITE_XML_MODE                     : i32 = 179``
    
.. index:: liinfitem
.. index:: LIINF_...
.. _enum_liinfitem:
.. _liinfitem_bi_clean_dual_deg_iter:
.. _liinfitem_bi_clean_dual_iter:
.. _liinfitem_bi_clean_primal_deg_iter:
.. _liinfitem_bi_clean_primal_iter:
.. _liinfitem_bi_dual_iter:
.. _liinfitem_bi_primal_iter:
.. _liinfitem_intpnt_factor_num_nz:
.. _liinfitem_mio_anz:
.. _liinfitem_mio_intpnt_iter:
.. _liinfitem_mio_presolved_anz:
.. _liinfitem_mio_simplex_iter:
.. _liinfitem_rd_numanz:
.. _liinfitem_rd_numqnz:

``liinfitem``
-------------



``const MSK_LIINF_BI_CLEAN_DUAL_DEG_ITER   : i32 = 0``
    
``const MSK_LIINF_BI_CLEAN_DUAL_ITER       : i32 = 1``
    
``const MSK_LIINF_BI_CLEAN_PRIMAL_DEG_ITER : i32 = 2``
    
``const MSK_LIINF_BI_CLEAN_PRIMAL_ITER     : i32 = 3``
    
``const MSK_LIINF_BI_DUAL_ITER             : i32 = 4``
    
``const MSK_LIINF_BI_PRIMAL_ITER           : i32 = 5``
    
``const MSK_LIINF_INTPNT_FACTOR_NUM_NZ     : i32 = 6``
    
``const MSK_LIINF_MIO_ANZ                  : i32 = 7``
    
``const MSK_LIINF_MIO_INTPNT_ITER          : i32 = 8``
    
``const MSK_LIINF_MIO_PRESOLVED_ANZ        : i32 = 9``
    
``const MSK_LIINF_MIO_SIMPLEX_ITER         : i32 = 10``
    
``const MSK_LIINF_RD_NUMANZ                : i32 = 11``
    
``const MSK_LIINF_RD_NUMQNZ                : i32 = 12``
    
.. index:: mark
.. index:: MARK_...
.. _enum_mark:
.. _mark_lo:
.. _mark_up:

``mark``
--------



``const MSK_MARK_LO : i32 = 0``
    
``const MSK_MARK_UP : i32 = 1``
    
.. index:: miocontsoltype
.. index:: MIO_CONT_SOL_...
.. _enum_miocontsoltype:
.. _miocontsoltype_itg:
.. _miocontsoltype_itg_rel:
.. _miocontsoltype_none:
.. _miocontsoltype_root:

``miocontsoltype``
------------------



``const MSK_MIO_CONT_SOL_ITG     : i32 = 2``
    
``const MSK_MIO_CONT_SOL_ITG_REL : i32 = 3``
    
``const MSK_MIO_CONT_SOL_NONE    : i32 = 0``
    
``const MSK_MIO_CONT_SOL_ROOT    : i32 = 1``
    
.. index:: miomode
.. index:: MIO_MODE_...
.. _enum_miomode:
.. _miomode_ignored:
.. _miomode_satisfied:

``miomode``
-----------



``const MSK_MIO_MODE_IGNORED   : i32 = 0``
    
``const MSK_MIO_MODE_SATISFIED : i32 = 1``
    
.. index:: mionodeseltype
.. index:: MIO_NODE_SELECTION_...
.. _enum_mionodeseltype:
.. _mionodeseltype_best:
.. _mionodeseltype_first:
.. _mionodeseltype_free:
.. _mionodeseltype_pseudo:

``mionodeseltype``
------------------



``const MSK_MIO_NODE_SELECTION_BEST   : i32 = 2``
    
``const MSK_MIO_NODE_SELECTION_FIRST  : i32 = 1``
    
``const MSK_MIO_NODE_SELECTION_FREE   : i32 = 0``
    
``const MSK_MIO_NODE_SELECTION_PSEUDO : i32 = 3``
    
.. index:: mpsformat
.. index:: MPS_FORMAT_...
.. _enum_mpsformat:
.. _mpsformat_cplex:
.. _mpsformat_free:
.. _mpsformat_relaxed:
.. _mpsformat_strict:

``mpsformat``
-------------



``const MSK_MPS_FORMAT_CPLEX   : i32 = 3``
    
``const MSK_MPS_FORMAT_FREE    : i32 = 2``
    
``const MSK_MPS_FORMAT_RELAXED : i32 = 1``
    
``const MSK_MPS_FORMAT_STRICT  : i32 = 0``
    
.. index:: nametype
.. index:: NAME_TYPE_...
.. _enum_nametype:
.. _nametype_gen:
.. _nametype_lp:
.. _nametype_mps:

``nametype``
------------



``const MSK_NAME_TYPE_GEN : i32 = 0``
    
``const MSK_NAME_TYPE_LP  : i32 = 2``
    
``const MSK_NAME_TYPE_MPS : i32 = 1``
    
.. index:: objsense
.. index:: OBJECTIVE_SENSE_...
.. _enum_objsense:
.. _objsense_maximize:
.. _objsense_minimize:

``objsense``
------------



``const MSK_OBJECTIVE_SENSE_MAXIMIZE : i32 = 1``
    
``const MSK_OBJECTIVE_SENSE_MINIMIZE : i32 = 0``
    
.. index:: onoffkey
.. index:: ...
.. _enum_onoffkey:
.. _onoffkey_off:
.. _onoffkey_on:

``onoffkey``
------------



``const MSK_OFF : i32 = 0``
    
``const MSK_ON  : i32 = 1``
    
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



``const MSK_OPTIMIZER_CONIC          : i32 = 0``
    
``const MSK_OPTIMIZER_DUAL_SIMPLEX   : i32 = 1``
    
``const MSK_OPTIMIZER_FREE           : i32 = 2``
    
``const MSK_OPTIMIZER_FREE_SIMPLEX   : i32 = 3``
    
``const MSK_OPTIMIZER_INTPNT         : i32 = 4``
    
``const MSK_OPTIMIZER_MIXED_INT      : i32 = 5``
    
``const MSK_OPTIMIZER_PRIMAL_SIMPLEX : i32 = 6``
    
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



``const MSK_ORDER_METHOD_APPMINLOC      : i32 = 1``
    
``const MSK_ORDER_METHOD_EXPERIMENTAL   : i32 = 2``
    
``const MSK_ORDER_METHOD_FORCE_GRAPHPAR : i32 = 4``
    
``const MSK_ORDER_METHOD_FREE           : i32 = 0``
    
``const MSK_ORDER_METHOD_NONE           : i32 = 5``
    
``const MSK_ORDER_METHOD_TRY_GRAPHPAR   : i32 = 3``
    
.. index:: parametertype
.. index:: PAR_...
.. _enum_parametertype:
.. _parametertype_dou_type:
.. _parametertype_int_type:
.. _parametertype_invalid_type:
.. _parametertype_str_type:

``parametertype``
-----------------



``const MSK_PAR_DOU_TYPE     : i32 = 1``
    
``const MSK_PAR_INT_TYPE     : i32 = 2``
    
``const MSK_PAR_INVALID_TYPE : i32 = 0``
    
``const MSK_PAR_STR_TYPE     : i32 = 3``
    
.. index:: presolvemode
.. index:: PRESOLVE_MODE_...
.. _enum_presolvemode:
.. _presolvemode_free:
.. _presolvemode_off:
.. _presolvemode_on:

``presolvemode``
----------------



``const MSK_PRESOLVE_MODE_FREE : i32 = 2``
    
``const MSK_PRESOLVE_MODE_OFF  : i32 = 0``
    
``const MSK_PRESOLVE_MODE_ON   : i32 = 1``
    
.. index:: problemitem
.. index:: PI_...
.. _enum_problemitem:
.. _problemitem_con:
.. _problemitem_cone:
.. _problemitem_var:

``problemitem``
---------------



``const MSK_PI_CON  : i32 = 1``
    
``const MSK_PI_CONE : i32 = 2``
    
``const MSK_PI_VAR  : i32 = 0``
    
.. index:: problemtype
.. index:: PROBTYPE_...
.. _enum_problemtype:
.. _problemtype_conic:
.. _problemtype_lo:
.. _problemtype_mixed:
.. _problemtype_qcqo:
.. _problemtype_qo:

``problemtype``
---------------



``const MSK_PROBTYPE_CONIC : i32 = 3``
    
``const MSK_PROBTYPE_LO    : i32 = 0``
    
``const MSK_PROBTYPE_MIXED : i32 = 4``
    
``const MSK_PROBTYPE_QCQO  : i32 = 2``
    
``const MSK_PROBTYPE_QO    : i32 = 1``
    
.. index:: prosta
.. index:: PRO_STA_...
.. _enum_prosta:
.. _prosta_dual_feas:
.. _prosta_dual_infeas:
.. _prosta_ill_posed:
.. _prosta_prim_and_dual_feas:
.. _prosta_prim_and_dual_infeas:
.. _prosta_prim_feas:
.. _prosta_prim_infeas:
.. _prosta_prim_infeas_or_unbounded:
.. _prosta_unknown:

``prosta``
----------



``const MSK_PRO_STA_DUAL_FEAS                : i32 = 3``
    
``const MSK_PRO_STA_DUAL_INFEAS              : i32 = 5``
    
``const MSK_PRO_STA_ILL_POSED                : i32 = 7``
    
``const MSK_PRO_STA_PRIM_AND_DUAL_FEAS       : i32 = 1``
    
``const MSK_PRO_STA_PRIM_AND_DUAL_INFEAS     : i32 = 6``
    
``const MSK_PRO_STA_PRIM_FEAS                : i32 = 2``
    
``const MSK_PRO_STA_PRIM_INFEAS              : i32 = 4``
    
``const MSK_PRO_STA_PRIM_INFEAS_OR_UNBOUNDED : i32 = 8``
    
``const MSK_PRO_STA_UNKNOWN                  : i32 = 0``
    
.. index:: purify
.. index:: PURIFY_...
.. _enum_purify:
.. _purify_auto:
.. _purify_dual:
.. _purify_none:
.. _purify_primal:
.. _purify_primal_dual:

``purify``
----------



``const MSK_PURIFY_AUTO        : i32 = 4``
    
``const MSK_PURIFY_DUAL        : i32 = 2``
    
``const MSK_PURIFY_NONE        : i32 = 0``
    
``const MSK_PURIFY_PRIMAL      : i32 = 1``
    
``const MSK_PURIFY_PRIMAL_DUAL : i32 = 3``
    
.. index:: rescode
.. index:: RES_...
.. _enum_rescode:
.. _rescode_err_ad_invalid_codelist:
.. _rescode_err_api_array_too_small:
.. _rescode_err_api_cb_connect:
.. _rescode_err_api_fatal_error:
.. _rescode_err_api_internal:
.. _rescode_err_appending_too_big_cone:
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
.. _rescode_err_cbf_duplicate_acoord:
.. _rescode_err_cbf_duplicate_bcoord:
.. _rescode_err_cbf_duplicate_con:
.. _rescode_err_cbf_duplicate_int:
.. _rescode_err_cbf_duplicate_obj:
.. _rescode_err_cbf_duplicate_objacoord:
.. _rescode_err_cbf_duplicate_pow_cones:
.. _rescode_err_cbf_duplicate_pow_star_cones:
.. _rescode_err_cbf_duplicate_psdvar:
.. _rescode_err_cbf_duplicate_var:
.. _rescode_err_cbf_invalid_con_type:
.. _rescode_err_cbf_invalid_dimension_of_cones:
.. _rescode_err_cbf_invalid_domain_dimension:
.. _rescode_err_cbf_invalid_exp_dimension:
.. _rescode_err_cbf_invalid_int_index:
.. _rescode_err_cbf_invalid_number_of_cones:
.. _rescode_err_cbf_invalid_power:
.. _rescode_err_cbf_invalid_power_cone_index:
.. _rescode_err_cbf_invalid_power_star_cone_index:
.. _rescode_err_cbf_invalid_psdvar_dimension:
.. _rescode_err_cbf_invalid_var_type:
.. _rescode_err_cbf_no_variables:
.. _rescode_err_cbf_no_version_specified:
.. _rescode_err_cbf_obj_sense:
.. _rescode_err_cbf_parse:
.. _rescode_err_cbf_power_cone_is_too_long:
.. _rescode_err_cbf_power_cone_mismatch:
.. _rescode_err_cbf_power_star_cone_mismatch:
.. _rescode_err_cbf_syntax:
.. _rescode_err_cbf_too_few_constraints:
.. _rescode_err_cbf_too_few_ints:
.. _rescode_err_cbf_too_few_psdvar:
.. _rescode_err_cbf_too_few_variables:
.. _rescode_err_cbf_too_many_constraints:
.. _rescode_err_cbf_too_many_ints:
.. _rescode_err_cbf_too_many_variables:
.. _rescode_err_cbf_unhandled_power_cone_type:
.. _rescode_err_cbf_unhandled_power_star_cone_type:
.. _rescode_err_cbf_unsupported:
.. _rescode_err_con_q_not_nsd:
.. _rescode_err_con_q_not_psd:
.. _rescode_err_cone_index:
.. _rescode_err_cone_overlap:
.. _rescode_err_cone_overlap_append:
.. _rescode_err_cone_parameter:
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
.. _rescode_err_final_solution:
.. _rescode_err_first:
.. _rescode_err_firsti:
.. _rescode_err_firstj:
.. _rescode_err_fixed_bound_values:
.. _rescode_err_flexlm:
.. _rescode_err_format_string:
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
.. _rescode_err_invalid_aij:
.. _rescode_err_invalid_ampl_stub:
.. _rescode_err_invalid_barvar_name:
.. _rescode_err_invalid_cj:
.. _rescode_err_invalid_compression:
.. _rescode_err_invalid_con_name:
.. _rescode_err_invalid_cone_name:
.. _rescode_err_invalid_file_format_for_cfix:
.. _rescode_err_invalid_file_format_for_cones:
.. _rescode_err_invalid_file_format_for_free_constraints:
.. _rescode_err_invalid_file_format_for_nonlinear:
.. _rescode_err_invalid_file_format_for_ranged_constraints:
.. _rescode_err_invalid_file_format_for_sym_mat:
.. _rescode_err_invalid_file_name:
.. _rescode_err_invalid_format_type:
.. _rescode_err_invalid_idx:
.. _rescode_err_invalid_iomode:
.. _rescode_err_invalid_max_num:
.. _rescode_err_invalid_name_in_sol_file:
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
.. _rescode_err_nonlinear_ranged:
.. _rescode_err_null_env:
.. _rescode_err_null_pointer:
.. _rescode_err_null_task:
.. _rescode_err_num_arguments:
.. _rescode_err_numconlim:
.. _rescode_err_numvarlim:
.. _rescode_err_obj_q_not_nsd:
.. _rescode_err_obj_q_not_psd:
.. _rescode_err_objective_range:
.. _rescode_err_older_dll:
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
.. _rescode_err_ptf_format:
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
.. _rescode_err_server_problem_size:
.. _rescode_err_server_protocol:
.. _rescode_err_server_status:
.. _rescode_err_server_token:
.. _rescode_err_shape_is_too_large:
.. _rescode_err_size_license:
.. _rescode_err_size_license_con:
.. _rescode_err_size_license_intvar:
.. _rescode_err_size_license_numcores:
.. _rescode_err_size_license_var:
.. _rescode_err_slice_size:
.. _rescode_err_sol_file_invalid_number:
.. _rescode_err_solitem:
.. _rescode_err_solver_probtype:
.. _rescode_err_space:
.. _rescode_err_space_leaking:
.. _rescode_err_space_no_info:
.. _rescode_err_sym_mat_duplicate:
.. _rescode_err_sym_mat_huge:
.. _rescode_err_sym_mat_invalid:
.. _rescode_err_sym_mat_invalid_col_index:
.. _rescode_err_sym_mat_invalid_row_index:
.. _rescode_err_sym_mat_invalid_value:
.. _rescode_err_sym_mat_not_lower_tringular:
.. _rescode_err_task_incompatible:
.. _rescode_err_task_invalid:
.. _rescode_err_task_write:
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
.. _rescode_err_too_small_a_truncation_value:
.. _rescode_err_too_small_max_num_nz:
.. _rescode_err_too_small_maxnumanz:
.. _rescode_err_unb_step_size:
.. _rescode_err_undef_solution:
.. _rescode_err_undefined_objective_sense:
.. _rescode_err_unhandled_solution_status:
.. _rescode_err_unknown:
.. _rescode_err_upper_bound_is_a_nan:
.. _rescode_err_upper_triangle:
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
.. _rescode_wrn_dropped_nz_qobj:
.. _rescode_wrn_duplicate_barvariable_names:
.. _rescode_wrn_duplicate_cone_names:
.. _rescode_wrn_duplicate_constraint_names:
.. _rescode_wrn_duplicate_variable_names:
.. _rescode_wrn_eliminator_space:
.. _rescode_wrn_empty_name:
.. _rescode_wrn_exp_cones_with_variables_fixed_at_zero:
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
.. _rescode_wrn_nz_in_upr_tri:
.. _rescode_wrn_open_param_file:
.. _rescode_wrn_param_ignored_cmio:
.. _rescode_wrn_param_name_dou:
.. _rescode_wrn_param_name_int:
.. _rescode_wrn_param_name_str:
.. _rescode_wrn_param_str_value:
.. _rescode_wrn_pow_cones_with_root_fixed_at_zero:
.. _rescode_wrn_presolve_outofspace:
.. _rescode_wrn_quad_cones_with_root_fixed_at_zero:
.. _rescode_wrn_rquad_cones_with_root_fixed_at_zero:
.. _rescode_wrn_sol_file_ignored_con:
.. _rescode_wrn_sol_file_ignored_var:
.. _rescode_wrn_sol_filter:
.. _rescode_wrn_spar_max_len:
.. _rescode_wrn_sym_mat_large:
.. _rescode_wrn_too_few_basis_vars:
.. _rescode_wrn_too_many_basis_vars:
.. _rescode_wrn_undef_sol_file_name:
.. _rescode_wrn_using_generic_names:
.. _rescode_wrn_write_changed_names:
.. _rescode_wrn_write_discarded_cfix:
.. _rescode_wrn_zero_aij:
.. _rescode_wrn_zeros_in_sparse_col:
.. _rescode_wrn_zeros_in_sparse_row:

``rescode``
-----------



``const MSK_RES_ERR_AD_INVALID_CODELIST                        : i32 = 3102``
    
``const MSK_RES_ERR_API_ARRAY_TOO_SMALL                        : i32 = 3001``
    
``const MSK_RES_ERR_API_CB_CONNECT                             : i32 = 3002``
    
``const MSK_RES_ERR_API_FATAL_ERROR                            : i32 = 3005``
    
``const MSK_RES_ERR_API_INTERNAL                               : i32 = 3999``
    
``const MSK_RES_ERR_APPENDING_TOO_BIG_CONE                     : i32 = 1311``
    
``const MSK_RES_ERR_ARG_IS_TOO_LARGE                           : i32 = 1227``
    
``const MSK_RES_ERR_ARG_IS_TOO_SMALL                           : i32 = 1226``
    
``const MSK_RES_ERR_ARGUMENT_DIMENSION                         : i32 = 1201``
    
``const MSK_RES_ERR_ARGUMENT_IS_TOO_LARGE                      : i32 = 5005``
    
``const MSK_RES_ERR_ARGUMENT_LENNEQ                            : i32 = 1197``
    
``const MSK_RES_ERR_ARGUMENT_PERM_ARRAY                        : i32 = 1299``
    
``const MSK_RES_ERR_ARGUMENT_TYPE                              : i32 = 1198``
    
``const MSK_RES_ERR_BAR_VAR_DIM                                : i32 = 3920``
    
``const MSK_RES_ERR_BASIS                                      : i32 = 1266``
    
``const MSK_RES_ERR_BASIS_FACTOR                               : i32 = 1610``
    
``const MSK_RES_ERR_BASIS_SINGULAR                             : i32 = 1615``
    
``const MSK_RES_ERR_BLANK_NAME                                 : i32 = 1070``
    
``const MSK_RES_ERR_CBF_DUPLICATE_ACOORD                       : i32 = 7116``
    
``const MSK_RES_ERR_CBF_DUPLICATE_BCOORD                       : i32 = 7115``
    
``const MSK_RES_ERR_CBF_DUPLICATE_CON                          : i32 = 7108``
    
``const MSK_RES_ERR_CBF_DUPLICATE_INT                          : i32 = 7110``
    
``const MSK_RES_ERR_CBF_DUPLICATE_OBJ                          : i32 = 7107``
    
``const MSK_RES_ERR_CBF_DUPLICATE_OBJACOORD                    : i32 = 7114``
    
``const MSK_RES_ERR_CBF_DUPLICATE_POW_CONES                    : i32 = 7130``
    
``const MSK_RES_ERR_CBF_DUPLICATE_POW_STAR_CONES               : i32 = 7131``
    
``const MSK_RES_ERR_CBF_DUPLICATE_PSDVAR                       : i32 = 7123``
    
``const MSK_RES_ERR_CBF_DUPLICATE_VAR                          : i32 = 7109``
    
``const MSK_RES_ERR_CBF_INVALID_CON_TYPE                       : i32 = 7112``
    
``const MSK_RES_ERR_CBF_INVALID_DIMENSION_OF_CONES             : i32 = 7741``
    
``const MSK_RES_ERR_CBF_INVALID_DOMAIN_DIMENSION               : i32 = 7113``
    
``const MSK_RES_ERR_CBF_INVALID_EXP_DIMENSION                  : i32 = 7126``
    
``const MSK_RES_ERR_CBF_INVALID_INT_INDEX                      : i32 = 7121``
    
``const MSK_RES_ERR_CBF_INVALID_NUMBER_OF_CONES                : i32 = 7740``
    
``const MSK_RES_ERR_CBF_INVALID_POWER                          : i32 = 7132``
    
``const MSK_RES_ERR_CBF_INVALID_POWER_CONE_INDEX               : i32 = 7134``
    
``const MSK_RES_ERR_CBF_INVALID_POWER_STAR_CONE_INDEX          : i32 = 7135``
    
``const MSK_RES_ERR_CBF_INVALID_PSDVAR_DIMENSION               : i32 = 7124``
    
``const MSK_RES_ERR_CBF_INVALID_VAR_TYPE                       : i32 = 7111``
    
``const MSK_RES_ERR_CBF_NO_VARIABLES                           : i32 = 7102``
    
``const MSK_RES_ERR_CBF_NO_VERSION_SPECIFIED                   : i32 = 7105``
    
``const MSK_RES_ERR_CBF_OBJ_SENSE                              : i32 = 7101``
    
``const MSK_RES_ERR_CBF_PARSE                                  : i32 = 7100``
    
``const MSK_RES_ERR_CBF_POWER_CONE_IS_TOO_LONG                 : i32 = 7133``
    
``const MSK_RES_ERR_CBF_POWER_CONE_MISMATCH                    : i32 = 7138``
    
``const MSK_RES_ERR_CBF_POWER_STAR_CONE_MISMATCH               : i32 = 7139``
    
``const MSK_RES_ERR_CBF_SYNTAX                                 : i32 = 7106``
    
``const MSK_RES_ERR_CBF_TOO_FEW_CONSTRAINTS                    : i32 = 7118``
    
``const MSK_RES_ERR_CBF_TOO_FEW_INTS                           : i32 = 7119``
    
``const MSK_RES_ERR_CBF_TOO_FEW_PSDVAR                         : i32 = 7125``
    
``const MSK_RES_ERR_CBF_TOO_FEW_VARIABLES                      : i32 = 7117``
    
``const MSK_RES_ERR_CBF_TOO_MANY_CONSTRAINTS                   : i32 = 7103``
    
``const MSK_RES_ERR_CBF_TOO_MANY_INTS                          : i32 = 7120``
    
``const MSK_RES_ERR_CBF_TOO_MANY_VARIABLES                     : i32 = 7104``
    
``const MSK_RES_ERR_CBF_UNHANDLED_POWER_CONE_TYPE              : i32 = 7136``
    
``const MSK_RES_ERR_CBF_UNHANDLED_POWER_STAR_CONE_TYPE         : i32 = 7137``
    
``const MSK_RES_ERR_CBF_UNSUPPORTED                            : i32 = 7122``
    
``const MSK_RES_ERR_CON_Q_NOT_NSD                              : i32 = 1294``
    
``const MSK_RES_ERR_CON_Q_NOT_PSD                              : i32 = 1293``
    
``const MSK_RES_ERR_CONE_INDEX                                 : i32 = 1300``
    
``const MSK_RES_ERR_CONE_OVERLAP                               : i32 = 1302``
    
``const MSK_RES_ERR_CONE_OVERLAP_APPEND                        : i32 = 1307``
    
``const MSK_RES_ERR_CONE_PARAMETER                             : i32 = 1320``
    
``const MSK_RES_ERR_CONE_REP_VAR                               : i32 = 1303``
    
``const MSK_RES_ERR_CONE_SIZE                                  : i32 = 1301``
    
``const MSK_RES_ERR_CONE_TYPE                                  : i32 = 1305``
    
``const MSK_RES_ERR_CONE_TYPE_STR                              : i32 = 1306``
    
``const MSK_RES_ERR_DATA_FILE_EXT                              : i32 = 1055``
    
``const MSK_RES_ERR_DUP_NAME                                   : i32 = 1071``
    
``const MSK_RES_ERR_DUPLICATE_AIJ                              : i32 = 1385``
    
``const MSK_RES_ERR_DUPLICATE_BARVARIABLE_NAMES                : i32 = 4502``
    
``const MSK_RES_ERR_DUPLICATE_CONE_NAMES                       : i32 = 4503``
    
``const MSK_RES_ERR_DUPLICATE_CONSTRAINT_NAMES                 : i32 = 4500``
    
``const MSK_RES_ERR_DUPLICATE_VARIABLE_NAMES                   : i32 = 4501``
    
``const MSK_RES_ERR_END_OF_FILE                                : i32 = 1059``
    
``const MSK_RES_ERR_FACTOR                                     : i32 = 1650``
    
``const MSK_RES_ERR_FEASREPAIR_CANNOT_RELAX                    : i32 = 1700``
    
``const MSK_RES_ERR_FEASREPAIR_INCONSISTENT_BOUND              : i32 = 1702``
    
``const MSK_RES_ERR_FEASREPAIR_SOLVING_RELAXED                 : i32 = 1701``
    
``const MSK_RES_ERR_FILE_LICENSE                               : i32 = 1007``
    
``const MSK_RES_ERR_FILE_OPEN                                  : i32 = 1052``
    
``const MSK_RES_ERR_FILE_READ                                  : i32 = 1053``
    
``const MSK_RES_ERR_FILE_WRITE                                 : i32 = 1054``
    
``const MSK_RES_ERR_FINAL_SOLUTION                             : i32 = 1560``
    
``const MSK_RES_ERR_FIRST                                      : i32 = 1570``
    
``const MSK_RES_ERR_FIRSTI                                     : i32 = 1285``
    
``const MSK_RES_ERR_FIRSTJ                                     : i32 = 1287``
    
``const MSK_RES_ERR_FIXED_BOUND_VALUES                         : i32 = 1420``
    
``const MSK_RES_ERR_FLEXLM                                     : i32 = 1014``
    
``const MSK_RES_ERR_FORMAT_STRING                              : i32 = 1072``
    
``const MSK_RES_ERR_GLOBAL_INV_CONIC_PROBLEM                   : i32 = 1503``
    
``const MSK_RES_ERR_HUGE_AIJ                                   : i32 = 1380``
    
``const MSK_RES_ERR_HUGE_C                                     : i32 = 1375``
    
``const MSK_RES_ERR_IDENTICAL_TASKS                            : i32 = 3101``
    
``const MSK_RES_ERR_IN_ARGUMENT                                : i32 = 1200``
    
``const MSK_RES_ERR_INDEX                                      : i32 = 1235``
    
``const MSK_RES_ERR_INDEX_ARR_IS_TOO_LARGE                     : i32 = 1222``
    
``const MSK_RES_ERR_INDEX_ARR_IS_TOO_SMALL                     : i32 = 1221``
    
``const MSK_RES_ERR_INDEX_IS_TOO_LARGE                         : i32 = 1204``
    
``const MSK_RES_ERR_INDEX_IS_TOO_SMALL                         : i32 = 1203``
    
``const MSK_RES_ERR_INF_DOU_INDEX                              : i32 = 1219``
    
``const MSK_RES_ERR_INF_DOU_NAME                               : i32 = 1230``
    
``const MSK_RES_ERR_INF_INT_INDEX                              : i32 = 1220``
    
``const MSK_RES_ERR_INF_INT_NAME                               : i32 = 1231``
    
``const MSK_RES_ERR_INF_LINT_INDEX                             : i32 = 1225``
    
``const MSK_RES_ERR_INF_LINT_NAME                              : i32 = 1234``
    
``const MSK_RES_ERR_INF_TYPE                                   : i32 = 1232``
    
``const MSK_RES_ERR_INFEAS_UNDEFINED                           : i32 = 3910``
    
``const MSK_RES_ERR_INFINITE_BOUND                             : i32 = 1400``
    
``const MSK_RES_ERR_INT64_TO_INT32_CAST                        : i32 = 3800``
    
``const MSK_RES_ERR_INTERNAL                                   : i32 = 3000``
    
``const MSK_RES_ERR_INTERNAL_TEST_FAILED                       : i32 = 3500``
    
``const MSK_RES_ERR_INV_APTRE                                  : i32 = 1253``
    
``const MSK_RES_ERR_INV_BK                                     : i32 = 1255``
    
``const MSK_RES_ERR_INV_BKC                                    : i32 = 1256``
    
``const MSK_RES_ERR_INV_BKX                                    : i32 = 1257``
    
``const MSK_RES_ERR_INV_CONE_TYPE                              : i32 = 1272``
    
``const MSK_RES_ERR_INV_CONE_TYPE_STR                          : i32 = 1271``
    
``const MSK_RES_ERR_INV_MARKI                                  : i32 = 2501``
    
``const MSK_RES_ERR_INV_MARKJ                                  : i32 = 2502``
    
``const MSK_RES_ERR_INV_NAME_ITEM                              : i32 = 1280``
    
``const MSK_RES_ERR_INV_NUMI                                   : i32 = 2503``
    
``const MSK_RES_ERR_INV_NUMJ                                   : i32 = 2504``
    
``const MSK_RES_ERR_INV_OPTIMIZER                              : i32 = 1550``
    
``const MSK_RES_ERR_INV_PROBLEM                                : i32 = 1500``
    
``const MSK_RES_ERR_INV_QCON_SUBI                              : i32 = 1405``
    
``const MSK_RES_ERR_INV_QCON_SUBJ                              : i32 = 1406``
    
``const MSK_RES_ERR_INV_QCON_SUBK                              : i32 = 1404``
    
``const MSK_RES_ERR_INV_QCON_VAL                               : i32 = 1407``
    
``const MSK_RES_ERR_INV_QOBJ_SUBI                              : i32 = 1401``
    
``const MSK_RES_ERR_INV_QOBJ_SUBJ                              : i32 = 1402``
    
``const MSK_RES_ERR_INV_QOBJ_VAL                               : i32 = 1403``
    
``const MSK_RES_ERR_INV_SK                                     : i32 = 1270``
    
``const MSK_RES_ERR_INV_SK_STR                                 : i32 = 1269``
    
``const MSK_RES_ERR_INV_SKC                                    : i32 = 1267``
    
``const MSK_RES_ERR_INV_SKN                                    : i32 = 1274``
    
``const MSK_RES_ERR_INV_SKX                                    : i32 = 1268``
    
``const MSK_RES_ERR_INV_VAR_TYPE                               : i32 = 1258``
    
``const MSK_RES_ERR_INVALID_AIJ                                : i32 = 1473``
    
``const MSK_RES_ERR_INVALID_AMPL_STUB                          : i32 = 3700``
    
``const MSK_RES_ERR_INVALID_BARVAR_NAME                        : i32 = 1079``
    
``const MSK_RES_ERR_INVALID_CJ                                 : i32 = 1474``
    
``const MSK_RES_ERR_INVALID_COMPRESSION                        : i32 = 1800``
    
``const MSK_RES_ERR_INVALID_CON_NAME                           : i32 = 1076``
    
``const MSK_RES_ERR_INVALID_CONE_NAME                          : i32 = 1078``
    
``const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_CFIX               : i32 = 4001``
    
``const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_CONES              : i32 = 4005``
    
``const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_FREE_CONSTRAINTS   : i32 = 4003``
    
``const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_NONLINEAR          : i32 = 4010``
    
``const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_RANGED_CONSTRAINTS : i32 = 4002``
    
``const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_SYM_MAT            : i32 = 4000``
    
``const MSK_RES_ERR_INVALID_FILE_NAME                          : i32 = 1056``
    
``const MSK_RES_ERR_INVALID_FORMAT_TYPE                        : i32 = 1283``
    
``const MSK_RES_ERR_INVALID_IDX                                : i32 = 1246``
    
``const MSK_RES_ERR_INVALID_IOMODE                             : i32 = 1801``
    
``const MSK_RES_ERR_INVALID_MAX_NUM                            : i32 = 1247``
    
``const MSK_RES_ERR_INVALID_NAME_IN_SOL_FILE                   : i32 = 1170``
    
``const MSK_RES_ERR_INVALID_OBJ_NAME                           : i32 = 1075``
    
``const MSK_RES_ERR_INVALID_OBJECTIVE_SENSE                    : i32 = 1445``
    
``const MSK_RES_ERR_INVALID_PROBLEM_TYPE                       : i32 = 6000``
    
``const MSK_RES_ERR_INVALID_SOL_FILE_NAME                      : i32 = 1057``
    
``const MSK_RES_ERR_INVALID_STREAM                             : i32 = 1062``
    
``const MSK_RES_ERR_INVALID_SURPLUS                            : i32 = 1275``
    
``const MSK_RES_ERR_INVALID_SYM_MAT_DIM                        : i32 = 3950``
    
``const MSK_RES_ERR_INVALID_TASK                               : i32 = 1064``
    
``const MSK_RES_ERR_INVALID_UTF8                               : i32 = 2900``
    
``const MSK_RES_ERR_INVALID_VAR_NAME                           : i32 = 1077``
    
``const MSK_RES_ERR_INVALID_WCHAR                              : i32 = 2901``
    
``const MSK_RES_ERR_INVALID_WHICHSOL                           : i32 = 1228``
    
``const MSK_RES_ERR_JSON_DATA                                  : i32 = 1179``
    
``const MSK_RES_ERR_JSON_FORMAT                                : i32 = 1178``
    
``const MSK_RES_ERR_JSON_MISSING_DATA                          : i32 = 1180``
    
``const MSK_RES_ERR_JSON_NUMBER_OVERFLOW                       : i32 = 1177``
    
``const MSK_RES_ERR_JSON_STRING                                : i32 = 1176``
    
``const MSK_RES_ERR_JSON_SYNTAX                                : i32 = 1175``
    
``const MSK_RES_ERR_LAST                                       : i32 = 1571``
    
``const MSK_RES_ERR_LASTI                                      : i32 = 1286``
    
``const MSK_RES_ERR_LASTJ                                      : i32 = 1288``
    
``const MSK_RES_ERR_LAU_ARG_K                                  : i32 = 7012``
    
``const MSK_RES_ERR_LAU_ARG_M                                  : i32 = 7010``
    
``const MSK_RES_ERR_LAU_ARG_N                                  : i32 = 7011``
    
``const MSK_RES_ERR_LAU_ARG_TRANS                              : i32 = 7018``
    
``const MSK_RES_ERR_LAU_ARG_TRANSA                             : i32 = 7015``
    
``const MSK_RES_ERR_LAU_ARG_TRANSB                             : i32 = 7016``
    
``const MSK_RES_ERR_LAU_ARG_UPLO                               : i32 = 7017``
    
``const MSK_RES_ERR_LAU_INVALID_LOWER_TRIANGULAR_MATRIX        : i32 = 7002``
    
``const MSK_RES_ERR_LAU_INVALID_SPARSE_SYMMETRIC_MATRIX        : i32 = 7019``
    
``const MSK_RES_ERR_LAU_NOT_POSITIVE_DEFINITE                  : i32 = 7001``
    
``const MSK_RES_ERR_LAU_SINGULAR_MATRIX                        : i32 = 7000``
    
``const MSK_RES_ERR_LAU_UNKNOWN                                : i32 = 7005``
    
``const MSK_RES_ERR_LICENSE                                    : i32 = 1000``
    
``const MSK_RES_ERR_LICENSE_CANNOT_ALLOCATE                    : i32 = 1020``
    
``const MSK_RES_ERR_LICENSE_CANNOT_CONNECT                     : i32 = 1021``
    
``const MSK_RES_ERR_LICENSE_EXPIRED                            : i32 = 1001``
    
``const MSK_RES_ERR_LICENSE_FEATURE                            : i32 = 1018``
    
``const MSK_RES_ERR_LICENSE_INVALID_HOSTID                     : i32 = 1025``
    
``const MSK_RES_ERR_LICENSE_MAX                                : i32 = 1016``
    
``const MSK_RES_ERR_LICENSE_MOSEKLM_DAEMON                     : i32 = 1017``
    
``const MSK_RES_ERR_LICENSE_NO_SERVER_LINE                     : i32 = 1028``
    
``const MSK_RES_ERR_LICENSE_NO_SERVER_SUPPORT                  : i32 = 1027``
    
``const MSK_RES_ERR_LICENSE_SERVER                             : i32 = 1015``
    
``const MSK_RES_ERR_LICENSE_SERVER_VERSION                     : i32 = 1026``
    
``const MSK_RES_ERR_LICENSE_VERSION                            : i32 = 1002``
    
``const MSK_RES_ERR_LINK_FILE_DLL                              : i32 = 1040``
    
``const MSK_RES_ERR_LIVING_TASKS                               : i32 = 1066``
    
``const MSK_RES_ERR_LOWER_BOUND_IS_A_NAN                       : i32 = 1390``
    
``const MSK_RES_ERR_LP_DUP_SLACK_NAME                          : i32 = 1152``
    
``const MSK_RES_ERR_LP_EMPTY                                   : i32 = 1151``
    
``const MSK_RES_ERR_LP_FILE_FORMAT                             : i32 = 1157``
    
``const MSK_RES_ERR_LP_FORMAT                                  : i32 = 1160``
    
``const MSK_RES_ERR_LP_FREE_CONSTRAINT                         : i32 = 1155``
    
``const MSK_RES_ERR_LP_INCOMPATIBLE                            : i32 = 1150``
    
``const MSK_RES_ERR_LP_INVALID_CON_NAME                        : i32 = 1171``
    
``const MSK_RES_ERR_LP_INVALID_VAR_NAME                        : i32 = 1154``
    
``const MSK_RES_ERR_LP_WRITE_CONIC_PROBLEM                     : i32 = 1163``
    
``const MSK_RES_ERR_LP_WRITE_GECO_PROBLEM                      : i32 = 1164``
    
``const MSK_RES_ERR_LU_MAX_NUM_TRIES                           : i32 = 2800``
    
``const MSK_RES_ERR_MAX_LEN_IS_TOO_SMALL                       : i32 = 1289``
    
``const MSK_RES_ERR_MAXNUMBARVAR                               : i32 = 1242``
    
``const MSK_RES_ERR_MAXNUMCON                                  : i32 = 1240``
    
``const MSK_RES_ERR_MAXNUMCONE                                 : i32 = 1304``
    
``const MSK_RES_ERR_MAXNUMQNZ                                  : i32 = 1243``
    
``const MSK_RES_ERR_MAXNUMVAR                                  : i32 = 1241``
    
``const MSK_RES_ERR_MIO_INTERNAL                               : i32 = 5010``
    
``const MSK_RES_ERR_MIO_INVALID_NODE_OPTIMIZER                 : i32 = 7701``
    
``const MSK_RES_ERR_MIO_INVALID_ROOT_OPTIMIZER                 : i32 = 7700``
    
``const MSK_RES_ERR_MIO_NO_OPTIMIZER                           : i32 = 1551``
    
``const MSK_RES_ERR_MISSING_LICENSE_FILE                       : i32 = 1008``
    
``const MSK_RES_ERR_MIXED_CONIC_AND_NL                         : i32 = 1501``
    
``const MSK_RES_ERR_MPS_CONE_OVERLAP                           : i32 = 1118``
    
``const MSK_RES_ERR_MPS_CONE_REPEAT                            : i32 = 1119``
    
``const MSK_RES_ERR_MPS_CONE_TYPE                              : i32 = 1117``
    
``const MSK_RES_ERR_MPS_DUPLICATE_Q_ELEMENT                    : i32 = 1121``
    
``const MSK_RES_ERR_MPS_FILE                                   : i32 = 1100``
    
``const MSK_RES_ERR_MPS_INV_BOUND_KEY                          : i32 = 1108``
    
``const MSK_RES_ERR_MPS_INV_CON_KEY                            : i32 = 1107``
    
``const MSK_RES_ERR_MPS_INV_FIELD                              : i32 = 1101``
    
``const MSK_RES_ERR_MPS_INV_MARKER                             : i32 = 1102``
    
``const MSK_RES_ERR_MPS_INV_SEC_NAME                           : i32 = 1109``
    
``const MSK_RES_ERR_MPS_INV_SEC_ORDER                          : i32 = 1115``
    
``const MSK_RES_ERR_MPS_INVALID_OBJ_NAME                       : i32 = 1128``
    
``const MSK_RES_ERR_MPS_INVALID_OBJSENSE                       : i32 = 1122``
    
``const MSK_RES_ERR_MPS_MUL_CON_NAME                           : i32 = 1112``
    
``const MSK_RES_ERR_MPS_MUL_CSEC                               : i32 = 1116``
    
``const MSK_RES_ERR_MPS_MUL_QOBJ                               : i32 = 1114``
    
``const MSK_RES_ERR_MPS_MUL_QSEC                               : i32 = 1113``
    
``const MSK_RES_ERR_MPS_NO_OBJECTIVE                           : i32 = 1110``
    
``const MSK_RES_ERR_MPS_NON_SYMMETRIC_Q                        : i32 = 1120``
    
``const MSK_RES_ERR_MPS_NULL_CON_NAME                          : i32 = 1103``
    
``const MSK_RES_ERR_MPS_NULL_VAR_NAME                          : i32 = 1104``
    
``const MSK_RES_ERR_MPS_SPLITTED_VAR                           : i32 = 1111``
    
``const MSK_RES_ERR_MPS_TAB_IN_FIELD2                          : i32 = 1125``
    
``const MSK_RES_ERR_MPS_TAB_IN_FIELD3                          : i32 = 1126``
    
``const MSK_RES_ERR_MPS_TAB_IN_FIELD5                          : i32 = 1127``
    
``const MSK_RES_ERR_MPS_UNDEF_CON_NAME                         : i32 = 1105``
    
``const MSK_RES_ERR_MPS_UNDEF_VAR_NAME                         : i32 = 1106``
    
``const MSK_RES_ERR_MUL_A_ELEMENT                              : i32 = 1254``
    
``const MSK_RES_ERR_NAME_IS_NULL                               : i32 = 1760``
    
``const MSK_RES_ERR_NAME_MAX_LEN                               : i32 = 1750``
    
``const MSK_RES_ERR_NAN_IN_BLC                                 : i32 = 1461``
    
``const MSK_RES_ERR_NAN_IN_BLX                                 : i32 = 1471``
    
``const MSK_RES_ERR_NAN_IN_BUC                                 : i32 = 1462``
    
``const MSK_RES_ERR_NAN_IN_BUX                                 : i32 = 1472``
    
``const MSK_RES_ERR_NAN_IN_C                                   : i32 = 1470``
    
``const MSK_RES_ERR_NAN_IN_DOUBLE_DATA                         : i32 = 1450``
    
``const MSK_RES_ERR_NEGATIVE_APPEND                            : i32 = 1578``
    
``const MSK_RES_ERR_NEGATIVE_SURPLUS                           : i32 = 1573``
    
``const MSK_RES_ERR_NEWER_DLL                                  : i32 = 1036``
    
``const MSK_RES_ERR_NO_BARS_FOR_SOLUTION                       : i32 = 3916``
    
``const MSK_RES_ERR_NO_BARX_FOR_SOLUTION                       : i32 = 3915``
    
``const MSK_RES_ERR_NO_BASIS_SOL                               : i32 = 1600``
    
``const MSK_RES_ERR_NO_DUAL_FOR_ITG_SOL                        : i32 = 2950``
    
``const MSK_RES_ERR_NO_DUAL_INFEAS_CER                         : i32 = 2001``
    
``const MSK_RES_ERR_NO_INIT_ENV                                : i32 = 1063``
    
``const MSK_RES_ERR_NO_OPTIMIZER_VAR_TYPE                      : i32 = 1552``
    
``const MSK_RES_ERR_NO_PRIMAL_INFEAS_CER                       : i32 = 2000``
    
``const MSK_RES_ERR_NO_SNX_FOR_BAS_SOL                         : i32 = 2953``
    
``const MSK_RES_ERR_NO_SOLUTION_IN_CALLBACK                    : i32 = 2500``
    
``const MSK_RES_ERR_NON_UNIQUE_ARRAY                           : i32 = 5000``
    
``const MSK_RES_ERR_NONCONVEX                                  : i32 = 1291``
    
``const MSK_RES_ERR_NONLINEAR_EQUALITY                         : i32 = 1290``
    
``const MSK_RES_ERR_NONLINEAR_RANGED                           : i32 = 1292``
    
``const MSK_RES_ERR_NULL_ENV                                   : i32 = 1060``
    
``const MSK_RES_ERR_NULL_POINTER                               : i32 = 1065``
    
``const MSK_RES_ERR_NULL_TASK                                  : i32 = 1061``
    
``const MSK_RES_ERR_NUM_ARGUMENTS                              : i32 = 1199``
    
``const MSK_RES_ERR_NUMCONLIM                                  : i32 = 1250``
    
``const MSK_RES_ERR_NUMVARLIM                                  : i32 = 1251``
    
``const MSK_RES_ERR_OBJ_Q_NOT_NSD                              : i32 = 1296``
    
``const MSK_RES_ERR_OBJ_Q_NOT_PSD                              : i32 = 1295``
    
``const MSK_RES_ERR_OBJECTIVE_RANGE                            : i32 = 1260``
    
``const MSK_RES_ERR_OLDER_DLL                                  : i32 = 1035``
    
``const MSK_RES_ERR_OPF_FORMAT                                 : i32 = 1168``
    
``const MSK_RES_ERR_OPF_NEW_VARIABLE                           : i32 = 1169``
    
``const MSK_RES_ERR_OPF_PREMATURE_EOF                          : i32 = 1172``
    
``const MSK_RES_ERR_OPTIMIZER_LICENSE                          : i32 = 1013``
    
``const MSK_RES_ERR_OVERFLOW                                   : i32 = 1590``
    
``const MSK_RES_ERR_PARAM_INDEX                                : i32 = 1210``
    
``const MSK_RES_ERR_PARAM_IS_TOO_LARGE                         : i32 = 1215``
    
``const MSK_RES_ERR_PARAM_IS_TOO_SMALL                         : i32 = 1216``
    
``const MSK_RES_ERR_PARAM_NAME                                 : i32 = 1205``
    
``const MSK_RES_ERR_PARAM_NAME_DOU                             : i32 = 1206``
    
``const MSK_RES_ERR_PARAM_NAME_INT                             : i32 = 1207``
    
``const MSK_RES_ERR_PARAM_NAME_STR                             : i32 = 1208``
    
``const MSK_RES_ERR_PARAM_TYPE                                 : i32 = 1218``
    
``const MSK_RES_ERR_PARAM_VALUE_STR                            : i32 = 1217``
    
``const MSK_RES_ERR_PLATFORM_NOT_LICENSED                      : i32 = 1019``
    
``const MSK_RES_ERR_POSTSOLVE                                  : i32 = 1580``
    
``const MSK_RES_ERR_PRO_ITEM                                   : i32 = 1281``
    
``const MSK_RES_ERR_PROB_LICENSE                               : i32 = 1006``
    
``const MSK_RES_ERR_PTF_FORMAT                                 : i32 = 1167``
    
``const MSK_RES_ERR_QCON_SUBI_TOO_LARGE                        : i32 = 1409``
    
``const MSK_RES_ERR_QCON_SUBI_TOO_SMALL                        : i32 = 1408``
    
``const MSK_RES_ERR_QCON_UPPER_TRIANGLE                        : i32 = 1417``
    
``const MSK_RES_ERR_QOBJ_UPPER_TRIANGLE                        : i32 = 1415``
    
``const MSK_RES_ERR_READ_FORMAT                                : i32 = 1090``
    
``const MSK_RES_ERR_READ_LP_MISSING_END_TAG                    : i32 = 1159``
    
``const MSK_RES_ERR_READ_LP_NONEXISTING_NAME                   : i32 = 1162``
    
``const MSK_RES_ERR_REMOVE_CONE_VARIABLE                       : i32 = 1310``
    
``const MSK_RES_ERR_REPAIR_INVALID_PROBLEM                     : i32 = 1710``
    
``const MSK_RES_ERR_REPAIR_OPTIMIZATION_FAILED                 : i32 = 1711``
    
``const MSK_RES_ERR_SEN_BOUND_INVALID_LO                       : i32 = 3054``
    
``const MSK_RES_ERR_SEN_BOUND_INVALID_UP                       : i32 = 3053``
    
``const MSK_RES_ERR_SEN_FORMAT                                 : i32 = 3050``
    
``const MSK_RES_ERR_SEN_INDEX_INVALID                          : i32 = 3055``
    
``const MSK_RES_ERR_SEN_INDEX_RANGE                            : i32 = 3052``
    
``const MSK_RES_ERR_SEN_INVALID_REGEXP                         : i32 = 3056``
    
``const MSK_RES_ERR_SEN_NUMERICAL                              : i32 = 3058``
    
``const MSK_RES_ERR_SEN_SOLUTION_STATUS                        : i32 = 3057``
    
``const MSK_RES_ERR_SEN_UNDEF_NAME                             : i32 = 3051``
    
``const MSK_RES_ERR_SEN_UNHANDLED_PROBLEM_TYPE                 : i32 = 3080``
    
``const MSK_RES_ERR_SERVER_CONNECT                             : i32 = 8000``
    
``const MSK_RES_ERR_SERVER_PROBLEM_SIZE                        : i32 = 8008``
    
``const MSK_RES_ERR_SERVER_PROTOCOL                            : i32 = 8001``
    
``const MSK_RES_ERR_SERVER_STATUS                              : i32 = 8002``
    
``const MSK_RES_ERR_SERVER_TOKEN                               : i32 = 8003``
    
``const MSK_RES_ERR_SHAPE_IS_TOO_LARGE                         : i32 = 1202``
    
``const MSK_RES_ERR_SIZE_LICENSE                               : i32 = 1005``
    
``const MSK_RES_ERR_SIZE_LICENSE_CON                           : i32 = 1010``
    
``const MSK_RES_ERR_SIZE_LICENSE_INTVAR                        : i32 = 1012``
    
``const MSK_RES_ERR_SIZE_LICENSE_NUMCORES                      : i32 = 3900``
    
``const MSK_RES_ERR_SIZE_LICENSE_VAR                           : i32 = 1011``
    
``const MSK_RES_ERR_SLICE_SIZE                                 : i32 = 1572``
    
``const MSK_RES_ERR_SOL_FILE_INVALID_NUMBER                    : i32 = 1350``
    
``const MSK_RES_ERR_SOLITEM                                    : i32 = 1237``
    
``const MSK_RES_ERR_SOLVER_PROBTYPE                            : i32 = 1259``
    
``const MSK_RES_ERR_SPACE                                      : i32 = 1051``
    
``const MSK_RES_ERR_SPACE_LEAKING                              : i32 = 1080``
    
``const MSK_RES_ERR_SPACE_NO_INFO                              : i32 = 1081``
    
``const MSK_RES_ERR_SYM_MAT_DUPLICATE                          : i32 = 3944``
    
``const MSK_RES_ERR_SYM_MAT_HUGE                               : i32 = 1482``
    
``const MSK_RES_ERR_SYM_MAT_INVALID                            : i32 = 1480``
    
``const MSK_RES_ERR_SYM_MAT_INVALID_COL_INDEX                  : i32 = 3941``
    
``const MSK_RES_ERR_SYM_MAT_INVALID_ROW_INDEX                  : i32 = 3940``
    
``const MSK_RES_ERR_SYM_MAT_INVALID_VALUE                      : i32 = 3943``
    
``const MSK_RES_ERR_SYM_MAT_NOT_LOWER_TRINGULAR                : i32 = 3942``
    
``const MSK_RES_ERR_TASK_INCOMPATIBLE                          : i32 = 2560``
    
``const MSK_RES_ERR_TASK_INVALID                               : i32 = 2561``
    
``const MSK_RES_ERR_TASK_WRITE                                 : i32 = 2562``
    
``const MSK_RES_ERR_THREAD_COND_INIT                           : i32 = 1049``
    
``const MSK_RES_ERR_THREAD_CREATE                              : i32 = 1048``
    
``const MSK_RES_ERR_THREAD_MUTEX_INIT                          : i32 = 1045``
    
``const MSK_RES_ERR_THREAD_MUTEX_LOCK                          : i32 = 1046``
    
``const MSK_RES_ERR_THREAD_MUTEX_UNLOCK                        : i32 = 1047``
    
``const MSK_RES_ERR_TOCONIC_CONSTR_NOT_CONIC                   : i32 = 7803``
    
``const MSK_RES_ERR_TOCONIC_CONSTR_Q_NOT_PSD                   : i32 = 7800``
    
``const MSK_RES_ERR_TOCONIC_CONSTRAINT_FX                      : i32 = 7801``
    
``const MSK_RES_ERR_TOCONIC_CONSTRAINT_RA                      : i32 = 7802``
    
``const MSK_RES_ERR_TOCONIC_OBJECTIVE_NOT_PSD                  : i32 = 7804``
    
``const MSK_RES_ERR_TOO_SMALL_A_TRUNCATION_VALUE               : i32 = 1421``
    
``const MSK_RES_ERR_TOO_SMALL_MAX_NUM_NZ                       : i32 = 1245``
    
``const MSK_RES_ERR_TOO_SMALL_MAXNUMANZ                        : i32 = 1252``
    
``const MSK_RES_ERR_UNB_STEP_SIZE                              : i32 = 3100``
    
``const MSK_RES_ERR_UNDEF_SOLUTION                             : i32 = 1265``
    
``const MSK_RES_ERR_UNDEFINED_OBJECTIVE_SENSE                  : i32 = 1446``
    
``const MSK_RES_ERR_UNHANDLED_SOLUTION_STATUS                  : i32 = 6010``
    
``const MSK_RES_ERR_UNKNOWN                                    : i32 = 1050``
    
``const MSK_RES_ERR_UPPER_BOUND_IS_A_NAN                       : i32 = 1391``
    
``const MSK_RES_ERR_UPPER_TRIANGLE                             : i32 = 6020``
    
``const MSK_RES_ERR_WHICHITEM_NOT_ALLOWED                      : i32 = 1238``
    
``const MSK_RES_ERR_WHICHSOL                                   : i32 = 1236``
    
``const MSK_RES_ERR_WRITE_LP_FORMAT                            : i32 = 1158``
    
``const MSK_RES_ERR_WRITE_LP_NON_UNIQUE_NAME                   : i32 = 1161``
    
``const MSK_RES_ERR_WRITE_MPS_INVALID_NAME                     : i32 = 1153``
    
``const MSK_RES_ERR_WRITE_OPF_INVALID_VAR_NAME                 : i32 = 1156``
    
``const MSK_RES_ERR_WRITING_FILE                               : i32 = 1166``
    
``const MSK_RES_ERR_XML_INVALID_PROBLEM_TYPE                   : i32 = 3600``
    
``const MSK_RES_ERR_Y_IS_UNDEFINED                             : i32 = 1449``
    
``const MSK_RES_OK                                             : i32 = 0``
    
``const MSK_RES_TRM_INTERNAL                                   : i32 = 10030``
    
``const MSK_RES_TRM_INTERNAL_STOP                              : i32 = 10031``
    
``const MSK_RES_TRM_MAX_ITERATIONS                             : i32 = 10000``
    
``const MSK_RES_TRM_MAX_NUM_SETBACKS                           : i32 = 10020``
    
``const MSK_RES_TRM_MAX_TIME                                   : i32 = 10001``
    
``const MSK_RES_TRM_MIO_NUM_BRANCHES                           : i32 = 10009``
    
``const MSK_RES_TRM_MIO_NUM_RELAXS                             : i32 = 10008``
    
``const MSK_RES_TRM_NUM_MAX_NUM_INT_SOLUTIONS                  : i32 = 10015``
    
``const MSK_RES_TRM_NUMERICAL_PROBLEM                          : i32 = 10025``
    
``const MSK_RES_TRM_OBJECTIVE_RANGE                            : i32 = 10002``
    
``const MSK_RES_TRM_STALL                                      : i32 = 10006``
    
``const MSK_RES_TRM_USER_CALLBACK                              : i32 = 10007``
    
``const MSK_RES_WRN_ANA_ALMOST_INT_BOUNDS                      : i32 = 904``
    
``const MSK_RES_WRN_ANA_C_ZERO                                 : i32 = 901``
    
``const MSK_RES_WRN_ANA_CLOSE_BOUNDS                           : i32 = 903``
    
``const MSK_RES_WRN_ANA_EMPTY_COLS                             : i32 = 902``
    
``const MSK_RES_WRN_ANA_LARGE_BOUNDS                           : i32 = 900``
    
``const MSK_RES_WRN_DROPPED_NZ_QOBJ                            : i32 = 201``
    
``const MSK_RES_WRN_DUPLICATE_BARVARIABLE_NAMES                : i32 = 852``
    
``const MSK_RES_WRN_DUPLICATE_CONE_NAMES                       : i32 = 853``
    
``const MSK_RES_WRN_DUPLICATE_CONSTRAINT_NAMES                 : i32 = 850``
    
``const MSK_RES_WRN_DUPLICATE_VARIABLE_NAMES                   : i32 = 851``
    
``const MSK_RES_WRN_ELIMINATOR_SPACE                           : i32 = 801``
    
``const MSK_RES_WRN_EMPTY_NAME                                 : i32 = 502``
    
``const MSK_RES_WRN_EXP_CONES_WITH_VARIABLES_FIXED_AT_ZERO     : i32 = 932``
    
``const MSK_RES_WRN_IGNORE_INTEGER                             : i32 = 250``
    
``const MSK_RES_WRN_INCOMPLETE_LINEAR_DEPENDENCY_CHECK         : i32 = 800``
    
``const MSK_RES_WRN_LARGE_AIJ                                  : i32 = 62``
    
``const MSK_RES_WRN_LARGE_BOUND                                : i32 = 51``
    
``const MSK_RES_WRN_LARGE_CJ                                   : i32 = 57``
    
``const MSK_RES_WRN_LARGE_CON_FX                               : i32 = 54``
    
``const MSK_RES_WRN_LARGE_LO_BOUND                             : i32 = 52``
    
``const MSK_RES_WRN_LARGE_UP_BOUND                             : i32 = 53``
    
``const MSK_RES_WRN_LICENSE_EXPIRE                             : i32 = 500``
    
``const MSK_RES_WRN_LICENSE_FEATURE_EXPIRE                     : i32 = 505``
    
``const MSK_RES_WRN_LICENSE_SERVER                             : i32 = 501``
    
``const MSK_RES_WRN_LP_DROP_VARIABLE                           : i32 = 85``
    
``const MSK_RES_WRN_LP_OLD_QUAD_FORMAT                         : i32 = 80``
    
``const MSK_RES_WRN_MIO_INFEASIBLE_FINAL                       : i32 = 270``
    
``const MSK_RES_WRN_MPS_SPLIT_BOU_VECTOR                       : i32 = 72``
    
``const MSK_RES_WRN_MPS_SPLIT_RAN_VECTOR                       : i32 = 71``
    
``const MSK_RES_WRN_MPS_SPLIT_RHS_VECTOR                       : i32 = 70``
    
``const MSK_RES_WRN_NAME_MAX_LEN                               : i32 = 65``
    
``const MSK_RES_WRN_NO_DUALIZER                                : i32 = 950``
    
``const MSK_RES_WRN_NO_GLOBAL_OPTIMIZER                        : i32 = 251``
    
``const MSK_RES_WRN_NZ_IN_UPR_TRI                              : i32 = 200``
    
``const MSK_RES_WRN_OPEN_PARAM_FILE                            : i32 = 50``
    
``const MSK_RES_WRN_PARAM_IGNORED_CMIO                         : i32 = 516``
    
``const MSK_RES_WRN_PARAM_NAME_DOU                             : i32 = 510``
    
``const MSK_RES_WRN_PARAM_NAME_INT                             : i32 = 511``
    
``const MSK_RES_WRN_PARAM_NAME_STR                             : i32 = 512``
    
``const MSK_RES_WRN_PARAM_STR_VALUE                            : i32 = 515``
    
``const MSK_RES_WRN_POW_CONES_WITH_ROOT_FIXED_AT_ZERO          : i32 = 933``
    
``const MSK_RES_WRN_PRESOLVE_OUTOFSPACE                        : i32 = 802``
    
``const MSK_RES_WRN_QUAD_CONES_WITH_ROOT_FIXED_AT_ZERO         : i32 = 930``
    
``const MSK_RES_WRN_RQUAD_CONES_WITH_ROOT_FIXED_AT_ZERO        : i32 = 931``
    
``const MSK_RES_WRN_SOL_FILE_IGNORED_CON                       : i32 = 351``
    
``const MSK_RES_WRN_SOL_FILE_IGNORED_VAR                       : i32 = 352``
    
``const MSK_RES_WRN_SOL_FILTER                                 : i32 = 300``
    
``const MSK_RES_WRN_SPAR_MAX_LEN                               : i32 = 66``
    
``const MSK_RES_WRN_SYM_MAT_LARGE                              : i32 = 960``
    
``const MSK_RES_WRN_TOO_FEW_BASIS_VARS                         : i32 = 400``
    
``const MSK_RES_WRN_TOO_MANY_BASIS_VARS                        : i32 = 405``
    
``const MSK_RES_WRN_UNDEF_SOL_FILE_NAME                        : i32 = 350``
    
``const MSK_RES_WRN_USING_GENERIC_NAMES                        : i32 = 503``
    
``const MSK_RES_WRN_WRITE_CHANGED_NAMES                        : i32 = 803``
    
``const MSK_RES_WRN_WRITE_DISCARDED_CFIX                       : i32 = 804``
    
``const MSK_RES_WRN_ZERO_AIJ                                   : i32 = 63``
    
``const MSK_RES_WRN_ZEROS_IN_SPARSE_COL                        : i32 = 710``
    
``const MSK_RES_WRN_ZEROS_IN_SPARSE_ROW                        : i32 = 705``
    
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



``const MSK_RESPONSE_ERR : i32 = 3``
    
``const MSK_RESPONSE_OK  : i32 = 0``
    
``const MSK_RESPONSE_TRM : i32 = 2``
    
``const MSK_RESPONSE_UNK : i32 = 4``
    
``const MSK_RESPONSE_WRN : i32 = 1``
    
.. index:: scalingmethod
.. index:: SCALING_METHOD_...
.. _enum_scalingmethod:
.. _scalingmethod_free:
.. _scalingmethod_pow2:

``scalingmethod``
-----------------



``const MSK_SCALING_METHOD_FREE : i32 = 1``
    
``const MSK_SCALING_METHOD_POW2 : i32 = 0``
    
.. index:: scalingtype
.. index:: SCALING_...
.. _enum_scalingtype:
.. _scalingtype_aggressive:
.. _scalingtype_free:
.. _scalingtype_moderate:
.. _scalingtype_none:

``scalingtype``
---------------



``const MSK_SCALING_AGGRESSIVE : i32 = 3``
    
``const MSK_SCALING_FREE       : i32 = 0``
    
``const MSK_SCALING_MODERATE   : i32 = 2``
    
``const MSK_SCALING_NONE       : i32 = 1``
    
.. index:: scopr
.. index:: OPR_...
.. _enum_scopr:
.. _scopr_ent:
.. _scopr_exp:
.. _scopr_log:
.. _scopr_pow:
.. _scopr_sqrt:

``scopr``
---------



``const MSK_OPR_ENT  : i32 = 0``
    
``const MSK_OPR_EXP  : i32 = 1``
    
``const MSK_OPR_LOG  : i32 = 2``
    
``const MSK_OPR_POW  : i32 = 3``
    
``const MSK_OPR_SQRT : i32 = 4``
    
.. index:: sensitivitytype
.. index:: SENSITIVITY_TYPE_...
.. _enum_sensitivitytype:
.. _sensitivitytype_basis:

``sensitivitytype``
-------------------



``const MSK_SENSITIVITY_TYPE_BASIS : i32 = 0``
    
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



``const MSK_SIM_DEGEN_AGGRESSIVE : i32 = 2``
    
``const MSK_SIM_DEGEN_FREE       : i32 = 1``
    
``const MSK_SIM_DEGEN_MINIMUM    : i32 = 4``
    
``const MSK_SIM_DEGEN_MODERATE   : i32 = 3``
    
``const MSK_SIM_DEGEN_NONE       : i32 = 0``
    
.. index:: simdupvec
.. index:: SIM_EXPLOIT_DUPVEC_...
.. _enum_simdupvec:
.. _simdupvec_free:
.. _simdupvec_off:
.. _simdupvec_on:

``simdupvec``
-------------



``const MSK_SIM_EXPLOIT_DUPVEC_FREE : i32 = 2``
    
``const MSK_SIM_EXPLOIT_DUPVEC_OFF  : i32 = 0``
    
``const MSK_SIM_EXPLOIT_DUPVEC_ON   : i32 = 1``
    
.. index:: simhotstart
.. index:: SIM_HOTSTART_...
.. _enum_simhotstart:
.. _simhotstart_free:
.. _simhotstart_none:
.. _simhotstart_status_keys:

``simhotstart``
---------------



``const MSK_SIM_HOTSTART_FREE        : i32 = 1``
    
``const MSK_SIM_HOTSTART_NONE        : i32 = 0``
    
``const MSK_SIM_HOTSTART_STATUS_KEYS : i32 = 2``
    
.. index:: simreform
.. index:: SIM_REFORMULATION_...
.. _enum_simreform:
.. _simreform_aggressive:
.. _simreform_free:
.. _simreform_off:
.. _simreform_on:

``simreform``
-------------



``const MSK_SIM_REFORMULATION_AGGRESSIVE : i32 = 3``
    
``const MSK_SIM_REFORMULATION_FREE       : i32 = 2``
    
``const MSK_SIM_REFORMULATION_OFF        : i32 = 0``
    
``const MSK_SIM_REFORMULATION_ON         : i32 = 1``
    
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



``const MSK_SIM_SELECTION_ASE     : i32 = 2``
    
``const MSK_SIM_SELECTION_DEVEX   : i32 = 3``
    
``const MSK_SIM_SELECTION_FREE    : i32 = 0``
    
``const MSK_SIM_SELECTION_FULL    : i32 = 1``
    
``const MSK_SIM_SELECTION_PARTIAL : i32 = 5``
    
``const MSK_SIM_SELECTION_SE      : i32 = 4``
    
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



``const MSK_SOL_ITEM_SLC : i32 = 3``
    
``const MSK_SOL_ITEM_SLX : i32 = 5``
    
``const MSK_SOL_ITEM_SNX : i32 = 7``
    
``const MSK_SOL_ITEM_SUC : i32 = 4``
    
``const MSK_SOL_ITEM_SUX : i32 = 6``
    
``const MSK_SOL_ITEM_XC  : i32 = 0``
    
``const MSK_SOL_ITEM_XX  : i32 = 1``
    
``const MSK_SOL_ITEM_Y   : i32 = 2``
    
.. index:: solsta
.. index:: SOL_STA_...
.. _enum_solsta:
.. _solsta_dual_feas:
.. _solsta_dual_illposed_cer:
.. _solsta_dual_infeas_cer:
.. _solsta_integer_optimal:
.. _solsta_optimal:
.. _solsta_prim_and_dual_feas:
.. _solsta_prim_feas:
.. _solsta_prim_illposed_cer:
.. _solsta_prim_infeas_cer:
.. _solsta_unknown:

``solsta``
----------



``const MSK_SOL_STA_DUAL_FEAS          : i32 = 3``
    
``const MSK_SOL_STA_DUAL_ILLPOSED_CER  : i32 = 8``
    
``const MSK_SOL_STA_DUAL_INFEAS_CER    : i32 = 6``
    
``const MSK_SOL_STA_INTEGER_OPTIMAL    : i32 = 9``
    
``const MSK_SOL_STA_OPTIMAL            : i32 = 1``
    
``const MSK_SOL_STA_PRIM_AND_DUAL_FEAS : i32 = 4``
    
``const MSK_SOL_STA_PRIM_FEAS          : i32 = 2``
    
``const MSK_SOL_STA_PRIM_ILLPOSED_CER  : i32 = 7``
    
``const MSK_SOL_STA_PRIM_INFEAS_CER    : i32 = 5``
    
``const MSK_SOL_STA_UNKNOWN            : i32 = 0``
    
.. index:: soltype
.. index:: SOL_...
.. _enum_soltype:
.. _soltype_bas:
.. _soltype_itg:
.. _soltype_itr:

``soltype``
-----------



``const MSK_SOL_BAS : i32 = 1``
    
``const MSK_SOL_ITG : i32 = 2``
    
``const MSK_SOL_ITR : i32 = 0``
    
.. index:: solveform
.. index:: SOLVE_...
.. _enum_solveform:
.. _solveform_dual:
.. _solveform_free:
.. _solveform_primal:

``solveform``
-------------



``const MSK_SOLVE_DUAL   : i32 = 2``
    
``const MSK_SOLVE_FREE   : i32 = 0``
    
``const MSK_SOLVE_PRIMAL : i32 = 1``
    
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



``const MSK_SPAR_BAS_SOL_FILE_NAME         : i32 = 0``
    
``const MSK_SPAR_DATA_FILE_NAME            : i32 = 1``
    
``const MSK_SPAR_DEBUG_FILE_NAME           : i32 = 2``
    
``const MSK_SPAR_INT_SOL_FILE_NAME         : i32 = 3``
    
``const MSK_SPAR_ITR_SOL_FILE_NAME         : i32 = 4``
    
``const MSK_SPAR_MIO_DEBUG_STRING          : i32 = 5``
    
``const MSK_SPAR_PARAM_COMMENT_SIGN        : i32 = 6``
    
``const MSK_SPAR_PARAM_READ_FILE_NAME      : i32 = 7``
    
``const MSK_SPAR_PARAM_WRITE_FILE_NAME     : i32 = 8``
    
``const MSK_SPAR_READ_MPS_BOU_NAME         : i32 = 9``
    
``const MSK_SPAR_READ_MPS_OBJ_NAME         : i32 = 10``
    
``const MSK_SPAR_READ_MPS_RAN_NAME         : i32 = 11``
    
``const MSK_SPAR_READ_MPS_RHS_NAME         : i32 = 12``
    
``const MSK_SPAR_REMOTE_ACCESS_TOKEN       : i32 = 13``
    
``const MSK_SPAR_SENSITIVITY_FILE_NAME     : i32 = 14``
    
``const MSK_SPAR_SENSITIVITY_RES_FILE_NAME : i32 = 15``
    
``const MSK_SPAR_SOL_FILTER_XC_LOW         : i32 = 16``
    
``const MSK_SPAR_SOL_FILTER_XC_UPR         : i32 = 17``
    
``const MSK_SPAR_SOL_FILTER_XX_LOW         : i32 = 18``
    
``const MSK_SPAR_SOL_FILTER_XX_UPR         : i32 = 19``
    
``const MSK_SPAR_STAT_FILE_NAME            : i32 = 20``
    
``const MSK_SPAR_STAT_KEY                  : i32 = 21``
    
``const MSK_SPAR_STAT_NAME                 : i32 = 22``
    
``const MSK_SPAR_WRITE_LP_GEN_VAR_NAME     : i32 = 23``
    
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



``const MSK_SK_BAS    : i32 = 1``
    
``const MSK_SK_FIX    : i32 = 5``
    
``const MSK_SK_INF    : i32 = 6``
    
``const MSK_SK_LOW    : i32 = 3``
    
``const MSK_SK_SUPBAS : i32 = 2``
    
``const MSK_SK_UNK    : i32 = 0``
    
``const MSK_SK_UPR    : i32 = 4``
    
.. index:: startpointtype
.. index:: STARTING_POINT_...
.. _enum_startpointtype:
.. _startpointtype_constant:
.. _startpointtype_free:
.. _startpointtype_guess:
.. _startpointtype_satisfy_bounds:

``startpointtype``
------------------



``const MSK_STARTING_POINT_CONSTANT       : i32 = 2``
    
``const MSK_STARTING_POINT_FREE           : i32 = 0``
    
``const MSK_STARTING_POINT_GUESS          : i32 = 1``
    
``const MSK_STARTING_POINT_SATISFY_BOUNDS : i32 = 3``
    
.. index:: streamtype
.. index:: STREAM_...
.. _enum_streamtype:
.. _streamtype_err:
.. _streamtype_log:
.. _streamtype_msg:
.. _streamtype_wrn:

``streamtype``
--------------



``const MSK_STREAM_ERR : i32 = 2``
    
``const MSK_STREAM_LOG : i32 = 0``
    
``const MSK_STREAM_MSG : i32 = 1``
    
``const MSK_STREAM_WRN : i32 = 3``
    
.. index:: symmattype
.. index:: SYMMAT_TYPE_...
.. _enum_symmattype:
.. _symmattype_sparse:

``symmattype``
--------------



``const MSK_SYMMAT_TYPE_SPARSE : i32 = 0``
    
.. index:: transpose
.. index:: TRANSPOSE_...
.. _enum_transpose:
.. _transpose_no:
.. _transpose_yes:

``transpose``
-------------



``const MSK_TRANSPOSE_NO  : i32 = 0``
    
``const MSK_TRANSPOSE_YES : i32 = 1``
    
.. index:: uplo
.. index:: UPLO_...
.. _enum_uplo:
.. _uplo_lo:
.. _uplo_up:

``uplo``
--------



``const MSK_UPLO_LO : i32 = 0``
    
``const MSK_UPLO_UP : i32 = 1``
    
.. index:: value
.. index:: ...
.. _enum_value:
.. _value_license_buffer_length:
.. _value_max_str_len:

``value``
---------



``const MSK_LICENSE_BUFFER_LENGTH : i32 = 21``
    
``const MSK_MAX_STR_LEN           : i32 = 1024``
    
.. index:: variabletype
.. index:: VAR_...
.. _enum_variabletype:
.. _variabletype_type_cont:
.. _variabletype_type_int:

``variabletype``
----------------



``const MSK_VAR_TYPE_CONT : i32 = 0``
    
``const MSK_VAR_TYPE_INT  : i32 = 1``
    
.. index:: xmlwriteroutputtype
.. index:: WRITE_XML_MODE_...
.. _enum_xmlwriteroutputtype:
.. _xmlwriteroutputtype_col:
.. _xmlwriteroutputtype_row:

``xmlwriteroutputtype``
-----------------------



``const MSK_WRITE_XML_MODE_COL : i32 = 1``
    
``const MSK_WRITE_XML_MODE_ROW : i32 = 0``
    
