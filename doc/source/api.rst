
.. |mosek| replace:: MOSEK
.. |null| replace:: ``None``

Module level functions
======================

.. index:: callback_code_to_str

.. _optimizer_env_callbackcodetostr:

``callback_code_to_str()``
--------------------------

.. code-block:: rust

    pub fn callback_code_to_str ( code : i32 ) -> Result<String,String>

``code``
    
*Returns:* ``callbackcodestr``
    ``callbackcodestr : String``
        


.. index:: get_build_info

.. _optimizer_env_getbuildinfo:

``get_build_info()``
--------------------

.. code-block:: rust

    pub fn get_build_info (  ) -> Result<(String,String),String>

*Returns:* ``(buildstate,builddate)``
    ``buildstate : String``
        
    ``builddate : String``
        


.. index:: get_code_desc

.. _optimizer_env_getcodedesc:

``get_code_desc()``
-------------------

.. code-block:: rust

    pub fn get_code_desc ( code : i32 ) -> Result<(String,String),String>

``code``
    
*Returns:* ``(symname,str)``
    ``symname : String``
        
    ``str : String``
        


.. index:: get_response_class

.. _optimizer_env_getresponseclass:

``get_response_class()``
------------------------

.. code-block:: rust

    pub fn get_response_class ( r : i32 ) -> Result<i32,String>

``r``
    
*Returns:* ``rc``
    ``rc : i32``
        


.. index:: get_version

.. _optimizer_env_getversion:

``get_version()``
-----------------

.. code-block:: rust

    pub fn get_version (  ) -> Result<(i32,i32,i32),String>

*Returns:* ``(major,minor,revision)``
    ``major : i32``
        
    ``minor : i32``
        
    ``revision : i32``
        


.. index:: is_infinity

.. _optimizer_env_isinfinity:

``is_infinity()``
-----------------

.. code-block:: rust

    pub fn is_infinity ( value : f64 ) -> Result<(),String>

``value``
    


.. index:: licensecleanup

.. _optimizer_env_licensecleanup:

``licensecleanup()``
--------------------

.. code-block:: rust

    pub fn licensecleanup (  ) -> Result<(),String>



.. index:: sym_nam_to_value

.. _optimizer_env_symnamtovalue:

``sym_nam_to_value()``
----------------------

.. code-block:: rust

    pub fn sym_nam_to_value ( name : &str ) -> Result<String,String>

``name``
    
*Returns:* ``value``
    ``value : String``
        

Env methods
===========

.. index:: axpy

.. _optimizer_env_axpy:

``axpy()``
----------

.. code-block:: rust

    pub fn axpy ( &self,
                  n     : i32,
                  alpha : f64,
                  x_    : & [f64],
                  y     : & mut [f64]) -> Result<(),String>

``n``
    
``alpha``
    
``x``
    
``y``
    


.. index:: check_in_all

.. _optimizer_env_checkinall:

``check_in_all()``
------------------

.. code-block:: rust

    pub fn check_in_all ( &self ) -> Result<(),String>



.. index:: check_in_license

.. _optimizer_env_checkinlicense:

``check_in_license()``
----------------------

.. code-block:: rust

    pub fn check_in_license ( &self,feature : i32 ) -> Result<(),String>

``feature``
    


.. index:: check_mem

.. _optimizer_env_checkmemenv:

``check_mem()``
---------------

.. code-block:: rust

    pub fn check_mem ( &self,
                       file  : &str,
                       line  : i32) -> Result<(),String>

``file``
    
``line``
    


.. index:: check_version

.. _optimizer_env_checkversion:

``check_version()``
-------------------

.. code-block:: rust

    pub fn check_version ( &self,
                           major    : i32,
                           minor    : i32,
                           revision : i32) -> Result<(),String>

``major``
    
``minor``
    
``revision``
    


.. index:: checkout_license

.. _optimizer_env_checkoutlicense:

``checkout_license()``
----------------------

.. code-block:: rust

    pub fn checkout_license ( &self,feature : i32 ) -> Result<(),String>

``feature``
    


.. index:: dot

.. _optimizer_env_dot:

``dot()``
---------

.. code-block:: rust

    pub fn dot ( &self,
                 n     : i32,
                 x_    : & [f64],
                 y_    : & [f64]) -> Result<f64,String>

``n``
    
``x``
    
``y``
    
*Returns:* ``xty``
    ``xty : f64``
        


.. index:: echo_env

.. _optimizer_env_echoenv:

``echo_env()``
--------------

.. code-block:: rust

    pub fn echo_env ( &self,
                      whichstream : i32,
                      format      : &str) -> Result<(),String>

``whichstream``
    
``format``
    


.. index:: echo_intro

.. _optimizer_env_echointro:

``echo_intro()``
----------------

.. code-block:: rust

    pub fn echo_intro ( &self,longver : i32 ) -> Result<(),String>

``longver``
    


.. index:: gemm

.. _optimizer_env_gemm:

``gemm()``
----------

.. code-block:: rust

    pub fn gemm ( &self,
                  transa : i32,
                  transb : i32,
                  m      : i32,
                  n      : i32,
                  k      : i32,
                  alpha  : f64,
                  a_     : & [f64],
                  b_     : & [f64],
                  beta   : f64,
                  c      : & mut [f64]) -> Result<(),String>

``transa``
    
``transb``
    
``m``
    
``n``
    
``k``
    
``alpha``
    
``a``
    
``b``
    
``beta``
    
``c``
    


.. index:: gemv

.. _optimizer_env_gemv:

``gemv()``
----------

.. code-block:: rust

    pub fn gemv ( &self,
                  transa : i32,
                  m      : i32,
                  n      : i32,
                  alpha  : f64,
                  a_     : & [f64],
                  x_     : & [f64],
                  beta   : f64,
                  y      : & mut [f64]) -> Result<(),String>

``transa``
    
``m``
    
``n``
    
``alpha``
    
``a``
    
``x``
    
``beta``
    
``y``
    


.. index:: link_file_to_env_stream

.. _optimizer_env_linkfiletoenvstream:

``link_file_to_env_stream()``
-----------------------------

.. code-block:: rust

    pub fn link_file_to_env_stream ( &self,
                                     whichstream : i32,
                                     filename    : &str,
                                     append      : i32) -> Result<(),String>

``whichstream``
    
``filename``
    
``append``
    


.. index:: new

``new()``
---------

.. code-block:: rust

    pub fn new() -> Env

*Returns* : ``env : Env``
    Returns a new environment instance.

.. code-block:: rust

    pub fn new_mem_debug(dbgfile : &str) -> Env

``dbgfile``
    Filename where MOSEK will dump memory debug information.
*Returns* : ``env : Env``
    Returns a new environment instance. 

.. index:: potrf

.. _optimizer_env_potrf:

``potrf()``
-----------

.. code-block:: rust

    pub fn potrf ( &self,
                   uplo  : i32,
                   n     : i32,
                   a     : & mut [f64]) -> Result<(),String>

``uplo``
    
``n``
    
``a``
    


.. index:: put_license_code

.. _optimizer_env_putlicensecode:

``put_license_code()``
----------------------

.. code-block:: rust

    pub fn put_license_code ( &self,code_ : & [i32] ) -> Result<(),String>

``code``
    


.. index:: put_license_debug

.. _optimizer_env_putlicensedebug:

``put_license_debug()``
-----------------------

.. code-block:: rust

    pub fn put_license_debug ( &self,licdebug : i32 ) -> Result<(),String>

``licdebug``
    


.. index:: put_license_path

.. _optimizer_env_putlicensepath:

``put_license_path()``
----------------------

.. code-block:: rust

    pub fn put_license_path ( &self,licensepath : &str ) -> Result<(),String>

``licensepath``
    


.. index:: put_license_wait

.. _optimizer_env_putlicensewait:

``put_license_wait()``
----------------------

.. code-block:: rust

    pub fn put_license_wait ( &self,licwait : i32 ) -> Result<(),String>

``licwait``
    


.. index:: setup_threads

.. _optimizer_env_setupthreads:

``setup_threads()``
-------------------

.. code-block:: rust

    pub fn setup_threads ( &self,numthreads : i32 ) -> Result<(),String>

``numthreads``
    


.. index:: sparse_triangular_solve_dense

.. _optimizer_env_sparsetriangularsolvedense:

``sparse_triangular_solve_dense()``
-----------------------------------

.. code-block:: rust

    pub fn sparse_triangular_solve_dense ( &self,
                                           transposed : i32,
                                           lnzc_      : & [i32],
                                           lptrc_     : & [i64],
                                           lsubc_     : & [i32],
                                           lvalc_     : & [f64],
                                           b          : & mut [f64]) -> Result<(),String>

``transposed``
    
``lnzc``
    
``lptrc``
    
``lsubc``
    
``lvalc``
    
``b``
    


.. index:: syeig

.. _optimizer_env_syeig:

``syeig()``
-----------

.. code-block:: rust

    pub fn syeig ( &self,
                   uplo  : i32,
                   n     : i32,
                   a_    : & [f64],
                   w     : & mut [f64]) -> Result<(),String>

``uplo``
    
``n``
    
``a``
    
``w``
    


.. index:: syevd

.. _optimizer_env_syevd:

``syevd()``
-----------

.. code-block:: rust

    pub fn syevd ( &self,
                   uplo  : i32,
                   n     : i32,
                   a     : & mut [f64],
                   w     : & mut [f64]) -> Result<(),String>

``uplo``
    
``n``
    
``a``
    
``w``
    


.. index:: syrk

.. _optimizer_env_syrk:

``syrk()``
----------

.. code-block:: rust

    pub fn syrk ( &self,
                  uplo  : i32,
                  trans : i32,
                  n     : i32,
                  k     : i32,
                  alpha : f64,
                  a_    : & [f64],
                  beta  : f64,
                  c     : & mut [f64]) -> Result<(),String>

``uplo``
    
``trans``
    
``n``
    
``k``
    
``alpha``
    
``a``
    
``beta``
    
``c``
    


.. index:: task

``task()``
----------

.. code-block:: rust

    pub fn task<H>(&self) -> Task<H>

*Returns:*
    Returns a new task. The type ``H`` is the type passed to callback
    functions. If you do not intend to use callback functions, you can
    let this be ``()`` (unit).


.. index:: task_with_capacity

``task_with_capacity()``
------------------------

.. code-block:: rust

    pub fn task<H>(&self,numcon : i32, numvar : i32) -> Task<H>

``numcon``
    Reserve space for this many columns.
``numvar``
    Reserve space for this many rows.
*Returns:*
    Returns a new task. The type ``H`` is the type passed to callback
    functions. If you do not intend to use callback functions, you can
    let this be ``()`` (unit).


.. index:: unlink_func_from_stream

.. _optimizer_env_unlinkfuncfromenvstream:

``unlink_func_from_stream()``
-----------------------------

.. code-block:: rust

    pub fn unlink_func_from_stream ( &self,whichstream : i32 ) -> Result<(),String>

``whichstream``
    

Task methods
============

.. index:: analyze_names

.. _optimizer_task_analyzenames:

``analyze_names()``
-------------------

.. code-block:: rust

    pub fn analyze_names ( &self,
                           whichstream : i32,
                           nametype    : i32) -> Result<(),String>

``whichstream``
    
``nametype``
    


.. index:: analyze_problem

.. _optimizer_task_analyzeproblem:

``analyze_problem()``
---------------------

.. code-block:: rust

    pub fn analyze_problem ( &self,whichstream : i32 ) -> Result<(),String>

``whichstream``
    


.. index:: analyze_solution

.. _optimizer_task_analyzesolution:

``analyze_solution()``
----------------------

.. code-block:: rust

    pub fn analyze_solution ( &self,
                              whichstream : i32,
                              whichsol    : i32) -> Result<(),String>

``whichstream``
    
``whichsol``
    


.. index:: append_barvars

.. _optimizer_task_appendbarvars:

``append_barvars()``
--------------------

.. code-block:: rust

    pub fn append_barvars ( &self,dim_  : & [i32] ) -> Result<(),String>

``dim``
    


.. index:: append_cone

.. _optimizer_task_appendcone:

``append_cone()``
-----------------

.. code-block:: rust

    pub fn append_cone ( &self,
                         ct      : i32,
                         conepar : f64,
                         submem_ : & [i32]) -> Result<(),String>

``ct``
    
``conepar``
    
``submem``
    


.. index:: append_cone_seq

.. _optimizer_task_appendconeseq:

``append_cone_seq()``
---------------------

.. code-block:: rust

    pub fn append_cone_seq ( &self,
                             ct      : i32,
                             conepar : f64,
                             nummem  : i32,
                             j       : i32) -> Result<(),String>

``ct``
    
``conepar``
    
``nummem``
    
``j``
    


.. index:: append_cones_seq

.. _optimizer_task_appendconesseq:

``append_cones_seq()``
----------------------

.. code-block:: rust

    pub fn append_cones_seq ( &self,
                              ct_      : & [i32],
                              conepar_ : & [f64],
                              nummem_  : & [i32],
                              j        : i32) -> Result<(),String>

``ct``
    
``conepar``
    
``nummem``
    
``j``
    


.. index:: append_cons

.. _optimizer_task_appendcons:

``append_cons()``
-----------------

.. code-block:: rust

    pub fn append_cons ( &self,num   : i32 ) -> Result<(),String>

``num``
    


.. index:: append_sparse_sym_mat

.. _optimizer_task_appendsparsesymmat:

``append_sparse_sym_mat()``
---------------------------

.. code-block:: rust

    pub fn append_sparse_sym_mat ( &self,
                                   dim    : i32,
                                   subi_  : & [i32],
                                   subj_  : & [i32],
                                   valij_ : & [f64]) -> Result<i64,String>

``dim``
    
``subi``
    
``subj``
    
``valij``
    
*Returns:* ``idx``
    ``idx : i64``
        


.. index:: append_sparse_sym_mat_list

.. _optimizer_task_appendsparsesymmatlist:

``append_sparse_sym_mat_list()``
--------------------------------

