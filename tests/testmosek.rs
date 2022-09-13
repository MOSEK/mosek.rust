extern crate mosek;

use mosek::*;

const POW1_PTF : &str = "Task
Objective
    Maximize - @x0 + @x3 + @x4
Constraints
    @c0 [2] + @x0 + @x1 + 0.5 @x2
    @C0 [PPOW(3;0.2,0.8)]
        @ac1: + @x0
        @ac2: + @x1
        @ac3: + @x3
    @C1 [PPOW(3;4,6)]
        @ac4: + @x2
        @ac5: + 1
        @ac6: + @x4
Variables
    @x0
    @x1
    @x2
    @x3
    @x4
Solutions
    Solution interior
        SolutionStatus optimal optimal
        ProblemStatus feasible feasible
        Objective 1.8073406401566796 1.8073406505947798
        Variables
            @x0 [super_basic] 0.06393848654829891 -0 -0
            @x1 [super_basic] 0.78328037748384 -0 -0
            @x2 [super_basic] 2.305562285385002 -0 -0
            @x3 [super_basic] 0.4745546535458124 -0 -0
            @x4 [super_basic] 1.396724473159166 -0 -0
        Constraints
            @c0 [fixed] 2 -0 -0.4846636659524163
            @C0
                @ac1  [unknown] 0.06393848654829891 -1.4846636665508237
                @ac2  [unknown] 0.78328037748384 -0.48466367436403157
                @ac3  [unknown] 0.4745546535458124 0.9999999937303522
            @C1
                @ac4  [unknown] 2.305562285385002 -0.24233184039455552
                @ac5  [unknown] 1 -0.8380133186899471
                @ac6  [unknown] 1.396724473159166 0.9999999937303522
";

const CQO1_PTF : &str = "Task
Objective
    Minimize + @x3 + @x4 + @x5
Constraints
    @c0 [1] + @x0 + @x1 + @x2
    @C0 [QUAD(3)]
        @ac1: + @x3
        @ac2: + @x0
        @ac3: + @x1
    @C1 [RQUAD(3)]
        @ac4: + @x4
        @ac5: + @x5
        @ac6: + @x2
Variables
    @x0 [0;+inf]
    @x1 [0;+inf]
    @x2 [0;+inf]
    @x3
    @x4
    @x5
Solutions
    Solution interior
        SolutionStatus optimal optimal
        ProblemStatus feasible feasible
        Objective 0.7071067811865216 0.7071067811865334
        Variables
            @x0 [super_basic] 0.4999999999999294 9.753833610330672e-15 0
            @x1 [super_basic] 0.4999999999999294 9.753833610330672e-15 0
            @x2 [at_lower] 4.4839754664113277e-14 0.37686702671638306 0
            @x3 [super_basic] 0.7071067811864488 0 0
            @x4 [super_basic] 3.645487569458706e-14 0 0
            @x5 [super_basic] 3.645487569458706e-14 0 0
        Constraints
            @c0 [fixed] 1 0.7071067811865334 0
            @C0
                @ac1  [unknown] 0.7071067811864488 0.9999999999996085
                @ac2  [unknown] 0.4999999999999294 -0.7071067811862661
                @ac3  [unknown] 0.4999999999999294 -0.7071067811862661
            @C1
                @ac4  [unknown] 3.645487569458706e-14 0.9999999999999979
                @ac5  [unknown] 3.645487569458706e-14 0.9999999999999979
                @ac6  [unknown] 4.4839754664113277e-14 -1.0839738079029158
";

const SDO1_PTF : &str = "Task
Objective
    Minimize + @x0 + <M0;@X0>
Constraints
    @c0 [1] + @x0 + <M1;@X0>
    @c1 [0.5] + @x1 + @x2 + <M2;@X0>
    @C0 [QUAD(3)]
        @ac2: + @x0
        @ac3: + @x1
        @ac4: + @x2
Variables
    @x0
    @x1
    @x2
    @X0 [PSD(3)]
SymmetricMatrixes
    M0 SYMMAT(3) (0,0,2) (1,0,1) (1,1,2) (2,1,1) (2,2,2)
    M1 SYMMAT(3) (0,0,1) (1,1,1) (2,2,1)
    M2 SYMMAT(3) (0,0,1) (1,0,1) (2,0,1) (1,1,1) (2,1,1) (2,2,1)
Solutions
    Solution interior
        SolutionStatus optimal optimal
        ProblemStatus feasible feasible
        Objective 0.7057104926954896 0.7057104898500683
        Variables
            @x0 [super_basic] 0.25440485111854266 0 0
            @x1 [super_basic] 0.17989139511433158 0 0
            @x2 [super_basic] 0.17989139511433158 0 0
            @X0
                [unknown] 0.21725335998059825 1.1333372326233473
                [unknown] -0.25997116466128883 0.6780954453658373
                [unknown] 0.21725335966508608 -0.32190455420544756
                [unknown] 0.311088430149855 1.1333372325730822
                [unknown] -0.2599711646612884 0.6780954453659692
                [unknown] 0.21725335998059744 1.1333372326235056
        Constraints
            @c0 [fixed] 1 0.5447582127613013 0
            @c1 [fixed] 0.5 0.3219045541775341 0
            @C0
                @ac2  [unknown] 0.25440485111854266 0.4552417872385532
                @ac3  [unknown] 0.17989139511433158 -0.32190455417743125
                @ac4  [unknown] 0.17989139511433158 -0.32190455417743125
