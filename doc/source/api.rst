
.. |mosek| replace:: MOSEK
.. |null| replace:: ``None``

Module level functions
======================

.. index:: get_code_desc

.. _optimizer_env_getcodedesc:

``get_code_desc()``
-------------------

.. code-block:: rust

    pub fn get_code_desc ( code : i32 ) -> (String,String)

``code``
    A valid response code.
*Returns:* ``(symname,str)``
    ``symname : String``
        Symbolic name corresponding to the code.
    ``str : String``
        Obtains a short description of a response code.

Obtains a short description of a response code.

.. index:: get_version

.. _optimizer_env_getversion:

``get_version()``
-----------------

.. code-block:: rust

    pub fn get_version (  ) -> (i32,i32,i32,i32)

*Returns:* ``(major,minor,build,revision)``
    ``major : i32``
        Major version number.
    ``minor : i32``
        Minor version number.
    ``build : i32``
        Build number.
    ``revision : i32``
        Revision number.

Obtains |mosek| version information.

.. index:: licensecleanup

.. _optimizer_env_licensecleanup:

``licensecleanup()``
--------------------

.. code-block:: rust

    pub fn licensecleanup (  )


Stops all threads and delete all handles used by the license system.
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
                  y     : & mut [f64] )

``n``
    Length of the vectors.
``alpha``
    The scalar that multiplies x.
``x``
    The :math:`x` vector.
``y``
    The :math:`y` vector.

Adds alpha times x to y.

.. index:: check_in_all

.. _optimizer_env_checkinall:

``check_in_all()``
------------------

.. code-block:: rust

    pub fn check_in_all ( &self )


Check in all unsued license features to the license token server.

.. index:: check_in_license

.. _optimizer_env_checkinlicense:

``check_in_license()``
----------------------

.. code-block:: rust

    pub fn check_in_license ( &self,feature : i32 )

``feature``
    Feature to check in to the license system.

Check in a license feature from the license server ahead of time.

.. index:: checkout_license

.. _optimizer_env_checkoutlicense:

``checkout_license()``
----------------------

.. code-block:: rust

    pub fn checkout_license ( &self,feature : i32 )

``feature``
    Feature to check out from the license system.

Check out a license feature from the license server ahead of time.

.. index:: dot

.. _optimizer_env_dot:

``dot()``
---------

.. code-block:: rust

    pub fn dot ( &self,
                 n     : i32,
                 x_    : & [f64],
                 y_    : & [f64] )

``n``
    Length of the vectors.
``x``
    The x vector.
``y``
    The y vector.
*Returns:* ``xty``
    ``xty : f64``
        The result of the inner product.

Computes the inner product of two vectors.

.. index:: echo_intro

.. _optimizer_env_echointro:

``echo_intro()``
----------------

.. code-block:: rust

    pub fn echo_intro ( &self,longver : i32 )

``longver``
    If non-zero, then the intro is slightly longer.

Prints an intro to message stream.

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
                  c      : & mut [f64] )

``transa``
    Indicates whether the matrix A must be transposed.
``transb``
    Indicates whether the matrix B must be transposed.
``m``
    Indicates the number of rows of matrices A and C.
``n``
    Indicates the number of columns of matrices B and C.
``k``
    Specifies the number of columns of the matrix A and the number of rows of the matrix B.
``alpha``
    A scalar value multipling the result of the matrix multiplication.
``a``
    The pointer to the array storing matrix A in a column-major format.
``b``
    Indicates the number of rows of matrix B and columns of matrix A.
``beta``
    A scalar value that multiplies C.
``c``
    The pointer to the array storing matrix C in a column-major format.

Performs a dense matrix multiplication.

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
                  y      : & mut [f64] )

``transa``
    Indicates whether the matrix A must be transposed.
``m``
    Specifies the number of rows of the matrix A.
``n``
    Specifies the number of columns of the matrix A.
``alpha``
    A scalar value multipling the matrix A.
``a``
    A pointer to the array storing matrix A in a column-major format.
``x``
    A pointer to the array storing the vector x.
``beta``
    A scalar value multipling thevector y.
``y``
    A pointer to the array storing the vector y.

Computes dense matrix times a dense vector product.

.. index:: linkfiletostream

.. _optimizer_env_linkfiletoenvstream:

``linkfiletostream()``
----------------------

.. code-block:: rust

    pub fn linkfiletostream ( &self,
                              whichstream : i32,
                              filename    : &str,
                              append      : i32 )

``whichstream``
    
``filename``
    Name of the file to write stream data to.
``append``
    If this argument is non-zero, the output is appended to the file.

Directs all output from a stream to a file.

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
                   a     : & mut [f64] )

``uplo``
    Indicates whether the upper or lower triangular part of the matrix is stored.
``n``
    Dimension of the symmetric matrix.
``a``
    A symmetric matrix stored in column-major order. Only the lower or the upper triangular part is used, accordingly with the uplo parameter. It will contain the result on exit.

Computes a Cholesky factorization a dense matrix.

.. index:: put_license_code

.. _optimizer_env_putlicensecode:

``put_license_code()``
----------------------

.. code-block:: rust

    pub fn put_license_code ( &self,code_ : & [i32] )

``code``
    A license key string.

The purpose of this function is to input a runtime license code.

.. index:: put_license_debug

.. _optimizer_env_putlicensedebug:

``put_license_debug()``
-----------------------

.. code-block:: rust

    pub fn put_license_debug ( &self,licdebug : i32 )

``licdebug``
    Enable output of license check-out debug information.

Enables debug information for the license system.

.. index:: put_license_path

.. _optimizer_env_putlicensepath:

``put_license_path()``
----------------------

.. code-block:: rust

    pub fn put_license_path ( &self,licensepath : &str )

``licensepath``
    A path specifycing where to search for the license.

Set the path to the license file.

.. index:: put_license_wait

.. _optimizer_env_putlicensewait:

``put_license_wait()``
----------------------

.. code-block:: rust

    pub fn put_license_wait ( &self,licwait : i32 )

``licwait``
    Enable waiting for a license.

Control whether mosek should wait for an available license if no license is available.

.. index:: syeig

.. _optimizer_env_syeig:

``syeig()``
-----------

.. code-block:: rust

    pub fn syeig ( &self,
                   uplo  : i32,
                   n     : i32,
                   a_    : & [f64],
                   w     : & mut [f64] )

``uplo``
    Indicates whether the upper or lower triangular part is used.
``n``
    Dimension of the symmetric input matrix.
``a``
    A symmetric matrix stored in column-major order. Only the lower-triangular part is used.
``w``
    Array of minimum dimension n where eigenvalues will be stored.

Computes all eigenvalues of a symmetric dense matrix.

.. index:: syevd

.. _optimizer_env_syevd:

``syevd()``
-----------

.. code-block:: rust

    pub fn syevd ( &self,
                   uplo  : i32,
                   n     : i32,
                   a     : & mut [f64],
                   w     : & mut [f64] )

``uplo``
    Indicates whether the upper or lower triangular part is used.
``n``
    Dimension of symmetric input matrix.
``a``
    A symmetric matrix stored in column-major order. Only the lower-triangular part is used. It will be overwritten on exit.
``w``
    An array where eigenvalues will be stored. Its lenght must be at least the dimension of the input matrix.

Computes all the eigenvalue and eigenvectors of a symmetric dense matrix, and thus its eigenvalue decomposition.

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
                  c     : & mut [f64] )

``uplo``
    Indicates whether the upper or lower triangular part of C is stored.
``trans``
    Indicates whether the matrix A must be transposed.
``n``
    Specifies the order of C.
``k``
    Indicates the number of rows or columns of A, and its rank.
``alpha``
    A scalar value multipling the result of the matrix multiplication.
``a``
    The pointer to the array storing matrix A in a column-major format.
``beta``
    A scalar value that multiplies C.
``c``
    The pointer to the array storing matrix C in a column-major format.

Performs a rank-k update of a symmetric matrix.

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

Task methods
============

.. index:: analyze_names

.. _optimizer_task_analyzenames:

``analyze_names()``
-------------------

.. code-block:: rust

    pub fn analyze_names ( &self,
                           whichstream : i32,
                           nametype    : i32 )

``whichstream``
    
``nametype``
    The type of names e.g. valid in MPS or LP files.

Analyze the names and issue an error for the first invalid name.

.. index:: analyze_problem

.. _optimizer_task_analyzeproblem:

``analyze_problem()``
---------------------

.. code-block:: rust

    pub fn analyze_problem ( &self,whichstream : i32 )

``whichstream``
    

Analyze the data of a task.

.. index:: analyze_solution

.. _optimizer_task_analyzesolution:

``analyze_solution()``
----------------------

.. code-block:: rust

    pub fn analyze_solution ( &self,
                              whichstream : i32,
                              whichsol    : i32 )

``whichstream``
    
``whichsol``
    

Print information related to the quality of the solution.

.. index:: append_barvars

.. _optimizer_task_appendbarvars:

``append_barvars()``
--------------------

.. code-block:: rust

    pub fn append_barvars ( &self,dim_  : & [i32] )

``dim``
    Dimension of symmetric matrix variables to be added.

Appends a semidefinite  variable of dimension dim to the problem.

.. index:: append_cone

.. _optimizer_task_appendcone:

``append_cone()``
-----------------

.. code-block:: rust

    pub fn append_cone ( &self,
                         ct      : i32,
                         conepar : f64,
                         submem_ : & [i32] )

``ct``
    
``conepar``
    
``submem``
    

Appends a new cone constraint to the problem.

.. index:: append_cone_seq

.. _optimizer_task_appendconeseq:

``append_cone_seq()``
---------------------

.. code-block:: rust

    pub fn append_cone_seq ( &self,
                             ct      : i32,
                             conepar : f64,
                             nummem  : i32,
                             j       : i32 )

``ct``
    
``conepar``
    
``nummem``
    Dimension of the conic constraint.
``j``
    Index of the first variable in the conic constraint.

Appends a new conic constraint to the problem.

.. index:: append_cones_seq

.. _optimizer_task_appendconesseq:

``append_cones_seq()``
----------------------

.. code-block:: rust

    pub fn append_cones_seq ( &self,
                              ct_      : & [i32],
                              conepar_ : & [f64],
                              nummem_  : & [i32],
                              j        : i32 )

``ct``
    
``conepar``
    
``nummem``
    
``j``
    Index of the first variable in the first cone to be appended.

Appends a multiple conic constraints to the problem.

.. index:: append_cons

.. _optimizer_task_appendcons:

``append_cons()``
-----------------

.. code-block:: rust

    pub fn append_cons ( &self,num   : i32 )

``num``
    Number of constraints which should be appended.

Appends a number of constraints to the optimization task.

.. index:: append_sparse_sym_mat

.. _optimizer_task_appendsparsesymmat:

``append_sparse_sym_mat()``
---------------------------

.. code-block:: rust

    pub fn append_sparse_sym_mat ( &self,
                                   dim    : i32,
                                   subi_  : & [i32],
                                   subj_  : & [i32],
                                   valij_ : & [f64] )

``dim``
    Dimension of the symmetric matrix that is appended.
``subi``
    Row subscript in the triplets.
``subj``
    Column subscripts in the triplets.
``valij``
    Values of each triplet.
*Returns:* ``idx``
    ``idx : i64``
        Unique index assigned to inputted matrix.

Appends a general sparse symmetric matrix to the vector E of symmetric matrixes.

.. index:: append_vars

.. _optimizer_task_appendvars:

``append_vars()``
-----------------

.. code-block:: rust

    pub fn append_vars ( &self,num   : i32 )

``num``
    Number of variables which should be appended.

Appends a number of variables to the optimization task.

.. index:: basis_cond

.. _optimizer_task_basiscond:

``basis_cond()``
----------------

.. code-block:: rust

    pub fn basis_cond ( &self ) -> (f64,f64)

*Returns:* ``(nrmbasis,nrminvbasis)``
    ``nrmbasis : f64``
        An estimate for the 1 norm of the basis.
    ``nrminvbasis : f64``
        An estimate for the 1 norm of the inverse of the basis.

Computes conditioning information for the basis matrix.

.. index:: check_convexity

.. _optimizer_task_checkconvexity:

``check_convexity()``
---------------------

.. code-block:: rust

    pub fn check_convexity ( &self )


Checks if a quadratic optimization problem is convex.

.. index:: check_mem

.. _optimizer_task_checkmemtask:

``check_mem()``
---------------

.. code-block:: rust

    pub fn check_mem ( &self,
                       file  : &str,
                       line  : i32 )

``file``
    File from which the function is called.
``line``
    Line in the file from which the function is called.

Checks the memory allocated by the task.

.. index:: chg_bound

.. _optimizer_task_chgbound:

``chg_bound()``
---------------

.. code-block:: rust

    pub fn chg_bound ( &self,
                       accmode : i32,
                       i       : i32,
                       lower   : i32,
                       finite  : i32,
                       value   : f64 )

``accmode``
    
``i``
    Index of the constraint or variable for which the bounds should be changed.
``lower``
    If non-zero, then the lower bound is changed, otherwise
                                the upper bound is changed.
``finite``
    If non-zero, then the given value is assumed to be finite.
``value``
    New value for the bound.

Changes the bounds for one constraint or variable.

.. index:: chg_con_bound

.. _optimizer_task_chgconbound:

``chg_con_bound()``
-------------------

.. code-block:: rust

    pub fn chg_con_bound ( &self,
                           i      : i32,
                           lower  : i32,
                           finite : i32,
                           value  : f64 )

``i``
    Index of the constraint for which the bounds should be changed.
``lower``
    If non-zero, then the lower bound is changed, otherwise the upper bound is changed.
