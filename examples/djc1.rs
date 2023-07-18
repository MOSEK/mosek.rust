//
//   Copyright: MOSEK ApS
//
//   File:      djc1.rs
//
//   Purpose: Demonstrates how to solve the problem with two disjunctions:
//
//      minimize    2x0 + x1 + 3x2 + x3
//      subject to   x0 + x1 + x2 + x3 >= -10
//                  (x0-2x1<=-1 and x2=x3=0) or (x2-3x3<=-2 and x1=x2=0)
//                  x0=2.5 or x1=2.5 or x2=2.5 or x3=2.5
//

extern crate mosek;

use mosek::{Task,Boundkey,Objsense,Streamtype,Solsta,Soltype};

// Since the value of infinity is ignored, we define it solely
// for symbolic purposes
const INF : f64 = 0.0;


fn main() -> Result<(),String> {
    // Create a task object
    let mut task = match Task::new() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
        }.with_callbacks();

    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

    // Append free variables
    let numvar : i32 = 4;
    task.append_vars(numvar)?;
    let x : Vec<i32> = (0..numvar).collect();
    task.put_var_bound_slice_const(0, numvar, Boundkey::FR, -INF, INF)?;

    // The linear part: the linear constraint
    task.append_cons(1)?;
    task.put_a_row(0, x.as_slice(), vec![1.0; numvar as usize].as_slice())?;
    task.put_con_bound(0, Boundkey::LO, -10.0, -10.0)?;

    // The linear part: objective
    task.put_obj_sense(Objsense::MINIMIZE)?;
    task.put_c_list(x.as_slice(), &[2.0, 1.0, 3.0, 1.0])?;

    // Fill in the affine expression storage F, g
    let numafe : i64 = 10;
    task.append_afes(numafe)?;

    let fafeidx : &[i64] = &[0, 0, 1, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let fvaridx : &[i32] = &[0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3];
    let fval             = &[1.0, -2.0, 1.0, -3.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    let g                = &[1.0, 2.0, 0.0, 0.0, 0.0, 0.0, -2.5, -2.5, -2.5, -2.5];

    task.put_afe_f_entry_list(fafeidx, fvaridx, fval)?;
    task.put_afe_g_slice(0, numafe, g)?;

    // Create domains
    let zero1   = task.append_rzero_domain(1)?;
    let zero2   = task.append_rzero_domain(2)?;
    let rminus1 = task.append_rminus_domain(1)?;

    // Append disjunctive constraints
    let numdjc : i64 = 2;
    task.append_djcs(numdjc)?;

    // First disjunctive constraint
    task.put_djc(0,                                        // DJC index
                 &[rminus1, zero2, rminus1, zero2],        // Domains     (domidxlist)
                 &[0, 4, 5, 1, 2, 3],                      // AFE indices (afeidxlist)
                 &[0.0,0.0,0.0,0.0,0.0,0.0],               // Unused
                 &[2, 2])?;                                // Term sizes  (termsizelist)

    // Second disjunctive constraint
    task.put_djc(1,                                        // DJC index
                 &[zero1, zero1, zero1, zero1],            // Domains     (domidxlist)
                 &[6, 7, 8, 9],                            // AFE indices (afeidxlist)
                 &[0.0,0.0,0.0,0.0],                       // Unused
                 &[1, 1, 1, 1])?;                          // Term sizes  (termidxlist)

    // Useful for debugging
    task.write_data("djc1.ptf")?;                         // Write file in human-readable format

    // Solve the problem
    let _ = task.optimize()?;

    // Print a summary containing information
    // about the solution for debugging purposes
    task.solution_summary(Streamtype::MSG)?;

    // Get status information about the solution
    let sta = task.get_sol_sta(Soltype::ITG)?;

    let mut xx = vec![0.0; numvar as usize];
    task.get_xx(Soltype::ITG,xx.as_mut_slice())?;


    println!("Optimal solution: ");
    for (i,&xi) in xx.iter().enumerate() {
        println!("x[{}]={}",i,xi);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::main().unwrap();
    }
}