.. code-block:: rust

    pub fn append_sparse_sym_mat_list ( &self,
                                        dims_  : & [i32],
                                        nz_    : & [i64],
                                        subi_  : & [i32],
                                        subj_  : & [i32],
                                        valij_ : & [f64],
                                        idx    : & mut [i64]) -> Result<(),String>

``dims``
    
``nz``
    
``subi``
    
``subj``
    
``valij``
    
``idx``
    


.. index:: append_vars

.. _optimizer_task_appendvars:

``append_vars()``
-----------------

.. code-block:: rust

    pub fn append_vars ( &self,num   : i32 ) -> Result<(),String>

``num``
    


.. index:: async_get_result

.. _optimizer_task_asyncgetresult:

``async_get_result()``
----------------------

.. code-block:: rust

    pub fn async_get_result ( &self,
                              addr        : &str,
                              accesstoken : &str,
                              token       : &str) -> Result<(bool,i32,i32),String>

``addr``
    
``accesstoken``
    
``token``
    
*Returns:* ``(respavailable,resp,trm)``
    ``respavailable : bool``
        
    ``resp : i32``
        
    ``trm : i32``
        


.. index:: async_optimize

.. _optimizer_task_asyncoptimize:

``async_optimize()``
--------------------

.. code-block:: rust

    pub fn async_optimize ( &self,
                            addr        : &str,
                            accesstoken : &str) -> Result<String,String>

``addr``
    
``accesstoken``
    
*Returns:* ``token``
    ``token : String``
        


.. index:: async_poll

.. _optimizer_task_asyncpoll:

``async_poll()``
----------------

.. code-block:: rust

    pub fn async_poll ( &self,
                        addr        : &str,
                        accesstoken : &str,
                        token       : &str) -> Result<(bool,i32,i32),String>

``addr``
    
``accesstoken``
    
``token``
    
*Returns:* ``(respavailable,resp,trm)``
    ``respavailable : bool``
        
    ``resp : i32``
        
    ``trm : i32``
        


.. index:: async_stop

.. _optimizer_task_asyncstop:

``async_stop()``
----------------

.. code-block:: rust

    pub fn async_stop ( &self,
                        addr        : &str,
                        accesstoken : &str,
                        token       : &str) -> Result<(),String>

``addr``
    
``accesstoken``
    
``token``
    


.. index:: basis_cond

.. _optimizer_task_basiscond:

``basis_cond()``
----------------

.. code-block:: rust

    pub fn basis_cond ( &self ) -> Result<(f64,f64),String>

*Returns:* ``(nrmbasis,nrminvbasis)``
    ``nrmbasis : f64``
        
    ``nrminvbasis : f64``
        


.. index:: bk_to_str

.. _optimizer_task_bktostr:

``bk_to_str()``
---------------

.. code-block:: rust

    pub fn bk_to_str ( &self,bk    : i32 ) -> Result<String,String>

``bk``
    
*Returns:* ``str``
    ``str : String``
        


.. index:: check_mem

.. _optimizer_task_checkmemtask:

``check_mem()``
---------------

.. code-block:: rust

    pub fn check_mem ( &self,
                       file  : &str,
                       line  : i32) -> Result<(),String>

``file``
    
``line``
    


.. index:: chg_con_bound

.. _optimizer_task_chgconbound:

``chg_con_bound()``
-------------------

.. code-block:: rust

    pub fn chg_con_bound ( &self,
                           i      : i32,
                           lower  : i32,
                           finite : i32,
                           value  : f64) -> Result<(),String>

``i``
    
``lower``
    
``finite``
    
``value``
    


.. index:: chg_var_bound

.. _optimizer_task_chgvarbound:

``chg_var_bound()``
-------------------

.. code-block:: rust

    pub fn chg_var_bound ( &self,
                           j      : i32,
                           lower  : i32,
                           finite : i32,
                           value  : f64) -> Result<(),String>

``j``
    
``lower``
    
``finite``
    
``value``
    


.. index:: commit_changes

.. _optimizer_task_commitchanges:

``commit_changes()``
--------------------

.. code-block:: rust

    pub fn commit_changes ( &self ) -> Result<(),String>



.. index:: cone_type_to_str

.. _optimizer_task_conetypetostr:

``cone_type_to_str()``
----------------------

.. code-block:: rust

    pub fn cone_type_to_str ( &self,ct    : i32 ) -> Result<String,String>

``ct``
    
*Returns:* ``str``
    ``str : String``
        


.. index:: delete_solution

.. _optimizer_task_deletesolution:

``delete_solution()``
---------------------

.. code-block:: rust

    pub fn delete_solution ( &self,whichsol : i32 ) -> Result<(),String>

``whichsol``
    


.. index:: dual_sensitivity

.. _optimizer_task_dualsensitivity:

``dual_sensitivity()``
----------------------

.. code-block:: rust

    pub fn dual_sensitivity ( &self,
                              subj_       : & [i32],
                              leftpricej  : & mut [f64],
                              rightpricej : & mut [f64],
                              leftrangej  : & mut [f64],
                              rightrangej : & mut [f64]) -> Result<(),String>

``subj``
    
``leftpricej``
    
``rightpricej``
    
``leftrangej``
    
``rightrangej``
    


.. index:: echo_task

.. _optimizer_task_echotask:

``echo_task()``
---------------

.. code-block:: rust

    pub fn echo_task ( &self,
                       whichstream : i32,
                       format      : &str) -> Result<(),String>

``whichstream``
    
``format``
    


.. index:: generate_con_names

.. _optimizer_task_generateconnames:

``generate_con_names()``
------------------------

.. code-block:: rust

    pub fn generate_con_names ( &self,
                                subi_ : & [i32],
                                fmt   : &str,
                                dims_ : & [i32],
                                sp_   : & [i64]) -> Result<(),String>

``subi``
    
``fmt``
    
``dims``
    
``sp``
    


.. index:: generate_cone_names

.. _optimizer_task_generateconenames:

``generate_cone_names()``
-------------------------

.. code-block:: rust

    pub fn generate_cone_names ( &self,
                                 subk_ : & [i32],
                                 fmt   : &str,
                                 dims_ : & [i32],
                                 sp_   : & [i64]) -> Result<(),String>

``subk``
    
``fmt``
    
``dims``
    
``sp``
    


.. index:: generate_var_names

.. _optimizer_task_generatevarnames:

``generate_var_names()``
------------------------

.. code-block:: rust

    pub fn generate_var_names ( &self,
                                subj_ : & [i32],
                                fmt   : &str,
                                dims_ : & [i32],
                                sp_   : & [i64]) -> Result<(),String>

``subj``
    
``fmt``
    
``dims``
    
``sp``
    


.. index:: get_a_col

.. _optimizer_task_getacol:

``get_a_col()``
---------------

.. code-block:: rust

    pub fn get_a_col ( &self,
                       j     : i32,
                       subj  : & mut [i32],
                       valj  : & mut [f64]) -> Result<i32,String>

``j``
    
``subj``
    
``valj``
    
*Returns:* ``nzj``
    ``nzj : i32``
        


.. index:: get_a_col_num_nz

.. _optimizer_task_getacolnumnz:

``get_a_col_num_nz()``
----------------------

.. code-block:: rust

    pub fn get_a_col_num_nz ( &self,i     : i32 ) -> Result<i32,String>

``i``
    
*Returns:* ``nzj``
    ``nzj : i32``
        


.. index:: get_a_col_slice_num_nz

.. _optimizer_task_getacolslicenumnz64:

``get_a_col_slice_num_nz()``
----------------------------

.. code-block:: rust

    pub fn get_a_col_slice_num_nz ( &self,
                                    first : i32,
                                    last  : i32) -> Result<i64,String>

``first``
    
``last``
    
*Returns:* ``numnz``
    ``numnz : i64``
        


.. index:: get_a_piece_num_nz

.. _optimizer_task_getapiecenumnz:

``get_a_piece_num_nz()``
------------------------

.. code-block:: rust

    pub fn get_a_piece_num_nz ( &self,
                                firsti : i32,
                                lasti  : i32,
                                firstj : i32,
                                lastj  : i32) -> Result<i32,String>

``firsti``
    
``lasti``
    
``firstj``
    
``lastj``
    
*Returns:* ``numnz``
    ``numnz : i32``
        


.. index:: get_a_row

.. _optimizer_task_getarow:

``get_a_row()``
---------------

.. code-block:: rust

    pub fn get_a_row ( &self,
                       i     : i32,
                       subi  : & mut [i32],
                       vali  : & mut [f64]) -> Result<i32,String>

``i``
    
``subi``
    
``vali``
    
*Returns:* ``nzi``
    ``nzi : i32``
        


.. index:: get_a_row_num_nz

.. _optimizer_task_getarownumnz:

``get_a_row_num_nz()``
----------------------

.. code-block:: rust

    pub fn get_a_row_num_nz ( &self,i     : i32 ) -> Result<i32,String>

``i``
    
*Returns:* ``nzi``
    ``nzi : i32``
        


.. index:: get_a_row_slice_num_nz

.. _optimizer_task_getarowslicenumnz64:

``get_a_row_slice_num_nz()``
----------------------------

.. code-block:: rust

    pub fn get_a_row_slice_num_nz ( &self,
                                    first : i32,
                                    last  : i32) -> Result<i64,String>

``first``
    
``last``
    
*Returns:* ``numnz``
    ``numnz : i64``
        


.. index:: get_a_truncate_tol

.. _optimizer_task_getatruncatetol:

``get_a_truncate_tol()``
------------------------

.. code-block:: rust

    pub fn get_a_truncate_tol ( &self,tolzero : & mut [f64] ) -> Result<(),String>

``tolzero``
    


.. index:: get_aij

.. _optimizer_task_getaij:

``get_aij()``
-------------

.. code-block:: rust

    pub fn get_aij ( &self,
                     i     : i32,
                     j     : i32) -> Result<f64,String>

``i``
    
``j``
    
*Returns:* ``aij``
    ``aij : f64``
        


.. index:: get_bara_block_triplet

.. _optimizer_task_getbarablocktriplet:

``get_bara_block_triplet()``
----------------------------

.. code-block:: rust

    pub fn get_bara_block_triplet ( &self,
                                    subi    : & mut [i32],
                                    subj    : & mut [i32],
                                    subk    : & mut [i32],
                                    subl    : & mut [i32],
                                    valijkl : & mut [f64]) -> Result<i64,String>

``subi``
    
``subj``
    
``subk``
    
``subl``
    
``valijkl``
    
*Returns:* ``num``
    ``num : i64``
        


.. index:: get_bara_idx

.. _optimizer_task_getbaraidx:

``get_bara_idx()``
------------------

.. code-block:: rust

    pub fn get_bara_idx ( &self,
                          idx     : i64,
                          sub     : & mut [i64],
                          weights : & mut [f64]) -> Result<(i32,i32,i64),String>

``idx``
    
``sub``
    
``weights``
    
*Returns:* ``(i,j,num)``
    ``i : i32``
        
    ``j : i32``
        
    ``num : i64``
        


.. index:: get_bara_idx_i_j

.. _optimizer_task_getbaraidxij:

``get_bara_idx_i_j()``
----------------------

.. code-block:: rust

    pub fn get_bara_idx_i_j ( &self,idx   : i64 ) -> Result<(i32,i32),String>

``idx``
    
*Returns:* ``(i,j)``
    ``i : i32``
        
    ``j : i32``
        


.. index:: get_bara_idx_info

.. _optimizer_task_getbaraidxinfo:

``get_bara_idx_info()``
-----------------------

.. code-block:: rust

    pub fn get_bara_idx_info ( &self,idx   : i64 ) -> Result<i64,String>

``idx``
    
*Returns:* ``num``
    ``num : i64``
        


.. index:: get_bara_sparsity

.. _optimizer_task_getbarasparsity:

``get_bara_sparsity()``
-----------------------

.. code-block:: rust

    pub fn get_bara_sparsity ( &self,idxij : & mut [i64] ) -> Result<i64,String>

``idxij``
    
*Returns:* ``numnz``
    ``numnz : i64``
        


.. index:: get_barc_block_triplet

.. _optimizer_task_getbarcblocktriplet:

``get_barc_block_triplet()``
----------------------------

.. code-block:: rust

    pub fn get_barc_block_triplet ( &self,
                                    subj   : & mut [i32],
                                    subk   : & mut [i32],
                                    subl   : & mut [i32],
                                    valjkl : & mut [f64]) -> Result<i64,String>

``subj``
    
``subk``
    
``subl``
    
``valjkl``
    
*Returns:* ``num``
    ``num : i64``
        


.. index:: get_barc_idx

.. _optimizer_task_getbarcidx:

``get_barc_idx()``
------------------

.. code-block:: rust

    pub fn get_barc_idx ( &self,
                          idx     : i64,
                          sub     : & mut [i64],
                          weights : & mut [f64]) -> Result<(i32,i64),String>

``idx``
    
``sub``
    
``weights``
    
*Returns:* ``(j,num)``
    ``j : i32``
        
    ``num : i64``
        


.. index:: get_barc_idx_info

.. _optimizer_task_getbarcidxinfo:

``get_barc_idx_info()``
-----------------------

.. code-block:: rust

    pub fn get_barc_idx_info ( &self,idx   : i64 ) -> Result<i64,String>

``idx``
    
*Returns:* ``num``
    ``num : i64``
        


.. index:: get_barc_idx_j

.. _optimizer_task_getbarcidxj:

``get_barc_idx_j()``
--------------------

.. code-block:: rust

    pub fn get_barc_idx_j ( &self,idx   : i64 ) -> Result<i32,String>

``idx``
    
*Returns:* ``j``
    ``j : i32``
        


.. index:: get_barc_sparsity

.. _optimizer_task_getbarcsparsity:

``get_barc_sparsity()``
-----------------------

.. code-block:: rust

    pub fn get_barc_sparsity ( &self,idxj  : & mut [i64] ) -> Result<i64,String>

``idxj``
    
*Returns:* ``numnz``
    ``numnz : i64``
        


.. index:: get_bars_j

.. _optimizer_task_getbarsj:

``get_bars_j()``
----------------

.. code-block:: rust

    pub fn get_bars_j ( &self,
                        whichsol : i32,
                        j        : i32,
                        barsj    : & mut [f64]) -> Result<(),String>