``finite``
    If non-zero, then the given value is assumed to be finite.
``value``
    New value for the bound.

Changes the bounds for one constraint.

.. index:: chg_var_bound

.. _optimizer_task_chgvarbound:

``chg_var_bound()``
-------------------

.. code-block:: rust

    pub fn chg_var_bound ( &self,
                           j      : i32,
                           lower  : i32,
                           finite : i32,
                           value  : f64 )

``j``
    Index of the variable for which the bounds should be changed.
``lower``
    If non-zero, then the lower bound is changed, otherwise
                                the upper bound is changed.
``finite``
    If non-zero, then the given value is assumed to be finite.
``value``
    New value for the bound.

Changes the bounds for one variable.

.. index:: commit_changes

.. _optimizer_task_commitchanges:

``commit_changes()``
--------------------

.. code-block:: rust

    pub fn commit_changes ( &self )


Commits all cached problem changes.

.. index:: delete_solution

.. _optimizer_task_deletesolution:

``delete_solution()``
---------------------

.. code-block:: rust

    pub fn delete_solution ( &self,whichsol : i32 )

``whichsol``
    

Undefine a solution and frees the memory it uses.

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
                              rightrangej : & mut [f64] )

``subj``
    Index of objective coefficients to analyze.
``leftpricej``
    Left shadow prices for requested coefficients.
``rightpricej``
    Right shadow prices for requested coefficients.
``leftrangej``
    Left range for requested coefficients.
``rightrangej``
    Right range for requested coefficients.

Performs sensitivity analysis on objective coefficients.

.. index:: get_a_col

.. _optimizer_task_getacol:

``get_a_col()``
---------------

.. code-block:: rust

    pub fn get_a_col ( &self,
                       j     : i32,
                       subj  : & mut [i32],
                       valj  : & mut [f64] )

``j``
    Index of the column.
``subj``
    Index of the non-zeros in the row obtained.
``valj``
    Numerical values of the column obtained.
*Returns:* ``nzj``
    ``nzj : i32``
        Number of non-zeros in the column obtained.

Obtains one column of the linear constraint matrix.

.. index:: get_a_col_num_nz

.. _optimizer_task_getacolnumnz:

``get_a_col_num_nz()``
----------------------

.. code-block:: rust

    pub fn get_a_col_num_nz ( &self,i     : i32 ) -> i32

``i``
    Index of the column.
*Returns:* ``nzj``
    ``nzj : i32``
        Number of non-zeros in the j'th row or column of (A).

Obtains the number of non-zero elements in one column of the linear constraint matrix

.. index:: get_a_piece_num_nz

.. _optimizer_task_getapiecenumnz:

``get_a_piece_num_nz()``
------------------------

.. code-block:: rust

    pub fn get_a_piece_num_nz ( &self,
                                firsti : i32,
                                lasti  : i32,
                                firstj : i32,
                                lastj  : i32 )

``firsti``
    Index of the first row in the rectangular piece.
``lasti``
    Index of the last row plus one in the rectangular piece.
``firstj``
    Index of the first column in the rectangular piece.
``lastj``
    Index of the last column plus one in the rectangular piece.
*Returns:* ``numnz``
    ``numnz : i32``
        Number of non-zero elements in the rectangular piece of the linear constraint matrix.

Obtains the number non-zeros in a rectangular piece of the linear constraint matrix.

.. index:: get_a_row

.. _optimizer_task_getarow:

``get_a_row()``
---------------

.. code-block:: rust

    pub fn get_a_row ( &self,
                       i     : i32,
                       subi  : & mut [i32],
                       vali  : & mut [f64] )

``i``
    Index of the row or column.
``subi``
    Index of the non-zeros in the row obtained.
``vali``
    Numerical values of the row obtained.
*Returns:* ``nzi``
    ``nzi : i32``
        Number of non-zeros in the row obtained.

Obtains one row of the linear constraint matrix.

.. index:: get_a_row_num_nz

.. _optimizer_task_getarownumnz:

``get_a_row_num_nz()``
----------------------

.. code-block:: rust

    pub fn get_a_row_num_nz ( &self,i     : i32 ) -> i32

``i``
    Index of the row or column.
*Returns:* ``nzi``
    ``nzi : i32``
        Number of non-zeros in the i'th row of `A`.

Obtains the number of non-zero elements in one row of the linear constraint matrix

.. index:: get_a_slice_num_nz

.. _optimizer_task_getaslicenumnz64:

``get_a_slice_num_nz()``
------------------------

.. code-block:: rust

    pub fn get_a_slice_num_nz ( &self,
                                accmode : i32,
                                first   : i32,
                                last    : i32 )

``accmode``
    Defines whether non-zeros are counted in a column slice or a row slice.
``first``
    Index of the first row or column in the sequence.
``last``
    Index of the last row or column plus one in the sequence.
*Returns:* ``numnz``
    ``numnz : i64``
        Number of non-zeros in the slice.

Obtains the number of non-zeros in a slice of rows or columns of the coefficient matrix.

.. index:: get_aij

.. _optimizer_task_getaij:

``get_aij()``
-------------

.. code-block:: rust

    pub fn get_aij ( &self,
                     i     : i32,
                     j     : i32 )

``i``
    Row index of the coefficient to be returned.
``j``
    Column index of the coefficient to be returned.
*Returns:* ``aij``
    ``aij : f64``
        Returns the requested coefficient.

Obtains a single coefficient in linear constraint matrix.

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
                                    valijkl : & mut [f64] )

``subi``
    Constraint index.
``subj``
    Symmetric matrix variable index.
``subk``
    Block row index.
``subl``
    Block column index.
``valijkl``
    A list indexes of the elements from symmetric matrix storage that appears in the weighted sum.
*Returns:* ``num``
    ``num : i64``
        Number of elements in the block triplet form.

Obtains barA in block triplet form.

.. index:: get_bara_idx

.. _optimizer_task_getbaraidx:

``get_bara_idx()``
------------------

.. code-block:: rust

    pub fn get_bara_idx ( &self,
                          idx     : i64,
                          sub     : & mut [i64],
                          weights : & mut [f64] )

``idx``
    Position of the element in the vectorized form.
``sub``
    A list indexes   of the elements from symmetric matrix storage that appears in the weighted sum.
``weights``
    The weights associated with each term in the weighted sum.
*Returns:* ``(i,j,num)``
    ``i : i32``
        Row index of the element at position idx.
    ``j : i32``
        Column index of the element at position idx.
    ``num : i64``
        Number of terms in weighted sum that forms the element.

Obtains information about an element barA.

.. index:: get_bara_idx_i_j

.. _optimizer_task_getbaraidxij:

``get_bara_idx_i_j()``
----------------------

.. code-block:: rust

    pub fn get_bara_idx_i_j ( &self,idx   : i64 ) -> (i32,i32)

``idx``
    Position of the element in the vectorized form.
*Returns:* ``(i,j)``
    ``i : i32``
        Row index of the element at position idx.
    ``j : i32``
        Column index of the element at position idx.

Obtains information about an element barA.

.. index:: get_bara_idx_info

.. _optimizer_task_getbaraidxinfo:

``get_bara_idx_info()``
-----------------------

.. code-block:: rust

    pub fn get_bara_idx_info ( &self,idx   : i64 ) -> i64

``idx``
    The internal position of the element that should be obtained information for.
*Returns:* ``num``
    ``num : i64``
        Number of terms in the weighted sum that forms the specified element in barA.

Obtains the number terms in the weighted sum that forms a particular element in barA.

.. index:: get_bara_sparsity

.. _optimizer_task_getbarasparsity:

``get_bara_sparsity()``
-----------------------

.. code-block:: rust

    pub fn get_bara_sparsity ( &self,idxij : & mut [i64] ) -> i64

``idxij``
    Position of each nonzero element in the vector representation of barA.
*Returns:* ``numnz``
    ``numnz : i64``
        Number of nonzero elements in barA.

Obtains the sparsity pattern of the barA matrix.

.. index:: get_barc_block_triplet

.. _optimizer_task_getbarcblocktriplet:

``get_barc_block_triplet()``
----------------------------

.. code-block:: rust

    pub fn get_barc_block_triplet ( &self,
                                    subj    : & mut [i32],
                                    subk    : & mut [i32],
                                    subl    : & mut [i32],
                                    valijkl : & mut [f64] )

``subj``
    Symmetric matrix variable index.
``subk``
    Block row index.
``subl``
    Block column index.
``valijkl``
    A list indexes of the elements from symmetric matrix storage that appears in the weighted sum.
*Returns:* ``num``
    ``num : i64``
        Number of elements in the block triplet form.

Obtains barc in block triplet form.

.. index:: get_barc_idx

.. _optimizer_task_getbarcidx:

``get_barc_idx()``
------------------

.. code-block:: rust

    pub fn get_barc_idx ( &self,
                          idx     : i64,
                          sub     : & mut [i64],
                          weights : & mut [f64] )

``idx``
    Index of the element that should be obtained information about.
``sub``
    Elements appearing the weighted sum.
``weights``
    Weights of terms in the weighted sum.
*Returns:* ``(j,num)``
    ``j : i32``
        Row index in barc.
    ``num : i64``
        Number of terms in the weighted sum.

Obtains information about an element in barc.

.. index:: get_barc_idx_info

.. _optimizer_task_getbarcidxinfo:

``get_barc_idx_info()``
-----------------------

.. code-block:: rust

    pub fn get_barc_idx_info ( &self,idx   : i64 ) -> i64

``idx``
    Index of element that should be obtained information about. The value is an index of a symmetric sparse variable.
*Returns:* ``num``
    ``num : i64``
        Number of terms that appears in weighted that forms the requested element.

Obtains information about an element in barc.

.. index:: get_barc_idx_j

.. _optimizer_task_getbarcidxj:

``get_barc_idx_j()``
--------------------

.. code-block:: rust

    pub fn get_barc_idx_j ( &self,idx   : i64 ) -> i32

``idx``
    Index of the element that should be obtained information about.
*Returns:* ``j``
    ``j : i32``
        Row index in barc.

Obtains the row index of an element in barc.

.. index:: get_barc_sparsity

.. _optimizer_task_getbarcsparsity:

``get_barc_sparsity()``
-----------------------

.. code-block:: rust

    pub fn get_barc_sparsity ( &self,idxj  : & mut [i64] ) -> i64

``idxj``
    Internal positions of the nonzeros elements in barc.
*Returns:* ``numnz``
    ``numnz : i64``
        Number of nonzero elements in barc.

Get the positions of the nonzero elements in barc.

.. index:: get_bars_j

.. _optimizer_task_getbarsj:

``get_bars_j()``
----------------

.. code-block:: rust

    pub fn get_bars_j ( &self,
                        whichsol : i32,
                        j        : i32,
                        barsj    : & mut [f64] )

``whichsol``
    
``j``
    Index of the semidefinite variable.
``barsj``
    Value of the j'th variable of barx.

Obtains the dual solution for a semidefinite variable.

.. index:: get_barvar_name

.. _optimizer_task_getbarvarname:

``get_barvar_name()``
---------------------

.. code-block:: rust

    pub fn get_barvar_name ( &self,i     : i32 ) -> String

``i``
    Index.
*Returns:* ``name``
    ``name : String``
        The requested name is copied to this buffer.

Obtains a name of a semidefinite variable.

.. index:: get_barvar_name_index

.. _optimizer_task_getbarvarnameindex:

``get_barvar_name_index()``
---------------------------

.. code-block:: rust

    pub fn get_barvar_name_index ( &self,somename : &str ) -> (i32,i32)

``somename``
    The requested name is copied to this buffer.
*Returns:* ``(asgn,index)``
    ``asgn : i32``
        Is non-zero if name somename is assigned to a semidefinite variable.
    ``index : i32``
        If the name somename is assigned to a semidefinite variable, then index is the name of the constraint.

Obtains the index of name of semidefinite variable.

.. index:: get_barvar_name_len

.. _optimizer_task_getbarvarnamelen:

``get_barvar_name_len()``
-------------------------

.. code-block:: rust

    pub fn get_barvar_name_len ( &self,i     : i32 ) -> i32

``i``
    Index.
*Returns:* ``len``
    ``len : i32``
        Returns the length of the indicated name.

Obtains the length of a name of a semidefinite variable.

.. index:: get_barx_j

.. _optimizer_task_getbarxj:

``get_barx_j()``
----------------

.. code-block:: rust

    pub fn get_barx_j ( &self,
                        whichsol : i32,
                        j        : i32,
                        barxj    : & mut [f64] )

``whichsol``
    
``j``
    Index of the semidefinite variable.
``barxj``
    Value of the j'th variable of barx.

Obtains the primal solution for a semidefinite variable.

.. index:: get_bound

.. _optimizer_task_getbound:

``get_bound()``
---------------

.. code-block:: rust

    pub fn get_bound ( &self,
                       accmode : i32,
                       i       : i32 )

``accmode``
    
``i``
    Index of the constraint or variable for which the bound information should be obtained.
*Returns:* ``(bk,bl,bu)``
    ``bk : i32``
        
    ``bl : f64``
        
    ``bu : f64``
        

Obtains bound information for one constraint or variable.

.. index:: get_bound_slice

.. _optimizer_task_getboundslice:

``get_bound_slice()``
---------------------

.. code-block:: rust

    pub fn get_bound_slice ( &self,
                             accmode : i32,
                             first   : i32,
                             last    : i32,
                             bk      : & mut [i32],
                             bl      : & mut [f64],
                             bu      : & mut [f64] )

``accmode``
    
``first``
    
``last``
    
``bk``
    
``bl``
    
``bu``
    