";

const MILO1_PTF : &str = "Task
Objective
    Maximize + @x0 + 0.64 @x1
Constraints
    @c0 [-inf;250] + 50 @x0 + 31 @x1
    @c1 [-4;+inf] + 3 @x0 - 2 @x1
Variables
    @x0 [0;+inf]
    @x1 [0;+inf]
Integers
    @x0 @x1
Solutions
    Solution integer
        SolutionStatus optimal unknown
        ProblemStatus feasible unknown
        Objective 5.000000000000001 .
        Variables
            @x0 [super_basic] 5 . .
            @x1 [super_basic] 1.0150610510858574e-15 . .
        Constraints
            @c0 [super_basic] 250.00000000000003 . .
            @c1 [super_basic] 14.999999999999998 . .
";

const SDOX_PTF : &str = "Task
Objective
    Minimize + @x0 + <M0;@X0> + <M1;@X1> + <M2;@X2>
Constraints
    @c0 [1] + @x0 + <M1;@X0>
    @c1 [0.5] + @x1 + @x2 + <M2;@X0> + <M1;@X1> + <M0;@X2>
    @C0 [QUAD(3)]
        @ac2: + @x0 + <M0;@X0>
        @ac3: + @x1 + <M1;@X1>
        @ac4: + @x2 + <M2;@X2>
Variables
    @x0
    @x1
    @x2
    @X0 [PSD(3)]
    @X1 [PSD(3)]
    @X2 [PSD(3)]
SymmetricMatrixes
    M0 SYMMAT(3) (0,0,2) (1,0,1) (1,1,2) (2,1,1) (2,2,2)
    M1 SYMMAT(3) (0,0,1) (1,1,1) (2,2,1)
    M2 SYMMAT(3) (0,0,1) (1,0,1) (2,0,1) (1,1,1) (2,1,1) (2,2,1)
";

const EMPTY_PTF : &str = "Task
Constraints
Variables
";

#[test]
fn basic_tests() {
    let mut t = Task::new().unwrap().with_callbacks();
    t.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg)).unwrap();
    t.read_ptf_string(POW1_PTF).unwrap();  let _ = t.optimize().unwrap();
    assert_eq!(2, t.get_num_acc().unwrap(),"Expected two ACCs");
    assert_eq!(5, t.get_num_var().unwrap(),"Expected empty task");
    assert_eq!(1, t.get_num_con().unwrap(),"Expected empty task");
    assert_eq!(0, t.get_num_barvar().unwrap(),"Expected empty task");
    assert_eq!(6, t.get_num_afe().unwrap(),"Expected empty task");

    t.append_barvars(&[3]).unwrap();
    t.read_ptf_string(EMPTY_PTF).unwrap();
    assert_eq!(0, t.get_num_barvar().unwrap(),"Expected empty task");
    assert_eq!(0, t.get_num_afe().unwrap(),"Expected empty task");
    assert_eq!(0, t.get_num_var().unwrap(),"Expected empty task");
    assert_eq!(0, t.get_num_con().unwrap(),"Expected empty task");
    assert_eq!(0, t.get_num_acc().unwrap(),"Expected two ACCs");

    t.read_ptf_string(CQO1_PTF).unwrap();  let _ = t.optimize().unwrap();
    t.append_cone(mosek::Conetype::QUAD,0.0,&[0,1,2]).unwrap();
    t.append_cone(mosek::Conetype::RQUAD,0.0,&[3,4,5]).unwrap();
    assert_eq!(2, t.get_num_acc().unwrap(),"Expected two accs");
    assert_eq!(2, t.get_num_cone().unwrap(),"Expected two cones");
    if let Ok(_) = t.remove_cones(&[0]) { panic!("Fail: Should no be allowed to remove variable belonging to a cone"); }
    assert_eq!(2, t.get_num_cone().unwrap(),"Expected two cones");
    t.remove_cones(&[0]).unwrap();
    t.remove_cones(&[0]).unwrap();
    t.remove_vars(&[0]).unwrap();
    t.read_ptf_string(SDO1_PTF).unwrap();  let _ = t.optimize().unwrap();
    t.remove_barvars(&[0]).unwrap();
    t.read_ptf_string(MILO1_PTF).unwrap(); let _ = t.optimize().unwrap();
    t.remove_vars(&[0]).unwrap();
    t.read_ptf_string(SDOX_PTF).unwrap();  let _ = t.optimize().unwrap();
    t.remove_barvars(&[0]).unwrap();
    assert_eq!(2, t.get_num_barvar().unwrap(),"Expected two PSD cones");
}