``whichsol``
    
``j``
    
``barsj``
    


.. index:: get_bars_slice

.. _optimizer_task_getbarsslice:

``get_bars_slice()``
--------------------

.. code-block:: rust

    pub fn get_bars_slice ( &self,
                            whichsol  : i32,
                            first     : i32,
                            last      : i32,
                            slicesize : i64,
                            barsslice : & mut [f64]) -> Result<(),String>

``whichsol``
    
``first``
    
``last``
    
``slicesize``
    
``barsslice``
    


.. index:: get_barvar_name

.. _optimizer_task_getbarvarname:

``get_barvar_name()``
---------------------

.. code-block:: rust

    pub fn get_barvar_name ( &self,i     : i32 ) -> Result<String,String>

``i``
    
*Returns:* ``name``
    ``name : String``
        


.. index:: get_barvar_name_index

.. _optimizer_task_getbarvarnameindex:

``get_barvar_name_index()``
---------------------------

.. code-block:: rust

    pub fn get_barvar_name_index ( &self,somename : &str ) -> Result<(i32,i32),String>

``somename``
    
*Returns:* ``(asgn,index)``
    ``asgn : i32``
        
    ``index : i32``
        


.. index:: get_barvar_name_len

.. _optimizer_task_getbarvarnamelen:

``get_barvar_name_len()``
-------------------------

.. code-block:: rust

    pub fn get_barvar_name_len ( &self,i     : i32 ) -> Result<i32,String>

``i``
    
*Returns:* ``len``
    ``len : i32``
        


.. index:: get_barx_j

.. _optimizer_task_getbarxj:

``get_barx_j()``
----------------

.. code-block:: rust

    pub fn get_barx_j ( &self,
                        whichsol : i32,
                        j        : i32,
                        barxj    : & mut [f64]) -> Result<(),String>

``whichsol``
    
``j``
    
``barxj``
    


.. index:: get_barx_slice

.. _optimizer_task_getbarxslice:

``get_barx_slice()``
--------------------

.. code-block:: rust

    pub fn get_barx_slice ( &self,
                            whichsol  : i32,
                            first     : i32,
                            last      : i32,
                            slicesize : i64,
                            barxslice : & mut [f64]) -> Result<(),String>

``whichsol``
    
``first``
    
``last``
    
``slicesize``
    
``barxslice``
    


.. index:: get_c

.. _optimizer_task_getc:

``get_c()``
-----------

.. code-block:: rust

    pub fn get_c ( &self,c     : & mut [f64] ) -> Result<(),String>

``c``
    


.. index:: get_c_j

.. _optimizer_task_getcj:

``get_c_j()``
-------------

.. code-block:: rust

    pub fn get_c_j ( &self,j     : i32 ) -> Result<f64,String>

``j``
    
*Returns:* ``cj``
    ``cj : f64``
        


.. index:: get_c_list

.. _optimizer_task_getclist:

``get_c_list()``
----------------

.. code-block:: rust

    pub fn get_c_list ( &self,
                        subj_ : & [i32],
                        c     : & mut [f64]) -> Result<(),String>

``subj``
    
``c``
    


.. index:: get_c_slice

.. _optimizer_task_getcslice:

``get_c_slice()``
-----------------

.. code-block:: rust

    pub fn get_c_slice ( &self,
                         first : i32,
                         last  : i32,
                         c     : & mut [f64]) -> Result<(),String>

``first``
    
``last``
    
``c``
    


.. index:: get_cfix

.. _optimizer_task_getcfix:

``get_cfix()``
--------------

.. code-block:: rust

    pub fn get_cfix ( &self ) -> Result<f64,String>

*Returns:* ``cfix``
    ``cfix : f64``
        


.. index:: get_con_bound

.. _optimizer_task_getconbound:

``get_con_bound()``
-------------------

.. code-block:: rust

    pub fn get_con_bound ( &self,i     : i32 ) -> Result<(i32,f64,f64),String>

``i``
    
*Returns:* ``(bk,bl,bu)``
    ``bk : i32``
        
    ``bl : f64``
        
    ``bu : f64``
        


.. index:: get_con_bound_slice

.. _optimizer_task_getconboundslice:

``get_con_bound_slice()``
-------------------------

.. code-block:: rust

    pub fn get_con_bound_slice ( &self,
                                 first : i32,
                                 last  : i32,
                                 bk    : & mut [i32],
                                 bl    : & mut [f64],
                                 bu    : & mut [f64]) -> Result<(),String>

``first``
    
``last``
    
``bk``
    
``bl``
    
``bu``
    


.. index:: get_con_name

.. _optimizer_task_getconname:

``get_con_name()``
------------------

.. code-block:: rust

    pub fn get_con_name ( &self,i     : i32 ) -> Result<String,String>

``i``
    
*Returns:* ``name``
    ``name : String``
        


.. index:: get_con_name_index

.. _optimizer_task_getconnameindex:

``get_con_name_index()``
------------------------

.. code-block:: rust

    pub fn get_con_name_index ( &self,somename : &str ) -> Result<(i32,i32),String>

``somename``
    
*Returns:* ``(asgn,index)``
    ``asgn : i32``
        
    ``index : i32``
        


.. index:: get_con_name_len

.. _optimizer_task_getconnamelen:

``get_con_name_len()``
----------------------

.. code-block:: rust

    pub fn get_con_name_len ( &self,i     : i32 ) -> Result<i32,String>

``i``
    
*Returns:* ``len``
    ``len : i32``
        


.. index:: get_cone

.. _optimizer_task_getcone:

``get_cone()``
--------------

.. code-block:: rust

    pub fn get_cone ( &self,
                      k      : i32,
                      submem : & mut [i32]) -> Result<(i32,f64,i32),String>

``k``
    
``submem``
    
*Returns:* ``(ct,conepar,nummem)``
    ``ct : i32``
        
    ``conepar : f64``
        
    ``nummem : i32``
        


.. index:: get_cone_info

.. _optimizer_task_getconeinfo:

``get_cone_info()``
-------------------

.. code-block:: rust

    pub fn get_cone_info ( &self,k     : i32 ) -> Result<(i32,f64,i32),String>

``k``
    
*Returns:* ``(ct,conepar,nummem)``
    ``ct : i32``
        
    ``conepar : f64``
        
    ``nummem : i32``
        


.. index:: get_cone_name

.. _optimizer_task_getconename:

``get_cone_name()``
-------------------

.. code-block:: rust

    pub fn get_cone_name ( &self,i     : i32 ) -> Result<String,String>

``i``
    
*Returns:* ``name``
    ``name : String``
        


.. index:: get_cone_name_index

.. _optimizer_task_getconenameindex:

``get_cone_name_index()``
-------------------------

.. code-block:: rust

    pub fn get_cone_name_index ( &self,somename : &str ) -> Result<(i32,i32),String>

``somename``
    
*Returns:* ``(asgn,index)``
    ``asgn : i32``
        
    ``index : i32``
        


.. index:: get_cone_name_len

.. _optimizer_task_getconenamelen:

``get_cone_name_len()``
-----------------------

.. code-block:: rust

    pub fn get_cone_name_len ( &self,i     : i32 ) -> Result<i32,String>

``i``
    
*Returns:* ``len``
    ``len : i32``
        


.. index:: get_dim_barvar_j

.. _optimizer_task_getdimbarvarj:

``get_dim_barvar_j()``
----------------------

.. code-block:: rust

    pub fn get_dim_barvar_j ( &self,j     : i32 ) -> Result<i32,String>

``j``
    
*Returns:* ``dimbarvarj``
    ``dimbarvarj : i32``
        


.. index:: get_dou_inf

.. _optimizer_task_getdouinf:

``get_dou_inf()``
-----------------

.. code-block:: rust

    pub fn get_dou_inf ( &self,whichdinf : i32 ) -> Result<f64,String>

``whichdinf``
    
*Returns:* ``dvalue``
    ``dvalue : f64``
        


.. index:: get_dou_param

.. _optimizer_task_getdouparam:

``get_dou_param()``
-------------------

.. code-block:: rust

    pub fn get_dou_param ( &self,param : i32 ) -> Result<f64,String>

``param``
    
*Returns:* ``parvalue``
    ``parvalue : f64``
        


.. index:: get_dual_obj

.. _optimizer_task_getdualobj:

``get_dual_obj()``
------------------

.. code-block:: rust

    pub fn get_dual_obj ( &self,whichsol : i32 ) -> Result<f64,String>

``whichsol``
    
*Returns:* ``dualobj``
    ``dualobj : f64``
        


.. index:: get_dual_solution_norms

.. _optimizer_task_getdualsolutionnorms:

``get_dual_solution_norms()``
-----------------------------

.. code-block:: rust

    pub fn get_dual_solution_norms ( &self,whichsol : i32 ) -> Result<(f64,f64,f64,f64,f64,f64,f64),String>

``whichsol``
    
*Returns:* ``(nrmy,nrmslc,nrmsuc,nrmslx,nrmsux,nrmsnx,nrmbars)``
    ``nrmy : f64``
        
    ``nrmslc : f64``
        
    ``nrmsuc : f64``
        
    ``nrmslx : f64``
        
    ``nrmsux : f64``
        
    ``nrmsnx : f64``
        
    ``nrmbars : f64``
        


.. index:: get_dviol_barvar

.. _optimizer_task_getdviolbarvar:

``get_dviol_barvar()``
----------------------

.. code-block:: rust

    pub fn get_dviol_barvar ( &self,
                              whichsol : i32,
                              sub_     : & [i32],
                              viol     : & mut [f64]) -> Result<(),String>

``whichsol``
    
``sub``
    
``viol``
    


.. index:: get_dviol_con

.. _optimizer_task_getdviolcon:

``get_dviol_con()``
-------------------

.. code-block:: rust

    pub fn get_dviol_con ( &self,
                           whichsol : i32,
                           sub_     : & [i32],
                           viol     : & mut [f64]) -> Result<(),String>

``whichsol``
    
``sub``
    
``viol``
    


.. index:: get_dviol_cones

.. _optimizer_task_getdviolcones:

``get_dviol_cones()``
---------------------

.. code-block:: rust

    pub fn get_dviol_cones ( &self,
                             whichsol : i32,
                             sub_     : & [i32],
                             viol     : & mut [f64]) -> Result<(),String>

``whichsol``
    
``sub``
    
``viol``
    


.. index:: get_dviol_var

.. _optimizer_task_getdviolvar:

``get_dviol_var()``
-------------------

.. code-block:: rust

    pub fn get_dviol_var ( &self,
                           whichsol : i32,
                           sub_     : & [i32],
                           viol     : & mut [f64]) -> Result<(),String>

``whichsol``
    
``sub``
    
``viol``
    


.. index:: get_inf_index

.. _optimizer_task_getinfindex:

``get_inf_index()``
-------------------

.. code-block:: rust

    pub fn get_inf_index ( &self,
                           inftype : i32,
                           infname : &str) -> Result<i32,String>

``inftype``
    
``infname``
    
*Returns:* ``infindex``
    ``infindex : i32``
        


.. index:: get_inf_max

.. _optimizer_task_getinfmax:

``get_inf_max()``
-----------------

.. code-block:: rust

    pub fn get_inf_max ( &self,
                         inftype : i32,
                         infmax  : & mut [i32]) -> Result<(),String>

``inftype``
    
``infmax``
    


.. index:: get_inf_name

.. _optimizer_task_getinfname:

``get_inf_name()``
------------------

.. code-block:: rust

    pub fn get_inf_name ( &self,
                          inftype  : i32,
                          whichinf : i32) -> Result<String,String>

``inftype``
    
``whichinf``
    
*Returns:* ``infname``
    ``infname : String``
        


.. index:: get_int_inf

.. _optimizer_task_getintinf:

``get_int_inf()``
-----------------

.. code-block:: rust

    pub fn get_int_inf ( &self,whichiinf : i32 ) -> Result<i32,String>

``whichiinf``
    
*Returns:* ``ivalue``
    ``ivalue : i32``
        


.. index:: get_int_param

.. _optimizer_task_getintparam:

``get_int_param()``
-------------------

.. code-block:: rust

    pub fn get_int_param ( &self,param : i32 ) -> Result<i32,String>

``param``
    
*Returns:* ``parvalue``
    ``parvalue : i32``
        


.. index:: get_len_barvar_j

.. _optimizer_task_getlenbarvarj:

``get_len_barvar_j()``
----------------------

.. code-block:: rust

    pub fn get_len_barvar_j ( &self,j     : i32 ) -> Result<i64,String>

``j``
    
*Returns:* ``lenbarvarj``
    ``lenbarvarj : i64``
        


.. index:: get_lint_inf

.. _optimizer_task_getlintinf:

``get_lint_inf()``
------------------

.. code-block:: rust

    pub fn get_lint_inf ( &self,whichliinf : i32 ) -> Result<i64,String>

``whichliinf``
    
*Returns:* ``ivalue``
    ``ivalue : i64``
        


.. index:: get_max_name_len

.. _optimizer_task_getmaxnamelen:

``get_max_name_len()``
----------------------

.. code-block:: rust

    pub fn get_max_name_len ( &self ) -> Result<i32,String>

*Returns:* ``maxlen``
    ``maxlen : i32``
        


.. index:: get_max_num_a_nz

.. _optimizer_task_getmaxnumanz64:

``get_max_num_a_nz()``
----------------------

.. code-block:: rust

    pub fn get_max_num_a_nz ( &self ) -> Result<i64,String>

*Returns:* ``maxnumanz``
    ``maxnumanz : i64``
        


.. index:: get_max_num_barvar

.. _optimizer_task_getmaxnumbarvar:

``get_max_num_barvar()``
------------------------

.. code-block:: rust

    pub fn get_max_num_barvar ( &self ) -> Result<i32,String>

*Returns:* ``maxnumbarvar``
    ``maxnumbarvar : i32``
        


.. index:: get_max_num_con

.. _optimizer_task_getmaxnumcon:

``get_max_num_con()``
---------------------

.. code-block:: rust

    pub fn get_max_num_con ( &self ) -> Result<i32,String>