Obtains bounds information for a sequence of variables or constraints.

.. index:: get_c

.. _optimizer_task_getc:

``get_c()``
-----------

.. code-block:: rust

    pub fn get_c ( &self,c     : & mut [f64] )

``c``
    

Obtains all objective coefficients.

.. index:: get_c_j

.. _optimizer_task_getcj:

``get_c_j()``
-------------

.. code-block:: rust

    pub fn get_c_j ( &self,j     : i32 ) -> f64

``j``
    Index of the variable for which c coefficient should be obtained.
*Returns:* ``cj``
    ``cj : f64``
        The c coefficient value.

Obtains one coefficient of c.

.. index:: get_c_slice

.. _optimizer_task_getcslice:

``get_c_slice()``
-----------------

.. code-block:: rust

    pub fn get_c_slice ( &self,
                         first : i32,
                         last  : i32,
                         c     : & mut [f64] )

``first``
    
``last``
    
``c``
    

Obtains a sequence of coefficients from the objective.

.. index:: get_cfix

.. _optimizer_task_getcfix:

``get_cfix()``
--------------

.. code-block:: rust

    pub fn get_cfix ( &self ) -> f64

*Returns:* ``cfix``
    ``cfix : f64``
        

Obtains the fixed term in the objective.

.. index:: get_con_bound

.. _optimizer_task_getconbound:

``get_con_bound()``
-------------------

.. code-block:: rust

    pub fn get_con_bound ( &self,i     : i32 ) -> (i32,f64,f64)

``i``
    Index of the constraint for which the bound information should be obtained.
*Returns:* ``(bk,bl,bu)``
    ``bk : i32``
        
    ``bl : f64``
        
    ``bu : f64``
        

Obtains bound information for one constraint.

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
                                 bu    : & mut [f64] )

``first``
    
``last``
    
``bk``
    
``bl``
    
``bu``
    

Obtains bounds information for a slice of the constraints.

.. index:: get_con_name

.. _optimizer_task_getconname:

``get_con_name()``
------------------

.. code-block:: rust

    pub fn get_con_name ( &self,i     : i32 ) -> String

``i``
    Index.
*Returns:* ``name``
    ``name : String``
        Is assigned the required name.

Obtains a name of a constraint.

.. index:: get_con_name_index

.. _optimizer_task_getconnameindex:

``get_con_name_index()``
------------------------

.. code-block:: rust

    pub fn get_con_name_index ( &self,somename : &str ) -> (i32,i32)

``somename``
    The name which should be checked.
*Returns:* ``(asgn,index)``
    ``asgn : i32``
        Is non-zero if name somename is assigned to a constraint.
    ``index : i32``
        If the name somename is assigned to a constraint, then index is the name of the constraint.

Checks whether the name somename has been assigned  to any constraint.

.. index:: get_con_name_len

.. _optimizer_task_getconnamelen:

``get_con_name_len()``
----------------------

.. code-block:: rust

    pub fn get_con_name_len ( &self,i     : i32 ) -> i32

``i``
    Index.
*Returns:* ``len``
    ``len : i32``
        Returns the length of the indicated name.

Obtains the length of a name of a constraint variable.

.. index:: get_cone

.. _optimizer_task_getcone:

``get_cone()``
--------------

.. code-block:: rust

    pub fn get_cone ( &self,
                      k      : i32,
                      submem : & mut [i32] )

``k``
    Index of the cone constraint.
``submem``
    
*Returns:* ``(ct,conepar,nummem)``
    ``ct : i32``
        
    ``conepar : f64``
        
    ``nummem : i32``
        

Obtains a conic constraint.

.. index:: get_cone_info

.. _optimizer_task_getconeinfo:

``get_cone_info()``
-------------------

.. code-block:: rust

    pub fn get_cone_info ( &self,k     : i32 ) -> (i32,f64,i32)

``k``
    Index of the conic constraint.
*Returns:* ``(ct,conepar,nummem)``
    ``ct : i32``
        
    ``conepar : f64``
        
    ``nummem : i32``
        

Obtains information about a conic constraint.

.. index:: get_cone_name

.. _optimizer_task_getconename:

``get_cone_name()``
-------------------

.. code-block:: rust

    pub fn get_cone_name ( &self,i     : i32 ) -> String

``i``
    Index.
*Returns:* ``name``
    ``name : String``
        Is assigned the required name.

Obtains a name of a cone.

.. index:: get_cone_name_index

.. _optimizer_task_getconenameindex:

``get_cone_name_index()``
-------------------------

.. code-block:: rust

    pub fn get_cone_name_index ( &self,somename : &str ) -> (i32,i32)

``somename``
    The name which should be checked.
*Returns:* ``(asgn,index)``
    ``asgn : i32``
        Is non-zero if name somename is assigned to a cone.
    ``index : i32``
        If the name somename is assigned to a cone, then index is the name of the cone.

Checks whether the name somename has been assigned  to any cone.

.. index:: get_cone_name_len

.. _optimizer_task_getconenamelen:

``get_cone_name_len()``
-----------------------

.. code-block:: rust

    pub fn get_cone_name_len ( &self,i     : i32 ) -> i32

``i``
    Index.
*Returns:* ``len``
    ``len : i32``
        Returns the length of the indicated name.

Obtains the length of a name of a cone.

.. index:: get_dim_barvar_j

.. _optimizer_task_getdimbarvarj:

``get_dim_barvar_j()``
----------------------

.. code-block:: rust

    pub fn get_dim_barvar_j ( &self,j     : i32 ) -> i32

``j``
    Index of the semidefinite variable whose dimension is requested.
*Returns:* ``dimbarvarj``
    ``dimbarvarj : i32``
        The dimension of the j'th semidefinite variable.

Obtains the dimension of a symmetric matrix variable.

.. index:: get_dou_inf

.. _optimizer_task_getdouinf:

``get_dou_inf()``
-----------------

.. code-block:: rust

    pub fn get_dou_inf ( &self,whichdinf : i32 ) -> f64

``whichdinf``
    
*Returns:* ``dvalue``
    ``dvalue : f64``
        The value of the required double information item.

Obtains a double information item.

.. index:: get_dou_param

.. _optimizer_task_getdouparam:

``get_dou_param()``
-------------------

.. code-block:: rust

    pub fn get_dou_param ( &self,param : i32 ) -> f64

``param``
    
*Returns:* ``parvalue``
    ``parvalue : f64``
        

Obtains a double parameter.

.. index:: get_dual_obj

.. _optimizer_task_getdualobj:

``get_dual_obj()``
------------------

.. code-block:: rust

    pub fn get_dual_obj ( &self,whichsol : i32 ) -> f64

``whichsol``
    
*Returns:* ``dualobj``
    ``dualobj : f64``
        

Computes the dual objective value associated with the solution.

.. index:: get_dual_solution_norms

.. _optimizer_task_getdualsolutionnorms:

``get_dual_solution_norms()``
-----------------------------

.. code-block:: rust

    pub fn get_dual_solution_norms ( &self,whichsol : i32 ) -> (f64,f64,f64,f64,f64,f64,f64)

``whichsol``
    
*Returns:* ``(nrmy,nrmslc,nrmsuc,nrmslx,nrmsux,nrmsnx,nrmbars)``
    ``nrmy : f64``
        The norm of the y vector.
    ``nrmslc : f64``
        The norm of the slc vector.
    ``nrmsuc : f64``
        The norm of the suc vector.
    ``nrmslx : f64``
        The norm of the slx vector.
    ``nrmsux : f64``
        The norm of the sux vector.
    ``nrmsnx : f64``
        The norm of the snx vector.
    ``nrmbars : f64``
        The norm of the bars vector.

Compute norms of the primal solution.

.. index:: get_dviol_barvar

.. _optimizer_task_getdviolbarvar:

``get_dviol_barvar()``
----------------------

.. code-block:: rust

    pub fn get_dviol_barvar ( &self,
                              whichsol : i32,
                              sub_     : & [i32],
                              viol     : & mut [f64] )

``whichsol``
    
``sub``
    An array of indexes of barx variables.
``viol``
    List of violations corresponding to sub.

Computes the violation of dual solution for a set of barx variables.

.. index:: get_dviol_con

.. _optimizer_task_getdviolcon:

``get_dviol_con()``
-------------------

.. code-block:: rust

    pub fn get_dviol_con ( &self,
                           whichsol : i32,
                           sub_     : & [i32],
                           viol     : & mut [f64] )

``whichsol``
    
``sub``
    An array of indexes of constraints.
``viol``
    List of violations corresponding to sub.

Computes the violation of a dual solution associated with a set of constraints.

.. index:: get_dviol_cones

.. _optimizer_task_getdviolcones:

``get_dviol_cones()``
---------------------

.. code-block:: rust

    pub fn get_dviol_cones ( &self,
                             whichsol : i32,
                             sub_     : & [i32],
                             viol     : & mut [f64] )

``whichsol``
    
``sub``
    An array of indexes of barx variables.
``viol``
    List of violations corresponding to sub.

Computes the violation of a solution for set of dual conic constraints.

.. index:: get_dviol_var

.. _optimizer_task_getdviolvar:

``get_dviol_var()``
-------------------

.. code-block:: rust

    pub fn get_dviol_var ( &self,
                           whichsol : i32,
                           sub_     : & [i32],
                           viol     : & mut [f64] )

``whichsol``
    
``sub``
    An array of indexes of x variables.
``viol``
    List of violations corresponding to sub.

Computes the violation of a dual solution associated with a set of x variables.

.. index:: get_inf_index

.. _optimizer_task_getinfindex:

``get_inf_index()``
-------------------

.. code-block:: rust

    pub fn get_inf_index ( &self,
                           inftype : i32,
                           infname : &str )

``inftype``
    
``infname``
    
*Returns:* ``infindex``
    ``infindex : i32``
        The item index.

Obtains the index of a named information item.

.. index:: get_inf_max

.. _optimizer_task_getinfmax:

``get_inf_max()``
-----------------

.. code-block:: rust

    pub fn get_inf_max ( &self,
                         inftype : i32,
                         infmax  : & mut [i32] )

``inftype``
    
``infmax``
    

Obtains the maximum index of an information of a given type inftype plus 1.

.. index:: get_inf_name

.. _optimizer_task_getinfname:

``get_inf_name()``
------------------

.. code-block:: rust

    pub fn get_inf_name ( &self,
                          inftype  : i32,
                          whichinf : i32 )

``inftype``
    
``whichinf``
    
*Returns:* ``infname``
    ``infname : String``
        

Obtains the name of an information item.

.. index:: get_int_inf

.. _optimizer_task_getintinf:

``get_int_inf()``
-----------------

.. code-block:: rust

    pub fn get_int_inf ( &self,whichiinf : i32 ) -> i32

``whichiinf``
    
*Returns:* ``ivalue``
    ``ivalue : i32``
        The value of the required integer information item.

Obtains an integer information item.

.. index:: get_int_param

.. _optimizer_task_getintparam:

``get_int_param()``
-------------------

.. code-block:: rust

    pub fn get_int_param ( &self,param : i32 ) -> i32

``param``
    
*Returns:* ``parvalue``
    ``parvalue : i32``
        

Obtains an integer parameter.

.. index:: get_len_barvar_j

.. _optimizer_task_getlenbarvarj:

``get_len_barvar_j()``
----------------------

.. code-block:: rust

    pub fn get_len_barvar_j ( &self,j     : i32 ) -> i64

``j``
    Index of the semidefinite variable whose length if requested.
*Returns:* ``lenbarvarj``
    ``lenbarvarj : i64``
        Number of scalar elements in the lower triangular part of the semidefinite variable.

Obtains the length if the j'th semidefinite variables.

.. index:: get_lint_inf

.. _optimizer_task_getlintinf:

``get_lint_inf()``
------------------

.. code-block:: rust

    pub fn get_lint_inf ( &self,whichliinf : i32 ) -> i64

``whichliinf``
    
*Returns:* ``ivalue``
    ``ivalue : i64``
        The value of the required integer information item.

Obtains an integer information item.

.. index:: get_max_num_a_nz

.. _optimizer_task_getmaxnumanz64:

``get_max_num_a_nz()``
----------------------

.. code-block:: rust

    pub fn get_max_num_a_nz ( &self ) -> i64

*Returns:* ``maxnumanz``
    ``maxnumanz : i64``
        

Obtains number of preallocated non-zeros in the linear constraint matrix.

.. index:: get_max_num_barvar

.. _optimizer_task_getmaxnumbarvar:

``get_max_num_barvar()``
------------------------

.. code-block:: rust

    pub fn get_max_num_barvar ( &self ) -> i32

*Returns:* ``maxnumbarvar``
    ``maxnumbarvar : i32``
        Obtains maximum number of semidefinite variable currently allowed.

Obtains the number of semidefinite variables.

.. index:: get_max_num_con

.. _optimizer_task_getmaxnumcon:

``get_max_num_con()``
---------------------

.. code-block:: rust

    pub fn get_max_num_con ( &self ) -> i32

*Returns:* ``maxnumcon``
    ``maxnumcon : i32``
        

Obtains the number of preallocated constraints in the optimization task.

.. index:: get_max_num_cone

.. _optimizer_task_getmaxnumcone:

``get_max_num_cone()``
----------------------

.. code-block:: rust

    pub fn get_max_num_cone ( &self ) -> i32

*Returns:* ``maxnumcone``
    ``maxnumcone : i32``
        

Obtains the number of preallocated cones in the optimization task.

.. index:: get_max_num_q_nz

.. _optimizer_task_getmaxnumqnz64:

``get_max_num_q_nz()``
----------------------

.. code-block:: rust

    pub fn get_max_num_q_nz ( &self ) -> i64

