extern crate mosek;



fn main() {
    let args : Vec<String> = std::env::args().collect();
    _ = mosek::Task::new().unwrap()
        .with_stream_callback(
            mosek::Streamtype::LOG,
            &mut |msg| print!("{}",msg),
            |task| task.with_callback(
                & mut |caller| {
                    if caller == mosek::Callbackcode::NEW_INT_MIO {
                        println!("@@@@@@@@@@@@@@@@@ caller = {}",caller);
                    }
                    false 
                },
                |task| task.with_itg_sol_callback(
                    & mut |xx| {
                        println!("@@@@@@@@@@@@@@@@@ xx = {:?}",xx);
                        false 
                    },
                    |task| {
                        task.read_data(args[1].as_str())?;
                        _ = task.optimize()?;
                        task.solution_summary(mosek::Streamtype::MSG)?;
                        Ok::<(),String>(())
                    })));
}
