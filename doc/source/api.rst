
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
    A valid |mosek| response code. 
*Returns:* ``(symname,str)``
    ``symname : String``
        Symbolic name corresponding to ``code``. 
    ``str : String``
        Obtains a short description of a response code.

Obtains a short description of the meaning of the response code given by ``code``.  

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



Stops all threads and delete all handles used by the license system. If this
function is called, it must be called as the last |mosek| API call. No other
|mosek| API calls are valid after this.

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
    The scalar that multiplies :math:`x`.  
``x``
    The  vector.
``y``
    The  vector.

Adds :math:`\alpha x` to :math:`y`. 

.. index:: check_in_license

.. _optimizer_env_checkinlicense:

``check_in_license()``
----------------------

.. code-block:: rust

    pub fn check_in_license ( &self,feature : i32 )

``feature``
    Feature to check in to the license system.


Check in a license feature to the license server. By default all licenses
consumed by functions using a single environment is kept checked out for the
lifetime of the |mosek| environment. This function checks in a given license
feature to the license server immediately.

If the given license feature is not checked out or is in use by a call to
:ref:`fusion_optimizetrm` calling this function has no effect.

Please note that returning a license to the license server incurs a small
overhead, so frequent calls to this function should be avoided.


.. index:: checkout_license

.. _optimizer_env_checkoutlicense:

``checkout_license()``
----------------------

.. code-block:: rust

    pub fn checkout_license ( &self,feature : i32 )

``feature``
    Feature to check out from the license system.


Check out a license feature from the license server. Normally the required
license features will be automatically checked out the first time it is needed
by the function :ref:`fusion_optimizetrm`. This function can be used to check out one
or more features ahead of time.

The license will remain checked out until the environment is deleted or the function
:ref:`fusion_checkinlicense` is called.

If a given feature is already checked out when this function is called, only
one feature will be checked out from the license server.


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
    The :math:`x` vector. 
``y``
    The :math:`y` vector.
*Returns:* ``xty``
    ``xty : f64``
        The result of the inner product between :math:`x` and :math:`y`.


Computes the inner product of two vectors :math:`x,y` of lenght :math:`n\geq 0`, i.e

.. math:: x\cdot y= \sum_{i=1}^n x_i y_i.

Note that if :math:`n=0`, then the results of the operation is 0.


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
    Indicates whether the matrix :math:`A` must be transposed. 
``transb``
    Indicates whether the matrix :math:`B` must be transposed. 
``m``
    Indicates the number of rows of matrices :math:`A` and :math:`C`. 
``n``
    Indicates the number of columns of matrices :math:`B` and :math:`C`.
``k``
    Specifies the number of columns of the matrix :math:`A` and the number of rows of the matrix :math:`B`.
``alpha``
    A scalar value multipling the result of the matrix multiplication.
``a``
    The pointer to the array storing matrix A in a column-major format.
``b``
    Indicates the number of rows of matrix :math:`B` and columns of matrix :math:`A`.
``beta``
    A scalar value that multiplies :math:`C`.
``c``
    The pointer to the array storing matrix :math:`C` in a column-major format.


Performs a matrix multiplication plus addition of dense matrices. Given
:math:`A`, :math:`B` and :math:`C` of compatible dimensions, this function
computes 

.. math:: C:= \alpha op(A)op(B) + \beta C

where :math:`\alpha,\beta` are two scalar values. The function :math:`op(X)`
return :math:`X` if transX is YES, or :math:`X^T` if set to NO. Dimensions of
:math:`A,b` must therefore match those of :math:`C`.

The result of this operation is stored in :math:`C`.                  


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
    Specifies the number of rows of the matrix :math:`A`.  
``n``
    Specifies the number of columns of the matrix :math:`A`.  
``alpha``
    A scalar value multipling the matrix :math:`A`.
``a``
    A pointer to the array storing matrix :math:`A` in a column-major format.
``x``
    A pointer to the array storing the vector :math:`x`.
``beta``
    A scalar value multipling the vector :math:`y`.
``y``
    A pointer to the array storing the vector :math:`y`.


Computes the multiplication of a scaled dense matrix times a dense vector product, plus a scaled dense vector. In formula

.. math:: y = \alpha A x + \beta y,

or if trans is set to transpose.yes

.. math:: y = \alpha A^T x + \beta y,

where :math:`\alpha,\beta` are scalar values. :math:`A` is an :math:`n\times m`
matrix, :math:`x\in \mathbb{R}^{m}` and :math:`y\in \mathbb{R}^n`.

Note that the result is stored overwriting :math:`y`.


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
    Sends all output from the stream defined by ``whichstream`` to the file given by ``filename``.  
``append``
    If this argument is non-zero, the output is appended to the file.

Directs all output from a stream to a file.

..index:: new
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

.. index:: put_keep_dlls

.. _optimizer_env_putkeepdlls:

``put_keep_dlls()``
-------------------

.. code-block:: rust

    pub fn put_keep_dlls ( &self,keepdlls : i32 )

``keepdlls``
    Controls whether explicitly loaded DLLs should be kept.

Controls whether explicitly loaded DLLs should be kept.

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
    If this argument is non-zero, then |mosek| will print debug info regarding the license checkout.  

If ``licdebug`` is  non-zero, then |mosek| will print debug info regarding the license checkout.  

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
    
    If this argument is non-zero, then |mosek| will wait for a license if no license
    is available. Moreover, ``licwait-1`` is the number of milliseconds to wait between each check for an available license.
    


If ``licwait`` is non-zero, then |mosek| will wait for a license if no license
is available. Moreover, ``licwait-1`` is the number of milliseconds to wait between each check for an available license.


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

Computes all eigenvalues of a real symmetric matrix :math:`A`. Eigenvalues are stored in the :math:`w` array.  

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


Computes all the eigenvalues and eigenvectors a real symmetric matrix. 

Given the input matrix :math:`A\in \mathbb{R}^{n\times n}`, this function returns a
vector :math:`w\in \mathbb{R}^n`  containing the eigenvalues of :math:`A` and the
corresponding eigenvectors, stored in :math:`A` as well.

Therefore, this function compute the eigenvalue decomposition of :math:`A` as 

.. math:: A= U V U^T,

where :math:`V=diag(w)` and :math:`U` contains the eigen-vectors of :math:`A`.


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
    Indicates whether the upper or lower triangular part of :math:`C` is stored.
``trans``
    Indicates whether the matrix :math:`A` must be transposed. 
``n``
    Specifies the order of :math:`C`.
``k``
    Indicates the number of rows or columns of :math:`A`, and its rank.
``alpha``
    A scalar value multipling the result of the matrix multiplication.
``a``
    The pointer to the array storing matrix :math:`A` in a column-major format.
``beta``
    A scalar value that multiplies :math:`C`.
``c``
    The pointer to the array storing matrix :math:`C` in a column-major format.


Performs a symmetric rank-:math:`k` update for a symmetric matrix. 

Given a symmetric matrix :math:`C\in \mathbb{R}^{n\times n}`, two scalars
:math:`\alpha,\beta` and a matrix :math:`A` of rank :math:`k\leq n`, it
computes either 

.. math:: C= \alpha A A^T + \beta C,

or 

.. math:: C= \alpha A^T A + \beta C.

In the first case :math:`A\in \mathbb{R}^{k\times n}`, in the second :math:`A\in
\mathbb{R}^{n\times k}`.

Note that the results overwrite the matrix :math:`C`.


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
    


Print information related to the quality of the solution and
other solution statistics.

By default this function prints information about the
largest infeasibilites in the solution, the primal (and
possibly dual) objective value and the solution status.

Following parameters can be used to configure the printed statistics:

* :ref:`iparam_ana_sol_basis: enables or disables printing of statistics specific to the basis solution (condition number, number of basic variables etc_)_ Default is on_
* :coderef:`iparam.ana_sol_print_violated` enables or disables listing names of all constraints (both primal and dual) which are violated by the solution. Default is off.
* :ref:`dparam_ana_sol_infeas_tol` is the tolerance defining when a constraint is considered violated. If a constraint is violated more than this, it will be listed in the summary.



.. index:: append_barvars

.. _optimizer_task_appendbarvars:

``append_barvars()``
--------------------

.. code-block:: rust

    pub fn append_barvars ( &self,dim_  : & [i32] )

``dim``
    Dimension of symmetric matrix variables to be added.


Appends a positive semidefinite matrix variable of dimension ``dim`` to the problem.


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
    


Appends a new conic constraint to the problem. Hence, add a constraint

.. math:: \hat{x} \in \mathcal{C}

to the problem where :math:`\mathcal{C}` is a convex cone. :math:`\hat{x}` is a
subset of the variables which will be specified by the argument
``submem``.

Depending on the value of ``ct`` this function appends a normal (:ref:`fusion_`ctQuad`) or rotated quadratic cone (:ref:`fusion_ctRquad`).

Define 

.. math:: \hat{x} = x_{\mathtt{submem}[0]},\ldots,x_{\mathtt{submem}[\mathtt{nummem}-1]}.

Depending on the value of ``ct`` this function appends one of the constraints:

* Quadratic cone (:ref:`fusion_ctQuad`) : 

  .. math:: \hat{x}_0 \geq \sqrt{\sum_{i=1}^{i<\mathtt{nummem}} \hat{x}_i^2}

* Rotated quadratic cone (:ref:`fusion_ctRquad`) : 

  .. math:: 2 \hat{x}_0 \hat{x}_1 \geq \sum_{i=2}^{i<\mathtt{nummem}} \hat{x}^2_i, \quad \hat{x}_{0}, \hat{x}_1 \geq 0

Please note that the sets of variables appearing in different conic constraints must be disjoint.

For an explained code example see Section :ref:`shared-conic-opt`.



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


Appends a new conic constraint to the problem. The function assumes the members of cone are sequential where the first member has index ``j`` and the last ``j+nummem-1``.


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


Appends a number conic constraints to the problem. The :math:`k`\ th
cone is assumed to be of dimension ``nummem[k]``. Moreover, is assumed
that the first variable of the first cone has index :math:`j` and the
index of the variable in each cone are sequential. Finally, it assumed
in the second cone is the last index of first cone plus one and so
forth.


.. index:: append_cons

.. _optimizer_task_appendcons:

``append_cons()``
-----------------

.. code-block:: rust

    pub fn append_cons ( &self,num   : i32 )

``num``
    Number of constraints which should be appended.


Appends a number of constraints to the model. Appended constraints will be declared free. Please note that |mosek| will automatically expand the problem dimension to accommodate the additional constraints.


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
        Each matrix that is appended to :math:`E` is assigned a unique index i.e. ``idx`` that can be used for later reference. 


|mosek| maintains a storage of symmetric data matrixes that is used to build
the :math:`\bar{c}` and :math:`\bar{A}`. The storage can be thought of as a vector of
symmetric matrixes denoted :math:`E`. Hence, :math:`E_i` is a symmetric matrix of certain
dimension.