*Returns:* ``maxnumqnz``
    ``maxnumqnz : i64``
        

Obtains the number of preallocated non-zeros for all quadratic terms in objective and constraints.

.. index:: get_max_num_var

.. _optimizer_task_getmaxnumvar:

``get_max_num_var()``
---------------------

.. code-block:: rust

    pub fn get_max_num_var ( &self ) -> i32

*Returns:* ``maxnumvar``
    ``maxnumvar : i32``
        

Obtains the maximum number variables allowed.

.. index:: get_mem_usage

.. _optimizer_task_getmemusagetask:

``get_mem_usage()``
-------------------

.. code-block:: rust

    pub fn get_mem_usage ( &self ) -> (i64,i64)

*Returns:* ``(meminuse,maxmemuse)``
    ``meminuse : i64``
        Amount of memory currently used by the task.
    ``maxmemuse : i64``
        Maximum amount of memory used by the task until now.

Obtains information about the amount of memory used by a task.

.. index:: get_num_a_nz

.. _optimizer_task_getnumanz:

``get_num_a_nz()``
------------------

.. code-block:: rust

    pub fn get_num_a_nz ( &self ) -> i32

*Returns:* ``numanz``
    ``numanz : i32``
        

Obtains the number of non-zeros in the coefficient matrix.

.. index:: get_num_a_nz_64

.. _optimizer_task_getnumanz64:

``get_num_a_nz_64()``
---------------------

.. code-block:: rust

    pub fn get_num_a_nz_64 ( &self ) -> i64

*Returns:* ``numanz``
    ``numanz : i64``
        

Obtains the number of non-zeros in the coefficient matrix.

.. index:: get_num_bara_block_triplets

.. _optimizer_task_getnumbarablocktriplets:

``get_num_bara_block_triplets()``
---------------------------------

.. code-block:: rust

    pub fn get_num_bara_block_triplets ( &self ) -> i64

*Returns:* ``num``
    ``num : i64``
        Number elements in the block triplet form of bara.

Obtains an upper bound on the number of scalar elements in the block triplet form of bara.

.. index:: get_num_bara_nz

.. _optimizer_task_getnumbaranz:

``get_num_bara_nz()``
---------------------

.. code-block:: rust

    pub fn get_num_bara_nz ( &self ) -> i64

*Returns:* ``nz``
    ``nz : i64``
        The number of nonzero block elements in barA.

Get the number of nonzero elements in barA.

.. index:: get_num_barc_block_triplets

.. _optimizer_task_getnumbarcblocktriplets:

``get_num_barc_block_triplets()``
---------------------------------

.. code-block:: rust

    pub fn get_num_barc_block_triplets ( &self ) -> i64

*Returns:* ``num``
    ``num : i64``
        An upper bound on the number elements in the block trip let form of barc.

Obtains an upper bound on the number of elements in the block triplet form of barc.

.. index:: get_num_barc_nz

.. _optimizer_task_getnumbarcnz:

``get_num_barc_nz()``
---------------------

.. code-block:: rust

    pub fn get_num_barc_nz ( &self ) -> i64

*Returns:* ``nz``
    ``nz : i64``
        The number of nonzero elements in barc.

Obtains the number of nonzero elements in barc.

.. index:: get_num_barvar

.. _optimizer_task_getnumbarvar:

``get_num_barvar()``
--------------------

.. code-block:: rust

    pub fn get_num_barvar ( &self ) -> i32

*Returns:* ``numbarvar``
    ``numbarvar : i32``
        Number of semidefinite variable in the problem.

Obtains the number of semidefinite variables.

.. index:: get_num_con

.. _optimizer_task_getnumcon:

``get_num_con()``
-----------------

.. code-block:: rust

    pub fn get_num_con ( &self ) -> i32

*Returns:* ``numcon``
    ``numcon : i32``
        

Obtains the number of constraints.

.. index:: get_num_cone

.. _optimizer_task_getnumcone:

``get_num_cone()``
------------------

.. code-block:: rust

    pub fn get_num_cone ( &self ) -> i32

*Returns:* ``numcone``
    ``numcone : i32``
        Number conic constraints.

Obtains the number of cones.

.. index:: get_num_cone_mem

.. _optimizer_task_getnumconemem:

``get_num_cone_mem()``
----------------------

.. code-block:: rust

    pub fn get_num_cone_mem ( &self,k     : i32 ) -> i32

``k``
    Index of the cone.
*Returns:* ``nummem``
    ``nummem : i32``
        

Obtains the number of members in a cone.

.. index:: get_num_int_var

.. _optimizer_task_getnumintvar:

``get_num_int_var()``
---------------------

.. code-block:: rust

    pub fn get_num_int_var ( &self ) -> i32

*Returns:* ``numintvar``
    ``numintvar : i32``
        Number of integer variables.

Obtains the number of integer-constrained variables.

.. index:: get_num_param

.. _optimizer_task_getnumparam:

``get_num_param()``
-------------------

.. code-block:: rust

    pub fn get_num_param ( &self,partype : i32 ) -> i32

``partype``
    
*Returns:* ``numparam``
    ``numparam : i32``
        Returns the number of parameters of the requested type.

Obtains the number of parameters of a given type.

.. index:: get_num_q_con_k_nz

.. _optimizer_task_getnumqconknz64:

``get_num_q_con_k_nz()``
------------------------

.. code-block:: rust

    pub fn get_num_q_con_k_nz ( &self,k     : i32 ) -> i64

``k``
    Index of the constraint for which the number quadratic terms should be obtained.
*Returns:* ``numqcnz``
    ``numqcnz : i64``
        

Obtains the number of non-zero quadratic terms in a constraint.

.. index:: get_num_q_obj_nz

.. _optimizer_task_getnumqobjnz64:

``get_num_q_obj_nz()``
----------------------

.. code-block:: rust

    pub fn get_num_q_obj_nz ( &self ) -> i64

*Returns:* ``numqonz``
    ``numqonz : i64``
        

Obtains the number of non-zero quadratic terms in the objective.

.. index:: get_num_sym_mat

.. _optimizer_task_getnumsymmat:

``get_num_sym_mat()``
---------------------

.. code-block:: rust

    pub fn get_num_sym_mat ( &self ) -> i64

*Returns:* ``num``
    ``num : i64``
        Returns the number of symmetric sparse matrixes.

Get the number of symmetric matrixes stored.

.. index:: get_num_var

.. _optimizer_task_getnumvar:

``get_num_var()``
-----------------

.. code-block:: rust

    pub fn get_num_var ( &self ) -> i32

*Returns:* ``numvar``
    ``numvar : i32``
        

Obtains the number of variables.

.. index:: get_obj_name

.. _optimizer_task_getobjname:

``get_obj_name()``
------------------

.. code-block:: rust

    pub fn get_obj_name ( &self ) -> String

*Returns:* ``objname``
    ``objname : String``
        Assigned the objective name.

Obtains the name assigned to the objective function.

.. index:: get_obj_name_len

.. _optimizer_task_getobjnamelen:

``get_obj_name_len()``
----------------------

.. code-block:: rust

    pub fn get_obj_name_len ( &self ) -> i32

*Returns:* ``len``
    ``len : i32``
        Assigned the length of the objective name.

Obtains the length of the name assigned to the objective function.

.. index:: get_obj_sense

.. _optimizer_task_getobjsense:

``get_obj_sense()``
-------------------

.. code-block:: rust

    pub fn get_obj_sense ( &self ) -> i32

*Returns:* ``sense``
    ``sense : i32``
        The returned objective sense.

Gets the objective sense.

.. index:: get_param_max

.. _optimizer_task_getparammax:

``get_param_max()``
-------------------

.. code-block:: rust

    pub fn get_param_max ( &self,partype : i32 ) -> i32

``partype``
    
*Returns:* ``parammax``
    ``parammax : i32``
        

Obtains the maximum index of a parameter of a given type plus 1.

.. index:: get_param_name

.. _optimizer_task_getparamname:

``get_param_name()``
--------------------

.. code-block:: rust

    pub fn get_param_name ( &self,
                            partype : i32,
                            param   : i32 )

``partype``
    
``param``
    
*Returns:* ``parname``
    ``parname : String``
        

Obtains the name of a parameter.

.. index:: get_primal_obj

.. _optimizer_task_getprimalobj:

``get_primal_obj()``
--------------------

.. code-block:: rust

    pub fn get_primal_obj ( &self,whichsol : i32 ) -> f64

``whichsol``
    
*Returns:* ``primalobj``
    ``primalobj : f64``
        

Computes the primal objective value for the desired solution.

.. index:: get_primal_solution_norms

.. _optimizer_task_getprimalsolutionnorms:

``get_primal_solution_norms()``
-------------------------------

.. code-block:: rust

    pub fn get_primal_solution_norms ( &self,whichsol : i32 ) -> (f64,f64,f64)

``whichsol``
    
*Returns:* ``(nrmxc,nrmxx,nrmbarx)``
    ``nrmxc : f64``
        The norm of xc vector.
    ``nrmxx : f64``
        The norm of xx vector.
    ``nrmbarx : f64``
        The norm of barx vector.

Compute norms of the primal solution.

.. index:: get_pro_sta

.. _optimizer_task_getprosta:

``get_pro_sta()``
-----------------

.. code-block:: rust

    pub fn get_pro_sta ( &self,whichsol : i32 ) -> i32

``whichsol``
    
*Returns:* ``prosta``
    ``prosta : i32``
        

Obtains the problem status.

.. index:: get_prob_type

.. _optimizer_task_getprobtype:

``get_prob_type()``
-------------------

.. code-block:: rust

    pub fn get_prob_type ( &self ) -> i32

*Returns:* ``probtype``
    ``probtype : i32``
        The problem type.

Obtains the problem type.

.. index:: get_pviol_barvar

.. _optimizer_task_getpviolbarvar:

``get_pviol_barvar()``
----------------------

.. code-block:: rust

    pub fn get_pviol_barvar ( &self,
                              whichsol : i32,
                              sub_     : & [i32],
                              viol     : & mut [f64] )

``whichsol``
    
``sub``
    An array of indexes of barx variables.
``viol``
    List of violations corresponding to sub.

Computes the violation of a primal solution for a list of barx variables.

.. index:: get_pviol_con

.. _optimizer_task_getpviolcon:

``get_pviol_con()``
-------------------

.. code-block:: rust

    pub fn get_pviol_con ( &self,
                           whichsol : i32,
                           sub_     : & [i32],
                           viol     : & mut [f64] )

``whichsol``
    
``sub``
    An array of indexes of constraints.
``viol``
    List of violations corresponding to sub.

Computes the violation of a primal solution for a list of xc variables.

.. index:: get_pviol_cones

.. _optimizer_task_getpviolcones:

``get_pviol_cones()``
---------------------

.. code-block:: rust

    pub fn get_pviol_cones ( &self,
                             whichsol : i32,
                             sub_     : & [i32],
                             viol     : & mut [f64] )

``whichsol``
    
``sub``
    An array of indexes of barx variables.
``viol``
    List of violations corresponding to sub.

Computes the violation of a solution for set of conic constraints.

.. index:: get_pviol_var

.. _optimizer_task_getpviolvar:

``get_pviol_var()``
-------------------

.. code-block:: rust

    pub fn get_pviol_var ( &self,
                           whichsol : i32,
                           sub_     : & [i32],
                           viol     : & mut [f64] )

``whichsol``
    
``sub``
    An array of indexes of x variables.
``viol``
    List of violations corresponding to sub.

Computes the violation of a primal solution for a list of x variables.

.. index:: get_q_obj_i_j

.. _optimizer_task_getqobjij:

``get_q_obj_i_j()``
-------------------

.. code-block:: rust

    pub fn get_q_obj_i_j ( &self,
                           i     : i32,
                           j     : i32 )

``i``
    Row index of the coefficient.
``j``
    Column index of coefficient.
*Returns:* ``qoij``
    ``qoij : f64``
        The required coefficient.

Obtains one coefficient from the quadratic term of the objective

.. index:: get_reduced_costs

.. _optimizer_task_getreducedcosts:

``get_reduced_costs()``
-----------------------

.. code-block:: rust

    pub fn get_reduced_costs ( &self,
                               whichsol : i32,
                               first    : i32,
                               last     : i32,
                               redcosts : & mut [f64] )

``whichsol``
    
``first``
    See the documentation for a full description.
``last``
    See the documentation for a full description.
``redcosts``
    Returns the requested reduced costs. See documentation for a full description.

Obtains the difference of (slx-sux) for a sequence of variables.

.. index:: get_skc

.. _optimizer_task_getskc:

``get_skc()``
-------------

.. code-block:: rust

    pub fn get_skc ( &self,
                     whichsol : i32,
                     skc      : & mut [i32] )

``whichsol``
    
``skc``
    

Obtains the status keys for the constraints.

.. index:: get_skc_slice

.. _optimizer_task_getskcslice:

``get_skc_slice()``
-------------------

.. code-block:: rust

    pub fn get_skc_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           skc      : & mut [i32] )

``whichsol``
    
``first``
    
``last``
    
``skc``
    

Obtains the status keys for the constraints.

.. index:: get_skx

.. _optimizer_task_getskx:

``get_skx()``
-------------

.. code-block:: rust

    pub fn get_skx ( &self,
                     whichsol : i32,
                     skx      : & mut [i32] )

``whichsol``
    
``skx``
    

Obtains the status keys for the scalar variables.

.. index:: get_skx_slice

.. _optimizer_task_getskxslice:

``get_skx_slice()``
-------------------

.. code-block:: rust

    pub fn get_skx_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           skx      : & mut [i32] )

``whichsol``
    