*Returns:* ``maxnumcon``
    ``maxnumcon : i32``
        


.. index:: get_max_num_cone

.. _optimizer_task_getmaxnumcone:

``get_max_num_cone()``
----------------------

.. code-block:: rust

    pub fn get_max_num_cone ( &self ) -> Result<i32,String>

*Returns:* ``maxnumcone``
    ``maxnumcone : i32``
        


.. index:: get_max_num_q_nz

.. _optimizer_task_getmaxnumqnz64:

``get_max_num_q_nz()``
----------------------

.. code-block:: rust

    pub fn get_max_num_q_nz ( &self ) -> Result<i64,String>

*Returns:* ``maxnumqnz``
    ``maxnumqnz : i64``
        


.. index:: get_max_num_var

.. _optimizer_task_getmaxnumvar:

``get_max_num_var()``
---------------------

.. code-block:: rust

    pub fn get_max_num_var ( &self ) -> Result<i32,String>

*Returns:* ``maxnumvar``
    ``maxnumvar : i32``
        


.. index:: get_mem_usage

.. _optimizer_task_getmemusagetask:

``get_mem_usage()``
-------------------

.. code-block:: rust

    pub fn get_mem_usage ( &self ) -> Result<(i64,i64),String>

*Returns:* ``(meminuse,maxmemuse)``
    ``meminuse : i64``
        
    ``maxmemuse : i64``
        


.. index:: get_na_dou_inf

.. _optimizer_task_getnadouinf:

``get_na_dou_inf()``
--------------------

.. code-block:: rust

    pub fn get_na_dou_inf ( &self,infitemname : &str ) -> Result<f64,String>

``infitemname``
    
*Returns:* ``dvalue``
    ``dvalue : f64``
        


.. index:: get_na_dou_param

.. _optimizer_task_getnadouparam:

``get_na_dou_param()``
----------------------

.. code-block:: rust

    pub fn get_na_dou_param ( &self,paramname : &str ) -> Result<f64,String>

``paramname``
    
*Returns:* ``parvalue``
    ``parvalue : f64``
        


.. index:: get_na_int_inf

.. _optimizer_task_getnaintinf:

``get_na_int_inf()``
--------------------

.. code-block:: rust

    pub fn get_na_int_inf ( &self,infitemname : &str ) -> Result<i32,String>

``infitemname``
    
*Returns:* ``ivalue``
    ``ivalue : i32``
        


.. index:: get_na_int_param

.. _optimizer_task_getnaintparam:

``get_na_int_param()``
----------------------

.. code-block:: rust

    pub fn get_na_int_param ( &self,paramname : &str ) -> Result<i32,String>

``paramname``
    
*Returns:* ``parvalue``
    ``parvalue : i32``
        


.. index:: get_na_str_param

.. _optimizer_task_getnastrparam:

``get_na_str_param()``
----------------------

.. code-block:: rust

    pub fn get_na_str_param ( &self,
                              paramname     : &str,
                              sizeparamname : i32) -> Result<(i32,String),String>

``paramname``
    
``sizeparamname``
    
*Returns:* ``(len,parvalue)``
    ``len : i32``
        
    ``parvalue : String``
        


.. index:: get_num_a_nz

.. _optimizer_task_getnumanz:

``get_num_a_nz()``
------------------

.. code-block:: rust

    pub fn get_num_a_nz ( &self ) -> Result<i32,String>

*Returns:* ``numanz``
    ``numanz : i32``
        


.. index:: get_num_a_nz_64

.. _optimizer_task_getnumanz64:

``get_num_a_nz_64()``
---------------------

.. code-block:: rust

    pub fn get_num_a_nz_64 ( &self ) -> Result<i64,String>

*Returns:* ``numanz``
    ``numanz : i64``
        


.. index:: get_num_bara_block_triplets

.. _optimizer_task_getnumbarablocktriplets:

``get_num_bara_block_triplets()``
---------------------------------

.. code-block:: rust

    pub fn get_num_bara_block_triplets ( &self ) -> Result<i64,String>

*Returns:* ``num``
    ``num : i64``
        


.. index:: get_num_bara_nz

.. _optimizer_task_getnumbaranz:

``get_num_bara_nz()``
---------------------

.. code-block:: rust

    pub fn get_num_bara_nz ( &self ) -> Result<i64,String>

*Returns:* ``nz``
    ``nz : i64``
        


.. index:: get_num_barc_block_triplets

.. _optimizer_task_getnumbarcblocktriplets:

``get_num_barc_block_triplets()``
---------------------------------

.. code-block:: rust

    pub fn get_num_barc_block_triplets ( &self ) -> Result<i64,String>

*Returns:* ``num``
    ``num : i64``
        


.. index:: get_num_barc_nz

.. _optimizer_task_getnumbarcnz:

``get_num_barc_nz()``
---------------------

.. code-block:: rust

    pub fn get_num_barc_nz ( &self ) -> Result<i64,String>

*Returns:* ``nz``
    ``nz : i64``
        


.. index:: get_num_barvar

.. _optimizer_task_getnumbarvar:

``get_num_barvar()``
--------------------

.. code-block:: rust

    pub fn get_num_barvar ( &self ) -> Result<i32,String>

*Returns:* ``numbarvar``
    ``numbarvar : i32``
        


.. index:: get_num_con

.. _optimizer_task_getnumcon:

``get_num_con()``
-----------------

.. code-block:: rust

    pub fn get_num_con ( &self ) -> Result<i32,String>

*Returns:* ``numcon``
    ``numcon : i32``
        


.. index:: get_num_cone

.. _optimizer_task_getnumcone:

``get_num_cone()``
------------------

.. code-block:: rust

    pub fn get_num_cone ( &self ) -> Result<i32,String>

*Returns:* ``numcone``
    ``numcone : i32``
        


.. index:: get_num_cone_mem

.. _optimizer_task_getnumconemem:

``get_num_cone_mem()``
----------------------

.. code-block:: rust

    pub fn get_num_cone_mem ( &self,k     : i32 ) -> Result<i32,String>

``k``
    
*Returns:* ``nummem``
    ``nummem : i32``
        


.. index:: get_num_int_var

.. _optimizer_task_getnumintvar:

``get_num_int_var()``
---------------------

.. code-block:: rust

    pub fn get_num_int_var ( &self ) -> Result<i32,String>

*Returns:* ``numintvar``
    ``numintvar : i32``
        


.. index:: get_num_param

.. _optimizer_task_getnumparam:

``get_num_param()``
-------------------

.. code-block:: rust

    pub fn get_num_param ( &self,partype : i32 ) -> Result<i32,String>

``partype``
    
*Returns:* ``numparam``
    ``numparam : i32``
        


.. index:: get_num_q_con_k_nz

.. _optimizer_task_getnumqconknz64:

``get_num_q_con_k_nz()``
------------------------

.. code-block:: rust

    pub fn get_num_q_con_k_nz ( &self,k     : i32 ) -> Result<i64,String>

``k``
    
*Returns:* ``numqcnz``
    ``numqcnz : i64``
        


.. index:: get_num_q_obj_nz

.. _optimizer_task_getnumqobjnz64:

``get_num_q_obj_nz()``
----------------------

.. code-block:: rust

    pub fn get_num_q_obj_nz ( &self ) -> Result<i64,String>

*Returns:* ``numqonz``
    ``numqonz : i64``
        


.. index:: get_num_sym_mat

.. _optimizer_task_getnumsymmat:

``get_num_sym_mat()``
---------------------

.. code-block:: rust

    pub fn get_num_sym_mat ( &self ) -> Result<i64,String>

*Returns:* ``num``
    ``num : i64``
        


.. index:: get_num_var

.. _optimizer_task_getnumvar:

``get_num_var()``
-----------------

.. code-block:: rust

    pub fn get_num_var ( &self ) -> Result<i32,String>

*Returns:* ``numvar``
    ``numvar : i32``
        


.. index:: get_obj_name

.. _optimizer_task_getobjname:

``get_obj_name()``
------------------

.. code-block:: rust

    pub fn get_obj_name ( &self ) -> Result<String,String>

*Returns:* ``objname``
    ``objname : String``
        


.. index:: get_obj_name_len

.. _optimizer_task_getobjnamelen:

``get_obj_name_len()``
----------------------

.. code-block:: rust

    pub fn get_obj_name_len ( &self ) -> Result<i32,String>

*Returns:* ``len``
    ``len : i32``
        


.. index:: get_obj_sense

.. _optimizer_task_getobjsense:

``get_obj_sense()``
-------------------

.. code-block:: rust

    pub fn get_obj_sense ( &self ) -> Result<i32,String>

*Returns:* ``sense``
    ``sense : i32``
        


.. index:: get_param_max

.. _optimizer_task_getparammax:

``get_param_max()``
-------------------

.. code-block:: rust

    pub fn get_param_max ( &self,partype : i32 ) -> Result<i32,String>

``partype``
    
*Returns:* ``parammax``
    ``parammax : i32``
        


.. index:: get_param_name

.. _optimizer_task_getparamname:

``get_param_name()``
--------------------

.. code-block:: rust

    pub fn get_param_name ( &self,
                            partype : i32,
                            param   : i32) -> Result<String,String>

``partype``
    
``param``
    
*Returns:* ``parname``
    ``parname : String``
        


.. index:: get_primal_obj

.. _optimizer_task_getprimalobj:

``get_primal_obj()``
--------------------

.. code-block:: rust

    pub fn get_primal_obj ( &self,whichsol : i32 ) -> Result<f64,String>

``whichsol``
    
*Returns:* ``primalobj``
    ``primalobj : f64``
        


.. index:: get_primal_solution_norms

.. _optimizer_task_getprimalsolutionnorms:

``get_primal_solution_norms()``
-------------------------------

.. code-block:: rust

    pub fn get_primal_solution_norms ( &self,whichsol : i32 ) -> Result<(f64,f64,f64),String>

``whichsol``
    
*Returns:* ``(nrmxc,nrmxx,nrmbarx)``
    ``nrmxc : f64``
        
    ``nrmxx : f64``
        
    ``nrmbarx : f64``
        


.. index:: get_pro_sta

.. _optimizer_task_getprosta:

``get_pro_sta()``
-----------------

.. code-block:: rust

    pub fn get_pro_sta ( &self,whichsol : i32 ) -> Result<i32,String>

``whichsol``
    
*Returns:* ``prosta``
    ``prosta : i32``
        


.. index:: get_prob_type

.. _optimizer_task_getprobtype:

``get_prob_type()``
-------------------

.. code-block:: rust

    pub fn get_prob_type ( &self ) -> Result<i32,String>

*Returns:* ``probtype``
    ``probtype : i32``
        


.. index:: get_pviol_barvar

.. _optimizer_task_getpviolbarvar:

``get_pviol_barvar()``
----------------------

.. code-block:: rust

    pub fn get_pviol_barvar ( &self,
                              whichsol : i32,
                              sub_     : & [i32],
                              viol     : & mut [f64]) -> Result<(),String>

``whichsol``
    
``sub``
    
``viol``
    


.. index:: get_pviol_con

.. _optimizer_task_getpviolcon:

``get_pviol_con()``
-------------------

.. code-block:: rust

    pub fn get_pviol_con ( &self,
                           whichsol : i32,
                           sub_     : & [i32],
                           viol     : & mut [f64]) -> Result<(),String>

``whichsol``
    
``sub``
    
``viol``
    


.. index:: get_pviol_cones

.. _optimizer_task_getpviolcones:

``get_pviol_cones()``
---------------------

.. code-block:: rust

    pub fn get_pviol_cones ( &self,
                             whichsol : i32,
                             sub_     : & [i32],
                             viol     : & mut [f64]) -> Result<(),String>

``whichsol``
    
``sub``
    
``viol``
    


.. index:: get_pviol_var

.. _optimizer_task_getpviolvar:

``get_pviol_var()``
-------------------

.. code-block:: rust

    pub fn get_pviol_var ( &self,
                           whichsol : i32,
                           sub_     : & [i32],
                           viol     : & mut [f64]) -> Result<(),String>

``whichsol``
    
``sub``
    
``viol``
    


.. index:: get_q_obj_i_j

.. _optimizer_task_getqobjij:

``get_q_obj_i_j()``
-------------------

.. code-block:: rust

    pub fn get_q_obj_i_j ( &self,
                           i     : i32,
                           j     : i32) -> Result<f64,String>

``i``
    
``j``
    
*Returns:* ``qoij``
    ``qoij : f64``
        


.. index:: get_reduced_costs

.. _optimizer_task_getreducedcosts:

``get_reduced_costs()``
-----------------------

.. code-block:: rust

    pub fn get_reduced_costs ( &self,
                               whichsol : i32,
                               first    : i32,
                               last     : i32,
                               redcosts : & mut [f64]) -> Result<(),String>

``whichsol``
    
``first``
    
``last``
    
``redcosts``
    


.. index:: get_skc

.. _optimizer_task_getskc:

``get_skc()``
-------------

.. code-block:: rust

    pub fn get_skc ( &self,
                     whichsol : i32,
                     skc      : & mut [i32]) -> Result<(),String>

``whichsol``
    
``skc``
    


.. index:: get_skc_slice

.. _optimizer_task_getskcslice:

``get_skc_slice()``
-------------------

.. code-block:: rust

    pub fn get_skc_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           skc      : & mut [i32]) -> Result<(),String>

``whichsol``
    
``first``
    
``last``
    
``skc``
    


.. index:: get_skn

.. _optimizer_task_getskn:

``get_skn()``
-------------

.. code-block:: rust

    pub fn get_skn ( &self,
                     whichsol : i32,
                     skn      : & mut [i32]) -> Result<(),String>

``whichsol``
    
``skn``
    


.. index:: get_skx

.. _optimizer_task_getskx:

``get_skx()``
-------------

.. code-block:: rust

    pub fn get_skx ( &self,
                     whichsol : i32,
                     skx      : & mut [i32]) -> Result<(),String>

``whichsol``
    
``skx``
    


.. index:: get_skx_slice

.. _optimizer_task_getskxslice:

``get_skx_slice()``
-------------------

