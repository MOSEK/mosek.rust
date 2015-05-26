extern crate libc;
use std::ffi::CString;

#[link(name = "mosek64")]
extern
{
    fn MSK_makeenv(env : * mut * const u8, dbgfile : * const libc::c_char) -> i32;
    fn MSK_deleteenv(env : * mut * const u8) -> i32;
    fn MSK_maketask(env : * const u8, maxnumcon : i32, maxnumvar : i32, task : * mut * const u8) -> u32;
    fn MSK_deletetask(task : * mut * const u8) -> i32;

    fn MSK_analyzenames(task_ : * const u8,whichstream_ : i32,nametype_ : i32) -> i32;
    fn MSK_analyzeproblem(task_ : * const u8,whichstream_ : i32) -> i32;
    fn MSK_analyzesolution(task_ : * const u8,whichstream_ : i32,whichsol_ : i32) -> i32;
    fn MSK_appendbarvars(task_ : * const u8,num_ : libc::int32_t,dim_ : * const libc::int32_t) -> i32;
    fn MSK_appendcone(task_ : * const u8,ct_ : i32,conepar_ : f64,nummem_ : libc::int32_t,submem_ : * const libc::int32_t) -> i32;
    fn MSK_appendconeseq(task_ : * const u8,ct_ : i32,conepar_ : f64,nummem_ : libc::int32_t,j_ : libc::int32_t) -> i32;
    fn MSK_appendconesseq(task_ : * const u8,num_ : libc::int32_t,ct_ : * const i32,conepar_ : * const f64,nummem_ : * const libc::int32_t,j_ : libc::int32_t) -> i32;
    fn MSK_appendcons(task_ : * const u8,num_ : libc::int32_t) -> i32;
    fn MSK_appendsparsesymmat(task_ : * const u8,dim_ : libc::int32_t,nz_ : libc::int64_t,subi_ : * const libc::int32_t,subj_ : * const libc::int32_t,valij_ : * const f64,idx_ : & mut libc::int64_t) -> i32;
    fn MSK_appendstat(task_ : * const u8) -> i32;
    fn MSK_appendvars(task_ : * const u8,num_ : libc::int32_t) -> i32;
    fn MSK_basiscond(task_ : * const u8,nrmbasis_ : & mut f64,nrminvbasis_ : & mut f64) -> i32;
    fn MSK_checkconvexity(task_ : * const u8) -> i32;
    fn MSK_checkmemtask(task_ : * const u8,file_ : * const libc::c_char,line_ : libc::int32_t) -> i32;
    fn MSK_chgbound(task_ : * const u8,accmode_ : i32,i_ : libc::int32_t,lower_ : libc::int32_t,finite_ : libc::int32_t,value_ : f64) -> i32;
    fn MSK_chgconbound(task_ : * const u8,i_ : libc::int32_t,lower_ : libc::int32_t,finite_ : libc::int32_t,value_ : f64) -> i32;
    fn MSK_chgvarbound(task_ : * const u8,j_ : libc::int32_t,lower_ : libc::int32_t,finite_ : libc::int32_t,value_ : f64) -> i32;
    fn MSK_commitchanges(task_ : * const u8) -> i32;
    fn MSK_deletesolution(task_ : * const u8,whichsol_ : i32) -> i32;
    fn MSK_dualsensitivity(task_ : * const u8,numj_ : libc::int32_t,subj_ : * const libc::int32_t,leftpricej_ : * mut f64,rightpricej_ : * mut f64,leftrangej_ : * mut f64,rightrangej_ : * mut f64) -> i32;
    fn MSK_getacol(task_ : * const u8,j_ : libc::int32_t,nzj_ : & mut libc::int32_t,subj_ : * mut libc::int32_t,valj_ : * mut f64) -> i32;
    fn MSK_getacolnumnz(task_ : * const u8,i_ : libc::int32_t,nzj_ : & mut libc::int32_t) -> i32;
    fn MSK_getaij(task_ : * const u8,i_ : libc::int32_t,j_ : libc::int32_t,aij_ : & mut f64) -> i32;
    fn MSK_getapiecenumnz(task_ : * const u8,firsti_ : libc::int32_t,lasti_ : libc::int32_t,firstj_ : libc::int32_t,lastj_ : libc::int32_t,numnz_ : & mut libc::int32_t) -> i32;
    fn MSK_getarow(task_ : * const u8,i_ : libc::int32_t,nzi_ : & mut libc::int32_t,subi_ : * mut libc::int32_t,vali_ : * mut f64) -> i32;
    fn MSK_getarownumnz(task_ : * const u8,i_ : libc::int32_t,nzi_ : & mut libc::int32_t) -> i32;
    fn MSK_getaslicenumnz64(task_ : * const u8,accmode_ : i32,first_ : libc::int32_t,last_ : libc::int32_t,numnz_ : & mut libc::int64_t) -> i32;
    fn MSK_getbarablocktriplet(task_ : * const u8,maxnum_ : libc::int64_t,num_ : & mut libc::int64_t,subi_ : * mut libc::int32_t,subj_ : * mut libc::int32_t,subk_ : * mut libc::int32_t,subl_ : * mut libc::int32_t,valijkl_ : * mut f64) -> i32;
    fn MSK_getbaraidx(task_ : * const u8,idx_ : libc::int64_t,maxnum_ : libc::int64_t,i_ : & mut libc::int32_t,j_ : & mut libc::int32_t,num_ : & mut libc::int64_t,sub_ : * mut libc::int64_t,weights_ : * mut f64) -> i32;
    fn MSK_getbaraidxij(task_ : * const u8,idx_ : libc::int64_t,i_ : & mut libc::int32_t,j_ : & mut libc::int32_t) -> i32;
    fn MSK_getbaraidxinfo(task_ : * const u8,idx_ : libc::int64_t,num_ : & mut libc::int64_t) -> i32;
    fn MSK_getbarasparsity(task_ : * const u8,maxnumnz_ : libc::int64_t,numnz_ : & mut libc::int64_t,idxij_ : * mut libc::int64_t) -> i32;
    fn MSK_getbarcblocktriplet(task_ : * const u8,maxnum_ : libc::int64_t,num_ : & mut libc::int64_t,subj_ : * mut libc::int32_t,subk_ : * mut libc::int32_t,subl_ : * mut libc::int32_t,valijkl_ : * mut f64) -> i32;
    fn MSK_getbarcidx(task_ : * const u8,idx_ : libc::int64_t,maxnum_ : libc::int64_t,j_ : & mut libc::int32_t,num_ : & mut libc::int64_t,sub_ : * mut libc::int64_t,weights_ : * mut f64) -> i32;
    fn MSK_getbarcidxinfo(task_ : * const u8,idx_ : libc::int64_t,num_ : & mut libc::int64_t) -> i32;
    fn MSK_getbarcidxj(task_ : * const u8,idx_ : libc::int64_t,j_ : & mut libc::int32_t) -> i32;
    fn MSK_getbarcsparsity(task_ : * const u8,maxnumnz_ : libc::int64_t,numnz_ : & mut libc::int64_t,idxj_ : * mut libc::int64_t) -> i32;
    fn MSK_getbarsj(task_ : * const u8,whichsol_ : i32,j_ : libc::int32_t,barsj_ : * mut f64) -> i32;
    fn MSK_getbarvarname(task_ : * const u8,i_ : libc::int32_t,maxlen_ : libc::int32_t,name_ : * const u8) -> i32;
    fn MSK_getbarvarnameindex(task_ : * const u8,somename_ : * const libc::c_char,asgn_ : & mut libc::int32_t,index_ : & mut libc::int32_t) -> i32;
    fn MSK_getbarvarnamelen(task_ : * const u8,i_ : libc::int32_t,len_ : & mut libc::int32_t) -> i32;
    fn MSK_getbarxj(task_ : * const u8,whichsol_ : i32,j_ : libc::int32_t,barxj_ : * mut f64) -> i32;
    fn MSK_getbound(task_ : * const u8,accmode_ : i32,i_ : libc::int32_t,bk_ : & mut i32,bl_ : & mut f64,bu_ : & mut f64) -> i32;
    fn MSK_getboundslice(task_ : * const u8,accmode_ : i32,first_ : libc::int32_t,last_ : libc::int32_t,bk_ : * mut i32,bl_ : * mut f64,bu_ : * mut f64) -> i32;
    fn MSK_getc(task_ : * const u8,c_ : * mut f64) -> i32;
    fn MSK_getcfix(task_ : * const u8,cfix_ : & mut f64) -> i32;
    fn MSK_getcj(task_ : * const u8,j_ : libc::int32_t,cj_ : & mut f64) -> i32;
    fn MSK_getconbound(task_ : * const u8,i_ : libc::int32_t,bk_ : & mut i32,bl_ : & mut f64,bu_ : & mut f64) -> i32;
    fn MSK_getconboundslice(task_ : * const u8,first_ : libc::int32_t,last_ : libc::int32_t,bk_ : * mut i32,bl_ : * mut f64,bu_ : * mut f64) -> i32;
    fn MSK_getcone(task_ : * const u8,k_ : libc::int32_t,ct_ : & mut i32,conepar_ : & mut f64,nummem_ : & mut libc::int32_t,submem_ : * mut libc::int32_t) -> i32;
    fn MSK_getconeinfo(task_ : * const u8,k_ : libc::int32_t,ct_ : & mut i32,conepar_ : & mut f64,nummem_ : & mut libc::int32_t) -> i32;
    fn MSK_getconename(task_ : * const u8,i_ : libc::int32_t,maxlen_ : libc::int32_t,name_ : * const u8) -> i32;
    fn MSK_getconenameindex(task_ : * const u8,somename_ : * const libc::c_char,asgn_ : & mut libc::int32_t,index_ : & mut libc::int32_t) -> i32;
    fn MSK_getconenamelen(task_ : * const u8,i_ : libc::int32_t,len_ : & mut libc::int32_t) -> i32;
    fn MSK_getconname(task_ : * const u8,i_ : libc::int32_t,maxlen_ : libc::int32_t,name_ : * const u8) -> i32;
    fn MSK_getconnameindex(task_ : * const u8,somename_ : * const libc::c_char,asgn_ : & mut libc::int32_t,index_ : & mut libc::int32_t) -> i32;
    fn MSK_getconnamelen(task_ : * const u8,i_ : libc::int32_t,len_ : & mut libc::int32_t) -> i32;
    fn MSK_getcslice(task_ : * const u8,first_ : libc::int32_t,last_ : libc::int32_t,c_ : * mut f64) -> i32;
    fn MSK_getdimbarvarj(task_ : * const u8,j_ : libc::int32_t,dimbarvarj_ : & mut libc::int32_t) -> i32;
    fn MSK_getdouinf(task_ : * const u8,whichdinf_ : i32,dvalue_ : & mut f64) -> i32;
    fn MSK_getdouparam(task_ : * const u8,param_ : i32,parvalue_ : & mut f64) -> i32;
    fn MSK_getdualobj(task_ : * const u8,whichsol_ : i32,dualobj_ : & mut f64) -> i32;
    fn MSK_getdviolbarvar(task_ : * const u8,whichsol_ : i32,num_ : libc::int32_t,sub_ : * const libc::int32_t,viol_ : * mut f64) -> i32;
    fn MSK_getdviolcon(task_ : * const u8,whichsol_ : i32,num_ : libc::int32_t,sub_ : * const libc::int32_t,viol_ : * mut f64) -> i32;
    fn MSK_getdviolcones(task_ : * const u8,whichsol_ : i32,num_ : libc::int32_t,sub_ : * const libc::int32_t,viol_ : * mut f64) -> i32;
    fn MSK_getdviolvar(task_ : * const u8,whichsol_ : i32,num_ : libc::int32_t,sub_ : * const libc::int32_t,viol_ : * mut f64) -> i32;
    fn MSK_getinfindex(task_ : * const u8,inftype_ : i32,infname_ : * const libc::c_char,infindex_ : & mut libc::int32_t) -> i32;
    fn MSK_getinfmax(task_ : * const u8,inftype_ : i32,infmax_ : * mut libc::int32_t) -> i32;
    fn MSK_getinfname(task_ : * const u8,inftype_ : i32,whichinf_ : libc::int32_t,infname_ : * const u8) -> i32;
    fn MSK_getintinf(task_ : * const u8,whichiinf_ : i32,ivalue_ : & mut libc::int32_t) -> i32;
    fn MSK_getintparam(task_ : * const u8,param_ : i32,parvalue_ : & mut libc::int32_t) -> i32;
    fn MSK_getlenbarvarj(task_ : * const u8,j_ : libc::int32_t,lenbarvarj_ : & mut libc::int64_t) -> i32;
    fn MSK_getlintinf(task_ : * const u8,whichliinf_ : i32,ivalue_ : & mut libc::int64_t) -> i32;
    fn MSK_getmaxnumanz64(task_ : * const u8,maxnumanz_ : & mut libc::int64_t) -> i32;
    fn MSK_getmaxnumbarvar(task_ : * const u8,maxnumbarvar_ : & mut libc::int32_t) -> i32;
    fn MSK_getmaxnumcon(task_ : * const u8,maxnumcon_ : & mut libc::int32_t) -> i32;
    fn MSK_getmaxnumcone(task_ : * const u8,maxnumcone_ : & mut libc::int32_t) -> i32;
    fn MSK_getmaxnumqnz64(task_ : * const u8,maxnumqnz_ : & mut libc::int64_t) -> i32;
    fn MSK_getmaxnumvar(task_ : * const u8,maxnumvar_ : & mut libc::int32_t) -> i32;
    fn MSK_getmemusagetask(task_ : * const u8,meminuse_ : & mut libc::int64_t,maxmemuse_ : & mut libc::int64_t) -> i32;
    fn MSK_getnumanz(task_ : * const u8,numanz_ : & mut libc::int32_t) -> i32;
    fn MSK_getnumanz64(task_ : * const u8,numanz_ : & mut libc::int64_t) -> i32;
    fn MSK_getnumbarablocktriplets(task_ : * const u8,num_ : & mut libc::int64_t) -> i32;
    fn MSK_getnumbaranz(task_ : * const u8,nz_ : & mut libc::int64_t) -> i32;
    fn MSK_getnumbarcblocktriplets(task_ : * const u8,num_ : & mut libc::int64_t) -> i32;
    fn MSK_getnumbarcnz(task_ : * const u8,nz_ : & mut libc::int64_t) -> i32;
    fn MSK_getnumbarvar(task_ : * const u8,numbarvar_ : & mut libc::int32_t) -> i32;
    fn MSK_getnumcon(task_ : * const u8,numcon_ : & mut libc::int32_t) -> i32;
    fn MSK_getnumcone(task_ : * const u8,numcone_ : & mut libc::int32_t) -> i32;
    fn MSK_getnumconemem(task_ : * const u8,k_ : libc::int32_t,nummem_ : & mut libc::int32_t) -> i32;
    fn MSK_getnumintvar(task_ : * const u8,numintvar_ : & mut libc::int32_t) -> i32;
    fn MSK_getnumparam(task_ : * const u8,partype_ : i32,numparam_ : & mut libc::int32_t) -> i32;
    fn MSK_getnumqconknz64(task_ : * const u8,k_ : libc::int32_t,numqcnz_ : & mut libc::int64_t) -> i32;
    fn MSK_getnumqobjnz64(task_ : * const u8,numqonz_ : & mut libc::int64_t) -> i32;
    fn MSK_getnumsymmat(task_ : * const u8,num_ : & mut libc::int64_t) -> i32;
    fn MSK_getnumvar(task_ : * const u8,numvar_ : & mut libc::int32_t) -> i32;
    fn MSK_getobjname(task_ : * const u8,maxlen_ : libc::int32_t,objname_ : * const u8) -> i32;
    fn MSK_getobjnamelen(task_ : * const u8,len_ : & mut libc::int32_t) -> i32;
    fn MSK_getobjsense(task_ : * const u8,sense_ : & mut i32) -> i32;
    fn MSK_getparammax(task_ : * const u8,partype_ : i32,parammax_ : & mut libc::int32_t) -> i32;
    fn MSK_getparamname(task_ : * const u8,partype_ : i32,param_ : libc::int32_t,parname_ : * const u8) -> i32;
    fn MSK_getprimalobj(task_ : * const u8,whichsol_ : i32,primalobj_ : & mut f64) -> i32;
    fn MSK_getprobtype(task_ : * const u8,probtype_ : & mut i32) -> i32;
    fn MSK_getprosta(task_ : * const u8,whichsol_ : i32,prosta_ : & mut i32) -> i32;
    fn MSK_getpviolbarvar(task_ : * const u8,whichsol_ : i32,num_ : libc::int32_t,sub_ : * const libc::int32_t,viol_ : * mut f64) -> i32;
    fn MSK_getpviolcon(task_ : * const u8,whichsol_ : i32,num_ : libc::int32_t,sub_ : * const libc::int32_t,viol_ : * mut f64) -> i32;
    fn MSK_getpviolcones(task_ : * const u8,whichsol_ : i32,num_ : libc::int32_t,sub_ : * const libc::int32_t,viol_ : * mut f64) -> i32;
    fn MSK_getpviolvar(task_ : * const u8,whichsol_ : i32,num_ : libc::int32_t,sub_ : * const libc::int32_t,viol_ : * mut f64) -> i32;
    fn MSK_getqobjij(task_ : * const u8,i_ : libc::int32_t,j_ : libc::int32_t,qoij_ : & mut f64) -> i32;
    fn MSK_getreducedcosts(task_ : * const u8,whichsol_ : i32,first_ : libc::int32_t,last_ : libc::int32_t,redcosts_ : * mut f64) -> i32;
    fn MSK_getskc(task_ : * const u8,whichsol_ : i32,skc_ : * mut i32) -> i32;
    fn MSK_getskcslice(task_ : * const u8,whichsol_ : i32,first_ : libc::int32_t,last_ : libc::int32_t,skc_ : * mut i32) -> i32;
    fn MSK_getskx(task_ : * const u8,whichsol_ : i32,skx_ : * mut i32) -> i32;
    fn MSK_getskxslice(task_ : * const u8,whichsol_ : i32,first_ : libc::int32_t,last_ : libc::int32_t,skx_ : * mut i32) -> i32;
    fn MSK_getslc(task_ : * const u8,whichsol_ : i32,slc_ : * mut f64) -> i32;
    fn MSK_getslcslice(task_ : * const u8,whichsol_ : i32,first_ : libc::int32_t,last_ : libc::int32_t,slc_ : * mut f64) -> i32;
    fn MSK_getslx(task_ : * const u8,whichsol_ : i32,slx_ : * mut f64) -> i32;
    fn MSK_getslxslice(task_ : * const u8,whichsol_ : i32,first_ : libc::int32_t,last_ : libc::int32_t,slx_ : * mut f64) -> i32;
    fn MSK_getsnx(task_ : * const u8,whichsol_ : i32,snx_ : * mut f64) -> i32;
    fn MSK_getsnxslice(task_ : * const u8,whichsol_ : i32,first_ : libc::int32_t,last_ : libc::int32_t,snx_ : * mut f64) -> i32;
    fn MSK_getsolsta(task_ : * const u8,whichsol_ : i32,solsta_ : & mut i32) -> i32;
    fn MSK_getsolution(task_ : * const u8,whichsol_ : i32,prosta_ : & mut i32,solsta_ : & mut i32,skc_ : * mut i32,skx_ : * mut i32,skn_ : * mut i32,xc_ : * mut f64,xx_ : * mut f64,y_ : * mut f64,slc_ : * mut f64,suc_ : * mut f64,slx_ : * mut f64,sux_ : * mut f64,snx_ : * mut f64) -> i32;
    fn MSK_getsolutioni(task_ : * const u8,accmode_ : i32,i_ : libc::int32_t,whichsol_ : i32,sk_ : & mut i32,x_ : & mut f64,sl_ : & mut f64,su_ : & mut f64,sn_ : & mut f64) -> i32;
    fn MSK_getsolutioninfo(task_ : * const u8,whichsol_ : i32,pobj_ : & mut f64,pviolcon_ : & mut f64,pviolvar_ : & mut f64,pviolbarvar_ : & mut f64,pviolcone_ : & mut f64,pviolitg_ : & mut f64,dobj_ : & mut f64,dviolcon_ : & mut f64,dviolvar_ : & mut f64,dviolbarvar_ : & mut f64,dviolcone_ : & mut f64) -> i32;
    fn MSK_getsolutionslice(task_ : * const u8,whichsol_ : i32,solitem_ : i32,first_ : libc::int32_t,last_ : libc::int32_t,values_ : * mut f64) -> i32;
    fn MSK_getsparsesymmat(task_ : * const u8,idx_ : libc::int64_t,maxlen_ : libc::int64_t,subi_ : * mut libc::int32_t,subj_ : * mut libc::int32_t,valij_ : * mut f64) -> i32;
    fn MSK_getstrparam(task_ : * const u8,param_ : i32,maxlen_ : libc::int32_t,len_ : & mut libc::int32_t,parvalue_ : * const u8) -> i32;
    fn MSK_getstrparamlen(task_ : * const u8,param_ : i32,len_ : & mut libc::int32_t) -> i32;
    fn MSK_getsuc(task_ : * const u8,whichsol_ : i32,suc_ : * mut f64) -> i32;
    fn MSK_getsucslice(task_ : * const u8,whichsol_ : i32,first_ : libc::int32_t,last_ : libc::int32_t,suc_ : * mut f64) -> i32;
    fn MSK_getsux(task_ : * const u8,whichsol_ : i32,sux_ : * mut f64) -> i32;
    fn MSK_getsuxslice(task_ : * const u8,whichsol_ : i32,first_ : libc::int32_t,last_ : libc::int32_t,sux_ : * mut f64) -> i32;
    fn MSK_getsymmatinfo(task_ : * const u8,idx_ : libc::int64_t,dim_ : & mut libc::int32_t,nz_ : & mut libc::int64_t,type_ : & mut i32) -> i32;
    fn MSK_gettaskname(task_ : * const u8,maxlen_ : libc::int32_t,taskname_ : * const u8) -> i32;
    fn MSK_gettasknamelen(task_ : * const u8,len_ : & mut libc::int32_t) -> i32;
    fn MSK_getvarbound(task_ : * const u8,i_ : libc::int32_t,bk_ : & mut i32,bl_ : & mut f64,bu_ : & mut f64) -> i32;
    fn MSK_getvarboundslice(task_ : * const u8,first_ : libc::int32_t,last_ : libc::int32_t,bk_ : * mut i32,bl_ : * mut f64,bu_ : * mut f64) -> i32;
    fn MSK_getvarbranchdir(task_ : * const u8,j_ : libc::int32_t,direction_ : & mut i32) -> i32;
    fn MSK_getvarbranchorder(task_ : * const u8,j_ : libc::int32_t,priority_ : & mut libc::int32_t,direction_ : & mut i32) -> i32;
    fn MSK_getvarbranchpri(task_ : * const u8,j_ : libc::int32_t,priority_ : & mut libc::int32_t) -> i32;
    fn MSK_getvarname(task_ : * const u8,j_ : libc::int32_t,maxlen_ : libc::int32_t,name_ : * const u8) -> i32;
    fn MSK_getvarnameindex(task_ : * const u8,somename_ : * const libc::c_char,asgn_ : & mut libc::int32_t,index_ : & mut libc::int32_t) -> i32;
    fn MSK_getvarnamelen(task_ : * const u8,i_ : libc::int32_t,len_ : & mut libc::int32_t) -> i32;
    fn MSK_getvartype(task_ : * const u8,j_ : libc::int32_t,vartype_ : & mut i32) -> i32;
    fn MSK_getvartypelist(task_ : * const u8,num_ : libc::int32_t,subj_ : * const libc::int32_t,vartype_ : * mut i32) -> i32;
    fn MSK_getxc(task_ : * const u8,whichsol_ : i32,xc_ : * mut f64) -> i32;
    fn MSK_getxcslice(task_ : * const u8,whichsol_ : i32,first_ : libc::int32_t,last_ : libc::int32_t,xc_ : * mut f64) -> i32;
    fn MSK_getxx(task_ : * const u8,whichsol_ : i32,xx_ : * mut f64) -> i32;
    fn MSK_getxxslice(task_ : * const u8,whichsol_ : i32,first_ : libc::int32_t,last_ : libc::int32_t,xx_ : * mut f64) -> i32;
    fn MSK_gety(task_ : * const u8,whichsol_ : i32,y_ : * mut f64) -> i32;
    fn MSK_getyslice(task_ : * const u8,whichsol_ : i32,first_ : libc::int32_t,last_ : libc::int32_t,y_ : * mut f64) -> i32;
    fn MSK_initbasissolve(task_ : * const u8,basis_ : * mut libc::int32_t) -> i32;
    fn MSK_inputdata64(task_ : * const u8,maxnumcon_ : libc::int32_t,maxnumvar_ : libc::int32_t,numcon_ : libc::int32_t,numvar_ : libc::int32_t,c_ : * const f64,cfix_ : f64,aptrb_ : * const libc::int64_t,aptre_ : * const libc::int64_t,asub_ : * const libc::int32_t,aval_ : * const f64,bkc_ : * const i32,blc_ : * const f64,buc_ : * const f64,bkx_ : * const i32,blx_ : * const f64,bux_ : * const f64) -> i32;
    fn MSK_isdouparname(task_ : * const u8,parname_ : * const libc::c_char,param_ : & mut i32) -> i32;
    fn MSK_isintparname(task_ : * const u8,parname_ : * const libc::c_char,param_ : & mut i32) -> i32;
    fn MSK_isstrparname(task_ : * const u8,parname_ : * const libc::c_char,param_ : & mut i32) -> i32;
    fn MSK_linkfiletotaskstream(task_ : * const u8,whichstream_ : i32,filename_ : * const libc::c_char,append_ : libc::int32_t) -> i32;
    fn MSK_onesolutionsummary(task_ : * const u8,whichstream_ : i32,whichsol_ : i32) -> i32;
    fn MSK_optimizersummary(task_ : * const u8,whichstream_ : i32) -> i32;
    fn MSK_optimizetrm(task_ : * const u8,trmcode_ : & mut i32) -> i32;
    fn MSK_primalrepair(task_ : * const u8,wlc_ : * const f64,wuc_ : * const f64,wlx_ : * const f64,wux_ : * const f64) -> i32;
    fn MSK_primalsensitivity(task_ : * const u8,numi_ : libc::int32_t,subi_ : * const libc::int32_t,marki_ : * const i32,numj_ : libc::int32_t,subj_ : * const libc::int32_t,markj_ : * const i32,leftpricei_ : * mut f64,rightpricei_ : * mut f64,leftrangei_ : * mut f64,rightrangei_ : * mut f64,leftpricej_ : * mut f64,rightpricej_ : * mut f64,leftrangej_ : * mut f64,rightrangej_ : * mut f64) -> i32;
    fn MSK_probtypetostr(task_ : * const u8,probtype_ : i32,str_ : * const u8) -> i32;
    fn MSK_prostatostr(task_ : * const u8,prosta_ : i32,str_ : * const u8) -> i32;
    fn MSK_putacol(task_ : * const u8,j_ : libc::int32_t,nzj_ : libc::int32_t,subj_ : * const libc::int32_t,valj_ : * const f64) -> i32;
    fn MSK_putacollist(task_ : * const u8,num_ : libc::int32_t,sub_ : * const libc::int32_t,ptrb_ : * const libc::int32_t,ptre_ : * const libc::int32_t,asub_ : * const libc::int32_t,aval_ : * const f64) -> i32;
    fn MSK_putacolslice64(task_ : * const u8,first_ : libc::int32_t,last_ : libc::int32_t,ptrb_ : * const libc::int64_t,ptre_ : * const libc::int64_t,asub_ : * const libc::int32_t,aval_ : * const f64) -> i32;
    fn MSK_putaij(task_ : * const u8,i_ : libc::int32_t,j_ : libc::int32_t,aij_ : f64) -> i32;
    fn MSK_putaijlist64(task_ : * const u8,num_ : libc::int64_t,subi_ : * const libc::int32_t,subj_ : * const libc::int32_t,valij_ : * const f64) -> i32;
    fn MSK_putarow(task_ : * const u8,i_ : libc::int32_t,nzi_ : libc::int32_t,subi_ : * const libc::int32_t,vali_ : * const f64) -> i32;
    fn MSK_putarowlist(task_ : * const u8,num_ : libc::int32_t,sub_ : * const libc::int32_t,aptrb_ : * const libc::int32_t,aptre_ : * const libc::int32_t,asub_ : * const libc::int32_t,aval_ : * const f64) -> i32;
    fn MSK_putarowslice64(task_ : * const u8,first_ : libc::int32_t,last_ : libc::int32_t,ptrb_ : * const libc::int64_t,ptre_ : * const libc::int64_t,asub_ : * const libc::int32_t,aval_ : * const f64) -> i32;
    fn MSK_putbarablocktriplet(task_ : * const u8,num_ : libc::int64_t,subi_ : * const libc::int32_t,subj_ : * const libc::int32_t,subk_ : * const libc::int32_t,subl_ : * const libc::int32_t,valijkl_ : * const f64) -> i32;
    fn MSK_putbaraij(task_ : * const u8,i_ : libc::int32_t,j_ : libc::int32_t,num_ : libc::int64_t,sub_ : * const libc::int64_t,weights_ : * const f64) -> i32;
    fn MSK_putbarcblocktriplet(task_ : * const u8,num_ : libc::int64_t,subj_ : * const libc::int32_t,subk_ : * const libc::int32_t,subl_ : * const libc::int32_t,valjkl_ : * const f64) -> i32;
    fn MSK_putbarcj(task_ : * const u8,j_ : libc::int32_t,num_ : libc::int64_t,sub_ : * const libc::int64_t,weights_ : * const f64) -> i32;
    fn MSK_putbarsj(task_ : * const u8,whichsol_ : i32,j_ : libc::int32_t,barsj_ : * const f64) -> i32;
    fn MSK_putbarvarname(task_ : * const u8,j_ : libc::int32_t,name_ : * const libc::c_char) -> i32;
    fn MSK_putbarxj(task_ : * const u8,whichsol_ : i32,j_ : libc::int32_t,barxj_ : * const f64) -> i32;
    fn MSK_putbound(task_ : * const u8,accmode_ : i32,i_ : libc::int32_t,bk_ : i32,bl_ : f64,bu_ : f64) -> i32;
    fn MSK_putboundlist(task_ : * const u8,accmode_ : i32,num_ : libc::int32_t,sub_ : * const libc::int32_t,bk_ : * const i32,bl_ : * const f64,bu_ : * const f64) -> i32;
    fn MSK_putboundslice(task_ : * const u8,con_ : i32,first_ : libc::int32_t,last_ : libc::int32_t,bk_ : * const i32,bl_ : * const f64,bu_ : * const f64) -> i32;
    fn MSK_putcfix(task_ : * const u8,cfix_ : f64) -> i32;
    fn MSK_putcj(task_ : * const u8,j_ : libc::int32_t,cj_ : f64) -> i32;
    fn MSK_putclist(task_ : * const u8,num_ : libc::int32_t,subj_ : * const libc::int32_t,val_ : * const f64) -> i32;
    fn MSK_putconbound(task_ : * const u8,i_ : libc::int32_t,bk_ : i32,bl_ : f64,bu_ : f64) -> i32;
    fn MSK_putconboundlist(task_ : * const u8,num_ : libc::int32_t,sub_ : * const libc::int32_t,bkc_ : * const i32,blc_ : * const f64,buc_ : * const f64) -> i32;
    fn MSK_putconboundslice(task_ : * const u8,first_ : libc::int32_t,last_ : libc::int32_t,bk_ : * const i32,bl_ : * const f64,bu_ : * const f64) -> i32;
    fn MSK_putcone(task_ : * const u8,k_ : libc::int32_t,ct_ : i32,conepar_ : f64,nummem_ : libc::int32_t,submem_ : * const libc::int32_t) -> i32;
    fn MSK_putconename(task_ : * const u8,j_ : libc::int32_t,name_ : * const libc::c_char) -> i32;
    fn MSK_putconname(task_ : * const u8,i_ : libc::int32_t,name_ : * const libc::c_char) -> i32;
    fn MSK_putcslice(task_ : * const u8,first_ : libc::int32_t,last_ : libc::int32_t,slice_ : * const f64) -> i32;
    fn MSK_putdouparam(task_ : * const u8,param_ : i32,parvalue_ : f64) -> i32;
    fn MSK_putintparam(task_ : * const u8,param_ : i32,parvalue_ : libc::int32_t) -> i32;
    fn MSK_putmaxnumanz(task_ : * const u8,maxnumanz_ : libc::int64_t) -> i32;
    fn MSK_putmaxnumbarvar(task_ : * const u8,maxnumbarvar_ : libc::int32_t) -> i32;
    fn MSK_putmaxnumcon(task_ : * const u8,maxnumcon_ : libc::int32_t) -> i32;
    fn MSK_putmaxnumcone(task_ : * const u8,maxnumcone_ : libc::int32_t) -> i32;
    fn MSK_putmaxnumqnz(task_ : * const u8,maxnumqnz_ : libc::int64_t) -> i32;
    fn MSK_putmaxnumvar(task_ : * const u8,maxnumvar_ : libc::int32_t) -> i32;
    fn MSK_putnadouparam(task_ : * const u8,paramname_ : * const libc::c_char,parvalue_ : f64) -> i32;
    fn MSK_putnaintparam(task_ : * const u8,paramname_ : * const libc::c_char,parvalue_ : libc::int32_t) -> i32;
    fn MSK_putnastrparam(task_ : * const u8,paramname_ : * const libc::c_char,parvalue_ : * const libc::c_char) -> i32;
    fn MSK_putobjname(task_ : * const u8,objname_ : * const libc::c_char) -> i32;
    fn MSK_putobjsense(task_ : * const u8,sense_ : i32) -> i32;
    fn MSK_putparam(task_ : * const u8,parname_ : * const libc::c_char,parvalue_ : * const libc::c_char) -> i32;
    fn MSK_putqcon(task_ : * const u8,numqcnz_ : libc::int32_t,qcsubk_ : * const libc::int32_t,qcsubi_ : * const libc::int32_t,qcsubj_ : * const libc::int32_t,qcval_ : * const f64) -> i32;
    fn MSK_putqconk(task_ : * const u8,k_ : libc::int32_t,numqcnz_ : libc::int32_t,qcsubi_ : * const libc::int32_t,qcsubj_ : * const libc::int32_t,qcval_ : * const f64) -> i32;
    fn MSK_putqobj(task_ : * const u8,numqonz_ : libc::int32_t,qosubi_ : * const libc::int32_t,qosubj_ : * const libc::int32_t,qoval_ : * const f64) -> i32;
    fn MSK_putqobjij(task_ : * const u8,i_ : libc::int32_t,j_ : libc::int32_t,qoij_ : f64) -> i32;
    fn MSK_putskc(task_ : * const u8,whichsol_ : i32,skc_ : * const i32) -> i32;
    fn MSK_putskcslice(task_ : * const u8,whichsol_ : i32,first_ : libc::int32_t,last_ : libc::int32_t,skc_ : * const i32) -> i32;
    fn MSK_putskx(task_ : * const u8,whichsol_ : i32,skx_ : * const i32) -> i32;
    fn MSK_putskxslice(task_ : * const u8,whichsol_ : i32,first_ : libc::int32_t,last_ : libc::int32_t,skx_ : * const i32) -> i32;
    fn MSK_putslc(task_ : * const u8,whichsol_ : i32,slc_ : * const f64) -> i32;
    fn MSK_putslcslice(task_ : * const u8,whichsol_ : i32,first_ : libc::int32_t,last_ : libc::int32_t,slc_ : * const f64) -> i32;
    fn MSK_putslx(task_ : * const u8,whichsol_ : i32,slx_ : * const f64) -> i32;
    fn MSK_putslxslice(task_ : * const u8,whichsol_ : i32,first_ : libc::int32_t,last_ : libc::int32_t,slx_ : * const f64) -> i32;
    fn MSK_putsnx(task_ : * const u8,whichsol_ : i32,sux_ : * const f64) -> i32;
    fn MSK_putsnxslice(task_ : * const u8,whichsol_ : i32,first_ : libc::int32_t,last_ : libc::int32_t,snx_ : * const f64) -> i32;
    fn MSK_putsolution(task_ : * const u8,whichsol_ : i32,skc_ : * const i32,skx_ : * const i32,skn_ : * const i32,xc_ : * const f64,xx_ : * const f64,y_ : * const f64,slc_ : * const f64,suc_ : * const f64,slx_ : * const f64,sux_ : * const f64,snx_ : * const f64) -> i32;
    fn MSK_putsolutioni(task_ : * const u8,accmode_ : i32,i_ : libc::int32_t,whichsol_ : i32,sk_ : i32,x_ : f64,sl_ : f64,su_ : f64,sn_ : f64) -> i32;
    fn MSK_putsolutionyi(task_ : * const u8,i_ : libc::int32_t,whichsol_ : i32,y_ : f64) -> i32;
    fn MSK_putstrparam(task_ : * const u8,param_ : i32,parvalue_ : * const libc::c_char) -> i32;
    fn MSK_putsuc(task_ : * const u8,whichsol_ : i32,suc_ : * const f64) -> i32;
    fn MSK_putsucslice(task_ : * const u8,whichsol_ : i32,first_ : libc::int32_t,last_ : libc::int32_t,suc_ : * const f64) -> i32;
    fn MSK_putsux(task_ : * const u8,whichsol_ : i32,sux_ : * const f64) -> i32;
    fn MSK_putsuxslice(task_ : * const u8,whichsol_ : i32,first_ : libc::int32_t,last_ : libc::int32_t,sux_ : * const f64) -> i32;
    fn MSK_puttaskname(task_ : * const u8,taskname_ : * const libc::c_char) -> i32;
    fn MSK_putvarbound(task_ : * const u8,j_ : libc::int32_t,bk_ : i32,bl_ : f64,bu_ : f64) -> i32;
    fn MSK_putvarboundlist(task_ : * const u8,num_ : libc::int32_t,sub_ : * const libc::int32_t,bkx_ : * const i32,blx_ : * const f64,bux_ : * const f64) -> i32;
    fn MSK_putvarboundslice(task_ : * const u8,first_ : libc::int32_t,last_ : libc::int32_t,bk_ : * const i32,bl_ : * const f64,bu_ : * const f64) -> i32;
    fn MSK_putvarbranchorder(task_ : * const u8,j_ : libc::int32_t,priority_ : libc::int32_t,direction_ : i32) -> i32;
    fn MSK_putvarname(task_ : * const u8,j_ : libc::int32_t,name_ : * const libc::c_char) -> i32;
    fn MSK_putvartype(task_ : * const u8,j_ : libc::int32_t,vartype_ : i32) -> i32;
    fn MSK_putvartypelist(task_ : * const u8,num_ : libc::int32_t,subj_ : * const libc::int32_t,vartype_ : * const i32) -> i32;
    fn MSK_putxc(task_ : * const u8,whichsol_ : i32,xc_ : * mut f64) -> i32;
    fn MSK_putxcslice(task_ : * const u8,whichsol_ : i32,first_ : libc::int32_t,last_ : libc::int32_t,xc_ : * const f64) -> i32;
    fn MSK_putxx(task_ : * const u8,whichsol_ : i32,xx_ : * const f64) -> i32;
    fn MSK_putxxslice(task_ : * const u8,whichsol_ : i32,first_ : libc::int32_t,last_ : libc::int32_t,xx_ : * const f64) -> i32;
    fn MSK_puty(task_ : * const u8,whichsol_ : i32,y_ : * const f64) -> i32;
    fn MSK_putyslice(task_ : * const u8,whichsol_ : i32,first_ : libc::int32_t,last_ : libc::int32_t,y_ : * const f64) -> i32;
    fn MSK_readbranchpriorities(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
    fn MSK_readdataautoformat(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
    fn MSK_readdataformat(task_ : * const u8,filename_ : * const libc::c_char,format_ : i32,compress_ : i32) -> i32;
    fn MSK_readparamfile(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
    fn MSK_readsolution(task_ : * const u8,whichsol_ : i32,filename_ : * const libc::c_char) -> i32;
    fn MSK_readsummary(task_ : * const u8,whichstream_ : i32) -> i32;
    fn MSK_readtask(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
    fn MSK_removebarvars(task_ : * const u8,num_ : libc::int32_t,subset_ : * const libc::int32_t) -> i32;
    fn MSK_removecones(task_ : * const u8,num_ : libc::int32_t,subset_ : * const libc::int32_t) -> i32;
    fn MSK_removecons(task_ : * const u8,num_ : libc::int32_t,subset_ : * const libc::int32_t) -> i32;
    fn MSK_removevars(task_ : * const u8,num_ : libc::int32_t,subset_ : * const libc::int32_t) -> i32;
    fn MSK_resizetask(task_ : * const u8,maxnumcon_ : libc::int32_t,maxnumvar_ : libc::int32_t,maxnumcone_ : libc::int32_t,maxnumanz_ : libc::int64_t,maxnumqnz_ : libc::int64_t) -> i32;
    fn MSK_sensitivityreport(task_ : * const u8,whichstream_ : i32) -> i32;
    fn MSK_setdefaults(task_ : * const u8) -> i32;
    fn MSK_sktostr(task_ : * const u8,sk_ : i32,str_ : * const u8) -> i32;
    fn MSK_solstatostr(task_ : * const u8,solsta_ : i32,str_ : * const u8) -> i32;
    fn MSK_solutiondef(task_ : * const u8,whichsol_ : i32,isdef_ : & mut libc::int32_t) -> i32;
    fn MSK_solutionsummary(task_ : * const u8,whichstream_ : i32) -> i32;
    fn MSK_solvewithbasis(task_ : * const u8,transp_ : libc::int32_t,numnz_ : & mut libc::int32_t,sub_ : * mut libc::int32_t,val_ : * mut f64) -> i32;
    fn MSK_startstat(task_ : * const u8) -> i32;
    fn MSK_stopstat(task_ : * const u8) -> i32;
    fn MSK_strtoconetype(task_ : * const u8,str_ : * const libc::c_char,conetype_ : & mut i32) -> i32;
    fn MSK_strtosk(task_ : * const u8,str_ : * const libc::c_char,sk_ : & mut libc::int32_t) -> i32;
    fn MSK_toconic(task_ : * const u8) -> i32;
    fn MSK_updatesolutioninfo(task_ : * const u8,whichsol_ : i32) -> i32;
    fn MSK_writebranchpriorities(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
    fn MSK_writedata(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
    fn MSK_writeparamfile(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
    fn MSK_writesolution(task_ : * const u8,whichsol_ : i32,filename_ : * const libc::c_char) -> i32;
    fn MSK_writetask(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
    fn MSK_axpy(env_ : * const u8,n_ : libc::int32_t,alpha_ : f64,x_ : * const f64,y_ : * mut f64) -> i32;
    fn MSK_checkinlicense(env_ : * const u8,feature_ : i32) -> i32;
    fn MSK_checkoutlicense(env_ : * const u8,feature_ : i32) -> i32;
    fn MSK_dot(env_ : * const u8,n_ : libc::int32_t,x_ : * const f64,y_ : * const f64,xty_ : & mut f64) -> i32;
    fn MSK_echointro(env_ : * const u8,longver_ : libc::int32_t) -> i32;
    fn MSK_gemm(env_ : * const u8,transa_ : i32,transb_ : i32,m_ : libc::int32_t,n_ : libc::int32_t,k_ : libc::int32_t,alpha_ : f64,a_ : * const f64,b_ : * const f64,beta_ : f64,c_ : * mut f64) -> i32;
    fn MSK_gemv(env_ : * const u8,transa_ : i32,m_ : libc::int32_t,n_ : libc::int32_t,alpha_ : f64,a_ : * const f64,x_ : * const f64,beta_ : f64,y_ : * mut f64) -> i32;
    fn MSK_getcodedesc(code_ : i32,symname_ : * const u8,str_ : * const u8) -> i32;
    fn MSK_getversion(major_ : & mut libc::int32_t,minor_ : & mut libc::int32_t,build_ : & mut libc::int32_t,revision_ : & mut libc::int32_t) -> i32;
    fn MSK_licensecleanup() -> i32;
    fn MSK_linkfiletoenvstream(env_ : * const u8,whichstream_ : i32,filename_ : * const libc::c_char,append_ : libc::int32_t) -> i32;
    fn MSK_potrf(env_ : * const u8,uplo_ : i32,n_ : libc::int32_t,a_ : * mut f64) -> i32;
    fn MSK_putkeepdlls(env_ : * const u8,keepdlls_ : libc::int32_t) -> i32;
    fn MSK_putlicensecode(env_ : * const u8,code_ : * const libc::int32_t) -> i32;
    fn MSK_putlicensedebug(env_ : * const u8,licdebug_ : libc::int32_t) -> i32;
    fn MSK_putlicensepath(env_ : * const u8,licensepath_ : * const libc::c_char) -> i32;
    fn MSK_putlicensewait(env_ : * const u8,licwait_ : libc::int32_t) -> i32;
    fn MSK_syeig(env_ : * const u8,uplo_ : i32,n_ : libc::int32_t,a_ : * const f64,w_ : * mut f64) -> i32;
    fn MSK_syevd(env_ : * const u8,uplo_ : i32,n_ : libc::int32_t,a_ : * mut f64,w_ : * mut f64) -> i32;
    fn MSK_syrk(env_ : * const u8,uplo_ : i32,trans_ : i32,n_ : libc::int32_t,k_ : libc::int32_t,alpha_ : f64,a_ : * const f64,beta_ : f64,c_ : * mut f64) -> i32;
}

macro_rules! callMSK
{
    ( $n:ident, $( $a : expr ),* ) => {
        {
            let res = unsafe { $n ( $( $a,)* ) };
            if 0 != res
            {
                panic!(format!("Fail in call: {}",stringify!($n)));
            }
        }
    };
    ( $n:ident ) => {
        {
            let res = unsafe { $n () };
            if 0 != res
            {
                panic!(format!("Fail in call: {}",stringify!($n)));
            }
        }
    }

}

// accmode
pub const MSK_ACC_CON : i32 = 1;
pub const MSK_ACC_VAR : i32 = 0;
pub const MSK_ACC_BEGIN : i32 = 0;
pub const MSK_ACC_END   : i32 = 2;

// basindtype
pub const MSK_BI_ALWAYS      : i32 = 1;
pub const MSK_BI_IF_FEASIBLE : i32 = 3;
pub const MSK_BI_NEVER       : i32 = 0;
pub const MSK_BI_NO_ERROR    : i32 = 2;
pub const MSK_BI_RESERVERED  : i32 = 4;
pub const MSK_BI_BEGIN : i32 = 0;
pub const MSK_BI_END   : i32 = 5;

// boundkey
pub const MSK_BK_FR : i32 = 3;
pub const MSK_BK_FX : i32 = 2;
pub const MSK_BK_LO : i32 = 0;
pub const MSK_BK_RA : i32 = 4;
pub const MSK_BK_UP : i32 = 1;
pub const MSK_BK_BEGIN : i32 = 0;
pub const MSK_BK_END   : i32 = 5;

// branchdir
pub const MSK_BRANCH_DIR_DOWN          : i32 = 2;
pub const MSK_BRANCH_DIR_FAR           : i32 = 4;
pub const MSK_BRANCH_DIR_FREE          : i32 = 0;
pub const MSK_BRANCH_DIR_GUIDED        : i32 = 6;
pub const MSK_BRANCH_DIR_INTERVAL_SIZE : i32 = 8;
pub const MSK_BRANCH_DIR_NEAR          : i32 = 3;
pub const MSK_BRANCH_DIR_PSEUDOCOST    : i32 = 7;
pub const MSK_BRANCH_DIR_ROOT_LP       : i32 = 5;
pub const MSK_BRANCH_DIR_UP            : i32 = 1;
pub const MSK_BRANCH_DIR_BEGIN : i32 = 0;
pub const MSK_BRANCH_DIR_END   : i32 = 9;

// callbackcode
pub const MSK_CALLBACK_BEGIN_BI                      : i32 = 0;
pub const MSK_CALLBACK_BEGIN_CONCURRENT              : i32 = 1;
pub const MSK_CALLBACK_BEGIN_CONIC                   : i32 = 2;
pub const MSK_CALLBACK_BEGIN_DUAL_BI                 : i32 = 3;
pub const MSK_CALLBACK_BEGIN_DUAL_SENSITIVITY        : i32 = 4;
pub const MSK_CALLBACK_BEGIN_DUAL_SETUP_BI           : i32 = 5;
pub const MSK_CALLBACK_BEGIN_DUAL_SIMPLEX            : i32 = 6;
pub const MSK_CALLBACK_BEGIN_DUAL_SIMPLEX_BI         : i32 = 7;
pub const MSK_CALLBACK_BEGIN_FULL_CONVEXITY_CHECK    : i32 = 8;
pub const MSK_CALLBACK_BEGIN_INFEAS_ANA              : i32 = 9;
pub const MSK_CALLBACK_BEGIN_INTPNT                  : i32 = 10;
pub const MSK_CALLBACK_BEGIN_LICENSE_WAIT            : i32 = 11;
pub const MSK_CALLBACK_BEGIN_MIO                     : i32 = 12;
pub const MSK_CALLBACK_BEGIN_NETWORK_DUAL_SIMPLEX    : i32 = 13;
pub const MSK_CALLBACK_BEGIN_NETWORK_PRIMAL_SIMPLEX  : i32 = 14;
pub const MSK_CALLBACK_BEGIN_NETWORK_SIMPLEX         : i32 = 15;
pub const MSK_CALLBACK_BEGIN_OPTIMIZER               : i32 = 16;
pub const MSK_CALLBACK_BEGIN_PRESOLVE                : i32 = 17;
pub const MSK_CALLBACK_BEGIN_PRIMAL_BI               : i32 = 18;
pub const MSK_CALLBACK_BEGIN_PRIMAL_DUAL_SIMPLEX     : i32 = 19;
pub const MSK_CALLBACK_BEGIN_PRIMAL_DUAL_SIMPLEX_BI  : i32 = 20;
pub const MSK_CALLBACK_BEGIN_PRIMAL_REPAIR           : i32 = 21;
pub const MSK_CALLBACK_BEGIN_PRIMAL_SENSITIVITY      : i32 = 22;
pub const MSK_CALLBACK_BEGIN_PRIMAL_SETUP_BI         : i32 = 23;
pub const MSK_CALLBACK_BEGIN_PRIMAL_SIMPLEX          : i32 = 24;
pub const MSK_CALLBACK_BEGIN_PRIMAL_SIMPLEX_BI       : i32 = 25;
pub const MSK_CALLBACK_BEGIN_QCQO_REFORMULATE        : i32 = 26;
pub const MSK_CALLBACK_BEGIN_READ                    : i32 = 27;
pub const MSK_CALLBACK_BEGIN_ROOT_CUTGEN             : i32 = 28;
pub const MSK_CALLBACK_BEGIN_SIMPLEX                 : i32 = 29;
pub const MSK_CALLBACK_BEGIN_SIMPLEX_BI              : i32 = 30;
pub const MSK_CALLBACK_BEGIN_SIMPLEX_NETWORK_DETECT  : i32 = 31;
pub const MSK_CALLBACK_BEGIN_WRITE                   : i32 = 32;
pub const MSK_CALLBACK_CONIC                         : i32 = 33;
pub const MSK_CALLBACK_DUAL_SIMPLEX                  : i32 = 34;
pub const MSK_CALLBACK_END_BI                        : i32 = 35;
pub const MSK_CALLBACK_END_CONCURRENT                : i32 = 36;
pub const MSK_CALLBACK_END_CONIC                     : i32 = 37;
pub const MSK_CALLBACK_END_DUAL_BI                   : i32 = 38;
pub const MSK_CALLBACK_END_DUAL_SENSITIVITY          : i32 = 39;
pub const MSK_CALLBACK_END_DUAL_SETUP_BI             : i32 = 40;
pub const MSK_CALLBACK_END_DUAL_SIMPLEX              : i32 = 41;
pub const MSK_CALLBACK_END_DUAL_SIMPLEX_BI           : i32 = 42;
pub const MSK_CALLBACK_END_FULL_CONVEXITY_CHECK      : i32 = 43;
pub const MSK_CALLBACK_END_INFEAS_ANA                : i32 = 44;
pub const MSK_CALLBACK_END_INTPNT                    : i32 = 45;
pub const MSK_CALLBACK_END_LICENSE_WAIT              : i32 = 46;
pub const MSK_CALLBACK_END_MIO                       : i32 = 47;
pub const MSK_CALLBACK_END_NETWORK_DUAL_SIMPLEX      : i32 = 48;
pub const MSK_CALLBACK_END_NETWORK_PRIMAL_SIMPLEX    : i32 = 49;
pub const MSK_CALLBACK_END_NETWORK_SIMPLEX           : i32 = 50;
pub const MSK_CALLBACK_END_OPTIMIZER                 : i32 = 51;
pub const MSK_CALLBACK_END_PRESOLVE                  : i32 = 52;
pub const MSK_CALLBACK_END_PRIMAL_BI                 : i32 = 53;
pub const MSK_CALLBACK_END_PRIMAL_DUAL_SIMPLEX       : i32 = 54;
pub const MSK_CALLBACK_END_PRIMAL_DUAL_SIMPLEX_BI    : i32 = 55;
pub const MSK_CALLBACK_END_PRIMAL_REPAIR             : i32 = 56;
pub const MSK_CALLBACK_END_PRIMAL_SENSITIVITY        : i32 = 57;
pub const MSK_CALLBACK_END_PRIMAL_SETUP_BI           : i32 = 58;
pub const MSK_CALLBACK_END_PRIMAL_SIMPLEX            : i32 = 59;
pub const MSK_CALLBACK_END_PRIMAL_SIMPLEX_BI         : i32 = 60;
pub const MSK_CALLBACK_END_QCQO_REFORMULATE          : i32 = 61;
pub const MSK_CALLBACK_END_READ                      : i32 = 62;
pub const MSK_CALLBACK_END_ROOT_CUTGEN               : i32 = 63;
pub const MSK_CALLBACK_END_SIMPLEX                   : i32 = 64;
pub const MSK_CALLBACK_END_SIMPLEX_BI                : i32 = 65;
pub const MSK_CALLBACK_END_SIMPLEX_NETWORK_DETECT    : i32 = 66;
pub const MSK_CALLBACK_END_WRITE                     : i32 = 67;
pub const MSK_CALLBACK_IM_BI                         : i32 = 68;
pub const MSK_CALLBACK_IM_CONIC                      : i32 = 69;
pub const MSK_CALLBACK_IM_DUAL_BI                    : i32 = 70;
pub const MSK_CALLBACK_IM_DUAL_SENSIVITY             : i32 = 71;
pub const MSK_CALLBACK_IM_DUAL_SIMPLEX               : i32 = 72;
pub const MSK_CALLBACK_IM_FULL_CONVEXITY_CHECK       : i32 = 73;
pub const MSK_CALLBACK_IM_INTPNT                     : i32 = 74;
pub const MSK_CALLBACK_IM_LICENSE_WAIT               : i32 = 75;
pub const MSK_CALLBACK_IM_LU                         : i32 = 76;
pub const MSK_CALLBACK_IM_MIO                        : i32 = 77;
pub const MSK_CALLBACK_IM_MIO_DUAL_SIMPLEX           : i32 = 78;
pub const MSK_CALLBACK_IM_MIO_INTPNT                 : i32 = 79;
pub const MSK_CALLBACK_IM_MIO_PRIMAL_SIMPLEX         : i32 = 80;
pub const MSK_CALLBACK_IM_NETWORK_DUAL_SIMPLEX       : i32 = 81;
pub const MSK_CALLBACK_IM_NETWORK_PRIMAL_SIMPLEX     : i32 = 82;
pub const MSK_CALLBACK_IM_ORDER                      : i32 = 83;
pub const MSK_CALLBACK_IM_PRESOLVE                   : i32 = 84;
pub const MSK_CALLBACK_IM_PRIMAL_BI                  : i32 = 85;
pub const MSK_CALLBACK_IM_PRIMAL_DUAL_SIMPLEX        : i32 = 86;
pub const MSK_CALLBACK_IM_PRIMAL_SENSIVITY           : i32 = 87;
pub const MSK_CALLBACK_IM_PRIMAL_SIMPLEX             : i32 = 88;
pub const MSK_CALLBACK_IM_QO_REFORMULATE             : i32 = 89;
pub const MSK_CALLBACK_IM_READ                       : i32 = 90;
pub const MSK_CALLBACK_IM_ROOT_CUTGEN                : i32 = 91;
pub const MSK_CALLBACK_IM_SIMPLEX                    : i32 = 92;
pub const MSK_CALLBACK_IM_SIMPLEX_BI                 : i32 = 93;
pub const MSK_CALLBACK_INTPNT                        : i32 = 94;
pub const MSK_CALLBACK_NEW_INT_MIO                   : i32 = 95;
pub const MSK_CALLBACK_PRIMAL_SIMPLEX                : i32 = 96;
pub const MSK_CALLBACK_READ_OPF                      : i32 = 97;
pub const MSK_CALLBACK_READ_OPF_SECTION              : i32 = 98;
pub const MSK_CALLBACK_UPDATE_DUAL_BI                : i32 = 99;
pub const MSK_CALLBACK_UPDATE_DUAL_SIMPLEX           : i32 = 100;
pub const MSK_CALLBACK_UPDATE_DUAL_SIMPLEX_BI        : i32 = 101;
pub const MSK_CALLBACK_UPDATE_NETWORK_DUAL_SIMPLEX   : i32 = 102;
pub const MSK_CALLBACK_UPDATE_NETWORK_PRIMAL_SIMPLEX : i32 = 103;
pub const MSK_CALLBACK_UPDATE_PRESOLVE               : i32 = 104;
pub const MSK_CALLBACK_UPDATE_PRIMAL_BI              : i32 = 105;
pub const MSK_CALLBACK_UPDATE_PRIMAL_DUAL_SIMPLEX    : i32 = 106;
pub const MSK_CALLBACK_UPDATE_PRIMAL_DUAL_SIMPLEX_BI : i32 = 107;
pub const MSK_CALLBACK_UPDATE_PRIMAL_SIMPLEX         : i32 = 108;
pub const MSK_CALLBACK_UPDATE_PRIMAL_SIMPLEX_BI      : i32 = 109;
pub const MSK_CALLBACK_WRITE_OPF                     : i32 = 110;
pub const MSK_CALLBACK_BEGIN : i32 = 0;
pub const MSK_CALLBACK_END   : i32 = 111;

// checkconvexitytype
pub const MSK_CHECK_CONVEXITY_FULL   : i32 = 2;
pub const MSK_CHECK_CONVEXITY_NONE   : i32 = 0;
pub const MSK_CHECK_CONVEXITY_SIMPLE : i32 = 1;
pub const MSK_CHECK_CONVEXITY_BEGIN : i32 = 0;
pub const MSK_CHECK_CONVEXITY_END   : i32 = 3;

// compresstype
pub const MSK_COMPRESS_FREE : i32 = 1;
pub const MSK_COMPRESS_GZIP : i32 = 2;
pub const MSK_COMPRESS_NONE : i32 = 0;
pub const MSK_COMPRESS_BEGIN : i32 = 0;
pub const MSK_COMPRESS_END   : i32 = 3;

// conetype
pub const MSK_CT_QUAD  : i32 = 0;
pub const MSK_CT_RQUAD : i32 = 1;
pub const MSK_CT_BEGIN : i32 = 0;
pub const MSK_CT_END   : i32 = 2;

// dataformat
pub const MSK_DATA_FORMAT_CB        : i32 = 7;
pub const MSK_DATA_FORMAT_EXTENSION : i32 = 0;
pub const MSK_DATA_FORMAT_FREE_MPS  : i32 = 5;
pub const MSK_DATA_FORMAT_LP        : i32 = 2;
pub const MSK_DATA_FORMAT_MPS       : i32 = 1;
pub const MSK_DATA_FORMAT_OP        : i32 = 3;
pub const MSK_DATA_FORMAT_TASK      : i32 = 6;
pub const MSK_DATA_FORMAT_XML       : i32 = 4;
pub const MSK_DATA_FORMAT_BEGIN : i32 = 0;
pub const MSK_DATA_FORMAT_END   : i32 = 8;

// dinfitem
pub const MSK_DINF_BI_CLEAN_DUAL_TIME                             : i32 = 0;
pub const MSK_DINF_BI_CLEAN_PRIMAL_DUAL_TIME                      : i32 = 1;
pub const MSK_DINF_BI_CLEAN_PRIMAL_TIME                           : i32 = 2;
pub const MSK_DINF_BI_CLEAN_TIME                                  : i32 = 3;
pub const MSK_DINF_BI_DUAL_TIME                                   : i32 = 4;
pub const MSK_DINF_BI_PRIMAL_TIME                                 : i32 = 5;
pub const MSK_DINF_BI_TIME                                        : i32 = 6;
pub const MSK_DINF_CONCURRENT_TIME                                : i32 = 7;
pub const MSK_DINF_INTPNT_DUAL_FEAS                               : i32 = 8;
pub const MSK_DINF_INTPNT_DUAL_OBJ                                : i32 = 9;
pub const MSK_DINF_INTPNT_FACTOR_NUM_FLOPS                        : i32 = 10;
pub const MSK_DINF_INTPNT_OPT_STATUS                              : i32 = 11;
pub const MSK_DINF_INTPNT_ORDER_TIME                              : i32 = 12;
pub const MSK_DINF_INTPNT_PRIMAL_FEAS                             : i32 = 13;
pub const MSK_DINF_INTPNT_PRIMAL_OBJ                              : i32 = 14;
pub const MSK_DINF_INTPNT_TIME                                    : i32 = 15;
pub const MSK_DINF_MIO_CLIQUE_SEPARATION_TIME                     : i32 = 16;
pub const MSK_DINF_MIO_CMIR_SEPARATION_TIME                       : i32 = 17;
pub const MSK_DINF_MIO_CONSTRUCT_SOLUTION_OBJ                     : i32 = 18;
pub const MSK_DINF_MIO_DUAL_BOUND_AFTER_PRESOLVE                  : i32 = 19;
pub const MSK_DINF_MIO_GMI_SEPARATION_TIME                        : i32 = 20;
pub const MSK_DINF_MIO_HEURISTIC_TIME                             : i32 = 21;
pub const MSK_DINF_MIO_KNAPSACK_COVER_SEPARATION_TIME             : i32 = 22;
pub const MSK_DINF_MIO_OBJ_ABS_GAP                                : i32 = 23;
pub const MSK_DINF_MIO_OBJ_BOUND                                  : i32 = 24;
pub const MSK_DINF_MIO_OBJ_INT                                    : i32 = 25;
pub const MSK_DINF_MIO_OBJ_REL_GAP                                : i32 = 26;
pub const MSK_DINF_MIO_OPTIMIZER_TIME                             : i32 = 27;
pub const MSK_DINF_MIO_PROBING_TIME                               : i32 = 28;
pub const MSK_DINF_MIO_ROOT_CUTGEN_TIME                           : i32 = 29;
pub const MSK_DINF_MIO_ROOT_OPTIMIZER_TIME                        : i32 = 30;
pub const MSK_DINF_MIO_ROOT_PRESOLVE_TIME                         : i32 = 31;
pub const MSK_DINF_MIO_TIME                                       : i32 = 32;
pub const MSK_DINF_MIO_USER_OBJ_CUT                               : i32 = 33;
pub const MSK_DINF_OPTIMIZER_TIME                                 : i32 = 34;
pub const MSK_DINF_PRESOLVE_ELI_TIME                              : i32 = 35;
pub const MSK_DINF_PRESOLVE_LINDEP_TIME                           : i32 = 36;
pub const MSK_DINF_PRESOLVE_TIME                                  : i32 = 37;
pub const MSK_DINF_PRIMAL_REPAIR_PENALTY_OBJ                      : i32 = 38;
pub const MSK_DINF_QCQO_REFORMULATE_MAX_PERTURBATION              : i32 = 39;
pub const MSK_DINF_QCQO_REFORMULATE_TIME                          : i32 = 40;
pub const MSK_DINF_QCQO_REFORMULATE_WORST_CHOLESKY_COLUMN_SCALING : i32 = 41;
pub const MSK_DINF_QCQO_REFORMULATE_WORST_CHOLESKY_DIAG_SCALING   : i32 = 42;
pub const MSK_DINF_RD_TIME                                        : i32 = 43;
pub const MSK_DINF_SIM_DUAL_TIME                                  : i32 = 44;
pub const MSK_DINF_SIM_FEAS                                       : i32 = 45;
pub const MSK_DINF_SIM_NETWORK_DUAL_TIME                          : i32 = 46;
pub const MSK_DINF_SIM_NETWORK_PRIMAL_TIME                        : i32 = 47;
pub const MSK_DINF_SIM_NETWORK_TIME                               : i32 = 48;
pub const MSK_DINF_SIM_OBJ                                        : i32 = 49;
pub const MSK_DINF_SIM_PRIMAL_DUAL_TIME                           : i32 = 50;
pub const MSK_DINF_SIM_PRIMAL_TIME                                : i32 = 51;
pub const MSK_DINF_SIM_TIME                                       : i32 = 52;
pub const MSK_DINF_SOL_BAS_DUAL_OBJ                               : i32 = 53;
pub const MSK_DINF_SOL_BAS_DVIOLCON                               : i32 = 54;
pub const MSK_DINF_SOL_BAS_DVIOLVAR                               : i32 = 55;
pub const MSK_DINF_SOL_BAS_PRIMAL_OBJ                             : i32 = 56;
pub const MSK_DINF_SOL_BAS_PVIOLCON                               : i32 = 57;
pub const MSK_DINF_SOL_BAS_PVIOLVAR                               : i32 = 58;
pub const MSK_DINF_SOL_ITG_PRIMAL_OBJ                             : i32 = 59;
pub const MSK_DINF_SOL_ITG_PVIOLBARVAR                            : i32 = 60;
pub const MSK_DINF_SOL_ITG_PVIOLCON                               : i32 = 61;
pub const MSK_DINF_SOL_ITG_PVIOLCONES                             : i32 = 62;
pub const MSK_DINF_SOL_ITG_PVIOLITG                               : i32 = 63;
pub const MSK_DINF_SOL_ITG_PVIOLVAR                               : i32 = 64;
pub const MSK_DINF_SOL_ITR_DUAL_OBJ                               : i32 = 65;
pub const MSK_DINF_SOL_ITR_DVIOLBARVAR                            : i32 = 66;
pub const MSK_DINF_SOL_ITR_DVIOLCON                               : i32 = 67;
pub const MSK_DINF_SOL_ITR_DVIOLCONES                             : i32 = 68;
pub const MSK_DINF_SOL_ITR_DVIOLVAR                               : i32 = 69;
pub const MSK_DINF_SOL_ITR_PRIMAL_OBJ                             : i32 = 70;
pub const MSK_DINF_SOL_ITR_PVIOLBARVAR                            : i32 = 71;
pub const MSK_DINF_SOL_ITR_PVIOLCON                               : i32 = 72;
pub const MSK_DINF_SOL_ITR_PVIOLCONES                             : i32 = 73;
pub const MSK_DINF_SOL_ITR_PVIOLVAR                               : i32 = 74;
pub const MSK_DINF_BEGIN : i32 = 0;
pub const MSK_DINF_END   : i32 = 75;

// dparam
pub const MSK_DPAR_ANA_SOL_INFEAS_TOL                 : i32 = 0;
pub const MSK_DPAR_BASIS_REL_TOL_S                    : i32 = 1;
pub const MSK_DPAR_BASIS_TOL_S                        : i32 = 2;
pub const MSK_DPAR_BASIS_TOL_X                        : i32 = 3;
pub const MSK_DPAR_CHECK_CONVEXITY_REL_TOL            : i32 = 4;
pub const MSK_DPAR_DATA_TOL_AIJ                       : i32 = 5;
pub const MSK_DPAR_DATA_TOL_AIJ_HUGE                  : i32 = 6;
pub const MSK_DPAR_DATA_TOL_AIJ_LARGE                 : i32 = 7;
pub const MSK_DPAR_DATA_TOL_BOUND_INF                 : i32 = 8;
pub const MSK_DPAR_DATA_TOL_BOUND_WRN                 : i32 = 9;
pub const MSK_DPAR_DATA_TOL_C_HUGE                    : i32 = 10;
pub const MSK_DPAR_DATA_TOL_CJ_LARGE                  : i32 = 11;
pub const MSK_DPAR_DATA_TOL_QIJ                       : i32 = 12;
pub const MSK_DPAR_DATA_TOL_X                         : i32 = 13;
pub const MSK_DPAR_FEASREPAIR_TOL                     : i32 = 14;
pub const MSK_DPAR_INTPNT_CO_TOL_DFEAS                : i32 = 15;
pub const MSK_DPAR_INTPNT_CO_TOL_INFEAS               : i32 = 16;
pub const MSK_DPAR_INTPNT_CO_TOL_MU_RED               : i32 = 17;
pub const MSK_DPAR_INTPNT_CO_TOL_NEAR_REL             : i32 = 18;
pub const MSK_DPAR_INTPNT_CO_TOL_PFEAS                : i32 = 19;
pub const MSK_DPAR_INTPNT_CO_TOL_REL_GAP              : i32 = 20;
pub const MSK_DPAR_INTPNT_NL_MERIT_BAL                : i32 = 21;
pub const MSK_DPAR_INTPNT_NL_TOL_DFEAS                : i32 = 22;
pub const MSK_DPAR_INTPNT_NL_TOL_MU_RED               : i32 = 23;
pub const MSK_DPAR_INTPNT_NL_TOL_NEAR_REL             : i32 = 24;
pub const MSK_DPAR_INTPNT_NL_TOL_PFEAS                : i32 = 25;
pub const MSK_DPAR_INTPNT_NL_TOL_REL_GAP              : i32 = 26;
pub const MSK_DPAR_INTPNT_NL_TOL_REL_STEP             : i32 = 27;
pub const MSK_DPAR_INTPNT_QO_TOL_DFEAS                : i32 = 28;
pub const MSK_DPAR_INTPNT_QO_TOL_INFEAS               : i32 = 29;
pub const MSK_DPAR_INTPNT_QO_TOL_MU_RED               : i32 = 30;
pub const MSK_DPAR_INTPNT_QO_TOL_NEAR_REL             : i32 = 31;
pub const MSK_DPAR_INTPNT_QO_TOL_PFEAS                : i32 = 32;
pub const MSK_DPAR_INTPNT_QO_TOL_REL_GAP              : i32 = 33;
pub const MSK_DPAR_INTPNT_TOL_DFEAS                   : i32 = 34;
pub const MSK_DPAR_INTPNT_TOL_DSAFE                   : i32 = 35;
pub const MSK_DPAR_INTPNT_TOL_INFEAS                  : i32 = 36;
pub const MSK_DPAR_INTPNT_TOL_MU_RED                  : i32 = 37;
pub const MSK_DPAR_INTPNT_TOL_PATH                    : i32 = 38;
pub const MSK_DPAR_INTPNT_TOL_PFEAS                   : i32 = 39;
pub const MSK_DPAR_INTPNT_TOL_PSAFE                   : i32 = 40;
pub const MSK_DPAR_INTPNT_TOL_REL_GAP                 : i32 = 41;
pub const MSK_DPAR_INTPNT_TOL_REL_STEP                : i32 = 42;
pub const MSK_DPAR_INTPNT_TOL_STEP_SIZE               : i32 = 43;
pub const MSK_DPAR_LOWER_OBJ_CUT                      : i32 = 44;
pub const MSK_DPAR_LOWER_OBJ_CUT_FINITE_TRH           : i32 = 45;
pub const MSK_DPAR_MIO_DISABLE_TERM_TIME              : i32 = 46;
pub const MSK_DPAR_MIO_HEURISTIC_TIME                 : i32 = 47;
pub const MSK_DPAR_MIO_MAX_TIME                       : i32 = 48;
pub const MSK_DPAR_MIO_MAX_TIME_APRX_OPT              : i32 = 49;
pub const MSK_DPAR_MIO_NEAR_TOL_ABS_GAP               : i32 = 50;
pub const MSK_DPAR_MIO_NEAR_TOL_REL_GAP               : i32 = 51;
pub const MSK_DPAR_MIO_REL_ADD_CUT_LIMITED            : i32 = 52;
pub const MSK_DPAR_MIO_REL_GAP_CONST                  : i32 = 53;
pub const MSK_DPAR_MIO_TOL_ABS_GAP                    : i32 = 54;
pub const MSK_DPAR_MIO_TOL_ABS_RELAX_INT              : i32 = 55;
pub const MSK_DPAR_MIO_TOL_FEAS                       : i32 = 56;
pub const MSK_DPAR_MIO_TOL_MAX_CUT_FRAC_RHS           : i32 = 57;
pub const MSK_DPAR_MIO_TOL_MIN_CUT_FRAC_RHS           : i32 = 58;
pub const MSK_DPAR_MIO_TOL_REL_DUAL_BOUND_IMPROVEMENT : i32 = 59;
pub const MSK_DPAR_MIO_TOL_REL_GAP                    : i32 = 60;
pub const MSK_DPAR_MIO_TOL_REL_RELAX_INT              : i32 = 61;
pub const MSK_DPAR_MIO_TOL_X                          : i32 = 62;
pub const MSK_DPAR_OPTIMIZER_MAX_TIME                 : i32 = 63;
pub const MSK_DPAR_PRESOLVE_TOL_ABS_LINDEP            : i32 = 64;
pub const MSK_DPAR_PRESOLVE_TOL_AIJ                   : i32 = 65;
pub const MSK_DPAR_PRESOLVE_TOL_REL_LINDEP            : i32 = 66;
pub const MSK_DPAR_PRESOLVE_TOL_S                     : i32 = 67;
pub const MSK_DPAR_PRESOLVE_TOL_X                     : i32 = 68;
pub const MSK_DPAR_QCQO_REFORMULATE_REL_DROP_TOL      : i32 = 69;
pub const MSK_DPAR_SEMIDEFINITE_TOL_APPROX            : i32 = 70;
pub const MSK_DPAR_SIM_LU_TOL_REL_PIV                 : i32 = 71;
pub const MSK_DPAR_SIMPLEX_ABS_TOL_PIV                : i32 = 72;
pub const MSK_DPAR_UPPER_OBJ_CUT                      : i32 = 73;
pub const MSK_DPAR_UPPER_OBJ_CUT_FINITE_TRH           : i32 = 74;
pub const MSK_DPAR_BEGIN : i32 = 0;
pub const MSK_DPAR_END   : i32 = 75;

// feasrepairtype
pub const MSK_FEASREPAIR_OPTIMIZE_COMBINED : i32 = 2;
pub const MSK_FEASREPAIR_OPTIMIZE_NONE     : i32 = 0;
pub const MSK_FEASREPAIR_OPTIMIZE_PENALTY  : i32 = 1;
pub const MSK_FEASREPAIR_BEGIN : i32 = 0;
pub const MSK_FEASREPAIR_END   : i32 = 3;

// feature
pub const MSK_FEATURE_PTOM : i32 = 2;
pub const MSK_FEATURE_PTON : i32 = 1;
pub const MSK_FEATURE_PTOX : i32 = 3;
pub const MSK_FEATURE_PTS  : i32 = 0;
pub const MSK_FEATURE_BEGIN : i32 = 0;
pub const MSK_FEATURE_END   : i32 = 4;

// iinfitem
pub const MSK_IINF_ANA_PRO_NUM_CON                : i32 = 0;
pub const MSK_IINF_ANA_PRO_NUM_CON_EQ             : i32 = 1;
pub const MSK_IINF_ANA_PRO_NUM_CON_FR             : i32 = 2;
pub const MSK_IINF_ANA_PRO_NUM_CON_LO             : i32 = 3;
pub const MSK_IINF_ANA_PRO_NUM_CON_RA             : i32 = 4;
pub const MSK_IINF_ANA_PRO_NUM_CON_UP             : i32 = 5;
pub const MSK_IINF_ANA_PRO_NUM_VAR                : i32 = 6;
pub const MSK_IINF_ANA_PRO_NUM_VAR_BIN            : i32 = 7;
pub const MSK_IINF_ANA_PRO_NUM_VAR_CONT           : i32 = 8;
pub const MSK_IINF_ANA_PRO_NUM_VAR_EQ             : i32 = 9;
pub const MSK_IINF_ANA_PRO_NUM_VAR_FR             : i32 = 10;
pub const MSK_IINF_ANA_PRO_NUM_VAR_INT            : i32 = 11;
pub const MSK_IINF_ANA_PRO_NUM_VAR_LO             : i32 = 12;
pub const MSK_IINF_ANA_PRO_NUM_VAR_RA             : i32 = 13;
pub const MSK_IINF_ANA_PRO_NUM_VAR_UP             : i32 = 14;
pub const MSK_IINF_CONCURRENT_FASTEST_OPTIMIZER   : i32 = 15;
pub const MSK_IINF_INTPNT_FACTOR_DIM_DENSE        : i32 = 16;
pub const MSK_IINF_INTPNT_ITER                    : i32 = 17;
pub const MSK_IINF_INTPNT_NUM_THREADS             : i32 = 18;
pub const MSK_IINF_INTPNT_SOLVE_DUAL              : i32 = 19;
pub const MSK_IINF_MIO_ABSGAP_SATISFIED           : i32 = 20;
pub const MSK_IINF_MIO_CLIQUE_TABLE_SIZE          : i32 = 21;
pub const MSK_IINF_MIO_CONSTRUCT_NUM_ROUNDINGS    : i32 = 22;
pub const MSK_IINF_MIO_CONSTRUCT_SOLUTION         : i32 = 23;
pub const MSK_IINF_MIO_INITIAL_SOLUTION           : i32 = 24;
pub const MSK_IINF_MIO_NEAR_ABSGAP_SATISFIED      : i32 = 25;
pub const MSK_IINF_MIO_NEAR_RELGAP_SATISFIED      : i32 = 26;
pub const MSK_IINF_MIO_NODE_DEPTH                 : i32 = 27;
pub const MSK_IINF_MIO_NUM_ACTIVE_NODES           : i32 = 28;
pub const MSK_IINF_MIO_NUM_BRANCH                 : i32 = 29;
pub const MSK_IINF_MIO_NUM_CLIQUE_CUTS            : i32 = 30;
pub const MSK_IINF_MIO_NUM_CMIR_CUTS              : i32 = 31;
pub const MSK_IINF_MIO_NUM_GOMORY_CUTS            : i32 = 32;
pub const MSK_IINF_MIO_NUM_INT_SOLUTIONS          : i32 = 33;
pub const MSK_IINF_MIO_NUM_KNAPSACK_COVER_CUTS    : i32 = 34;
pub const MSK_IINF_MIO_NUM_RELAX                  : i32 = 35;
pub const MSK_IINF_MIO_NUM_REPEATED_PRESOLVE      : i32 = 36;
pub const MSK_IINF_MIO_NUMCON                     : i32 = 37;
pub const MSK_IINF_MIO_NUMINT                     : i32 = 38;
pub const MSK_IINF_MIO_NUMVAR                     : i32 = 39;
pub const MSK_IINF_MIO_OBJ_BOUND_DEFINED          : i32 = 40;
pub const MSK_IINF_MIO_PRESOLVED_NUMBIN           : i32 = 41;
pub const MSK_IINF_MIO_PRESOLVED_NUMCON           : i32 = 42;
pub const MSK_IINF_MIO_PRESOLVED_NUMCONT          : i32 = 43;
pub const MSK_IINF_MIO_PRESOLVED_NUMINT           : i32 = 44;
pub const MSK_IINF_MIO_PRESOLVED_NUMVAR           : i32 = 45;
pub const MSK_IINF_MIO_RELGAP_SATISFIED           : i32 = 46;
pub const MSK_IINF_MIO_TOTAL_NUM_CUTS             : i32 = 47;
pub const MSK_IINF_MIO_USER_OBJ_CUT               : i32 = 48;
pub const MSK_IINF_OPT_NUMCON                     : i32 = 49;
pub const MSK_IINF_OPT_NUMVAR                     : i32 = 50;
pub const MSK_IINF_OPTIMIZE_RESPONSE              : i32 = 51;
pub const MSK_IINF_RD_NUMBARVAR                   : i32 = 52;
pub const MSK_IINF_RD_NUMCON                      : i32 = 53;
pub const MSK_IINF_RD_NUMCONE                     : i32 = 54;
pub const MSK_IINF_RD_NUMINTVAR                   : i32 = 55;
pub const MSK_IINF_RD_NUMQ                        : i32 = 56;
pub const MSK_IINF_RD_NUMVAR                      : i32 = 57;
pub const MSK_IINF_RD_PROTYPE                     : i32 = 58;
pub const MSK_IINF_SIM_DUAL_DEG_ITER              : i32 = 59;
pub const MSK_IINF_SIM_DUAL_HOTSTART              : i32 = 60;
pub const MSK_IINF_SIM_DUAL_HOTSTART_LU           : i32 = 61;
pub const MSK_IINF_SIM_DUAL_INF_ITER              : i32 = 62;
pub const MSK_IINF_SIM_DUAL_ITER                  : i32 = 63;
pub const MSK_IINF_SIM_NETWORK_DUAL_DEG_ITER      : i32 = 64;
pub const MSK_IINF_SIM_NETWORK_DUAL_HOTSTART      : i32 = 65;
pub const MSK_IINF_SIM_NETWORK_DUAL_HOTSTART_LU   : i32 = 66;
pub const MSK_IINF_SIM_NETWORK_DUAL_INF_ITER      : i32 = 67;
pub const MSK_IINF_SIM_NETWORK_DUAL_ITER          : i32 = 68;
pub const MSK_IINF_SIM_NETWORK_PRIMAL_DEG_ITER    : i32 = 69;
pub const MSK_IINF_SIM_NETWORK_PRIMAL_HOTSTART    : i32 = 70;
pub const MSK_IINF_SIM_NETWORK_PRIMAL_HOTSTART_LU : i32 = 71;
pub const MSK_IINF_SIM_NETWORK_PRIMAL_INF_ITER    : i32 = 72;
pub const MSK_IINF_SIM_NETWORK_PRIMAL_ITER        : i32 = 73;
pub const MSK_IINF_SIM_NUMCON                     : i32 = 74;
pub const MSK_IINF_SIM_NUMVAR                     : i32 = 75;
pub const MSK_IINF_SIM_PRIMAL_DEG_ITER            : i32 = 76;
pub const MSK_IINF_SIM_PRIMAL_DUAL_DEG_ITER       : i32 = 77;
pub const MSK_IINF_SIM_PRIMAL_DUAL_HOTSTART       : i32 = 78;
pub const MSK_IINF_SIM_PRIMAL_DUAL_HOTSTART_LU    : i32 = 79;
pub const MSK_IINF_SIM_PRIMAL_DUAL_INF_ITER       : i32 = 80;
pub const MSK_IINF_SIM_PRIMAL_DUAL_ITER           : i32 = 81;
pub const MSK_IINF_SIM_PRIMAL_HOTSTART            : i32 = 82;
pub const MSK_IINF_SIM_PRIMAL_HOTSTART_LU         : i32 = 83;
pub const MSK_IINF_SIM_PRIMAL_INF_ITER            : i32 = 84;
pub const MSK_IINF_SIM_PRIMAL_ITER                : i32 = 85;
pub const MSK_IINF_SIM_SOLVE_DUAL                 : i32 = 86;
pub const MSK_IINF_SOL_BAS_PROSTA                 : i32 = 87;
pub const MSK_IINF_SOL_BAS_SOLSTA                 : i32 = 88;
pub const MSK_IINF_SOL_ITG_PROSTA                 : i32 = 89;
pub const MSK_IINF_SOL_ITG_SOLSTA                 : i32 = 90;
pub const MSK_IINF_SOL_ITR_PROSTA                 : i32 = 91;
pub const MSK_IINF_SOL_ITR_SOLSTA                 : i32 = 92;
pub const MSK_IINF_STO_NUM_A_CACHE_FLUSHES        : i32 = 93;
pub const MSK_IINF_STO_NUM_A_REALLOC              : i32 = 94;
pub const MSK_IINF_STO_NUM_A_TRANSPOSES           : i32 = 95;
pub const MSK_IINF_BEGIN : i32 = 0;
pub const MSK_IINF_END   : i32 = 96;

// inftype
pub const MSK_INF_DOU_TYPE  : i32 = 0;
pub const MSK_INF_INT_TYPE  : i32 = 1;
pub const MSK_INF_LINT_TYPE : i32 = 2;
pub const MSK_INF_BEGIN : i32 = 0;
pub const MSK_INF_END   : i32 = 3;

// intpnthotstart
pub const MSK_INTPNT_HOTSTART_DUAL        : i32 = 2;
pub const MSK_INTPNT_HOTSTART_NONE        : i32 = 0;
pub const MSK_INTPNT_HOTSTART_PRIMAL      : i32 = 1;
pub const MSK_INTPNT_HOTSTART_PRIMAL_DUAL : i32 = 3;
pub const MSK_INTPNT_HOTSTART_BEGIN : i32 = 0;
pub const MSK_INTPNT_HOTSTART_END   : i32 = 4;

// iomode
pub const MSK_IOMODE_READ      : i32 = 0;
pub const MSK_IOMODE_READWRITE : i32 = 2;
pub const MSK_IOMODE_WRITE     : i32 = 1;
pub const MSK_IOMODE_BEGIN : i32 = 0;
pub const MSK_IOMODE_END   : i32 = 3;

// iparam
pub const MSK_IPAR_ANA_SOL_BASIS                         : i32 = 0;
pub const MSK_IPAR_ANA_SOL_PRINT_VIOLATED                : i32 = 1;
pub const MSK_IPAR_AUTO_SORT_A_BEFORE_OPT                : i32 = 2;
pub const MSK_IPAR_AUTO_UPDATE_SOL_INFO                  : i32 = 3;
pub const MSK_IPAR_BASIS_SOLVE_USE_PLUS_ONE              : i32 = 4;
pub const MSK_IPAR_BI_CLEAN_OPTIMIZER                    : i32 = 5;
pub const MSK_IPAR_BI_IGNORE_MAX_ITER                    : i32 = 6;
pub const MSK_IPAR_BI_IGNORE_NUM_ERROR                   : i32 = 7;
pub const MSK_IPAR_BI_MAX_ITERATIONS                     : i32 = 8;
pub const MSK_IPAR_CACHE_LICENSE                         : i32 = 9;
pub const MSK_IPAR_CHECK_CONVEXITY                       : i32 = 10;
pub const MSK_IPAR_COMPRESS_STATFILE                     : i32 = 11;
pub const MSK_IPAR_CONCURRENT_NUM_OPTIMIZERS             : i32 = 12;
pub const MSK_IPAR_CONCURRENT_PRIORITY_DUAL_SIMPLEX      : i32 = 13;
pub const MSK_IPAR_CONCURRENT_PRIORITY_FREE_SIMPLEX      : i32 = 14;
pub const MSK_IPAR_CONCURRENT_PRIORITY_INTPNT            : i32 = 15;
pub const MSK_IPAR_CONCURRENT_PRIORITY_PRIMAL_SIMPLEX    : i32 = 16;
pub const MSK_IPAR_FEASREPAIR_OPTIMIZE                   : i32 = 17;
pub const MSK_IPAR_INFEAS_GENERIC_NAMES                  : i32 = 18;
pub const MSK_IPAR_INFEAS_PREFER_PRIMAL                  : i32 = 19;
pub const MSK_IPAR_INFEAS_REPORT_AUTO                    : i32 = 20;
pub const MSK_IPAR_INFEAS_REPORT_LEVEL                   : i32 = 21;
pub const MSK_IPAR_INTPNT_BASIS                          : i32 = 22;
pub const MSK_IPAR_INTPNT_DIFF_STEP                      : i32 = 23;
pub const MSK_IPAR_INTPNT_FACTOR_DEBUG_LVL               : i32 = 24;
pub const MSK_IPAR_INTPNT_FACTOR_METHOD                  : i32 = 25;
pub const MSK_IPAR_INTPNT_HOTSTART                       : i32 = 26;
pub const MSK_IPAR_INTPNT_MAX_ITERATIONS                 : i32 = 27;
pub const MSK_IPAR_INTPNT_MAX_NUM_COR                    : i32 = 28;
pub const MSK_IPAR_INTPNT_MAX_NUM_REFINEMENT_STEPS       : i32 = 29;
pub const MSK_IPAR_INTPNT_OFF_COL_TRH                    : i32 = 30;
pub const MSK_IPAR_INTPNT_ORDER_METHOD                   : i32 = 31;
pub const MSK_IPAR_INTPNT_REGULARIZATION_USE             : i32 = 32;
pub const MSK_IPAR_INTPNT_SCALING                        : i32 = 33;
pub const MSK_IPAR_INTPNT_SOLVE_FORM                     : i32 = 34;
pub const MSK_IPAR_INTPNT_STARTING_POINT                 : i32 = 35;
pub const MSK_IPAR_LIC_TRH_EXPIRY_WRN                    : i32 = 36;
pub const MSK_IPAR_LICENSE_DEBUG                         : i32 = 37;
pub const MSK_IPAR_LICENSE_PAUSE_TIME                    : i32 = 38;
pub const MSK_IPAR_LICENSE_SUPPRESS_EXPIRE_WRNS          : i32 = 39;
pub const MSK_IPAR_LICENSE_WAIT                          : i32 = 40;
pub const MSK_IPAR_LOG                                   : i32 = 41;
pub const MSK_IPAR_LOG_ANA_PRO                           : i32 = 42;
pub const MSK_IPAR_LOG_BI                                : i32 = 43;
pub const MSK_IPAR_LOG_BI_FREQ                           : i32 = 44;
pub const MSK_IPAR_LOG_CHECK_CONVEXITY                   : i32 = 45;
pub const MSK_IPAR_LOG_CONCURRENT                        : i32 = 46;
pub const MSK_IPAR_LOG_CUT_SECOND_OPT                    : i32 = 47;
pub const MSK_IPAR_LOG_EXPAND                            : i32 = 48;
pub const MSK_IPAR_LOG_FACTOR                            : i32 = 49;
pub const MSK_IPAR_LOG_FEAS_REPAIR                       : i32 = 50;
pub const MSK_IPAR_LOG_FILE                              : i32 = 51;
pub const MSK_IPAR_LOG_HEAD                              : i32 = 52;
pub const MSK_IPAR_LOG_INFEAS_ANA                        : i32 = 53;
pub const MSK_IPAR_LOG_INTPNT                            : i32 = 54;
pub const MSK_IPAR_LOG_MIO                               : i32 = 55;
pub const MSK_IPAR_LOG_MIO_FREQ                          : i32 = 56;
pub const MSK_IPAR_LOG_OPTIMIZER                         : i32 = 57;
pub const MSK_IPAR_LOG_ORDER                             : i32 = 58;
pub const MSK_IPAR_LOG_PRESOLVE                          : i32 = 59;
pub const MSK_IPAR_LOG_RESPONSE                          : i32 = 60;
pub const MSK_IPAR_LOG_SENSITIVITY                       : i32 = 61;
pub const MSK_IPAR_LOG_SENSITIVITY_OPT                   : i32 = 62;
pub const MSK_IPAR_LOG_SIM                               : i32 = 63;
pub const MSK_IPAR_LOG_SIM_FREQ                          : i32 = 64;
pub const MSK_IPAR_LOG_SIM_MINOR                         : i32 = 65;
pub const MSK_IPAR_LOG_STORAGE                           : i32 = 66;
pub const MSK_IPAR_MAX_NUM_WARNINGS                      : i32 = 67;
pub const MSK_IPAR_MIO_BRANCH_DIR                        : i32 = 68;
pub const MSK_IPAR_MIO_BRANCH_PRIORITIES_USE             : i32 = 69;
pub const MSK_IPAR_MIO_CONSTRUCT_SOL                     : i32 = 70;
pub const MSK_IPAR_MIO_CUT_CLIQUE                        : i32 = 71;
pub const MSK_IPAR_MIO_CUT_CMIR                          : i32 = 72;
pub const MSK_IPAR_MIO_CUT_GMI                           : i32 = 73;
pub const MSK_IPAR_MIO_CUT_KNAPSACK_COVER                : i32 = 74;
pub const MSK_IPAR_MIO_CUT_LEVEL_ROOT                    : i32 = 75;
pub const MSK_IPAR_MIO_CUT_LEVEL_TREE                    : i32 = 76;
pub const MSK_IPAR_MIO_FEASPUMP_LEVEL                    : i32 = 77;
pub const MSK_IPAR_MIO_HEURISTIC_LEVEL                   : i32 = 78;
pub const MSK_IPAR_MIO_HOTSTART                          : i32 = 79;
pub const MSK_IPAR_MIO_KEEP_BASIS                        : i32 = 80;
pub const MSK_IPAR_MIO_LOCAL_BRANCH_NUMBER               : i32 = 81;
pub const MSK_IPAR_MIO_MAX_NUM_BRANCHES                  : i32 = 82;
pub const MSK_IPAR_MIO_MAX_NUM_RELAXS                    : i32 = 83;
pub const MSK_IPAR_MIO_MAX_NUM_SOLUTIONS                 : i32 = 84;
pub const MSK_IPAR_MIO_MODE                              : i32 = 85;
pub const MSK_IPAR_MIO_MT_USER_CB                        : i32 = 86;
pub const MSK_IPAR_MIO_NODE_OPTIMIZER                    : i32 = 87;
pub const MSK_IPAR_MIO_NODE_SELECTION                    : i32 = 88;
pub const MSK_IPAR_MIO_OPTIMIZER_MODE                    : i32 = 89;
pub const MSK_IPAR_MIO_PRESOLVE_AGGREGATE                : i32 = 90;
pub const MSK_IPAR_MIO_PROBING_LEVEL                     : i32 = 91;
pub const MSK_IPAR_MIO_RINS_MAX_NODES                    : i32 = 92;
pub const MSK_IPAR_MIO_ROOT_OPTIMIZER                    : i32 = 93;
pub const MSK_IPAR_MIO_ROOT_REPEAT_PRESOLVE_LEVEL        : i32 = 94;
pub const MSK_IPAR_MIO_STRONG_BRANCH                     : i32 = 95;
pub const MSK_IPAR_MT_SPINCOUNT                          : i32 = 96;
pub const MSK_IPAR_NUM_THREADS                           : i32 = 97;
pub const MSK_IPAR_OPF_MAX_TERMS_PER_LINE                : i32 = 98;
pub const MSK_IPAR_OPF_WRITE_HEADER                      : i32 = 99;
pub const MSK_IPAR_OPF_WRITE_HINTS                       : i32 = 100;
pub const MSK_IPAR_OPF_WRITE_PARAMETERS                  : i32 = 101;
pub const MSK_IPAR_OPF_WRITE_PROBLEM                     : i32 = 102;
pub const MSK_IPAR_OPF_WRITE_SOL_BAS                     : i32 = 103;
pub const MSK_IPAR_OPF_WRITE_SOL_ITG                     : i32 = 104;
pub const MSK_IPAR_OPF_WRITE_SOL_ITR                     : i32 = 105;
pub const MSK_IPAR_OPF_WRITE_SOLUTIONS                   : i32 = 106;
pub const MSK_IPAR_OPTIMIZER                             : i32 = 107;
pub const MSK_IPAR_PARAM_READ_CASE_NAME                  : i32 = 108;
pub const MSK_IPAR_PARAM_READ_IGN_ERROR                  : i32 = 109;
pub const MSK_IPAR_PRESOLVE_ELIMINATOR_MAX_FILL          : i32 = 110;
pub const MSK_IPAR_PRESOLVE_ELIMINATOR_MAX_NUM_TRIES     : i32 = 111;
pub const MSK_IPAR_PRESOLVE_LEVEL                        : i32 = 112;
pub const MSK_IPAR_PRESOLVE_LINDEP_ABS_WORK_TRH          : i32 = 113;
pub const MSK_IPAR_PRESOLVE_LINDEP_REL_WORK_TRH          : i32 = 114;
pub const MSK_IPAR_PRESOLVE_LINDEP_USE                   : i32 = 115;
pub const MSK_IPAR_PRESOLVE_MAX_NUM_REDUCTIONS           : i32 = 116;
pub const MSK_IPAR_PRESOLVE_USE                          : i32 = 117;
pub const MSK_IPAR_PRIMAL_REPAIR_OPTIMIZER               : i32 = 118;
pub const MSK_IPAR_QO_SEPARABLE_REFORMULATION            : i32 = 119;
pub const MSK_IPAR_READ_DATA_COMPRESSED                  : i32 = 120;
pub const MSK_IPAR_READ_DATA_FORMAT                      : i32 = 121;
pub const MSK_IPAR_READ_DEBUG                            : i32 = 122;
pub const MSK_IPAR_READ_KEEP_FREE_CON                    : i32 = 123;
pub const MSK_IPAR_READ_LP_DROP_NEW_VARS_IN_BOU          : i32 = 124;
pub const MSK_IPAR_READ_LP_QUOTED_NAMES                  : i32 = 125;
pub const MSK_IPAR_READ_MPS_FORMAT                       : i32 = 126;
pub const MSK_IPAR_READ_MPS_KEEP_INT                     : i32 = 127;
pub const MSK_IPAR_READ_MPS_RELAX                        : i32 = 128;
pub const MSK_IPAR_READ_MPS_WIDTH                        : i32 = 129;
pub const MSK_IPAR_READ_TASK_IGNORE_PARAM                : i32 = 130;
pub const MSK_IPAR_SENSITIVITY_ALL                       : i32 = 131;
pub const MSK_IPAR_SENSITIVITY_OPTIMIZER                 : i32 = 132;
pub const MSK_IPAR_SENSITIVITY_TYPE                      : i32 = 133;
pub const MSK_IPAR_SIM_BASIS_FACTOR_USE                  : i32 = 134;
pub const MSK_IPAR_SIM_DEGEN                             : i32 = 135;
pub const MSK_IPAR_SIM_DUAL_CRASH                        : i32 = 136;
pub const MSK_IPAR_SIM_DUAL_PHASEONE_METHOD              : i32 = 137;
pub const MSK_IPAR_SIM_DUAL_RESTRICT_SELECTION           : i32 = 138;
pub const MSK_IPAR_SIM_DUAL_SELECTION                    : i32 = 139;
pub const MSK_IPAR_SIM_EXPLOIT_DUPVEC                    : i32 = 140;
pub const MSK_IPAR_SIM_HOTSTART                          : i32 = 141;
pub const MSK_IPAR_SIM_HOTSTART_LU                       : i32 = 142;
pub const MSK_IPAR_SIM_INTEGER                           : i32 = 143;
pub const MSK_IPAR_SIM_MAX_ITERATIONS                    : i32 = 144;
pub const MSK_IPAR_SIM_MAX_NUM_SETBACKS                  : i32 = 145;
pub const MSK_IPAR_SIM_NON_SINGULAR                      : i32 = 146;
pub const MSK_IPAR_SIM_PRIMAL_CRASH                      : i32 = 147;
pub const MSK_IPAR_SIM_PRIMAL_PHASEONE_METHOD            : i32 = 148;
pub const MSK_IPAR_SIM_PRIMAL_RESTRICT_SELECTION         : i32 = 149;
pub const MSK_IPAR_SIM_PRIMAL_SELECTION                  : i32 = 150;
pub const MSK_IPAR_SIM_REFACTOR_FREQ                     : i32 = 151;
pub const MSK_IPAR_SIM_REFORMULATION                     : i32 = 152;
pub const MSK_IPAR_SIM_SAVE_LU                           : i32 = 153;
pub const MSK_IPAR_SIM_SCALING                           : i32 = 154;
pub const MSK_IPAR_SIM_SCALING_METHOD                    : i32 = 155;
pub const MSK_IPAR_SIM_SOLVE_FORM                        : i32 = 156;
pub const MSK_IPAR_SIM_STABILITY_PRIORITY                : i32 = 157;
pub const MSK_IPAR_SIM_SWITCH_OPTIMIZER                  : i32 = 158;
pub const MSK_IPAR_SOL_FILTER_KEEP_BASIC                 : i32 = 159;
pub const MSK_IPAR_SOL_FILTER_KEEP_RANGED                : i32 = 160;
pub const MSK_IPAR_SOL_READ_NAME_WIDTH                   : i32 = 161;
pub const MSK_IPAR_SOL_READ_WIDTH                        : i32 = 162;
pub const MSK_IPAR_SOLUTION_CALLBACK                     : i32 = 163;
pub const MSK_IPAR_TIMING_LEVEL                          : i32 = 164;
pub const MSK_IPAR_WRITE_BAS_CONSTRAINTS                 : i32 = 165;
pub const MSK_IPAR_WRITE_BAS_HEAD                        : i32 = 166;
pub const MSK_IPAR_WRITE_BAS_VARIABLES                   : i32 = 167;
pub const MSK_IPAR_WRITE_DATA_COMPRESSED                 : i32 = 168;
pub const MSK_IPAR_WRITE_DATA_FORMAT                     : i32 = 169;
pub const MSK_IPAR_WRITE_DATA_PARAM                      : i32 = 170;
pub const MSK_IPAR_WRITE_FREE_CON                        : i32 = 171;
pub const MSK_IPAR_WRITE_GENERIC_NAMES                   : i32 = 172;
pub const MSK_IPAR_WRITE_GENERIC_NAMES_IO                : i32 = 173;
pub const MSK_IPAR_WRITE_IGNORE_INCOMPATIBLE_CONIC_ITEMS : i32 = 174;
pub const MSK_IPAR_WRITE_IGNORE_INCOMPATIBLE_ITEMS       : i32 = 175;
pub const MSK_IPAR_WRITE_IGNORE_INCOMPATIBLE_NL_ITEMS    : i32 = 176;
pub const MSK_IPAR_WRITE_IGNORE_INCOMPATIBLE_PSD_ITEMS   : i32 = 177;
pub const MSK_IPAR_WRITE_INT_CONSTRAINTS                 : i32 = 178;
pub const MSK_IPAR_WRITE_INT_HEAD                        : i32 = 179;
pub const MSK_IPAR_WRITE_INT_VARIABLES                   : i32 = 180;
pub const MSK_IPAR_WRITE_LP_FULL_OBJ                     : i32 = 181;
pub const MSK_IPAR_WRITE_LP_LINE_WIDTH                   : i32 = 182;
pub const MSK_IPAR_WRITE_LP_QUOTED_NAMES                 : i32 = 183;
pub const MSK_IPAR_WRITE_LP_STRICT_FORMAT                : i32 = 184;
pub const MSK_IPAR_WRITE_LP_TERMS_PER_LINE               : i32 = 185;
pub const MSK_IPAR_WRITE_MPS_FORMAT                      : i32 = 186;
pub const MSK_IPAR_WRITE_MPS_INT                         : i32 = 187;
pub const MSK_IPAR_WRITE_PRECISION                       : i32 = 188;
pub const MSK_IPAR_WRITE_SOL_BARVARIABLES                : i32 = 189;
pub const MSK_IPAR_WRITE_SOL_CONSTRAINTS                 : i32 = 190;
pub const MSK_IPAR_WRITE_SOL_HEAD                        : i32 = 191;
pub const MSK_IPAR_WRITE_SOL_IGNORE_INVALID_NAMES        : i32 = 192;
pub const MSK_IPAR_WRITE_SOL_VARIABLES                   : i32 = 193;
pub const MSK_IPAR_WRITE_TASK_INC_SOL                    : i32 = 194;
pub const MSK_IPAR_WRITE_XML_MODE                        : i32 = 195;
pub const MSK_IPAR_BEGIN : i32 = 0;
pub const MSK_IPAR_END   : i32 = 196;

// language
pub const MSK_LANG_DAN : i32 = 1;
pub const MSK_LANG_ENG : i32 = 0;
pub const MSK_LANG_BEGIN : i32 = 0;
pub const MSK_LANG_END   : i32 = 2;

// liinfitem
pub const MSK_LIINF_BI_CLEAN_DUAL_DEG_ITER        : i32 = 0;
pub const MSK_LIINF_BI_CLEAN_DUAL_ITER            : i32 = 1;
pub const MSK_LIINF_BI_CLEAN_PRIMAL_DEG_ITER      : i32 = 2;
pub const MSK_LIINF_BI_CLEAN_PRIMAL_DUAL_DEG_ITER : i32 = 3;
pub const MSK_LIINF_BI_CLEAN_PRIMAL_DUAL_ITER     : i32 = 4;
pub const MSK_LIINF_BI_CLEAN_PRIMAL_DUAL_SUB_ITER : i32 = 5;
pub const MSK_LIINF_BI_CLEAN_PRIMAL_ITER          : i32 = 6;
pub const MSK_LIINF_BI_DUAL_ITER                  : i32 = 7;
pub const MSK_LIINF_BI_PRIMAL_ITER                : i32 = 8;
pub const MSK_LIINF_INTPNT_FACTOR_NUM_NZ          : i32 = 9;
pub const MSK_LIINF_MIO_INTPNT_ITER               : i32 = 10;
pub const MSK_LIINF_MIO_PRESOLVED_ANZ             : i32 = 11;
pub const MSK_LIINF_MIO_SIM_MAXITER_SETBACKS      : i32 = 12;
pub const MSK_LIINF_MIO_SIMPLEX_ITER              : i32 = 13;
pub const MSK_LIINF_RD_NUMANZ                     : i32 = 14;
pub const MSK_LIINF_RD_NUMQNZ                     : i32 = 15;
pub const MSK_LIINF_BEGIN : i32 = 0;
pub const MSK_LIINF_END   : i32 = 16;

// mark
pub const MSK_MARK_LO : i32 = 0;
pub const MSK_MARK_UP : i32 = 1;
pub const MSK_MARK_BEGIN : i32 = 0;
pub const MSK_MARK_END   : i32 = 2;

// miocontsoltype
pub const MSK_MIO_CONT_SOL_ITG     : i32 = 2;
pub const MSK_MIO_CONT_SOL_ITG_REL : i32 = 3;
pub const MSK_MIO_CONT_SOL_NONE    : i32 = 0;
pub const MSK_MIO_CONT_SOL_ROOT    : i32 = 1;
pub const MSK_MIO_CONT_SOL_BEGIN : i32 = 0;
pub const MSK_MIO_CONT_SOL_END   : i32 = 4;

// miomode
pub const MSK_MIO_MODE_IGNORED   : i32 = 0;
pub const MSK_MIO_MODE_SATISFIED : i32 = 1;
pub const MSK_MIO_MODE_BEGIN : i32 = 0;
pub const MSK_MIO_MODE_END   : i32 = 2;

// mionodeseltype
pub const MSK_MIO_NODE_SELECTION_BEST   : i32 = 2;
pub const MSK_MIO_NODE_SELECTION_FIRST  : i32 = 1;
pub const MSK_MIO_NODE_SELECTION_FREE   : i32 = 0;
pub const MSK_MIO_NODE_SELECTION_HYBRID : i32 = 4;
pub const MSK_MIO_NODE_SELECTION_PSEUDO : i32 = 5;
pub const MSK_MIO_NODE_SELECTION_WORST  : i32 = 3;
pub const MSK_MIO_NODE_SELECTION_BEGIN : i32 = 0;
pub const MSK_MIO_NODE_SELECTION_END   : i32 = 6;

// mpsformat
pub const MSK_MPS_FORMAT_CPLEX   : i32 = 3;
pub const MSK_MPS_FORMAT_FREE    : i32 = 2;
pub const MSK_MPS_FORMAT_RELAXED : i32 = 1;
pub const MSK_MPS_FORMAT_STRICT  : i32 = 0;
pub const MSK_MPS_FORMAT_BEGIN : i32 = 0;
pub const MSK_MPS_FORMAT_END   : i32 = 4;

// msgkey
pub const MSK_MSG_MPS_SELECTED : i32 = 1100;
pub const MSK_MSG_READING_FILE : i32 = 1000;
pub const MSK_MSG_WRITING_FILE : i32 = 1001;

// nametype
pub const MSK_NAME_TYPE_GEN : i32 = 0;
pub const MSK_NAME_TYPE_LP  : i32 = 2;
pub const MSK_NAME_TYPE_MPS : i32 = 1;
pub const MSK_NAME_TYPE_BEGIN : i32 = 0;
pub const MSK_NAME_TYPE_END   : i32 = 3;

// objsense
pub const MSK_OBJECTIVE_SENSE_MAXIMIZE : i32 = 1;
pub const MSK_OBJECTIVE_SENSE_MINIMIZE : i32 = 0;
pub const MSK_OBJECTIVE_SENSE_BEGIN : i32 = 0;
pub const MSK_OBJECTIVE_SENSE_END   : i32 = 2;

// onoffkey
pub const MSK_OFF : i32 = 0;
pub const MSK_ON  : i32 = 1;

// optimizertype
pub const MSK_OPTIMIZER_CONCURRENT             : i32 = 10;
pub const MSK_OPTIMIZER_CONIC                  : i32 = 2;
pub const MSK_OPTIMIZER_DUAL_SIMPLEX           : i32 = 4;
pub const MSK_OPTIMIZER_FREE                   : i32 = 0;
pub const MSK_OPTIMIZER_FREE_SIMPLEX           : i32 = 6;
pub const MSK_OPTIMIZER_INTPNT                 : i32 = 1;
pub const MSK_OPTIMIZER_MIXED_INT              : i32 = 9;
pub const MSK_OPTIMIZER_MIXED_INT_CONIC        : i32 = 8;
pub const MSK_OPTIMIZER_NETWORK_PRIMAL_SIMPLEX : i32 = 7;
pub const MSK_OPTIMIZER_PRIMAL_DUAL_SIMPLEX    : i32 = 5;
pub const MSK_OPTIMIZER_PRIMAL_SIMPLEX         : i32 = 3;
pub const MSK_OPTIMIZER_BEGIN : i32 = 0;
pub const MSK_OPTIMIZER_END   : i32 = 11;

// orderingtype
pub const MSK_ORDER_METHOD_APPMINLOC      : i32 = 1;
pub const MSK_ORDER_METHOD_EXPERIMENTAL   : i32 = 2;
pub const MSK_ORDER_METHOD_FORCE_GRAPHPAR : i32 = 4;
pub const MSK_ORDER_METHOD_FREE           : i32 = 0;
pub const MSK_ORDER_METHOD_NONE           : i32 = 5;
pub const MSK_ORDER_METHOD_TRY_GRAPHPAR   : i32 = 3;
pub const MSK_ORDER_METHOD_BEGIN : i32 = 0;
pub const MSK_ORDER_METHOD_END   : i32 = 6;

// parametertype
pub const MSK_PAR_DOU_TYPE     : i32 = 1;
pub const MSK_PAR_INT_TYPE     : i32 = 2;
pub const MSK_PAR_INVALID_TYPE : i32 = 0;
pub const MSK_PAR_STR_TYPE     : i32 = 3;
pub const MSK_PAR_BEGIN : i32 = 0;
pub const MSK_PAR_END   : i32 = 4;

// presolvemode
pub const MSK_PRESOLVE_MODE_FREE : i32 = 2;
pub const MSK_PRESOLVE_MODE_OFF  : i32 = 0;
pub const MSK_PRESOLVE_MODE_ON   : i32 = 1;
pub const MSK_PRESOLVE_MODE_BEGIN : i32 = 0;
pub const MSK_PRESOLVE_MODE_END   : i32 = 3;

// problemitem
pub const MSK_PI_CON  : i32 = 1;
pub const MSK_PI_CONE : i32 = 2;
pub const MSK_PI_VAR  : i32 = 0;
pub const MSK_PI_BEGIN : i32 = 0;
pub const MSK_PI_END   : i32 = 3;

// problemtype
pub const MSK_PROBTYPE_CONIC : i32 = 4;
pub const MSK_PROBTYPE_GECO  : i32 = 3;
pub const MSK_PROBTYPE_LO    : i32 = 0;
pub const MSK_PROBTYPE_MIXED : i32 = 5;
pub const MSK_PROBTYPE_QCQO  : i32 = 2;
pub const MSK_PROBTYPE_QO    : i32 = 1;
pub const MSK_PROBTYPE_BEGIN : i32 = 0;
pub const MSK_PROBTYPE_END   : i32 = 6;

// prosta
pub const MSK_PRO_STA_DUAL_FEAS                : i32 = 3;
pub const MSK_PRO_STA_DUAL_INFEAS              : i32 = 5;
pub const MSK_PRO_STA_ILL_POSED                : i32 = 7;
pub const MSK_PRO_STA_NEAR_DUAL_FEAS           : i32 = 10;
pub const MSK_PRO_STA_NEAR_PRIM_AND_DUAL_FEAS  : i32 = 8;
pub const MSK_PRO_STA_NEAR_PRIM_FEAS           : i32 = 9;
pub const MSK_PRO_STA_PRIM_AND_DUAL_FEAS       : i32 = 1;
pub const MSK_PRO_STA_PRIM_AND_DUAL_INFEAS     : i32 = 6;
pub const MSK_PRO_STA_PRIM_FEAS                : i32 = 2;
pub const MSK_PRO_STA_PRIM_INFEAS              : i32 = 4;
pub const MSK_PRO_STA_PRIM_INFEAS_OR_UNBOUNDED : i32 = 11;
pub const MSK_PRO_STA_UNKNOWN                  : i32 = 0;
pub const MSK_PRO_STA_BEGIN : i32 = 0;
pub const MSK_PRO_STA_END   : i32 = 12;

// rescode
pub const MSK_RES_ERR_AD_INVALID_CODELIST                 : i32 = 3102;
pub const MSK_RES_ERR_AD_INVALID_OPERAND                  : i32 = 3104;
pub const MSK_RES_ERR_AD_INVALID_OPERATOR                 : i32 = 3103;
pub const MSK_RES_ERR_AD_MISSING_OPERAND                  : i32 = 3105;
pub const MSK_RES_ERR_AD_MISSING_RETURN                   : i32 = 3106;
pub const MSK_RES_ERR_API_ARRAY_TOO_SMALL                 : i32 = 3001;
pub const MSK_RES_ERR_API_CB_CONNECT                      : i32 = 3002;
pub const MSK_RES_ERR_API_FATAL_ERROR                     : i32 = 3005;
pub const MSK_RES_ERR_API_INTERNAL                        : i32 = 3999;
pub const MSK_RES_ERR_ARG_IS_TOO_LARGE                    : i32 = 1227;
pub const MSK_RES_ERR_ARG_IS_TOO_SMALL                    : i32 = 1226;
pub const MSK_RES_ERR_ARGUMENT_DIMENSION                  : i32 = 1201;
pub const MSK_RES_ERR_ARGUMENT_IS_TOO_LARGE               : i32 = 5005;
pub const MSK_RES_ERR_ARGUMENT_LENNEQ                     : i32 = 1197;
pub const MSK_RES_ERR_ARGUMENT_PERM_ARRAY                 : i32 = 1299;
pub const MSK_RES_ERR_ARGUMENT_TYPE                       : i32 = 1198;
pub const MSK_RES_ERR_BAR_VAR_DIM                         : i32 = 3920;
pub const MSK_RES_ERR_BASIS                               : i32 = 1266;
pub const MSK_RES_ERR_BASIS_FACTOR                        : i32 = 1610;
pub const MSK_RES_ERR_BASIS_SINGULAR                      : i32 = 1615;
pub const MSK_RES_ERR_BLANK_NAME                          : i32 = 1070;
pub const MSK_RES_ERR_CANNOT_CLONE_NL                     : i32 = 2505;
pub const MSK_RES_ERR_CANNOT_HANDLE_NL                    : i32 = 2506;
pub const MSK_RES_ERR_CBF_DUPLICATE_ACOORD                : i32 = 7116;
pub const MSK_RES_ERR_CBF_DUPLICATE_BCOORD                : i32 = 7115;
pub const MSK_RES_ERR_CBF_DUPLICATE_CON                   : i32 = 7108;
pub const MSK_RES_ERR_CBF_DUPLICATE_INT                   : i32 = 7110;
pub const MSK_RES_ERR_CBF_DUPLICATE_OBJ                   : i32 = 7107;
pub const MSK_RES_ERR_CBF_DUPLICATE_OBJACOORD             : i32 = 7114;
pub const MSK_RES_ERR_CBF_DUPLICATE_VAR                   : i32 = 7109;
pub const MSK_RES_ERR_CBF_INVALID_CON_TYPE                : i32 = 7112;
pub const MSK_RES_ERR_CBF_INVALID_DOMAIN_DIMENSION        : i32 = 7113;
pub const MSK_RES_ERR_CBF_INVALID_INT_INDEX               : i32 = 7121;
pub const MSK_RES_ERR_CBF_INVALID_VAR_TYPE                : i32 = 7111;
pub const MSK_RES_ERR_CBF_NO_VARIABLES                    : i32 = 7102;
pub const MSK_RES_ERR_CBF_NO_VERSION_SPECIFIED            : i32 = 7105;
pub const MSK_RES_ERR_CBF_OBJ_SENSE                       : i32 = 7101;
pub const MSK_RES_ERR_CBF_PARSE                           : i32 = 7100;
pub const MSK_RES_ERR_CBF_SYNTAX                          : i32 = 7106;
pub const MSK_RES_ERR_CBF_TOO_FEW_CONSTRAINTS             : i32 = 7118;
pub const MSK_RES_ERR_CBF_TOO_FEW_INTS                    : i32 = 7119;
pub const MSK_RES_ERR_CBF_TOO_FEW_VARIABLES               : i32 = 7117;
pub const MSK_RES_ERR_CBF_TOO_MANY_CONSTRAINTS            : i32 = 7103;
pub const MSK_RES_ERR_CBF_TOO_MANY_INTS                   : i32 = 7120;
pub const MSK_RES_ERR_CBF_TOO_MANY_VARIABLES              : i32 = 7104;
pub const MSK_RES_ERR_CBF_UNSUPPORTED                     : i32 = 7122;
pub const MSK_RES_ERR_CON_Q_NOT_NSD                       : i32 = 1294;
pub const MSK_RES_ERR_CON_Q_NOT_PSD                       : i32 = 1293;
pub const MSK_RES_ERR_CONCURRENT_OPTIMIZER                : i32 = 3059;
pub const MSK_RES_ERR_CONE_INDEX                          : i32 = 1300;
pub const MSK_RES_ERR_CONE_OVERLAP                        : i32 = 1302;
pub const MSK_RES_ERR_CONE_OVERLAP_APPEND                 : i32 = 1307;
pub const MSK_RES_ERR_CONE_REP_VAR                        : i32 = 1303;
pub const MSK_RES_ERR_CONE_SIZE                           : i32 = 1301;
pub const MSK_RES_ERR_CONE_TYPE                           : i32 = 1305;
pub const MSK_RES_ERR_CONE_TYPE_STR                       : i32 = 1306;
pub const MSK_RES_ERR_DATA_FILE_EXT                       : i32 = 1055;
pub const MSK_RES_ERR_DUP_NAME                            : i32 = 1071;
pub const MSK_RES_ERR_DUPLICATE_AIJ                       : i32 = 1385;
pub const MSK_RES_ERR_DUPLICATE_BARVARIABLE_NAMES         : i32 = 4502;
pub const MSK_RES_ERR_DUPLICATE_CONE_NAMES                : i32 = 4503;
pub const MSK_RES_ERR_DUPLICATE_CONSTRAINT_NAMES          : i32 = 4500;
pub const MSK_RES_ERR_DUPLICATE_VARIABLE_NAMES            : i32 = 4501;
pub const MSK_RES_ERR_END_OF_FILE                         : i32 = 1059;
pub const MSK_RES_ERR_FACTOR                              : i32 = 1650;
pub const MSK_RES_ERR_FEASREPAIR_CANNOT_RELAX             : i32 = 1700;
pub const MSK_RES_ERR_FEASREPAIR_INCONSISTENT_BOUND       : i32 = 1702;
pub const MSK_RES_ERR_FEASREPAIR_SOLVING_RELAXED          : i32 = 1701;
pub const MSK_RES_ERR_FILE_LICENSE                        : i32 = 1007;
pub const MSK_RES_ERR_FILE_OPEN                           : i32 = 1052;
pub const MSK_RES_ERR_FILE_READ                           : i32 = 1053;
pub const MSK_RES_ERR_FILE_WRITE                          : i32 = 1054;
pub const MSK_RES_ERR_FIRST                               : i32 = 1261;
pub const MSK_RES_ERR_FIRSTI                              : i32 = 1285;
pub const MSK_RES_ERR_FIRSTJ                              : i32 = 1287;
pub const MSK_RES_ERR_FIXED_BOUND_VALUES                  : i32 = 1425;
pub const MSK_RES_ERR_FLEXLM                              : i32 = 1014;
pub const MSK_RES_ERR_GLOBAL_INV_CONIC_PROBLEM            : i32 = 1503;
pub const MSK_RES_ERR_HUGE_AIJ                            : i32 = 1380;
pub const MSK_RES_ERR_HUGE_C                              : i32 = 1375;
pub const MSK_RES_ERR_IDENTICAL_TASKS                     : i32 = 3101;
pub const MSK_RES_ERR_IN_ARGUMENT                         : i32 = 1200;
pub const MSK_RES_ERR_INDEX                               : i32 = 1235;
pub const MSK_RES_ERR_INDEX_ARR_IS_TOO_LARGE              : i32 = 1222;
pub const MSK_RES_ERR_INDEX_ARR_IS_TOO_SMALL              : i32 = 1221;
pub const MSK_RES_ERR_INDEX_IS_TOO_LARGE                  : i32 = 1204;
pub const MSK_RES_ERR_INDEX_IS_TOO_SMALL                  : i32 = 1203;
pub const MSK_RES_ERR_INF_DOU_INDEX                       : i32 = 1219;
pub const MSK_RES_ERR_INF_DOU_NAME                        : i32 = 1230;
pub const MSK_RES_ERR_INF_INT_INDEX                       : i32 = 1220;
pub const MSK_RES_ERR_INF_INT_NAME                        : i32 = 1231;
pub const MSK_RES_ERR_INF_LINT_INDEX                      : i32 = 1225;
pub const MSK_RES_ERR_INF_LINT_NAME                       : i32 = 1234;
pub const MSK_RES_ERR_INF_TYPE                            : i32 = 1232;
pub const MSK_RES_ERR_INFEAS_UNDEFINED                    : i32 = 3910;
pub const MSK_RES_ERR_INFINITE_BOUND                      : i32 = 1400;
pub const MSK_RES_ERR_INT64_TO_INT32_CAST                 : i32 = 3800;
pub const MSK_RES_ERR_INTERNAL                            : i32 = 3000;
pub const MSK_RES_ERR_INTERNAL_TEST_FAILED                : i32 = 3500;
pub const MSK_RES_ERR_INV_APTRE                           : i32 = 1253;
pub const MSK_RES_ERR_INV_BK                              : i32 = 1255;
pub const MSK_RES_ERR_INV_BKC                             : i32 = 1256;
pub const MSK_RES_ERR_INV_BKX                             : i32 = 1257;
pub const MSK_RES_ERR_INV_CONE_TYPE                       : i32 = 1272;
pub const MSK_RES_ERR_INV_CONE_TYPE_STR                   : i32 = 1271;
pub const MSK_RES_ERR_INV_MARKI                           : i32 = 2501;
pub const MSK_RES_ERR_INV_MARKJ                           : i32 = 2502;
pub const MSK_RES_ERR_INV_NAME_ITEM                       : i32 = 1280;
pub const MSK_RES_ERR_INV_NUMI                            : i32 = 2503;
pub const MSK_RES_ERR_INV_NUMJ                            : i32 = 2504;
pub const MSK_RES_ERR_INV_OPTIMIZER                       : i32 = 1550;
pub const MSK_RES_ERR_INV_PROBLEM                         : i32 = 1500;
pub const MSK_RES_ERR_INV_QCON_SUBI                       : i32 = 1405;
pub const MSK_RES_ERR_INV_QCON_SUBJ                       : i32 = 1406;
pub const MSK_RES_ERR_INV_QCON_SUBK                       : i32 = 1404;
pub const MSK_RES_ERR_INV_QCON_VAL                        : i32 = 1407;
pub const MSK_RES_ERR_INV_QOBJ_SUBI                       : i32 = 1401;
pub const MSK_RES_ERR_INV_QOBJ_SUBJ                       : i32 = 1402;
pub const MSK_RES_ERR_INV_QOBJ_VAL                        : i32 = 1403;
pub const MSK_RES_ERR_INV_SK                              : i32 = 1270;
pub const MSK_RES_ERR_INV_SK_STR                          : i32 = 1269;
pub const MSK_RES_ERR_INV_SKC                             : i32 = 1267;
pub const MSK_RES_ERR_INV_SKN                             : i32 = 1274;
pub const MSK_RES_ERR_INV_SKX                             : i32 = 1268;
pub const MSK_RES_ERR_INV_VAR_TYPE                        : i32 = 1258;
pub const MSK_RES_ERR_INVALID_ACCMODE                     : i32 = 2520;
pub const MSK_RES_ERR_INVALID_AIJ                         : i32 = 1473;
pub const MSK_RES_ERR_INVALID_AMPL_STUB                   : i32 = 3700;
pub const MSK_RES_ERR_INVALID_BARVAR_NAME                 : i32 = 1079;
pub const MSK_RES_ERR_INVALID_BRANCH_DIRECTION            : i32 = 3200;
pub const MSK_RES_ERR_INVALID_BRANCH_PRIORITY             : i32 = 3201;
pub const MSK_RES_ERR_INVALID_COMPRESSION                 : i32 = 1800;
pub const MSK_RES_ERR_INVALID_CON_NAME                    : i32 = 1076;
pub const MSK_RES_ERR_INVALID_CONE_NAME                   : i32 = 1078;
pub const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_CONES       : i32 = 4005;
pub const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_GENERAL_NL  : i32 = 4010;
pub const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_SYM_MAT     : i32 = 4000;
pub const MSK_RES_ERR_INVALID_FILE_NAME                   : i32 = 1056;
pub const MSK_RES_ERR_INVALID_FORMAT_TYPE                 : i32 = 1283;
pub const MSK_RES_ERR_INVALID_IDX                         : i32 = 1246;
pub const MSK_RES_ERR_INVALID_IOMODE                      : i32 = 1801;
pub const MSK_RES_ERR_INVALID_MAX_NUM                     : i32 = 1247;
pub const MSK_RES_ERR_INVALID_NAME_IN_SOL_FILE            : i32 = 1170;
pub const MSK_RES_ERR_INVALID_NETWORK_PROBLEM             : i32 = 1504;
pub const MSK_RES_ERR_INVALID_OBJ_NAME                    : i32 = 1075;
pub const MSK_RES_ERR_INVALID_OBJECTIVE_SENSE             : i32 = 1445;
pub const MSK_RES_ERR_INVALID_PROBLEM_TYPE                : i32 = 6000;
pub const MSK_RES_ERR_INVALID_SOL_FILE_NAME               : i32 = 1057;
pub const MSK_RES_ERR_INVALID_STREAM                      : i32 = 1062;
pub const MSK_RES_ERR_INVALID_SURPLUS                     : i32 = 1275;
pub const MSK_RES_ERR_INVALID_SYM_MAT_DIM                 : i32 = 3950;
pub const MSK_RES_ERR_INVALID_TASK                        : i32 = 1064;
pub const MSK_RES_ERR_INVALID_UTF8                        : i32 = 2900;
pub const MSK_RES_ERR_INVALID_VAR_NAME                    : i32 = 1077;
pub const MSK_RES_ERR_INVALID_WCHAR                       : i32 = 2901;
pub const MSK_RES_ERR_INVALID_WHICHSOL                    : i32 = 1228;
pub const MSK_RES_ERR_LAST                                : i32 = 1262;
pub const MSK_RES_ERR_LASTI                               : i32 = 1286;
pub const MSK_RES_ERR_LASTJ                               : i32 = 1288;
pub const MSK_RES_ERR_LAU_ARG_K                           : i32 = 7012;
pub const MSK_RES_ERR_LAU_ARG_M                           : i32 = 7010;
pub const MSK_RES_ERR_LAU_ARG_N                           : i32 = 7011;
pub const MSK_RES_ERR_LAU_ARG_TRANS                       : i32 = 7018;
pub const MSK_RES_ERR_LAU_ARG_TRANSA                      : i32 = 7015;
pub const MSK_RES_ERR_LAU_ARG_TRANSB                      : i32 = 7016;
pub const MSK_RES_ERR_LAU_ARG_UPLO                        : i32 = 7017;
pub const MSK_RES_ERR_LAU_NOT_POSITIVE_DEFINITE           : i32 = 7001;
pub const MSK_RES_ERR_LAU_SINGULAR_MATRIX                 : i32 = 7000;
pub const MSK_RES_ERR_LAU_UNKNOWN                         : i32 = 7005;
pub const MSK_RES_ERR_LICENSE                             : i32 = 1000;
pub const MSK_RES_ERR_LICENSE_CANNOT_ALLOCATE             : i32 = 1020;
pub const MSK_RES_ERR_LICENSE_CANNOT_CONNECT              : i32 = 1021;
pub const MSK_RES_ERR_LICENSE_EXPIRED                     : i32 = 1001;
pub const MSK_RES_ERR_LICENSE_FEATURE                     : i32 = 1018;
pub const MSK_RES_ERR_LICENSE_INVALID_HOSTID              : i32 = 1025;
pub const MSK_RES_ERR_LICENSE_MAX                         : i32 = 1016;
pub const MSK_RES_ERR_LICENSE_MOSEKLM_DAEMON              : i32 = 1017;
pub const MSK_RES_ERR_LICENSE_NO_SERVER_LINE              : i32 = 1028;
pub const MSK_RES_ERR_LICENSE_NO_SERVER_SUPPORT           : i32 = 1027;
pub const MSK_RES_ERR_LICENSE_SERVER                      : i32 = 1015;
pub const MSK_RES_ERR_LICENSE_SERVER_VERSION              : i32 = 1026;
pub const MSK_RES_ERR_LICENSE_VERSION                     : i32 = 1002;
pub const MSK_RES_ERR_LINK_FILE_DLL                       : i32 = 1040;
pub const MSK_RES_ERR_LIVING_TASKS                        : i32 = 1066;
pub const MSK_RES_ERR_LOWER_BOUND_IS_A_NAN                : i32 = 1390;
pub const MSK_RES_ERR_LP_DUP_SLACK_NAME                   : i32 = 1152;
pub const MSK_RES_ERR_LP_EMPTY                            : i32 = 1151;
pub const MSK_RES_ERR_LP_FILE_FORMAT                      : i32 = 1157;
pub const MSK_RES_ERR_LP_FORMAT                           : i32 = 1160;
pub const MSK_RES_ERR_LP_FREE_CONSTRAINT                  : i32 = 1155;
pub const MSK_RES_ERR_LP_INCOMPATIBLE                     : i32 = 1150;
pub const MSK_RES_ERR_LP_INVALID_CON_NAME                 : i32 = 1171;
pub const MSK_RES_ERR_LP_INVALID_VAR_NAME                 : i32 = 1154;
pub const MSK_RES_ERR_LP_WRITE_CONIC_PROBLEM              : i32 = 1163;
pub const MSK_RES_ERR_LP_WRITE_GECO_PROBLEM               : i32 = 1164;
pub const MSK_RES_ERR_LU_MAX_NUM_TRIES                    : i32 = 2800;
pub const MSK_RES_ERR_MAX_LEN_IS_TOO_SMALL                : i32 = 1289;
pub const MSK_RES_ERR_MAXNUMBARVAR                        : i32 = 1242;
pub const MSK_RES_ERR_MAXNUMCON                           : i32 = 1240;
pub const MSK_RES_ERR_MAXNUMCONE                          : i32 = 1304;
pub const MSK_RES_ERR_MAXNUMQNZ                           : i32 = 1243;
pub const MSK_RES_ERR_MAXNUMVAR                           : i32 = 1241;
pub const MSK_RES_ERR_MIO_INTERNAL                        : i32 = 5010;
pub const MSK_RES_ERR_MIO_INVALID_NODE_OPTIMIZER          : i32 = 7131;
pub const MSK_RES_ERR_MIO_INVALID_ROOT_OPTIMIZER          : i32 = 7130;
pub const MSK_RES_ERR_MIO_NO_OPTIMIZER                    : i32 = 1551;
pub const MSK_RES_ERR_MIO_NOT_LOADED                      : i32 = 1553;
pub const MSK_RES_ERR_MISSING_LICENSE_FILE                : i32 = 1008;
pub const MSK_RES_ERR_MIXED_CONIC_AND_NL                  : i32 = 1501;
pub const MSK_RES_ERR_MPS_CONE_OVERLAP                    : i32 = 1118;
pub const MSK_RES_ERR_MPS_CONE_REPEAT                     : i32 = 1119;
pub const MSK_RES_ERR_MPS_CONE_TYPE                       : i32 = 1117;
pub const MSK_RES_ERR_MPS_DUPLICATE_Q_ELEMENT             : i32 = 1121;
pub const MSK_RES_ERR_MPS_FILE                            : i32 = 1100;
pub const MSK_RES_ERR_MPS_INV_BOUND_KEY                   : i32 = 1108;
pub const MSK_RES_ERR_MPS_INV_CON_KEY                     : i32 = 1107;
pub const MSK_RES_ERR_MPS_INV_FIELD                       : i32 = 1101;
pub const MSK_RES_ERR_MPS_INV_MARKER                      : i32 = 1102;
pub const MSK_RES_ERR_MPS_INV_SEC_NAME                    : i32 = 1109;
pub const MSK_RES_ERR_MPS_INV_SEC_ORDER                   : i32 = 1115;
pub const MSK_RES_ERR_MPS_INVALID_OBJ_NAME                : i32 = 1128;
pub const MSK_RES_ERR_MPS_INVALID_OBJSENSE                : i32 = 1122;
pub const MSK_RES_ERR_MPS_MUL_CON_NAME                    : i32 = 1112;
pub const MSK_RES_ERR_MPS_MUL_CSEC                        : i32 = 1116;
pub const MSK_RES_ERR_MPS_MUL_QOBJ                        : i32 = 1114;
pub const MSK_RES_ERR_MPS_MUL_QSEC                        : i32 = 1113;
pub const MSK_RES_ERR_MPS_NO_OBJECTIVE                    : i32 = 1110;
pub const MSK_RES_ERR_MPS_NON_SYMMETRIC_Q                 : i32 = 1120;
pub const MSK_RES_ERR_MPS_NULL_CON_NAME                   : i32 = 1103;
pub const MSK_RES_ERR_MPS_NULL_VAR_NAME                   : i32 = 1104;
pub const MSK_RES_ERR_MPS_SPLITTED_VAR                    : i32 = 1111;
pub const MSK_RES_ERR_MPS_TAB_IN_FIELD2                   : i32 = 1125;
pub const MSK_RES_ERR_MPS_TAB_IN_FIELD3                   : i32 = 1126;
pub const MSK_RES_ERR_MPS_TAB_IN_FIELD5                   : i32 = 1127;
pub const MSK_RES_ERR_MPS_UNDEF_CON_NAME                  : i32 = 1105;
pub const MSK_RES_ERR_MPS_UNDEF_VAR_NAME                  : i32 = 1106;
pub const MSK_RES_ERR_MUL_A_ELEMENT                       : i32 = 1254;
pub const MSK_RES_ERR_NAME_IS_NULL                        : i32 = 1760;
pub const MSK_RES_ERR_NAME_MAX_LEN                        : i32 = 1750;
pub const MSK_RES_ERR_NAN_IN_BLC                          : i32 = 1461;
pub const MSK_RES_ERR_NAN_IN_BLX                          : i32 = 1471;
pub const MSK_RES_ERR_NAN_IN_BUC                          : i32 = 1462;
pub const MSK_RES_ERR_NAN_IN_BUX                          : i32 = 1472;
pub const MSK_RES_ERR_NAN_IN_C                            : i32 = 1470;
pub const MSK_RES_ERR_NAN_IN_DOUBLE_DATA                  : i32 = 1450;
pub const MSK_RES_ERR_NEGATIVE_APPEND                     : i32 = 1264;
pub const MSK_RES_ERR_NEGATIVE_SURPLUS                    : i32 = 1263;
pub const MSK_RES_ERR_NEWER_DLL                           : i32 = 1036;
pub const MSK_RES_ERR_NO_BARS_FOR_SOLUTION                : i32 = 3916;
pub const MSK_RES_ERR_NO_BARX_FOR_SOLUTION                : i32 = 3915;
pub const MSK_RES_ERR_NO_BASIS_SOL                        : i32 = 1600;
pub const MSK_RES_ERR_NO_DUAL_FOR_ITG_SOL                 : i32 = 2950;
pub const MSK_RES_ERR_NO_DUAL_INFEAS_CER                  : i32 = 2001;
pub const MSK_RES_ERR_NO_INIT_ENV                         : i32 = 1063;
pub const MSK_RES_ERR_NO_OPTIMIZER_VAR_TYPE               : i32 = 1552;
pub const MSK_RES_ERR_NO_PRIMAL_INFEAS_CER                : i32 = 2000;
pub const MSK_RES_ERR_NO_SNX_FOR_BAS_SOL                  : i32 = 2953;
pub const MSK_RES_ERR_NO_SOLUTION_IN_CALLBACK             : i32 = 2500;
pub const MSK_RES_ERR_NON_UNIQUE_ARRAY                    : i32 = 5000;
pub const MSK_RES_ERR_NONCONVEX                           : i32 = 1291;
pub const MSK_RES_ERR_NONLINEAR_EQUALITY                  : i32 = 1290;
pub const MSK_RES_ERR_NONLINEAR_FUNCTIONS_NOT_ALLOWED     : i32 = 1428;
pub const MSK_RES_ERR_NONLINEAR_RANGED                    : i32 = 1292;
pub const MSK_RES_ERR_NR_ARGUMENTS                        : i32 = 1199;
pub const MSK_RES_ERR_NULL_ENV                            : i32 = 1060;
pub const MSK_RES_ERR_NULL_POINTER                        : i32 = 1065;
pub const MSK_RES_ERR_NULL_TASK                           : i32 = 1061;
pub const MSK_RES_ERR_NUMCONLIM                           : i32 = 1250;
pub const MSK_RES_ERR_NUMVARLIM                           : i32 = 1251;
pub const MSK_RES_ERR_OBJ_Q_NOT_NSD                       : i32 = 1296;
pub const MSK_RES_ERR_OBJ_Q_NOT_PSD                       : i32 = 1295;
pub const MSK_RES_ERR_OBJECTIVE_RANGE                     : i32 = 1260;
pub const MSK_RES_ERR_OLDER_DLL                           : i32 = 1035;
pub const MSK_RES_ERR_OPEN_DL                             : i32 = 1030;
pub const MSK_RES_ERR_OPF_FORMAT                          : i32 = 1168;
pub const MSK_RES_ERR_OPF_NEW_VARIABLE                    : i32 = 1169;
pub const MSK_RES_ERR_OPF_PREMATURE_EOF                   : i32 = 1172;
pub const MSK_RES_ERR_OPTIMIZER_LICENSE                   : i32 = 1013;
pub const MSK_RES_ERR_ORD_INVALID                         : i32 = 1131;
pub const MSK_RES_ERR_ORD_INVALID_BRANCH_DIR              : i32 = 1130;
pub const MSK_RES_ERR_OVERFLOW                            : i32 = 1590;
pub const MSK_RES_ERR_PARAM_INDEX                         : i32 = 1210;
pub const MSK_RES_ERR_PARAM_IS_TOO_LARGE                  : i32 = 1215;
pub const MSK_RES_ERR_PARAM_IS_TOO_SMALL                  : i32 = 1216;
pub const MSK_RES_ERR_PARAM_NAME                          : i32 = 1205;
pub const MSK_RES_ERR_PARAM_NAME_DOU                      : i32 = 1206;
pub const MSK_RES_ERR_PARAM_NAME_INT                      : i32 = 1207;
pub const MSK_RES_ERR_PARAM_NAME_STR                      : i32 = 1208;
pub const MSK_RES_ERR_PARAM_TYPE                          : i32 = 1218;
pub const MSK_RES_ERR_PARAM_VALUE_STR                     : i32 = 1217;
pub const MSK_RES_ERR_PLATFORM_NOT_LICENSED               : i32 = 1019;
pub const MSK_RES_ERR_POSTSOLVE                           : i32 = 1580;
pub const MSK_RES_ERR_PRO_ITEM                            : i32 = 1281;
pub const MSK_RES_ERR_PROB_LICENSE                        : i32 = 1006;
pub const MSK_RES_ERR_QCON_SUBI_TOO_LARGE                 : i32 = 1409;
pub const MSK_RES_ERR_QCON_SUBI_TOO_SMALL                 : i32 = 1408;
pub const MSK_RES_ERR_QCON_UPPER_TRIANGLE                 : i32 = 1417;
pub const MSK_RES_ERR_QOBJ_UPPER_TRIANGLE                 : i32 = 1415;
pub const MSK_RES_ERR_READ_FORMAT                         : i32 = 1090;
pub const MSK_RES_ERR_READ_LP_MISSING_END_TAG             : i32 = 1159;
pub const MSK_RES_ERR_READ_LP_NONEXISTING_NAME            : i32 = 1162;
pub const MSK_RES_ERR_REMOVE_CONE_VARIABLE                : i32 = 1310;
pub const MSK_RES_ERR_REPAIR_INVALID_PROBLEM              : i32 = 1710;
pub const MSK_RES_ERR_REPAIR_OPTIMIZATION_FAILED          : i32 = 1711;
pub const MSK_RES_ERR_SEN_BOUND_INVALID_LO                : i32 = 3054;
pub const MSK_RES_ERR_SEN_BOUND_INVALID_UP                : i32 = 3053;
pub const MSK_RES_ERR_SEN_FORMAT                          : i32 = 3050;
pub const MSK_RES_ERR_SEN_INDEX_INVALID                   : i32 = 3055;
pub const MSK_RES_ERR_SEN_INDEX_RANGE                     : i32 = 3052;
pub const MSK_RES_ERR_SEN_INVALID_REGEXP                  : i32 = 3056;
pub const MSK_RES_ERR_SEN_NUMERICAL                       : i32 = 3058;
pub const MSK_RES_ERR_SEN_SOLUTION_STATUS                 : i32 = 3057;
pub const MSK_RES_ERR_SEN_UNDEF_NAME                      : i32 = 3051;
pub const MSK_RES_ERR_SEN_UNHANDLED_PROBLEM_TYPE          : i32 = 3080;
pub const MSK_RES_ERR_SIZE_LICENSE                        : i32 = 1005;
pub const MSK_RES_ERR_SIZE_LICENSE_CON                    : i32 = 1010;
pub const MSK_RES_ERR_SIZE_LICENSE_INTVAR                 : i32 = 1012;
pub const MSK_RES_ERR_SIZE_LICENSE_NUMCORES               : i32 = 3900;
pub const MSK_RES_ERR_SIZE_LICENSE_VAR                    : i32 = 1011;
pub const MSK_RES_ERR_SOL_FILE_INVALID_NUMBER             : i32 = 1350;
pub const MSK_RES_ERR_SOLITEM                             : i32 = 1237;
pub const MSK_RES_ERR_SOLVER_PROBTYPE                     : i32 = 1259;
pub const MSK_RES_ERR_SPACE                               : i32 = 1051;
pub const MSK_RES_ERR_SPACE_LEAKING                       : i32 = 1080;
pub const MSK_RES_ERR_SPACE_NO_INFO                       : i32 = 1081;
pub const MSK_RES_ERR_SYM_MAT_DUPLICATE                   : i32 = 3944;
pub const MSK_RES_ERR_SYM_MAT_INVALID_COL_INDEX           : i32 = 3941;
pub const MSK_RES_ERR_SYM_MAT_INVALID_ROW_INDEX           : i32 = 3940;
pub const MSK_RES_ERR_SYM_MAT_INVALID_VALUE               : i32 = 3943;
pub const MSK_RES_ERR_SYM_MAT_NOT_LOWER_TRINGULAR         : i32 = 3942;
pub const MSK_RES_ERR_TASK_INCOMPATIBLE                   : i32 = 2560;
pub const MSK_RES_ERR_TASK_INVALID                        : i32 = 2561;
pub const MSK_RES_ERR_THREAD_COND_INIT                    : i32 = 1049;
pub const MSK_RES_ERR_THREAD_CREATE                       : i32 = 1048;
pub const MSK_RES_ERR_THREAD_MUTEX_INIT                   : i32 = 1045;
pub const MSK_RES_ERR_THREAD_MUTEX_LOCK                   : i32 = 1046;
pub const MSK_RES_ERR_THREAD_MUTEX_UNLOCK                 : i32 = 1047;
pub const MSK_RES_ERR_TOCONIC_CONVERSION_FAIL             : i32 = 7200;
pub const MSK_RES_ERR_TOO_MANY_CONCURRENT_TASKS           : i32 = 3090;
pub const MSK_RES_ERR_TOO_SMALL_MAX_NUM_NZ                : i32 = 1245;
pub const MSK_RES_ERR_TOO_SMALL_MAXNUMANZ                 : i32 = 1252;
pub const MSK_RES_ERR_UNB_STEP_SIZE                       : i32 = 3100;
pub const MSK_RES_ERR_UNDEF_SOLUTION                      : i32 = 1265;
pub const MSK_RES_ERR_UNDEFINED_OBJECTIVE_SENSE           : i32 = 1446;
pub const MSK_RES_ERR_UNHANDLED_SOLUTION_STATUS           : i32 = 6010;
pub const MSK_RES_ERR_UNKNOWN                             : i32 = 1050;
pub const MSK_RES_ERR_UPPER_BOUND_IS_A_NAN                : i32 = 1391;
pub const MSK_RES_ERR_UPPER_TRIANGLE                      : i32 = 6020;
pub const MSK_RES_ERR_USER_FUNC_RET                       : i32 = 1430;
pub const MSK_RES_ERR_USER_FUNC_RET_DATA                  : i32 = 1431;
pub const MSK_RES_ERR_USER_NLO_EVAL                       : i32 = 1433;
pub const MSK_RES_ERR_USER_NLO_EVAL_HESSUBI               : i32 = 1440;
pub const MSK_RES_ERR_USER_NLO_EVAL_HESSUBJ               : i32 = 1441;
pub const MSK_RES_ERR_USER_NLO_FUNC                       : i32 = 1432;
pub const MSK_RES_ERR_WHICHITEM_NOT_ALLOWED               : i32 = 1238;
pub const MSK_RES_ERR_WHICHSOL                            : i32 = 1236;
pub const MSK_RES_ERR_WRITE_LP_FORMAT                     : i32 = 1158;
pub const MSK_RES_ERR_WRITE_LP_NON_UNIQUE_NAME            : i32 = 1161;
pub const MSK_RES_ERR_WRITE_MPS_INVALID_NAME              : i32 = 1153;
pub const MSK_RES_ERR_WRITE_OPF_INVALID_VAR_NAME          : i32 = 1156;
pub const MSK_RES_ERR_WRITING_FILE                        : i32 = 1166;
pub const MSK_RES_ERR_XML_INVALID_PROBLEM_TYPE            : i32 = 3600;
pub const MSK_RES_ERR_Y_IS_UNDEFINED                      : i32 = 1449;
pub const MSK_RES_OK                                      : i32 = 0;
pub const MSK_RES_TRM_INTERNAL                            : i32 = 10030;
pub const MSK_RES_TRM_INTERNAL_STOP                       : i32 = 10031;
pub const MSK_RES_TRM_MAX_ITERATIONS                      : i32 = 10000;
pub const MSK_RES_TRM_MAX_NUM_SETBACKS                    : i32 = 10020;
pub const MSK_RES_TRM_MAX_TIME                            : i32 = 10001;
pub const MSK_RES_TRM_MIO_NEAR_ABS_GAP                    : i32 = 10004;
pub const MSK_RES_TRM_MIO_NEAR_REL_GAP                    : i32 = 10003;
pub const MSK_RES_TRM_MIO_NUM_BRANCHES                    : i32 = 10009;
pub const MSK_RES_TRM_MIO_NUM_RELAXS                      : i32 = 10008;
pub const MSK_RES_TRM_NUM_MAX_NUM_INT_SOLUTIONS           : i32 = 10015;
pub const MSK_RES_TRM_NUMERICAL_PROBLEM                   : i32 = 10025;
pub const MSK_RES_TRM_OBJECTIVE_RANGE                     : i32 = 10002;
pub const MSK_RES_TRM_STALL                               : i32 = 10006;
pub const MSK_RES_TRM_USER_CALLBACK                       : i32 = 10007;
pub const MSK_RES_WRN_ANA_ALMOST_INT_BOUNDS               : i32 = 904;
pub const MSK_RES_WRN_ANA_C_ZERO                          : i32 = 901;
pub const MSK_RES_WRN_ANA_CLOSE_BOUNDS                    : i32 = 903;
pub const MSK_RES_WRN_ANA_EMPTY_COLS                      : i32 = 902;
pub const MSK_RES_WRN_ANA_LARGE_BOUNDS                    : i32 = 900;
pub const MSK_RES_WRN_CONSTRUCT_INVALID_SOL_ITG           : i32 = 807;
pub const MSK_RES_WRN_CONSTRUCT_NO_SOL_ITG                : i32 = 810;
pub const MSK_RES_WRN_CONSTRUCT_SOLUTION_INFEAS           : i32 = 805;
pub const MSK_RES_WRN_DROPPED_NZ_QOBJ                     : i32 = 201;
pub const MSK_RES_WRN_DUPLICATE_BARVARIABLE_NAMES         : i32 = 852;
pub const MSK_RES_WRN_DUPLICATE_CONE_NAMES                : i32 = 853;
pub const MSK_RES_WRN_DUPLICATE_CONSTRAINT_NAMES          : i32 = 850;
pub const MSK_RES_WRN_DUPLICATE_VARIABLE_NAMES            : i32 = 851;
pub const MSK_RES_WRN_ELIMINATOR_SPACE                    : i32 = 801;
pub const MSK_RES_WRN_EMPTY_NAME                          : i32 = 502;
pub const MSK_RES_WRN_IGNORE_INTEGER                      : i32 = 250;
pub const MSK_RES_WRN_INCOMPLETE_LINEAR_DEPENDENCY_CHECK  : i32 = 800;
pub const MSK_RES_WRN_LARGE_AIJ                           : i32 = 62;
pub const MSK_RES_WRN_LARGE_BOUND                         : i32 = 51;
pub const MSK_RES_WRN_LARGE_CJ                            : i32 = 57;
pub const MSK_RES_WRN_LARGE_CON_FX                        : i32 = 54;
pub const MSK_RES_WRN_LARGE_LO_BOUND                      : i32 = 52;
pub const MSK_RES_WRN_LARGE_UP_BOUND                      : i32 = 53;
pub const MSK_RES_WRN_LICENSE_EXPIRE                      : i32 = 500;
pub const MSK_RES_WRN_LICENSE_FEATURE_EXPIRE              : i32 = 505;
pub const MSK_RES_WRN_LICENSE_SERVER                      : i32 = 501;
pub const MSK_RES_WRN_LP_DROP_VARIABLE                    : i32 = 85;
pub const MSK_RES_WRN_LP_OLD_QUAD_FORMAT                  : i32 = 80;
pub const MSK_RES_WRN_MIO_INFEASIBLE_FINAL                : i32 = 270;
pub const MSK_RES_WRN_MPS_SPLIT_BOU_VECTOR                : i32 = 72;
pub const MSK_RES_WRN_MPS_SPLIT_RAN_VECTOR                : i32 = 71;
pub const MSK_RES_WRN_MPS_SPLIT_RHS_VECTOR                : i32 = 70;
pub const MSK_RES_WRN_NAME_MAX_LEN                        : i32 = 65;
pub const MSK_RES_WRN_NO_DUALIZER                         : i32 = 950;
pub const MSK_RES_WRN_NO_GLOBAL_OPTIMIZER                 : i32 = 251;
pub const MSK_RES_WRN_NO_NONLINEAR_FUNCTION_WRITE         : i32 = 450;
pub const MSK_RES_WRN_NZ_IN_UPR_TRI                       : i32 = 200;
pub const MSK_RES_WRN_OPEN_PARAM_FILE                     : i32 = 50;
pub const MSK_RES_WRN_PARAM_IGNORED_CMIO                  : i32 = 516;
pub const MSK_RES_WRN_PARAM_NAME_DOU                      : i32 = 510;
pub const MSK_RES_WRN_PARAM_NAME_INT                      : i32 = 511;
pub const MSK_RES_WRN_PARAM_NAME_STR                      : i32 = 512;
pub const MSK_RES_WRN_PARAM_STR_VALUE                     : i32 = 515;
pub const MSK_RES_WRN_PRESOLVE_OUTOFSPACE                 : i32 = 802;
pub const MSK_RES_WRN_QUAD_CONES_WITH_ROOT_FIXED_AT_ZERO  : i32 = 930;
pub const MSK_RES_WRN_RQUAD_CONES_WITH_ROOT_FIXED_AT_ZERO : i32 = 931;
pub const MSK_RES_WRN_SOL_FILE_IGNORED_CON                : i32 = 351;
pub const MSK_RES_WRN_SOL_FILE_IGNORED_VAR                : i32 = 352;
pub const MSK_RES_WRN_SOL_FILTER                          : i32 = 300;
pub const MSK_RES_WRN_SPAR_MAX_LEN                        : i32 = 66;
pub const MSK_RES_WRN_TOO_FEW_BASIS_VARS                  : i32 = 400;
pub const MSK_RES_WRN_TOO_MANY_BASIS_VARS                 : i32 = 405;
pub const MSK_RES_WRN_TOO_MANY_THREADS_CONCURRENT         : i32 = 750;
pub const MSK_RES_WRN_UNDEF_SOL_FILE_NAME                 : i32 = 350;
pub const MSK_RES_WRN_USING_GENERIC_NAMES                 : i32 = 503;
pub const MSK_RES_WRN_WRITE_CHANGED_NAMES                 : i32 = 803;
pub const MSK_RES_WRN_WRITE_DISCARDED_CFIX                : i32 = 804;
pub const MSK_RES_WRN_ZERO_AIJ                            : i32 = 63;
pub const MSK_RES_WRN_ZEROS_IN_SPARSE_COL                 : i32 = 710;
pub const MSK_RES_WRN_ZEROS_IN_SPARSE_ROW                 : i32 = 705;

// rescodetype
pub const MSK_RESPONSE_ERR : i32 = 3;
pub const MSK_RESPONSE_OK  : i32 = 0;
pub const MSK_RESPONSE_TRM : i32 = 2;
pub const MSK_RESPONSE_UNK : i32 = 4;
pub const MSK_RESPONSE_WRN : i32 = 1;
pub const MSK_RESPONSE_BEGIN : i32 = 0;
pub const MSK_RESPONSE_END   : i32 = 5;

// scalingmethod
pub const MSK_SCALING_METHOD_FREE : i32 = 1;
pub const MSK_SCALING_METHOD_POW2 : i32 = 0;
pub const MSK_SCALING_METHOD_BEGIN : i32 = 0;
pub const MSK_SCALING_METHOD_END   : i32 = 2;

// scalingtype
pub const MSK_SCALING_AGGRESSIVE : i32 = 3;
pub const MSK_SCALING_FREE       : i32 = 0;
pub const MSK_SCALING_MODERATE   : i32 = 2;
pub const MSK_SCALING_NONE       : i32 = 1;
pub const MSK_SCALING_BEGIN : i32 = 0;
pub const MSK_SCALING_END   : i32 = 4;

// sensitivitytype
pub const MSK_SENSITIVITY_TYPE_BASIS             : i32 = 0;
pub const MSK_SENSITIVITY_TYPE_OPTIMAL_PARTITION : i32 = 1;
pub const MSK_SENSITIVITY_TYPE_BEGIN : i32 = 0;
pub const MSK_SENSITIVITY_TYPE_END   : i32 = 2;

// simdegen
pub const MSK_SIM_DEGEN_AGGRESSIVE : i32 = 2;
pub const MSK_SIM_DEGEN_FREE       : i32 = 1;
pub const MSK_SIM_DEGEN_MINIMUM    : i32 = 4;
pub const MSK_SIM_DEGEN_MODERATE   : i32 = 3;
pub const MSK_SIM_DEGEN_NONE       : i32 = 0;
pub const MSK_SIM_DEGEN_BEGIN : i32 = 0;
pub const MSK_SIM_DEGEN_END   : i32 = 5;

// simdupvec
pub const MSK_SIM_EXPLOIT_DUPVEC_FREE : i32 = 2;
pub const MSK_SIM_EXPLOIT_DUPVEC_OFF  : i32 = 0;
pub const MSK_SIM_EXPLOIT_DUPVEC_ON   : i32 = 1;
pub const MSK_SIM_EXPLOIT_DUPVEC_BEGIN : i32 = 0;
pub const MSK_SIM_EXPLOIT_DUPVEC_END   : i32 = 3;

// simhotstart
pub const MSK_SIM_HOTSTART_FREE        : i32 = 1;
pub const MSK_SIM_HOTSTART_NONE        : i32 = 0;
pub const MSK_SIM_HOTSTART_STATUS_KEYS : i32 = 2;
pub const MSK_SIM_HOTSTART_BEGIN : i32 = 0;
pub const MSK_SIM_HOTSTART_END   : i32 = 3;

// simreform
pub const MSK_SIM_REFORMULATION_AGGRESSIVE : i32 = 3;
pub const MSK_SIM_REFORMULATION_FREE       : i32 = 2;
pub const MSK_SIM_REFORMULATION_OFF        : i32 = 0;
pub const MSK_SIM_REFORMULATION_ON         : i32 = 1;
pub const MSK_SIM_REFORMULATION_BEGIN : i32 = 0;
pub const MSK_SIM_REFORMULATION_END   : i32 = 4;

// simseltype
pub const MSK_SIM_SELECTION_ASE     : i32 = 2;
pub const MSK_SIM_SELECTION_DEVEX   : i32 = 3;
pub const MSK_SIM_SELECTION_FREE    : i32 = 0;
pub const MSK_SIM_SELECTION_FULL    : i32 = 1;
pub const MSK_SIM_SELECTION_PARTIAL : i32 = 5;
pub const MSK_SIM_SELECTION_SE      : i32 = 4;
pub const MSK_SIM_SELECTION_BEGIN : i32 = 0;
pub const MSK_SIM_SELECTION_END   : i32 = 6;

// solitem
pub const MSK_SOL_ITEM_SLC : i32 = 3;
pub const MSK_SOL_ITEM_SLX : i32 = 5;
pub const MSK_SOL_ITEM_SNX : i32 = 7;
pub const MSK_SOL_ITEM_SUC : i32 = 4;
pub const MSK_SOL_ITEM_SUX : i32 = 6;
pub const MSK_SOL_ITEM_XC  : i32 = 0;
pub const MSK_SOL_ITEM_XX  : i32 = 1;
pub const MSK_SOL_ITEM_Y   : i32 = 2;
pub const MSK_SOL_ITEM_BEGIN : i32 = 0;
pub const MSK_SOL_ITEM_END   : i32 = 8;

// solsta
pub const MSK_SOL_STA_DUAL_FEAS               : i32 = 3;
pub const MSK_SOL_STA_DUAL_INFEAS_CER         : i32 = 6;
pub const MSK_SOL_STA_INTEGER_OPTIMAL         : i32 = 14;
pub const MSK_SOL_STA_NEAR_DUAL_FEAS          : i32 = 10;
pub const MSK_SOL_STA_NEAR_DUAL_INFEAS_CER    : i32 = 13;
pub const MSK_SOL_STA_NEAR_INTEGER_OPTIMAL    : i32 = 15;
pub const MSK_SOL_STA_NEAR_OPTIMAL            : i32 = 8;
pub const MSK_SOL_STA_NEAR_PRIM_AND_DUAL_FEAS : i32 = 11;
pub const MSK_SOL_STA_NEAR_PRIM_FEAS          : i32 = 9;
pub const MSK_SOL_STA_NEAR_PRIM_INFEAS_CER    : i32 = 12;
pub const MSK_SOL_STA_OPTIMAL                 : i32 = 1;
pub const MSK_SOL_STA_PRIM_AND_DUAL_FEAS      : i32 = 4;
pub const MSK_SOL_STA_PRIM_FEAS               : i32 = 2;
pub const MSK_SOL_STA_PRIM_INFEAS_CER         : i32 = 5;
pub const MSK_SOL_STA_UNKNOWN                 : i32 = 0;
pub const MSK_SOL_STA_BEGIN : i32 = 0;
pub const MSK_SOL_STA_END   : i32 = 16;

// soltype
pub const MSK_SOL_BAS : i32 = 1;
pub const MSK_SOL_ITG : i32 = 2;
pub const MSK_SOL_ITR : i32 = 0;
pub const MSK_SOL_BEGIN : i32 = 0;
pub const MSK_SOL_END   : i32 = 3;

// solveform
pub const MSK_SOLVE_DUAL   : i32 = 2;
pub const MSK_SOLVE_FREE   : i32 = 0;
pub const MSK_SOLVE_PRIMAL : i32 = 1;
pub const MSK_SOLVE_BEGIN : i32 = 0;
pub const MSK_SOLVE_END   : i32 = 3;

// sparam
pub const MSK_SPAR_BAS_SOL_FILE_NAME         : i32 = 0;
pub const MSK_SPAR_DATA_FILE_NAME            : i32 = 1;
pub const MSK_SPAR_DEBUG_FILE_NAME           : i32 = 2;
pub const MSK_SPAR_INT_SOL_FILE_NAME         : i32 = 3;
pub const MSK_SPAR_ITR_SOL_FILE_NAME         : i32 = 4;
pub const MSK_SPAR_MIO_DEBUG_STRING          : i32 = 5;
pub const MSK_SPAR_PARAM_COMMENT_SIGN        : i32 = 6;
pub const MSK_SPAR_PARAM_READ_FILE_NAME      : i32 = 7;
pub const MSK_SPAR_PARAM_WRITE_FILE_NAME     : i32 = 8;
pub const MSK_SPAR_READ_MPS_BOU_NAME         : i32 = 9;
pub const MSK_SPAR_READ_MPS_OBJ_NAME         : i32 = 10;
pub const MSK_SPAR_READ_MPS_RAN_NAME         : i32 = 11;
pub const MSK_SPAR_READ_MPS_RHS_NAME         : i32 = 12;
pub const MSK_SPAR_SENSITIVITY_FILE_NAME     : i32 = 13;
pub const MSK_SPAR_SENSITIVITY_RES_FILE_NAME : i32 = 14;
pub const MSK_SPAR_SOL_FILTER_XC_LOW         : i32 = 15;
pub const MSK_SPAR_SOL_FILTER_XC_UPR         : i32 = 16;
pub const MSK_SPAR_SOL_FILTER_XX_LOW         : i32 = 17;
pub const MSK_SPAR_SOL_FILTER_XX_UPR         : i32 = 18;
pub const MSK_SPAR_STAT_FILE_NAME            : i32 = 19;
pub const MSK_SPAR_STAT_KEY                  : i32 = 20;
pub const MSK_SPAR_STAT_NAME                 : i32 = 21;
pub const MSK_SPAR_WRITE_LP_GEN_VAR_NAME     : i32 = 22;
pub const MSK_SPAR_BEGIN : i32 = 0;
pub const MSK_SPAR_END   : i32 = 23;

// stakey
pub const MSK_SK_BAS    : i32 = 1;
pub const MSK_SK_FIX    : i32 = 5;
pub const MSK_SK_INF    : i32 = 6;
pub const MSK_SK_LOW    : i32 = 3;
pub const MSK_SK_SUPBAS : i32 = 2;
pub const MSK_SK_UNK    : i32 = 0;
pub const MSK_SK_UPR    : i32 = 4;
pub const MSK_SK_BEGIN : i32 = 0;
pub const MSK_SK_END   : i32 = 7;

// startpointtype
pub const MSK_STARTING_POINT_CONSTANT       : i32 = 2;
pub const MSK_STARTING_POINT_FREE           : i32 = 0;
pub const MSK_STARTING_POINT_GUESS          : i32 = 1;
pub const MSK_STARTING_POINT_SATISFY_BOUNDS : i32 = 3;
pub const MSK_STARTING_POINT_BEGIN : i32 = 0;
pub const MSK_STARTING_POINT_END   : i32 = 4;

// streamtype
pub const MSK_STREAM_ERR : i32 = 2;
pub const MSK_STREAM_LOG : i32 = 0;
pub const MSK_STREAM_MSG : i32 = 1;
pub const MSK_STREAM_WRN : i32 = 3;
pub const MSK_STREAM_BEGIN : i32 = 0;
pub const MSK_STREAM_END   : i32 = 4;

// symmattype
pub const MSK_SYMMAT_TYPE_SPARSE : i32 = 0;
pub const MSK_SYMMAT_TYPE_BEGIN : i32 = 0;
pub const MSK_SYMMAT_TYPE_END   : i32 = 1;

// transpose
pub const MSK_TRANSPOSE_NO  : i32 = 0;
pub const MSK_TRANSPOSE_YES : i32 = 1;
pub const MSK_TRANSPOSE_BEGIN : i32 = 0;
pub const MSK_TRANSPOSE_END   : i32 = 2;

// uplo
pub const MSK_UPLO_LO : i32 = 0;
pub const MSK_UPLO_UP : i32 = 1;
pub const MSK_UPLO_BEGIN : i32 = 0;
pub const MSK_UPLO_END   : i32 = 2;

// value
pub const MSK_LICENSE_BUFFER_LENGTH : i32 = 20;
pub const MSK_MAX_STR_LEN           : i32 = 1024;

// variabletype
pub const MSK_VAR_TYPE_CONT : i32 = 0;
pub const MSK_VAR_TYPE_INT  : i32 = 1;
pub const MSK_VAR_BEGIN : i32 = 0;
pub const MSK_VAR_END   : i32 = 2;

// xmlwriteroutputtype
pub const MSK_WRITE_XML_MODE_COL : i32 = 1;
pub const MSK_WRITE_XML_MODE_ROW : i32 = 0;
pub const MSK_WRITE_XML_MODE_BEGIN : i32 = 0;
pub const MSK_WRITE_XML_MODE_END   : i32 = 2;

pub struct Env
{
    ptr : * const u8,
}

pub struct Task
{
    ptr : * const u8,
}

impl Env
{
    pub fn new() -> Env
    {
        let mut env : * const u8 = std::ptr::null();
        let res = unsafe { MSK_makeenv(& mut env, std::ptr::null()) };
        if res != 0 { panic!("Failed: MSK_getversion"); }

        return Env { ptr : env };
    }

    pub fn new_mem_debug(dbgfile : &str) -> Env
    {
        let mut env : * const u8 = std::ptr::null();
        let res = unsafe { MSK_makeenv(& mut env, CString::new(dbgfile).unwrap().as_ptr()) };
        if res != 0 { panic!("Failed: MSK_makeenv"); }

        return Env { ptr : env };
    }

    pub fn task(&self) -> Task
    {
        let mut task : * const u8 = std::ptr::null();
        if 0 != unsafe { MSK_maketask(self.ptr, 0,0, & mut task) }
        {
            panic!("Failed: MSK_maketask");
        }

        return Task { ptr : task };
    }

    pub fn task_with_capacity(&self, numcon : i32, numvar : i32) -> Task
    {
        let mut task : * const u8 = std::ptr::null();
        if 0 != unsafe { MSK_maketask(self.ptr, numcon,numvar, & mut task) }
        {
            panic!("Failed: MSK_maketask");
        }

        return Task { ptr : task };
    }

    
    // axpy
    pub fn axpy(&self,n_ : i32,alpha_ : f64,x_ : & [f64],y_ : & mut [f64])
    {
      if x_.len() != ((n_) as usize) { panic!("Argument 'x_' is too short in call to 'axpy'") }
      if y_.len() != ((n_) as usize) { panic!("Argument 'y_' is too short in call to 'axpy'") }
      callMSK!(MSK_axpy,self.ptr,n_ as libc::int32_t,alpha_ as f64,x_.as_ptr(),y_.as_mut_ptr());
    }
    
    // checkinlicense
    pub fn check_in_license(&self,feature_ : i32)
    {
      callMSK!(MSK_checkinlicense,self.ptr,feature_);
    }
    
    // checkoutlicense
    pub fn checkout_license(&self,feature_ : i32)
    {
      callMSK!(MSK_checkoutlicense,self.ptr,feature_);
    }
    
    // dot
    pub fn dot(&self,n_ : i32,x_ : & [f64],y_ : & [f64]) -> f64
    {
      if x_.len() != ((n_) as usize) { panic!("Argument 'x_' is too short in call to 'dot'") }
      if y_.len() != ((n_) as usize) { panic!("Argument 'y_' is too short in call to 'dot'") }
      let mut _ref_xty_ : f64 = 0 as f64;
      callMSK!(MSK_dot,self.ptr,n_ as libc::int32_t,x_.as_ptr(),y_.as_ptr(),& mut _ref_xty_);
      _ref_xty_ as f64
    }
    
    // echointro
    pub fn echo_intro(&self,longver_ : i32)
    {
      callMSK!(MSK_echointro,self.ptr,longver_ as libc::int32_t);
    }
    
    // gemm
    pub fn gemm(&self,transa_ : i32,transb_ : i32,m_ : i32,n_ : i32,k_ : i32,alpha_ : f64,a_ : & [f64],b_ : & [f64],beta_ : f64,c_ : & mut [f64])
    {
      if a_.len() != ((m_ * k_) as usize) { panic!("Argument 'a_' is too short in call to 'gemm'") }
      if b_.len() != ((k_ * n_) as usize) { panic!("Argument 'b_' is too short in call to 'gemm'") }
      if c_.len() != ((m_ * n_) as usize) { panic!("Argument 'c_' is too short in call to 'gemm'") }
      callMSK!(MSK_gemm,self.ptr,transa_,transb_,m_ as libc::int32_t,n_ as libc::int32_t,k_ as libc::int32_t,alpha_ as f64,a_.as_ptr(),b_.as_ptr(),beta_ as f64,c_.as_mut_ptr());
    }
    
    // gemv
    pub fn gemv(&self,transa_ : i32,m_ : i32,n_ : i32,alpha_ : f64,a_ : & [f64],x_ : & [f64],beta_ : f64,y_ : & mut [f64])
    {
      if a_.len() != ((n_ * m_) as usize) { panic!("Argument 'a_' is too short in call to 'gemv'") }
      let tmp_var_3__ = 
        if (transa_ == MSK_TRANSPOSE_NO) {
          n_
        }  else {
          m_
        };
      if x_.len() != ((tmp_var_3__) as usize) { panic!("Argument 'x_' is too short in call to 'gemv'") }
      let tmp_var_9__ = 
        if (transa_ == MSK_TRANSPOSE_NO) {
          m_
        }  else {
          n_
        };
      if y_.len() != ((tmp_var_9__) as usize) { panic!("Argument 'y_' is too short in call to 'gemv'") }
      callMSK!(MSK_gemv,self.ptr,transa_,m_ as libc::int32_t,n_ as libc::int32_t,alpha_ as f64,a_.as_ptr(),x_.as_ptr(),beta_ as f64,y_.as_mut_ptr());
    }
    
    // linkfiletoenvstream
    pub fn linkfiletostream(&self,whichstream_ : i32,filename_ : &str,append_ : i32)
    {
      callMSK!(MSK_linkfiletoenvstream,self.ptr,whichstream_,CString::new(filename_).unwrap().as_ptr(),append_ as libc::int32_t);
    }
    
    // potrf
    pub fn potrf(&self,uplo_ : i32,n_ : i32,a_ : & mut [f64])
    {
      if a_.len() != ((n_ * n_) as usize) { panic!("Argument 'a_' is too short in call to 'potrf'") }
      callMSK!(MSK_potrf,self.ptr,uplo_,n_ as libc::int32_t,a_.as_mut_ptr());
    }
    
    // putkeepdlls
    pub fn put_keep_dlls(&self,keepdlls_ : i32)
    {
      callMSK!(MSK_putkeepdlls,self.ptr,keepdlls_ as libc::int32_t);
    }
    
    // putlicensecode
    pub fn put_license_code(&self,code_ : & [i32])
    {
      if code_.len() != ((MSK_LICENSE_BUFFER_LENGTH) as usize) { panic!("Argument 'code_' is too short in call to 'put_license_code'") }
      callMSK!(MSK_putlicensecode,self.ptr,code_.as_ptr());
    }
    
    // putlicensedebug
    pub fn put_license_debug(&self,licdebug_ : i32)
    {
      callMSK!(MSK_putlicensedebug,self.ptr,licdebug_ as libc::int32_t);
    }
    
    // putlicensepath
    pub fn put_license_path(&self,licensepath_ : &str)
    {
      callMSK!(MSK_putlicensepath,self.ptr,CString::new(licensepath_).unwrap().as_ptr());
    }
    
    // putlicensewait
    pub fn put_license_wait(&self,licwait_ : i32)
    {
      callMSK!(MSK_putlicensewait,self.ptr,licwait_ as libc::int32_t);
    }
    
    // syeig
    pub fn syeig(&self,uplo_ : i32,n_ : i32,a_ : & [f64],w_ : & mut [f64])
    {
      if a_.len() != ((n_ * n_) as usize) { panic!("Argument 'a_' is too short in call to 'syeig'") }
      if w_.len() != ((n_) as usize) { panic!("Argument 'w_' is too short in call to 'syeig'") }
      callMSK!(MSK_syeig,self.ptr,uplo_,n_ as libc::int32_t,a_.as_ptr(),w_.as_mut_ptr());
    }
    
    // syevd
    pub fn syevd(&self,uplo_ : i32,n_ : i32,a_ : & mut [f64],w_ : & mut [f64])
    {
      if a_.len() != ((n_ * n_) as usize) { panic!("Argument 'a_' is too short in call to 'syevd'") }
      if w_.len() != ((n_) as usize) { panic!("Argument 'w_' is too short in call to 'syevd'") }
      callMSK!(MSK_syevd,self.ptr,uplo_,n_ as libc::int32_t,a_.as_mut_ptr(),w_.as_mut_ptr());
    }
    
    // syrk
    pub fn syrk(&self,uplo_ : i32,trans_ : i32,n_ : i32,k_ : i32,alpha_ : f64,a_ : & [f64],beta_ : f64,c_ : & mut [f64])
    {
      if a_.len() != ((n_ * k_) as usize) { panic!("Argument 'a_' is too short in call to 'syrk'") }
      if c_.len() != ((n_ * n_) as usize) { panic!("Argument 'c_' is too short in call to 'syrk'") }
      callMSK!(MSK_syrk,self.ptr,uplo_,trans_,n_ as libc::int32_t,k_ as libc::int32_t,alpha_ as f64,a_.as_ptr(),beta_ as f64,c_.as_mut_ptr());
    }

}

impl Task
{
    
    // analyzenames
    pub fn analyze_names(&self,whichstream_ : i32,nametype_ : i32)
    {
      callMSK!(MSK_analyzenames,self.ptr,whichstream_,nametype_);
    }
    
    // analyzeproblem
    pub fn analyze_problem(&self,whichstream_ : i32)
    {
      callMSK!(MSK_analyzeproblem,self.ptr,whichstream_);
    }
    
    // analyzesolution
    pub fn analyze_solution(&self,whichstream_ : i32,whichsol_ : i32)
    {
      callMSK!(MSK_analyzesolution,self.ptr,whichstream_,whichsol_);
    }
    
    // appendbarvars
    pub fn append_barvars(&self,dim_ : & [i32])
    {
      let mut num_ = dim_.len();
      callMSK!(MSK_appendbarvars,self.ptr,num_ as libc::int32_t,dim_.as_ptr());
    }
    
    // appendcone
    pub fn append_cone(&self,ct_ : i32,conepar_ : f64,submem_ : & [i32])
    {
      let mut nummem_ = submem_.len();
      callMSK!(MSK_appendcone,self.ptr,ct_,conepar_ as f64,nummem_ as libc::int32_t,submem_.as_ptr());
    }
    
    // appendconeseq
    pub fn append_cone_seq(&self,ct_ : i32,conepar_ : f64,nummem_ : i32,j_ : i32)
    {
      callMSK!(MSK_appendconeseq,self.ptr,ct_,conepar_ as f64,nummem_ as libc::int32_t,j_ as libc::int32_t);
    }
    
    // appendconesseq
    pub fn append_cones_seq(&self,ct_ : & [i32],conepar_ : & [f64],nummem_ : & [i32],j_ : i32)
    {
      let mut num_ = ct_.len();
      if conepar_.len() > num_ { num_ = conepar_.len() };
      if nummem_.len() > num_ { num_ = nummem_.len() };
      callMSK!(MSK_appendconesseq,self.ptr,num_ as libc::int32_t,ct_.as_ptr(),conepar_.as_ptr(),nummem_.as_ptr(),j_ as libc::int32_t);
    }
    
    // appendcons
    pub fn append_cons(&self,num_ : i32)
    {
      callMSK!(MSK_appendcons,self.ptr,num_ as libc::int32_t);
    }
    
    // appendsparsesymmat
    pub fn append_sparse_sym_mat(&self,dim_ : i32,subi_ : & [i32],subj_ : & [i32],valij_ : & [f64]) -> i64
    {
      let mut nz_ = subi_.len();
      if subj_.len() > nz_ { nz_ = subj_.len() };
      if valij_.len() > nz_ { nz_ = valij_.len() };
      let mut _ref_idx_ : libc::int64_t = 0 as libc::int64_t;
      callMSK!(MSK_appendsparsesymmat,self.ptr,dim_ as libc::int32_t,nz_ as libc::int64_t,subi_.as_ptr(),subj_.as_ptr(),valij_.as_ptr(),& mut _ref_idx_);
      _ref_idx_ as i64
    }
    
    // appendstat
    pub fn append_stat(&self)
    {
      callMSK!(MSK_appendstat,self.ptr);
    }
    
    // appendvars
    pub fn append_vars(&self,num_ : i32)
    {
      callMSK!(MSK_appendvars,self.ptr,num_ as libc::int32_t);
    }
    
    // basiscond
    pub fn basis_cond(&self) -> (f64,f64)
    {
      let mut _ref_nrmbasis_ : f64 = 0 as f64;
      let mut _ref_nrminvbasis_ : f64 = 0 as f64;
      callMSK!(MSK_basiscond,self.ptr,& mut _ref_nrmbasis_,& mut _ref_nrminvbasis_);
      return (_ref_nrmbasis_ as f64,_ref_nrminvbasis_ as f64)
    }
    
    // checkconvexity
    pub fn check_convexity(&self)
    {
      callMSK!(MSK_checkconvexity,self.ptr);
    }
    
    // checkmemtask
    pub fn check_mem(&self,file_ : &str,line_ : i32)
    {
      callMSK!(MSK_checkmemtask,self.ptr,CString::new(file_).unwrap().as_ptr(),line_ as libc::int32_t);
    }
    
    // chgbound
    pub fn chg_bound(&self,accmode_ : i32,i_ : i32,lower_ : i32,finite_ : i32,value_ : f64)
    {
      callMSK!(MSK_chgbound,self.ptr,accmode_,i_ as libc::int32_t,lower_ as libc::int32_t,finite_ as libc::int32_t,value_ as f64);
    }
    
    // chgconbound
    pub fn chg_con_bound(&self,i_ : i32,lower_ : i32,finite_ : i32,value_ : f64)
    {
      callMSK!(MSK_chgconbound,self.ptr,i_ as libc::int32_t,lower_ as libc::int32_t,finite_ as libc::int32_t,value_ as f64);
    }
    
    // chgvarbound
    pub fn chg_var_bound(&self,j_ : i32,lower_ : i32,finite_ : i32,value_ : f64)
    {
      callMSK!(MSK_chgvarbound,self.ptr,j_ as libc::int32_t,lower_ as libc::int32_t,finite_ as libc::int32_t,value_ as f64);
    }
    
    // commitchanges
    pub fn commit_changes(&self)
    {
      callMSK!(MSK_commitchanges,self.ptr);
    }
    
    // deletesolution
    pub fn delete_solution(&self,whichsol_ : i32)
    {
      callMSK!(MSK_deletesolution,self.ptr,whichsol_);
    }
    
    // dualsensitivity
    pub fn dual_sensitivity(&self,subj_ : & [i32],leftpricej_ : & mut [f64],rightpricej_ : & mut [f64],leftrangej_ : & mut [f64],rightrangej_ : & mut [f64])
    {
      let mut numj_ = subj_.len();
      if leftpricej_.len() != ((numj_) as usize) { panic!("Argument 'leftpricej_' is too short in call to 'dual_sensitivity'") }
      if rightpricej_.len() != ((numj_) as usize) { panic!("Argument 'rightpricej_' is too short in call to 'dual_sensitivity'") }
      if leftrangej_.len() != ((numj_) as usize) { panic!("Argument 'leftrangej_' is too short in call to 'dual_sensitivity'") }
      if rightrangej_.len() != ((numj_) as usize) { panic!("Argument 'rightrangej_' is too short in call to 'dual_sensitivity'") }
      callMSK!(MSK_dualsensitivity,self.ptr,numj_ as libc::int32_t,subj_.as_ptr(),leftpricej_.as_mut_ptr(),rightpricej_.as_mut_ptr(),leftrangej_.as_mut_ptr(),rightrangej_.as_mut_ptr());
    }
    
    // getacol
    pub fn get_a_col(&self,j_ : i32,subj_ : & mut [i32],valj_ : & mut [f64]) -> i32
    {
      let mut _ref_nzj_ : libc::int32_t = 0 as libc::int32_t;
      let tmp_var_1__ = self.get_a_col_num_nz(j_);
      if subj_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'subj_' is too short in call to 'get_a_col'") }
      let tmp_var_4__ = self.get_a_col_num_nz(j_);
      if valj_.len() != ((tmp_var_4__) as usize) { panic!("Argument 'valj_' is too short in call to 'get_a_col'") }
      callMSK!(MSK_getacol,self.ptr,j_ as libc::int32_t,& mut _ref_nzj_,subj_.as_mut_ptr(),valj_.as_mut_ptr());
      _ref_nzj_ as i32
    }
    
    // getacolnumnz
    pub fn get_a_col_num_nz(&self,i_ : i32) -> i32
    {
      let mut _ref_nzj_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getacolnumnz,self.ptr,i_ as libc::int32_t,& mut _ref_nzj_);
      _ref_nzj_ as i32
    }
    
    // getaij
    pub fn get_aij(&self,i_ : i32,j_ : i32) -> f64
    {
      let mut _ref_aij_ : f64 = 0 as f64;
      callMSK!(MSK_getaij,self.ptr,i_ as libc::int32_t,j_ as libc::int32_t,& mut _ref_aij_);
      _ref_aij_ as f64
    }
    
    // getapiecenumnz
    pub fn get_a_piece_num_nz(&self,firsti_ : i32,lasti_ : i32,firstj_ : i32,lastj_ : i32) -> i32
    {
      let mut _ref_numnz_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getapiecenumnz,self.ptr,firsti_ as libc::int32_t,lasti_ as libc::int32_t,firstj_ as libc::int32_t,lastj_ as libc::int32_t,& mut _ref_numnz_);
      _ref_numnz_ as i32
    }
    
    // getarow
    pub fn get_a_row(&self,i_ : i32,subi_ : & mut [i32],vali_ : & mut [f64]) -> i32
    {
      let mut _ref_nzi_ : libc::int32_t = 0 as libc::int32_t;
      let tmp_var_1__ = self.get_a_row_num_nz(i_);
      if subi_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'subi_' is too short in call to 'get_a_row'") }
      let tmp_var_4__ = self.get_a_row_num_nz(i_);
      if vali_.len() != ((tmp_var_4__) as usize) { panic!("Argument 'vali_' is too short in call to 'get_a_row'") }
      callMSK!(MSK_getarow,self.ptr,i_ as libc::int32_t,& mut _ref_nzi_,subi_.as_mut_ptr(),vali_.as_mut_ptr());
      _ref_nzi_ as i32
    }
    
    // getarownumnz
    pub fn get_a_row_num_nz(&self,i_ : i32) -> i32
    {
      let mut _ref_nzi_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getarownumnz,self.ptr,i_ as libc::int32_t,& mut _ref_nzi_);
      _ref_nzi_ as i32
    }
    
    // getaslicenumnz64
    pub fn get_a_slice_num_nz(&self,accmode_ : i32,first_ : i32,last_ : i32) -> i64
    {
      let mut _ref_numnz_ : libc::int64_t = 0 as libc::int64_t;
      callMSK!(MSK_getaslicenumnz64,self.ptr,accmode_,first_ as libc::int32_t,last_ as libc::int32_t,& mut _ref_numnz_);
      _ref_numnz_ as i64
    }
    
    // getbarablocktriplet
    pub fn get_bara_block_triplet(&self,subi_ : & mut [i32],subj_ : & mut [i32],subk_ : & mut [i32],subl_ : & mut [i32],valijkl_ : & mut [f64]) -> i64
    {
      let tmp_var_1__ = self.get_num_bara_block_triplets();
      let maxnum_ = tmp_var_1__;
      let mut _ref_num_ : libc::int64_t = 0 as libc::int64_t;
      if subi_.len() != ((maxnum_) as usize) { panic!("Argument 'subi_' is too short in call to 'get_bara_block_triplet'") }
      if subj_.len() != ((maxnum_) as usize) { panic!("Argument 'subj_' is too short in call to 'get_bara_block_triplet'") }
      if subk_.len() != ((maxnum_) as usize) { panic!("Argument 'subk_' is too short in call to 'get_bara_block_triplet'") }
      if subl_.len() != ((maxnum_) as usize) { panic!("Argument 'subl_' is too short in call to 'get_bara_block_triplet'") }
      if valijkl_.len() != ((maxnum_) as usize) { panic!("Argument 'valijkl_' is too short in call to 'get_bara_block_triplet'") }
      callMSK!(MSK_getbarablocktriplet,self.ptr,maxnum_ as libc::int64_t,& mut _ref_num_,subi_.as_mut_ptr(),subj_.as_mut_ptr(),subk_.as_mut_ptr(),subl_.as_mut_ptr(),valijkl_.as_mut_ptr());
      _ref_num_ as i64
    }
    
    // getbaraidx
    pub fn get_bara_idx(&self,idx_ : i64,sub_ : & mut [i64],weights_ : & mut [f64]) -> (i32,i32,i64)
    {
      let tmp_var_1__ = self.get_bara_idx_info(idx_);
      let maxnum_ = tmp_var_1__;
      let mut _ref_i_ : libc::int32_t = 0 as libc::int32_t;
      let mut _ref_j_ : libc::int32_t = 0 as libc::int32_t;
      let mut _ref_num_ : libc::int64_t = 0 as libc::int64_t;
      if sub_.len() != ((maxnum_) as usize) { panic!("Argument 'sub_' is too short in call to 'get_bara_idx'") }
      if weights_.len() != ((maxnum_) as usize) { panic!("Argument 'weights_' is too short in call to 'get_bara_idx'") }
      callMSK!(MSK_getbaraidx,self.ptr,idx_ as libc::int64_t,maxnum_ as libc::int64_t,& mut _ref_i_,& mut _ref_j_,& mut _ref_num_,sub_.as_mut_ptr(),weights_.as_mut_ptr());
      return (_ref_i_ as i32,_ref_j_ as i32,_ref_num_ as i64)
    }
    
    // getbaraidxij
    pub fn get_bara_idx_i_j(&self,idx_ : i64) -> (i32,i32)
    {
      let mut _ref_i_ : libc::int32_t = 0 as libc::int32_t;
      let mut _ref_j_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getbaraidxij,self.ptr,idx_ as libc::int64_t,& mut _ref_i_,& mut _ref_j_);
      return (_ref_i_ as i32,_ref_j_ as i32)
    }
    
    // getbaraidxinfo
    pub fn get_bara_idx_info(&self,idx_ : i64) -> i64
    {
      let mut _ref_num_ : libc::int64_t = 0 as libc::int64_t;
      callMSK!(MSK_getbaraidxinfo,self.ptr,idx_ as libc::int64_t,& mut _ref_num_);
      _ref_num_ as i64
    }
    
    // getbarasparsity
    pub fn get_bara_sparsity(&self,idxij_ : & mut [i64]) -> i64
    {
      let tmp_var_1__ = self.get_num_bara_nz();
      let maxnumnz_ = tmp_var_1__;
      let mut _ref_numnz_ : libc::int64_t = 0 as libc::int64_t;
      if idxij_.len() != ((maxnumnz_) as usize) { panic!("Argument 'idxij_' is too short in call to 'get_bara_sparsity'") }
      callMSK!(MSK_getbarasparsity,self.ptr,maxnumnz_ as libc::int64_t,& mut _ref_numnz_,idxij_.as_mut_ptr());
      _ref_numnz_ as i64
    }
    
    // getbarcblocktriplet
    pub fn get_barc_block_triplet(&self,subj_ : & mut [i32],subk_ : & mut [i32],subl_ : & mut [i32],valijkl_ : & mut [f64]) -> i64
    {
      let tmp_var_1__ = self.get_num_barc_block_triplets();
      let maxnum_ = tmp_var_1__;
      let mut _ref_num_ : libc::int64_t = 0 as libc::int64_t;
      if subj_.len() != ((maxnum_) as usize) { panic!("Argument 'subj_' is too short in call to 'get_barc_block_triplet'") }
      if subk_.len() != ((maxnum_) as usize) { panic!("Argument 'subk_' is too short in call to 'get_barc_block_triplet'") }
      if subl_.len() != ((maxnum_) as usize) { panic!("Argument 'subl_' is too short in call to 'get_barc_block_triplet'") }
      if valijkl_.len() != ((maxnum_) as usize) { panic!("Argument 'valijkl_' is too short in call to 'get_barc_block_triplet'") }
      callMSK!(MSK_getbarcblocktriplet,self.ptr,maxnum_ as libc::int64_t,& mut _ref_num_,subj_.as_mut_ptr(),subk_.as_mut_ptr(),subl_.as_mut_ptr(),valijkl_.as_mut_ptr());
      _ref_num_ as i64
    }
    
    // getbarcidx
    pub fn get_barc_idx(&self,idx_ : i64,sub_ : & mut [i64],weights_ : & mut [f64]) -> (i32,i64)
    {
      let tmp_var_1__ = self.get_barc_idx_info(idx_);
      let maxnum_ = tmp_var_1__;
      let mut _ref_j_ : libc::int32_t = 0 as libc::int32_t;
      let mut _ref_num_ : libc::int64_t = 0 as libc::int64_t;
      if sub_.len() != ((maxnum_) as usize) { panic!("Argument 'sub_' is too short in call to 'get_barc_idx'") }
      if weights_.len() != ((maxnum_) as usize) { panic!("Argument 'weights_' is too short in call to 'get_barc_idx'") }
      callMSK!(MSK_getbarcidx,self.ptr,idx_ as libc::int64_t,maxnum_ as libc::int64_t,& mut _ref_j_,& mut _ref_num_,sub_.as_mut_ptr(),weights_.as_mut_ptr());
      return (_ref_j_ as i32,_ref_num_ as i64)
    }
    
    // getbarcidxinfo
    pub fn get_barc_idx_info(&self,idx_ : i64) -> i64
    {
      let mut _ref_num_ : libc::int64_t = 0 as libc::int64_t;
      callMSK!(MSK_getbarcidxinfo,self.ptr,idx_ as libc::int64_t,& mut _ref_num_);
      _ref_num_ as i64
    }
    
    // getbarcidxj
    pub fn get_barc_idx_j(&self,idx_ : i64) -> i32
    {
      let mut _ref_j_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getbarcidxj,self.ptr,idx_ as libc::int64_t,& mut _ref_j_);
      _ref_j_ as i32
    }
    
    // getbarcsparsity
    pub fn get_barc_sparsity(&self,idxj_ : & mut [i64]) -> i64
    {
      let tmp_var_1__ = self.get_num_barc_nz();
      let maxnumnz_ = tmp_var_1__;
      let mut _ref_numnz_ : libc::int64_t = 0 as libc::int64_t;
      if idxj_.len() != ((maxnumnz_) as usize) { panic!("Argument 'idxj_' is too short in call to 'get_barc_sparsity'") }
      callMSK!(MSK_getbarcsparsity,self.ptr,maxnumnz_ as libc::int64_t,& mut _ref_numnz_,idxj_.as_mut_ptr());
      _ref_numnz_ as i64
    }
    
    // getbarsj
    pub fn get_bars_j(&self,whichsol_ : i32,j_ : i32,barsj_ : & mut [f64])
    {
      let tmp_var_1__ = self.get_len_barvar_j(j_);
      if barsj_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'barsj_' is too short in call to 'get_bars_j'") }
      callMSK!(MSK_getbarsj,self.ptr,whichsol_,j_ as libc::int32_t,barsj_.as_mut_ptr());
    }
    
    // getbarvarname
    pub fn get_barvar_name(&self,i_ : i32) -> String
    {
      let tmp_var_3__ = self.get_barvar_name_len(i_);
      let maxlen_ = 1 + tmp_var_3__;
      let mut _name__bytes = Vec::with_capacity(maxlen_ as usize);
      callMSK!(MSK_getbarvarname,self.ptr,i_ as libc::int32_t,maxlen_ as libc::int32_t,_name__bytes.as_mut_ptr());
      unsafe { _name__bytes.set_len((maxlen_) as usize) };
      String::from_utf8_lossy(&_name__bytes[..]).into_owned()
    }
    
    // getbarvarnameindex
    pub fn get_barvar_name_index(&self,somename_ : &str) -> (i32,i32)
    {
      let mut _ref_asgn_ : libc::int32_t = 0 as libc::int32_t;
      let mut _ref_index_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getbarvarnameindex,self.ptr,CString::new(somename_).unwrap().as_ptr(),& mut _ref_asgn_,& mut _ref_index_);
      return (_ref_asgn_ as i32,_ref_index_ as i32)
    }
    
    // getbarvarnamelen
    pub fn get_barvar_name_len(&self,i_ : i32) -> i32
    {
      let mut _ref_len_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getbarvarnamelen,self.ptr,i_ as libc::int32_t,& mut _ref_len_);
      _ref_len_ as i32
    }
    
    // getbarxj
    pub fn get_barx_j(&self,whichsol_ : i32,j_ : i32,barxj_ : & mut [f64])
    {
      let tmp_var_1__ = self.get_len_barvar_j(j_);
      if barxj_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'barxj_' is too short in call to 'get_barx_j'") }
      callMSK!(MSK_getbarxj,self.ptr,whichsol_,j_ as libc::int32_t,barxj_.as_mut_ptr());
    }
    
    // getbound
    pub fn get_bound(&self,accmode_ : i32,i_ : i32) -> (i32,f64,f64)
    {
      let mut _ref_bk_ : i32 = 0 as i32;
      let mut _ref_bl_ : f64 = 0 as f64;
      let mut _ref_bu_ : f64 = 0 as f64;
      callMSK!(MSK_getbound,self.ptr,accmode_,i_ as libc::int32_t,& mut _ref_bk_,& mut _ref_bl_,& mut _ref_bu_);
      return (_ref_bk_ as i32,_ref_bl_ as f64,_ref_bu_ as f64)
    }
    
    // getboundslice
    pub fn get_bound_slice(&self,accmode_ : i32,first_ : i32,last_ : i32,bk_ : & mut [i32],bl_ : & mut [f64],bu_ : & mut [f64])
    {
      if bk_.len() != ((last_ - first_) as usize) { panic!("Argument 'bk_' is too short in call to 'get_bound_slice'") }
      if bl_.len() != ((last_ - first_) as usize) { panic!("Argument 'bl_' is too short in call to 'get_bound_slice'") }
      if bu_.len() != ((last_ - first_) as usize) { panic!("Argument 'bu_' is too short in call to 'get_bound_slice'") }
      callMSK!(MSK_getboundslice,self.ptr,accmode_,first_ as libc::int32_t,last_ as libc::int32_t,bk_.as_mut_ptr(),bl_.as_mut_ptr(),bu_.as_mut_ptr());
    }
    
    // getc
    pub fn get_c(&self,c_ : & mut [f64])
    {
      let tmp_var_1__ = self.get_num_var();
      if c_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'c_' is too short in call to 'get_c'") }
      callMSK!(MSK_getc,self.ptr,c_.as_mut_ptr());
    }
    
    // getcfix
    pub fn get_cfix(&self) -> f64
    {
      let mut _ref_cfix_ : f64 = 0 as f64;
      callMSK!(MSK_getcfix,self.ptr,& mut _ref_cfix_);
      _ref_cfix_ as f64
    }
    
    // getcj
    pub fn get_c_j(&self,j_ : i32) -> f64
    {
      let mut _ref_cj_ : f64 = 0 as f64;
      callMSK!(MSK_getcj,self.ptr,j_ as libc::int32_t,& mut _ref_cj_);
      _ref_cj_ as f64
    }
    
    // getconbound
    pub fn get_con_bound(&self,i_ : i32) -> (i32,f64,f64)
    {
      let mut _ref_bk_ : i32 = 0 as i32;
      let mut _ref_bl_ : f64 = 0 as f64;
      let mut _ref_bu_ : f64 = 0 as f64;
      callMSK!(MSK_getconbound,self.ptr,i_ as libc::int32_t,& mut _ref_bk_,& mut _ref_bl_,& mut _ref_bu_);
      return (_ref_bk_ as i32,_ref_bl_ as f64,_ref_bu_ as f64)
    }
    
    // getconboundslice
    pub fn get_con_bound_slice(&self,first_ : i32,last_ : i32,bk_ : & mut [i32],bl_ : & mut [f64],bu_ : & mut [f64])
    {
      if bk_.len() != ((last_ - first_) as usize) { panic!("Argument 'bk_' is too short in call to 'get_con_bound_slice'") }
      if bl_.len() != ((last_ - first_) as usize) { panic!("Argument 'bl_' is too short in call to 'get_con_bound_slice'") }
      if bu_.len() != ((last_ - first_) as usize) { panic!("Argument 'bu_' is too short in call to 'get_con_bound_slice'") }
      callMSK!(MSK_getconboundslice,self.ptr,first_ as libc::int32_t,last_ as libc::int32_t,bk_.as_mut_ptr(),bl_.as_mut_ptr(),bu_.as_mut_ptr());
    }
    
    // getcone
    pub fn get_cone(&self,k_ : i32,submem_ : & mut [i32]) -> (i32,f64,i32)
    {
      let mut _ref_ct_ : i32 = 0 as i32;
      let mut _ref_conepar_ : f64 = 0 as f64;
      let mut _ref_nummem_ : libc::int32_t = 0 as libc::int32_t;
      let tmp_var_1__ = self.get_cone_info(k_).2;
      if submem_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'submem_' is too short in call to 'get_cone'") }
      callMSK!(MSK_getcone,self.ptr,k_ as libc::int32_t,& mut _ref_ct_,& mut _ref_conepar_,& mut _ref_nummem_,submem_.as_mut_ptr());
      return (_ref_ct_ as i32,_ref_conepar_ as f64,_ref_nummem_ as i32)
    }
    
    // getconeinfo
    pub fn get_cone_info(&self,k_ : i32) -> (i32,f64,i32)
    {
      let mut _ref_ct_ : i32 = 0 as i32;
      let mut _ref_conepar_ : f64 = 0 as f64;
      let mut _ref_nummem_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getconeinfo,self.ptr,k_ as libc::int32_t,& mut _ref_ct_,& mut _ref_conepar_,& mut _ref_nummem_);
      return (_ref_ct_ as i32,_ref_conepar_ as f64,_ref_nummem_ as i32)
    }
    
    // getconename
    pub fn get_cone_name(&self,i_ : i32) -> String
    {
      let tmp_var_3__ = self.get_cone_name_len(i_);
      let maxlen_ = 1 + tmp_var_3__;
      let mut _name__bytes = Vec::with_capacity(maxlen_ as usize);
      callMSK!(MSK_getconename,self.ptr,i_ as libc::int32_t,maxlen_ as libc::int32_t,_name__bytes.as_mut_ptr());
      unsafe { _name__bytes.set_len((maxlen_) as usize) };
      String::from_utf8_lossy(&_name__bytes[..]).into_owned()
    }
    
    // getconenameindex
    pub fn get_cone_name_index(&self,somename_ : &str) -> (i32,i32)
    {
      let mut _ref_asgn_ : libc::int32_t = 0 as libc::int32_t;
      let mut _ref_index_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getconenameindex,self.ptr,CString::new(somename_).unwrap().as_ptr(),& mut _ref_asgn_,& mut _ref_index_);
      return (_ref_asgn_ as i32,_ref_index_ as i32)
    }
    
    // getconenamelen
    pub fn get_cone_name_len(&self,i_ : i32) -> i32
    {
      let mut _ref_len_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getconenamelen,self.ptr,i_ as libc::int32_t,& mut _ref_len_);
      _ref_len_ as i32
    }
    
    // getconname
    pub fn get_con_name(&self,i_ : i32) -> String
    {
      let tmp_var_3__ = self.get_con_name_len(i_);
      let maxlen_ = 1 + tmp_var_3__;
      let mut _name__bytes = Vec::with_capacity(maxlen_ as usize);
      callMSK!(MSK_getconname,self.ptr,i_ as libc::int32_t,maxlen_ as libc::int32_t,_name__bytes.as_mut_ptr());
      unsafe { _name__bytes.set_len((maxlen_) as usize) };
      String::from_utf8_lossy(&_name__bytes[..]).into_owned()
    }
    
    // getconnameindex
    pub fn get_con_name_index(&self,somename_ : &str) -> (i32,i32)
    {
      let mut _ref_asgn_ : libc::int32_t = 0 as libc::int32_t;
      let mut _ref_index_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getconnameindex,self.ptr,CString::new(somename_).unwrap().as_ptr(),& mut _ref_asgn_,& mut _ref_index_);
      return (_ref_asgn_ as i32,_ref_index_ as i32)
    }
    
    // getconnamelen
    pub fn get_con_name_len(&self,i_ : i32) -> i32
    {
      let mut _ref_len_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getconnamelen,self.ptr,i_ as libc::int32_t,& mut _ref_len_);
      _ref_len_ as i32
    }
    
    // getcslice
    pub fn get_c_slice(&self,first_ : i32,last_ : i32,c_ : & mut [f64])
    {
      if c_.len() != ((last_ - first_) as usize) { panic!("Argument 'c_' is too short in call to 'get_c_slice'") }
      callMSK!(MSK_getcslice,self.ptr,first_ as libc::int32_t,last_ as libc::int32_t,c_.as_mut_ptr());
    }
    
    // getdimbarvarj
    pub fn get_dim_barvar_j(&self,j_ : i32) -> i32
    {
      let mut _ref_dimbarvarj_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getdimbarvarj,self.ptr,j_ as libc::int32_t,& mut _ref_dimbarvarj_);
      _ref_dimbarvarj_ as i32
    }
    
    // getdouinf
    pub fn get_dou_inf(&self,whichdinf_ : i32) -> f64
    {
      let mut _ref_dvalue_ : f64 = 0 as f64;
      callMSK!(MSK_getdouinf,self.ptr,whichdinf_,& mut _ref_dvalue_);
      _ref_dvalue_ as f64
    }
    
    // getdouparam
    pub fn get_dou_param(&self,param_ : i32) -> f64
    {
      let mut _ref_parvalue_ : f64 = 0 as f64;
      callMSK!(MSK_getdouparam,self.ptr,param_,& mut _ref_parvalue_);
      _ref_parvalue_ as f64
    }
    
    // getdualobj
    pub fn get_dual_obj(&self,whichsol_ : i32) -> f64
    {
      let mut _ref_dualobj_ : f64 = 0 as f64;
      callMSK!(MSK_getdualobj,self.ptr,whichsol_,& mut _ref_dualobj_);
      _ref_dualobj_ as f64
    }
    
    // getdviolbarvar
    pub fn get_dviol_barvar(&self,whichsol_ : i32,sub_ : & [i32],viol_ : & mut [f64])
    {
      let mut num_ = sub_.len();
      if viol_.len() != ((num_) as usize) { panic!("Argument 'viol_' is too short in call to 'get_dviol_barvar'") }
      callMSK!(MSK_getdviolbarvar,self.ptr,whichsol_,num_ as libc::int32_t,sub_.as_ptr(),viol_.as_mut_ptr());
    }
    
    // getdviolcon
    pub fn get_dviol_con(&self,whichsol_ : i32,sub_ : & [i32],viol_ : & mut [f64])
    {
      let mut num_ = sub_.len();
      if viol_.len() != ((num_) as usize) { panic!("Argument 'viol_' is too short in call to 'get_dviol_con'") }
      callMSK!(MSK_getdviolcon,self.ptr,whichsol_,num_ as libc::int32_t,sub_.as_ptr(),viol_.as_mut_ptr());
    }
    
    // getdviolcones
    pub fn get_dviol_cones(&self,whichsol_ : i32,sub_ : & [i32],viol_ : & mut [f64])
    {
      let mut num_ = sub_.len();
      if viol_.len() != ((num_) as usize) { panic!("Argument 'viol_' is too short in call to 'get_dviol_cones'") }
      callMSK!(MSK_getdviolcones,self.ptr,whichsol_,num_ as libc::int32_t,sub_.as_ptr(),viol_.as_mut_ptr());
    }
    
    // getdviolvar
    pub fn get_dviol_var(&self,whichsol_ : i32,sub_ : & [i32],viol_ : & mut [f64])
    {
      let mut num_ = sub_.len();
      if viol_.len() != ((num_) as usize) { panic!("Argument 'viol_' is too short in call to 'get_dviol_var'") }
      callMSK!(MSK_getdviolvar,self.ptr,whichsol_,num_ as libc::int32_t,sub_.as_ptr(),viol_.as_mut_ptr());
    }
    
    // getinfindex
    pub fn get_inf_index(&self,inftype_ : i32,infname_ : &str) -> i32
    {
      let mut _ref_infindex_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getinfindex,self.ptr,inftype_,CString::new(infname_).unwrap().as_ptr(),& mut _ref_infindex_);
      _ref_infindex_ as i32
    }
    
    // getinfmax
    pub fn get_inf_max(&self,inftype_ : i32,infmax_ : & mut [i32])
    {
      if infmax_.len() != ((MSK_MAX_STR_LEN) as usize) { panic!("Argument 'infmax_' is too short in call to 'get_inf_max'") }
      callMSK!(MSK_getinfmax,self.ptr,inftype_,infmax_.as_mut_ptr());
    }
    
    // getinfname
    pub fn get_inf_name(&self,inftype_ : i32,whichinf_ : i32) -> String
    {
      let mut _infname__bytes = Vec::with_capacity(MSK_MAX_STR_LEN as usize);
      callMSK!(MSK_getinfname,self.ptr,inftype_,whichinf_ as libc::int32_t,_infname__bytes.as_mut_ptr());
      unsafe { _infname__bytes.set_len((MSK_MAX_STR_LEN) as usize) };
      String::from_utf8_lossy(&_infname__bytes[..]).into_owned()
    }
    
    // getintinf
    pub fn get_int_inf(&self,whichiinf_ : i32) -> i32
    {
      let mut _ref_ivalue_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getintinf,self.ptr,whichiinf_,& mut _ref_ivalue_);
      _ref_ivalue_ as i32
    }
    
    // getintparam
    pub fn get_int_param(&self,param_ : i32) -> i32
    {
      let mut _ref_parvalue_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getintparam,self.ptr,param_,& mut _ref_parvalue_);
      _ref_parvalue_ as i32
    }
    
    // getlenbarvarj
    pub fn get_len_barvar_j(&self,j_ : i32) -> i64
    {
      let mut _ref_lenbarvarj_ : libc::int64_t = 0 as libc::int64_t;
      callMSK!(MSK_getlenbarvarj,self.ptr,j_ as libc::int32_t,& mut _ref_lenbarvarj_);
      _ref_lenbarvarj_ as i64
    }
    
    // getlintinf
    pub fn get_lint_inf(&self,whichliinf_ : i32) -> i64
    {
      let mut _ref_ivalue_ : libc::int64_t = 0 as libc::int64_t;
      callMSK!(MSK_getlintinf,self.ptr,whichliinf_,& mut _ref_ivalue_);
      _ref_ivalue_ as i64
    }
    
    // getmaxnumanz64
    pub fn get_max_num_a_nz(&self) -> i64
    {
      let mut _ref_maxnumanz_ : libc::int64_t = 0 as libc::int64_t;
      callMSK!(MSK_getmaxnumanz64,self.ptr,& mut _ref_maxnumanz_);
      _ref_maxnumanz_ as i64
    }
    
    // getmaxnumbarvar
    pub fn get_max_num_barvar(&self) -> i32
    {
      let mut _ref_maxnumbarvar_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getmaxnumbarvar,self.ptr,& mut _ref_maxnumbarvar_);
      _ref_maxnumbarvar_ as i32
    }
    
    // getmaxnumcon
    pub fn get_max_num_con(&self) -> i32
    {
      let mut _ref_maxnumcon_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getmaxnumcon,self.ptr,& mut _ref_maxnumcon_);
      _ref_maxnumcon_ as i32
    }
    
    // getmaxnumcone
    pub fn get_max_num_cone(&self) -> i32
    {
      let mut _ref_maxnumcone_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getmaxnumcone,self.ptr,& mut _ref_maxnumcone_);
      _ref_maxnumcone_ as i32
    }
    
    // getmaxnumqnz64
    pub fn get_max_num_q_nz(&self) -> i64
    {
      let mut _ref_maxnumqnz_ : libc::int64_t = 0 as libc::int64_t;
      callMSK!(MSK_getmaxnumqnz64,self.ptr,& mut _ref_maxnumqnz_);
      _ref_maxnumqnz_ as i64
    }
    
    // getmaxnumvar
    pub fn get_max_num_var(&self) -> i32
    {
      let mut _ref_maxnumvar_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getmaxnumvar,self.ptr,& mut _ref_maxnumvar_);
      _ref_maxnumvar_ as i32
    }
    
    // getmemusagetask
    pub fn get_mem_usage(&self) -> (i64,i64)
    {
      let mut _ref_meminuse_ : libc::int64_t = 0 as libc::int64_t;
      let mut _ref_maxmemuse_ : libc::int64_t = 0 as libc::int64_t;
      callMSK!(MSK_getmemusagetask,self.ptr,& mut _ref_meminuse_,& mut _ref_maxmemuse_);
      return (_ref_meminuse_ as i64,_ref_maxmemuse_ as i64)
    }
    
    // getnumanz
    pub fn get_num_a_nz(&self) -> i32
    {
      let mut _ref_numanz_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getnumanz,self.ptr,& mut _ref_numanz_);
      _ref_numanz_ as i32
    }
    
    // getnumanz64
    pub fn get_num_a_nz_64(&self) -> i64
    {
      let mut _ref_numanz_ : libc::int64_t = 0 as libc::int64_t;
      callMSK!(MSK_getnumanz64,self.ptr,& mut _ref_numanz_);
      _ref_numanz_ as i64
    }
    
    // getnumbarablocktriplets
    pub fn get_num_bara_block_triplets(&self) -> i64
    {
      let mut _ref_num_ : libc::int64_t = 0 as libc::int64_t;
      callMSK!(MSK_getnumbarablocktriplets,self.ptr,& mut _ref_num_);
      _ref_num_ as i64
    }
    
    // getnumbaranz
    pub fn get_num_bara_nz(&self) -> i64
    {
      let mut _ref_nz_ : libc::int64_t = 0 as libc::int64_t;
      callMSK!(MSK_getnumbaranz,self.ptr,& mut _ref_nz_);
      _ref_nz_ as i64
    }
    
    // getnumbarcblocktriplets
    pub fn get_num_barc_block_triplets(&self) -> i64
    {
      let mut _ref_num_ : libc::int64_t = 0 as libc::int64_t;
      callMSK!(MSK_getnumbarcblocktriplets,self.ptr,& mut _ref_num_);
      _ref_num_ as i64
    }
    
    // getnumbarcnz
    pub fn get_num_barc_nz(&self) -> i64
    {
      let mut _ref_nz_ : libc::int64_t = 0 as libc::int64_t;
      callMSK!(MSK_getnumbarcnz,self.ptr,& mut _ref_nz_);
      _ref_nz_ as i64
    }
    
    // getnumbarvar
    pub fn get_num_barvar(&self) -> i32
    {
      let mut _ref_numbarvar_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getnumbarvar,self.ptr,& mut _ref_numbarvar_);
      _ref_numbarvar_ as i32
    }
    
    // getnumcon
    pub fn get_num_con(&self) -> i32
    {
      let mut _ref_numcon_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getnumcon,self.ptr,& mut _ref_numcon_);
      _ref_numcon_ as i32
    }
    
    // getnumcone
    pub fn get_num_cone(&self) -> i32
    {
      let mut _ref_numcone_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getnumcone,self.ptr,& mut _ref_numcone_);
      _ref_numcone_ as i32
    }
    
    // getnumconemem
    pub fn get_num_cone_mem(&self,k_ : i32) -> i32
    {
      let mut _ref_nummem_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getnumconemem,self.ptr,k_ as libc::int32_t,& mut _ref_nummem_);
      _ref_nummem_ as i32
    }
    
    // getnumintvar
    pub fn get_num_int_var(&self) -> i32
    {
      let mut _ref_numintvar_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getnumintvar,self.ptr,& mut _ref_numintvar_);
      _ref_numintvar_ as i32
    }
    
    // getnumparam
    pub fn get_num_param(&self,partype_ : i32) -> i32
    {
      let mut _ref_numparam_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getnumparam,self.ptr,partype_,& mut _ref_numparam_);
      _ref_numparam_ as i32
    }
    
    // getnumqconknz64
    pub fn get_num_q_con_k_nz(&self,k_ : i32) -> i64
    {
      let mut _ref_numqcnz_ : libc::int64_t = 0 as libc::int64_t;
      callMSK!(MSK_getnumqconknz64,self.ptr,k_ as libc::int32_t,& mut _ref_numqcnz_);
      _ref_numqcnz_ as i64
    }
    
    // getnumqobjnz64
    pub fn get_num_q_obj_nz(&self) -> i64
    {
      let mut _ref_numqonz_ : libc::int64_t = 0 as libc::int64_t;
      callMSK!(MSK_getnumqobjnz64,self.ptr,& mut _ref_numqonz_);
      _ref_numqonz_ as i64
    }
    
    // getnumsymmat
    pub fn get_num_sym_mat(&self) -> i64
    {
      let mut _ref_num_ : libc::int64_t = 0 as libc::int64_t;
      callMSK!(MSK_getnumsymmat,self.ptr,& mut _ref_num_);
      _ref_num_ as i64
    }
    
    // getnumvar
    pub fn get_num_var(&self) -> i32
    {
      let mut _ref_numvar_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getnumvar,self.ptr,& mut _ref_numvar_);
      _ref_numvar_ as i32
    }
    
    // getobjname
    pub fn get_obj_name(&self) -> String
    {
      let tmp_var_3__ = self.get_obj_name_len();
      let maxlen_ = 1 + tmp_var_3__;
      let mut _objname__bytes = Vec::with_capacity(maxlen_ as usize);
      callMSK!(MSK_getobjname,self.ptr,maxlen_ as libc::int32_t,_objname__bytes.as_mut_ptr());
      unsafe { _objname__bytes.set_len((maxlen_) as usize) };
      String::from_utf8_lossy(&_objname__bytes[..]).into_owned()
    }
    
    // getobjnamelen
    pub fn get_obj_name_len(&self) -> i32
    {
      let mut _ref_len_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getobjnamelen,self.ptr,& mut _ref_len_);
      _ref_len_ as i32
    }
    
    // getobjsense
    pub fn get_obj_sense(&self) -> i32
    {
      let mut _ref_sense_ : i32 = 0 as i32;
      callMSK!(MSK_getobjsense,self.ptr,& mut _ref_sense_);
      _ref_sense_ as i32
    }
    
    // getparammax
    pub fn get_param_max(&self,partype_ : i32) -> i32
    {
      let mut _ref_parammax_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getparammax,self.ptr,partype_,& mut _ref_parammax_);
      _ref_parammax_ as i32
    }
    
    // getparamname
    pub fn get_param_name(&self,partype_ : i32,param_ : i32) -> String
    {
      let mut _parname__bytes = Vec::with_capacity(MSK_MAX_STR_LEN as usize);
      callMSK!(MSK_getparamname,self.ptr,partype_,param_ as libc::int32_t,_parname__bytes.as_mut_ptr());
      unsafe { _parname__bytes.set_len((MSK_MAX_STR_LEN) as usize) };
      String::from_utf8_lossy(&_parname__bytes[..]).into_owned()
    }
    
    // getprimalobj
    pub fn get_primal_obj(&self,whichsol_ : i32) -> f64
    {
      let mut _ref_primalobj_ : f64 = 0 as f64;
      callMSK!(MSK_getprimalobj,self.ptr,whichsol_,& mut _ref_primalobj_);
      _ref_primalobj_ as f64
    }
    
    // getprobtype
    pub fn get_prob_type(&self) -> i32
    {
      let mut _ref_probtype_ : i32 = 0 as i32;
      callMSK!(MSK_getprobtype,self.ptr,& mut _ref_probtype_);
      _ref_probtype_ as i32
    }
    
    // getprosta
    pub fn get_pro_sta(&self,whichsol_ : i32) -> i32
    {
      let mut _ref_prosta_ : i32 = 0 as i32;
      callMSK!(MSK_getprosta,self.ptr,whichsol_,& mut _ref_prosta_);
      _ref_prosta_ as i32
    }
    
    // getpviolbarvar
    pub fn get_pviol_barvar(&self,whichsol_ : i32,sub_ : & [i32],viol_ : & mut [f64])
    {
      let mut num_ = sub_.len();
      if viol_.len() != ((num_) as usize) { panic!("Argument 'viol_' is too short in call to 'get_pviol_barvar'") }
      callMSK!(MSK_getpviolbarvar,self.ptr,whichsol_,num_ as libc::int32_t,sub_.as_ptr(),viol_.as_mut_ptr());
    }
    
    // getpviolcon
    pub fn get_pviol_con(&self,whichsol_ : i32,sub_ : & [i32],viol_ : & mut [f64])
    {
      let mut num_ = sub_.len();
      if viol_.len() != ((num_) as usize) { panic!("Argument 'viol_' is too short in call to 'get_pviol_con'") }
      callMSK!(MSK_getpviolcon,self.ptr,whichsol_,num_ as libc::int32_t,sub_.as_ptr(),viol_.as_mut_ptr());
    }
    
    // getpviolcones
    pub fn get_pviol_cones(&self,whichsol_ : i32,sub_ : & [i32],viol_ : & mut [f64])
    {
      let mut num_ = sub_.len();
      if viol_.len() != ((num_) as usize) { panic!("Argument 'viol_' is too short in call to 'get_pviol_cones'") }
      callMSK!(MSK_getpviolcones,self.ptr,whichsol_,num_ as libc::int32_t,sub_.as_ptr(),viol_.as_mut_ptr());
    }
    
    // getpviolvar
    pub fn get_pviol_var(&self,whichsol_ : i32,sub_ : & [i32],viol_ : & mut [f64])
    {
      let mut num_ = sub_.len();
      if viol_.len() != ((num_) as usize) { panic!("Argument 'viol_' is too short in call to 'get_pviol_var'") }
      callMSK!(MSK_getpviolvar,self.ptr,whichsol_,num_ as libc::int32_t,sub_.as_ptr(),viol_.as_mut_ptr());
    }
    
    // getqobjij
    pub fn get_q_obj_i_j(&self,i_ : i32,j_ : i32) -> f64
    {
      let mut _ref_qoij_ : f64 = 0 as f64;
      callMSK!(MSK_getqobjij,self.ptr,i_ as libc::int32_t,j_ as libc::int32_t,& mut _ref_qoij_);
      _ref_qoij_ as f64
    }
    
    // getreducedcosts
    pub fn get_reduced_costs(&self,whichsol_ : i32,first_ : i32,last_ : i32,redcosts_ : & mut [f64])
    {
      if redcosts_.len() != ((last_ - first_) as usize) { panic!("Argument 'redcosts_' is too short in call to 'get_reduced_costs'") }
      callMSK!(MSK_getreducedcosts,self.ptr,whichsol_,first_ as libc::int32_t,last_ as libc::int32_t,redcosts_.as_mut_ptr());
    }
    
    // getskc
    pub fn get_skc(&self,whichsol_ : i32,skc_ : & mut [i32])
    {
      let tmp_var_1__ = self.get_num_con();
      if skc_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'skc_' is too short in call to 'get_skc'") }
      callMSK!(MSK_getskc,self.ptr,whichsol_,skc_.as_mut_ptr());
    }
    
    // getskcslice
    pub fn get_skc_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,skc_ : & mut [i32])
    {
      if skc_.len() != ((last_ - first_) as usize) { panic!("Argument 'skc_' is too short in call to 'get_skc_slice'") }
      callMSK!(MSK_getskcslice,self.ptr,whichsol_,first_ as libc::int32_t,last_ as libc::int32_t,skc_.as_mut_ptr());
    }
    
    // getskx
    pub fn get_skx(&self,whichsol_ : i32,skx_ : & mut [i32])
    {
      let tmp_var_1__ = self.get_num_var();
      if skx_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'skx_' is too short in call to 'get_skx'") }
      callMSK!(MSK_getskx,self.ptr,whichsol_,skx_.as_mut_ptr());
    }
    
    // getskxslice
    pub fn get_skx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,skx_ : & mut [i32])
    {
      if skx_.len() != ((last_ - first_) as usize) { panic!("Argument 'skx_' is too short in call to 'get_skx_slice'") }
      callMSK!(MSK_getskxslice,self.ptr,whichsol_,first_ as libc::int32_t,last_ as libc::int32_t,skx_.as_mut_ptr());
    }
    
    // getslc
    pub fn get_slc(&self,whichsol_ : i32,slc_ : & mut [f64])
    {
      let tmp_var_1__ = self.get_num_con();
      if slc_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'slc_' is too short in call to 'get_slc'") }
      callMSK!(MSK_getslc,self.ptr,whichsol_,slc_.as_mut_ptr());
    }
    
    // getslcslice
    pub fn get_slc_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,slc_ : & mut [f64])
    {
      if slc_.len() != ((last_ - first_) as usize) { panic!("Argument 'slc_' is too short in call to 'get_slc_slice'") }
      callMSK!(MSK_getslcslice,self.ptr,whichsol_,first_ as libc::int32_t,last_ as libc::int32_t,slc_.as_mut_ptr());
    }
    
    // getslx
    pub fn get_slx(&self,whichsol_ : i32,slx_ : & mut [f64])
    {
      let tmp_var_1__ = self.get_num_var();
      if slx_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'slx_' is too short in call to 'get_slx'") }
      callMSK!(MSK_getslx,self.ptr,whichsol_,slx_.as_mut_ptr());
    }
    
    // getslxslice
    pub fn get_slx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,slx_ : & mut [f64])
    {
      if slx_.len() != ((last_ - first_) as usize) { panic!("Argument 'slx_' is too short in call to 'get_slx_slice'") }
      callMSK!(MSK_getslxslice,self.ptr,whichsol_,first_ as libc::int32_t,last_ as libc::int32_t,slx_.as_mut_ptr());
    }
    
    // getsnx
    pub fn get_snx(&self,whichsol_ : i32,snx_ : & mut [f64])
    {
      let tmp_var_1__ = self.get_num_var();
      if snx_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'snx_' is too short in call to 'get_snx'") }
      callMSK!(MSK_getsnx,self.ptr,whichsol_,snx_.as_mut_ptr());
    }
    
    // getsnxslice
    pub fn get_snx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,snx_ : & mut [f64])
    {
      if snx_.len() != ((last_ - first_) as usize) { panic!("Argument 'snx_' is too short in call to 'get_snx_slice'") }
      callMSK!(MSK_getsnxslice,self.ptr,whichsol_,first_ as libc::int32_t,last_ as libc::int32_t,snx_.as_mut_ptr());
    }
    
    // getsolsta
    pub fn get_sol_sta(&self,whichsol_ : i32) -> i32
    {
      let mut _ref_solsta_ : i32 = 0 as i32;
      callMSK!(MSK_getsolsta,self.ptr,whichsol_,& mut _ref_solsta_);
      _ref_solsta_ as i32
    }
    
    // getsolution
    pub fn get_solution(&self,whichsol_ : i32,skc_ : & mut [i32],skx_ : & mut [i32],skn_ : & mut [i32],xc_ : & mut [f64],xx_ : & mut [f64],y_ : & mut [f64],slc_ : & mut [f64],suc_ : & mut [f64],slx_ : & mut [f64],sux_ : & mut [f64],snx_ : & mut [f64]) -> (i32,i32)
    {
      let mut _ref_prosta_ : i32 = 0 as i32;
      let mut _ref_solsta_ : i32 = 0 as i32;
      let tmp_var_1__ = self.get_num_con();
      if skc_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'skc_' is too short in call to 'get_solution'") }
      let tmp_var_3__ = self.get_num_var();
      if skx_.len() != ((tmp_var_3__) as usize) { panic!("Argument 'skx_' is too short in call to 'get_solution'") }
      let tmp_var_5__ = self.get_num_cone();
      if skn_.len() != ((tmp_var_5__) as usize) { panic!("Argument 'skn_' is too short in call to 'get_solution'") }
      let tmp_var_7__ = self.get_num_con();
      if xc_.len() != ((tmp_var_7__) as usize) { panic!("Argument 'xc_' is too short in call to 'get_solution'") }
      let tmp_var_9__ = self.get_num_var();
      if xx_.len() != ((tmp_var_9__) as usize) { panic!("Argument 'xx_' is too short in call to 'get_solution'") }
      let tmp_var_11__ = self.get_num_con();
      if y_.len() != ((tmp_var_11__) as usize) { panic!("Argument 'y_' is too short in call to 'get_solution'") }
      let tmp_var_13__ = self.get_num_con();
      if slc_.len() != ((tmp_var_13__) as usize) { panic!("Argument 'slc_' is too short in call to 'get_solution'") }
      let tmp_var_15__ = self.get_num_con();
      if suc_.len() != ((tmp_var_15__) as usize) { panic!("Argument 'suc_' is too short in call to 'get_solution'") }
      let tmp_var_17__ = self.get_num_var();
      if slx_.len() != ((tmp_var_17__) as usize) { panic!("Argument 'slx_' is too short in call to 'get_solution'") }
      let tmp_var_19__ = self.get_num_var();
      if sux_.len() != ((tmp_var_19__) as usize) { panic!("Argument 'sux_' is too short in call to 'get_solution'") }
      let tmp_var_21__ = self.get_num_cone();
      if snx_.len() != ((tmp_var_21__) as usize) { panic!("Argument 'snx_' is too short in call to 'get_solution'") }
      callMSK!(MSK_getsolution,self.ptr,whichsol_,& mut _ref_prosta_,& mut _ref_solsta_,skc_.as_mut_ptr(),skx_.as_mut_ptr(),skn_.as_mut_ptr(),xc_.as_mut_ptr(),xx_.as_mut_ptr(),y_.as_mut_ptr(),slc_.as_mut_ptr(),suc_.as_mut_ptr(),slx_.as_mut_ptr(),sux_.as_mut_ptr(),snx_.as_mut_ptr());
      return (_ref_prosta_ as i32,_ref_solsta_ as i32)
    }
    
    // getsolutioni
    pub fn get_solution_i(&self,accmode_ : i32,i_ : i32,whichsol_ : i32) -> (i32,f64,f64,f64,f64)
    {
      let mut _ref_sk_ : i32 = 0 as i32;
      let mut _ref_x_ : f64 = 0 as f64;
      let mut _ref_sl_ : f64 = 0 as f64;
      let mut _ref_su_ : f64 = 0 as f64;
      let mut _ref_sn_ : f64 = 0 as f64;
      callMSK!(MSK_getsolutioni,self.ptr,accmode_,i_ as libc::int32_t,whichsol_,& mut _ref_sk_,& mut _ref_x_,& mut _ref_sl_,& mut _ref_su_,& mut _ref_sn_);
      return (_ref_sk_ as i32,_ref_x_ as f64,_ref_sl_ as f64,_ref_su_ as f64,_ref_sn_ as f64)
    }
    
    // getsolutioninfo
    pub fn get_solution_info(&self,whichsol_ : i32) -> (f64,f64,f64,f64,f64,f64,f64,f64,f64,f64,f64)
    {
      let mut _ref_pobj_ : f64 = 0 as f64;
      let mut _ref_pviolcon_ : f64 = 0 as f64;
      let mut _ref_pviolvar_ : f64 = 0 as f64;
      let mut _ref_pviolbarvar_ : f64 = 0 as f64;
      let mut _ref_pviolcone_ : f64 = 0 as f64;
      let mut _ref_pviolitg_ : f64 = 0 as f64;
      let mut _ref_dobj_ : f64 = 0 as f64;
      let mut _ref_dviolcon_ : f64 = 0 as f64;
      let mut _ref_dviolvar_ : f64 = 0 as f64;
      let mut _ref_dviolbarvar_ : f64 = 0 as f64;
      let mut _ref_dviolcone_ : f64 = 0 as f64;
      callMSK!(MSK_getsolutioninfo,self.ptr,whichsol_,& mut _ref_pobj_,& mut _ref_pviolcon_,& mut _ref_pviolvar_,& mut _ref_pviolbarvar_,& mut _ref_pviolcone_,& mut _ref_pviolitg_,& mut _ref_dobj_,& mut _ref_dviolcon_,& mut _ref_dviolvar_,& mut _ref_dviolbarvar_,& mut _ref_dviolcone_);
      return (_ref_pobj_ as f64,_ref_pviolcon_ as f64,_ref_pviolvar_ as f64,_ref_pviolbarvar_ as f64,_ref_pviolcone_ as f64,_ref_pviolitg_ as f64,_ref_dobj_ as f64,_ref_dviolcon_ as f64,_ref_dviolvar_ as f64,_ref_dviolbarvar_ as f64,_ref_dviolcone_ as f64)
    }
    
    // getsolutionslice
    pub fn get_solution_slice(&self,whichsol_ : i32,solitem_ : i32,first_ : i32,last_ : i32,values_ : & mut [f64])
    {
      if values_.len() != ((last_ - first_) as usize) { panic!("Argument 'values_' is too short in call to 'get_solution_slice'") }
      callMSK!(MSK_getsolutionslice,self.ptr,whichsol_,solitem_,first_ as libc::int32_t,last_ as libc::int32_t,values_.as_mut_ptr());
    }
    
    // getsparsesymmat
    pub fn get_sparse_sym_mat(&self,idx_ : i64,subi_ : & mut [i32],subj_ : & mut [i32],valij_ : & mut [f64])
    {
      let tmp_var_1__ = self.get_sym_mat_info(idx_).1;
      let maxlen_ = tmp_var_1__;
      if subi_.len() != ((maxlen_) as usize) { panic!("Argument 'subi_' is too short in call to 'get_sparse_sym_mat'") }
      if subj_.len() != ((maxlen_) as usize) { panic!("Argument 'subj_' is too short in call to 'get_sparse_sym_mat'") }
      if valij_.len() != ((maxlen_) as usize) { panic!("Argument 'valij_' is too short in call to 'get_sparse_sym_mat'") }
      callMSK!(MSK_getsparsesymmat,self.ptr,idx_ as libc::int64_t,maxlen_ as libc::int64_t,subi_.as_mut_ptr(),subj_.as_mut_ptr(),valij_.as_mut_ptr());
    }
    
    // getstrparam
    pub fn get_str_param(&self,param_ : i32) -> (i32,String)
    {
      let tmp_var_3__ = self.get_str_param_len(param_);
      let maxlen_ = 1 + tmp_var_3__;
      let mut _ref_len_ : libc::int32_t = 0 as libc::int32_t;
      let mut _parvalue__bytes = Vec::with_capacity(maxlen_ as usize);
      callMSK!(MSK_getstrparam,self.ptr,param_,maxlen_ as libc::int32_t,& mut _ref_len_,_parvalue__bytes.as_mut_ptr());
      unsafe { _parvalue__bytes.set_len((maxlen_) as usize) };
      return (_ref_len_ as i32,String::from_utf8_lossy(&_parvalue__bytes[..]).into_owned())
    }
    
    // getstrparamlen
    pub fn get_str_param_len(&self,param_ : i32) -> i32
    {
      let mut _ref_len_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getstrparamlen,self.ptr,param_,& mut _ref_len_);
      _ref_len_ as i32
    }
    
    // getsuc
    pub fn get_suc(&self,whichsol_ : i32,suc_ : & mut [f64])
    {
      let tmp_var_1__ = self.get_num_con();
      if suc_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'suc_' is too short in call to 'get_suc'") }
      callMSK!(MSK_getsuc,self.ptr,whichsol_,suc_.as_mut_ptr());
    }
    
    // getsucslice
    pub fn get_suc_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,suc_ : & mut [f64])
    {
      if suc_.len() != ((last_ - first_) as usize) { panic!("Argument 'suc_' is too short in call to 'get_suc_slice'") }
      callMSK!(MSK_getsucslice,self.ptr,whichsol_,first_ as libc::int32_t,last_ as libc::int32_t,suc_.as_mut_ptr());
    }
    
    // getsux
    pub fn get_sux(&self,whichsol_ : i32,sux_ : & mut [f64])
    {
      let tmp_var_1__ = self.get_num_var();
      if sux_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'sux_' is too short in call to 'get_sux'") }
      callMSK!(MSK_getsux,self.ptr,whichsol_,sux_.as_mut_ptr());
    }
    
    // getsuxslice
    pub fn get_sux_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,sux_ : & mut [f64])
    {
      if sux_.len() != ((last_ - first_) as usize) { panic!("Argument 'sux_' is too short in call to 'get_sux_slice'") }
      callMSK!(MSK_getsuxslice,self.ptr,whichsol_,first_ as libc::int32_t,last_ as libc::int32_t,sux_.as_mut_ptr());
    }
    
    // getsymmatinfo
    pub fn get_sym_mat_info(&self,idx_ : i64) -> (i32,i64,i32)
    {
      let mut _ref_dim_ : libc::int32_t = 0 as libc::int32_t;
      let mut _ref_nz_ : libc::int64_t = 0 as libc::int64_t;
      let mut _ref_type_ : i32 = 0 as i32;
      callMSK!(MSK_getsymmatinfo,self.ptr,idx_ as libc::int64_t,& mut _ref_dim_,& mut _ref_nz_,& mut _ref_type_);
      return (_ref_dim_ as i32,_ref_nz_ as i64,_ref_type_ as i32)
    }
    
    // gettaskname
    pub fn get_task_name(&self) -> String
    {
      let tmp_var_3__ = self.get_task_name_len();
      let maxlen_ = 1 + tmp_var_3__;
      let mut _taskname__bytes = Vec::with_capacity(maxlen_ as usize);
      callMSK!(MSK_gettaskname,self.ptr,maxlen_ as libc::int32_t,_taskname__bytes.as_mut_ptr());
      unsafe { _taskname__bytes.set_len((maxlen_) as usize) };
      String::from_utf8_lossy(&_taskname__bytes[..]).into_owned()
    }
    
    // gettasknamelen
    pub fn get_task_name_len(&self) -> i32
    {
      let mut _ref_len_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_gettasknamelen,self.ptr,& mut _ref_len_);
      _ref_len_ as i32
    }
    
    // getvarbound
    pub fn get_var_bound(&self,i_ : i32) -> (i32,f64,f64)
    {
      let mut _ref_bk_ : i32 = 0 as i32;
      let mut _ref_bl_ : f64 = 0 as f64;
      let mut _ref_bu_ : f64 = 0 as f64;
      callMSK!(MSK_getvarbound,self.ptr,i_ as libc::int32_t,& mut _ref_bk_,& mut _ref_bl_,& mut _ref_bu_);
      return (_ref_bk_ as i32,_ref_bl_ as f64,_ref_bu_ as f64)
    }
    
    // getvarboundslice
    pub fn get_var_bound_slice(&self,first_ : i32,last_ : i32,bk_ : & mut [i32],bl_ : & mut [f64],bu_ : & mut [f64])
    {
      if bk_.len() != ((last_ - first_) as usize) { panic!("Argument 'bk_' is too short in call to 'get_var_bound_slice'") }
      if bl_.len() != ((last_ - first_) as usize) { panic!("Argument 'bl_' is too short in call to 'get_var_bound_slice'") }
      if bu_.len() != ((last_ - first_) as usize) { panic!("Argument 'bu_' is too short in call to 'get_var_bound_slice'") }
      callMSK!(MSK_getvarboundslice,self.ptr,first_ as libc::int32_t,last_ as libc::int32_t,bk_.as_mut_ptr(),bl_.as_mut_ptr(),bu_.as_mut_ptr());
    }
    
    // getvarbranchdir
    pub fn get_var_branch_dir(&self,j_ : i32) -> i32
    {
      let mut _ref_direction_ : i32 = 0 as i32;
      callMSK!(MSK_getvarbranchdir,self.ptr,j_ as libc::int32_t,& mut _ref_direction_);
      _ref_direction_ as i32
    }
    
    // getvarbranchorder
    pub fn get_var_branch_order(&self,j_ : i32) -> (i32,i32)
    {
      let mut _ref_priority_ : libc::int32_t = 0 as libc::int32_t;
      let mut _ref_direction_ : i32 = 0 as i32;
      callMSK!(MSK_getvarbranchorder,self.ptr,j_ as libc::int32_t,& mut _ref_priority_,& mut _ref_direction_);
      return (_ref_priority_ as i32,_ref_direction_ as i32)
    }
    
    // getvarbranchpri
    pub fn get_var_branch_pri(&self,j_ : i32) -> i32
    {
      let mut _ref_priority_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getvarbranchpri,self.ptr,j_ as libc::int32_t,& mut _ref_priority_);
      _ref_priority_ as i32
    }
    
    // getvarname
    pub fn get_var_name(&self,j_ : i32) -> String
    {
      let tmp_var_3__ = self.get_var_name_len(j_);
      let maxlen_ = 1 + tmp_var_3__;
      let mut _name__bytes = Vec::with_capacity(maxlen_ as usize);
      callMSK!(MSK_getvarname,self.ptr,j_ as libc::int32_t,maxlen_ as libc::int32_t,_name__bytes.as_mut_ptr());
      unsafe { _name__bytes.set_len((maxlen_) as usize) };
      String::from_utf8_lossy(&_name__bytes[..]).into_owned()
    }
    
    // getvarnameindex
    pub fn get_var_name_index(&self,somename_ : &str) -> (i32,i32)
    {
      let mut _ref_asgn_ : libc::int32_t = 0 as libc::int32_t;
      let mut _ref_index_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getvarnameindex,self.ptr,CString::new(somename_).unwrap().as_ptr(),& mut _ref_asgn_,& mut _ref_index_);
      return (_ref_asgn_ as i32,_ref_index_ as i32)
    }
    
    // getvarnamelen
    pub fn get_var_name_len(&self,i_ : i32) -> i32
    {
      let mut _ref_len_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_getvarnamelen,self.ptr,i_ as libc::int32_t,& mut _ref_len_);
      _ref_len_ as i32
    }
    
    // getvartype
    pub fn get_var_type(&self,j_ : i32) -> i32
    {
      let mut _ref_vartype_ : i32 = 0 as i32;
      callMSK!(MSK_getvartype,self.ptr,j_ as libc::int32_t,& mut _ref_vartype_);
      _ref_vartype_ as i32
    }
    
    // getvartypelist
    pub fn get_var_type_list(&self,subj_ : & [i32],vartype_ : & mut [i32])
    {
      let mut num_ = subj_.len();
      if vartype_.len() != ((num_) as usize) { panic!("Argument 'vartype_' is too short in call to 'get_var_type_list'") }
      callMSK!(MSK_getvartypelist,self.ptr,num_ as libc::int32_t,subj_.as_ptr(),vartype_.as_mut_ptr());
    }
    
    // getxc
    pub fn get_xc(&self,whichsol_ : i32,xc_ : & mut [f64])
    {
      let tmp_var_1__ = self.get_num_con();
      if xc_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'xc_' is too short in call to 'get_xc'") }
      callMSK!(MSK_getxc,self.ptr,whichsol_,xc_.as_mut_ptr());
    }
    
    // getxcslice
    pub fn get_xc_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,xc_ : & mut [f64])
    {
      if xc_.len() != ((last_ - first_) as usize) { panic!("Argument 'xc_' is too short in call to 'get_xc_slice'") }
      callMSK!(MSK_getxcslice,self.ptr,whichsol_,first_ as libc::int32_t,last_ as libc::int32_t,xc_.as_mut_ptr());
    }
    
    // getxx
    pub fn get_xx(&self,whichsol_ : i32,xx_ : & mut [f64])
    {
      let tmp_var_1__ = self.get_num_var();
      if xx_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'xx_' is too short in call to 'get_xx'") }
      callMSK!(MSK_getxx,self.ptr,whichsol_,xx_.as_mut_ptr());
    }
    
    // getxxslice
    pub fn get_xx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,xx_ : & mut [f64])
    {
      if xx_.len() != ((last_ - first_) as usize) { panic!("Argument 'xx_' is too short in call to 'get_xx_slice'") }
      callMSK!(MSK_getxxslice,self.ptr,whichsol_,first_ as libc::int32_t,last_ as libc::int32_t,xx_.as_mut_ptr());
    }
    
    // gety
    pub fn get_y(&self,whichsol_ : i32,y_ : & mut [f64])
    {
      let tmp_var_1__ = self.get_num_con();
      if y_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'y_' is too short in call to 'get_y'") }
      callMSK!(MSK_gety,self.ptr,whichsol_,y_.as_mut_ptr());
    }
    
    // getyslice
    pub fn get_y_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,y_ : & mut [f64])
    {
      if y_.len() != ((last_ - first_) as usize) { panic!("Argument 'y_' is too short in call to 'get_y_slice'") }
      callMSK!(MSK_getyslice,self.ptr,whichsol_,first_ as libc::int32_t,last_ as libc::int32_t,y_.as_mut_ptr());
    }
    
    // initbasissolve
    pub fn init_basis_solve(&self,basis_ : & mut [i32])
    {
      let tmp_var_1__ = self.get_num_con();
      if basis_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'basis_' is too short in call to 'init_basis_solve'") }
      callMSK!(MSK_initbasissolve,self.ptr,basis_.as_mut_ptr());
    }
    
    // inputdata64
    pub fn input_data(&self,maxnumcon_ : i32,maxnumvar_ : i32,c_ : & [f64],cfix_ : f64,aptrb_ : & [i64],aptre_ : & [i64],asub_ : & [i32],aval_ : & [f64],bkc_ : & [i32],blc_ : & [f64],buc_ : & [f64],bkx_ : & [i32],blx_ : & [f64],bux_ : & [f64])
    {
      let mut numcon_ = buc_.len();
      if blc_.len() > numcon_ { numcon_ = blc_.len() };
      if bkc_.len() > numcon_ { numcon_ = bkc_.len() };
      let mut numvar_ = c_.len();
      if bux_.len() > numvar_ { numvar_ = bux_.len() };
      if blx_.len() > numvar_ { numvar_ = blx_.len() };
      if bkx_.len() > numvar_ { numvar_ = bkx_.len() };
      if aptrb_.len() > numvar_ { numvar_ = aptrb_.len() };
      if aptre_.len() > numvar_ { numvar_ = aptre_.len() };
      callMSK!(MSK_inputdata64,self.ptr,maxnumcon_ as libc::int32_t,maxnumvar_ as libc::int32_t,numcon_ as libc::int32_t,numvar_ as libc::int32_t,c_.as_ptr(),cfix_ as f64,aptrb_.as_ptr(),aptre_.as_ptr(),asub_.as_ptr(),aval_.as_ptr(),bkc_.as_ptr(),blc_.as_ptr(),buc_.as_ptr(),bkx_.as_ptr(),blx_.as_ptr(),bux_.as_ptr());
    }
    
    // isdouparname
    pub fn is_dou_par_name(&self,parname_ : &str) -> i32
    {
      let mut _ref_param_ : i32 = 0 as i32;
      callMSK!(MSK_isdouparname,self.ptr,CString::new(parname_).unwrap().as_ptr(),& mut _ref_param_);
      _ref_param_ as i32
    }
    
    // isintparname
    pub fn is_int_par_name(&self,parname_ : &str) -> i32
    {
      let mut _ref_param_ : i32 = 0 as i32;
      callMSK!(MSK_isintparname,self.ptr,CString::new(parname_).unwrap().as_ptr(),& mut _ref_param_);
      _ref_param_ as i32
    }
    
    // isstrparname
    pub fn is_str_par_name(&self,parname_ : &str) -> i32
    {
      let mut _ref_param_ : i32 = 0 as i32;
      callMSK!(MSK_isstrparname,self.ptr,CString::new(parname_).unwrap().as_ptr(),& mut _ref_param_);
      _ref_param_ as i32
    }
    
    // linkfiletotaskstream
    pub fn link_file_to_stream(&self,whichstream_ : i32,filename_ : &str,append_ : i32)
    {
      callMSK!(MSK_linkfiletotaskstream,self.ptr,whichstream_,CString::new(filename_).unwrap().as_ptr(),append_ as libc::int32_t);
    }
    
    // onesolutionsummary
    pub fn one_solution_summary(&self,whichstream_ : i32,whichsol_ : i32)
    {
      callMSK!(MSK_onesolutionsummary,self.ptr,whichstream_,whichsol_);
    }
    
    // optimizersummary
    pub fn optimizer_summary(&self,whichstream_ : i32)
    {
      callMSK!(MSK_optimizersummary,self.ptr,whichstream_);
    }
    
    // optimizetrm
    pub fn optimize(&self) -> i32
    {
      let mut _ref_trmcode_ : i32 = 0 as i32;
      callMSK!(MSK_optimizetrm,self.ptr,& mut _ref_trmcode_);
      _ref_trmcode_ as i32
    }
    
    // primalrepair
    pub fn primal_repair(&self,wlc_ : & [f64],wuc_ : & [f64],wlx_ : & [f64],wux_ : & [f64])
    {
      let tmp_var_1__ = self.get_num_con();
      if wlc_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'wlc_' is too short in call to 'primal_repair'") }
      let tmp_var_3__ = self.get_num_con();
      if wuc_.len() != ((tmp_var_3__) as usize) { panic!("Argument 'wuc_' is too short in call to 'primal_repair'") }
      let tmp_var_5__ = self.get_num_var();
      if wlx_.len() != ((tmp_var_5__) as usize) { panic!("Argument 'wlx_' is too short in call to 'primal_repair'") }
      let tmp_var_7__ = self.get_num_var();
      if wux_.len() != ((tmp_var_7__) as usize) { panic!("Argument 'wux_' is too short in call to 'primal_repair'") }
      callMSK!(MSK_primalrepair,self.ptr,wlc_.as_ptr(),wuc_.as_ptr(),wlx_.as_ptr(),wux_.as_ptr());
    }
    
    // primalsensitivity
    pub fn primal_sensitivity(&self,subi_ : & [i32],marki_ : & [i32],subj_ : & [i32],markj_ : & [i32],leftpricei_ : & mut [f64],rightpricei_ : & mut [f64],leftrangei_ : & mut [f64],rightrangei_ : & mut [f64],leftpricej_ : & mut [f64],rightpricej_ : & mut [f64],leftrangej_ : & mut [f64],rightrangej_ : & mut [f64])
    {
      let mut numi_ = subi_.len();
      if marki_.len() > numi_ { numi_ = marki_.len() };
      let mut numj_ = subj_.len();
      if markj_.len() > numj_ { numj_ = markj_.len() };
      if leftpricei_.len() != ((numi_) as usize) { panic!("Argument 'leftpricei_' is too short in call to 'primal_sensitivity'") }
      if rightpricei_.len() != ((numi_) as usize) { panic!("Argument 'rightpricei_' is too short in call to 'primal_sensitivity'") }
      if leftrangei_.len() != ((numi_) as usize) { panic!("Argument 'leftrangei_' is too short in call to 'primal_sensitivity'") }
      if rightrangei_.len() != ((numi_) as usize) { panic!("Argument 'rightrangei_' is too short in call to 'primal_sensitivity'") }
      if leftpricej_.len() != ((numj_) as usize) { panic!("Argument 'leftpricej_' is too short in call to 'primal_sensitivity'") }
      if rightpricej_.len() != ((numj_) as usize) { panic!("Argument 'rightpricej_' is too short in call to 'primal_sensitivity'") }
      if leftrangej_.len() != ((numj_) as usize) { panic!("Argument 'leftrangej_' is too short in call to 'primal_sensitivity'") }
      if rightrangej_.len() != ((numj_) as usize) { panic!("Argument 'rightrangej_' is too short in call to 'primal_sensitivity'") }
      callMSK!(MSK_primalsensitivity,self.ptr,numi_ as libc::int32_t,subi_.as_ptr(),marki_.as_ptr(),numj_ as libc::int32_t,subj_.as_ptr(),markj_.as_ptr(),leftpricei_.as_mut_ptr(),rightpricei_.as_mut_ptr(),leftrangei_.as_mut_ptr(),rightrangei_.as_mut_ptr(),leftpricej_.as_mut_ptr(),rightpricej_.as_mut_ptr(),leftrangej_.as_mut_ptr(),rightrangej_.as_mut_ptr());
    }
    
    // probtypetostr
    pub fn prob_type_to_str(&self,probtype_ : i32) -> String
    {
      let mut _str__bytes = Vec::with_capacity(MSK_MAX_STR_LEN as usize);
      callMSK!(MSK_probtypetostr,self.ptr,probtype_,_str__bytes.as_mut_ptr());
      unsafe { _str__bytes.set_len((MSK_MAX_STR_LEN) as usize) };
      String::from_utf8_lossy(&_str__bytes[..]).into_owned()
    }
    
    // prostatostr
    pub fn pro_sta_to_str(&self,prosta_ : i32) -> String
    {
      let mut _str__bytes = Vec::with_capacity(MSK_MAX_STR_LEN as usize);
      callMSK!(MSK_prostatostr,self.ptr,prosta_,_str__bytes.as_mut_ptr());
      unsafe { _str__bytes.set_len((MSK_MAX_STR_LEN) as usize) };
      String::from_utf8_lossy(&_str__bytes[..]).into_owned()
    }
    
    // putacol
    pub fn put_a_col(&self,j_ : i32,subj_ : & [i32],valj_ : & [f64])
    {
      let mut nzj_ = subj_.len();
      if valj_.len() > nzj_ { nzj_ = valj_.len() };
      callMSK!(MSK_putacol,self.ptr,j_ as libc::int32_t,nzj_ as libc::int32_t,subj_.as_ptr(),valj_.as_ptr());
    }
    
    // putacollist
    pub fn put_a_col_list(&self,sub_ : & [i32],ptrb_ : & [i32],ptre_ : & [i32],asub_ : & [i32],aval_ : & [f64])
    {
      let mut num_ = sub_.len();
      if ptrb_.len() > num_ { num_ = ptrb_.len() };
      if ptre_.len() > num_ { num_ = ptre_.len() };
      callMSK!(MSK_putacollist,self.ptr,num_ as libc::int32_t,sub_.as_ptr(),ptrb_.as_ptr(),ptre_.as_ptr(),asub_.as_ptr(),aval_.as_ptr());
    }
    
    // putacolslice64
    pub fn put_a_col_slice(&self,first_ : i32,last_ : i32,ptrb_ : & [i64],ptre_ : & [i64],asub_ : & [i32],aval_ : & [f64])
    {
      if ptrb_.len() != ((last_ - first_) as usize) { panic!("Argument 'ptrb_' is too short in call to 'put_a_col_slice'") }
      if ptre_.len() != ((last_ - first_) as usize) { panic!("Argument 'ptre_' is too short in call to 'put_a_col_slice'") }
      callMSK!(MSK_putacolslice64,self.ptr,first_ as libc::int32_t,last_ as libc::int32_t,ptrb_.as_ptr(),ptre_.as_ptr(),asub_.as_ptr(),aval_.as_ptr());
    }
    
    // putaij
    pub fn put_aij(&self,i_ : i32,j_ : i32,aij_ : f64)
    {
      callMSK!(MSK_putaij,self.ptr,i_ as libc::int32_t,j_ as libc::int32_t,aij_ as f64);
    }
    
    // putaijlist64
    pub fn put_aij_list(&self,subi_ : & [i32],subj_ : & [i32],valij_ : & [f64])
    {
      let mut num_ = subi_.len();
      if subj_.len() > num_ { num_ = subj_.len() };
      if valij_.len() > num_ { num_ = valij_.len() };
      callMSK!(MSK_putaijlist64,self.ptr,num_ as libc::int64_t,subi_.as_ptr(),subj_.as_ptr(),valij_.as_ptr());
    }
    
    // putarow
    pub fn put_a_row(&self,i_ : i32,subi_ : & [i32],vali_ : & [f64])
    {
      let mut nzi_ = subi_.len();
      if vali_.len() > nzi_ { nzi_ = vali_.len() };
      callMSK!(MSK_putarow,self.ptr,i_ as libc::int32_t,nzi_ as libc::int32_t,subi_.as_ptr(),vali_.as_ptr());
    }
    
    // putarowlist
    pub fn put_a_row_list(&self,sub_ : & [i32],aptrb_ : & [i32],aptre_ : & [i32],asub_ : & [i32],aval_ : & [f64])
    {
      let mut num_ = sub_.len();
      if aptrb_.len() > num_ { num_ = aptrb_.len() };
      if aptre_.len() > num_ { num_ = aptre_.len() };
      callMSK!(MSK_putarowlist,self.ptr,num_ as libc::int32_t,sub_.as_ptr(),aptrb_.as_ptr(),aptre_.as_ptr(),asub_.as_ptr(),aval_.as_ptr());
    }
    
    // putarowslice64
    pub fn put_a_row_slice(&self,first_ : i32,last_ : i32,ptrb_ : & [i64],ptre_ : & [i64],asub_ : & [i32],aval_ : & [f64])
    {
      if ptrb_.len() != ((last_ - first_) as usize) { panic!("Argument 'ptrb_' is too short in call to 'put_a_row_slice'") }
      if ptre_.len() != ((last_ - first_) as usize) { panic!("Argument 'ptre_' is too short in call to 'put_a_row_slice'") }
      callMSK!(MSK_putarowslice64,self.ptr,first_ as libc::int32_t,last_ as libc::int32_t,ptrb_.as_ptr(),ptre_.as_ptr(),asub_.as_ptr(),aval_.as_ptr());
    }
    
    // putbarablocktriplet
    pub fn put_bara_block_triplet(&self,num_ : i64,subi_ : & [i32],subj_ : & [i32],subk_ : & [i32],subl_ : & [i32],valijkl_ : & [f64])
    {
      if subi_.len() != ((num_) as usize) { panic!("Argument 'subi_' is too short in call to 'put_bara_block_triplet'") }
      if subj_.len() != ((num_) as usize) { panic!("Argument 'subj_' is too short in call to 'put_bara_block_triplet'") }
      if subk_.len() != ((num_) as usize) { panic!("Argument 'subk_' is too short in call to 'put_bara_block_triplet'") }
      if subl_.len() != ((num_) as usize) { panic!("Argument 'subl_' is too short in call to 'put_bara_block_triplet'") }
      if valijkl_.len() != ((num_) as usize) { panic!("Argument 'valijkl_' is too short in call to 'put_bara_block_triplet'") }
      callMSK!(MSK_putbarablocktriplet,self.ptr,num_ as libc::int64_t,subi_.as_ptr(),subj_.as_ptr(),subk_.as_ptr(),subl_.as_ptr(),valijkl_.as_ptr());
    }
    
    // putbaraij
    pub fn put_bar_aij(&self,i_ : i32,j_ : i32,sub_ : & [i64],weights_ : & [f64])
    {
      let mut num_ = sub_.len();
      if weights_.len() > num_ { num_ = weights_.len() };
      callMSK!(MSK_putbaraij,self.ptr,i_ as libc::int32_t,j_ as libc::int32_t,num_ as libc::int64_t,sub_.as_ptr(),weights_.as_ptr());
    }
    
    // putbarcblocktriplet
    pub fn put_barc_block_triplet(&self,num_ : i64,subj_ : & [i32],subk_ : & [i32],subl_ : & [i32],valjkl_ : & [f64])
    {
      if subj_.len() != ((num_) as usize) { panic!("Argument 'subj_' is too short in call to 'put_barc_block_triplet'") }
      if subk_.len() != ((num_) as usize) { panic!("Argument 'subk_' is too short in call to 'put_barc_block_triplet'") }
      if subl_.len() != ((num_) as usize) { panic!("Argument 'subl_' is too short in call to 'put_barc_block_triplet'") }
      if valjkl_.len() != ((num_) as usize) { panic!("Argument 'valjkl_' is too short in call to 'put_barc_block_triplet'") }
      callMSK!(MSK_putbarcblocktriplet,self.ptr,num_ as libc::int64_t,subj_.as_ptr(),subk_.as_ptr(),subl_.as_ptr(),valjkl_.as_ptr());
    }
    
    // putbarcj
    pub fn put_barc_j(&self,j_ : i32,sub_ : & [i64],weights_ : & [f64])
    {
      let mut num_ = sub_.len();
      if weights_.len() > num_ { num_ = weights_.len() };
      callMSK!(MSK_putbarcj,self.ptr,j_ as libc::int32_t,num_ as libc::int64_t,sub_.as_ptr(),weights_.as_ptr());
    }
    
    // putbarsj
    pub fn put_bars_j(&self,whichsol_ : i32,j_ : i32,barsj_ : & [f64])
    {
      let tmp_var_1__ = self.get_len_barvar_j(j_);
      if barsj_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'barsj_' is too short in call to 'put_bars_j'") }
      callMSK!(MSK_putbarsj,self.ptr,whichsol_,j_ as libc::int32_t,barsj_.as_ptr());
    }
    
    // putbarvarname
    pub fn put_barvar_name(&self,j_ : i32,name_ : &str)
    {
      callMSK!(MSK_putbarvarname,self.ptr,j_ as libc::int32_t,CString::new(name_).unwrap().as_ptr());
    }
    
    // putbarxj
    pub fn put_barx_j(&self,whichsol_ : i32,j_ : i32,barxj_ : & [f64])
    {
      let tmp_var_1__ = self.get_len_barvar_j(j_);
      if barxj_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'barxj_' is too short in call to 'put_barx_j'") }
      callMSK!(MSK_putbarxj,self.ptr,whichsol_,j_ as libc::int32_t,barxj_.as_ptr());
    }
    
    // putbound
    pub fn put_bound(&self,accmode_ : i32,i_ : i32,bk_ : i32,bl_ : f64,bu_ : f64)
    {
      callMSK!(MSK_putbound,self.ptr,accmode_,i_ as libc::int32_t,bk_,bl_ as f64,bu_ as f64);
    }
    
    // putboundlist
    pub fn put_bound_list(&self,accmode_ : i32,sub_ : & [i32],bk_ : & [i32],bl_ : & [f64],bu_ : & [f64])
    {
      let mut num_ = sub_.len();
      if bk_.len() > num_ { num_ = bk_.len() };
      if bl_.len() > num_ { num_ = bl_.len() };
      if bu_.len() > num_ { num_ = bu_.len() };
      callMSK!(MSK_putboundlist,self.ptr,accmode_,num_ as libc::int32_t,sub_.as_ptr(),bk_.as_ptr(),bl_.as_ptr(),bu_.as_ptr());
    }
    
    // putboundslice
    pub fn put_bound_slice(&self,con_ : i32,first_ : i32,last_ : i32,bk_ : & [i32],bl_ : & [f64],bu_ : & [f64])
    {
      if bk_.len() != ((last_ - first_) as usize) { panic!("Argument 'bk_' is too short in call to 'put_bound_slice'") }
      if bl_.len() != ((last_ - first_) as usize) { panic!("Argument 'bl_' is too short in call to 'put_bound_slice'") }
      if bu_.len() != ((last_ - first_) as usize) { panic!("Argument 'bu_' is too short in call to 'put_bound_slice'") }
      callMSK!(MSK_putboundslice,self.ptr,con_,first_ as libc::int32_t,last_ as libc::int32_t,bk_.as_ptr(),bl_.as_ptr(),bu_.as_ptr());
    }
    
    // putcfix
    pub fn put_cfix(&self,cfix_ : f64)
    {
      callMSK!(MSK_putcfix,self.ptr,cfix_ as f64);
    }
    
    // putcj
    pub fn put_c_j(&self,j_ : i32,cj_ : f64)
    {
      callMSK!(MSK_putcj,self.ptr,j_ as libc::int32_t,cj_ as f64);
    }
    
    // putclist
    pub fn put_c_list(&self,subj_ : & [i32],val_ : & [f64])
    {
      let mut num_ = subj_.len();
      if val_.len() > num_ { num_ = val_.len() };
      callMSK!(MSK_putclist,self.ptr,num_ as libc::int32_t,subj_.as_ptr(),val_.as_ptr());
    }
    
    // putconbound
    pub fn put_con_bound(&self,i_ : i32,bk_ : i32,bl_ : f64,bu_ : f64)
    {
      callMSK!(MSK_putconbound,self.ptr,i_ as libc::int32_t,bk_,bl_ as f64,bu_ as f64);
    }
    
    // putconboundlist
    pub fn put_con_bound_list(&self,sub_ : & [i32],bkc_ : & [i32],blc_ : & [f64],buc_ : & [f64])
    {
      let mut num_ = sub_.len();
      if bkc_.len() > num_ { num_ = bkc_.len() };
      if blc_.len() > num_ { num_ = blc_.len() };
      if buc_.len() > num_ { num_ = buc_.len() };
      callMSK!(MSK_putconboundlist,self.ptr,num_ as libc::int32_t,sub_.as_ptr(),bkc_.as_ptr(),blc_.as_ptr(),buc_.as_ptr());
    }
    
    // putconboundslice
    pub fn put_con_bound_slice(&self,first_ : i32,last_ : i32,bk_ : & [i32],bl_ : & [f64],bu_ : & [f64])
    {
      if bk_.len() != ((last_ - first_) as usize) { panic!("Argument 'bk_' is too short in call to 'put_con_bound_slice'") }
      if bl_.len() != ((last_ - first_) as usize) { panic!("Argument 'bl_' is too short in call to 'put_con_bound_slice'") }
      if bu_.len() != ((last_ - first_) as usize) { panic!("Argument 'bu_' is too short in call to 'put_con_bound_slice'") }
      callMSK!(MSK_putconboundslice,self.ptr,first_ as libc::int32_t,last_ as libc::int32_t,bk_.as_ptr(),bl_.as_ptr(),bu_.as_ptr());
    }
    
    // putcone
    pub fn put_cone(&self,k_ : i32,ct_ : i32,conepar_ : f64,submem_ : & [i32])
    {
      let mut nummem_ = submem_.len();
      callMSK!(MSK_putcone,self.ptr,k_ as libc::int32_t,ct_,conepar_ as f64,nummem_ as libc::int32_t,submem_.as_ptr());
    }
    
    // putconename
    pub fn put_cone_name(&self,j_ : i32,name_ : &str)
    {
      callMSK!(MSK_putconename,self.ptr,j_ as libc::int32_t,CString::new(name_).unwrap().as_ptr());
    }
    
    // putconname
    pub fn put_con_name(&self,i_ : i32,name_ : &str)
    {
      callMSK!(MSK_putconname,self.ptr,i_ as libc::int32_t,CString::new(name_).unwrap().as_ptr());
    }
    
    // putcslice
    pub fn put_c_slice(&self,first_ : i32,last_ : i32,slice_ : & [f64])
    {
      if slice_.len() != ((last_ - first_) as usize) { panic!("Argument 'slice_' is too short in call to 'put_c_slice'") }
      callMSK!(MSK_putcslice,self.ptr,first_ as libc::int32_t,last_ as libc::int32_t,slice_.as_ptr());
    }
    
    // putdouparam
    pub fn put_dou_param(&self,param_ : i32,parvalue_ : f64)
    {
      callMSK!(MSK_putdouparam,self.ptr,param_,parvalue_ as f64);
    }
    
    // putintparam
    pub fn put_int_param(&self,param_ : i32,parvalue_ : i32)
    {
      callMSK!(MSK_putintparam,self.ptr,param_,parvalue_ as libc::int32_t);
    }
    
    // putmaxnumanz
    pub fn put_max_num_a_nz(&self,maxnumanz_ : i64)
    {
      callMSK!(MSK_putmaxnumanz,self.ptr,maxnumanz_ as libc::int64_t);
    }
    
    // putmaxnumbarvar
    pub fn put_max_num_barvar(&self,maxnumbarvar_ : i32)
    {
      callMSK!(MSK_putmaxnumbarvar,self.ptr,maxnumbarvar_ as libc::int32_t);
    }
    
    // putmaxnumcon
    pub fn put_max_num_con(&self,maxnumcon_ : i32)
    {
      callMSK!(MSK_putmaxnumcon,self.ptr,maxnumcon_ as libc::int32_t);
    }
    
    // putmaxnumcone
    pub fn put_max_num_cone(&self,maxnumcone_ : i32)
    {
      callMSK!(MSK_putmaxnumcone,self.ptr,maxnumcone_ as libc::int32_t);
    }
    
    // putmaxnumqnz
    pub fn put_max_num_q_nz(&self,maxnumqnz_ : i64)
    {
      callMSK!(MSK_putmaxnumqnz,self.ptr,maxnumqnz_ as libc::int64_t);
    }
    
    // putmaxnumvar
    pub fn put_max_num_var(&self,maxnumvar_ : i32)
    {
      callMSK!(MSK_putmaxnumvar,self.ptr,maxnumvar_ as libc::int32_t);
    }
    
    // putnadouparam
    pub fn put_na_dou_param(&self,paramname_ : &str,parvalue_ : f64)
    {
      callMSK!(MSK_putnadouparam,self.ptr,CString::new(paramname_).unwrap().as_ptr(),parvalue_ as f64);
    }
    
    // putnaintparam
    pub fn put_na_int_param(&self,paramname_ : &str,parvalue_ : i32)
    {
      callMSK!(MSK_putnaintparam,self.ptr,CString::new(paramname_).unwrap().as_ptr(),parvalue_ as libc::int32_t);
    }
    
    // putnastrparam
    pub fn put_na_str_param(&self,paramname_ : &str,parvalue_ : &str)
    {
      callMSK!(MSK_putnastrparam,self.ptr,CString::new(paramname_).unwrap().as_ptr(),CString::new(parvalue_).unwrap().as_ptr());
    }
    
    // putobjname
    pub fn put_obj_name(&self,objname_ : &str)
    {
      callMSK!(MSK_putobjname,self.ptr,CString::new(objname_).unwrap().as_ptr());
    }
    
    // putobjsense
    pub fn put_obj_sense(&self,sense_ : i32)
    {
      callMSK!(MSK_putobjsense,self.ptr,sense_);
    }
    
    // putparam
    pub fn put_param(&self,parname_ : &str,parvalue_ : &str)
    {
      callMSK!(MSK_putparam,self.ptr,CString::new(parname_).unwrap().as_ptr(),CString::new(parvalue_).unwrap().as_ptr());
    }
    
    // putqcon
    pub fn put_q_con(&self,qcsubk_ : & [i32],qcsubi_ : & [i32],qcsubj_ : & [i32],qcval_ : & [f64])
    {
      let mut numqcnz_ = qcsubi_.len();
      if qcsubj_.len() > numqcnz_ { numqcnz_ = qcsubj_.len() };
      if qcval_.len() > numqcnz_ { numqcnz_ = qcval_.len() };
      callMSK!(MSK_putqcon,self.ptr,numqcnz_ as libc::int32_t,qcsubk_.as_ptr(),qcsubi_.as_ptr(),qcsubj_.as_ptr(),qcval_.as_ptr());
    }
    
    // putqconk
    pub fn put_q_con_k(&self,k_ : i32,qcsubi_ : & [i32],qcsubj_ : & [i32],qcval_ : & [f64])
    {
      let mut numqcnz_ = qcsubi_.len();
      if qcsubj_.len() > numqcnz_ { numqcnz_ = qcsubj_.len() };
      if qcval_.len() > numqcnz_ { numqcnz_ = qcval_.len() };
      callMSK!(MSK_putqconk,self.ptr,k_ as libc::int32_t,numqcnz_ as libc::int32_t,qcsubi_.as_ptr(),qcsubj_.as_ptr(),qcval_.as_ptr());
    }
    
    // putqobj
    pub fn put_q_obj(&self,qosubi_ : & [i32],qosubj_ : & [i32],qoval_ : & [f64])
    {
      let mut numqonz_ = qosubi_.len();
      if qosubj_.len() > numqonz_ { numqonz_ = qosubj_.len() };
      if qoval_.len() > numqonz_ { numqonz_ = qoval_.len() };
      callMSK!(MSK_putqobj,self.ptr,numqonz_ as libc::int32_t,qosubi_.as_ptr(),qosubj_.as_ptr(),qoval_.as_ptr());
    }
    
    // putqobjij
    pub fn put_q_obj_i_j(&self,i_ : i32,j_ : i32,qoij_ : f64)
    {
      callMSK!(MSK_putqobjij,self.ptr,i_ as libc::int32_t,j_ as libc::int32_t,qoij_ as f64);
    }
    
    // putskc
    pub fn put_skc(&self,whichsol_ : i32,skc_ : & [i32])
    {
      let tmp_var_1__ = self.get_num_con();
      if skc_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'skc_' is too short in call to 'put_skc'") }
      callMSK!(MSK_putskc,self.ptr,whichsol_,skc_.as_ptr());
    }
    
    // putskcslice
    pub fn put_skc_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,skc_ : & [i32])
    {
      if skc_.len() != ((last_ - first_) as usize) { panic!("Argument 'skc_' is too short in call to 'put_skc_slice'") }
      callMSK!(MSK_putskcslice,self.ptr,whichsol_,first_ as libc::int32_t,last_ as libc::int32_t,skc_.as_ptr());
    }
    
    // putskx
    pub fn put_skx(&self,whichsol_ : i32,skx_ : & [i32])
    {
      let tmp_var_1__ = self.get_num_var();
      if skx_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'skx_' is too short in call to 'put_skx'") }
      callMSK!(MSK_putskx,self.ptr,whichsol_,skx_.as_ptr());
    }
    
    // putskxslice
    pub fn put_skx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,skx_ : & [i32])
    {
      if skx_.len() != ((last_ - first_) as usize) { panic!("Argument 'skx_' is too short in call to 'put_skx_slice'") }
      callMSK!(MSK_putskxslice,self.ptr,whichsol_,first_ as libc::int32_t,last_ as libc::int32_t,skx_.as_ptr());
    }
    
    // putslc
    pub fn put_slc(&self,whichsol_ : i32,slc_ : & [f64])
    {
      let tmp_var_1__ = self.get_num_con();
      if slc_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'slc_' is too short in call to 'put_slc'") }
      callMSK!(MSK_putslc,self.ptr,whichsol_,slc_.as_ptr());
    }
    
    // putslcslice
    pub fn put_slc_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,slc_ : & [f64])
    {
      if slc_.len() != ((last_ - first_) as usize) { panic!("Argument 'slc_' is too short in call to 'put_slc_slice'") }
      callMSK!(MSK_putslcslice,self.ptr,whichsol_,first_ as libc::int32_t,last_ as libc::int32_t,slc_.as_ptr());
    }
    
    // putslx
    pub fn put_slx(&self,whichsol_ : i32,slx_ : & [f64])
    {
      let tmp_var_1__ = self.get_num_var();
      if slx_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'slx_' is too short in call to 'put_slx'") }
      callMSK!(MSK_putslx,self.ptr,whichsol_,slx_.as_ptr());
    }
    
    // putslxslice
    pub fn put_slx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,slx_ : & [f64])
    {
      if slx_.len() != ((last_ - first_) as usize) { panic!("Argument 'slx_' is too short in call to 'put_slx_slice'") }
      callMSK!(MSK_putslxslice,self.ptr,whichsol_,first_ as libc::int32_t,last_ as libc::int32_t,slx_.as_ptr());
    }
    
    // putsnx
    pub fn put_snx(&self,whichsol_ : i32,sux_ : & [f64])
    {
      let tmp_var_1__ = self.get_num_var();
      if sux_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'sux_' is too short in call to 'put_snx'") }
      callMSK!(MSK_putsnx,self.ptr,whichsol_,sux_.as_ptr());
    }
    
    // putsnxslice
    pub fn put_snx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,snx_ : & [f64])
    {
      if snx_.len() != ((last_ - first_) as usize) { panic!("Argument 'snx_' is too short in call to 'put_snx_slice'") }
      callMSK!(MSK_putsnxslice,self.ptr,whichsol_,first_ as libc::int32_t,last_ as libc::int32_t,snx_.as_ptr());
    }
    
    // putsolution
    pub fn put_solution(&self,whichsol_ : i32,skc_ : & [i32],skx_ : & [i32],skn_ : & [i32],xc_ : & [f64],xx_ : & [f64],y_ : & [f64],slc_ : & [f64],suc_ : & [f64],slx_ : & [f64],sux_ : & [f64],snx_ : & [f64])
    {
      callMSK!(MSK_putsolution,self.ptr,whichsol_,skc_.as_ptr(),skx_.as_ptr(),skn_.as_ptr(),xc_.as_ptr(),xx_.as_ptr(),y_.as_ptr(),slc_.as_ptr(),suc_.as_ptr(),slx_.as_ptr(),sux_.as_ptr(),snx_.as_ptr());
    }
    
    // putsolutioni
    pub fn put_solution_i(&self,accmode_ : i32,i_ : i32,whichsol_ : i32,sk_ : i32,x_ : f64,sl_ : f64,su_ : f64,sn_ : f64)
    {
      callMSK!(MSK_putsolutioni,self.ptr,accmode_,i_ as libc::int32_t,whichsol_,sk_,x_ as f64,sl_ as f64,su_ as f64,sn_ as f64);
    }
    
    // putsolutionyi
    pub fn put_solution_y_i(&self,i_ : i32,whichsol_ : i32,y_ : f64)
    {
      callMSK!(MSK_putsolutionyi,self.ptr,i_ as libc::int32_t,whichsol_,y_ as f64);
    }
    
    // putstrparam
    pub fn put_str_param(&self,param_ : i32,parvalue_ : &str)
    {
      callMSK!(MSK_putstrparam,self.ptr,param_,CString::new(parvalue_).unwrap().as_ptr());
    }
    
    // putsuc
    pub fn put_suc(&self,whichsol_ : i32,suc_ : & [f64])
    {
      let tmp_var_1__ = self.get_num_con();
      if suc_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'suc_' is too short in call to 'put_suc'") }
      callMSK!(MSK_putsuc,self.ptr,whichsol_,suc_.as_ptr());
    }
    
    // putsucslice
    pub fn put_suc_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,suc_ : & [f64])
    {
      if suc_.len() != ((last_ - first_) as usize) { panic!("Argument 'suc_' is too short in call to 'put_suc_slice'") }
      callMSK!(MSK_putsucslice,self.ptr,whichsol_,first_ as libc::int32_t,last_ as libc::int32_t,suc_.as_ptr());
    }
    
    // putsux
    pub fn put_sux(&self,whichsol_ : i32,sux_ : & [f64])
    {
      let tmp_var_1__ = self.get_num_var();
      if sux_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'sux_' is too short in call to 'put_sux'") }
      callMSK!(MSK_putsux,self.ptr,whichsol_,sux_.as_ptr());
    }
    
    // putsuxslice
    pub fn put_sux_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,sux_ : & [f64])
    {
      if sux_.len() != ((last_ - first_) as usize) { panic!("Argument 'sux_' is too short in call to 'put_sux_slice'") }
      callMSK!(MSK_putsuxslice,self.ptr,whichsol_,first_ as libc::int32_t,last_ as libc::int32_t,sux_.as_ptr());
    }
    
    // puttaskname
    pub fn put_task_name(&self,taskname_ : &str)
    {
      callMSK!(MSK_puttaskname,self.ptr,CString::new(taskname_).unwrap().as_ptr());
    }
    
    // putvarbound
    pub fn put_var_bound(&self,j_ : i32,bk_ : i32,bl_ : f64,bu_ : f64)
    {
      callMSK!(MSK_putvarbound,self.ptr,j_ as libc::int32_t,bk_,bl_ as f64,bu_ as f64);
    }
    
    // putvarboundlist
    pub fn put_var_bound_list(&self,sub_ : & [i32],bkx_ : & [i32],blx_ : & [f64],bux_ : & [f64])
    {
      let mut num_ = sub_.len();
      if bkx_.len() > num_ { num_ = bkx_.len() };
      if blx_.len() > num_ { num_ = blx_.len() };
      if bux_.len() > num_ { num_ = bux_.len() };
      callMSK!(MSK_putvarboundlist,self.ptr,num_ as libc::int32_t,sub_.as_ptr(),bkx_.as_ptr(),blx_.as_ptr(),bux_.as_ptr());
    }
    
    // putvarboundslice
    pub fn put_var_bound_slice(&self,first_ : i32,last_ : i32,bk_ : & [i32],bl_ : & [f64],bu_ : & [f64])
    {
      if bk_.len() != ((last_ - first_) as usize) { panic!("Argument 'bk_' is too short in call to 'put_var_bound_slice'") }
      if bl_.len() != ((last_ - first_) as usize) { panic!("Argument 'bl_' is too short in call to 'put_var_bound_slice'") }
      if bu_.len() != ((last_ - first_) as usize) { panic!("Argument 'bu_' is too short in call to 'put_var_bound_slice'") }
      callMSK!(MSK_putvarboundslice,self.ptr,first_ as libc::int32_t,last_ as libc::int32_t,bk_.as_ptr(),bl_.as_ptr(),bu_.as_ptr());
    }
    
    // putvarbranchorder
    pub fn put_var_branch_order(&self,j_ : i32,priority_ : i32,direction_ : i32)
    {
      callMSK!(MSK_putvarbranchorder,self.ptr,j_ as libc::int32_t,priority_ as libc::int32_t,direction_);
    }
    
    // putvarname
    pub fn put_var_name(&self,j_ : i32,name_ : &str)
    {
      callMSK!(MSK_putvarname,self.ptr,j_ as libc::int32_t,CString::new(name_).unwrap().as_ptr());
    }
    
    // putvartype
    pub fn put_var_type(&self,j_ : i32,vartype_ : i32)
    {
      callMSK!(MSK_putvartype,self.ptr,j_ as libc::int32_t,vartype_);
    }
    
    // putvartypelist
    pub fn put_var_type_list(&self,subj_ : & [i32],vartype_ : & [i32])
    {
      let mut num_ = subj_.len();
      if vartype_.len() > num_ { num_ = vartype_.len() };
      callMSK!(MSK_putvartypelist,self.ptr,num_ as libc::int32_t,subj_.as_ptr(),vartype_.as_ptr());
    }
    
    // putxc
    pub fn put_xc(&self,whichsol_ : i32,xc_ : & mut [f64])
    {
      let tmp_var_1__ = self.get_num_con();
      if xc_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'xc_' is too short in call to 'put_xc'") }
      callMSK!(MSK_putxc,self.ptr,whichsol_,xc_.as_mut_ptr());
    }
    
    // putxcslice
    pub fn put_xc_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,xc_ : & [f64])
    {
      if xc_.len() != ((last_ - first_) as usize) { panic!("Argument 'xc_' is too short in call to 'put_xc_slice'") }
      callMSK!(MSK_putxcslice,self.ptr,whichsol_,first_ as libc::int32_t,last_ as libc::int32_t,xc_.as_ptr());
    }
    
    // putxx
    pub fn put_xx(&self,whichsol_ : i32,xx_ : & [f64])
    {
      let tmp_var_1__ = self.get_num_var();
      if xx_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'xx_' is too short in call to 'put_xx'") }
      callMSK!(MSK_putxx,self.ptr,whichsol_,xx_.as_ptr());
    }
    
    // putxxslice
    pub fn put_xx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,xx_ : & [f64])
    {
      if xx_.len() != ((last_ - first_) as usize) { panic!("Argument 'xx_' is too short in call to 'put_xx_slice'") }
      callMSK!(MSK_putxxslice,self.ptr,whichsol_,first_ as libc::int32_t,last_ as libc::int32_t,xx_.as_ptr());
    }
    
    // puty
    pub fn put_y(&self,whichsol_ : i32,y_ : & [f64])
    {
      let tmp_var_1__ = self.get_num_con();
      if y_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'y_' is too short in call to 'put_y'") }
      callMSK!(MSK_puty,self.ptr,whichsol_,y_.as_ptr());
    }
    
    // putyslice
    pub fn put_y_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,y_ : & [f64])
    {
      if y_.len() != ((last_ - first_) as usize) { panic!("Argument 'y_' is too short in call to 'put_y_slice'") }
      callMSK!(MSK_putyslice,self.ptr,whichsol_,first_ as libc::int32_t,last_ as libc::int32_t,y_.as_ptr());
    }
    
    // readbranchpriorities
    pub fn read_branch_priorities(&self,filename_ : &str)
    {
      callMSK!(MSK_readbranchpriorities,self.ptr,CString::new(filename_).unwrap().as_ptr());
    }
    
    // readdataautoformat
    pub fn read_data(&self,filename_ : &str)
    {
      callMSK!(MSK_readdataautoformat,self.ptr,CString::new(filename_).unwrap().as_ptr());
    }
    
    // readdataformat
    pub fn read_data_format(&self,filename_ : &str,format_ : i32,compress_ : i32)
    {
      callMSK!(MSK_readdataformat,self.ptr,CString::new(filename_).unwrap().as_ptr(),format_,compress_);
    }
    
    // readparamfile
    pub fn read_param_file(&self,filename_ : &str)
    {
      callMSK!(MSK_readparamfile,self.ptr,CString::new(filename_).unwrap().as_ptr());
    }
    
    // readsolution
    pub fn read_solution(&self,whichsol_ : i32,filename_ : &str)
    {
      callMSK!(MSK_readsolution,self.ptr,whichsol_,CString::new(filename_).unwrap().as_ptr());
    }
    
    // readsummary
    pub fn read_summary(&self,whichstream_ : i32)
    {
      callMSK!(MSK_readsummary,self.ptr,whichstream_);
    }
    
    // readtask
    pub fn read_task(&self,filename_ : &str)
    {
      callMSK!(MSK_readtask,self.ptr,CString::new(filename_).unwrap().as_ptr());
    }
    
    // removebarvars
    pub fn remove_barvars(&self,subset_ : & [i32])
    {
      let mut num_ = subset_.len();
      callMSK!(MSK_removebarvars,self.ptr,num_ as libc::int32_t,subset_.as_ptr());
    }
    
    // removecones
    pub fn remove_cones(&self,subset_ : & [i32])
    {
      let mut num_ = subset_.len();
      callMSK!(MSK_removecones,self.ptr,num_ as libc::int32_t,subset_.as_ptr());
    }
    
    // removecons
    pub fn remove_cons(&self,subset_ : & [i32])
    {
      let mut num_ = subset_.len();
      callMSK!(MSK_removecons,self.ptr,num_ as libc::int32_t,subset_.as_ptr());
    }
    
    // removevars
    pub fn remove_vars(&self,subset_ : & [i32])
    {
      let mut num_ = subset_.len();
      callMSK!(MSK_removevars,self.ptr,num_ as libc::int32_t,subset_.as_ptr());
    }
    
    // resizetask
    pub fn resize_task(&self,maxnumcon_ : i32,maxnumvar_ : i32,maxnumcone_ : i32,maxnumanz_ : i64,maxnumqnz_ : i64)
    {
      callMSK!(MSK_resizetask,self.ptr,maxnumcon_ as libc::int32_t,maxnumvar_ as libc::int32_t,maxnumcone_ as libc::int32_t,maxnumanz_ as libc::int64_t,maxnumqnz_ as libc::int64_t);
    }
    
    // sensitivityreport
    pub fn sensitivity_report(&self,whichstream_ : i32)
    {
      callMSK!(MSK_sensitivityreport,self.ptr,whichstream_);
    }
    
    // setdefaults
    pub fn set_defaults(&self)
    {
      callMSK!(MSK_setdefaults,self.ptr);
    }
    
    // sktostr
    pub fn sk_to_str(&self,sk_ : i32) -> String
    {
      let mut _str__bytes = Vec::with_capacity(MSK_MAX_STR_LEN as usize);
      callMSK!(MSK_sktostr,self.ptr,sk_,_str__bytes.as_mut_ptr());
      unsafe { _str__bytes.set_len((MSK_MAX_STR_LEN) as usize) };
      String::from_utf8_lossy(&_str__bytes[..]).into_owned()
    }
    
    // solstatostr
    pub fn sol_sta_to_str(&self,solsta_ : i32) -> String
    {
      let mut _str__bytes = Vec::with_capacity(MSK_MAX_STR_LEN as usize);
      callMSK!(MSK_solstatostr,self.ptr,solsta_,_str__bytes.as_mut_ptr());
      unsafe { _str__bytes.set_len((MSK_MAX_STR_LEN) as usize) };
      String::from_utf8_lossy(&_str__bytes[..]).into_owned()
    }
    
    // solutiondef
    pub fn solution_def(&self,whichsol_ : i32) -> bool
    {
      let mut _ref_isdef_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_solutiondef,self.ptr,whichsol_,& mut _ref_isdef_);
      _ref_isdef_ != 0
    }
    
    // solutionsummary
    pub fn solution_summary(&self,whichstream_ : i32)
    {
      callMSK!(MSK_solutionsummary,self.ptr,whichstream_);
    }
    
    // solvewithbasis
    pub fn solve_with_basis(&self,transp_ : i32,numnz_ : i32,sub_ : & mut [i32],val_ : & mut [f64]) -> i32
    {
      let mut _ref_numnz_ = numnz_;
      let tmp_var_1__ = self.get_num_con();
      if sub_.len() != ((tmp_var_1__) as usize) { panic!("Argument 'sub_' is too short in call to 'solve_with_basis'") }
      let tmp_var_3__ = self.get_num_con();
      if val_.len() != ((tmp_var_3__) as usize) { panic!("Argument 'val_' is too short in call to 'solve_with_basis'") }
      callMSK!(MSK_solvewithbasis,self.ptr,transp_ as libc::int32_t,& mut _ref_numnz_,sub_.as_mut_ptr(),val_.as_mut_ptr());
      _ref_numnz_ as i32
    }
    
    // startstat
    pub fn start_stat(&self)
    {
      callMSK!(MSK_startstat,self.ptr);
    }
    
    // stopstat
    pub fn stop_stat(&self)
    {
      callMSK!(MSK_stopstat,self.ptr);
    }
    
    // strtoconetype
    pub fn str_to_cone_type(&self,str_ : &str) -> i32
    {
      let mut _ref_conetype_ : i32 = 0 as i32;
      callMSK!(MSK_strtoconetype,self.ptr,CString::new(str_).unwrap().as_ptr(),& mut _ref_conetype_);
      _ref_conetype_ as i32
    }
    
    // strtosk
    pub fn str_to_sk(&self,str_ : &str) -> i32
    {
      let mut _ref_sk_ : libc::int32_t = 0 as libc::int32_t;
      callMSK!(MSK_strtosk,self.ptr,CString::new(str_).unwrap().as_ptr(),& mut _ref_sk_);
      _ref_sk_ as i32
    }
    
    // toconic
    pub fn toconic(&self)
    {
      callMSK!(MSK_toconic,self.ptr);
    }
    
    // updatesolutioninfo
    pub fn update_solution_info(&self,whichsol_ : i32)
    {
      callMSK!(MSK_updatesolutioninfo,self.ptr,whichsol_);
    }
    
    // writebranchpriorities
    pub fn write_branch_priorities(&self,filename_ : &str)
    {
      callMSK!(MSK_writebranchpriorities,self.ptr,CString::new(filename_).unwrap().as_ptr());
    }
    
    // writedata
    pub fn write_data(&self,filename_ : &str)
    {
      callMSK!(MSK_writedata,self.ptr,CString::new(filename_).unwrap().as_ptr());
    }
    
    // writeparamfile
    pub fn write_param_file(&self,filename_ : &str)
    {
      callMSK!(MSK_writeparamfile,self.ptr,CString::new(filename_).unwrap().as_ptr());
    }
    
    // writesolution
    pub fn write_solution(&self,whichsol_ : i32,filename_ : &str)
    {
      callMSK!(MSK_writesolution,self.ptr,whichsol_,CString::new(filename_).unwrap().as_ptr());
    }
    
    // writetask
    pub fn write_task(&self,filename_ : &str)
    {
      callMSK!(MSK_writetask,self.ptr,CString::new(filename_).unwrap().as_ptr());
    }
}