``first``
    
``last``
    
``skx``
    

Obtains the status keys for the variables.

.. index:: get_slc

.. _optimizer_task_getslc:

``get_slc()``
-------------

.. code-block:: rust

    pub fn get_slc ( &self,
                     whichsol : i32,
                     slc      : & mut [f64] )

``whichsol``
    
``slc``
    The slc vector.

Obtains the slc vector for a solution.

.. index:: get_slc_slice

.. _optimizer_task_getslcslice:

``get_slc_slice()``
-------------------

.. code-block:: rust

    pub fn get_slc_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           slc      : & mut [f64] )

``whichsol``
    
``first``
    
``last``
    
``slc``
    

Obtains a slice of the slc vector for a solution.

.. index:: get_slx

.. _optimizer_task_getslx:

``get_slx()``
-------------

.. code-block:: rust

    pub fn get_slx ( &self,
                     whichsol : i32,
                     slx      : & mut [f64] )

``whichsol``
    
``slx``
    The slx vector.

Obtains the slx vector for a solution.

.. index:: get_slx_slice

.. _optimizer_task_getslxslice:

``get_slx_slice()``
-------------------

.. code-block:: rust

    pub fn get_slx_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           slx      : & mut [f64] )

``whichsol``
    
``first``
    
``last``
    
``slx``
    

Obtains a slice of the slx vector for a solution.

.. index:: get_snx

.. _optimizer_task_getsnx:

``get_snx()``
-------------

.. code-block:: rust

    pub fn get_snx ( &self,
                     whichsol : i32,
                     snx      : & mut [f64] )

``whichsol``
    
``snx``
    The snx vector.

Obtains the snx vector for a solution.

.. index:: get_snx_slice

.. _optimizer_task_getsnxslice:

``get_snx_slice()``
-------------------

.. code-block:: rust

    pub fn get_snx_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           snx      : & mut [f64] )

``whichsol``
    
``first``
    
``last``
    
``snx``
    

Obtains a slice of the snx vector for a solution.

.. index:: get_sol_sta

.. _optimizer_task_getsolsta:

``get_sol_sta()``
-----------------

.. code-block:: rust

    pub fn get_sol_sta ( &self,whichsol : i32 ) -> i32

``whichsol``
    
*Returns:* ``solsta``
    ``solsta : i32``
        

Obtains the solution status.

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
                          snx      : & mut [f64] )

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
        

Obtains the complete solution.

.. index:: get_solution_i

.. _optimizer_task_getsolutioni:

``get_solution_i()``
--------------------

.. code-block:: rust

    pub fn get_solution_i ( &self,
                            accmode  : i32,
                            i        : i32,
                            whichsol : i32 )

``accmode``
    Defines whether solution information for a constraint or for a variable is retrieved.
``i``
    Index of the constraint or variable.
``whichsol``
    
*Returns:* ``(sk,x,sl,su,sn)``
    ``sk : i32``
        Status key of the constraint of variable.
    ``x : f64``
        Solution value of the primal variable.
    ``sl : f64``
        Solution value of the dual variable associated with the lower bound.
    ``su : f64``
        Solution value of the dual variable associated with the upper bound.
    ``sn : f64``
        Solution value of the dual variable associated with the cone constraint.

Obtains the solution for a single constraint or variable.

.. index:: get_solution_info

.. _optimizer_task_getsolutioninfo:

``get_solution_info()``
-----------------------

.. code-block:: rust

    pub fn get_solution_info ( &self,whichsol : i32 ) -> (f64,f64,f64,f64,f64,f64,f64,f64,f64,f64,f64)

``whichsol``
    
*Returns:* ``(pobj,pviolcon,pviolvar,pviolbarvar,pviolcone,pviolitg,dobj,dviolcon,dviolvar,dviolbarvar,dviolcone)``
    ``pobj : f64``
        The primal objective value.
    ``pviolcon : f64``
        Maximal primal bound violation for a xc variable.
    ``pviolvar : f64``
        Maximal primal bound violation for a xx variable.
    ``pviolbarvar : f64``
        Maximal primal bound violation for a barx variable.
    ``pviolcone : f64``
        Maximal primal violation of the solution with respect to the conic constraints.
    ``pviolitg : f64``
        Maximal violation in the integer constraints.
    ``dobj : f64``
        Dual objective value.
    ``dviolcon : f64``
        Maximal dual bound violation a xc variable.
    ``dviolvar : f64``
        Maximal dual bound violation xx variable.
    ``dviolbarvar : f64``
        Maximal dual bound violation for a bars variable.
    ``dviolcone : f64``
        Maximum violation of the dual solution in the dual conic constraints .

Obtains information about of a solution.

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
                                values   : & mut [f64] )

``whichsol``
    
``solitem``
    
``first``
    Index of the first value in the slice.
``last``
    Value of the last index+1 in the slice.
``values``
    The values of the requested solution elements.

Obtains a slice of the solution.

.. index:: get_sparse_sym_mat

.. _optimizer_task_getsparsesymmat:

``get_sparse_sym_mat()``
------------------------

.. code-block:: rust

    pub fn get_sparse_sym_mat ( &self,
                                idx   : i64,
                                subi  : & mut [i32],
                                subj  : & mut [i32],
                                valij : & mut [f64] )

``idx``
    Index of the matrix to get.
``subi``
    Row subscripts of the matrix non-zero elements.
``subj``
    Column subscripts of the matrix non-zero elements.
``valij``
    Coefficients of the matrix non-zero elements.

Gets a single symmetric matrix from the matrix store.

.. index:: get_str_param

.. _optimizer_task_getstrparam:

``get_str_param()``
-------------------

.. code-block:: rust

    pub fn get_str_param ( &self,param : i32 ) -> (i32,String)

``param``
    
*Returns:* ``(len,parvalue)``
    ``len : i32``
        The length of the parameter value.
    ``parvalue : String``
        If this is not |null|, the parameter value is stored here.

Obtains the value of a string parameter.

.. index:: get_str_param_len

.. _optimizer_task_getstrparamlen:

``get_str_param_len()``
-----------------------

.. code-block:: rust

    pub fn get_str_param_len ( &self,param : i32 ) -> i32

``param``
    
*Returns:* ``len``
    ``len : i32``
        The length of the parameter value.

Obtains the length of a string parameter.

.. index:: get_suc

.. _optimizer_task_getsuc:

``get_suc()``
-------------

.. code-block:: rust

    pub fn get_suc ( &self,
                     whichsol : i32,
                     suc      : & mut [f64] )

``whichsol``
    
``suc``
    The suc vector.

Obtains the suc vector for a solution.

.. index:: get_suc_slice

.. _optimizer_task_getsucslice:

``get_suc_slice()``
-------------------

.. code-block:: rust

    pub fn get_suc_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           suc      : & mut [f64] )

``whichsol``
    
``first``
    
``last``
    
``suc``
    

Obtains a slice of the suc vector for a solution.

.. index:: get_sux

.. _optimizer_task_getsux:

``get_sux()``
-------------

.. code-block:: rust

    pub fn get_sux ( &self,
                     whichsol : i32,
                     sux      : & mut [f64] )

``whichsol``
    
``sux``
    The sux vector.

Obtains the sux vector for a solution.

.. index:: get_sux_slice

.. _optimizer_task_getsuxslice:

``get_sux_slice()``
-------------------

.. code-block:: rust

    pub fn get_sux_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           sux      : & mut [f64] )

``whichsol``
    
``first``
    
``last``
    
``sux``
    

Obtains a slice of the sux vector for a solution.

.. index:: get_sym_mat_info

.. _optimizer_task_getsymmatinfo:

``get_sym_mat_info()``
----------------------

.. code-block:: rust

    pub fn get_sym_mat_info ( &self,idx   : i64 ) -> (i32,i64,i32)

``idx``
    Index of the matrix that is requested information about.
*Returns:* ``(dim,nz,type)``
    ``dim : i32``
        Returns the dimension of the requested matrix.
    ``nz : i64``
        Returns the number of non-zeros in the requested matrix.
    ``type : i32``
        Returns the type of the requested matrix.

Obtains information of  a matrix from the symmetric matrix storage E.

.. index:: get_task_name

.. _optimizer_task_gettaskname:

``get_task_name()``
-------------------

.. code-block:: rust

    pub fn get_task_name ( &self ) -> String

*Returns:* ``taskname``
    ``taskname : String``
        Is assigned the task name.

Obtains the task name.

.. index:: get_task_name_len

.. _optimizer_task_gettasknamelen:

``get_task_name_len()``
-----------------------

.. code-block:: rust

    pub fn get_task_name_len ( &self ) -> i32

*Returns:* ``len``
    ``len : i32``
        Returns the length of the task name.

Obtains the length the task name.

.. index:: get_var_bound

.. _optimizer_task_getvarbound:

``get_var_bound()``
-------------------

.. code-block:: rust

    pub fn get_var_bound ( &self,i     : i32 ) -> (i32,f64,f64)

``i``
    Index of the variable for which the bound information should be obtained.
*Returns:* ``(bk,bl,bu)``
    ``bk : i32``
        
    ``bl : f64``
        
    ``bu : f64``
        

Obtains bound information for one variable.

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
                                 bu    : & mut [f64] )

``first``
    
``last``
    
``bk``
    
``bl``
    
``bu``
    

Obtains bounds information for a slice of the variables.

.. index:: get_var_name

.. _optimizer_task_getvarname:

``get_var_name()``
------------------

.. code-block:: rust

    pub fn get_var_name ( &self,j     : i32 ) -> String

``j``
    Index.
*Returns:* ``name``
    ``name : String``
        Returns the required name.

Obtains a name of a variable.

.. index:: get_var_name_index

.. _optimizer_task_getvarnameindex:

``get_var_name_index()``
------------------------

.. code-block:: rust

    pub fn get_var_name_index ( &self,somename : &str ) -> (i32,i32)

``somename``
    The name which should be checked.
*Returns:* ``(asgn,index)``
    ``asgn : i32``
        Is non-zero if name somename is assigned to a variable.
    ``index : i32``
        If the name somename is assigned to a variable, then index is the name of the variable.

Checks whether the name somename has been assigned  to any variable.

.. index:: get_var_name_len

.. _optimizer_task_getvarnamelen:

``get_var_name_len()``
----------------------

.. code-block:: rust

    pub fn get_var_name_len ( &self,i     : i32 ) -> i32

``i``
    Index.
*Returns:* ``len``
    ``len : i32``
        Returns the length of the indicated name.

Obtains the length of a name of a variable variable.

.. index:: get_var_type

.. _optimizer_task_getvartype:

``get_var_type()``
------------------

.. code-block:: rust

    pub fn get_var_type ( &self,j     : i32 ) -> i32

``j``
    Index of the variable.
*Returns:* ``vartype``
    ``vartype : i32``
        Variable type of variable index j.

Gets the variable type of one variable.

.. index:: get_var_type_list

.. _optimizer_task_getvartypelist:

``get_var_type_list()``
-----------------------

.. code-block:: rust

    pub fn get_var_type_list ( &self,
                               subj_   : & [i32],
                               vartype : & mut [i32] )

``subj``
    A list of variable indexes.
``vartype``
    Returns the variables types corresponding the variable indexes requested.

Obtains the variable type for one or more variables.

.. index:: get_xc

.. _optimizer_task_getxc:

``get_xc()``
------------

.. code-block:: rust

    pub fn get_xc ( &self,
                    whichsol : i32,
                    xc       : & mut [f64] )

``whichsol``
    
``xc``
    The xc vector.

Obtains the xc vector for a solution.

.. index:: get_xc_slice

.. _optimizer_task_getxcslice:

``get_xc_slice()``
------------------

.. code-block:: rust

    pub fn get_xc_slice ( &self,
                          whichsol : i32,
                          first    : i32,
                          last     : i32,
                          xc       : & mut [f64] )

``whichsol``
    
``first``
    
``last``
    
``xc``
    

Obtains a slice of the xc vector for a solution.

.. index:: get_xx

.. _optimizer_task_getxx:

``get_xx()``
------------

.. code-block:: rust

    pub fn get_xx ( &self,
                    whichsol : i32,
                    xx       : & mut [f64] )

``whichsol``
    
``xx``
    The xx vector.

Obtains the xx vector for a solution.

.. index:: get_xx_slice

.. _optimizer_task_getxxslice:

``get_xx_slice()``
------------------

.. code-block:: rust

    pub fn get_xx_slice ( &self,
                          whichsol : i32,
                          first    : i32,
                          last     : i32,
                          xx       : & mut [f64] )

``whichsol``
    
``first``
    
``last``
    
``xx``
    

Obtains a slice of the xx vector for a solution.

.. index:: get_y

.. _optimizer_task_gety:

``get_y()``
-----------

.. code-block:: rust

    pub fn get_y ( &self,
                   whichsol : i32,
                   y        : & mut [f64] )

``whichsol``
    
``y``
    The y vector.

Obtains the y vector for a solution.

.. index:: get_y_slice

.. _optimizer_task_getyslice:

``get_y_slice()``
-----------------

.. code-block:: rust

    pub fn get_y_slice ( &self,
                         whichsol : i32,
                         first    : i32,
                         last     : i32,
                         y        : & mut [f64] )

``whichsol``
    
``first``
    
``last``
    
``y``
    

Obtains a slice of the y vector for a solution.

.. index:: init_basis_solve

.. _optimizer_task_initbasissolve:

``init_basis_solve()``
----------------------

.. code-block:: rust

    pub fn init_basis_solve ( &self,basis : & mut [i32] )

``basis``
    The array of basis indexes to use.

Prepare a task for basis solver.

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
                        bux_      : & [f64] )

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
    