.. code-block:: rust

    pub fn get_skx_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           skx      : & mut [i32]) -> Result<(),String>

``whichsol``
    
``first``
    
``last``
    
``skx``
    


.. index:: get_slc

.. _optimizer_task_getslc:

``get_slc()``
-------------

.. code-block:: rust

    pub fn get_slc ( &self,
                     whichsol : i32,
                     slc      : & mut [f64]) -> Result<(),String>

``whichsol``
    
``slc``
    


.. index:: get_slc_slice

.. _optimizer_task_getslcslice:

``get_slc_slice()``
-------------------

.. code-block:: rust

    pub fn get_slc_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           slc      : & mut [f64]) -> Result<(),String>

``whichsol``
    
``first``
    
``last``
    
``slc``
    


.. index:: get_slx

.. _optimizer_task_getslx:

``get_slx()``
-------------

.. code-block:: rust

    pub fn get_slx ( &self,
                     whichsol : i32,
                     slx      : & mut [f64]) -> Result<(),String>

``whichsol``
    
``slx``
    


.. index:: get_slx_slice

.. _optimizer_task_getslxslice:

``get_slx_slice()``
-------------------

.. code-block:: rust

    pub fn get_slx_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           slx      : & mut [f64]) -> Result<(),String>

``whichsol``
    
``first``
    
``last``
    
``slx``
    


.. index:: get_snx

.. _optimizer_task_getsnx:

``get_snx()``
-------------

.. code-block:: rust

    pub fn get_snx ( &self,
                     whichsol : i32,
                     snx      : & mut [f64]) -> Result<(),String>

``whichsol``
    
``snx``
    


.. index:: get_snx_slice

.. _optimizer_task_getsnxslice:

``get_snx_slice()``
-------------------

.. code-block:: rust

    pub fn get_snx_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           snx      : & mut [f64]) -> Result<(),String>

``whichsol``
    
``first``
    
``last``
    
``snx``
    


.. index:: get_sol_sta

.. _optimizer_task_getsolsta:

``get_sol_sta()``
-----------------

.. code-block:: rust

    pub fn get_sol_sta ( &self,whichsol : i32 ) -> Result<i32,String>

``whichsol``
    
*Returns:* ``solsta``
    ``solsta : i32``
        


.. index:: get_solution

.. _optimizer_task_getsolution:

``get_solution()``
------------------

.. code-block:: rust

    pub fn get_solution ( &self,
                          whichsol : i32,
                          skc      : & mut [i32],
                          skx      : & mut [i32],
                          skn      : & mut [i32],
                          xc       : & mut [f64],
                          xx       : & mut [f64],
                          y        : & mut [f64],
                          slc      : & mut [f64],
                          suc      : & mut [f64],
                          slx      : & mut [f64],
                          sux      : & mut [f64],
                          snx      : & mut [f64]) -> Result<(i32,i32),String>

``whichsol``
    
``skc``
    
``skx``
    
``skn``
    
``xc``
    
``xx``
    
``y``
    
``slc``
    
``suc``
    
``slx``
    
``sux``
    
``snx``
    
*Returns:* ``(prosta,solsta)``
    ``prosta : i32``
        
    ``solsta : i32``
        


.. index:: get_solution_info

.. _optimizer_task_getsolutioninfo:

``get_solution_info()``
-----------------------

.. code-block:: rust

    pub fn get_solution_info ( &self,whichsol : i32 ) -> Result<(f64,f64,f64,f64,f64,f64,f64,f64,f64,f64,f64),String>

``whichsol``
    
*Returns:* ``(pobj,pviolcon,pviolvar,pviolbarvar,pviolcone,pviolitg,dobj,dviolcon,dviolvar,dviolbarvar,dviolcone)``
    ``pobj : f64``
        
    ``pviolcon : f64``
        
    ``pviolvar : f64``
        
    ``pviolbarvar : f64``
        
    ``pviolcone : f64``
        
    ``pviolitg : f64``
        
    ``dobj : f64``
        
    ``dviolcon : f64``
        
    ``dviolvar : f64``
        
    ``dviolbarvar : f64``
        
    ``dviolcone : f64``
        


.. index:: get_solution_slice

.. _optimizer_task_getsolutionslice:

``get_solution_slice()``
------------------------

.. code-block:: rust

    pub fn get_solution_slice ( &self,
                                whichsol : i32,
                                solitem  : i32,
                                first    : i32,
                                last     : i32,
                                values   : & mut [f64]) -> Result<(),String>

``whichsol``
    
``solitem``
    
``first``
    
``last``
    
``values``
    


.. index:: get_sparse_sym_mat

.. _optimizer_task_getsparsesymmat:

``get_sparse_sym_mat()``
------------------------

.. code-block:: rust

    pub fn get_sparse_sym_mat ( &self,
                                idx   : i64,
                                subi  : & mut [i32],
                                subj  : & mut [i32],
                                valij : & mut [f64]) -> Result<(),String>

``idx``
    
``subi``
    
``subj``
    
``valij``
    


.. index:: get_str_param

.. _optimizer_task_getstrparam:

``get_str_param()``
-------------------

.. code-block:: rust

    pub fn get_str_param ( &self,param : i32 ) -> Result<(i32,String),String>

``param``
    
*Returns:* ``(len,parvalue)``
    ``len : i32``
        
    ``parvalue : String``
        


.. index:: get_str_param_len

.. _optimizer_task_getstrparamlen:

``get_str_param_len()``
-----------------------

.. code-block:: rust

    pub fn get_str_param_len ( &self,param : i32 ) -> Result<i32,String>

``param``
    
*Returns:* ``len``
    ``len : i32``
        


.. index:: get_suc

.. _optimizer_task_getsuc:

``get_suc()``
-------------

.. code-block:: rust

    pub fn get_suc ( &self,
                     whichsol : i32,
                     suc      : & mut [f64]) -> Result<(),String>

``whichsol``
    
``suc``
    


.. index:: get_suc_slice

.. _optimizer_task_getsucslice:

``get_suc_slice()``
-------------------

.. code-block:: rust

    pub fn get_suc_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           suc      : & mut [f64]) -> Result<(),String>

``whichsol``
    
``first``
    
``last``
    
``suc``
    


.. index:: get_sux

.. _optimizer_task_getsux:

``get_sux()``
-------------

.. code-block:: rust

    pub fn get_sux ( &self,
                     whichsol : i32,
                     sux      : & mut [f64]) -> Result<(),String>

``whichsol``
    
``sux``
    


.. index:: get_sux_slice

.. _optimizer_task_getsuxslice:

``get_sux_slice()``
-------------------

.. code-block:: rust

    pub fn get_sux_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           sux      : & mut [f64]) -> Result<(),String>

``whichsol``
    
``first``
    
``last``
    
``sux``
    


.. index:: get_sym_mat_info

.. _optimizer_task_getsymmatinfo:

``get_sym_mat_info()``
----------------------

.. code-block:: rust

    pub fn get_sym_mat_info ( &self,idx   : i64 ) -> Result<(i32,i64,i32),String>

``idx``
    
*Returns:* ``(dim,nz,type)``
    ``dim : i32``
        
    ``nz : i64``
        
    ``type : i32``
        


.. index:: get_symb_con

.. _optimizer_task_getsymbcon:

``get_symb_con()``
------------------

.. code-block:: rust

    pub fn get_symb_con ( &self,i     : i32 ) -> Result<(String,i32),String>

``i``
    
*Returns:* ``(name,value)``
    ``name : String``
        
    ``value : i32``
        


.. index:: get_task_name

.. _optimizer_task_gettaskname:

``get_task_name()``
-------------------

.. code-block:: rust

    pub fn get_task_name ( &self ) -> Result<String,String>

*Returns:* ``taskname``
    ``taskname : String``
        


.. index:: get_task_name_len

.. _optimizer_task_gettasknamelen:

``get_task_name_len()``
-----------------------

.. code-block:: rust

    pub fn get_task_name_len ( &self ) -> Result<i32,String>

*Returns:* ``len``
    ``len : i32``
        


.. index:: get_var_bound

.. _optimizer_task_getvarbound:

``get_var_bound()``
-------------------

.. code-block:: rust

    pub fn get_var_bound ( &self,i     : i32 ) -> Result<(i32,f64,f64),String>

``i``
    
*Returns:* ``(bk,bl,bu)``
    ``bk : i32``
        
    ``bl : f64``
        
    ``bu : f64``
        


.. index:: get_var_bound_slice

.. _optimizer_task_getvarboundslice:

``get_var_bound_slice()``
-------------------------

.. code-block:: rust

    pub fn get_var_bound_slice ( &self,
                                 first : i32,
                                 last  : i32,
                                 bk    : & mut [i32],
                                 bl    : & mut [f64],
                                 bu    : & mut [f64]) -> Result<(),String>

``first``
    
``last``
    
``bk``
    
``bl``
    
``bu``
    


.. index:: get_var_name

.. _optimizer_task_getvarname:

``get_var_name()``
------------------

.. code-block:: rust

    pub fn get_var_name ( &self,j     : i32 ) -> Result<String,String>

``j``
    
*Returns:* ``name``
    ``name : String``
        


.. index:: get_var_name_index

.. _optimizer_task_getvarnameindex:

``get_var_name_index()``
------------------------

.. code-block:: rust

    pub fn get_var_name_index ( &self,somename : &str ) -> Result<(i32,i32),String>

``somename``
    
*Returns:* ``(asgn,index)``
    ``asgn : i32``
        
    ``index : i32``
        


.. index:: get_var_name_len

.. _optimizer_task_getvarnamelen:

``get_var_name_len()``
----------------------

.. code-block:: rust

    pub fn get_var_name_len ( &self,i     : i32 ) -> Result<i32,String>

``i``
    
*Returns:* ``len``
    ``len : i32``
        


.. index:: get_var_type

.. _optimizer_task_getvartype:

``get_var_type()``
------------------

.. code-block:: rust

    pub fn get_var_type ( &self,j     : i32 ) -> Result<i32,String>

``j``
    
*Returns:* ``vartype``
    ``vartype : i32``
        


.. index:: get_var_type_list

.. _optimizer_task_getvartypelist:

``get_var_type_list()``
-----------------------

.. code-block:: rust

    pub fn get_var_type_list ( &self,
                               subj_   : & [i32],
                               vartype : & mut [i32]) -> Result<(),String>

``subj``
    
``vartype``
    


.. index:: get_xc

.. _optimizer_task_getxc:

``get_xc()``
------------

.. code-block:: rust

    pub fn get_xc ( &self,
                    whichsol : i32,
                    xc       : & mut [f64]) -> Result<(),String>

``whichsol``
    
``xc``
    


.. index:: get_xc_slice

.. _optimizer_task_getxcslice:

``get_xc_slice()``
------------------

.. code-block:: rust

    pub fn get_xc_slice ( &self,
                          whichsol : i32,
                          first    : i32,
                          last     : i32,
                          xc       : & mut [f64]) -> Result<(),String>

``whichsol``
    
``first``
    
``last``
    
``xc``
    


.. index:: get_xx

.. _optimizer_task_getxx:

``get_xx()``
------------

.. code-block:: rust

    pub fn get_xx ( &self,
                    whichsol : i32,
                    xx       : & mut [f64]) -> Result<(),String>

``whichsol``
    
``xx``
    


.. index:: get_xx_slice

.. _optimizer_task_getxxslice:

``get_xx_slice()``
------------------

.. code-block:: rust

    pub fn get_xx_slice ( &self,
                          whichsol : i32,
                          first    : i32,
                          last     : i32,
                          xx       : & mut [f64]) -> Result<(),String>

``whichsol``
    
``first``
    
``last``
    
``xx``
    


.. index:: get_y

.. _optimizer_task_gety:

``get_y()``
-----------

.. code-block:: rust

    pub fn get_y ( &self,
                   whichsol : i32,
                   y        : & mut [f64]) -> Result<(),String>

``whichsol``
    
``y``
    


.. index:: get_y_slice

.. _optimizer_task_getyslice:

``get_y_slice()``
-----------------

.. code-block:: rust

    pub fn get_y_slice ( &self,
                         whichsol : i32,
                         first    : i32,
                         last     : i32,
                         y        : & mut [f64]) -> Result<(),String>

``whichsol``
    
``first``
    
``last``
    
``y``
    


.. index:: init_basis_solve

.. _optimizer_task_initbasissolve:

``init_basis_solve()``
----------------------

.. code-block:: rust

    pub fn init_basis_solve ( &self,basis : & mut [i32] ) -> Result<(),String>

``basis``
    


.. index:: input_data

.. _optimizer_task_inputdata64:

``input_data()``
----------------

.. code-block:: rust

    pub fn input_data ( &self,
                        maxnumcon : i32,
                        maxnumvar : i32,
                        c_        : & [f64],
                        cfix      : f64,
                        aptrb_    : & [i64],
                        aptre_    : & [i64],
                        asub_     : & [i32],
                        aval_     : & [f64],
                        bkc_      : & [i32],
                        blc_      : & [f64],
                        buc_      : & [f64],
                        bkx_      : & [i32],
                        blx_      : & [f64],
                        bux_      : & [f64]) -> Result<(),String>

``maxnumcon``
    
``maxnumvar``
    
``c``
    
``cfix``
    
``aptrb``
    
``aptre``
    
``asub``
    
``aval``
    
``bkc``
    
``blc``
    
``buc``
    
``bkx``
    
``blx``
    
``bux``
    


.. index:: is_dou_par_name

.. _optimizer_task_isdouparname:

``is_dou_par_name()``
---------------------

.. code-block:: rust

    pub fn is_dou_par_name ( &self,parname : &str ) -> Result<i32,String>

``parname``
    
*Returns:* ``param``
    ``param : i32``
        


.. index:: is_int_par_name

.. _optimizer_task_isintparname:

``is_int_par_name()``
---------------------

.. code-block:: rust

    pub fn is_int_par_name ( &self,parname : &str ) -> Result<i32,String>

``parname``
    
*Returns:* ``param``
    ``param : i32``
        


.. index:: is_str_par_name

.. _optimizer_task_isstrparname:

``is_str_par_name()``
---------------------