impl Drop for Env
{
    fn drop( & mut self)
    {
        let mut env = self.ptr;
        unsafe { MSK_deleteenv(& mut env); };
    }
}

impl Drop for Task
{
    fn drop( & mut self)
    {
        let mut task = self.ptr;
        unsafe { MSK_deletetask(& mut task) };
    }
}



// getcodedesc
pub fn get_code_desc(code_ : i32) -> (String,String)
{
  let mut _symname__bytes = Vec::with_capacity(MSK_MAX_STR_LEN as usize);
  let mut _str__bytes = Vec::with_capacity(MSK_MAX_STR_LEN as usize);
  callMSK!(MSK_getcodedesc,code_,_symname__bytes.as_mut_ptr(),_str__bytes.as_mut_ptr());
  unsafe { _symname__bytes.set_len((MSK_MAX_STR_LEN) as usize) };
  unsafe { _str__bytes.set_len((MSK_MAX_STR_LEN) as usize) };
  return (String::from_utf8_lossy(&_symname__bytes[..]).into_owned(),String::from_utf8_lossy(&_str__bytes[..]).into_owned())
}

// getversion
pub fn get_version() -> (i32,i32,i32,i32)
{
  let mut _ref_major_ : libc::int32_t = 0 as libc::int32_t;
  let mut _ref_minor_ : libc::int32_t = 0 as libc::int32_t;
  let mut _ref_build_ : libc::int32_t = 0 as libc::int32_t;
  let mut _ref_revision_ : libc::int32_t = 0 as libc::int32_t;
  callMSK!(MSK_getversion,& mut _ref_major_,& mut _ref_minor_,& mut _ref_build_,& mut _ref_revision_);
  return (_ref_major_ as i32,_ref_minor_ as i32,_ref_build_ as i32,_ref_revision_ as i32)
}

// licensecleanup
pub fn licensecleanup()
{
  callMSK!(MSK_licensecleanup);
}