Input the linear part of an optimization task in one function call.

.. index:: is_dou_par_name

.. _optimizer_task_isdouparname:

``is_dou_par_name()``
---------------------

.. code-block:: rust

    pub fn is_dou_par_name ( &self,parname : &str ) -> i32

``parname``
    
*Returns:* ``param``
    ``param : i32``
        

Checks a double parameter name.

.. index:: is_int_par_name

.. _optimizer_task_isintparname:

``is_int_par_name()``
---------------------

.. code-block:: rust

    pub fn is_int_par_name ( &self,parname : &str ) -> i32

``parname``
    
*Returns:* ``param``
    ``param : i32``
        

Checks an integer parameter name.

.. index:: is_str_par_name

.. _optimizer_task_isstrparname:

``is_str_par_name()``
---------------------

.. code-block:: rust

    pub fn is_str_par_name ( &self,parname : &str ) -> i32

``parname``
    
*Returns:* ``param``
    ``param : i32``
        

Checks a string parameter name.

.. index:: link_file_to_stream

.. _optimizer_task_linkfiletotaskstream:

``link_file_to_stream()``
-------------------------

.. code-block:: rust

    pub fn link_file_to_stream ( &self,
                                 whichstream : i32,
                                 filename    : &str,
                                 append      : i32 )

``whichstream``
    
``filename``
    The name of the file where the stream is written.
``append``
    If this argument is 0 the output file will be overwritten, otherwise text is append to the output file.

Directs all output from a task stream to a file.

.. index:: one_solution_summary

.. _optimizer_task_onesolutionsummary:

``one_solution_summary()``
--------------------------

.. code-block:: rust

    pub fn one_solution_summary ( &self,
                                  whichstream : i32,
                                  whichsol    : i32 )

``whichstream``
    
``whichsol``
    

Prints a short summary for the specified solution.

.. index:: optimize

.. _optimizer_task_optimizetrm:

``optimize()``
--------------

.. code-block:: rust

    pub fn optimize ( &self ) -> i32

*Returns:* ``trmcode``
    ``trmcode : i32``
        Is either OK or a termination response code.

Optimizes the problem.

.. index:: optimizer_summary

.. _optimizer_task_optimizersummary:

``optimizer_summary()``
-----------------------

.. code-block:: rust

    pub fn optimizer_summary ( &self,whichstream : i32 )

``whichstream``
    

Prints a short summary with optimizer statistics for last optimization.

.. index:: primal_repair

.. _optimizer_task_primalrepair:

``primal_repair()``
-------------------

.. code-block:: rust

    pub fn primal_repair ( &self,
                           wlc_  : & [f64],
                           wuc_  : & [f64],
                           wlx_  : & [f64],
                           wux_  : & [f64] )

``wlc``
    Weights associated with relaxing lower bounds on the constraints.
``wuc``
    Weights associated with relaxing the upper bound on the constraints.
``wlx``
    Weights associated with relaxing the lower bounds of the variables.
``wux``
    Weights associated with relaxing the upper bounds of variables.

The function repairs a primal infeasible optimization problem by adjusting the bounds on the constraints and variables.

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
                                rightrangej : & mut [f64] )

``subi``
    Indexes of bounds on constraints to analyze.
``marki``
    Mark which constraint bounds to analyze.
``subj``
    Indexes of bounds on variables to analyze.
``markj``
    Mark which variable bounds to analyze.
``leftpricei``
    Left shadow price for constraints.
``rightpricei``
    Right shadow price for constraints.
``leftrangei``
    Left range for constraints.
``rightrangei``
    Right range for constraints.
``leftpricej``
    Left price for variables.
``rightpricej``
    Right price for variables.
``leftrangej``
    Left range for variables.
``rightrangej``
    Right range for variables.

Perform sensitivity analysis on bounds.

.. index:: pro_sta_to_str

.. _optimizer_task_prostatostr:

``pro_sta_to_str()``
--------------------

.. code-block:: rust

    pub fn pro_sta_to_str ( &self,prosta : i32 ) -> String

``prosta``
    
*Returns:* ``str``
    ``str : String``
        String corresponding to the status key.

Obtains a string containing the name of a problem status given.

.. index:: prob_type_to_str

.. _optimizer_task_probtypetostr:

``prob_type_to_str()``
----------------------

.. code-block:: rust

    pub fn prob_type_to_str ( &self,probtype : i32 ) -> String

``probtype``
    
*Returns:* ``str``
    ``str : String``
        String corresponding to the problem type key.

Obtains a string containing the name of a problem type given.

.. index:: put_a_col

.. _optimizer_task_putacol:

``put_a_col()``
---------------

.. code-block:: rust

    pub fn put_a_col ( &self,
                       j     : i32,
                       subj_ : & [i32],
                       valj_ : & [f64] )

``j``
    Column index.
``subj``
    Row indexes of non-zero values in column.
``valj``
    New non-zero values of column.

Replaces all elements in one column of A.

.. index:: put_a_col_list

.. _optimizer_task_putacollist:

``put_a_col_list()``
--------------------

.. code-block:: rust

    pub fn put_a_col_list ( &self,
                            sub_  : & [i32],
                            ptrb_ : & [i32],
                            ptre_ : & [i32],
                            asub_ : & [i32],
                            aval_ : & [f64] )

``sub``
    Indexes of columns that should be replaced.
``ptrb``
    Array of pointers to the first element in the columns.
``ptre``
    Array of pointers to the last element plus one in the columns.
``asub``
    Variable indexes.
``aval``
    

Replaces all elements in several columns the linear constraint matrix by new values.

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
                             aval_ : & [f64] )

``first``
    First column in the slice.
``last``
    Last column plus one in the slice.
``ptrb``
    Array of pointers to the first element in the columns.
``ptre``
    Array of pointers to the last element plus one in the columns.
``asub``
    Variable indexes.
``aval``
    

Replaces all elements in several columns the linear constraint matrix by new values.

.. index:: put_a_row

.. _optimizer_task_putarow:

``put_a_row()``
---------------

.. code-block:: rust

    pub fn put_a_row ( &self,
                       i     : i32,
                       subi_ : & [i32],
                       vali_ : & [f64] )

``i``
    row index.
``subi``
    Row indexes of non-zero values in row.
``vali``
    New non-zero values of row.

Replaces all elements in one row of A.

.. index:: put_a_row_list

.. _optimizer_task_putarowlist:

``put_a_row_list()``
--------------------

.. code-block:: rust

    pub fn put_a_row_list ( &self,
                            sub_   : & [i32],
                            aptrb_ : & [i32],
                            aptre_ : & [i32],
                            asub_  : & [i32],
                            aval_  : & [f64] )

``sub``
    Indexes of rows or columns that should be replaced.
``aptrb``
    Array of pointers to the first element in the rows or columns.
``aptre``
    Array of pointers to the last element plus one in the rows or columns.
``asub``
    Variable indexes.
``aval``
    

Replaces all elements in several rows the linear constraint matrix by new values.

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
                             aval_ : & [f64] )

``first``
    First row in the slice.
``last``
    Last row plus one in the slice.
``ptrb``
    Array of pointers to the first element in the rows.
``ptre``
    Array of pointers to the last element plus one in the rows.
``asub``
    Variable indexes.
``aval``
    

Replaces all elements in several rows the linear constraint matrix by new values.

.. index:: put_aij

.. _optimizer_task_putaij:

``put_aij()``
-------------

.. code-block:: rust

    pub fn put_aij ( &self,
                     i     : i32,
                     j     : i32,
                     aij   : f64 )

``i``
    Index of the constraint in which the change should occur.
``j``
    Index of the variable in which the change should occur.
``aij``
    New coefficient.

Changes a single value in the linear coefficient matrix.

.. index:: put_aij_list

.. _optimizer_task_putaijlist64:

``put_aij_list()``
------------------

.. code-block:: rust

    pub fn put_aij_list ( &self,
                          subi_  : & [i32],
                          subj_  : & [i32],
                          valij_ : & [f64] )

``subi``
    Constraint indexes in which the change should occur.
``subj``
    Variable indexes in which the change should occur.
``valij``
    New coefficient values.

Changes one or more coefficients in the linear constraint matrix.

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
                                    valijkl_ : & [f64] )

``num``
    Number of elements in the block triplet form.
``subi``
    Constraint index.
``subj``
    Symmetric matrix variable index.
``subk``
    Block row index.
``subl``
    Block column index.
``valijkl``
    The numerical value associated with the block triplet.

Inputs barA in block triplet form.

.. index:: put_bara_ij

.. _optimizer_task_putbaraij:

``put_bara_ij()``
-----------------

.. code-block:: rust

    pub fn put_bara_ij ( &self,
                         i        : i32,
                         j        : i32,
                         sub_     : & [i64],
                         weights_ : & [f64] )

``i``
    Row index of barA.
``j``
    Column index of barA.
``sub``
    See argument weights for an explanation.
``weights``
    Weights in the weighted sum.

Inputs an element of barA.

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
                                    valjkl_ : & [f64] )

``num``
    Number of elements in the block triplet form.
``subj``
    Symmetric matrix variable index.
``subk``
    Block row index.
``subl``
    Block column index.
``valjkl``
    The numerical value associated with the block triplet.

Inputs barC in block triplet form.

.. index:: put_barc_j

.. _optimizer_task_putbarcj:

``put_barc_j()``
----------------

.. code-block:: rust

    pub fn put_barc_j ( &self,
                        j        : i32,
                        sub_     : & [i64],
                        weights_ : & [f64] )