.. code-block:: rust

    pub fn is_str_par_name ( &self,parname : &str ) -> Result<i32,String>

``parname``
    
*Returns:* ``param``
    ``param : i32``
        


.. index:: link_file_to_stream

.. _optimizer_task_linkfiletotaskstream:

``link_file_to_stream()``
-------------------------

.. code-block:: rust

    pub fn link_file_to_stream ( &self,
                                 whichstream : i32,
                                 filename    : &str,
                                 append      : i32) -> Result<(),String>

``whichstream``
    
``filename``
    
``append``
    


.. index:: one_solution_summary

.. _optimizer_task_onesolutionsummary:

``one_solution_summary()``
--------------------------

.. code-block:: rust

    pub fn one_solution_summary ( &self,
                                  whichstream : i32,
                                  whichsol    : i32) -> Result<(),String>

``whichstream``
    
``whichsol``
    


.. index:: optimize

.. _optimizer_task_optimizetrm:

``optimize()``
--------------

.. code-block:: rust

    pub fn optimize ( &self ) -> Result<i32,String>

*Returns:* ``trmcode``
    ``trmcode : i32``
        


.. index:: optimize_rmt

.. _optimizer_task_optimizermt:

``optimize_rmt()``
------------------

.. code-block:: rust

    pub fn optimize_rmt ( &self,
                          addr        : &str,
                          accesstoken : &str) -> Result<i32,String>

``addr``
    
``accesstoken``
    
*Returns:* ``trmcode``
    ``trmcode : i32``
        


.. index:: optimizer_summary

.. _optimizer_task_optimizersummary:

``optimizer_summary()``
-----------------------

.. code-block:: rust

    pub fn optimizer_summary ( &self,whichstream : i32 ) -> Result<(),String>

``whichstream``
    


.. index:: primal_repair

.. _optimizer_task_primalrepair:

``primal_repair()``
-------------------

.. code-block:: rust

    pub fn primal_repair ( &self,
                           wlc_  : & [f64],
                           wuc_  : & [f64],
                           wlx_  : & [f64],
                           wux_  : & [f64]) -> Result<(),String>

``wlc``
    
``wuc``
    
``wlx``
    
``wux``
    


.. index:: primal_sensitivity

.. _optimizer_task_primalsensitivity:

``primal_sensitivity()``
------------------------

.. code-block:: rust

    pub fn primal_sensitivity ( &self,
                                subi_       : & [i32],
                                marki_      : & [i32],
                                subj_       : & [i32],
                                markj_      : & [i32],
                                leftpricei  : & mut [f64],
                                rightpricei : & mut [f64],
                                leftrangei  : & mut [f64],
                                rightrangei : & mut [f64],
                                leftpricej  : & mut [f64],
                                rightpricej : & mut [f64],
                                leftrangej  : & mut [f64],
                                rightrangej : & mut [f64]) -> Result<(),String>

``subi``
    
``marki``
    
``subj``
    
``markj``
    
``leftpricei``
    
``rightpricei``
    
``leftrangei``
    
``rightrangei``
    
``leftpricej``
    
``rightpricej``
    
``leftrangej``
    
``rightrangej``
    


.. index:: print_param

.. _optimizer_task_printparam:

``print_param()``
-----------------

.. code-block:: rust

    pub fn print_param ( &self ) -> Result<(),String>



.. index:: pro_sta_to_str

.. _optimizer_task_prostatostr:

``pro_sta_to_str()``
--------------------

.. code-block:: rust

    pub fn pro_sta_to_str ( &self,prosta : i32 ) -> Result<String,String>

``prosta``
    
*Returns:* ``str``
    ``str : String``
        


.. index:: prob_type_to_str

.. _optimizer_task_probtypetostr:

``prob_type_to_str()``
----------------------

.. code-block:: rust

    pub fn prob_type_to_str ( &self,probtype : i32 ) -> Result<String,String>

``probtype``
    
*Returns:* ``str``
    ``str : String``
        


.. index:: put_a_col

.. _optimizer_task_putacol:

``put_a_col()``
---------------

.. code-block:: rust

    pub fn put_a_col ( &self,
                       j     : i32,
                       subj_ : & [i32],
                       valj_ : & [f64]) -> Result<(),String>

``j``
    
``subj``
    
``valj``
    


.. index:: put_a_col_list

.. _optimizer_task_putacollist64:

``put_a_col_list()``
--------------------

.. code-block:: rust

    pub fn put_a_col_list ( &self,
                            sub_  : & [i32],
                            ptrb_ : & [i64],
                            ptre_ : & [i64],
                            asub_ : & [i32],
                            aval_ : & [f64]) -> Result<(),String>

``sub``
    
``ptrb``
    
``ptre``
    
``asub``
    
``aval``
    


.. index:: put_a_col_slice

.. _optimizer_task_putacolslice64:

``put_a_col_slice()``
---------------------

.. code-block:: rust

    pub fn put_a_col_slice ( &self,
                             first : i32,
                             last  : i32,
                             ptrb_ : & [i64],
                             ptre_ : & [i64],
                             asub_ : & [i32],
                             aval_ : & [f64]) -> Result<(),String>

``first``
    
``last``
    
``ptrb``
    
``ptre``
    
``asub``
    
``aval``
    


.. index:: put_a_row

.. _optimizer_task_putarow:

``put_a_row()``
---------------

.. code-block:: rust

    pub fn put_a_row ( &self,
                       i     : i32,
                       subi_ : & [i32],
                       vali_ : & [f64]) -> Result<(),String>

``i``
    
``subi``
    
``vali``
    


.. index:: put_a_row_list

.. _optimizer_task_putarowlist64:

``put_a_row_list()``
--------------------

.. code-block:: rust

    pub fn put_a_row_list ( &self,
                            sub_  : & [i32],
                            ptrb_ : & [i64],
                            ptre_ : & [i64],
                            asub_ : & [i32],
                            aval_ : & [f64]) -> Result<(),String>

``sub``
    
``ptrb``
    
``ptre``
    
``asub``
    
``aval``
    


.. index:: put_a_row_slice

.. _optimizer_task_putarowslice64:

``put_a_row_slice()``
---------------------

.. code-block:: rust

    pub fn put_a_row_slice ( &self,
                             first : i32,
                             last  : i32,
                             ptrb_ : & [i64],
                             ptre_ : & [i64],
                             asub_ : & [i32],
                             aval_ : & [f64]) -> Result<(),String>

``first``
    
``last``
    
``ptrb``
    
``ptre``
    
``asub``
    
``aval``
    


.. index:: put_a_truncate_tol

.. _optimizer_task_putatruncatetol:

``put_a_truncate_tol()``
------------------------

.. code-block:: rust

    pub fn put_a_truncate_tol ( &self,tolzero : f64 ) -> Result<(),String>

``tolzero``
    


.. index:: put_aij

.. _optimizer_task_putaij:

``put_aij()``
-------------

.. code-block:: rust

    pub fn put_aij ( &self,
                     i     : i32,
                     j     : i32,
                     aij   : f64) -> Result<(),String>

``i``
    
``j``
    
``aij``
    


.. index:: put_aij_list

.. _optimizer_task_putaijlist64:

``put_aij_list()``
------------------

.. code-block:: rust

    pub fn put_aij_list ( &self,
                          subi_  : & [i32],
                          subj_  : & [i32],
                          valij_ : & [f64]) -> Result<(),String>

``subi``
    
``subj``
    
``valij``
    


.. index:: put_bara_block_triplet

.. _optimizer_task_putbarablocktriplet:

``put_bara_block_triplet()``
----------------------------

.. code-block:: rust

    pub fn put_bara_block_triplet ( &self,
                                    num      : i64,
                                    subi_    : & [i32],
                                    subj_    : & [i32],
                                    subk_    : & [i32],
                                    subl_    : & [i32],
                                    valijkl_ : & [f64]) -> Result<(),String>

``num``
    
``subi``
    
``subj``
    
``subk``
    
``subl``
    
``valijkl``
    


.. index:: put_bara_ij

.. _optimizer_task_putbaraij:

``put_bara_ij()``
-----------------

.. code-block:: rust

    pub fn put_bara_ij ( &self,
                         i        : i32,
                         j        : i32,
                         sub_     : & [i64],
                         weights_ : & [f64]) -> Result<(),String>

``i``
    
``j``
    
``sub``
    
``weights``
    


.. index:: put_bara_ij_list

.. _optimizer_task_putbaraijlist:

``put_bara_ij_list()``
----------------------

.. code-block:: rust

    pub fn put_bara_ij_list ( &self,
                              subi_      : & [i32],
                              subj_      : & [i32],
                              alphaptrb_ : & [i64],
                              alphaptre_ : & [i64],
                              matidx_    : & [i64],
                              weights_   : & [f64]) -> Result<(),String>

``subi``
    
``subj``
    
``alphaptrb``
    
``alphaptre``
    
``matidx``
    
``weights``
    


.. index:: put_bara_row_list

.. _optimizer_task_putbararowlist:

``put_bara_row_list()``
-----------------------

.. code-block:: rust

    pub fn put_bara_row_list ( &self,
                               subi_    : & [i32],
                               ptrb_    : & [i64],
                               ptre_    : & [i64],
                               subj_    : & [i32],
                               nummat_  : & [i64],
                               matidx_  : & [i64],
                               weights_ : & [f64]) -> Result<(),String>

``subi``
    
``ptrb``
    
``ptre``
    
``subj``
    
``nummat``
    
``matidx``
    
``weights``
    


.. index:: put_barc_block_triplet

.. _optimizer_task_putbarcblocktriplet:

``put_barc_block_triplet()``
----------------------------

.. code-block:: rust

    pub fn put_barc_block_triplet ( &self,
                                    num     : i64,
                                    subj_   : & [i32],
                                    subk_   : & [i32],
                                    subl_   : & [i32],
                                    valjkl_ : & [f64]) -> Result<(),String>

``num``
    
``subj``
    
``subk``
    
``subl``
    
``valjkl``
    


.. index:: put_barc_j

.. _optimizer_task_putbarcj:

``put_barc_j()``
----------------

.. code-block:: rust

    pub fn put_barc_j ( &self,
                        j        : i32,
                        sub_     : & [i64],
                        weights_ : & [f64]) -> Result<(),String>

``j``
    
``sub``
    
``weights``
    


.. index:: put_bars_j

.. _optimizer_task_putbarsj:

``put_bars_j()``
----------------

.. code-block:: rust

    pub fn put_bars_j ( &self,
                        whichsol : i32,
                        j        : i32,
                        barsj_   : & [f64]) -> Result<(),String>

``whichsol``
    
``j``
    
``barsj``
    


.. index:: put_barvar_name

.. _optimizer_task_putbarvarname:

``put_barvar_name()``
---------------------

.. code-block:: rust

    pub fn put_barvar_name ( &self,
                             j     : i32,
                             name  : &str) -> Result<(),String>

``j``
    
``name``
    


.. index:: put_barx_j

.. _optimizer_task_putbarxj:

``put_barx_j()``
----------------

.. code-block:: rust

    pub fn put_barx_j ( &self,
                        whichsol : i32,
                        j        : i32,
                        barxj_   : & [f64]) -> Result<(),String>

``whichsol``
    
``j``
    
``barxj``
    


.. index:: put_c_j

.. _optimizer_task_putcj:

``put_c_j()``
-------------

.. code-block:: rust

    pub fn put_c_j ( &self,
                     j     : i32,
                     cj    : f64) -> Result<(),String>

``j``
    
``cj``
    


.. index:: put_c_list

.. _optimizer_task_putclist:

``put_c_list()``
----------------

.. code-block:: rust

    pub fn put_c_list ( &self,
                        subj_ : & [i32],
                        val_  : & [f64]) -> Result<(),String>

``subj``
    
``val``
    


.. index:: put_c_slice

.. _optimizer_task_putcslice:

``put_c_slice()``
-----------------

.. code-block:: rust

    pub fn put_c_slice ( &self,
                         first  : i32,
                         last   : i32,
                         slice_ : & [f64]) -> Result<(),String>

``first``
    
``last``
    
``slice``
    


.. index:: put_callback

``put_callback()``
------------------

.. code-block:: rust

    pub fn put_callback(& mut self,
                        func   : fn(&H,i32,&[f64],&[i32],&[i64]) -> bool,
                        handle : H)

``handle``
    An object of type ``H``, as defined from ``Task<H>``.
``func``
    A callback function of the form

    .. code-block:: rust

        fn ( handle  : &H,
             caller  : i32,
             douinf  : &[f64],
             intinf  : &[i32],
             lintinf : &[i64]) -> i32

    ``handle``
        The handle object.
    ``caller``
        An integer indicating where the callback was called from (see :ref:`calbackcode`).
    ``douinf``
        Information values
    ``intinf``
        Information values
    ``lintinf``
        Information values
    *Returns:*
        ``false`` to indicate that the optimizer should terminate, otherwise ``true``.


.. index:: put_cfix

.. _optimizer_task_putcfix:

``put_cfix()``
--------------

.. code-block:: rust

    pub fn put_cfix ( &self,cfix  : f64 ) -> Result<(),String>

``cfix``
    


.. index:: put_con_bound

.. _optimizer_task_putconbound:

``put_con_bound()``
-------------------

.. code-block:: rust

    pub fn put_con_bound ( &self,
                           i     : i32,
                           bkc   : i32,
                           blc   : f64,
                           buc   : f64) -> Result<(),String>

``i``
    
``bkc``
    
``blc``
    
``buc``
    


.. index:: put_con_bound_list

.. _optimizer_task_putconboundlist:

``put_con_bound_list()``
------------------------

.. code-block:: rust

    pub fn put_con_bound_list ( &self,
                                sub_  : & [i32],
                                bkc_  : & [i32],
                                blc_  : & [f64],
                                buc_  : & [f64]) -> Result<(),String>

``sub``
    
``bkc``
    
``blc``
    
``buc``
    


.. index:: put_con_bound_list_const

.. _optimizer_task_putconboundlistconst:

``put_con_bound_list_const()``
------------------------------