This function appends a general sparse symmetric matrix on triplet form to the
vector :math:`E` of symmetric matrixes.  The vectors ``subi``, ``subj``, and
``valij`` contains the row subscripts, column subscripts and values of each
element in the symmetric matrix to be appended.  Since the matrix that is
appended is symmetric then only the lower triangular part should be specified.
Moreover, duplicates are not allowed.

Observe the function reports the index (position) of the appended matrix in
:math:`E`. This index should be used for later references to the appended matrix.


.. index:: append_stat

.. _optimizer_task_appendstat:

``append_stat()``
-----------------

.. code-block:: rust

    pub fn append_stat ( &self )


Appends a record the statistics file.

.. index:: append_vars

.. _optimizer_task_appendvars:

``append_vars()``
-----------------

.. code-block:: rust

    pub fn append_vars ( &self,num   : i32 )

``num``
    Number of variables which should be appended.


Appends a number of variables to the model. Appended variables will be fixed at zero. Please note that |mosek| will automatically expand the problem dimension to accommodate the additional variables.


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


If a basic solution is available and it defines a nonsingular basis, then
this function computes the 1-norm estimate of the basis matrix and an 1-norm estimate
for the inverse of the basis matrix. The 1-norm estimates are computed using the method
outlined in :cite:`stewart:98:a`, pp. 388-391.

By definition the 1-norm condition number of a matrix :math:`B` is defined as

.. math:: \mathcal{K}_1(B) := \|B\|_1 \|B^{-1}|.

Moreover, the larger the condition number is the harder it is to solve
linear equation systems involving :math:`B`.  Given estimates for
:math:`\|B\|_1` and :math:`\|B^{-1}\|_1` it is also possible to
estimate :math:`\kappa_1(B)`.


.. index:: check_convexity

.. _optimizer_task_checkconvexity:

``check_convexity()``
---------------------

.. code-block:: rust

    pub fn check_convexity ( &self )



This function checks if a quadratic optimization problem is convex.  The amount
of checking is controlled by :ref:`iparam_check_convexity`.

The function reports an error if the problem is not convex.


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


Changes a bound for one constraint or variable. If ``accmode`` equals :ref:`fusion_accCon`, a constraint bound is changed, otherwise a variable
bound is changed.

If ``lower`` is non-zero, then the lower bound is changed as follows:

