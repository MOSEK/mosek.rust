// Generated for MOSEK v10.0.2
extern crate libc;
use std::ffi::CString;
use std::ffi::CStr;
use libc::c_void;

pub mod conic_solver_mosek;
pub mod affine_conic_api;
mod namegen;

//#[link(name = "mosek64")]
extern {
    fn MSK_makeenv(env : * mut * const u8, dbgfile : * const libc::c_char) -> i32;
    fn MSK_deleteenv(env : * mut * const u8) -> i32;
    fn MSK_maketask(env : * const u8, maxnumcon : i32, maxnumvar : i32, task : * mut * const u8) -> u32;
    fn MSK_deletetask(task : * mut * const u8) -> i32;

    fn MSK_linkfunctotaskstream(task        : * const u8,
                                whichstream : i32,
                                handle      : * const c_void,
                                func        : extern fn (handle : * const c_void, msg : * const libc::c_char)) -> i32;

    fn MSK_putcallbackfunc(task        : * const u8,
                           func        : extern fn (task : * const c_void, handle : * const c_void, caller : i32, douinf : * const f64, intinf : * const i32, lintinf : * const i64) -> i32,
                           handle      : * const c_void) -> i32;
    fn MSK_getlasterror64(task         : * const u8,
                          lastreacode  : * mut i32,
                          sizelastmsg  : i64,
                          lastmsglen   : * mut i64,
                          lastmsg      : * mut u8) -> i32;
    fn MSK_analyzenames(task_ : * const u8,whichstream_ : i32,nametype_ : i32) -> i32;
    fn MSK_analyzeproblem(task_ : * const u8,whichstream_ : i32) -> i32;
    fn MSK_analyzesolution(task_ : * const u8,whichstream_ : i32,whichsol_ : i32) -> i32;
    fn MSK_appendacc(task_ : * const u8,domidx_ : i64,numafeidx_ : i64,afeidxlist_ : * const i64,b_ : * const f64) -> i32;
    fn MSK_appendaccs(task_ : * const u8,numaccs_ : i64,domidxs_ : * const i64,numafeidx_ : i64,afeidxlist_ : * const i64,b_ : * const f64) -> i32;
    fn MSK_appendaccseq(task_ : * const u8,domidx_ : i64,numafeidx_ : i64,afeidxfirst_ : i64,b_ : * const f64) -> i32;
    fn MSK_appendaccsseq(task_ : * const u8,numaccs_ : i64,domidxs_ : * const i64,numafeidx_ : i64,afeidxfirst_ : i64,b_ : * const f64) -> i32;
    fn MSK_appendafes(task_ : * const u8,num_ : i64) -> i32;
    fn MSK_appendbarvars(task_ : * const u8,num_ : i32,dim_ : * const i32) -> i32;
    fn MSK_appendcone(task_ : * const u8,ct_ : i32,conepar_ : f64,nummem_ : i32,submem_ : * const i32) -> i32;
    fn MSK_appendconeseq(task_ : * const u8,ct_ : i32,conepar_ : f64,nummem_ : i32,j_ : i32) -> i32;
    fn MSK_appendconesseq(task_ : * const u8,num_ : i32,ct_ : * const i32,conepar_ : * const f64,nummem_ : * const i32,j_ : i32) -> i32;
    fn MSK_appendcons(task_ : * const u8,num_ : i32) -> i32;
    fn MSK_appenddjcs(task_ : * const u8,num_ : i64) -> i32;
    fn MSK_appenddualexpconedomain(task_ : * const u8,domidx_ : & mut i64) -> i32;
    fn MSK_appenddualgeomeanconedomain(task_ : * const u8,n_ : i64,domidx_ : & mut i64) -> i32;
    fn MSK_appenddualpowerconedomain(task_ : * const u8,n_ : i64,nleft_ : i64,alpha_ : * const f64,domidx_ : & mut i64) -> i32;
    fn MSK_appendinfnormconedomain(task_ : * const u8,n_ : i64,domidx_ : & mut i64) -> i32;
    fn MSK_appendonenormconedomain(task_ : * const u8,n_ : i64,domidx_ : & mut i64) -> i32;
    fn MSK_appendprimalexpconedomain(task_ : * const u8,domidx_ : & mut i64) -> i32;
    fn MSK_appendprimalgeomeanconedomain(task_ : * const u8,n_ : i64,domidx_ : & mut i64) -> i32;
    fn MSK_appendprimalpowerconedomain(task_ : * const u8,n_ : i64,nleft_ : i64,alpha_ : * const f64,domidx_ : & mut i64) -> i32;
    fn MSK_appendpsdconedomain(task_ : * const u8,n_ : i64,domidx_ : & mut i64) -> i32;
    fn MSK_appendquadraticconedomain(task_ : * const u8,n_ : i64,domidx_ : & mut i64) -> i32;
    fn MSK_appendrdomain(task_ : * const u8,n_ : i64,domidx_ : & mut i64) -> i32;
    fn MSK_appendrminusdomain(task_ : * const u8,n_ : i64,domidx_ : & mut i64) -> i32;
    fn MSK_appendrplusdomain(task_ : * const u8,n_ : i64,domidx_ : & mut i64) -> i32;
    fn MSK_appendrquadraticconedomain(task_ : * const u8,n_ : i64,domidx_ : & mut i64) -> i32;
    fn MSK_appendrzerodomain(task_ : * const u8,n_ : i64,domidx_ : & mut i64) -> i32;
    fn MSK_appendsparsesymmat(task_ : * const u8,dim_ : i32,nz_ : i64,subi_ : * const i32,subj_ : * const i32,valij_ : * const f64,idx_ : & mut i64) -> i32;
    fn MSK_appendsparsesymmatlist(task_ : * const u8,num_ : i32,dims_ : * const i32,nz_ : * const i64,subi_ : * const i32,subj_ : * const i32,valij_ : * const f64,idx_ : * mut i64) -> i32;
    fn MSK_appendvars(task_ : * const u8,num_ : i32) -> i32;
    fn MSK_asyncgetresult(task_ : * const u8,addr_ : * const libc::c_char,accesstoken_ : * const libc::c_char,token_ : * const libc::c_char,respavailable_ : & mut i32,resp_ : & mut i32,trm_ : & mut i32) -> i32;
    fn MSK_asyncoptimize(task_ : * const u8,addr_ : * const libc::c_char,accesstoken_ : * const libc::c_char,token_ : * const u8) -> i32;
    fn MSK_asyncpoll(task_ : * const u8,addr_ : * const libc::c_char,accesstoken_ : * const libc::c_char,token_ : * const libc::c_char,respavailable_ : & mut i32,resp_ : & mut i32,trm_ : & mut i32) -> i32;
    fn MSK_asyncstop(task_ : * const u8,addr_ : * const libc::c_char,accesstoken_ : * const libc::c_char,token_ : * const libc::c_char) -> i32;
    fn MSK_axpy(env_ : * const u8,n_ : i32,alpha_ : f64,x_ : * const f64,y_ : * mut f64) -> i32;
    fn MSK_basiscond(task_ : * const u8,nrmbasis_ : & mut f64,nrminvbasis_ : & mut f64) -> i32;
    fn MSK_bktostr(task_ : * const u8,bk_ : i32,str_ : * const u8) -> i32;
    fn MSK_callbackcodetostr(code_ : i32,callbackcodestr_ : * const u8) -> i32;
    fn MSK_checkinall(env_ : * const u8) -> i32;
    fn MSK_checkinlicense(env_ : * const u8,feature_ : i32) -> i32;
    fn MSK_checkmemenv(env_ : * const u8,file_ : * const libc::c_char,line_ : i32) -> i32;
    fn MSK_checkmemtask(task_ : * const u8,file_ : * const libc::c_char,line_ : i32) -> i32;
    fn MSK_checkoutlicense(env_ : * const u8,feature_ : i32) -> i32;
    fn MSK_checkversion(env_ : * const u8,major_ : i32,minor_ : i32,revision_ : i32) -> i32;
    fn MSK_chgconbound(task_ : * const u8,i_ : i32,lower_ : i32,finite_ : i32,value_ : f64) -> i32;
    fn MSK_chgvarbound(task_ : * const u8,j_ : i32,lower_ : i32,finite_ : i32,value_ : f64) -> i32;
    fn MSK_commitchanges(task_ : * const u8) -> i32;
    fn MSK_conetypetostr(task_ : * const u8,ct_ : i32,str_ : * const u8) -> i32;
    fn MSK_deletesolution(task_ : * const u8,whichsol_ : i32) -> i32;
    fn MSK_dot(env_ : * const u8,n_ : i32,x_ : * const f64,y_ : * const f64,xty_ : & mut f64) -> i32;
    fn MSK_dualsensitivity(task_ : * const u8,numj_ : i32,subj_ : * const i32,leftpricej_ : * mut f64,rightpricej_ : * mut f64,leftrangej_ : * mut f64,rightrangej_ : * mut f64) -> i32;
    fn MSK_echoenv(env_ : * const u8,whichstream_ : i32,format_ : * const libc::c_char) -> i32;
    fn MSK_echointro(env_ : * const u8,longver_ : i32) -> i32;
    fn MSK_echotask(task_ : * const u8,whichstream_ : i32,format_ : * const libc::c_char) -> i32;
    fn MSK_emptyafebarfrow(task_ : * const u8,afeidx_ : i64) -> i32;
    fn MSK_emptyafebarfrowlist(task_ : * const u8,numafeidx_ : i64,afeidxlist_ : * const i64) -> i32;
    fn MSK_emptyafefcol(task_ : * const u8,j_ : i32) -> i32;
    fn MSK_emptyafefcollist(task_ : * const u8,numvaridx_ : i64,varidxlist_ : * const i32) -> i32;
    fn MSK_emptyafefrow(task_ : * const u8,afeidx_ : i64) -> i32;
    fn MSK_emptyafefrowlist(task_ : * const u8,numafeidx_ : i64,afeidxlist_ : * const i64) -> i32;
    fn MSK_evaluateacc(task_ : * const u8,whichsol_ : i32,accidx_ : i64,activity_ : * mut f64) -> i32;
    fn MSK_evaluateaccs(task_ : * const u8,whichsol_ : i32,activity_ : * mut f64) -> i32;
    fn MSK_gemm(env_ : * const u8,transa_ : i32,transb_ : i32,m_ : i32,n_ : i32,k_ : i32,alpha_ : f64,a_ : * const f64,b_ : * const f64,beta_ : f64,c_ : * mut f64) -> i32;
    fn MSK_gemv(env_ : * const u8,transa_ : i32,m_ : i32,n_ : i32,alpha_ : f64,a_ : * const f64,x_ : * const f64,beta_ : f64,y_ : * mut f64) -> i32;
    fn MSK_generateconenames(task_ : * const u8,num_ : i32,subk_ : * const i32,fmt_ : * const libc::c_char,ndims_ : i32,dims_ : * const i32,sp_ : * const i64) -> i32;
    fn MSK_generateconnames(task_ : * const u8,num_ : i32,subi_ : * const i32,fmt_ : * const libc::c_char,ndims_ : i32,dims_ : * const i32,sp_ : * const i64) -> i32;
    fn MSK_generatevarnames(task_ : * const u8,num_ : i32,subj_ : * const i32,fmt_ : * const libc::c_char,ndims_ : i32,dims_ : * const i32,sp_ : * const i64) -> i32;
    fn MSK_getaccafeidxlist(task_ : * const u8,accidx_ : i64,afeidxlist_ : * mut i64) -> i32;
    fn MSK_getaccb(task_ : * const u8,accidx_ : i64,b_ : * mut f64) -> i32;
    fn MSK_getaccdomain(task_ : * const u8,accidx_ : i64,domidx_ : & mut i64) -> i32;
    fn MSK_getaccdoty(task_ : * const u8,whichsol_ : i32,accidx_ : i64,doty_ : * mut f64) -> i32;
    fn MSK_getaccn(task_ : * const u8,accidx_ : i64,n_ : & mut i64) -> i32;
    fn MSK_getaccname(task_ : * const u8,accidx_ : i64,sizename_ : i32,name_ : * const u8) -> i32;
    fn MSK_getaccnamelen(task_ : * const u8,accidx_ : i64,len_ : & mut i32) -> i32;
    fn MSK_getaccntot(task_ : * const u8,n_ : & mut i64) -> i32;
    fn MSK_getaccs(task_ : * const u8,domidxlist_ : * mut i64,afeidxlist_ : * mut i64,b_ : * mut f64) -> i32;
    fn MSK_getacol(task_ : * const u8,j_ : i32,nzj_ : & mut i32,subj_ : * mut i32,valj_ : * mut f64) -> i32;
    fn MSK_getacolnumnz(task_ : * const u8,i_ : i32,nzj_ : & mut i32) -> i32;
    fn MSK_getacolslicenumnz64(task_ : * const u8,first_ : i32,last_ : i32,numnz_ : & mut i64) -> i32;
    fn MSK_getafebarfnumrowentries(task_ : * const u8,afeidx_ : i64,numentries_ : & mut i32) -> i32;
    fn MSK_getafebarfrow(task_ : * const u8,afeidx_ : i64,barvaridx_ : * mut i32,termptr_ : * mut i64,numterms_ : * mut i64,termidx_ : * mut i64,termweight_ : * mut f64) -> i32;
    fn MSK_getafebarfrowinfo(task_ : * const u8,afeidx_ : i64,numentries_ : & mut i32,numterms_ : & mut i64) -> i32;
    fn MSK_getafefnumnz(task_ : * const u8,numfnz_ : & mut i64) -> i32;
    fn MSK_getafefrow(task_ : * const u8,afeidx_ : i64,nzi_ : & mut i32,subi_ : * mut i32,vali_ : * mut f64) -> i32;
    fn MSK_getafefrownumnz(task_ : * const u8,afeidx_ : i64,nzi_ : & mut i32) -> i32;
    fn MSK_getafeg(task_ : * const u8,afeidx_ : i64,g_ : & mut f64) -> i32;
    fn MSK_getafegslice(task_ : * const u8,first_ : i64,last_ : i64,g_ : * mut f64) -> i32;
    fn MSK_getaij(task_ : * const u8,i_ : i32,j_ : i32,aij_ : & mut f64) -> i32;
    fn MSK_getapiecenumnz(task_ : * const u8,firsti_ : i32,lasti_ : i32,firstj_ : i32,lastj_ : i32,numnz_ : & mut i32) -> i32;
    fn MSK_getarow(task_ : * const u8,i_ : i32,nzi_ : & mut i32,subi_ : * mut i32,vali_ : * mut f64) -> i32;
    fn MSK_getarownumnz(task_ : * const u8,i_ : i32,nzi_ : & mut i32) -> i32;
    fn MSK_getarowslicenumnz64(task_ : * const u8,first_ : i32,last_ : i32,numnz_ : & mut i64) -> i32;
    fn MSK_getatruncatetol(task_ : * const u8,tolzero_ : * mut f64) -> i32;
    fn MSK_getbarablocktriplet(task_ : * const u8,maxnum_ : i64,num_ : & mut i64,subi_ : * mut i32,subj_ : * mut i32,subk_ : * mut i32,subl_ : * mut i32,valijkl_ : * mut f64) -> i32;
    fn MSK_getbaraidx(task_ : * const u8,idx_ : i64,maxnum_ : i64,i_ : & mut i32,j_ : & mut i32,num_ : & mut i64,sub_ : * mut i64,weights_ : * mut f64) -> i32;
    fn MSK_getbaraidxij(task_ : * const u8,idx_ : i64,i_ : & mut i32,j_ : & mut i32) -> i32;
    fn MSK_getbaraidxinfo(task_ : * const u8,idx_ : i64,num_ : & mut i64) -> i32;
    fn MSK_getbarasparsity(task_ : * const u8,maxnumnz_ : i64,numnz_ : & mut i64,idxij_ : * mut i64) -> i32;
    fn MSK_getbarcblocktriplet(task_ : * const u8,maxnum_ : i64,num_ : & mut i64,subj_ : * mut i32,subk_ : * mut i32,subl_ : * mut i32,valjkl_ : * mut f64) -> i32;
    fn MSK_getbarcidx(task_ : * const u8,idx_ : i64,maxnum_ : i64,j_ : & mut i32,num_ : & mut i64,sub_ : * mut i64,weights_ : * mut f64) -> i32;
    fn MSK_getbarcidxinfo(task_ : * const u8,idx_ : i64,num_ : & mut i64) -> i32;
    fn MSK_getbarcidxj(task_ : * const u8,idx_ : i64,j_ : & mut i32) -> i32;
    fn MSK_getbarcsparsity(task_ : * const u8,maxnumnz_ : i64,numnz_ : & mut i64,idxj_ : * mut i64) -> i32;
    fn MSK_getbarsj(task_ : * const u8,whichsol_ : i32,j_ : i32,barsj_ : * mut f64) -> i32;
    fn MSK_getbarsslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,slicesize_ : i64,barsslice_ : * mut f64) -> i32;
    fn MSK_getbarvarname(task_ : * const u8,i_ : i32,sizename_ : i32,name_ : * const u8) -> i32;
    fn MSK_getbarvarnameindex(task_ : * const u8,somename_ : * const libc::c_char,asgn_ : & mut i32,index_ : & mut i32) -> i32;
    fn MSK_getbarvarnamelen(task_ : * const u8,i_ : i32,len_ : & mut i32) -> i32;
    fn MSK_getbarxj(task_ : * const u8,whichsol_ : i32,j_ : i32,barxj_ : * mut f64) -> i32;
    fn MSK_getbarxslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,slicesize_ : i64,barxslice_ : * mut f64) -> i32;
    fn MSK_getbuildinfo(buildstate_ : * const u8,builddate_ : * const u8) -> i32;
    fn MSK_getc(task_ : * const u8,c_ : * mut f64) -> i32;
    fn MSK_getcfix(task_ : * const u8,cfix_ : & mut f64) -> i32;
    fn MSK_getcj(task_ : * const u8,j_ : i32,cj_ : & mut f64) -> i32;
    fn MSK_getclist(task_ : * const u8,num_ : i32,subj_ : * const i32,c_ : * mut f64) -> i32;
    fn MSK_getcodedesc(code_ : i32,symname_ : * const u8,str_ : * const u8) -> i32;
    fn MSK_getconbound(task_ : * const u8,i_ : i32,bk_ : & mut i32,bl_ : & mut f64,bu_ : & mut f64) -> i32;
    fn MSK_getconboundslice(task_ : * const u8,first_ : i32,last_ : i32,bk_ : * mut i32,bl_ : * mut f64,bu_ : * mut f64) -> i32;
    fn MSK_getcone(task_ : * const u8,k_ : i32,ct_ : & mut i32,conepar_ : & mut f64,nummem_ : & mut i32,submem_ : * mut i32) -> i32;
    fn MSK_getconeinfo(task_ : * const u8,k_ : i32,ct_ : & mut i32,conepar_ : & mut f64,nummem_ : & mut i32) -> i32;
    fn MSK_getconename(task_ : * const u8,i_ : i32,sizename_ : i32,name_ : * const u8) -> i32;
    fn MSK_getconenameindex(task_ : * const u8,somename_ : * const libc::c_char,asgn_ : & mut i32,index_ : & mut i32) -> i32;
    fn MSK_getconenamelen(task_ : * const u8,i_ : i32,len_ : & mut i32) -> i32;
    fn MSK_getconname(task_ : * const u8,i_ : i32,sizename_ : i32,name_ : * const u8) -> i32;
    fn MSK_getconnameindex(task_ : * const u8,somename_ : * const libc::c_char,asgn_ : & mut i32,index_ : & mut i32) -> i32;
    fn MSK_getconnamelen(task_ : * const u8,i_ : i32,len_ : & mut i32) -> i32;
    fn MSK_getcslice(task_ : * const u8,first_ : i32,last_ : i32,c_ : * mut f64) -> i32;
    fn MSK_getdimbarvarj(task_ : * const u8,j_ : i32,dimbarvarj_ : & mut i32) -> i32;
    fn MSK_getdjcafeidxlist(task_ : * const u8,djcidx_ : i64,afeidxlist_ : * mut i64) -> i32;
    fn MSK_getdjcb(task_ : * const u8,djcidx_ : i64,b_ : * mut f64) -> i32;
    fn MSK_getdjcdomainidxlist(task_ : * const u8,djcidx_ : i64,domidxlist_ : * mut i64) -> i32;
    fn MSK_getdjcname(task_ : * const u8,djcidx_ : i64,sizename_ : i32,name_ : * const u8) -> i32;
    fn MSK_getdjcnamelen(task_ : * const u8,djcidx_ : i64,len_ : & mut i32) -> i32;
    fn MSK_getdjcnumafe(task_ : * const u8,idjc_ : i64,numafe_ : & mut i64) -> i32;
    fn MSK_getdjcnumafetot(task_ : * const u8,numafetot_ : & mut i64) -> i32;
    fn MSK_getdjcnumdomain(task_ : * const u8,idjc_ : i64,numdomain_ : & mut i64) -> i32;
    fn MSK_getdjcnumdomaintot(task_ : * const u8,numdomaintot_ : & mut i64) -> i32;
    fn MSK_getdjcnumterm(task_ : * const u8,idjc_ : i64,numterm_ : & mut i64) -> i32;
    fn MSK_getdjcnumtermtot(task_ : * const u8,numtermtot_ : & mut i64) -> i32;
    fn MSK_getdjcs(task_ : * const u8,domidxlist_ : * mut i64,afeidxlist_ : * mut i64,b_ : * mut f64,termsizelist_ : * mut i64,numterms_ : * mut i64) -> i32;
    fn MSK_getdjctermsizelist(task_ : * const u8,djcidx_ : i64,termsizelist_ : * mut i64) -> i32;
    fn MSK_getdomainn(task_ : * const u8,domidx_ : i64,n_ : & mut i64) -> i32;
    fn MSK_getdomainname(task_ : * const u8,domidx_ : i64,sizename_ : i32,name_ : * const u8) -> i32;
    fn MSK_getdomainnamelen(task_ : * const u8,domidx_ : i64,len_ : & mut i32) -> i32;
    fn MSK_getdomaintype(task_ : * const u8,domidx_ : i64,domaintype_ : & mut i32) -> i32;
    fn MSK_getdouinf(task_ : * const u8,whichdinf_ : i32,dvalue_ : & mut f64) -> i32;
    fn MSK_getdouparam(task_ : * const u8,param_ : i32,parvalue_ : & mut f64) -> i32;
    fn MSK_getdualobj(task_ : * const u8,whichsol_ : i32,dualobj_ : & mut f64) -> i32;
    fn MSK_getdualsolutionnorms(task_ : * const u8,whichsol_ : i32,nrmy_ : & mut f64,nrmslc_ : & mut f64,nrmsuc_ : & mut f64,nrmslx_ : & mut f64,nrmsux_ : & mut f64,nrmsnx_ : & mut f64,nrmbars_ : & mut f64) -> i32;
    fn MSK_getdviolacc(task_ : * const u8,whichsol_ : i32,numaccidx_ : i64,accidxlist_ : * const i64,viol_ : * mut f64) -> i32;
    fn MSK_getdviolbarvar(task_ : * const u8,whichsol_ : i32,num_ : i32,sub_ : * const i32,viol_ : * mut f64) -> i32;
    fn MSK_getdviolcon(task_ : * const u8,whichsol_ : i32,num_ : i32,sub_ : * const i32,viol_ : * mut f64) -> i32;
    fn MSK_getdviolcones(task_ : * const u8,whichsol_ : i32,num_ : i32,sub_ : * const i32,viol_ : * mut f64) -> i32;
    fn MSK_getdviolvar(task_ : * const u8,whichsol_ : i32,num_ : i32,sub_ : * const i32,viol_ : * mut f64) -> i32;
    fn MSK_getinfindex(task_ : * const u8,inftype_ : i32,infname_ : * const libc::c_char,infindex_ : & mut i32) -> i32;
    fn MSK_getinfmax(task_ : * const u8,inftype_ : i32,infmax_ : * mut i32) -> i32;
    fn MSK_getinfname(task_ : * const u8,inftype_ : i32,whichinf_ : i32,infname_ : * const u8) -> i32;
    fn MSK_getintinf(task_ : * const u8,whichiinf_ : i32,ivalue_ : & mut i32) -> i32;
    fn MSK_getintparam(task_ : * const u8,param_ : i32,parvalue_ : & mut i32) -> i32;
    fn MSK_getlenbarvarj(task_ : * const u8,j_ : i32,lenbarvarj_ : & mut i64) -> i32;
    fn MSK_getlintinf(task_ : * const u8,whichliinf_ : i32,ivalue_ : & mut i64) -> i32;
    fn MSK_getmaxnamelen(task_ : * const u8,maxlen_ : & mut i32) -> i32;
    fn MSK_getmaxnumanz64(task_ : * const u8,maxnumanz_ : & mut i64) -> i32;
    fn MSK_getmaxnumbarvar(task_ : * const u8,maxnumbarvar_ : & mut i32) -> i32;
    fn MSK_getmaxnumcon(task_ : * const u8,maxnumcon_ : & mut i32) -> i32;
    fn MSK_getmaxnumcone(task_ : * const u8,maxnumcone_ : & mut i32) -> i32;
    fn MSK_getmaxnumqnz64(task_ : * const u8,maxnumqnz_ : & mut i64) -> i32;
    fn MSK_getmaxnumvar(task_ : * const u8,maxnumvar_ : & mut i32) -> i32;
    fn MSK_getmemusagetask(task_ : * const u8,meminuse_ : & mut i64,maxmemuse_ : & mut i64) -> i32;
    fn MSK_getnadouinf(task_ : * const u8,infitemname_ : * const libc::c_char,dvalue_ : & mut f64) -> i32;
    fn MSK_getnadouparam(task_ : * const u8,paramname_ : * const libc::c_char,parvalue_ : & mut f64) -> i32;
    fn MSK_getnaintinf(task_ : * const u8,infitemname_ : * const libc::c_char,ivalue_ : & mut i32) -> i32;
    fn MSK_getnaintparam(task_ : * const u8,paramname_ : * const libc::c_char,parvalue_ : & mut i32) -> i32;
    fn MSK_getnastrparam(task_ : * const u8,paramname_ : * const libc::c_char,sizeparamname_ : i32,len_ : & mut i32,parvalue_ : * const u8) -> i32;
    fn MSK_getnumacc(task_ : * const u8,num_ : & mut i64) -> i32;
    fn MSK_getnumafe(task_ : * const u8,numafe_ : & mut i64) -> i32;
    fn MSK_getnumanz(task_ : * const u8,numanz_ : & mut i32) -> i32;
    fn MSK_getnumanz64(task_ : * const u8,numanz_ : & mut i64) -> i32;
    fn MSK_getnumbarablocktriplets(task_ : * const u8,num_ : & mut i64) -> i32;
    fn MSK_getnumbaranz(task_ : * const u8,nz_ : & mut i64) -> i32;
    fn MSK_getnumbarcblocktriplets(task_ : * const u8,num_ : & mut i64) -> i32;
    fn MSK_getnumbarcnz(task_ : * const u8,nz_ : & mut i64) -> i32;
    fn MSK_getnumbarvar(task_ : * const u8,numbarvar_ : & mut i32) -> i32;
    fn MSK_getnumcon(task_ : * const u8,numcon_ : & mut i32) -> i32;
    fn MSK_getnumcone(task_ : * const u8,numcone_ : & mut i32) -> i32;
    fn MSK_getnumconemem(task_ : * const u8,k_ : i32,nummem_ : & mut i32) -> i32;
    fn MSK_getnumdjc(task_ : * const u8,num_ : & mut i64) -> i32;
    fn MSK_getnumdomain(task_ : * const u8,numdomain_ : & mut i64) -> i32;
    fn MSK_getnumintvar(task_ : * const u8,numintvar_ : & mut i32) -> i32;
    fn MSK_getnumparam(task_ : * const u8,partype_ : i32,numparam_ : & mut i32) -> i32;
    fn MSK_getnumqconknz64(task_ : * const u8,k_ : i32,numqcnz_ : & mut i64) -> i32;
    fn MSK_getnumqobjnz64(task_ : * const u8,numqonz_ : & mut i64) -> i32;
    fn MSK_getnumsymmat(task_ : * const u8,num_ : & mut i64) -> i32;
    fn MSK_getnumvar(task_ : * const u8,numvar_ : & mut i32) -> i32;
    fn MSK_getobjname(task_ : * const u8,sizeobjname_ : i32,objname_ : * const u8) -> i32;
    fn MSK_getobjnamelen(task_ : * const u8,len_ : & mut i32) -> i32;
    fn MSK_getobjsense(task_ : * const u8,sense_ : & mut i32) -> i32;
    fn MSK_getparammax(task_ : * const u8,partype_ : i32,parammax_ : & mut i32) -> i32;
    fn MSK_getparamname(task_ : * const u8,partype_ : i32,param_ : i32,parname_ : * const u8) -> i32;
    fn MSK_getpowerdomainalpha(task_ : * const u8,domidx_ : i64,alpha_ : * mut f64) -> i32;
    fn MSK_getpowerdomaininfo(task_ : * const u8,domidx_ : i64,n_ : & mut i64,nleft_ : & mut i64) -> i32;
    fn MSK_getprimalobj(task_ : * const u8,whichsol_ : i32,primalobj_ : & mut f64) -> i32;
    fn MSK_getprimalsolutionnorms(task_ : * const u8,whichsol_ : i32,nrmxc_ : & mut f64,nrmxx_ : & mut f64,nrmbarx_ : & mut f64) -> i32;
    fn MSK_getprobtype(task_ : * const u8,probtype_ : & mut i32) -> i32;
    fn MSK_getprosta(task_ : * const u8,whichsol_ : i32,prosta_ : & mut i32) -> i32;
    fn MSK_getpviolacc(task_ : * const u8,whichsol_ : i32,numaccidx_ : i64,accidxlist_ : * const i64,viol_ : * mut f64) -> i32;
    fn MSK_getpviolbarvar(task_ : * const u8,whichsol_ : i32,num_ : i32,sub_ : * const i32,viol_ : * mut f64) -> i32;
    fn MSK_getpviolcon(task_ : * const u8,whichsol_ : i32,num_ : i32,sub_ : * const i32,viol_ : * mut f64) -> i32;
    fn MSK_getpviolcones(task_ : * const u8,whichsol_ : i32,num_ : i32,sub_ : * const i32,viol_ : * mut f64) -> i32;
    fn MSK_getpvioldjc(task_ : * const u8,whichsol_ : i32,numdjcidx_ : i64,djcidxlist_ : * const i64,viol_ : * mut f64) -> i32;
    fn MSK_getpviolvar(task_ : * const u8,whichsol_ : i32,num_ : i32,sub_ : * const i32,viol_ : * mut f64) -> i32;
    fn MSK_getqobjij(task_ : * const u8,i_ : i32,j_ : i32,qoij_ : & mut f64) -> i32;
    fn MSK_getreducedcosts(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,redcosts_ : * mut f64) -> i32;
    fn MSK_getresponseclass(r_ : i32,rc_ : & mut i32) -> i32;
    fn MSK_getskc(task_ : * const u8,whichsol_ : i32,skc_ : * mut i32) -> i32;
    fn MSK_getskcslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,skc_ : * mut i32) -> i32;
    fn MSK_getskn(task_ : * const u8,whichsol_ : i32,skn_ : * mut i32) -> i32;
    fn MSK_getskx(task_ : * const u8,whichsol_ : i32,skx_ : * mut i32) -> i32;
    fn MSK_getskxslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,skx_ : * mut i32) -> i32;
    fn MSK_getslc(task_ : * const u8,whichsol_ : i32,slc_ : * mut f64) -> i32;
    fn MSK_getslcslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,slc_ : * mut f64) -> i32;
    fn MSK_getslx(task_ : * const u8,whichsol_ : i32,slx_ : * mut f64) -> i32;
    fn MSK_getslxslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,slx_ : * mut f64) -> i32;
    fn MSK_getsnx(task_ : * const u8,whichsol_ : i32,snx_ : * mut f64) -> i32;
    fn MSK_getsnxslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,snx_ : * mut f64) -> i32;
    fn MSK_getsolsta(task_ : * const u8,whichsol_ : i32,solsta_ : & mut i32) -> i32;
    fn MSK_getsolution(task_ : * const u8,whichsol_ : i32,prosta_ : & mut i32,solsta_ : & mut i32,skc_ : * mut i32,skx_ : * mut i32,skn_ : * mut i32,xc_ : * mut f64,xx_ : * mut f64,y_ : * mut f64,slc_ : * mut f64,suc_ : * mut f64,slx_ : * mut f64,sux_ : * mut f64,snx_ : * mut f64) -> i32;
    fn MSK_getsolutioninfo(task_ : * const u8,whichsol_ : i32,pobj_ : & mut f64,pviolcon_ : & mut f64,pviolvar_ : & mut f64,pviolbarvar_ : & mut f64,pviolcone_ : & mut f64,pviolitg_ : & mut f64,dobj_ : & mut f64,dviolcon_ : & mut f64,dviolvar_ : & mut f64,dviolbarvar_ : & mut f64,dviolcone_ : & mut f64) -> i32;
    fn MSK_getsolutioninfonew(task_ : * const u8,whichsol_ : i32,pobj_ : & mut f64,pviolcon_ : & mut f64,pviolvar_ : & mut f64,pviolbarvar_ : & mut f64,pviolcone_ : & mut f64,pviolacc_ : & mut f64,pvioldjc_ : & mut f64,pviolitg_ : & mut f64,dobj_ : & mut f64,dviolcon_ : & mut f64,dviolvar_ : & mut f64,dviolbarvar_ : & mut f64,dviolcone_ : & mut f64,dviolacc_ : & mut f64) -> i32;
    fn MSK_getsolutionnew(task_ : * const u8,whichsol_ : i32,prosta_ : & mut i32,solsta_ : & mut i32,skc_ : * mut i32,skx_ : * mut i32,skn_ : * mut i32,xc_ : * mut f64,xx_ : * mut f64,y_ : * mut f64,slc_ : * mut f64,suc_ : * mut f64,slx_ : * mut f64,sux_ : * mut f64,snx_ : * mut f64,doty_ : * mut f64) -> i32;
    fn MSK_getsolutionslice(task_ : * const u8,whichsol_ : i32,solitem_ : i32,first_ : i32,last_ : i32,values_ : * mut f64) -> i32;
    fn MSK_getsparsesymmat(task_ : * const u8,idx_ : i64,maxlen_ : i64,subi_ : * mut i32,subj_ : * mut i32,valij_ : * mut f64) -> i32;
    fn MSK_getstrparam(task_ : * const u8,param_ : i32,maxlen_ : i32,len_ : & mut i32,parvalue_ : * const u8) -> i32;
    fn MSK_getstrparamlen(task_ : * const u8,param_ : i32,len_ : & mut i32) -> i32;
    fn MSK_getsuc(task_ : * const u8,whichsol_ : i32,suc_ : * mut f64) -> i32;
    fn MSK_getsucslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,suc_ : * mut f64) -> i32;
    fn MSK_getsux(task_ : * const u8,whichsol_ : i32,sux_ : * mut f64) -> i32;
    fn MSK_getsuxslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,sux_ : * mut f64) -> i32;
    fn MSK_getsymbcon(task_ : * const u8,i_ : i32,sizevalue_ : i32,name_ : * const u8,value_ : & mut i32) -> i32;
    fn MSK_getsymmatinfo(task_ : * const u8,idx_ : i64,dim_ : & mut i32,nz_ : & mut i64,type_ : & mut i32) -> i32;
    fn MSK_gettaskname(task_ : * const u8,sizetaskname_ : i32,taskname_ : * const u8) -> i32;
    fn MSK_gettasknamelen(task_ : * const u8,len_ : & mut i32) -> i32;
    fn MSK_getvarbound(task_ : * const u8,i_ : i32,bk_ : & mut i32,bl_ : & mut f64,bu_ : & mut f64) -> i32;
    fn MSK_getvarboundslice(task_ : * const u8,first_ : i32,last_ : i32,bk_ : * mut i32,bl_ : * mut f64,bu_ : * mut f64) -> i32;
    fn MSK_getvarname(task_ : * const u8,j_ : i32,sizename_ : i32,name_ : * const u8) -> i32;
    fn MSK_getvarnameindex(task_ : * const u8,somename_ : * const libc::c_char,asgn_ : & mut i32,index_ : & mut i32) -> i32;
    fn MSK_getvarnamelen(task_ : * const u8,i_ : i32,len_ : & mut i32) -> i32;
    fn MSK_getvartype(task_ : * const u8,j_ : i32,vartype_ : & mut i32) -> i32;
    fn MSK_getvartypelist(task_ : * const u8,num_ : i32,subj_ : * const i32,vartype_ : * mut i32) -> i32;
    fn MSK_getversion(major_ : & mut i32,minor_ : & mut i32,revision_ : & mut i32) -> i32;
    fn MSK_getxc(task_ : * const u8,whichsol_ : i32,xc_ : * mut f64) -> i32;
    fn MSK_getxcslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,xc_ : * mut f64) -> i32;
    fn MSK_getxx(task_ : * const u8,whichsol_ : i32,xx_ : * mut f64) -> i32;
    fn MSK_getxxslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,xx_ : * mut f64) -> i32;
    fn MSK_gety(task_ : * const u8,whichsol_ : i32,y_ : * mut f64) -> i32;
    fn MSK_getyslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,y_ : * mut f64) -> i32;
    fn MSK_initbasissolve(task_ : * const u8,basis_ : * mut i32) -> i32;
    fn MSK_inputdata64(task_ : * const u8,maxnumcon_ : i32,maxnumvar_ : i32,numcon_ : i32,numvar_ : i32,c_ : * const f64,cfix_ : f64,aptrb_ : * const i64,aptre_ : * const i64,asub_ : * const i32,aval_ : * const f64,bkc_ : * const i32,blc_ : * const f64,buc_ : * const f64,bkx_ : * const i32,blx_ : * const f64,bux_ : * const f64) -> i32;
    fn MSK_isdouparname(task_ : * const u8,parname_ : * const libc::c_char,param_ : & mut i32) -> i32;
    fn MSK_isinfinity(value_ : f64) -> i32;
    fn MSK_isintparname(task_ : * const u8,parname_ : * const libc::c_char,param_ : & mut i32) -> i32;
    fn MSK_isstrparname(task_ : * const u8,parname_ : * const libc::c_char,param_ : & mut i32) -> i32;
    fn MSK_licensecleanup() -> i32;
    fn MSK_linkfiletoenvstream(env_ : * const u8,whichstream_ : i32,filename_ : * const libc::c_char,append_ : i32) -> i32;
    fn MSK_linkfiletotaskstream(task_ : * const u8,whichstream_ : i32,filename_ : * const libc::c_char,append_ : i32) -> i32;
    fn MSK_onesolutionsummary(task_ : * const u8,whichstream_ : i32,whichsol_ : i32) -> i32;
    fn MSK_optimizermt(task_ : * const u8,addr_ : * const libc::c_char,accesstoken_ : * const libc::c_char,trmcode_ : & mut i32) -> i32;
    fn MSK_optimizersummary(task_ : * const u8,whichstream_ : i32) -> i32;
    fn MSK_optimizetrm(task_ : * const u8,trmcode_ : & mut i32) -> i32;
    fn MSK_potrf(env_ : * const u8,uplo_ : i32,n_ : i32,a_ : * mut f64) -> i32;
    fn MSK_primalrepair(task_ : * const u8,wlc_ : * const f64,wuc_ : * const f64,wlx_ : * const f64,wux_ : * const f64) -> i32;
    fn MSK_primalsensitivity(task_ : * const u8,numi_ : i32,subi_ : * const i32,marki_ : * const i32,numj_ : i32,subj_ : * const i32,markj_ : * const i32,leftpricei_ : * mut f64,rightpricei_ : * mut f64,leftrangei_ : * mut f64,rightrangei_ : * mut f64,leftpricej_ : * mut f64,rightpricej_ : * mut f64,leftrangej_ : * mut f64,rightrangej_ : * mut f64) -> i32;
    fn MSK_printparam(task_ : * const u8) -> i32;
    fn MSK_probtypetostr(task_ : * const u8,probtype_ : i32,str_ : * const u8) -> i32;
    fn MSK_prostatostr(task_ : * const u8,prosta_ : i32,str_ : * const u8) -> i32;
    fn MSK_putacc(task_ : * const u8,accidx_ : i64,domidx_ : i64,numafeidx_ : i64,afeidxlist_ : * const i64,b_ : * const f64) -> i32;
    fn MSK_putaccb(task_ : * const u8,accidx_ : i64,lengthb_ : i64,b_ : * const f64) -> i32;
    fn MSK_putaccbj(task_ : * const u8,accidx_ : i64,j_ : i64,bj_ : f64) -> i32;
    fn MSK_putaccdoty(task_ : * const u8,whichsol_ : i32,accidx_ : i64,doty_ : * mut f64) -> i32;
    fn MSK_putacclist(task_ : * const u8,numaccs_ : i64,accidxs_ : * const i64,domidxs_ : * const i64,numafeidx_ : i64,afeidxlist_ : * const i64,b_ : * const f64) -> i32;
    fn MSK_putaccname(task_ : * const u8,accidx_ : i64,name_ : * const libc::c_char) -> i32;
    fn MSK_putacol(task_ : * const u8,j_ : i32,nzj_ : i32,subj_ : * const i32,valj_ : * const f64) -> i32;
    fn MSK_putacollist64(task_ : * const u8,num_ : i32,sub_ : * const i32,ptrb_ : * const i64,ptre_ : * const i64,asub_ : * const i32,aval_ : * const f64) -> i32;
    fn MSK_putacolslice64(task_ : * const u8,first_ : i32,last_ : i32,ptrb_ : * const i64,ptre_ : * const i64,asub_ : * const i32,aval_ : * const f64) -> i32;
    fn MSK_putafebarfblocktriplet(task_ : * const u8,num_ : i64,afeidx_ : * const i64,barvaridx_ : * const i32,subk_ : * const i32,subl_ : * const i32,valkl_ : * const f64) -> i32;
    fn MSK_putafebarfentry(task_ : * const u8,afeidx_ : i64,barvaridx_ : i32,numterms_ : i64,termidx_ : * const i64,termweight_ : * const f64) -> i32;
    fn MSK_putafebarfentrylist(task_ : * const u8,lenlist_ : i64,afeidxlist_ : * const i64,barvaridxlist_ : * const i32,numtermslist_ : * const i64,ptrtermslist_ : * const i64,lenterms_ : i64,termidx_ : * const i64,termweight_ : * const f64) -> i32;
    fn MSK_putafebarfrow(task_ : * const u8,afeidx_ : i64,numentries_ : i32,barvaridxlist_ : * const i32,numtermlist_ : * const i64,ptrtermlist_ : * const i64,lenterms_ : i64,termidx_ : * const i64,termweight_ : * const f64) -> i32;
    fn MSK_putafefentry(task_ : * const u8,afeidx_ : i64,j_ : i32,value_ : f64) -> i32;
    fn MSK_putafefrow(task_ : * const u8,afeidx_ : i64,nzi_ : i32,subi_ : * const i32,vali_ : * const f64) -> i32;
    fn MSK_putafefrowlist(task_ : * const u8,numafeidx_ : i64,afeidxlist_ : * const i64,nzrow_ : * const i32,ptrrow_ : * const i64,lenidxval_ : i64,idxrow_ : * const i32,valrow_ : * const f64) -> i32;
    fn MSK_putafeg(task_ : * const u8,afeidx_ : i64,gi_ : f64) -> i32;
    fn MSK_putafeglist(task_ : * const u8,numafeidx_ : i64,afeidxlist_ : * const i64,glist_ : * const f64) -> i32;
    fn MSK_putafegslice(task_ : * const u8,first_ : i64,last_ : i64,slice_ : * mut f64) -> i32;
    fn MSK_putaij(task_ : * const u8,i_ : i32,j_ : i32,aij_ : f64) -> i32;
    fn MSK_putaijlist64(task_ : * const u8,num_ : i64,subi_ : * const i32,subj_ : * const i32,valij_ : * const f64) -> i32;
    fn MSK_putarow(task_ : * const u8,i_ : i32,nzi_ : i32,subi_ : * const i32,vali_ : * const f64) -> i32;
    fn MSK_putarowlist64(task_ : * const u8,num_ : i32,sub_ : * const i32,ptrb_ : * const i64,ptre_ : * const i64,asub_ : * const i32,aval_ : * const f64) -> i32;
    fn MSK_putarowslice64(task_ : * const u8,first_ : i32,last_ : i32,ptrb_ : * const i64,ptre_ : * const i64,asub_ : * const i32,aval_ : * const f64) -> i32;
    fn MSK_putatruncatetol(task_ : * const u8,tolzero_ : f64) -> i32;
    fn MSK_putbarablocktriplet(task_ : * const u8,num_ : i64,subi_ : * const i32,subj_ : * const i32,subk_ : * const i32,subl_ : * const i32,valijkl_ : * const f64) -> i32;
    fn MSK_putbaraij(task_ : * const u8,i_ : i32,j_ : i32,num_ : i64,sub_ : * const i64,weights_ : * const f64) -> i32;
    fn MSK_putbaraijlist(task_ : * const u8,num_ : i32,subi_ : * const i32,subj_ : * const i32,alphaptrb_ : * const i64,alphaptre_ : * const i64,matidx_ : * const i64,weights_ : * const f64) -> i32;
    fn MSK_putbararowlist(task_ : * const u8,num_ : i32,subi_ : * const i32,ptrb_ : * const i64,ptre_ : * const i64,subj_ : * const i32,nummat_ : * const i64,matidx_ : * const i64,weights_ : * const f64) -> i32;
    fn MSK_putbarcblocktriplet(task_ : * const u8,num_ : i64,subj_ : * const i32,subk_ : * const i32,subl_ : * const i32,valjkl_ : * const f64) -> i32;
    fn MSK_putbarcj(task_ : * const u8,j_ : i32,num_ : i64,sub_ : * const i64,weights_ : * const f64) -> i32;
    fn MSK_putbarsj(task_ : * const u8,whichsol_ : i32,j_ : i32,barsj_ : * const f64) -> i32;
    fn MSK_putbarvarname(task_ : * const u8,j_ : i32,name_ : * const libc::c_char) -> i32;
    fn MSK_putbarxj(task_ : * const u8,whichsol_ : i32,j_ : i32,barxj_ : * const f64) -> i32;
    fn MSK_putcfix(task_ : * const u8,cfix_ : f64) -> i32;
    fn MSK_putcj(task_ : * const u8,j_ : i32,cj_ : f64) -> i32;
    fn MSK_putclist(task_ : * const u8,num_ : i32,subj_ : * const i32,val_ : * const f64) -> i32;
    fn MSK_putconbound(task_ : * const u8,i_ : i32,bkc_ : i32,blc_ : f64,buc_ : f64) -> i32;
    fn MSK_putconboundlist(task_ : * const u8,num_ : i32,sub_ : * const i32,bkc_ : * const i32,blc_ : * const f64,buc_ : * const f64) -> i32;
    fn MSK_putconboundlistconst(task_ : * const u8,num_ : i32,sub_ : * const i32,bkc_ : i32,blc_ : f64,buc_ : f64) -> i32;
    fn MSK_putconboundslice(task_ : * const u8,first_ : i32,last_ : i32,bkc_ : * const i32,blc_ : * const f64,buc_ : * const f64) -> i32;
    fn MSK_putconboundsliceconst(task_ : * const u8,first_ : i32,last_ : i32,bkc_ : i32,blc_ : f64,buc_ : f64) -> i32;
    fn MSK_putcone(task_ : * const u8,k_ : i32,ct_ : i32,conepar_ : f64,nummem_ : i32,submem_ : * const i32) -> i32;
    fn MSK_putconename(task_ : * const u8,j_ : i32,name_ : * const libc::c_char) -> i32;
    fn MSK_putconname(task_ : * const u8,i_ : i32,name_ : * const libc::c_char) -> i32;
    fn MSK_putconsolutioni(task_ : * const u8,i_ : i32,whichsol_ : i32,sk_ : i32,x_ : f64,sl_ : f64,su_ : f64) -> i32;
    fn MSK_putcslice(task_ : * const u8,first_ : i32,last_ : i32,slice_ : * const f64) -> i32;
    fn MSK_putdjc(task_ : * const u8,djcidx_ : i64,numdomidx_ : i64,domidxlist_ : * const i64,numafeidx_ : i64,afeidxlist_ : * const i64,b_ : * const f64,numterms_ : i64,termsizelist_ : * const i64) -> i32;
    fn MSK_putdjcname(task_ : * const u8,djcidx_ : i64,name_ : * const libc::c_char) -> i32;
    fn MSK_putdjcslice(task_ : * const u8,idxfirst_ : i64,idxlast_ : i64,numdomidx_ : i64,domidxlist_ : * const i64,numafeidx_ : i64,afeidxlist_ : * const i64,b_ : * const f64,numterms_ : i64,termsizelist_ : * const i64,termsindjc_ : * const i64) -> i32;
    fn MSK_putdomainname(task_ : * const u8,domidx_ : i64,name_ : * const libc::c_char) -> i32;
    fn MSK_putdouparam(task_ : * const u8,param_ : i32,parvalue_ : f64) -> i32;
    fn MSK_putintparam(task_ : * const u8,param_ : i32,parvalue_ : i32) -> i32;
    fn MSK_putlicensecode(env_ : * const u8,code_ : * const i32) -> i32;
    fn MSK_putlicensedebug(env_ : * const u8,licdebug_ : i32) -> i32;
    fn MSK_putlicensepath(env_ : * const u8,licensepath_ : * const libc::c_char) -> i32;
    fn MSK_putlicensewait(env_ : * const u8,licwait_ : i32) -> i32;
    fn MSK_putmaxnumacc(task_ : * const u8,maxnumacc_ : i64) -> i32;
    fn MSK_putmaxnumafe(task_ : * const u8,maxnumafe_ : i64) -> i32;
    fn MSK_putmaxnumanz(task_ : * const u8,maxnumanz_ : i64) -> i32;
    fn MSK_putmaxnumbarvar(task_ : * const u8,maxnumbarvar_ : i32) -> i32;
    fn MSK_putmaxnumcon(task_ : * const u8,maxnumcon_ : i32) -> i32;
    fn MSK_putmaxnumcone(task_ : * const u8,maxnumcone_ : i32) -> i32;
    fn MSK_putmaxnumdjc(task_ : * const u8,maxnumdjc_ : i64) -> i32;
    fn MSK_putmaxnumdomain(task_ : * const u8,maxnumdomain_ : i64) -> i32;
    fn MSK_putmaxnumqnz(task_ : * const u8,maxnumqnz_ : i64) -> i32;
    fn MSK_putmaxnumvar(task_ : * const u8,maxnumvar_ : i32) -> i32;
    fn MSK_putnadouparam(task_ : * const u8,paramname_ : * const libc::c_char,parvalue_ : f64) -> i32;
    fn MSK_putnaintparam(task_ : * const u8,paramname_ : * const libc::c_char,parvalue_ : i32) -> i32;
    fn MSK_putnastrparam(task_ : * const u8,paramname_ : * const libc::c_char,parvalue_ : * const libc::c_char) -> i32;
    fn MSK_putobjname(task_ : * const u8,objname_ : * const libc::c_char) -> i32;
    fn MSK_putobjsense(task_ : * const u8,sense_ : i32) -> i32;
    fn MSK_putoptserverhost(task_ : * const u8,host_ : * const libc::c_char) -> i32;
    fn MSK_putparam(task_ : * const u8,parname_ : * const libc::c_char,parvalue_ : * const libc::c_char) -> i32;
    fn MSK_putqcon(task_ : * const u8,numqcnz_ : i32,qcsubk_ : * const i32,qcsubi_ : * const i32,qcsubj_ : * const i32,qcval_ : * const f64) -> i32;
    fn MSK_putqconk(task_ : * const u8,k_ : i32,numqcnz_ : i32,qcsubi_ : * const i32,qcsubj_ : * const i32,qcval_ : * const f64) -> i32;
    fn MSK_putqobj(task_ : * const u8,numqonz_ : i32,qosubi_ : * const i32,qosubj_ : * const i32,qoval_ : * const f64) -> i32;
    fn MSK_putqobjij(task_ : * const u8,i_ : i32,j_ : i32,qoij_ : f64) -> i32;
    fn MSK_putskc(task_ : * const u8,whichsol_ : i32,skc_ : * const i32) -> i32;
    fn MSK_putskcslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,skc_ : * const i32) -> i32;
    fn MSK_putskx(task_ : * const u8,whichsol_ : i32,skx_ : * const i32) -> i32;
    fn MSK_putskxslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,skx_ : * const i32) -> i32;
    fn MSK_putslc(task_ : * const u8,whichsol_ : i32,slc_ : * const f64) -> i32;
    fn MSK_putslcslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,slc_ : * const f64) -> i32;
    fn MSK_putslx(task_ : * const u8,whichsol_ : i32,slx_ : * const f64) -> i32;
    fn MSK_putslxslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,slx_ : * const f64) -> i32;
    fn MSK_putsnx(task_ : * const u8,whichsol_ : i32,sux_ : * const f64) -> i32;
    fn MSK_putsnxslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,snx_ : * const f64) -> i32;
    fn MSK_putsolution(task_ : * const u8,whichsol_ : i32,skc_ : * const i32,skx_ : * const i32,skn_ : * const i32,xc_ : * const f64,xx_ : * const f64,y_ : * const f64,slc_ : * const f64,suc_ : * const f64,slx_ : * const f64,sux_ : * const f64,snx_ : * const f64) -> i32;
    fn MSK_putsolutionnew(task_ : * const u8,whichsol_ : i32,skc_ : * const i32,skx_ : * const i32,skn_ : * const i32,xc_ : * const f64,xx_ : * const f64,y_ : * const f64,slc_ : * const f64,suc_ : * const f64,slx_ : * const f64,sux_ : * const f64,snx_ : * const f64,doty_ : * const f64) -> i32;
    fn MSK_putsolutionyi(task_ : * const u8,i_ : i32,whichsol_ : i32,y_ : f64) -> i32;
    fn MSK_putstrparam(task_ : * const u8,param_ : i32,parvalue_ : * const libc::c_char) -> i32;
    fn MSK_putsuc(task_ : * const u8,whichsol_ : i32,suc_ : * const f64) -> i32;
    fn MSK_putsucslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,suc_ : * const f64) -> i32;
    fn MSK_putsux(task_ : * const u8,whichsol_ : i32,sux_ : * const f64) -> i32;
    fn MSK_putsuxslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,sux_ : * const f64) -> i32;
    fn MSK_puttaskname(task_ : * const u8,taskname_ : * const libc::c_char) -> i32;
    fn MSK_putvarbound(task_ : * const u8,j_ : i32,bkx_ : i32,blx_ : f64,bux_ : f64) -> i32;
    fn MSK_putvarboundlist(task_ : * const u8,num_ : i32,sub_ : * const i32,bkx_ : * const i32,blx_ : * const f64,bux_ : * const f64) -> i32;
    fn MSK_putvarboundlistconst(task_ : * const u8,num_ : i32,sub_ : * const i32,bkx_ : i32,blx_ : f64,bux_ : f64) -> i32;
    fn MSK_putvarboundslice(task_ : * const u8,first_ : i32,last_ : i32,bkx_ : * const i32,blx_ : * const f64,bux_ : * const f64) -> i32;
    fn MSK_putvarboundsliceconst(task_ : * const u8,first_ : i32,last_ : i32,bkx_ : i32,blx_ : f64,bux_ : f64) -> i32;
    fn MSK_putvarname(task_ : * const u8,j_ : i32,name_ : * const libc::c_char) -> i32;
    fn MSK_putvarsolutionj(task_ : * const u8,j_ : i32,whichsol_ : i32,sk_ : i32,x_ : f64,sl_ : f64,su_ : f64,sn_ : f64) -> i32;
    fn MSK_putvartype(task_ : * const u8,j_ : i32,vartype_ : i32) -> i32;
    fn MSK_putvartypelist(task_ : * const u8,num_ : i32,subj_ : * const i32,vartype_ : * const i32) -> i32;
    fn MSK_putxc(task_ : * const u8,whichsol_ : i32,xc_ : * mut f64) -> i32;
    fn MSK_putxcslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,xc_ : * const f64) -> i32;
    fn MSK_putxx(task_ : * const u8,whichsol_ : i32,xx_ : * const f64) -> i32;
    fn MSK_putxxslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,xx_ : * const f64) -> i32;
    fn MSK_puty(task_ : * const u8,whichsol_ : i32,y_ : * const f64) -> i32;
    fn MSK_putyslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,y_ : * const f64) -> i32;
    fn MSK_readdataautoformat(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
    fn MSK_readdataformat(task_ : * const u8,filename_ : * const libc::c_char,format_ : i32,compress_ : i32) -> i32;
    fn MSK_readjsonstring(task_ : * const u8,data_ : * const libc::c_char) -> i32;
    fn MSK_readlpstring(task_ : * const u8,data_ : * const libc::c_char) -> i32;
    fn MSK_readopfstring(task_ : * const u8,data_ : * const libc::c_char) -> i32;
    fn MSK_readparamfile(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
    fn MSK_readptfstring(task_ : * const u8,data_ : * const libc::c_char) -> i32;
    fn MSK_readsolution(task_ : * const u8,whichsol_ : i32,filename_ : * const libc::c_char) -> i32;
    fn MSK_readsummary(task_ : * const u8,whichstream_ : i32) -> i32;
    fn MSK_readtask(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
    fn MSK_removebarvars(task_ : * const u8,num_ : i32,subset_ : * const i32) -> i32;
    fn MSK_removecones(task_ : * const u8,num_ : i32,subset_ : * const i32) -> i32;
    fn MSK_removecons(task_ : * const u8,num_ : i32,subset_ : * const i32) -> i32;
    fn MSK_removevars(task_ : * const u8,num_ : i32,subset_ : * const i32) -> i32;
    fn MSK_resizetask(task_ : * const u8,maxnumcon_ : i32,maxnumvar_ : i32,maxnumcone_ : i32,maxnumanz_ : i64,maxnumqnz_ : i64) -> i32;
    fn MSK_sensitivityreport(task_ : * const u8,whichstream_ : i32) -> i32;
    fn MSK_setdefaults(task_ : * const u8) -> i32;
    fn MSK_setupthreads(env_ : * const u8,numthreads_ : i32) -> i32;
    fn MSK_sktostr(task_ : * const u8,sk_ : i32,str_ : * const u8) -> i32;
    fn MSK_solstatostr(task_ : * const u8,solsta_ : i32,str_ : * const u8) -> i32;
    fn MSK_solutiondef(task_ : * const u8,whichsol_ : i32,isdef_ : & mut i32) -> i32;
    fn MSK_solutionsummary(task_ : * const u8,whichstream_ : i32) -> i32;
    fn MSK_solvewithbasis(task_ : * const u8,transp_ : i32,numnz_ : & mut i32,sub_ : * mut i32,val_ : * mut f64) -> i32;
    fn MSK_sparsetriangularsolvedense(env_ : * const u8,transposed_ : i32,n_ : i32,lnzc_ : * const i32,lptrc_ : * const i64,lensubnval_ : i64,lsubc_ : * const i32,lvalc_ : * const f64,b_ : * mut f64) -> i32;
    fn MSK_strduptask(task_ : * const u8,str_ : * const libc::c_char) -> i32;
    fn MSK_strtoconetype(task_ : * const u8,str_ : * const libc::c_char,conetype_ : & mut i32) -> i32;
    fn MSK_strtosk(task_ : * const u8,str_ : * const libc::c_char,sk_ : & mut i32) -> i32;
    fn MSK_syeig(env_ : * const u8,uplo_ : i32,n_ : i32,a_ : * const f64,w_ : * mut f64) -> i32;
    fn MSK_syevd(env_ : * const u8,uplo_ : i32,n_ : i32,a_ : * mut f64,w_ : * mut f64) -> i32;
    fn MSK_symnamtovalue(name_ : * const libc::c_char,value_ : * const u8) -> i32;
    fn MSK_syrk(env_ : * const u8,uplo_ : i32,trans_ : i32,n_ : i32,k_ : i32,alpha_ : f64,a_ : * const f64,beta_ : f64,c_ : * mut f64) -> i32;
    fn MSK_toconic(task_ : * const u8) -> i32;
    fn MSK_unlinkfuncfromenvstream(env_ : * const u8,whichstream_ : i32) -> i32;
    fn MSK_unlinkfuncfromtaskstream(task_ : * const u8,whichstream_ : i32) -> i32;
    fn MSK_updatesolutioninfo(task_ : * const u8,whichsol_ : i32) -> i32;
    fn MSK_whichparam(task_ : * const u8,parname_ : * const libc::c_char,partype_ : & mut i32,param_ : & mut i32) -> i32;
    fn MSK_writedata(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
    fn MSK_writejsonsol(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
    fn MSK_writeparamfile(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
    fn MSK_writesolution(task_ : * const u8,whichsol_ : i32,filename_ : * const libc::c_char) -> i32;
    fn MSK_writetask(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
}

macro_rules! callMSK
{
    ( $n:ident, $( $a : expr ),* ) => {
        {
            let res = unsafe { $n ( $( $a,)* ) };
            if 0 != res
            {
                return Result::Err(format!("Error in call to {:?}: {:?}",stringify!($n),res));
            }
        }
    };
    ( $n:ident ) => {
        {
            let res = unsafe { $n () };
            if 0 != res
            {
                return Result::Err(format!("Error in call to {:?}: {:?}",stringify!($n),res));
            }
        }
    }
}

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
pub const MSK_BRANCH_DIR_DOWN       : i32 = 2;
pub const MSK_BRANCH_DIR_FAR        : i32 = 4;
pub const MSK_BRANCH_DIR_FREE       : i32 = 0;
pub const MSK_BRANCH_DIR_GUIDED     : i32 = 6;
pub const MSK_BRANCH_DIR_NEAR       : i32 = 3;
pub const MSK_BRANCH_DIR_PSEUDOCOST : i32 = 7;
pub const MSK_BRANCH_DIR_ROOT_LP    : i32 = 5;
pub const MSK_BRANCH_DIR_UP         : i32 = 1;
pub const MSK_BRANCH_DIR_BEGIN : i32 = 0;
pub const MSK_BRANCH_DIR_END   : i32 = 8;

// callbackcode
pub const MSK_CALLBACK_BEGIN_BI                 : i32 = 0;
pub const MSK_CALLBACK_BEGIN_CONIC              : i32 = 1;
pub const MSK_CALLBACK_BEGIN_DUAL_BI            : i32 = 2;
pub const MSK_CALLBACK_BEGIN_DUAL_SENSITIVITY   : i32 = 3;
pub const MSK_CALLBACK_BEGIN_DUAL_SETUP_BI      : i32 = 4;
pub const MSK_CALLBACK_BEGIN_DUAL_SIMPLEX       : i32 = 5;
pub const MSK_CALLBACK_BEGIN_DUAL_SIMPLEX_BI    : i32 = 6;
pub const MSK_CALLBACK_BEGIN_INFEAS_ANA         : i32 = 7;
pub const MSK_CALLBACK_BEGIN_INTPNT             : i32 = 8;
pub const MSK_CALLBACK_BEGIN_LICENSE_WAIT       : i32 = 9;
pub const MSK_CALLBACK_BEGIN_MIO                : i32 = 10;
pub const MSK_CALLBACK_BEGIN_OPTIMIZER          : i32 = 11;
pub const MSK_CALLBACK_BEGIN_PRESOLVE           : i32 = 12;
pub const MSK_CALLBACK_BEGIN_PRIMAL_BI          : i32 = 13;
pub const MSK_CALLBACK_BEGIN_PRIMAL_REPAIR      : i32 = 14;
pub const MSK_CALLBACK_BEGIN_PRIMAL_SENSITIVITY : i32 = 15;
pub const MSK_CALLBACK_BEGIN_PRIMAL_SETUP_BI    : i32 = 16;
pub const MSK_CALLBACK_BEGIN_PRIMAL_SIMPLEX     : i32 = 17;
pub const MSK_CALLBACK_BEGIN_PRIMAL_SIMPLEX_BI  : i32 = 18;
pub const MSK_CALLBACK_BEGIN_QCQO_REFORMULATE   : i32 = 19;
pub const MSK_CALLBACK_BEGIN_READ               : i32 = 20;
pub const MSK_CALLBACK_BEGIN_ROOT_CUTGEN        : i32 = 21;
pub const MSK_CALLBACK_BEGIN_SIMPLEX            : i32 = 22;
pub const MSK_CALLBACK_BEGIN_SIMPLEX_BI         : i32 = 23;
pub const MSK_CALLBACK_BEGIN_TO_CONIC           : i32 = 24;
pub const MSK_CALLBACK_BEGIN_WRITE              : i32 = 25;
pub const MSK_CALLBACK_CONIC                    : i32 = 26;
pub const MSK_CALLBACK_DUAL_SIMPLEX             : i32 = 27;
pub const MSK_CALLBACK_END_BI                   : i32 = 28;
pub const MSK_CALLBACK_END_CONIC                : i32 = 29;
pub const MSK_CALLBACK_END_DUAL_BI              : i32 = 30;
pub const MSK_CALLBACK_END_DUAL_SENSITIVITY     : i32 = 31;
pub const MSK_CALLBACK_END_DUAL_SETUP_BI        : i32 = 32;
pub const MSK_CALLBACK_END_DUAL_SIMPLEX         : i32 = 33;
pub const MSK_CALLBACK_END_DUAL_SIMPLEX_BI      : i32 = 34;
pub const MSK_CALLBACK_END_INFEAS_ANA           : i32 = 35;
pub const MSK_CALLBACK_END_INTPNT               : i32 = 36;
pub const MSK_CALLBACK_END_LICENSE_WAIT         : i32 = 37;
pub const MSK_CALLBACK_END_MIO                  : i32 = 38;
pub const MSK_CALLBACK_END_OPTIMIZER            : i32 = 39;
pub const MSK_CALLBACK_END_PRESOLVE             : i32 = 40;
pub const MSK_CALLBACK_END_PRIMAL_BI            : i32 = 41;
pub const MSK_CALLBACK_END_PRIMAL_REPAIR        : i32 = 42;
pub const MSK_CALLBACK_END_PRIMAL_SENSITIVITY   : i32 = 43;
pub const MSK_CALLBACK_END_PRIMAL_SETUP_BI      : i32 = 44;
pub const MSK_CALLBACK_END_PRIMAL_SIMPLEX       : i32 = 45;
pub const MSK_CALLBACK_END_PRIMAL_SIMPLEX_BI    : i32 = 46;
pub const MSK_CALLBACK_END_QCQO_REFORMULATE     : i32 = 47;
pub const MSK_CALLBACK_END_READ                 : i32 = 48;
pub const MSK_CALLBACK_END_ROOT_CUTGEN          : i32 = 49;
pub const MSK_CALLBACK_END_SIMPLEX              : i32 = 50;
pub const MSK_CALLBACK_END_SIMPLEX_BI           : i32 = 51;
pub const MSK_CALLBACK_END_TO_CONIC             : i32 = 52;
pub const MSK_CALLBACK_END_WRITE                : i32 = 53;
pub const MSK_CALLBACK_IM_BI                    : i32 = 54;
pub const MSK_CALLBACK_IM_CONIC                 : i32 = 55;
pub const MSK_CALLBACK_IM_DUAL_BI               : i32 = 56;
pub const MSK_CALLBACK_IM_DUAL_SENSIVITY        : i32 = 57;
pub const MSK_CALLBACK_IM_DUAL_SIMPLEX          : i32 = 58;
pub const MSK_CALLBACK_IM_INTPNT                : i32 = 59;
pub const MSK_CALLBACK_IM_LICENSE_WAIT          : i32 = 60;
pub const MSK_CALLBACK_IM_LU                    : i32 = 61;
pub const MSK_CALLBACK_IM_MIO                   : i32 = 62;
pub const MSK_CALLBACK_IM_MIO_DUAL_SIMPLEX      : i32 = 63;
pub const MSK_CALLBACK_IM_MIO_INTPNT            : i32 = 64;
pub const MSK_CALLBACK_IM_MIO_PRIMAL_SIMPLEX    : i32 = 65;
pub const MSK_CALLBACK_IM_ORDER                 : i32 = 66;
pub const MSK_CALLBACK_IM_PRESOLVE              : i32 = 67;
pub const MSK_CALLBACK_IM_PRIMAL_BI             : i32 = 68;
pub const MSK_CALLBACK_IM_PRIMAL_SENSIVITY      : i32 = 69;
pub const MSK_CALLBACK_IM_PRIMAL_SIMPLEX        : i32 = 70;
pub const MSK_CALLBACK_IM_QO_REFORMULATE        : i32 = 71;
pub const MSK_CALLBACK_IM_READ                  : i32 = 72;
pub const MSK_CALLBACK_IM_ROOT_CUTGEN           : i32 = 73;
pub const MSK_CALLBACK_IM_SIMPLEX               : i32 = 74;
pub const MSK_CALLBACK_IM_SIMPLEX_BI            : i32 = 75;
pub const MSK_CALLBACK_INTPNT                   : i32 = 76;
pub const MSK_CALLBACK_NEW_INT_MIO              : i32 = 77;
pub const MSK_CALLBACK_PRIMAL_SIMPLEX           : i32 = 78;
pub const MSK_CALLBACK_READ_OPF                 : i32 = 79;
pub const MSK_CALLBACK_READ_OPF_SECTION         : i32 = 80;
pub const MSK_CALLBACK_SOLVING_REMOTE           : i32 = 81;
pub const MSK_CALLBACK_UPDATE_DUAL_BI           : i32 = 82;
pub const MSK_CALLBACK_UPDATE_DUAL_SIMPLEX      : i32 = 83;
pub const MSK_CALLBACK_UPDATE_DUAL_SIMPLEX_BI   : i32 = 84;
pub const MSK_CALLBACK_UPDATE_PRESOLVE          : i32 = 85;
pub const MSK_CALLBACK_UPDATE_PRIMAL_BI         : i32 = 86;
pub const MSK_CALLBACK_UPDATE_PRIMAL_SIMPLEX    : i32 = 87;
pub const MSK_CALLBACK_UPDATE_PRIMAL_SIMPLEX_BI : i32 = 88;
pub const MSK_CALLBACK_UPDATE_SIMPLEX           : i32 = 89;
pub const MSK_CALLBACK_WRITE_OPF                : i32 = 90;
pub const MSK_CALLBACK_BEGIN : i32 = 0;
pub const MSK_CALLBACK_END   : i32 = 91;

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
pub const MSK_COMPRESS_ZSTD : i32 = 3;
pub const MSK_COMPRESS_BEGIN : i32 = 0;
pub const MSK_COMPRESS_END   : i32 = 4;

// conetype
pub const MSK_CT_DEXP     : i32 = 3;
pub const MSK_CT_DGEOMEAN : i32 = 10;
pub const MSK_CT_DPOW     : i32 = 5;
pub const MSK_CT_INFNRM   : i32 = 7;
pub const MSK_CT_ONENRM   : i32 = 8;
pub const MSK_CT_PEXP     : i32 = 2;
pub const MSK_CT_PGEOMEAN : i32 = 9;
pub const MSK_CT_PPOW     : i32 = 4;
pub const MSK_CT_QUAD     : i32 = 0;
pub const MSK_CT_RQUAD    : i32 = 1;
pub const MSK_CT_ZERO     : i32 = 6;
pub const MSK_CT_BEGIN : i32 = 0;
pub const MSK_CT_END   : i32 = 11;

// dataformat
pub const MSK_DATA_FORMAT_CB        : i32 = 7;
pub const MSK_DATA_FORMAT_EXTENSION : i32 = 0;
pub const MSK_DATA_FORMAT_FREE_MPS  : i32 = 4;
pub const MSK_DATA_FORMAT_JSON_TASK : i32 = 8;
pub const MSK_DATA_FORMAT_LP        : i32 = 2;
pub const MSK_DATA_FORMAT_MPS       : i32 = 1;
pub const MSK_DATA_FORMAT_OP        : i32 = 3;
pub const MSK_DATA_FORMAT_PTF       : i32 = 6;
pub const MSK_DATA_FORMAT_TASK      : i32 = 5;
pub const MSK_DATA_FORMAT_BEGIN : i32 = 0;
pub const MSK_DATA_FORMAT_END   : i32 = 9;

// dinfitem
pub const MSK_DINF_BI_CLEAN_DUAL_TIME                             : i32 = 0;
pub const MSK_DINF_BI_CLEAN_PRIMAL_TIME                           : i32 = 1;
pub const MSK_DINF_BI_CLEAN_TIME                                  : i32 = 2;
pub const MSK_DINF_BI_DUAL_TIME                                   : i32 = 3;
pub const MSK_DINF_BI_PRIMAL_TIME                                 : i32 = 4;
pub const MSK_DINF_BI_TIME                                        : i32 = 5;
pub const MSK_DINF_INTPNT_DUAL_FEAS                               : i32 = 6;
pub const MSK_DINF_INTPNT_DUAL_OBJ                                : i32 = 7;
pub const MSK_DINF_INTPNT_FACTOR_NUM_FLOPS                        : i32 = 8;
pub const MSK_DINF_INTPNT_OPT_STATUS                              : i32 = 9;
pub const MSK_DINF_INTPNT_ORDER_TIME                              : i32 = 10;
pub const MSK_DINF_INTPNT_PRIMAL_FEAS                             : i32 = 11;
pub const MSK_DINF_INTPNT_PRIMAL_OBJ                              : i32 = 12;
pub const MSK_DINF_INTPNT_TIME                                    : i32 = 13;
pub const MSK_DINF_MIO_CLIQUE_SEPARATION_TIME                     : i32 = 14;
pub const MSK_DINF_MIO_CMIR_SEPARATION_TIME                       : i32 = 15;
pub const MSK_DINF_MIO_CONSTRUCT_SOLUTION_OBJ                     : i32 = 16;
pub const MSK_DINF_MIO_DUAL_BOUND_AFTER_PRESOLVE                  : i32 = 17;
pub const MSK_DINF_MIO_GMI_SEPARATION_TIME                        : i32 = 18;
pub const MSK_DINF_MIO_IMPLIED_BOUND_TIME                         : i32 = 19;
pub const MSK_DINF_MIO_KNAPSACK_COVER_SEPARATION_TIME             : i32 = 20;
pub const MSK_DINF_MIO_OBJ_ABS_GAP                                : i32 = 21;
pub const MSK_DINF_MIO_OBJ_BOUND                                  : i32 = 22;
pub const MSK_DINF_MIO_OBJ_INT                                    : i32 = 23;
pub const MSK_DINF_MIO_OBJ_REL_GAP                                : i32 = 24;
pub const MSK_DINF_MIO_PROBING_TIME                               : i32 = 25;
pub const MSK_DINF_MIO_ROOT_CUTGEN_TIME                           : i32 = 26;
pub const MSK_DINF_MIO_ROOT_OPTIMIZER_TIME                        : i32 = 27;
pub const MSK_DINF_MIO_ROOT_PRESOLVE_TIME                         : i32 = 28;
pub const MSK_DINF_MIO_TIME                                       : i32 = 29;
pub const MSK_DINF_MIO_USER_OBJ_CUT                               : i32 = 30;
pub const MSK_DINF_OPTIMIZER_TIME                                 : i32 = 31;
pub const MSK_DINF_PRESOLVE_ELI_TIME                              : i32 = 32;
pub const MSK_DINF_PRESOLVE_LINDEP_TIME                           : i32 = 33;
pub const MSK_DINF_PRESOLVE_TIME                                  : i32 = 34;
pub const MSK_DINF_PRIMAL_REPAIR_PENALTY_OBJ                      : i32 = 35;
pub const MSK_DINF_QCQO_REFORMULATE_MAX_PERTURBATION              : i32 = 36;
pub const MSK_DINF_QCQO_REFORMULATE_TIME                          : i32 = 37;
pub const MSK_DINF_QCQO_REFORMULATE_WORST_CHOLESKY_COLUMN_SCALING : i32 = 38;
pub const MSK_DINF_QCQO_REFORMULATE_WORST_CHOLESKY_DIAG_SCALING   : i32 = 39;
pub const MSK_DINF_RD_TIME                                        : i32 = 40;
pub const MSK_DINF_SIM_DUAL_TIME                                  : i32 = 41;
pub const MSK_DINF_SIM_FEAS                                       : i32 = 42;
pub const MSK_DINF_SIM_OBJ                                        : i32 = 43;
pub const MSK_DINF_SIM_PRIMAL_TIME                                : i32 = 44;
pub const MSK_DINF_SIM_TIME                                       : i32 = 45;
pub const MSK_DINF_SOL_BAS_DUAL_OBJ                               : i32 = 46;
pub const MSK_DINF_SOL_BAS_DVIOLCON                               : i32 = 47;
pub const MSK_DINF_SOL_BAS_DVIOLVAR                               : i32 = 48;
pub const MSK_DINF_SOL_BAS_NRM_BARX                               : i32 = 49;
pub const MSK_DINF_SOL_BAS_NRM_SLC                                : i32 = 50;
pub const MSK_DINF_SOL_BAS_NRM_SLX                                : i32 = 51;
pub const MSK_DINF_SOL_BAS_NRM_SUC                                : i32 = 52;
pub const MSK_DINF_SOL_BAS_NRM_SUX                                : i32 = 53;
pub const MSK_DINF_SOL_BAS_NRM_XC                                 : i32 = 54;
pub const MSK_DINF_SOL_BAS_NRM_XX                                 : i32 = 55;
pub const MSK_DINF_SOL_BAS_NRM_Y                                  : i32 = 56;
pub const MSK_DINF_SOL_BAS_PRIMAL_OBJ                             : i32 = 57;
pub const MSK_DINF_SOL_BAS_PVIOLCON                               : i32 = 58;
pub const MSK_DINF_SOL_BAS_PVIOLVAR                               : i32 = 59;
pub const MSK_DINF_SOL_ITG_NRM_BARX                               : i32 = 60;
pub const MSK_DINF_SOL_ITG_NRM_XC                                 : i32 = 61;
pub const MSK_DINF_SOL_ITG_NRM_XX                                 : i32 = 62;
pub const MSK_DINF_SOL_ITG_PRIMAL_OBJ                             : i32 = 63;
pub const MSK_DINF_SOL_ITG_PVIOLBARVAR                            : i32 = 64;
pub const MSK_DINF_SOL_ITG_PVIOLCON                               : i32 = 65;
pub const MSK_DINF_SOL_ITG_PVIOLCONES                             : i32 = 66;
pub const MSK_DINF_SOL_ITG_PVIOLITG                               : i32 = 67;
pub const MSK_DINF_SOL_ITG_PVIOLVAR                               : i32 = 68;
pub const MSK_DINF_SOL_ITR_DUAL_OBJ                               : i32 = 69;
pub const MSK_DINF_SOL_ITR_DVIOLACC                               : i32 = 70;
pub const MSK_DINF_SOL_ITR_DVIOLBARVAR                            : i32 = 71;
pub const MSK_DINF_SOL_ITR_DVIOLCON                               : i32 = 72;
pub const MSK_DINF_SOL_ITR_DVIOLCONES                             : i32 = 73;
pub const MSK_DINF_SOL_ITR_DVIOLVAR                               : i32 = 74;
pub const MSK_DINF_SOL_ITR_NRM_BARS                               : i32 = 75;
pub const MSK_DINF_SOL_ITR_NRM_BARX                               : i32 = 76;
pub const MSK_DINF_SOL_ITR_NRM_SLC                                : i32 = 77;
pub const MSK_DINF_SOL_ITR_NRM_SLX                                : i32 = 78;
pub const MSK_DINF_SOL_ITR_NRM_SNX                                : i32 = 79;
pub const MSK_DINF_SOL_ITR_NRM_SUC                                : i32 = 80;
pub const MSK_DINF_SOL_ITR_NRM_SUX                                : i32 = 81;
pub const MSK_DINF_SOL_ITR_NRM_XC                                 : i32 = 82;
pub const MSK_DINF_SOL_ITR_NRM_XX                                 : i32 = 83;
pub const MSK_DINF_SOL_ITR_NRM_Y                                  : i32 = 84;
pub const MSK_DINF_SOL_ITR_PRIMAL_OBJ                             : i32 = 85;
pub const MSK_DINF_SOL_ITR_PVIOLACC                               : i32 = 86;
pub const MSK_DINF_SOL_ITR_PVIOLBARVAR                            : i32 = 87;
pub const MSK_DINF_SOL_ITR_PVIOLCON                               : i32 = 88;
pub const MSK_DINF_SOL_ITR_PVIOLCONES                             : i32 = 89;
pub const MSK_DINF_SOL_ITR_PVIOLVAR                               : i32 = 90;
pub const MSK_DINF_TO_CONIC_TIME                                  : i32 = 91;
pub const MSK_DINF_BEGIN : i32 = 0;
pub const MSK_DINF_END   : i32 = 92;

// domaintype
pub const MSK_DOMAIN_DUAL_EXP_CONE        : i32 = 7;
pub const MSK_DOMAIN_DUAL_GEO_MEAN_CONE   : i32 = 11;
pub const MSK_DOMAIN_DUAL_POWER_CONE      : i32 = 9;
pub const MSK_DOMAIN_INF_NORM_CONE        : i32 = 12;
pub const MSK_DOMAIN_ONE_NORM_CONE        : i32 = 13;
pub const MSK_DOMAIN_PRIMAL_EXP_CONE      : i32 = 6;
pub const MSK_DOMAIN_PRIMAL_GEO_MEAN_CONE : i32 = 10;
pub const MSK_DOMAIN_PRIMAL_POWER_CONE    : i32 = 8;
pub const MSK_DOMAIN_PSD_CONE             : i32 = 14;
pub const MSK_DOMAIN_QUADRATIC_CONE       : i32 = 4;
pub const MSK_DOMAIN_R                    : i32 = 0;
pub const MSK_DOMAIN_RMINUS               : i32 = 3;
pub const MSK_DOMAIN_RPLUS                : i32 = 2;
pub const MSK_DOMAIN_RQUADRATIC_CONE      : i32 = 5;
pub const MSK_DOMAIN_RZERO                : i32 = 1;
pub const MSK_DOMAIN_BEGIN : i32 = 0;
pub const MSK_DOMAIN_END   : i32 = 15;

// dparam
pub const MSK_DPAR_ANA_SOL_INFEAS_TOL                 : i32 = 0;
pub const MSK_DPAR_BASIS_REL_TOL_S                    : i32 = 1;
pub const MSK_DPAR_BASIS_TOL_S                        : i32 = 2;
pub const MSK_DPAR_BASIS_TOL_X                        : i32 = 3;
pub const MSK_DPAR_CHECK_CONVEXITY_REL_TOL            : i32 = 4;
pub const MSK_DPAR_DATA_SYM_MAT_TOL                   : i32 = 5;
pub const MSK_DPAR_DATA_SYM_MAT_TOL_HUGE              : i32 = 6;
pub const MSK_DPAR_DATA_SYM_MAT_TOL_LARGE             : i32 = 7;
pub const MSK_DPAR_DATA_TOL_AIJ_HUGE                  : i32 = 8;
pub const MSK_DPAR_DATA_TOL_AIJ_LARGE                 : i32 = 9;
pub const MSK_DPAR_DATA_TOL_BOUND_INF                 : i32 = 10;
pub const MSK_DPAR_DATA_TOL_BOUND_WRN                 : i32 = 11;
pub const MSK_DPAR_DATA_TOL_C_HUGE                    : i32 = 12;
pub const MSK_DPAR_DATA_TOL_CJ_LARGE                  : i32 = 13;
pub const MSK_DPAR_DATA_TOL_QIJ                       : i32 = 14;
pub const MSK_DPAR_DATA_TOL_X                         : i32 = 15;
pub const MSK_DPAR_INTPNT_CO_TOL_DFEAS                : i32 = 16;
pub const MSK_DPAR_INTPNT_CO_TOL_INFEAS               : i32 = 17;
pub const MSK_DPAR_INTPNT_CO_TOL_MU_RED               : i32 = 18;
pub const MSK_DPAR_INTPNT_CO_TOL_NEAR_REL             : i32 = 19;
pub const MSK_DPAR_INTPNT_CO_TOL_PFEAS                : i32 = 20;
pub const MSK_DPAR_INTPNT_CO_TOL_REL_GAP              : i32 = 21;
pub const MSK_DPAR_INTPNT_QO_TOL_DFEAS                : i32 = 22;
pub const MSK_DPAR_INTPNT_QO_TOL_INFEAS               : i32 = 23;
pub const MSK_DPAR_INTPNT_QO_TOL_MU_RED               : i32 = 24;
pub const MSK_DPAR_INTPNT_QO_TOL_NEAR_REL             : i32 = 25;
pub const MSK_DPAR_INTPNT_QO_TOL_PFEAS                : i32 = 26;
pub const MSK_DPAR_INTPNT_QO_TOL_REL_GAP              : i32 = 27;
pub const MSK_DPAR_INTPNT_TOL_DFEAS                   : i32 = 28;
pub const MSK_DPAR_INTPNT_TOL_DSAFE                   : i32 = 29;
pub const MSK_DPAR_INTPNT_TOL_INFEAS                  : i32 = 30;
pub const MSK_DPAR_INTPNT_TOL_MU_RED                  : i32 = 31;
pub const MSK_DPAR_INTPNT_TOL_PATH                    : i32 = 32;
pub const MSK_DPAR_INTPNT_TOL_PFEAS                   : i32 = 33;
pub const MSK_DPAR_INTPNT_TOL_PSAFE                   : i32 = 34;
pub const MSK_DPAR_INTPNT_TOL_REL_GAP                 : i32 = 35;
pub const MSK_DPAR_INTPNT_TOL_REL_STEP                : i32 = 36;
pub const MSK_DPAR_INTPNT_TOL_STEP_SIZE               : i32 = 37;
pub const MSK_DPAR_LOWER_OBJ_CUT                      : i32 = 38;
pub const MSK_DPAR_LOWER_OBJ_CUT_FINITE_TRH           : i32 = 39;
pub const MSK_DPAR_MIO_MAX_TIME                       : i32 = 40;
pub const MSK_DPAR_MIO_REL_GAP_CONST                  : i32 = 41;
pub const MSK_DPAR_MIO_TOL_ABS_GAP                    : i32 = 42;
pub const MSK_DPAR_MIO_TOL_ABS_RELAX_INT              : i32 = 43;
pub const MSK_DPAR_MIO_TOL_FEAS                       : i32 = 44;
pub const MSK_DPAR_MIO_TOL_REL_DUAL_BOUND_IMPROVEMENT : i32 = 45;
pub const MSK_DPAR_MIO_TOL_REL_GAP                    : i32 = 46;
pub const MSK_DPAR_OPTIMIZER_MAX_TIME                 : i32 = 47;
pub const MSK_DPAR_PRESOLVE_TOL_ABS_LINDEP            : i32 = 48;
pub const MSK_DPAR_PRESOLVE_TOL_AIJ                   : i32 = 49;
pub const MSK_DPAR_PRESOLVE_TOL_REL_LINDEP            : i32 = 50;
pub const MSK_DPAR_PRESOLVE_TOL_S                     : i32 = 51;
pub const MSK_DPAR_PRESOLVE_TOL_X                     : i32 = 52;
pub const MSK_DPAR_QCQO_REFORMULATE_REL_DROP_TOL      : i32 = 53;
pub const MSK_DPAR_SEMIDEFINITE_TOL_APPROX            : i32 = 54;
pub const MSK_DPAR_SIM_LU_TOL_REL_PIV                 : i32 = 55;
pub const MSK_DPAR_SIMPLEX_ABS_TOL_PIV                : i32 = 56;
pub const MSK_DPAR_UPPER_OBJ_CUT                      : i32 = 57;
pub const MSK_DPAR_UPPER_OBJ_CUT_FINITE_TRH           : i32 = 58;
pub const MSK_DPAR_BEGIN : i32 = 0;
pub const MSK_DPAR_END   : i32 = 59;

// feature
pub const MSK_FEATURE_PTON : i32 = 1;
pub const MSK_FEATURE_PTS  : i32 = 0;
pub const MSK_FEATURE_BEGIN : i32 = 0;
pub const MSK_FEATURE_END   : i32 = 2;

// iinfitem
pub const MSK_IINF_ANA_PRO_NUM_CON              : i32 = 0;
pub const MSK_IINF_ANA_PRO_NUM_CON_EQ           : i32 = 1;
pub const MSK_IINF_ANA_PRO_NUM_CON_FR           : i32 = 2;
pub const MSK_IINF_ANA_PRO_NUM_CON_LO           : i32 = 3;
pub const MSK_IINF_ANA_PRO_NUM_CON_RA           : i32 = 4;
pub const MSK_IINF_ANA_PRO_NUM_CON_UP           : i32 = 5;
pub const MSK_IINF_ANA_PRO_NUM_VAR              : i32 = 6;
pub const MSK_IINF_ANA_PRO_NUM_VAR_BIN          : i32 = 7;
pub const MSK_IINF_ANA_PRO_NUM_VAR_CONT         : i32 = 8;
pub const MSK_IINF_ANA_PRO_NUM_VAR_EQ           : i32 = 9;
pub const MSK_IINF_ANA_PRO_NUM_VAR_FR           : i32 = 10;
pub const MSK_IINF_ANA_PRO_NUM_VAR_INT          : i32 = 11;
pub const MSK_IINF_ANA_PRO_NUM_VAR_LO           : i32 = 12;
pub const MSK_IINF_ANA_PRO_NUM_VAR_RA           : i32 = 13;
pub const MSK_IINF_ANA_PRO_NUM_VAR_UP           : i32 = 14;
pub const MSK_IINF_INTPNT_FACTOR_DIM_DENSE      : i32 = 15;
pub const MSK_IINF_INTPNT_ITER                  : i32 = 16;
pub const MSK_IINF_INTPNT_NUM_THREADS           : i32 = 17;
pub const MSK_IINF_INTPNT_SOLVE_DUAL            : i32 = 18;
pub const MSK_IINF_MIO_ABSGAP_SATISFIED         : i32 = 19;
pub const MSK_IINF_MIO_CLIQUE_TABLE_SIZE        : i32 = 20;
pub const MSK_IINF_MIO_CONSTRUCT_SOLUTION       : i32 = 21;
pub const MSK_IINF_MIO_NODE_DEPTH               : i32 = 22;
pub const MSK_IINF_MIO_NUM_ACTIVE_NODES         : i32 = 23;
pub const MSK_IINF_MIO_NUM_BRANCH               : i32 = 24;
pub const MSK_IINF_MIO_NUM_CLIQUE_CUTS          : i32 = 25;
pub const MSK_IINF_MIO_NUM_CMIR_CUTS            : i32 = 26;
pub const MSK_IINF_MIO_NUM_GOMORY_CUTS          : i32 = 27;
pub const MSK_IINF_MIO_NUM_IMPLIED_BOUND_CUTS   : i32 = 28;
pub const MSK_IINF_MIO_NUM_INT_SOLUTIONS        : i32 = 29;
pub const MSK_IINF_MIO_NUM_KNAPSACK_COVER_CUTS  : i32 = 30;
pub const MSK_IINF_MIO_NUM_RELAX                : i32 = 31;
pub const MSK_IINF_MIO_NUM_REPEATED_PRESOLVE    : i32 = 32;
pub const MSK_IINF_MIO_NUMBIN                   : i32 = 33;
pub const MSK_IINF_MIO_NUMBINCONEVAR            : i32 = 34;
pub const MSK_IINF_MIO_NUMCON                   : i32 = 35;
pub const MSK_IINF_MIO_NUMCONE                  : i32 = 36;
pub const MSK_IINF_MIO_NUMCONEVAR               : i32 = 37;
pub const MSK_IINF_MIO_NUMCONT                  : i32 = 38;
pub const MSK_IINF_MIO_NUMCONTCONEVAR           : i32 = 39;
pub const MSK_IINF_MIO_NUMDEXPCONES             : i32 = 40;
pub const MSK_IINF_MIO_NUMDPOWCONES             : i32 = 41;
pub const MSK_IINF_MIO_NUMINT                   : i32 = 42;
pub const MSK_IINF_MIO_NUMINTCONEVAR            : i32 = 43;
pub const MSK_IINF_MIO_NUMPEXPCONES             : i32 = 44;
pub const MSK_IINF_MIO_NUMPPOWCONES             : i32 = 45;
pub const MSK_IINF_MIO_NUMQCONES                : i32 = 46;
pub const MSK_IINF_MIO_NUMRQCONES               : i32 = 47;
pub const MSK_IINF_MIO_NUMVAR                   : i32 = 48;
pub const MSK_IINF_MIO_OBJ_BOUND_DEFINED        : i32 = 49;
pub const MSK_IINF_MIO_PRESOLVED_NUMBIN         : i32 = 50;
pub const MSK_IINF_MIO_PRESOLVED_NUMBINCONEVAR  : i32 = 51;
pub const MSK_IINF_MIO_PRESOLVED_NUMCON         : i32 = 52;
pub const MSK_IINF_MIO_PRESOLVED_NUMCONE        : i32 = 53;
pub const MSK_IINF_MIO_PRESOLVED_NUMCONEVAR     : i32 = 54;
pub const MSK_IINF_MIO_PRESOLVED_NUMCONT        : i32 = 55;
pub const MSK_IINF_MIO_PRESOLVED_NUMCONTCONEVAR : i32 = 56;
pub const MSK_IINF_MIO_PRESOLVED_NUMDEXPCONES   : i32 = 57;
pub const MSK_IINF_MIO_PRESOLVED_NUMDPOWCONES   : i32 = 58;
pub const MSK_IINF_MIO_PRESOLVED_NUMINT         : i32 = 59;
pub const MSK_IINF_MIO_PRESOLVED_NUMINTCONEVAR  : i32 = 60;
pub const MSK_IINF_MIO_PRESOLVED_NUMPEXPCONES   : i32 = 61;
pub const MSK_IINF_MIO_PRESOLVED_NUMPPOWCONES   : i32 = 62;
pub const MSK_IINF_MIO_PRESOLVED_NUMQCONES      : i32 = 63;
pub const MSK_IINF_MIO_PRESOLVED_NUMRQCONES     : i32 = 64;
pub const MSK_IINF_MIO_PRESOLVED_NUMVAR         : i32 = 65;
pub const MSK_IINF_MIO_RELGAP_SATISFIED         : i32 = 66;
pub const MSK_IINF_MIO_TOTAL_NUM_CUTS           : i32 = 67;
pub const MSK_IINF_MIO_USER_OBJ_CUT             : i32 = 68;
pub const MSK_IINF_OPT_NUMCON                   : i32 = 69;
pub const MSK_IINF_OPT_NUMVAR                   : i32 = 70;
pub const MSK_IINF_OPTIMIZE_RESPONSE            : i32 = 71;
pub const MSK_IINF_PURIFY_DUAL_SUCCESS          : i32 = 72;
pub const MSK_IINF_PURIFY_PRIMAL_SUCCESS        : i32 = 73;
pub const MSK_IINF_RD_NUMBARVAR                 : i32 = 74;
pub const MSK_IINF_RD_NUMCON                    : i32 = 75;
pub const MSK_IINF_RD_NUMCONE                   : i32 = 76;
pub const MSK_IINF_RD_NUMINTVAR                 : i32 = 77;
pub const MSK_IINF_RD_NUMQ                      : i32 = 78;
pub const MSK_IINF_RD_NUMVAR                    : i32 = 79;
pub const MSK_IINF_RD_PROTYPE                   : i32 = 80;
pub const MSK_IINF_SIM_DUAL_DEG_ITER            : i32 = 81;
pub const MSK_IINF_SIM_DUAL_HOTSTART            : i32 = 82;
pub const MSK_IINF_SIM_DUAL_HOTSTART_LU         : i32 = 83;
pub const MSK_IINF_SIM_DUAL_INF_ITER            : i32 = 84;
pub const MSK_IINF_SIM_DUAL_ITER                : i32 = 85;
pub const MSK_IINF_SIM_NUMCON                   : i32 = 86;
pub const MSK_IINF_SIM_NUMVAR                   : i32 = 87;
pub const MSK_IINF_SIM_PRIMAL_DEG_ITER          : i32 = 88;
pub const MSK_IINF_SIM_PRIMAL_HOTSTART          : i32 = 89;
pub const MSK_IINF_SIM_PRIMAL_HOTSTART_LU       : i32 = 90;
pub const MSK_IINF_SIM_PRIMAL_INF_ITER          : i32 = 91;
pub const MSK_IINF_SIM_PRIMAL_ITER              : i32 = 92;
pub const MSK_IINF_SIM_SOLVE_DUAL               : i32 = 93;
pub const MSK_IINF_SOL_BAS_PROSTA               : i32 = 94;
pub const MSK_IINF_SOL_BAS_SOLSTA               : i32 = 95;
pub const MSK_IINF_SOL_ITG_PROSTA               : i32 = 96;
pub const MSK_IINF_SOL_ITG_SOLSTA               : i32 = 97;
pub const MSK_IINF_SOL_ITR_PROSTA               : i32 = 98;
pub const MSK_IINF_SOL_ITR_SOLSTA               : i32 = 99;
pub const MSK_IINF_STO_NUM_A_REALLOC            : i32 = 100;
pub const MSK_IINF_BEGIN : i32 = 0;
pub const MSK_IINF_END   : i32 = 101;

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
pub const MSK_IPAR_ANA_SOL_BASIS                      : i32 = 0;
pub const MSK_IPAR_ANA_SOL_PRINT_VIOLATED             : i32 = 1;
pub const MSK_IPAR_AUTO_SORT_A_BEFORE_OPT             : i32 = 2;
pub const MSK_IPAR_AUTO_UPDATE_SOL_INFO               : i32 = 3;
pub const MSK_IPAR_BASIS_SOLVE_USE_PLUS_ONE           : i32 = 4;
pub const MSK_IPAR_BI_CLEAN_OPTIMIZER                 : i32 = 5;
pub const MSK_IPAR_BI_IGNORE_MAX_ITER                 : i32 = 6;
pub const MSK_IPAR_BI_IGNORE_NUM_ERROR                : i32 = 7;
pub const MSK_IPAR_BI_MAX_ITERATIONS                  : i32 = 8;
pub const MSK_IPAR_CACHE_LICENSE                      : i32 = 9;
pub const MSK_IPAR_CHECK_CONVEXITY                    : i32 = 10;
pub const MSK_IPAR_COMPRESS_STATFILE                  : i32 = 11;
pub const MSK_IPAR_INFEAS_GENERIC_NAMES               : i32 = 12;
pub const MSK_IPAR_INFEAS_PREFER_PRIMAL               : i32 = 13;
pub const MSK_IPAR_INFEAS_REPORT_AUTO                 : i32 = 14;
pub const MSK_IPAR_INFEAS_REPORT_LEVEL                : i32 = 15;
pub const MSK_IPAR_INTPNT_BASIS                       : i32 = 16;
pub const MSK_IPAR_INTPNT_DIFF_STEP                   : i32 = 17;
pub const MSK_IPAR_INTPNT_HOTSTART                    : i32 = 18;
pub const MSK_IPAR_INTPNT_MAX_ITERATIONS              : i32 = 19;
pub const MSK_IPAR_INTPNT_MAX_NUM_COR                 : i32 = 20;
pub const MSK_IPAR_INTPNT_MAX_NUM_REFINEMENT_STEPS    : i32 = 21;
pub const MSK_IPAR_INTPNT_MULTI_THREAD                : i32 = 22;
pub const MSK_IPAR_INTPNT_OFF_COL_TRH                 : i32 = 23;
pub const MSK_IPAR_INTPNT_ORDER_GP_NUM_SEEDS          : i32 = 24;
pub const MSK_IPAR_INTPNT_ORDER_METHOD                : i32 = 25;
pub const MSK_IPAR_INTPNT_PURIFY                      : i32 = 26;
pub const MSK_IPAR_INTPNT_REGULARIZATION_USE          : i32 = 27;
pub const MSK_IPAR_INTPNT_SCALING                     : i32 = 28;
pub const MSK_IPAR_INTPNT_SOLVE_FORM                  : i32 = 29;
pub const MSK_IPAR_INTPNT_STARTING_POINT              : i32 = 30;
pub const MSK_IPAR_LICENSE_DEBUG                      : i32 = 31;
pub const MSK_IPAR_LICENSE_PAUSE_TIME                 : i32 = 32;
pub const MSK_IPAR_LICENSE_SUPPRESS_EXPIRE_WRNS       : i32 = 33;
pub const MSK_IPAR_LICENSE_TRH_EXPIRY_WRN             : i32 = 34;
pub const MSK_IPAR_LICENSE_WAIT                       : i32 = 35;
pub const MSK_IPAR_LOG                                : i32 = 36;
pub const MSK_IPAR_LOG_ANA_PRO                        : i32 = 37;
pub const MSK_IPAR_LOG_BI                             : i32 = 38;
pub const MSK_IPAR_LOG_BI_FREQ                        : i32 = 39;
pub const MSK_IPAR_LOG_CHECK_CONVEXITY                : i32 = 40;
pub const MSK_IPAR_LOG_CUT_SECOND_OPT                 : i32 = 41;
pub const MSK_IPAR_LOG_EXPAND                         : i32 = 42;
pub const MSK_IPAR_LOG_FEAS_REPAIR                    : i32 = 43;
pub const MSK_IPAR_LOG_FILE                           : i32 = 44;
pub const MSK_IPAR_LOG_INCLUDE_SUMMARY                : i32 = 45;
pub const MSK_IPAR_LOG_INFEAS_ANA                     : i32 = 46;
pub const MSK_IPAR_LOG_INTPNT                         : i32 = 47;
pub const MSK_IPAR_LOG_LOCAL_INFO                     : i32 = 48;
pub const MSK_IPAR_LOG_MIO                            : i32 = 49;
pub const MSK_IPAR_LOG_MIO_FREQ                       : i32 = 50;
pub const MSK_IPAR_LOG_ORDER                          : i32 = 51;
pub const MSK_IPAR_LOG_PRESOLVE                       : i32 = 52;
pub const MSK_IPAR_LOG_RESPONSE                       : i32 = 53;
pub const MSK_IPAR_LOG_SENSITIVITY                    : i32 = 54;
pub const MSK_IPAR_LOG_SENSITIVITY_OPT                : i32 = 55;
pub const MSK_IPAR_LOG_SIM                            : i32 = 56;
pub const MSK_IPAR_LOG_SIM_FREQ                       : i32 = 57;
pub const MSK_IPAR_LOG_SIM_MINOR                      : i32 = 58;
pub const MSK_IPAR_LOG_STORAGE                        : i32 = 59;
pub const MSK_IPAR_MAX_NUM_WARNINGS                   : i32 = 60;
pub const MSK_IPAR_MIO_BRANCH_DIR                     : i32 = 61;
pub const MSK_IPAR_MIO_CONIC_OUTER_APPROXIMATION      : i32 = 62;
pub const MSK_IPAR_MIO_CUT_CLIQUE                     : i32 = 63;
pub const MSK_IPAR_MIO_CUT_CMIR                       : i32 = 64;
pub const MSK_IPAR_MIO_CUT_GMI                        : i32 = 65;
pub const MSK_IPAR_MIO_CUT_IMPLIED_BOUND              : i32 = 66;
pub const MSK_IPAR_MIO_CUT_KNAPSACK_COVER             : i32 = 67;
pub const MSK_IPAR_MIO_CUT_SELECTION_LEVEL            : i32 = 68;
pub const MSK_IPAR_MIO_FEASPUMP_LEVEL                 : i32 = 69;
pub const MSK_IPAR_MIO_HEURISTIC_LEVEL                : i32 = 70;
pub const MSK_IPAR_MIO_MAX_NUM_BRANCHES               : i32 = 71;
pub const MSK_IPAR_MIO_MAX_NUM_RELAXS                 : i32 = 72;
pub const MSK_IPAR_MIO_MAX_NUM_ROOT_CUT_ROUNDS        : i32 = 73;
pub const MSK_IPAR_MIO_MAX_NUM_SOLUTIONS              : i32 = 74;
pub const MSK_IPAR_MIO_MODE                           : i32 = 75;
pub const MSK_IPAR_MIO_NODE_OPTIMIZER                 : i32 = 76;
pub const MSK_IPAR_MIO_NODE_SELECTION                 : i32 = 77;
pub const MSK_IPAR_MIO_PERSPECTIVE_REFORMULATE        : i32 = 78;
pub const MSK_IPAR_MIO_PROBING_LEVEL                  : i32 = 79;
pub const MSK_IPAR_MIO_PROPAGATE_OBJECTIVE_CONSTRAINT : i32 = 80;
pub const MSK_IPAR_MIO_RINS_MAX_NODES                 : i32 = 81;
pub const MSK_IPAR_MIO_ROOT_OPTIMIZER                 : i32 = 82;
pub const MSK_IPAR_MIO_ROOT_REPEAT_PRESOLVE_LEVEL     : i32 = 83;
pub const MSK_IPAR_MIO_SEED                           : i32 = 84;
pub const MSK_IPAR_MIO_VB_DETECTION_LEVEL             : i32 = 85;
pub const MSK_IPAR_MT_SPINCOUNT                       : i32 = 86;
pub const MSK_IPAR_NG                                 : i32 = 87;
pub const MSK_IPAR_NUM_THREADS                        : i32 = 88;
pub const MSK_IPAR_OPF_WRITE_HEADER                   : i32 = 89;
pub const MSK_IPAR_OPF_WRITE_HINTS                    : i32 = 90;
pub const MSK_IPAR_OPF_WRITE_LINE_LENGTH              : i32 = 91;
pub const MSK_IPAR_OPF_WRITE_PARAMETERS               : i32 = 92;
pub const MSK_IPAR_OPF_WRITE_PROBLEM                  : i32 = 93;
pub const MSK_IPAR_OPF_WRITE_SOL_BAS                  : i32 = 94;
pub const MSK_IPAR_OPF_WRITE_SOL_ITG                  : i32 = 95;
pub const MSK_IPAR_OPF_WRITE_SOL_ITR                  : i32 = 96;
pub const MSK_IPAR_OPF_WRITE_SOLUTIONS                : i32 = 97;
pub const MSK_IPAR_OPTIMIZER                          : i32 = 98;
pub const MSK_IPAR_PARAM_READ_CASE_NAME               : i32 = 99;
pub const MSK_IPAR_PARAM_READ_IGN_ERROR               : i32 = 100;
pub const MSK_IPAR_PRESOLVE_ELIMINATOR_MAX_FILL       : i32 = 101;
pub const MSK_IPAR_PRESOLVE_ELIMINATOR_MAX_NUM_TRIES  : i32 = 102;
pub const MSK_IPAR_PRESOLVE_LEVEL                     : i32 = 103;
pub const MSK_IPAR_PRESOLVE_LINDEP_ABS_WORK_TRH       : i32 = 104;
pub const MSK_IPAR_PRESOLVE_LINDEP_REL_WORK_TRH       : i32 = 105;
pub const MSK_IPAR_PRESOLVE_LINDEP_USE                : i32 = 106;
pub const MSK_IPAR_PRESOLVE_MAX_NUM_PASS              : i32 = 107;
pub const MSK_IPAR_PRESOLVE_MAX_NUM_REDUCTIONS        : i32 = 108;
pub const MSK_IPAR_PRESOLVE_USE                       : i32 = 109;
pub const MSK_IPAR_PRIMAL_REPAIR_OPTIMIZER            : i32 = 110;
pub const MSK_IPAR_PTF_WRITE_TRANSFORM                : i32 = 111;
pub const MSK_IPAR_READ_DEBUG                         : i32 = 112;
pub const MSK_IPAR_READ_KEEP_FREE_CON                 : i32 = 113;
pub const MSK_IPAR_READ_LP_DROP_NEW_VARS_IN_BOU       : i32 = 114;
pub const MSK_IPAR_READ_LP_QUOTED_NAMES               : i32 = 115;
pub const MSK_IPAR_READ_MPS_FORMAT                    : i32 = 116;
pub const MSK_IPAR_READ_MPS_WIDTH                     : i32 = 117;
pub const MSK_IPAR_READ_TASK_IGNORE_PARAM             : i32 = 118;
pub const MSK_IPAR_REMOVE_UNUSED_SOLUTIONS            : i32 = 119;
pub const MSK_IPAR_SENSITIVITY_ALL                    : i32 = 120;
pub const MSK_IPAR_SENSITIVITY_OPTIMIZER              : i32 = 121;
pub const MSK_IPAR_SENSITIVITY_TYPE                   : i32 = 122;
pub const MSK_IPAR_SIM_BASIS_FACTOR_USE               : i32 = 123;
pub const MSK_IPAR_SIM_DEGEN                          : i32 = 124;
pub const MSK_IPAR_SIM_DUAL_CRASH                     : i32 = 125;
pub const MSK_IPAR_SIM_DUAL_PHASEONE_METHOD           : i32 = 126;
pub const MSK_IPAR_SIM_DUAL_RESTRICT_SELECTION        : i32 = 127;
pub const MSK_IPAR_SIM_DUAL_SELECTION                 : i32 = 128;
pub const MSK_IPAR_SIM_EXPLOIT_DUPVEC                 : i32 = 129;
pub const MSK_IPAR_SIM_HOTSTART                       : i32 = 130;
pub const MSK_IPAR_SIM_HOTSTART_LU                    : i32 = 131;
pub const MSK_IPAR_SIM_MAX_ITERATIONS                 : i32 = 132;
pub const MSK_IPAR_SIM_MAX_NUM_SETBACKS               : i32 = 133;
pub const MSK_IPAR_SIM_NON_SINGULAR                   : i32 = 134;
pub const MSK_IPAR_SIM_PRIMAL_CRASH                   : i32 = 135;
pub const MSK_IPAR_SIM_PRIMAL_PHASEONE_METHOD         : i32 = 136;
pub const MSK_IPAR_SIM_PRIMAL_RESTRICT_SELECTION      : i32 = 137;
pub const MSK_IPAR_SIM_PRIMAL_SELECTION               : i32 = 138;
pub const MSK_IPAR_SIM_REFACTOR_FREQ                  : i32 = 139;
pub const MSK_IPAR_SIM_REFORMULATION                  : i32 = 140;
pub const MSK_IPAR_SIM_SAVE_LU                        : i32 = 141;
pub const MSK_IPAR_SIM_SCALING                        : i32 = 142;
pub const MSK_IPAR_SIM_SCALING_METHOD                 : i32 = 143;
pub const MSK_IPAR_SIM_SEED                           : i32 = 144;
pub const MSK_IPAR_SIM_SOLVE_FORM                     : i32 = 145;
pub const MSK_IPAR_SIM_STABILITY_PRIORITY             : i32 = 146;
pub const MSK_IPAR_SIM_SWITCH_OPTIMIZER               : i32 = 147;
pub const MSK_IPAR_SOL_FILTER_KEEP_BASIC              : i32 = 148;
pub const MSK_IPAR_SOL_FILTER_KEEP_RANGED             : i32 = 149;
pub const MSK_IPAR_SOL_READ_NAME_WIDTH                : i32 = 150;
pub const MSK_IPAR_SOL_READ_WIDTH                     : i32 = 151;
pub const MSK_IPAR_SOLUTION_CALLBACK                  : i32 = 152;
pub const MSK_IPAR_TIMING_LEVEL                       : i32 = 153;
pub const MSK_IPAR_WRITE_BAS_CONSTRAINTS              : i32 = 154;
pub const MSK_IPAR_WRITE_BAS_HEAD                     : i32 = 155;
pub const MSK_IPAR_WRITE_BAS_VARIABLES                : i32 = 156;
pub const MSK_IPAR_WRITE_COMPRESSION                  : i32 = 157;
pub const MSK_IPAR_WRITE_DATA_PARAM                   : i32 = 158;
pub const MSK_IPAR_WRITE_FREE_CON                     : i32 = 159;
pub const MSK_IPAR_WRITE_GENERIC_NAMES                : i32 = 160;
pub const MSK_IPAR_WRITE_GENERIC_NAMES_IO             : i32 = 161;
pub const MSK_IPAR_WRITE_IGNORE_INCOMPATIBLE_ITEMS    : i32 = 162;
pub const MSK_IPAR_WRITE_INT_CONSTRAINTS              : i32 = 163;
pub const MSK_IPAR_WRITE_INT_HEAD                     : i32 = 164;
pub const MSK_IPAR_WRITE_INT_VARIABLES                : i32 = 165;
pub const MSK_IPAR_WRITE_LP_FULL_OBJ                  : i32 = 166;
pub const MSK_IPAR_WRITE_LP_LINE_WIDTH                : i32 = 167;
pub const MSK_IPAR_WRITE_LP_QUOTED_NAMES              : i32 = 168;
pub const MSK_IPAR_WRITE_LP_STRICT_FORMAT             : i32 = 169;
pub const MSK_IPAR_WRITE_LP_TERMS_PER_LINE            : i32 = 170;
pub const MSK_IPAR_WRITE_MPS_FORMAT                   : i32 = 171;
pub const MSK_IPAR_WRITE_MPS_INT                      : i32 = 172;
pub const MSK_IPAR_WRITE_PRECISION                    : i32 = 173;
pub const MSK_IPAR_WRITE_SOL_BARVARIABLES             : i32 = 174;
pub const MSK_IPAR_WRITE_SOL_CONSTRAINTS              : i32 = 175;
pub const MSK_IPAR_WRITE_SOL_HEAD                     : i32 = 176;
pub const MSK_IPAR_WRITE_SOL_IGNORE_INVALID_NAMES     : i32 = 177;
pub const MSK_IPAR_WRITE_SOL_VARIABLES                : i32 = 178;
pub const MSK_IPAR_WRITE_TASK_INC_SOL                 : i32 = 179;
pub const MSK_IPAR_WRITE_XML_MODE                     : i32 = 180;
pub const MSK_IPAR_BEGIN : i32 = 0;
pub const MSK_IPAR_END   : i32 = 181;

// liinfitem
pub const MSK_LIINF_BI_CLEAN_DUAL_DEG_ITER   : i32 = 0;
pub const MSK_LIINF_BI_CLEAN_DUAL_ITER       : i32 = 1;
pub const MSK_LIINF_BI_CLEAN_PRIMAL_DEG_ITER : i32 = 2;
pub const MSK_LIINF_BI_CLEAN_PRIMAL_ITER     : i32 = 3;
pub const MSK_LIINF_BI_DUAL_ITER             : i32 = 4;
pub const MSK_LIINF_BI_PRIMAL_ITER           : i32 = 5;
pub const MSK_LIINF_INTPNT_FACTOR_NUM_NZ     : i32 = 6;
pub const MSK_LIINF_MIO_ANZ                  : i32 = 7;
pub const MSK_LIINF_MIO_INTPNT_ITER          : i32 = 8;
pub const MSK_LIINF_MIO_PRESOLVED_ANZ        : i32 = 9;
pub const MSK_LIINF_MIO_SIMPLEX_ITER         : i32 = 10;
pub const MSK_LIINF_RD_NUMACC                : i32 = 11;
pub const MSK_LIINF_RD_NUMANZ                : i32 = 12;
pub const MSK_LIINF_RD_NUMDJC                : i32 = 13;
pub const MSK_LIINF_RD_NUMQNZ                : i32 = 14;
pub const MSK_LIINF_SIMPLEX_ITER             : i32 = 15;
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
pub const MSK_MIO_NODE_SELECTION_PSEUDO : i32 = 3;
pub const MSK_MIO_NODE_SELECTION_BEGIN : i32 = 0;
pub const MSK_MIO_NODE_SELECTION_END   : i32 = 4;

// mpsformat
pub const MSK_MPS_FORMAT_CPLEX   : i32 = 3;
pub const MSK_MPS_FORMAT_FREE    : i32 = 2;
pub const MSK_MPS_FORMAT_RELAXED : i32 = 1;
pub const MSK_MPS_FORMAT_STRICT  : i32 = 0;
pub const MSK_MPS_FORMAT_BEGIN : i32 = 0;
pub const MSK_MPS_FORMAT_END   : i32 = 4;

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
pub const MSK_OPTIMIZER_CONIC          : i32 = 0;
pub const MSK_OPTIMIZER_DUAL_SIMPLEX   : i32 = 1;
pub const MSK_OPTIMIZER_FREE           : i32 = 2;
pub const MSK_OPTIMIZER_FREE_SIMPLEX   : i32 = 3;
pub const MSK_OPTIMIZER_INTPNT         : i32 = 4;
pub const MSK_OPTIMIZER_MIXED_INT      : i32 = 5;
pub const MSK_OPTIMIZER_PRIMAL_SIMPLEX : i32 = 6;
pub const MSK_OPTIMIZER_BEGIN : i32 = 0;
pub const MSK_OPTIMIZER_END   : i32 = 7;

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
pub const MSK_PROBTYPE_CONIC : i32 = 3;
pub const MSK_PROBTYPE_LO    : i32 = 0;
pub const MSK_PROBTYPE_MIXED : i32 = 4;
pub const MSK_PROBTYPE_QCQO  : i32 = 2;
pub const MSK_PROBTYPE_QO    : i32 = 1;
pub const MSK_PROBTYPE_BEGIN : i32 = 0;
pub const MSK_PROBTYPE_END   : i32 = 5;

// prosta
pub const MSK_PRO_STA_DUAL_FEAS                : i32 = 3;
pub const MSK_PRO_STA_DUAL_INFEAS              : i32 = 5;
pub const MSK_PRO_STA_ILL_POSED                : i32 = 7;
pub const MSK_PRO_STA_PRIM_AND_DUAL_FEAS       : i32 = 1;
pub const MSK_PRO_STA_PRIM_AND_DUAL_INFEAS     : i32 = 6;
pub const MSK_PRO_STA_PRIM_FEAS                : i32 = 2;
pub const MSK_PRO_STA_PRIM_INFEAS              : i32 = 4;
pub const MSK_PRO_STA_PRIM_INFEAS_OR_UNBOUNDED : i32 = 8;
pub const MSK_PRO_STA_UNKNOWN                  : i32 = 0;
pub const MSK_PRO_STA_BEGIN : i32 = 0;
pub const MSK_PRO_STA_END   : i32 = 9;

// purify
pub const MSK_PURIFY_AUTO        : i32 = 4;
pub const MSK_PURIFY_DUAL        : i32 = 2;
pub const MSK_PURIFY_NONE        : i32 = 0;
pub const MSK_PURIFY_PRIMAL      : i32 = 1;
pub const MSK_PURIFY_PRIMAL_DUAL : i32 = 3;
pub const MSK_PURIFY_BEGIN : i32 = 0;
pub const MSK_PURIFY_END   : i32 = 5;

// rescode
pub const MSK_RES_ERR_ACC_AFE_DOMAIN_MISMATCH                          : i32 = 20602;
pub const MSK_RES_ERR_ACC_INVALID_ENTRY_INDEX                          : i32 = 20601;
pub const MSK_RES_ERR_ACC_INVALID_INDEX                                : i32 = 20600;
pub const MSK_RES_ERR_AD_INVALID_CODELIST                              : i32 = 3102;
pub const MSK_RES_ERR_AFE_INVALID_INDEX                                : i32 = 20500;
pub const MSK_RES_ERR_API_ARRAY_TOO_SMALL                              : i32 = 3001;
pub const MSK_RES_ERR_API_CB_CONNECT                                   : i32 = 3002;
pub const MSK_RES_ERR_API_FATAL_ERROR                                  : i32 = 3005;
pub const MSK_RES_ERR_API_INTERNAL                                     : i32 = 3999;
pub const MSK_RES_ERR_APPENDING_TOO_BIG_CONE                           : i32 = 1311;
pub const MSK_RES_ERR_ARG_IS_TOO_LARGE                                 : i32 = 1227;
pub const MSK_RES_ERR_ARG_IS_TOO_SMALL                                 : i32 = 1226;
pub const MSK_RES_ERR_ARGUMENT_DIMENSION                               : i32 = 1201;
pub const MSK_RES_ERR_ARGUMENT_IS_TOO_LARGE                            : i32 = 5005;
pub const MSK_RES_ERR_ARGUMENT_LENNEQ                                  : i32 = 1197;
pub const MSK_RES_ERR_ARGUMENT_PERM_ARRAY                              : i32 = 1299;
pub const MSK_RES_ERR_ARGUMENT_TYPE                                    : i32 = 1198;
pub const MSK_RES_ERR_BAR_VAR_DIM                                      : i32 = 3920;
pub const MSK_RES_ERR_BASIS                                            : i32 = 1266;
pub const MSK_RES_ERR_BASIS_FACTOR                                     : i32 = 1610;
pub const MSK_RES_ERR_BASIS_SINGULAR                                   : i32 = 1615;
pub const MSK_RES_ERR_BLANK_NAME                                       : i32 = 1070;
pub const MSK_RES_ERR_CBF_DUPLICATE_ACOORD                             : i32 = 7117;
pub const MSK_RES_ERR_CBF_DUPLICATE_BCOORD                             : i32 = 7116;
pub const MSK_RES_ERR_CBF_DUPLICATE_CON                                : i32 = 7108;
pub const MSK_RES_ERR_CBF_DUPLICATE_INT                                : i32 = 7111;
pub const MSK_RES_ERR_CBF_DUPLICATE_OBJ                                : i32 = 7107;
pub const MSK_RES_ERR_CBF_DUPLICATE_OBJACOORD                          : i32 = 7115;
pub const MSK_RES_ERR_CBF_DUPLICATE_POW_CONES                          : i32 = 7130;
pub const MSK_RES_ERR_CBF_DUPLICATE_POW_STAR_CONES                     : i32 = 7131;
pub const MSK_RES_ERR_CBF_DUPLICATE_PSDCON                             : i32 = 7201;
pub const MSK_RES_ERR_CBF_DUPLICATE_PSDVAR                             : i32 = 7124;
pub const MSK_RES_ERR_CBF_DUPLICATE_VAR                                : i32 = 7110;
pub const MSK_RES_ERR_CBF_INVALID_CON_TYPE                             : i32 = 7113;
pub const MSK_RES_ERR_CBF_INVALID_DIMENSION_OF_CONES                   : i32 = 7141;
pub const MSK_RES_ERR_CBF_INVALID_DIMENSION_OF_PSDCON                  : i32 = 7202;
pub const MSK_RES_ERR_CBF_INVALID_DOMAIN_DIMENSION                     : i32 = 7114;
pub const MSK_RES_ERR_CBF_INVALID_EXP_DIMENSION                        : i32 = 7127;
pub const MSK_RES_ERR_CBF_INVALID_INT_INDEX                            : i32 = 7122;
pub const MSK_RES_ERR_CBF_INVALID_NUM_PSDCON                           : i32 = 7200;
pub const MSK_RES_ERR_CBF_INVALID_NUMBER_OF_CONES                      : i32 = 7140;
pub const MSK_RES_ERR_CBF_INVALID_POWER                                : i32 = 7132;
pub const MSK_RES_ERR_CBF_INVALID_POWER_CONE_INDEX                     : i32 = 7134;
pub const MSK_RES_ERR_CBF_INVALID_POWER_STAR_CONE_INDEX                : i32 = 7135;
pub const MSK_RES_ERR_CBF_INVALID_PSDCON_BLOCK_INDEX                   : i32 = 7205;
pub const MSK_RES_ERR_CBF_INVALID_PSDCON_INDEX                         : i32 = 7203;
pub const MSK_RES_ERR_CBF_INVALID_PSDCON_VARIABLE_INDEX                : i32 = 7204;
pub const MSK_RES_ERR_CBF_INVALID_PSDVAR_DIMENSION                     : i32 = 7125;
pub const MSK_RES_ERR_CBF_INVALID_VAR_TYPE                             : i32 = 7112;
pub const MSK_RES_ERR_CBF_NO_VARIABLES                                 : i32 = 7102;
pub const MSK_RES_ERR_CBF_NO_VERSION_SPECIFIED                         : i32 = 7105;
pub const MSK_RES_ERR_CBF_OBJ_SENSE                                    : i32 = 7101;
pub const MSK_RES_ERR_CBF_PARSE                                        : i32 = 7100;
pub const MSK_RES_ERR_CBF_POWER_CONE_IS_TOO_LONG                       : i32 = 7133;
pub const MSK_RES_ERR_CBF_POWER_CONE_MISMATCH                          : i32 = 7138;
pub const MSK_RES_ERR_CBF_POWER_STAR_CONE_MISMATCH                     : i32 = 7139;
pub const MSK_RES_ERR_CBF_SYNTAX                                       : i32 = 7106;
pub const MSK_RES_ERR_CBF_TOO_FEW_CONSTRAINTS                          : i32 = 7119;
pub const MSK_RES_ERR_CBF_TOO_FEW_INTS                                 : i32 = 7120;
pub const MSK_RES_ERR_CBF_TOO_FEW_PSDVAR                               : i32 = 7126;
pub const MSK_RES_ERR_CBF_TOO_FEW_VARIABLES                            : i32 = 7118;
pub const MSK_RES_ERR_CBF_TOO_MANY_CONSTRAINTS                         : i32 = 7103;
pub const MSK_RES_ERR_CBF_TOO_MANY_INTS                                : i32 = 7121;
pub const MSK_RES_ERR_CBF_TOO_MANY_VARIABLES                           : i32 = 7104;
pub const MSK_RES_ERR_CBF_UNHANDLED_POWER_CONE_TYPE                    : i32 = 7136;
pub const MSK_RES_ERR_CBF_UNHANDLED_POWER_STAR_CONE_TYPE               : i32 = 7137;
pub const MSK_RES_ERR_CBF_UNSUPPORTED                                  : i32 = 7123;
pub const MSK_RES_ERR_CON_Q_NOT_NSD                                    : i32 = 1294;
pub const MSK_RES_ERR_CON_Q_NOT_PSD                                    : i32 = 1293;
pub const MSK_RES_ERR_CONE_INDEX                                       : i32 = 1300;
pub const MSK_RES_ERR_CONE_OVERLAP                                     : i32 = 1302;
pub const MSK_RES_ERR_CONE_OVERLAP_APPEND                              : i32 = 1307;
pub const MSK_RES_ERR_CONE_PARAMETER                                   : i32 = 1320;
pub const MSK_RES_ERR_CONE_REP_VAR                                     : i32 = 1303;
pub const MSK_RES_ERR_CONE_SIZE                                        : i32 = 1301;
pub const MSK_RES_ERR_CONE_TYPE                                        : i32 = 1305;
pub const MSK_RES_ERR_CONE_TYPE_STR                                    : i32 = 1306;
pub const MSK_RES_ERR_DATA_FILE_EXT                                    : i32 = 1055;
pub const MSK_RES_ERR_DJC_AFE_DOMAIN_MISMATCH                          : i32 = 20702;
pub const MSK_RES_ERR_DJC_DOMAIN_TERMSIZE_MISMATCH                     : i32 = 20704;
pub const MSK_RES_ERR_DJC_INVALID_INDEX                                : i32 = 20700;
pub const MSK_RES_ERR_DJC_INVALID_TERM_SIZE                            : i32 = 20703;
pub const MSK_RES_ERR_DJC_TOTAL_NUM_TERMS_MISMATCH                     : i32 = 20705;
pub const MSK_RES_ERR_DJC_UNSUPPORTED_DOMAIN_TYPE                      : i32 = 20701;
pub const MSK_RES_ERR_DOMAIN_DIMENSION                                 : i32 = 20401;
pub const MSK_RES_ERR_DOMAIN_DIMENSION_PSD                             : i32 = 20402;
pub const MSK_RES_ERR_DOMAIN_INVALID_INDEX                             : i32 = 20400;
pub const MSK_RES_ERR_DOMAIN_POWER_INVALID_ALPHA                       : i32 = 20404;
pub const MSK_RES_ERR_DOMAIN_POWER_NEGATIVE_ALPHA                      : i32 = 20405;
pub const MSK_RES_ERR_DOMAIN_POWER_NLEFT                               : i32 = 20406;
pub const MSK_RES_ERR_DUP_NAME                                         : i32 = 1071;
pub const MSK_RES_ERR_DUPLICATE_AIJ                                    : i32 = 1385;
pub const MSK_RES_ERR_DUPLICATE_BARVARIABLE_NAMES                      : i32 = 4502;
pub const MSK_RES_ERR_DUPLICATE_CONE_NAMES                             : i32 = 4503;
pub const MSK_RES_ERR_DUPLICATE_CONSTRAINT_NAMES                       : i32 = 4500;
pub const MSK_RES_ERR_DUPLICATE_DJC_NAMES                              : i32 = 4505;
pub const MSK_RES_ERR_DUPLICATE_DOMAIN_NAMES                           : i32 = 4504;
pub const MSK_RES_ERR_DUPLICATE_FIJ                                    : i32 = 20100;
pub const MSK_RES_ERR_DUPLICATE_VARIABLE_NAMES                         : i32 = 4501;
pub const MSK_RES_ERR_END_OF_FILE                                      : i32 = 1059;
pub const MSK_RES_ERR_FACTOR                                           : i32 = 1650;
pub const MSK_RES_ERR_FEASREPAIR_CANNOT_RELAX                          : i32 = 1700;
pub const MSK_RES_ERR_FEASREPAIR_INCONSISTENT_BOUND                    : i32 = 1702;
pub const MSK_RES_ERR_FEASREPAIR_SOLVING_RELAXED                       : i32 = 1701;
pub const MSK_RES_ERR_FILE_LICENSE                                     : i32 = 1007;
pub const MSK_RES_ERR_FILE_OPEN                                        : i32 = 1052;
pub const MSK_RES_ERR_FILE_READ                                        : i32 = 1053;
pub const MSK_RES_ERR_FILE_WRITE                                       : i32 = 1054;
pub const MSK_RES_ERR_FINAL_SOLUTION                                   : i32 = 1560;
pub const MSK_RES_ERR_FIRST                                            : i32 = 1570;
pub const MSK_RES_ERR_FIRSTI                                           : i32 = 1285;
pub const MSK_RES_ERR_FIRSTJ                                           : i32 = 1287;
pub const MSK_RES_ERR_FIXED_BOUND_VALUES                               : i32 = 1420;
pub const MSK_RES_ERR_FLEXLM                                           : i32 = 1014;
pub const MSK_RES_ERR_FORMAT_STRING                                    : i32 = 1072;
pub const MSK_RES_ERR_GLOBAL_INV_CONIC_PROBLEM                         : i32 = 1503;
pub const MSK_RES_ERR_HUGE_AIJ                                         : i32 = 1380;
pub const MSK_RES_ERR_HUGE_C                                           : i32 = 1375;
pub const MSK_RES_ERR_HUGE_FIJ                                         : i32 = 20102;
pub const MSK_RES_ERR_IDENTICAL_TASKS                                  : i32 = 3101;
pub const MSK_RES_ERR_IN_ARGUMENT                                      : i32 = 1200;
pub const MSK_RES_ERR_INDEX                                            : i32 = 1235;
pub const MSK_RES_ERR_INDEX_ARR_IS_TOO_LARGE                           : i32 = 1222;
pub const MSK_RES_ERR_INDEX_ARR_IS_TOO_SMALL                           : i32 = 1221;
pub const MSK_RES_ERR_INDEX_IS_NOT_UNIQUE                              : i32 = 1205;
pub const MSK_RES_ERR_INDEX_IS_TOO_LARGE                               : i32 = 1204;
pub const MSK_RES_ERR_INDEX_IS_TOO_SMALL                               : i32 = 1203;
pub const MSK_RES_ERR_INF_DOU_INDEX                                    : i32 = 1219;
pub const MSK_RES_ERR_INF_DOU_NAME                                     : i32 = 1230;
pub const MSK_RES_ERR_INF_INT_INDEX                                    : i32 = 1220;
pub const MSK_RES_ERR_INF_INT_NAME                                     : i32 = 1231;
pub const MSK_RES_ERR_INF_LINT_INDEX                                   : i32 = 1225;
pub const MSK_RES_ERR_INF_LINT_NAME                                    : i32 = 1234;
pub const MSK_RES_ERR_INF_TYPE                                         : i32 = 1232;
pub const MSK_RES_ERR_INFEAS_UNDEFINED                                 : i32 = 3910;
pub const MSK_RES_ERR_INFINITE_BOUND                                   : i32 = 1400;
pub const MSK_RES_ERR_INT64_TO_INT32_CAST                              : i32 = 3800;
pub const MSK_RES_ERR_INTERNAL                                         : i32 = 3000;
pub const MSK_RES_ERR_INTERNAL_TEST_FAILED                             : i32 = 3500;
pub const MSK_RES_ERR_INV_APTRE                                        : i32 = 1253;
pub const MSK_RES_ERR_INV_BK                                           : i32 = 1255;
pub const MSK_RES_ERR_INV_BKC                                          : i32 = 1256;
pub const MSK_RES_ERR_INV_BKX                                          : i32 = 1257;
pub const MSK_RES_ERR_INV_CONE_TYPE                                    : i32 = 1272;
pub const MSK_RES_ERR_INV_CONE_TYPE_STR                                : i32 = 1271;
pub const MSK_RES_ERR_INV_MARKI                                        : i32 = 2501;
pub const MSK_RES_ERR_INV_MARKJ                                        : i32 = 2502;
pub const MSK_RES_ERR_INV_NAME_ITEM                                    : i32 = 1280;
pub const MSK_RES_ERR_INV_NUMI                                         : i32 = 2503;
pub const MSK_RES_ERR_INV_NUMJ                                         : i32 = 2504;
pub const MSK_RES_ERR_INV_OPTIMIZER                                    : i32 = 1550;
pub const MSK_RES_ERR_INV_PROBLEM                                      : i32 = 1500;
pub const MSK_RES_ERR_INV_QCON_SUBI                                    : i32 = 1405;
pub const MSK_RES_ERR_INV_QCON_SUBJ                                    : i32 = 1406;
pub const MSK_RES_ERR_INV_QCON_SUBK                                    : i32 = 1404;
pub const MSK_RES_ERR_INV_QCON_VAL                                     : i32 = 1407;
pub const MSK_RES_ERR_INV_QOBJ_SUBI                                    : i32 = 1401;
pub const MSK_RES_ERR_INV_QOBJ_SUBJ                                    : i32 = 1402;
pub const MSK_RES_ERR_INV_QOBJ_VAL                                     : i32 = 1403;
pub const MSK_RES_ERR_INV_SK                                           : i32 = 1270;
pub const MSK_RES_ERR_INV_SK_STR                                       : i32 = 1269;
pub const MSK_RES_ERR_INV_SKC                                          : i32 = 1267;
pub const MSK_RES_ERR_INV_SKN                                          : i32 = 1274;
pub const MSK_RES_ERR_INV_SKX                                          : i32 = 1268;
pub const MSK_RES_ERR_INV_VAR_TYPE                                     : i32 = 1258;
pub const MSK_RES_ERR_INVALID_AIJ                                      : i32 = 1473;
pub const MSK_RES_ERR_INVALID_AMPL_STUB                                : i32 = 3700;
pub const MSK_RES_ERR_INVALID_B                                        : i32 = 20150;
pub const MSK_RES_ERR_INVALID_BARVAR_NAME                              : i32 = 1079;
pub const MSK_RES_ERR_INVALID_COMPRESSION                              : i32 = 1800;
pub const MSK_RES_ERR_INVALID_CON_NAME                                 : i32 = 1076;
pub const MSK_RES_ERR_INVALID_CONE_NAME                                : i32 = 1078;
pub const MSK_RES_ERR_INVALID_FIJ                                      : i32 = 20101;
pub const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_AFFINE_CONIC_CONSTRAINTS : i32 = 4012;
pub const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_CFIX                     : i32 = 4001;
pub const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_CONES                    : i32 = 4005;
pub const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_DISJUNCTIVE_CONSTRAINTS  : i32 = 4011;
pub const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_FREE_CONSTRAINTS         : i32 = 4003;
pub const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_NONLINEAR                : i32 = 4010;
pub const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_RANGED_CONSTRAINTS       : i32 = 4002;
pub const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_SYM_MAT                  : i32 = 4000;
pub const MSK_RES_ERR_INVALID_FILE_NAME                                : i32 = 1056;
pub const MSK_RES_ERR_INVALID_FORMAT_TYPE                              : i32 = 1283;
pub const MSK_RES_ERR_INVALID_G                                        : i32 = 20103;
pub const MSK_RES_ERR_INVALID_IDX                                      : i32 = 1246;
pub const MSK_RES_ERR_INVALID_IOMODE                                   : i32 = 1801;
pub const MSK_RES_ERR_INVALID_MAX_NUM                                  : i32 = 1247;
pub const MSK_RES_ERR_INVALID_NAME_IN_SOL_FILE                         : i32 = 1170;
pub const MSK_RES_ERR_INVALID_OBJ_NAME                                 : i32 = 1075;
pub const MSK_RES_ERR_INVALID_OBJECTIVE_SENSE                          : i32 = 1445;
pub const MSK_RES_ERR_INVALID_PROBLEM_TYPE                             : i32 = 6000;
pub const MSK_RES_ERR_INVALID_SOL_FILE_NAME                            : i32 = 1057;
pub const MSK_RES_ERR_INVALID_STREAM                                   : i32 = 1062;
pub const MSK_RES_ERR_INVALID_SURPLUS                                  : i32 = 1275;
pub const MSK_RES_ERR_INVALID_SYM_MAT_DIM                              : i32 = 3950;
pub const MSK_RES_ERR_INVALID_TASK                                     : i32 = 1064;
pub const MSK_RES_ERR_INVALID_UTF8                                     : i32 = 2900;
pub const MSK_RES_ERR_INVALID_VAR_NAME                                 : i32 = 1077;
pub const MSK_RES_ERR_INVALID_WCHAR                                    : i32 = 2901;
pub const MSK_RES_ERR_INVALID_WHICHSOL                                 : i32 = 1228;
pub const MSK_RES_ERR_JSON_DATA                                        : i32 = 1179;
pub const MSK_RES_ERR_JSON_FORMAT                                      : i32 = 1178;
pub const MSK_RES_ERR_JSON_MISSING_DATA                                : i32 = 1180;
pub const MSK_RES_ERR_JSON_NUMBER_OVERFLOW                             : i32 = 1177;
pub const MSK_RES_ERR_JSON_STRING                                      : i32 = 1176;
pub const MSK_RES_ERR_JSON_SYNTAX                                      : i32 = 1175;
pub const MSK_RES_ERR_LAST                                             : i32 = 1571;
pub const MSK_RES_ERR_LASTI                                            : i32 = 1286;
pub const MSK_RES_ERR_LASTJ                                            : i32 = 1288;
pub const MSK_RES_ERR_LAU_ARG_K                                        : i32 = 7012;
pub const MSK_RES_ERR_LAU_ARG_M                                        : i32 = 7010;
pub const MSK_RES_ERR_LAU_ARG_N                                        : i32 = 7011;
pub const MSK_RES_ERR_LAU_ARG_TRANS                                    : i32 = 7018;
pub const MSK_RES_ERR_LAU_ARG_TRANSA                                   : i32 = 7015;
pub const MSK_RES_ERR_LAU_ARG_TRANSB                                   : i32 = 7016;
pub const MSK_RES_ERR_LAU_ARG_UPLO                                     : i32 = 7017;
pub const MSK_RES_ERR_LAU_INVALID_LOWER_TRIANGULAR_MATRIX              : i32 = 7002;
pub const MSK_RES_ERR_LAU_INVALID_SPARSE_SYMMETRIC_MATRIX              : i32 = 7019;
pub const MSK_RES_ERR_LAU_NOT_POSITIVE_DEFINITE                        : i32 = 7001;
pub const MSK_RES_ERR_LAU_SINGULAR_MATRIX                              : i32 = 7000;
pub const MSK_RES_ERR_LAU_UNKNOWN                                      : i32 = 7005;
pub const MSK_RES_ERR_LICENSE                                          : i32 = 1000;
pub const MSK_RES_ERR_LICENSE_CANNOT_ALLOCATE                          : i32 = 1020;
pub const MSK_RES_ERR_LICENSE_CANNOT_CONNECT                           : i32 = 1021;
pub const MSK_RES_ERR_LICENSE_EXPIRED                                  : i32 = 1001;
pub const MSK_RES_ERR_LICENSE_FEATURE                                  : i32 = 1018;
pub const MSK_RES_ERR_LICENSE_INVALID_HOSTID                           : i32 = 1025;
pub const MSK_RES_ERR_LICENSE_MAX                                      : i32 = 1016;
pub const MSK_RES_ERR_LICENSE_MOSEKLM_DAEMON                           : i32 = 1017;
pub const MSK_RES_ERR_LICENSE_NO_SERVER_LINE                           : i32 = 1028;
pub const MSK_RES_ERR_LICENSE_NO_SERVER_SUPPORT                        : i32 = 1027;
pub const MSK_RES_ERR_LICENSE_SERVER                                   : i32 = 1015;
pub const MSK_RES_ERR_LICENSE_SERVER_VERSION                           : i32 = 1026;
pub const MSK_RES_ERR_LICENSE_VERSION                                  : i32 = 1002;
pub const MSK_RES_ERR_LINK_FILE_DLL                                    : i32 = 1040;
pub const MSK_RES_ERR_LIVING_TASKS                                     : i32 = 1066;
pub const MSK_RES_ERR_LOWER_BOUND_IS_A_NAN                             : i32 = 1390;
pub const MSK_RES_ERR_LP_DUP_SLACK_NAME                                : i32 = 1152;
pub const MSK_RES_ERR_LP_EMPTY                                         : i32 = 1151;
pub const MSK_RES_ERR_LP_FILE_FORMAT                                   : i32 = 1157;
pub const MSK_RES_ERR_LP_FORMAT                                        : i32 = 1160;
pub const MSK_RES_ERR_LP_FREE_CONSTRAINT                               : i32 = 1155;
pub const MSK_RES_ERR_LP_INCOMPATIBLE                                  : i32 = 1150;
pub const MSK_RES_ERR_LP_INVALID_CON_NAME                              : i32 = 1171;
pub const MSK_RES_ERR_LP_INVALID_VAR_NAME                              : i32 = 1154;
pub const MSK_RES_ERR_LP_WRITE_CONIC_PROBLEM                           : i32 = 1163;
pub const MSK_RES_ERR_LP_WRITE_GECO_PROBLEM                            : i32 = 1164;
pub const MSK_RES_ERR_LU_MAX_NUM_TRIES                                 : i32 = 2800;
pub const MSK_RES_ERR_MAX_LEN_IS_TOO_SMALL                             : i32 = 1289;
pub const MSK_RES_ERR_MAXNUMBARVAR                                     : i32 = 1242;
pub const MSK_RES_ERR_MAXNUMCON                                        : i32 = 1240;
pub const MSK_RES_ERR_MAXNUMCONE                                       : i32 = 1304;
pub const MSK_RES_ERR_MAXNUMQNZ                                        : i32 = 1243;
pub const MSK_RES_ERR_MAXNUMVAR                                        : i32 = 1241;
pub const MSK_RES_ERR_MIO_INTERNAL                                     : i32 = 5010;
pub const MSK_RES_ERR_MIO_INVALID_NODE_OPTIMIZER                       : i32 = 7701;
pub const MSK_RES_ERR_MIO_INVALID_ROOT_OPTIMIZER                       : i32 = 7700;
pub const MSK_RES_ERR_MIO_NO_OPTIMIZER                                 : i32 = 1551;
pub const MSK_RES_ERR_MISSING_LICENSE_FILE                             : i32 = 1008;
pub const MSK_RES_ERR_MIXED_CONIC_AND_NL                               : i32 = 1501;
pub const MSK_RES_ERR_MPS_CONE_OVERLAP                                 : i32 = 1118;
pub const MSK_RES_ERR_MPS_CONE_REPEAT                                  : i32 = 1119;
pub const MSK_RES_ERR_MPS_CONE_TYPE                                    : i32 = 1117;
pub const MSK_RES_ERR_MPS_DUPLICATE_Q_ELEMENT                          : i32 = 1121;
pub const MSK_RES_ERR_MPS_FILE                                         : i32 = 1100;
pub const MSK_RES_ERR_MPS_INV_FIELD                                    : i32 = 1101;
pub const MSK_RES_ERR_MPS_INV_MARKER                                   : i32 = 1102;
pub const MSK_RES_ERR_MPS_INV_SEC_ORDER                                : i32 = 1115;
pub const MSK_RES_ERR_MPS_INVALID_BOUND_KEY                            : i32 = 1108;
pub const MSK_RES_ERR_MPS_INVALID_CON_KEY                              : i32 = 1107;
pub const MSK_RES_ERR_MPS_INVALID_INDICATOR_CONSTRAINT                 : i32 = 1130;
pub const MSK_RES_ERR_MPS_INVALID_INDICATOR_QUADRATIC_CONSTRAINT       : i32 = 1133;
pub const MSK_RES_ERR_MPS_INVALID_INDICATOR_VALUE                      : i32 = 1132;
pub const MSK_RES_ERR_MPS_INVALID_INDICATOR_VARIABLE                   : i32 = 1131;
pub const MSK_RES_ERR_MPS_INVALID_KEY                                  : i32 = 1129;
pub const MSK_RES_ERR_MPS_INVALID_OBJ_NAME                             : i32 = 1128;
pub const MSK_RES_ERR_MPS_INVALID_OBJSENSE                             : i32 = 1122;
pub const MSK_RES_ERR_MPS_INVALID_SEC_NAME                             : i32 = 1109;
pub const MSK_RES_ERR_MPS_MUL_CON_NAME                                 : i32 = 1112;
pub const MSK_RES_ERR_MPS_MUL_CSEC                                     : i32 = 1116;
pub const MSK_RES_ERR_MPS_MUL_QOBJ                                     : i32 = 1114;
pub const MSK_RES_ERR_MPS_MUL_QSEC                                     : i32 = 1113;
pub const MSK_RES_ERR_MPS_NO_OBJECTIVE                                 : i32 = 1110;
pub const MSK_RES_ERR_MPS_NON_SYMMETRIC_Q                              : i32 = 1120;
pub const MSK_RES_ERR_MPS_NULL_CON_NAME                                : i32 = 1103;
pub const MSK_RES_ERR_MPS_NULL_VAR_NAME                                : i32 = 1104;
pub const MSK_RES_ERR_MPS_SPLITTED_VAR                                 : i32 = 1111;
pub const MSK_RES_ERR_MPS_TAB_IN_FIELD2                                : i32 = 1125;
pub const MSK_RES_ERR_MPS_TAB_IN_FIELD3                                : i32 = 1126;
pub const MSK_RES_ERR_MPS_TAB_IN_FIELD5                                : i32 = 1127;
pub const MSK_RES_ERR_MPS_UNDEF_CON_NAME                               : i32 = 1105;
pub const MSK_RES_ERR_MPS_UNDEF_VAR_NAME                               : i32 = 1106;
pub const MSK_RES_ERR_MPS_WRITE_CPLEX_INVALID_CONE_TYPE                : i32 = 7750;
pub const MSK_RES_ERR_MUL_A_ELEMENT                                    : i32 = 1254;
pub const MSK_RES_ERR_NAME_IS_NULL                                     : i32 = 1760;
pub const MSK_RES_ERR_NAME_MAX_LEN                                     : i32 = 1750;
pub const MSK_RES_ERR_NAN_IN_BLC                                       : i32 = 1461;
pub const MSK_RES_ERR_NAN_IN_BLX                                       : i32 = 1471;
pub const MSK_RES_ERR_NAN_IN_BUC                                       : i32 = 1462;
pub const MSK_RES_ERR_NAN_IN_BUX                                       : i32 = 1472;
pub const MSK_RES_ERR_NAN_IN_C                                         : i32 = 1470;
pub const MSK_RES_ERR_NAN_IN_DOUBLE_DATA                               : i32 = 1450;
pub const MSK_RES_ERR_NEGATIVE_APPEND                                  : i32 = 1578;
pub const MSK_RES_ERR_NEGATIVE_SURPLUS                                 : i32 = 1573;
pub const MSK_RES_ERR_NEWER_DLL                                        : i32 = 1036;
pub const MSK_RES_ERR_NO_BARS_FOR_SOLUTION                             : i32 = 3916;
pub const MSK_RES_ERR_NO_BARX_FOR_SOLUTION                             : i32 = 3915;
pub const MSK_RES_ERR_NO_BASIS_SOL                                     : i32 = 1600;
pub const MSK_RES_ERR_NO_DOTY                                          : i32 = 22010;
pub const MSK_RES_ERR_NO_DUAL_FOR_ITG_SOL                              : i32 = 2950;
pub const MSK_RES_ERR_NO_DUAL_INFEAS_CER                               : i32 = 2001;
pub const MSK_RES_ERR_NO_INIT_ENV                                      : i32 = 1063;
pub const MSK_RES_ERR_NO_OPTIMIZER_VAR_TYPE                            : i32 = 1552;
pub const MSK_RES_ERR_NO_PRIMAL_INFEAS_CER                             : i32 = 2000;
pub const MSK_RES_ERR_NO_SNX_FOR_BAS_SOL                               : i32 = 2953;
pub const MSK_RES_ERR_NO_SOLUTION_IN_CALLBACK                          : i32 = 2500;
pub const MSK_RES_ERR_NON_UNIQUE_ARRAY                                 : i32 = 5000;
pub const MSK_RES_ERR_NONCONVEX                                        : i32 = 1291;
pub const MSK_RES_ERR_NONLINEAR_EQUALITY                               : i32 = 1290;
pub const MSK_RES_ERR_NONLINEAR_RANGED                                 : i32 = 1292;
pub const MSK_RES_ERR_NOT_POWER_DOMAIN                                 : i32 = 20403;
pub const MSK_RES_ERR_NULL_ENV                                         : i32 = 1060;
pub const MSK_RES_ERR_NULL_POINTER                                     : i32 = 1065;
pub const MSK_RES_ERR_NULL_TASK                                        : i32 = 1061;
pub const MSK_RES_ERR_NUM_ARGUMENTS                                    : i32 = 1199;
pub const MSK_RES_ERR_NUMCONLIM                                        : i32 = 1250;
pub const MSK_RES_ERR_NUMVARLIM                                        : i32 = 1251;
pub const MSK_RES_ERR_OBJ_Q_NOT_NSD                                    : i32 = 1296;
pub const MSK_RES_ERR_OBJ_Q_NOT_PSD                                    : i32 = 1295;
pub const MSK_RES_ERR_OBJECTIVE_RANGE                                  : i32 = 1260;
pub const MSK_RES_ERR_OLDER_DLL                                        : i32 = 1035;
pub const MSK_RES_ERR_OPF_FORMAT                                       : i32 = 1168;
pub const MSK_RES_ERR_OPF_NEW_VARIABLE                                 : i32 = 1169;
pub const MSK_RES_ERR_OPF_PREMATURE_EOF                                : i32 = 1172;
pub const MSK_RES_ERR_OPTIMIZER_LICENSE                                : i32 = 1013;
pub const MSK_RES_ERR_OVERFLOW                                         : i32 = 1590;
pub const MSK_RES_ERR_PARAM_INDEX                                      : i32 = 1210;
pub const MSK_RES_ERR_PARAM_IS_TOO_LARGE                               : i32 = 1215;
pub const MSK_RES_ERR_PARAM_IS_TOO_SMALL                               : i32 = 1216;
pub const MSK_RES_ERR_PARAM_NAME                                       : i32 = 1206;
pub const MSK_RES_ERR_PARAM_NAME_DOU                                   : i32 = 1207;
pub const MSK_RES_ERR_PARAM_NAME_INT                                   : i32 = 1208;
pub const MSK_RES_ERR_PARAM_NAME_STR                                   : i32 = 1209;
pub const MSK_RES_ERR_PARAM_TYPE                                       : i32 = 1218;
pub const MSK_RES_ERR_PARAM_VALUE_STR                                  : i32 = 1217;
pub const MSK_RES_ERR_PLATFORM_NOT_LICENSED                            : i32 = 1019;
pub const MSK_RES_ERR_POSTSOLVE                                        : i32 = 1580;
pub const MSK_RES_ERR_PRO_ITEM                                         : i32 = 1281;
pub const MSK_RES_ERR_PROB_LICENSE                                     : i32 = 1006;
pub const MSK_RES_ERR_PTF_FORMAT                                       : i32 = 1167;
pub const MSK_RES_ERR_PTF_INCOMPATIBILITY                              : i32 = 1181;
pub const MSK_RES_ERR_PTF_INCONSISTENCY                                : i32 = 1174;
pub const MSK_RES_ERR_PTF_UNDEFINED_ITEM                               : i32 = 1173;
pub const MSK_RES_ERR_QCON_SUBI_TOO_LARGE                              : i32 = 1409;
pub const MSK_RES_ERR_QCON_SUBI_TOO_SMALL                              : i32 = 1408;
pub const MSK_RES_ERR_QCON_UPPER_TRIANGLE                              : i32 = 1417;
pub const MSK_RES_ERR_QOBJ_UPPER_TRIANGLE                              : i32 = 1415;
pub const MSK_RES_ERR_READ_FORMAT                                      : i32 = 1090;
pub const MSK_RES_ERR_READ_LP_MISSING_END_TAG                          : i32 = 1159;
pub const MSK_RES_ERR_READ_LP_NONEXISTING_NAME                         : i32 = 1162;
pub const MSK_RES_ERR_REMOVE_CONE_VARIABLE                             : i32 = 1310;
pub const MSK_RES_ERR_REPAIR_INVALID_PROBLEM                           : i32 = 1710;
pub const MSK_RES_ERR_REPAIR_OPTIMIZATION_FAILED                       : i32 = 1711;
pub const MSK_RES_ERR_SEN_BOUND_INVALID_LO                             : i32 = 3054;
pub const MSK_RES_ERR_SEN_BOUND_INVALID_UP                             : i32 = 3053;
pub const MSK_RES_ERR_SEN_FORMAT                                       : i32 = 3050;
pub const MSK_RES_ERR_SEN_INDEX_INVALID                                : i32 = 3055;
pub const MSK_RES_ERR_SEN_INDEX_RANGE                                  : i32 = 3052;
pub const MSK_RES_ERR_SEN_INVALID_REGEXP                               : i32 = 3056;
pub const MSK_RES_ERR_SEN_NUMERICAL                                    : i32 = 3058;
pub const MSK_RES_ERR_SEN_SOLUTION_STATUS                              : i32 = 3057;
pub const MSK_RES_ERR_SEN_UNDEF_NAME                                   : i32 = 3051;
pub const MSK_RES_ERR_SEN_UNHANDLED_PROBLEM_TYPE                       : i32 = 3080;
pub const MSK_RES_ERR_SERVER_ACCESS_TOKEN                              : i32 = 8007;
pub const MSK_RES_ERR_SERVER_ADDRESS                                   : i32 = 8004;
pub const MSK_RES_ERR_SERVER_CERTIFICATE                               : i32 = 8005;
pub const MSK_RES_ERR_SERVER_CONNECT                                   : i32 = 8000;
pub const MSK_RES_ERR_SERVER_PROBLEM_SIZE                              : i32 = 8008;
pub const MSK_RES_ERR_SERVER_PROTOCOL                                  : i32 = 8001;
pub const MSK_RES_ERR_SERVER_STATUS                                    : i32 = 8002;
pub const MSK_RES_ERR_SERVER_TLS_CLIENT                                : i32 = 8006;
pub const MSK_RES_ERR_SERVER_TOKEN                                     : i32 = 8003;
pub const MSK_RES_ERR_SHAPE_IS_TOO_LARGE                               : i32 = 1202;
pub const MSK_RES_ERR_SIZE_LICENSE                                     : i32 = 1005;
pub const MSK_RES_ERR_SIZE_LICENSE_CON                                 : i32 = 1010;
pub const MSK_RES_ERR_SIZE_LICENSE_INTVAR                              : i32 = 1012;
pub const MSK_RES_ERR_SIZE_LICENSE_NUMCORES                            : i32 = 3900;
pub const MSK_RES_ERR_SIZE_LICENSE_VAR                                 : i32 = 1011;
pub const MSK_RES_ERR_SLICE_SIZE                                       : i32 = 1572;
pub const MSK_RES_ERR_SOL_FILE_INVALID_NUMBER                          : i32 = 1350;
pub const MSK_RES_ERR_SOLITEM                                          : i32 = 1237;
pub const MSK_RES_ERR_SOLVER_PROBTYPE                                  : i32 = 1259;
pub const MSK_RES_ERR_SPACE                                            : i32 = 1051;
pub const MSK_RES_ERR_SPACE_LEAKING                                    : i32 = 1080;
pub const MSK_RES_ERR_SPACE_NO_INFO                                    : i32 = 1081;
pub const MSK_RES_ERR_SYM_MAT_DUPLICATE                                : i32 = 3944;
pub const MSK_RES_ERR_SYM_MAT_HUGE                                     : i32 = 1482;
pub const MSK_RES_ERR_SYM_MAT_INVALID                                  : i32 = 1480;
pub const MSK_RES_ERR_SYM_MAT_INVALID_COL_INDEX                        : i32 = 3941;
pub const MSK_RES_ERR_SYM_MAT_INVALID_ROW_INDEX                        : i32 = 3940;
pub const MSK_RES_ERR_SYM_MAT_INVALID_VALUE                            : i32 = 3943;
pub const MSK_RES_ERR_SYM_MAT_NOT_LOWER_TRINGULAR                      : i32 = 3942;
pub const MSK_RES_ERR_TASK_INCOMPATIBLE                                : i32 = 2560;
pub const MSK_RES_ERR_TASK_INVALID                                     : i32 = 2561;
pub const MSK_RES_ERR_TASK_WRITE                                       : i32 = 2562;
pub const MSK_RES_ERR_THREAD_COND_INIT                                 : i32 = 1049;
pub const MSK_RES_ERR_THREAD_CREATE                                    : i32 = 1048;
pub const MSK_RES_ERR_THREAD_MUTEX_INIT                                : i32 = 1045;
pub const MSK_RES_ERR_THREAD_MUTEX_LOCK                                : i32 = 1046;
pub const MSK_RES_ERR_THREAD_MUTEX_UNLOCK                              : i32 = 1047;
pub const MSK_RES_ERR_TOCONIC_CONSTR_NOT_CONIC                         : i32 = 7803;
pub const MSK_RES_ERR_TOCONIC_CONSTR_Q_NOT_PSD                         : i32 = 7800;
pub const MSK_RES_ERR_TOCONIC_CONSTRAINT_FX                            : i32 = 7801;
pub const MSK_RES_ERR_TOCONIC_CONSTRAINT_RA                            : i32 = 7802;
pub const MSK_RES_ERR_TOCONIC_OBJECTIVE_NOT_PSD                        : i32 = 7804;
pub const MSK_RES_ERR_TOO_SMALL_A_TRUNCATION_VALUE                     : i32 = 1421;
pub const MSK_RES_ERR_TOO_SMALL_MAX_NUM_NZ                             : i32 = 1245;
pub const MSK_RES_ERR_TOO_SMALL_MAXNUMANZ                              : i32 = 1252;
pub const MSK_RES_ERR_UNALLOWED_WHICHSOL                               : i32 = 1248;
pub const MSK_RES_ERR_UNB_STEP_SIZE                                    : i32 = 3100;
pub const MSK_RES_ERR_UNDEF_SOLUTION                                   : i32 = 22000;
pub const MSK_RES_ERR_UNDEFINED_OBJECTIVE_SENSE                        : i32 = 1446;
pub const MSK_RES_ERR_UNHANDLED_SOLUTION_STATUS                        : i32 = 6010;
pub const MSK_RES_ERR_UNKNOWN                                          : i32 = 1050;
pub const MSK_RES_ERR_UPPER_BOUND_IS_A_NAN                             : i32 = 1391;
pub const MSK_RES_ERR_UPPER_TRIANGLE                                   : i32 = 6020;
pub const MSK_RES_ERR_WHICHITEM_NOT_ALLOWED                            : i32 = 1238;
pub const MSK_RES_ERR_WHICHSOL                                         : i32 = 1236;
pub const MSK_RES_ERR_WRITE_LP_FORMAT                                  : i32 = 1158;
pub const MSK_RES_ERR_WRITE_LP_NON_UNIQUE_NAME                         : i32 = 1161;
pub const MSK_RES_ERR_WRITE_MPS_INVALID_NAME                           : i32 = 1153;
pub const MSK_RES_ERR_WRITE_OPF_INVALID_VAR_NAME                       : i32 = 1156;
pub const MSK_RES_ERR_WRITING_FILE                                     : i32 = 1166;
pub const MSK_RES_ERR_XML_INVALID_PROBLEM_TYPE                         : i32 = 3600;
pub const MSK_RES_ERR_Y_IS_UNDEFINED                                   : i32 = 1449;
pub const MSK_RES_OK                                                   : i32 = 0;
pub const MSK_RES_TRM_INTERNAL                                         : i32 = 100030;
pub const MSK_RES_TRM_INTERNAL_STOP                                    : i32 = 100031;
pub const MSK_RES_TRM_MAX_ITERATIONS                                   : i32 = 100000;
pub const MSK_RES_TRM_MAX_NUM_SETBACKS                                 : i32 = 100020;
pub const MSK_RES_TRM_MAX_TIME                                         : i32 = 100001;
pub const MSK_RES_TRM_MIO_NUM_BRANCHES                                 : i32 = 100009;
pub const MSK_RES_TRM_MIO_NUM_RELAXS                                   : i32 = 100008;
pub const MSK_RES_TRM_NUM_MAX_NUM_INT_SOLUTIONS                        : i32 = 100015;
pub const MSK_RES_TRM_NUMERICAL_PROBLEM                                : i32 = 100025;
pub const MSK_RES_TRM_OBJECTIVE_RANGE                                  : i32 = 100002;
pub const MSK_RES_TRM_STALL                                            : i32 = 100006;
pub const MSK_RES_TRM_USER_CALLBACK                                    : i32 = 100007;
pub const MSK_RES_WRN_ANA_ALMOST_INT_BOUNDS                            : i32 = 904;
pub const MSK_RES_WRN_ANA_C_ZERO                                       : i32 = 901;
pub const MSK_RES_WRN_ANA_CLOSE_BOUNDS                                 : i32 = 903;
pub const MSK_RES_WRN_ANA_EMPTY_COLS                                   : i32 = 902;
pub const MSK_RES_WRN_ANA_LARGE_BOUNDS                                 : i32 = 900;
pub const MSK_RES_WRN_DROPPED_NZ_QOBJ                                  : i32 = 201;
pub const MSK_RES_WRN_DUPLICATE_BARVARIABLE_NAMES                      : i32 = 852;
pub const MSK_RES_WRN_DUPLICATE_CONE_NAMES                             : i32 = 853;
pub const MSK_RES_WRN_DUPLICATE_CONSTRAINT_NAMES                       : i32 = 850;
pub const MSK_RES_WRN_DUPLICATE_VARIABLE_NAMES                         : i32 = 851;
pub const MSK_RES_WRN_ELIMINATOR_SPACE                                 : i32 = 801;
pub const MSK_RES_WRN_EMPTY_NAME                                       : i32 = 502;
pub const MSK_RES_WRN_EXP_CONES_WITH_VARIABLES_FIXED_AT_ZERO           : i32 = 932;
pub const MSK_RES_WRN_IGNORE_INTEGER                                   : i32 = 250;
pub const MSK_RES_WRN_INCOMPLETE_LINEAR_DEPENDENCY_CHECK               : i32 = 800;
pub const MSK_RES_WRN_INVALID_MPS_NAME                                 : i32 = 504;
pub const MSK_RES_WRN_INVALID_MPS_OBJ_NAME                             : i32 = 505;
pub const MSK_RES_WRN_LARGE_AIJ                                        : i32 = 62;
pub const MSK_RES_WRN_LARGE_BOUND                                      : i32 = 51;
pub const MSK_RES_WRN_LARGE_CJ                                         : i32 = 57;
pub const MSK_RES_WRN_LARGE_CON_FX                                     : i32 = 54;
pub const MSK_RES_WRN_LARGE_FIJ                                        : i32 = 980;
pub const MSK_RES_WRN_LARGE_LO_BOUND                                   : i32 = 52;
pub const MSK_RES_WRN_LARGE_UP_BOUND                                   : i32 = 53;
pub const MSK_RES_WRN_LICENSE_EXPIRE                                   : i32 = 500;
pub const MSK_RES_WRN_LICENSE_FEATURE_EXPIRE                           : i32 = 509;
pub const MSK_RES_WRN_LICENSE_SERVER                                   : i32 = 501;
pub const MSK_RES_WRN_LP_DROP_VARIABLE                                 : i32 = 85;
pub const MSK_RES_WRN_LP_OLD_QUAD_FORMAT                               : i32 = 80;
pub const MSK_RES_WRN_MIO_INFEASIBLE_FINAL                             : i32 = 270;
pub const MSK_RES_WRN_MODIFIED_DOUBLE_PARAMETER                        : i32 = 970;
pub const MSK_RES_WRN_MPS_SPLIT_BOU_VECTOR                             : i32 = 72;
pub const MSK_RES_WRN_MPS_SPLIT_RAN_VECTOR                             : i32 = 71;
pub const MSK_RES_WRN_MPS_SPLIT_RHS_VECTOR                             : i32 = 70;
pub const MSK_RES_WRN_NAME_MAX_LEN                                     : i32 = 65;
pub const MSK_RES_WRN_NO_DUALIZER                                      : i32 = 950;
pub const MSK_RES_WRN_NO_GLOBAL_OPTIMIZER                              : i32 = 251;
pub const MSK_RES_WRN_NZ_IN_UPR_TRI                                    : i32 = 200;
pub const MSK_RES_WRN_OPEN_PARAM_FILE                                  : i32 = 50;
pub const MSK_RES_WRN_PARAM_IGNORED_CMIO                               : i32 = 516;
pub const MSK_RES_WRN_PARAM_NAME_DOU                                   : i32 = 510;
pub const MSK_RES_WRN_PARAM_NAME_INT                                   : i32 = 511;
pub const MSK_RES_WRN_PARAM_NAME_STR                                   : i32 = 512;
pub const MSK_RES_WRN_PARAM_STR_VALUE                                  : i32 = 515;
pub const MSK_RES_WRN_POW_CONES_WITH_ROOT_FIXED_AT_ZERO                : i32 = 933;
pub const MSK_RES_WRN_PRESOLVE_OUTOFSPACE                              : i32 = 802;
pub const MSK_RES_WRN_QUAD_CONES_WITH_ROOT_FIXED_AT_ZERO               : i32 = 930;
pub const MSK_RES_WRN_RQUAD_CONES_WITH_ROOT_FIXED_AT_ZERO              : i32 = 931;
pub const MSK_RES_WRN_SOL_FILE_IGNORED_CON                             : i32 = 351;
pub const MSK_RES_WRN_SOL_FILE_IGNORED_VAR                             : i32 = 352;
pub const MSK_RES_WRN_SOL_FILTER                                       : i32 = 300;
pub const MSK_RES_WRN_SPAR_MAX_LEN                                     : i32 = 66;
pub const MSK_RES_WRN_SYM_MAT_LARGE                                    : i32 = 960;
pub const MSK_RES_WRN_TOO_FEW_BASIS_VARS                               : i32 = 400;
pub const MSK_RES_WRN_TOO_MANY_BASIS_VARS                              : i32 = 405;
pub const MSK_RES_WRN_UNDEF_SOL_FILE_NAME                              : i32 = 350;
pub const MSK_RES_WRN_USING_GENERIC_NAMES                              : i32 = 503;
pub const MSK_RES_WRN_WRITE_CHANGED_NAMES                              : i32 = 803;
pub const MSK_RES_WRN_WRITE_DISCARDED_CFIX                             : i32 = 804;
pub const MSK_RES_WRN_WRITE_LP_DUPLICATE_CON_NAMES                     : i32 = 857;
pub const MSK_RES_WRN_WRITE_LP_DUPLICATE_VAR_NAMES                     : i32 = 855;
pub const MSK_RES_WRN_WRITE_LP_INVALID_CON_NAMES                       : i32 = 856;
pub const MSK_RES_WRN_WRITE_LP_INVALID_VAR_NAMES                       : i32 = 854;
pub const MSK_RES_WRN_ZERO_AIJ                                         : i32 = 63;
pub const MSK_RES_WRN_ZEROS_IN_SPARSE_COL                              : i32 = 710;
pub const MSK_RES_WRN_ZEROS_IN_SPARSE_ROW                              : i32 = 705;

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
pub const MSK_SCALING_FREE : i32 = 0;
pub const MSK_SCALING_NONE : i32 = 1;
pub const MSK_SCALING_BEGIN : i32 = 0;
pub const MSK_SCALING_END   : i32 = 2;

// sensitivitytype
pub const MSK_SENSITIVITY_TYPE_BASIS : i32 = 0;
pub const MSK_SENSITIVITY_TYPE_BEGIN : i32 = 0;
pub const MSK_SENSITIVITY_TYPE_END   : i32 = 1;

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
pub const MSK_SOL_STA_DUAL_FEAS          : i32 = 3;
pub const MSK_SOL_STA_DUAL_ILLPOSED_CER  : i32 = 8;
pub const MSK_SOL_STA_DUAL_INFEAS_CER    : i32 = 6;
pub const MSK_SOL_STA_INTEGER_OPTIMAL    : i32 = 9;
pub const MSK_SOL_STA_OPTIMAL            : i32 = 1;
pub const MSK_SOL_STA_PRIM_AND_DUAL_FEAS : i32 = 4;
pub const MSK_SOL_STA_PRIM_FEAS          : i32 = 2;
pub const MSK_SOL_STA_PRIM_ILLPOSED_CER  : i32 = 7;
pub const MSK_SOL_STA_PRIM_INFEAS_CER    : i32 = 5;
pub const MSK_SOL_STA_UNKNOWN            : i32 = 0;
pub const MSK_SOL_STA_BEGIN : i32 = 0;
pub const MSK_SOL_STA_END   : i32 = 10;

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
pub const MSK_SPAR_REMOTE_TLS_CERT           : i32 = 13;
pub const MSK_SPAR_REMOTE_TLS_CERT_PATH      : i32 = 14;
pub const MSK_SPAR_SENSITIVITY_FILE_NAME     : i32 = 15;
pub const MSK_SPAR_SENSITIVITY_RES_FILE_NAME : i32 = 16;
pub const MSK_SPAR_SOL_FILTER_XC_LOW         : i32 = 17;
pub const MSK_SPAR_SOL_FILTER_XC_UPR         : i32 = 18;
pub const MSK_SPAR_SOL_FILTER_XX_LOW         : i32 = 19;
pub const MSK_SPAR_SOL_FILTER_XX_UPR         : i32 = 20;
pub const MSK_SPAR_STAT_KEY                  : i32 = 21;
pub const MSK_SPAR_STAT_NAME                 : i32 = 22;
pub const MSK_SPAR_WRITE_LP_GEN_VAR_NAME     : i32 = 23;
pub const MSK_SPAR_BEGIN : i32 = 0;
pub const MSK_SPAR_END   : i32 = 24;

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
pub const MSK_LICENSE_BUFFER_LENGTH : i32 = 21;
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

fn handle_res_static(r : i32, funname : &str) -> Result<(),String> {
    return
        ( if r != 0 { Err(format!("Error in call to {}: {}",r,funname)) }
          else {
              Ok(())
          })
}


pub struct Env
{
    ptr : * const u8,
}

pub struct Task
{
    ptr       : * const u8,
    streamcb  : [ Option<Box<Box<dyn Fn(&str)>>>; 4 ],
    valuecb   : Option<Box<Box<dyn FnMut(i32,&[f64],&[i32],&[i64]) -> bool>>>,
}

impl Env
{
    pub fn new() -> Option<Env> {
        let mut env : * const u8 = std::ptr::null();
        let res = unsafe { MSK_makeenv(& mut env, std::ptr::null()) };
        if res != 0 { return None; }

        return Some(Env { ptr : env });
    }

    pub fn new_mem_debug(dbgfile : &str) -> Option<Env> {
        let mut env : * const u8 = std::ptr::null();
        let res = unsafe { MSK_makeenv(& mut env, CString::new(dbgfile).unwrap().as_ptr()) };
        if res != 0 { return None; }

        return Some(Env { ptr : env });
    }

    pub fn task(&self) -> Option<Task> {
        let mut task : * const u8 = std::ptr::null();
        if 0 != unsafe { MSK_maketask(self.ptr, 0,0, & mut task) } {
            return None;
        }

        return Some(Task { ptr      : task,
                           streamcb : [None,None,None,None],
                           valuecb  : None,});
    }

    pub fn task_with_capacity(&self, numcon : i32, numvar : i32) -> Option<Task>
    {
        let mut task : * const u8 = std::ptr::null();
        if 0 != unsafe { MSK_maketask(self.ptr, numcon,numvar, & mut task) }
        {
            return None;
        }

        return Some(Task { ptr      : task,
                           streamcb : [None,None,None,None],
                           valuecb  : None, });
    }

    fn handle_res(&self, r : i32, funname : &str) -> Result<(),String> {
        return handle_res_static(r,funname);
    }


    // axpy
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn axpy(&self,n_ : i32,alpha_ : f64,x_ : & [f64],y_ : & mut [f64]) -> Result<(),String>
    {
      if x_.len() != ((n_) as usize) { return Result::Err("Argument 'x_' is too short in call to 'axpy'".to_string()) }
      if y_.len() != ((n_) as usize) { return Result::Err("Argument 'y_' is too short in call to 'axpy'".to_string()) }
      let call_res = unsafe { (MSK_axpy(self.ptr,n_ as i32,alpha_ as f64,x_.as_ptr(),y_.as_mut_ptr())) };
      self.handle_res(call_res,"axpy")?;
      return Result::Ok(());
    }

    // checkinall
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn check_in_all(& mut self) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_checkinall(self.ptr)) };
      self.handle_res(call_res,"checkinall")?;
      return Result::Ok(());
    }

    // checkinlicense
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn check_in_license(& mut self,feature_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_checkinlicense(self.ptr,feature_)) };
      self.handle_res(call_res,"checkinlicense")?;
      return Result::Ok(());
    }

    // checkmemenv
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn check_mem(& mut self,file_ : &str,line_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_checkmemenv(self.ptr,CString::new(file_).unwrap().as_ptr(),line_ as i32)) };
      self.handle_res(call_res,"checkmemenv")?;
      return Result::Ok(());
    }

    // checkoutlicense
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn checkout_license(& mut self,feature_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_checkoutlicense(self.ptr,feature_)) };
      self.handle_res(call_res,"checkoutlicense")?;
      return Result::Ok(());
    }

    // checkversion
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn check_version(&self,major_ : i32,minor_ : i32,revision_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_checkversion(self.ptr,major_ as i32,minor_ as i32,revision_ as i32)) };
      self.handle_res(call_res,"checkversion")?;
      return Result::Ok(());
    }

    // dot
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn dot(&self,n_ : i32,x_ : & [f64],y_ : & [f64]) -> Result<f64,String>
    {
      if x_.len() != ((n_) as usize) { return Result::Err("Argument 'x_' is too short in call to 'dot'".to_string()) }
      if y_.len() != ((n_) as usize) { return Result::Err("Argument 'y_' is too short in call to 'dot'".to_string()) }
      let mut _ref_xty_ : f64 = 0 as f64;
      let call_res = unsafe { (MSK_dot(self.ptr,n_ as i32,x_.as_ptr(),y_.as_ptr(),& mut _ref_xty_)) };
      self.handle_res(call_res,"dot")?;
      return Result::Ok((_ref_xty_ as f64));
    }

    // echoenv
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn echo_env(&self,whichstream_ : i32,format_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_echoenv(self.ptr,whichstream_,CString::new(format_).unwrap().as_ptr())) };
      self.handle_res(call_res,"echoenv")?;
      return Result::Ok(());
    }

    // echointro
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn echo_intro(&self,longver_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_echointro(self.ptr,longver_ as i32)) };
      self.handle_res(call_res,"echointro")?;
      return Result::Ok(());
    }

    // gemm
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn gemm(&self,transa_ : i32,transb_ : i32,m_ : i32,n_ : i32,k_ : i32,alpha_ : f64,a_ : & [f64],b_ : & [f64],beta_ : f64,c_ : & mut [f64]) -> Result<(),String>
    {
      if a_.len() != ((m_ * k_) as usize) { return Result::Err("Argument 'a_' is too short in call to 'gemm'".to_string()) }
      if b_.len() != ((k_ * n_) as usize) { return Result::Err("Argument 'b_' is too short in call to 'gemm'".to_string()) }
      if c_.len() != ((m_ * n_) as usize) { return Result::Err("Argument 'c_' is too short in call to 'gemm'".to_string()) }
      let call_res = unsafe { (MSK_gemm(self.ptr,transa_,transb_,m_ as i32,n_ as i32,k_ as i32,alpha_ as f64,a_.as_ptr(),b_.as_ptr(),beta_ as f64,c_.as_mut_ptr())) };
      self.handle_res(call_res,"gemm")?;
      return Result::Ok(());
    }

    // gemv
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn gemv(&self,transa_ : i32,m_ : i32,n_ : i32,alpha_ : f64,a_ : & [f64],x_ : & [f64],beta_ : f64,y_ : & mut [f64]) -> Result<(),String>
    {
      if a_.len() != ((n_ * m_) as usize) { return Result::Err("Argument 'a_' is too short in call to 'gemv'".to_string()) }
      let tmp_var_3__ =
        if (transa_ == MSK_TRANSPOSE_NO) {
          n_
        }  else {
          m_
        };
      if x_.len() != ((tmp_var_3__) as usize) { return Result::Err("Argument 'x_' is too short in call to 'gemv'".to_string()) }
      let tmp_var_9__ =
        if (transa_ == MSK_TRANSPOSE_NO) {
          m_
        }  else {
          n_
        };
      if y_.len() != ((tmp_var_9__) as usize) { return Result::Err("Argument 'y_' is too short in call to 'gemv'".to_string()) }
      let call_res = unsafe { (MSK_gemv(self.ptr,transa_,m_ as i32,n_ as i32,alpha_ as f64,a_.as_ptr(),x_.as_ptr(),beta_ as f64,y_.as_mut_ptr())) };
      self.handle_res(call_res,"gemv")?;
      return Result::Ok(());
    }

    // linkfiletoenvstream
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn link_file_to_env_stream(& mut self,whichstream_ : i32,filename_ : &str,append_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_linkfiletoenvstream(self.ptr,whichstream_,CString::new(filename_).unwrap().as_ptr(),append_ as i32)) };
      self.handle_res(call_res,"linkfiletoenvstream")?;
      return Result::Ok(());
    }

    // potrf
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn potrf(&self,uplo_ : i32,n_ : i32,a_ : & mut [f64]) -> Result<(),String>
    {
      if a_.len() != ((n_ * n_) as usize) { return Result::Err("Argument 'a_' is too short in call to 'potrf'".to_string()) }
      let call_res = unsafe { (MSK_potrf(self.ptr,uplo_,n_ as i32,a_.as_mut_ptr())) };
      self.handle_res(call_res,"potrf")?;
      return Result::Ok(());
    }

    // putlicensecode
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_license_code(& mut self,code_ : & [i32]) -> Result<(),String>
    {
      if code_.len() != ((MSK_LICENSE_BUFFER_LENGTH) as usize) { return Result::Err("Argument 'code_' is too short in call to 'put_license_code'".to_string()) }
      let call_res = unsafe { (MSK_putlicensecode(self.ptr,code_.as_ptr())) };
      self.handle_res(call_res,"putlicensecode")?;
      return Result::Ok(());
    }

    // putlicensedebug
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_license_debug(& mut self,licdebug_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putlicensedebug(self.ptr,licdebug_ as i32)) };
      self.handle_res(call_res,"putlicensedebug")?;
      return Result::Ok(());
    }

    // putlicensepath
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_license_path(& mut self,licensepath_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putlicensepath(self.ptr,CString::new(licensepath_).unwrap().as_ptr())) };
      self.handle_res(call_res,"putlicensepath")?;
      return Result::Ok(());
    }

    // putlicensewait
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_license_wait(& mut self,licwait_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putlicensewait(self.ptr,licwait_ as i32)) };
      self.handle_res(call_res,"putlicensewait")?;
      return Result::Ok(());
    }

    // setupthreads
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn setup_threads(& mut self,numthreads_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_setupthreads(self.ptr,numthreads_ as i32)) };
      self.handle_res(call_res,"setupthreads")?;
      return Result::Ok(());
    }

    // sparsetriangularsolvedense
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn sparse_triangular_solve_dense(&self,transposed_ : i32,lnzc_ : & [i32],lptrc_ : & [i64],lsubc_ : & [i32],lvalc_ : & [f64],b_ : & mut [f64]) -> Result<(),String>
    {
      let mut n_ = b_.len();
      if lnzc_.len() < n_ { n_ = lnzc_.len() };
      if lptrc_.len() < n_ { n_ = lptrc_.len() };
      if lnzc_.len() != ((n_) as usize) { return Result::Err("Argument 'lnzc_' is too short in call to 'sparse_triangular_solve_dense'".to_string()) }
      if lptrc_.len() != ((n_) as usize) { return Result::Err("Argument 'lptrc_' is too short in call to 'sparse_triangular_solve_dense'".to_string()) }
      let mut lensubnval_ = lsubc_.len();
      if lvalc_.len() < lensubnval_ { lensubnval_ = lvalc_.len() };
      if lsubc_.len() != ((lensubnval_) as usize) { return Result::Err("Argument 'lsubc_' is too short in call to 'sparse_triangular_solve_dense'".to_string()) }
      if lvalc_.len() != ((lensubnval_) as usize) { return Result::Err("Argument 'lvalc_' is too short in call to 'sparse_triangular_solve_dense'".to_string()) }
      if b_.len() != ((n_) as usize) { return Result::Err("Argument 'b_' is too short in call to 'sparse_triangular_solve_dense'".to_string()) }
      let call_res = unsafe { (MSK_sparsetriangularsolvedense(self.ptr,transposed_,n_ as i32,lnzc_.as_ptr(),lptrc_.as_ptr(),lensubnval_ as i64,lsubc_.as_ptr(),lvalc_.as_ptr(),b_.as_mut_ptr())) };
      self.handle_res(call_res,"sparsetriangularsolvedense")?;
      return Result::Ok(());
    }

    // syeig
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn syeig(&self,uplo_ : i32,n_ : i32,a_ : & [f64],w_ : & mut [f64]) -> Result<(),String>
    {
      if a_.len() != ((n_ * n_) as usize) { return Result::Err("Argument 'a_' is too short in call to 'syeig'".to_string()) }
      if w_.len() != ((n_) as usize) { return Result::Err("Argument 'w_' is too short in call to 'syeig'".to_string()) }
      let call_res = unsafe { (MSK_syeig(self.ptr,uplo_,n_ as i32,a_.as_ptr(),w_.as_mut_ptr())) };
      self.handle_res(call_res,"syeig")?;
      return Result::Ok(());
    }

    // syevd
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn syevd(&self,uplo_ : i32,n_ : i32,a_ : & mut [f64],w_ : & mut [f64]) -> Result<(),String>
    {
      if a_.len() != ((n_ * n_) as usize) { return Result::Err("Argument 'a_' is too short in call to 'syevd'".to_string()) }
      if w_.len() != ((n_) as usize) { return Result::Err("Argument 'w_' is too short in call to 'syevd'".to_string()) }
      let call_res = unsafe { (MSK_syevd(self.ptr,uplo_,n_ as i32,a_.as_mut_ptr(),w_.as_mut_ptr())) };
      self.handle_res(call_res,"syevd")?;
      return Result::Ok(());
    }

    // syrk
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn syrk(&self,uplo_ : i32,trans_ : i32,n_ : i32,k_ : i32,alpha_ : f64,a_ : & [f64],beta_ : f64,c_ : & mut [f64]) -> Result<(),String>
    {
      if a_.len() != ((n_ * k_) as usize) { return Result::Err("Argument 'a_' is too short in call to 'syrk'".to_string()) }
      if c_.len() != ((n_ * n_) as usize) { return Result::Err("Argument 'c_' is too short in call to 'syrk'".to_string()) }
      let call_res = unsafe { (MSK_syrk(self.ptr,uplo_,trans_,n_ as i32,k_ as i32,alpha_ as f64,a_.as_ptr(),beta_ as f64,c_.as_mut_ptr())) };
      self.handle_res(call_res,"syrk")?;
      return Result::Ok(());
    }

    // unlinkfuncfromenvstream
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn unlink_func_from_stream(& mut self,whichstream_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_unlinkfuncfromenvstream(self.ptr,whichstream_)) };
      self.handle_res(call_res,"unlinkfuncfromenvstream")?;
      return Result::Ok(());
    }

}




extern fn stream_callback_proxy(handle : * const libc::c_void, msg : * const libc::c_char) {
    let h = handle as * const Box<Fn(&str)>;
    unsafe
    {
        let cstr = CStr::from_ptr(msg);
        let cstr_bytes = cstr.to_bytes();
        let s = String::from_utf8_lossy(cstr_bytes).into_owned();
        (*h)(&s);
    }
}


extern fn callback_proxy(_       : * const c_void,
                         handle  : * const libc::c_void,
                         caller  : i32,
                         douinf  : * const f64,
                         intinf  : * const i32,
                         lintinf : * const i64 ) -> i32
{
    let h = handle as * mut Box<FnMut(i32,&[f64],&[i32],&[i64]) -> bool>;
    unsafe
    {
        let r = (*h)(caller,
                     & std::slice::from_raw_parts(douinf, MSK_DINF_END as usize),
                     & std::slice::from_raw_parts(intinf, MSK_IINF_END as usize),
                     & std::slice::from_raw_parts(lintinf, MSK_LIINF_END as usize));
        return if r { 0 } else { 1 }
    }
}

impl Task
{
    fn handle_res(&self, r : i32, funname : &str) -> Result<(),String> {
        return (
            if 0 != r {
                let mut lastsz   : i64 = 0;
                let mut lastcode : i32 = 0;
                if 0 == unsafe{ MSK_getlasterror64(self.ptr,& mut lastcode,0,& mut lastsz,std::ptr::null_mut()) } {
                    let mut lastmsg : Vec<u8> = vec![0; (lastsz+1) as usize];
                    unsafe{ MSK_getlasterror64(self.ptr,& mut lastcode,lastsz+1,& mut lastsz,lastmsg.as_mut_ptr()) };
                    let lastmsgstr = String::from_utf8_lossy(&lastmsg[0..lastsz as usize]);
                    Result::Err(format!("Error in call to {}: ({}) {:?}",funname,r,lastmsgstr))
                }
                else {
                    handle_res_static(r,funname)
                }
            }
            else {
                Ok(())
            }
        );
    }


    // NOTE on callback with handles:
    //   http://aatch.github.io/blog/2015/01/17/unboxed-closures-and-ffi-callbacks/
    pub fn put_stream_callback<F>(& mut self,whichstream : i32, func : F) -> Result<(),String>
    where F : 'static+Fn(&str) {
        if whichstream >= 0 && whichstream < 4 //
        {
            self.streamcb[whichstream as usize] = Some(Box::new(Box::new(func)));

            match self.streamcb[whichstream as usize] {
                Some(ref bf) => {
                    let hnd =  &(**bf) as * const _ as * mut libc::c_void;
                    callMSK!(MSK_linkfunctotaskstream,self.ptr,whichstream, hnd, stream_callback_proxy);
                }
                None => {}
            }
        }
        return Ok(());
    }


    pub fn clear_stream_callback(&mut self,whichstream : i32) -> Result<(),String> {
        match self.streamcb[whichstream as usize] {
            Some(ref f) => {
                callMSK!(MSK_unlinkfuncfromtaskstream,self.ptr, whichstream);
            }
            None => {}
        }
        self.streamcb[whichstream as usize] = None;
        return Ok(());
    }

    pub fn put_callback<F>(& mut self,func : F) -> Result<(),String>
    where F : 'static +FnMut(i32,&[f64],&[i32],&[i64]) -> bool {
        self.valuecb = Some(Box::new(Box::new(func)));
        match self.valuecb {
            Some(ref f) => {
                let hnd =  &(**f) as * const _ as * mut libc::c_void;
                callMSK!(MSK_putcallbackfunc,self.ptr, callback_proxy, hnd);
            }
            None => {}
        }
        return Ok(());
    }


    // analyzenames
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn analyze_names(&self,whichstream_ : i32,nametype_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_analyzenames(self.ptr,whichstream_,nametype_)) };
      self.handle_res(call_res,"analyzenames")?;
      return Result::Ok(());
    }

    // analyzeproblem
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn analyze_problem(&self,whichstream_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_analyzeproblem(self.ptr,whichstream_)) };
      self.handle_res(call_res,"analyzeproblem")?;
      return Result::Ok(());
    }

    // analyzesolution
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn analyze_solution(&self,whichstream_ : i32,whichsol_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_analyzesolution(self.ptr,whichstream_,whichsol_)) };
      self.handle_res(call_res,"analyzesolution")?;
      return Result::Ok(());
    }

    // appendacc
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_acc(& mut self,domidx_ : i64,afeidxlist_ : & [i64],b_ : & [f64]) -> Result<(),String>
    {
      let mut numafeidx_ = afeidxlist_.len();
      if b_.len() != ((numafeidx_) as usize) { return Result::Err("Argument 'b_' is too short in call to 'append_acc'".to_string()) }
      let call_res = unsafe { (MSK_appendacc(self.ptr,domidx_ as i64,numafeidx_ as i64,afeidxlist_.as_ptr(),b_.as_ptr())) };
      self.handle_res(call_res,"appendacc")?;
      return Result::Ok(());
    }

    // appendaccs
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_accs(& mut self,domidxs_ : & [i64],afeidxlist_ : & [i64],b_ : & [f64]) -> Result<(),String>
    {
      let mut numaccs_ = domidxs_.len();
      let mut numafeidx_ = afeidxlist_.len();
      if b_.len() != ((numafeidx_) as usize) { return Result::Err("Argument 'b_' is too short in call to 'append_accs'".to_string()) }
      let call_res = unsafe { (MSK_appendaccs(self.ptr,numaccs_ as i64,domidxs_.as_ptr(),numafeidx_ as i64,afeidxlist_.as_ptr(),b_.as_ptr())) };
      self.handle_res(call_res,"appendaccs")?;
      return Result::Ok(());
    }

    // appendaccseq
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_acc_seq(& mut self,domidx_ : i64,afeidxfirst_ : i64,b_ : & [f64]) -> Result<(),String>
    {
      let mut tmp_var_3__ : i64 = 0 as i64;
      let tmp_var_1__ = unsafe { MSK_getdomainn(self.ptr,domidx_,& mut tmp_var_3__) };
      self.handle_res(tmp_var_1__,"getdomainn")?;
      let numafeidx_ = tmp_var_3__;
      if b_.len() != ((numafeidx_) as usize) { return Result::Err("Argument 'b_' is too short in call to 'append_acc_seq'".to_string()) }
      let call_res = unsafe { (MSK_appendaccseq(self.ptr,domidx_ as i64,numafeidx_ as i64,afeidxfirst_ as i64,b_.as_ptr())) };
      self.handle_res(call_res,"appendaccseq")?;
      return Result::Ok(());
    }

    // appendaccsseq
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_accs_seq(& mut self,domidxs_ : & [i64],numafeidx_ : i64,afeidxfirst_ : i64,b_ : & [f64]) -> Result<(),String>
    {
      let mut numaccs_ = domidxs_.len();
      if b_.len() != ((numafeidx_) as usize) { return Result::Err("Argument 'b_' is too short in call to 'append_accs_seq'".to_string()) }
      let call_res = unsafe { (MSK_appendaccsseq(self.ptr,numaccs_ as i64,domidxs_.as_ptr(),numafeidx_ as i64,afeidxfirst_ as i64,b_.as_ptr())) };
      self.handle_res(call_res,"appendaccsseq")?;
      return Result::Ok(());
    }

    // appendafes
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_afes(& mut self,num_ : i64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_appendafes(self.ptr,num_ as i64)) };
      self.handle_res(call_res,"appendafes")?;
      return Result::Ok(());
    }

    // appendbarvars
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_barvars(& mut self,dim_ : & [i32]) -> Result<(),String>
    {
      let mut num_ = dim_.len();
      let call_res = unsafe { (MSK_appendbarvars(self.ptr,num_ as i32,dim_.as_ptr())) };
      self.handle_res(call_res,"appendbarvars")?;
      return Result::Ok(());
    }

    // appendcone
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_cone(& mut self,ct_ : i32,conepar_ : f64,submem_ : & [i32]) -> Result<(),String>
    {
      let mut nummem_ = submem_.len();
      let call_res = unsafe { (MSK_appendcone(self.ptr,ct_,conepar_ as f64,nummem_ as i32,submem_.as_ptr())) };
      self.handle_res(call_res,"appendcone")?;
      return Result::Ok(());
    }

    // appendconeseq
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_cone_seq(& mut self,ct_ : i32,conepar_ : f64,nummem_ : i32,j_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_appendconeseq(self.ptr,ct_,conepar_ as f64,nummem_ as i32,j_ as i32)) };
      self.handle_res(call_res,"appendconeseq")?;
      return Result::Ok(());
    }

    // appendconesseq
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_cones_seq(& mut self,ct_ : & [i32],conepar_ : & [f64],nummem_ : & [i32],j_ : i32) -> Result<(),String>
    {
      let mut num_ = ct_.len();
      if conepar_.len() < num_ { num_ = conepar_.len() };
      if nummem_.len() < num_ { num_ = nummem_.len() };
      let call_res = unsafe { (MSK_appendconesseq(self.ptr,num_ as i32,ct_.as_ptr(),conepar_.as_ptr(),nummem_.as_ptr(),j_ as i32)) };
      self.handle_res(call_res,"appendconesseq")?;
      return Result::Ok(());
    }

    // appendcons
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_cons(& mut self,num_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_appendcons(self.ptr,num_ as i32)) };
      self.handle_res(call_res,"appendcons")?;
      return Result::Ok(());
    }

    // appenddjcs
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_djcs(& mut self,num_ : i64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_appenddjcs(self.ptr,num_ as i64)) };
      self.handle_res(call_res,"appenddjcs")?;
      return Result::Ok(());
    }

    // appenddualexpconedomain
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_dual_exp_cone_domain(& mut self) -> Result<i64,String>
    {
      let mut _ref_domidx_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_appenddualexpconedomain(self.ptr,& mut _ref_domidx_)) };
      self.handle_res(call_res,"appenddualexpconedomain")?;
      return Result::Ok((_ref_domidx_ as i64));
    }

    // appenddualgeomeanconedomain
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_dual_geo_mean_cone_domain(& mut self,n_ : i64) -> Result<i64,String>
    {
      let mut _ref_domidx_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_appenddualgeomeanconedomain(self.ptr,n_ as i64,& mut _ref_domidx_)) };
      self.handle_res(call_res,"appenddualgeomeanconedomain")?;
      return Result::Ok((_ref_domidx_ as i64));
    }

    // appenddualpowerconedomain
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_dual_power_cone_domain(& mut self,n_ : i64,alpha_ : & [f64]) -> Result<i64,String>
    {
      let mut nleft_ = alpha_.len();
      let mut _ref_domidx_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_appenddualpowerconedomain(self.ptr,n_ as i64,nleft_ as i64,alpha_.as_ptr(),& mut _ref_domidx_)) };
      self.handle_res(call_res,"appenddualpowerconedomain")?;
      return Result::Ok((_ref_domidx_ as i64));
    }

    // appendinfnormconedomain
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_inf_norm_cone_domain(& mut self,n_ : i64) -> Result<i64,String>
    {
      let mut _ref_domidx_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_appendinfnormconedomain(self.ptr,n_ as i64,& mut _ref_domidx_)) };
      self.handle_res(call_res,"appendinfnormconedomain")?;
      return Result::Ok((_ref_domidx_ as i64));
    }

    // appendonenormconedomain
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_one_norm_cone_domain(& mut self,n_ : i64) -> Result<i64,String>
    {
      let mut _ref_domidx_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_appendonenormconedomain(self.ptr,n_ as i64,& mut _ref_domidx_)) };
      self.handle_res(call_res,"appendonenormconedomain")?;
      return Result::Ok((_ref_domidx_ as i64));
    }

    // appendprimalexpconedomain
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_primal_exp_cone_domain(& mut self) -> Result<i64,String>
    {
      let mut _ref_domidx_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_appendprimalexpconedomain(self.ptr,& mut _ref_domidx_)) };
      self.handle_res(call_res,"appendprimalexpconedomain")?;
      return Result::Ok((_ref_domidx_ as i64));
    }

    // appendprimalgeomeanconedomain
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_primal_geo_mean_cone_domain(& mut self,n_ : i64) -> Result<i64,String>
    {
      let mut _ref_domidx_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_appendprimalgeomeanconedomain(self.ptr,n_ as i64,& mut _ref_domidx_)) };
      self.handle_res(call_res,"appendprimalgeomeanconedomain")?;
      return Result::Ok((_ref_domidx_ as i64));
    }

    // appendprimalpowerconedomain
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_primal_power_cone_domain(& mut self,n_ : i64,alpha_ : & [f64]) -> Result<i64,String>
    {
      let mut nleft_ = alpha_.len();
      let mut _ref_domidx_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_appendprimalpowerconedomain(self.ptr,n_ as i64,nleft_ as i64,alpha_.as_ptr(),& mut _ref_domidx_)) };
      self.handle_res(call_res,"appendprimalpowerconedomain")?;
      return Result::Ok((_ref_domidx_ as i64));
    }

    // appendpsdconedomain
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_psd_cone_domain(& mut self,n_ : i64) -> Result<i64,String>
    {
      let mut _ref_domidx_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_appendpsdconedomain(self.ptr,n_ as i64,& mut _ref_domidx_)) };
      self.handle_res(call_res,"appendpsdconedomain")?;
      return Result::Ok((_ref_domidx_ as i64));
    }

    // appendquadraticconedomain
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_quadratic_cone_domain(& mut self,n_ : i64) -> Result<i64,String>
    {
      let mut _ref_domidx_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_appendquadraticconedomain(self.ptr,n_ as i64,& mut _ref_domidx_)) };
      self.handle_res(call_res,"appendquadraticconedomain")?;
      return Result::Ok((_ref_domidx_ as i64));
    }

    // appendrdomain
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_r_domain(& mut self,n_ : i64) -> Result<i64,String>
    {
      let mut _ref_domidx_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_appendrdomain(self.ptr,n_ as i64,& mut _ref_domidx_)) };
      self.handle_res(call_res,"appendrdomain")?;
      return Result::Ok((_ref_domidx_ as i64));
    }

    // appendrminusdomain
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_rminus_domain(& mut self,n_ : i64) -> Result<i64,String>
    {
      let mut _ref_domidx_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_appendrminusdomain(self.ptr,n_ as i64,& mut _ref_domidx_)) };
      self.handle_res(call_res,"appendrminusdomain")?;
      return Result::Ok((_ref_domidx_ as i64));
    }

    // appendrplusdomain
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_rplus_domain(& mut self,n_ : i64) -> Result<i64,String>
    {
      let mut _ref_domidx_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_appendrplusdomain(self.ptr,n_ as i64,& mut _ref_domidx_)) };
      self.handle_res(call_res,"appendrplusdomain")?;
      return Result::Ok((_ref_domidx_ as i64));
    }

    // appendrquadraticconedomain
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_r_quadratic_cone_domain(& mut self,n_ : i64) -> Result<i64,String>
    {
      let mut _ref_domidx_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_appendrquadraticconedomain(self.ptr,n_ as i64,& mut _ref_domidx_)) };
      self.handle_res(call_res,"appendrquadraticconedomain")?;
      return Result::Ok((_ref_domidx_ as i64));
    }

    // appendrzerodomain
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_rzero_domain(& mut self,n_ : i64) -> Result<i64,String>
    {
      let mut _ref_domidx_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_appendrzerodomain(self.ptr,n_ as i64,& mut _ref_domidx_)) };
      self.handle_res(call_res,"appendrzerodomain")?;
      return Result::Ok((_ref_domidx_ as i64));
    }

    // appendsparsesymmat
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_sparse_sym_mat(& mut self,dim_ : i32,subi_ : & [i32],subj_ : & [i32],valij_ : & [f64]) -> Result<i64,String>
    {
      let mut nz_ = subi_.len();
      if subj_.len() < nz_ { nz_ = subj_.len() };
      if valij_.len() < nz_ { nz_ = valij_.len() };
      let mut _ref_idx_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_appendsparsesymmat(self.ptr,dim_ as i32,nz_ as i64,subi_.as_ptr(),subj_.as_ptr(),valij_.as_ptr(),& mut _ref_idx_)) };
      self.handle_res(call_res,"appendsparsesymmat")?;
      return Result::Ok((_ref_idx_ as i64));
    }

    // appendsparsesymmatlist
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_sparse_sym_mat_list(& mut self,dims_ : & [i32],nz_ : & [i64],subi_ : & [i32],subj_ : & [i32],valij_ : & [f64],idx_ : & mut [i64]) -> Result<(),String>
    {
      let mut num_ = dims_.len();
      if nz_.len() < num_ { num_ = nz_.len() };
      let tmp_var_0__ = nz_.iter().fold(0,|res,v| res + v);
      if subi_.len() != ((tmp_var_0__) as usize) { return Result::Err("Argument 'subi_' is too short in call to 'append_sparse_sym_mat_list'".to_string()) }
      let tmp_var_2__ = nz_.iter().fold(0,|res,v| res + v);
      if subj_.len() != ((tmp_var_2__) as usize) { return Result::Err("Argument 'subj_' is too short in call to 'append_sparse_sym_mat_list'".to_string()) }
      let tmp_var_4__ = nz_.iter().fold(0,|res,v| res + v);
      if valij_.len() != ((tmp_var_4__) as usize) { return Result::Err("Argument 'valij_' is too short in call to 'append_sparse_sym_mat_list'".to_string()) }
      if idx_.len() != ((num_) as usize) { return Result::Err("Argument 'idx_' is too short in call to 'append_sparse_sym_mat_list'".to_string()) }
      let call_res = unsafe { (MSK_appendsparsesymmatlist(self.ptr,num_ as i32,dims_.as_ptr(),nz_.as_ptr(),subi_.as_ptr(),subj_.as_ptr(),valij_.as_ptr(),idx_.as_mut_ptr())) };
      self.handle_res(call_res,"appendsparsesymmatlist")?;
      return Result::Ok(());
    }

    // appendvars
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn append_vars(& mut self,num_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_appendvars(self.ptr,num_ as i32)) };
      self.handle_res(call_res,"appendvars")?;
      return Result::Ok(());
    }

    // asyncgetresult
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn async_get_result(& mut self,addr_ : &str,accesstoken_ : &str,token_ : &str) -> Result<(bool,i32,i32),String>
    {
      let mut _ref_respavailable_ : i32 = 0 as i32;
      let mut _ref_resp_ : i32 = 0 as i32;
      let mut _ref_trm_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_asyncgetresult(self.ptr,CString::new(addr_).unwrap().as_ptr(),CString::new(accesstoken_).unwrap().as_ptr(),CString::new(token_).unwrap().as_ptr(),& mut _ref_respavailable_,& mut _ref_resp_,& mut _ref_trm_)) };
      self.handle_res(call_res,"asyncgetresult")?;
      return Result::Ok((_ref_respavailable_ != 0,_ref_resp_ as i32,_ref_trm_ as i32));
    }

    // asyncoptimize
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn async_optimize(& mut self,addr_ : &str,accesstoken_ : &str) -> Result<String,String>
    {
      let mut _token__bytes = Vec::with_capacity(33 as usize);
      let call_res = unsafe { (MSK_asyncoptimize(self.ptr,CString::new(addr_).unwrap().as_ptr(),CString::new(accesstoken_).unwrap().as_ptr(),_token__bytes.as_mut_ptr())) };
      self.handle_res(call_res,"asyncoptimize")?;
      unsafe { _token__bytes.set_len((33) as usize) };
      return Result::Ok((String::from_utf8_lossy(&_token__bytes[..]).into_owned()));
    }

    // asyncpoll
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn async_poll(& mut self,addr_ : &str,accesstoken_ : &str,token_ : &str) -> Result<(bool,i32,i32),String>
    {
      let mut _ref_respavailable_ : i32 = 0 as i32;
      let mut _ref_resp_ : i32 = 0 as i32;
      let mut _ref_trm_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_asyncpoll(self.ptr,CString::new(addr_).unwrap().as_ptr(),CString::new(accesstoken_).unwrap().as_ptr(),CString::new(token_).unwrap().as_ptr(),& mut _ref_respavailable_,& mut _ref_resp_,& mut _ref_trm_)) };
      self.handle_res(call_res,"asyncpoll")?;
      return Result::Ok((_ref_respavailable_ != 0,_ref_resp_ as i32,_ref_trm_ as i32));
    }

    // asyncstop
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn async_stop(& mut self,addr_ : &str,accesstoken_ : &str,token_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_asyncstop(self.ptr,CString::new(addr_).unwrap().as_ptr(),CString::new(accesstoken_).unwrap().as_ptr(),CString::new(token_).unwrap().as_ptr())) };
      self.handle_res(call_res,"asyncstop")?;
      return Result::Ok(());
    }

    // basiscond
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn basis_cond(& mut self) -> Result<(f64,f64),String>
    {
      let mut _ref_nrmbasis_ : f64 = 0 as f64;
      let mut _ref_nrminvbasis_ : f64 = 0 as f64;
      let call_res = unsafe { (MSK_basiscond(self.ptr,& mut _ref_nrmbasis_,& mut _ref_nrminvbasis_)) };
      self.handle_res(call_res,"basiscond")?;
      return Result::Ok((_ref_nrmbasis_ as f64,_ref_nrminvbasis_ as f64));
    }

    // bktostr
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn bk_to_str(&self,bk_ : i32) -> Result<String,String>
    {
      let mut _str__bytes = Vec::with_capacity(MSK_MAX_STR_LEN as usize);
      let call_res = unsafe { (MSK_bktostr(self.ptr,bk_,_str__bytes.as_mut_ptr())) };
      self.handle_res(call_res,"bktostr")?;
      unsafe { _str__bytes.set_len((MSK_MAX_STR_LEN) as usize) };
      return Result::Ok((String::from_utf8_lossy(&_str__bytes[..]).into_owned()));
    }

    // checkmemtask
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn check_mem(& mut self,file_ : &str,line_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_checkmemtask(self.ptr,CString::new(file_).unwrap().as_ptr(),line_ as i32)) };
      self.handle_res(call_res,"checkmemtask")?;
      return Result::Ok(());
    }

    // chgconbound
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn chg_con_bound(& mut self,i_ : i32,lower_ : i32,finite_ : i32,value_ : f64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_chgconbound(self.ptr,i_ as i32,lower_ as i32,finite_ as i32,value_ as f64)) };
      self.handle_res(call_res,"chgconbound")?;
      return Result::Ok(());
    }

    // chgvarbound
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn chg_var_bound(& mut self,j_ : i32,lower_ : i32,finite_ : i32,value_ : f64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_chgvarbound(self.ptr,j_ as i32,lower_ as i32,finite_ as i32,value_ as f64)) };
      self.handle_res(call_res,"chgvarbound")?;
      return Result::Ok(());
    }

    // commitchanges
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn commit_changes(& mut self) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_commitchanges(self.ptr)) };
      self.handle_res(call_res,"commitchanges")?;
      return Result::Ok(());
    }

    // conetypetostr
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn cone_type_to_str(&self,ct_ : i32) -> Result<String,String>
    {
      let mut _str__bytes = Vec::with_capacity(1024 as usize);
      let call_res = unsafe { (MSK_conetypetostr(self.ptr,ct_,_str__bytes.as_mut_ptr())) };
      self.handle_res(call_res,"conetypetostr")?;
      unsafe { _str__bytes.set_len((1024) as usize) };
      return Result::Ok((String::from_utf8_lossy(&_str__bytes[..]).into_owned()));
    }

    // deletesolution
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn delete_solution(& mut self,whichsol_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_deletesolution(self.ptr,whichsol_)) };
      self.handle_res(call_res,"deletesolution")?;
      return Result::Ok(());
    }

    // dualsensitivity
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn dual_sensitivity(&self,subj_ : & [i32],leftpricej_ : & mut [f64],rightpricej_ : & mut [f64],leftrangej_ : & mut [f64],rightrangej_ : & mut [f64]) -> Result<(),String>
    {
      let mut numj_ = subj_.len();
      if leftpricej_.len() != ((numj_) as usize) { return Result::Err("Argument 'leftpricej_' is too short in call to 'dual_sensitivity'".to_string()) }
      if rightpricej_.len() != ((numj_) as usize) { return Result::Err("Argument 'rightpricej_' is too short in call to 'dual_sensitivity'".to_string()) }
      if leftrangej_.len() != ((numj_) as usize) { return Result::Err("Argument 'leftrangej_' is too short in call to 'dual_sensitivity'".to_string()) }
      if rightrangej_.len() != ((numj_) as usize) { return Result::Err("Argument 'rightrangej_' is too short in call to 'dual_sensitivity'".to_string()) }
      let call_res = unsafe { (MSK_dualsensitivity(self.ptr,numj_ as i32,subj_.as_ptr(),leftpricej_.as_mut_ptr(),rightpricej_.as_mut_ptr(),leftrangej_.as_mut_ptr(),rightrangej_.as_mut_ptr())) };
      self.handle_res(call_res,"dualsensitivity")?;
      return Result::Ok(());
    }

    // echotask
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn echo_task(&self,whichstream_ : i32,format_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_echotask(self.ptr,whichstream_,CString::new(format_).unwrap().as_ptr())) };
      self.handle_res(call_res,"echotask")?;
      return Result::Ok(());
    }

    // emptyafebarfrow
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn empty_afe_barf_row(& mut self,afeidx_ : i64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_emptyafebarfrow(self.ptr,afeidx_ as i64)) };
      self.handle_res(call_res,"emptyafebarfrow")?;
      return Result::Ok(());
    }

    // emptyafebarfrowlist
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn empty_afe_barf_row_list(& mut self,afeidxlist_ : & [i64]) -> Result<(),String>
    {
      let mut numafeidx_ = afeidxlist_.len();
      let call_res = unsafe { (MSK_emptyafebarfrowlist(self.ptr,numafeidx_ as i64,afeidxlist_.as_ptr())) };
      self.handle_res(call_res,"emptyafebarfrowlist")?;
      return Result::Ok(());
    }

    // emptyafefcol
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn empty_afe_f_col(& mut self,j_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_emptyafefcol(self.ptr,j_ as i32)) };
      self.handle_res(call_res,"emptyafefcol")?;
      return Result::Ok(());
    }

    // emptyafefcollist
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn empty_afe_f_col_list(& mut self,varidxlist_ : & [i32]) -> Result<(),String>
    {
      let mut numvaridx_ = varidxlist_.len();
      let call_res = unsafe { (MSK_emptyafefcollist(self.ptr,numvaridx_ as i64,varidxlist_.as_ptr())) };
      self.handle_res(call_res,"emptyafefcollist")?;
      return Result::Ok(());
    }

    // emptyafefrow
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn empty_afe_f_row(& mut self,afeidx_ : i64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_emptyafefrow(self.ptr,afeidx_ as i64)) };
      self.handle_res(call_res,"emptyafefrow")?;
      return Result::Ok(());
    }

    // emptyafefrowlist
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn empty_afe_f_row_list(& mut self,afeidxlist_ : & [i64]) -> Result<(),String>
    {
      let mut numafeidx_ = afeidxlist_.len();
      let call_res = unsafe { (MSK_emptyafefrowlist(self.ptr,numafeidx_ as i64,afeidxlist_.as_ptr())) };
      self.handle_res(call_res,"emptyafefrowlist")?;
      return Result::Ok(());
    }

    // evaluateacc
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn evaluate_acc(&self,whichsol_ : i32,accidx_ : i64,activity_ : & mut [f64]) -> Result<(),String>
    {
      let mut tmp_var_3__ : i64 = 0 as i64;
      let tmp_var_1__ = unsafe { MSK_getaccn(self.ptr,accidx_,& mut tmp_var_3__) };
      self.handle_res(tmp_var_1__,"getaccn")?;
      if activity_.len() != ((tmp_var_3__) as usize) { return Result::Err("Argument 'activity_' is too short in call to 'evaluate_acc'".to_string()) }
      let call_res = unsafe { (MSK_evaluateacc(self.ptr,whichsol_,accidx_ as i64,activity_.as_mut_ptr())) };
      self.handle_res(call_res,"evaluateacc")?;
      return Result::Ok(());
    }

    // evaluateaccs
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn evaluate_accs(&self,whichsol_ : i32,activity_ : & mut [f64]) -> Result<(),String>
    {
      let mut tmp_var_2__ : i64 = 0 as i64;
      let tmp_var_1__ = unsafe { MSK_getaccntot(self.ptr,& mut tmp_var_2__) };
      self.handle_res(tmp_var_1__,"getaccntot")?;
      if activity_.len() != ((tmp_var_2__) as usize) { return Result::Err("Argument 'activity_' is too short in call to 'evaluate_accs'".to_string()) }
      let call_res = unsafe { (MSK_evaluateaccs(self.ptr,whichsol_,activity_.as_mut_ptr())) };
      self.handle_res(call_res,"evaluateaccs")?;
      return Result::Ok(());
    }

    // generateconenames
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn generate_cone_names(& mut self,subk_ : & [i32],fmt_ : &str,dims_ : & [i32],sp_ : & [i64]) -> Result<(),String>
    {
      let mut num_ = subk_.len();
      let mut ndims_ = dims_.len();
      if sp_.len() != ((num_) as usize) { return Result::Err("Argument 'sp_' is too short in call to 'generate_cone_names'".to_string()) }
      let call_res = unsafe { (MSK_generateconenames(self.ptr,num_ as i32,subk_.as_ptr(),CString::new(fmt_).unwrap().as_ptr(),ndims_ as i32,dims_.as_ptr(),sp_.as_ptr())) };
      self.handle_res(call_res,"generateconenames")?;
      return Result::Ok(());
    }

    // generateconnames
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn generate_con_names(& mut self,subi_ : & [i32],fmt_ : &str,dims_ : & [i32],sp_ : & [i64]) -> Result<(),String>
    {
      let mut num_ = subi_.len();
      let mut ndims_ = dims_.len();
      if sp_.len() != ((num_) as usize) { return Result::Err("Argument 'sp_' is too short in call to 'generate_con_names'".to_string()) }
      let call_res = unsafe { (MSK_generateconnames(self.ptr,num_ as i32,subi_.as_ptr(),CString::new(fmt_).unwrap().as_ptr(),ndims_ as i32,dims_.as_ptr(),sp_.as_ptr())) };
      self.handle_res(call_res,"generateconnames")?;
      return Result::Ok(());
    }

    // generatevarnames
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn generate_var_names(& mut self,subj_ : & [i32],fmt_ : &str,dims_ : & [i32],sp_ : & [i64]) -> Result<(),String>
    {
      let mut num_ = subj_.len();
      let mut ndims_ = dims_.len();
      if sp_.len() != ((num_) as usize) { return Result::Err("Argument 'sp_' is too short in call to 'generate_var_names'".to_string()) }
      let call_res = unsafe { (MSK_generatevarnames(self.ptr,num_ as i32,subj_.as_ptr(),CString::new(fmt_).unwrap().as_ptr(),ndims_ as i32,dims_.as_ptr(),sp_.as_ptr())) };
      self.handle_res(call_res,"generatevarnames")?;
      return Result::Ok(());
    }

    // getaccafeidxlist
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_acc_afe_idx_list(& mut self,accidx_ : i64,afeidxlist_ : & mut [i64]) -> Result<(),String>
    {
      let mut tmp_var_3__ : i64 = 0 as i64;
      let tmp_var_1__ = unsafe { MSK_getaccn(self.ptr,accidx_,& mut tmp_var_3__) };
      self.handle_res(tmp_var_1__,"getaccn")?;
      if afeidxlist_.len() != ((tmp_var_3__) as usize) { return Result::Err("Argument 'afeidxlist_' is too short in call to 'get_acc_afe_idx_list'".to_string()) }
      let call_res = unsafe { (MSK_getaccafeidxlist(self.ptr,accidx_ as i64,afeidxlist_.as_mut_ptr())) };
      self.handle_res(call_res,"getaccafeidxlist")?;
      return Result::Ok(());
    }

    // getaccb
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_acc_b(& mut self,accidx_ : i64,b_ : & mut [f64]) -> Result<(),String>
    {
      let mut tmp_var_3__ : i64 = 0 as i64;
      let tmp_var_1__ = unsafe { MSK_getaccn(self.ptr,accidx_,& mut tmp_var_3__) };
      self.handle_res(tmp_var_1__,"getaccn")?;
      if b_.len() != ((tmp_var_3__) as usize) { return Result::Err("Argument 'b_' is too short in call to 'get_acc_b'".to_string()) }
      let call_res = unsafe { (MSK_getaccb(self.ptr,accidx_ as i64,b_.as_mut_ptr())) };
      self.handle_res(call_res,"getaccb")?;
      return Result::Ok(());
    }

    // getaccdomain
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_acc_domain(& mut self,accidx_ : i64) -> Result<i64,String>
    {
      let mut _ref_domidx_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getaccdomain(self.ptr,accidx_ as i64,& mut _ref_domidx_)) };
      self.handle_res(call_res,"getaccdomain")?;
      return Result::Ok((_ref_domidx_ as i64));
    }

    // getaccdoty
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_acc_dot_y(&self,whichsol_ : i32,accidx_ : i64,doty_ : & mut [f64]) -> Result<(),String>
    {
      let mut tmp_var_3__ : i64 = 0 as i64;
      let tmp_var_1__ = unsafe { MSK_getaccn(self.ptr,accidx_,& mut tmp_var_3__) };
      self.handle_res(tmp_var_1__,"getaccn")?;
      if doty_.len() != ((tmp_var_3__) as usize) { return Result::Err("Argument 'doty_' is too short in call to 'get_acc_dot_y'".to_string()) }
      let call_res = unsafe { (MSK_getaccdoty(self.ptr,whichsol_,accidx_ as i64,doty_.as_mut_ptr())) };
      self.handle_res(call_res,"getaccdoty")?;
      return Result::Ok(());
    }

    // getaccn
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_acc_n(& mut self,accidx_ : i64) -> Result<i64,String>
    {
      let mut _ref_n_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getaccn(self.ptr,accidx_ as i64,& mut _ref_n_)) };
      self.handle_res(call_res,"getaccn")?;
      return Result::Ok((_ref_n_ as i64));
    }

    // getaccname
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_acc_name(&self,accidx_ : i64) -> Result<String,String>
    {
      let mut tmp_var_5__ : i32 = 0 as i32;
      let tmp_var_3__ = unsafe { MSK_getaccnamelen(self.ptr,accidx_,& mut tmp_var_5__) };
      self.handle_res(tmp_var_3__,"getaccnamelen")?;
      let sizename_ = 1 + tmp_var_5__;
      let mut _name__bytes = Vec::with_capacity(sizename_ as usize);
      let call_res = unsafe { (MSK_getaccname(self.ptr,accidx_ as i64,sizename_ as i32,_name__bytes.as_mut_ptr())) };
      self.handle_res(call_res,"getaccname")?;
      unsafe { _name__bytes.set_len((sizename_) as usize) };
      return Result::Ok((String::from_utf8_lossy(&_name__bytes[..]).into_owned()));
    }

    // getaccnamelen
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_acc_name_len(&self,accidx_ : i64) -> Result<i32,String>
    {
      let mut _ref_len_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getaccnamelen(self.ptr,accidx_ as i64,& mut _ref_len_)) };
      self.handle_res(call_res,"getaccnamelen")?;
      return Result::Ok((_ref_len_ as i32));
    }

    // getaccntot
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_acc_n_tot(& mut self) -> Result<i64,String>
    {
      let mut _ref_n_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getaccntot(self.ptr,& mut _ref_n_)) };
      self.handle_res(call_res,"getaccntot")?;
      return Result::Ok((_ref_n_ as i64));
    }

    // getaccs
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_accs(& mut self,domidxlist_ : & mut [i64],afeidxlist_ : & mut [i64],b_ : & mut [f64]) -> Result<(),String>
    {
      let mut tmp_var_2__ : i64 = 0 as i64;
      let tmp_var_1__ = unsafe { MSK_getnumacc(self.ptr,& mut tmp_var_2__) };
      self.handle_res(tmp_var_1__,"getnumacc")?;
      if domidxlist_.len() != ((tmp_var_2__) as usize) { return Result::Err("Argument 'domidxlist_' is too short in call to 'get_accs'".to_string()) }
      let mut tmp_var_5__ : i64 = 0 as i64;
      let tmp_var_4__ = unsafe { MSK_getaccntot(self.ptr,& mut tmp_var_5__) };
      self.handle_res(tmp_var_4__,"getaccntot")?;
      if afeidxlist_.len() != ((tmp_var_5__) as usize) { return Result::Err("Argument 'afeidxlist_' is too short in call to 'get_accs'".to_string()) }
      let mut tmp_var_8__ : i64 = 0 as i64;
      let tmp_var_7__ = unsafe { MSK_getaccntot(self.ptr,& mut tmp_var_8__) };
      self.handle_res(tmp_var_7__,"getaccntot")?;
      if b_.len() != ((tmp_var_8__) as usize) { return Result::Err("Argument 'b_' is too short in call to 'get_accs'".to_string()) }
      let call_res = unsafe { (MSK_getaccs(self.ptr,domidxlist_.as_mut_ptr(),afeidxlist_.as_mut_ptr(),b_.as_mut_ptr())) };
      self.handle_res(call_res,"getaccs")?;
      return Result::Ok(());
    }

    // getacol
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_a_col(&self,j_ : i32,subj_ : & mut [i32],valj_ : & mut [f64]) -> Result<i32,String>
    {
      let mut _ref_nzj_ : i32 = 0 as i32;
      let tmp_var_1__ = self.get_a_col_num_nz(j_)?;
      if subj_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'subj_' is too short in call to 'get_a_col'".to_string()) }
      let tmp_var_4__ = self.get_a_col_num_nz(j_)?;
      if valj_.len() != ((tmp_var_4__) as usize) { return Result::Err("Argument 'valj_' is too short in call to 'get_a_col'".to_string()) }
      let call_res = unsafe { (MSK_getacol(self.ptr,j_ as i32,& mut _ref_nzj_,subj_.as_mut_ptr(),valj_.as_mut_ptr())) };
      self.handle_res(call_res,"getacol")?;
      return Result::Ok((_ref_nzj_ as i32));
    }

    // getacolnumnz
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_a_col_num_nz(&self,i_ : i32) -> Result<i32,String>
    {
      let mut _ref_nzj_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getacolnumnz(self.ptr,i_ as i32,& mut _ref_nzj_)) };
      self.handle_res(call_res,"getacolnumnz")?;
      return Result::Ok((_ref_nzj_ as i32));
    }

    // getacolslicenumnz64
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_a_col_slice_num_nz(&self,first_ : i32,last_ : i32) -> Result<i64,String>
    {
      let mut _ref_numnz_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getacolslicenumnz64(self.ptr,first_ as i32,last_ as i32,& mut _ref_numnz_)) };
      self.handle_res(call_res,"getacolslicenumnz64")?;
      return Result::Ok((_ref_numnz_ as i64));
    }

    // getafebarfnumrowentries
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_afe_barf_num_row_entries(& mut self,afeidx_ : i64) -> Result<i32,String>
    {
      let mut _ref_numentries_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getafebarfnumrowentries(self.ptr,afeidx_ as i64,& mut _ref_numentries_)) };
      self.handle_res(call_res,"getafebarfnumrowentries")?;
      return Result::Ok((_ref_numentries_ as i32));
    }

    // getafebarfrow
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_afe_barf_row(& mut self,afeidx_ : i64,barvaridx_ : & mut [i32],termptr_ : & mut [i64],numterms_ : & mut [i64],termidx_ : & mut [i64],termweight_ : & mut [f64]) -> Result<(),String>
    {
      let mut tmp_var_3__ : i32 = 0 as i32;
      let mut tmp_var_4__ : i64 = 0 as i64;
      let tmp_var_1__ = unsafe { MSK_getafebarfrowinfo(self.ptr,afeidx_,& mut tmp_var_3__,& mut tmp_var_4__) };
      self.handle_res(tmp_var_1__,"getafebarfrowinfo")?;
      if barvaridx_.len() != ((tmp_var_3__) as usize) { return Result::Err("Argument 'barvaridx_' is too short in call to 'get_afe_barf_row'".to_string()) }
      let mut tmp_var_8__ : i32 = 0 as i32;
      let mut tmp_var_9__ : i64 = 0 as i64;
      let tmp_var_6__ = unsafe { MSK_getafebarfrowinfo(self.ptr,afeidx_,& mut tmp_var_8__,& mut tmp_var_9__) };
      self.handle_res(tmp_var_6__,"getafebarfrowinfo")?;
      if termptr_.len() != ((tmp_var_8__) as usize) { return Result::Err("Argument 'termptr_' is too short in call to 'get_afe_barf_row'".to_string()) }
      let mut tmp_var_13__ : i32 = 0 as i32;
      let mut tmp_var_14__ : i64 = 0 as i64;
      let tmp_var_11__ = unsafe { MSK_getafebarfrowinfo(self.ptr,afeidx_,& mut tmp_var_13__,& mut tmp_var_14__) };
      self.handle_res(tmp_var_11__,"getafebarfrowinfo")?;
      if numterms_.len() != ((tmp_var_13__) as usize) { return Result::Err("Argument 'numterms_' is too short in call to 'get_afe_barf_row'".to_string()) }
      let mut tmp_var_18__ : i32 = 0 as i32;
      let mut tmp_var_19__ : i64 = 0 as i64;
      let tmp_var_16__ = unsafe { MSK_getafebarfrowinfo(self.ptr,afeidx_,& mut tmp_var_18__,& mut tmp_var_19__) };
      self.handle_res(tmp_var_16__,"getafebarfrowinfo")?;
      if termidx_.len() != ((tmp_var_19__) as usize) { return Result::Err("Argument 'termidx_' is too short in call to 'get_afe_barf_row'".to_string()) }
      let mut tmp_var_23__ : i32 = 0 as i32;
      let mut tmp_var_24__ : i64 = 0 as i64;
      let tmp_var_21__ = unsafe { MSK_getafebarfrowinfo(self.ptr,afeidx_,& mut tmp_var_23__,& mut tmp_var_24__) };
      self.handle_res(tmp_var_21__,"getafebarfrowinfo")?;
      if termweight_.len() != ((tmp_var_24__) as usize) { return Result::Err("Argument 'termweight_' is too short in call to 'get_afe_barf_row'".to_string()) }
      let call_res = unsafe { (MSK_getafebarfrow(self.ptr,afeidx_ as i64,barvaridx_.as_mut_ptr(),termptr_.as_mut_ptr(),numterms_.as_mut_ptr(),termidx_.as_mut_ptr(),termweight_.as_mut_ptr())) };
      self.handle_res(call_res,"getafebarfrow")?;
      return Result::Ok(());
    }

    // getafebarfrowinfo
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_afe_barf_row_info(& mut self,afeidx_ : i64) -> Result<(i32,i64),String>
    {
      let mut _ref_numentries_ : i32 = 0 as i32;
      let mut _ref_numterms_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getafebarfrowinfo(self.ptr,afeidx_ as i64,& mut _ref_numentries_,& mut _ref_numterms_)) };
      self.handle_res(call_res,"getafebarfrowinfo")?;
      return Result::Ok((_ref_numentries_ as i32,_ref_numterms_ as i64));
    }

    // getafefnumnz
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_afe_f_num_nz(& mut self) -> Result<i64,String>
    {
      let mut _ref_numfnz_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getafefnumnz(self.ptr,& mut _ref_numfnz_)) };
      self.handle_res(call_res,"getafefnumnz")?;
      return Result::Ok((_ref_numfnz_ as i64));
    }

    // getafefrow
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_afe_f_row(& mut self,afeidx_ : i64,subi_ : & mut [i32],vali_ : & mut [f64]) -> Result<i32,String>
    {
      let mut _ref_nzi_ : i32 = 0 as i32;
      let mut tmp_var_3__ : i32 = 0 as i32;
      let tmp_var_1__ = unsafe { MSK_getafefrownumnz(self.ptr,afeidx_,& mut tmp_var_3__) };
      self.handle_res(tmp_var_1__,"getafefrownumnz")?;
      if subi_.len() != ((tmp_var_3__) as usize) { return Result::Err("Argument 'subi_' is too short in call to 'get_afe_f_row'".to_string()) }
      let mut tmp_var_7__ : i32 = 0 as i32;
      let tmp_var_5__ = unsafe { MSK_getafefrownumnz(self.ptr,afeidx_,& mut tmp_var_7__) };
      self.handle_res(tmp_var_5__,"getafefrownumnz")?;
      if vali_.len() != ((tmp_var_7__) as usize) { return Result::Err("Argument 'vali_' is too short in call to 'get_afe_f_row'".to_string()) }
      let call_res = unsafe { (MSK_getafefrow(self.ptr,afeidx_ as i64,& mut _ref_nzi_,subi_.as_mut_ptr(),vali_.as_mut_ptr())) };
      self.handle_res(call_res,"getafefrow")?;
      return Result::Ok((_ref_nzi_ as i32));
    }

    // getafefrownumnz
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_afe_f_row_num_nz(& mut self,afeidx_ : i64) -> Result<i32,String>
    {
      let mut _ref_nzi_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getafefrownumnz(self.ptr,afeidx_ as i64,& mut _ref_nzi_)) };
      self.handle_res(call_res,"getafefrownumnz")?;
      return Result::Ok((_ref_nzi_ as i32));
    }

    // getafeg
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_afe_g(& mut self,afeidx_ : i64) -> Result<f64,String>
    {
      let mut _ref_g_ : f64 = 0 as f64;
      let call_res = unsafe { (MSK_getafeg(self.ptr,afeidx_ as i64,& mut _ref_g_)) };
      self.handle_res(call_res,"getafeg")?;
      return Result::Ok((_ref_g_ as f64));
    }

    // getafegslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_afe_g_slice(&self,first_ : i64,last_ : i64,g_ : & mut [f64]) -> Result<(),String>
    {
      if g_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'g_' is too short in call to 'get_afe_g_slice'".to_string()) }
      let call_res = unsafe { (MSK_getafegslice(self.ptr,first_ as i64,last_ as i64,g_.as_mut_ptr())) };
      self.handle_res(call_res,"getafegslice")?;
      return Result::Ok(());
    }

    // getaij
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_aij(&self,i_ : i32,j_ : i32) -> Result<f64,String>
    {
      let mut _ref_aij_ : f64 = 0 as f64;
      let call_res = unsafe { (MSK_getaij(self.ptr,i_ as i32,j_ as i32,& mut _ref_aij_)) };
      self.handle_res(call_res,"getaij")?;
      return Result::Ok((_ref_aij_ as f64));
    }

    // getapiecenumnz
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_a_piece_num_nz(&self,firsti_ : i32,lasti_ : i32,firstj_ : i32,lastj_ : i32) -> Result<i32,String>
    {
      let mut _ref_numnz_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getapiecenumnz(self.ptr,firsti_ as i32,lasti_ as i32,firstj_ as i32,lastj_ as i32,& mut _ref_numnz_)) };
      self.handle_res(call_res,"getapiecenumnz")?;
      return Result::Ok((_ref_numnz_ as i32));
    }

    // getarow
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_a_row(&self,i_ : i32,subi_ : & mut [i32],vali_ : & mut [f64]) -> Result<i32,String>
    {
      let mut _ref_nzi_ : i32 = 0 as i32;
      let tmp_var_1__ = self.get_a_row_num_nz(i_)?;
      if subi_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'subi_' is too short in call to 'get_a_row'".to_string()) }
      let tmp_var_4__ = self.get_a_row_num_nz(i_)?;
      if vali_.len() != ((tmp_var_4__) as usize) { return Result::Err("Argument 'vali_' is too short in call to 'get_a_row'".to_string()) }
      let call_res = unsafe { (MSK_getarow(self.ptr,i_ as i32,& mut _ref_nzi_,subi_.as_mut_ptr(),vali_.as_mut_ptr())) };
      self.handle_res(call_res,"getarow")?;
      return Result::Ok((_ref_nzi_ as i32));
    }

    // getarownumnz
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_a_row_num_nz(&self,i_ : i32) -> Result<i32,String>
    {
      let mut _ref_nzi_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getarownumnz(self.ptr,i_ as i32,& mut _ref_nzi_)) };
      self.handle_res(call_res,"getarownumnz")?;
      return Result::Ok((_ref_nzi_ as i32));
    }

    // getarowslicenumnz64
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_a_row_slice_num_nz(&self,first_ : i32,last_ : i32) -> Result<i64,String>
    {
      let mut _ref_numnz_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getarowslicenumnz64(self.ptr,first_ as i32,last_ as i32,& mut _ref_numnz_)) };
      self.handle_res(call_res,"getarowslicenumnz64")?;
      return Result::Ok((_ref_numnz_ as i64));
    }

    // getatruncatetol
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_a_truncate_tol(&self,tolzero_ : & mut [f64]) -> Result<(),String>
    {
      if tolzero_.len() != ((1) as usize) { return Result::Err("Argument 'tolzero_' is too short in call to 'get_a_truncate_tol'".to_string()) }
      let call_res = unsafe { (MSK_getatruncatetol(self.ptr,tolzero_.as_mut_ptr())) };
      self.handle_res(call_res,"getatruncatetol")?;
      return Result::Ok(());
    }

    // getbarablocktriplet
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_bara_block_triplet(&self,subi_ : & mut [i32],subj_ : & mut [i32],subk_ : & mut [i32],subl_ : & mut [i32],valijkl_ : & mut [f64]) -> Result<i64,String>
    {
      let tmp_var_1__ = self.get_num_bara_block_triplets()?;
      let maxnum_ = tmp_var_1__;
      let mut _ref_num_ : i64 = 0 as i64;
      if subi_.len() != ((maxnum_) as usize) { return Result::Err("Argument 'subi_' is too short in call to 'get_bara_block_triplet'".to_string()) }
      if subj_.len() != ((maxnum_) as usize) { return Result::Err("Argument 'subj_' is too short in call to 'get_bara_block_triplet'".to_string()) }
      if subk_.len() != ((maxnum_) as usize) { return Result::Err("Argument 'subk_' is too short in call to 'get_bara_block_triplet'".to_string()) }
      if subl_.len() != ((maxnum_) as usize) { return Result::Err("Argument 'subl_' is too short in call to 'get_bara_block_triplet'".to_string()) }
      if valijkl_.len() != ((maxnum_) as usize) { return Result::Err("Argument 'valijkl_' is too short in call to 'get_bara_block_triplet'".to_string()) }
      let call_res = unsafe { (MSK_getbarablocktriplet(self.ptr,maxnum_ as i64,& mut _ref_num_,subi_.as_mut_ptr(),subj_.as_mut_ptr(),subk_.as_mut_ptr(),subl_.as_mut_ptr(),valijkl_.as_mut_ptr())) };
      self.handle_res(call_res,"getbarablocktriplet")?;
      return Result::Ok((_ref_num_ as i64));
    }

    // getbaraidx
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_bara_idx(&self,idx_ : i64,sub_ : & mut [i64],weights_ : & mut [f64]) -> Result<(i32,i32,i64),String>
    {
      let tmp_var_1__ = self.get_bara_idx_info(idx_)?;
      let maxnum_ = tmp_var_1__;
      let mut _ref_i_ : i32 = 0 as i32;
      let mut _ref_j_ : i32 = 0 as i32;
      let mut _ref_num_ : i64 = 0 as i64;
      if sub_.len() != ((maxnum_) as usize) { return Result::Err("Argument 'sub_' is too short in call to 'get_bara_idx'".to_string()) }
      if weights_.len() != ((maxnum_) as usize) { return Result::Err("Argument 'weights_' is too short in call to 'get_bara_idx'".to_string()) }
      let call_res = unsafe { (MSK_getbaraidx(self.ptr,idx_ as i64,maxnum_ as i64,& mut _ref_i_,& mut _ref_j_,& mut _ref_num_,sub_.as_mut_ptr(),weights_.as_mut_ptr())) };
      self.handle_res(call_res,"getbaraidx")?;
      return Result::Ok((_ref_i_ as i32,_ref_j_ as i32,_ref_num_ as i64));
    }

    // getbaraidxij
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_bara_idx_i_j(&self,idx_ : i64) -> Result<(i32,i32),String>
    {
      let mut _ref_i_ : i32 = 0 as i32;
      let mut _ref_j_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getbaraidxij(self.ptr,idx_ as i64,& mut _ref_i_,& mut _ref_j_)) };
      self.handle_res(call_res,"getbaraidxij")?;
      return Result::Ok((_ref_i_ as i32,_ref_j_ as i32));
    }

    // getbaraidxinfo
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_bara_idx_info(&self,idx_ : i64) -> Result<i64,String>
    {
      let mut _ref_num_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getbaraidxinfo(self.ptr,idx_ as i64,& mut _ref_num_)) };
      self.handle_res(call_res,"getbaraidxinfo")?;
      return Result::Ok((_ref_num_ as i64));
    }

    // getbarasparsity
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_bara_sparsity(&self,idxij_ : & mut [i64]) -> Result<i64,String>
    {
      let tmp_var_1__ = self.get_num_bara_nz()?;
      let maxnumnz_ = tmp_var_1__;
      let mut _ref_numnz_ : i64 = 0 as i64;
      if idxij_.len() != ((maxnumnz_) as usize) { return Result::Err("Argument 'idxij_' is too short in call to 'get_bara_sparsity'".to_string()) }
      let call_res = unsafe { (MSK_getbarasparsity(self.ptr,maxnumnz_ as i64,& mut _ref_numnz_,idxij_.as_mut_ptr())) };
      self.handle_res(call_res,"getbarasparsity")?;
      return Result::Ok((_ref_numnz_ as i64));
    }

    // getbarcblocktriplet
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_barc_block_triplet(&self,subj_ : & mut [i32],subk_ : & mut [i32],subl_ : & mut [i32],valjkl_ : & mut [f64]) -> Result<i64,String>
    {
      let tmp_var_1__ = self.get_num_barc_block_triplets()?;
      let maxnum_ = tmp_var_1__;
      let mut _ref_num_ : i64 = 0 as i64;
      if subj_.len() != ((maxnum_) as usize) { return Result::Err("Argument 'subj_' is too short in call to 'get_barc_block_triplet'".to_string()) }
      if subk_.len() != ((maxnum_) as usize) { return Result::Err("Argument 'subk_' is too short in call to 'get_barc_block_triplet'".to_string()) }
      if subl_.len() != ((maxnum_) as usize) { return Result::Err("Argument 'subl_' is too short in call to 'get_barc_block_triplet'".to_string()) }
      if valjkl_.len() != ((maxnum_) as usize) { return Result::Err("Argument 'valjkl_' is too short in call to 'get_barc_block_triplet'".to_string()) }
      let call_res = unsafe { (MSK_getbarcblocktriplet(self.ptr,maxnum_ as i64,& mut _ref_num_,subj_.as_mut_ptr(),subk_.as_mut_ptr(),subl_.as_mut_ptr(),valjkl_.as_mut_ptr())) };
      self.handle_res(call_res,"getbarcblocktriplet")?;
      return Result::Ok((_ref_num_ as i64));
    }

    // getbarcidx
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_barc_idx(&self,idx_ : i64,sub_ : & mut [i64],weights_ : & mut [f64]) -> Result<(i32,i64),String>
    {
      let tmp_var_1__ = self.get_barc_idx_info(idx_)?;
      let maxnum_ = tmp_var_1__;
      let mut _ref_j_ : i32 = 0 as i32;
      let mut _ref_num_ : i64 = 0 as i64;
      if sub_.len() != ((maxnum_) as usize) { return Result::Err("Argument 'sub_' is too short in call to 'get_barc_idx'".to_string()) }
      if weights_.len() != ((maxnum_) as usize) { return Result::Err("Argument 'weights_' is too short in call to 'get_barc_idx'".to_string()) }
      let call_res = unsafe { (MSK_getbarcidx(self.ptr,idx_ as i64,maxnum_ as i64,& mut _ref_j_,& mut _ref_num_,sub_.as_mut_ptr(),weights_.as_mut_ptr())) };
      self.handle_res(call_res,"getbarcidx")?;
      return Result::Ok((_ref_j_ as i32,_ref_num_ as i64));
    }

    // getbarcidxinfo
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_barc_idx_info(&self,idx_ : i64) -> Result<i64,String>
    {
      let mut _ref_num_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getbarcidxinfo(self.ptr,idx_ as i64,& mut _ref_num_)) };
      self.handle_res(call_res,"getbarcidxinfo")?;
      return Result::Ok((_ref_num_ as i64));
    }

    // getbarcidxj
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_barc_idx_j(&self,idx_ : i64) -> Result<i32,String>
    {
      let mut _ref_j_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getbarcidxj(self.ptr,idx_ as i64,& mut _ref_j_)) };
      self.handle_res(call_res,"getbarcidxj")?;
      return Result::Ok((_ref_j_ as i32));
    }

    // getbarcsparsity
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_barc_sparsity(&self,idxj_ : & mut [i64]) -> Result<i64,String>
    {
      let tmp_var_1__ = self.get_num_barc_nz()?;
      let maxnumnz_ = tmp_var_1__;
      let mut _ref_numnz_ : i64 = 0 as i64;
      if idxj_.len() != ((maxnumnz_) as usize) { return Result::Err("Argument 'idxj_' is too short in call to 'get_barc_sparsity'".to_string()) }
      let call_res = unsafe { (MSK_getbarcsparsity(self.ptr,maxnumnz_ as i64,& mut _ref_numnz_,idxj_.as_mut_ptr())) };
      self.handle_res(call_res,"getbarcsparsity")?;
      return Result::Ok((_ref_numnz_ as i64));
    }

    // getbarsj
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_bars_j(&self,whichsol_ : i32,j_ : i32,barsj_ : & mut [f64]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_len_barvar_j(j_)?;
      if barsj_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'barsj_' is too short in call to 'get_bars_j'".to_string()) }
      let call_res = unsafe { (MSK_getbarsj(self.ptr,whichsol_,j_ as i32,barsj_.as_mut_ptr())) };
      self.handle_res(call_res,"getbarsj")?;
      return Result::Ok(());
    }

    // getbarsslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_bars_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,slicesize_ : i64,barsslice_ : & mut [f64]) -> Result<(),String>
    {
      if barsslice_.len() != ((slicesize_) as usize) { return Result::Err("Argument 'barsslice_' is too short in call to 'get_bars_slice'".to_string()) }
      let call_res = unsafe { (MSK_getbarsslice(self.ptr,whichsol_,first_ as i32,last_ as i32,slicesize_ as i64,barsslice_.as_mut_ptr())) };
      self.handle_res(call_res,"getbarsslice")?;
      return Result::Ok(());
    }

    // getbarvarname
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_barvar_name(&self,i_ : i32) -> Result<String,String>
    {
      let tmp_var_3__ = self.get_barvar_name_len(i_)?;
      let sizename_ = 1 + tmp_var_3__;
      let mut _name__bytes = Vec::with_capacity(sizename_ as usize);
      let call_res = unsafe { (MSK_getbarvarname(self.ptr,i_ as i32,sizename_ as i32,_name__bytes.as_mut_ptr())) };
      self.handle_res(call_res,"getbarvarname")?;
      unsafe { _name__bytes.set_len((sizename_) as usize) };
      return Result::Ok((String::from_utf8_lossy(&_name__bytes[..]).into_owned()));
    }

    // getbarvarnameindex
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_barvar_name_index(&self,somename_ : &str) -> Result<(i32,i32),String>
    {
      let mut _ref_asgn_ : i32 = 0 as i32;
      let mut _ref_index_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getbarvarnameindex(self.ptr,CString::new(somename_).unwrap().as_ptr(),& mut _ref_asgn_,& mut _ref_index_)) };
      self.handle_res(call_res,"getbarvarnameindex")?;
      return Result::Ok((_ref_asgn_ as i32,_ref_index_ as i32));
    }

    // getbarvarnamelen
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_barvar_name_len(&self,i_ : i32) -> Result<i32,String>
    {
      let mut _ref_len_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getbarvarnamelen(self.ptr,i_ as i32,& mut _ref_len_)) };
      self.handle_res(call_res,"getbarvarnamelen")?;
      return Result::Ok((_ref_len_ as i32));
    }

    // getbarxj
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_barx_j(&self,whichsol_ : i32,j_ : i32,barxj_ : & mut [f64]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_len_barvar_j(j_)?;
      if barxj_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'barxj_' is too short in call to 'get_barx_j'".to_string()) }
      let call_res = unsafe { (MSK_getbarxj(self.ptr,whichsol_,j_ as i32,barxj_.as_mut_ptr())) };
      self.handle_res(call_res,"getbarxj")?;
      return Result::Ok(());
    }

    // getbarxslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_barx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,slicesize_ : i64,barxslice_ : & mut [f64]) -> Result<(),String>
    {
      if barxslice_.len() != ((slicesize_) as usize) { return Result::Err("Argument 'barxslice_' is too short in call to 'get_barx_slice'".to_string()) }
      let call_res = unsafe { (MSK_getbarxslice(self.ptr,whichsol_,first_ as i32,last_ as i32,slicesize_ as i64,barxslice_.as_mut_ptr())) };
      self.handle_res(call_res,"getbarxslice")?;
      return Result::Ok(());
    }

    // getc
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_c(&self,c_ : & mut [f64]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_num_var()?;
      if c_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'c_' is too short in call to 'get_c'".to_string()) }
      let call_res = unsafe { (MSK_getc(self.ptr,c_.as_mut_ptr())) };
      self.handle_res(call_res,"getc")?;
      return Result::Ok(());
    }

    // getcfix
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_cfix(&self) -> Result<f64,String>
    {
      let mut _ref_cfix_ : f64 = 0 as f64;
      let call_res = unsafe { (MSK_getcfix(self.ptr,& mut _ref_cfix_)) };
      self.handle_res(call_res,"getcfix")?;
      return Result::Ok((_ref_cfix_ as f64));
    }

    // getcj
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_c_j(&self,j_ : i32) -> Result<f64,String>
    {
      let mut _ref_cj_ : f64 = 0 as f64;
      let call_res = unsafe { (MSK_getcj(self.ptr,j_ as i32,& mut _ref_cj_)) };
      self.handle_res(call_res,"getcj")?;
      return Result::Ok((_ref_cj_ as f64));
    }

    // getclist
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_c_list(&self,subj_ : & [i32],c_ : & mut [f64]) -> Result<(),String>
    {
      let mut num_ = subj_.len();
      if c_.len() != ((num_) as usize) { return Result::Err("Argument 'c_' is too short in call to 'get_c_list'".to_string()) }
      let call_res = unsafe { (MSK_getclist(self.ptr,num_ as i32,subj_.as_ptr(),c_.as_mut_ptr())) };
      self.handle_res(call_res,"getclist")?;
      return Result::Ok(());
    }

    // getconbound
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_con_bound(&self,i_ : i32) -> Result<(i32,f64,f64),String>
    {
      let mut _ref_bk_ : i32 = 0 as i32;
      let mut _ref_bl_ : f64 = 0 as f64;
      let mut _ref_bu_ : f64 = 0 as f64;
      let call_res = unsafe { (MSK_getconbound(self.ptr,i_ as i32,& mut _ref_bk_,& mut _ref_bl_,& mut _ref_bu_)) };
      self.handle_res(call_res,"getconbound")?;
      return Result::Ok((_ref_bk_ as i32,_ref_bl_ as f64,_ref_bu_ as f64));
    }

    // getconboundslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_con_bound_slice(&self,first_ : i32,last_ : i32,bk_ : & mut [i32],bl_ : & mut [f64],bu_ : & mut [f64]) -> Result<(),String>
    {
      if bk_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'bk_' is too short in call to 'get_con_bound_slice'".to_string()) }
      if bl_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'bl_' is too short in call to 'get_con_bound_slice'".to_string()) }
      if bu_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'bu_' is too short in call to 'get_con_bound_slice'".to_string()) }
      let call_res = unsafe { (MSK_getconboundslice(self.ptr,first_ as i32,last_ as i32,bk_.as_mut_ptr(),bl_.as_mut_ptr(),bu_.as_mut_ptr())) };
      self.handle_res(call_res,"getconboundslice")?;
      return Result::Ok(());
    }

    // getcone
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_cone(& mut self,k_ : i32,submem_ : & mut [i32]) -> Result<(i32,f64,i32),String>
    {
      let mut _ref_ct_ : i32 = 0 as i32;
      let mut _ref_conepar_ : f64 = 0 as f64;
      let mut _ref_nummem_ : i32 = 0 as i32;
      let tmp_var_1__ = self.get_cone_info(k_)?.2;
      if submem_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'submem_' is too short in call to 'get_cone'".to_string()) }
      let call_res = unsafe { (MSK_getcone(self.ptr,k_ as i32,& mut _ref_ct_,& mut _ref_conepar_,& mut _ref_nummem_,submem_.as_mut_ptr())) };
      self.handle_res(call_res,"getcone")?;
      return Result::Ok((_ref_ct_ as i32,_ref_conepar_ as f64,_ref_nummem_ as i32));
    }

    // getconeinfo
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_cone_info(&self,k_ : i32) -> Result<(i32,f64,i32),String>
    {
      let mut _ref_ct_ : i32 = 0 as i32;
      let mut _ref_conepar_ : f64 = 0 as f64;
      let mut _ref_nummem_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getconeinfo(self.ptr,k_ as i32,& mut _ref_ct_,& mut _ref_conepar_,& mut _ref_nummem_)) };
      self.handle_res(call_res,"getconeinfo")?;
      return Result::Ok((_ref_ct_ as i32,_ref_conepar_ as f64,_ref_nummem_ as i32));
    }

    // getconename
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_cone_name(&self,i_ : i32) -> Result<String,String>
    {
      let tmp_var_3__ = self.get_cone_name_len(i_)?;
      let sizename_ = 1 + tmp_var_3__;
      let mut _name__bytes = Vec::with_capacity(sizename_ as usize);
      let call_res = unsafe { (MSK_getconename(self.ptr,i_ as i32,sizename_ as i32,_name__bytes.as_mut_ptr())) };
      self.handle_res(call_res,"getconename")?;
      unsafe { _name__bytes.set_len((sizename_) as usize) };
      return Result::Ok((String::from_utf8_lossy(&_name__bytes[..]).into_owned()));
    }

    // getconenameindex
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_cone_name_index(&self,somename_ : &str) -> Result<(i32,i32),String>
    {
      let mut _ref_asgn_ : i32 = 0 as i32;
      let mut _ref_index_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getconenameindex(self.ptr,CString::new(somename_).unwrap().as_ptr(),& mut _ref_asgn_,& mut _ref_index_)) };
      self.handle_res(call_res,"getconenameindex")?;
      return Result::Ok((_ref_asgn_ as i32,_ref_index_ as i32));
    }

    // getconenamelen
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_cone_name_len(&self,i_ : i32) -> Result<i32,String>
    {
      let mut _ref_len_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getconenamelen(self.ptr,i_ as i32,& mut _ref_len_)) };
      self.handle_res(call_res,"getconenamelen")?;
      return Result::Ok((_ref_len_ as i32));
    }

    // getconname
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_con_name(&self,i_ : i32) -> Result<String,String>
    {
      let tmp_var_3__ = self.get_con_name_len(i_)?;
      let sizename_ = 1 + tmp_var_3__;
      let mut _name__bytes = Vec::with_capacity(sizename_ as usize);
      let call_res = unsafe { (MSK_getconname(self.ptr,i_ as i32,sizename_ as i32,_name__bytes.as_mut_ptr())) };
      self.handle_res(call_res,"getconname")?;
      unsafe { _name__bytes.set_len((sizename_) as usize) };
      return Result::Ok((String::from_utf8_lossy(&_name__bytes[..]).into_owned()));
    }

    // getconnameindex
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_con_name_index(&self,somename_ : &str) -> Result<(i32,i32),String>
    {
      let mut _ref_asgn_ : i32 = 0 as i32;
      let mut _ref_index_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getconnameindex(self.ptr,CString::new(somename_).unwrap().as_ptr(),& mut _ref_asgn_,& mut _ref_index_)) };
      self.handle_res(call_res,"getconnameindex")?;
      return Result::Ok((_ref_asgn_ as i32,_ref_index_ as i32));
    }

    // getconnamelen
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_con_name_len(&self,i_ : i32) -> Result<i32,String>
    {
      let mut _ref_len_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getconnamelen(self.ptr,i_ as i32,& mut _ref_len_)) };
      self.handle_res(call_res,"getconnamelen")?;
      return Result::Ok((_ref_len_ as i32));
    }

    // getcslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_c_slice(&self,first_ : i32,last_ : i32,c_ : & mut [f64]) -> Result<(),String>
    {
      if c_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'c_' is too short in call to 'get_c_slice'".to_string()) }
      let call_res = unsafe { (MSK_getcslice(self.ptr,first_ as i32,last_ as i32,c_.as_mut_ptr())) };
      self.handle_res(call_res,"getcslice")?;
      return Result::Ok(());
    }

    // getdimbarvarj
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_dim_barvar_j(&self,j_ : i32) -> Result<i32,String>
    {
      let mut _ref_dimbarvarj_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getdimbarvarj(self.ptr,j_ as i32,& mut _ref_dimbarvarj_)) };
      self.handle_res(call_res,"getdimbarvarj")?;
      return Result::Ok((_ref_dimbarvarj_ as i32));
    }

    // getdjcafeidxlist
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_djc_afe_idx_list(& mut self,djcidx_ : i64,afeidxlist_ : & mut [i64]) -> Result<(),String>
    {
      let mut tmp_var_3__ : i64 = 0 as i64;
      let tmp_var_1__ = unsafe { MSK_getdjcnumafe(self.ptr,djcidx_,& mut tmp_var_3__) };
      self.handle_res(tmp_var_1__,"getdjcnumafe")?;
      if afeidxlist_.len() != ((tmp_var_3__) as usize) { return Result::Err("Argument 'afeidxlist_' is too short in call to 'get_djc_afe_idx_list'".to_string()) }
      let call_res = unsafe { (MSK_getdjcafeidxlist(self.ptr,djcidx_ as i64,afeidxlist_.as_mut_ptr())) };
      self.handle_res(call_res,"getdjcafeidxlist")?;
      return Result::Ok(());
    }

    // getdjcb
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_djc_b(& mut self,djcidx_ : i64,b_ : & mut [f64]) -> Result<(),String>
    {
      let mut tmp_var_3__ : i64 = 0 as i64;
      let tmp_var_1__ = unsafe { MSK_getdjcnumafe(self.ptr,djcidx_,& mut tmp_var_3__) };
      self.handle_res(tmp_var_1__,"getdjcnumafe")?;
      if b_.len() != ((tmp_var_3__) as usize) { return Result::Err("Argument 'b_' is too short in call to 'get_djc_b'".to_string()) }
      let call_res = unsafe { (MSK_getdjcb(self.ptr,djcidx_ as i64,b_.as_mut_ptr())) };
      self.handle_res(call_res,"getdjcb")?;
      return Result::Ok(());
    }

    // getdjcdomainidxlist
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_djc_domain_idx_list(& mut self,djcidx_ : i64,domidxlist_ : & mut [i64]) -> Result<(),String>
    {
      let mut tmp_var_3__ : i64 = 0 as i64;
      let tmp_var_1__ = unsafe { MSK_getdjcnumdomain(self.ptr,djcidx_,& mut tmp_var_3__) };
      self.handle_res(tmp_var_1__,"getdjcnumdomain")?;
      if domidxlist_.len() != ((tmp_var_3__) as usize) { return Result::Err("Argument 'domidxlist_' is too short in call to 'get_djc_domain_idx_list'".to_string()) }
      let call_res = unsafe { (MSK_getdjcdomainidxlist(self.ptr,djcidx_ as i64,domidxlist_.as_mut_ptr())) };
      self.handle_res(call_res,"getdjcdomainidxlist")?;
      return Result::Ok(());
    }

    // getdjcname
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_djc_name(&self,djcidx_ : i64) -> Result<String,String>
    {
      let mut tmp_var_5__ : i32 = 0 as i32;
      let tmp_var_3__ = unsafe { MSK_getdjcnamelen(self.ptr,djcidx_,& mut tmp_var_5__) };
      self.handle_res(tmp_var_3__,"getdjcnamelen")?;
      let sizename_ = 1 + tmp_var_5__;
      let mut _name__bytes = Vec::with_capacity(sizename_ as usize);
      let call_res = unsafe { (MSK_getdjcname(self.ptr,djcidx_ as i64,sizename_ as i32,_name__bytes.as_mut_ptr())) };
      self.handle_res(call_res,"getdjcname")?;
      unsafe { _name__bytes.set_len((sizename_) as usize) };
      return Result::Ok((String::from_utf8_lossy(&_name__bytes[..]).into_owned()));
    }

    // getdjcnamelen
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_djc_name_len(&self,djcidx_ : i64) -> Result<i32,String>
    {
      let mut _ref_len_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getdjcnamelen(self.ptr,djcidx_ as i64,& mut _ref_len_)) };
      self.handle_res(call_res,"getdjcnamelen")?;
      return Result::Ok((_ref_len_ as i32));
    }

    // getdjcnumafe
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_djc_num_afe(& mut self,idjc_ : i64) -> Result<i64,String>
    {
      let mut _ref_numafe_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getdjcnumafe(self.ptr,idjc_ as i64,& mut _ref_numafe_)) };
      self.handle_res(call_res,"getdjcnumafe")?;
      return Result::Ok((_ref_numafe_ as i64));
    }

    // getdjcnumafetot
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_djc_num_afe_tot(& mut self) -> Result<i64,String>
    {
      let mut _ref_numafetot_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getdjcnumafetot(self.ptr,& mut _ref_numafetot_)) };
      self.handle_res(call_res,"getdjcnumafetot")?;
      return Result::Ok((_ref_numafetot_ as i64));
    }

    // getdjcnumdomain
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_djc_num_domain(& mut self,idjc_ : i64) -> Result<i64,String>
    {
      let mut _ref_numdomain_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getdjcnumdomain(self.ptr,idjc_ as i64,& mut _ref_numdomain_)) };
      self.handle_res(call_res,"getdjcnumdomain")?;
      return Result::Ok((_ref_numdomain_ as i64));
    }

    // getdjcnumdomaintot
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_djc_num_domain_tot(& mut self) -> Result<i64,String>
    {
      let mut _ref_numdomaintot_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getdjcnumdomaintot(self.ptr,& mut _ref_numdomaintot_)) };
      self.handle_res(call_res,"getdjcnumdomaintot")?;
      return Result::Ok((_ref_numdomaintot_ as i64));
    }

    // getdjcnumterm
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_djc_num_term(& mut self,idjc_ : i64) -> Result<i64,String>
    {
      let mut _ref_numterm_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getdjcnumterm(self.ptr,idjc_ as i64,& mut _ref_numterm_)) };
      self.handle_res(call_res,"getdjcnumterm")?;
      return Result::Ok((_ref_numterm_ as i64));
    }

    // getdjcnumtermtot
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_djc_num_term_tot(& mut self) -> Result<i64,String>
    {
      let mut _ref_numtermtot_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getdjcnumtermtot(self.ptr,& mut _ref_numtermtot_)) };
      self.handle_res(call_res,"getdjcnumtermtot")?;
      return Result::Ok((_ref_numtermtot_ as i64));
    }

    // getdjcs
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_djcs(& mut self,domidxlist_ : & mut [i64],afeidxlist_ : & mut [i64],b_ : & mut [f64],termsizelist_ : & mut [i64],numterms_ : & mut [i64]) -> Result<(),String>
    {
      let mut tmp_var_2__ : i64 = 0 as i64;
      let tmp_var_1__ = unsafe { MSK_getdjcnumdomaintot(self.ptr,& mut tmp_var_2__) };
      self.handle_res(tmp_var_1__,"getdjcnumdomaintot")?;
      if domidxlist_.len() != ((tmp_var_2__) as usize) { return Result::Err("Argument 'domidxlist_' is too short in call to 'get_djcs'".to_string()) }
      let mut tmp_var_5__ : i64 = 0 as i64;
      let tmp_var_4__ = unsafe { MSK_getdjcnumafetot(self.ptr,& mut tmp_var_5__) };
      self.handle_res(tmp_var_4__,"getdjcnumafetot")?;
      if afeidxlist_.len() != ((tmp_var_5__) as usize) { return Result::Err("Argument 'afeidxlist_' is too short in call to 'get_djcs'".to_string()) }
      let mut tmp_var_8__ : i64 = 0 as i64;
      let tmp_var_7__ = unsafe { MSK_getdjcnumafetot(self.ptr,& mut tmp_var_8__) };
      self.handle_res(tmp_var_7__,"getdjcnumafetot")?;
      if b_.len() != ((tmp_var_8__) as usize) { return Result::Err("Argument 'b_' is too short in call to 'get_djcs'".to_string()) }
      let mut tmp_var_11__ : i64 = 0 as i64;
      let tmp_var_10__ = unsafe { MSK_getdjcnumtermtot(self.ptr,& mut tmp_var_11__) };
      self.handle_res(tmp_var_10__,"getdjcnumtermtot")?;
      if termsizelist_.len() != ((tmp_var_11__) as usize) { return Result::Err("Argument 'termsizelist_' is too short in call to 'get_djcs'".to_string()) }
      let mut tmp_var_14__ : i64 = 0 as i64;
      let tmp_var_13__ = unsafe { MSK_getnumdjc(self.ptr,& mut tmp_var_14__) };
      self.handle_res(tmp_var_13__,"getnumdjc")?;
      if numterms_.len() != ((tmp_var_14__) as usize) { return Result::Err("Argument 'numterms_' is too short in call to 'get_djcs'".to_string()) }
      let call_res = unsafe { (MSK_getdjcs(self.ptr,domidxlist_.as_mut_ptr(),afeidxlist_.as_mut_ptr(),b_.as_mut_ptr(),termsizelist_.as_mut_ptr(),numterms_.as_mut_ptr())) };
      self.handle_res(call_res,"getdjcs")?;
      return Result::Ok(());
    }

    // getdjctermsizelist
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_djc_term_size_list(& mut self,djcidx_ : i64,termsizelist_ : & mut [i64]) -> Result<(),String>
    {
      let mut tmp_var_3__ : i64 = 0 as i64;
      let tmp_var_1__ = unsafe { MSK_getdjcnumterm(self.ptr,djcidx_,& mut tmp_var_3__) };
      self.handle_res(tmp_var_1__,"getdjcnumterm")?;
      if termsizelist_.len() != ((tmp_var_3__) as usize) { return Result::Err("Argument 'termsizelist_' is too short in call to 'get_djc_term_size_list'".to_string()) }
      let call_res = unsafe { (MSK_getdjctermsizelist(self.ptr,djcidx_ as i64,termsizelist_.as_mut_ptr())) };
      self.handle_res(call_res,"getdjctermsizelist")?;
      return Result::Ok(());
    }

    // getdomainn
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_domain_n(& mut self,domidx_ : i64) -> Result<i64,String>
    {
      let mut _ref_n_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getdomainn(self.ptr,domidx_ as i64,& mut _ref_n_)) };
      self.handle_res(call_res,"getdomainn")?;
      return Result::Ok((_ref_n_ as i64));
    }

    // getdomainname
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_domain_name(&self,domidx_ : i64) -> Result<String,String>
    {
      let mut tmp_var_5__ : i32 = 0 as i32;
      let tmp_var_3__ = unsafe { MSK_getdomainnamelen(self.ptr,domidx_,& mut tmp_var_5__) };
      self.handle_res(tmp_var_3__,"getdomainnamelen")?;
      let sizename_ = 1 + tmp_var_5__;
      let mut _name__bytes = Vec::with_capacity(sizename_ as usize);
      let call_res = unsafe { (MSK_getdomainname(self.ptr,domidx_ as i64,sizename_ as i32,_name__bytes.as_mut_ptr())) };
      self.handle_res(call_res,"getdomainname")?;
      unsafe { _name__bytes.set_len((sizename_) as usize) };
      return Result::Ok((String::from_utf8_lossy(&_name__bytes[..]).into_owned()));
    }

    // getdomainnamelen
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_domain_name_len(&self,domidx_ : i64) -> Result<i32,String>
    {
      let mut _ref_len_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getdomainnamelen(self.ptr,domidx_ as i64,& mut _ref_len_)) };
      self.handle_res(call_res,"getdomainnamelen")?;
      return Result::Ok((_ref_len_ as i32));
    }

    // getdomaintype
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_domain_type(& mut self,domidx_ : i64) -> Result<i32,String>
    {
      let mut _ref_domaintype_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getdomaintype(self.ptr,domidx_ as i64,& mut _ref_domaintype_)) };
      self.handle_res(call_res,"getdomaintype")?;
      return Result::Ok((_ref_domaintype_ as i32));
    }

    // getdouinf
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_dou_inf(&self,whichdinf_ : i32) -> Result<f64,String>
    {
      let mut _ref_dvalue_ : f64 = 0 as f64;
      let call_res = unsafe { (MSK_getdouinf(self.ptr,whichdinf_,& mut _ref_dvalue_)) };
      self.handle_res(call_res,"getdouinf")?;
      return Result::Ok((_ref_dvalue_ as f64));
    }

    // getdouparam
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_dou_param(&self,param_ : i32) -> Result<f64,String>
    {
      let mut _ref_parvalue_ : f64 = 0 as f64;
      let call_res = unsafe { (MSK_getdouparam(self.ptr,param_,& mut _ref_parvalue_)) };
      self.handle_res(call_res,"getdouparam")?;
      return Result::Ok((_ref_parvalue_ as f64));
    }

    // getdualobj
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_dual_obj(&self,whichsol_ : i32) -> Result<f64,String>
    {
      let mut _ref_dualobj_ : f64 = 0 as f64;
      let call_res = unsafe { (MSK_getdualobj(self.ptr,whichsol_,& mut _ref_dualobj_)) };
      self.handle_res(call_res,"getdualobj")?;
      return Result::Ok((_ref_dualobj_ as f64));
    }

    // getdualsolutionnorms
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_dual_solution_norms(&self,whichsol_ : i32) -> Result<(f64,f64,f64,f64,f64,f64,f64),String>
    {
      let mut _ref_nrmy_ : f64 = 0 as f64;
      let mut _ref_nrmslc_ : f64 = 0 as f64;
      let mut _ref_nrmsuc_ : f64 = 0 as f64;
      let mut _ref_nrmslx_ : f64 = 0 as f64;
      let mut _ref_nrmsux_ : f64 = 0 as f64;
      let mut _ref_nrmsnx_ : f64 = 0 as f64;
      let mut _ref_nrmbars_ : f64 = 0 as f64;
      let call_res = unsafe { (MSK_getdualsolutionnorms(self.ptr,whichsol_,& mut _ref_nrmy_,& mut _ref_nrmslc_,& mut _ref_nrmsuc_,& mut _ref_nrmslx_,& mut _ref_nrmsux_,& mut _ref_nrmsnx_,& mut _ref_nrmbars_)) };
      self.handle_res(call_res,"getdualsolutionnorms")?;
      return Result::Ok((_ref_nrmy_ as f64,_ref_nrmslc_ as f64,_ref_nrmsuc_ as f64,_ref_nrmslx_ as f64,_ref_nrmsux_ as f64,_ref_nrmsnx_ as f64,_ref_nrmbars_ as f64));
    }

    // getdviolacc
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_dviol_acc(&self,whichsol_ : i32,accidxlist_ : & [i64],viol_ : & mut [f64]) -> Result<(),String>
    {
      let mut numaccidx_ = accidxlist_.len();
      if viol_.len() != ((numaccidx_) as usize) { return Result::Err("Argument 'viol_' is too short in call to 'get_dviol_acc'".to_string()) }
      let call_res = unsafe { (MSK_getdviolacc(self.ptr,whichsol_,numaccidx_ as i64,accidxlist_.as_ptr(),viol_.as_mut_ptr())) };
      self.handle_res(call_res,"getdviolacc")?;
      return Result::Ok(());
    }

    // getdviolbarvar
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_dviol_barvar(&self,whichsol_ : i32,sub_ : & [i32],viol_ : & mut [f64]) -> Result<(),String>
    {
      let mut num_ = sub_.len();
      if viol_.len() != ((num_) as usize) { return Result::Err("Argument 'viol_' is too short in call to 'get_dviol_barvar'".to_string()) }
      let call_res = unsafe { (MSK_getdviolbarvar(self.ptr,whichsol_,num_ as i32,sub_.as_ptr(),viol_.as_mut_ptr())) };
      self.handle_res(call_res,"getdviolbarvar")?;
      return Result::Ok(());
    }

    // getdviolcon
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_dviol_con(&self,whichsol_ : i32,sub_ : & [i32],viol_ : & mut [f64]) -> Result<(),String>
    {
      let mut num_ = sub_.len();
      if viol_.len() != ((num_) as usize) { return Result::Err("Argument 'viol_' is too short in call to 'get_dviol_con'".to_string()) }
      let call_res = unsafe { (MSK_getdviolcon(self.ptr,whichsol_,num_ as i32,sub_.as_ptr(),viol_.as_mut_ptr())) };
      self.handle_res(call_res,"getdviolcon")?;
      return Result::Ok(());
    }

    // getdviolcones
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_dviol_cones(&self,whichsol_ : i32,sub_ : & [i32],viol_ : & mut [f64]) -> Result<(),String>
    {
      let mut num_ = sub_.len();
      if viol_.len() != ((num_) as usize) { return Result::Err("Argument 'viol_' is too short in call to 'get_dviol_cones'".to_string()) }
      let call_res = unsafe { (MSK_getdviolcones(self.ptr,whichsol_,num_ as i32,sub_.as_ptr(),viol_.as_mut_ptr())) };
      self.handle_res(call_res,"getdviolcones")?;
      return Result::Ok(());
    }

    // getdviolvar
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_dviol_var(&self,whichsol_ : i32,sub_ : & [i32],viol_ : & mut [f64]) -> Result<(),String>
    {
      let mut num_ = sub_.len();
      if viol_.len() != ((num_) as usize) { return Result::Err("Argument 'viol_' is too short in call to 'get_dviol_var'".to_string()) }
      let call_res = unsafe { (MSK_getdviolvar(self.ptr,whichsol_,num_ as i32,sub_.as_ptr(),viol_.as_mut_ptr())) };
      self.handle_res(call_res,"getdviolvar")?;
      return Result::Ok(());
    }

    // getinfindex
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_inf_index(&self,inftype_ : i32,infname_ : &str) -> Result<i32,String>
    {
      let mut _ref_infindex_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getinfindex(self.ptr,inftype_,CString::new(infname_).unwrap().as_ptr(),& mut _ref_infindex_)) };
      self.handle_res(call_res,"getinfindex")?;
      return Result::Ok((_ref_infindex_ as i32));
    }

    // getinfmax
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_inf_max(&self,inftype_ : i32,infmax_ : & mut [i32]) -> Result<(),String>
    {
      if infmax_.len() != ((MSK_MAX_STR_LEN) as usize) { return Result::Err("Argument 'infmax_' is too short in call to 'get_inf_max'".to_string()) }
      let call_res = unsafe { (MSK_getinfmax(self.ptr,inftype_,infmax_.as_mut_ptr())) };
      self.handle_res(call_res,"getinfmax")?;
      return Result::Ok(());
    }

    // getinfname
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_inf_name(&self,inftype_ : i32,whichinf_ : i32) -> Result<String,String>
    {
      let mut _infname__bytes = Vec::with_capacity(MSK_MAX_STR_LEN as usize);
      let call_res = unsafe { (MSK_getinfname(self.ptr,inftype_,whichinf_ as i32,_infname__bytes.as_mut_ptr())) };
      self.handle_res(call_res,"getinfname")?;
      unsafe { _infname__bytes.set_len((MSK_MAX_STR_LEN) as usize) };
      return Result::Ok((String::from_utf8_lossy(&_infname__bytes[..]).into_owned()));
    }

    // getintinf
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_int_inf(&self,whichiinf_ : i32) -> Result<i32,String>
    {
      let mut _ref_ivalue_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getintinf(self.ptr,whichiinf_,& mut _ref_ivalue_)) };
      self.handle_res(call_res,"getintinf")?;
      return Result::Ok((_ref_ivalue_ as i32));
    }

    // getintparam
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_int_param(&self,param_ : i32) -> Result<i32,String>
    {
      let mut _ref_parvalue_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getintparam(self.ptr,param_,& mut _ref_parvalue_)) };
      self.handle_res(call_res,"getintparam")?;
      return Result::Ok((_ref_parvalue_ as i32));
    }

    // getlenbarvarj
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_len_barvar_j(&self,j_ : i32) -> Result<i64,String>
    {
      let mut _ref_lenbarvarj_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getlenbarvarj(self.ptr,j_ as i32,& mut _ref_lenbarvarj_)) };
      self.handle_res(call_res,"getlenbarvarj")?;
      return Result::Ok((_ref_lenbarvarj_ as i64));
    }

    // getlintinf
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_lint_inf(&self,whichliinf_ : i32) -> Result<i64,String>
    {
      let mut _ref_ivalue_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getlintinf(self.ptr,whichliinf_,& mut _ref_ivalue_)) };
      self.handle_res(call_res,"getlintinf")?;
      return Result::Ok((_ref_ivalue_ as i64));
    }

    // getmaxnamelen
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_max_name_len(&self) -> Result<i32,String>
    {
      let mut _ref_maxlen_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getmaxnamelen(self.ptr,& mut _ref_maxlen_)) };
      self.handle_res(call_res,"getmaxnamelen")?;
      return Result::Ok((_ref_maxlen_ as i32));
    }

    // getmaxnumanz64
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_max_num_a_nz(&self) -> Result<i64,String>
    {
      let mut _ref_maxnumanz_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getmaxnumanz64(self.ptr,& mut _ref_maxnumanz_)) };
      self.handle_res(call_res,"getmaxnumanz64")?;
      return Result::Ok((_ref_maxnumanz_ as i64));
    }

    // getmaxnumbarvar
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_max_num_barvar(&self) -> Result<i32,String>
    {
      let mut _ref_maxnumbarvar_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getmaxnumbarvar(self.ptr,& mut _ref_maxnumbarvar_)) };
      self.handle_res(call_res,"getmaxnumbarvar")?;
      return Result::Ok((_ref_maxnumbarvar_ as i32));
    }

    // getmaxnumcon
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_max_num_con(&self) -> Result<i32,String>
    {
      let mut _ref_maxnumcon_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getmaxnumcon(self.ptr,& mut _ref_maxnumcon_)) };
      self.handle_res(call_res,"getmaxnumcon")?;
      return Result::Ok((_ref_maxnumcon_ as i32));
    }

    // getmaxnumcone
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_max_num_cone(&self) -> Result<i32,String>
    {
      let mut _ref_maxnumcone_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getmaxnumcone(self.ptr,& mut _ref_maxnumcone_)) };
      self.handle_res(call_res,"getmaxnumcone")?;
      return Result::Ok((_ref_maxnumcone_ as i32));
    }

    // getmaxnumqnz64
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_max_num_q_nz(&self) -> Result<i64,String>
    {
      let mut _ref_maxnumqnz_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getmaxnumqnz64(self.ptr,& mut _ref_maxnumqnz_)) };
      self.handle_res(call_res,"getmaxnumqnz64")?;
      return Result::Ok((_ref_maxnumqnz_ as i64));
    }

    // getmaxnumvar
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_max_num_var(&self) -> Result<i32,String>
    {
      let mut _ref_maxnumvar_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getmaxnumvar(self.ptr,& mut _ref_maxnumvar_)) };
      self.handle_res(call_res,"getmaxnumvar")?;
      return Result::Ok((_ref_maxnumvar_ as i32));
    }

    // getmemusagetask
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_mem_usage(&self) -> Result<(i64,i64),String>
    {
      let mut _ref_meminuse_ : i64 = 0 as i64;
      let mut _ref_maxmemuse_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getmemusagetask(self.ptr,& mut _ref_meminuse_,& mut _ref_maxmemuse_)) };
      self.handle_res(call_res,"getmemusagetask")?;
      return Result::Ok((_ref_meminuse_ as i64,_ref_maxmemuse_ as i64));
    }

    // getnadouinf
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_na_dou_inf(&self,infitemname_ : &str) -> Result<f64,String>
    {
      let mut _ref_dvalue_ : f64 = 0 as f64;
      let call_res = unsafe { (MSK_getnadouinf(self.ptr,CString::new(infitemname_).unwrap().as_ptr(),& mut _ref_dvalue_)) };
      self.handle_res(call_res,"getnadouinf")?;
      return Result::Ok((_ref_dvalue_ as f64));
    }

    // getnadouparam
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_na_dou_param(&self,paramname_ : &str) -> Result<f64,String>
    {
      let mut _ref_parvalue_ : f64 = 0 as f64;
      let call_res = unsafe { (MSK_getnadouparam(self.ptr,CString::new(paramname_).unwrap().as_ptr(),& mut _ref_parvalue_)) };
      self.handle_res(call_res,"getnadouparam")?;
      return Result::Ok((_ref_parvalue_ as f64));
    }

    // getnaintinf
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_na_int_inf(&self,infitemname_ : &str) -> Result<i32,String>
    {
      let mut _ref_ivalue_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getnaintinf(self.ptr,CString::new(infitemname_).unwrap().as_ptr(),& mut _ref_ivalue_)) };
      self.handle_res(call_res,"getnaintinf")?;
      return Result::Ok((_ref_ivalue_ as i32));
    }

    // getnaintparam
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_na_int_param(&self,paramname_ : &str) -> Result<i32,String>
    {
      let mut _ref_parvalue_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getnaintparam(self.ptr,CString::new(paramname_).unwrap().as_ptr(),& mut _ref_parvalue_)) };
      self.handle_res(call_res,"getnaintparam")?;
      return Result::Ok((_ref_parvalue_ as i32));
    }

    // getnastrparam
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_na_str_param(&self,paramname_ : &str,sizeparamname_ : i32) -> Result<(i32,String),String>
    {
      let mut _ref_len_ : i32 = 0 as i32;
      let mut _parvalue__bytes = Vec::with_capacity(sizeparamname_ as usize);
      let call_res = unsafe { (MSK_getnastrparam(self.ptr,CString::new(paramname_).unwrap().as_ptr(),sizeparamname_ as i32,& mut _ref_len_,_parvalue__bytes.as_mut_ptr())) };
      self.handle_res(call_res,"getnastrparam")?;
      unsafe { _parvalue__bytes.set_len((sizeparamname_) as usize) };
      return Result::Ok((_ref_len_ as i32,String::from_utf8_lossy(&_parvalue__bytes[..]).into_owned()));
    }

    // getnumacc
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_num_acc(& mut self) -> Result<i64,String>
    {
      let mut _ref_num_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getnumacc(self.ptr,& mut _ref_num_)) };
      self.handle_res(call_res,"getnumacc")?;
      return Result::Ok((_ref_num_ as i64));
    }

    // getnumafe
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_num_afe(& mut self) -> Result<i64,String>
    {
      let mut _ref_numafe_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getnumafe(self.ptr,& mut _ref_numafe_)) };
      self.handle_res(call_res,"getnumafe")?;
      return Result::Ok((_ref_numafe_ as i64));
    }

    // getnumanz
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_num_a_nz(&self) -> Result<i32,String>
    {
      let mut _ref_numanz_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getnumanz(self.ptr,& mut _ref_numanz_)) };
      self.handle_res(call_res,"getnumanz")?;
      return Result::Ok((_ref_numanz_ as i32));
    }

    // getnumanz64
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_num_a_nz_64(&self) -> Result<i64,String>
    {
      let mut _ref_numanz_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getnumanz64(self.ptr,& mut _ref_numanz_)) };
      self.handle_res(call_res,"getnumanz64")?;
      return Result::Ok((_ref_numanz_ as i64));
    }

    // getnumbarablocktriplets
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_num_bara_block_triplets(&self) -> Result<i64,String>
    {
      let mut _ref_num_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getnumbarablocktriplets(self.ptr,& mut _ref_num_)) };
      self.handle_res(call_res,"getnumbarablocktriplets")?;
      return Result::Ok((_ref_num_ as i64));
    }

    // getnumbaranz
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_num_bara_nz(&self) -> Result<i64,String>
    {
      let mut _ref_nz_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getnumbaranz(self.ptr,& mut _ref_nz_)) };
      self.handle_res(call_res,"getnumbaranz")?;
      return Result::Ok((_ref_nz_ as i64));
    }

    // getnumbarcblocktriplets
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_num_barc_block_triplets(&self) -> Result<i64,String>
    {
      let mut _ref_num_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getnumbarcblocktriplets(self.ptr,& mut _ref_num_)) };
      self.handle_res(call_res,"getnumbarcblocktriplets")?;
      return Result::Ok((_ref_num_ as i64));
    }

    // getnumbarcnz
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_num_barc_nz(&self) -> Result<i64,String>
    {
      let mut _ref_nz_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getnumbarcnz(self.ptr,& mut _ref_nz_)) };
      self.handle_res(call_res,"getnumbarcnz")?;
      return Result::Ok((_ref_nz_ as i64));
    }

    // getnumbarvar
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_num_barvar(&self) -> Result<i32,String>
    {
      let mut _ref_numbarvar_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getnumbarvar(self.ptr,& mut _ref_numbarvar_)) };
      self.handle_res(call_res,"getnumbarvar")?;
      return Result::Ok((_ref_numbarvar_ as i32));
    }

    // getnumcon
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_num_con(&self) -> Result<i32,String>
    {
      let mut _ref_numcon_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getnumcon(self.ptr,& mut _ref_numcon_)) };
      self.handle_res(call_res,"getnumcon")?;
      return Result::Ok((_ref_numcon_ as i32));
    }

    // getnumcone
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_num_cone(&self) -> Result<i32,String>
    {
      let mut _ref_numcone_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getnumcone(self.ptr,& mut _ref_numcone_)) };
      self.handle_res(call_res,"getnumcone")?;
      return Result::Ok((_ref_numcone_ as i32));
    }

    // getnumconemem
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_num_cone_mem(&self,k_ : i32) -> Result<i32,String>
    {
      let mut _ref_nummem_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getnumconemem(self.ptr,k_ as i32,& mut _ref_nummem_)) };
      self.handle_res(call_res,"getnumconemem")?;
      return Result::Ok((_ref_nummem_ as i32));
    }

    // getnumdjc
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_num_djc(& mut self) -> Result<i64,String>
    {
      let mut _ref_num_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getnumdjc(self.ptr,& mut _ref_num_)) };
      self.handle_res(call_res,"getnumdjc")?;
      return Result::Ok((_ref_num_ as i64));
    }

    // getnumdomain
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_num_domain(& mut self) -> Result<i64,String>
    {
      let mut _ref_numdomain_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getnumdomain(self.ptr,& mut _ref_numdomain_)) };
      self.handle_res(call_res,"getnumdomain")?;
      return Result::Ok((_ref_numdomain_ as i64));
    }

    // getnumintvar
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_num_int_var(&self) -> Result<i32,String>
    {
      let mut _ref_numintvar_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getnumintvar(self.ptr,& mut _ref_numintvar_)) };
      self.handle_res(call_res,"getnumintvar")?;
      return Result::Ok((_ref_numintvar_ as i32));
    }

    // getnumparam
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_num_param(&self,partype_ : i32) -> Result<i32,String>
    {
      let mut _ref_numparam_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getnumparam(self.ptr,partype_,& mut _ref_numparam_)) };
      self.handle_res(call_res,"getnumparam")?;
      return Result::Ok((_ref_numparam_ as i32));
    }

    // getnumqconknz64
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_num_q_con_k_nz(&self,k_ : i32) -> Result<i64,String>
    {
      let mut _ref_numqcnz_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getnumqconknz64(self.ptr,k_ as i32,& mut _ref_numqcnz_)) };
      self.handle_res(call_res,"getnumqconknz64")?;
      return Result::Ok((_ref_numqcnz_ as i64));
    }

    // getnumqobjnz64
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_num_q_obj_nz(&self) -> Result<i64,String>
    {
      let mut _ref_numqonz_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getnumqobjnz64(self.ptr,& mut _ref_numqonz_)) };
      self.handle_res(call_res,"getnumqobjnz64")?;
      return Result::Ok((_ref_numqonz_ as i64));
    }

    // getnumsymmat
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_num_sym_mat(&self) -> Result<i64,String>
    {
      let mut _ref_num_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getnumsymmat(self.ptr,& mut _ref_num_)) };
      self.handle_res(call_res,"getnumsymmat")?;
      return Result::Ok((_ref_num_ as i64));
    }

    // getnumvar
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_num_var(&self) -> Result<i32,String>
    {
      let mut _ref_numvar_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getnumvar(self.ptr,& mut _ref_numvar_)) };
      self.handle_res(call_res,"getnumvar")?;
      return Result::Ok((_ref_numvar_ as i32));
    }

    // getobjname
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_obj_name(&self) -> Result<String,String>
    {
      let tmp_var_3__ = self.get_obj_name_len()?;
      let sizeobjname_ = 1 + tmp_var_3__;
      let mut _objname__bytes = Vec::with_capacity(sizeobjname_ as usize);
      let call_res = unsafe { (MSK_getobjname(self.ptr,sizeobjname_ as i32,_objname__bytes.as_mut_ptr())) };
      self.handle_res(call_res,"getobjname")?;
      unsafe { _objname__bytes.set_len((sizeobjname_) as usize) };
      return Result::Ok((String::from_utf8_lossy(&_objname__bytes[..]).into_owned()));
    }

    // getobjnamelen
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_obj_name_len(&self) -> Result<i32,String>
    {
      let mut _ref_len_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getobjnamelen(self.ptr,& mut _ref_len_)) };
      self.handle_res(call_res,"getobjnamelen")?;
      return Result::Ok((_ref_len_ as i32));
    }

    // getobjsense
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_obj_sense(&self) -> Result<i32,String>
    {
      let mut _ref_sense_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getobjsense(self.ptr,& mut _ref_sense_)) };
      self.handle_res(call_res,"getobjsense")?;
      return Result::Ok((_ref_sense_ as i32));
    }

    // getparammax
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_param_max(&self,partype_ : i32) -> Result<i32,String>
    {
      let mut _ref_parammax_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getparammax(self.ptr,partype_,& mut _ref_parammax_)) };
      self.handle_res(call_res,"getparammax")?;
      return Result::Ok((_ref_parammax_ as i32));
    }

    // getparamname
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_param_name(&self,partype_ : i32,param_ : i32) -> Result<String,String>
    {
      let mut _parname__bytes = Vec::with_capacity(MSK_MAX_STR_LEN as usize);
      let call_res = unsafe { (MSK_getparamname(self.ptr,partype_,param_ as i32,_parname__bytes.as_mut_ptr())) };
      self.handle_res(call_res,"getparamname")?;
      unsafe { _parname__bytes.set_len((MSK_MAX_STR_LEN) as usize) };
      return Result::Ok((String::from_utf8_lossy(&_parname__bytes[..]).into_owned()));
    }

    // getpowerdomainalpha
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_power_domain_alpha(& mut self,domidx_ : i64,alpha_ : & mut [f64]) -> Result<(),String>
    {
      let mut tmp_var_3__ : i64 = 0 as i64;
      let mut tmp_var_4__ : i64 = 0 as i64;
      let tmp_var_1__ = unsafe { MSK_getpowerdomaininfo(self.ptr,domidx_,& mut tmp_var_3__,& mut tmp_var_4__) };
      self.handle_res(tmp_var_1__,"getpowerdomaininfo")?;
      if alpha_.len() != ((tmp_var_4__) as usize) { return Result::Err("Argument 'alpha_' is too short in call to 'get_power_domain_alpha'".to_string()) }
      let call_res = unsafe { (MSK_getpowerdomainalpha(self.ptr,domidx_ as i64,alpha_.as_mut_ptr())) };
      self.handle_res(call_res,"getpowerdomainalpha")?;
      return Result::Ok(());
    }

    // getpowerdomaininfo
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_power_domain_info(& mut self,domidx_ : i64) -> Result<(i64,i64),String>
    {
      let mut _ref_n_ : i64 = 0 as i64;
      let mut _ref_nleft_ : i64 = 0 as i64;
      let call_res = unsafe { (MSK_getpowerdomaininfo(self.ptr,domidx_ as i64,& mut _ref_n_,& mut _ref_nleft_)) };
      self.handle_res(call_res,"getpowerdomaininfo")?;
      return Result::Ok((_ref_n_ as i64,_ref_nleft_ as i64));
    }

    // getprimalobj
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_primal_obj(&self,whichsol_ : i32) -> Result<f64,String>
    {
      let mut _ref_primalobj_ : f64 = 0 as f64;
      let call_res = unsafe { (MSK_getprimalobj(self.ptr,whichsol_,& mut _ref_primalobj_)) };
      self.handle_res(call_res,"getprimalobj")?;
      return Result::Ok((_ref_primalobj_ as f64));
    }

    // getprimalsolutionnorms
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_primal_solution_norms(&self,whichsol_ : i32) -> Result<(f64,f64,f64),String>
    {
      let mut _ref_nrmxc_ : f64 = 0 as f64;
      let mut _ref_nrmxx_ : f64 = 0 as f64;
      let mut _ref_nrmbarx_ : f64 = 0 as f64;
      let call_res = unsafe { (MSK_getprimalsolutionnorms(self.ptr,whichsol_,& mut _ref_nrmxc_,& mut _ref_nrmxx_,& mut _ref_nrmbarx_)) };
      self.handle_res(call_res,"getprimalsolutionnorms")?;
      return Result::Ok((_ref_nrmxc_ as f64,_ref_nrmxx_ as f64,_ref_nrmbarx_ as f64));
    }

    // getprobtype
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_prob_type(&self) -> Result<i32,String>
    {
      let mut _ref_probtype_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getprobtype(self.ptr,& mut _ref_probtype_)) };
      self.handle_res(call_res,"getprobtype")?;
      return Result::Ok((_ref_probtype_ as i32));
    }

    // getprosta
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_pro_sta(&self,whichsol_ : i32) -> Result<i32,String>
    {
      let mut _ref_prosta_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getprosta(self.ptr,whichsol_,& mut _ref_prosta_)) };
      self.handle_res(call_res,"getprosta")?;
      return Result::Ok((_ref_prosta_ as i32));
    }

    // getpviolacc
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_pviol_acc(&self,whichsol_ : i32,accidxlist_ : & [i64],viol_ : & mut [f64]) -> Result<(),String>
    {
      let mut numaccidx_ = accidxlist_.len();
      if viol_.len() != ((numaccidx_) as usize) { return Result::Err("Argument 'viol_' is too short in call to 'get_pviol_acc'".to_string()) }
      let call_res = unsafe { (MSK_getpviolacc(self.ptr,whichsol_,numaccidx_ as i64,accidxlist_.as_ptr(),viol_.as_mut_ptr())) };
      self.handle_res(call_res,"getpviolacc")?;
      return Result::Ok(());
    }

    // getpviolbarvar
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_pviol_barvar(&self,whichsol_ : i32,sub_ : & [i32],viol_ : & mut [f64]) -> Result<(),String>
    {
      let mut num_ = sub_.len();
      if viol_.len() != ((num_) as usize) { return Result::Err("Argument 'viol_' is too short in call to 'get_pviol_barvar'".to_string()) }
      let call_res = unsafe { (MSK_getpviolbarvar(self.ptr,whichsol_,num_ as i32,sub_.as_ptr(),viol_.as_mut_ptr())) };
      self.handle_res(call_res,"getpviolbarvar")?;
      return Result::Ok(());
    }

    // getpviolcon
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_pviol_con(&self,whichsol_ : i32,sub_ : & [i32],viol_ : & mut [f64]) -> Result<(),String>
    {
      let mut num_ = sub_.len();
      if viol_.len() != ((num_) as usize) { return Result::Err("Argument 'viol_' is too short in call to 'get_pviol_con'".to_string()) }
      let call_res = unsafe { (MSK_getpviolcon(self.ptr,whichsol_,num_ as i32,sub_.as_ptr(),viol_.as_mut_ptr())) };
      self.handle_res(call_res,"getpviolcon")?;
      return Result::Ok(());
    }

    // getpviolcones
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_pviol_cones(&self,whichsol_ : i32,sub_ : & [i32],viol_ : & mut [f64]) -> Result<(),String>
    {
      let mut num_ = sub_.len();
      if viol_.len() != ((num_) as usize) { return Result::Err("Argument 'viol_' is too short in call to 'get_pviol_cones'".to_string()) }
      let call_res = unsafe { (MSK_getpviolcones(self.ptr,whichsol_,num_ as i32,sub_.as_ptr(),viol_.as_mut_ptr())) };
      self.handle_res(call_res,"getpviolcones")?;
      return Result::Ok(());
    }

    // getpvioldjc
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_pviol_djc(&self,whichsol_ : i32,djcidxlist_ : & [i64],viol_ : & mut [f64]) -> Result<(),String>
    {
      let mut numdjcidx_ = djcidxlist_.len();
      if viol_.len() != ((numdjcidx_) as usize) { return Result::Err("Argument 'viol_' is too short in call to 'get_pviol_djc'".to_string()) }
      let call_res = unsafe { (MSK_getpvioldjc(self.ptr,whichsol_,numdjcidx_ as i64,djcidxlist_.as_ptr(),viol_.as_mut_ptr())) };
      self.handle_res(call_res,"getpvioldjc")?;
      return Result::Ok(());
    }

    // getpviolvar
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_pviol_var(&self,whichsol_ : i32,sub_ : & [i32],viol_ : & mut [f64]) -> Result<(),String>
    {
      let mut num_ = sub_.len();
      if viol_.len() != ((num_) as usize) { return Result::Err("Argument 'viol_' is too short in call to 'get_pviol_var'".to_string()) }
      let call_res = unsafe { (MSK_getpviolvar(self.ptr,whichsol_,num_ as i32,sub_.as_ptr(),viol_.as_mut_ptr())) };
      self.handle_res(call_res,"getpviolvar")?;
      return Result::Ok(());
    }

    // getqobjij
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_q_obj_i_j(&self,i_ : i32,j_ : i32) -> Result<f64,String>
    {
      let mut _ref_qoij_ : f64 = 0 as f64;
      let call_res = unsafe { (MSK_getqobjij(self.ptr,i_ as i32,j_ as i32,& mut _ref_qoij_)) };
      self.handle_res(call_res,"getqobjij")?;
      return Result::Ok((_ref_qoij_ as f64));
    }

    // getreducedcosts
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_reduced_costs(&self,whichsol_ : i32,first_ : i32,last_ : i32,redcosts_ : & mut [f64]) -> Result<(),String>
    {
      if redcosts_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'redcosts_' is too short in call to 'get_reduced_costs'".to_string()) }
      let call_res = unsafe { (MSK_getreducedcosts(self.ptr,whichsol_,first_ as i32,last_ as i32,redcosts_.as_mut_ptr())) };
      self.handle_res(call_res,"getreducedcosts")?;
      return Result::Ok(());
    }

    // getskc
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_skc(&self,whichsol_ : i32,skc_ : & mut [i32]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_num_con()?;
      if skc_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'skc_' is too short in call to 'get_skc'".to_string()) }
      let call_res = unsafe { (MSK_getskc(self.ptr,whichsol_,skc_.as_mut_ptr())) };
      self.handle_res(call_res,"getskc")?;
      return Result::Ok(());
    }

    // getskcslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_skc_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,skc_ : & mut [i32]) -> Result<(),String>
    {
      if skc_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'skc_' is too short in call to 'get_skc_slice'".to_string()) }
      let call_res = unsafe { (MSK_getskcslice(self.ptr,whichsol_,first_ as i32,last_ as i32,skc_.as_mut_ptr())) };
      self.handle_res(call_res,"getskcslice")?;
      return Result::Ok(());
    }

    // getskn
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_skn(&self,whichsol_ : i32,skn_ : & mut [i32]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_num_cone()?;
      if skn_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'skn_' is too short in call to 'get_skn'".to_string()) }
      let call_res = unsafe { (MSK_getskn(self.ptr,whichsol_,skn_.as_mut_ptr())) };
      self.handle_res(call_res,"getskn")?;
      return Result::Ok(());
    }

    // getskx
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_skx(&self,whichsol_ : i32,skx_ : & mut [i32]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_num_var()?;
      if skx_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'skx_' is too short in call to 'get_skx'".to_string()) }
      let call_res = unsafe { (MSK_getskx(self.ptr,whichsol_,skx_.as_mut_ptr())) };
      self.handle_res(call_res,"getskx")?;
      return Result::Ok(());
    }

    // getskxslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_skx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,skx_ : & mut [i32]) -> Result<(),String>
    {
      if skx_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'skx_' is too short in call to 'get_skx_slice'".to_string()) }
      let call_res = unsafe { (MSK_getskxslice(self.ptr,whichsol_,first_ as i32,last_ as i32,skx_.as_mut_ptr())) };
      self.handle_res(call_res,"getskxslice")?;
      return Result::Ok(());
    }

    // getslc
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_slc(&self,whichsol_ : i32,slc_ : & mut [f64]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_num_con()?;
      if slc_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'slc_' is too short in call to 'get_slc'".to_string()) }
      let call_res = unsafe { (MSK_getslc(self.ptr,whichsol_,slc_.as_mut_ptr())) };
      self.handle_res(call_res,"getslc")?;
      return Result::Ok(());
    }

    // getslcslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_slc_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,slc_ : & mut [f64]) -> Result<(),String>
    {
      if slc_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'slc_' is too short in call to 'get_slc_slice'".to_string()) }
      let call_res = unsafe { (MSK_getslcslice(self.ptr,whichsol_,first_ as i32,last_ as i32,slc_.as_mut_ptr())) };
      self.handle_res(call_res,"getslcslice")?;
      return Result::Ok(());
    }

    // getslx
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_slx(&self,whichsol_ : i32,slx_ : & mut [f64]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_num_var()?;
      if slx_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'slx_' is too short in call to 'get_slx'".to_string()) }
      let call_res = unsafe { (MSK_getslx(self.ptr,whichsol_,slx_.as_mut_ptr())) };
      self.handle_res(call_res,"getslx")?;
      return Result::Ok(());
    }

    // getslxslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_slx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,slx_ : & mut [f64]) -> Result<(),String>
    {
      if slx_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'slx_' is too short in call to 'get_slx_slice'".to_string()) }
      let call_res = unsafe { (MSK_getslxslice(self.ptr,whichsol_,first_ as i32,last_ as i32,slx_.as_mut_ptr())) };
      self.handle_res(call_res,"getslxslice")?;
      return Result::Ok(());
    }

    // getsnx
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_snx(&self,whichsol_ : i32,snx_ : & mut [f64]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_num_var()?;
      if snx_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'snx_' is too short in call to 'get_snx'".to_string()) }
      let call_res = unsafe { (MSK_getsnx(self.ptr,whichsol_,snx_.as_mut_ptr())) };
      self.handle_res(call_res,"getsnx")?;
      return Result::Ok(());
    }

    // getsnxslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_snx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,snx_ : & mut [f64]) -> Result<(),String>
    {
      if snx_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'snx_' is too short in call to 'get_snx_slice'".to_string()) }
      let call_res = unsafe { (MSK_getsnxslice(self.ptr,whichsol_,first_ as i32,last_ as i32,snx_.as_mut_ptr())) };
      self.handle_res(call_res,"getsnxslice")?;
      return Result::Ok(());
    }

    // getsolsta
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_sol_sta(&self,whichsol_ : i32) -> Result<i32,String>
    {
      let mut _ref_solsta_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getsolsta(self.ptr,whichsol_,& mut _ref_solsta_)) };
      self.handle_res(call_res,"getsolsta")?;
      return Result::Ok((_ref_solsta_ as i32));
    }

    // getsolution
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_solution(&self,whichsol_ : i32,skc_ : & mut [i32],skx_ : & mut [i32],skn_ : & mut [i32],xc_ : & mut [f64],xx_ : & mut [f64],y_ : & mut [f64],slc_ : & mut [f64],suc_ : & mut [f64],slx_ : & mut [f64],sux_ : & mut [f64],snx_ : & mut [f64]) -> Result<(i32,i32),String>
    {
      let mut _ref_prosta_ : i32 = 0 as i32;
      let mut _ref_solsta_ : i32 = 0 as i32;
      let tmp_var_1__ = self.get_num_con()?;
      if skc_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'skc_' is too short in call to 'get_solution'".to_string()) }
      let tmp_var_3__ = self.get_num_var()?;
      if skx_.len() != ((tmp_var_3__) as usize) { return Result::Err("Argument 'skx_' is too short in call to 'get_solution'".to_string()) }
      let tmp_var_5__ = self.get_num_cone()?;
      if skn_.len() != ((tmp_var_5__) as usize) { return Result::Err("Argument 'skn_' is too short in call to 'get_solution'".to_string()) }
      let tmp_var_7__ = self.get_num_con()?;
      if xc_.len() != ((tmp_var_7__) as usize) { return Result::Err("Argument 'xc_' is too short in call to 'get_solution'".to_string()) }
      let tmp_var_9__ = self.get_num_var()?;
      if xx_.len() != ((tmp_var_9__) as usize) { return Result::Err("Argument 'xx_' is too short in call to 'get_solution'".to_string()) }
      let tmp_var_11__ = self.get_num_con()?;
      if y_.len() != ((tmp_var_11__) as usize) { return Result::Err("Argument 'y_' is too short in call to 'get_solution'".to_string()) }
      let tmp_var_13__ = self.get_num_con()?;
      if slc_.len() != ((tmp_var_13__) as usize) { return Result::Err("Argument 'slc_' is too short in call to 'get_solution'".to_string()) }
      let tmp_var_15__ = self.get_num_con()?;
      if suc_.len() != ((tmp_var_15__) as usize) { return Result::Err("Argument 'suc_' is too short in call to 'get_solution'".to_string()) }
      let tmp_var_17__ = self.get_num_var()?;
      if slx_.len() != ((tmp_var_17__) as usize) { return Result::Err("Argument 'slx_' is too short in call to 'get_solution'".to_string()) }
      let tmp_var_19__ = self.get_num_var()?;
      if sux_.len() != ((tmp_var_19__) as usize) { return Result::Err("Argument 'sux_' is too short in call to 'get_solution'".to_string()) }
      let tmp_var_21__ = self.get_num_var()?;
      if snx_.len() != ((tmp_var_21__) as usize) { return Result::Err("Argument 'snx_' is too short in call to 'get_solution'".to_string()) }
      let call_res = unsafe { (MSK_getsolution(self.ptr,whichsol_,& mut _ref_prosta_,& mut _ref_solsta_,skc_.as_mut_ptr(),skx_.as_mut_ptr(),skn_.as_mut_ptr(),xc_.as_mut_ptr(),xx_.as_mut_ptr(),y_.as_mut_ptr(),slc_.as_mut_ptr(),suc_.as_mut_ptr(),slx_.as_mut_ptr(),sux_.as_mut_ptr(),snx_.as_mut_ptr())) };
      self.handle_res(call_res,"getsolution")?;
      return Result::Ok((_ref_prosta_ as i32,_ref_solsta_ as i32));
    }

    // getsolutioninfo
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_solution_info(&self,whichsol_ : i32) -> Result<(f64,f64,f64,f64,f64,f64,f64,f64,f64,f64,f64),String>
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
      let call_res = unsafe { (MSK_getsolutioninfo(self.ptr,whichsol_,& mut _ref_pobj_,& mut _ref_pviolcon_,& mut _ref_pviolvar_,& mut _ref_pviolbarvar_,& mut _ref_pviolcone_,& mut _ref_pviolitg_,& mut _ref_dobj_,& mut _ref_dviolcon_,& mut _ref_dviolvar_,& mut _ref_dviolbarvar_,& mut _ref_dviolcone_)) };
      self.handle_res(call_res,"getsolutioninfo")?;
      return Result::Ok((_ref_pobj_ as f64,_ref_pviolcon_ as f64,_ref_pviolvar_ as f64,_ref_pviolbarvar_ as f64,_ref_pviolcone_ as f64,_ref_pviolitg_ as f64,_ref_dobj_ as f64,_ref_dviolcon_ as f64,_ref_dviolvar_ as f64,_ref_dviolbarvar_ as f64,_ref_dviolcone_ as f64));
    }

    // getsolutioninfonew
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_solution_info_new(&self,whichsol_ : i32) -> Result<(f64,f64,f64,f64,f64,f64,f64,f64,f64,f64,f64,f64,f64,f64),String>
    {
      let mut _ref_pobj_ : f64 = 0 as f64;
      let mut _ref_pviolcon_ : f64 = 0 as f64;
      let mut _ref_pviolvar_ : f64 = 0 as f64;
      let mut _ref_pviolbarvar_ : f64 = 0 as f64;
      let mut _ref_pviolcone_ : f64 = 0 as f64;
      let mut _ref_pviolacc_ : f64 = 0 as f64;
      let mut _ref_pvioldjc_ : f64 = 0 as f64;
      let mut _ref_pviolitg_ : f64 = 0 as f64;
      let mut _ref_dobj_ : f64 = 0 as f64;
      let mut _ref_dviolcon_ : f64 = 0 as f64;
      let mut _ref_dviolvar_ : f64 = 0 as f64;
      let mut _ref_dviolbarvar_ : f64 = 0 as f64;
      let mut _ref_dviolcone_ : f64 = 0 as f64;
      let mut _ref_dviolacc_ : f64 = 0 as f64;
      let call_res = unsafe { (MSK_getsolutioninfonew(self.ptr,whichsol_,& mut _ref_pobj_,& mut _ref_pviolcon_,& mut _ref_pviolvar_,& mut _ref_pviolbarvar_,& mut _ref_pviolcone_,& mut _ref_pviolacc_,& mut _ref_pvioldjc_,& mut _ref_pviolitg_,& mut _ref_dobj_,& mut _ref_dviolcon_,& mut _ref_dviolvar_,& mut _ref_dviolbarvar_,& mut _ref_dviolcone_,& mut _ref_dviolacc_)) };
      self.handle_res(call_res,"getsolutioninfonew")?;
      return Result::Ok((_ref_pobj_ as f64,_ref_pviolcon_ as f64,_ref_pviolvar_ as f64,_ref_pviolbarvar_ as f64,_ref_pviolcone_ as f64,_ref_pviolacc_ as f64,_ref_pvioldjc_ as f64,_ref_pviolitg_ as f64,_ref_dobj_ as f64,_ref_dviolcon_ as f64,_ref_dviolvar_ as f64,_ref_dviolbarvar_ as f64,_ref_dviolcone_ as f64,_ref_dviolacc_ as f64));
    }

    // getsolutionnew
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_solution_new(&self,whichsol_ : i32,skc_ : & mut [i32],skx_ : & mut [i32],skn_ : & mut [i32],xc_ : & mut [f64],xx_ : & mut [f64],y_ : & mut [f64],slc_ : & mut [f64],suc_ : & mut [f64],slx_ : & mut [f64],sux_ : & mut [f64],snx_ : & mut [f64],doty_ : & mut [f64]) -> Result<(i32,i32),String>
    {
      let mut _ref_prosta_ : i32 = 0 as i32;
      let mut _ref_solsta_ : i32 = 0 as i32;
      let tmp_var_1__ = self.get_num_con()?;
      if skc_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'skc_' is too short in call to 'get_solution_new'".to_string()) }
      let tmp_var_3__ = self.get_num_var()?;
      if skx_.len() != ((tmp_var_3__) as usize) { return Result::Err("Argument 'skx_' is too short in call to 'get_solution_new'".to_string()) }
      let tmp_var_5__ = self.get_num_cone()?;
      if skn_.len() != ((tmp_var_5__) as usize) { return Result::Err("Argument 'skn_' is too short in call to 'get_solution_new'".to_string()) }
      let tmp_var_7__ = self.get_num_con()?;
      if xc_.len() != ((tmp_var_7__) as usize) { return Result::Err("Argument 'xc_' is too short in call to 'get_solution_new'".to_string()) }
      let tmp_var_9__ = self.get_num_var()?;
      if xx_.len() != ((tmp_var_9__) as usize) { return Result::Err("Argument 'xx_' is too short in call to 'get_solution_new'".to_string()) }
      let tmp_var_11__ = self.get_num_con()?;
      if y_.len() != ((tmp_var_11__) as usize) { return Result::Err("Argument 'y_' is too short in call to 'get_solution_new'".to_string()) }
      let tmp_var_13__ = self.get_num_con()?;
      if slc_.len() != ((tmp_var_13__) as usize) { return Result::Err("Argument 'slc_' is too short in call to 'get_solution_new'".to_string()) }
      let tmp_var_15__ = self.get_num_con()?;
      if suc_.len() != ((tmp_var_15__) as usize) { return Result::Err("Argument 'suc_' is too short in call to 'get_solution_new'".to_string()) }
      let tmp_var_17__ = self.get_num_var()?;
      if slx_.len() != ((tmp_var_17__) as usize) { return Result::Err("Argument 'slx_' is too short in call to 'get_solution_new'".to_string()) }
      let tmp_var_19__ = self.get_num_var()?;
      if sux_.len() != ((tmp_var_19__) as usize) { return Result::Err("Argument 'sux_' is too short in call to 'get_solution_new'".to_string()) }
      let tmp_var_21__ = self.get_num_var()?;
      if snx_.len() != ((tmp_var_21__) as usize) { return Result::Err("Argument 'snx_' is too short in call to 'get_solution_new'".to_string()) }
      let mut tmp_var_24__ : i64 = 0 as i64;
      let tmp_var_23__ = unsafe { MSK_getaccntot(self.ptr,& mut tmp_var_24__) };
      self.handle_res(tmp_var_23__,"getaccntot")?;
      if doty_.len() != ((tmp_var_24__) as usize) { return Result::Err("Argument 'doty_' is too short in call to 'get_solution_new'".to_string()) }
      let call_res = unsafe { (MSK_getsolutionnew(self.ptr,whichsol_,& mut _ref_prosta_,& mut _ref_solsta_,skc_.as_mut_ptr(),skx_.as_mut_ptr(),skn_.as_mut_ptr(),xc_.as_mut_ptr(),xx_.as_mut_ptr(),y_.as_mut_ptr(),slc_.as_mut_ptr(),suc_.as_mut_ptr(),slx_.as_mut_ptr(),sux_.as_mut_ptr(),snx_.as_mut_ptr(),doty_.as_mut_ptr())) };
      self.handle_res(call_res,"getsolutionnew")?;
      return Result::Ok((_ref_prosta_ as i32,_ref_solsta_ as i32));
    }

    // getsolutionslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_solution_slice(&self,whichsol_ : i32,solitem_ : i32,first_ : i32,last_ : i32,values_ : & mut [f64]) -> Result<(),String>
    {
      if values_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'values_' is too short in call to 'get_solution_slice'".to_string()) }
      let call_res = unsafe { (MSK_getsolutionslice(self.ptr,whichsol_,solitem_,first_ as i32,last_ as i32,values_.as_mut_ptr())) };
      self.handle_res(call_res,"getsolutionslice")?;
      return Result::Ok(());
    }

    // getsparsesymmat
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_sparse_sym_mat(&self,idx_ : i64,subi_ : & mut [i32],subj_ : & mut [i32],valij_ : & mut [f64]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_sym_mat_info(idx_)?.1;
      let maxlen_ = tmp_var_1__;
      if subi_.len() != ((maxlen_) as usize) { return Result::Err("Argument 'subi_' is too short in call to 'get_sparse_sym_mat'".to_string()) }
      if subj_.len() != ((maxlen_) as usize) { return Result::Err("Argument 'subj_' is too short in call to 'get_sparse_sym_mat'".to_string()) }
      if valij_.len() != ((maxlen_) as usize) { return Result::Err("Argument 'valij_' is too short in call to 'get_sparse_sym_mat'".to_string()) }
      let call_res = unsafe { (MSK_getsparsesymmat(self.ptr,idx_ as i64,maxlen_ as i64,subi_.as_mut_ptr(),subj_.as_mut_ptr(),valij_.as_mut_ptr())) };
      self.handle_res(call_res,"getsparsesymmat")?;
      return Result::Ok(());
    }

    // getstrparam
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_str_param(&self,param_ : i32) -> Result<(i32,String),String>
    {
      let tmp_var_3__ = self.get_str_param_len(param_)?;
      let maxlen_ = 1 + tmp_var_3__;
      let mut _ref_len_ : i32 = 0 as i32;
      let mut _parvalue__bytes = Vec::with_capacity(maxlen_ as usize);
      let call_res = unsafe { (MSK_getstrparam(self.ptr,param_,maxlen_ as i32,& mut _ref_len_,_parvalue__bytes.as_mut_ptr())) };
      self.handle_res(call_res,"getstrparam")?;
      unsafe { _parvalue__bytes.set_len((maxlen_) as usize) };
      return Result::Ok((_ref_len_ as i32,String::from_utf8_lossy(&_parvalue__bytes[..]).into_owned()));
    }

    // getstrparamlen
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_str_param_len(&self,param_ : i32) -> Result<i32,String>
    {
      let mut _ref_len_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getstrparamlen(self.ptr,param_,& mut _ref_len_)) };
      self.handle_res(call_res,"getstrparamlen")?;
      return Result::Ok((_ref_len_ as i32));
    }

    // getsuc
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_suc(&self,whichsol_ : i32,suc_ : & mut [f64]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_num_con()?;
      if suc_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'suc_' is too short in call to 'get_suc'".to_string()) }
      let call_res = unsafe { (MSK_getsuc(self.ptr,whichsol_,suc_.as_mut_ptr())) };
      self.handle_res(call_res,"getsuc")?;
      return Result::Ok(());
    }

    // getsucslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_suc_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,suc_ : & mut [f64]) -> Result<(),String>
    {
      if suc_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'suc_' is too short in call to 'get_suc_slice'".to_string()) }
      let call_res = unsafe { (MSK_getsucslice(self.ptr,whichsol_,first_ as i32,last_ as i32,suc_.as_mut_ptr())) };
      self.handle_res(call_res,"getsucslice")?;
      return Result::Ok(());
    }

    // getsux
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_sux(&self,whichsol_ : i32,sux_ : & mut [f64]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_num_var()?;
      if sux_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'sux_' is too short in call to 'get_sux'".to_string()) }
      let call_res = unsafe { (MSK_getsux(self.ptr,whichsol_,sux_.as_mut_ptr())) };
      self.handle_res(call_res,"getsux")?;
      return Result::Ok(());
    }

    // getsuxslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_sux_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,sux_ : & mut [f64]) -> Result<(),String>
    {
      if sux_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'sux_' is too short in call to 'get_sux_slice'".to_string()) }
      let call_res = unsafe { (MSK_getsuxslice(self.ptr,whichsol_,first_ as i32,last_ as i32,sux_.as_mut_ptr())) };
      self.handle_res(call_res,"getsuxslice")?;
      return Result::Ok(());
    }

    // getsymbcon
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_symb_con(&self,i_ : i32) -> Result<(String,i32),String>
    {
      let sizevalue_ = MSK_MAX_STR_LEN;
      let mut _name__bytes = Vec::with_capacity(MSK_MAX_STR_LEN as usize);
      let mut _ref_value_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getsymbcon(self.ptr,i_ as i32,sizevalue_ as i32,_name__bytes.as_mut_ptr(),& mut _ref_value_)) };
      self.handle_res(call_res,"getsymbcon")?;
      unsafe { _name__bytes.set_len((MSK_MAX_STR_LEN) as usize) };
      return Result::Ok((String::from_utf8_lossy(&_name__bytes[..]).into_owned(),_ref_value_ as i32));
    }

    // getsymmatinfo
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_sym_mat_info(&self,idx_ : i64) -> Result<(i32,i64,i32),String>
    {
      let mut _ref_dim_ : i32 = 0 as i32;
      let mut _ref_nz_ : i64 = 0 as i64;
      let mut _ref_type_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getsymmatinfo(self.ptr,idx_ as i64,& mut _ref_dim_,& mut _ref_nz_,& mut _ref_type_)) };
      self.handle_res(call_res,"getsymmatinfo")?;
      return Result::Ok((_ref_dim_ as i32,_ref_nz_ as i64,_ref_type_ as i32));
    }

    // gettaskname
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_task_name(&self) -> Result<String,String>
    {
      let tmp_var_3__ = self.get_task_name_len()?;
      let sizetaskname_ = 1 + tmp_var_3__;
      let mut _taskname__bytes = Vec::with_capacity(sizetaskname_ as usize);
      let call_res = unsafe { (MSK_gettaskname(self.ptr,sizetaskname_ as i32,_taskname__bytes.as_mut_ptr())) };
      self.handle_res(call_res,"gettaskname")?;
      unsafe { _taskname__bytes.set_len((sizetaskname_) as usize) };
      return Result::Ok((String::from_utf8_lossy(&_taskname__bytes[..]).into_owned()));
    }

    // gettasknamelen
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_task_name_len(&self) -> Result<i32,String>
    {
      let mut _ref_len_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_gettasknamelen(self.ptr,& mut _ref_len_)) };
      self.handle_res(call_res,"gettasknamelen")?;
      return Result::Ok((_ref_len_ as i32));
    }

    // getvarbound
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_var_bound(&self,i_ : i32) -> Result<(i32,f64,f64),String>
    {
      let mut _ref_bk_ : i32 = 0 as i32;
      let mut _ref_bl_ : f64 = 0 as f64;
      let mut _ref_bu_ : f64 = 0 as f64;
      let call_res = unsafe { (MSK_getvarbound(self.ptr,i_ as i32,& mut _ref_bk_,& mut _ref_bl_,& mut _ref_bu_)) };
      self.handle_res(call_res,"getvarbound")?;
      return Result::Ok((_ref_bk_ as i32,_ref_bl_ as f64,_ref_bu_ as f64));
    }

    // getvarboundslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_var_bound_slice(&self,first_ : i32,last_ : i32,bk_ : & mut [i32],bl_ : & mut [f64],bu_ : & mut [f64]) -> Result<(),String>
    {
      if bk_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'bk_' is too short in call to 'get_var_bound_slice'".to_string()) }
      if bl_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'bl_' is too short in call to 'get_var_bound_slice'".to_string()) }
      if bu_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'bu_' is too short in call to 'get_var_bound_slice'".to_string()) }
      let call_res = unsafe { (MSK_getvarboundslice(self.ptr,first_ as i32,last_ as i32,bk_.as_mut_ptr(),bl_.as_mut_ptr(),bu_.as_mut_ptr())) };
      self.handle_res(call_res,"getvarboundslice")?;
      return Result::Ok(());
    }

    // getvarname
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_var_name(&self,j_ : i32) -> Result<String,String>
    {
      let tmp_var_3__ = self.get_var_name_len(j_)?;
      let sizename_ = 1 + tmp_var_3__;
      let mut _name__bytes = Vec::with_capacity(sizename_ as usize);
      let call_res = unsafe { (MSK_getvarname(self.ptr,j_ as i32,sizename_ as i32,_name__bytes.as_mut_ptr())) };
      self.handle_res(call_res,"getvarname")?;
      unsafe { _name__bytes.set_len((sizename_) as usize) };
      return Result::Ok((String::from_utf8_lossy(&_name__bytes[..]).into_owned()));
    }

    // getvarnameindex
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_var_name_index(&self,somename_ : &str) -> Result<(i32,i32),String>
    {
      let mut _ref_asgn_ : i32 = 0 as i32;
      let mut _ref_index_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getvarnameindex(self.ptr,CString::new(somename_).unwrap().as_ptr(),& mut _ref_asgn_,& mut _ref_index_)) };
      self.handle_res(call_res,"getvarnameindex")?;
      return Result::Ok((_ref_asgn_ as i32,_ref_index_ as i32));
    }

    // getvarnamelen
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_var_name_len(&self,i_ : i32) -> Result<i32,String>
    {
      let mut _ref_len_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getvarnamelen(self.ptr,i_ as i32,& mut _ref_len_)) };
      self.handle_res(call_res,"getvarnamelen")?;
      return Result::Ok((_ref_len_ as i32));
    }

    // getvartype
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_var_type(&self,j_ : i32) -> Result<i32,String>
    {
      let mut _ref_vartype_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_getvartype(self.ptr,j_ as i32,& mut _ref_vartype_)) };
      self.handle_res(call_res,"getvartype")?;
      return Result::Ok((_ref_vartype_ as i32));
    }

    // getvartypelist
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_var_type_list(&self,subj_ : & [i32],vartype_ : & mut [i32]) -> Result<(),String>
    {
      let mut num_ = subj_.len();
      if vartype_.len() != ((num_) as usize) { return Result::Err("Argument 'vartype_' is too short in call to 'get_var_type_list'".to_string()) }
      let call_res = unsafe { (MSK_getvartypelist(self.ptr,num_ as i32,subj_.as_ptr(),vartype_.as_mut_ptr())) };
      self.handle_res(call_res,"getvartypelist")?;
      return Result::Ok(());
    }

    // getxc
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_xc(&self,whichsol_ : i32,xc_ : & mut [f64]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_num_con()?;
      if xc_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'xc_' is too short in call to 'get_xc'".to_string()) }
      let call_res = unsafe { (MSK_getxc(self.ptr,whichsol_,xc_.as_mut_ptr())) };
      self.handle_res(call_res,"getxc")?;
      return Result::Ok(());
    }

    // getxcslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_xc_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,xc_ : & mut [f64]) -> Result<(),String>
    {
      if xc_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'xc_' is too short in call to 'get_xc_slice'".to_string()) }
      let call_res = unsafe { (MSK_getxcslice(self.ptr,whichsol_,first_ as i32,last_ as i32,xc_.as_mut_ptr())) };
      self.handle_res(call_res,"getxcslice")?;
      return Result::Ok(());
    }

    // getxx
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_xx(&self,whichsol_ : i32,xx_ : & mut [f64]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_num_var()?;
      if xx_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'xx_' is too short in call to 'get_xx'".to_string()) }
      let call_res = unsafe { (MSK_getxx(self.ptr,whichsol_,xx_.as_mut_ptr())) };
      self.handle_res(call_res,"getxx")?;
      return Result::Ok(());
    }

    // getxxslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_xx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,xx_ : & mut [f64]) -> Result<(),String>
    {
      if xx_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'xx_' is too short in call to 'get_xx_slice'".to_string()) }
      let call_res = unsafe { (MSK_getxxslice(self.ptr,whichsol_,first_ as i32,last_ as i32,xx_.as_mut_ptr())) };
      self.handle_res(call_res,"getxxslice")?;
      return Result::Ok(());
    }

    // gety
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_y(&self,whichsol_ : i32,y_ : & mut [f64]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_num_con()?;
      if y_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'y_' is too short in call to 'get_y'".to_string()) }
      let call_res = unsafe { (MSK_gety(self.ptr,whichsol_,y_.as_mut_ptr())) };
      self.handle_res(call_res,"gety")?;
      return Result::Ok(());
    }

    // getyslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn get_y_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,y_ : & mut [f64]) -> Result<(),String>
    {
      if y_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'y_' is too short in call to 'get_y_slice'".to_string()) }
      let call_res = unsafe { (MSK_getyslice(self.ptr,whichsol_,first_ as i32,last_ as i32,y_.as_mut_ptr())) };
      self.handle_res(call_res,"getyslice")?;
      return Result::Ok(());
    }

    // initbasissolve
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn init_basis_solve(& mut self,basis_ : & mut [i32]) -> Result<(),String>
    {
      let mut tmp_var_2__ : i32 = 0 as i32;
      let tmp_var_1__ = unsafe { MSK_getnumcon(self.ptr,& mut tmp_var_2__) };
      self.handle_res(tmp_var_1__,"getnumcon")?;
      if basis_.len() != ((tmp_var_2__) as usize) { return Result::Err("Argument 'basis_' is too short in call to 'init_basis_solve'".to_string()) }
      let call_res = unsafe { (MSK_initbasissolve(self.ptr,basis_.as_mut_ptr())) };
      self.handle_res(call_res,"initbasissolve")?;
      return Result::Ok(());
    }

    // inputdata64
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn input_data(& mut self,maxnumcon_ : i32,maxnumvar_ : i32,c_ : & [f64],cfix_ : f64,aptrb_ : & [i64],aptre_ : & [i64],asub_ : & [i32],aval_ : & [f64],bkc_ : & [i32],blc_ : & [f64],buc_ : & [f64],bkx_ : & [i32],blx_ : & [f64],bux_ : & [f64]) -> Result<(),String>
    {
      let mut numcon_ = buc_.len();
      if blc_.len() < numcon_ { numcon_ = blc_.len() };
      if bkc_.len() < numcon_ { numcon_ = bkc_.len() };
      let mut numvar_ = c_.len();
      if bux_.len() < numvar_ { numvar_ = bux_.len() };
      if blx_.len() < numvar_ { numvar_ = blx_.len() };
      if bkx_.len() < numvar_ { numvar_ = bkx_.len() };
      if aptrb_.len() < numvar_ { numvar_ = aptrb_.len() };
      if aptre_.len() < numvar_ { numvar_ = aptre_.len() };
      if ! aptrb_.iter().all(|i| *i >= 0 && *i <= asub_.len() as i64 && *i <= aval_.len() as i64) { return Err("Invalid index in argument aptrb".to_string()) }
      if ! aptre_.iter().all(|i| *i >= 0 && *i <= asub_.len() as i64 && *i <= aval_.len() as i64) { return Err("Invalid index in argument aptre".to_string()) }
      let call_res = unsafe { (MSK_inputdata64(self.ptr,maxnumcon_ as i32,maxnumvar_ as i32,numcon_ as i32,numvar_ as i32,c_.as_ptr(),cfix_ as f64,aptrb_.as_ptr(),aptre_.as_ptr(),asub_.as_ptr(),aval_.as_ptr(),bkc_.as_ptr(),blc_.as_ptr(),buc_.as_ptr(),bkx_.as_ptr(),blx_.as_ptr(),bux_.as_ptr())) };
      self.handle_res(call_res,"inputdata64")?;
      return Result::Ok(());
    }

    // isdouparname
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn is_dou_par_name(&self,parname_ : &str) -> Result<i32,String>
    {
      let mut _ref_param_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_isdouparname(self.ptr,CString::new(parname_).unwrap().as_ptr(),& mut _ref_param_)) };
      self.handle_res(call_res,"isdouparname")?;
      return Result::Ok((_ref_param_ as i32));
    }

    // isintparname
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn is_int_par_name(&self,parname_ : &str) -> Result<i32,String>
    {
      let mut _ref_param_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_isintparname(self.ptr,CString::new(parname_).unwrap().as_ptr(),& mut _ref_param_)) };
      self.handle_res(call_res,"isintparname")?;
      return Result::Ok((_ref_param_ as i32));
    }

    // isstrparname
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn is_str_par_name(&self,parname_ : &str) -> Result<i32,String>
    {
      let mut _ref_param_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_isstrparname(self.ptr,CString::new(parname_).unwrap().as_ptr(),& mut _ref_param_)) };
      self.handle_res(call_res,"isstrparname")?;
      return Result::Ok((_ref_param_ as i32));
    }

    // linkfiletotaskstream
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn link_file_to_stream(& mut self,whichstream_ : i32,filename_ : &str,append_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_linkfiletotaskstream(self.ptr,whichstream_,CString::new(filename_).unwrap().as_ptr(),append_ as i32)) };
      self.handle_res(call_res,"linkfiletotaskstream")?;
      return Result::Ok(());
    }

    // onesolutionsummary
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn one_solution_summary(&self,whichstream_ : i32,whichsol_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_onesolutionsummary(self.ptr,whichstream_,whichsol_)) };
      self.handle_res(call_res,"onesolutionsummary")?;
      return Result::Ok(());
    }

    // optimizermt
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn optimize_rmt(& mut self,addr_ : &str,accesstoken_ : &str) -> Result<i32,String>
    {
      let mut _ref_trmcode_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_optimizermt(self.ptr,CString::new(addr_).unwrap().as_ptr(),CString::new(accesstoken_).unwrap().as_ptr(),& mut _ref_trmcode_)) };
      self.handle_res(call_res,"optimizermt")?;
      return Result::Ok((_ref_trmcode_ as i32));
    }

    // optimizersummary
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn optimizer_summary(&self,whichstream_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_optimizersummary(self.ptr,whichstream_)) };
      self.handle_res(call_res,"optimizersummary")?;
      return Result::Ok(());
    }

    // optimizetrm
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn optimize(& mut self) -> Result<i32,String>
    {
      let mut _ref_trmcode_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_optimizetrm(self.ptr,& mut _ref_trmcode_)) };
      self.handle_res(call_res,"optimizetrm")?;
      return Result::Ok((_ref_trmcode_ as i32));
    }

    // primalrepair
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn primal_repair(& mut self,wlc_ : & [f64],wuc_ : & [f64],wlx_ : & [f64],wux_ : & [f64]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_num_con()?;
      if wlc_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'wlc_' is too short in call to 'primal_repair'".to_string()) }
      let tmp_var_3__ = self.get_num_con()?;
      if wuc_.len() != ((tmp_var_3__) as usize) { return Result::Err("Argument 'wuc_' is too short in call to 'primal_repair'".to_string()) }
      let tmp_var_5__ = self.get_num_var()?;
      if wlx_.len() != ((tmp_var_5__) as usize) { return Result::Err("Argument 'wlx_' is too short in call to 'primal_repair'".to_string()) }
      let tmp_var_7__ = self.get_num_var()?;
      if wux_.len() != ((tmp_var_7__) as usize) { return Result::Err("Argument 'wux_' is too short in call to 'primal_repair'".to_string()) }
      let call_res = unsafe { (MSK_primalrepair(self.ptr,wlc_.as_ptr(),wuc_.as_ptr(),wlx_.as_ptr(),wux_.as_ptr())) };
      self.handle_res(call_res,"primalrepair")?;
      return Result::Ok(());
    }

    // primalsensitivity
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn primal_sensitivity(& mut self,subi_ : & [i32],marki_ : & [i32],subj_ : & [i32],markj_ : & [i32],leftpricei_ : & mut [f64],rightpricei_ : & mut [f64],leftrangei_ : & mut [f64],rightrangei_ : & mut [f64],leftpricej_ : & mut [f64],rightpricej_ : & mut [f64],leftrangej_ : & mut [f64],rightrangej_ : & mut [f64]) -> Result<(),String>
    {
      let mut numi_ = subi_.len();
      if marki_.len() < numi_ { numi_ = marki_.len() };
      let mut numj_ = subj_.len();
      if markj_.len() < numj_ { numj_ = markj_.len() };
      if leftpricei_.len() != ((numi_) as usize) { return Result::Err("Argument 'leftpricei_' is too short in call to 'primal_sensitivity'".to_string()) }
      if rightpricei_.len() != ((numi_) as usize) { return Result::Err("Argument 'rightpricei_' is too short in call to 'primal_sensitivity'".to_string()) }
      if leftrangei_.len() != ((numi_) as usize) { return Result::Err("Argument 'leftrangei_' is too short in call to 'primal_sensitivity'".to_string()) }
      if rightrangei_.len() != ((numi_) as usize) { return Result::Err("Argument 'rightrangei_' is too short in call to 'primal_sensitivity'".to_string()) }
      if leftpricej_.len() != ((numj_) as usize) { return Result::Err("Argument 'leftpricej_' is too short in call to 'primal_sensitivity'".to_string()) }
      if rightpricej_.len() != ((numj_) as usize) { return Result::Err("Argument 'rightpricej_' is too short in call to 'primal_sensitivity'".to_string()) }
      if leftrangej_.len() != ((numj_) as usize) { return Result::Err("Argument 'leftrangej_' is too short in call to 'primal_sensitivity'".to_string()) }
      if rightrangej_.len() != ((numj_) as usize) { return Result::Err("Argument 'rightrangej_' is too short in call to 'primal_sensitivity'".to_string()) }
      let call_res = unsafe { (MSK_primalsensitivity(self.ptr,numi_ as i32,subi_.as_ptr(),marki_.as_ptr(),numj_ as i32,subj_.as_ptr(),markj_.as_ptr(),leftpricei_.as_mut_ptr(),rightpricei_.as_mut_ptr(),leftrangei_.as_mut_ptr(),rightrangei_.as_mut_ptr(),leftpricej_.as_mut_ptr(),rightpricej_.as_mut_ptr(),leftrangej_.as_mut_ptr(),rightrangej_.as_mut_ptr())) };
      self.handle_res(call_res,"primalsensitivity")?;
      return Result::Ok(());
    }

    // printparam
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn print_param(&self) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_printparam(self.ptr)) };
      self.handle_res(call_res,"printparam")?;
      return Result::Ok(());
    }

    // probtypetostr
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn prob_type_to_str(& mut self,probtype_ : i32) -> Result<String,String>
    {
      let mut _str__bytes = Vec::with_capacity(MSK_MAX_STR_LEN as usize);
      let call_res = unsafe { (MSK_probtypetostr(self.ptr,probtype_,_str__bytes.as_mut_ptr())) };
      self.handle_res(call_res,"probtypetostr")?;
      unsafe { _str__bytes.set_len((MSK_MAX_STR_LEN) as usize) };
      return Result::Ok((String::from_utf8_lossy(&_str__bytes[..]).into_owned()));
    }

    // prostatostr
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn pro_sta_to_str(&self,prosta_ : i32) -> Result<String,String>
    {
      let mut _str__bytes = Vec::with_capacity(MSK_MAX_STR_LEN as usize);
      let call_res = unsafe { (MSK_prostatostr(self.ptr,prosta_,_str__bytes.as_mut_ptr())) };
      self.handle_res(call_res,"prostatostr")?;
      unsafe { _str__bytes.set_len((MSK_MAX_STR_LEN) as usize) };
      return Result::Ok((String::from_utf8_lossy(&_str__bytes[..]).into_owned()));
    }

    // putacc
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_acc(& mut self,accidx_ : i64,domidx_ : i64,afeidxlist_ : & [i64],b_ : & [f64]) -> Result<(),String>
    {
      let mut numafeidx_ = afeidxlist_.len();
      if b_.len() != ((numafeidx_) as usize) { return Result::Err("Argument 'b_' is too short in call to 'put_acc'".to_string()) }
      let call_res = unsafe { (MSK_putacc(self.ptr,accidx_ as i64,domidx_ as i64,numafeidx_ as i64,afeidxlist_.as_ptr(),b_.as_ptr())) };
      self.handle_res(call_res,"putacc")?;
      return Result::Ok(());
    }

    // putaccb
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_acc_b(& mut self,accidx_ : i64,b_ : & [f64]) -> Result<(),String>
    {
      let mut lengthb_ = b_.len();
      let call_res = unsafe { (MSK_putaccb(self.ptr,accidx_ as i64,lengthb_ as i64,b_.as_ptr())) };
      self.handle_res(call_res,"putaccb")?;
      return Result::Ok(());
    }

    // putaccbj
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_acc_b_j(& mut self,accidx_ : i64,j_ : i64,bj_ : f64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putaccbj(self.ptr,accidx_ as i64,j_ as i64,bj_ as f64)) };
      self.handle_res(call_res,"putaccbj")?;
      return Result::Ok(());
    }

    // putaccdoty
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_acc_dot_y(&self,whichsol_ : i32,accidx_ : i64,doty_ : & mut [f64]) -> Result<(),String>
    {
      let mut tmp_var_3__ : i64 = 0 as i64;
      let tmp_var_1__ = unsafe { MSK_getaccn(self.ptr,accidx_,& mut tmp_var_3__) };
      self.handle_res(tmp_var_1__,"getaccn")?;
      if doty_.len() != ((tmp_var_3__) as usize) { return Result::Err("Argument 'doty_' is too short in call to 'put_acc_dot_y'".to_string()) }
      let call_res = unsafe { (MSK_putaccdoty(self.ptr,whichsol_,accidx_ as i64,doty_.as_mut_ptr())) };
      self.handle_res(call_res,"putaccdoty")?;
      return Result::Ok(());
    }

    // putacclist
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_acc_list(& mut self,accidxs_ : & [i64],domidxs_ : & [i64],afeidxlist_ : & [i64],b_ : & [f64]) -> Result<(),String>
    {
      let mut numaccs_ = domidxs_.len();
      if accidxs_.len() < numaccs_ { numaccs_ = accidxs_.len() };
      let mut numafeidx_ = afeidxlist_.len();
      if b_.len() != ((numafeidx_) as usize) { return Result::Err("Argument 'b_' is too short in call to 'put_acc_list'".to_string()) }
      let call_res = unsafe { (MSK_putacclist(self.ptr,numaccs_ as i64,accidxs_.as_ptr(),domidxs_.as_ptr(),numafeidx_ as i64,afeidxlist_.as_ptr(),b_.as_ptr())) };
      self.handle_res(call_res,"putacclist")?;
      return Result::Ok(());
    }

    // putaccname
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_acc_name(& mut self,accidx_ : i64,name_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putaccname(self.ptr,accidx_ as i64,CString::new(name_).unwrap().as_ptr())) };
      self.handle_res(call_res,"putaccname")?;
      return Result::Ok(());
    }

    // putacol
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_a_col(& mut self,j_ : i32,subj_ : & [i32],valj_ : & [f64]) -> Result<(),String>
    {
      let mut nzj_ = subj_.len();
      if valj_.len() < nzj_ { nzj_ = valj_.len() };
      let call_res = unsafe { (MSK_putacol(self.ptr,j_ as i32,nzj_ as i32,subj_.as_ptr(),valj_.as_ptr())) };
      self.handle_res(call_res,"putacol")?;
      return Result::Ok(());
    }

    // putacollist64
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_a_col_list(& mut self,sub_ : & [i32],ptrb_ : & [i64],ptre_ : & [i64],asub_ : & [i32],aval_ : & [f64]) -> Result<(),String>
    {
      let mut num_ = sub_.len();
      if ptrb_.len() < num_ { num_ = ptrb_.len() };
      if ptre_.len() < num_ { num_ = ptre_.len() };
      if ! ptrb_.iter().all(|i| *i >= 0 && *i <= asub_.len() as i64 && *i <= aval_.len() as i64) { return Err("Invalid index in argument ptrb".to_string()) }
      if ! ptre_.iter().all(|i| *i >= 0 && *i <= asub_.len() as i64 && *i <= aval_.len() as i64) { return Err("Invalid index in argument ptre".to_string()) }
      let call_res = unsafe { (MSK_putacollist64(self.ptr,num_ as i32,sub_.as_ptr(),ptrb_.as_ptr(),ptre_.as_ptr(),asub_.as_ptr(),aval_.as_ptr())) };
      self.handle_res(call_res,"putacollist64")?;
      return Result::Ok(());
    }

    // putacolslice64
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_a_col_slice(& mut self,first_ : i32,last_ : i32,ptrb_ : & [i64],ptre_ : & [i64],asub_ : & [i32],aval_ : & [f64]) -> Result<(),String>
    {
      if ! ptrb_.iter().all(|i| *i >= 0 && *i <= asub_.len() as i64 && *i <= aval_.len() as i64) { return Err("Invalid index in argument ptrb".to_string()) }
      if ! ptre_.iter().all(|i| *i >= 0 && *i <= asub_.len() as i64 && *i <= aval_.len() as i64) { return Err("Invalid index in argument ptre".to_string()) }
      let call_res = unsafe { (MSK_putacolslice64(self.ptr,first_ as i32,last_ as i32,ptrb_.as_ptr(),ptre_.as_ptr(),asub_.as_ptr(),aval_.as_ptr())) };
      self.handle_res(call_res,"putacolslice64")?;
      return Result::Ok(());
    }

    // putafebarfblocktriplet
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_afe_barf_block_triplet(& mut self,num_ : i64,afeidx_ : & [i64],barvaridx_ : & [i32],subk_ : & [i32],subl_ : & [i32],valkl_ : & [f64]) -> Result<(),String>
    {
      if afeidx_.len() != ((num_) as usize) { return Result::Err("Argument 'afeidx_' is too short in call to 'put_afe_barf_block_triplet'".to_string()) }
      if barvaridx_.len() != ((num_) as usize) { return Result::Err("Argument 'barvaridx_' is too short in call to 'put_afe_barf_block_triplet'".to_string()) }
      if subk_.len() != ((num_) as usize) { return Result::Err("Argument 'subk_' is too short in call to 'put_afe_barf_block_triplet'".to_string()) }
      if subl_.len() != ((num_) as usize) { return Result::Err("Argument 'subl_' is too short in call to 'put_afe_barf_block_triplet'".to_string()) }
      if valkl_.len() != ((num_) as usize) { return Result::Err("Argument 'valkl_' is too short in call to 'put_afe_barf_block_triplet'".to_string()) }
      let call_res = unsafe { (MSK_putafebarfblocktriplet(self.ptr,num_ as i64,afeidx_.as_ptr(),barvaridx_.as_ptr(),subk_.as_ptr(),subl_.as_ptr(),valkl_.as_ptr())) };
      self.handle_res(call_res,"putafebarfblocktriplet")?;
      return Result::Ok(());
    }

    // putafebarfentry
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_afe_barf_entry(& mut self,afeidx_ : i64,barvaridx_ : i32,termidx_ : & [i64],termweight_ : & [f64]) -> Result<(),String>
    {
      let mut numterms_ = termidx_.len();
      if termweight_.len() < numterms_ { numterms_ = termweight_.len() };
      let call_res = unsafe { (MSK_putafebarfentry(self.ptr,afeidx_ as i64,barvaridx_ as i32,numterms_ as i64,termidx_.as_ptr(),termweight_.as_ptr())) };
      self.handle_res(call_res,"putafebarfentry")?;
      return Result::Ok(());
    }

    // putafebarfentrylist
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_afe_barf_entry_list(& mut self,afeidxlist_ : & [i64],barvaridxlist_ : & [i32],numtermslist_ : & [i64],ptrtermslist_ : & [i64],termidx_ : & [i64],termweight_ : & [f64]) -> Result<(),String>
    {
      let mut lenlist_ = afeidxlist_.len();
      if barvaridxlist_.len() < lenlist_ { lenlist_ = barvaridxlist_.len() };
      if numtermslist_.len() < lenlist_ { lenlist_ = numtermslist_.len() };
      if ptrtermslist_.len() < lenlist_ { lenlist_ = ptrtermslist_.len() };
      let mut lenterms_ = termidx_.len();
      if termweight_.len() < lenterms_ { lenterms_ = termweight_.len() };
      let call_res = unsafe { (MSK_putafebarfentrylist(self.ptr,lenlist_ as i64,afeidxlist_.as_ptr(),barvaridxlist_.as_ptr(),numtermslist_.as_ptr(),ptrtermslist_.as_ptr(),lenterms_ as i64,termidx_.as_ptr(),termweight_.as_ptr())) };
      self.handle_res(call_res,"putafebarfentrylist")?;
      return Result::Ok(());
    }

    // putafebarfrow
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_afe_barf_row(& mut self,afeidx_ : i64,barvaridxlist_ : & [i32],numtermlist_ : & [i64],ptrtermlist_ : & [i64],termidx_ : & [i64],termweight_ : & [f64]) -> Result<(),String>
    {
      let mut numentries_ = barvaridxlist_.len();
      if numtermlist_.len() < numentries_ { numentries_ = numtermlist_.len() };
      if ptrtermlist_.len() < numentries_ { numentries_ = ptrtermlist_.len() };
      let mut lenterms_ = termidx_.len();
      if termweight_.len() < lenterms_ { lenterms_ = termweight_.len() };
      let call_res = unsafe { (MSK_putafebarfrow(self.ptr,afeidx_ as i64,numentries_ as i32,barvaridxlist_.as_ptr(),numtermlist_.as_ptr(),ptrtermlist_.as_ptr(),lenterms_ as i64,termidx_.as_ptr(),termweight_.as_ptr())) };
      self.handle_res(call_res,"putafebarfrow")?;
      return Result::Ok(());
    }

    // putafefentry
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_afe_f_entry(& mut self,afeidx_ : i64,j_ : i32,value_ : f64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putafefentry(self.ptr,afeidx_ as i64,j_ as i32,value_ as f64)) };
      self.handle_res(call_res,"putafefentry")?;
      return Result::Ok(());
    }

    // putafefrow
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_afe_f_row(& mut self,afeidx_ : i64,subi_ : & [i32],vali_ : & [f64]) -> Result<(),String>
    {
      let mut nzi_ = subi_.len();
      if vali_.len() < nzi_ { nzi_ = vali_.len() };
      let call_res = unsafe { (MSK_putafefrow(self.ptr,afeidx_ as i64,nzi_ as i32,subi_.as_ptr(),vali_.as_ptr())) };
      self.handle_res(call_res,"putafefrow")?;
      return Result::Ok(());
    }

    // putafefrowlist
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_afe_f_row_list(& mut self,afeidxlist_ : & [i64],nzrow_ : & [i32],ptrrow_ : & [i64],idxrow_ : & [i32],valrow_ : & [f64]) -> Result<(),String>
    {
      let mut numafeidx_ = afeidxlist_.len();
      if nzrow_.len() < numafeidx_ { numafeidx_ = nzrow_.len() };
      if ptrrow_.len() < numafeidx_ { numafeidx_ = ptrrow_.len() };
      let mut lenidxval_ = idxrow_.len();
      if valrow_.len() < lenidxval_ { lenidxval_ = valrow_.len() };
      let call_res = unsafe { (MSK_putafefrowlist(self.ptr,numafeidx_ as i64,afeidxlist_.as_ptr(),nzrow_.as_ptr(),ptrrow_.as_ptr(),lenidxval_ as i64,idxrow_.as_ptr(),valrow_.as_ptr())) };
      self.handle_res(call_res,"putafefrowlist")?;
      return Result::Ok(());
    }

    // putafeg
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_afe_g(& mut self,afeidx_ : i64,gi_ : f64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putafeg(self.ptr,afeidx_ as i64,gi_ as f64)) };
      self.handle_res(call_res,"putafeg")?;
      return Result::Ok(());
    }

    // putafeglist
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_afe_g_list(& mut self,afeidxlist_ : & [i64],glist_ : & [f64]) -> Result<(),String>
    {
      let mut numafeidx_ = afeidxlist_.len();
      if glist_.len() < numafeidx_ { numafeidx_ = glist_.len() };
      let call_res = unsafe { (MSK_putafeglist(self.ptr,numafeidx_ as i64,afeidxlist_.as_ptr(),glist_.as_ptr())) };
      self.handle_res(call_res,"putafeglist")?;
      return Result::Ok(());
    }

    // putafegslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_afe_g_slice(& mut self,first_ : i64,last_ : i64,slice_ : & mut [f64]) -> Result<(),String>
    {
      if slice_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'slice_' is too short in call to 'put_afe_g_slice'".to_string()) }
      let call_res = unsafe { (MSK_putafegslice(self.ptr,first_ as i64,last_ as i64,slice_.as_mut_ptr())) };
      self.handle_res(call_res,"putafegslice")?;
      return Result::Ok(());
    }

    // putaij
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_aij(& mut self,i_ : i32,j_ : i32,aij_ : f64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putaij(self.ptr,i_ as i32,j_ as i32,aij_ as f64)) };
      self.handle_res(call_res,"putaij")?;
      return Result::Ok(());
    }

    // putaijlist64
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_aij_list(& mut self,subi_ : & [i32],subj_ : & [i32],valij_ : & [f64]) -> Result<(),String>
    {
      let mut num_ = subi_.len();
      if subj_.len() < num_ { num_ = subj_.len() };
      if valij_.len() < num_ { num_ = valij_.len() };
      let call_res = unsafe { (MSK_putaijlist64(self.ptr,num_ as i64,subi_.as_ptr(),subj_.as_ptr(),valij_.as_ptr())) };
      self.handle_res(call_res,"putaijlist64")?;
      return Result::Ok(());
    }

    // putarow
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_a_row(& mut self,i_ : i32,subi_ : & [i32],vali_ : & [f64]) -> Result<(),String>
    {
      let mut nzi_ = subi_.len();
      if vali_.len() < nzi_ { nzi_ = vali_.len() };
      let call_res = unsafe { (MSK_putarow(self.ptr,i_ as i32,nzi_ as i32,subi_.as_ptr(),vali_.as_ptr())) };
      self.handle_res(call_res,"putarow")?;
      return Result::Ok(());
    }

    // putarowlist64
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_a_row_list(& mut self,sub_ : & [i32],ptrb_ : & [i64],ptre_ : & [i64],asub_ : & [i32],aval_ : & [f64]) -> Result<(),String>
    {
      let mut num_ = sub_.len();
      if ptrb_.len() < num_ { num_ = ptrb_.len() };
      if ptre_.len() < num_ { num_ = ptre_.len() };
      if ! ptrb_.iter().all(|i| *i >= 0 && *i <= asub_.len() as i64 && *i <= aval_.len() as i64) { return Err("Invalid index in argument ptrb".to_string()) }
      if ! ptre_.iter().all(|i| *i >= 0 && *i <= asub_.len() as i64 && *i <= aval_.len() as i64) { return Err("Invalid index in argument ptre".to_string()) }
      let call_res = unsafe { (MSK_putarowlist64(self.ptr,num_ as i32,sub_.as_ptr(),ptrb_.as_ptr(),ptre_.as_ptr(),asub_.as_ptr(),aval_.as_ptr())) };
      self.handle_res(call_res,"putarowlist64")?;
      return Result::Ok(());
    }

    // putarowslice64
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_a_row_slice(& mut self,first_ : i32,last_ : i32,ptrb_ : & [i64],ptre_ : & [i64],asub_ : & [i32],aval_ : & [f64]) -> Result<(),String>
    {
      if ptrb_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'ptrb_' is too short in call to 'put_a_row_slice'".to_string()) }
      if ptre_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'ptre_' is too short in call to 'put_a_row_slice'".to_string()) }
      if ! ptrb_.iter().all(|i| *i >= 0 && *i <= asub_.len() as i64 && *i <= aval_.len() as i64) { return Err("Invalid index in argument ptrb".to_string()) }
      if ! ptre_.iter().all(|i| *i >= 0 && *i <= asub_.len() as i64 && *i <= aval_.len() as i64) { return Err("Invalid index in argument ptre".to_string()) }
      let call_res = unsafe { (MSK_putarowslice64(self.ptr,first_ as i32,last_ as i32,ptrb_.as_ptr(),ptre_.as_ptr(),asub_.as_ptr(),aval_.as_ptr())) };
      self.handle_res(call_res,"putarowslice64")?;
      return Result::Ok(());
    }

    // putatruncatetol
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_a_truncate_tol(& mut self,tolzero_ : f64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putatruncatetol(self.ptr,tolzero_ as f64)) };
      self.handle_res(call_res,"putatruncatetol")?;
      return Result::Ok(());
    }

    // putbarablocktriplet
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_bara_block_triplet(& mut self,num_ : i64,subi_ : & [i32],subj_ : & [i32],subk_ : & [i32],subl_ : & [i32],valijkl_ : & [f64]) -> Result<(),String>
    {
      if subi_.len() != ((num_) as usize) { return Result::Err("Argument 'subi_' is too short in call to 'put_bara_block_triplet'".to_string()) }
      if subj_.len() != ((num_) as usize) { return Result::Err("Argument 'subj_' is too short in call to 'put_bara_block_triplet'".to_string()) }
      if subk_.len() != ((num_) as usize) { return Result::Err("Argument 'subk_' is too short in call to 'put_bara_block_triplet'".to_string()) }
      if subl_.len() != ((num_) as usize) { return Result::Err("Argument 'subl_' is too short in call to 'put_bara_block_triplet'".to_string()) }
      if valijkl_.len() != ((num_) as usize) { return Result::Err("Argument 'valijkl_' is too short in call to 'put_bara_block_triplet'".to_string()) }
      let call_res = unsafe { (MSK_putbarablocktriplet(self.ptr,num_ as i64,subi_.as_ptr(),subj_.as_ptr(),subk_.as_ptr(),subl_.as_ptr(),valijkl_.as_ptr())) };
      self.handle_res(call_res,"putbarablocktriplet")?;
      return Result::Ok(());
    }

    // putbaraij
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_bara_ij(& mut self,i_ : i32,j_ : i32,sub_ : & [i64],weights_ : & [f64]) -> Result<(),String>
    {
      let mut num_ = sub_.len();
      if weights_.len() < num_ { num_ = weights_.len() };
      let call_res = unsafe { (MSK_putbaraij(self.ptr,i_ as i32,j_ as i32,num_ as i64,sub_.as_ptr(),weights_.as_ptr())) };
      self.handle_res(call_res,"putbaraij")?;
      return Result::Ok(());
    }

    // putbaraijlist
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_bara_ij_list(& mut self,subi_ : & [i32],subj_ : & [i32],alphaptrb_ : & [i64],alphaptre_ : & [i64],matidx_ : & [i64],weights_ : & [f64]) -> Result<(),String>
    {
      let mut num_ = subi_.len();
      if subj_.len() < num_ { num_ = subj_.len() };
      if alphaptrb_.len() < num_ { num_ = alphaptrb_.len() };
      if alphaptre_.len() < num_ { num_ = alphaptre_.len() };
      if ! alphaptrb_.iter().all(|i| *i >= 0 && *i <= matidx_.len() as i64 && *i <= weights_.len() as i64) { return Err("Invalid index in argument alphaptrb".to_string()) }
      if ! alphaptre_.iter().all(|i| *i >= 0 && *i <= matidx_.len() as i64 && *i <= weights_.len() as i64) { return Err("Invalid index in argument alphaptre".to_string()) }
      let call_res = unsafe { (MSK_putbaraijlist(self.ptr,num_ as i32,subi_.as_ptr(),subj_.as_ptr(),alphaptrb_.as_ptr(),alphaptre_.as_ptr(),matidx_.as_ptr(),weights_.as_ptr())) };
      self.handle_res(call_res,"putbaraijlist")?;
      return Result::Ok(());
    }

    // putbararowlist
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_bara_row_list(& mut self,subi_ : & [i32],ptrb_ : & [i64],ptre_ : & [i64],subj_ : & [i32],nummat_ : & [i64],matidx_ : & [i64],weights_ : & [f64]) -> Result<(),String>
    {
      let mut num_ = subi_.len();
      if ptrb_.len() < num_ { num_ = ptrb_.len() };
      if ptre_.len() < num_ { num_ = ptre_.len() };
      let tmp_var_0__ = subj_.len();
      if nummat_.len() != ((tmp_var_0__) as usize) { return Result::Err("Argument 'nummat_' is too short in call to 'put_bara_row_list'".to_string()) }
      let tmp_var_2__ = nummat_.iter().fold(0,|res,v| res + v);
      if matidx_.len() != ((tmp_var_2__) as usize) { return Result::Err("Argument 'matidx_' is too short in call to 'put_bara_row_list'".to_string()) }
      let tmp_var_4__ = nummat_.iter().fold(0,|res,v| res + v);
      if weights_.len() != ((tmp_var_4__) as usize) { return Result::Err("Argument 'weights_' is too short in call to 'put_bara_row_list'".to_string()) }
      if ! ptrb_.iter().all(|i| *i >= 0 && *i <= subj_.len() as i64 && *i <= nummat_.len() as i64) { return Err("Invalid index in argument ptrb".to_string()) }
      if ! ptre_.iter().all(|i| *i >= 0 && *i <= subj_.len() as i64 && *i <= nummat_.len() as i64) { return Err("Invalid index in argument ptre".to_string()) }
      let call_res = unsafe { (MSK_putbararowlist(self.ptr,num_ as i32,subi_.as_ptr(),ptrb_.as_ptr(),ptre_.as_ptr(),subj_.as_ptr(),nummat_.as_ptr(),matidx_.as_ptr(),weights_.as_ptr())) };
      self.handle_res(call_res,"putbararowlist")?;
      return Result::Ok(());
    }

    // putbarcblocktriplet
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_barc_block_triplet(& mut self,num_ : i64,subj_ : & [i32],subk_ : & [i32],subl_ : & [i32],valjkl_ : & [f64]) -> Result<(),String>
    {
      if subj_.len() != ((num_) as usize) { return Result::Err("Argument 'subj_' is too short in call to 'put_barc_block_triplet'".to_string()) }
      if subk_.len() != ((num_) as usize) { return Result::Err("Argument 'subk_' is too short in call to 'put_barc_block_triplet'".to_string()) }
      if subl_.len() != ((num_) as usize) { return Result::Err("Argument 'subl_' is too short in call to 'put_barc_block_triplet'".to_string()) }
      if valjkl_.len() != ((num_) as usize) { return Result::Err("Argument 'valjkl_' is too short in call to 'put_barc_block_triplet'".to_string()) }
      let call_res = unsafe { (MSK_putbarcblocktriplet(self.ptr,num_ as i64,subj_.as_ptr(),subk_.as_ptr(),subl_.as_ptr(),valjkl_.as_ptr())) };
      self.handle_res(call_res,"putbarcblocktriplet")?;
      return Result::Ok(());
    }

    // putbarcj
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_barc_j(& mut self,j_ : i32,sub_ : & [i64],weights_ : & [f64]) -> Result<(),String>
    {
      let mut num_ = sub_.len();
      if weights_.len() < num_ { num_ = weights_.len() };
      let call_res = unsafe { (MSK_putbarcj(self.ptr,j_ as i32,num_ as i64,sub_.as_ptr(),weights_.as_ptr())) };
      self.handle_res(call_res,"putbarcj")?;
      return Result::Ok(());
    }

    // putbarsj
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_bars_j(& mut self,whichsol_ : i32,j_ : i32,barsj_ : & [f64]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_len_barvar_j(j_)?;
      if barsj_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'barsj_' is too short in call to 'put_bars_j'".to_string()) }
      let call_res = unsafe { (MSK_putbarsj(self.ptr,whichsol_,j_ as i32,barsj_.as_ptr())) };
      self.handle_res(call_res,"putbarsj")?;
      return Result::Ok(());
    }

    // putbarvarname
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_barvar_name(& mut self,j_ : i32,name_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putbarvarname(self.ptr,j_ as i32,CString::new(name_).unwrap().as_ptr())) };
      self.handle_res(call_res,"putbarvarname")?;
      return Result::Ok(());
    }

    // putbarxj
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_barx_j(& mut self,whichsol_ : i32,j_ : i32,barxj_ : & [f64]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_len_barvar_j(j_)?;
      if barxj_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'barxj_' is too short in call to 'put_barx_j'".to_string()) }
      let call_res = unsafe { (MSK_putbarxj(self.ptr,whichsol_,j_ as i32,barxj_.as_ptr())) };
      self.handle_res(call_res,"putbarxj")?;
      return Result::Ok(());
    }

    // putcfix
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_cfix(& mut self,cfix_ : f64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putcfix(self.ptr,cfix_ as f64)) };
      self.handle_res(call_res,"putcfix")?;
      return Result::Ok(());
    }

    // putcj
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_c_j(& mut self,j_ : i32,cj_ : f64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putcj(self.ptr,j_ as i32,cj_ as f64)) };
      self.handle_res(call_res,"putcj")?;
      return Result::Ok(());
    }

    // putclist
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_c_list(& mut self,subj_ : & [i32],val_ : & [f64]) -> Result<(),String>
    {
      let mut num_ = subj_.len();
      if val_.len() < num_ { num_ = val_.len() };
      let call_res = unsafe { (MSK_putclist(self.ptr,num_ as i32,subj_.as_ptr(),val_.as_ptr())) };
      self.handle_res(call_res,"putclist")?;
      return Result::Ok(());
    }

    // putconbound
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_con_bound(& mut self,i_ : i32,bkc_ : i32,blc_ : f64,buc_ : f64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putconbound(self.ptr,i_ as i32,bkc_,blc_ as f64,buc_ as f64)) };
      self.handle_res(call_res,"putconbound")?;
      return Result::Ok(());
    }

    // putconboundlist
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_con_bound_list(& mut self,sub_ : & [i32],bkc_ : & [i32],blc_ : & [f64],buc_ : & [f64]) -> Result<(),String>
    {
      let mut num_ = sub_.len();
      if bkc_.len() < num_ { num_ = bkc_.len() };
      if blc_.len() < num_ { num_ = blc_.len() };
      if buc_.len() < num_ { num_ = buc_.len() };
      let call_res = unsafe { (MSK_putconboundlist(self.ptr,num_ as i32,sub_.as_ptr(),bkc_.as_ptr(),blc_.as_ptr(),buc_.as_ptr())) };
      self.handle_res(call_res,"putconboundlist")?;
      return Result::Ok(());
    }

    // putconboundlistconst
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_con_bound_list_const(& mut self,sub_ : & [i32],bkc_ : i32,blc_ : f64,buc_ : f64) -> Result<(),String>
    {
      let mut num_ = sub_.len();
      let call_res = unsafe { (MSK_putconboundlistconst(self.ptr,num_ as i32,sub_.as_ptr(),bkc_,blc_ as f64,buc_ as f64)) };
      self.handle_res(call_res,"putconboundlistconst")?;
      return Result::Ok(());
    }

    // putconboundslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_con_bound_slice(& mut self,first_ : i32,last_ : i32,bkc_ : & [i32],blc_ : & [f64],buc_ : & [f64]) -> Result<(),String>
    {
      if bkc_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'bkc_' is too short in call to 'put_con_bound_slice'".to_string()) }
      if blc_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'blc_' is too short in call to 'put_con_bound_slice'".to_string()) }
      if buc_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'buc_' is too short in call to 'put_con_bound_slice'".to_string()) }
      let call_res = unsafe { (MSK_putconboundslice(self.ptr,first_ as i32,last_ as i32,bkc_.as_ptr(),blc_.as_ptr(),buc_.as_ptr())) };
      self.handle_res(call_res,"putconboundslice")?;
      return Result::Ok(());
    }

    // putconboundsliceconst
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_con_bound_slice_const(& mut self,first_ : i32,last_ : i32,bkc_ : i32,blc_ : f64,buc_ : f64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putconboundsliceconst(self.ptr,first_ as i32,last_ as i32,bkc_,blc_ as f64,buc_ as f64)) };
      self.handle_res(call_res,"putconboundsliceconst")?;
      return Result::Ok(());
    }

    // putcone
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_cone(& mut self,k_ : i32,ct_ : i32,conepar_ : f64,submem_ : & [i32]) -> Result<(),String>
    {
      let mut nummem_ = submem_.len();
      let call_res = unsafe { (MSK_putcone(self.ptr,k_ as i32,ct_,conepar_ as f64,nummem_ as i32,submem_.as_ptr())) };
      self.handle_res(call_res,"putcone")?;
      return Result::Ok(());
    }

    // putconename
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_cone_name(& mut self,j_ : i32,name_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putconename(self.ptr,j_ as i32,CString::new(name_).unwrap().as_ptr())) };
      self.handle_res(call_res,"putconename")?;
      return Result::Ok(());
    }

    // putconname
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_con_name(& mut self,i_ : i32,name_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putconname(self.ptr,i_ as i32,CString::new(name_).unwrap().as_ptr())) };
      self.handle_res(call_res,"putconname")?;
      return Result::Ok(());
    }

    // putconsolutioni
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_con_solution_i(& mut self,i_ : i32,whichsol_ : i32,sk_ : i32,x_ : f64,sl_ : f64,su_ : f64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putconsolutioni(self.ptr,i_ as i32,whichsol_,sk_,x_ as f64,sl_ as f64,su_ as f64)) };
      self.handle_res(call_res,"putconsolutioni")?;
      return Result::Ok(());
    }

    // putcslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_c_slice(& mut self,first_ : i32,last_ : i32,slice_ : & [f64]) -> Result<(),String>
    {
      if slice_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'slice_' is too short in call to 'put_c_slice'".to_string()) }
      let call_res = unsafe { (MSK_putcslice(self.ptr,first_ as i32,last_ as i32,slice_.as_ptr())) };
      self.handle_res(call_res,"putcslice")?;
      return Result::Ok(());
    }

    // putdjc
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_djc(& mut self,djcidx_ : i64,domidxlist_ : & [i64],afeidxlist_ : & [i64],b_ : & [f64],termsizelist_ : & [i64]) -> Result<(),String>
    {
      let mut numdomidx_ = domidxlist_.len();
      let mut numafeidx_ = afeidxlist_.len();
      if b_.len() != ((numafeidx_) as usize) { return Result::Err("Argument 'b_' is too short in call to 'put_djc'".to_string()) }
      let mut numterms_ = termsizelist_.len();
      let call_res = unsafe { (MSK_putdjc(self.ptr,djcidx_ as i64,numdomidx_ as i64,domidxlist_.as_ptr(),numafeidx_ as i64,afeidxlist_.as_ptr(),b_.as_ptr(),numterms_ as i64,termsizelist_.as_ptr())) };
      self.handle_res(call_res,"putdjc")?;
      return Result::Ok(());
    }

    // putdjcname
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_djc_name(& mut self,djcidx_ : i64,name_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putdjcname(self.ptr,djcidx_ as i64,CString::new(name_).unwrap().as_ptr())) };
      self.handle_res(call_res,"putdjcname")?;
      return Result::Ok(());
    }

    // putdjcslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_djc_slice(& mut self,idxfirst_ : i64,idxlast_ : i64,domidxlist_ : & [i64],afeidxlist_ : & [i64],b_ : & [f64],termsizelist_ : & [i64],termsindjc_ : & [i64]) -> Result<(),String>
    {
      let mut numdomidx_ = domidxlist_.len();
      let mut numafeidx_ = afeidxlist_.len();
      if b_.len() != ((numafeidx_) as usize) { return Result::Err("Argument 'b_' is too short in call to 'put_djc_slice'".to_string()) }
      let mut numterms_ = termsizelist_.len();
      if termsindjc_.len() != ((idxlast_ - idxfirst_) as usize) { return Result::Err("Argument 'termsindjc_' is too short in call to 'put_djc_slice'".to_string()) }
      let call_res = unsafe { (MSK_putdjcslice(self.ptr,idxfirst_ as i64,idxlast_ as i64,numdomidx_ as i64,domidxlist_.as_ptr(),numafeidx_ as i64,afeidxlist_.as_ptr(),b_.as_ptr(),numterms_ as i64,termsizelist_.as_ptr(),termsindjc_.as_ptr())) };
      self.handle_res(call_res,"putdjcslice")?;
      return Result::Ok(());
    }

    // putdomainname
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_domain_name(& mut self,domidx_ : i64,name_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putdomainname(self.ptr,domidx_ as i64,CString::new(name_).unwrap().as_ptr())) };
      self.handle_res(call_res,"putdomainname")?;
      return Result::Ok(());
    }

    // putdouparam
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_dou_param(& mut self,param_ : i32,parvalue_ : f64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putdouparam(self.ptr,param_,parvalue_ as f64)) };
      self.handle_res(call_res,"putdouparam")?;
      return Result::Ok(());
    }

    // putintparam
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_int_param(& mut self,param_ : i32,parvalue_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putintparam(self.ptr,param_,parvalue_ as i32)) };
      self.handle_res(call_res,"putintparam")?;
      return Result::Ok(());
    }

    // putmaxnumacc
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_max_num_acc(& mut self,maxnumacc_ : i64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putmaxnumacc(self.ptr,maxnumacc_ as i64)) };
      self.handle_res(call_res,"putmaxnumacc")?;
      return Result::Ok(());
    }

    // putmaxnumafe
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_max_num_afe(& mut self,maxnumafe_ : i64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putmaxnumafe(self.ptr,maxnumafe_ as i64)) };
      self.handle_res(call_res,"putmaxnumafe")?;
      return Result::Ok(());
    }

    // putmaxnumanz
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_max_num_a_nz(& mut self,maxnumanz_ : i64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putmaxnumanz(self.ptr,maxnumanz_ as i64)) };
      self.handle_res(call_res,"putmaxnumanz")?;
      return Result::Ok(());
    }

    // putmaxnumbarvar
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_max_num_barvar(& mut self,maxnumbarvar_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putmaxnumbarvar(self.ptr,maxnumbarvar_ as i32)) };
      self.handle_res(call_res,"putmaxnumbarvar")?;
      return Result::Ok(());
    }

    // putmaxnumcon
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_max_num_con(& mut self,maxnumcon_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putmaxnumcon(self.ptr,maxnumcon_ as i32)) };
      self.handle_res(call_res,"putmaxnumcon")?;
      return Result::Ok(());
    }

    // putmaxnumcone
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_max_num_cone(& mut self,maxnumcone_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putmaxnumcone(self.ptr,maxnumcone_ as i32)) };
      self.handle_res(call_res,"putmaxnumcone")?;
      return Result::Ok(());
    }

    // putmaxnumdjc
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_max_num_djc(& mut self,maxnumdjc_ : i64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putmaxnumdjc(self.ptr,maxnumdjc_ as i64)) };
      self.handle_res(call_res,"putmaxnumdjc")?;
      return Result::Ok(());
    }

    // putmaxnumdomain
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_max_num_domain(& mut self,maxnumdomain_ : i64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putmaxnumdomain(self.ptr,maxnumdomain_ as i64)) };
      self.handle_res(call_res,"putmaxnumdomain")?;
      return Result::Ok(());
    }

    // putmaxnumqnz
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_max_num_q_nz(& mut self,maxnumqnz_ : i64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putmaxnumqnz(self.ptr,maxnumqnz_ as i64)) };
      self.handle_res(call_res,"putmaxnumqnz")?;
      return Result::Ok(());
    }

    // putmaxnumvar
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_max_num_var(& mut self,maxnumvar_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putmaxnumvar(self.ptr,maxnumvar_ as i32)) };
      self.handle_res(call_res,"putmaxnumvar")?;
      return Result::Ok(());
    }

    // putnadouparam
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_na_dou_param(& mut self,paramname_ : &str,parvalue_ : f64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putnadouparam(self.ptr,CString::new(paramname_).unwrap().as_ptr(),parvalue_ as f64)) };
      self.handle_res(call_res,"putnadouparam")?;
      return Result::Ok(());
    }

    // putnaintparam
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_na_int_param(& mut self,paramname_ : &str,parvalue_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putnaintparam(self.ptr,CString::new(paramname_).unwrap().as_ptr(),parvalue_ as i32)) };
      self.handle_res(call_res,"putnaintparam")?;
      return Result::Ok(());
    }

    // putnastrparam
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_na_str_param(& mut self,paramname_ : &str,parvalue_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putnastrparam(self.ptr,CString::new(paramname_).unwrap().as_ptr(),CString::new(parvalue_).unwrap().as_ptr())) };
      self.handle_res(call_res,"putnastrparam")?;
      return Result::Ok(());
    }

    // putobjname
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_obj_name(& mut self,objname_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putobjname(self.ptr,CString::new(objname_).unwrap().as_ptr())) };
      self.handle_res(call_res,"putobjname")?;
      return Result::Ok(());
    }

    // putobjsense
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_obj_sense(& mut self,sense_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putobjsense(self.ptr,sense_)) };
      self.handle_res(call_res,"putobjsense")?;
      return Result::Ok(());
    }

    // putoptserverhost
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_optserver_host(& mut self,host_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putoptserverhost(self.ptr,CString::new(host_).unwrap().as_ptr())) };
      self.handle_res(call_res,"putoptserverhost")?;
      return Result::Ok(());
    }

    // putparam
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_param(& mut self,parname_ : &str,parvalue_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putparam(self.ptr,CString::new(parname_).unwrap().as_ptr(),CString::new(parvalue_).unwrap().as_ptr())) };
      self.handle_res(call_res,"putparam")?;
      return Result::Ok(());
    }

    // putqcon
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_q_con(& mut self,qcsubk_ : & [i32],qcsubi_ : & [i32],qcsubj_ : & [i32],qcval_ : & [f64]) -> Result<(),String>
    {
      let mut numqcnz_ = qcsubi_.len();
      if qcsubj_.len() < numqcnz_ { numqcnz_ = qcsubj_.len() };
      if qcval_.len() < numqcnz_ { numqcnz_ = qcval_.len() };
      let call_res = unsafe { (MSK_putqcon(self.ptr,numqcnz_ as i32,qcsubk_.as_ptr(),qcsubi_.as_ptr(),qcsubj_.as_ptr(),qcval_.as_ptr())) };
      self.handle_res(call_res,"putqcon")?;
      return Result::Ok(());
    }

    // putqconk
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_q_con_k(& mut self,k_ : i32,qcsubi_ : & [i32],qcsubj_ : & [i32],qcval_ : & [f64]) -> Result<(),String>
    {
      let mut numqcnz_ = qcsubi_.len();
      if qcsubj_.len() < numqcnz_ { numqcnz_ = qcsubj_.len() };
      if qcval_.len() < numqcnz_ { numqcnz_ = qcval_.len() };
      let call_res = unsafe { (MSK_putqconk(self.ptr,k_ as i32,numqcnz_ as i32,qcsubi_.as_ptr(),qcsubj_.as_ptr(),qcval_.as_ptr())) };
      self.handle_res(call_res,"putqconk")?;
      return Result::Ok(());
    }

    // putqobj
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_q_obj(& mut self,qosubi_ : & [i32],qosubj_ : & [i32],qoval_ : & [f64]) -> Result<(),String>
    {
      let mut numqonz_ = qosubi_.len();
      if qosubj_.len() < numqonz_ { numqonz_ = qosubj_.len() };
      if qoval_.len() < numqonz_ { numqonz_ = qoval_.len() };
      let call_res = unsafe { (MSK_putqobj(self.ptr,numqonz_ as i32,qosubi_.as_ptr(),qosubj_.as_ptr(),qoval_.as_ptr())) };
      self.handle_res(call_res,"putqobj")?;
      return Result::Ok(());
    }

    // putqobjij
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_q_obj_i_j(& mut self,i_ : i32,j_ : i32,qoij_ : f64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putqobjij(self.ptr,i_ as i32,j_ as i32,qoij_ as f64)) };
      self.handle_res(call_res,"putqobjij")?;
      return Result::Ok(());
    }

    // putskc
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_skc(& mut self,whichsol_ : i32,skc_ : & [i32]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_num_con()?;
      if skc_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'skc_' is too short in call to 'put_skc'".to_string()) }
      let call_res = unsafe { (MSK_putskc(self.ptr,whichsol_,skc_.as_ptr())) };
      self.handle_res(call_res,"putskc")?;
      return Result::Ok(());
    }

    // putskcslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_skc_slice(& mut self,whichsol_ : i32,first_ : i32,last_ : i32,skc_ : & [i32]) -> Result<(),String>
    {
      if skc_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'skc_' is too short in call to 'put_skc_slice'".to_string()) }
      let call_res = unsafe { (MSK_putskcslice(self.ptr,whichsol_,first_ as i32,last_ as i32,skc_.as_ptr())) };
      self.handle_res(call_res,"putskcslice")?;
      return Result::Ok(());
    }

    // putskx
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_skx(& mut self,whichsol_ : i32,skx_ : & [i32]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_num_var()?;
      if skx_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'skx_' is too short in call to 'put_skx'".to_string()) }
      let call_res = unsafe { (MSK_putskx(self.ptr,whichsol_,skx_.as_ptr())) };
      self.handle_res(call_res,"putskx")?;
      return Result::Ok(());
    }

    // putskxslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_skx_slice(& mut self,whichsol_ : i32,first_ : i32,last_ : i32,skx_ : & [i32]) -> Result<(),String>
    {
      if skx_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'skx_' is too short in call to 'put_skx_slice'".to_string()) }
      let call_res = unsafe { (MSK_putskxslice(self.ptr,whichsol_,first_ as i32,last_ as i32,skx_.as_ptr())) };
      self.handle_res(call_res,"putskxslice")?;
      return Result::Ok(());
    }

    // putslc
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_slc(& mut self,whichsol_ : i32,slc_ : & [f64]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_num_con()?;
      if slc_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'slc_' is too short in call to 'put_slc'".to_string()) }
      let call_res = unsafe { (MSK_putslc(self.ptr,whichsol_,slc_.as_ptr())) };
      self.handle_res(call_res,"putslc")?;
      return Result::Ok(());
    }

    // putslcslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_slc_slice(& mut self,whichsol_ : i32,first_ : i32,last_ : i32,slc_ : & [f64]) -> Result<(),String>
    {
      if slc_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'slc_' is too short in call to 'put_slc_slice'".to_string()) }
      let call_res = unsafe { (MSK_putslcslice(self.ptr,whichsol_,first_ as i32,last_ as i32,slc_.as_ptr())) };
      self.handle_res(call_res,"putslcslice")?;
      return Result::Ok(());
    }

    // putslx
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_slx(& mut self,whichsol_ : i32,slx_ : & [f64]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_num_var()?;
      if slx_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'slx_' is too short in call to 'put_slx'".to_string()) }
      let call_res = unsafe { (MSK_putslx(self.ptr,whichsol_,slx_.as_ptr())) };
      self.handle_res(call_res,"putslx")?;
      return Result::Ok(());
    }

    // putslxslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_slx_slice(& mut self,whichsol_ : i32,first_ : i32,last_ : i32,slx_ : & [f64]) -> Result<(),String>
    {
      if slx_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'slx_' is too short in call to 'put_slx_slice'".to_string()) }
      let call_res = unsafe { (MSK_putslxslice(self.ptr,whichsol_,first_ as i32,last_ as i32,slx_.as_ptr())) };
      self.handle_res(call_res,"putslxslice")?;
      return Result::Ok(());
    }

    // putsnx
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_snx(& mut self,whichsol_ : i32,sux_ : & [f64]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_num_var()?;
      if sux_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'sux_' is too short in call to 'put_snx'".to_string()) }
      let call_res = unsafe { (MSK_putsnx(self.ptr,whichsol_,sux_.as_ptr())) };
      self.handle_res(call_res,"putsnx")?;
      return Result::Ok(());
    }

    // putsnxslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_snx_slice(& mut self,whichsol_ : i32,first_ : i32,last_ : i32,snx_ : & [f64]) -> Result<(),String>
    {
      if snx_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'snx_' is too short in call to 'put_snx_slice'".to_string()) }
      let call_res = unsafe { (MSK_putsnxslice(self.ptr,whichsol_,first_ as i32,last_ as i32,snx_.as_ptr())) };
      self.handle_res(call_res,"putsnxslice")?;
      return Result::Ok(());
    }

    // putsolution
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_solution(& mut self,whichsol_ : i32,skc_ : & [i32],skx_ : & [i32],skn_ : & [i32],xc_ : & [f64],xx_ : & [f64],y_ : & [f64],slc_ : & [f64],suc_ : & [f64],slx_ : & [f64],sux_ : & [f64],snx_ : & [f64]) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putsolution(self.ptr,whichsol_,skc_.as_ptr(),skx_.as_ptr(),skn_.as_ptr(),xc_.as_ptr(),xx_.as_ptr(),y_.as_ptr(),slc_.as_ptr(),suc_.as_ptr(),slx_.as_ptr(),sux_.as_ptr(),snx_.as_ptr())) };
      self.handle_res(call_res,"putsolution")?;
      return Result::Ok(());
    }

    // putsolutionnew
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_solution_new(& mut self,whichsol_ : i32,skc_ : & [i32],skx_ : & [i32],skn_ : & [i32],xc_ : & [f64],xx_ : & [f64],y_ : & [f64],slc_ : & [f64],suc_ : & [f64],slx_ : & [f64],sux_ : & [f64],snx_ : & [f64],doty_ : & [f64]) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putsolutionnew(self.ptr,whichsol_,skc_.as_ptr(),skx_.as_ptr(),skn_.as_ptr(),xc_.as_ptr(),xx_.as_ptr(),y_.as_ptr(),slc_.as_ptr(),suc_.as_ptr(),slx_.as_ptr(),sux_.as_ptr(),snx_.as_ptr(),doty_.as_ptr())) };
      self.handle_res(call_res,"putsolutionnew")?;
      return Result::Ok(());
    }

    // putsolutionyi
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_solution_y_i(& mut self,i_ : i32,whichsol_ : i32,y_ : f64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putsolutionyi(self.ptr,i_ as i32,whichsol_,y_ as f64)) };
      self.handle_res(call_res,"putsolutionyi")?;
      return Result::Ok(());
    }

    // putstrparam
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_str_param(& mut self,param_ : i32,parvalue_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putstrparam(self.ptr,param_,CString::new(parvalue_).unwrap().as_ptr())) };
      self.handle_res(call_res,"putstrparam")?;
      return Result::Ok(());
    }

    // putsuc
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_suc(& mut self,whichsol_ : i32,suc_ : & [f64]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_num_con()?;
      if suc_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'suc_' is too short in call to 'put_suc'".to_string()) }
      let call_res = unsafe { (MSK_putsuc(self.ptr,whichsol_,suc_.as_ptr())) };
      self.handle_res(call_res,"putsuc")?;
      return Result::Ok(());
    }

    // putsucslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_suc_slice(& mut self,whichsol_ : i32,first_ : i32,last_ : i32,suc_ : & [f64]) -> Result<(),String>
    {
      if suc_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'suc_' is too short in call to 'put_suc_slice'".to_string()) }
      let call_res = unsafe { (MSK_putsucslice(self.ptr,whichsol_,first_ as i32,last_ as i32,suc_.as_ptr())) };
      self.handle_res(call_res,"putsucslice")?;
      return Result::Ok(());
    }

    // putsux
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_sux(& mut self,whichsol_ : i32,sux_ : & [f64]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_num_var()?;
      if sux_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'sux_' is too short in call to 'put_sux'".to_string()) }
      let call_res = unsafe { (MSK_putsux(self.ptr,whichsol_,sux_.as_ptr())) };
      self.handle_res(call_res,"putsux")?;
      return Result::Ok(());
    }

    // putsuxslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_sux_slice(& mut self,whichsol_ : i32,first_ : i32,last_ : i32,sux_ : & [f64]) -> Result<(),String>
    {
      if sux_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'sux_' is too short in call to 'put_sux_slice'".to_string()) }
      let call_res = unsafe { (MSK_putsuxslice(self.ptr,whichsol_,first_ as i32,last_ as i32,sux_.as_ptr())) };
      self.handle_res(call_res,"putsuxslice")?;
      return Result::Ok(());
    }

    // puttaskname
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_task_name(& mut self,taskname_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_puttaskname(self.ptr,CString::new(taskname_).unwrap().as_ptr())) };
      self.handle_res(call_res,"puttaskname")?;
      return Result::Ok(());
    }

    // putvarbound
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_var_bound(& mut self,j_ : i32,bkx_ : i32,blx_ : f64,bux_ : f64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putvarbound(self.ptr,j_ as i32,bkx_,blx_ as f64,bux_ as f64)) };
      self.handle_res(call_res,"putvarbound")?;
      return Result::Ok(());
    }

    // putvarboundlist
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_var_bound_list(& mut self,sub_ : & [i32],bkx_ : & [i32],blx_ : & [f64],bux_ : & [f64]) -> Result<(),String>
    {
      let mut num_ = sub_.len();
      if bkx_.len() < num_ { num_ = bkx_.len() };
      if blx_.len() < num_ { num_ = blx_.len() };
      if bux_.len() < num_ { num_ = bux_.len() };
      let call_res = unsafe { (MSK_putvarboundlist(self.ptr,num_ as i32,sub_.as_ptr(),bkx_.as_ptr(),blx_.as_ptr(),bux_.as_ptr())) };
      self.handle_res(call_res,"putvarboundlist")?;
      return Result::Ok(());
    }

    // putvarboundlistconst
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_var_bound_list_const(& mut self,sub_ : & [i32],bkx_ : i32,blx_ : f64,bux_ : f64) -> Result<(),String>
    {
      let mut num_ = sub_.len();
      let call_res = unsafe { (MSK_putvarboundlistconst(self.ptr,num_ as i32,sub_.as_ptr(),bkx_,blx_ as f64,bux_ as f64)) };
      self.handle_res(call_res,"putvarboundlistconst")?;
      return Result::Ok(());
    }

    // putvarboundslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_var_bound_slice(& mut self,first_ : i32,last_ : i32,bkx_ : & [i32],blx_ : & [f64],bux_ : & [f64]) -> Result<(),String>
    {
      if bkx_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'bkx_' is too short in call to 'put_var_bound_slice'".to_string()) }
      if blx_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'blx_' is too short in call to 'put_var_bound_slice'".to_string()) }
      if bux_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'bux_' is too short in call to 'put_var_bound_slice'".to_string()) }
      let call_res = unsafe { (MSK_putvarboundslice(self.ptr,first_ as i32,last_ as i32,bkx_.as_ptr(),blx_.as_ptr(),bux_.as_ptr())) };
      self.handle_res(call_res,"putvarboundslice")?;
      return Result::Ok(());
    }

    // putvarboundsliceconst
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_var_bound_slice_const(& mut self,first_ : i32,last_ : i32,bkx_ : i32,blx_ : f64,bux_ : f64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putvarboundsliceconst(self.ptr,first_ as i32,last_ as i32,bkx_,blx_ as f64,bux_ as f64)) };
      self.handle_res(call_res,"putvarboundsliceconst")?;
      return Result::Ok(());
    }

    // putvarname
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_var_name(& mut self,j_ : i32,name_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putvarname(self.ptr,j_ as i32,CString::new(name_).unwrap().as_ptr())) };
      self.handle_res(call_res,"putvarname")?;
      return Result::Ok(());
    }

    // putvarsolutionj
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_var_solution_j(& mut self,j_ : i32,whichsol_ : i32,sk_ : i32,x_ : f64,sl_ : f64,su_ : f64,sn_ : f64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putvarsolutionj(self.ptr,j_ as i32,whichsol_,sk_,x_ as f64,sl_ as f64,su_ as f64,sn_ as f64)) };
      self.handle_res(call_res,"putvarsolutionj")?;
      return Result::Ok(());
    }

    // putvartype
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_var_type(& mut self,j_ : i32,vartype_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_putvartype(self.ptr,j_ as i32,vartype_)) };
      self.handle_res(call_res,"putvartype")?;
      return Result::Ok(());
    }

    // putvartypelist
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_var_type_list(& mut self,subj_ : & [i32],vartype_ : & [i32]) -> Result<(),String>
    {
      let mut num_ = subj_.len();
      if vartype_.len() < num_ { num_ = vartype_.len() };
      let call_res = unsafe { (MSK_putvartypelist(self.ptr,num_ as i32,subj_.as_ptr(),vartype_.as_ptr())) };
      self.handle_res(call_res,"putvartypelist")?;
      return Result::Ok(());
    }

    // putxc
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_xc(& mut self,whichsol_ : i32,xc_ : & mut [f64]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_num_con()?;
      if xc_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'xc_' is too short in call to 'put_xc'".to_string()) }
      let call_res = unsafe { (MSK_putxc(self.ptr,whichsol_,xc_.as_mut_ptr())) };
      self.handle_res(call_res,"putxc")?;
      return Result::Ok(());
    }

    // putxcslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_xc_slice(& mut self,whichsol_ : i32,first_ : i32,last_ : i32,xc_ : & [f64]) -> Result<(),String>
    {
      if xc_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'xc_' is too short in call to 'put_xc_slice'".to_string()) }
      let call_res = unsafe { (MSK_putxcslice(self.ptr,whichsol_,first_ as i32,last_ as i32,xc_.as_ptr())) };
      self.handle_res(call_res,"putxcslice")?;
      return Result::Ok(());
    }

    // putxx
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_xx(& mut self,whichsol_ : i32,xx_ : & [f64]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_num_var()?;
      if xx_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'xx_' is too short in call to 'put_xx'".to_string()) }
      let call_res = unsafe { (MSK_putxx(self.ptr,whichsol_,xx_.as_ptr())) };
      self.handle_res(call_res,"putxx")?;
      return Result::Ok(());
    }

    // putxxslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_xx_slice(& mut self,whichsol_ : i32,first_ : i32,last_ : i32,xx_ : & [f64]) -> Result<(),String>
    {
      if xx_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'xx_' is too short in call to 'put_xx_slice'".to_string()) }
      let call_res = unsafe { (MSK_putxxslice(self.ptr,whichsol_,first_ as i32,last_ as i32,xx_.as_ptr())) };
      self.handle_res(call_res,"putxxslice")?;
      return Result::Ok(());
    }

    // puty
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_y(& mut self,whichsol_ : i32,y_ : & [f64]) -> Result<(),String>
    {
      let tmp_var_1__ = self.get_num_con()?;
      if y_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'y_' is too short in call to 'put_y'".to_string()) }
      let call_res = unsafe { (MSK_puty(self.ptr,whichsol_,y_.as_ptr())) };
      self.handle_res(call_res,"puty")?;
      return Result::Ok(());
    }

    // putyslice
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn put_y_slice(& mut self,whichsol_ : i32,first_ : i32,last_ : i32,y_ : & [f64]) -> Result<(),String>
    {
      if y_.len() != ((last_ - first_) as usize) { return Result::Err("Argument 'y_' is too short in call to 'put_y_slice'".to_string()) }
      let call_res = unsafe { (MSK_putyslice(self.ptr,whichsol_,first_ as i32,last_ as i32,y_.as_ptr())) };
      self.handle_res(call_res,"putyslice")?;
      return Result::Ok(());
    }

    // readdataautoformat
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn read_data(& mut self,filename_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_readdataautoformat(self.ptr,CString::new(filename_).unwrap().as_ptr())) };
      self.handle_res(call_res,"readdataautoformat")?;
      return Result::Ok(());
    }

    // readdataformat
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn read_data_format(& mut self,filename_ : &str,format_ : i32,compress_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_readdataformat(self.ptr,CString::new(filename_).unwrap().as_ptr(),format_,compress_)) };
      self.handle_res(call_res,"readdataformat")?;
      return Result::Ok(());
    }

    // readjsonstring
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn read_json_string(& mut self,data_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_readjsonstring(self.ptr,CString::new(data_).unwrap().as_ptr())) };
      self.handle_res(call_res,"readjsonstring")?;
      return Result::Ok(());
    }

    // readlpstring
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn read_lp_string(& mut self,data_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_readlpstring(self.ptr,CString::new(data_).unwrap().as_ptr())) };
      self.handle_res(call_res,"readlpstring")?;
      return Result::Ok(());
    }

    // readopfstring
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn read_opf_string(& mut self,data_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_readopfstring(self.ptr,CString::new(data_).unwrap().as_ptr())) };
      self.handle_res(call_res,"readopfstring")?;
      return Result::Ok(());
    }

    // readparamfile
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn read_param_file(& mut self,filename_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_readparamfile(self.ptr,CString::new(filename_).unwrap().as_ptr())) };
      self.handle_res(call_res,"readparamfile")?;
      return Result::Ok(());
    }

    // readptfstring
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn read_ptf_string(& mut self,data_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_readptfstring(self.ptr,CString::new(data_).unwrap().as_ptr())) };
      self.handle_res(call_res,"readptfstring")?;
      return Result::Ok(());
    }

    // readsolution
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn read_solution(& mut self,whichsol_ : i32,filename_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_readsolution(self.ptr,whichsol_,CString::new(filename_).unwrap().as_ptr())) };
      self.handle_res(call_res,"readsolution")?;
      return Result::Ok(());
    }

    // readsummary
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn read_summary(& mut self,whichstream_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_readsummary(self.ptr,whichstream_)) };
      self.handle_res(call_res,"readsummary")?;
      return Result::Ok(());
    }

    // readtask
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn read_task(& mut self,filename_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_readtask(self.ptr,CString::new(filename_).unwrap().as_ptr())) };
      self.handle_res(call_res,"readtask")?;
      return Result::Ok(());
    }

    // removebarvars
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn remove_barvars(& mut self,subset_ : & [i32]) -> Result<(),String>
    {
      let mut num_ = subset_.len();
      let call_res = unsafe { (MSK_removebarvars(self.ptr,num_ as i32,subset_.as_ptr())) };
      self.handle_res(call_res,"removebarvars")?;
      return Result::Ok(());
    }

    // removecones
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn remove_cones(& mut self,subset_ : & [i32]) -> Result<(),String>
    {
      let mut num_ = subset_.len();
      let call_res = unsafe { (MSK_removecones(self.ptr,num_ as i32,subset_.as_ptr())) };
      self.handle_res(call_res,"removecones")?;
      return Result::Ok(());
    }

    // removecons
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn remove_cons(& mut self,subset_ : & [i32]) -> Result<(),String>
    {
      let mut num_ = subset_.len();
      let call_res = unsafe { (MSK_removecons(self.ptr,num_ as i32,subset_.as_ptr())) };
      self.handle_res(call_res,"removecons")?;
      return Result::Ok(());
    }

    // removevars
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn remove_vars(& mut self,subset_ : & [i32]) -> Result<(),String>
    {
      let mut num_ = subset_.len();
      let call_res = unsafe { (MSK_removevars(self.ptr,num_ as i32,subset_.as_ptr())) };
      self.handle_res(call_res,"removevars")?;
      return Result::Ok(());
    }

    // resizetask
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn resize_task(& mut self,maxnumcon_ : i32,maxnumvar_ : i32,maxnumcone_ : i32,maxnumanz_ : i64,maxnumqnz_ : i64) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_resizetask(self.ptr,maxnumcon_ as i32,maxnumvar_ as i32,maxnumcone_ as i32,maxnumanz_ as i64,maxnumqnz_ as i64)) };
      self.handle_res(call_res,"resizetask")?;
      return Result::Ok(());
    }

    // sensitivityreport
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn sensitivity_report(&self,whichstream_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_sensitivityreport(self.ptr,whichstream_)) };
      self.handle_res(call_res,"sensitivityreport")?;
      return Result::Ok(());
    }

    // setdefaults
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn set_defaults(& mut self) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_setdefaults(self.ptr)) };
      self.handle_res(call_res,"setdefaults")?;
      return Result::Ok(());
    }

    // sktostr
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn sk_to_str(&self,sk_ : i32) -> Result<String,String>
    {
      let mut _str__bytes = Vec::with_capacity(MSK_MAX_STR_LEN as usize);
      let call_res = unsafe { (MSK_sktostr(self.ptr,sk_,_str__bytes.as_mut_ptr())) };
      self.handle_res(call_res,"sktostr")?;
      unsafe { _str__bytes.set_len((MSK_MAX_STR_LEN) as usize) };
      return Result::Ok((String::from_utf8_lossy(&_str__bytes[..]).into_owned()));
    }

    // solstatostr
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn sol_sta_to_str(&self,solsta_ : i32) -> Result<String,String>
    {
      let mut _str__bytes = Vec::with_capacity(MSK_MAX_STR_LEN as usize);
      let call_res = unsafe { (MSK_solstatostr(self.ptr,solsta_,_str__bytes.as_mut_ptr())) };
      self.handle_res(call_res,"solstatostr")?;
      unsafe { _str__bytes.set_len((MSK_MAX_STR_LEN) as usize) };
      return Result::Ok((String::from_utf8_lossy(&_str__bytes[..]).into_owned()));
    }

    // solutiondef
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn solution_def(&self,whichsol_ : i32) -> Result<bool,String>
    {
      let mut _ref_isdef_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_solutiondef(self.ptr,whichsol_,& mut _ref_isdef_)) };
      self.handle_res(call_res,"solutiondef")?;
      return Result::Ok((_ref_isdef_ != 0));
    }

    // solutionsummary
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn solution_summary(&self,whichstream_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_solutionsummary(self.ptr,whichstream_)) };
      self.handle_res(call_res,"solutionsummary")?;
      return Result::Ok(());
    }

    // solvewithbasis
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn solve_with_basis(& mut self,transp_ : i32,numnz_ : i32,sub_ : & mut [i32],val_ : & mut [f64]) -> Result<i32,String>
    {
      let mut _ref_numnz_ = numnz_;
      let tmp_var_1__ = self.get_num_con()?;
      if sub_.len() != ((tmp_var_1__) as usize) { return Result::Err("Argument 'sub_' is too short in call to 'solve_with_basis'".to_string()) }
      let tmp_var_3__ = self.get_num_con()?;
      if val_.len() != ((tmp_var_3__) as usize) { return Result::Err("Argument 'val_' is too short in call to 'solve_with_basis'".to_string()) }
      let call_res = unsafe { (MSK_solvewithbasis(self.ptr,transp_ as i32,& mut _ref_numnz_,sub_.as_mut_ptr(),val_.as_mut_ptr())) };
      self.handle_res(call_res,"solvewithbasis")?;
      return Result::Ok((_ref_numnz_ as i32));
    }

    // strduptask
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn str_dup_task(&self,str_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_strduptask(self.ptr,CString::new(str_).unwrap().as_ptr())) };
      self.handle_res(call_res,"strduptask")?;
      return Result::Ok(());
    }

    // strtoconetype
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn str_to_cone_type(&self,str_ : &str) -> Result<i32,String>
    {
      let mut _ref_conetype_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_strtoconetype(self.ptr,CString::new(str_).unwrap().as_ptr(),& mut _ref_conetype_)) };
      self.handle_res(call_res,"strtoconetype")?;
      return Result::Ok((_ref_conetype_ as i32));
    }

    // strtosk
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn str_to_sk(&self,str_ : &str) -> Result<i32,String>
    {
      let mut _ref_sk_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_strtosk(self.ptr,CString::new(str_).unwrap().as_ptr(),& mut _ref_sk_)) };
      self.handle_res(call_res,"strtosk")?;
      return Result::Ok((_ref_sk_ as i32));
    }

    // toconic
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn toconic(& mut self) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_toconic(self.ptr)) };
      self.handle_res(call_res,"toconic")?;
      return Result::Ok(());
    }

    // unlinkfuncfromtaskstream
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn unlink_func_from_stream(& mut self,whichstream_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_unlinkfuncfromtaskstream(self.ptr,whichstream_)) };
      self.handle_res(call_res,"unlinkfuncfromtaskstream")?;
      return Result::Ok(());
    }

    // updatesolutioninfo
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn update_solution_info(& mut self,whichsol_ : i32) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_updatesolutioninfo(self.ptr,whichsol_)) };
      self.handle_res(call_res,"updatesolutioninfo")?;
      return Result::Ok(());
    }

    // whichparam
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn which_param(&self,parname_ : &str) -> Result<(i32,i32),String>
    {
      let mut _ref_partype_ : i32 = 0 as i32;
      let mut _ref_param_ : i32 = 0 as i32;
      let call_res = unsafe { (MSK_whichparam(self.ptr,CString::new(parname_).unwrap().as_ptr(),& mut _ref_partype_,& mut _ref_param_)) };
      self.handle_res(call_res,"whichparam")?;
      return Result::Ok((_ref_partype_ as i32,_ref_param_ as i32));
    }

    // writedata
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn write_data(&self,filename_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_writedata(self.ptr,CString::new(filename_).unwrap().as_ptr())) };
      self.handle_res(call_res,"writedata")?;
      return Result::Ok(());
    }

    // writejsonsol
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn write_json_sol(&self,filename_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_writejsonsol(self.ptr,CString::new(filename_).unwrap().as_ptr())) };
      self.handle_res(call_res,"writejsonsol")?;
      return Result::Ok(());
    }

    // writeparamfile
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn write_param_file(&self,filename_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_writeparamfile(self.ptr,CString::new(filename_).unwrap().as_ptr())) };
      self.handle_res(call_res,"writeparamfile")?;
      return Result::Ok(());
    }

    // writesolution
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn write_solution(&self,whichsol_ : i32,filename_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_writesolution(self.ptr,whichsol_,CString::new(filename_).unwrap().as_ptr())) };
      self.handle_res(call_res,"writesolution")?;
      return Result::Ok(());
    }

    // writetask
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    pub fn write_task(&self,filename_ : &str) -> Result<(),String>
    {
      let call_res = unsafe { (MSK_writetask(self.ptr,CString::new(filename_).unwrap().as_ptr())) };
      self.handle_res(call_res,"writetask")?;
      return Result::Ok(());
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



// callbackcodetostr
#[allow(non_snake_case)]
#[allow(unused_mut)]
#[allow(unused_parens)]
#[allow(unused_variables)]
pub fn callback_code_to_str(code_ : i32) -> Result<String,String>
{
  let mut _callbackcodestr__bytes = Vec::with_capacity(MSK_MAX_STR_LEN as usize);
  let call_res = unsafe { (MSK_callbackcodetostr(code_,_callbackcodestr__bytes.as_mut_ptr())) };
  handle_res_static(call_res,"callbackcodetostr")?;
  unsafe { _callbackcodestr__bytes.set_len((MSK_MAX_STR_LEN) as usize) };
  return Result::Ok((String::from_utf8_lossy(&_callbackcodestr__bytes[..]).into_owned()));
}

// getbuildinfo
#[allow(non_snake_case)]
#[allow(unused_mut)]
#[allow(unused_parens)]
#[allow(unused_variables)]
pub fn get_build_info() -> Result<(String,String),String>
{
  let mut _buildstate__bytes = Vec::with_capacity(MSK_MAX_STR_LEN as usize);
  let mut _builddate__bytes = Vec::with_capacity(MSK_MAX_STR_LEN as usize);
  let call_res = unsafe { (MSK_getbuildinfo(_buildstate__bytes.as_mut_ptr(),_builddate__bytes.as_mut_ptr())) };
  handle_res_static(call_res,"getbuildinfo")?;
  unsafe { _buildstate__bytes.set_len((MSK_MAX_STR_LEN) as usize) };
  unsafe { _builddate__bytes.set_len((MSK_MAX_STR_LEN) as usize) };
  return Result::Ok((String::from_utf8_lossy(&_buildstate__bytes[..]).into_owned(),String::from_utf8_lossy(&_builddate__bytes[..]).into_owned()));
}

// getcodedesc
#[allow(non_snake_case)]
#[allow(unused_mut)]
#[allow(unused_parens)]
#[allow(unused_variables)]
pub fn get_code_desc(code_ : i32) -> Result<(String,String),String>
{
  let mut _symname__bytes = Vec::with_capacity(MSK_MAX_STR_LEN as usize);
  let mut _str__bytes = Vec::with_capacity(MSK_MAX_STR_LEN as usize);
  let call_res = unsafe { (MSK_getcodedesc(code_,_symname__bytes.as_mut_ptr(),_str__bytes.as_mut_ptr())) };
  handle_res_static(call_res,"getcodedesc")?;
  unsafe { _symname__bytes.set_len((MSK_MAX_STR_LEN) as usize) };
  unsafe { _str__bytes.set_len((MSK_MAX_STR_LEN) as usize) };
  return Result::Ok((String::from_utf8_lossy(&_symname__bytes[..]).into_owned(),String::from_utf8_lossy(&_str__bytes[..]).into_owned()));
}

// getresponseclass
#[allow(non_snake_case)]
#[allow(unused_mut)]
#[allow(unused_parens)]
#[allow(unused_variables)]
pub fn get_response_class(r_ : i32) -> Result<i32,String>
{
  let mut _ref_rc_ : i32 = 0 as i32;
  let call_res = unsafe { (MSK_getresponseclass(r_,& mut _ref_rc_)) };
  handle_res_static(call_res,"getresponseclass")?;
  return Result::Ok((_ref_rc_ as i32));
}

// getversion
#[allow(non_snake_case)]
#[allow(unused_mut)]
#[allow(unused_parens)]
#[allow(unused_variables)]
pub fn get_version() -> Result<(i32,i32,i32),String>
{
  let mut _ref_major_ : i32 = 0 as i32;
  let mut _ref_minor_ : i32 = 0 as i32;
  let mut _ref_revision_ : i32 = 0 as i32;
  let call_res = unsafe { (MSK_getversion(& mut _ref_major_,& mut _ref_minor_,& mut _ref_revision_)) };
  handle_res_static(call_res,"getversion")?;
  return Result::Ok((_ref_major_ as i32,_ref_minor_ as i32,_ref_revision_ as i32));
}

// isinfinity
#[allow(non_snake_case)]
#[allow(unused_mut)]
#[allow(unused_parens)]
#[allow(unused_variables)]
pub fn is_infinity(value_ : f64) -> Result<(),String>
{
  let call_res = unsafe { (MSK_isinfinity(value_ as f64)) };
  handle_res_static(call_res,"isinfinity")?;
  return Result::Ok(());
}

// licensecleanup
#[allow(non_snake_case)]
#[allow(unused_mut)]
#[allow(unused_parens)]
#[allow(unused_variables)]
pub fn licensecleanup() -> Result<(),String>
{
  let call_res = unsafe { (MSK_licensecleanup()) };
  handle_res_static(call_res,"licensecleanup")?;
  return Result::Ok(());
}

// symnamtovalue
#[allow(non_snake_case)]
#[allow(unused_mut)]
#[allow(unused_parens)]
#[allow(unused_variables)]
pub fn sym_nam_to_value(name_ : &str) -> Result<String,String>
{
  let mut _value__bytes = Vec::with_capacity(MSK_MAX_STR_LEN as usize);
  let call_res = unsafe { (MSK_symnamtovalue(CString::new(name_).unwrap().as_ptr(),_value__bytes.as_mut_ptr())) };
  handle_res_static(call_res,"symnamtovalue")?;
  unsafe { _value__bytes.set_len((MSK_MAX_STR_LEN) as usize) };
  return Result::Ok((String::from_utf8_lossy(&_value__bytes[..]).into_owned()));
}