``j``
    Index of the element in barc` that should be changed.
``sub``
    sub is list of indexes of those symmetric matrices appearing in sum.
``weights``
    The weights of the terms in the weighted sum.

Changes one element in barc.

.. index:: put_bars_j

.. _optimizer_task_putbarsj:

``put_bars_j()``
----------------

.. code-block:: rust

    pub fn put_bars_j ( &self,
                        whichsol : i32,
                        j        : i32,
                        barsj_   : & [f64] )

``whichsol``
    
``j``
    Index of the semidefinite variable.
``barsj``
    Value of the j'th variable of barx.

Sets the dual solution for a semidefinite variable.

.. index:: put_barvar_name

.. _optimizer_task_putbarvarname:

``put_barvar_name()``
---------------------

.. code-block:: rust

    pub fn put_barvar_name ( &self,
                             j     : i32,
                             name  : &str )

``j``
    Index of the variable.
``name``
    The variable name.

Puts the name of a semidefinite variable.

.. index:: put_barx_j

.. _optimizer_task_putbarxj:

``put_barx_j()``
----------------

.. code-block:: rust

    pub fn put_barx_j ( &self,
                        whichsol : i32,
                        j        : i32,
                        barxj_   : & [f64] )

``whichsol``
    
``j``
    Index of the semidefinite variable.
``barxj``
    Value of the j'th variable of barx.

Sets the primal solution for a semidefinite variable.

.. index:: put_bound

.. _optimizer_task_putbound:

``put_bound()``
---------------

.. code-block:: rust

    pub fn put_bound ( &self,
                       accmode : i32,
                       i       : i32,
                       bk      : i32,
                       bl      : f64,
                       bu      : f64 )

``accmode``
    Defines whether the bound for a constraint or a variable is changed.
``i``
    Index of the constraint or variable.
``bk``
    New bound key.
``bl``
    New lower bound.
``bu``
    New upper bound.

Changes the bound for either one constraint or one variable.

.. index:: put_bound_list

.. _optimizer_task_putboundlist:

``put_bound_list()``
--------------------

.. code-block:: rust

    pub fn put_bound_list ( &self,
                            accmode : i32,
                            sub_    : & [i32],
                            bk_     : & [i32],
                            bl_     : & [f64],
                            bu_     : & [f64] )

``accmode``
    Defines whether to access bounds on variables or constraints.
``sub``
    Subscripts of the bounds that should be changed.
``bk``
    Bound keys for variables or constraints.
``bl``
    Bound keys for variables or constraints.
``bu``
    Constraint or variable upper bounds.

Changes the bounds of constraints or variables.

.. index:: put_bound_slice

.. _optimizer_task_putboundslice:

``put_bound_slice()``
---------------------

.. code-block:: rust

    pub fn put_bound_slice ( &self,
                             con   : i32,
                             first : i32,
                             last  : i32,
                             bk_   : & [i32],
                             bl_   : & [f64],
                             bu_   : & [f64] )

``con``
    Determines whether variables or constraints are modified.
``first``
    
``last``
    
``bk``
    
``bl``
    
``bu``
    

Modifies bounds.

.. index:: put_c_j

.. _optimizer_task_putcj:

``put_c_j()``
-------------

.. code-block:: rust

    pub fn put_c_j ( &self,
                     j     : i32,
                     cj    : f64 )

``j``
    Index of the variable whose objective coefficient should be changed.
``cj``
    New coefficient value.

Modifies one linear coefficient in the objective.

.. index:: put_c_list

.. _optimizer_task_putclist:

``put_c_list()``
----------------

.. code-block:: rust

    pub fn put_c_list ( &self,
                        subj_ : & [i32],
                        val_  : & [f64] )

``subj``
    Index of variables for which objective coefficients should be changed.
``val``
    New numerical values for the objective coefficients that should be modified.

Modifies a part of the linear objective coefficients.

.. index:: put_c_slice

.. _optimizer_task_putcslice:

``put_c_slice()``
-----------------

.. code-block:: rust

    pub fn put_c_slice ( &self,
                         first  : i32,
                         last   : i32,
                         slice_ : & [f64] )

``first``
    First element in the slice of c.
``last``
    Last element plus 1 of the slice in c to be changed.
``slice``
    New numerical values for the objective coefficients that should be modified.

Modifies a slice of the linear objective coefficients.

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

    pub fn put_cfix ( &self,cfix  : f64 )

``cfix``
    

Replaces the fixed term in the objective.

.. index:: put_con_bound

.. _optimizer_task_putconbound:

``put_con_bound()``
-------------------

.. code-block:: rust

    pub fn put_con_bound ( &self,
                           i     : i32,
                           bk    : i32,
                           bl    : f64,
                           bu    : f64 )

``i``
    Index of the constraint.
``bk``
    New bound key.
``bl``
    New lower bound.
``bu``
    New upper bound.

Changes the bound for one constraint.

.. index:: put_con_bound_list

.. _optimizer_task_putconboundlist:

``put_con_bound_list()``
------------------------

.. code-block:: rust

    pub fn put_con_bound_list ( &self,
                                sub_  : & [i32],
                                bkc_  : & [i32],
                                blc_  : & [f64],
                                buc_  : & [f64] )

``sub``
    List constraints indexes.
``bkc``
    New bound keys.
``blc``
    New lower bound values.
``buc``
    New upper bounds values.

Changes the bounds of a list of constraints.

.. index:: put_con_bound_slice

.. _optimizer_task_putconboundslice:

``put_con_bound_slice()``
-------------------------

.. code-block:: rust

    pub fn put_con_bound_slice ( &self,
                                 first : i32,
                                 last  : i32,
                                 bk_   : & [i32],
                                 bl_   : & [f64],
                                 bu_   : & [f64] )

``first``
    Index of the first constraint in the slice.
``last``
    Index of the last constraint in the slice plus 1.
``bk``
    New bound keys.
``bl``
    New lower bounds.
``bu``
    New upper bounds.

Changes the bounds for a slice of the constraints.

.. index:: put_con_name

.. _optimizer_task_putconname:

``put_con_name()``
------------------

.. code-block:: rust

    pub fn put_con_name ( &self,
                          i     : i32,
                          name  : &str )

``i``
    Index of the constraint.
``name``
    The variable name.

Puts the name of a constraint.

.. index:: put_cone

.. _optimizer_task_putcone:

``put_cone()``
--------------

.. code-block:: rust

    pub fn put_cone ( &self,
                      k       : i32,
                      ct      : i32,
                      conepar : f64,
                      submem_ : & [i32] )

``k``
    Index of the cone.
``ct``
    
``conepar``
    
``submem``
    

Replaces a conic constraint.

.. index:: put_cone_name

.. _optimizer_task_putconename:

``put_cone_name()``
-------------------

.. code-block:: rust

    pub fn put_cone_name ( &self,
                           j     : i32,
                           name  : &str )

``j``
    Index of the cone.
``name``
    The variable name.

Puts the name of a cone.

.. index:: put_dou_param

.. _optimizer_task_putdouparam:

``put_dou_param()``
-------------------

.. code-block:: rust

    pub fn put_dou_param ( &self,
                           param    : i32,
                           parvalue : f64 )

``param``
    
``parvalue``
    

Sets a double parameter.

.. index:: put_int_param

.. _optimizer_task_putintparam:

``put_int_param()``
-------------------

.. code-block:: rust

    pub fn put_int_param ( &self,
                           param    : i32,
                           parvalue : i32 )

``param``
    
``parvalue``
    

Sets an integer parameter.

.. index:: put_max_num_a_nz

.. _optimizer_task_putmaxnumanz:

``put_max_num_a_nz()``
----------------------

.. code-block:: rust

    pub fn put_max_num_a_nz ( &self,maxnumanz : i64 )

``maxnumanz``
    New size of the storage reserved for storing the linear coefficient matrix.

The function changes the size of the preallocated storage for linear coefficients.

.. index:: put_max_num_barvar

.. _optimizer_task_putmaxnumbarvar:

``put_max_num_barvar()``
------------------------

.. code-block:: rust

    pub fn put_max_num_barvar ( &self,maxnumbarvar : i32 )

``maxnumbarvar``
    The maximum number of semidefinite variables.

Sets the number of preallocated symmetric matrix variables in the optimization task.

.. index:: put_max_num_con

.. _optimizer_task_putmaxnumcon:

``put_max_num_con()``
---------------------

.. code-block:: rust

    pub fn put_max_num_con ( &self,maxnumcon : i32 )

``maxnumcon``
    

Sets the number of preallocated constraints in the optimization task.

.. index:: put_max_num_cone

.. _optimizer_task_putmaxnumcone:

``put_max_num_cone()``
----------------------

.. code-block:: rust

    pub fn put_max_num_cone ( &self,maxnumcone : i32 )

``maxnumcone``
    

Sets the number of preallocated conic constraints in the optimization task.

.. index:: put_max_num_q_nz

.. _optimizer_task_putmaxnumqnz:

``put_max_num_q_nz()``
----------------------

.. code-block:: rust

    pub fn put_max_num_q_nz ( &self,maxnumqnz : i64 )

``maxnumqnz``
    

Changes the size of the preallocated storage for quadratic terms.

.. index:: put_max_num_var

.. _optimizer_task_putmaxnumvar:

``put_max_num_var()``
---------------------

.. code-block:: rust

    pub fn put_max_num_var ( &self,maxnumvar : i32 )

``maxnumvar``
    

Sets the number of preallocated variables in the optimization task.

.. index:: put_na_dou_param

.. _optimizer_task_putnadouparam:

``put_na_dou_param()``
----------------------

.. code-block:: rust

    pub fn put_na_dou_param ( &self,
                              paramname : &str,
                              parvalue  : f64 )

``paramname``
    
``parvalue``
    

Sets a double parameter.

.. index:: put_na_int_param

.. _optimizer_task_putnaintparam:

``put_na_int_param()``
----------------------

.. code-block:: rust

    pub fn put_na_int_param ( &self,
                              paramname : &str,
                              parvalue  : i32 )

``paramname``
    
``parvalue``
    

Sets an integer parameter.

.. index:: put_na_str_param

.. _optimizer_task_putnastrparam:

``put_na_str_param()``
----------------------

.. code-block:: rust

    pub fn put_na_str_param ( &self,
                              paramname : &str,
                              parvalue  : &str )

``paramname``
    
``parvalue``
    

Sets a string parameter.

.. index:: put_obj_name

.. _optimizer_task_putobjname:

``put_obj_name()``
------------------

.. code-block:: rust

    pub fn put_obj_name ( &self,objname : &str )

``objname``
    

Assigns a new name to the objective.

.. index:: put_obj_sense

.. _optimizer_task_putobjsense:

``put_obj_sense()``
-------------------

.. code-block:: rust

    pub fn put_obj_sense ( &self,sense : i32 )

``sense``
    The objective sense of the task

Sets the objective sense.

.. index:: put_param

.. _optimizer_task_putparam:

``put_param()``
---------------

.. code-block:: rust

    pub fn put_param ( &self,
                       parname  : &str,
                       parvalue : &str )

``parname``
    
``parvalue``
    

Modifies the value of parameter.

.. index:: put_q_con

.. _optimizer_task_putqcon:

``put_q_con()``
---------------

.. code-block:: rust

    pub fn put_q_con ( &self,
                       qcsubk_ : & [i32],
                       qcsubi_ : & [i32],
                       qcsubj_ : & [i32],
                       qcval_  : & [f64] )

``qcsubk``
    
``qcsubi``
    
``qcsubj``
    
``qcval``
    

Replaces all quadratic terms in constraints.

.. index:: put_q_con_k

.. _optimizer_task_putqconk:

``put_q_con_k()``
-----------------

.. code-block:: rust

    pub fn put_q_con_k ( &self,
                         k       : i32,
                         qcsubi_ : & [i32],
                         qcsubj_ : & [i32],
                         qcval_  : & [f64] )

``k``
    The constraint in which the new quadratic elements are inserted.
``qcsubi``
    
``qcsubj``
    
``qcval``
    

Replaces all quadratic terms in a single constraint.

.. index:: put_q_obj

.. _optimizer_task_putqobj:

``put_q_obj()``
---------------

.. code-block:: rust

    pub fn put_q_obj ( &self,
                       qosubi_ : & [i32],
                       qosubj_ : & [i32],
                       qoval_  : & [f64] )

``qosubi``
    
``qosubj``
    
``qoval``
    

Replaces all quadratic terms in the objective.

.. index:: put_q_obj_i_j

.. _optimizer_task_putqobjij:

``put_q_obj_i_j()``
-------------------

.. code-block:: rust

    pub fn put_q_obj_i_j ( &self,
                           i     : i32,
                           j     : i32,
                           qoij  : f64 )

``i``
    Row index for the coefficient to be replaced.
``j``
    Column index for the coefficient to be replaced.
``qoij``
    The new coefficient value.

Replaces one coefficient in the quadratic term in the objective.

.. index:: put_skc

.. _optimizer_task_putskc:

``put_skc()``
-------------

.. code-block:: rust

    pub fn put_skc ( &self,
                     whichsol : i32,
                     skc_     : & [i32] )

``whichsol``
    
``skc``
    

Sets the status keys for the constraints.

.. index:: put_skc_slice

.. _optimizer_task_putskcslice:

``put_skc_slice()``
-------------------

.. code-block:: rust

    pub fn put_skc_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           skc_     : & [i32] )

``whichsol``
    
``first``
    
``last``
    
``skc``
    

Sets the status keys for the constraints.

.. index:: put_skx

.. _optimizer_task_putskx:

``put_skx()``
-------------

.. code-block:: rust

    pub fn put_skx ( &self,
                     whichsol : i32,
                     skx_     : & [i32] )

``whichsol``
    
``skx``
    

Sets the status keys for the scalar variables.

.. index:: put_skx_slice

.. _optimizer_task_putskxslice:

``put_skx_slice()``
-------------------

.. code-block:: rust

    pub fn put_skx_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           skx_     : & [i32] )

``whichsol``
    
``first``
    
``last``
    
``skx``
    

Sets the status keys for the variables.

.. index:: put_slc

.. _optimizer_task_putslc:

``put_slc()``
-------------

.. code-block:: rust

    pub fn put_slc ( &self,
                     whichsol : i32,
                     slc_     : & [f64] )

``whichsol``
    
``slc``
    The slc vector.

Sets the slc vector for a solution.

.. index:: put_slc_slice

.. _optimizer_task_putslcslice:

``put_slc_slice()``
-------------------

.. code-block:: rust

    pub fn put_slc_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           slc_     : & [f64] )

``whichsol``
    
``first``
    
``last``
    
``slc``
    

Sets a slice of the slc vector for a solution.

.. index:: put_slx

.. _optimizer_task_putslx:

``put_slx()``
-------------

.. code-block:: rust

    pub fn put_slx ( &self,
                     whichsol : i32,
                     slx_     : & [f64] )

``whichsol``
    
``slx``
    The slx vector.

Sets the slx vector for a solution.

.. index:: put_slx_slice

.. _optimizer_task_putslxslice:

``put_slx_slice()``
-------------------

.. code-block:: rust

    pub fn put_slx_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           slx_     : & [f64] )

``whichsol``
    
``first``
    
``last``
    
``slx``
    

Sets a slice of the slx vector for a solution.

.. index:: put_snx

.. _optimizer_task_putsnx:

``put_snx()``
-------------

.. code-block:: rust

    pub fn put_snx ( &self,
                     whichsol : i32,
                     sux_     : & [f64] )

``whichsol``
    
``sux``
    The snx vector.

Sets the snx vector for a solution.

.. index:: put_snx_slice

.. _optimizer_task_putsnxslice:

``put_snx_slice()``
-------------------

.. code-block:: rust

    pub fn put_snx_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           snx_     : & [f64] )

``whichsol``
    
``first``
    
``last``
    
``snx``
    

Sets a slice of the snx vector for a solution.

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
                          snx_     : & [f64] )

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
    

Inserts a solution.

.. index:: put_solution_i

.. _optimizer_task_putsolutioni:

``put_solution_i()``
--------------------

.. code-block:: rust

    pub fn put_solution_i ( &self,
                            accmode  : i32,
                            i        : i32,
                            whichsol : i32,
                            sk       : i32,
                            x        : f64,
                            sl       : f64,
                            su       : f64,
                            sn       : f64 )

``accmode``
    Defines whether solution information for a constraint or for a variable is modified.
``i``
    Index of the constraint or variable.
``whichsol``
    
``sk``
    Status key of the constraint or variable.
``x``
    Solution value of the primal constraint or variable.
``sl``
    Solution value of the dual variable associated with the lower bound.
``su``
    Solution value of the dual variable associated with the upper bound.
``sn``
    Solution value of the dual variable associated with the cone constraint.

Sets the primal and dual solution information for a single constraint or variable.

.. index:: put_solution_y_i

.. _optimizer_task_putsolutionyi:

``put_solution_y_i()``
----------------------

.. code-block:: rust

    pub fn put_solution_y_i ( &self,
                              i        : i32,
                              whichsol : i32,
                              y        : f64 )

``i``
    Index of the dual variable.
``whichsol``
    
``y``
    Solution value of the dual variable.

Inputs the dual variable of a solution.

.. index:: put_str_param

.. _optimizer_task_putstrparam:

``put_str_param()``
-------------------

.. code-block:: rust

    pub fn put_str_param ( &self,
                           param    : i32,
                           parvalue : &str )

``param``
    
``parvalue``
    

Sets a string parameter.

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
                     suc_     : & [f64] )

``whichsol``
    
``suc``
    The suc vector.

Sets the suc vector for a solution.

.. index:: put_suc_slice

.. _optimizer_task_putsucslice:

``put_suc_slice()``
-------------------

.. code-block:: rust

    pub fn put_suc_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           suc_     : & [f64] )

``whichsol``
    
``first``
    
``last``
    
``suc``
    

Sets a slice of the suc vector for a solution.

.. index:: put_sux

.. _optimizer_task_putsux:

``put_sux()``
-------------

.. code-block:: rust

    pub fn put_sux ( &self,
                     whichsol : i32,
                     sux_     : & [f64] )

``whichsol``
    
``sux``
    The sux vector.

Sets the sux vector for a solution.

.. index:: put_sux_slice

.. _optimizer_task_putsuxslice:

``put_sux_slice()``
-------------------

.. code-block:: rust

    pub fn put_sux_slice ( &self,
                           whichsol : i32,
                           first    : i32,
                           last     : i32,
                           sux_     : & [f64] )

``whichsol``
    
``first``
    
``last``
    
``sux``
    

Sets a slice of the sux vector for a solution.

.. index:: put_task_name

.. _optimizer_task_puttaskname:

``put_task_name()``
-------------------

.. code-block:: rust

    pub fn put_task_name ( &self,taskname : &str )

``taskname``
    

Assigns a new name to the task.

.. index:: put_var_bound

.. _optimizer_task_putvarbound:

``put_var_bound()``
-------------------

.. code-block:: rust

    pub fn put_var_bound ( &self,
                           j     : i32,
                           bk    : i32,
                           bl    : f64,
                           bu    : f64 )

``j``
    Index of the variable.
``bk``
    New bound key.
``bl``
    New lower bound.
``bu``
    New upper bound.

Changes the bound for one variable.

.. index:: put_var_bound_list

.. _optimizer_task_putvarboundlist:

``put_var_bound_list()``
------------------------

.. code-block:: rust

    pub fn put_var_bound_list ( &self,
                                sub_  : & [i32],
                                bkx_  : & [i32],
                                blx_  : & [f64],
                                bux_  : & [f64] )

``sub``
    List of variable indexes.
``bkx``
    New bound keys.
``blx``
    New lower bound values.
``bux``
    New upper bounds values.

Changes the bounds of a list of variables.

.. index:: put_var_bound_slice

.. _optimizer_task_putvarboundslice:

``put_var_bound_slice()``
-------------------------

.. code-block:: rust

    pub fn put_var_bound_slice ( &self,
                                 first : i32,
                                 last  : i32,
                                 bk_   : & [i32],
                                 bl_   : & [f64],
                                 bu_   : & [f64] )

``first``
    Index of the first variable in the slice.
``last``
    Index of the last variable in the slice plus 1.
``bk``
    New bound keys.
``bl``
    New lower bounds.
``bu``
    New upper bounds.

Changes the bounds for a slice of the variables.

.. index:: put_var_name

.. _optimizer_task_putvarname:

``put_var_name()``
------------------

.. code-block:: rust

    pub fn put_var_name ( &self,
                          j     : i32,
                          name  : &str )

``j``
    Index of the variable.
``name``
    The variable name.

Puts the name of a variable.

.. index:: put_var_type

.. _optimizer_task_putvartype:

``put_var_type()``
------------------

.. code-block:: rust

    pub fn put_var_type ( &self,
                          j       : i32,
                          vartype : i32 )

``j``
    Index of the variable.
``vartype``
    The new variable type.

Sets the variable type of one variable.

.. index:: put_var_type_list

.. _optimizer_task_putvartypelist:

``put_var_type_list()``
-----------------------

.. code-block:: rust

    pub fn put_var_type_list ( &self,
                               subj_    : & [i32],
                               vartype_ : & [i32] )

``subj``
    A list of variable indexes for which the variable
                               type should be changed.
``vartype``
    A list of variable types.

Sets the variable type for one or more variables.

.. index:: put_xc

.. _optimizer_task_putxc:

``put_xc()``
------------

.. code-block:: rust

    pub fn put_xc ( &self,
                    whichsol : i32,
                    xc       : & mut [f64] )

``whichsol``
    
``xc``
    The xc vector.

Sets the xc vector for a solution.

.. index:: put_xc_slice

.. _optimizer_task_putxcslice:

``put_xc_slice()``
------------------

.. code-block:: rust

    pub fn put_xc_slice ( &self,
                          whichsol : i32,
                          first    : i32,
                          last     : i32,
                          xc_      : & [f64] )

``whichsol``
    
``first``
    
``last``
    
``xc``
    

Sets a slice of the xc vector for a solution.

.. index:: put_xx

.. _optimizer_task_putxx:

``put_xx()``
------------

.. code-block:: rust

    pub fn put_xx ( &self,
                    whichsol : i32,
                    xx_      : & [f64] )

``whichsol``
    
``xx``
    The xx vector.

Sets the xx vector for a solution.

.. index:: put_xx_slice

.. _optimizer_task_putxxslice:

``put_xx_slice()``
------------------

.. code-block:: rust

    pub fn put_xx_slice ( &self,
                          whichsol : i32,
                          first    : i32,
                          last     : i32,
                          xx_      : & [f64] )

``whichsol``
    
``first``
    
``last``
    
``xx``
    

Obtains a slice of the xx vector for a solution.

.. index:: put_y

.. _optimizer_task_puty:

``put_y()``
-----------

.. code-block:: rust

    pub fn put_y ( &self,
                   whichsol : i32,
                   y_       : & [f64] )

``whichsol``
    
``y``
    The y vector.

Sets the y vector for a solution.

.. index:: put_y_slice

.. _optimizer_task_putyslice:

``put_y_slice()``
-----------------

.. code-block:: rust

    pub fn put_y_slice ( &self,
                         whichsol : i32,
                         first    : i32,
                         last     : i32,
                         y_       : & [f64] )

``whichsol``
    
``first``
    
``last``
    
``y``
    

Sets a slice of the y vector for a solution.

.. index:: read_data

.. _optimizer_task_readdataautoformat:

``read_data()``
---------------

.. code-block:: rust

    pub fn read_data ( &self,filename : &str )

``filename``
    Input data file name.

Reads problem data from a file.

.. index:: read_data_format

.. _optimizer_task_readdataformat:

``read_data_format()``
----------------------

.. code-block:: rust

    pub fn read_data_format ( &self,
                              filename : &str,
                              format   : i32,
                              compress : i32 )

``filename``
    Input data file name.
``format``
    File data format.
``compress``
    File compression type.

Reads problem data from a file.

.. index:: read_param_file

.. _optimizer_task_readparamfile:

``read_param_file()``
---------------------

.. code-block:: rust

    pub fn read_param_file ( &self,filename : &str )

``filename``
    Input data file name.

Reads a parameter file.

.. index:: read_solution

.. _optimizer_task_readsolution:

``read_solution()``
-------------------

.. code-block:: rust

    pub fn read_solution ( &self,
                           whichsol : i32,
                           filename : &str )

``whichsol``
    
``filename``
    

Reads a solution from a file.

.. index:: read_summary

.. _optimizer_task_readsummary:

``read_summary()``
------------------

.. code-block:: rust

    pub fn read_summary ( &self,whichstream : i32 )

``whichstream``
    

Prints information about last file read.

.. index:: read_task

.. _optimizer_task_readtask:

``read_task()``
---------------

.. code-block:: rust

    pub fn read_task ( &self,filename : &str )

``filename``
    Input file name.

Load task data from a file.

.. index:: remove_barvars

.. _optimizer_task_removebarvars:

``remove_barvars()``
--------------------

.. code-block:: rust

    pub fn remove_barvars ( &self,subset_ : & [i32] )

``subset``
    Indexes of symmetric matrix which should be removed.

The function removes a number of symmetric matrix.

.. index:: remove_cones

.. _optimizer_task_removecones:

``remove_cones()``
------------------

.. code-block:: rust

    pub fn remove_cones ( &self,subset_ : & [i32] )

``subset``
    Indexes of cones which should be removed.

Removes a conic constraint from the problem.

.. index:: remove_cons

.. _optimizer_task_removecons:

``remove_cons()``
-----------------

.. code-block:: rust

    pub fn remove_cons ( &self,subset_ : & [i32] )

``subset``
    Indexes of constraints which should be removed.

The function removes a number of constraints.

.. index:: remove_vars

.. _optimizer_task_removevars:

``remove_vars()``
-----------------

.. code-block:: rust

    pub fn remove_vars ( &self,subset_ : & [i32] )

``subset``
    Indexes of variables which should be removed.

The function removes a number of variables.

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
                         maxnumqnz  : i64 )

``maxnumcon``
    New maximum number of constraints.
``maxnumvar``
    New maximum number of variables.
``maxnumcone``
    New maximum number of cones.
``maxnumanz``
    New maximum number of linear non-zero elements.
``maxnumqnz``
    New maximum number of quadratic non-zeros elements.

Resizes an optimization task.

.. index:: sensitivity_report

.. _optimizer_task_sensitivityreport:

``sensitivity_report()``
------------------------

.. code-block:: rust

    pub fn sensitivity_report ( &self,whichstream : i32 )

``whichstream``
    

Creates a sensitivity report.

.. index:: set_defaults

.. _optimizer_task_setdefaults:

``set_defaults()``
------------------

.. code-block:: rust

    pub fn set_defaults ( &self )


Resets all parameters values.

.. index:: sk_to_str

.. _optimizer_task_sktostr:

``sk_to_str()``
---------------

.. code-block:: rust

    pub fn sk_to_str ( &self,sk    : i32 ) -> String

``sk``
    A valid status key.
*Returns:* ``str``
    ``str : String``
        String corresponding to the status key.

Obtains a status key string.

.. index:: sol_sta_to_str

.. _optimizer_task_solstatostr:

``sol_sta_to_str()``
--------------------

.. code-block:: rust

    pub fn sol_sta_to_str ( &self,solsta : i32 ) -> String

``solsta``
    
*Returns:* ``str``
    ``str : String``
        String corresponding to the solution status.

Obtains a solution status string.

.. index:: solution_def

.. _optimizer_task_solutiondef:

``solution_def()``
------------------

.. code-block:: rust

    pub fn solution_def ( &self,whichsol : i32 ) -> bool

``whichsol``
    
*Returns:* ``isdef``
    ``isdef : bool``
        Is non-zero if the requested solution is defined.

Checks whether a solution is defined.

.. index:: solution_summary

.. _optimizer_task_solutionsummary:

``solution_summary()``
----------------------

.. code-block:: rust

    pub fn solution_summary ( &self,whichstream : i32 )

``whichstream``
    

Prints a short summary of the current solutions.

.. index:: solve_with_basis

.. _optimizer_task_solvewithbasis:

``solve_with_basis()``
----------------------

.. code-block:: rust

    pub fn solve_with_basis ( &self,
                              transp : i32,
                              numnz  : i32,
                              sub    : & mut [i32],
                              val    : & mut [f64] )

``transp``
    Controls which problem formulation is solved.
``numnz``
    Input (number of non-zeros in right-hand side) and output (number of non-zeros in solution vector).
``sub``
    Input (indexes of non-zeros in right-hand side) and output (indexes of non-zeros in solution vector).
``val``
    Input (right-hand side values) and output (solution vector values).
*Returns:* ``numnz``
    ``numnz : i32``
        Input (number of non-zeros in right-hand side) and output (number of non-zeros in solution vector).

Solve a linear equation system involving a basis matrix.

.. index:: str_to_cone_type

.. _optimizer_task_strtoconetype:

``str_to_cone_type()``
----------------------

.. code-block:: rust

    pub fn str_to_cone_type ( &self,str   : &str ) -> i32

``str``
    String corresponding to the cone type code.
*Returns:* ``conetype``
    ``conetype : i32``
        The cone type corresponding to str.

Obtains a cone type code.

.. index:: str_to_sk

.. _optimizer_task_strtosk:

``str_to_sk()``
---------------

.. code-block:: rust

    pub fn str_to_sk ( &self,str   : &str ) -> i32

``str``
    Status key string.
*Returns:* ``sk``
    ``sk : i32``
        Status key corresponding to the string.

Obtains a status key.

.. index:: toconic

.. _optimizer_task_toconic:

``toconic()``
-------------

.. code-block:: rust

    pub fn toconic ( &self )


Inplace reformulation of a QCQP to a COP

.. index:: update_solution_info

.. _optimizer_task_updatesolutioninfo:

``update_solution_info()``
--------------------------

.. code-block:: rust

    pub fn update_solution_info ( &self,whichsol : i32 )

``whichsol``
    

Update the information items related to the solution.

.. index:: write_data

.. _optimizer_task_writedata:

``write_data()``
----------------

.. code-block:: rust

    pub fn write_data ( &self,filename : &str )

``filename``
    Output file name.

Writes problem data to a file.

.. index:: write_json_sol

.. _optimizer_task_writejsonsol:

``write_json_sol()``
--------------------

.. code-block:: rust

    pub fn write_json_sol ( &self,filename : &str )

``filename``
    

Write a solution to a file.

.. index:: write_param_file

.. _optimizer_task_writeparamfile:

``write_param_file()``
----------------------

.. code-block:: rust

    pub fn write_param_file ( &self,filename : &str )

``filename``
    The name of parameter file.

Writes all the parameters to a parameter file.

.. index:: write_solution

.. _optimizer_task_writesolution:

``write_solution()``
--------------------

.. code-block:: rust

    pub fn write_solution ( &self,
                            whichsol : i32,
                            filename : &str )

``whichsol``
    
``filename``
    

Write a solution to a file.

.. index:: write_task

.. _optimizer_task_writetask:

``write_task()``
----------------

.. code-block:: rust

    pub fn write_task ( &self,filename : &str )

``filename``
    Output file name.

Write a complete binary dump of the task data.