.. code-block:: rust

    pub fn put_con_bound_list_const ( &self,
                                      sub_  : & [i32],
                                      bkc   : i32,
                                      blc   : f64,
                                      buc   : f64) -> Result<(),String>

``sub``
    
``bkc``
    
``blc``
    
``buc``
    


.. index:: put_con_bound_slice

.. _optimizer_task_putconboundslice:

``put_con_bound_slice()``
-------------------------

.. code-block:: rust

    pub fn put_con_bound_slice ( &self,
                                 first : i32,
                                 last  : i32,
                                 bkc_  : & [i32],
                                 blc_  : & [f64],
                                 buc_  : & [f64]) -> Result<(),String>

``first``
    
``last``
    
``bkc``
    
``blc``
    
``buc``
    


.. index:: put_con_bound_slice_const

.. _optimizer_task_putconboundsliceconst:

``put_con_bound_slice_const()``
-------------------------------

.. code-block:: rust

    pub fn put_con_bound_slice_const ( &self,
                                       first : i32,
                                       last  : i32,
                                       bkc   : i32,
                                       blc   : f64,
                                       buc   : f64) -> Result<(),String>

``first``
    
``last``
    
``bkc``
    
``blc``
    
``buc``
    


.. index:: put_con_name

.. _optimizer_task_putconname:

``put_con_name()``
------------------

.. code-block:: rust

    pub fn put_con_name ( &self,
                          i     : i32,
                          name  : &str) -> Result<(),String>

``i``
    
``name``
    


.. index:: put_con_solution_i

.. _optimizer_task_putconsolutioni:

``put_con_solution_i()``
------------------------

.. code-block:: rust

    pub fn put_con_solution_i ( &self,
                                i        : i32,
                                whichsol : i32,
                                sk       : i32,
                                x        : f64,
                                sl       : f64,
                                su       : f64) -> Result<(),String>

``i``
    
``whichsol``
    
``sk``
    
``x``
    
``sl``
    
``su``
    


.. index:: put_cone

.. _optimizer_task_putcone:

``put_cone()``
--------------

.. code-block:: rust

    pub fn put_cone ( &self,
                      k       : i32,
                      ct      : i32,
                      conepar : f64,
                      submem_ : & [i32]) -> Result<(),String>

``k``
    
``ct``
    
``conepar``
    
``submem``
    


.. index:: put_cone_name

.. _optimizer_task_putconename:

``put_cone_name()``
-------------------

.. code-block:: rust

    pub fn put_cone_name ( &self,
                           j     : i32,
                           name  : &str) -> Result<(),String>

``j``
    
``name``
    


.. index:: put_dou_param

.. _optimizer_task_putdouparam:

``put_dou_param()``
-------------------

.. code-block:: rust

    pub fn put_dou_param ( &self,
                           param    : i32,
                           parvalue : f64) -> Result<(),String>

``param``
    
``parvalue``
    


.. index:: put_int_param

.. _optimizer_task_putintparam:

``put_int_param()``
-------------------

.. code-block:: rust

    pub fn put_int_param ( &self,
                           param    : i32,
                           parvalue : i32) -> Result<(),String>

``param``
    
``parvalue``
    


.. index:: put_max_num_a_nz

.. _optimizer_task_putmaxnumanz:

``put_max_num_a_nz()``
----------------------

.. code-block:: rust

    pub fn put_max_num_a_nz ( &self,maxnumanz : i64 ) -> Result<(),String>

``maxnumanz``
    


.. index:: put_max_num_barvar

.. _optimizer_task_putmaxnumbarvar:

``put_max_num_barvar()``
------------------------

.. code-block:: rust

    pub fn put_max_num_barvar ( &self,maxnumbarvar : i32 ) -> Result<(),String>

``maxnumbarvar``
    


.. index:: put_max_num_con

.. _optimizer_task_putmaxnumcon:

``put_max_num_con()``
---------------------

.. code-block:: rust

    pub fn put_max_num_con ( &self,maxnumcon : i32 ) -> Result<(),String>

``maxnumcon``
    


.. index:: put_max_num_cone

.. _optimizer_task_putmaxnumcone:

``put_max_num_cone()``
----------------------

.. code-block:: rust

    pub fn put_max_num_cone ( &self,maxnumcone : i32 ) -> Result<(),String>

``maxnumcone``
    


.. index:: put_max_num_q_nz

.. _optimizer_task_putmaxnumqnz:

``put_max_num_q_nz()``
----------------------

.. code-block:: rust

    pub fn put_max_num_q_nz ( &self,maxnumqnz : i64 ) -> Result<(),String>

``maxnumqnz``
    


.. index:: put_max_num_var

.. _optimizer_task_putmaxnumvar:

``put_max_num_var()``
---------------------

.. code-block:: rust

    pub fn put_max_num_var ( &self,maxnumvar : i32 ) -> Result<(),String>

``maxnumvar``
    


.. index:: put_na_dou_param

.. _optimizer_task_putnadouparam:

``put_na_dou_param()``
----------------------

.. code-block:: rust

    pub fn put_na_dou_param ( &self,
                              paramname : &str,
                              parvalue  : f64) -> Result<(),String>

``paramname``
    
``parvalue``
    


.. index:: put_na_int_param

.. _optimizer_task_putnaintparam:

``put_na_int_param()``
----------------------

.. code-block:: rust

    pub fn put_na_int_param ( &self,
                              paramname : &str,
                              parvalue  : i32) -> Result<(),String>

``paramname``
    
``parvalue``
    


.. index:: put_na_str_param

.. _optimizer_task_putnastrparam:

``put_na_str_param()``
----------------------

.. code-block:: rust

    pub fn put_na_str_param ( &self,
                              paramname : &str,
                              parvalue  : &str) -> Result<(),String>

``paramname``
    
``parvalue``
    


.. index:: put_obj_name

.. _optimizer_task_putobjname:

``put_obj_name()``
------------------

.. code-block:: rust

    pub fn put_obj_name ( &self,objname : &str ) -> Result<(),String>

``objname``
    


.. index:: put_obj_sense

.. _optimizer_task_putobjsense:

``put_obj_sense()``
-------------------

.. code-block:: rust

    pub fn put_obj_sense ( &self,sense : i32 ) -> Result<(),String>

``sense``
    


.. index:: put_param

.. _optimizer_task_putparam:

``put_param()``
---------------

.. code-block:: rust

    pub fn put_param ( &self,
                       parname  : &str,
                       parvalue : &str) -> Result<(),String>

``parname``
    
``parvalue``
    


.. index:: put_q_con

.. _optimizer_task_putqcon:

``put_q_con()``
---------------

.. code-block:: rust

    pub fn put_q_con ( &self,
                       qcsubk_ : & [i32],
                       qcsubi_ : & [i32],
                       qcsubj_ : & [i32],
                       qcval_  : & [f64]) -> Result<(),String>

``qcsubk``
    
``qcsubi``
    
``qcsubj``
    
``qcval``
    


.. index:: put_q_con_k

.. _optimizer_task_putqconk:

``put_q_con_k()``
-----------------

.. code-block:: rust

    pub fn put_q_con_k ( &self,
                         k       : i32,
                         qcsubi_ : & [i32],
                         qcsubj_ : & [i32],
                         qcval_  : & [f64]) -> Result<(),String>

``k``
    
``qcsubi``
    
``qcsubj``
    
``qcval``
    


.. index:: put_q_obj

.. _optimizer_task_putqobj:

``put_q_obj()``
---------------

.. code-block:: rust

    pub fn put_q_obj ( &self,
                       qosubi_ : & [i32],
                       qosubj_ : & [i32],
                       qoval_  : & [f64]) -> Result<(),String>

``qosubi``
    
``qosubj``
    
``qoval``
    


.. index:: put_q_obj_i_j

.. _optimizer_task_putqobjij:

``put_q_obj_i_j()``
-------------------

.. code-block:: rust

    pub fn put_q_obj_i_j ( &self,
                           i     : i32,
                           j     : i32,
                           qoij  : f64) -> Result<(),String>

``i``
    
``j``
    
``qoij``
    


.. index:: put_skc

.. _optimizer_task_putskc:

``put_skc()``
-------------

.. code-block:: rust

    pub fn put_skc ( &self,
                     whichsol : i32,
                     skc_     : & [i32]) -> Result<(),String>

``whichsol``
    
``skc``
    


.. index:: put_skc_slice

.. _optimizer_task_putskcslice:

``put_skc_slice()``
-------------------

.. code-block:: rust

    pub fn put_skc_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           skc_     : & [i32]) -> Result<(),String>

``whichsol``
    
``first``
    
``last``
    
``skc``
    


.. index:: put_skx

.. _optimizer_task_putskx:

``put_skx()``
-------------

.. code-block:: rust

    pub fn put_skx ( &self,
                     whichsol : i32,
                     skx_     : & [i32]) -> Result<(),String>

``whichsol``
    
``skx``
    


.. index:: put_skx_slice

.. _optimizer_task_putskxslice:

``put_skx_slice()``
-------------------

.. code-block:: rust

    pub fn put_skx_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           skx_     : & [i32]) -> Result<(),String>

``whichsol``
    
``first``
    
``last``
    
``skx``
    


.. index:: put_slc

.. _optimizer_task_putslc:

``put_slc()``
-------------

.. code-block:: rust

    pub fn put_slc ( &self,
                     whichsol : i32,
                     slc_     : & [f64]) -> Result<(),String>

``whichsol``
    
``slc``
    


.. index:: put_slc_slice

.. _optimizer_task_putslcslice:

``put_slc_slice()``
-------------------

.. code-block:: rust

    pub fn put_slc_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           slc_     : & [f64]) -> Result<(),String>

``whichsol``
    
``first``
    
``last``
    
``slc``
    


.. index:: put_slx

.. _optimizer_task_putslx:

``put_slx()``
-------------

.. code-block:: rust

    pub fn put_slx ( &self,
                     whichsol : i32,
                     slx_     : & [f64]) -> Result<(),String>

``whichsol``
    
``slx``
    


.. index:: put_slx_slice

.. _optimizer_task_putslxslice:

``put_slx_slice()``
-------------------

.. code-block:: rust

    pub fn put_slx_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           slx_     : & [f64]) -> Result<(),String>

``whichsol``
    
``first``
    
``last``
    
``slx``
    


.. index:: put_snx

.. _optimizer_task_putsnx:

``put_snx()``
-------------

.. code-block:: rust

    pub fn put_snx ( &self,
                     whichsol : i32,
                     sux_     : & [f64]) -> Result<(),String>

``whichsol``
    
``sux``
    


.. index:: put_snx_slice

.. _optimizer_task_putsnxslice:

``put_snx_slice()``
-------------------

.. code-block:: rust

    pub fn put_snx_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           snx_     : & [f64]) -> Result<(),String>

``whichsol``
    
``first``
    
``last``
    
``snx``
    


.. index:: put_solution

.. _optimizer_task_putsolution:

``put_solution()``
------------------

.. code-block:: rust

    pub fn put_solution ( &self,
                          whichsol : i32,
                          skc_     : & [i32],
                          skx_     : & [i32],
                          skn_     : & [i32],
                          xc_      : & [f64],
                          xx_      : & [f64],
                          y_       : & [f64],
                          slc_     : & [f64],
                          suc_     : & [f64],
                          slx_     : & [f64],
                          sux_     : & [f64],
                          snx_     : & [f64]) -> Result<(),String>

``whichsol``
    
``skc``
    
``skx``
    
``skn``
    
``xc``
    
``xx``
    
``y``
    
``slc``
    
``suc``
    
``slx``
    
``sux``
    
``snx``
    


.. index:: put_solution_y_i

.. _optimizer_task_putsolutionyi:

``put_solution_y_i()``
----------------------

.. code-block:: rust

    pub fn put_solution_y_i ( &self,
                              i        : i32,
                              whichsol : i32,
                              y        : f64) -> Result<(),String>

``i``
    
``whichsol``
    
``y``
    


.. index:: put_str_param

.. _optimizer_task_putstrparam:

``put_str_param()``
-------------------

.. code-block:: rust

    pub fn put_str_param ( &self,
                           param    : i32,
                           parvalue : &str) -> Result<(),String>

``param``
    
``parvalue``
    


.. index:: put_stream_callback
``put_stream_callback()``
-------------------------

.. code-block:: rust

    pub fn put_stream_callback(& mut self,
                               whichstream : i32,
                               func        : fn(&H,&String),
                               handle      : H)

Sets the callback function and handle for the given stream in the
``Task``. Note that ownership of the handle is passed to the ``Task``.

``whichstream``
    Which stream to link to (see :ref:`streamtype`).
``func``
    A printer function. This takes the object specified in ``handle``
    and a string.
``handle``
    An object of type ``H``, as defined from ``Task<H>``.



.. index:: put_suc

.. _optimizer_task_putsuc:

``put_suc()``
-------------

.. code-block:: rust

    pub fn put_suc ( &self,
                     whichsol : i32,
                     suc_     : & [f64]) -> Result<(),String>

``whichsol``
    
``suc``
    


.. index:: put_suc_slice

.. _optimizer_task_putsucslice:

``put_suc_slice()``
-------------------

.. code-block:: rust

    pub fn put_suc_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           suc_     : & [f64]) -> Result<(),String>

``whichsol``
    
``first``
    
``last``
    
``suc``
    


.. index:: put_sux

.. _optimizer_task_putsux:

``put_sux()``
-------------

.. code-block:: rust

    pub fn put_sux ( &self,
                     whichsol : i32,
                     sux_     : & [f64]) -> Result<(),String>

``whichsol``
    
``sux``
    


.. index:: put_sux_slice

.. _optimizer_task_putsuxslice:

``put_sux_slice()``
-------------------

.. code-block:: rust

    pub fn put_sux_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           sux_     : & [f64]) -> Result<(),String>

``whichsol``
    
``first``
    
``last``
    
``sux``
    


.. index:: put_task_name

.. _optimizer_task_puttaskname:

``put_task_name()``
-------------------

.. code-block:: rust

    pub fn put_task_name ( &self,taskname : &str ) -> Result<(),String>

``taskname``
    


.. index:: put_var_bound

.. _optimizer_task_putvarbound:

