
//!   Copyright : MOSEK ApS
//!
//!   Purpose :   To demonstrate how to use the MSK_relaxprimal function to
//!               locate the cause of an infeasibility.
//!
//!   Syntax :     On command line
//!
//!                   feasrepairex1 feasrepair.lp
//!
//!                feasrepair.lp is located in mosek/<version>/tools/examples/data
//!*/

//TAG:begin-code
extern crate mosek;

use std::env;
use mosek::{Task,Streamtype};

/// Small infeasible linear problem
const FEASREPAIR_LP : &str = "
minimize
 obj: - 10 x1 - 9 x2
st
 c1: + 7e-01 x1 + x2 <= 630
 c2: + 5e-01 x1 + 8.333333333e-01 x2 <= 600
 c3: + x1 + 6.6666667e-01 x2 <= 708
 c4: + 1e-01 x1 + 2.5e-01 x2 <= 135
bounds
x2 >= 650
end
";

fn main() -> Result<(),String> {
    let args: Vec<String> = env::args().collect();

    let mut task = Task::new().unwrap().with_callbacks();
    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

    if args.len() > 1 {
        task.read_data(args[1].as_str())?;
    }
    else {
        task.read_lp_string(FEASREPAIR_LP)?;
    }
    task.put_int_param(mosek::Iparam::LOG_FEAS_REPAIR, 3)?;

    let wc = vec![0.0; task.get_num_con()? as usize];
    let wx = vec![0.0; task.get_num_var()? as usize];
    task.primal_repair(wc.as_slice(),wc.as_slice(),wx.as_slice(),wx.as_slice())?;

    let sum_viol = task.get_dou_inf(mosek::Dinfitem::PRIMAL_REPAIR_PENALTY_OBJ)?;

    println!("Minimized sum of violations = {}", sum_viol);

    let _ = task.optimize()?;

    task.solution_summary(mosek::Streamtype::MSG)?;

    Ok(())
}

/*TAG:end-code*/