.. math::

    \mbox{new lower bound} =
        \left\{
            \begin{array}{ll}
                - \infty,     & \mathtt{finite}=0, \\
                \mathtt{value} & \mbox{otherwise}. 
            \end{array}
        \right.


Otherwise if ``lower`` is zero, then

.. math:: 

    \mbox{new upper bound} = 
        \left\{ 
            \begin{array}{ll}
                \infty,     & \mathtt{finite}=0, \\
                \mathtt{value} & \mbox{otherwise}. 
            \end{array}
        \right.


Please note that this function automatically updates the bound key for  bound, in particular, if the lower and upper bounds are identical, the  bound key is changed to ``fixed``.



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
    If non-zero, then the lower bound is changed, otherwise
                                the upper bound is changed.
``finite``
    If non-zero, then ``value` is assumed to be finite.  
``value``
    New value for the bound.


Changes a bound for one constraint.

If ``lower`` is non-zero, then the lower bound is changed as follows:

.. math::

    \mbox{new lower bound} =
      \left\{
        \begin{array}{ll}
          - \infty,       & \mathtt{finite}=0, \\
          \mathtt{value}  & \mbox{otherwise}. 
        \end{array}
      \right.

Otherwise if ``lower`` is zero, then

.. math::

    \mbox{new upper bound} = 
      \left\{
        \begin{array}{ll}
          \infty,        & \mathtt{finite}=0, \\
          \mathtt{value} & \mbox{otherwise}. 
        \end{array}
      \right.


Please note that this function automatically updates the bound key for
bound, in particular, if the lower and upper bounds are identical, the
bound key is changed to ``fixed``.


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
    If non-zero, then ``value`` is assumed to be finite.  
``value``
    New value for the bound.


Changes a bound for on variable.

If ``lower`` is non-zero, then the lower bound is changed as follows:

.. math::

    \mbox{new lower bound} =
      \left\{
        \begin{array}{ll}
          - \infty,     & \mathtt{finite}=0, \\
          \mathtt{value} & \mbox{otherwise}. 
        \end{array}
      \right.

Otherwise if ``lower`` is zero, then

.. math::

    \mbox{new upper bound} = 
      \left\{
        \begin{array}{ll}
          \infty,     & \mathtt{finite}=0, \\
          \mathtt{value} & \mbox{otherwise}. 
        \end{array}
      \right.

Please note that this function automatically updates the bound key for bound,
in particular, if the lower and upper bounds are identical, the bound key is
changed to ``fixed``.


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
    
    :math:`\mathtt{leftpricej}[j]` is the left shadow price for the
    coefficients with index :math:`\mathtt{subj[j]}`.
    
``rightpricej``
    
    :math:`\mathtt{rightpricej}[j]` is the right shadow price for the
    coefficients with index :math:`\mathtt{subj[j]}`.
    
``leftrangej``
    
    :math:`\mathtt{leftrangej}[j]` is the left range :math:`\beta_1` for the
    coefficient with index :math:`\mathtt{subj[j]}`.
    
``rightrangej``
    
    :math:`\mathtt{rightrangej}[j]` is the right range :math:`\beta_2` for the
    coefficient with index :math:`\mathtt{subj[j]}`.
    


Calculates sensitivity information for objective
coefficients. The indexes of the coefficients to analyze are

.. math:: \{\mathtt{subj}[i] | i \in 0,\ldots,\mathtt{numj}-1\}

The results are returned so that e.g :math:`\mathtt{leftprice}[j]` is the left
shadow price of the objective coefficient with index :math:`\mathtt{subj}[j]`.

The type of sensitivity analysis to perform (basis or optimal partition) is
controlled by the parameter :ref:`iparam_sensitivity_ype`.

For an example, please see Section :ref:`shared-sensitivity-apiex`.



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

Obtains one row of :math:`A` in a sparse format.  

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
        Number of non-zeros in the :math:`j`\ th row or column of :math:`A`.  

Obtains the number of non-zero elements in one column of :math:`A`.  

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
        Number of non-zero :math:`A` elements in the rectangular piece.  


Obtains the number non-zeros in a rectangular piece of :math:`A`, i.e. the number

.. math:: \left| (i,j): ~ a_{i,j} \neq 0,~ \mathtt{firsti} \leq i \leq \mathtt{lasti}-1, ~\mathtt{firstj} \leq j \leq \mathtt{lastj}-1\} \right|

where :math:`|\mathcal{I}|` means the number of elements in the set :math:`\mathcal{I}`.

This function is not an efficient way to obtain the number of non-zeros in one
row or column. In that case use the function :ref:`optimizer_task_getarownumnz` or :ref:`optimizer_task_getacolnumnz`.


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

Obtains one row of :math:`A` in a sparse format.  

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
        Number of non-zeros in the $i$th row of :math:`A`.  

Obtains the number of non-zero elements in one row of :math:`A`.  

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
    Index of the last row or column **plus one** in the sequence.  
*Returns:* ``numnz``
    ``numnz : i64``
        Number of non-zeros in the slice.

Obtains the number of non-zeros in a slice of rows or columns of :math:`A`.  

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
        The required coefficient :math:`a_{i,j}`. 

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

Obtains :math:`\bar{A}` in block triplet form.  

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
        Row index of the element at position ``idx``.  
    ``j : i32``
        Column index of the element at position ``idx``.  
    ``num : i64``
        Number of terms in weighted sum that forms the element.


Obtains information about an element in :math:`\bar{A}`. Since :math:`\bar{A}`
is a sparse matrix of symmetric matrixes then only the nonzero elements in
:math:`\bar{A}` are stored in order to save space. Now :math:`\bar{A}` is
stored vectorized form i.e. as one long vector.  This function makes it
possible to obtain information such as the row index and the column index of a
particular element of the vectorized form of :math:`\bar{A}`.

Please observe if one element of :math:`\bar{A}` is inputted multiple times
then it may be stored several times in vectorized form. In that case the
element with the highest index is the one that is used.


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
        Row index of the element at position ``idx``.  
    ``j : i32``
        Column index of the element at position ``idx``.  


Obtains information about an element in :math:`\bar{A}`. Since :math:`\bar{A}`
is a sparse matrix of symmetric matrixes only the nonzero elements in
:math:`\bar{A}` are stored in order to save space. Now :math:`\bar{A}` is
stored vectorized form i.e. as one long vector.  This function makes it
possible to obtain information such as the row index and the column index of a
particular element of the vectorized form of :math:`\bar{A}`.

Please note that if one element of :math:`\bar{A}` is inputted multiple times
then it may be stored several times in vectorized form. In that case the
element with the highest index is the one that is used.


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
        Number of terms in the weighted sum that forms the specified element in :math:`\bar{A}`.  


Each nonzero element in :math:`\bar{A}_{ij}` is formed as a weighted sum of
symmetric matrices. Using this function the number terms in the weighted sum
can be obtained. See description of :ref:`optimizer_task_appendsparsesymmat` for details
about the weighted sum.  


.. index:: get_bara_sparsity

.. _optimizer_task_getbarasparsity:

``get_bara_sparsity()``
-----------------------

.. code-block:: rust

    pub fn get_bara_sparsity ( &self,idxij : & mut [i64] ) -> i64

``idxij``
    
    Position of each nonzero element in the vectorized form of :math:`\bar{A}_{ij}`.
    Hence, ``idxij[k]`` is the vector position of the element in row
    ``subi[k]`` and column ``subj[k]`` of :math:`\bar{A}_{ij}`.
    
*Returns:* ``numnz``
    ``numnz : i64``
        Number of nonzero elements in :math:`\bar{A}`. 


The matrix :math:`\bar{A}` is assumed to be a sparse matrix of symmetric matrices.
This implies that many of elements in :math:`\bar{A}` is likely to be zero matrixes.
Therefore, in order to save space only nonzero elements in :math:`\bar{A}` are stored
on vectorized form. This function is used to obtain the sparsity pattern of
:math:`\bar{A}` and the position of each nonzero element in the vectorized form of
:math:`\bar{A}`.


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

Obtains :math:`\bar{C}` in block triplet form.  

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
        Row index in :math:`\bar{c}`.  
    ``num : i64``
        Number of terms in the weighted sum.

Obtains information about an element in :math:`\bar{c}`.  

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

Obtains information about the :math:`\bar{c}_{ij}`.  

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
        Row index in :math:`\bar{c}`.  

Obtains the row index of an element in :math:`\bar{c}`.  

.. index:: get_barc_sparsity

.. _optimizer_task_getbarcsparsity:

``get_barc_sparsity()``
-----------------------

.. code-block:: rust

    pub fn get_barc_sparsity ( &self,idxj  : & mut [i64] ) -> i64

``idxj``
    Internal positions of the nonzeros elements in :math:`\bar{c}`.  
*Returns:* ``numnz``
    ``numnz : i64``
        Number of nonzero elements in :math:`\bar{C}`. 


Internally only the nonzero elements of :math:`\bar{c}` is stored 

in a vector. This function returns which elements :math:`\bar{c}` that are
nonzero (in ``subj``) and their internal position (in ``idx``). Using the
position detailed information about each nonzero :math:`\bar{C}_j` can be
obtained using :ref:`optimizer_task_getbarcidxinfo` and :ref:`optimizer_task_getbarcidx`.


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

Obtains the dual solution for a semidefinite variable. Only the lower triangle part of :math:`\bar{s}_j` is returned because the matrix by construction is symmetric. The format is that the columns are stored sequentially in the natural order.  

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
        Is non-zero if the name ``somename`` is assigned to a semidefinite variable. 
    ``index : i32``
        If the name ``somename`` is assigned to a semidefinite variable, then ``index`` is the name of the constraint.  

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
    Value of :math:`\bar{X}_j`. 

Obtains the primal solution for a semidefinite variable. Only the lower triangle part of :math:`\bar{x}_j` is returned because the matrix by construction is symmetric. The format is that the columns are stored sequentially in the natural order.  

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
    

Obtains all objective coefficients :math:`c`. 

.. index:: get_c_j

.. _optimizer_task_getcj:

``get_c_j()``
-------------

.. code-block:: rust

    pub fn get_c_j ( &self,j     : i32 ) -> f64

``j``
    Index of the variable for which :math:`c` coefficient should be obtained.  
*Returns:* ``cj``
    ``cj : f64``
        The value of :math:`c_j`. 

Obtains one coefficient of :math:`c`.  

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
    

Obtains a sequence of elements in :math:`c`. 

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
        Is non-zero if the name ``somename`` is assigned to a constraint. 
    ``index : i32``
        If the name ``somename`` is assigned to a constraint, then ``index`` is the name of the constraint.  

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
        Is non-zero if the name ``somename`` is assigned to a cone. 
    ``index : i32``
        If the name ``somename`` is assigned to a cone, then ``index`` is the name of the cone.  

Checks whether the name ``somename`` has been assigned  to any cone. If it has been assigned to cone, then index of the cone is reported.  

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
    An array of indexes of :math:`\bar{X}` variables. 
``viol``
    ``viol[k]`` is violation of the solution for the constraint :math:`\bar{S}_{\mathtt{sub}[k]} \in \symmat`. 


Let :math:`(\bar{S}_j)^*` be the value of variable :math:`\bar{S}_j` for the
specified solution.  Then the dual violation of the solution associated with
variable :math:`\bar{S}_j` is given by

.. math:: \max(-\lambda_{\min}(\bar{S}_j),0.0).

Both when the solution is a certificate of primal infeasibility or when it is
dual feasibleness solution the violation should be small.


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
    ``viol[k]`` is the violation of dual solution associated with the constraint ``sub[k]``.


The violation of the dual solution associated with the :math:`i`'th constraint
is computed as follows

.. math:: \max( \rho( (s_l^c)_i^*,(b_l^c)_i ), \rho( (s_u^c)_i^*, -(b_u^c)_i) , |-y_i+(s_l^c)_i^*-(s_u^c)_i^*| )

where

.. math::

    \rho(x,l) =
      \left\{
        \begin{array}{ll}
           -x,   & l > -\infty , \\
           |x|, &  \mbox{otherwise}\\
        \end{array}
      \right.
 
Both when the solution is a certificate of primal infeasibility or it is a dual feasibleness solution the violation should be small.                 


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
    An array of indexes of :math:`\bar{X}` variables. 
``viol``
    ``viol[k]`` violation of the solution associated with ``sub[k]``'th dual conic constraint. 


Let :math:`(s_n^x)^*` be the value of variable :math:`(s_n^x)` for the
specified solution. For simplicity let us assume that :math:`s_n^x` is a member
of quadratic cone, then the violation is computed as follows

.. math::
    
    \left\{
      \begin{array}{ll}
        \max(0,\|(s_n^x\|_{2;n}^*-(s_n^x)_1^*) / \sqrt{2}, & (s_n^x)^* \geq -\|(s_n^x)_{2:n}^*\|, \\
        \|(s_n^x)^*\|, & \mbox{otherwise.}
      \end{array}
    \right.

Both when the solution is a certificate of primal infeasibility or when it is a
dual feasibleness solution the violation should be small.


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
    ``viol[k]`` is the maximal violation of the solution for the constraints :math:`(s_l^x)_{\mathtt{sub}[k]}\geq 0` and :math:`(s_u^x)_{\mathtt{sub}[k]}\geq 0`. 


The violation of the dual solution associated with the :math:`j`'th variable is
computed as follows

.. math:: \max \left(\rho((s_l^x)_i^*,(b_l^x)_i),\rho((s_u^x)_i^*,-(b_u^x)_i),|\sum{j=|idxbeg|}^{|idxend:numcon|} a_{ij} y_i+(s_l^x)_i^*-(s_u^x)_i^* - \tau c_j| \right)

where

.. math::

  \rho(x,l) =
    \left\{
      \begin{array}{ll}
         -x,   & l > -\infty , \\
         |x|, &  \mbox{otherwise}
      \end{array}
    \right.


:math:`\tau=0` if the solution is certificate of dual infeasibility and
:math:`\tau=1` otherwise. The formula for computing the violation is only shown
for linear case but is generalized appropriately for the more general problems.


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
    

Obtains the maximum index of an information of a given type ``inftype`` plus 1.  

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

Obtains the length of the :math:`j`\ th semidefinite variable i.e. the number of elements in the triangular part. 

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
        

Obtains number of preallocated non-zeros in $A$. When this number of non-zeros is reached |mosek| will automatically allocate more space for :math:`A`.  

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
        

Obtains the number of preallocated constraints in the optimization task. When this number of constraints is reached |mosek| will automatically allocate more space for constraints.  

.. index:: get_max_num_cone

.. _optimizer_task_getmaxnumcone:

``get_max_num_cone()``
----------------------

.. code-block:: rust

    pub fn get_max_num_cone ( &self ) -> i32

*Returns:* ``maxnumcone``
    ``maxnumcone : i32``
        


Obtains the number of preallocated cones in the optimization task. When this
number of cones is reached |mosek| will automatically allocate space for more
cones.


.. index:: get_max_num_q_nz

.. _optimizer_task_getmaxnumqnz64:

``get_max_num_q_nz()``
----------------------

.. code-block:: rust

    pub fn get_max_num_q_nz ( &self ) -> i64

*Returns:* ``maxnumqnz``
    ``maxnumqnz : i64``
        


Obtains the number of preallocated non-zeros for :math:`Q` (both objective and
constraints). When this number of non-zeros is reached |mosek| will
automatically allocate more space for :math:`Q`.


.. index:: get_max_num_var

.. _optimizer_task_getmaxnumvar:

``get_max_num_var()``
---------------------

.. code-block:: rust

    pub fn get_max_num_var ( &self ) -> i32

*Returns:* ``maxnumvar``
    ``maxnumvar : i32``
        

Obtains the number of preallocated variables in the optimization task. When this number of variables is reached |mosek| will automatically allocate more space for constraints.  

.. index:: get_mem_usage

.. _optimizer_task_getmemusagetask:

``get_mem_usage()``
-------------------

.. code-block:: rust

    pub fn get_mem_usage ( &self ) -> (i64,i64)

*Returns:* ``(meminuse,maxmemuse)``
    ``meminuse : i64``
        Amount of memory currently used by the ``task``.  
    ``maxmemuse : i64``
        Maximum amount of memory used by the ``task`` until now.  

Obtains information about the amount of memory used by a task.

.. index:: get_num_a_nz

.. _optimizer_task_getnumanz:

``get_num_a_nz()``
------------------

.. code-block:: rust

    pub fn get_num_a_nz ( &self ) -> i32

*Returns:* ``numanz``
    ``numanz : i32``
        

Obtains the number of non-zeros in :math:`A`. 

.. index:: get_num_a_nz_64

.. _optimizer_task_getnumanz64:

``get_num_a_nz_64()``
---------------------

.. code-block:: rust

    pub fn get_num_a_nz_64 ( &self ) -> i64

*Returns:* ``numanz``
    ``numanz : i64``
        

Obtains the number of non-zeros in :math:`A`. 

.. index:: get_num_bara_block_triplets

.. _optimizer_task_getnumbarablocktriplets:

``get_num_bara_block_triplets()``
---------------------------------

.. code-block:: rust

    pub fn get_num_bara_block_triplets ( &self ) -> i64

*Returns:* ``num``
    ``num : i64``
        Number elements in the block triplet form of :math:`\bar{A}.` 

Obtains an upper bound on the number of elements in the block triplet form of :math:`\bar{A}`.  

.. index:: get_num_bara_nz

.. _optimizer_task_getnumbaranz:

``get_num_bara_nz()``
---------------------

.. code-block:: rust

    pub fn get_num_bara_nz ( &self ) -> i64

*Returns:* ``nz``
    ``nz : i64``
        The number of nonzero elements in :math:`\bar{A}` i.e. the number of :math:`\bar{a}_{ij}` elements that is nonzero.

Get the number of nonzero elements in :math:`\bar{A}`.  

.. index:: get_num_barc_block_triplets

.. _optimizer_task_getnumbarcblocktriplets:

``get_num_barc_block_triplets()``
---------------------------------

.. code-block:: rust

    pub fn get_num_barc_block_triplets ( &self ) -> i64

*Returns:* ``num``
    ``num : i64``
        An upper bound on the number elements in the block trip let form of :math:`\bar{c}.` 

Obtains an upper bound on the number of elements in the block triplet form of :math:`\bar{C}`.  

.. index:: get_num_barc_nz

.. _optimizer_task_getnumbarcnz:

``get_num_barc_nz()``
---------------------

.. code-block:: rust

    pub fn get_num_barc_nz ( &self ) -> i64

*Returns:* ``nz``
    ``nz : i64``
        The number of nonzeros in :math:`\bar{c}` i.e. the number of elements :math:`\bar{c}_j` that is different from 0.

Obtains the number of nonzero elements in :math:`\bar{c}`.  

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
        Identical to the number of parameters of the type ``partype``.  

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

Get the number of symmetric matrixes stored in the vector :math:`E`.  

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
        

Obtains the name for a parameter ``param`` of type ``partype``.  

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
    An array of indexes of :math:`\bar{X}` variables. 
``viol``
    ``viol[k]`` is how much the solution violate the constraint :math:`\bar{X}_{\mathtt{sub}[k]} \in \symmat^+`. 


Let :math:`(\bar{X}_j)^*` be the value of variable :math:`\bar{X}_j` for the
specified solution.  Then the primal violation of the solution associated with
variable :math:`\bar{X}_j` is given by

.. math:: \max(-\lambda_{\min}(\bar{X}_j),0.0).


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
    ``viol[k]`` associated with the solution for the ``sub[k]``'th constraint. 


The primal violation of the solution associated of constraint is computed by

.. math:: \max(l_i^c \tau - (x_i^c)^*),(x_i^c)^* \tau - u_i^c\tau,|\sum_{j=|idxbeg|}^{|idxend:numvar|} a_{ij} x_j^* - x_i^c|)

where :math:`\tau` is defined as follows. If the solution is a certificate of
dual infeasibility, then :math:`\tau=0` and otherwise :math:`\tau=1`. Both when
the solution is a valid certificate of dual infeasibility or when it is primal
feasibleness solution the violation should be small. The above is only shown
for linear case but is appropriately generalized for the other cases.


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
    An array of indexes of :math:`\bar{X}` variables. 
``viol``
    ``viol[k]`` violation of the solution associated with ``sub[k]``'th conic constraint. 


Let :math:`x^*` be the value of variable :math:`x` for the specified solution.
For simplicity let us assume that :math:`x` is a member of quadratic cone, then
the violation is computed as follows

.. math::

  \left\{
    \begin{array}{ll}
      \max(0,\|x_{2;n}\|-x_1) / \sqrt{2}, & x_1 \geq -\|x_{2:n}\|, \\
      \|x\|, & \mbox{otherwise.}
    \end{array}
  \right.

Both when the solution is a certificate of dual infeasibility or when it is a
primal feasibleness solution the violation should be small.


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
    An array of indexes of :math:`x` variables. 
``viol``
    ``viol[k]`` is the violation associated the solution for variable :math:`x_j`.


Let :math:`x_j^*` be the value of variable :math:`x_j` for the specified
solution.  Then the primal violation of the solution associated with variable
:math:`x_j` is given by

.. math:: \max(l_j^x \tau - x_j^*,x_j^* - u_j^x\tau).

where :math:`\tau` is defined as follows. If the solution is a certificate of
dual infeasibility, then :math:`\tau=0` and otherwise :math:`\tau=1`. Both when
the solution is a valid certificate of dual infeasibility or when it is primal
feasibleness solution the violation should be small.


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

Obtains one coefficient :math:`q_{ij}^o` in the quadratic term of the objective.  

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
    See formula :eq:`ais-eq-redcost` for the definition. 
``last``
    See formula :eq:`ais-eq-redcost` for the definition.  
``redcosts``
    
    The reduced costs in the required sequence of variables are stored sequentially
    in ``redcosts`` starting at ``redcosts[|idxbeg|]``.
    


Computes the reduced costs for a sequence of variables and return them in the variable ``redcosts`` i.e.

.. math::
    :label: ais-eq-redcost

    \mathtt{redcosts}[j-\mathtt{first}] = (s_l^x)_j-(s_u^x)_j, ~j=\mathtt{first},\ldots,|idxend:last|



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
    The :math:`s_l^c` vector. 

Obtains the :math:`s_l^c` vector for a solution.  

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
    

Obtains a slice of the :math:`s_l^c` vector for a solution.  

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
    The :math:`s_l^x` vector. 

Obtains the :math:`s_l^x` vector for a solution. 

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
    

Obtains a slice of the :math:`s_l^x` vector for a solution.  

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
    The :math:`s_n^x` vector. 

Obtains the :math:`s_n^x` vector for a solution.  

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
    

Obtains a slice of the :math:`s_n^x` vector for a solution.  

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

Consider the case of linear programming. The primal problem is given by

.. math::

   \begin{array}{lccccl}
     \mbox{minimize}              &      &      & c^T x+c^f &      &        \\
     \mbox{subject to} &  l^c & \leq & A x       & \leq & u^c,     \\
     &  l^x & \leq & x         & \leq & u^x.   \\
   \end{array}


and the corresponding dual problem is

.. math::

   \begin{array}{lccl}
     \mbox{maximize}   & (l^c)^T s_l^c - (u^c)^T s_u^c         &  \\
     & + (l^x)^T s_l^x - (u^x)^T s_u^x + c^f &  \\
     \mbox{subject to} & A^T y + s_l^x - s_u^x                 & = & c, \\
     & -y    + s_l^c - s_u^c                 & = & 0, \\
     & s_l^c,s_u^c,s_l^x,s_u^x \geq 0.       &   &    \\
   \end{array}


In this case the mapping between variables and arguments to the function is as
follows:
  
* ``xx`` : Corresponds to variable :math:`x`.
* ``y``  : Corresponds to variable :math:`y`.
* ``slc``: Corresponds to variable :math:`s_l^c`.
* ``suc``: Corresponds to variable :math:`s_u^c`.
* ``slx``: Corresponds to variable :math:`s_l^x`.
* ``sux``: Corresponds to variable :math:`s_u^x`.
* ``xc`` : Corresponds to :math:`Ax`.

The meaning of the values returned by this function depend on the *solution status* returned in the argument ``solsta``. The most important possible values  of ``solsta`` are:

* :ref:`fusion_solStaOptimal` : An optimal solution satisfying the optimality criteria for continuous problems is returned.

* :ref:`fusion_solStaIntegerOptimal` : An optimal solution satisfying the optimality criteria for integer problems is returned.

* :ref:`fusion_solStaPrimFeas` : A solution satisfying the feasibility criteria.

* :ref:`fusion_solStaPrimInfeasCer` : A primal certificate of infeasibility is returned.

* :ref:`fusion_solStaDualInfeasCer` : A dual certificate of infeasibility is returned.



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
    
    If set to :ref:`fusion_accCon` the solution information for a constraint
    is retrieved. Otherwise for a variable.
    
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
        The primal objective value as computed by :ref:`optimizer_task_getprimalobj`.  
    ``pviolcon : f64``
        Maximal primal violation of the solution associated with the :math:`x^c` variables where the violations are computed by :ref:`optimizer_task_getpviolcon`.  
    ``pviolvar : f64``
        Maximal primal violation of the solution for the :math:`x^x` variables where the violations are computed by :ref:`optimizer_task_getpviolvar`.  
    ``pviolbarvar : f64``
        Maximal primal violation of solution for the :math:`\bar{X}` variables where the violations are computed by :ref:`optimizer_task_getpviolbarvar`.  
    ``pviolcone : f64``
        Maximal primal violation of solution for the conic constraints where the violations are computed by :ref:`optimizer_task_getpviolcones`.  
    ``pviolitg : f64``
        
        Maximal violation in the integer constraints. The violation for an integer
        constrained variable :math:`x_j` is given by
        
        .. math:: \min(x_j-\lfloor x_j \rfloor,\lceil x_j \rceil - x_j).
        
        This number is always zero for the interior-point and the basic solutions.
        
    ``dobj : f64``
        Dual objective value as computed as computed by :ref:`optimizer_task_getdualobj`.  
    ``dviolcon : f64``
        Maximal violation of the dual solution associated with the :math:`x^c` variable as computed by as computed by :ref:`optimizer_task_getdviolcon`.  
    ``dviolvar : f64``
        Maximal violation of the dual solution associated with the $x$ variable as computed by as computed by :ref:`optimizer_task_getdviolvar`.  
    ``dviolbarvar : f64``
        Maximal violation of the dual solution associated with the :math:`\bar{s}` variable as computed by as computed by :ref:`optimizer_task_getdviolbarvar`.  
    ``dviolcone : f64``
        Maximal violation of the dual solution associated with the dual conic constraints as computed by :ref:`optimizer_task_getdviolcones`.  

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
    
    value of the last index+1 in the slice, e.g. if :math:`xx[5,\ldots,9]` is required
    ``last`` should be :math:`10`.
    
``values``
    
    The values in the required sequence are stored sequentially
    in ``values`` starting at ``values[|idxbeg|]``.
    


Obtains a slice of the solution.

Consider the case of linear programming. The primal problem is given by

.. math::

  \begin{array}{lccccl}
    \mbox{minimize}              &      &      & c^T x+c^f &      &        \\
    \mbox{subject to} &  l^c & \leq & A x       & \leq & u^c,     \\
    &  l^x & \leq & x         & \leq & u^x.   \\
  \end{array}

and the corresponding dual problem is

.. math::
  
  \begin{array}{lccl}
    \mbox{maximize}   & (l^c)^T s_l^c - (u^c)^T s_u^c         &  \\
    & + (l^x)^T s_l^x - (u^x)^T s_u^x + c^f &  \\
    \mbox{subject to} & A^T y + s_l^x - s_u^x                 & = & c, \\
    & -y    + s_l^c - s_u^c                 & = & 0, \\
    & s_l^c,s_u^c,s_l^x,s_u^x \geq 0.       &   &    \\
  \end{array}

The ``solitem`` argument determines which part of the solution is returned:
  
* :ref:`fusion_solItemXx`  : The variable ``values`` return :math:`x`.
* :ref:`fusion_solItemY`   : The variable ``values`` return :math:`y`.
* :ref:`fusion_solItemSlc` : The variable ``values`` return :math:`s_l^c`.
* :ref:`fusion_solItemSuc` : The variable ``values`` return :math:`s_u^c`.
* :ref:`fusion_solItemSlx` : The variable ``values`` return :math:`s_l^x`.
* :ref:`fusion_solItemSux` : The variable ``values`` return :math:`s_u^x`.

A conic optimization problem has the same primal variables as in the linear case. Recall that the dual of a conic optimization problem is given by:

.. math::
  
  \begin{array}{lccccc}
    \mbox{maximize}   & (l^c)^T s_l^c - (u^c)^T s_u^c         &      &    \\
    & +(l^x)^T s_l^x - (u^x)^T s_u^x + c^f  &      &    \\
    \mbox{subject to} & A^T y + s_l^x - s_u^x + s_n^x         & =    & c, \\
    & -y + s_l^c - s_u^c                    & =    & 0, \\
    & s_l^c,s_u^c,s_l^x,s_u^x               & \geq & 0, \\
    & s_n^x \in \mathcal{C}^*               &      &    \\
  \end{array}

This introduces one additional dual variable :math:`s_n^x`. This variable can be acceded by selecting ``solitem`` as :ref:`fusion_solItemSnx`.

The meaning of the values returned by this function also depends on the *solution status* which can be obtained with :ref:`fusion_getsolsta`.
Depending on the solution status ``value`` will be:
    
* :ref:`constant_solsta_sol_sta-optimal`  A part of the  optimal solution satisfying the optimality criteria for continuous problems.
* :ref:`constant_solsta_sol_sta-IntegerOptimal`  A part of the  optimal solution satisfying the optimality criteria for integer problems.
* :ref:`constant_solsta_sol_sta-PrimFeas`        A part of the solution satisfying the feasibility criteria.
* :ref:`constant_solsta_sol_sta-PrimInfeasCer`   A part of the primal certificate of infeasibility.
* :ref:`constant_solsta_sol_sta-sol-DualInfeasCer`   A part of the dual certificate of infeasibility.



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
        If this is not NULL, the parameter value is stored here.

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
    The :math:`s_u^c` vector. 

Obtains the :math:`s_u^c` vector for a solution.  

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
    

Obtains a slice of the :math:`s_u^c` vector for a solution.  

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

Obtains the :math:`s_u^x` vector for a solution.  

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
    

Obtains a slice of the :math:`s_u^x` vector for a solution.  

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


|mosek| maintains a vector denoted by :math:`E` of symmetric data matrixes. This function makes it possible to obtain important information about an data matrix in :math:`E`.


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

.. index:: get_var_branch_dir

.. _optimizer_task_getvarbranchdir:

``get_var_branch_dir()``
------------------------

.. code-block:: rust

    pub fn get_var_branch_dir ( &self,j     : i32 ) -> i32

``j``
    Index of the variable.
*Returns:* ``direction``
    ``direction : i32``
        The branching direction assigned to variable :math:`j`. 

Obtains the branching direction for a given variable :math:`j`.

.. index:: get_var_branch_order

.. _optimizer_task_getvarbranchorder:

``get_var_branch_order()``
--------------------------

.. code-block:: rust

    pub fn get_var_branch_order ( &self,j     : i32 ) -> (i32,i32)

``j``
    Index of the variable.
*Returns:* ``(priority,direction)``
    ``priority : i32``
        The branching priority assigned to variable :math:`j`. 
    ``direction : i32``
        The preferred branching direction for the :math:`j`'th variable. 

Obtains the branching priority and direction for a given variable :math:`j`.

.. index:: get_var_branch_pri

.. _optimizer_task_getvarbranchpri:

``get_var_branch_pri()``
------------------------

.. code-block:: rust

    pub fn get_var_branch_pri ( &self,j     : i32 ) -> i32

``j``
    Index of the variable.
*Returns:* ``priority``
    ``priority : i32``
        The branching priority assigned to variable :math:`j`. 

Obtains the branching priority for a given variable :math:`j`.

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
        Is non-zero if the name ``somename`` is assigned to a variable. 
    ``index : i32``
        If the name ``somename`` is assigned to a variable, then ``index`` is the name of the variable.  

Checks whether the name ``somename`` has been assigned  to any variable. If it has been assigned to variable, then index of the variable is reported.  

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
        Variable type of variable ``j``. 

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
    The variables types corresponding to the variables specified by ``subj``.  


Obtains the variable type of one or more variables.

Upon return ``vartype[k]`` is the variable type of variable ``subj[k]``.


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
    The :math:`x^c` vector. 

Obtains the :math:`x^c` vector for a solution.  

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
    The :math:`x^x` vector. 

Obtains the :math:`x^x` vector for a solution.  

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
    

Obtains a slice of the :math:`x^x` vector for a solution.  

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
    The :math:`y` vector. 

Obtains the :math:`y` vector for a solution.  

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
    

Obtains a slice of the :math:`y` vector for a solution.  

.. index:: init_basis_solve

.. _optimizer_task_initbasissolve:

``init_basis_solve()``
----------------------

.. code-block:: rust

    pub fn init_basis_solve ( &self,basis : & mut [i32] )

``basis``
    
    The array of basis indexes to use.
    
    The array is interpreted as follows: If :math:`\mathtt{basis}[i] \leq |idxend:numcon|`, then :math:`x_{\mathtt{basis}[i]}^c` is in the basis at position :math:`i`, otherwise :math:`x_{\mathtt{basis}[i]-\mathtt{numcon}}` is in the basis at position :math:`i`.
    
    


Prepare a task for use with the :ref:`optimizer.task.solvewithbasis` function.

This function should be called

* immediately before the first call to :ref:`optimizer_task_solvewithbasis`, and
* immediately before any subsequent call to :ref:`optimizer_task_solvewithbasis` if the task has been modified. 

If the basis is singular i.e. not invertible, then the error :ref:`rescode_err-basis-singular` is reported.



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
    


Input the linear part of an optimization problem.


The non-zeros of :math:`A` are inputted column-wise in the format described in Section :ref:`optimizer-intro-cmo-rmo-matrix`.

For an explained code example see Section :ref:`optimizer-apiintro-linear-optimization` and Section :ref:`optimizer-matrix-formats`.



.. index:: is_dou_par_name

.. _optimizer_task_isdouparname:

``is_dou_par_name()``
---------------------

.. code-block:: rust

    pub fn is_dou_par_name ( &self,parname : &str ) -> i32

``parname``
    
*Returns:* ``param``
    ``param : i32``
        

Checks whether ``parname`` is a valid double parameter name.  

.. index:: is_int_par_name

.. _optimizer_task_isintparname:

``is_int_par_name()``
---------------------

.. code-block:: rust

    pub fn is_int_par_name ( &self,parname : &str ) -> i32

``parname``
    
*Returns:* ``param``
    ``param : i32``
        

Checks whether ``parname`` is a valid integer parameter name.  

.. index:: is_str_par_name

.. _optimizer_task_isstrparname:

``is_str_par_name()``
---------------------

.. code-block:: rust

    pub fn is_str_par_name ( &self,parname : &str ) -> i32

``parname``
    
*Returns:* ``param``
    ``param : i32``
        

Checks whether ``parname`` is a valid string parameter name.  

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
    The name of the file where text from the stream defined by ``whichstream`` is written.  
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
        Is either :ref:`fusion_resOk` or a termination response code.  


Calls the optimizer. Depending on the problem type and the selected optimizer
this will call one of the optimizers in |mosek|. By default the interior point
optimizer will be selected for continuous problems.  The optimizer may be
selected manually by setting the parameter :ref:`iparam_optimizer`.

.. ifconfig: msk_lang=='c'

   This function is equivalent to :ref:`optimizer_task_optimize` except in the case where
   :ref:`optimizer_task_optimize` would have returned a termination response code such as

   * :ref:`rescode_trm_max_iterations` or
   * :ref:`rescode_trm_stall`.

   Response codes comes in three categories:

   *  Errors: Indicate that an error has occurred during the optimization. E.g  that the optimizer has run out of memory (:ref:`fusion_resErrSpace`). 
   *  Warnings: Less fatal than errors. E.g :ref:`fusion_resWrnLargeCj` indicating possibly problematic problem data
   *  Termination codes: Relaying information about the conditions under which the optimizer terminated. E.g :ref:`fusion_resTrmMaxIterations` indicates that
      the optimizer finished because it reached the maximum number of iterations specified by the user. 

This function returns errors on the left hand side. Warnings are not returned and termination codes are returned in the separate argument ``trmcode``.



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
    
    :math:`(w_l^c)_i` is the weight associated with relaxing the lower bound on
    constraint :math:`i`. If the weight is negative, then the lower bound is not
    relaxed. Moreover, if the argument is |null|, then all the weights are assumed
    to be :math:`1`.
    
``wuc``
    
    :math:`(w_u^c)_i` is the weight associated with relaxing the upper bound on
    constraint :math:`i`. If the weight is negative, then the upper bound is not
    relaxed. Moreover, if the argument is |null|, then all the weights are assumed
    to be :math:`1`.
    
``wlx``
    
    :math:`(w_l^x)_j` is the weight associated with relaxing the upper bound on constraint :math:`j`. If the weight is negative, then the lower bound is not relaxed. Moreover,
    if the argument is |null|, then all the weights are assumed to be :math:`1`.
    
``wux``
    
    :math:`(w_l^x)_i` is the weight associated with relaxing the upper bound on variable :math:`j`. If the weight is negative, then the upper bound is not relaxed. Moreover,
    if the argument is |null|, then all the weights are assumed to be :math:`1`.
    


The function repairs a primal infeasible optimization problem by adjusting the bounds on the constraints and variables where the adjustment
is computed as the minimal weighted sum relaxation to the bounds on the constraints and variables. Observe the function only repairs the problem but does not
compute an optimal solution to the repaired problem. If an optimal solution is required the problem should be optimized after the repair.

The function is applicable to linear and conic problems possibly having integer constrained variables.

Observe that when computing the minimal weighted relaxation then the termination tolerance specified by the parameters of the task is employed. For instance
the parameter :ref:`iparam_mio_mode` can be used make |mosek| ignore the integer constraints during the repair which usually leads to a much faster repair.
However, the drawback is of course that the repaired problem may not have an integer feasible solution.

Note the function modifies the bounds on the constraints and variables. If this is not a desired feature, then
apply the function to a cloned task. 


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
    
    The value of ``marki[i]`` specifies for which bound (upper or lower) on
    constraint ``subi[i]`` sensitivity analysis should be
    performed.
    
``subj``
    Indexes of bounds on variables to analyze.
``markj``
    
    The value of ``markj[j]`` specifies for which bound (upper or lower) on
    variable ``subj[j]`` sensitivity analysis should be performed.  
    
``leftpricei``
    
    ``leftpricei[i]`` is the left shadow price for the upper/lower bound (indicated
    by ``marki[i]``) of the constraint with index ``subi[i]``.
    
``rightpricei``
    
    ``rightpricei[i]`` is the right shadow price for
    the  upper/lower bound (indicated by ``marki[i]``)
    of the constraint with index ``subi[i]``.
    
``leftrangei``
    
    ``leftrangei[i]`` is the left range for the upper/lower bound (indicated by ``marki[i]``)
    of the constraint with index ``subi[i]``.
    
``rightrangei``
    
    ``rightrangei[i]`` is the right range for the upper/lower bound (indicated by ``marki[i]``)
    of the constraint with index ``subi[i]``.
    
``leftpricej``
    
    ``leftpricej[j]`` is the left shadow price for the upper/lower bound (indicated by ``marki[j]``) on variable  ``subj[j]``.
    
``rightpricej``
    
    ``rightpricej[j]`` is the right shadow price for the upper/lower bound (indicated by ``marki[j]``)
     on variable  ``subj[j]``.
    
``leftrangej``
    
    ``leftrangej[j]`` is the left range for the upper/lower bound (indicated by ``marki[j]``)
     on variable ``subj[j]``.
    
``rightrangej``
    
    ``rightrangej[j]`` is the right range for the upper/lower bound (indicated by ``marki[j]``)
    on variable ``subj[j]``.
    


Calculates sensitivity information for bounds on variables and constraints.

For details on sensitivity analysis and the definitions of *shadow price* and
*linearity interval* see Section :ref:`shared-sensitivity-analysis`.

The constraints for which sensitivity analysis is performed are given by the
data structures:


#. ``subi`` Index of constraint to analyze.
#. ``marki`` Indicate for which bound of constraint ``subi[i]`` sensitivity
   analysis is performed.  If ``marki[i]`` = :ref:`fusion_markUp` the upper bound of
   constraint ``subi[i]`` is analyzed, and if ``marki[i]`` = :ref:`fusion_markLo` the
   lower bound is analyzed.  If ``subi[i]`` is an equality constraint, either
   \mskitem{mark.lo} or :ref:`fusion_markUp` can be used to select the
   constraint for sensitivity analysis.

Consider the problem:

.. math::

    \begin{array}{lccl}
    \mbox{minimize}   & x_1 + x_2 &  &\\
    \mbox{subject to} -1 \leq & x_1 - x_2                  & \leq & 1, \\
                      & x_1                       & = & 0, \\
                      & x_1 \geq 0,x_2 \geq 0  & &
    \end{array}

Suppose that

* ``numi = 1;``
* ``subi = [0];``
* ``marki`` = [:ref:`fusion_markUp`]

then

``leftpricei[0]``, ``rightpricei[0]``, ``leftrangei[0]`` and ``rightrangei[0]``
will contain the sensitivity information for the upper bound on constraint $0$
given by the expression:

.. math:: x_1 - x_2 \leq  1

Similarly, the variables for which to perform sensitivity analysis are given by
the structures:
  
#. ``subj`` Index of variables to analyze.

#. ``markj`` Indicate for which bound of variable ``subi[j]`` sensitivity
   analysis is performed.  If ``markj[j]`` = :ref:`fusion_markUp` the upper bound of
   constraint ``subi[j]`` is analyzed, and if ``markj[j]`` = :ref:`fusion_markLo` the
   lower bound is analyzed.

#. If ``subi[j]`` is an equality constraint, either :ref:`fusion_markLo` or
   :ref:`fusion_markUp` can be used to select the constraint for sensitivity
   analysis.


For an example, please see Section :ref:`shared-sensitivity-apiex`.

The type of sensitivity analysis to be performed (basis or optimal partition)
is controlled by the parameter :ref:`iparam_sensitivity_type`.



.. index:: pro_sta_to_str

.. _optimizer_task_prostatostr:

``pro_sta_to_str()``
--------------------

.. code-block:: rust

    pub fn pro_sta_to_str ( &self,prosta : i32 ) -> String

``prosta``
    
*Returns:* ``str``
    ``str : String``
        String corresponding to the status key ``prosta``.  

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
        String corresponding to the problem type key ``probtype``.  

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
    Index of column in :math:`A`.   
``subj``
    Row indexes of non-zero values in column :math:`j` of :math:`A`. 
``valj``
    New non-zero values of column :math:`j` in :math:`A`. 


Resets all the elements in column :math:`j` to zero and then do
   
.. math:: A_{\mathtt{subj}[k],\mathtt{j}} = \mathtt{valj}[k], \quad k=0,\ldots,\mathtt{nzj}-1. 


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
    Indexes of columns that should be replaced. ``comp`` should not contain duplicate values.  
``ptrb``
    
    Array of pointers to the first element in the columns stored in ``comp`` and ``comp``. 
    
    For an explanation of the meaning of ``comp`` see Section :ref:`optimizer-intro-cmo-rmo-matrix`.
    
    
``ptre``
    
    Array of pointers to the last element plus one in the columns stored in ``comp`` and ``comp``. 
    
    For an explanation of the meaning of ``comp`` see Section :ref:`optimizer-intro-cmo-rmo-matrix`.
    
    
``asub``
    ``comp`` contains the new variable indexes.  
``aval``
    


Replaces all elements in a set of columns of :math:`A`. The elements are replaced as follows  

.. math::

    \begin{array}{rl}
      \mathtt{for} & i=|idxbeg|,\ldots,|idxend:num|\\
                  & a_{\mathtt{asub}[k],\mathtt{sub}[i]} = \mathtt{aval}[k],\quad k=\mathtt{aptrb}[i],\ldots,\mathtt{aptre}[i]-1. 
    \end{array}


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
    
    Array of pointers to the first element in the columns stored in ``comp`` and ``comp``. 
    
    For an explanation of the meaning of ``comp`` see Section :ref:`optimizer-intro-cmo-rmo-matrix`.
    
    
``ptre``
    
    Array of pointers to the last element plus one in the columns stored in ``comp`` and ``comp``. 
    
    For an explanation of the meaning of ``comp`` see Section :ref:`optimizer-intro-cmo-rmo-matrix`.
    
    
``asub``
    ``comp`` contains the new variable indexes.  
``aval``
    

Replaces all elements in a set of columns of :math:`A`.  

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
    Index of row in :math:`A`.   
``subi``
    Row indexes of non-zero values in row :math:`i` of :math:`A`. 
``vali``
    New non-zero values of row :math:`i` in :math:`A`. 


Resets all the elements in row :math:`i` to zero and then do

.. math:: A_{\mathtt{i},\mathtt{subi}[k]} = \mathtt{vali}[k], \quad k=0,\ldots,\mathtt{nzi}-1. 


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
    Indexes of rows or columns that should be replaced. ``comp`` should not contain duplicate values.  
``aptrb``
    
    array of pointers to the first element in the rows 
    stored in ``comp`` and ``comp``. 
    
    for an explanation of the meaning of ``comp`` see section :ref:`optimizer-intro-cmo-rmo-matrix`.
    
    
``aptre``
    
    Array of pointers to the last element plus one in the rows
    stored in ``comp`` and ``comp``. 
    
    For an explanation of the meaning of ``comp``
     see Section :ref:`optimizer-intro-cmo-rmo-matrix`.
    
    
``asub``
    ``comp`` contains the new variable indexes.  
``aval``
    


Replaces all elements in a set of rows of :math:`A`. The elements are replaced as follows  

.. math::

    \begin{array}{rl}
      \mathtt{for} & i=|idxbeg|,\ldots,|idxend:num| \\
                   & a_{\mathtt{sub}[i],\mathtt{asub}[k]} = \mathtt{aval}[k],\quad k=\mathtt{aptrb}[i],\ldots,\mathtt{aptre}[i]-1. 
    \end{array}


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
    
    Array of pointers to the first element in the rows stored in ``comp`` and
    ``comp``. 
    
    For an explanation of the meaning of ``comp`` see Section :ref:`optimizer-intro-cmo-rmo-matrix`.
    
    
``ptre``
    
    Array of pointers to the last element plus one in the rows
    stored in ``comp`` and ``comp``. 
    
    For an explanation of the meaning of ``comp`` see Section :ref:`optimizer-intro-cmo-rmo-matrix`.
    
    
``asub``
    ``comp`` contains the new variable indexes.  
``aval``
    


Replaces all elements in a set of rows of :math:`A`. The elements is replaced as follows
.. math::

    \begin{array}{rl}
      \mathtt{for} & i=\mathtt{first},\ldots,\mathtt{last}-1 \\
                  & a_{\mathtt{sub}[i],\mathtt{asub}[k]} = \mathtt{aval}[k],\quad k=\mathtt{aptrb}[i],\ldots,\mathtt{aptre}[i]-1. 
    \end{array}



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
    New coefficient for :math:`a_{i,j}`. 


Changes a coefficient in :math:`A` using the method

.. math:: a_{\mathtt{i}\mathtt{j}} = \mathtt{aij}.


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
    New coefficient values for :math:`a_{i,j}`. 


Changes one or more coefficients in :math:`A` using the method

.. math:: a_{\mathtt{subi[k]},\mathtt{subj[k]}} = \mathtt{valij[k]}, \quad k=0,\ldots,\mathtt{num}-1.


.. index:: put_bar_aij

.. _optimizer_task_putbaraij:

``put_bar_aij()``
-----------------

.. code-block:: rust

    pub fn put_bar_aij ( &self,
                         i        : i32,
                         j        : i32,
                         sub_     : & [i64],
                         weights_ : & [f64] )

``i``
    Row index of :math:`\bar{A}`.  
``j``
    Column index of :math:`\bar{A}`.  
``sub``
    See argument ``texttt`` for an explanation.  
``weights``
    ``texttt`` times ``texttt``'th term of $E$ is added to :math:`\bar{A}_{ij}`.  


This function puts one element associated with :math:`\bar{X}_j` in the :math:`\bar{A}` matrix.

Each element in the :math:`\bar{A}` matrix is a weighted sum of
symmetric matrixes, i.e. :math:`\bar{A}_{ij}` is a symmetric matrix
with dimensions as :math:`\bar{X}_j`. By default all elements in
:math:`\bar{A}` are 0, so only non-zero elements need be added.

Setting the same elements again will overwrite the earlier entry. 

The symmetric matrixes themselves are defined separately
using the function :ref:`optimizer_task_appendsparsesymmat`.


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

Inputs the :math:`\bar{A}` in block triplet form.  

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

Inputs the :math:`\bar{C}` in block triplet form.  

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
    Index of the element in :math:`\bar{c}` that should be changed.  
``sub``
    ``sub`` is list of indexes of those symmetric matrices appearing in sum.  
``weights``
    The weights of the terms in the weighted sum that forms :math:`\mathtt{c}_j`.  


This function puts one element associated with :math:`\bar{X}_j` in the :math:`\bar{c}` vector. 

Each element in the :math:`\bar{c}` vector is a weighted sum of symmetric
matrixes, i.e. :math:`\bar{c}_j` is a symmetric matrix with dimensions as
:math:`\bar{X}_j`. By default all elements in :math:`\bar{c}` are :math:`0`, so only non-zero elements need be added.

Setting the same elements again will overwrite the earlier entry. 

The symmetric matrixes themselves are defined separately using the function
:ref:`optimizer_task_appendsparsesymmat`.


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
    Value of :math:`\bar{s}_j`. 

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
    Value of :math:`\bar{X}_j`. 

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


Changes the bounds for either one constraint or one variable.

If the a bound value specified is numerically larger than
:ref:`dparam_data_tol_bound_inf` it is considered infinite and the bound key is
changed accordingly. If a bound value is numerically larger than
:ref:`dparam_data_tol_bound_wrn`, a warning will be displayed, but the bound is
inputted as specified.


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
    Defines whether bounds for constraints (:ref:`fusion_accCon`) or variables (:ref:`fusion_accVar`) are changed.  
``sub``
    Subscripts of the bounds that should be changed.
``bk``
    Constraint or variable index ``sub[t]`` is assigned the bound key ``bk[t]``.  
``bl``
    Constraint or variable index ``sub[t]`` is assigned the lower bound ``bl[t]``.  
``bu``
    Constraint or variable index ``sub[t]`` is assigned the upper bound ``bu[t]``.  

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
    Defines whether bounds for constraints (:ref:`fusion_accCon`) or variables (:ref:`fusion_accVar`) are changed.  
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
    New value of :math:`c_j`. 


Modifies one coefficient in the linear objective vector :math:`c`, i.e.

.. math:: c_{\mathtt{j}} = \mathtt{cj}.


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


Modifies elements in the linear term :math:`c` in the objective using the principle

.. math:: c_{\mathtt{subj[t]}} = \mathtt{val[t]}, \quad t=0,\ldots,\mathtt{num}-1.

If a variable index is specified multiple times in ``subj`` only the last entry is used.


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
    First element in the slice of :math:`c`. 
``last``
    Last element plus 1 of the slice in :math:`c` to be changed. 
``slice``
    New numerical values for coefficients in :math:`c` that should be modified.  


Modifies a slice in the linear term :math:`c` in the objective using the principle

.. math:: c_{\mathtt{j}} = \mathtt{slice[j-first]}, \quad j=first,..,|idxend:last|



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
        An integer indicating where the callback was called from. 
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


Changes the bounds for one constraint.

If the a bound value specified is numerically larger than
:ref:`dparam__data_tol_bound_inf` it is considered infinite and the bound key is
changed accordingly. If a bound value is numerically larger than
:ref:`dparam_data_tol_bound_wWrn`, a warning will be displayed, but the bound is
inputted as specified.


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
    


Sets the value of an integer parameter.

.. ifconfig:: msk_lang=='java'

   Please notice that some parameters take values that are defined in Enum
   classes. This function accepts only integer values, so to use e.g. the value
   :ref:`fusion_on`, is necessary to use the member ``.value``. For example: ::

       task.putintparam(Env.iparam.opf_write_problem,Env.onoffkey.on.value)




.. index:: put_max_num_a_nz

.. _optimizer_task_putmaxnumanz:

``put_max_num_a_nz()``
----------------------

.. code-block:: rust

    pub fn put_max_num_a_nz ( &self,maxnumanz : i64 )

``maxnumanz``
    New size of the storage reserved for storing :math:`A`. 


|mosek| stores only the non-zero elements in :math:`A`.  Therefore1 |mosek|
cannot predict how much storage is required to store :math:`A`. Using this
function it is possible to specify the number of non-zeros to preallocate for
storing :math:`A`.

If the number of non-zeros in the problem is known, it is a good idea to set
``maxnumanz`` slightly larger than this number, otherwise a rough estimate can
be used. In general, if :math:`A` is inputted in many small chunks, setting
this value may speed up the data input phase.

It is not mandatory to call this function, since |mosek| will reallocate
internal structures whenever it is necessary.

Observe the function call has no effect if both ``maxnumcon`` and ``maxnumvar``
is zero.


.. index:: put_max_num_barvar

.. _optimizer_task_putmaxnumbarvar:

``put_max_num_barvar()``
------------------------

.. code-block:: rust

    pub fn put_max_num_barvar ( &self,maxnumbarvar : i32 )

``maxnumbarvar``
    The maximum number of semidefinite variables.


Sets the number of preallocated symmetric matrix variables in the optimization
task. When this number of variables is reached |mosek| will automatically
allocate more space for variables.

It is not mandatory to call this function, since its only function is to give a
hint of the amount of data to preallocate for efficiency reasons.

Please note that ``maxnumbarvar`` must be larger than the current number of
variables in the task.


.. index:: put_max_num_con

.. _optimizer_task_putmaxnumcon:

``put_max_num_con()``
---------------------

.. code-block:: rust

    pub fn put_max_num_con ( &self,maxnumcon : i32 )

``maxnumcon``
    


Sets the number of preallocated constraints in the optimization task. When this
number of constraints is reached |mosek| will automatically allocate more space
for constraints.

It is never mandatory to call this function, since |mosek| will reallocate any
internal structures whenever it is required.

Please note that ``maxnumcon`` must be larger than the current number of
constraints in the task.


.. index:: put_max_num_cone

.. _optimizer_task_putmaxnumcone:

``put_max_num_cone()``
----------------------

.. code-block:: rust

    pub fn put_max_num_cone ( &self,maxnumcone : i32 )

``maxnumcone``
    


Sets the number of preallocated conic constraints in the optimization task.
When this number of conic constraints is reached |mosek| will automatically
allocate more space for conic constraints.

It is never mandatory to call this function, since |mosek| will reallocate any
internal structures whenever it is required.

Please note that ``maxnumcon`` must be larger than the current number of
constraints in the task.


.. index:: put_max_num_q_nz

.. _optimizer_task_putmaxnumqnz:

``put_max_num_q_nz()``
----------------------

.. code-block:: rust

    pub fn put_max_num_q_nz ( &self,maxnumqnz : i64 )

``maxnumqnz``
    


|mosek| stores only the non-zero elements in :math:`Q`. Therefore, |mosek|
cannot predict how much storage is required to store :math:`Q`. Using this
function it is possible to specify the number non-zeros to preallocate for
storing :math:`Q` (both objective and constraints).

It may be advantageous to reserve more non-zeros for :math:`Q` than actually
needed since it may improve the internal efficiency of |mosek|, however, it is
never worthwhile to specify more than the double of the anticipated number of
non-zeros in :math:`Q`.

It is never mandatory to call this function, since its only function is to give
a hint of the amount of data to preallocate for efficiency reasons.


.. index:: put_max_num_var

.. _optimizer_task_putmaxnumvar:

``put_max_num_var()``
---------------------

.. code-block:: rust

    pub fn put_max_num_var ( &self,maxnumvar : i32 )

``maxnumvar``
    


Sets the number of preallocated variables in the optimization task. When this
number of variables is reached |mosek| will automatically allocate more space
for variables.

It is never mandatory to call this function, since its only function is to give
a hint of the amount of data to preallocate for efficiency reasons.

Please note that ``maxnumvar`` must be larger than the current number of
variables in the task.


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
    

Assigns the name given by ``objname`` to the objective function.  

.. index:: put_obj_sense

.. _optimizer_task_putobjsense:

``put_obj_sense()``
-------------------

.. code-block:: rust

    pub fn put_obj_sense ( &self,sense : i32 )

``sense``
    
    The objective sense of the task. The values :ref:`fusion_objectiveSenseMaximize`  and
    :ref:`fusion_objectiveSenseMinimize`  means that the problem is maximized or
    minimized respectively.
    

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
    


Checks if a ``parname`` is valid parameter name. If it is, the parameter is
assigned the value specified by ``parvalue``.


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
    


Replace all quadratic entries in the constraints. consider constraints on the form:

.. math:: l_k^c \leq  \frac{1}{2} \sum_{i=0}^{|idxend:numvar|} \sum_{j=0}^{|idxend:numvar|} q_{ij}^k x_i x_j + \sum_{j=0}^{|idxend:numvar|} a_{kj} x_j \leq u_k^c, ~  k=0,\ldots,m-1.

the function assigns values to :math:`q` such that:

.. math:: q_{\mathtt{qcsubi[t]},\mathtt{qcsubj[t]}}^{\mathtt{qcsubk[t]}} = \mathtt{qcval[t]},~t=0,\ldots,\mathtt{numqcnz}-1.

and

.. math:: q_{\mathtt{\mathtt{qcsubj[t]},qcsubi[t]}}^{\mathtt{qcsubk[t]}} = \mathtt{qcval[t]},~t=0,\ldots,\mathtt{numqcnz}-1.

values not assigned are set to zero.

Please note that duplicate entries are added together.


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
    


Replaces all the quadratic entries in one constraint :math:`k` of the form:

 .. math:: l_k^c \leq  \frac{1}{2} \sum_{i=|idxbeg|}^{|idxend:numvar|} \sum_{j=|idxbeg|}^{|idxend:numvar|} q_{ij}^k x_i x_j + \sum_{j=|idxbeg|}^{|idxend:numvar|} a_{kj} x_j \leq u_k^c.

It is assumed that :math:`Q^k` is symmetric, i.e. :math:`q^k_{ij} = q^k_{ji}`,and therefore, only the values of :math:`q^k_{ij}` for which :math:`i \geq j`
should be inputted to |mosek|.  To be precise, |mosek| uses the following procedure

.. math::

    \begin{array}{ll}
    1. & Q^{k}  = 0\\
    2. & \mbox{for } t=|idxbeg| \mbox{ to }|idxend:numqcnz| \\
    3. & \qquad q_{\mathtt{qcsubi[t]},\mathtt{qcsubj[t]}}^{k} = q_{\mathtt{qcsubi[t]},\mathtt{qcsubj[t]}}^{k} + \mathtt{qcval[t]} \\
    3. & \qquad q_{\mathtt{qcsubj[t]},\mathtt{qcsubi[t]}}^{k} = q_{\mathtt{qcsubj[t]},\mathtt{qcsubi[t]}}^{k} + \mathtt{qcval[t]} \\
    \end{array}

Please note that:

*   For large problems it is essential for the efficiency that the function
    :ref:`optimizer_task_putmaxnumqnz` is employed to specify an appropriate
    \texttt{maxnumqnz}.
*   Only the lower triangular part should be specified because :math:`Q^k` is
    symmetric. Specifying values for :math:`q^k_{ij}` where :math:`i < j`
    will result in an error. 
*   Only non-zero elements should be specified.
*   The order in which the non-zero elements are specified is insignificant.
*   Duplicate elements are added together. Hence, it is recommended not to
    specify the same element multiple times in \texttt{qcsubi},
    \texttt{qcsubj}, and \texttt{qcval}.

For a code example see Section :ref:`optimizer-quadratic-opt`


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
    


Replaces all the quadratic terms in the objective

.. math:: \frac{1}{2} \sum_{i=|idxbeg|}^{|idxend:numvar|} \sum_{j=|idxbeg|}^{|idxend:numvar|} q_{ij}^o x_i x_j + \sum_{j=|idxbeg|}^{|idxend:numvar|} c_j x_j + c^f.

It is assumed that :math:`Q^o` is symmetric, i.e. :math:`q^o_{ij} = q^o_{ji}`, and therefore, only the values of :math:`q^o_{ij}` for which :math:`i \geq j` should be specified.  To be precise, |mosek| uses the following procedure

.. math::

   \begin{array}{ll}
    1. & Q^o = 0\\
    2. & \mbox{for } t=|idxbeg| \mbox{ to } |idxend:numqonz| \\
    3. & \qquad q_{\mathtt{qosubi[t]},\mathtt{qosubj[t]}}^o = q_{\mathtt{qosubi[t]},\mathtt{qosubj[t]}}^o + \mathtt{qoval[t]} \\
    3. & \qquad q_{\mathtt{qosubj[t]},\mathtt{qosubi[t]}}^o = q_{\mathtt{qosubj[t]},\mathtt{qosubi[t]}}^o + \mathtt{qoval[t]} \\
    \end{array}

Please note that:

* Only the lower triangular part should be specified because :math:`Q^o` is symmetric. Specifying values for :math:`q^o_{ij}` where :math:`i < j` will result in an error. 

* Only non-zero elements should be specified.

* The order in which the non-zero elements are specified is insignificant.

* Duplicate entries are added to together.

For a code example see Section :ref:`optimizer-quadratic-objective`.



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
    The new value for :math:`q_{ij}^o`. 


Replaces one coefficient in the quadratic term in the objective. The function
performs the assignment

.. math:: q_{\mathtt{i}\mathtt{j}}^o = \mathtt{qoij}.

Only the elements in the lower triangular part are accepted. Setting
:math:`q_{ij}` with :math:`j>i` will cause an error.

Please note that replacing all quadratic element, one at a time, is more
computationally expensive than replacing all elements at once. Use
:ref:`optimizer_task_putqobj` instead whenever possible.


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
    The :math:`s_l^c` vector. 

Sets the :math:`s_l^c` vector for a solution.  

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
    

Sets a slice of the :math:`s_l^c` vector for a solution.  

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
    The :math:`s_l^x` vector. 

Sets the :math:`s_l^x` vector for a solution.  

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
    

Sets a slice of the :math:`s_l^x` vector for a solution.  

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
    The :math:`s_n^x` vector. 

Sets the :math:`s_n^x` vector for a solution.  

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
    

Sets a slice of the :math:`s_n^x` vector for a solution.  

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
    
    If set to :ref:`fusion_accCon` the solution information for a constraint
    is modified. Otherwise for a variable.
    
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
    The :math:`s_u^c` vector. 

Sets the :math:`s_u^c` vector for a solution.  

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
    

Sets a slice of the :math:`s_u^c` vector for a solution.  

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
    The :math:`s_u^x` vector. 

Sets the :math:`s_u^x` vector for a solution.  

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
    

Sets a slice of the :math:`s_u^x` vector for a solution.  

.. index:: put_task_name

.. _optimizer_task_puttaskname:

``put_task_name()``
-------------------

.. code-block:: rust

    pub fn put_task_name ( &self,taskname : &str )

``taskname``
    

Assigns the name ``taskname`` to the task. 

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


Changes the bounds for one variable.

If the a bound value specified is numerically larger than
:ref:`dparam_data_tol_bound_inf` it is considered infinite and the bound key is
changed accordingly. If a bound value is numerically larger than
:ref:`dparam_data_tol_bound_wrn`, a warning will be displayed, but the bound is
inputted as specified.


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

.. index:: put_var_branch_order

.. _optimizer_task_putvarbranchorder:

``put_var_branch_order()``
--------------------------

.. code-block:: rust

    pub fn put_var_branch_order ( &self,
                                  j         : i32,
                                  priority  : i32,
                                  direction : i32 )

``j``
    Index of the variable.
``priority``
    The branching priority that should be assigned to variable :math:`j`. 
``direction``
    Specifies the preferred branching direction for variable :math:`j`. 

Assigns a branching priority and direction to a variable.

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
    
    A list of variable types that should be assigned to the variables specified by
    ``subj``. See section :ref:`fusion_variabletype` for the possible values of
    ``vartype``.
    


Sets the variable type for one or more variables, i.e.  variable number
:math:`\mathtt{subj}[k]` is assigned the variable type
:math:`\mathtt{vartype}[k]`.

If the same index is specified multiple times in ``subj`` only the last entry
takes effect.


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

Sets the :math:`x^c` vector for a solution.  

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
    

Sets a slice of the :math:`x^c` vector for a solution.  

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
    The :math:`x^x` vector. 

Sets the :math:`x^x` vector for a solution.  

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
    The :math:`y` vector. 

Sets the :math:`y` vector for a solution.  

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
    

Sets a slice of the :math:`y` vector for a solution.  

.. index:: read_branch_priorities

.. _optimizer_task_readbranchpriorities:

``read_branch_priorities()``
----------------------------

.. code-block:: rust

    pub fn read_branch_priorities ( &self,filename : &str )

``filename``
    Data is read from the file ``filename``.  

Reads branching priority data from a file.

.. index:: read_data

.. _optimizer_task_readdataautoformat:

``read_data()``
---------------

.. code-block:: rust

    pub fn read_data ( &self,filename : &str )

``filename``
    Data is read from the file ``filename``.  

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
    
    Data is read from the file ``filename``
    if it is a nonempty string. Otherwise data is read
    from the file specified by :ref:`sparam_param_read_file_name`.
    

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
    

Reads a solution file and inserts the solution into the solution ``whichsol``.  

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


Load task data from a file, replacing any data that already is in the task
object. All problem data are resorted, but if the file contains solutions, the
solution status after loading a file is still unknown, even if it was optimal
or otherwise well-defined when the file was dumped.

See section :ref:`shared-taskformat` for a description of the Task format.


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
    New maximum number of non-zeros in :math:`A`. 
``maxnumqnz``
    New maximum number of non-zeros in all :math:`Q` matrices. 


Sets the amount of preallocated space assigned for each type of data in an
optimization task.

It is never mandatory to call this function, since its only function is to give
a hint of the amount of data to preallocate for efficiency reasons.

Please note that the procedure is **destructive** in the sense that all
existing data stored in the task is destroyed.


.. index:: sensitivity_report

.. _optimizer_task_sensitivityreport:

``sensitivity_report()``
------------------------

.. code-block:: rust

    pub fn sensitivity_report ( &self,whichstream : i32 )

``whichstream``
    


Reads a sensitivity format file from a location given by
:ref:`sparam_sensitivity_file_name` and writes the result to the stream
``whichstream``. If :ref:`sparam_sensitivity_res_file_name` is set to a non-empty
string, then the sensitivity report is also written to a file of this name.


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
        String corresponding to the status key ``sk``. 

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
        String corresponding to the solution status ``solsta``.  

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
    
    If this argument is non-zero, then :eq:`ais-eq-Btxb` is solved. Otherwise the system :eq:`ais-eq-Bxb` is solved.
    
``numnz``
    
    As input it is the number of non-zeros in :math:`b`.  As output it is the number of non-zeros in :math:`\bar{X}`.
    
``sub``
    
    As input it contains the positions of the non-zeros in :math:`b`, i.e.
    
    .. math:: b[ \mathtt{sub} [k] ] \neq 0, \quad k=|idxbeg|,\ldots,|idxend:numnz[|idxbeg|]|.
    
    As output it contains the positions of the non-zeros in :math:`\bar{X}`. It is important that ``sub`` has room for ``numcon`` elements.
    
    
``val``
    
    
    As input it is the vector :math:`b`. Although the positions of the non-zero elements are specified in ``sub`` it is required that
    :math:`\mathtt{val}[i] = 0` if :math:`b[i] = 0`.  As output ``val`` is the vector :math:`\bar{X}`.
    
    Please note that ``val`` is a dense vector --- not a packed sparse vector. This implies that ``val`` has room for ``numcon`` elements.
    
    
*Returns:* ``numnz``
    ``numnz : i32``
        
        As input it is the number of non-zeros in :math:`b`.  As output it is the number of non-zeros in :math:`\bar{X}`.
        


If a basic solution is available, then exactly :math:`\mathtt{numcon}`
basis variables are defined.  These :math:`\mathtt{numcon}` basis
variables are denoted the basis.  Associated with the basis is a basis
matrix denoted :math:`B`.  This function solves either the linear
equation system

.. math:: 
   :label: ais-eq-Bxb

   B \bar{X} = b                       

or the system

.. math::
   :label: ais-eq-Btxb

   B^T \bar{X} = b

for the unknowns :math:`\bar{X}`, with :math:`b` being a user-defined  vector.

In order to make sense of the solution :math:`\bar{X}` it is important
to know the ordering of the variables in the basis because the
ordering specifies how :math:`B` is constructed. When calling
:ref:`optimizer_task_initbasissolve` an ordering of the basis variables is
obtained, which can be used to deduce how |mosek| has constructed
:math:`B`. Indeed if the :math:`k`\ th basis variable is variable
:math:`x_j` it implies that


.. math:: B_{i,k} = A_{i,j}, ~i=|idxbeg|,\ldots,|idxend:numcon|.


Otherwise if the :math:`k`\ th basis variable is variable :math:`x_j^c` it implies that

.. math::
    
  B_{i,k} = \left\{ \begin{array}{ll}
                          -1, & i = j, \\
                          0 , & i \neq j. \\
                      \end{array} 
              \right.


Given the knowledge of how :math:`B` is constructed it is possible to interpret the solution :math:`\bar{X}` correctly.

Please note that this function exploits the sparsity in the vector :math:`b` to speed up the computations.



.. index:: start_stat

.. _optimizer_task_startstat:

``start_stat()``
----------------

.. code-block:: rust

    pub fn start_stat ( &self )


Starts the statistics file.

.. index:: stop_stat

.. _optimizer_task_stopstat:

``stop_stat()``
---------------

.. code-block:: rust

    pub fn stop_stat ( &self )


Stops the statistics file.

.. index:: str_to_cone_type

.. _optimizer_task_strtoconetype:

``str_to_cone_type()``
----------------------

.. code-block:: rust

    pub fn str_to_cone_type ( &self,str   : &str ) -> i32

``str``
    String corresponding to the cone type code ``codetype``.  
*Returns:* ``conetype``
    ``conetype : i32``
        The cone type corresponding to the string ``str``.  

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



This function tries to reformulate a given Quadratically Constrained Quadratic
Optimization problem (QCQP) as a Conic Quadratic Optimization problem (CQO).
The first step of the reformulation is to convert the quadratic term of the
objective function as a constraint, if any. Then the following steps are
repeated for each quadratic constraint:

* a conic constraint is added along with a suitable number of auxiliary variables and constraints;
* the original quadratic constraint is not removed, but all its coefficients are zeroed out.


Note that the reformulation preserves all the original variables.

The conversion is performed in-place, i.e. the task passed as argument is
modified on exit. That also means that if the reformulation fails, i.e. the
given QCQP is not representable as a CQO, then the task has an undefined state.
In some cases, users may want to clone the task to ensure a clean copy is
preserved.


.. index:: update_solution_info

.. _optimizer_task_updatesolutioninfo:

``update_solution_info()``
--------------------------

.. code-block:: rust

    pub fn update_solution_info ( &self,whichsol : i32 )

``whichsol``
    

Update the information items related to the solution.

.. index:: write_branch_priorities

.. _optimizer_task_writebranchpriorities:

``write_branch_priorities()``
-----------------------------

.. code-block:: rust

    pub fn write_branch_priorities ( &self,filename : &str )

``filename``
    Data is written to the file ``filename``.  

Writes branching priority data to a file.

.. index:: write_data

.. _optimizer_task_writedata:

``write_data()``
----------------

.. code-block:: rust

    pub fn write_data ( &self,filename : &str )

``filename``
    
    Data is written to the file ``filename``
    if it is a nonempty string. Otherwise data is written
    to the file specified by :ref:`sparam_data_file_name`.
    


Writes problem data associated with the optimization task to a file in one of
four formats:

LP:
    A text based row oriented format. File extension ``.lp``. See Appendix
    :ref:`shared-lpformat`.
MPS:
    A text based column oriented format. File extension ``.mps``. See Appendix
    :ref:`shared-mpsformat`.
OPF:
    A text based row oriented format. File extension ``.opf``. Supports more
    problem types than MPS and LP. See Appendix :ref:`shared-opfformat`.
TASK:
    A MOSEK specific binary format for fast reading and writing. File extension ``.task``.

By default the data file format is determined by the file name extension. This
behaviour can be overridden by setting the :ref:`iparam_write_data_format`
parameter.

|mosek| is able to read and write files in a compressed format (gzip). To write
in the compressed format append the extension "``.gz``".  E.g to write a gzip
compressed MPS file use the extension ``mps.gz``.

Please note that MPS, LP and OPF files require all variables to have unique
names. If a task contains no names, it is possible to write the file with
automatically generated anonymous names by setting the
:ref:`iparam_write_generic_names` parameter to :ref:`fusion_on`.

Please note that if a general nonlinear function appears in the problem then
such function *cannot* be written to file and |mosek| will issue a warning.


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


Write a binary dump of the task data. This format saves all problem data, but
not callback-functions and general non-linear terms.

See section :ref:`shared-taskformat` for a description of the Task format.