``put_var_bound()``
-------------------

.. code-block:: rust

    pub fn put_var_bound ( &self,
                           j     : i32,
                           bkx   : i32,
                           blx   : f64,
                           bux   : f64) -> Result<(),String>

``j``
    
``bkx``
    
``blx``
    
``bux``
    


.. index:: put_var_bound_list

.. _optimizer_task_putvarboundlist:

``put_var_bound_list()``
------------------------

.. code-block:: rust

    pub fn put_var_bound_list ( &self,
                                sub_  : & [i32],
                                bkx_  : & [i32],
                                blx_  : & [f64],
                                bux_  : & [f64]) -> Result<(),String>

``sub``
    
``bkx``
    
``blx``
    
``bux``
    


.. index:: put_var_bound_list_const

.. _optimizer_task_putvarboundlistconst:

``put_var_bound_list_const()``
------------------------------

.. code-block:: rust

    pub fn put_var_bound_list_const ( &self,
                                      sub_  : & [i32],
                                      bkx   : i32,
                                      blx   : f64,
                                      bux   : f64) -> Result<(),String>

``sub``
    
``bkx``
    
``blx``
    
``bux``
    


.. index:: put_var_bound_slice

.. _optimizer_task_putvarboundslice:

``put_var_bound_slice()``
-------------------------

.. code-block:: rust

    pub fn put_var_bound_slice ( &self,
                                 first : i32,
                                 last  : i32,
                                 bkx_  : & [i32],
                                 blx_  : & [f64],
                                 bux_  : & [f64]) -> Result<(),String>

``first``
    
``last``
    
``bkx``
    
``blx``
    
``bux``
    


.. index:: put_var_bound_slice_const

.. _optimizer_task_putvarboundsliceconst:

``put_var_bound_slice_const()``
-------------------------------

.. code-block:: rust

    pub fn put_var_bound_slice_const ( &self,
                                       first : i32,
                                       last  : i32,
                                       bkx   : i32,
                                       blx   : f64,
                                       bux   : f64) -> Result<(),String>

``first``
    
``last``
    
``bkx``
    
``blx``
    
``bux``
    


.. index:: put_var_name

.. _optimizer_task_putvarname:

``put_var_name()``
------------------

.. code-block:: rust

    pub fn put_var_name ( &self,
                          j     : i32,
                          name  : &str) -> Result<(),String>

``j``
    
``name``
    


.. index:: put_var_solution_j

.. _optimizer_task_putvarsolutionj:

``put_var_solution_j()``
------------------------

.. code-block:: rust

    pub fn put_var_solution_j ( &self,
                                j        : i32,
                                whichsol : i32,
                                sk       : i32,
                                x        : f64,
                                sl       : f64,
                                su       : f64,
                                sn       : f64) -> Result<(),String>

``j``
    
``whichsol``
    
``sk``
    
``x``
    
``sl``
    
``su``
    
``sn``
    


.. index:: put_var_type

.. _optimizer_task_putvartype:

``put_var_type()``
------------------

.. code-block:: rust

    pub fn put_var_type ( &self,
                          j       : i32,
                          vartype : i32) -> Result<(),String>

``j``
    
``vartype``
    


.. index:: put_var_type_list

.. _optimizer_task_putvartypelist:

``put_var_type_list()``
-----------------------

.. code-block:: rust

    pub fn put_var_type_list ( &self,
                               subj_    : & [i32],
                               vartype_ : & [i32]) -> Result<(),String>

``subj``
    
``vartype``
    


.. index:: put_xc

.. _optimizer_task_putxc:

``put_xc()``
------------

.. code-block:: rust

    pub fn put_xc ( &self,
                    whichsol : i32,
                    xc       : & mut [f64]) -> Result<(),String>

``whichsol``
    
``xc``
    


.. index:: put_xc_slice

.. _optimizer_task_putxcslice:

``put_xc_slice()``
------------------

.. code-block:: rust

    pub fn put_xc_slice ( &self,
                          whichsol : i32,
                          first    : i32,
                          last     : i32,
                          xc_      : & [f64]) -> Result<(),String>

``whichsol``
    
``first``
    
``last``
    
``xc``
    


.. index:: put_xx

.. _optimizer_task_putxx:

``put_xx()``
------------

.. code-block:: rust

    pub fn put_xx ( &self,
                    whichsol : i32,
                    xx_      : & [f64]) -> Result<(),String>

``whichsol``
    
``xx``
    


.. index:: put_xx_slice

.. _optimizer_task_putxxslice:

``put_xx_slice()``
------------------

.. code-block:: rust

    pub fn put_xx_slice ( &self,
                          whichsol : i32,
                          first    : i32,
                          last     : i32,
                          xx_      : & [f64]) -> Result<(),String>

``whichsol``
    
``first``
    
``last``
    
``xx``
    


.. index:: put_y

.. _optimizer_task_puty:

``put_y()``
-----------

.. code-block:: rust

    pub fn put_y ( &self,
                   whichsol : i32,
                   y_       : & [f64]) -> Result<(),String>

``whichsol``
    
``y``
    


.. index:: put_y_slice

.. _optimizer_task_putyslice:

``put_y_slice()``
-----------------

.. code-block:: rust

    pub fn put_y_slice ( &self,
                         whichsol : i32,
                         first    : i32,
                         last     : i32,
                         y_       : & [f64]) -> Result<(),String>

``whichsol``
    
``first``
    
``last``
    
``y``
    


.. index:: read_data

.. _optimizer_task_readdataautoformat:

``read_data()``
---------------

.. code-block:: rust

    pub fn read_data ( &self,filename : &str ) -> Result<(),String>

``filename``
    


.. index:: read_data_format

.. _optimizer_task_readdataformat:

``read_data_format()``
----------------------

.. code-block:: rust

    pub fn read_data_format ( &self,
                              filename : &str,
                              format   : i32,
                              compress : i32) -> Result<(),String>

``filename``
    
``format``
    
``compress``
    


.. index:: read_json_string

.. _optimizer_task_readjsonstring:

``read_json_string()``
----------------------

.. code-block:: rust

    pub fn read_json_string ( &self,data  : &str ) -> Result<(),String>

``data``
    


.. index:: read_lp_string

.. _optimizer_task_readlpstring:

``read_lp_string()``
--------------------

.. code-block:: rust

    pub fn read_lp_string ( &self,data  : &str ) -> Result<(),String>

``data``
    


.. index:: read_opf_string

.. _optimizer_task_readopfstring:

``read_opf_string()``
---------------------

.. code-block:: rust

    pub fn read_opf_string ( &self,data  : &str ) -> Result<(),String>

``data``
    


.. index:: read_param_file

.. _optimizer_task_readparamfile:

``read_param_file()``
---------------------

.. code-block:: rust

    pub fn read_param_file ( &self,filename : &str ) -> Result<(),String>

``filename``
    


.. index:: read_ptf_string

.. _optimizer_task_readptfstring:

``read_ptf_string()``
---------------------

.. code-block:: rust

    pub fn read_ptf_string ( &self,data  : &str ) -> Result<(),String>

``data``
    


.. index:: read_solution

.. _optimizer_task_readsolution:

``read_solution()``
-------------------

.. code-block:: rust

    pub fn read_solution ( &self,
                           whichsol : i32,
                           filename : &str) -> Result<(),String>

``whichsol``
    
``filename``
    


.. index:: read_summary

.. _optimizer_task_readsummary:

``read_summary()``
------------------

.. code-block:: rust

    pub fn read_summary ( &self,whichstream : i32 ) -> Result<(),String>

``whichstream``
    


.. index:: read_task

.. _optimizer_task_readtask:

``read_task()``
---------------

.. code-block:: rust

    pub fn read_task ( &self,filename : &str ) -> Result<(),String>

``filename``
    


.. index:: remove_barvars

.. _optimizer_task_removebarvars:

``remove_barvars()``
--------------------

.. code-block:: rust

    pub fn remove_barvars ( &self,subset_ : & [i32] ) -> Result<(),String>

``subset``
    


.. index:: remove_cones

.. _optimizer_task_removecones:

``remove_cones()``
------------------

.. code-block:: rust

    pub fn remove_cones ( &self,subset_ : & [i32] ) -> Result<(),String>

``subset``
    


.. index:: remove_cons

.. _optimizer_task_removecons:

``remove_cons()``
-----------------

.. code-block:: rust

    pub fn remove_cons ( &self,subset_ : & [i32] ) -> Result<(),String>

``subset``
    


.. index:: remove_vars

.. _optimizer_task_removevars:

``remove_vars()``
-----------------

.. code-block:: rust

    pub fn remove_vars ( &self,subset_ : & [i32] ) -> Result<(),String>

``subset``
    


.. index:: resize_task

.. _optimizer_task_resizetask:

``resize_task()``
-----------------

.. code-block:: rust

    pub fn resize_task ( &self,
                         maxnumcon  : i32,
                         maxnumvar  : i32,
                         maxnumcone : i32,
                         maxnumanz  : i64,
                         maxnumqnz  : i64) -> Result<(),String>

``maxnumcon``
    
``maxnumvar``
    
``maxnumcone``
    
``maxnumanz``
    
``maxnumqnz``
    


.. index:: sensitivity_report

.. _optimizer_task_sensitivityreport:

``sensitivity_report()``
------------------------

.. code-block:: rust

    pub fn sensitivity_report ( &self,whichstream : i32 ) -> Result<(),String>

``whichstream``
    


.. index:: set_defaults

.. _optimizer_task_setdefaults:

``set_defaults()``
------------------

.. code-block:: rust

    pub fn set_defaults ( &self ) -> Result<(),String>



.. index:: sk_to_str

.. _optimizer_task_sktostr:

``sk_to_str()``
---------------

.. code-block:: rust

    pub fn sk_to_str ( &self,sk    : i32 ) -> Result<String,String>

``sk``
    
*Returns:* ``str``
    ``str : String``
        


.. index:: sol_sta_to_str

.. _optimizer_task_solstatostr:

``sol_sta_to_str()``
--------------------

.. code-block:: rust

    pub fn sol_sta_to_str ( &self,solsta : i32 ) -> Result<String,String>

``solsta``
    
*Returns:* ``str``
    ``str : String``
        


.. index:: solution_def

.. _optimizer_task_solutiondef:

``solution_def()``
------------------

.. code-block:: rust

    pub fn solution_def ( &self,whichsol : i32 ) -> Result<bool,String>

``whichsol``
    
*Returns:* ``isdef``
    ``isdef : bool``
        


.. index:: solution_summary

.. _optimizer_task_solutionsummary:

``solution_summary()``
----------------------

.. code-block:: rust

    pub fn solution_summary ( &self,whichstream : i32 ) -> Result<(),String>

``whichstream``
    


.. index:: solve_with_basis

.. _optimizer_task_solvewithbasis:

``solve_with_basis()``
----------------------

.. code-block:: rust

    pub fn solve_with_basis ( &self,
                              transp : i32,
                              numnz  : i32,
                              sub    : & mut [i32],
                              val    : & mut [f64]) -> Result<i32,String>

``transp``
    
``numnz``
    
``sub``
    
``val``
    
*Returns:* ``numnz``
    ``numnz : i32``
        


.. index:: str_dup_task

.. _optimizer_task_strduptask:

``str_dup_task()``
------------------

.. code-block:: rust

    pub fn str_dup_task ( &self,str   : &str ) -> Result<(),String>

``str``
    


.. index:: str_to_cone_type

.. _optimizer_task_strtoconetype:

``str_to_cone_type()``
----------------------

.. code-block:: rust

    pub fn str_to_cone_type ( &self,str   : &str ) -> Result<i32,String>

``str``
    
*Returns:* ``conetype``
    ``conetype : i32``
        


.. index:: str_to_sk

.. _optimizer_task_strtosk:

``str_to_sk()``
---------------

.. code-block:: rust

    pub fn str_to_sk ( &self,str   : &str ) -> Result<i32,String>

``str``
    
*Returns:* ``sk``
    ``sk : i32``
        


.. index:: toconic

.. _optimizer_task_toconic:

``toconic()``
-------------

.. code-block:: rust

    pub fn toconic ( &self ) -> Result<(),String>



.. index:: unlink_func_from_stream

.. _optimizer_task_unlinkfuncfromtaskstream:

``unlink_func_from_stream()``
-----------------------------

.. code-block:: rust

    pub fn unlink_func_from_stream ( &self,whichstream : i32 ) -> Result<(),String>

``whichstream``
    


.. index:: update_solution_info

.. _optimizer_task_updatesolutioninfo:

``update_solution_info()``
--------------------------

.. code-block:: rust

    pub fn update_solution_info ( &self,whichsol : i32 ) -> Result<(),String>

``whichsol``
    


.. index:: which_param

.. _optimizer_task_whichparam:

``which_param()``
-----------------

.. code-block:: rust

    pub fn which_param ( &self,parname : &str ) -> Result<(i32,i32),String>

``parname``
    
*Returns:* ``(partype,param)``
    ``partype : i32``
        
    ``param : i32``
        


.. index:: write_data

.. _optimizer_task_writedata:

``write_data()``
----------------

.. code-block:: rust

    pub fn write_data ( &self,filename : &str ) -> Result<(),String>

``filename``
    


.. index:: write_json_sol

.. _optimizer_task_writejsonsol:

``write_json_sol()``
--------------------

.. code-block:: rust

    pub fn write_json_sol ( &self,filename : &str ) -> Result<(),String>

``filename``
    


.. index:: write_param_file

.. _optimizer_task_writeparamfile:

``write_param_file()``
----------------------

.. code-block:: rust

    pub fn write_param_file ( &self,filename : &str ) -> Result<(),String>

``filename``
    


.. index:: write_solution

.. _optimizer_task_writesolution:

``write_solution()``
--------------------

.. code-block:: rust

    pub fn write_solution ( &self,
                            whichsol : i32,
                            filename : &str) -> Result<(),String>

``whichsol``
    
``filename``
    


.. index:: write_task

.. _optimizer_task_writetask:

``write_task()``
----------------

.. code-block:: rust

    pub fn write_task ( &self,filename : &str ) -> Result<(),String>

``filename``
    

