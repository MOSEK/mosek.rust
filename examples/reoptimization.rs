//!   Copyright : MOSEK ApS
//!
//!   Purpose :   Demonstrates how to solve a  linear
//!               optimization problem using the MOSEK API
//!               and modify and re-optimize the problem.

/*TAG:begin-code*/
extern crate mosek;

use mosek::{Task,Boundkey,Objsense,Streamtype,Solsta,Soltype};

const INF : f64 = 0.0;

fn main() -> Result<(),String> {
    /*TAG:begin-setup-problem*/

    let numcon = 3;
    let numvar = 3;
    let c = &[1.5, 2.5, 3.0 ];
    let bkc = &[ Boundkey::UP,
                 Boundkey::UP,
                 Boundkey::UP ];
    let blc = &[ -INF,
                 -INF,
                 -INF ];
    let buc = &[ 100000.0,
                 50000.0,
                 60000.0 ];
    let bkx = &[ Boundkey::LO,
                 Boundkey::LO,
                 Boundkey::LO
                             ];
    let blx = &[ 0.0, 0.0, 0.0 ];
    let bux = &[ INF,
                 INF,
                 INF ];

    let asub = &[
        &[ 0, 1, 2 ],
        &[ 0, 1, 2 ],
        &[ 0, 1, 2 ] ];

    let aval = &[
        &[ 2.0, 3.0, 2.0 ],
        &[ 4.0, 2.0, 3.0 ],
        &[ 3.0, 3.0, 2.0 ] ];


    let task = Task::new().unwrap();
    /* Append the constraints. */
    task.appendcons(numcon);

    /* Append the variables. */
    task.appendvars(numvar);

    /* Put C. */
    for (int j = 0; j < numvar; ++j)
        task.putcj(j, c[j]);

    /* Put constraint bounds. */
    for (int i = 0; i < numcon; ++i)
        task.putconbound(i, bkc[i], blc[i], buc[i]);

    /* Put variable bounds. */
    for (int j = 0; j < numvar; ++j)
        task.putvarbound(j, bkx[j], blx[j], bux[j]);

    /* Put A. */
    if ( numcon > 0 ) {
        for (int j = 0; j < numvar; ++j)
            task.putacol(j,
                         asub[j],
                         aval[j]);
    }

    /* A maximization problem */
    task.put_obj_sense(mosek::Objsense::MAXIMIZE)?;
    /* Solve the problem */
    let termcode = task.optimize()?;

    let mut xx = vec![0.0; task.get_num_var()? as usize];
    task.get_xx(mosek::Soltype::BAS, // Request the basic solution.
                xx.as_mut_slice())?;
    /*TAG:end-setup-problem*/

    for (j,cj) in xx.iter().enumerate() {
        println!("x[{}]: {}",j,xj);
    }

    /****************** Make a change to the A matrix ******************/
    /*TAG:begin-putaij*/
    task.put_aij(0, 0, 3.0)?;
    /*TAG:end-putaij*/
    /*TAG:begin-reoptimize1*/

    let termcode = task.optimize()?;
    task.get_xx(mosek::Soltype::BAS, // Request the basic solution.
                xx.as_mut_slice());

    for (j,cj) in xx.iter().enumerate() {
        println!("x[{}]: {}",j,xj);
    }
    /*TAG:end-reoptimize1*/

    /*TAG:begin-addcol*/
    /***************** Add a new variable ******************************/
    /* Get index of new variable. */

    let varidx = new int[1];
    task.getnumvar(varidx);

    /* Append a new variable x_3 to the problem */
    task.appendvars(1);
    numvar++;

    /* Set bounds on new varaible */
    task.putvarbound(varidx[0],
                     mosek.boundkey.lo,
                     0,
                     +infinity);

    /* Change objective */
    task.putcj(varidx[0], 1.0);

    /* Put new values in the A matrix */
    int[] acolsub    =  new int[] {0,   2};
    double[] acolval =  new double[] {4.0, 1.0};

    task.putacol(varidx[0], /* column index */
                 acolsub,
                 acolval);
    /*TAG:end-addcol*/

    /*TAG:begin-reoptimize2*/
    /* Change optimizer to simplex free and reoptimize */
    task.putintparam(mosek.iparam.optimizer, mosek.optimizertype.free_simplex.value);
    termcode = task.optimize();

    xx = new double[numvar];
    task.getxx(mosek.soltype.bas, // Request the basic solution.
               xx);

    for (int j = 0; j < numvar; ++j)
        System.out.println ("x[" + j + "]:" + xx[j]);
    /*TAG:end-reoptimize2*/

    /*TAG:begin-addcon*/
    /********************** Add a new constraint ***************************/
    /* Get index of new constraint. */
    int[] conidx = new int[1];
    task.getnumcon(conidx);

    /* Append a new constraint */
    task.appendcons(1);
    numcon++;

    /* Set bounds on new constraint */
    task.putconbound(conidx[0],
                     mosek.boundkey.up,
                     -infinity,
                     30000);

    /* Put new values in the A matrix */
    int[] arowsub = new int[] {0,   1,   2,   3  };
    double[] arowval = new double[]  {1.0, 2.0, 1.0, 1.0};

    task.putarow(conidx[0], /* row index */
                 arowsub,
                 arowval);

    /*TAG:end-addcon*/
    /*TAG:begin-reoptimize3*/
    termcode = task.optimize();

    task.getxx(mosek.soltype.bas, // Request the basic solution.
               xx);

    for (int j = 0; j < numvar; ++j)
        System.out.println ("x[" + j + "]:" + xx[j]);
    /*TAG:end-reoptimize3*/


    /*TAG:begin-changebounds*/
    /********************** Change constraint bounds ********************/
    mosek.boundkey[] newbkc  = {mosek.boundkey.up,
                                mosek.boundkey.up,
                                mosek.boundkey.up,
                                mosek.boundkey.up
    };
    double[] newblc          = { -infinity,
                                  -infinity,
                                  -infinity,
                                  -infinity
    };
    double[] newbuc          = { 80000, 40000, 50000, 22000 };

    task.putconboundslice(0, numcon, newbkc, newblc, newbuc);
    /*TAG:end-changebounds*/

    task.optimize();

    task.getxx(mosek.soltype.bas, // Request the basic solution.
               xx);

    for (int j = 0; j < numvar; ++j)
        System.out.println ("x[" + j + "]:" + xx[j]);
    
} catch (mosek.Exception e)
/* Catch both Error and Warning */
{
    System.out.println ("An error was encountered");
    System.out.println (e.getMessage ());
    throw e;
}
  }
}
/*TAG:end-code*/
