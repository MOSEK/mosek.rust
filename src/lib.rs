/*
Copyright (c) 2021 MOSEK ApS. All rights reserved.

Redistribution and use in source and binary forms, with or without modification,
are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice,
this list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
this list of conditions and the following disclaimer in the documentation
and/or other materials provided with the distribution.

3. All advertising materials mentioning features or use of this software must
display the following acknowledgement:
This product includes software developed by the the organization.

4. Neither the name of the copyright holder nor the names of its contributors
may be used to endorse or promote products derived from this software without
specific prior written permission.

THIS SOFTWARE IS PROVIDED BY COPYRIGHT HOLDER "AS IS" AND ANY EXPRESS OR IMPLIED
WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY
AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL COPYRIGHT
HOLDER BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY,
OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE
GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
DAMAGE.
*/


/// MOSEK Solver API for Rust.
///
/// Most functionality is provided through the [Task] object and it's
/// member functions.

// Generted for MOSEK v[10, 1, 0]

extern crate libc;
use std::ffi::CString;
use std::ffi::CStr;
use libc::c_void;
use std::convert::TryInto;
use std::default::Default;
use std::marker::Send;

//#[link(name = "mosek64")]
extern {
    fn MSK_linkfunctotaskstream(task        : * const u8,
                                whichstream : i32,
                                handle      : * const c_void,
                                func        : extern fn (handle : * const c_void, msg : * const libc::c_char)) -> i32;

    fn MSK_putcallbackfunc(task        : * const u8,
                           func        : extern fn (task : * const c_void, handle : * const c_void, caller : i32, douinf : * const f64, intinf : * const i32, lintinf : * const i64) -> i32,
                           handle      : * const c_void) -> i32;
    #[link_name = "MSK_putcallbackfunc"]
    #[allow(clashing_extern_declarations)]
    fn MSK_putcallbackfunc_ptr(task        : * const u8,
                               func        : * const u8,
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
    fn MSK_appendprimalexpconedomain(task_ : * const u8,domidx_ : & mut i64) -> i32;
    fn MSK_appendprimalgeomeanconedomain(task_ : * const u8,n_ : i64,domidx_ : & mut i64) -> i32;
    fn MSK_appendprimalpowerconedomain(task_ : * const u8,n_ : i64,nleft_ : i64,alpha_ : * const f64,domidx_ : & mut i64) -> i32;
    fn MSK_appendquadraticconedomain(task_ : * const u8,n_ : i64,domidx_ : & mut i64) -> i32;
    fn MSK_appendrdomain(task_ : * const u8,n_ : i64,domidx_ : & mut i64) -> i32;
    fn MSK_appendrminusdomain(task_ : * const u8,n_ : i64,domidx_ : & mut i64) -> i32;
    fn MSK_appendrplusdomain(task_ : * const u8,n_ : i64,domidx_ : & mut i64) -> i32;
    fn MSK_appendrquadraticconedomain(task_ : * const u8,n_ : i64,domidx_ : & mut i64) -> i32;
    fn MSK_appendrzerodomain(task_ : * const u8,n_ : i64,domidx_ : & mut i64) -> i32;
    fn MSK_appendsparsesymmat(task_ : * const u8,dim_ : i32,nz_ : i64,subi_ : * const i32,subj_ : * const i32,valij_ : * const f64,idx_ : & mut i64) -> i32;
    fn MSK_appendsparsesymmatlist(task_ : * const u8,num_ : i32,dims_ : * const i32,nz_ : * const i64,subi_ : * const i32,subj_ : * const i32,valij_ : * const f64,idx_ : * mut i64) -> i32;
    fn MSK_appendsvecpsdconedomain(task_ : * const u8,n_ : i64,domidx_ : & mut i64) -> i32;
    fn MSK_appendvars(task_ : * const u8,num_ : i32) -> i32;
    fn MSK_asyncgetlog(task_ : * const u8,addr_ : * const libc::c_char,accesstoken_ : * const libc::c_char,token_ : * const libc::c_char) -> i32;
    fn MSK_asyncgetresult(task_ : * const u8,address_ : * const libc::c_char,accesstoken_ : * const libc::c_char,token_ : * const libc::c_char,respavailable_ : & mut i32,resp_ : &mut i32,trm_ : &mut i32) -> i32;
    fn MSK_asyncoptimize(task_ : * const u8,address_ : * const libc::c_char,accesstoken_ : * const libc::c_char,token : * mut u8) -> i32;
    fn MSK_asyncpoll(task_ : * const u8,address_ : * const libc::c_char,accesstoken_ : * const libc::c_char,token_ : * const libc::c_char,respavailable_ : & mut i32,resp_ : &mut i32,trm_ : &mut i32) -> i32;
    fn MSK_asyncstop(task_ : * const u8,address_ : * const libc::c_char,accesstoken_ : * const libc::c_char,token_ : * const libc::c_char) -> i32;
    fn MSK_basiscond(task_ : * const u8,nrmbasis_ : & mut f64,nrminvbasis_ : & mut f64) -> i32;
    fn MSK_chgconbound(task_ : * const u8,i_ : i32,lower_ : i32,finite_ : i32,value_ : f64) -> i32;
    fn MSK_chgvarbound(task_ : * const u8,j_ : i32,lower_ : i32,finite_ : i32,value_ : f64) -> i32;
    #[allow(dead_code)]
    fn MSK_clonetask(task_ : * const u8,clonedtask_ : & mut * const u8) -> i32;
    fn MSK_commitchanges(task_ : * const u8) -> i32;
    fn MSK_deletesolution(task_ : * const u8,whichsol_ : i32) -> i32;
    #[allow(dead_code)]
    fn MSK_deletetask(task_ : & mut * const u8) -> i32;
    fn MSK_dualsensitivity(task_ : * const u8,numj_ : i32,subj_ : * const i32,leftpricej_ : * mut f64,rightpricej_ : * mut f64,leftrangej_ : * mut f64,rightrangej_ : * mut f64) -> i32;
    fn MSK_emptyafebarfrow(task_ : * const u8,afeidx_ : i64) -> i32;
    fn MSK_emptyafebarfrowlist(task_ : * const u8,numafeidx_ : i64,afeidxlist_ : * const i64) -> i32;
    fn MSK_emptyafefcol(task_ : * const u8,varidx_ : i32) -> i32;
    fn MSK_emptyafefcollist(task_ : * const u8,numvaridx_ : i64,varidx_ : * const i32) -> i32;
    fn MSK_emptyafefrow(task_ : * const u8,afeidx_ : i64) -> i32;
    fn MSK_emptyafefrowlist(task_ : * const u8,numafeidx_ : i64,afeidx_ : * const i64) -> i32;
    fn MSK_evaluateacc(task_ : * const u8,whichsol_ : i32,accidx_ : i64,activity_ : * mut f64) -> i32;
    fn MSK_evaluateaccs(task_ : * const u8,whichsol_ : i32,activity_ : * mut f64) -> i32;
    #[allow(dead_code)]
    fn MSK_freetask(task_ : * const u8,buffer_ : *mut u8) -> i32;
    fn MSK_generateaccnames(task_ : * const u8,num_ : i64,sub_ : * const i64,fmt_ : * const libc::c_char,ndims_ : i32,dims_ : * const i32,sp_ : * const i64,numnamedaxis_ : i32,namedaxisidxs_ : * const i32,numnames_ : i64,names_ : * const * const libc::c_char) -> i32;
    fn MSK_generatebarvarnames(task_ : * const u8,num_ : i32,subj_ : * const i32,fmt_ : * const libc::c_char,ndims_ : i32,dims_ : * const i32,sp_ : * const i64,numnamedaxis_ : i32,namedaxisidxs_ : * const i32,numnames_ : i64,names_ : * const * const libc::c_char) -> i32;
    fn MSK_generateconenames(task_ : * const u8,num_ : i32,subk_ : * const i32,fmt_ : * const libc::c_char,ndims_ : i32,dims_ : * const i32,sp_ : * const i64,numnamedaxis_ : i32,namedaxisidxs_ : * const i32,numnames_ : i64,names_ : * const * const libc::c_char) -> i32;
    fn MSK_generateconnames(task_ : * const u8,num_ : i32,subi_ : * const i32,fmt_ : * const libc::c_char,ndims_ : i32,dims_ : * const i32,sp_ : * const i64,numnamedaxis_ : i32,namedaxisidxs_ : * const i32,numnames_ : i64,names_ : * const * const libc::c_char) -> i32;
    fn MSK_generatedjcnames(task_ : * const u8,num_ : i64,sub_ : * const i64,fmt_ : * const libc::c_char,ndims_ : i32,dims_ : * const i32,sp_ : * const i64,numnamedaxis_ : i32,namedaxisidxs_ : * const i32,numnames_ : i64,names_ : * const * const libc::c_char) -> i32;
    fn MSK_generatevarnames(task_ : * const u8,num_ : i32,subj_ : * const i32,fmt_ : * const libc::c_char,ndims_ : i32,dims_ : * const i32,sp_ : * const i64,numnamedaxis_ : i32,namedaxisidxs_ : * const i32,numnames_ : i64,names_ : * const * const libc::c_char) -> i32;
    fn MSK_getaccafeidxlist(task_ : * const u8,accidx_ : i64,afeidxlist_ : * mut i64) -> i32;
    fn MSK_getaccb(task_ : * const u8,accidx_ : i64,b_ : * mut f64) -> i32;
    fn MSK_getaccbarfblocktriplet(task_ : * const u8,maxnumtrip_ : i64,numtrip_ : & mut i64,acc_afe_ : * mut i64,bar_var_ : * mut i32,blk_row_ : * mut i32,blk_col_ : * mut i32,blk_val_ : * mut f64) -> i32;
    fn MSK_getaccbarfnumblocktriplets(task_ : * const u8,numtrip_ : & mut i64) -> i32;
    fn MSK_getaccdomain(task_ : * const u8,accidx_ : i64,domidx_ : & mut i64) -> i32;
    fn MSK_getaccdoty(task_ : * const u8,whichsol_ : i32,accidx_ : i64,doty_ : * mut f64) -> i32;
    fn MSK_getaccdotys(task_ : * const u8,whichsol_ : i32,doty_ : * mut f64) -> i32;
    fn MSK_getaccfnumnz(task_ : * const u8,accfnnz_ : & mut i64) -> i32;
    fn MSK_getaccftrip(task_ : * const u8,frow_ : * mut i64,fcol_ : * mut i32,fval_ : * mut f64) -> i32;
    fn MSK_getaccgvector(task_ : * const u8,g_ : * mut f64) -> i32;
    fn MSK_getaccn(task_ : * const u8,accidx_ : i64,n_ : & mut i64) -> i32;
    fn MSK_getaccname(task_ : * const u8,accidx_ : i64,sizename_ : i32,name : * mut u8) -> i32;
    fn MSK_getaccnamelen(task_ : * const u8,accidx_ : i64,len_ : & mut i32) -> i32;
    fn MSK_getaccntot(task_ : * const u8,n_ : & mut i64) -> i32;
    fn MSK_getaccs(task_ : * const u8,domidxlist_ : * mut i64,afeidxlist_ : * mut i64,b_ : * mut f64) -> i32;
    fn MSK_getacol(task_ : * const u8,j_ : i32,nzj_ : & mut i32,subj_ : * mut i32,valj_ : * mut f64) -> i32;
    fn MSK_getacolnumnz(task_ : * const u8,i_ : i32,nzj_ : & mut i32) -> i32;
    fn MSK_getacolslice64(task_ : * const u8,first_ : i32,last_ : i32,maxnumnz_ : i64,ptrb_ : * mut i64,ptre_ : * mut i64,sub_ : * mut i32,val_ : * mut f64) -> i32;
    fn MSK_getacolslicenumnz64(task_ : * const u8,first_ : i32,last_ : i32,numnz_ : & mut i64) -> i32;
    fn MSK_getacolslicetrip(task_ : * const u8,first_ : i32,last_ : i32,maxnumnz_ : i64,subi_ : * mut i32,subj_ : * mut i32,val_ : * mut f64) -> i32;
    fn MSK_getafebarfblocktriplet(task_ : * const u8,maxnumtrip_ : i64,numtrip_ : & mut i64,afeidx_ : * mut i64,barvaridx_ : * mut i32,subk_ : * mut i32,subl_ : * mut i32,valkl_ : * mut f64) -> i32;
    fn MSK_getafebarfnumblocktriplets(task_ : * const u8,numtrip_ : & mut i64) -> i32;
    fn MSK_getafebarfnumrowentries(task_ : * const u8,afeidx_ : i64,numentr_ : & mut i32) -> i32;
    fn MSK_getafebarfrow(task_ : * const u8,afeidx_ : i64,barvaridx_ : * mut i32,ptrterm_ : * mut i64,numterm_ : * mut i64,termidx_ : * mut i64,termweight_ : * mut f64) -> i32;
    fn MSK_getafebarfrowinfo(task_ : * const u8,afeidx_ : i64,numentr_ : & mut i32,numterm_ : & mut i64) -> i32;
    fn MSK_getafefnumnz(task_ : * const u8,numnz_ : & mut i64) -> i32;
    fn MSK_getafefrow(task_ : * const u8,afeidx_ : i64,numnz_ : & mut i32,varidx_ : * mut i32,val_ : * mut f64) -> i32;
    fn MSK_getafefrownumnz(task_ : * const u8,afeidx_ : i64,numnz_ : & mut i32) -> i32;
    fn MSK_getafeftrip(task_ : * const u8,afeidx_ : * mut i64,varidx_ : * mut i32,val_ : * mut f64) -> i32;
    fn MSK_getafeg(task_ : * const u8,afeidx_ : i64,g_ : & mut f64) -> i32;
    fn MSK_getafegslice(task_ : * const u8,first_ : i64,last_ : i64,g_ : * mut f64) -> i32;
    fn MSK_getaij(task_ : * const u8,i_ : i32,j_ : i32,aij_ : & mut f64) -> i32;
    fn MSK_getapiecenumnz(task_ : * const u8,firsti_ : i32,lasti_ : i32,firstj_ : i32,lastj_ : i32,numnz_ : & mut i32) -> i32;
    fn MSK_getarow(task_ : * const u8,i_ : i32,nzi_ : & mut i32,subi_ : * mut i32,vali_ : * mut f64) -> i32;
    fn MSK_getarownumnz(task_ : * const u8,i_ : i32,nzi_ : & mut i32) -> i32;
    fn MSK_getarowslice64(task_ : * const u8,first_ : i32,last_ : i32,maxnumnz_ : i64,ptrb_ : * mut i64,ptre_ : * mut i64,sub_ : * mut i32,val_ : * mut f64) -> i32;
    fn MSK_getarowslicenumnz64(task_ : * const u8,first_ : i32,last_ : i32,numnz_ : & mut i64) -> i32;
    fn MSK_getarowslicetrip(task_ : * const u8,first_ : i32,last_ : i32,maxnumnz_ : i64,subi_ : * mut i32,subj_ : * mut i32,val_ : * mut f64) -> i32;
    fn MSK_getatrip(task_ : * const u8,maxnumnz_ : i64,subi_ : * mut i32,subj_ : * mut i32,val_ : * mut f64) -> i32;
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
    fn MSK_getbarvarname(task_ : * const u8,i_ : i32,sizename_ : i32,name : * mut u8) -> i32;
    fn MSK_getbarvarnameindex(task_ : * const u8,somename_ : * const libc::c_char,asgn_ : & mut i32,index_ : & mut i32) -> i32;
    fn MSK_getbarvarnamelen(task_ : * const u8,i_ : i32,len_ : & mut i32) -> i32;
    fn MSK_getbarxj(task_ : * const u8,whichsol_ : i32,j_ : i32,barxj_ : * mut f64) -> i32;
    fn MSK_getbarxslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,slicesize_ : i64,barxslice_ : * mut f64) -> i32;
    fn MSK_getc(task_ : * const u8,c_ : * mut f64) -> i32;
    fn MSK_getcfix(task_ : * const u8,cfix_ : & mut f64) -> i32;
    fn MSK_getcj(task_ : * const u8,j_ : i32,cj_ : & mut f64) -> i32;
    fn MSK_getclist(task_ : * const u8,num_ : i32,subj_ : * const i32,c_ : * mut f64) -> i32;
    fn MSK_getconbound(task_ : * const u8,i_ : i32,bk_ : &mut i32,bl_ : & mut f64,bu_ : & mut f64) -> i32;
    fn MSK_getconboundslice(task_ : * const u8,first_ : i32,last_ : i32,bk_ : * mut i32,bl_ : * mut f64,bu_ : * mut f64) -> i32;
    fn MSK_getcone(task_ : * const u8,k_ : i32,ct_ : &mut i32,conepar_ : & mut f64,nummem_ : & mut i32,submem_ : * mut i32) -> i32;
    fn MSK_getconeinfo(task_ : * const u8,k_ : i32,ct_ : &mut i32,conepar_ : & mut f64,nummem_ : & mut i32) -> i32;
    fn MSK_getconename(task_ : * const u8,i_ : i32,sizename_ : i32,name : * mut u8) -> i32;
    fn MSK_getconenameindex(task_ : * const u8,somename_ : * const libc::c_char,asgn_ : & mut i32,index_ : & mut i32) -> i32;
    fn MSK_getconenamelen(task_ : * const u8,i_ : i32,len_ : & mut i32) -> i32;
    fn MSK_getconname(task_ : * const u8,i_ : i32,sizename_ : i32,name : * mut u8) -> i32;
    fn MSK_getconnameindex(task_ : * const u8,somename_ : * const libc::c_char,asgn_ : & mut i32,index_ : & mut i32) -> i32;
    fn MSK_getconnamelen(task_ : * const u8,i_ : i32,len_ : & mut i32) -> i32;
    fn MSK_getcslice(task_ : * const u8,first_ : i32,last_ : i32,c_ : * mut f64) -> i32;
    fn MSK_getdimbarvarj(task_ : * const u8,j_ : i32,dimbarvarj_ : & mut i32) -> i32;
    fn MSK_getdjcafeidxlist(task_ : * const u8,djcidx_ : i64,afeidxlist_ : * mut i64) -> i32;
    fn MSK_getdjcb(task_ : * const u8,djcidx_ : i64,b_ : * mut f64) -> i32;
    fn MSK_getdjcdomainidxlist(task_ : * const u8,djcidx_ : i64,domidxlist_ : * mut i64) -> i32;
    fn MSK_getdjcname(task_ : * const u8,djcidx_ : i64,sizename_ : i32,name : * mut u8) -> i32;
    fn MSK_getdjcnamelen(task_ : * const u8,djcidx_ : i64,len_ : & mut i32) -> i32;
    fn MSK_getdjcnumafe(task_ : * const u8,djcidx_ : i64,numafe_ : & mut i64) -> i32;
    fn MSK_getdjcnumafetot(task_ : * const u8,numafetot_ : & mut i64) -> i32;
    fn MSK_getdjcnumdomain(task_ : * const u8,djcidx_ : i64,numdomain_ : & mut i64) -> i32;
    fn MSK_getdjcnumdomaintot(task_ : * const u8,numdomaintot_ : & mut i64) -> i32;
    fn MSK_getdjcnumterm(task_ : * const u8,djcidx_ : i64,numterm_ : & mut i64) -> i32;
    fn MSK_getdjcnumtermtot(task_ : * const u8,numtermtot_ : & mut i64) -> i32;
    fn MSK_getdjcs(task_ : * const u8,domidxlist_ : * mut i64,afeidxlist_ : * mut i64,b_ : * mut f64,termsizelist_ : * mut i64,numterms_ : * mut i64) -> i32;
    fn MSK_getdjctermsizelist(task_ : * const u8,djcidx_ : i64,termsizelist_ : * mut i64) -> i32;
    fn MSK_getdomainn(task_ : * const u8,domidx_ : i64,n_ : & mut i64) -> i32;
    fn MSK_getdomainname(task_ : * const u8,domidx_ : i64,sizename_ : i32,name : * mut u8) -> i32;
    fn MSK_getdomainnamelen(task_ : * const u8,domidx_ : i64,len_ : & mut i32) -> i32;
    fn MSK_getdomaintype(task_ : * const u8,domidx_ : i64,domtype_ : &mut i32) -> i32;
    fn MSK_getdouinf(task_ : * const u8,whichdinf_ : i32,dvalue_ : & mut f64) -> i32;
    fn MSK_getdouparam(task_ : * const u8,param_ : i32,parvalue_ : & mut f64) -> i32;
    fn MSK_getdualobj(task_ : * const u8,whichsol_ : i32,dualobj_ : & mut f64) -> i32;
    fn MSK_getdualsolutionnorms(task_ : * const u8,whichsol_ : i32,nrmy_ : & mut f64,nrmslc_ : & mut f64,nrmsuc_ : & mut f64,nrmslx_ : & mut f64,nrmsux_ : & mut f64,nrmsnx_ : & mut f64,nrmbars_ : & mut f64) -> i32;
    fn MSK_getdviolacc(task_ : * const u8,whichsol_ : i32,numaccidx_ : i64,accidxlist_ : * const i64,viol_ : * mut f64) -> i32;
    fn MSK_getdviolbarvar(task_ : * const u8,whichsol_ : i32,num_ : i32,sub_ : * const i32,viol_ : * mut f64) -> i32;
    fn MSK_getdviolcon(task_ : * const u8,whichsol_ : i32,num_ : i32,sub_ : * const i32,viol_ : * mut f64) -> i32;
    fn MSK_getdviolcones(task_ : * const u8,whichsol_ : i32,num_ : i32,sub_ : * const i32,viol_ : * mut f64) -> i32;
    fn MSK_getdviolvar(task_ : * const u8,whichsol_ : i32,num_ : i32,sub_ : * const i32,viol_ : * mut f64) -> i32;
    #[allow(dead_code)]
    fn MSK_getenv(task_ : * const u8,env_ : & mut * const u8) -> i32;
    #[allow(dead_code)]
    fn MSK_getinfeasiblesubproblem(task_ : * const u8,whichsol_ : i32,inftask_ : & mut * const u8) -> i32;
    fn MSK_getinfindex(task_ : * const u8,inftype_ : i32,infname_ : * const libc::c_char,infindex_ : & mut i32) -> i32;
    fn MSK_getinfmax(task_ : * const u8,inftype_ : i32,infmax_ : * mut i32) -> i32;
    fn MSK_getinfname(task_ : * const u8,inftype_ : i32,whichinf_ : i32,infname : * mut u8) -> i32;
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
    fn MSK_getmionumthreads(task_ : * const u8,numthreads_ : & mut i32) -> i32;
    fn MSK_getnadouinf(task_ : * const u8,infitemname_ : * const libc::c_char,dvalue_ : & mut f64) -> i32;
    fn MSK_getnadouparam(task_ : * const u8,paramname_ : * const libc::c_char,parvalue_ : & mut f64) -> i32;
    fn MSK_getnaintinf(task_ : * const u8,infitemname_ : * const libc::c_char,ivalue_ : & mut i32) -> i32;
    fn MSK_getnaintparam(task_ : * const u8,paramname_ : * const libc::c_char,parvalue_ : & mut i32) -> i32;
    fn MSK_getnastrparam(task_ : * const u8,paramname_ : * const libc::c_char,sizeparamname_ : i32,len_ : & mut i32,parvalue : * mut u8) -> i32;
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
    fn MSK_getobjname(task_ : * const u8,sizeobjname_ : i32,objname : * mut u8) -> i32;
    fn MSK_getobjnamelen(task_ : * const u8,len_ : & mut i32) -> i32;
    fn MSK_getobjsense(task_ : * const u8,sense_ : &mut i32) -> i32;
    fn MSK_getparammax(task_ : * const u8,partype_ : i32,parammax_ : & mut i32) -> i32;
    fn MSK_getparamname(task_ : * const u8,partype_ : i32,param_ : i32,parname : * mut u8) -> i32;
    fn MSK_getpowerdomainalpha(task_ : * const u8,domidx_ : i64,alpha_ : * mut f64) -> i32;
    fn MSK_getpowerdomaininfo(task_ : * const u8,domidx_ : i64,n_ : & mut i64,nleft_ : & mut i64) -> i32;
    fn MSK_getprimalobj(task_ : * const u8,whichsol_ : i32,primalobj_ : & mut f64) -> i32;
    fn MSK_getprimalsolutionnorms(task_ : * const u8,whichsol_ : i32,nrmxc_ : & mut f64,nrmxx_ : & mut f64,nrmbarx_ : & mut f64) -> i32;
    fn MSK_getprobtype(task_ : * const u8,probtype_ : &mut i32) -> i32;
    fn MSK_getprosta(task_ : * const u8,whichsol_ : i32,problemsta_ : &mut i32) -> i32;
    fn MSK_getpviolacc(task_ : * const u8,whichsol_ : i32,numaccidx_ : i64,accidxlist_ : * const i64,viol_ : * mut f64) -> i32;
    fn MSK_getpviolbarvar(task_ : * const u8,whichsol_ : i32,num_ : i32,sub_ : * const i32,viol_ : * mut f64) -> i32;
    fn MSK_getpviolcon(task_ : * const u8,whichsol_ : i32,num_ : i32,sub_ : * const i32,viol_ : * mut f64) -> i32;
    fn MSK_getpviolcones(task_ : * const u8,whichsol_ : i32,num_ : i32,sub_ : * const i32,viol_ : * mut f64) -> i32;
    fn MSK_getpvioldjc(task_ : * const u8,whichsol_ : i32,numdjcidx_ : i64,djcidxlist_ : * const i64,viol_ : * mut f64) -> i32;
    fn MSK_getpviolvar(task_ : * const u8,whichsol_ : i32,num_ : i32,sub_ : * const i32,viol_ : * mut f64) -> i32;
    fn MSK_getqconk64(task_ : * const u8,k_ : i32,maxnumqcnz_ : i64,numqcnz_ : & mut i64,qcsubi_ : * mut i32,qcsubj_ : * mut i32,qcval_ : * mut f64) -> i32;
    fn MSK_getqobj64(task_ : * const u8,maxnumqonz_ : i64,numqonz_ : & mut i64,qosubi_ : * mut i32,qosubj_ : * mut i32,qoval_ : * mut f64) -> i32;
    fn MSK_getqobjij(task_ : * const u8,i_ : i32,j_ : i32,qoij_ : & mut f64) -> i32;
    fn MSK_getreducedcosts(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,redcosts_ : * mut f64) -> i32;
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
    fn MSK_getsolsta(task_ : * const u8,whichsol_ : i32,solutionsta_ : &mut i32) -> i32;
    fn MSK_getsolution(task_ : * const u8,whichsol_ : i32,problemsta_ : &mut i32,solutionsta_ : &mut i32,skc_ : * mut i32,skx_ : * mut i32,skn_ : * mut i32,xc_ : * mut f64,xx_ : * mut f64,y_ : * mut f64,slc_ : * mut f64,suc_ : * mut f64,slx_ : * mut f64,sux_ : * mut f64,snx_ : * mut f64) -> i32;
    fn MSK_getsolutioninfo(task_ : * const u8,whichsol_ : i32,pobj_ : & mut f64,pviolcon_ : & mut f64,pviolvar_ : & mut f64,pviolbarvar_ : & mut f64,pviolcone_ : & mut f64,pviolitg_ : & mut f64,dobj_ : & mut f64,dviolcon_ : & mut f64,dviolvar_ : & mut f64,dviolbarvar_ : & mut f64,dviolcone_ : & mut f64) -> i32;
    fn MSK_getsolutioninfonew(task_ : * const u8,whichsol_ : i32,pobj_ : & mut f64,pviolcon_ : & mut f64,pviolvar_ : & mut f64,pviolbarvar_ : & mut f64,pviolcone_ : & mut f64,pviolacc_ : & mut f64,pvioldjc_ : & mut f64,pviolitg_ : & mut f64,dobj_ : & mut f64,dviolcon_ : & mut f64,dviolvar_ : & mut f64,dviolbarvar_ : & mut f64,dviolcone_ : & mut f64,dviolacc_ : & mut f64) -> i32;
    fn MSK_getsolutionnew(task_ : * const u8,whichsol_ : i32,problemsta_ : &mut i32,solutionsta_ : &mut i32,skc_ : * mut i32,skx_ : * mut i32,skn_ : * mut i32,xc_ : * mut f64,xx_ : * mut f64,y_ : * mut f64,slc_ : * mut f64,suc_ : * mut f64,slx_ : * mut f64,sux_ : * mut f64,snx_ : * mut f64,doty_ : * mut f64) -> i32;
    fn MSK_getsolutionslice(task_ : * const u8,whichsol_ : i32,solitem_ : i32,first_ : i32,last_ : i32,values_ : * mut f64) -> i32;
    fn MSK_getsparsesymmat(task_ : * const u8,idx_ : i64,maxlen_ : i64,subi_ : * mut i32,subj_ : * mut i32,valij_ : * mut f64) -> i32;
    fn MSK_getstrparam(task_ : * const u8,param_ : i32,maxlen_ : i32,len_ : & mut i32,parvalue : * mut u8) -> i32;
    fn MSK_getstrparamlen(task_ : * const u8,param_ : i32,len_ : & mut i32) -> i32;
    fn MSK_getsuc(task_ : * const u8,whichsol_ : i32,suc_ : * mut f64) -> i32;
    fn MSK_getsucslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,suc_ : * mut f64) -> i32;
    fn MSK_getsux(task_ : * const u8,whichsol_ : i32,sux_ : * mut f64) -> i32;
    fn MSK_getsuxslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,sux_ : * mut f64) -> i32;
    fn MSK_getsymbcon(task_ : * const u8,i_ : i32,sizevalue_ : i32,name : * mut u8,value_ : & mut i32) -> i32;
    fn MSK_getsymmatinfo(task_ : * const u8,idx_ : i64,dim_ : & mut i32,nz_ : & mut i64,mattype_ : &mut i32) -> i32;
    fn MSK_gettaskname(task_ : * const u8,sizetaskname_ : i32,taskname : * mut u8) -> i32;
    fn MSK_gettasknamelen(task_ : * const u8,len_ : & mut i32) -> i32;
    fn MSK_getvarbound(task_ : * const u8,i_ : i32,bk_ : &mut i32,bl_ : & mut f64,bu_ : & mut f64) -> i32;
    fn MSK_getvarboundslice(task_ : * const u8,first_ : i32,last_ : i32,bk_ : * mut i32,bl_ : * mut f64,bu_ : * mut f64) -> i32;
    fn MSK_getvarname(task_ : * const u8,j_ : i32,sizename_ : i32,name : * mut u8) -> i32;
    fn MSK_getvarnameindex(task_ : * const u8,somename_ : * const libc::c_char,asgn_ : & mut i32,index_ : & mut i32) -> i32;
    fn MSK_getvarnamelen(task_ : * const u8,i_ : i32,len_ : & mut i32) -> i32;
    fn MSK_getvartype(task_ : * const u8,j_ : i32,vartype_ : &mut i32) -> i32;
    fn MSK_getvartypelist(task_ : * const u8,num_ : i32,subj_ : * const i32,vartype_ : * mut i32) -> i32;
    fn MSK_getxc(task_ : * const u8,whichsol_ : i32,xc_ : * mut f64) -> i32;
    fn MSK_getxcslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,xc_ : * mut f64) -> i32;
    fn MSK_getxx(task_ : * const u8,whichsol_ : i32,xx_ : * mut f64) -> i32;
    fn MSK_getxxslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,xx_ : * mut f64) -> i32;
    fn MSK_gety(task_ : * const u8,whichsol_ : i32,y_ : * mut f64) -> i32;
    fn MSK_getyslice(task_ : * const u8,whichsol_ : i32,first_ : i32,last_ : i32,y_ : * mut f64) -> i32;
    fn MSK_infeasibilityreport(task_ : * const u8,whichstream_ : i32,whichsol_ : i32) -> i32;
    fn MSK_initbasissolve(task_ : * const u8,basis_ : * mut i32) -> i32;
    fn MSK_inputdata64(task_ : * const u8,maxnumcon_ : i32,maxnumvar_ : i32,numcon_ : i32,numvar_ : i32,c_ : * const f64,cfix_ : f64,aptrb_ : * const i64,aptre_ : * const i64,asub_ : * const i32,aval_ : * const f64,bkc_ : * const i32,blc_ : * const f64,buc_ : * const f64,bkx_ : * const i32,blx_ : * const f64,bux_ : * const f64) -> i32;
    fn MSK_isdouparname(task_ : * const u8,parname_ : * const libc::c_char,param_ : &mut i32) -> i32;
    fn MSK_isintparname(task_ : * const u8,parname_ : * const libc::c_char,param_ : &mut i32) -> i32;
    fn MSK_isstrparname(task_ : * const u8,parname_ : * const libc::c_char,param_ : &mut i32) -> i32;
    fn MSK_linkfiletotaskstream(task_ : * const u8,whichstream_ : i32,filename_ : * const libc::c_char,append_ : i32) -> i32;
    fn MSK_onesolutionsummary(task_ : * const u8,whichstream_ : i32,whichsol_ : i32) -> i32;
    fn MSK_optimizermt(task_ : * const u8,address_ : * const libc::c_char,accesstoken_ : * const libc::c_char,trmcode_ : &mut i32) -> i32;
    fn MSK_optimizersummary(task_ : * const u8,whichstream_ : i32) -> i32;
    fn MSK_optimizetrm(task_ : * const u8,trmcode_ : &mut i32) -> i32;
    fn MSK_primalrepair(task_ : * const u8,wlc_ : * const f64,wuc_ : * const f64,wlx_ : * const f64,wux_ : * const f64) -> i32;
    fn MSK_primalsensitivity(task_ : * const u8,numi_ : i32,subi_ : * const i32,marki_ : * const i32,numj_ : i32,subj_ : * const i32,markj_ : * const i32,leftpricei_ : * mut f64,rightpricei_ : * mut f64,leftrangei_ : * mut f64,rightrangei_ : * mut f64,leftpricej_ : * mut f64,rightpricej_ : * mut f64,leftrangej_ : * mut f64,rightrangej_ : * mut f64) -> i32;
    fn MSK_printparam(task_ : * const u8) -> i32;
    fn MSK_putacc(task_ : * const u8,accidx_ : i64,domidx_ : i64,numafeidx_ : i64,afeidxlist_ : * const i64,b_ : * const f64) -> i32;
    fn MSK_putaccb(task_ : * const u8,accidx_ : i64,lengthb_ : i64,b_ : * const f64) -> i32;
    fn MSK_putaccbj(task_ : * const u8,accidx_ : i64,j_ : i64,bj_ : f64) -> i32;
    fn MSK_putaccdoty(task_ : * const u8,whichsol_ : i32,accidx_ : i64,doty_ : * mut f64) -> i32;
    fn MSK_putacclist(task_ : * const u8,numaccs_ : i64,accidxs_ : * const i64,domidxs_ : * const i64,numafeidx_ : i64,afeidxlist_ : * const i64,b_ : * const f64) -> i32;
    fn MSK_putaccname(task_ : * const u8,accidx_ : i64,name_ : * const libc::c_char) -> i32;
    fn MSK_putacol(task_ : * const u8,j_ : i32,nzj_ : i32,subj_ : * const i32,valj_ : * const f64) -> i32;
    fn MSK_putacollist64(task_ : * const u8,num_ : i32,sub_ : * const i32,ptrb_ : * const i64,ptre_ : * const i64,asub_ : * const i32,aval_ : * const f64) -> i32;
    fn MSK_putacolslice64(task_ : * const u8,first_ : i32,last_ : i32,ptrb_ : * const i64,ptre_ : * const i64,asub_ : * const i32,aval_ : * const f64) -> i32;
    fn MSK_putafebarfblocktriplet(task_ : * const u8,numtrip_ : i64,afeidx_ : * const i64,barvaridx_ : * const i32,subk_ : * const i32,subl_ : * const i32,valkl_ : * const f64) -> i32;
    fn MSK_putafebarfentry(task_ : * const u8,afeidx_ : i64,barvaridx_ : i32,numterm_ : i64,termidx_ : * const i64,termweight_ : * const f64) -> i32;
    fn MSK_putafebarfentrylist(task_ : * const u8,numafeidx_ : i64,afeidx_ : * const i64,barvaridx_ : * const i32,numterm_ : * const i64,ptrterm_ : * const i64,lenterm_ : i64,termidx_ : * const i64,termweight_ : * const f64) -> i32;
    fn MSK_putafebarfrow(task_ : * const u8,afeidx_ : i64,numentr_ : i32,barvaridx_ : * const i32,numterm_ : * const i64,ptrterm_ : * const i64,lenterm_ : i64,termidx_ : * const i64,termweight_ : * const f64) -> i32;
    fn MSK_putafefcol(task_ : * const u8,varidx_ : i32,numnz_ : i64,afeidx_ : * const i64,val_ : * const f64) -> i32;
    fn MSK_putafefentry(task_ : * const u8,afeidx_ : i64,varidx_ : i32,value_ : f64) -> i32;
    fn MSK_putafefentrylist(task_ : * const u8,numentr_ : i64,afeidx_ : * const i64,varidx_ : * const i32,val_ : * const f64) -> i32;
    fn MSK_putafefrow(task_ : * const u8,afeidx_ : i64,numnz_ : i32,varidx_ : * const i32,val_ : * const f64) -> i32;
    fn MSK_putafefrowlist(task_ : * const u8,numafeidx_ : i64,afeidx_ : * const i64,numnzrow_ : * const i32,ptrrow_ : * const i64,lenidxval_ : i64,varidx_ : * const i32,val_ : * const f64) -> i32;
    fn MSK_putafeg(task_ : * const u8,afeidx_ : i64,g_ : f64) -> i32;
    fn MSK_putafeglist(task_ : * const u8,numafeidx_ : i64,afeidx_ : * const i64,g_ : * const f64) -> i32;
    fn MSK_putafegslice(task_ : * const u8,first_ : i64,last_ : i64,slice_ : * const f64) -> i32;
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
    fn MSK_readbsolution(task_ : * const u8,filename_ : * const libc::c_char,compress_ : i32) -> i32;
    fn MSK_readdataautoformat(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
    fn MSK_readdataformat(task_ : * const u8,filename_ : * const libc::c_char,format_ : i32,compress_ : i32) -> i32;
    fn MSK_readjsonsol(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
    fn MSK_readjsonstring(task_ : * const u8,data_ : * const libc::c_char) -> i32;
    fn MSK_readlpstring(task_ : * const u8,data_ : * const libc::c_char) -> i32;
    fn MSK_readopfstring(task_ : * const u8,data_ : * const libc::c_char) -> i32;
    fn MSK_readparamfile(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
    fn MSK_readptfstring(task_ : * const u8,data_ : * const libc::c_char) -> i32;
    fn MSK_readsolution(task_ : * const u8,whichsol_ : i32,filename_ : * const libc::c_char) -> i32;
    fn MSK_readsolutionfile(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
    fn MSK_readsummary(task_ : * const u8,whichstream_ : i32) -> i32;
    fn MSK_readtask(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
    fn MSK_removebarvars(task_ : * const u8,num_ : i32,subset_ : * const i32) -> i32;
    fn MSK_removecones(task_ : * const u8,num_ : i32,subset_ : * const i32) -> i32;
    fn MSK_removecons(task_ : * const u8,num_ : i32,subset_ : * const i32) -> i32;
    fn MSK_removevars(task_ : * const u8,num_ : i32,subset_ : * const i32) -> i32;
    fn MSK_resizetask(task_ : * const u8,maxnumcon_ : i32,maxnumvar_ : i32,maxnumcone_ : i32,maxnumanz_ : i64,maxnumqnz_ : i64) -> i32;
    fn MSK_sensitivityreport(task_ : * const u8,whichstream_ : i32) -> i32;
    fn MSK_setdefaults(task_ : * const u8) -> i32;
    fn MSK_solutiondef(task_ : * const u8,whichsol_ : i32,isdef_ : & mut i32) -> i32;
    fn MSK_solutionsummary(task_ : * const u8,whichstream_ : i32) -> i32;
    fn MSK_solvewithbasis(task_ : * const u8,transp_ : bool,numnz_ : i32,sub_ : * mut i32,val_ : * mut f64,numnzout_ : & mut i32) -> i32;
    fn MSK_strtoconetype(task_ : * const u8,str_ : * const libc::c_char,conetype_ : &mut i32) -> i32;
    fn MSK_strtosk(task_ : * const u8,str_ : * const libc::c_char,sk_ : &mut i32) -> i32;
    fn MSK_toconic(task_ : * const u8) -> i32;
    fn MSK_unlinkfuncfromtaskstream(task_ : * const u8,whichstream_ : i32) -> i32;
    fn MSK_updatesolutioninfo(task_ : * const u8,whichsol_ : i32) -> i32;
    fn MSK_whichparam(task_ : * const u8,parname_ : * const libc::c_char,partype_ : &mut i32,param_ : & mut i32) -> i32;
    fn MSK_writebsolution(task_ : * const u8,filename_ : * const libc::c_char,compress_ : i32) -> i32;
    fn MSK_writedata(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
    fn MSK_writejsonsol(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
    fn MSK_writeparamfile(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
    fn MSK_writesolution(task_ : * const u8,whichsol_ : i32,filename_ : * const libc::c_char) -> i32;
    fn MSK_writesolutionfile(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
    fn MSK_writestat(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
    fn MSK_writetask(task_ : * const u8,filename_ : * const libc::c_char) -> i32;
    fn MSK_writetasksolverresult_file(task_ : * const u8,filename_ : * const libc::c_char,compress_ : i32) -> i32;
    fn MSK_axpy(env_ : * const u8,n_ : i32,alpha_ : f64,x_ : * const f64,y_ : * mut f64) -> i32;
    fn MSK_checkinall(env_ : * const u8) -> i32;
    fn MSK_checkinlicense(env_ : * const u8,feature_ : i32) -> i32;
    fn MSK_checkoutlicense(env_ : * const u8,feature_ : i32) -> i32;
    fn MSK_checkversion(env_ : * const u8,major_ : i32,minor_ : i32,revision_ : i32) -> i32;
    fn MSK_computesparsecholesky(env_ : * const u8,numthreads_ : i32,ordermethod_ : i32,tolsingular_ : f64,n_ : i32,anzc_ : * const i32,aptrc_ : * const i64,asubc_ : * const i32,avalc_ : * const f64,perm_ : & mut * const i32,diag_ : & mut * const f64,lnzc_ : & mut * const i32,lptrc_ : & mut * const i64,lensubnval_ : & mut i64,lsubc_ : & mut * const i32,lvalc_ : & mut * const f64) -> i32;
    #[allow(dead_code)]
    fn MSK_deleteenv(env_ : & mut * const u8) -> i32;
    fn MSK_dot(env_ : * const u8,n_ : i32,x_ : * const f64,y_ : * const f64,xty_ : & mut f64) -> i32;
    fn MSK_echointro(env_ : * const u8,longver_ : i32) -> i32;
    fn MSK_enablegarcolenv(env_ : * const u8) -> i32;
    fn MSK_expirylicenses(env_ : * const u8,expiry_ : & mut i64) -> i32;
    #[allow(dead_code)]
    fn MSK_freeenv(env_ : * const u8,buffer_ : *mut u8) -> i32;
    fn MSK_gemm(env_ : * const u8,transa_ : i32,transb_ : i32,m_ : i32,n_ : i32,k_ : i32,alpha_ : f64,a_ : * const f64,b_ : * const f64,beta_ : f64,c_ : * mut f64) -> i32;
    fn MSK_gemv(env_ : * const u8,transa_ : i32,m_ : i32,n_ : i32,alpha_ : f64,a_ : * const f64,x_ : * const f64,beta_ : f64,y_ : * mut f64) -> i32;
    fn MSK_getbuildinfo(buildstate : * mut u8,builddate : * mut u8) -> i32;
    fn MSK_getcodedesc(code_ : i32,symname : * mut u8,str : * mut u8) -> i32;
    fn MSK_getresponseclass(r_ : i32,rc_ : &mut i32) -> i32;
    fn MSK_getversion(major_ : & mut i32,minor_ : & mut i32,revision_ : & mut i32) -> i32;
    fn MSK_isinfinity(value_ : f64) -> i32;
    fn MSK_licensecleanup() -> i32;
    fn MSK_linkfiletoenvstream(env_ : * const u8,whichstream_ : i32,filename_ : * const libc::c_char,append_ : i32) -> i32;
    #[allow(dead_code)]
    fn MSK_makeemptytask(env_ : * const u8,task_ : & mut * const u8) -> i32;
    #[allow(dead_code)]
    fn MSK_makeenv(env_ : & mut * const u8,dbgfile_ : * const libc::c_char) -> i32;
    #[allow(dead_code)]
    fn MSK_makeenvdebug(env_ : & mut * const u8,maxnumalloc_ : i64,dbgfile_ : * const libc::c_char) -> i32;
    #[allow(dead_code)]
    fn MSK_maketask(env_ : * const u8,maxnumcon_ : i32,maxnumvar_ : i32,task_ : & mut * const u8) -> i32;
    fn MSK_optimizebatch(env_ : * const u8,israce_ : bool,maxtime_ : f64,numthreads_ : i32,numtask_ : i64,task_ : * const * const u8,trmcode_ : * mut i32,rcode_ : * mut i32) -> i32;
    fn MSK_potrf(env_ : * const u8,uplo_ : i32,n_ : i32,a_ : * mut f64) -> i32;
    fn MSK_putlicensecode(env_ : * const u8,code_ : * const i32) -> i32;
    fn MSK_putlicensedebug(env_ : * const u8,licdebug_ : i32) -> i32;
    fn MSK_putlicensepath(env_ : * const u8,licensepath_ : * const libc::c_char) -> i32;
    fn MSK_putlicensewait(env_ : * const u8,licwait_ : i32) -> i32;
    fn MSK_resetexpirylicenses(env_ : * const u8) -> i32;
    fn MSK_sparsetriangularsolvedense(env_ : * const u8,transposed_ : i32,n_ : i32,lnzc_ : * const i32,lptrc_ : * const i64,lensubnval_ : i64,lsubc_ : * const i32,lvalc_ : * const f64,b_ : * mut f64) -> i32;
    fn MSK_syeig(env_ : * const u8,uplo_ : i32,n_ : i32,a_ : * const f64,w_ : * mut f64) -> i32;
    fn MSK_syevd(env_ : * const u8,uplo_ : i32,n_ : i32,a_ : * mut f64,w_ : * mut f64) -> i32;
    fn MSK_symnamtovalue(name_ : * const libc::c_char,value : * mut u8) -> i32;
    fn MSK_syrk(env_ : * const u8,uplo_ : i32,trans_ : i32,n_ : i32,k_ : i32,alpha_ : f64,a_ : * const f64,beta_ : f64,c_ : * mut f64) -> i32;

}


/// Basis identification
#[non_exhaustive]
pub struct Basindtype;
impl Basindtype {
  /// Never do basis identification.
  pub const NEVER : i32 = 0;
  /// Basis identification is always performed even if the interior-point optimizer terminates abnormally.
  pub const ALWAYS : i32 = 1;
  /// Basis identification is performed if the interior-point optimizer terminates without an error.
  pub const NO_ERROR : i32 = 2;
  /// Basis identification is not performed if the interior-point optimizer terminates with a problem status saying that the problem is primal or dual infeasible.
  pub const IF_FEASIBLE : i32 = 3;
  /// Not currently in use.
  pub const RESERVERED : i32 = 4;
} // impl Basindtype

/// Bound keys
#[non_exhaustive]
pub struct Boundkey;
impl Boundkey {
  /// The constraint or variable has a finite lower bound and an infinite upper bound.
  pub const LO : i32 = 0;
  /// The constraint or variable has an infinite lower bound and an finite upper bound.
  pub const UP : i32 = 1;
  /// The constraint or variable is fixed.
  pub const FX : i32 = 2;
  /// The constraint or variable is free.
  pub const FR : i32 = 3;
  /// The constraint or variable is ranged.
  pub const RA : i32 = 4;
} // impl Boundkey

/// Mark
#[non_exhaustive]
pub struct Mark;
impl Mark {
  /// The lower bound is selected for sensitivity analysis.
  pub const LO : i32 = 0;
  /// The upper bound is selected for sensitivity analysis.
  pub const UP : i32 = 1;
} // impl Mark

/// Degeneracy strategies
#[non_exhaustive]
pub struct Simdegen;
impl Simdegen {
  /// The simplex optimizer should use no degeneration strategy.
  pub const NONE : i32 = 0;
  /// The simplex optimizer chooses the degeneration strategy.
  pub const FREE : i32 = 1;
  /// The simplex optimizer should use an aggressive degeneration strategy.
  pub const AGGRESSIVE : i32 = 2;
  /// The simplex optimizer should use a moderate degeneration strategy.
  pub const MODERATE : i32 = 3;
  /// The simplex optimizer should use a minimum degeneration strategy.
  pub const MINIMUM : i32 = 4;
} // impl Simdegen

/// Transposed matrix.
#[non_exhaustive]
pub struct Transpose;
impl Transpose {
  /// No transpose is applied.
  pub const NO : i32 = 0;
  /// A transpose is applied.
  pub const YES : i32 = 1;
} // impl Transpose

/// Triangular part of a symmetric matrix.
#[non_exhaustive]
pub struct Uplo;
impl Uplo {
  /// Lower part.
  pub const LO : i32 = 0;
  /// Upper part.
  pub const UP : i32 = 1;
} // impl Uplo

/// Problem reformulation.
#[non_exhaustive]
pub struct Simreform;
impl Simreform {
  /// Disallow the simplex optimizer to reformulate the problem.
  pub const OFF : i32 = 0;
  /// Allow the simplex optimizer to reformulate the problem.
  pub const ON : i32 = 1;
  /// The simplex optimizer can choose freely.
  pub const FREE : i32 = 2;
  /// The simplex optimizer should use an aggressive reformulation strategy.
  pub const AGGRESSIVE : i32 = 3;
} // impl Simreform

/// Exploit duplicate columns.
#[non_exhaustive]
pub struct Simdupvec;
impl Simdupvec {
  /// Disallow the simplex optimizer to exploit duplicated columns.
  pub const OFF : i32 = 0;
  /// Allow the simplex optimizer to exploit duplicated columns.
  pub const ON : i32 = 1;
  /// The simplex optimizer can choose freely.
  pub const FREE : i32 = 2;
} // impl Simdupvec

/// Hot-start type employed by the simplex optimizer
#[non_exhaustive]
pub struct Simhotstart;
impl Simhotstart {
  /// The simplex optimizer performs a coldstart.
  pub const NONE : i32 = 0;
  /// The simplex optimize chooses the hot-start type.
  pub const FREE : i32 = 1;
  /// Only the status keys of the constraints and variables are used to choose the type of hot-start.
  pub const STATUS_KEYS : i32 = 2;
} // impl Simhotstart

/// Hot-start type employed by the interior-point optimizers.
#[non_exhaustive]
pub struct Intpnthotstart;
impl Intpnthotstart {
  /// The interior-point optimizer performs a coldstart.
  pub const NONE : i32 = 0;
  /// The interior-point optimizer exploits the primal solution only.
  pub const PRIMAL : i32 = 1;
  /// The interior-point optimizer exploits the dual solution only.
  pub const DUAL : i32 = 2;
  /// The interior-point optimizer exploits both the primal and dual solution.
  pub const PRIMAL_DUAL : i32 = 3;
} // impl Intpnthotstart

/// Solution purification employed optimizer.
#[non_exhaustive]
pub struct Purify;
impl Purify {
  /// The optimizer performs no solution purification.
  pub const NONE : i32 = 0;
  /// The optimizer purifies the primal solution.
  pub const PRIMAL : i32 = 1;
  /// The optimizer purifies the dual solution.
  pub const DUAL : i32 = 2;
  /// The optimizer purifies both the primal and dual solution.
  pub const PRIMAL_DUAL : i32 = 3;
  /// TBD
  pub const AUTO : i32 = 4;
} // impl Purify

/// Progress callback codes
#[non_exhaustive]
pub struct Callbackcode;
impl Callbackcode {
  /// The basis identification procedure has been started.
  pub const BEGIN_BI : i32 = 0;
  /// The callback function is called when the conic optimizer is started.
  pub const BEGIN_CONIC : i32 = 1;
  /// The callback function is called from within the basis identification procedure when the dual phase is started.
  pub const BEGIN_DUAL_BI : i32 = 2;
  /// Dual sensitivity analysis is started.
  pub const BEGIN_DUAL_SENSITIVITY : i32 = 3;
  /// The callback function is called when the dual BI phase is started.
  pub const BEGIN_DUAL_SETUP_BI : i32 = 4;
  /// The callback function is called when the dual simplex optimizer started.
  pub const BEGIN_DUAL_SIMPLEX : i32 = 5;
  /// The callback function is called from within the basis identification procedure when the dual simplex clean-up phase is started.
  pub const BEGIN_DUAL_SIMPLEX_BI : i32 = 6;
  /// The callback function is called when the infeasibility analyzer is started.
  pub const BEGIN_INFEAS_ANA : i32 = 7;
  /// The callback function is called when the interior-point optimizer is started.
  pub const BEGIN_INTPNT : i32 = 8;
  /// Begin waiting for license.
  pub const BEGIN_LICENSE_WAIT : i32 = 9;
  /// The callback function is called when the mixed-integer optimizer is started.
  pub const BEGIN_MIO : i32 = 10;
  /// The callback function is called when the optimizer is started.
  pub const BEGIN_OPTIMIZER : i32 = 11;
  /// The callback function is called when the presolve is started.
  pub const BEGIN_PRESOLVE : i32 = 12;
  /// The callback function is called from within the basis identification procedure when the primal phase is started.
  pub const BEGIN_PRIMAL_BI : i32 = 13;
  /// Begin primal feasibility repair.
  pub const BEGIN_PRIMAL_REPAIR : i32 = 14;
  /// Primal sensitivity analysis is started.
  pub const BEGIN_PRIMAL_SENSITIVITY : i32 = 15;
  /// The callback function is called when the primal BI setup is started.
  pub const BEGIN_PRIMAL_SETUP_BI : i32 = 16;
  /// The callback function is called when the primal simplex optimizer is started.
  pub const BEGIN_PRIMAL_SIMPLEX : i32 = 17;
  /// The callback function is called from within the basis identification procedure when the primal simplex clean-up phase is started.
  pub const BEGIN_PRIMAL_SIMPLEX_BI : i32 = 18;
  /// Begin QCQO reformulation.
  pub const BEGIN_QCQO_REFORMULATE : i32 = 19;
  /// MOSEK has started reading a problem file.
  pub const BEGIN_READ : i32 = 20;
  /// The callback function is called when root cut generation is started.
  pub const BEGIN_ROOT_CUTGEN : i32 = 21;
  /// The callback function is called when the simplex optimizer is started.
  pub const BEGIN_SIMPLEX : i32 = 22;
  /// The callback function is called from within the basis identification procedure when the simplex clean-up phase is started.
  pub const BEGIN_SIMPLEX_BI : i32 = 23;
  /// The callback function is called when solution of root relaxation is started.
  pub const BEGIN_SOLVE_ROOT_RELAX : i32 = 24;
  /// Begin conic reformulation.
  pub const BEGIN_TO_CONIC : i32 = 25;
  /// MOSEK has started writing a problem file.
  pub const BEGIN_WRITE : i32 = 26;
  /// The callback function is called from within the conic optimizer after the information database has been updated.
  pub const CONIC : i32 = 27;
  /// The callback function is called from within the dual simplex optimizer.
  pub const DUAL_SIMPLEX : i32 = 28;
  /// The callback function is called when the basis identification procedure is terminated.
  pub const END_BI : i32 = 29;
  /// The callback function is called when the conic optimizer is terminated.
  pub const END_CONIC : i32 = 30;
  /// The callback function is called from within the basis identification procedure when the dual phase is terminated.
  pub const END_DUAL_BI : i32 = 31;
  /// Dual sensitivity analysis is terminated.
  pub const END_DUAL_SENSITIVITY : i32 = 32;
  /// The callback function is called when the dual BI phase is terminated.
  pub const END_DUAL_SETUP_BI : i32 = 33;
  /// The callback function is called when the dual simplex optimizer is terminated.
  pub const END_DUAL_SIMPLEX : i32 = 34;
  /// The callback function is called from within the basis identification procedure when the dual clean-up phase is terminated.
  pub const END_DUAL_SIMPLEX_BI : i32 = 35;
  /// The callback function is called when the infeasibility analyzer is terminated.
  pub const END_INFEAS_ANA : i32 = 36;
  /// The callback function is called when the interior-point optimizer is terminated.
  pub const END_INTPNT : i32 = 37;
  /// End waiting for license.
  pub const END_LICENSE_WAIT : i32 = 38;
  /// The callback function is called when the mixed-integer optimizer is terminated.
  pub const END_MIO : i32 = 39;
  /// The callback function is called when the optimizer is terminated.
  pub const END_OPTIMIZER : i32 = 40;
  /// The callback function is called when the presolve is completed.
  pub const END_PRESOLVE : i32 = 41;
  /// The callback function is called from within the basis identification procedure when the primal phase is terminated.
  pub const END_PRIMAL_BI : i32 = 42;
  /// End primal feasibility repair.
  pub const END_PRIMAL_REPAIR : i32 = 43;
  /// Primal sensitivity analysis is terminated.
  pub const END_PRIMAL_SENSITIVITY : i32 = 44;
  /// The callback function is called when the primal BI setup is terminated.
  pub const END_PRIMAL_SETUP_BI : i32 = 45;
  /// The callback function is called when the primal simplex optimizer is terminated.
  pub const END_PRIMAL_SIMPLEX : i32 = 46;
  /// The callback function is called from within the basis identification procedure when the primal clean-up phase is terminated.
  pub const END_PRIMAL_SIMPLEX_BI : i32 = 47;
  /// End QCQO reformulation.
  pub const END_QCQO_REFORMULATE : i32 = 48;
  /// MOSEK has finished reading a problem file.
  pub const END_READ : i32 = 49;
  /// The callback function is called when root cut generation is terminated.
  pub const END_ROOT_CUTGEN : i32 = 50;
  /// The callback function is called when the simplex optimizer is terminated.
  pub const END_SIMPLEX : i32 = 51;
  /// The callback function is called from within the basis identification procedure when the simplex clean-up phase is terminated.
  pub const END_SIMPLEX_BI : i32 = 52;
  /// The callback function is called when solution of root relaxation is terminated.
  pub const END_SOLVE_ROOT_RELAX : i32 = 53;
  /// End conic reformulation.
  pub const END_TO_CONIC : i32 = 54;
  /// MOSEK has finished writing a problem file.
  pub const END_WRITE : i32 = 55;
  /// The callback function is called from within the basis identification procedure at an intermediate point.
  pub const IM_BI : i32 = 56;
  /// The callback function is called at an intermediate stage within the conic optimizer where the information database has not been updated.
  pub const IM_CONIC : i32 = 57;
  /// The callback function is called from within the basis identification procedure at an intermediate point in the dual phase.
  pub const IM_DUAL_BI : i32 = 58;
  /// The callback function is called at an intermediate stage of the dual sensitivity analysis.
  pub const IM_DUAL_SENSIVITY : i32 = 59;
  /// The callback function is called at an intermediate point in the dual simplex optimizer.
  pub const IM_DUAL_SIMPLEX : i32 = 60;
  /// The callback function is called at an intermediate stage within the interior-point optimizer where the information database has not been updated.
  pub const IM_INTPNT : i32 = 61;
  /// MOSEK is waiting for a license.
  pub const IM_LICENSE_WAIT : i32 = 62;
  /// The callback function is called from within the LU factorization procedure at an intermediate point.
  pub const IM_LU : i32 = 63;
  /// The callback function is called at an intermediate point in the mixed-integer optimizer.
  pub const IM_MIO : i32 = 64;
  /// The callback function is called at an intermediate point in the mixed-integer optimizer while running the dual simplex optimizer.
  pub const IM_MIO_DUAL_SIMPLEX : i32 = 65;
  /// The callback function is called at an intermediate point in the mixed-integer optimizer while running the interior-point optimizer.
  pub const IM_MIO_INTPNT : i32 = 66;
  /// The callback function is called at an intermediate point in the mixed-integer optimizer while running the primal simplex optimizer.
  pub const IM_MIO_PRIMAL_SIMPLEX : i32 = 67;
  /// The callback function is called from within the matrix ordering procedure at an intermediate point.
  pub const IM_ORDER : i32 = 68;
  /// The callback function is called from within the presolve procedure at an intermediate stage.
  pub const IM_PRESOLVE : i32 = 69;
  /// The callback function is called from within the basis identification procedure at an intermediate point in the primal phase.
  pub const IM_PRIMAL_BI : i32 = 70;
  /// The callback function is called at an intermediate stage of the primal sensitivity analysis.
  pub const IM_PRIMAL_SENSIVITY : i32 = 71;
  /// The callback function is called at an intermediate point in the primal simplex optimizer.
  pub const IM_PRIMAL_SIMPLEX : i32 = 72;
  /// The callback function is called at an intermediate stage of the conic quadratic reformulation.
  pub const IM_QO_REFORMULATE : i32 = 73;
  /// Intermediate stage in reading.
  pub const IM_READ : i32 = 74;
  /// The callback is called from within root cut generation at an intermediate stage.
  pub const IM_ROOT_CUTGEN : i32 = 75;
  /// The callback function is called from within the simplex optimizer at an intermediate point.
  pub const IM_SIMPLEX : i32 = 76;
  /// The callback function is called from within the basis identification procedure at an intermediate point in the simplex clean-up phase.
  pub const IM_SIMPLEX_BI : i32 = 77;
  /// The callback function is called from within the interior-point optimizer after the information database has been updated.
  pub const INTPNT : i32 = 78;
  /// The callback function is called after a new integer solution has been located by the mixed-integer optimizer.
  pub const NEW_INT_MIO : i32 = 79;
  /// The callback function is called from within the primal simplex optimizer.
  pub const PRIMAL_SIMPLEX : i32 = 80;
  /// The callback function is called from the OPF reader.
  pub const READ_OPF : i32 = 81;
  /// A chunk of Q non-zeros has been read from a problem file.
  pub const READ_OPF_SECTION : i32 = 82;
  /// The callback function is called while the task is being solved on a remote server.
  pub const SOLVING_REMOTE : i32 = 83;
  /// The callback function is called from within the basis identification procedure at an intermediate point in the dual phase.
  pub const UPDATE_DUAL_BI : i32 = 84;
  /// The callback function is called in the dual simplex optimizer.
  pub const UPDATE_DUAL_SIMPLEX : i32 = 85;
  /// The callback function is called from within the basis identification procedure at an intermediate point in the dual simplex clean-up phase.
  pub const UPDATE_DUAL_SIMPLEX_BI : i32 = 86;
  /// The callback function is called from within the presolve procedure.
  pub const UPDATE_PRESOLVE : i32 = 87;
  /// The callback function is called from within the basis identification procedure at an intermediate point in the primal phase.
  pub const UPDATE_PRIMAL_BI : i32 = 88;
  /// The callback function is called  in the primal simplex optimizer.
  pub const UPDATE_PRIMAL_SIMPLEX : i32 = 89;
  /// The callback function is called from within the basis identification procedure at an intermediate point in the primal simplex clean-up phase.
  pub const UPDATE_PRIMAL_SIMPLEX_BI : i32 = 90;
  /// The callback function is called from simplex optimizer.
  pub const UPDATE_SIMPLEX : i32 = 91;
  /// The callback function is called from the OPF writer.
  pub const WRITE_OPF : i32 = 92;
} // impl Callbackcode

/// Compression types
#[non_exhaustive]
pub struct Compresstype;
impl Compresstype {
  /// No compression is used.
  pub const NONE : i32 = 0;
  /// The type of compression used is chosen automatically.
  pub const FREE : i32 = 1;
  /// The type of compression used is gzip compatible.
  pub const GZIP : i32 = 2;
  /// The type of compression used is zstd compatible.
  pub const ZSTD : i32 = 3;
} // impl Compresstype

/// Cone types
#[non_exhaustive]
pub struct Conetype;
impl Conetype {
  /// The cone is a quadratic cone.
  pub const QUAD : i32 = 0;
  /// The cone is a rotated quadratic cone.
  pub const RQUAD : i32 = 1;
  /// A primal exponential cone.
  pub const PEXP : i32 = 2;
  /// A dual exponential cone.
  pub const DEXP : i32 = 3;
  /// A primal power cone.
  pub const PPOW : i32 = 4;
  /// A dual power cone.
  pub const DPOW : i32 = 5;
  /// The zero cone.
  pub const ZERO : i32 = 6;
} // impl Conetype

/// Cone types
#[non_exhaustive]
pub struct Domaintype;
impl Domaintype {
  /// R.
  pub const R : i32 = 0;
  /// The zero vector.
  pub const RZERO : i32 = 1;
  /// The positive orthant.
  pub const RPLUS : i32 = 2;
  /// The negative orthant.
  pub const RMINUS : i32 = 3;
  /// The quadratic cone.
  pub const QUADRATIC_CONE : i32 = 4;
  /// The rotated quadratic cone.
  pub const RQUADRATIC_CONE : i32 = 5;
  /// The primal exponential cone.
  pub const PRIMAL_EXP_CONE : i32 = 6;
  /// The dual exponential cone.
  pub const DUAL_EXP_CONE : i32 = 7;
  /// The primal power cone.
  pub const PRIMAL_POWER_CONE : i32 = 8;
  /// The dual power cone.
  pub const DUAL_POWER_CONE : i32 = 9;
  /// The primal geometric mean cone.
  pub const PRIMAL_GEO_MEAN_CONE : i32 = 10;
  /// The dual geometric mean cone.
  pub const DUAL_GEO_MEAN_CONE : i32 = 11;
  /// The vectorized positive semidefinite cone.
  pub const SVEC_PSD_CONE : i32 = 12;
} // impl Domaintype

/// Name types
#[non_exhaustive]
pub struct Nametype;
impl Nametype {
  /// General names. However, no duplicate and blank names are allowed.
  pub const GEN : i32 = 0;
  /// MPS type names.
  pub const MPS : i32 = 1;
  /// LP type names.
  pub const LP : i32 = 2;
} // impl Nametype

/// Cone types
#[non_exhaustive]
pub struct Symmattype;
impl Symmattype {
  /// Sparse symmetric matrix.
  pub const SPARSE : i32 = 0;
} // impl Symmattype

/// Data format types
#[non_exhaustive]
pub struct Dataformat;
impl Dataformat {
  /// The file extension is used to determine the data file format.
  pub const EXTENSION : i32 = 0;
  /// The data file is MPS formatted.
  pub const MPS : i32 = 1;
  /// The data file is LP formatted.
  pub const LP : i32 = 2;
  /// The data file is an optimization problem formatted file.
  pub const OP : i32 = 3;
  /// The data a free MPS formatted file.
  pub const FREE_MPS : i32 = 4;
  /// Generic task dump file.
  pub const TASK : i32 = 5;
  /// (P)retty (T)ext (F)format.
  pub const PTF : i32 = 6;
  /// Conic benchmark format,
  pub const CB : i32 = 7;
  /// JSON based task format.
  pub const JSON_TASK : i32 = 8;
} // impl Dataformat

/// Data format types
#[non_exhaustive]
pub struct Solformat;
impl Solformat {
  /// The file extension is used to determine the data file format.
  pub const EXTENSION : i32 = 0;
  /// Simple binary format
  pub const B : i32 = 1;
  /// Tar based format.
  pub const TASK : i32 = 2;
  /// JSON based format.
  pub const JSON_TASK : i32 = 3;
} // impl Solformat

/// Double information items
#[non_exhaustive]
pub struct Dinfitem;
impl Dinfitem {
  /// Density percentage of the scalarized constraint matrix.
  pub const ANA_PRO_SCALARIZED_CONSTRAINT_MATRIX_DENSITY : i32 = 0;
  /// Time  spent within the dual clean-up optimizer of the basis identification procedure since its invocation.
  pub const BI_CLEAN_DUAL_TIME : i32 = 1;
  /// Time spent within the primal clean-up optimizer of the basis identification procedure since its invocation.
  pub const BI_CLEAN_PRIMAL_TIME : i32 = 2;
  /// Time spent within the clean-up phase of the basis identification procedure since its invocation.
  pub const BI_CLEAN_TIME : i32 = 3;
  /// Time spent within the dual phase basis identification procedure since its invocation.
  pub const BI_DUAL_TIME : i32 = 4;
  /// Time  spent within the primal phase of the basis identification procedure since its invocation.
  pub const BI_PRIMAL_TIME : i32 = 5;
  /// Time spent within the basis identification procedure since its invocation.
  pub const BI_TIME : i32 = 6;
  /// Dual feasibility measure reported by the interior-point optimizer.
  pub const INTPNT_DUAL_FEAS : i32 = 7;
  /// Dual objective value reported by the interior-point optimizer.
  pub const INTPNT_DUAL_OBJ : i32 = 8;
  /// An estimate of the number of flops used in the factorization.
  pub const INTPNT_FACTOR_NUM_FLOPS : i32 = 9;
  /// A measure of optimality of the solution.
  pub const INTPNT_OPT_STATUS : i32 = 10;
  /// Order time (in seconds).
  pub const INTPNT_ORDER_TIME : i32 = 11;
  /// Primal feasibility measure reported by the interior-point optimizer.
  pub const INTPNT_PRIMAL_FEAS : i32 = 12;
  /// Primal objective value reported by the interior-point optimizer.
  pub const INTPNT_PRIMAL_OBJ : i32 = 13;
  /// Time spent within the interior-point optimizer since its invocation.
  pub const INTPNT_TIME : i32 = 14;
  /// Separation time for clique cuts.
  pub const MIO_CLIQUE_SEPARATION_TIME : i32 = 15;
  /// Separation time for CMIR cuts.
  pub const MIO_CMIR_SEPARATION_TIME : i32 = 16;
  /// Optimal objective value corresponding to the feasible solution.
  pub const MIO_CONSTRUCT_SOLUTION_OBJ : i32 = 17;
  /// Value of the dual bound after presolve but before cut generation.
  pub const MIO_DUAL_BOUND_AFTER_PRESOLVE : i32 = 18;
  /// Separation time for GMI cuts.
  pub const MIO_GMI_SEPARATION_TIME : i32 = 19;
  /// Separation time for implied bound cuts.
  pub const MIO_IMPLIED_BOUND_TIME : i32 = 20;
  /// Optimal objective value corresponding to the user provided initial solution.
  pub const MIO_INITIAL_FEASIBLE_SOLUTION_OBJ : i32 = 21;
  /// Separation time for knapsack cover.
  pub const MIO_KNAPSACK_COVER_SEPARATION_TIME : i32 = 22;
  /// Separation time for lift-and-project cuts.
  pub const MIO_LIPRO_SEPARATION_TIME : i32 = 23;
  /// If the mixed-integer optimizer has computed a feasible solution and a bound, this contains the absolute gap.
  pub const MIO_OBJ_ABS_GAP : i32 = 24;
  /// The best bound on the objective value known.
  pub const MIO_OBJ_BOUND : i32 = 25;
  /// The primal objective value corresponding to the best integer feasible solution.
  pub const MIO_OBJ_INT : i32 = 26;
  /// If the mixed-integer optimizer has computed a feasible solution and a bound, this contains the relative gap.
  pub const MIO_OBJ_REL_GAP : i32 = 27;
  /// Total time for probing.
  pub const MIO_PROBING_TIME : i32 = 28;
  /// Total time for cut generation.
  pub const MIO_ROOT_CUTGEN_TIME : i32 = 29;
  /// Time spent in the contiuous optimizer while processing the root node relaxation.
  pub const MIO_ROOT_OPTIMIZER_TIME : i32 = 30;
  /// Time spent presolving the problem at the root node.
  pub const MIO_ROOT_PRESOLVE_TIME : i32 = 31;
  /// Time spent processing the root node.
  pub const MIO_ROOT_TIME : i32 = 32;
  /// Time spent in the mixed-integer optimizer.
  pub const MIO_TIME : i32 = 33;
  /// If the objective cut is used, then this information item has the value of the cut.
  pub const MIO_USER_OBJ_CUT : i32 = 34;
  /// Total number of ticks spent in the optimizer since it was invoked. It is strictly negative if it is not available.
  pub const OPTIMIZER_TICKS : i32 = 35;
  /// Total time spent in the optimizer since it was invoked.
  pub const OPTIMIZER_TIME : i32 = 36;
  /// Total time spent in the eliminator since the presolve was invoked.
  pub const PRESOLVE_ELI_TIME : i32 = 37;
  /// Total time spent  in the linear dependency checker since the presolve was invoked.
  pub const PRESOLVE_LINDEP_TIME : i32 = 38;
  /// Total time (in seconds) spent in the presolve since it was invoked.
  pub const PRESOLVE_TIME : i32 = 39;
  /// Total perturbation of the bounds of the primal problem.
  pub const PRESOLVE_TOTAL_PRIMAL_PERTURBATION : i32 = 40;
  /// The optimal objective value of the penalty function.
  pub const PRIMAL_REPAIR_PENALTY_OBJ : i32 = 41;
  /// Maximum absolute diagonal perturbation occurring during the QCQO reformulation.
  pub const QCQO_REFORMULATE_MAX_PERTURBATION : i32 = 42;
  /// Time spent with conic quadratic reformulation.
  pub const QCQO_REFORMULATE_TIME : i32 = 43;
  /// Worst Cholesky column scaling.
  pub const QCQO_REFORMULATE_WORST_CHOLESKY_COLUMN_SCALING : i32 = 44;
  /// Worst Cholesky diagonal scaling.
  pub const QCQO_REFORMULATE_WORST_CHOLESKY_DIAG_SCALING : i32 = 45;
  /// Time spent reading the data file.
  pub const READ_DATA_TIME : i32 = 46;
  /// The total real time in seconds spent when optimizing on a server by the process performing the optimization on the server
  pub const REMOTE_TIME : i32 = 47;
  /// Time spent in the dual simplex optimizer since invoking it.
  pub const SIM_DUAL_TIME : i32 = 48;
  /// Feasibility measure reported by the simplex optimizer.
  pub const SIM_FEAS : i32 = 49;
  /// Objective value reported by the simplex optimizer.
  pub const SIM_OBJ : i32 = 50;
  /// Time spent in the primal simplex optimizer since invoking it.
  pub const SIM_PRIMAL_TIME : i32 = 51;
  /// Time spent in the simplex optimizer since invoking it.
  pub const SIM_TIME : i32 = 52;
  /// Dual objective value of the basic solution. Updated by the function updatesolutioninfo.
  pub const SOL_BAS_DUAL_OBJ : i32 = 53;
  /// Maximal dual bound violation for xx in the basic solution. Updated by the function updatesolutioninfo.
  pub const SOL_BAS_DVIOLCON : i32 = 54;
  /// Maximal dual bound violation for xx in the basic solution. Updated by the function updatesolutioninfo.
  pub const SOL_BAS_DVIOLVAR : i32 = 55;
  /// Infinity norm of barx in the basic solution.
  pub const SOL_BAS_NRM_BARX : i32 = 56;
  /// Infinity norm of slc in the basic solution.
  pub const SOL_BAS_NRM_SLC : i32 = 57;
  /// Infinity norm of slx in the basic solution.
  pub const SOL_BAS_NRM_SLX : i32 = 58;
  /// Infinity norm of suc in the basic solution.
  pub const SOL_BAS_NRM_SUC : i32 = 59;
  /// Infinity norm of sux in the basic solution.
  pub const SOL_BAS_NRM_SUX : i32 = 60;
  /// Infinity norm of xc in the basic solution.
  pub const SOL_BAS_NRM_XC : i32 = 61;
  /// Infinity norm of xx in the basic solution.
  pub const SOL_BAS_NRM_XX : i32 = 62;
  /// Infinity norm of Y in the basic solution.
  pub const SOL_BAS_NRM_Y : i32 = 63;
  /// Primal objective value of the basic solution. Updated by the function updatesolutioninfo.
  pub const SOL_BAS_PRIMAL_OBJ : i32 = 64;
  /// Maximal primal bound violation for xc in the basic solution. Updated by the function updatesolutioninfo.
  pub const SOL_BAS_PVIOLCON : i32 = 65;
  /// Maximal primal bound violation for xx in the basic solution. Updated by the function updatesolutioninfo.
  pub const SOL_BAS_PVIOLVAR : i32 = 66;
  /// Infinity norm of barx in the integer solution.
  pub const SOL_ITG_NRM_BARX : i32 = 67;
  /// Infinity norm of xc in the integer solution.
  pub const SOL_ITG_NRM_XC : i32 = 68;
  /// Infinity norm of xx in the integer solution.
  pub const SOL_ITG_NRM_XX : i32 = 69;
  /// Primal objective value of the integer solution. Updated by the function updatesolutioninfo.
  pub const SOL_ITG_PRIMAL_OBJ : i32 = 70;
  /// Maximal primal violation for affine conic constraints in the integer solution. Updated by the function updatesolutioninfo.
  pub const SOL_ITG_PVIOLACC : i32 = 71;
  /// Maximal primal bound violation for barx in the integer solution. Updated by the function updatesolutioninfo.
  pub const SOL_ITG_PVIOLBARVAR : i32 = 72;
  /// Maximal primal bound violation for xc in the integer solution. Updated by the function updatesolutioninfo.
  pub const SOL_ITG_PVIOLCON : i32 = 73;
  /// Maximal primal violation for primal conic constraints in the integer solution. Updated by the function updatesolutioninfo.
  pub const SOL_ITG_PVIOLCONES : i32 = 74;
  /// Maximal primal violation for disjunctive constraints in the integer solution. Updated by the function updatesolutioninfo.
  pub const SOL_ITG_PVIOLDJC : i32 = 75;
  /// Maximal violation for the integer constraints in the integer solution. Updated by the function updatesolutioninfo.
  pub const SOL_ITG_PVIOLITG : i32 = 76;
  /// Maximal primal bound violation for xx in the integer solution. Updated by the function updatesolutioninfo.
  pub const SOL_ITG_PVIOLVAR : i32 = 77;
  /// Dual objective value of the interior-point solution. Updated by the function updatesolutioninfo.
  pub const SOL_ITR_DUAL_OBJ : i32 = 78;
  /// Maximal dual violation for affine conic constraints in the interior-point solution. Updated by the function updatesolutioninfo.
  pub const SOL_ITR_DVIOLACC : i32 = 79;
  /// Maximal dual bound violation for barx in the interior-point solution. Updated by the function updatesolutioninfo.
  pub const SOL_ITR_DVIOLBARVAR : i32 = 80;
  /// Maximal dual bound violation for xc in the interior-point solution. Updated by the function updatesolutioninfo.
  pub const SOL_ITR_DVIOLCON : i32 = 81;
  /// Maximal dual violation for conic constraints in the interior-point solution. Updated by the function updatesolutioninfo.
  pub const SOL_ITR_DVIOLCONES : i32 = 82;
  /// Maximal dual bound violation for xx in the interior-point solution. Updated by the function updatesolutioninfo.
  pub const SOL_ITR_DVIOLVAR : i32 = 83;
  /// Infinity norm of bars in the interior-point solution.
  pub const SOL_ITR_NRM_BARS : i32 = 84;
  /// Infinity norm of barx in the interior-point solution.
  pub const SOL_ITR_NRM_BARX : i32 = 85;
  /// Infinity norm of slc in the interior-point solution.
  pub const SOL_ITR_NRM_SLC : i32 = 86;
  /// Infinity norm of slx in the interior-point solution.
  pub const SOL_ITR_NRM_SLX : i32 = 87;
  /// Infinity norm of snx in the interior-point solution.
  pub const SOL_ITR_NRM_SNX : i32 = 88;
  /// Infinity norm of suc in the interior-point solution.
  pub const SOL_ITR_NRM_SUC : i32 = 89;
  /// Infinity norm of sux in the interior-point solution.
  pub const SOL_ITR_NRM_SUX : i32 = 90;
  /// Infinity norm of xc in the interior-point solution.
  pub const SOL_ITR_NRM_XC : i32 = 91;
  /// Infinity norm of xx in the interior-point solution.
  pub const SOL_ITR_NRM_XX : i32 = 92;
  /// Infinity norm of Y in the interior-point solution.
  pub const SOL_ITR_NRM_Y : i32 = 93;
  /// Primal objective value of the interior-point solution.
  pub const SOL_ITR_PRIMAL_OBJ : i32 = 94;
  /// Maximal primal violation for affine conic constraints in the interior-point solution. Updated by the function updatesolutioninfo.
  pub const SOL_ITR_PVIOLACC : i32 = 95;
  /// Maximal primal bound violation for barx in the interior-point solution. Updated by the function updatesolutioninfo.
  pub const SOL_ITR_PVIOLBARVAR : i32 = 96;
  /// Maximal primal bound violation for xc in the interior-point solution. Updated by the function updatesolutioninfo.
  pub const SOL_ITR_PVIOLCON : i32 = 97;
  /// Maximal primal violation for conic constraints in the interior-point solution. Updated by the function updatesolutioninfo.
  pub const SOL_ITR_PVIOLCONES : i32 = 98;
  /// Maximal primal bound violation for xx in the interior-point solution. Updated by the function updatesolutioninfo.
  pub const SOL_ITR_PVIOLVAR : i32 = 99;
  /// Time spent in the last to conic reformulation.
  pub const TO_CONIC_TIME : i32 = 100;
  /// Time spent writing the data file.
  pub const WRITE_DATA_TIME : i32 = 101;
  pub const END : i32 = 101;
} // impl Dinfitem

/// License feature
#[non_exhaustive]
pub struct Feature;
impl Feature {
  /// Base system.
  pub const PTS : i32 = 0;
  /// Conic extension.
  pub const PTON : i32 = 1;
} // impl Feature

/// Double parameters
#[non_exhaustive]
pub struct Dparam;
impl Dparam {
  /// If a constraint violates its bound with an amount larger than this value, the constraint name, index and violation will be printed by the solution analyzer.
  pub const ANA_SOL_INFEAS_TOL : i32 = 0;
  /// Maximum relative dual bound violation allowed in an optimal basic solution.
  pub const BASIS_REL_TOL_S : i32 = 1;
  /// Maximum absolute dual bound violation in an optimal basic solution.
  pub const BASIS_TOL_S : i32 = 2;
  /// Maximum absolute primal bound violation allowed in an optimal basic solution.
  pub const BASIS_TOL_X : i32 = 3;
  /// Convexity check tolerance.
  pub const CHECK_CONVEXITY_REL_TOL : i32 = 4;
  /// Zero tolerance threshold for symmetric matrices.
  pub const DATA_SYM_MAT_TOL : i32 = 5;
  /// Data tolerance threshold.
  pub const DATA_SYM_MAT_TOL_HUGE : i32 = 6;
  /// Data tolerance threshold.
  pub const DATA_SYM_MAT_TOL_LARGE : i32 = 7;
  /// Data tolerance threshold.
  pub const DATA_TOL_AIJ_HUGE : i32 = 8;
  /// Data tolerance threshold.
  pub const DATA_TOL_AIJ_LARGE : i32 = 9;
  /// Data tolerance threshold.
  pub const DATA_TOL_BOUND_INF : i32 = 10;
  /// Data tolerance threshold.
  pub const DATA_TOL_BOUND_WRN : i32 = 11;
  /// Data tolerance threshold.
  pub const DATA_TOL_C_HUGE : i32 = 12;
  /// Data tolerance threshold.
  pub const DATA_TOL_CJ_LARGE : i32 = 13;
  /// Data tolerance threshold.
  pub const DATA_TOL_QIJ : i32 = 14;
  /// Data tolerance threshold.
  pub const DATA_TOL_X : i32 = 15;
  /// Dual feasibility tolerance used by the interior-point optimizer for conic problems.
  pub const INTPNT_CO_TOL_DFEAS : i32 = 16;
  /// Infeasibility tolerance used by the interior-point optimizer for conic problems.
  pub const INTPNT_CO_TOL_INFEAS : i32 = 17;
  /// Relative complementarity gap tolerance used by the interior-point optimizer for conic problems.
  pub const INTPNT_CO_TOL_MU_RED : i32 = 18;
  /// Optimality tolerance used by the interior-point optimizer for conic problems.
  pub const INTPNT_CO_TOL_NEAR_REL : i32 = 19;
  /// Primal feasibility tolerance used by the interior-point optimizer for conic problems.
  pub const INTPNT_CO_TOL_PFEAS : i32 = 20;
  /// Relative gap termination tolerance used by the interior-point optimizer for conic problems.
  pub const INTPNT_CO_TOL_REL_GAP : i32 = 21;
  /// Dual feasibility tolerance used by the interior-point optimizer for quadratic problems.
  pub const INTPNT_QO_TOL_DFEAS : i32 = 22;
  /// Infeasibility tolerance used by the interior-point optimizer for quadratic problems.
  pub const INTPNT_QO_TOL_INFEAS : i32 = 23;
  /// Relative complementarity gap tolerance used by the interior-point optimizer for quadratic problems.
  pub const INTPNT_QO_TOL_MU_RED : i32 = 24;
  /// Optimality tolerance used by the interior-point optimizer for quadratic problems.
  pub const INTPNT_QO_TOL_NEAR_REL : i32 = 25;
  /// Primal feasibility tolerance used by the interior-point optimizer for quadratic problems.
  pub const INTPNT_QO_TOL_PFEAS : i32 = 26;
  /// Relative gap termination tolerance used by the interior-point optimizer for quadratic problems.
  pub const INTPNT_QO_TOL_REL_GAP : i32 = 27;
  /// Dual feasibility tolerance used by the interior-point optimizer for linear problems.
  pub const INTPNT_TOL_DFEAS : i32 = 28;
  /// Controls the interior-point dual starting point.
  pub const INTPNT_TOL_DSAFE : i32 = 29;
  /// Infeasibility tolerance used by the interior-point optimizer for linear problems.
  pub const INTPNT_TOL_INFEAS : i32 = 30;
  /// Relative complementarity gap tolerance used by the interior-point optimizer for linear problems.
  pub const INTPNT_TOL_MU_RED : i32 = 31;
  /// Interior-point centering aggressiveness.
  pub const INTPNT_TOL_PATH : i32 = 32;
  /// Primal feasibility tolerance used by the interior-point optimizer for linear problems.
  pub const INTPNT_TOL_PFEAS : i32 = 33;
  /// Controls the interior-point primal starting point.
  pub const INTPNT_TOL_PSAFE : i32 = 34;
  /// Relative gap termination tolerance used by the interior-point optimizer for linear problems.
  pub const INTPNT_TOL_REL_GAP : i32 = 35;
  /// Relative step size to the boundary for linear and quadratic optimization problems.
  pub const INTPNT_TOL_REL_STEP : i32 = 36;
  /// Minimal step size tolerance for the interior-point optimizer.
  pub const INTPNT_TOL_STEP_SIZE : i32 = 37;
  /// Objective bound.
  pub const LOWER_OBJ_CUT : i32 = 38;
  /// Objective bound.
  pub const LOWER_OBJ_CUT_FINITE_TRH : i32 = 39;
  /// Maximum allowed big-M value when reformulating disjunctive constraints to linear constraints.
  pub const MIO_DJC_MAX_BIGM : i32 = 40;
  /// Time limit for the mixed-integer optimizer.
  pub const MIO_MAX_TIME : i32 = 41;
  /// This value is used to compute the relative gap for the solution to an integer optimization problem.
  pub const MIO_REL_GAP_CONST : i32 = 42;
  /// Absolute optimality tolerance employed by the mixed-integer optimizer.
  pub const MIO_TOL_ABS_GAP : i32 = 43;
  /// Integer feasibility tolerance.
  pub const MIO_TOL_ABS_RELAX_INT : i32 = 44;
  /// Feasibility tolerance for mixed integer solver.
  pub const MIO_TOL_FEAS : i32 = 45;
  /// Controls cut generation for mixed-integer optimizer.
  pub const MIO_TOL_REL_DUAL_BOUND_IMPROVEMENT : i32 = 46;
  /// Relative optimality tolerance employed by the mixed-integer optimizer.
  pub const MIO_TOL_REL_GAP : i32 = 47;
  /// Solver ticks limit.
  pub const OPTIMIZER_MAX_TICKS : i32 = 48;
  /// Solver time limit.
  pub const OPTIMIZER_MAX_TIME : i32 = 49;
  /// Absolute tolerance employed by the linear dependency checker.
  pub const PRESOLVE_TOL_ABS_LINDEP : i32 = 50;
  /// Absolute zero tolerance employed for constraint coefficients in the presolve.
  pub const PRESOLVE_TOL_AIJ : i32 = 51;
  /// The presolve is allowed to perturb a bound on a constraint or variable by this amount if it removes an infeasibility.
  pub const PRESOLVE_TOL_PRIMAL_INFEAS_PERTURBATION : i32 = 52;
  /// Relative tolerance employed by the linear dependency checker.
  pub const PRESOLVE_TOL_REL_LINDEP : i32 = 53;
  /// Absolute zero tolerance employed for slack variables in the presolve.
  pub const PRESOLVE_TOL_S : i32 = 54;
  /// Absolute zero tolerance employed for variables in the presolve.
  pub const PRESOLVE_TOL_X : i32 = 55;
  /// This parameter determines when columns are dropped in incomplete Cholesky factorization during reformulation of quadratic problems.
  pub const QCQO_REFORMULATE_REL_DROP_TOL : i32 = 56;
  /// Tolerance to define a matrix to be positive semidefinite.
  pub const SEMIDEFINITE_TOL_APPROX : i32 = 57;
  /// Relative pivot tolerance employed when computing the LU factorization of the basis matrix.
  pub const SIM_LU_TOL_REL_PIV : i32 = 58;
  /// Absolute pivot tolerance employed by the simplex optimizers.
  pub const SIMPLEX_ABS_TOL_PIV : i32 = 59;
  /// Objective bound.
  pub const UPPER_OBJ_CUT : i32 = 60;
  /// Objective bound.
  pub const UPPER_OBJ_CUT_FINITE_TRH : i32 = 61;
} // impl Dparam

/// Long integer information items.
#[non_exhaustive]
pub struct Liinfitem;
impl Liinfitem {
  /// Number of columns in the scalarized constraint matrix.
  pub const ANA_PRO_SCALARIZED_CONSTRAINT_MATRIX_NUM_COLUMNS : i32 = 0;
  /// Number of non-zero entries in the scalarized constraint matrix.
  pub const ANA_PRO_SCALARIZED_CONSTRAINT_MATRIX_NUM_NZ : i32 = 1;
  /// Number of rows in the scalarized constraint matrix.
  pub const ANA_PRO_SCALARIZED_CONSTRAINT_MATRIX_NUM_ROWS : i32 = 2;
  /// Number of dual degenerate clean iterations performed in the basis identification.
  pub const BI_CLEAN_DUAL_DEG_ITER : i32 = 3;
  /// Number of dual clean iterations performed in the basis identification.
  pub const BI_CLEAN_DUAL_ITER : i32 = 4;
  /// Number of primal degenerate clean iterations performed in the basis identification.
  pub const BI_CLEAN_PRIMAL_DEG_ITER : i32 = 5;
  /// Number of primal clean iterations performed in the basis identification.
  pub const BI_CLEAN_PRIMAL_ITER : i32 = 6;
  /// Number of dual pivots performed in the basis identification.
  pub const BI_DUAL_ITER : i32 = 7;
  /// Number of primal pivots performed in the basis identification.
  pub const BI_PRIMAL_ITER : i32 = 8;
  /// Number of non-zeros in factorization.
  pub const INTPNT_FACTOR_NUM_NZ : i32 = 9;
  /// Number of non-zero entries in the constraint matrix of the problem to be solved by the mixed-integer optimizer.
  pub const MIO_ANZ : i32 = 10;
  /// Number of interior-point iterations performed by the mixed-integer optimizer.
  pub const MIO_INTPNT_ITER : i32 = 11;
  /// Number of dual illposed certificates encountered by the mixed-integer optimizer.
  pub const MIO_NUM_DUAL_ILLPOSED_CER : i32 = 12;
  /// Number of primal illposed certificates encountered by the mixed-integer optimizer.
  pub const MIO_NUM_PRIM_ILLPOSED_CER : i32 = 13;
  /// Number of non-zero entries in the constraint matrix of the problem after the mixed-integer optimizer's presolve.
  pub const MIO_PRESOLVED_ANZ : i32 = 14;
  /// Number of simplex iterations performed by the mixed-integer optimizer.
  pub const MIO_SIMPLEX_ITER : i32 = 15;
  /// Number of affince conic constraints.
  pub const RD_NUMACC : i32 = 16;
  /// Number of non-zeros in A that is read.
  pub const RD_NUMANZ : i32 = 17;
  /// Number of disjuncive constraints.
  pub const RD_NUMDJC : i32 = 18;
  /// Number of Q non-zeros.
  pub const RD_NUMQNZ : i32 = 19;
  /// Number of iterations performed by the simplex optimizer.
  pub const SIMPLEX_ITER : i32 = 20;
  pub const END : i32 = 20;
} // impl Liinfitem

/// Integer information items.
#[non_exhaustive]
pub struct Iinfitem;
impl Iinfitem {
  /// Number of constraints in the problem.
  pub const ANA_PRO_NUM_CON : i32 = 0;
  /// Number of equality constraints.
  pub const ANA_PRO_NUM_CON_EQ : i32 = 1;
  /// Number of unbounded constraints.
  pub const ANA_PRO_NUM_CON_FR : i32 = 2;
  /// Number of constraints with a lower bound and an infinite upper bound.
  pub const ANA_PRO_NUM_CON_LO : i32 = 3;
  /// Number of constraints with finite lower and upper bounds.
  pub const ANA_PRO_NUM_CON_RA : i32 = 4;
  /// Number of constraints with an upper bound and an infinite lower bound.
  pub const ANA_PRO_NUM_CON_UP : i32 = 5;
  /// Number of variables in the problem.
  pub const ANA_PRO_NUM_VAR : i32 = 6;
  /// Number of binary variables.
  pub const ANA_PRO_NUM_VAR_BIN : i32 = 7;
  /// Number of continuous variables.
  pub const ANA_PRO_NUM_VAR_CONT : i32 = 8;
  /// Number of fixed variables.
  pub const ANA_PRO_NUM_VAR_EQ : i32 = 9;
  /// Number of unbounded constraints.
  pub const ANA_PRO_NUM_VAR_FR : i32 = 10;
  /// Number of general integer variables.
  pub const ANA_PRO_NUM_VAR_INT : i32 = 11;
  /// Number of variables with a lower bound and an infinite upper bound.
  pub const ANA_PRO_NUM_VAR_LO : i32 = 12;
  /// Number of variables with finite lower and upper bounds.
  pub const ANA_PRO_NUM_VAR_RA : i32 = 13;
  /// Number of variables with an upper bound and an infinite lower bound.
  pub const ANA_PRO_NUM_VAR_UP : i32 = 14;
  /// Dimension of the dense sub system in factorization.
  pub const INTPNT_FACTOR_DIM_DENSE : i32 = 15;
  /// Number of interior-point iterations since invoking the interior-point optimizer.
  pub const INTPNT_ITER : i32 = 16;
  /// Number of threads that the interior-point optimizer is using.
  pub const INTPNT_NUM_THREADS : i32 = 17;
  /// Non-zero if the interior-point optimizer is solving the dual problem.
  pub const INTPNT_SOLVE_DUAL : i32 = 18;
  /// Non-zero if absolute gap is within tolerances.
  pub const MIO_ABSGAP_SATISFIED : i32 = 19;
  /// Size of the clique table.
  pub const MIO_CLIQUE_TABLE_SIZE : i32 = 20;
  /// Informs if MOSEK successfully constructed an initial integer feasible solution.
  pub const MIO_CONSTRUCT_SOLUTION : i32 = 21;
  /// Informs if MOSEK found the solution provided by the user to be feasible
  pub const MIO_INITIAL_FEASIBLE_SOLUTION : i32 = 22;
  /// Depth of the last node solved.
  pub const MIO_NODE_DEPTH : i32 = 23;
  /// Number of active branch and bound nodes.
  pub const MIO_NUM_ACTIVE_NODES : i32 = 24;
  /// Number of branches performed during the optimization.
  pub const MIO_NUM_BRANCH : i32 = 25;
  /// Number of clique cuts.
  pub const MIO_NUM_CLIQUE_CUTS : i32 = 26;
  /// Number of Complemented Mixed Integer Rounding (CMIR) cuts.
  pub const MIO_NUM_CMIR_CUTS : i32 = 27;
  /// Number of Gomory cuts.
  pub const MIO_NUM_GOMORY_CUTS : i32 = 28;
  /// Number of implied bound cuts.
  pub const MIO_NUM_IMPLIED_BOUND_CUTS : i32 = 29;
  /// Number of integer feasible solutions that have been found.
  pub const MIO_NUM_INT_SOLUTIONS : i32 = 30;
  /// Number of clique cuts.
  pub const MIO_NUM_KNAPSACK_COVER_CUTS : i32 = 31;
  /// Number of lift-and-project cuts.
  pub const MIO_NUM_LIPRO_CUTS : i32 = 32;
  /// Number of relaxations solved during the optimization.
  pub const MIO_NUM_RELAX : i32 = 33;
  /// Number of times presolve was repeated at root.
  pub const MIO_NUM_REPEATED_PRESOLVE : i32 = 34;
  /// Number of binary variables in the problem to be solved by the mixed-integer optimizer.
  pub const MIO_NUMBIN : i32 = 35;
  /// Number of binary cone variables in the problem to be solved by the mixed-integer optimizer.
  pub const MIO_NUMBINCONEVAR : i32 = 36;
  /// Number of constraints in the problem to be solved by the mixed-integer optimizer.
  pub const MIO_NUMCON : i32 = 37;
  /// Number of cones in the problem to be solved by the mixed-integer optimizer.
  pub const MIO_NUMCONE : i32 = 38;
  /// Number of cone variables in the problem to be solved by the mixed-integer optimizer.
  pub const MIO_NUMCONEVAR : i32 = 39;
  /// Number of continuous variables in the problem to be solved by the mixed-integer optimizer.
  pub const MIO_NUMCONT : i32 = 40;
  /// Number of continuous cone variables in the problem to be solved by the mixed-integer optimizer.
  pub const MIO_NUMCONTCONEVAR : i32 = 41;
  /// Number of dual exponential cones in the problem to be solved by the mixed-integer optimizer.
  pub const MIO_NUMDEXPCONES : i32 = 42;
  /// Number of disjunctive constraints in the problem to be solved by the mixed-integer optimizer.
  pub const MIO_NUMDJC : i32 = 43;
  /// Number of dual power cones in the problem to be solved by the mixed-integer optimizer.
  pub const MIO_NUMDPOWCONES : i32 = 44;
  /// Number of integer variables in the problem to be solved by the mixed-integer optimizer.
  pub const MIO_NUMINT : i32 = 45;
  /// Number of integer cone variables in the problem to be solved by the mixed-integer optimizer.
  pub const MIO_NUMINTCONEVAR : i32 = 46;
  /// Number of primal exponential cones in the problem to be solved by the mixed-integer optimizer.
  pub const MIO_NUMPEXPCONES : i32 = 47;
  /// Number of primal power cones in the problem to be solved by the mixed-integer optimizer.
  pub const MIO_NUMPPOWCONES : i32 = 48;
  /// Number of quadratic cones in the problem to be solved by the mixed-integer optimizer.
  pub const MIO_NUMQCONES : i32 = 49;
  /// Number of rotated quadratic cones in the problem to be solved by the mixed-integer optimizer.
  pub const MIO_NUMRQCONES : i32 = 50;
  /// Number of variables in the problem to be solved by the mixed-integer optimizer.
  pub const MIO_NUMVAR : i32 = 51;
  /// Non-zero if a valid objective bound has been found, otherwise zero.
  pub const MIO_OBJ_BOUND_DEFINED : i32 = 52;
  /// Number of binary variables in the problem after the mixed-integer optimizer's presolve.
  pub const MIO_PRESOLVED_NUMBIN : i32 = 53;
  /// Number of binary cone variables in the problem after the mixed-integer optimizer's presolve.
  pub const MIO_PRESOLVED_NUMBINCONEVAR : i32 = 54;
  /// Number of constraints in the problem after the mixed-integer optimizer's presolve.
  pub const MIO_PRESOLVED_NUMCON : i32 = 55;
  /// Number of cones in the problem after the mixed-integer optimizer's presolve.
  pub const MIO_PRESOLVED_NUMCONE : i32 = 56;
  /// Number of cone variables in the problem after the mixed-integer optimizer's presolve.
  pub const MIO_PRESOLVED_NUMCONEVAR : i32 = 57;
  /// Number of continuous variables in the problem after the mixed-integer optimizer's presolve.
  pub const MIO_PRESOLVED_NUMCONT : i32 = 58;
  /// Number of continuous cone variables in the problem after the mixed-integer optimizer's presolve.
  pub const MIO_PRESOLVED_NUMCONTCONEVAR : i32 = 59;
  /// Number of dual exponential cones in the problem after the mixed-integer optimizer's presolve.
  pub const MIO_PRESOLVED_NUMDEXPCONES : i32 = 60;
  /// Number of disjunctive constraints in the problem after the mixed-integer optimizer's presolve.
  pub const MIO_PRESOLVED_NUMDJC : i32 = 61;
  /// Number of dual power cones in the problem after the mixed-integer optimizer's presolve.
  pub const MIO_PRESOLVED_NUMDPOWCONES : i32 = 62;
  /// Number of integer variables in the problem after the mixed-integer optimizer's presolve.
  pub const MIO_PRESOLVED_NUMINT : i32 = 63;
  /// Number of integer cone variables in the problem after the mixed-integer optimizer's presolve.
  pub const MIO_PRESOLVED_NUMINTCONEVAR : i32 = 64;
  /// Number of primal exponential cones in the problem after the mixed-integer optimizer's presolve.
  pub const MIO_PRESOLVED_NUMPEXPCONES : i32 = 65;
  /// Number of primal power cones in the problem after the mixed-integer optimizer's presolve.
  pub const MIO_PRESOLVED_NUMPPOWCONES : i32 = 66;
  /// Number of quadratic cones in the problem after the mixed-integer optimizer's presolve.
  pub const MIO_PRESOLVED_NUMQCONES : i32 = 67;
  /// Number of rotated quadratic cones in the problem after the mixed-integer optimizer's presolve.
  pub const MIO_PRESOLVED_NUMRQCONES : i32 = 68;
  /// Number of variables in the problem after the mixed-integer optimizer's presolve.
  pub const MIO_PRESOLVED_NUMVAR : i32 = 69;
  /// Non-zero if relative gap is within tolerances.
  pub const MIO_RELGAP_SATISFIED : i32 = 70;
  /// Total number of cuts generated by the mixed-integer optimizer.
  pub const MIO_TOTAL_NUM_CUTS : i32 = 71;
  /// If it is non-zero, then the objective cut is used.
  pub const MIO_USER_OBJ_CUT : i32 = 72;
  /// Number of constraints in the problem solved when the optimizer is called.
  pub const OPT_NUMCON : i32 = 73;
  /// Number of variables in the problem solved when the optimizer is called
  pub const OPT_NUMVAR : i32 = 74;
  /// The response code returned by optimize.
  pub const OPTIMIZE_RESPONSE : i32 = 75;
  /// Number perturbations to thhe bounds of the primal problem.
  pub const PRESOLVE_NUM_PRIMAL_PERTURBATIONS : i32 = 76;
  /// Is nonzero if the dual solution is purified.
  pub const PURIFY_DUAL_SUCCESS : i32 = 77;
  /// Is nonzero if the primal solution is purified.
  pub const PURIFY_PRIMAL_SUCCESS : i32 = 78;
  /// Number of symmetric variables read.
  pub const RD_NUMBARVAR : i32 = 79;
  /// Number of constraints read.
  pub const RD_NUMCON : i32 = 80;
  /// Number of conic constraints read.
  pub const RD_NUMCONE : i32 = 81;
  /// Number of integer-constrained variables read.
  pub const RD_NUMINTVAR : i32 = 82;
  /// Number of nonempty Q matrices read.
  pub const RD_NUMQ : i32 = 83;
  /// Number of variables read.
  pub const RD_NUMVAR : i32 = 84;
  /// Problem type.
  pub const RD_PROTYPE : i32 = 85;
  /// The number of dual degenerate iterations.
  pub const SIM_DUAL_DEG_ITER : i32 = 86;
  /// If 1 then the dual simplex algorithm is solving from an advanced basis.
  pub const SIM_DUAL_HOTSTART : i32 = 87;
  /// If 1 then a valid basis factorization of full rank was located and used by the dual simplex algorithm.
  pub const SIM_DUAL_HOTSTART_LU : i32 = 88;
  /// The number of iterations taken with dual infeasibility.
  pub const SIM_DUAL_INF_ITER : i32 = 89;
  /// Number of dual simplex iterations during the last optimization.
  pub const SIM_DUAL_ITER : i32 = 90;
  /// Number of constraints in the problem solved by the simplex optimizer.
  pub const SIM_NUMCON : i32 = 91;
  /// Number of variables in the problem solved by the simplex optimizer.
  pub const SIM_NUMVAR : i32 = 92;
  /// The number of primal degenerate iterations.
  pub const SIM_PRIMAL_DEG_ITER : i32 = 93;
  /// If 1 then the primal simplex algorithm is solving from an advanced basis.
  pub const SIM_PRIMAL_HOTSTART : i32 = 94;
  /// If 1 then a valid basis factorization of full rank was located and used by the primal simplex algorithm.
  pub const SIM_PRIMAL_HOTSTART_LU : i32 = 95;
  /// The number of iterations taken with primal infeasibility.
  pub const SIM_PRIMAL_INF_ITER : i32 = 96;
  /// Number of primal simplex iterations during the last optimization.
  pub const SIM_PRIMAL_ITER : i32 = 97;
  /// Is non-zero if dual problem is solved.
  pub const SIM_SOLVE_DUAL : i32 = 98;
  /// Problem status of the basic solution. Updated after each optimization.
  pub const SOL_BAS_PROSTA : i32 = 99;
  /// Solution status of the basic solution. Updated after each optimization.
  pub const SOL_BAS_SOLSTA : i32 = 100;
  /// Problem status of the integer solution. Updated after each optimization.
  pub const SOL_ITG_PROSTA : i32 = 101;
  /// Solution status of the integer solution. Updated after each optimization.
  pub const SOL_ITG_SOLSTA : i32 = 102;
  /// Problem status of the interior-point solution. Updated after each optimization.
  pub const SOL_ITR_PROSTA : i32 = 103;
  /// Solution status of the interior-point solution. Updated after each optimization.
  pub const SOL_ITR_SOLSTA : i32 = 104;
  /// Number of times the storage for storing the linear coefficient matrix has been changed.
  pub const STO_NUM_A_REALLOC : i32 = 105;
  pub const END : i32 = 105;
} // impl Iinfitem

/// Information item types
#[non_exhaustive]
pub struct Inftype;
impl Inftype {
  /// Is a double information type.
  pub const DOU_TYPE : i32 = 0;
  /// Is an integer.
  pub const INT_TYPE : i32 = 1;
  /// Is a long integer.
  pub const LINT_TYPE : i32 = 2;
} // impl Inftype

/// Input/output modes
#[non_exhaustive]
pub struct Iomode;
impl Iomode {
  /// The file is read-only.
  pub const READ : i32 = 0;
  /// The file is write-only. If the file exists then it is truncated when it is opened. Otherwise it is created when it is opened.
  pub const WRITE : i32 = 1;
  /// The file is to read and write.
  pub const READWRITE : i32 = 2;
} // impl Iomode

/// Integer parameters
#[non_exhaustive]
pub struct Iparam;
impl Iparam {
  /// Controls whether the basis matrix is analyzed in solution analyzer.
  pub const ANA_SOL_BASIS : i32 = 0;
  /// Controls whether a list of violated constraints is printed.
  pub const ANA_SOL_PRINT_VIOLATED : i32 = 1;
  /// Controls whether the elements in each column of A are sorted before an optimization is performed.
  pub const AUTO_SORT_A_BEFORE_OPT : i32 = 2;
  /// Controls whether the solution information items are automatically updated after an optimization is performed.
  pub const AUTO_UPDATE_SOL_INFO : i32 = 3;
  /// Controls the sign of the columns in the basis matrix corresponding to slack variables.
  pub const BASIS_SOLVE_USE_PLUS_ONE : i32 = 4;
  /// Controls which simplex optimizer is used in the clean-up phase.
  pub const BI_CLEAN_OPTIMIZER : i32 = 5;
  /// Turns on basis identification in case the interior-point optimizer is terminated due to maximum number of iterations.
  pub const BI_IGNORE_MAX_ITER : i32 = 6;
  /// Turns on basis identification in case the interior-point optimizer is terminated due to a numerical problem.
  pub const BI_IGNORE_NUM_ERROR : i32 = 7;
  /// Maximum number of iterations after basis identification.
  pub const BI_MAX_ITERATIONS : i32 = 8;
  /// Control license caching.
  pub const CACHE_LICENSE : i32 = 9;
  /// Control compression of stat files.
  pub const COMPRESS_STATFILE : i32 = 10;
  /// Controls the contents of the infeasibility report.
  pub const INFEAS_GENERIC_NAMES : i32 = 11;
  /// Controls which certificate is used if both primal- and dual- certificate of infeasibility is available.
  pub const INFEAS_PREFER_PRIMAL : i32 = 12;
  /// Turns the feasibility report on or off.
  pub const INFEAS_REPORT_AUTO : i32 = 13;
  /// Controls the contents of the infeasibility report.
  pub const INFEAS_REPORT_LEVEL : i32 = 14;
  /// Controls whether basis identification is performed.
  pub const INTPNT_BASIS : i32 = 15;
  /// Controls whether different step sizes are allowed in the primal and dual space.
  pub const INTPNT_DIFF_STEP : i32 = 16;
  /// Currently not in use.
  pub const INTPNT_HOTSTART : i32 = 17;
  /// Controls the maximum number of iterations allowed in the interior-point optimizer.
  pub const INTPNT_MAX_ITERATIONS : i32 = 18;
  /// Maximum number of correction steps.
  pub const INTPNT_MAX_NUM_COR : i32 = 19;
  /// Maximum number of steps to be used by the iterative search direction refinement.
  pub const INTPNT_MAX_NUM_REFINEMENT_STEPS : i32 = 20;
  /// Controls the aggressiveness of the offending column detection.
  pub const INTPNT_OFF_COL_TRH : i32 = 21;
  /// This parameter controls the number of random seeds tried.
  pub const INTPNT_ORDER_GP_NUM_SEEDS : i32 = 22;
  /// Controls the ordering strategy.
  pub const INTPNT_ORDER_METHOD : i32 = 23;
  /// Currently not in use.
  pub const INTPNT_PURIFY : i32 = 24;
  /// Controls whether regularization is allowed.
  pub const INTPNT_REGULARIZATION_USE : i32 = 25;
  /// Controls how the problem is scaled before the interior-point optimizer is used.
  pub const INTPNT_SCALING : i32 = 26;
  /// Controls whether the primal or the dual problem is solved.
  pub const INTPNT_SOLVE_FORM : i32 = 27;
  /// Starting point used by the interior-point optimizer.
  pub const INTPNT_STARTING_POINT : i32 = 28;
  /// Controls the license manager client debugging behavior.
  pub const LICENSE_DEBUG : i32 = 29;
  /// Controls license manager client behavior.
  pub const LICENSE_PAUSE_TIME : i32 = 30;
  /// Controls license manager client behavior.
  pub const LICENSE_SUPPRESS_EXPIRE_WRNS : i32 = 31;
  /// Controls when expiry warnings are issued.
  pub const LICENSE_TRH_EXPIRY_WRN : i32 = 32;
  /// Controls if MOSEK should queue for a license if none is available.
  pub const LICENSE_WAIT : i32 = 33;
  /// Controls the amount of log information.
  pub const LOG : i32 = 34;
  /// Controls amount of output from the problem analyzer.
  pub const LOG_ANA_PRO : i32 = 35;
  /// Controls the amount of output printed by the basis identification procedure. A higher level implies that more information is logged.
  pub const LOG_BI : i32 = 36;
  /// Controls the logging frequency.
  pub const LOG_BI_FREQ : i32 = 37;
  /// Controls the reduction in the log levels for the second and any subsequent optimizations.
  pub const LOG_CUT_SECOND_OPT : i32 = 38;
  /// Controls the amount of logging when a data item such as the maximum number constrains is expanded.
  pub const LOG_EXPAND : i32 = 39;
  /// Controls the amount of output printed when performing feasibility repair. A value higher than one means extensive logging.
  pub const LOG_FEAS_REPAIR : i32 = 40;
  /// If turned on, then some log info is printed when a file is written or read.
  pub const LOG_FILE : i32 = 41;
  /// Controls whether solution summary should be printed by the optimizer.
  pub const LOG_INCLUDE_SUMMARY : i32 = 42;
  /// Controls log level for the infeasibility analyzer.
  pub const LOG_INFEAS_ANA : i32 = 43;
  /// Controls the amount of log information from the interior-point optimizers.
  pub const LOG_INTPNT : i32 = 44;
  /// Control whether local identifying information is printed to the log.
  pub const LOG_LOCAL_INFO : i32 = 45;
  /// Controls the amount of log information from the mixed-integer optimizers.
  pub const LOG_MIO : i32 = 46;
  /// The mixed-integer optimizer logging frequency.
  pub const LOG_MIO_FREQ : i32 = 47;
  /// If turned on, then factor lines are added to the log.
  pub const LOG_ORDER : i32 = 48;
  /// Controls amount of output printed by the presolve procedure. A higher level implies that more information is logged.
  pub const LOG_PRESOLVE : i32 = 49;
  /// Controls amount of output printed when response codes are reported. A higher level implies that more information is logged.
  pub const LOG_RESPONSE : i32 = 50;
  /// Control logging in sensitivity analyzer.
  pub const LOG_SENSITIVITY : i32 = 51;
  /// Control logging in sensitivity analyzer.
  pub const LOG_SENSITIVITY_OPT : i32 = 52;
  /// Controls the amount of log information from the simplex optimizers.
  pub const LOG_SIM : i32 = 53;
  /// Controls simplex logging frequency.
  pub const LOG_SIM_FREQ : i32 = 54;
  /// Currently not in use.
  pub const LOG_SIM_MINOR : i32 = 55;
  /// Controls the memory related log information.
  pub const LOG_STORAGE : i32 = 56;
  /// Each warning is shown a limited number of times controlled by this parameter. A negative value is identical to infinite number of times.
  pub const MAX_NUM_WARNINGS : i32 = 57;
  /// Controls whether the mixed-integer optimizer is branching up or down by default.
  pub const MIO_BRANCH_DIR : i32 = 58;
  /// Toggles outer approximation for conic problems.
  pub const MIO_CONIC_OUTER_APPROXIMATION : i32 = 59;
  /// Controls if an initial mixed integer solution should be constructed from the values of the integer variables.
  pub const MIO_CONSTRUCT_SOL : i32 = 60;
  /// Controls whether clique cuts should be generated.
  pub const MIO_CUT_CLIQUE : i32 = 61;
  /// Controls whether mixed integer rounding cuts should be generated.
  pub const MIO_CUT_CMIR : i32 = 62;
  /// Controls whether GMI cuts should be generated.
  pub const MIO_CUT_GMI : i32 = 63;
  /// Controls whether implied bound cuts should be generated.
  pub const MIO_CUT_IMPLIED_BOUND : i32 = 64;
  /// Controls whether knapsack cover cuts should be generated.
  pub const MIO_CUT_KNAPSACK_COVER : i32 = 65;
  /// Controls whether lift-and-project cuts should be generated.
  pub const MIO_CUT_LIPRO : i32 = 66;
  /// Controls how aggressively generated cuts are selected to be included in the relaxation.
  pub const MIO_CUT_SELECTION_LEVEL : i32 = 67;
  /// Controls what problem data permutation method is appplied to mixed-integer problems.
  pub const MIO_DATA_PERMUTATION_METHOD : i32 = 68;
  /// Controls the way the Feasibility Pump heuristic is employed by the mixed-integer optimizer.
  pub const MIO_FEASPUMP_LEVEL : i32 = 69;
  /// Controls the heuristic employed by the mixed-integer optimizer to locate an initial integer feasible solution.
  pub const MIO_HEURISTIC_LEVEL : i32 = 70;
  /// Maximum number of branches allowed during the branch and bound search.
  pub const MIO_MAX_NUM_BRANCHES : i32 = 71;
  /// Maximum number of relaxations in branch and bound search.
  pub const MIO_MAX_NUM_RELAXS : i32 = 72;
  /// Maximum number of cut separation rounds at the root node.
  pub const MIO_MAX_NUM_ROOT_CUT_ROUNDS : i32 = 73;
  /// Controls how many feasible solutions the mixed-integer optimizer investigates.
  pub const MIO_MAX_NUM_SOLUTIONS : i32 = 74;
  /// Controls how much emphasis is put on reducing memory usage.
  pub const MIO_MEMORY_EMPHASIS_LEVEL : i32 = 75;
  /// Turns on/off the mixed-integer mode.
  pub const MIO_MODE : i32 = 76;
  /// Controls which optimizer is employed at the non-root nodes in the mixed-integer optimizer.
  pub const MIO_NODE_OPTIMIZER : i32 = 77;
  /// Controls the node selection strategy employed by the mixed-integer optimizer.
  pub const MIO_NODE_SELECTION : i32 = 78;
  /// Controls how much emphasis is put on reducing numerical problems
  pub const MIO_NUMERICAL_EMPHASIS_LEVEL : i32 = 79;
  /// Enables or disables perspective reformulation in presolve.
  pub const MIO_PERSPECTIVE_REFORMULATE : i32 = 80;
  /// Controls if the aggregator should be used.
  pub const MIO_PRESOLVE_AGGREGATOR_USE : i32 = 81;
  /// Controls the amount of probing employed by the mixed-integer optimizer in presolve.
  pub const MIO_PROBING_LEVEL : i32 = 82;
  /// Use objective domain propagation.
  pub const MIO_PROPAGATE_OBJECTIVE_CONSTRAINT : i32 = 83;
  /// Controls what reformulation method is applied to mixed-integer quadratic problems.
  pub const MIO_QCQO_REFORMULATION_METHOD : i32 = 84;
  /// Maximum number of nodes in each call to RINS.
  pub const MIO_RINS_MAX_NODES : i32 = 85;
  /// Controls which optimizer is employed at the root node in the mixed-integer optimizer.
  pub const MIO_ROOT_OPTIMIZER : i32 = 86;
  /// Controls whether presolve can be repeated at root node.
  pub const MIO_ROOT_REPEAT_PRESOLVE_LEVEL : i32 = 87;
  /// Sets the random seed used for randomization in the mixed integer optimizer.
  pub const MIO_SEED : i32 = 88;
  /// Controls the amount of symmetry detection and handling employed by the mixed-integer optimizer in presolve.
  pub const MIO_SYMMETRY_LEVEL : i32 = 89;
  /// Controls how much effort is put into detecting variable bounds.
  pub const MIO_VB_DETECTION_LEVEL : i32 = 90;
  /// Set the number of iterations to spin before sleeping.
  pub const MT_SPINCOUNT : i32 = 91;
  /// Not in use
  pub const NG : i32 = 92;
  /// The number of threads employed by the optimizer.
  pub const NUM_THREADS : i32 = 93;
  /// Write a text header with date and MOSEK version in an OPF file.
  pub const OPF_WRITE_HEADER : i32 = 94;
  /// Write a hint section with problem dimensions in the beginning of an OPF file.
  pub const OPF_WRITE_HINTS : i32 = 95;
  /// Aim to keep lines in OPF files not much longer than this.
  pub const OPF_WRITE_LINE_LENGTH : i32 = 96;
  /// Write a parameter section in an OPF file.
  pub const OPF_WRITE_PARAMETERS : i32 = 97;
  /// Write objective, constraints, bounds etc. to an OPF file.
  pub const OPF_WRITE_PROBLEM : i32 = 98;
  /// Controls what is written to the OPF files.
  pub const OPF_WRITE_SOL_BAS : i32 = 99;
  /// Controls what is written to the OPF files.
  pub const OPF_WRITE_SOL_ITG : i32 = 100;
  /// Controls what is written to the OPF files.
  pub const OPF_WRITE_SOL_ITR : i32 = 101;
  /// Enable inclusion of solutions in the OPF files.
  pub const OPF_WRITE_SOLUTIONS : i32 = 102;
  /// Controls which optimizer is used to optimize the task.
  pub const OPTIMIZER : i32 = 103;
  /// If turned on, then names in the parameter file are case sensitive.
  pub const PARAM_READ_CASE_NAME : i32 = 104;
  /// If turned on, then errors in parameter settings is ignored.
  pub const PARAM_READ_IGN_ERROR : i32 = 105;
  /// Maximum amount of fill-in created in one pivot during the elimination phase.
  pub const PRESOLVE_ELIMINATOR_MAX_FILL : i32 = 106;
  /// Control the maximum number of times the eliminator is tried.
  pub const PRESOLVE_ELIMINATOR_MAX_NUM_TRIES : i32 = 107;
  /// Currently not used.
  pub const PRESOLVE_LEVEL : i32 = 108;
  /// Controls linear dependency check in presolve.
  pub const PRESOLVE_LINDEP_ABS_WORK_TRH : i32 = 109;
  /// Controls whether whether a new experimental linear depdency checker is employed.
  pub const PRESOLVE_LINDEP_NEW : i32 = 110;
  /// Controls linear dependency check in presolve.
  pub const PRESOLVE_LINDEP_REL_WORK_TRH : i32 = 111;
  /// Controls whether the linear constraints are checked for linear dependencies.
  pub const PRESOLVE_LINDEP_USE : i32 = 112;
  /// Control the maximum number of times presolve passes over the problem.
  pub const PRESOLVE_MAX_NUM_PASS : i32 = 113;
  /// Controls the maximum number of reductions performed by the presolve.
  pub const PRESOLVE_MAX_NUM_REDUCTIONS : i32 = 114;
  /// Controls whether the presolve is applied to a problem before it is optimized.
  pub const PRESOLVE_USE : i32 = 115;
  /// Controls which optimizer that is used to find the optimal repair.
  pub const PRIMAL_REPAIR_OPTIMIZER : i32 = 116;
  /// Controls whether parameters section is written in PTF files.
  pub const PTF_WRITE_PARAMETERS : i32 = 117;
  /// Controls whether solution section is written in PTF files.
  pub const PTF_WRITE_SOLUTIONS : i32 = 118;
  /// Controls if simple transformation are done when writing PTF files.
  pub const PTF_WRITE_TRANSFORM : i32 = 119;
  /// Turns on additional debugging information when reading files.
  pub const READ_DEBUG : i32 = 120;
  /// Controls whether the free constraints are included in the problem.
  pub const READ_KEEP_FREE_CON : i32 = 121;
  /// Controls how strictly the MPS file reader interprets the MPS format.
  pub const READ_MPS_FORMAT : i32 = 122;
  /// Controls the maximal number of characters allowed in one line of the MPS file.
  pub const READ_MPS_WIDTH : i32 = 123;
  /// Controls what information is used from the task files.
  pub const READ_TASK_IGNORE_PARAM : i32 = 124;
  /// Use compression when sending data to an optimization server
  pub const REMOTE_USE_COMPRESSION : i32 = 125;
  /// Removes unused solutions before the optimization is performed.
  pub const REMOVE_UNUSED_SOLUTIONS : i32 = 126;
  /// Controls sensitivity report behavior.
  pub const SENSITIVITY_ALL : i32 = 127;
  /// Controls which optimizer is used for optimal partition sensitivity analysis.
  pub const SENSITIVITY_OPTIMIZER : i32 = 128;
  /// Controls which type of sensitivity analysis is to be performed.
  pub const SENSITIVITY_TYPE : i32 = 129;
  /// Controls whether an LU factorization of the basis is used in a hot-start.
  pub const SIM_BASIS_FACTOR_USE : i32 = 130;
  /// Controls how aggressively degeneration is handled.
  pub const SIM_DEGEN : i32 = 131;
  /// Not in use.
  pub const SIM_DETECT_PWL : i32 = 132;
  /// Controls whether crashing is performed in the dual simplex optimizer.
  pub const SIM_DUAL_CRASH : i32 = 133;
  /// An experimental feature.
  pub const SIM_DUAL_PHASEONE_METHOD : i32 = 134;
  /// Controls how aggressively restricted selection is used.
  pub const SIM_DUAL_RESTRICT_SELECTION : i32 = 135;
  /// Controls the dual simplex strategy.
  pub const SIM_DUAL_SELECTION : i32 = 136;
  /// Controls if the simplex optimizers are allowed to exploit duplicated columns.
  pub const SIM_EXPLOIT_DUPVEC : i32 = 137;
  /// Controls the type of hot-start that the simplex optimizer perform.
  pub const SIM_HOTSTART : i32 = 138;
  /// Determines if the simplex optimizer should exploit the initial factorization.
  pub const SIM_HOTSTART_LU : i32 = 139;
  /// Maximum number of iterations that can be used by a simplex optimizer.
  pub const SIM_MAX_ITERATIONS : i32 = 140;
  /// Controls how many set-backs that are allowed within a simplex optimizer.
  pub const SIM_MAX_NUM_SETBACKS : i32 = 141;
  /// Controls if the simplex optimizer ensures a non-singular basis, if possible.
  pub const SIM_NON_SINGULAR : i32 = 142;
  /// Controls the simplex crash.
  pub const SIM_PRIMAL_CRASH : i32 = 143;
  /// An experimental feature.
  pub const SIM_PRIMAL_PHASEONE_METHOD : i32 = 144;
  /// Controls how aggressively restricted selection is used.
  pub const SIM_PRIMAL_RESTRICT_SELECTION : i32 = 145;
  /// Controls the primal simplex strategy.
  pub const SIM_PRIMAL_SELECTION : i32 = 146;
  /// Controls the basis refactoring frequency.
  pub const SIM_REFACTOR_FREQ : i32 = 147;
  /// Controls if the simplex optimizers are allowed to reformulate the problem.
  pub const SIM_REFORMULATION : i32 = 148;
  /// Controls if the LU factorization stored should be replaced with the LU factorization corresponding to the initial basis.
  pub const SIM_SAVE_LU : i32 = 149;
  /// Controls how much effort is used in scaling the problem before a simplex optimizer is used.
  pub const SIM_SCALING : i32 = 150;
  /// Controls how the problem is scaled before a simplex optimizer is used.
  pub const SIM_SCALING_METHOD : i32 = 151;
  /// Sets the random seed used for randomization in the simplex optimizers.
  pub const SIM_SEED : i32 = 152;
  /// Controls whether the primal or the dual problem is solved by the primal-/dual-simplex optimizer.
  pub const SIM_SOLVE_FORM : i32 = 153;
  /// Controls how high priority the numerical stability should be given.
  pub const SIM_STABILITY_PRIORITY : i32 = 154;
  /// Controls the simplex behavior.
  pub const SIM_SWITCH_OPTIMIZER : i32 = 155;
  /// Control the contents of the solution files.
  pub const SOL_FILTER_KEEP_BASIC : i32 = 156;
  /// Control the contents of the solution files.
  pub const SOL_FILTER_KEEP_RANGED : i32 = 157;
  /// Controls the input solution file format.
  pub const SOL_READ_NAME_WIDTH : i32 = 158;
  /// Controls the input solution file format.
  pub const SOL_READ_WIDTH : i32 = 159;
  /// Indicates whether solution callbacks will be performed during the optimization.
  pub const SOLUTION_CALLBACK : i32 = 160;
  /// Controls the amount of timing performed inside MOSEK.
  pub const TIMING_LEVEL : i32 = 161;
  /// Controls the basic solution file format.
  pub const WRITE_BAS_CONSTRAINTS : i32 = 162;
  /// Controls the basic solution file format.
  pub const WRITE_BAS_HEAD : i32 = 163;
  /// Controls the basic solution file format.
  pub const WRITE_BAS_VARIABLES : i32 = 164;
  /// Controls output file compression.
  pub const WRITE_COMPRESSION : i32 = 165;
  /// Controls output file data.
  pub const WRITE_DATA_PARAM : i32 = 166;
  /// Controls the output file data.
  pub const WRITE_FREE_CON : i32 = 167;
  /// Controls the output file data.
  pub const WRITE_GENERIC_NAMES : i32 = 168;
  /// Index origin used in  generic names.
  pub const WRITE_GENERIC_NAMES_IO : i32 = 169;
  /// Controls if the writer ignores incompatible problem items when writing files.
  pub const WRITE_IGNORE_INCOMPATIBLE_ITEMS : i32 = 170;
  /// Controls the integer solution file format.
  pub const WRITE_INT_CONSTRAINTS : i32 = 171;
  /// Controls the integer solution file format.
  pub const WRITE_INT_HEAD : i32 = 172;
  /// Controls the integer solution file format.
  pub const WRITE_INT_VARIABLES : i32 = 173;
  /// When set, the JSON task and solution files are written with indentation for better readability.
  pub const WRITE_JSON_INDENTATION : i32 = 174;
  /// Write full linear objective
  pub const WRITE_LP_FULL_OBJ : i32 = 175;
  /// Controls the LP output file format.
  pub const WRITE_LP_LINE_WIDTH : i32 = 176;
  /// Controls in which format the MPS is written.
  pub const WRITE_MPS_FORMAT : i32 = 177;
  /// Controls the output file data.
  pub const WRITE_MPS_INT : i32 = 178;
  /// Controls the solution file format.
  pub const WRITE_SOL_BARVARIABLES : i32 = 179;
  /// Controls the solution file format.
  pub const WRITE_SOL_CONSTRAINTS : i32 = 180;
  /// Controls solution file format.
  pub const WRITE_SOL_HEAD : i32 = 181;
  /// Controls whether the user specified names are employed even if they are invalid names.
  pub const WRITE_SOL_IGNORE_INVALID_NAMES : i32 = 182;
  /// Controls the solution file format.
  pub const WRITE_SOL_VARIABLES : i32 = 183;
  /// Controls whether the solutions are stored in the task file too.
  pub const WRITE_TASK_INC_SOL : i32 = 184;
  /// Controls if linear coefficients should be written by row or column when writing in the XML file format.
  pub const WRITE_XML_MODE : i32 = 185;
} // impl Iparam

/// Specifies the branching direction.
#[non_exhaustive]
pub struct Branchdir;
impl Branchdir {
  /// The mixed-integer optimizer decides which branch to choose.
  pub const FREE : i32 = 0;
  /// The mixed-integer optimizer always chooses the up branch first.
  pub const UP : i32 = 1;
  /// The mixed-integer optimizer always chooses the down branch first.
  pub const DOWN : i32 = 2;
  /// Branch in direction nearest to selected fractional variable.
  pub const NEAR : i32 = 3;
  /// Branch in direction farthest from selected fractional variable.
  pub const FAR : i32 = 4;
  /// Chose direction based on root lp value of selected variable.
  pub const ROOT_LP : i32 = 5;
  /// Branch in direction of current incumbent.
  pub const GUIDED : i32 = 6;
  /// Branch based on the pseudocost of the variable.
  pub const PSEUDOCOST : i32 = 7;
} // impl Branchdir

/// Specifies the reformulation method for mixed-integer quadratic problems.
#[non_exhaustive]
pub struct Miqcqoreformmethod;
impl Miqcqoreformmethod {
  /// The mixed-integer optimizer decides which reformulation method to apply.
  pub const FREE : i32 = 0;
  /// No reformulation method is applied.
  pub const NONE : i32 = 1;
  /// A reformulation via linearization is applied.
  pub const LINEARIZATION : i32 = 2;
  /// The eigenvalue method is applied.
  pub const EIGEN_VAL_METHOD : i32 = 3;
  /// A perturbation of matrix diagonals via the solution of SDPs is applied.
  pub const DIAG_SDP : i32 = 4;
  /// A Reformulation based on the solution of an SDP-relaxation of the problem is applied.
  pub const RELAX_SDP : i32 = 5;
} // impl Miqcqoreformmethod

/// Specifies the problem data permutation method for mixed-integer problems.
#[non_exhaustive]
pub struct Miodatapermmethod;
impl Miodatapermmethod {
  /// No problem data permutation is applied.
  pub const NONE : i32 = 0;
  /// A random cyclic shift is applied to permute the problem data.
  pub const CYCLIC_SHIFT : i32 = 1;
  /// A random permutation is applied to the problem data.
  pub const RANDOM : i32 = 2;
} // impl Miodatapermmethod

/// Continuous mixed-integer solution type
#[non_exhaustive]
pub struct Miocontsoltype;
impl Miocontsoltype {
  /// No interior-point or basic solution.
  pub const NONE : i32 = 0;
  /// Solutions to the root node problem.
  pub const ROOT : i32 = 1;
  /// A feasible primal solution.
  pub const ITG : i32 = 2;
  /// A feasible primal solution or a root node solution if the problem is infeasible.
  pub const ITG_REL : i32 = 3;
} // impl Miocontsoltype

/// Integer restrictions
#[non_exhaustive]
pub struct Miomode;
impl Miomode {
  /// The integer constraints are ignored and the problem is solved as a continuous problem.
  pub const IGNORED : i32 = 0;
  /// Integer restrictions should be satisfied.
  pub const SATISFIED : i32 = 1;
} // impl Miomode

/// Mixed-integer node selection types
#[non_exhaustive]
pub struct Mionodeseltype;
impl Mionodeseltype {
  /// The optimizer decides the node selection strategy.
  pub const FREE : i32 = 0;
  /// The optimizer employs a depth first node selection strategy.
  pub const FIRST : i32 = 1;
  /// The optimizer employs a best bound node selection strategy.
  pub const BEST : i32 = 2;
  /// The optimizer employs selects the node based on a pseudo cost estimate.
  pub const PSEUDO : i32 = 3;
} // impl Mionodeseltype

/// MPS file format type
#[non_exhaustive]
pub struct Mpsformat;
impl Mpsformat {
  /// It is assumed that the input file satisfies the MPS format strictly.
  pub const STRICT : i32 = 0;
  /// It is assumed that the input file satisfies a slightly relaxed version of the MPS format.
  pub const RELAXED : i32 = 1;
  /// It is assumed that the input file satisfies the free MPS format. This implies that spaces are not allowed in names. Otherwise the format is free.
  pub const FREE : i32 = 2;
  /// The CPLEX compatible version of the MPS format is employed.
  pub const CPLEX : i32 = 3;
} // impl Mpsformat

/// Objective sense types
#[non_exhaustive]
pub struct Objsense;
impl Objsense {
  /// The problem should be minimized.
  pub const MINIMIZE : i32 = 0;
  /// The problem should be maximized.
  pub const MAXIMIZE : i32 = 1;
} // impl Objsense

/// On/off
#[non_exhaustive]
pub struct Onoffkey;
impl Onoffkey {
  /// Switch the option off.
  pub const OFF : i32 = 0;
  /// Switch the option on.
  pub const ON : i32 = 1;
} // impl Onoffkey

/// Optimizer types
#[non_exhaustive]
pub struct Optimizertype;
impl Optimizertype {
  /// The optimizer for problems having conic constraints.
  pub const CONIC : i32 = 0;
  /// The dual simplex optimizer is used.
  pub const DUAL_SIMPLEX : i32 = 1;
  /// The optimizer is chosen automatically.
  pub const FREE : i32 = 2;
  /// One of the simplex optimizers is used.
  pub const FREE_SIMPLEX : i32 = 3;
  /// The interior-point optimizer is used.
  pub const INTPNT : i32 = 4;
  /// The mixed-integer optimizer.
  pub const MIXED_INT : i32 = 5;
  /// The primal simplex optimizer is used.
  pub const PRIMAL_SIMPLEX : i32 = 6;
} // impl Optimizertype

/// Ordering strategies
#[non_exhaustive]
pub struct Orderingtype;
impl Orderingtype {
  /// The ordering method is chosen automatically.
  pub const FREE : i32 = 0;
  /// Approximate minimum local fill-in ordering is employed.
  pub const APPMINLOC : i32 = 1;
  /// This option should not be used.
  pub const EXPERIMENTAL : i32 = 2;
  /// Always try the graph partitioning based ordering.
  pub const TRY_GRAPHPAR : i32 = 3;
  /// Always use the graph partitioning based ordering even if it is worse than the approximate minimum local fill ordering.
  pub const FORCE_GRAPHPAR : i32 = 4;
  /// No ordering is used. Note using this value almost always leads to a significantly slow down.
  pub const NONE : i32 = 5;
} // impl Orderingtype

/// Presolve method.
#[non_exhaustive]
pub struct Presolvemode;
impl Presolvemode {
  /// The problem is not presolved before it is optimized.
  pub const OFF : i32 = 0;
  /// The problem is presolved before it is optimized.
  pub const ON : i32 = 1;
  /// It is decided automatically whether to presolve before the problem is optimized.
  pub const FREE : i32 = 2;
} // impl Presolvemode

/// Parameter type
#[non_exhaustive]
pub struct Parametertype;
impl Parametertype {
  /// Not a valid parameter.
  pub const INVALID_TYPE : i32 = 0;
  /// Is a double parameter.
  pub const DOU_TYPE : i32 = 1;
  /// Is an integer parameter.
  pub const INT_TYPE : i32 = 2;
  /// Is a string parameter.
  pub const STR_TYPE : i32 = 3;
} // impl Parametertype

/// Problem data items
#[non_exhaustive]
pub struct Problemitem;
impl Problemitem {
  /// Item is a variable.
  pub const VAR : i32 = 0;
  /// Item is a constraint.
  pub const CON : i32 = 1;
  /// Item is a cone.
  pub const CONE : i32 = 2;
} // impl Problemitem

/// Problem types
#[non_exhaustive]
pub struct Problemtype;
impl Problemtype {
  /// The problem is a linear optimization problem.
  pub const LO : i32 = 0;
  /// The problem is a quadratic optimization problem.
  pub const QO : i32 = 1;
  /// The problem is a quadratically constrained optimization problem.
  pub const QCQO : i32 = 2;
  /// A conic optimization.
  pub const CONIC : i32 = 3;
  /// General nonlinear constraints and conic constraints. This combination can not be solved by MOSEK.
  pub const MIXED : i32 = 4;
} // impl Problemtype

/// Problem status keys
#[non_exhaustive]
pub struct Prosta;
impl Prosta {
  /// Unknown problem status.
  pub const UNKNOWN : i32 = 0;
  /// The problem is primal and dual feasible.
  pub const PRIM_AND_DUAL_FEAS : i32 = 1;
  /// The problem is primal feasible.
  pub const PRIM_FEAS : i32 = 2;
  /// The problem is dual feasible.
  pub const DUAL_FEAS : i32 = 3;
  /// The problem is primal infeasible.
  pub const PRIM_INFEAS : i32 = 4;
  /// The problem is dual infeasible.
  pub const DUAL_INFEAS : i32 = 5;
  /// The problem is primal and dual infeasible.
  pub const PRIM_AND_DUAL_INFEAS : i32 = 6;
  /// The problem is ill-posed. For example, it may be primal and dual feasible but have a positive duality gap.
  pub const ILL_POSED : i32 = 7;
  /// The problem is either primal infeasible or unbounded. This may occur for mixed-integer problems.
  pub const PRIM_INFEAS_OR_UNBOUNDED : i32 = 8;
} // impl Prosta

/// XML writer output mode
#[non_exhaustive]
pub struct Xmlwriteroutputtype;
impl Xmlwriteroutputtype {
  /// Write in row order.
  pub const ROW : i32 = 0;
  /// Write in column order.
  pub const COL : i32 = 1;
} // impl Xmlwriteroutputtype

/// Response codes
#[non_exhaustive]
pub struct Rescode;
impl Rescode {
  /// No error occurred.
  pub const OK : i32 = 0;
  /// The parameter file could not be opened.
  pub const WRN_OPEN_PARAM_FILE : i32 = 50;
  /// A numerically large bound value is specified.
  pub const WRN_LARGE_BOUND : i32 = 51;
  /// A numerically large lower bound value is specified.
  pub const WRN_LARGE_LO_BOUND : i32 = 52;
  /// A numerically large upper bound value is specified.
  pub const WRN_LARGE_UP_BOUND : i32 = 53;
  /// A equality constraint is fixed to numerically large value.
  pub const WRN_LARGE_CON_FX : i32 = 54;
  /// A numerically large value is specified for one element in c.
  pub const WRN_LARGE_CJ : i32 = 57;
  /// A numerically large value is specified for an element in A.
  pub const WRN_LARGE_AIJ : i32 = 62;
  /// One or more zero elements are specified in A.
  pub const WRN_ZERO_AIJ : i32 = 63;
  /// A name is longer than the buffer that is supposed to hold it.
  pub const WRN_NAME_MAX_LEN : i32 = 65;
  /// A value for a string parameter is longer than the buffer that is supposed to hold it.
  pub const WRN_SPAR_MAX_LEN : i32 = 66;
  /// An RHS vector is split into several nonadjacent parts.
  pub const WRN_MPS_SPLIT_RHS_VECTOR : i32 = 70;
  /// A RANGE vector is split into several nonadjacent parts in an MPS file.
  pub const WRN_MPS_SPLIT_RAN_VECTOR : i32 = 71;
  /// A BOUNDS vector is split into several nonadjacent parts in an MPS file.
  pub const WRN_MPS_SPLIT_BOU_VECTOR : i32 = 72;
  /// Missing '/2' after quadratic expressions in bound or objective.
  pub const WRN_LP_OLD_QUAD_FORMAT : i32 = 80;
  /// Ignore a variable because the variable was not previously defined.
  pub const WRN_LP_DROP_VARIABLE : i32 = 85;
  /// Non-zero elements specified in the upper triangle of a matrix were ignored.
  pub const WRN_NZ_IN_UPR_TRI : i32 = 200;
  /// One or more non-zero elements were dropped in the Q matrix in the objective.
  pub const WRN_DROPPED_NZ_QOBJ : i32 = 201;
  /// Ignored integer constraints.
  pub const WRN_IGNORE_INTEGER : i32 = 250;
  /// No global optimizer is available.
  pub const WRN_NO_GLOBAL_OPTIMIZER : i32 = 251;
  /// The final mixed-integer problem with all the integer variables fixed at their optimal values is infeasible.
  pub const WRN_MIO_INFEASIBLE_FINAL : i32 = 270;
  /// Invalid solution filter is specified.
  pub const WRN_SOL_FILTER : i32 = 300;
  /// Undefined name occurred in a solution.
  pub const WRN_UNDEF_SOL_FILE_NAME : i32 = 350;
  /// One or more lines in the constraint section were ignored when reading a solution file.
  pub const WRN_SOL_FILE_IGNORED_CON : i32 = 351;
  /// One or more lines in the variable section were ignored when reading a solution file.
  pub const WRN_SOL_FILE_IGNORED_VAR : i32 = 352;
  /// An incomplete basis is specified.
  pub const WRN_TOO_FEW_BASIS_VARS : i32 = 400;
  /// A basis with too many variables is specified.
  pub const WRN_TOO_MANY_BASIS_VARS : i32 = 405;
  /// The license expires.
  pub const WRN_LICENSE_EXPIRE : i32 = 500;
  /// The license server is not responding.
  pub const WRN_LICENSE_SERVER : i32 = 501;
  /// A variable or constraint name is empty. The output file may be invalid.
  pub const WRN_EMPTY_NAME : i32 = 502;
  /// Generic names are used because a name is invalid for requested format.
  pub const WRN_USING_GENERIC_NAMES : i32 = 503;
  /// A name e.g. a row name is not a valid MPS name.
  pub const WRN_INVALID_MPS_NAME : i32 = 504;
  /// The objective name is not a valid MPS name.
  pub const WRN_INVALID_MPS_OBJ_NAME : i32 = 505;
  /// The license expires.
  pub const WRN_LICENSE_FEATURE_EXPIRE : i32 = 509;
  /// Parameter name not recognized.
  pub const WRN_PARAM_NAME_DOU : i32 = 510;
  /// Parameter name not recognized.
  pub const WRN_PARAM_NAME_INT : i32 = 511;
  /// Parameter name not recognized.
  pub const WRN_PARAM_NAME_STR : i32 = 512;
  /// A parameter value is not correct.
  pub const WRN_PARAM_STR_VALUE : i32 = 515;
  /// A parameter was ignored by the conic mixed integer optimizer.
  pub const WRN_PARAM_IGNORED_CMIO : i32 = 516;
  /// One or more (near) zero elements are specified in a sparse row of a matrix.
  pub const WRN_ZEROS_IN_SPARSE_ROW : i32 = 705;
  /// One or more (near) zero elements are specified in a sparse column of a matrix.
  pub const WRN_ZEROS_IN_SPARSE_COL : i32 = 710;
  /// The linear dependency check(s) is incomplete.
  pub const WRN_INCOMPLETE_LINEAR_DEPENDENCY_CHECK : i32 = 800;
  /// The eliminator is skipped at least once due to lack of space.
  pub const WRN_ELIMINATOR_SPACE : i32 = 801;
  /// The presolve is incomplete due to lack of space.
  pub const WRN_PRESOLVE_OUTOFSPACE : i32 = 802;
  /// The presolve perturbed the bounds of the primal problem. This is an indication that the problem is nearly infeasible.
  pub const WRN_PRESOLVE_PRIMAL_PERTUBATIONS : i32 = 803;
  /// Some names were changed because they were invalid for the output file format.
  pub const WRN_WRITE_CHANGED_NAMES : i32 = 830;
  /// The fixed objective term was discarded in the output file.
  pub const WRN_WRITE_DISCARDED_CFIX : i32 = 831;
  /// Two constraint names are identical.
  pub const WRN_DUPLICATE_CONSTRAINT_NAMES : i32 = 850;
  /// Two variable names are identical.
  pub const WRN_DUPLICATE_VARIABLE_NAMES : i32 = 851;
  /// Two barvariable names are identical.
  pub const WRN_DUPLICATE_BARVARIABLE_NAMES : i32 = 852;
  /// Two cone names are identical.
  pub const WRN_DUPLICATE_CONE_NAMES : i32 = 853;
  /// LP file will be written with generic variable names.
  pub const WRN_WRITE_LP_INVALID_VAR_NAMES : i32 = 854;
  /// LP file will be written with generic variable names.
  pub const WRN_WRITE_LP_DUPLICATE_VAR_NAMES : i32 = 855;
  /// LP file will be written with generic constraint names.
  pub const WRN_WRITE_LP_INVALID_CON_NAMES : i32 = 856;
  /// LP file will be written with generic constraint names.
  pub const WRN_WRITE_LP_DUPLICATE_CON_NAMES : i32 = 857;
  /// Warn against very large bounds.
  pub const WRN_ANA_LARGE_BOUNDS : i32 = 900;
  /// Warn against all objective coefficients being zero.
  pub const WRN_ANA_C_ZERO : i32 = 901;
  /// Warn against empty columns.
  pub const WRN_ANA_EMPTY_COLS : i32 = 902;
  /// Warn against close bounds.
  pub const WRN_ANA_CLOSE_BOUNDS : i32 = 903;
  /// Warn against almost integral bounds.
  pub const WRN_ANA_ALMOST_INT_BOUNDS : i32 = 904;
  /// An infeasibility report is not available when the problem contains matrix variables.
  pub const WRN_NO_INFEASIBILITY_REPORT_WHEN_MATRIX_VARIABLES : i32 = 930;
  /// No automatic dualizer is available for the specified problem.
  pub const WRN_NO_DUALIZER : i32 = 950;
  /// A numerically large value is specified for an element in E.
  pub const WRN_SYM_MAT_LARGE : i32 = 960;
  /// A double parameter related to solver tolerances has a non-default value.
  pub const WRN_MODIFIED_DOUBLE_PARAMETER : i32 = 970;
  /// A numerically large value is specified for an element in F.
  pub const WRN_LARGE_FIJ : i32 = 980;
  /// Invalid license.
  pub const ERR_LICENSE : i32 = 1000;
  /// The license has expired.
  pub const ERR_LICENSE_EXPIRED : i32 = 1001;
  /// Invalid license version.
  pub const ERR_LICENSE_VERSION : i32 = 1002;
  /// The license server version is too old.
  pub const ERR_LICENSE_OLD_SERVER_VERSION : i32 = 1003;
  /// The problem is bigger than the license.
  pub const ERR_SIZE_LICENSE : i32 = 1005;
  /// The software is not licensed to solve the problem.
  pub const ERR_PROB_LICENSE : i32 = 1006;
  /// Invalid license file.
  pub const ERR_FILE_LICENSE : i32 = 1007;
  /// A license cannot be located.
  pub const ERR_MISSING_LICENSE_FILE : i32 = 1008;
  /// The problem has too many constraints.
  pub const ERR_SIZE_LICENSE_CON : i32 = 1010;
  /// The problem has too many variables.
  pub const ERR_SIZE_LICENSE_VAR : i32 = 1011;
  /// The problem contains too many integer variables.
  pub const ERR_SIZE_LICENSE_INTVAR : i32 = 1012;
  /// The optimizer required is not licensed.
  pub const ERR_OPTIMIZER_LICENSE : i32 = 1013;
  /// The license manager reported an error.
  pub const ERR_FLEXLM : i32 = 1014;
  /// The license server is not responding.
  pub const ERR_LICENSE_SERVER : i32 = 1015;
  /// Maximum number of licenses is reached.
  pub const ERR_LICENSE_MAX : i32 = 1016;
  /// The MOSEKLM license manager daemon is not up and running.
  pub const ERR_LICENSE_MOSEKLM_DAEMON : i32 = 1017;
  /// A requested feature is not available in the license file(s).
  pub const ERR_LICENSE_FEATURE : i32 = 1018;
  /// A requested license feature is not available for the required platform.
  pub const ERR_PLATFORM_NOT_LICENSED : i32 = 1019;
  /// The license system cannot allocate the memory required.
  pub const ERR_LICENSE_CANNOT_ALLOCATE : i32 = 1020;
  /// MOSEK cannot connect to the license server.
  pub const ERR_LICENSE_CANNOT_CONNECT : i32 = 1021;
  /// The host ID specified in the license file does not match the host ID of the computer.
  pub const ERR_LICENSE_INVALID_HOSTID : i32 = 1025;
  /// The version specified in the checkout request is greater than the highest version number the daemon supports.
  pub const ERR_LICENSE_SERVER_VERSION : i32 = 1026;
  /// The license server does not support the requested feature.
  pub const ERR_LICENSE_NO_SERVER_SUPPORT : i32 = 1027;
  /// No SERVER lines in license file.
  pub const ERR_LICENSE_NO_SERVER_LINE : i32 = 1028;
  /// The dynamic link library is older than the specified version.
  pub const ERR_OLDER_DLL : i32 = 1035;
  /// The dynamic link library is newer than the specified version.
  pub const ERR_NEWER_DLL : i32 = 1036;
  /// A file cannot be linked to a stream in the DLL version.
  pub const ERR_LINK_FILE_DLL : i32 = 1040;
  /// Could not initialize a mutex.
  pub const ERR_THREAD_MUTEX_INIT : i32 = 1045;
  /// Could not lock a mutex.
  pub const ERR_THREAD_MUTEX_LOCK : i32 = 1046;
  /// Could not unlock a mutex.
  pub const ERR_THREAD_MUTEX_UNLOCK : i32 = 1047;
  /// Could not create a thread.
  pub const ERR_THREAD_CREATE : i32 = 1048;
  /// Could not initialize a condition.
  pub const ERR_THREAD_COND_INIT : i32 = 1049;
  /// Unknown error.
  pub const ERR_UNKNOWN : i32 = 1050;
  /// Out of space.
  pub const ERR_SPACE : i32 = 1051;
  /// An error occurred while opening a file.
  pub const ERR_FILE_OPEN : i32 = 1052;
  /// An error occurred while reading file.
  pub const ERR_FILE_READ : i32 = 1053;
  /// An error occurred while writing to a file.
  pub const ERR_FILE_WRITE : i32 = 1054;
  /// The data file format cannot be determined from the file name.
  pub const ERR_DATA_FILE_EXT : i32 = 1055;
  /// An invalid file name has been specified.
  pub const ERR_INVALID_FILE_NAME : i32 = 1056;
  /// An invalid file name has been specified.
  pub const ERR_INVALID_SOL_FILE_NAME : i32 = 1057;
  /// End of file reached.
  pub const ERR_END_OF_FILE : i32 = 1059;
  /// env is a null pointer.
  pub const ERR_NULL_ENV : i32 = 1060;
  /// task is a null pointer.
  pub const ERR_NULL_TASK : i32 = 1061;
  /// An invalid stream is referenced.
  pub const ERR_INVALID_STREAM : i32 = 1062;
  /// Environment is not initialized.
  pub const ERR_NO_INIT_ENV : i32 = 1063;
  /// The task is invalid.
  pub const ERR_INVALID_TASK : i32 = 1064;
  /// An argument to a function is unexpectedly a null pointer.
  pub const ERR_NULL_POINTER : i32 = 1065;
  /// Not all tasks associated with the environment have been deleted.
  pub const ERR_LIVING_TASKS : i32 = 1066;
  /// An all blank name has been specified.
  pub const ERR_BLANK_NAME : i32 = 1070;
  /// Duplicate names specified.
  pub const ERR_DUP_NAME : i32 = 1071;
  /// The name format string is invalid.
  pub const ERR_FORMAT_STRING : i32 = 1072;
  /// The sparsity included an index that was out of bounds of the shape.
  pub const ERR_SPARSITY_SPECIFICATION : i32 = 1073;
  /// Mismatching dimensions specified in arguments
  pub const ERR_MISMATCHING_DIMENSION : i32 = 1074;
  /// An invalid objective name is specified.
  pub const ERR_INVALID_OBJ_NAME : i32 = 1075;
  /// An invalid constraint name is used.
  pub const ERR_INVALID_CON_NAME : i32 = 1076;
  /// An invalid variable name is used.
  pub const ERR_INVALID_VAR_NAME : i32 = 1077;
  /// An invalid cone name is used.
  pub const ERR_INVALID_CONE_NAME : i32 = 1078;
  /// An invalid symmetric matrix variable name is used.
  pub const ERR_INVALID_BARVAR_NAME : i32 = 1079;
  /// MOSEK is leaking memory.
  pub const ERR_SPACE_LEAKING : i32 = 1080;
  /// No available information about the space usage.
  pub const ERR_SPACE_NO_INFO : i32 = 1081;
  /// Invalid dimension specification
  pub const ERR_DIMENSION_SPECIFICATION : i32 = 1082;
  /// Invalid axis names specification
  pub const ERR_AXIS_NAME_SPECIFICATION : i32 = 1083;
  /// The specified format cannot be read.
  pub const ERR_READ_FORMAT : i32 = 1090;
  /// An error occurred while reading an MPS file.
  pub const ERR_MPS_FILE : i32 = 1100;
  /// Invalid field occurred while reading an MPS file.
  pub const ERR_MPS_INV_FIELD : i32 = 1101;
  /// An invalid marker has been specified in the MPS file.
  pub const ERR_MPS_INV_MARKER : i32 = 1102;
  /// An empty constraint name is used in an MPS file.
  pub const ERR_MPS_NULL_CON_NAME : i32 = 1103;
  /// An empty variable name is used in an MPS file.
  pub const ERR_MPS_NULL_VAR_NAME : i32 = 1104;
  /// An undefined constraint name occurred in an MPS file.
  pub const ERR_MPS_UNDEF_CON_NAME : i32 = 1105;
  /// An undefined variable name occurred in an MPS file.
  pub const ERR_MPS_UNDEF_VAR_NAME : i32 = 1106;
  /// An invalid constraint key occurred in an MPS file.
  pub const ERR_MPS_INVALID_CON_KEY : i32 = 1107;
  /// An invalid bound key occurred in an MPS file.
  pub const ERR_MPS_INVALID_BOUND_KEY : i32 = 1108;
  /// An invalid section name occurred in an MPS file.
  pub const ERR_MPS_INVALID_SEC_NAME : i32 = 1109;
  /// No objective is defined in an MPS file.
  pub const ERR_MPS_NO_OBJECTIVE : i32 = 1110;
  /// The non-zero elements in A corresponding to a variable in an MPS file must be specified consecutively.
  pub const ERR_MPS_SPLITTED_VAR : i32 = 1111;
  /// A constraint name is specified multiple times in the ROWS section in an MPS file.
  pub const ERR_MPS_MUL_CON_NAME : i32 = 1112;
  /// Multiple QSECTIONs are specified for a constraint.
  pub const ERR_MPS_MUL_QSEC : i32 = 1113;
  /// The Q term in the objective is specified multiple times.
  pub const ERR_MPS_MUL_QOBJ : i32 = 1114;
  /// The sections in an MPS file is not in the correct order.
  pub const ERR_MPS_INV_SEC_ORDER : i32 = 1115;
  /// Multiple CSECTIONs are given the same name.
  pub const ERR_MPS_MUL_CSEC : i32 = 1116;
  /// Invalid cone type specified in a  CSECTION.
  pub const ERR_MPS_CONE_TYPE : i32 = 1117;
  /// A variable is specified to be a member of several cones.
  pub const ERR_MPS_CONE_OVERLAP : i32 = 1118;
  /// A variable is repeated within the CSECTION.
  pub const ERR_MPS_CONE_REPEAT : i32 = 1119;
  /// A non symmetric matrix has been speciefied.
  pub const ERR_MPS_NON_SYMMETRIC_Q : i32 = 1120;
  /// Duplicate elements is specified in a Q matrix.
  pub const ERR_MPS_DUPLICATE_Q_ELEMENT : i32 = 1121;
  /// An invalid objective sense is specified.
  pub const ERR_MPS_INVALID_OBJSENSE : i32 = 1122;
  /// A tab char occurred in field 2.
  pub const ERR_MPS_TAB_IN_FIELD2 : i32 = 1125;
  /// A tab char occurred in field 3.
  pub const ERR_MPS_TAB_IN_FIELD3 : i32 = 1126;
  /// A tab char occurred in field 5.
  pub const ERR_MPS_TAB_IN_FIELD5 : i32 = 1127;
  /// An invalid objective name is specified.
  pub const ERR_MPS_INVALID_OBJ_NAME : i32 = 1128;
  /// An invalid indicator key occurred in an MPS file.
  pub const ERR_MPS_INVALID_KEY : i32 = 1129;
  /// An invalid indicator constraint is used. It must not be a ranged constraint.
  pub const ERR_MPS_INVALID_INDICATOR_CONSTRAINT : i32 = 1130;
  /// An invalid indicator variable is specfied. It must be a binary variable.
  pub const ERR_MPS_INVALID_INDICATOR_VARIABLE : i32 = 1131;
  /// An invalid indicator value is specfied. It must be either 0 or 1.
  pub const ERR_MPS_INVALID_INDICATOR_VALUE : i32 = 1132;
  /// A quadratic constraint can be be an indicator constraint.
  pub const ERR_MPS_INVALID_INDICATOR_QUADRATIC_CONSTRAINT : i32 = 1133;
  /// Syntax error in an OPF file
  pub const ERR_OPF_SYNTAX : i32 = 1134;
  /// Premature end of file in an OPF file.
  pub const ERR_OPF_PREMATURE_EOF : i32 = 1136;
  /// Mismatched end-tag in OPF file
  pub const ERR_OPF_MISMATCHED_TAG : i32 = 1137;
  /// Either upper or lower bound was specified twice in OPF file
  pub const ERR_OPF_DUPLICATE_BOUND : i32 = 1138;
  /// Duplicate constraint name in OPF File
  pub const ERR_OPF_DUPLICATE_CONSTRAINT_NAME : i32 = 1139;
  /// Invalid cone type in OPF File
  pub const ERR_OPF_INVALID_CONE_TYPE : i32 = 1140;
  /// Invalid number of parameters in start-tag in OPF File
  pub const ERR_OPF_INCORRECT_TAG_PARAM : i32 = 1141;
  /// Invalid start-tag in OPF File
  pub const ERR_OPF_INVALID_TAG : i32 = 1142;
  /// Same variable appears in multiple cones in OPF File
  pub const ERR_OPF_DUPLICATE_CONE_ENTRY : i32 = 1143;
  /// The problem is too large to be correctly loaded
  pub const ERR_OPF_TOO_LARGE : i32 = 1144;
  /// Dual solution values are not allowed in OPF File
  pub const ERR_OPF_DUAL_INTEGER_SOLUTION : i32 = 1146;
  /// The problem cannot be written to an LP formatted file.
  pub const ERR_LP_EMPTY : i32 = 1151;
  /// An invalid name is created while writing an MPS file.
  pub const ERR_WRITE_MPS_INVALID_NAME : i32 = 1153;
  /// A variable name is invalid when used in an LP formatted file.
  pub const ERR_LP_INVALID_VAR_NAME : i32 = 1154;
  /// Empty variable names cannot be written to OPF files.
  pub const ERR_WRITE_OPF_INVALID_VAR_NAME : i32 = 1156;
  /// Syntax error in an LP file.
  pub const ERR_LP_FILE_FORMAT : i32 = 1157;
  /// Expected a number in LP file
  pub const ERR_LP_EXPECTED_NUMBER : i32 = 1158;
  /// Syntax error in LP fil. Possibly missing End tag.
  pub const ERR_READ_LP_MISSING_END_TAG : i32 = 1159;
  /// An indicator variable was not declared binary
  pub const ERR_LP_INDICATOR_VAR : i32 = 1160;
  /// Expected an objective section in LP file
  pub const ERR_LP_EXPECTED_OBJECTIVE : i32 = 1161;
  /// Expected constraint relation
  pub const ERR_LP_EXPECTED_CONSTRAINT_RELATION : i32 = 1162;
  /// Constraint has ambiguous or invalid bound
  pub const ERR_LP_AMBIGUOUS_CONSTRAINT_BOUND : i32 = 1163;
  /// Duplicate section
  pub const ERR_LP_DUPLICATE_SECTION : i32 = 1164;
  /// An error occurred while writing file
  pub const ERR_WRITING_FILE : i32 = 1166;
  /// An invalid name occurred in a solution file.
  pub const ERR_INVALID_NAME_IN_SOL_FILE : i32 = 1170;
  /// Syntax error in an JSON data
  pub const ERR_JSON_SYNTAX : i32 = 1175;
  /// Error in JSON string.
  pub const ERR_JSON_STRING : i32 = 1176;
  /// Invalid number entry - wrong type or value overflow.
  pub const ERR_JSON_NUMBER_OVERFLOW : i32 = 1177;
  /// Error in an JSON Task file
  pub const ERR_JSON_FORMAT : i32 = 1178;
  /// Inconsistent data in JSON Task file
  pub const ERR_JSON_DATA : i32 = 1179;
  /// Missing data section in JSON task file.
  pub const ERR_JSON_MISSING_DATA : i32 = 1180;
  /// Incompatible item
  pub const ERR_PTF_INCOMPATIBILITY : i32 = 1181;
  /// Undefined symbol referenced
  pub const ERR_PTF_UNDEFINED_ITEM : i32 = 1182;
  /// Inconsistent size of item
  pub const ERR_PTF_INCONSISTENCY : i32 = 1183;
  /// Syntax error in an PTF file
  pub const ERR_PTF_FORMAT : i32 = 1184;
  /// Incorrect length of arguments.
  pub const ERR_ARGUMENT_LENNEQ : i32 = 1197;
  /// Incorrect argument type.
  pub const ERR_ARGUMENT_TYPE : i32 = 1198;
  /// Incorrect number of function arguments.
  pub const ERR_NUM_ARGUMENTS : i32 = 1199;
  /// A function argument is incorrect.
  pub const ERR_IN_ARGUMENT : i32 = 1200;
  /// A function argument is of incorrect dimension.
  pub const ERR_ARGUMENT_DIMENSION : i32 = 1201;
  /// The size of the n-dimensional shape is too large.
  pub const ERR_SHAPE_IS_TOO_LARGE : i32 = 1202;
  /// An index in an argument is too small.
  pub const ERR_INDEX_IS_TOO_SMALL : i32 = 1203;
  /// An index in an argument is too large.
  pub const ERR_INDEX_IS_TOO_LARGE : i32 = 1204;
  /// An index in an argument is is unique.
  pub const ERR_INDEX_IS_NOT_UNIQUE : i32 = 1205;
  /// A parameter name is not correct.
  pub const ERR_PARAM_NAME : i32 = 1206;
  /// A parameter name is not correct.
  pub const ERR_PARAM_NAME_DOU : i32 = 1207;
  /// A parameter name is not correct.
  pub const ERR_PARAM_NAME_INT : i32 = 1208;
  /// A parameter name is not correct.
  pub const ERR_PARAM_NAME_STR : i32 = 1209;
  /// Parameter index is out of range.
  pub const ERR_PARAM_INDEX : i32 = 1210;
  /// A parameter value is too large.
  pub const ERR_PARAM_IS_TOO_LARGE : i32 = 1215;
  /// A parameter value is too small.
  pub const ERR_PARAM_IS_TOO_SMALL : i32 = 1216;
  /// A parameter value string is incorrect.
  pub const ERR_PARAM_VALUE_STR : i32 = 1217;
  /// A parameter type is invalid.
  pub const ERR_PARAM_TYPE : i32 = 1218;
  /// A double information index is out of range for the specified type.
  pub const ERR_INF_DOU_INDEX : i32 = 1219;
  /// An integer information index is out of range for the specified type.
  pub const ERR_INF_INT_INDEX : i32 = 1220;
  /// An index in an array argument is too small.
  pub const ERR_INDEX_ARR_IS_TOO_SMALL : i32 = 1221;
  /// An index in an array argument is too large.
  pub const ERR_INDEX_ARR_IS_TOO_LARGE : i32 = 1222;
  /// A long integer information index is out of range for the specified type.
  pub const ERR_INF_LINT_INDEX : i32 = 1225;
  /// The value of a argument is too small.
  pub const ERR_ARG_IS_TOO_SMALL : i32 = 1226;
  /// The value of a argument is too large.
  pub const ERR_ARG_IS_TOO_LARGE : i32 = 1227;
  /// whichsol is invalid.
  pub const ERR_INVALID_WHICHSOL : i32 = 1228;
  /// A double information name is invalid.
  pub const ERR_INF_DOU_NAME : i32 = 1230;
  /// An integer information name is invalid.
  pub const ERR_INF_INT_NAME : i32 = 1231;
  /// The information type is invalid.
  pub const ERR_INF_TYPE : i32 = 1232;
  /// A long integer information name is invalid.
  pub const ERR_INF_LINT_NAME : i32 = 1234;
  /// An index is out of range.
  pub const ERR_INDEX : i32 = 1235;
  /// The solution defined by whichsol does not exists.
  pub const ERR_WHICHSOL : i32 = 1236;
  /// The solution number  solemn does not exists.
  pub const ERR_SOLITEM : i32 = 1237;
  /// whichitem is unacceptable.
  pub const ERR_WHICHITEM_NOT_ALLOWED : i32 = 1238;
  /// Invalid maximum number of constraints specified.
  pub const ERR_MAXNUMCON : i32 = 1240;
  /// The maximum number of variables limit is too small.
  pub const ERR_MAXNUMVAR : i32 = 1241;
  /// The maximum number of semidefinite variables limit is too small.
  pub const ERR_MAXNUMBARVAR : i32 = 1242;
  /// Too small maximum number of non-zeros for the Q matrices is specified.
  pub const ERR_MAXNUMQNZ : i32 = 1243;
  /// The maximum number of non-zeros specified is too small.
  pub const ERR_TOO_SMALL_MAX_NUM_NZ : i32 = 1245;
  /// A specified index is invalid.
  pub const ERR_INVALID_IDX : i32 = 1246;
  /// A specified index is invalid.
  pub const ERR_INVALID_MAX_NUM : i32 = 1247;
  /// The value of whichsol is not allowed.
  pub const ERR_UNALLOWED_WHICHSOL : i32 = 1248;
  /// Maximum number of constraints limit is exceeded.
  pub const ERR_NUMCONLIM : i32 = 1250;
  /// Maximum number of variables limit is exceeded.
  pub const ERR_NUMVARLIM : i32 = 1251;
  /// Too small maximum number of non-zeros in A specified.
  pub const ERR_TOO_SMALL_MAXNUMANZ : i32 = 1252;
  /// aptre\[j\] is strictly smaller than aptrb\[j\] for some j.
  pub const ERR_INV_APTRE : i32 = 1253;
  /// An element in A is defined multiple times.
  pub const ERR_MUL_A_ELEMENT : i32 = 1254;
  /// Invalid bound key.
  pub const ERR_INV_BK : i32 = 1255;
  /// Invalid bound key is specified for a constraint.
  pub const ERR_INV_BKC : i32 = 1256;
  /// An invalid bound key is specified for a variable.
  pub const ERR_INV_BKX : i32 = 1257;
  /// An invalid variable type is specified for a variable.
  pub const ERR_INV_VAR_TYPE : i32 = 1258;
  /// Problem type does not match the chosen optimizer.
  pub const ERR_SOLVER_PROBTYPE : i32 = 1259;
  /// Empty objective range.
  pub const ERR_OBJECTIVE_RANGE : i32 = 1260;
  /// Invalid response code.
  pub const ERR_INV_RESCODE : i32 = 1261;
  /// Invalid integer information item.
  pub const ERR_INV_IINF : i32 = 1262;
  /// Invalid long integer information item.
  pub const ERR_INV_LIINF : i32 = 1263;
  /// Invalid double information item.
  pub const ERR_INV_DINF : i32 = 1264;
  /// Invalid basis is specified.
  pub const ERR_BASIS : i32 = 1266;
  /// Invalid value in skc encountered.
  pub const ERR_INV_SKC : i32 = 1267;
  /// Invalid value in skx encountered.
  pub const ERR_INV_SKX : i32 = 1268;
  /// Invalid status key string encountered.
  pub const ERR_INV_SK_STR : i32 = 1269;
  /// Invalid status key code encountered.
  pub const ERR_INV_SK : i32 = 1270;
  /// Invalid cone type string encountered.
  pub const ERR_INV_CONE_TYPE_STR : i32 = 1271;
  /// Invalid cone type code encountered.
  pub const ERR_INV_CONE_TYPE : i32 = 1272;
  /// Invalid value in skn encountered.
  pub const ERR_INV_SKN : i32 = 1274;
  /// Invalid surplus.
  pub const ERR_INVALID_SURPLUS : i32 = 1275;
  /// An invalid name item code is used.
  pub const ERR_INV_NAME_ITEM : i32 = 1280;
  /// An invalid problem item is used.
  pub const ERR_PRO_ITEM : i32 = 1281;
  /// Invalid format type.
  pub const ERR_INVALID_FORMAT_TYPE : i32 = 1283;
  /// Invalid firsti.
  pub const ERR_FIRSTI : i32 = 1285;
  /// Invalid lasti.
  pub const ERR_LASTI : i32 = 1286;
  /// Invalid firstj.
  pub const ERR_FIRSTJ : i32 = 1287;
  /// Invalid lastj.
  pub const ERR_LASTJ : i32 = 1288;
  /// A maximum length that is too small has been specified.
  pub const ERR_MAX_LEN_IS_TOO_SMALL : i32 = 1289;
  /// The model contains a nonlinear equality.
  pub const ERR_NONLINEAR_EQUALITY : i32 = 1290;
  /// The optimization problem is nonconvex.
  pub const ERR_NONCONVEX : i32 = 1291;
  /// The problem contains a nonlinear constraint with inite lower and upper bound.
  pub const ERR_NONLINEAR_RANGED : i32 = 1292;
  /// The quadratic constraint matrix is not PSD.
  pub const ERR_CON_Q_NOT_PSD : i32 = 1293;
  /// The quadratic constraint matrix is not NSD.
  pub const ERR_CON_Q_NOT_NSD : i32 = 1294;
  /// The quadratic coefficient matrix in the objective is not PSD.
  pub const ERR_OBJ_Q_NOT_PSD : i32 = 1295;
  /// The quadratic coefficient matrix in the objective is not NSD.
  pub const ERR_OBJ_Q_NOT_NSD : i32 = 1296;
  /// An invalid permutation array is specified.
  pub const ERR_ARGUMENT_PERM_ARRAY : i32 = 1299;
  /// An index of a non-existing cone has been specified.
  pub const ERR_CONE_INDEX : i32 = 1300;
  /// A cone with incorrect number of members is specified.
  pub const ERR_CONE_SIZE : i32 = 1301;
  /// One or more of variables in the cone to be added is already member of another cone.
  pub const ERR_CONE_OVERLAP : i32 = 1302;
  /// A variable is included multiple times in the cone.
  pub const ERR_CONE_REP_VAR : i32 = 1303;
  /// The value specified for maxnumcone is too small.
  pub const ERR_MAXNUMCONE : i32 = 1304;
  /// Invalid cone type specified.
  pub const ERR_CONE_TYPE : i32 = 1305;
  /// Invalid cone type specified.
  pub const ERR_CONE_TYPE_STR : i32 = 1306;
  /// The cone to be appended has one variable which is already member of another cone.
  pub const ERR_CONE_OVERLAP_APPEND : i32 = 1307;
  /// A variable cannot be removed because it will make a cone invalid.
  pub const ERR_REMOVE_CONE_VARIABLE : i32 = 1310;
  /// Trying to append a too big cone.
  pub const ERR_APPENDING_TOO_BIG_CONE : i32 = 1311;
  /// An invalid cone parameter.
  pub const ERR_CONE_PARAMETER : i32 = 1320;
  /// An invalid number is specified in a solution file.
  pub const ERR_SOL_FILE_INVALID_NUMBER : i32 = 1350;
  /// A huge value in absolute size is specified for an objective coefficient.
  pub const ERR_HUGE_C : i32 = 1375;
  /// A numerically huge value is specified for an element in A.
  pub const ERR_HUGE_AIJ : i32 = 1380;
  /// An element in the A matrix is specified twice.
  pub const ERR_DUPLICATE_AIJ : i32 = 1385;
  /// The lower bound specified is not a number (nan).
  pub const ERR_LOWER_BOUND_IS_A_NAN : i32 = 1390;
  /// The upper bound specified is not a number (nan).
  pub const ERR_UPPER_BOUND_IS_A_NAN : i32 = 1391;
  /// A numerically huge bound value is specified.
  pub const ERR_INFINITE_BOUND : i32 = 1400;
  /// Invalid value %d at qosubi.
  pub const ERR_INV_QOBJ_SUBI : i32 = 1401;
  /// Invalid value in qosubj.
  pub const ERR_INV_QOBJ_SUBJ : i32 = 1402;
  /// Invalid value in qoval.
  pub const ERR_INV_QOBJ_VAL : i32 = 1403;
  /// Invalid value in qcsubk.
  pub const ERR_INV_QCON_SUBK : i32 = 1404;
  /// Invalid value in qcsubi.
  pub const ERR_INV_QCON_SUBI : i32 = 1405;
  /// Invalid value in qcsubj.
  pub const ERR_INV_QCON_SUBJ : i32 = 1406;
  /// Invalid value in qcval.
  pub const ERR_INV_QCON_VAL : i32 = 1407;
  /// Invalid value in qcsubi.
  pub const ERR_QCON_SUBI_TOO_SMALL : i32 = 1408;
  /// Invalid value in qcsubi.
  pub const ERR_QCON_SUBI_TOO_LARGE : i32 = 1409;
  /// An element in the upper triangle of the quadratic term in the objective is specified.
  pub const ERR_QOBJ_UPPER_TRIANGLE : i32 = 1415;
  /// An element in the upper triangle of the quadratic term in a constraint.
  pub const ERR_QCON_UPPER_TRIANGLE : i32 = 1417;
  /// A fixed constraint/variable has been specified using the bound keys but the numerical bounds are different.
  pub const ERR_FIXED_BOUND_VALUES : i32 = 1420;
  /// A too small value for the A trucation value is specified.
  pub const ERR_TOO_SMALL_A_TRUNCATION_VALUE : i32 = 1421;
  /// An invalid objective sense is specified.
  pub const ERR_INVALID_OBJECTIVE_SENSE : i32 = 1445;
  /// The objective sense has not been specified before the optimization.
  pub const ERR_UNDEFINED_OBJECTIVE_SENSE : i32 = 1446;
  /// The solution item y is undefined.
  pub const ERR_Y_IS_UNDEFINED : i32 = 1449;
  /// An invalid floating value was used in some double data.
  pub const ERR_NAN_IN_DOUBLE_DATA : i32 = 1450;
  /// An infinite floating value was used in some double data.
  pub const ERR_INF_IN_DOUBLE_DATA : i32 = 1451;
  /// blc contains an invalid floating point value, i.e. a NaN.
  pub const ERR_NAN_IN_BLC : i32 = 1461;
  /// buc contains an invalid floating point value, i.e. a NaN.
  pub const ERR_NAN_IN_BUC : i32 = 1462;
  /// An invalid fixed term in the objective is speficied.
  pub const ERR_INVALID_CFIX : i32 = 1469;
  /// c contains an invalid floating point value, i.e. a NaN.
  pub const ERR_NAN_IN_C : i32 = 1470;
  /// blx contains an invalid floating point value, i.e. a NaN.
  pub const ERR_NAN_IN_BLX : i32 = 1471;
  /// bux contains an invalid floating point value, i.e. a NaN.
  pub const ERR_NAN_IN_BUX : i32 = 1472;
  /// a\[i,j\] contains an invalid floating point value, i.e. a NaN or an infinite value.
  pub const ERR_INVALID_AIJ : i32 = 1473;
  /// c\[j\] contains an invalid floating point value, i.e. a NaN or an infinite value.
  pub const ERR_INVALID_CJ : i32 = 1474;
  /// A symmetric matrix contains an invalid floating point value, i.e. a NaN or an infinite value.
  pub const ERR_SYM_MAT_INVALID : i32 = 1480;
  /// A numerically huge value is specified for an element in E.
  pub const ERR_SYM_MAT_HUGE : i32 = 1482;
  /// Invalid problem type.
  pub const ERR_INV_PROBLEM : i32 = 1500;
  /// The problem contains both conic and nonlinear constraints.
  pub const ERR_MIXED_CONIC_AND_NL : i32 = 1501;
  /// The global optimizer can only be applied to problems without semidefinite variables.
  pub const ERR_GLOBAL_INV_CONIC_PROBLEM : i32 = 1503;
  /// An invalid optimizer has been chosen for the problem.
  pub const ERR_INV_OPTIMIZER : i32 = 1550;
  /// No optimizer is available for the current class of integer optimization problems.
  pub const ERR_MIO_NO_OPTIMIZER : i32 = 1551;
  /// No optimizer is available for this class of optimization problems.
  pub const ERR_NO_OPTIMIZER_VAR_TYPE : i32 = 1552;
  /// An error occurred during the solution finalization.
  pub const ERR_FINAL_SOLUTION : i32 = 1560;
  /// Invalid first.
  pub const ERR_FIRST : i32 = 1570;
  /// Invalid last.
  pub const ERR_LAST : i32 = 1571;
  /// Invalid slice size specified.
  pub const ERR_SLICE_SIZE : i32 = 1572;
  /// Negative surplus.
  pub const ERR_NEGATIVE_SURPLUS : i32 = 1573;
  /// Cannot append a negative number.
  pub const ERR_NEGATIVE_APPEND : i32 = 1578;
  /// An error occurred during the postsolve.
  pub const ERR_POSTSOLVE : i32 = 1580;
  /// A computation produced an overflow.
  pub const ERR_OVERFLOW : i32 = 1590;
  /// No basic solution is defined.
  pub const ERR_NO_BASIS_SOL : i32 = 1600;
  /// The factorization of the basis is invalid.
  pub const ERR_BASIS_FACTOR : i32 = 1610;
  /// The basis is singular.
  pub const ERR_BASIS_SINGULAR : i32 = 1615;
  /// An error occurred while factorizing a matrix.
  pub const ERR_FACTOR : i32 = 1650;
  /// An optimization problem cannot be relaxed.
  pub const ERR_FEASREPAIR_CANNOT_RELAX : i32 = 1700;
  /// The relaxed problem could not be solved to optimality.
  pub const ERR_FEASREPAIR_SOLVING_RELAXED : i32 = 1701;
  /// The upper bound is less than the lower bound for a variable or a constraint.
  pub const ERR_FEASREPAIR_INCONSISTENT_BOUND : i32 = 1702;
  /// The feasibility repair does not support the specified problem type.
  pub const ERR_REPAIR_INVALID_PROBLEM : i32 = 1710;
  /// Computation the optimal relaxation failed.
  pub const ERR_REPAIR_OPTIMIZATION_FAILED : i32 = 1711;
  /// A name is longer than the buffer that is supposed to hold it.
  pub const ERR_NAME_MAX_LEN : i32 = 1750;
  /// The name buffer is a null pointer.
  pub const ERR_NAME_IS_NULL : i32 = 1760;
  /// Invalid compression type.
  pub const ERR_INVALID_COMPRESSION : i32 = 1800;
  /// Invalid io mode.
  pub const ERR_INVALID_IOMODE : i32 = 1801;
  /// A certificate of primal infeasibility is not available.
  pub const ERR_NO_PRIMAL_INFEAS_CER : i32 = 2000;
  /// A certificate of dual infeasibility is not available.
  pub const ERR_NO_DUAL_INFEAS_CER : i32 = 2001;
  /// The required solution is not available.
  pub const ERR_NO_SOLUTION_IN_CALLBACK : i32 = 2500;
  /// Invalid value in marki.
  pub const ERR_INV_MARKI : i32 = 2501;
  /// Invalid value in markj.
  pub const ERR_INV_MARKJ : i32 = 2502;
  /// Invalid numi.
  pub const ERR_INV_NUMI : i32 = 2503;
  /// Invalid numj.
  pub const ERR_INV_NUMJ : i32 = 2504;
  /// The Task file is incompatible with this platform.
  pub const ERR_TASK_INCOMPATIBLE : i32 = 2560;
  /// The Task file is invalid.
  pub const ERR_TASK_INVALID : i32 = 2561;
  /// Failed to write the task file.
  pub const ERR_TASK_WRITE : i32 = 2562;
  /// Could not compute the LU factors of the matrix within the maximum number of allowed tries.
  pub const ERR_LU_MAX_NUM_TRIES : i32 = 2800;
  /// An invalid UTF8 string is encountered.
  pub const ERR_INVALID_UTF8 : i32 = 2900;
  /// An invalid wchar string is encountered.
  pub const ERR_INVALID_WCHAR : i32 = 2901;
  /// No dual information is available for the integer solution.
  pub const ERR_NO_DUAL_FOR_ITG_SOL : i32 = 2950;
  /// snx is not available for the basis solution.
  pub const ERR_NO_SNX_FOR_BAS_SOL : i32 = 2953;
  /// An internal error occurred.
  pub const ERR_INTERNAL : i32 = 3000;
  /// An input array was too short.
  pub const ERR_API_ARRAY_TOO_SMALL : i32 = 3001;
  /// Failed to connect a callback object.
  pub const ERR_API_CB_CONNECT : i32 = 3002;
  /// An internal error occurred in the API. Please report this problem.
  pub const ERR_API_FATAL_ERROR : i32 = 3005;
  /// Syntax error in sensitivity analysis file.
  pub const ERR_SEN_FORMAT : i32 = 3050;
  /// An undefined name was encountered in the sensitivity analysis file.
  pub const ERR_SEN_UNDEF_NAME : i32 = 3051;
  /// Index out of range in the sensitivity analysis file.
  pub const ERR_SEN_INDEX_RANGE : i32 = 3052;
  /// Analysis of upper bound requested for an index, where no upper bound exists.
  pub const ERR_SEN_BOUND_INVALID_UP : i32 = 3053;
  /// Analysis of lower bound requested for an index, where no lower bound exists.
  pub const ERR_SEN_BOUND_INVALID_LO : i32 = 3054;
  /// Invalid range given in the sensitivity file.
  pub const ERR_SEN_INDEX_INVALID : i32 = 3055;
  /// Syntax error in regexp or regexp longer than 1024.
  pub const ERR_SEN_INVALID_REGEXP : i32 = 3056;
  /// No optimal solution found to the original problem given for sensitivity analysis.
  pub const ERR_SEN_SOLUTION_STATUS : i32 = 3057;
  /// Numerical difficulties encountered performing the sensitivity analysis.
  pub const ERR_SEN_NUMERICAL : i32 = 3058;
  /// Sensitivity analysis cannot be performed for the specified problem.
  pub const ERR_SEN_UNHANDLED_PROBLEM_TYPE : i32 = 3080;
  /// A step-size in an optimizer was unexpectedly unbounded.
  pub const ERR_UNB_STEP_SIZE : i32 = 3100;
  /// Some tasks related to this function call were identical. Unique tasks were expected.
  pub const ERR_IDENTICAL_TASKS : i32 = 3101;
  /// The code list data was invalid.
  pub const ERR_AD_INVALID_CODELIST : i32 = 3102;
  /// An internal unit test function failed.
  pub const ERR_INTERNAL_TEST_FAILED : i32 = 3500;
  /// The problem type is not supported by the XML format.
  pub const ERR_XML_INVALID_PROBLEM_TYPE : i32 = 3600;
  /// Invalid AMPL stub.
  pub const ERR_INVALID_AMPL_STUB : i32 = 3700;
  /// A 64 bit integer could not be cast to a 32 bit integer.
  pub const ERR_INT64_TO_INT32_CAST : i32 = 3800;
  /// The computer contains more cpu cores than the license allows for.
  pub const ERR_SIZE_LICENSE_NUMCORES : i32 = 3900;
  /// The requested value is not defined for this solution type.
  pub const ERR_INFEAS_UNDEFINED : i32 = 3910;
  /// There is no barx available for the solution specified.
  pub const ERR_NO_BARX_FOR_SOLUTION : i32 = 3915;
  /// There is no bars available for the solution specified.
  pub const ERR_NO_BARS_FOR_SOLUTION : i32 = 3916;
  /// The dimension of a symmetric matrix variable has to be greater than 0.
  pub const ERR_BAR_VAR_DIM : i32 = 3920;
  /// A row index specified for sparse symmetric matrix is invalid.
  pub const ERR_SYM_MAT_INVALID_ROW_INDEX : i32 = 3940;
  /// A column index specified for sparse symmetric matrix is invalid.
  pub const ERR_SYM_MAT_INVALID_COL_INDEX : i32 = 3941;
  /// Only the lower triangular part of sparse symmetric matrix should be specified.
  pub const ERR_SYM_MAT_NOT_LOWER_TRINGULAR : i32 = 3942;
  /// The numerical value specified in a sparse symmetric matrix is not a floating point value.
  pub const ERR_SYM_MAT_INVALID_VALUE : i32 = 3943;
  /// A value in a symmetric matric as been specified more than once.
  pub const ERR_SYM_MAT_DUPLICATE : i32 = 3944;
  /// A sparse symmetric matrix of invalid dimension is specified.
  pub const ERR_INVALID_SYM_MAT_DIM : i32 = 3950;
  /// An internal fatal error occurred in an interface function.
  pub const ERR_API_INTERNAL : i32 = 3999;
  /// The file format does not support a problem with symmetric matrix variables.
  pub const ERR_INVALID_FILE_FORMAT_FOR_SYM_MAT : i32 = 4000;
  /// The file format does not support a problem with nonzero fixed term in c.
  pub const ERR_INVALID_FILE_FORMAT_FOR_CFIX : i32 = 4001;
  /// The file format does not support a problem with ranged constraints.
  pub const ERR_INVALID_FILE_FORMAT_FOR_RANGED_CONSTRAINTS : i32 = 4002;
  /// The file format does not support a problem with free constraints.
  pub const ERR_INVALID_FILE_FORMAT_FOR_FREE_CONSTRAINTS : i32 = 4003;
  /// The file format does not support a problem with the simple cones (deprecated).
  pub const ERR_INVALID_FILE_FORMAT_FOR_CONES : i32 = 4005;
  /// The file format does not support a problem with quadratic terms.
  pub const ERR_INVALID_FILE_FORMAT_FOR_QUADRATIC_TERMS : i32 = 4006;
  /// The file format does not support a problem with nonlinear terms.
  pub const ERR_INVALID_FILE_FORMAT_FOR_NONLINEAR : i32 = 4010;
  /// The file format does not support a problem with disjunctive constraints.
  pub const ERR_INVALID_FILE_FORMAT_FOR_DISJUNCTIVE_CONSTRAINTS : i32 = 4011;
  /// The file format does not support a problem with affine conic constraints.
  pub const ERR_INVALID_FILE_FORMAT_FOR_AFFINE_CONIC_CONSTRAINTS : i32 = 4012;
  /// Two constraint names are identical.
  pub const ERR_DUPLICATE_CONSTRAINT_NAMES : i32 = 4500;
  /// Two variable names are identical.
  pub const ERR_DUPLICATE_VARIABLE_NAMES : i32 = 4501;
  /// Two barvariable names are identical.
  pub const ERR_DUPLICATE_BARVARIABLE_NAMES : i32 = 4502;
  /// Two cone names are identical.
  pub const ERR_DUPLICATE_CONE_NAMES : i32 = 4503;
  /// Two domain names are identical.
  pub const ERR_DUPLICATE_DOMAIN_NAMES : i32 = 4504;
  /// Two disjunctive constraint names are identical.
  pub const ERR_DUPLICATE_DJC_NAMES : i32 = 4505;
  /// An array does not contain unique elements.
  pub const ERR_NON_UNIQUE_ARRAY : i32 = 5000;
  /// The value of a function argument is too small.
  pub const ERR_ARGUMENT_IS_TOO_SMALL : i32 = 5004;
  /// The value of a function argument is too large.
  pub const ERR_ARGUMENT_IS_TOO_LARGE : i32 = 5005;
  /// A fatal error occurred in the mixed integer optimizer.  Please contact MOSEK support.
  pub const ERR_MIO_INTERNAL : i32 = 5010;
  /// An invalid problem type.
  pub const ERR_INVALID_PROBLEM_TYPE : i32 = 6000;
  /// Unhandled solution status.
  pub const ERR_UNHANDLED_SOLUTION_STATUS : i32 = 6010;
  /// An element in the upper triangle of a lower triangular matrix is specified.
  pub const ERR_UPPER_TRIANGLE : i32 = 6020;
  /// A matrix is singular.
  pub const ERR_LAU_SINGULAR_MATRIX : i32 = 7000;
  /// A matrix is not positive definite.
  pub const ERR_LAU_NOT_POSITIVE_DEFINITE : i32 = 7001;
  /// An invalid lower triangular matrix.
  pub const ERR_LAU_INVALID_LOWER_TRIANGULAR_MATRIX : i32 = 7002;
  /// An unknown error.
  pub const ERR_LAU_UNKNOWN : i32 = 7005;
  /// Invalid argument m.
  pub const ERR_LAU_ARG_M : i32 = 7010;
  /// Invalid argument n.
  pub const ERR_LAU_ARG_N : i32 = 7011;
  /// Invalid argument k.
  pub const ERR_LAU_ARG_K : i32 = 7012;
  /// Invalid argument transa.
  pub const ERR_LAU_ARG_TRANSA : i32 = 7015;
  /// Invalid argument transb.
  pub const ERR_LAU_ARG_TRANSB : i32 = 7016;
  /// Invalid argument uplo.
  pub const ERR_LAU_ARG_UPLO : i32 = 7017;
  /// Invalid argument trans.
  pub const ERR_LAU_ARG_TRANS : i32 = 7018;
  /// An invalid sparse symmetric matrix is specfified.
  pub const ERR_LAU_INVALID_SPARSE_SYMMETRIC_MATRIX : i32 = 7019;
  /// An error occurred while parsing an CBF file.
  pub const ERR_CBF_PARSE : i32 = 7100;
  /// An invalid objective sense is specified.
  pub const ERR_CBF_OBJ_SENSE : i32 = 7101;
  /// An invalid objective sense is specified.
  pub const ERR_CBF_NO_VARIABLES : i32 = 7102;
  /// Too many constraints specified.
  pub const ERR_CBF_TOO_MANY_CONSTRAINTS : i32 = 7103;
  /// Too many variables specified.
  pub const ERR_CBF_TOO_MANY_VARIABLES : i32 = 7104;
  /// No version specified.
  pub const ERR_CBF_NO_VERSION_SPECIFIED : i32 = 7105;
  /// Invalid syntax.
  pub const ERR_CBF_SYNTAX : i32 = 7106;
  /// Duplicate OBJ keyword.
  pub const ERR_CBF_DUPLICATE_OBJ : i32 = 7107;
  /// Duplicate CON keyword.
  pub const ERR_CBF_DUPLICATE_CON : i32 = 7108;
  /// Duplicate VAR keyword.
  pub const ERR_CBF_DUPLICATE_VAR : i32 = 7110;
  /// Duplicate INT keyword.
  pub const ERR_CBF_DUPLICATE_INT : i32 = 7111;
  /// Invalid variable type.
  pub const ERR_CBF_INVALID_VAR_TYPE : i32 = 7112;
  /// Invalid constraint type.
  pub const ERR_CBF_INVALID_CON_TYPE : i32 = 7113;
  /// Invalid domain dimension.
  pub const ERR_CBF_INVALID_DOMAIN_DIMENSION : i32 = 7114;
  /// Duplicate index in OBJCOORD.
  pub const ERR_CBF_DUPLICATE_OBJACOORD : i32 = 7115;
  /// Duplicate index in BCOORD.
  pub const ERR_CBF_DUPLICATE_BCOORD : i32 = 7116;
  /// Duplicate index in ACOORD.
  pub const ERR_CBF_DUPLICATE_ACOORD : i32 = 7117;
  /// Too few variables defined.
  pub const ERR_CBF_TOO_FEW_VARIABLES : i32 = 7118;
  /// Too few constraints defined.
  pub const ERR_CBF_TOO_FEW_CONSTRAINTS : i32 = 7119;
  /// Too ints specified.
  pub const ERR_CBF_TOO_FEW_INTS : i32 = 7120;
  /// Too ints specified.
  pub const ERR_CBF_TOO_MANY_INTS : i32 = 7121;
  /// Invalid INT index.
  pub const ERR_CBF_INVALID_INT_INDEX : i32 = 7122;
  /// Unsupported feature is present.
  pub const ERR_CBF_UNSUPPORTED : i32 = 7123;
  /// Duplicate PSDVAR keyword.
  pub const ERR_CBF_DUPLICATE_PSDVAR : i32 = 7124;
  /// Invalid PSDVAR dimension.
  pub const ERR_CBF_INVALID_PSDVAR_DIMENSION : i32 = 7125;
  /// Too few variables defined.
  pub const ERR_CBF_TOO_FEW_PSDVAR : i32 = 7126;
  /// Invalid dimension of a exponential cone.
  pub const ERR_CBF_INVALID_EXP_DIMENSION : i32 = 7127;
  /// Multiple POWCONES specified.
  pub const ERR_CBF_DUPLICATE_POW_CONES : i32 = 7130;
  /// Multiple POW*CONES specified.
  pub const ERR_CBF_DUPLICATE_POW_STAR_CONES : i32 = 7131;
  /// Invalid power specified.
  pub const ERR_CBF_INVALID_POWER : i32 = 7132;
  /// Power cone is too long.
  pub const ERR_CBF_POWER_CONE_IS_TOO_LONG : i32 = 7133;
  /// Invalid power cone index.
  pub const ERR_CBF_INVALID_POWER_CONE_INDEX : i32 = 7134;
  /// Invalid power star cone index.
  pub const ERR_CBF_INVALID_POWER_STAR_CONE_INDEX : i32 = 7135;
  /// An unhandled power cone type.
  pub const ERR_CBF_UNHANDLED_POWER_CONE_TYPE : i32 = 7136;
  /// An unhandled power star cone type.
  pub const ERR_CBF_UNHANDLED_POWER_STAR_CONE_TYPE : i32 = 7137;
  /// The power cone does not match with it definition.
  pub const ERR_CBF_POWER_CONE_MISMATCH : i32 = 7138;
  /// The power star cone does not match with it definition.
  pub const ERR_CBF_POWER_STAR_CONE_MISMATCH : i32 = 7139;
  /// Invalid number of cones.
  pub const ERR_CBF_INVALID_NUMBER_OF_CONES : i32 = 7140;
  /// Invalid number of cones.
  pub const ERR_CBF_INVALID_DIMENSION_OF_CONES : i32 = 7141;
  /// Invalid number of OBJACOORD.
  pub const ERR_CBF_INVALID_NUM_OBJACOORD : i32 = 7150;
  /// Invalid number of OBJFCOORD.
  pub const ERR_CBF_INVALID_NUM_OBJFCOORD : i32 = 7151;
  /// Invalid number of ACOORD.
  pub const ERR_CBF_INVALID_NUM_ACOORD : i32 = 7152;
  /// Invalid number of BCOORD.
  pub const ERR_CBF_INVALID_NUM_BCOORD : i32 = 7153;
  /// Invalid number of FCOORD.
  pub const ERR_CBF_INVALID_NUM_FCOORD : i32 = 7155;
  /// Invalid number of HCOORD.
  pub const ERR_CBF_INVALID_NUM_HCOORD : i32 = 7156;
  /// Invalid number of DCOORD.
  pub const ERR_CBF_INVALID_NUM_DCOORD : i32 = 7157;
  /// Invalid number of PSDCON.
  pub const ERR_CBF_INVALID_NUM_PSDCON : i32 = 7200;
  /// Duplicate CON keyword.
  pub const ERR_CBF_DUPLICATE_PSDCON : i32 = 7201;
  /// Invalid PSDCON dimension.
  pub const ERR_CBF_INVALID_DIMENSION_OF_PSDCON : i32 = 7202;
  /// Invalid PSDCON index.
  pub const ERR_CBF_INVALID_PSDCON_INDEX : i32 = 7203;
  /// Invalid PSDCON index.
  pub const ERR_CBF_INVALID_PSDCON_VARIABLE_INDEX : i32 = 7204;
  /// Invalid PSDCON index.
  pub const ERR_CBF_INVALID_PSDCON_BLOCK_INDEX : i32 = 7205;
  /// The CHANGE section is not supported.
  pub const ERR_CBF_UNSUPPORTED_CHANGE : i32 = 7210;
  /// An invalid root optimizer was selected for the problem type.
  pub const ERR_MIO_INVALID_ROOT_OPTIMIZER : i32 = 7700;
  /// An invalid node optimizer was selected for the problem type.
  pub const ERR_MIO_INVALID_NODE_OPTIMIZER : i32 = 7701;
  /// An invalid cone type occurs when writing a CPLEX formatted MPS file.
  pub const ERR_MPS_WRITE_CPLEX_INVALID_CONE_TYPE : i32 = 7750;
  /// The matrix defining the quadratric part of constraint is not positive semidefinite.
  pub const ERR_TOCONIC_CONSTR_Q_NOT_PSD : i32 = 7800;
  /// The quadratic constraint is an equality, thus not convex.
  pub const ERR_TOCONIC_CONSTRAINT_FX : i32 = 7801;
  /// The quadratic constraint has finite lower and upper bound, and therefore it is not convex.
  pub const ERR_TOCONIC_CONSTRAINT_RA : i32 = 7802;
  /// The constraint is not conic representable.
  pub const ERR_TOCONIC_CONSTR_NOT_CONIC : i32 = 7803;
  /// The matrix defining the quadratric part of the objective function is not positive semidefinite.
  pub const ERR_TOCONIC_OBJECTIVE_NOT_PSD : i32 = 7804;
  /// Failed to connect to remote solver server.
  pub const ERR_SERVER_CONNECT : i32 = 8000;
  /// Unexpected message or data from solver server.
  pub const ERR_SERVER_PROTOCOL : i32 = 8001;
  /// Server returned non-ok status code
  pub const ERR_SERVER_STATUS : i32 = 8002;
  /// Invalid job ID
  pub const ERR_SERVER_TOKEN : i32 = 8003;
  /// Invalid address
  pub const ERR_SERVER_ADDRESS : i32 = 8004;
  /// Invalid TLS certificate format or path
  pub const ERR_SERVER_CERTIFICATE : i32 = 8005;
  /// Failed to create TLS client
  pub const ERR_SERVER_TLS_CLIENT : i32 = 8006;
  /// Invalid access token
  pub const ERR_SERVER_ACCESS_TOKEN : i32 = 8007;
  /// The problem is too large.
  pub const ERR_SERVER_PROBLEM_SIZE : i32 = 8008;
  /// An element in a sparse matrix is specified twice.
  pub const ERR_DUPLICATE_INDEX_IN_A_SPARSE_MATRIX : i32 = 20050;
  /// An index is specified twice in an affine expression list.
  pub const ERR_DUPLICATE_INDEX_IN_AFEIDX_LIST : i32 = 20060;
  /// An element in the F matrix is specified twice.
  pub const ERR_DUPLICATE_FIJ : i32 = 20100;
  /// f\[i,j\] contains an invalid floating point value, i.e. a NaN or an infinite value.
  pub const ERR_INVALID_FIJ : i32 = 20101;
  /// A numerically huge value is specified for an element in F.
  pub const ERR_HUGE_FIJ : i32 = 20102;
  /// g contains an invalid floating point value, i.e. a NaN or an infinite value.
  pub const ERR_INVALID_G : i32 = 20103;
  /// b contains an invalid floating point value, i.e. a NaN or an infinite value.
  pub const ERR_INVALID_B : i32 = 20150;
  /// A domain index is invalid.
  pub const ERR_DOMAIN_INVALID_INDEX : i32 = 20400;
  /// A domain dimension is invalid.
  pub const ERR_DOMAIN_DIMENSION : i32 = 20401;
  /// A PSD domain dimension is invalid.
  pub const ERR_DOMAIN_DIMENSION_PSD : i32 = 20402;
  /// The function is only applicable to primal and dual power cone domains.
  pub const ERR_NOT_POWER_DOMAIN : i32 = 20403;
  /// Alpha contains an invalid floating point value, i.e. a NaN or an infinite value.
  pub const ERR_DOMAIN_POWER_INVALID_ALPHA : i32 = 20404;
  /// Alpha contains a negative value or zero.
  pub const ERR_DOMAIN_POWER_NEGATIVE_ALPHA : i32 = 20405;
  /// The value of nleft is too small or too large.
  pub const ERR_DOMAIN_POWER_NLEFT : i32 = 20406;
  /// An affine expression index is invalid.
  pub const ERR_AFE_INVALID_INDEX : i32 = 20500;
  /// A affine conic constraint index is invalid.
  pub const ERR_ACC_INVALID_INDEX : i32 = 20600;
  /// The index of an element in an affine conic constraint is invalid.
  pub const ERR_ACC_INVALID_ENTRY_INDEX : i32 = 20601;
  /// There is a mismatch between between the number of affine expressions and total dimension of the domain(s).
  pub const ERR_ACC_AFE_DOMAIN_MISMATCH : i32 = 20602;
  /// A disjunctive constraint index is invalid.
  pub const ERR_DJC_INVALID_INDEX : i32 = 20700;
  /// An unsupported domain type has been used in a disjunctive constraint.
  pub const ERR_DJC_UNSUPPORTED_DOMAIN_TYPE : i32 = 20701;
  /// There is a mismatch between the number of affine expressions and total dimension of the domain(s).
  pub const ERR_DJC_AFE_DOMAIN_MISMATCH : i32 = 20702;
  /// A termize is invalid.
  pub const ERR_DJC_INVALID_TERM_SIZE : i32 = 20703;
  /// There is a mismatch between the number of domains and the term sizes.
  pub const ERR_DJC_DOMAIN_TERMSIZE_MISMATCH : i32 = 20704;
  /// There total number of terms in all domains does not match.
  pub const ERR_DJC_TOTAL_NUM_TERMS_MISMATCH : i32 = 20705;
  /// The required solution is not defined.
  pub const ERR_UNDEF_SOLUTION : i32 = 22000;
  /// No doty is available.
  pub const ERR_NO_DOTY : i32 = 22010;
  /// The optimizer terminated at the maximum number of iterations.
  pub const TRM_MAX_ITERATIONS : i32 = 100000;
  /// The optimizer terminated at the maximum amount of time.
  pub const TRM_MAX_TIME : i32 = 100001;
  /// The optimizer terminated with an objective value outside the objective range.
  pub const TRM_OBJECTIVE_RANGE : i32 = 100002;
  /// The optimizer is terminated due to slow progress.
  pub const TRM_STALL : i32 = 100006;
  /// The user-defined progress callback function terminated the optimization.
  pub const TRM_USER_CALLBACK : i32 = 100007;
  /// The mixed-integer optimizer terminated as the maximum number of relaxations was reached.
  pub const TRM_MIO_NUM_RELAXS : i32 = 100008;
  /// The mixed-integer optimizer terminated as the maximum number of branches was reached.
  pub const TRM_MIO_NUM_BRANCHES : i32 = 100009;
  /// The mixed-integer optimizer terminated as the maximum number of feasible solutions was reached.
  pub const TRM_NUM_MAX_NUM_INT_SOLUTIONS : i32 = 100015;
  /// The optimizer terminated as the maximum number of set-backs was reached.
  pub const TRM_MAX_NUM_SETBACKS : i32 = 100020;
  /// The optimizer terminated due to a numerical problem.
  pub const TRM_NUMERICAL_PROBLEM : i32 = 100025;
  /// Lost a race.
  pub const TRM_LOST_RACE : i32 = 100027;
  /// The optimizer terminated due to some internal reason.
  pub const TRM_INTERNAL : i32 = 100030;
  /// The optimizer terminated for internal reasons.
  pub const TRM_INTERNAL_STOP : i32 = 100031;
} // impl Rescode

/// Response code type
#[non_exhaustive]
pub struct Rescodetype;
impl Rescodetype {
  /// The response code is OK.
  pub const OK : i32 = 0;
  /// The response code is a warning.
  pub const WRN : i32 = 1;
  /// The response code is an optimizer termination status.
  pub const TRM : i32 = 2;
  /// The response code is an error.
  pub const ERR : i32 = 3;
  /// The response code does not belong to any class.
  pub const UNK : i32 = 4;
} // impl Rescodetype

/// Scaling type
#[non_exhaustive]
pub struct Scalingtype;
impl Scalingtype {
  /// The optimizer chooses the scaling heuristic.
  pub const FREE : i32 = 0;
  /// No scaling is performed.
  pub const NONE : i32 = 1;
} // impl Scalingtype

/// Scaling method
#[non_exhaustive]
pub struct Scalingmethod;
impl Scalingmethod {
  /// Scales only with power of 2 leaving the mantissa untouched.
  pub const POW2 : i32 = 0;
  /// The optimizer chooses the scaling heuristic.
  pub const FREE : i32 = 1;
} // impl Scalingmethod

/// Sensitivity types
#[non_exhaustive]
pub struct Sensitivitytype;
impl Sensitivitytype {
  /// Basis sensitivity analysis is performed.
  pub const BASIS : i32 = 0;
} // impl Sensitivitytype

/// Simplex selection strategy
#[non_exhaustive]
pub struct Simseltype;
impl Simseltype {
  /// The optimizer chooses the pricing strategy.
  pub const FREE : i32 = 0;
  /// The optimizer uses full pricing.
  pub const FULL : i32 = 1;
  /// The optimizer uses approximate steepest-edge pricing.
  pub const ASE : i32 = 2;
  /// The optimizer uses devex steepest-edge pricing.
  pub const DEVEX : i32 = 3;
  /// The optimizer uses steepest-edge selection.
  pub const SE : i32 = 4;
  /// The optimizer uses a partial selection approach.
  pub const PARTIAL : i32 = 5;
} // impl Simseltype

/// Solution items
#[non_exhaustive]
pub struct Solitem;
impl Solitem {
  /// Solution for the constraints.
  pub const XC : i32 = 0;
  /// Variable solution.
  pub const XX : i32 = 1;
  /// Lagrange multipliers for equations.
  pub const Y : i32 = 2;
  /// Lagrange multipliers for lower bounds on the constraints.
  pub const SLC : i32 = 3;
  /// Lagrange multipliers for upper bounds on the constraints.
  pub const SUC : i32 = 4;
  /// Lagrange multipliers for lower bounds on the variables.
  pub const SLX : i32 = 5;
  /// Lagrange multipliers for upper bounds on the variables.
  pub const SUX : i32 = 6;
  /// Lagrange multipliers corresponding to the conic constraints on the variables.
  pub const SNX : i32 = 7;
} // impl Solitem

/// Solution status keys
#[non_exhaustive]
pub struct Solsta;
impl Solsta {
  /// Status of the solution is unknown.
  pub const UNKNOWN : i32 = 0;
  /// The solution is optimal.
  pub const OPTIMAL : i32 = 1;
  /// The solution is primal feasible.
  pub const PRIM_FEAS : i32 = 2;
  /// The solution is dual feasible.
  pub const DUAL_FEAS : i32 = 3;
  /// The solution is both primal and dual feasible.
  pub const PRIM_AND_DUAL_FEAS : i32 = 4;
  /// The solution is a certificate of primal infeasibility.
  pub const PRIM_INFEAS_CER : i32 = 5;
  /// The solution is a certificate of dual infeasibility.
  pub const DUAL_INFEAS_CER : i32 = 6;
  /// The solution is a certificate that the primal problem is illposed.
  pub const PRIM_ILLPOSED_CER : i32 = 7;
  /// The solution is a certificate that the dual problem is illposed.
  pub const DUAL_ILLPOSED_CER : i32 = 8;
  /// The primal solution is integer optimal.
  pub const INTEGER_OPTIMAL : i32 = 9;
} // impl Solsta

/// Solution types
#[non_exhaustive]
pub struct Soltype;
impl Soltype {
  /// The interior solution.
  pub const ITR : i32 = 0;
  /// The basic solution.
  pub const BAS : i32 = 1;
  /// The integer solution.
  pub const ITG : i32 = 2;
} // impl Soltype

/// Solve primal or dual form
#[non_exhaustive]
pub struct Solveform;
impl Solveform {
  /// The optimizer is free to solve either the primal or the dual problem.
  pub const FREE : i32 = 0;
  /// The optimizer should solve the primal problem.
  pub const PRIMAL : i32 = 1;
  /// The optimizer should solve the dual problem.
  pub const DUAL : i32 = 2;
} // impl Solveform

/// String parameters
#[non_exhaustive]
pub struct Sparam;
impl Sparam {
  /// Name of the bas solution file.
  pub const BAS_SOL_FILE_NAME : i32 = 0;
  /// Data are read and written to this file.
  pub const DATA_FILE_NAME : i32 = 1;
  /// MOSEK debug file.
  pub const DEBUG_FILE_NAME : i32 = 2;
  /// Name of the int solution file.
  pub const INT_SOL_FILE_NAME : i32 = 3;
  /// Name of the itr solution file.
  pub const ITR_SOL_FILE_NAME : i32 = 4;
  /// For internal debugging purposes.
  pub const MIO_DEBUG_STRING : i32 = 5;
  /// Solution file comment character.
  pub const PARAM_COMMENT_SIGN : i32 = 6;
  /// Modifications to the parameter database is read from this file.
  pub const PARAM_READ_FILE_NAME : i32 = 7;
  /// The parameter database is written to this file.
  pub const PARAM_WRITE_FILE_NAME : i32 = 8;
  /// Name of the BOUNDS vector used. An empty name means that the first BOUNDS vector is used.
  pub const READ_MPS_BOU_NAME : i32 = 9;
  /// Objective name in the MPS file.
  pub const READ_MPS_OBJ_NAME : i32 = 10;
  /// Name of the RANGE vector  used. An empty name means that the first RANGE vector is used.
  pub const READ_MPS_RAN_NAME : i32 = 11;
  /// Name of the RHS used. An empty name means that the first RHS vector is used.
  pub const READ_MPS_RHS_NAME : i32 = 12;
  /// URL of the remote optimization server.
  pub const REMOTE_OPTSERVER_HOST : i32 = 13;
  /// Known server certificates in PEM format
  pub const REMOTE_TLS_CERT : i32 = 14;
  /// Path to known server certificates in PEM format
  pub const REMOTE_TLS_CERT_PATH : i32 = 15;
  /// Sensitivity report file name.
  pub const SENSITIVITY_FILE_NAME : i32 = 16;
  /// Name of the sensitivity report output file.
  pub const SENSITIVITY_RES_FILE_NAME : i32 = 17;
  /// Solution file filter.
  pub const SOL_FILTER_XC_LOW : i32 = 18;
  /// Solution file filter.
  pub const SOL_FILTER_XC_UPR : i32 = 19;
  /// Solution file filter.
  pub const SOL_FILTER_XX_LOW : i32 = 20;
  /// Solution file filter.
  pub const SOL_FILTER_XX_UPR : i32 = 21;
  /// Key used when writing the summary file.
  pub const STAT_KEY : i32 = 22;
  /// Name used when writing the statistics file.
  pub const STAT_NAME : i32 = 23;
  /// Added variable names in the LP files.
  pub const WRITE_LP_GEN_VAR_NAME : i32 = 24;
} // impl Sparam

/// Status keys
#[non_exhaustive]
pub struct Stakey;
impl Stakey {
  /// The status for the constraint or variable is unknown.
  pub const UNK : i32 = 0;
  /// The constraint or variable is in the basis.
  pub const BAS : i32 = 1;
  /// The constraint or variable is super basic.
  pub const SUPBAS : i32 = 2;
  /// The constraint or variable is at its lower bound.
  pub const LOW : i32 = 3;
  /// The constraint or variable is at its upper bound.
  pub const UPR : i32 = 4;
  /// The constraint or variable is fixed.
  pub const FIX : i32 = 5;
  /// The constraint or variable is infeasible in the bounds.
  pub const INF : i32 = 6;
} // impl Stakey

/// Starting point types
#[non_exhaustive]
pub struct Startpointtype;
impl Startpointtype {
  /// The starting point is chosen automatically.
  pub const FREE : i32 = 0;
  /// The optimizer guesses a starting point.
  pub const GUESS : i32 = 1;
  /// The optimizer constructs a starting point by assigning a constant value to all primal and dual variables. This starting point is normally robust.
  pub const CONSTANT : i32 = 2;
  /// The starting point satisfies all the simple bounds on nonlinear variables.
  pub const SATISFY_BOUNDS : i32 = 3;
} // impl Startpointtype

/// Stream types
#[non_exhaustive]
pub struct Streamtype;
impl Streamtype {
  /// Log stream. Contains the aggregated contents of all other streams. This means that a message written to any other stream will also be written to this stream.
  pub const LOG : i32 = 0;
  /// Message stream. Log information relating to performance and progress of the optimization is written to this stream.
  pub const MSG : i32 = 1;
  /// Error stream. Error messages are written to this stream.
  pub const ERR : i32 = 2;
  /// Warning stream. Warning messages are written to this stream.
  pub const WRN : i32 = 3;
} // impl Streamtype

/// Integer values
#[non_exhaustive]
pub struct Value;
impl Value {
  /// The length of a license key buffer.
  pub const LICENSE_BUFFER_LENGTH : i32 = 21;
  /// Maximum string length allowed in MOSEK.
  pub const MAX_STR_LEN : i32 = 1024;
} // impl Value

/// Variable types
#[non_exhaustive]
pub struct Variabletype;
impl Variabletype {
  /// Is a continuous variable.
  pub const TYPE_CONT : i32 = 0;
  /// Is an integer variable.
  pub const TYPE_INT : i32 = 1;
} // impl Variabletype

#[allow(unused_parens)]
fn handle_res_static(r : i32, funname : &str) -> Result<(),String> {
    return
        ( if r != 0 { Err(format!("Error in call to {}: {}",funname,r)) }
          else {
              Ok(())
          })
}

/// Environment. Defines an environment in which tasks are created. In
/// most cases it is not necessary to create this directly since the
/// tasks can use the default environment.
pub struct Env {
    ptr : * const u8
}


/// The Task structure encapsulates a MOSEK Task containing problem
/// data, parameters and other information used to solve the
/// optimization problem, and provides the API for interacting with
/// the task data.
///
/// The Task object does not support callbacks, but can be passed or
/// shared between threads. If callbacks are needed, use the
/// `with_callbacks` metho to convert it into a `TaskCB` object.
pub struct Task {
    ptr : * const u8,
}
unsafe impl Send for Task {}


/// The `TaskCB` object has all the same API functions as the `Task`
/// object, plus functions for setting and clearing callbacks. The
/// `TaskCB` is not safe to pass or share between threads.
///
/// A `TaskCB` can be converted back into a `Task` by the member
/// function `without_callbacks()`.
pub struct TaskCB {
    task : Task,
    streamcb  : [ Option<Box<Box<dyn Fn(&str)>>>; 4 ],
    valuecb   : Option<Box<Box<dyn FnMut(i32,&[f64],&[i32],&[i64]) -> bool>>>,
}


impl Env {

    /// Create a new environment
    pub fn new() -> Option<Env> {
        let mut env : * const u8 = std::ptr::null();
        let res = unsafe { MSK_makeenv(& mut env, std::ptr::null()) };
        if res != 0 { return None; }

        return Some(Env { ptr : env });
    }

    /// Create a new environment, specifying an output file used for writing memory debugging information.
    pub fn new_mem_debug(dbgfile : &str) -> Option<Env> {
        let mut env : * const u8 = std::ptr::null();
        let dbgfile_cstr = CString::new(dbgfile).unwrap();
        let res = unsafe { MSK_makeenv(& mut env, dbgfile_cstr.as_ptr()) };
        if res != 0 { return None; }

        return Some(Env { ptr : env });
    }

    /// Create a new task in this environment
    pub fn task(&self) -> Option<Task> { Task::from_env(Some(&self)) }

    /// Create a new task in this environment with pre-defined capacities.
    pub fn task_with_capacity(&self, numcon : i32, numvar : i32) -> Option<Task> {
        return Task::with_capacity(Some(&self),numcon,numvar)
    }

    fn handle_res(&self, r : i32, funname : &str) -> Result<(),String> {
        return handle_res_static(r,funname);
    }

    /// Computes vector addition and multiplication by a scalar.
    ///
    /// # Arguments
    ///
    /// - `n_` Length of the vectors.
    /// - `alpha_` The scalar that multiplies x.
    /// - `x_` The x vector.
    /// - `y_` The y vector.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.axpy>
    #[allow(unused_parens)]
    pub fn axpy(&self,n_ : i32,alpha_ : f64,x_ : &[f64],y_ : &mut[f64]) -> Result<(),String> {
      if x_.len() != (n_).try_into().unwrap() {
        return Result::Err("axpy: Argument 'x' has the wrong length, expected n_".to_string());
      }
      if y_.len() != (n_).try_into().unwrap() {
        return Result::Err("axpy: Argument 'y' has the wrong length, expected n_".to_string());
      }
      handle_res_static(unsafe { MSK_axpy(self.ptr,n_,alpha_,x_.as_ptr(),y_.as_mut_ptr()) },"axpy")?;
      return Result::Ok(());
    } // axpy
    /// Check in all unused license features to the license token server.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.checkinall>
    #[allow(unused_parens)]
    pub fn check_in_all(&mut self) -> Result<(),String> {
      self.handle_res(unsafe { MSK_checkinall(self.ptr) },"check_in_all")?;
      return Result::Ok(());
    } // checkinall
    /// Check in a license feature back to the license server ahead of time.
    ///
    /// # Arguments
    ///
    /// - `feature_` Feature to check in to the license system.
    ///   
    ///   See [Feature]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.checkinlicense>
    #[allow(unused_parens)]
    pub fn check_in_license(&mut self,feature_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_checkinlicense(self.ptr,feature_) },"check_in_license")?;
      return Result::Ok(());
    } // checkinlicense
    /// Check out a license feature from the license server ahead of time.
    ///
    /// # Arguments
    ///
    /// - `feature_` Feature to check out from the license system.
    ///   
    ///   See [Feature]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.checkoutlicense>
    #[allow(unused_parens)]
    pub fn check_out_license(&mut self,feature_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_checkoutlicense(self.ptr,feature_) },"check_out_license")?;
      return Result::Ok(());
    } // checkoutlicense
    /// Compares a version of the MOSEK DLL with a specified version.
    ///
    /// # Arguments
    ///
    /// - `major_` Major version number.
    /// - `minor_` Minor version number.
    /// - `revision_` Revision number.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.checkversion>
    #[allow(unused_parens)]
    pub fn check_version(&self,major_ : i32,minor_ : i32,revision_ : i32) -> Result<(),String> {
      handle_res_static(unsafe { MSK_checkversion(self.ptr,major_,minor_,revision_) },"check_version")?;
      return Result::Ok(());
    } // checkversion
    /// Computes a Cholesky factorization of sparse matrix.
    ///
    /// # Arguments
    ///
    /// - `numthreads_` The number threads that can be used to do the computation. 0 means the code makes the choice.
    /// - `ordermethod_` If nonzero, then a sparsity preserving ordering will be employed.
    /// - `tolsingular_` A positive parameter controlling when a pivot is declared zero.
    /// - `anzc_` anzc\[j\] is the number of nonzeros in the jth column of A.
    /// - `aptrc_` aptrc\[j\] is a pointer to the first element in column j.
    /// - `asubc_` Row indexes for each column stored in increasing order.
    /// - `avalc_` The value corresponding to row indexed stored in asubc.
    /// - `perm_` Permutation array used to specify the permutation matrix P computed by the function.
    /// - `diag_` The diagonal elements of matrix D.
    /// - `lnzc_` lnzc\[j\] is the number of non zero elements in column j.
    /// - `lptrc_` lptrc\[j\] is a pointer to the first row index and value in column j.
    /// - `lensubnval_` Number of elements in lsubc and lvalc.
    /// - `lsubc_` Row indexes for each column stored in increasing order.
    /// - `lvalc_` The values corresponding to row indexed stored in lsubc.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.computesparsecholesky>
    #[allow(unused_parens)]
    pub fn compute_sparse_cholesky(&self,numthreads_ : i32,ordermethod_ : i32,tolsingular_ : f64,anzc_ : &[i32],aptrc_ : &[i64],asubc_ : &[i32],avalc_ : &[f64],perm_ : &mut Vec<i32>,diag_ : &mut Vec<f64>,lnzc_ : &mut Vec<i32>,lptrc_ : &mut Vec<i64>,lensubnval_ : &mut i64,lsubc_ : &mut Vec<i32>,lvalc_ : &mut Vec<f64>) -> Result<(),String> {
      let n_ : i32 = std::cmp::min(aptrc_.len(),anzc_.len()) as i32;
      let mut __tmp_0 : * const i32 = std::ptr::null();
      let mut __tmp_1 : * const f64 = std::ptr::null();
      let mut __tmp_2 : * const i32 = std::ptr::null();
      let mut __tmp_3 : * const i64 = std::ptr::null();
      let mut __tmp_5 : * const i32 = std::ptr::null();
      let mut __tmp_6 : * const f64 = std::ptr::null();
      handle_res_static(unsafe { MSK_computesparsecholesky(self.ptr,numthreads_,ordermethod_,tolsingular_,n_,anzc_.as_ptr(),aptrc_.as_ptr(),asubc_.as_ptr(),avalc_.as_ptr(),& mut __tmp_0,& mut __tmp_1,& mut __tmp_2,& mut __tmp_3,lensubnval_,& mut __tmp_5,& mut __tmp_6) },"compute_sparse_cholesky")?;
      let len_perm_ : usize = n_.try_into().unwrap_or(0);
      perm_.resize(len_perm_,Default::default());
      perm_.clone_from_slice(unsafe { std::slice::from_raw_parts(__tmp_0,len_perm_) } );
      let _ = unsafe { MSK_freeenv(self.ptr,__tmp_0 as * mut u8); };
      let len_diag_ : usize = n_.try_into().unwrap_or(0);
      diag_.resize(len_diag_,Default::default());
      diag_.clone_from_slice(unsafe { std::slice::from_raw_parts(__tmp_1,len_diag_) } );
      let _ = unsafe { MSK_freeenv(self.ptr,__tmp_1 as * mut u8); };
      let len_lnzc_ : usize = n_.try_into().unwrap_or(0);
      lnzc_.resize(len_lnzc_,Default::default());
      lnzc_.clone_from_slice(unsafe { std::slice::from_raw_parts(__tmp_2,len_lnzc_) } );
      let _ = unsafe { MSK_freeenv(self.ptr,__tmp_2 as * mut u8); };
      let len_lptrc_ : usize = n_.try_into().unwrap_or(0);
      lptrc_.resize(len_lptrc_,Default::default());
      lptrc_.clone_from_slice(unsafe { std::slice::from_raw_parts(__tmp_3,len_lptrc_) } );
      let _ = unsafe { MSK_freeenv(self.ptr,__tmp_3 as * mut u8); };
      let len_lsubc_ : usize = (*lensubnval_).try_into().unwrap_or(0);
      lsubc_.resize(len_lsubc_,Default::default());
      lsubc_.clone_from_slice(unsafe { std::slice::from_raw_parts(__tmp_5,len_lsubc_) } );
      let _ = unsafe { MSK_freeenv(self.ptr,__tmp_5 as * mut u8); };
      let len_lvalc_ : usize = (*lensubnval_).try_into().unwrap_or(0);
      lvalc_.resize(len_lvalc_,Default::default());
      lvalc_.clone_from_slice(unsafe { std::slice::from_raw_parts(__tmp_6,len_lvalc_) } );
      let _ = unsafe { MSK_freeenv(self.ptr,__tmp_6 as * mut u8); };
      return Result::Ok(());
    } // computesparsecholesky
    /// Computes the inner product of two vectors.
    ///
    /// # Arguments
    ///
    /// - `n_` Length of the vectors.
    /// - `x_` The x vector.
    /// - `y_` The y vector.
    /// - `xty_` The result of the inner product.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.dot>
    #[allow(unused_parens)]
    pub fn dot(&self,n_ : i32,x_ : &[f64],y_ : &[f64],xty_ : &mut f64) -> Result<(),String> {
      if x_.len() != (n_).try_into().unwrap() {
        return Result::Err("dot: Argument 'x' has the wrong length, expected n_".to_string());
      }
      if y_.len() != (n_).try_into().unwrap() {
        return Result::Err("dot: Argument 'y' has the wrong length, expected n_".to_string());
      }
      handle_res_static(unsafe { MSK_dot(self.ptr,n_,x_.as_ptr(),y_.as_ptr(),xty_) },"dot")?;
      return Result::Ok(());
    } // dot
    /// Prints an intro to message stream.
    ///
    /// # Arguments
    ///
    /// - `longver_` If non-zero, then the intro is slightly longer.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.echointro>
    #[allow(unused_parens)]
    pub fn echo_intro(&self,longver_ : i32) -> Result<(),String> {
      handle_res_static(unsafe { MSK_echointro(self.ptr,longver_) },"echo_intro")?;
      return Result::Ok(());
    } // echointro
    /// Enable reference counting for environments.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.enablegarcolenv>
    #[allow(unused_parens)]
    pub fn enable_gar_col_env(&mut self) -> Result<(),String> {
      self.handle_res(unsafe { MSK_enablegarcolenv(self.ptr) },"enable_gar_col_env")?;
      return Result::Ok(());
    } // enablegarcolenv
    /// Reports when the first license feature expires.
    ///
    /// # Arguments
    ///
    /// - `expiry_` If nonnegative, then it is the minimum number days to expiry of any feature that has been checked out.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.expirylicenses>
    #[allow(unused_parens)]
    pub fn expirylicenses(&mut self,expiry_ : &mut i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_expirylicenses(self.ptr,expiry_) },"expirylicenses")?;
      return Result::Ok(());
    } // expirylicenses
    /// Performs a dense matrix multiplication.
    ///
    /// # Arguments
    ///
    /// - `transa_` Indicates whether the matrix A must be transposed.
    ///   
    ///   See [Transpose]
    /// - `transb_` Indicates whether the matrix B must be transposed.
    ///   
    ///   See [Transpose]
    /// - `m_` Indicates the number of rows of matrix C.
    /// - `n_` Indicates the number of columns of matrix C.
    /// - `k_` Specifies the common dimension along which op(A) and op(B) are multiplied.
    /// - `alpha_` A scalar value multiplying the result of the matrix multiplication.
    /// - `a_` The pointer to the array storing matrix A in a column-major format.
    /// - `b_` The pointer to the array storing matrix B in a column-major format.
    /// - `beta_` A scalar value that multiplies C.
    /// - `c_` The pointer to the array storing matrix C in a column-major format.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.gemm>
    #[allow(unused_parens)]
    pub fn gemm(&self,transa_ : i32,transb_ : i32,m_ : i32,n_ : i32,k_ : i32,alpha_ : f64,a_ : &[f64],b_ : &[f64],beta_ : f64,c_ : &mut[f64]) -> Result<(),String> {
      if a_.len() != ((m_*k_)).try_into().unwrap() {
        return Result::Err("gemm: Argument 'a' has the wrong length, expected (m_*k_)".to_string());
      }
      if b_.len() != ((k_*n_)).try_into().unwrap() {
        return Result::Err("gemm: Argument 'b' has the wrong length, expected (k_*n_)".to_string());
      }
      if c_.len() != ((m_*n_)).try_into().unwrap() {
        return Result::Err("gemm: Argument 'c' has the wrong length, expected (m_*n_)".to_string());
      }
      handle_res_static(unsafe { MSK_gemm(self.ptr,transa_,transb_,m_,n_,k_,alpha_,a_.as_ptr(),b_.as_ptr(),beta_,c_.as_mut_ptr()) },"gemm")?;
      return Result::Ok(());
    } // gemm
    /// Computes dense matrix times a dense vector product.
    ///
    /// # Arguments
    ///
    /// - `transa_` Indicates whether the matrix A must be transposed.
    ///   
    ///   See [Transpose]
    /// - `m_` Specifies the number of rows of the matrix A.
    /// - `n_` Specifies the number of columns of the matrix A.
    /// - `alpha_` A scalar value multiplying the matrix A.
    /// - `a_` A pointer to the array storing matrix A in a column-major format.
    /// - `x_` A pointer to the array storing the vector x.
    /// - `beta_` A scalar value multiplying the vector y.
    /// - `y_` A pointer to the array storing the vector y.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.gemv>
    #[allow(unused_parens)]
    pub fn gemv(&self,transa_ : i32,m_ : i32,n_ : i32,alpha_ : f64,a_ : &[f64],x_ : &[f64],beta_ : f64,y_ : &mut[f64]) -> Result<(),String> {
      if a_.len() != ((n_*m_)).try_into().unwrap() {
        return Result::Err("gemv: Argument 'a' has the wrong length, expected (n_*m_)".to_string());
      }
      let __tmp_0 = if ((transa_==Transpose::NO)) {
        n_
      }
      else {
        m_
      };
      if x_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("gemv: Argument 'x' has the wrong length, expected __tmp_0".to_string());
      }
      let __tmp_1 = if ((transa_==Transpose::NO)) {
        m_
      }
      else {
        n_
      };
      if y_.len() != (__tmp_1).try_into().unwrap() {
        return Result::Err("gemv: Argument 'y' has the wrong length, expected __tmp_1".to_string());
      }
      handle_res_static(unsafe { MSK_gemv(self.ptr,transa_,m_,n_,alpha_,a_.as_ptr(),x_.as_ptr(),beta_,y_.as_mut_ptr()) },"gemv")?;
      return Result::Ok(());
    } // gemv
    /// Directs all output from a stream to a file.
    ///
    /// # Arguments
    ///
    /// - `whichstream_` Index of the stream.
    ///   
    ///   See [Streamtype]
    /// - `filename_` A valid file name.
    /// - `append_` If this argument is 0 the file will be overwritten, otherwise it will be appended to.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.linkfiletoenvstream>
    #[allow(unused_parens)]
    pub fn linkfiletostream(&mut self,whichstream_ : i32,filename_ : &str,append_ : i32) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_linkfiletoenvstream(self.ptr,whichstream_,__tmp_1.as_ptr(),append_) },"linkfiletostream")?;
      return Result::Ok(());
    } // linkfiletoenvstream
    /// Optimize a number of tasks in parallel using a specified number of threads.
    ///
    /// # Arguments
    ///
    /// - `israce_` If nonzero, then the function is terminated after the first task has been completed.
    /// - `maxtime_` Time limit for the function.
    /// - `numthreads_` Number of threads to be employed.
    /// - `trmcode_` The termination code for each task.
    ///   
    ///   See [Rescode]
    /// - `rcode_` The response code for each task.
    ///   
    ///   See [Rescode]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.optimizebatch>
    #[allow(unused_parens)]
    pub fn optimize_batch(&self,israce_ : bool,maxtime_ : f64,numthreads_ : i32,task_ : &[ & mut Task ],trmcode_ : &mut[i32],rcode_ : &mut[i32]) -> Result<(),String> {
      let numtask_ : i64 = task_.len() as i64;
      let arrptrs_task : Vec<* const u8> = task_.iter().map(|t| t.ptr).collect();
      if trmcode_.len() != (numtask_).try_into().unwrap() {
        return Result::Err("optimize_batch: Argument 'trmcode' has the wrong length, expected numtask_".to_string());
      }
      if rcode_.len() != (numtask_).try_into().unwrap() {
        return Result::Err("optimize_batch: Argument 'rcode' has the wrong length, expected numtask_".to_string());
      }
      handle_res_static(unsafe { MSK_optimizebatch(self.ptr,israce_,maxtime_,numthreads_,numtask_,arrptrs_task.as_ptr(),trmcode_.as_mut_ptr(),rcode_.as_mut_ptr()) },"optimize_batch")?;
      return Result::Ok(());
    } // optimizebatch
    /// Computes a Cholesky factorization of a dense matrix.
    ///
    /// # Arguments
    ///
    /// - `uplo_` Indicates whether the upper or lower triangular part of the matrix is stored.
    ///   
    ///   See [Uplo]
    /// - `n_` Dimension of the symmetric matrix.
    /// - `a_` A symmetric matrix stored in column-major order.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.potrf>
    #[allow(unused_parens)]
    pub fn potrf(&self,uplo_ : i32,n_ : i32,a_ : &mut[f64]) -> Result<(),String> {
      if a_.len() != ((n_*n_)).try_into().unwrap() {
        return Result::Err("potrf: Argument 'a' has the wrong length, expected (n_*n_)".to_string());
      }
      handle_res_static(unsafe { MSK_potrf(self.ptr,uplo_,n_,a_.as_mut_ptr()) },"potrf")?;
      return Result::Ok(());
    } // potrf
    /// Input a runtime license code.
    ///
    /// # Arguments
    ///
    /// - `code_` A license key string.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putlicensecode>
    #[allow(unused_parens)]
    pub fn put_license_code(&mut self,code_ : &[i32]) -> Result<(),String> {
      if code_.len() != (Value::LICENSE_BUFFER_LENGTH).try_into().unwrap() {
        return Result::Err("put_license_code: Argument 'code' has the wrong length, expected Value::LICENSE_BUFFER_LENGTH".to_string());
      }
      self.handle_res(unsafe { MSK_putlicensecode(self.ptr,code_.as_ptr()) },"put_license_code")?;
      return Result::Ok(());
    } // putlicensecode
    /// Enables debug information for the license system.
    ///
    /// # Arguments
    ///
    /// - `licdebug_` Enable output of license check-out debug information.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putlicensedebug>
    #[allow(unused_parens)]
    pub fn put_license_debug(&mut self,licdebug_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putlicensedebug(self.ptr,licdebug_) },"put_license_debug")?;
      return Result::Ok(());
    } // putlicensedebug
    /// Set the path to the license file.
    ///
    /// # Arguments
    ///
    /// - `licensepath_` A path specifying where to search for the license.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putlicensepath>
    #[allow(unused_parens)]
    pub fn put_license_path(&mut self,licensepath_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(licensepath_).unwrap();
      self.handle_res(unsafe { MSK_putlicensepath(self.ptr,__tmp_1.as_ptr()) },"put_license_path")?;
      return Result::Ok(());
    } // putlicensepath
    /// Control whether mosek should wait for an available license if no license is available.
    ///
    /// # Arguments
    ///
    /// - `licwait_` Enable waiting for a license.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putlicensewait>
    #[allow(unused_parens)]
    pub fn put_license_wait(&mut self,licwait_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putlicensewait(self.ptr,licwait_) },"put_license_wait")?;
      return Result::Ok(());
    } // putlicensewait
    /// Reset the license expiry reporting startpoint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.resetexpirylicenses>
    #[allow(unused_parens)]
    pub fn reset_expiry_licenses(&mut self) -> Result<(),String> {
      self.handle_res(unsafe { MSK_resetexpirylicenses(self.ptr) },"reset_expiry_licenses")?;
      return Result::Ok(());
    } // resetexpirylicenses
    /// Solves a sparse triangular system of linear equations.
    ///
    /// # Arguments
    ///
    /// - `transposed_` Controls whether the solve is with L or the transposed L.
    ///   
    ///   See [Transpose]
    /// - `lnzc_` lnzc\[j\] is the number of nonzeros in column j.
    /// - `lptrc_` lptrc\[j\] is a pointer to the first row index and value in column j.
    /// - `lsubc_` Row indexes for each column stored sequentially.
    /// - `lvalc_` The value corresponding to row indexed stored lsubc.
    /// - `b_` The right-hand side of linear equation system to be solved as a dense vector.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.sparsetriangularsolvedense>
    #[allow(unused_parens)]
    pub fn sparse_triangular_solve_dense(&self,transposed_ : i32,lnzc_ : &[i32],lptrc_ : &[i64],lsubc_ : &[i32],lvalc_ : &[f64],b_ : &mut[f64]) -> Result<(),String> {
      let n_ : i32 = std::cmp::min(std::cmp::min(lptrc_.len(),lnzc_.len()),b_.len()) as i32;
      if lnzc_.len() != (n_).try_into().unwrap() {
        return Result::Err("sparse_triangular_solve_dense: Argument 'lnzc' has the wrong length, expected n_".to_string());
      }
      if lptrc_.len() != (n_).try_into().unwrap() {
        return Result::Err("sparse_triangular_solve_dense: Argument 'lptrc' has the wrong length, expected n_".to_string());
      }
      let lensubnval_ : i64 = std::cmp::min(lvalc_.len(),lsubc_.len()) as i64;
      if lsubc_.len() != (lensubnval_).try_into().unwrap() {
        return Result::Err("sparse_triangular_solve_dense: Argument 'lsubc' has the wrong length, expected lensubnval_".to_string());
      }
      if lvalc_.len() != (lensubnval_).try_into().unwrap() {
        return Result::Err("sparse_triangular_solve_dense: Argument 'lvalc' has the wrong length, expected lensubnval_".to_string());
      }
      if b_.len() != (n_).try_into().unwrap() {
        return Result::Err("sparse_triangular_solve_dense: Argument 'b' has the wrong length, expected n_".to_string());
      }
      handle_res_static(unsafe { MSK_sparsetriangularsolvedense(self.ptr,transposed_,n_,lnzc_.as_ptr(),lptrc_.as_ptr(),lensubnval_,lsubc_.as_ptr(),lvalc_.as_ptr(),b_.as_mut_ptr()) },"sparse_triangular_solve_dense")?;
      return Result::Ok(());
    } // sparsetriangularsolvedense
    /// Computes all eigenvalues of a symmetric dense matrix.
    ///
    /// # Arguments
    ///
    /// - `uplo_` Indicates whether the upper or lower triangular part is used.
    ///   
    ///   See [Uplo]
    /// - `n_` Dimension of the symmetric input matrix.
    /// - `a_` Input matrix A.
    /// - `w_` Array of length at least n containing the eigenvalues of A.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.syeig>
    #[allow(unused_parens)]
    pub fn syeig(&self,uplo_ : i32,n_ : i32,a_ : &[f64],w_ : &mut[f64]) -> Result<(),String> {
      if a_.len() != ((n_*n_)).try_into().unwrap() {
        return Result::Err("syeig: Argument 'a' has the wrong length, expected (n_*n_)".to_string());
      }
      if w_.len() != (n_).try_into().unwrap() {
        return Result::Err("syeig: Argument 'w' has the wrong length, expected n_".to_string());
      }
      handle_res_static(unsafe { MSK_syeig(self.ptr,uplo_,n_,a_.as_ptr(),w_.as_mut_ptr()) },"syeig")?;
      return Result::Ok(());
    } // syeig
    /// Computes all the eigenvalues and eigenvectors of a symmetric dense matrix, and thus its eigenvalue decomposition.
    ///
    /// # Arguments
    ///
    /// - `uplo_` Indicates whether the upper or lower triangular part is used.
    ///   
    ///   See [Uplo]
    /// - `n_` Dimension of the symmetric input matrix.
    /// - `a_` Input matrix A.
    /// - `w_` Array of length at least n containing the eigenvalues of A.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.syevd>
    #[allow(unused_parens)]
    pub fn syevd(&self,uplo_ : i32,n_ : i32,a_ : &mut[f64],w_ : &mut[f64]) -> Result<(),String> {
      if a_.len() != ((n_*n_)).try_into().unwrap() {
        return Result::Err("syevd: Argument 'a' has the wrong length, expected (n_*n_)".to_string());
      }
      if w_.len() != (n_).try_into().unwrap() {
        return Result::Err("syevd: Argument 'w' has the wrong length, expected n_".to_string());
      }
      handle_res_static(unsafe { MSK_syevd(self.ptr,uplo_,n_,a_.as_mut_ptr(),w_.as_mut_ptr()) },"syevd")?;
      return Result::Ok(());
    } // syevd
    /// Performs a rank-k update of a symmetric matrix.
    ///
    /// # Arguments
    ///
    /// - `uplo_` Indicates whether the upper or lower triangular part of C is used.
    ///   
    ///   See [Uplo]
    /// - `trans_` Indicates whether the matrix A must be transposed.
    ///   
    ///   See [Transpose]
    /// - `n_` Specifies the order of C.
    /// - `k_` Indicates the number of rows or columns of A, and its rank.
    /// - `alpha_` A scalar value multiplying the result of the matrix multiplication.
    /// - `a_` The pointer to the array storing matrix A in a column-major format.
    /// - `beta_` A scalar value that multiplies C.
    /// - `c_` The pointer to the array storing matrix C in a column-major format.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.syrk>
    #[allow(unused_parens)]
    pub fn syrk(&self,uplo_ : i32,trans_ : i32,n_ : i32,k_ : i32,alpha_ : f64,a_ : &[f64],beta_ : f64,c_ : &mut[f64]) -> Result<(),String> {
      if a_.len() != ((n_*k_)).try_into().unwrap() {
        return Result::Err("syrk: Argument 'a' has the wrong length, expected (n_*k_)".to_string());
      }
      if c_.len() != ((n_*n_)).try_into().unwrap() {
        return Result::Err("syrk: Argument 'c' has the wrong length, expected (n_*n_)".to_string());
      }
      handle_res_static(unsafe { MSK_syrk(self.ptr,uplo_,trans_,n_,k_,alpha_,a_.as_ptr(),beta_,c_.as_mut_ptr()) },"syrk")?;
      return Result::Ok(());
    } // syrk

}

const MSK_GLOBAL_ENV : Env = Env{ ptr : std::ptr::null() };

extern fn stream_callback_proxy(handle : * const libc::c_void, msg : * const libc::c_char) {
    let h = handle as * const Box<dyn Fn(&str)>;
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
    let h = handle as * mut Box<dyn FnMut(i32,&[f64],&[i32],&[i64]) -> bool>;
    unsafe
    {
        let r = (*h)(caller,
                     & std::slice::from_raw_parts(douinf, Dinfitem::END as usize),
                     & std::slice::from_raw_parts(intinf, Iinfitem::END as usize),
                     & std::slice::from_raw_parts(lintinf, Liinfitem::END as usize));
        return if r { 0 } else { 1 }
    }
}

impl TaskCB {
    /// Create a new `TaskCB` object from a given `Task`.
    pub fn new(task : Task) -> TaskCB {
        TaskCB { task     : task,
                 streamcb : [None,None,None,None],
                 valuecb  : None,}
    }

    /// Convert a `TaskCB` object into a `Task` object.
    pub fn without_callbacks(self) -> Task {
        for (whichstream,obj) in self.streamcb.iter().enumerate() {
            if let Some(ref _f) = *obj {
                let _ = unsafe { MSK_unlinkfuncfromtaskstream(self.task.ptr, whichstream as i32) };
            }
        }
        let _ = unsafe { MSK_putcallbackfunc_ptr(self.task.ptr, std::ptr::null(), std::ptr::null()) };

        self.task
    }

    /// Clone a `TaskCB`. Since callbacks are not shaerd between
    /// cloned objects, this returns a plain `Task` object.
    pub fn clone(&self) -> Option<Task> { self.task.clone() }

    // NOTE on callback with handles:
    //   http://aatch.github.io/blog/2015/01/17/unboxed-closures-and-ffi-callbacks/
    /// Set a stream callback handler.
    ///
    /// # Arguments
    ///
    /// - `whichstream` defines which stream attach it to, use constants `MSK_STREAM_...`.
    /// - `func` is a function that receives a message to be printed.
    pub fn put_stream_callback<F>(& mut self,whichstream : i32, func : F) -> Result<(),String>
    where F : 'static+Fn(&str) {
        if whichstream >= 0 && whichstream < 4 {
            self.streamcb[whichstream as usize] = Some(Box::new(Box::new(func)));

            match self.streamcb[whichstream as usize] {
                Some(ref bf) => {
                    let hnd =  &(**bf) as * const _ as * mut libc::c_void;
                    if 0 != unsafe { MSK_linkfunctotaskstream(self.task.ptr,whichstream, hnd, stream_callback_proxy) } {
                        Err("put_stream_callback: Failed to attach stream callback".to_string())
                    }
                    else {
                        Ok(())
                    }
                }
                None => {
                    Err("put_stream_callback: Failed to attach stream callback".to_string())
                }
            }
        }
        else {
            Err("put_stream_callback: Invalid stream".to_string())
        }
    }


    /// Clear stream callback handler at a given stream.
    pub fn clear_stream_callback(&mut self,whichstream : i32) -> Result<(),String> {
        match self.streamcb[whichstream as usize] {
            Some(ref _f) => {
                if 0 != unsafe { MSK_unlinkfuncfromtaskstream(self.task.ptr, whichstream) } {
                    Err("clear_stream_callback: Failed to clear stream callback".to_string())
                }
                else {
                    self.streamcb[whichstream as usize] = None;
                    Ok(())
                }
            },
            None => Ok(())
        }
    }

    /// Sets an information callback handler in the task
    ///
    /// # Arguments:
    ///
    /// - `func` A function (caller,dinf,iinf,liinf) -> bool, that
    ///   returns false to indicate that the solver should terminate as
    ///   soon as possible, otherwise returns true.
    ///   - `caller` indicates what the solver is currently doing (see `MSK_CALLBACK_...` constants)
    ///   - `dinf` is a list of f64 information items (indexed with `MSK_DINF_...`)
    ///   - `iinf` is a list of i32 information items (indexed with `MSK_IINF_...`)
    ///   - `liinf` is a list of i64 information items (indexed with `MSK_LIINF_...`)
    pub fn put_callback<F>(& mut self,func : F) -> Result<(),String>
    where F : 'static +FnMut(i32,&[f64],&[i32],&[i64]) -> bool {
        self.valuecb = Some(Box::new(Box::new(func)));
        match self.valuecb {
            Some(ref f) => {
                let hnd =  &(**f) as * const _ as * mut libc::c_void;
                if 0 != unsafe { MSK_putcallbackfunc(self.task.ptr, callback_proxy, hnd) } {
                    Err("put_callback: Failed to attach callback".to_string())
                }
                else {
                    Ok(())
                }
            },
            None => Ok(())
        }
    }

    /// Analyze the names and issue an error for the first invalid name.
    ///
    /// # Arguments
    ///
    /// - `whichstream_` Index of the stream.
    ///   
    ///   See [Streamtype]
    /// - `nametype_` The type of names e.g. valid in MPS or LP files.
    ///   
    ///   See [Nametype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.analyzenames>
    pub fn analyze_names(&self,whichstream_ : i32,nametype_ : i32) -> Result<(),String> { self.task.analyze_names(whichstream_,nametype_) }
    /// Analyze the data of a task.
    ///
    /// # Arguments
    ///
    /// - `whichstream_` Index of the stream.
    ///   
    ///   See [Streamtype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.analyzeproblem>
    pub fn analyze_problem(&self,whichstream_ : i32) -> Result<(),String> { self.task.analyze_problem(whichstream_) }
    /// Print information related to the quality of the solution.
    ///
    /// # Arguments
    ///
    /// - `whichstream_` Index of the stream.
    ///   
    ///   See [Streamtype]
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.analyzesolution>
    pub fn analyze_solution(&self,whichstream_ : i32,whichsol_ : i32) -> Result<(),String> { self.task.analyze_solution(whichstream_,whichsol_) }
    /// Appends an affine conic constraint to the task.
    ///
    /// # Arguments
    ///
    /// - `domidx_` Domain index.
    /// - `afeidxlist_` List of affine expression indexes.
    /// - `b_` The vector of constant terms added to affine expressions. Optional, can be NULL.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendacc>
    pub fn append_acc(&mut self,domidx_ : i64,afeidxlist_ : &[i64],b_ : &[f64]) -> Result<(),String> { self.task.append_acc(domidx_,afeidxlist_,b_) }
    /// Appends a number of affine conic constraint to the task.
    ///
    /// # Arguments
    ///
    /// - `domidxs_` Domain indices.
    /// - `afeidxlist_` List of affine expression indexes.
    /// - `b_` The vector of constant terms added to affine expressions. Optional, can be NULL.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendaccs>
    pub fn append_accs(&mut self,domidxs_ : &[i64],afeidxlist_ : &[i64],b_ : &[f64]) -> Result<(),String> { self.task.append_accs(domidxs_,afeidxlist_,b_) }
    /// Appends an affine conic constraint to the task.
    ///
    /// # Arguments
    ///
    /// - `domidx_` Domain index.
    /// - `afeidxfirst_` Index of the first affine expression.
    /// - `b_` The vector of constant terms added to affine expressions. Optional, can be NULL.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendaccseq>
    pub fn append_acc_seq(&mut self,domidx_ : i64,afeidxfirst_ : i64,b_ : &[f64]) -> Result<(),String> { self.task.append_acc_seq(domidx_,afeidxfirst_,b_) }
    /// Appends a number of affine conic constraint to the task.
    ///
    /// # Arguments
    ///
    /// - `domidxs_` Domain indices.
    /// - `numafeidx_` Number of affine expressions in the affine expression list (must equal the sum of dimensions of the domains).
    /// - `afeidxfirst_` Index of the first affine expression.
    /// - `b_` The vector of constant terms added to affine expressions. Optional, can be NULL.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendaccsseq>
    pub fn append_accs_seq(&mut self,domidxs_ : &[i64],numafeidx_ : i64,afeidxfirst_ : i64,b_ : &[f64]) -> Result<(),String> { self.task.append_accs_seq(domidxs_,numafeidx_,afeidxfirst_,b_) }
    /// Appends a number of empty affine expressions to the optimization task.
    ///
    /// # Arguments
    ///
    /// - `num_` Number of empty affine expressions which should be appended.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendafes>
    pub fn append_afes(&mut self,num_ : i64) -> Result<(),String> { self.task.append_afes(num_) }
    /// Appends semidefinite variables to the problem.
    ///
    /// # Arguments
    ///
    /// - `dim_` Dimensions of symmetric matrix variables to be added.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendbarvars>
    pub fn append_barvars(&mut self,dim_ : &[i32]) -> Result<(),String> { self.task.append_barvars(dim_) }
    /// Appends a new conic constraint to the problem.
    ///
    /// # Arguments
    ///
    /// - `ct_` Specifies the type of the cone.
    ///   
    ///   See [Conetype]
    /// - `conepar_` For the power cone it denotes the exponent alpha. For other cone types it is unused and can be set to 0.
    /// - `submem_` Variable subscripts of the members in the cone.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendcone>
    pub fn append_cone(&mut self,ct_ : i32,conepar_ : f64,submem_ : &[i32]) -> Result<(),String> { self.task.append_cone(ct_,conepar_,submem_) }
    /// Appends a new conic constraint to the problem.
    ///
    /// # Arguments
    ///
    /// - `ct_` Specifies the type of the cone.
    ///   
    ///   See [Conetype]
    /// - `conepar_` For the power cone it denotes the exponent alpha. For other cone types it is unused and can be set to 0.
    /// - `nummem_` Number of member variables in the cone.
    /// - `j_` Index of the first variable in the conic constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendconeseq>
    pub fn append_cone_seq(&mut self,ct_ : i32,conepar_ : f64,nummem_ : i32,j_ : i32) -> Result<(),String> { self.task.append_cone_seq(ct_,conepar_,nummem_,j_) }
    /// Appends multiple conic constraints to the problem.
    ///
    /// # Arguments
    ///
    /// - `ct_` Specifies the type of the cone.
    ///   
    ///   See [Conetype]
    /// - `conepar_` For the power cone it denotes the exponent alpha. For other cone types it is unused and can be set to 0.
    /// - `nummem_` Numbers of member variables in the cones.
    /// - `j_` Index of the first variable in the first cone to be appended.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendconesseq>
    pub fn append_cones_seq(&mut self,ct_ : &[i32],conepar_ : &[f64],nummem_ : &[i32],j_ : i32) -> Result<(),String> { self.task.append_cones_seq(ct_,conepar_,nummem_,j_) }
    /// Appends a number of constraints to the optimization task.
    ///
    /// # Arguments
    ///
    /// - `num_` Number of constraints which should be appended.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendcons>
    pub fn append_cons(&mut self,num_ : i32) -> Result<(),String> { self.task.append_cons(num_) }
    /// Appends a number of empty disjunctive constraints to the task.
    ///
    /// # Arguments
    ///
    /// - `num_` Number of empty disjunctive constraints which should be appended.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appenddjcs>
    pub fn append_djcs(&mut self,num_ : i64) -> Result<(),String> { self.task.append_djcs(num_) }
    /// Appends the dual exponential cone domain.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appenddualexpconedomain>
    pub fn append_dual_exp_cone_domain(&mut self) -> Result<i64,String> { self.task.append_dual_exp_cone_domain() }
    /// Appends the dual geometric mean cone domain.
    ///
    /// # Arguments
    ///
    /// - `n_` Dimmension of the domain.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appenddualgeomeanconedomain>
    pub fn append_dual_geo_mean_cone_domain(&mut self,n_ : i64) -> Result<i64,String> { self.task.append_dual_geo_mean_cone_domain(n_) }
    /// Appends the dual power cone domain.
    ///
    /// # Arguments
    ///
    /// - `n_` Dimension of the domain.
    /// - `alpha_` The sequence proportional to exponents. Must be positive.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appenddualpowerconedomain>
    pub fn append_dual_power_cone_domain(&mut self,n_ : i64,alpha_ : &[f64]) -> Result<i64,String> { self.task.append_dual_power_cone_domain(n_,alpha_) }
    /// Appends the primal exponential cone domain.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendprimalexpconedomain>
    pub fn append_primal_exp_cone_domain(&mut self) -> Result<i64,String> { self.task.append_primal_exp_cone_domain() }
    /// Appends the primal geometric mean cone domain.
    ///
    /// # Arguments
    ///
    /// - `n_` Dimmension of the domain.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendprimalgeomeanconedomain>
    pub fn append_primal_geo_mean_cone_domain(&mut self,n_ : i64) -> Result<i64,String> { self.task.append_primal_geo_mean_cone_domain(n_) }
    /// Appends the primal power cone domain.
    ///
    /// # Arguments
    ///
    /// - `n_` Dimension of the domain.
    /// - `alpha_` The sequence proportional to exponents. Must be positive.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendprimalpowerconedomain>
    pub fn append_primal_power_cone_domain(&mut self,n_ : i64,alpha_ : &[f64]) -> Result<i64,String> { self.task.append_primal_power_cone_domain(n_,alpha_) }
    /// Appends the n dimensional quadratic cone domain.
    ///
    /// # Arguments
    ///
    /// - `n_` Dimmension of the domain.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendquadraticconedomain>
    pub fn append_quadratic_cone_domain(&mut self,n_ : i64) -> Result<i64,String> { self.task.append_quadratic_cone_domain(n_) }
    /// Appends the n dimensional real number domain.
    ///
    /// # Arguments
    ///
    /// - `n_` Dimmension of the domain.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendrdomain>
    pub fn append_r_domain(&mut self,n_ : i64) -> Result<i64,String> { self.task.append_r_domain(n_) }
    /// Appends the n dimensional negative orthant to the list of domains.
    ///
    /// # Arguments
    ///
    /// - `n_` Dimmension of the domain.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendrminusdomain>
    pub fn append_rminus_domain(&mut self,n_ : i64) -> Result<i64,String> { self.task.append_rminus_domain(n_) }
    /// Appends the n dimensional positive orthant to the list of domains.
    ///
    /// # Arguments
    ///
    /// - `n_` Dimmension of the domain.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendrplusdomain>
    pub fn append_rplus_domain(&mut self,n_ : i64) -> Result<i64,String> { self.task.append_rplus_domain(n_) }
    /// Appends the n dimensional rotated quadratic cone domain.
    ///
    /// # Arguments
    ///
    /// - `n_` Dimmension of the domain.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendrquadraticconedomain>
    pub fn append_r_quadratic_cone_domain(&mut self,n_ : i64) -> Result<i64,String> { self.task.append_r_quadratic_cone_domain(n_) }
    /// Appends the n dimensional 0 domain.
    ///
    /// # Arguments
    ///
    /// - `n_` Dimmension of the domain.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendrzerodomain>
    pub fn append_rzero_domain(&mut self,n_ : i64) -> Result<i64,String> { self.task.append_rzero_domain(n_) }
    /// Appends a general sparse symmetric matrix to the storage of symmetric matrices.
    ///
    /// # Arguments
    ///
    /// - `dim_` Dimension of the symmetric matrix that is appended.
    /// - `subi_` Row subscript in the triplets.
    /// - `subj_` Column subscripts in the triplets.
    /// - `valij_` Values of each triplet.
    ///
    /// # Returns
    ///
    ///   - `idx` Unique index assigned to the inputted matrix.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendsparsesymmat>
    pub fn append_sparse_sym_mat(&mut self,dim_ : i32,subi_ : &[i32],subj_ : &[i32],valij_ : &[f64]) -> Result<i64,String> { self.task.append_sparse_sym_mat(dim_,subi_,subj_,valij_) }
    /// Appends a general sparse symmetric matrix to the storage of symmetric matrices.
    ///
    /// # Arguments
    ///
    /// - `dims_` Dimensions of the symmetric matrixes.
    /// - `nz_` Number of nonzeros for each matrix.
    /// - `subi_` Row subscript in the triplets.
    /// - `subj_` Column subscripts in the triplets.
    /// - `valij_` Values of each triplet.
    /// - `idx_` Unique index assigned to the inputted matrix.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendsparsesymmatlist>
    pub fn append_sparse_sym_mat_list(&mut self,dims_ : &[i32],nz_ : &[i64],subi_ : &[i32],subj_ : &[i32],valij_ : &[f64],idx_ : &mut[i64]) -> Result<(),String> { self.task.append_sparse_sym_mat_list(dims_,nz_,subi_,subj_,valij_,idx_) }
    /// Appends the vectorized SVEC PSD cone domain.
    ///
    /// # Arguments
    ///
    /// - `n_` Dimension of the domain.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendsvecpsdconedomain>
    pub fn append_svec_psd_cone_domain(&mut self,n_ : i64) -> Result<i64,String> { self.task.append_svec_psd_cone_domain(n_) }
    /// Appends a number of variables to the optimization task.
    ///
    /// # Arguments
    ///
    /// - `num_` Number of variables which should be appended.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendvars>
    pub fn append_vars(&mut self,num_ : i32) -> Result<(),String> { self.task.append_vars(num_) }
    /// Get the optimizer log from a remote job.
    ///
    /// # Arguments
    ///
    /// - `addr_` Address of the solver server
    /// - `accesstoken_` Access token string or NULL
    /// - `token_` Job token
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.asyncgetlog>
    pub fn async_get_log(&mut self,addr_ : &str,accesstoken_ : &str,token_ : &str) -> Result<(),String> { self.task.async_get_log(addr_,accesstoken_,token_) }
    /// Request a solution from a remote job.
    ///
    /// # Arguments
    ///
    /// - `address_` Address of the OptServer.
    /// - `accesstoken_` Access token.
    /// - `token_` The task token.
    /// - `resp_` Is the response code from the remote solver.
    ///   
    ///   See [Rescode]
    /// - `trm_` Is either OK or a termination response code.
    ///   
    ///   See [Rescode]
    ///
    /// # Returns
    ///
    ///   - `respavailable` Indicates if a remote response is available.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.asyncgetresult>
    pub fn async_get_result(&mut self,address_ : &str,accesstoken_ : &str,token_ : &str,resp_ : & mut i32,trm_ : & mut i32) -> Result<bool,String> { self.task.async_get_result(address_,accesstoken_,token_,resp_,trm_) }
    /// Offload the optimization task to a solver server in asynchronous mode.
    ///
    /// # Arguments
    ///
    /// - `address_` Address of the OptServer.
    /// - `accesstoken_` Access token.
    ///
    /// # Returns
    ///
    ///   - `token` Returns the task token.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.asyncoptimize>
    pub fn async_optimize(&mut self,address_ : &str,accesstoken_ : &str) -> Result<String,String> { self.task.async_optimize(address_,accesstoken_) }
    /// Requests information about the status of the remote job.
    ///
    /// # Arguments
    ///
    /// - `address_` Address of the OptServer.
    /// - `accesstoken_` Access token.
    /// - `token_` The task token.
    /// - `resp_` Is the response code from the remote solver.
    ///   
    ///   See [Rescode]
    /// - `trm_` Is either OK or a termination response code.
    ///   
    ///   See [Rescode]
    ///
    /// # Returns
    ///
    ///   - `respavailable` Indicates if a remote response is available.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.asyncpoll>
    pub fn async_poll(&mut self,address_ : &str,accesstoken_ : &str,token_ : &str,resp_ : & mut i32,trm_ : & mut i32) -> Result<bool,String> { self.task.async_poll(address_,accesstoken_,token_,resp_,trm_) }
    /// Request that the job identified by the token is terminated.
    ///
    /// # Arguments
    ///
    /// - `address_` Address of the OptServer.
    /// - `accesstoken_` Access token.
    /// - `token_` The task token.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.asyncstop>
    pub fn async_stop(&mut self,address_ : &str,accesstoken_ : &str,token_ : &str) -> Result<(),String> { self.task.async_stop(address_,accesstoken_,token_) }
    /// Computes conditioning information for the basis matrix.
    ///
    /// # Arguments
    ///
    /// - `nrmbasis_` An estimate for the 1-norm of the basis.
    /// - `nrminvbasis_` An estimate for the 1-norm of the inverse of the basis.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.basiscond>
    pub fn basis_cond(&mut self,nrmbasis_ : &mut f64,nrminvbasis_ : &mut f64) -> Result<(),String> { self.task.basis_cond(nrmbasis_,nrminvbasis_) }
    /// Changes the bounds for one constraint.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the constraint for which the bounds should be changed.
    /// - `lower_` If non-zero, then the lower bound is changed, otherwise the upper bound is changed.
    /// - `finite_` If non-zero, then the given value is assumed to be finite.
    /// - `value_` New value for the bound.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.chgconbound>
    pub fn chg_con_bound(&mut self,i_ : i32,lower_ : i32,finite_ : i32,value_ : f64) -> Result<(),String> { self.task.chg_con_bound(i_,lower_,finite_,value_) }
    /// Changes the bounds for one variable.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the variable for which the bounds should be changed.
    /// - `lower_` If non-zero, then the lower bound is changed, otherwise the upper bound is changed.
    /// - `finite_` If non-zero, then the given value is assumed to be finite.
    /// - `value_` New value for the bound.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.chgvarbound>
    pub fn chg_var_bound(&mut self,j_ : i32,lower_ : i32,finite_ : i32,value_ : f64) -> Result<(),String> { self.task.chg_var_bound(j_,lower_,finite_,value_) }
    /// Commits all cached problem changes.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.commitchanges>
    pub fn commit_changes(&mut self) -> Result<(),String> { self.task.commit_changes() }
    /// Undefine a solution and free the memory it uses.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.deletesolution>
    pub fn delete_solution(&mut self,whichsol_ : i32) -> Result<(),String> { self.task.delete_solution(whichsol_) }
    /// Performs sensitivity analysis on objective coefficients.
    ///
    /// # Arguments
    ///
    /// - `subj_` Indexes of objective coefficients to analyze.
    /// - `leftpricej_` Left shadow prices for requested coefficients.
    /// - `rightpricej_` Right shadow prices for requested coefficients.
    /// - `leftrangej_` Left range for requested coefficients.
    /// - `rightrangej_` Right range for requested coefficients.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.dualsensitivity>
    pub fn dual_sensitivity(&self,subj_ : &[i32],leftpricej_ : &mut[f64],rightpricej_ : &mut[f64],leftrangej_ : &mut[f64],rightrangej_ : &mut[f64]) -> Result<(),String> { self.task.dual_sensitivity(subj_,leftpricej_,rightpricej_,leftrangej_,rightrangej_) }
    /// Clears a row in barF
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row index of barF.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.emptyafebarfrow>
    pub fn empty_afe_barf_row(&mut self,afeidx_ : i64) -> Result<(),String> { self.task.empty_afe_barf_row(afeidx_) }
    /// Clears rows in barF.
    ///
    /// # Arguments
    ///
    /// - `afeidxlist_` Indices of rows in barF to clear.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.emptyafebarfrowlist>
    pub fn empty_afe_barf_row_list(&mut self,afeidxlist_ : &[i64]) -> Result<(),String> { self.task.empty_afe_barf_row_list(afeidxlist_) }
    /// Clears a column in F.
    ///
    /// # Arguments
    ///
    /// - `varidx_` Variable index.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.emptyafefcol>
    pub fn empty_afe_f_col(&mut self,varidx_ : i32) -> Result<(),String> { self.task.empty_afe_f_col(varidx_) }
    /// Clears columns in F.
    ///
    /// # Arguments
    ///
    /// - `varidx_` Indices of variables in F to clear.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.emptyafefcollist>
    pub fn empty_afe_f_col_list(&mut self,varidx_ : &[i32]) -> Result<(),String> { self.task.empty_afe_f_col_list(varidx_) }
    /// Clears a row in F.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row index.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.emptyafefrow>
    pub fn empty_afe_f_row(&mut self,afeidx_ : i64) -> Result<(),String> { self.task.empty_afe_f_row(afeidx_) }
    /// Clears rows in F.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Indices of rows in F to clear.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.emptyafefrowlist>
    pub fn empty_afe_f_row_list(&mut self,afeidx_ : &[i64]) -> Result<(),String> { self.task.empty_afe_f_row_list(afeidx_) }
    /// Evaluates the activity of an affine conic constraint.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `accidx_` The index of the affine conic constraint.
    /// - `activity_` The activity of the affine conic constraint. The array should have length equal to the dimension of the constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.evaluateacc>
    pub fn evaluate_acc(&self,whichsol_ : i32,accidx_ : i64,activity_ : &mut[f64]) -> Result<(),String> { self.task.evaluate_acc(whichsol_,accidx_,activity_) }
    /// Evaluates the activities of all affine conic constraints.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `activity_` The activity of affine conic constraints. The array should have length equal to the sum of dimensions of all affine conic constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.evaluateaccs>
    pub fn evaluate_accs(&self,whichsol_ : i32,activity_ : &mut[f64]) -> Result<(),String> { self.task.evaluate_accs(whichsol_,activity_) }
    /// Generates systematic names for affine conic constraints.
    ///
    /// # Arguments
    ///
    /// - `sub_` Indexes of the affine conic constraints.
    /// - `fmt_` The variable name formatting string.
    /// - `dims_` Dimensions in the shape.
    /// - `sp_` Items that should be named.
    /// - `namedaxisidxs_` List if named index axes
    /// - `names_` All axis names.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.generateaccnames>
    pub fn generate_acc_names(&mut self,sub_ : &[i64],fmt_ : &str,dims_ : &[i32],sp_ : &[i64],namedaxisidxs_ : &[i32],names_ : &[String]) -> Result<(),String> { self.task.generate_acc_names(sub_,fmt_,dims_,sp_,namedaxisidxs_,names_) }
    /// Generates systematic names for variables.
    ///
    /// # Arguments
    ///
    /// - `subj_` Indexes of the variables.
    /// - `fmt_` The variable name formatting string.
    /// - `dims_` Dimensions in the shape.
    /// - `sp_` Items that should be named.
    /// - `namedaxisidxs_` List if named index axes
    /// - `names_` All axis names.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.generatebarvarnames>
    pub fn generate_barvar_names(&mut self,subj_ : &[i32],fmt_ : &str,dims_ : &[i32],sp_ : &[i64],namedaxisidxs_ : &[i32],names_ : &[String]) -> Result<(),String> { self.task.generate_barvar_names(subj_,fmt_,dims_,sp_,namedaxisidxs_,names_) }
    /// Generates systematic names for cone.
    ///
    /// # Arguments
    ///
    /// - `subk_` Indexes of the cone.
    /// - `fmt_` The cone name formatting string.
    /// - `dims_` Dimensions in the shape.
    /// - `sp_` Items that should be named.
    /// - `namedaxisidxs_` List if named index axes
    /// - `names_` All axis names.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.generateconenames>
    pub fn generate_cone_names(&mut self,subk_ : &[i32],fmt_ : &str,dims_ : &[i32],sp_ : &[i64],namedaxisidxs_ : &[i32],names_ : &[String]) -> Result<(),String> { self.task.generate_cone_names(subk_,fmt_,dims_,sp_,namedaxisidxs_,names_) }
    /// Generates systematic names for constraints.
    ///
    /// # Arguments
    ///
    /// - `subi_` Indexes of the constraints.
    /// - `fmt_` The constraint name formatting string.
    /// - `dims_` Dimensions in the shape.
    /// - `sp_` Items that should be named.
    /// - `namedaxisidxs_` List if named index axes
    /// - `names_` All axis names.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.generateconnames>
    pub fn generate_con_names(&mut self,subi_ : &[i32],fmt_ : &str,dims_ : &[i32],sp_ : &[i64],namedaxisidxs_ : &[i32],names_ : &[String]) -> Result<(),String> { self.task.generate_con_names(subi_,fmt_,dims_,sp_,namedaxisidxs_,names_) }
    /// Generates systematic names for affine conic constraints.
    ///
    /// # Arguments
    ///
    /// - `sub_` Indexes of the disjunctive constraints.
    /// - `fmt_` The variable name formatting string.
    /// - `dims_` Dimensions in the shape.
    /// - `sp_` Items that should be named.
    /// - `namedaxisidxs_` List if named index axes
    /// - `names_` All axis names.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.generatedjcnames>
    pub fn generate_djc_names(&mut self,sub_ : &[i64],fmt_ : &str,dims_ : &[i32],sp_ : &[i64],namedaxisidxs_ : &[i32],names_ : &[String]) -> Result<(),String> { self.task.generate_djc_names(sub_,fmt_,dims_,sp_,namedaxisidxs_,names_) }
    /// Generates systematic names for variables.
    ///
    /// # Arguments
    ///
    /// - `subj_` Indexes of the variables.
    /// - `fmt_` The variable name formatting string.
    /// - `dims_` Dimensions in the shape.
    /// - `sp_` Items that should be named.
    /// - `namedaxisidxs_` List if named index axes
    /// - `names_` All axis names.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.generatevarnames>
    pub fn generate_var_names(&mut self,subj_ : &[i32],fmt_ : &str,dims_ : &[i32],sp_ : &[i64],namedaxisidxs_ : &[i32],names_ : &[String]) -> Result<(),String> { self.task.generate_var_names(subj_,fmt_,dims_,sp_,namedaxisidxs_,names_) }
    /// Obtains the list of affine expressions appearing in the affine conic constraint.
    ///
    /// # Arguments
    ///
    /// - `accidx_` Index of the affine conic constraint.
    /// - `afeidxlist_` List of indexes of affine expressions appearing in the constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccafeidxlist>
    pub fn get_acc_afe_idx_list(&self,accidx_ : i64,afeidxlist_ : &mut[i64]) -> Result<(),String> { self.task.get_acc_afe_idx_list(accidx_,afeidxlist_) }
    /// Obtains the additional constant term vector appearing in the affine conic constraint.
    ///
    /// # Arguments
    ///
    /// - `accidx_` Index of the affine conic constraint.
    /// - `b_` The vector b appearing in the constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccb>
    pub fn get_acc_b(&self,accidx_ : i64,b_ : &mut[f64]) -> Result<(),String> { self.task.get_acc_b(accidx_,b_) }
    /// Obtains barF, implied by the ACCs, in block triplet form.
    ///
    /// # Arguments
    ///
    /// - `acc_afe_` Index of the AFE within the concatenated list of AFEs in ACCs.
    /// - `bar_var_` Symmetric matrix variable index.
    /// - `blk_row_` Block row index.
    /// - `blk_col_` Block column index.
    /// - `blk_val_` The numerical value associated with each block triplet.
    ///
    /// # Returns
    ///
    ///   - `numtrip` Number of elements in the block triplet form.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccbarfblocktriplet>
    pub fn get_acc_barf_block_triplet(&self,acc_afe_ : &mut[i64],bar_var_ : &mut[i32],blk_row_ : &mut[i32],blk_col_ : &mut[i32],blk_val_ : &mut[f64]) -> Result<i64,String> { self.task.get_acc_barf_block_triplet(acc_afe_,bar_var_,blk_row_,blk_col_,blk_val_) }
    /// Obtains an upper bound on the number of elements in the block triplet form of barf, as used within the ACCs.
    ///
    /// # Returns
    ///
    ///   - `numtrip` An upper bound on the number of elements in the block triplet form of barf, as used within the ACCs.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccbarfnumblocktriplets>
    pub fn get_acc_barf_num_block_triplets(&self) -> Result<i64,String> { self.task.get_acc_barf_num_block_triplets() }
    /// Obtains the domain appearing in the affine conic constraint.
    ///
    /// # Arguments
    ///
    /// - `accidx_` The index of the affine conic constraint.
    /// - `domidx_` The index of domain in the affine conic constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccdomain>
    pub fn get_acc_domain(&mut self,accidx_ : i64,domidx_ : &mut i64) -> Result<(),String> { self.task.get_acc_domain(accidx_,domidx_) }
    /// Obtains the doty vector for an affine conic constraint.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `accidx_` The index of the affine conic constraint.
    /// - `doty_` The dual values for this affine conic constraint. The array should have length equal to the dimension of the constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccdoty>
    pub fn get_acc_dot_y(&self,whichsol_ : i32,accidx_ : i64,doty_ : &mut[f64]) -> Result<(),String> { self.task.get_acc_dot_y(whichsol_,accidx_,doty_) }
    /// Obtains the doty vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `doty_` The dual values of affine conic constraints. The array should have length equal to the sum of dimensions of all affine conic constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccdotys>
    pub fn get_acc_dot_y_s(&self,whichsol_ : i32,doty_ : &mut[f64]) -> Result<(),String> { self.task.get_acc_dot_y_s(whichsol_,doty_) }
    /// Obtains the total number of nonzeros in the ACC implied F matrix.
    ///
    /// # Returns
    ///
    ///   - `accfnnz` Number of nonzeros in the F matrix implied by ACCs.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccfnumnz>
    pub fn get_acc_f_numnz(&mut self) -> Result<i64,String> { self.task.get_acc_f_numnz() }
    /// Obtains the F matrix (implied by the AFE ordering within the ACCs) in triplet format.
    ///
    /// # Arguments
    ///
    /// - `frow_` Row indices of nonzeros in the implied F matrix.
    /// - `fcol_` Column indices of nonzeros in the implied F matrix.
    /// - `fval_` Values of nonzero entries in the implied F matrix.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccftrip>
    pub fn get_acc_f_trip(&mut self,frow_ : &mut[i64],fcol_ : &mut[i32],fval_ : &mut[f64]) -> Result<(),String> { self.task.get_acc_f_trip(frow_,fcol_,fval_) }
    /// The g vector as used within the ACCs.
    ///
    /// # Arguments
    ///
    /// - `g_` The g vector as used within the ACCs.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccgvector>
    pub fn get_acc_g_vector(&self,g_ : &mut[f64]) -> Result<(),String> { self.task.get_acc_g_vector(g_) }
    /// Obtains the dimension of the affine conic constraint.
    ///
    /// # Arguments
    ///
    /// - `accidx_` The index of the affine conic constraint.
    ///
    /// # Returns
    ///
    ///   - `n` The dimension of the affine conic constraint (equal to the dimension of its domain).
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccn>
    pub fn get_acc_n(&mut self,accidx_ : i64) -> Result<i64,String> { self.task.get_acc_n(accidx_) }
    /// Obtains the name of an affine conic constraint.
    ///
    /// # Arguments
    ///
    /// - `accidx_` Index of an affine conic constraint.
    ///
    /// # Returns
    ///
    ///   - `name` Returns the required name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccname>
    pub fn get_acc_name(&self,accidx_ : i64) -> Result<String,String> { self.task.get_acc_name(accidx_) }
    /// Obtains the length of the name of an affine conic constraint.
    ///
    /// # Arguments
    ///
    /// - `accidx_` Index of an affine conic constraint.
    ///
    /// # Returns
    ///
    ///   - `len` Returns the length of the indicated name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccnamelen>
    pub fn get_acc_name_len(&self,accidx_ : i64) -> Result<i32,String> { self.task.get_acc_name_len(accidx_) }
    /// Obtains the total dimension of all affine conic constraints.
    ///
    /// # Returns
    ///
    ///   - `n` The total dimension of all affine conic constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccntot>
    pub fn get_acc_n_tot(&mut self) -> Result<i64,String> { self.task.get_acc_n_tot() }
    /// Obtains full data of all affine conic constraints.
    ///
    /// # Arguments
    ///
    /// - `domidxlist_` The list of domains appearing in all affine conic constraints.
    /// - `afeidxlist_` The concatenation of index lists of affine expressions appearing in all affine conic constraints.
    /// - `b_` The concatenation of vectors b appearing in all affine conic constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccs>
    pub fn get_accs(&self,domidxlist_ : &mut[i64],afeidxlist_ : &mut[i64],b_ : &mut[f64]) -> Result<(),String> { self.task.get_accs(domidxlist_,afeidxlist_,b_) }
    /// Obtains one column of the linear constraint matrix.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the column.
    /// - `nzj_` Number of non-zeros in the column obtained.
    /// - `subj_` Row indices of the non-zeros in the column obtained.
    /// - `valj_` Numerical values in the column obtained.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getacol>
    pub fn get_a_col(&self,j_ : i32,nzj_ : &mut i32,subj_ : &mut[i32],valj_ : &mut[f64]) -> Result<(),String> { self.task.get_a_col(j_,nzj_,subj_,valj_) }
    /// Obtains the number of non-zero elements in one column of the linear constraint matrix
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the column.
    ///
    /// # Returns
    ///
    ///   - `nzj` Number of non-zeros in the j'th column of (A).
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getacolnumnz>
    pub fn get_a_col_num_nz(&self,i_ : i32) -> Result<i32,String> { self.task.get_a_col_num_nz(i_) }
    /// Obtains a sequence of columns from the coefficient matrix.
    ///
    /// # Arguments
    ///
    /// - `first_` Index of the first column in the sequence.
    /// - `last_` Index of the last column in the sequence plus one.
    /// - `ptrb_` Column start pointers.
    /// - `ptre_` Column end pointers.
    /// - `sub_` Contains the row subscripts.
    /// - `val_` Contains the coefficient values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getacolslice64>
    pub fn get_a_col_slice(&self,first_ : i32,last_ : i32,ptrb_ : &mut[i64],ptre_ : &mut[i64],sub_ : &mut[i32],val_ : &mut[f64]) -> Result<(),String> { self.task.get_a_col_slice(first_,last_,ptrb_,ptre_,sub_,val_) }
    /// Obtains the number of non-zeros in a slice of columns of the coefficient matrix.
    ///
    /// # Arguments
    ///
    /// - `first_` Index of the first column in the sequence.
    /// - `last_` Index of the last column plus one in the sequence.
    ///
    /// # Returns
    ///
    ///   - `numnz` Number of non-zeros in the slice.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getacolslicenumnz64>
    pub fn get_a_col_slice_num_nz(&self,first_ : i32,last_ : i32) -> Result<i64,String> { self.task.get_a_col_slice_num_nz(first_,last_) }
    /// Obtains a sequence of columns from the coefficient matrix in triplet format.
    ///
    /// # Arguments
    ///
    /// - `first_` Index of the first column in the sequence.
    /// - `last_` Index of the last column in the sequence plus one.
    /// - `subi_` Constraint subscripts.
    /// - `subj_` Column subscripts.
    /// - `val_` Values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getacolslicetrip>
    pub fn get_a_col_slice_trip(&self,first_ : i32,last_ : i32,subi_ : &mut[i32],subj_ : &mut[i32],val_ : &mut[f64]) -> Result<(),String> { self.task.get_a_col_slice_trip(first_,last_,subi_,subj_,val_) }
    /// Obtains barF in block triplet form.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Constraint index.
    /// - `barvaridx_` Symmetric matrix variable index.
    /// - `subk_` Block row index.
    /// - `subl_` Block column index.
    /// - `valkl_` The numerical value associated with each block triplet.
    ///
    /// # Returns
    ///
    ///   - `numtrip` Number of elements in the block triplet form.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getafebarfblocktriplet>
    pub fn get_afe_barf_block_triplet(&self,afeidx_ : &mut[i64],barvaridx_ : &mut[i32],subk_ : &mut[i32],subl_ : &mut[i32],valkl_ : &mut[f64]) -> Result<i64,String> { self.task.get_afe_barf_block_triplet(afeidx_,barvaridx_,subk_,subl_,valkl_) }
    /// Obtains an upper bound on the number of elements in the block triplet form of barf.
    ///
    /// # Returns
    ///
    ///   - `numtrip` An upper bound on the number of elements in the block triplet form of barf.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getafebarfnumblocktriplets>
    pub fn get_afe_barf_num_block_triplets(&self) -> Result<i64,String> { self.task.get_afe_barf_num_block_triplets() }
    /// Obtains the number of nonzero entries in a row of barF.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row index of barF.
    ///
    /// # Returns
    ///
    ///   - `numentr` Number of nonzero entries in a row of barF.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getafebarfnumrowentries>
    pub fn get_afe_barf_num_row_entries(&mut self,afeidx_ : i64) -> Result<i32,String> { self.task.get_afe_barf_num_row_entries(afeidx_) }
    /// Obtains nonzero entries in one row of barF.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row index of barF.
    /// - `barvaridx_` Semidefinite variable indices.
    /// - `ptrterm_` Pointers to the description of entries.
    /// - `numterm_` Number of terms in each entry.
    /// - `termidx_` Indices of semidefinite matrices from E.
    /// - `termweight_` Weights appearing in the weighted sum representation.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getafebarfrow>
    pub fn get_afe_barf_row(&mut self,afeidx_ : i64,barvaridx_ : &mut[i32],ptrterm_ : &mut[i64],numterm_ : &mut[i64],termidx_ : &mut[i64],termweight_ : &mut[f64]) -> Result<(),String> { self.task.get_afe_barf_row(afeidx_,barvaridx_,ptrterm_,numterm_,termidx_,termweight_) }
    /// Obtains information about one row of barF.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row index of barF.
    /// - `numentr_` Number of nonzero entries in a row of barF.
    /// - `numterm_` Number of terms in the weighted sums representation of the row of barF.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getafebarfrowinfo>
    pub fn get_afe_barf_row_info(&mut self,afeidx_ : i64,numentr_ : &mut i32,numterm_ : &mut i64) -> Result<(),String> { self.task.get_afe_barf_row_info(afeidx_,numentr_,numterm_) }
    /// Obtains the total number of nonzeros in F.
    ///
    /// # Returns
    ///
    ///   - `numnz` Number of nonzeros in F.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getafefnumnz>
    pub fn get_afe_f_num_nz(&mut self) -> Result<i64,String> { self.task.get_afe_f_num_nz() }
    /// Obtains one row of F in sparse format.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row index.
    /// - `numnz_` Number of non-zeros in the row obtained.
    /// - `varidx_` Column indices of the non-zeros in the row obtained.
    /// - `val_` Values of the non-zeros in the row obtained.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getafefrow>
    pub fn get_afe_f_row(&mut self,afeidx_ : i64,numnz_ : &mut i32,varidx_ : &mut[i32],val_ : &mut[f64]) -> Result<(),String> { self.task.get_afe_f_row(afeidx_,numnz_,varidx_,val_) }
    /// Obtains the number of nonzeros in a row of F.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row index.
    ///
    /// # Returns
    ///
    ///   - `numnz` Number of non-zeros in the row.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getafefrownumnz>
    pub fn get_afe_f_row_num_nz(&mut self,afeidx_ : i64) -> Result<i32,String> { self.task.get_afe_f_row_num_nz(afeidx_) }
    /// Obtains the F matrix in triplet format.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row indices of nonzeros.
    /// - `varidx_` Column indices of nonzeros.
    /// - `val_` Values of nonzero entries.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getafeftrip>
    pub fn get_afe_f_trip(&mut self,afeidx_ : &mut[i64],varidx_ : &mut[i32],val_ : &mut[f64]) -> Result<(),String> { self.task.get_afe_f_trip(afeidx_,varidx_,val_) }
    /// Obtains a single coefficient in g.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Element index.
    ///
    /// # Returns
    ///
    ///   - `g` The entry in g.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getafeg>
    pub fn get_afe_g(&mut self,afeidx_ : i64) -> Result<f64,String> { self.task.get_afe_g(afeidx_) }
    /// Obtains a sequence of coefficients from the vector g.
    ///
    /// # Arguments
    ///
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `g_` The slice of g as a dense vector.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getafegslice>
    pub fn get_afe_g_slice(&self,first_ : i64,last_ : i64,g_ : &mut[f64]) -> Result<(),String> { self.task.get_afe_g_slice(first_,last_,g_) }
    /// Obtains a single coefficient in linear constraint matrix.
    ///
    /// # Arguments
    ///
    /// - `i_` Row index of the coefficient to be returned.
    /// - `j_` Column index of the coefficient to be returned.
    ///
    /// # Returns
    ///
    ///   - `aij` Returns the requested coefficient.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaij>
    pub fn get_aij(&self,i_ : i32,j_ : i32) -> Result<f64,String> { self.task.get_aij(i_,j_) }
    /// Obtains the number non-zeros in a rectangular piece of the linear constraint matrix.
    ///
    /// # Arguments
    ///
    /// - `firsti_` Index of the first row in the rectangular piece.
    /// - `lasti_` Index of the last row plus one in the rectangular piece.
    /// - `firstj_` Index of the first column in the rectangular piece.
    /// - `lastj_` Index of the last column plus one in the rectangular piece.
    ///
    /// # Returns
    ///
    ///   - `numnz` Number of non-zero elements in the rectangular piece of the linear constraint matrix.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getapiecenumnz>
    pub fn get_a_piece_num_nz(&self,firsti_ : i32,lasti_ : i32,firstj_ : i32,lastj_ : i32) -> Result<i32,String> { self.task.get_a_piece_num_nz(firsti_,lasti_,firstj_,lastj_) }
    /// Obtains one row of the linear constraint matrix.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the row.
    /// - `nzi_` Number of non-zeros in the row obtained.
    /// - `subi_` Column indices of the non-zeros in the row obtained.
    /// - `vali_` Numerical values of the row obtained.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getarow>
    pub fn get_a_row(&self,i_ : i32,nzi_ : &mut i32,subi_ : &mut[i32],vali_ : &mut[f64]) -> Result<(),String> { self.task.get_a_row(i_,nzi_,subi_,vali_) }
    /// Obtains the number of non-zero elements in one row of the linear constraint matrix
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the row.
    ///
    /// # Returns
    ///
    ///   - `nzi` Number of non-zeros in the i'th row of `A`.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getarownumnz>
    pub fn get_a_row_num_nz(&self,i_ : i32) -> Result<i32,String> { self.task.get_a_row_num_nz(i_) }
    /// Obtains a sequence of rows from the coefficient matrix.
    ///
    /// # Arguments
    ///
    /// - `first_` Index of the first row in the sequence.
    /// - `last_` Index of the last row in the sequence plus one.
    /// - `ptrb_` Row start pointers.
    /// - `ptre_` Row end pointers.
    /// - `sub_` Contains the column subscripts.
    /// - `val_` Contains the coefficient values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getarowslice64>
    pub fn get_a_row_slice(&self,first_ : i32,last_ : i32,ptrb_ : &mut[i64],ptre_ : &mut[i64],sub_ : &mut[i32],val_ : &mut[f64]) -> Result<(),String> { self.task.get_a_row_slice(first_,last_,ptrb_,ptre_,sub_,val_) }
    /// Obtains the number of non-zeros in a slice of rows of the coefficient matrix.
    ///
    /// # Arguments
    ///
    /// - `first_` Index of the first row in the sequence.
    /// - `last_` Index of the last row plus one in the sequence.
    ///
    /// # Returns
    ///
    ///   - `numnz` Number of non-zeros in the slice.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getarowslicenumnz64>
    pub fn get_a_row_slice_num_nz(&self,first_ : i32,last_ : i32) -> Result<i64,String> { self.task.get_a_row_slice_num_nz(first_,last_) }
    /// Obtains a sequence of rows from the coefficient matrix in sparse triplet format.
    ///
    /// # Arguments
    ///
    /// - `first_` Index of the first row in the sequence.
    /// - `last_` Index of the last row in the sequence plus one.
    /// - `subi_` Constraint subscripts.
    /// - `subj_` Column subscripts.
    /// - `val_` Values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getarowslicetrip>
    pub fn get_a_row_slice_trip(&self,first_ : i32,last_ : i32,subi_ : &mut[i32],subj_ : &mut[i32],val_ : &mut[f64]) -> Result<(),String> { self.task.get_a_row_slice_trip(first_,last_,subi_,subj_,val_) }
    /// Obtains the A matrix in sparse triplet format.
    ///
    /// # Arguments
    ///
    /// - `subi_` Constraint subscripts.
    /// - `subj_` Column subscripts.
    /// - `val_` Values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getatrip>
    pub fn get_a_trip(&self,subi_ : &mut[i32],subj_ : &mut[i32],val_ : &mut[f64]) -> Result<(),String> { self.task.get_a_trip(subi_,subj_,val_) }
    /// Gets the current A matrix truncation threshold.
    ///
    /// # Arguments
    ///
    /// - `tolzero_` Truncation tolerance.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getatruncatetol>
    pub fn get_a_truncate_tol(&self,tolzero_ : &mut[f64]) -> Result<(),String> { self.task.get_a_truncate_tol(tolzero_) }
    /// Obtains barA in block triplet form.
    ///
    /// # Arguments
    ///
    /// - `subi_` Constraint index.
    /// - `subj_` Symmetric matrix variable index.
    /// - `subk_` Block row index.
    /// - `subl_` Block column index.
    /// - `valijkl_` The numerical value associated with each block triplet.
    ///
    /// # Returns
    ///
    ///   - `num` Number of elements in the block triplet form.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarablocktriplet>
    pub fn get_bara_block_triplet(&self,subi_ : &mut[i32],subj_ : &mut[i32],subk_ : &mut[i32],subl_ : &mut[i32],valijkl_ : &mut[f64]) -> Result<i64,String> { self.task.get_bara_block_triplet(subi_,subj_,subk_,subl_,valijkl_) }
    /// Obtains information about an element in barA.
    ///
    /// # Arguments
    ///
    /// - `idx_` Position of the element in the vectorized form.
    /// - `i_` Row index of the element at position idx.
    /// - `j_` Column index of the element at position idx.
    /// - `sub_` A list indexes of the elements from symmetric matrix storage that appear in the weighted sum.
    /// - `weights_` The weights associated with each term in the weighted sum.
    ///
    /// # Returns
    ///
    ///   - `num` Number of terms in weighted sum that forms the element.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbaraidx>
    pub fn get_bara_idx(&self,idx_ : i64,i_ : &mut i32,j_ : &mut i32,sub_ : &mut[i64],weights_ : &mut[f64]) -> Result<i64,String> { self.task.get_bara_idx(idx_,i_,j_,sub_,weights_) }
    /// Obtains information about an element in barA.
    ///
    /// # Arguments
    ///
    /// - `idx_` Position of the element in the vectorized form.
    /// - `i_` Row index of the element at position idx.
    /// - `j_` Column index of the element at position idx.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbaraidxij>
    pub fn get_bara_idx_i_j(&self,idx_ : i64,i_ : &mut i32,j_ : &mut i32) -> Result<(),String> { self.task.get_bara_idx_i_j(idx_,i_,j_) }
    /// Obtains the number of terms in the weighted sum that form a particular element in barA.
    ///
    /// # Arguments
    ///
    /// - `idx_` The internal position of the element for which information should be obtained.
    ///
    /// # Returns
    ///
    ///   - `num` Number of terms in the weighted sum that form the specified element in barA.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbaraidxinfo>
    pub fn get_bara_idx_info(&self,idx_ : i64) -> Result<i64,String> { self.task.get_bara_idx_info(idx_) }
    /// Obtains the sparsity pattern of the barA matrix.
    ///
    /// # Arguments
    ///
    /// - `numnz_` Number of nonzero elements in barA.
    /// - `idxij_` Position of each nonzero element in the vector representation of barA.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarasparsity>
    pub fn get_bara_sparsity(&self,numnz_ : &mut i64,idxij_ : &mut[i64]) -> Result<(),String> { self.task.get_bara_sparsity(numnz_,idxij_) }
    /// Obtains barC in block triplet form.
    ///
    /// # Arguments
    ///
    /// - `subj_` Symmetric matrix variable index.
    /// - `subk_` Block row index.
    /// - `subl_` Block column index.
    /// - `valjkl_` The numerical value associated with each block triplet.
    ///
    /// # Returns
    ///
    ///   - `num` Number of elements in the block triplet form.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarcblocktriplet>
    pub fn get_barc_block_triplet(&self,subj_ : &mut[i32],subk_ : &mut[i32],subl_ : &mut[i32],valjkl_ : &mut[f64]) -> Result<i64,String> { self.task.get_barc_block_triplet(subj_,subk_,subl_,valjkl_) }
    /// Obtains information about an element in barc.
    ///
    /// # Arguments
    ///
    /// - `idx_` Index of the element for which information should be obtained.
    /// - `j_` Row index in barc.
    /// - `num_` Number of terms in the weighted sum.
    /// - `sub_` Elements appearing the weighted sum.
    /// - `weights_` Weights of terms in the weighted sum.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarcidx>
    pub fn get_barc_idx(&self,idx_ : i64,j_ : &mut i32,num_ : &mut i64,sub_ : &mut[i64],weights_ : &mut[f64]) -> Result<(),String> { self.task.get_barc_idx(idx_,j_,num_,sub_,weights_) }
    /// Obtains information about an element in barc.
    ///
    /// # Arguments
    ///
    /// - `idx_` Index of the element for which information should be obtained. The value is an index of a symmetric sparse variable.
    ///
    /// # Returns
    ///
    ///   - `num` Number of terms that appear in the weighted sum that forms the requested element.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarcidxinfo>
    pub fn get_barc_idx_info(&self,idx_ : i64) -> Result<i64,String> { self.task.get_barc_idx_info(idx_) }
    /// Obtains the row index of an element in barc.
    ///
    /// # Arguments
    ///
    /// - `idx_` Index of the element for which information should be obtained.
    /// - `j_` Row index in barc.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarcidxj>
    pub fn get_barc_idx_j(&self,idx_ : i64,j_ : &mut i32) -> Result<(),String> { self.task.get_barc_idx_j(idx_,j_) }
    /// Get the positions of the nonzero elements in barc.
    ///
    /// # Arguments
    ///
    /// - `numnz_` Number of nonzero elements in barc.
    /// - `idxj_` Internal positions of the nonzeros elements in barc.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarcsparsity>
    pub fn get_barc_sparsity(&self,numnz_ : &mut i64,idxj_ : &mut[i64]) -> Result<(),String> { self.task.get_barc_sparsity(numnz_,idxj_) }
    /// Obtains the dual solution for a semidefinite variable.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `j_` Index of the semidefinite variable.
    /// - `barsj_` Value of the j'th dual variable of barx.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarsj>
    pub fn get_bars_j(&self,whichsol_ : i32,j_ : i32,barsj_ : &mut[f64]) -> Result<(),String> { self.task.get_bars_j(whichsol_,j_,barsj_) }
    /// Obtains the dual solution for a sequence of semidefinite variables.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` Index of the first semidefinite variable in the slice.
    /// - `last_` Index of the last semidefinite variable in the slice plus one.
    /// - `slicesize_` Denotes the length of the array barsslice.
    /// - `barsslice_` Dual solution values of symmetric matrix variables in the slice, stored sequentially.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarsslice>
    pub fn get_bars_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,slicesize_ : i64,barsslice_ : &mut[f64]) -> Result<(),String> { self.task.get_bars_slice(whichsol_,first_,last_,slicesize_,barsslice_) }
    /// Obtains the name of a semidefinite variable.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the variable.
    ///
    /// # Returns
    ///
    ///   - `name` The requested name is copied to this buffer.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarvarname>
    pub fn get_barvar_name(&self,i_ : i32) -> Result<String,String> { self.task.get_barvar_name(i_) }
    /// Obtains the index of semidefinite variable from its name.
    ///
    /// # Arguments
    ///
    /// - `somename_` The name of the variable.
    /// - `asgn_` Non-zero if the name somename is assigned to some semidefinite variable.
    ///
    /// # Returns
    ///
    ///   - `index` The index of a semidefinite variable with the name somename (if one exists).
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarvarnameindex>
    pub fn get_barvar_name_index(&self,somename_ : &str,asgn_ : &mut i32) -> Result<i32,String> { self.task.get_barvar_name_index(somename_,asgn_) }
    /// Obtains the length of the name of a semidefinite variable.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the variable.
    ///
    /// # Returns
    ///
    ///   - `len` Returns the length of the indicated name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarvarnamelen>
    pub fn get_barvar_name_len(&self,i_ : i32) -> Result<i32,String> { self.task.get_barvar_name_len(i_) }
    /// Obtains the primal solution for a semidefinite variable.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `j_` Index of the semidefinite variable.
    /// - `barxj_` Value of the j'th variable of barx.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarxj>
    pub fn get_barx_j(&self,whichsol_ : i32,j_ : i32,barxj_ : &mut[f64]) -> Result<(),String> { self.task.get_barx_j(whichsol_,j_,barxj_) }
    /// Obtains the primal solution for a sequence of semidefinite variables.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` Index of the first semidefinite variable in the slice.
    /// - `last_` Index of the last semidefinite variable in the slice plus one.
    /// - `slicesize_` Denotes the length of the array barxslice.
    /// - `barxslice_` Solution values of symmetric matrix variables in the slice, stored sequentially.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarxslice>
    pub fn get_barx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,slicesize_ : i64,barxslice_ : &mut[f64]) -> Result<(),String> { self.task.get_barx_slice(whichsol_,first_,last_,slicesize_,barxslice_) }
    /// Obtains all objective coefficients.
    ///
    /// # Arguments
    ///
    /// - `c_` Linear terms of the objective as a dense vector. The length is the number of variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getc>
    pub fn get_c(&self,c_ : &mut[f64]) -> Result<(),String> { self.task.get_c(c_) }
    /// Obtains the fixed term in the objective.
    ///
    /// # Returns
    ///
    ///   - `cfix` Fixed term in the objective.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getcfix>
    pub fn get_cfix(&self) -> Result<f64,String> { self.task.get_cfix() }
    /// Obtains one objective coefficient.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the variable for which the c coefficient should be obtained.
    /// - `cj_` The c coefficient value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getcj>
    pub fn get_c_j(&self,j_ : i32,cj_ : &mut f64) -> Result<(),String> { self.task.get_c_j(j_,cj_) }
    /// Obtains a sequence of coefficients from the objective.
    ///
    /// # Arguments
    ///
    /// - `subj_` A list of variable indexes.
    /// - `c_` Linear terms of the requested list of the objective as a dense vector.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getclist>
    pub fn get_c_list(&self,subj_ : &[i32],c_ : &mut[f64]) -> Result<(),String> { self.task.get_c_list(subj_,c_) }
    /// Obtains bound information for one constraint.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the constraint for which the bound information should be obtained.
    /// - `bk_` Bound keys.
    ///   
    ///   See [Boundkey]
    /// - `bl_` Values for lower bounds.
    /// - `bu_` Values for upper bounds.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getconbound>
    pub fn get_con_bound(&self,i_ : i32,bk_ : & mut i32,bl_ : &mut f64,bu_ : &mut f64) -> Result<(),String> { self.task.get_con_bound(i_,bk_,bl_,bu_) }
    /// Obtains bounds information for a slice of the constraints.
    ///
    /// # Arguments
    ///
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `bk_` Bound keys.
    ///   
    ///   See [Boundkey]
    /// - `bl_` Values for lower bounds.
    /// - `bu_` Values for upper bounds.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getconboundslice>
    pub fn get_con_bound_slice(&self,first_ : i32,last_ : i32,bk_ : &mut[i32],bl_ : &mut[f64],bu_ : &mut[f64]) -> Result<(),String> { self.task.get_con_bound_slice(first_,last_,bk_,bl_,bu_) }
    /// Obtains a cone.
    ///
    /// # Arguments
    ///
    /// - `k_` Index of the cone.
    /// - `ct_` Specifies the type of the cone.
    ///   
    ///   See [Conetype]
    /// - `conepar_` For the power cone it denotes the exponent alpha. For other cone types it is unused and can be set to 0.
    /// - `nummem_` Number of member variables in the cone.
    /// - `submem_` Variable subscripts of the members in the cone.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getcone>
    pub fn get_cone(&mut self,k_ : i32,ct_ : & mut i32,conepar_ : &mut f64,nummem_ : &mut i32,submem_ : &mut[i32]) -> Result<(),String> { self.task.get_cone(k_,ct_,conepar_,nummem_,submem_) }
    /// Obtains information about a cone.
    ///
    /// # Arguments
    ///
    /// - `k_` Index of the cone.
    /// - `ct_` Specifies the type of the cone.
    ///   
    ///   See [Conetype]
    /// - `conepar_` For the power cone it denotes the exponent alpha. For other cone types it is unused and can be set to 0.
    /// - `nummem_` Number of member variables in the cone.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getconeinfo>
    pub fn get_cone_info(&self,k_ : i32,ct_ : & mut i32,conepar_ : &mut f64,nummem_ : &mut i32) -> Result<(),String> { self.task.get_cone_info(k_,ct_,conepar_,nummem_) }
    /// Obtains the name of a cone.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the cone.
    ///
    /// # Returns
    ///
    ///   - `name` The required name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getconename>
    pub fn get_cone_name(&self,i_ : i32) -> Result<String,String> { self.task.get_cone_name(i_) }
    /// Checks whether the name has been assigned to any cone.
    ///
    /// # Arguments
    ///
    /// - `somename_` The name which should be checked.
    /// - `asgn_` Is non-zero if the name somename is assigned to some cone.
    ///
    /// # Returns
    ///
    ///   - `index` If the name somename is assigned to some cone, this is the index of the cone.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getconenameindex>
    pub fn get_cone_name_index(&self,somename_ : &str,asgn_ : &mut i32) -> Result<i32,String> { self.task.get_cone_name_index(somename_,asgn_) }
    /// Obtains the length of the name of a cone.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the cone.
    ///
    /// # Returns
    ///
    ///   - `len` Returns the length of the indicated name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getconenamelen>
    pub fn get_cone_name_len(&self,i_ : i32) -> Result<i32,String> { self.task.get_cone_name_len(i_) }
    /// Obtains the name of a constraint.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the constraint.
    ///
    /// # Returns
    ///
    ///   - `name` The required name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getconname>
    pub fn get_con_name(&self,i_ : i32) -> Result<String,String> { self.task.get_con_name(i_) }
    /// Checks whether the name has been assigned to any constraint.
    ///
    /// # Arguments
    ///
    /// - `somename_` The name which should be checked.
    /// - `asgn_` Is non-zero if the name somename is assigned to some constraint.
    ///
    /// # Returns
    ///
    ///   - `index` If the name somename is assigned to a constraint, then return the index of the constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getconnameindex>
    pub fn get_con_name_index(&self,somename_ : &str,asgn_ : &mut i32) -> Result<i32,String> { self.task.get_con_name_index(somename_,asgn_) }
    /// Obtains the length of the name of a constraint.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the constraint.
    ///
    /// # Returns
    ///
    ///   - `len` Returns the length of the indicated name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getconnamelen>
    pub fn get_con_name_len(&self,i_ : i32) -> Result<i32,String> { self.task.get_con_name_len(i_) }
    /// Obtains a sequence of coefficients from the objective.
    ///
    /// # Arguments
    ///
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `c_` Linear terms of the requested slice of the objective as a dense vector.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getcslice>
    pub fn get_c_slice(&self,first_ : i32,last_ : i32,c_ : &mut[f64]) -> Result<(),String> { self.task.get_c_slice(first_,last_,c_) }
    /// Obtains the dimension of a symmetric matrix variable.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the semidefinite variable whose dimension is requested.
    ///
    /// # Returns
    ///
    ///   - `dimbarvarj` The dimension of the j'th semidefinite variable.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdimbarvarj>
    pub fn get_dim_barvar_j(&self,j_ : i32) -> Result<i32,String> { self.task.get_dim_barvar_j(j_) }
    /// Obtains the list of affine expression indexes in a disjunctive constraint.
    ///
    /// # Arguments
    ///
    /// - `djcidx_` Index of the disjunctive constraint.
    /// - `afeidxlist_` List of affine expression indexes.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjcafeidxlist>
    pub fn get_djc_afe_idx_list(&self,djcidx_ : i64,afeidxlist_ : &mut[i64]) -> Result<(),String> { self.task.get_djc_afe_idx_list(djcidx_,afeidxlist_) }
    /// Obtains the optional constant term vector of a disjunctive constraint.
    ///
    /// # Arguments
    ///
    /// - `djcidx_` Index of the disjunctive constraint.
    /// - `b_` The vector b.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjcb>
    pub fn get_djc_b(&self,djcidx_ : i64,b_ : &mut[f64]) -> Result<(),String> { self.task.get_djc_b(djcidx_,b_) }
    /// Obtains the list of domain indexes in a disjunctive constraint.
    ///
    /// # Arguments
    ///
    /// - `djcidx_` Index of the disjunctive constraint.
    /// - `domidxlist_` List of term sizes.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjcdomainidxlist>
    pub fn get_djc_domain_idx_list(&self,djcidx_ : i64,domidxlist_ : &mut[i64]) -> Result<(),String> { self.task.get_djc_domain_idx_list(djcidx_,domidxlist_) }
    /// Obtains the name of a disjunctive constraint.
    ///
    /// # Arguments
    ///
    /// - `djcidx_` Index of a disjunctive constraint.
    ///
    /// # Returns
    ///
    ///   - `name` Returns the required name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjcname>
    pub fn get_djc_name(&self,djcidx_ : i64) -> Result<String,String> { self.task.get_djc_name(djcidx_) }
    /// Obtains the length of the name of a disjunctive constraint.
    ///
    /// # Arguments
    ///
    /// - `djcidx_` Index of a disjunctive constraint.
    ///
    /// # Returns
    ///
    ///   - `len` Returns the length of the indicated name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjcnamelen>
    pub fn get_djc_name_len(&self,djcidx_ : i64) -> Result<i32,String> { self.task.get_djc_name_len(djcidx_) }
    /// Obtains the number of affine expressions in the disjunctive constraint.
    ///
    /// # Arguments
    ///
    /// - `djcidx_` Index of the disjunctive constraint.
    ///
    /// # Returns
    ///
    ///   - `numafe` Number of affine expressions in the disjunctive constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjcnumafe>
    pub fn get_djc_num_afe(&mut self,djcidx_ : i64) -> Result<i64,String> { self.task.get_djc_num_afe(djcidx_) }
    /// Obtains the number of affine expressions in all disjunctive constraints.
    ///
    /// # Returns
    ///
    ///   - `numafetot` Number of affine expressions in all disjunctive constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjcnumafetot>
    pub fn get_djc_num_afe_tot(&mut self) -> Result<i64,String> { self.task.get_djc_num_afe_tot() }
    /// Obtains the number of domains in the disjunctive constraint.
    ///
    /// # Arguments
    ///
    /// - `djcidx_` Index of the disjunctive constraint.
    ///
    /// # Returns
    ///
    ///   - `numdomain` Number of domains in the disjunctive constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjcnumdomain>
    pub fn get_djc_num_domain(&mut self,djcidx_ : i64) -> Result<i64,String> { self.task.get_djc_num_domain(djcidx_) }
    /// Obtains the number of domains in all disjunctive constraints.
    ///
    /// # Returns
    ///
    ///   - `numdomaintot` Number of domains in all disjunctive constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjcnumdomaintot>
    pub fn get_djc_num_domain_tot(&mut self) -> Result<i64,String> { self.task.get_djc_num_domain_tot() }
    /// Obtains the number terms in the disjunctive constraint.
    ///
    /// # Arguments
    ///
    /// - `djcidx_` Index of the disjunctive constraint.
    ///
    /// # Returns
    ///
    ///   - `numterm` Number of terms in the disjunctive constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjcnumterm>
    pub fn get_djc_num_term(&mut self,djcidx_ : i64) -> Result<i64,String> { self.task.get_djc_num_term(djcidx_) }
    /// Obtains the number of terms in all disjunctive constraints.
    ///
    /// # Returns
    ///
    ///   - `numtermtot` Total number of terms in all disjunctive constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjcnumtermtot>
    pub fn get_djc_num_term_tot(&mut self) -> Result<i64,String> { self.task.get_djc_num_term_tot() }
    /// Obtains full data of all disjunctive constraints.
    ///
    /// # Arguments
    ///
    /// - `domidxlist_` The concatenation of index lists of domains appearing in all disjunctive constraints.
    /// - `afeidxlist_` The concatenation of index lists of affine expressions appearing in all disjunctive constraints.
    /// - `b_` The concatenation of vectors b appearing in all disjunctive constraints.
    /// - `termsizelist_` The concatenation of lists of term sizes appearing in all disjunctive constraints.
    /// - `numterms_` The number of terms in each of the disjunctive constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjcs>
    pub fn get_djcs(&self,domidxlist_ : &mut[i64],afeidxlist_ : &mut[i64],b_ : &mut[f64],termsizelist_ : &mut[i64],numterms_ : &mut[i64]) -> Result<(),String> { self.task.get_djcs(domidxlist_,afeidxlist_,b_,termsizelist_,numterms_) }
    /// Obtains the list of term sizes in a disjunctive constraint.
    ///
    /// # Arguments
    ///
    /// - `djcidx_` Index of the disjunctive constraint.
    /// - `termsizelist_` List of term sizes.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjctermsizelist>
    pub fn get_djc_term_size_list(&self,djcidx_ : i64,termsizelist_ : &mut[i64]) -> Result<(),String> { self.task.get_djc_term_size_list(djcidx_,termsizelist_) }
    /// Obtains the dimension of the domain.
    ///
    /// # Arguments
    ///
    /// - `domidx_` Index of the domain.
    ///
    /// # Returns
    ///
    ///   - `n` Dimension of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdomainn>
    pub fn get_domain_n(&self,domidx_ : i64) -> Result<i64,String> { self.task.get_domain_n(domidx_) }
    /// Obtains the name of a domain.
    ///
    /// # Arguments
    ///
    /// - `domidx_` Index of a domain.
    ///
    /// # Returns
    ///
    ///   - `name` Returns the required name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdomainname>
    pub fn get_domain_name(&self,domidx_ : i64) -> Result<String,String> { self.task.get_domain_name(domidx_) }
    /// Obtains the length of the name of a domain.
    ///
    /// # Arguments
    ///
    /// - `domidx_` Index of a domain.
    ///
    /// # Returns
    ///
    ///   - `len` Returns the length of the indicated name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdomainnamelen>
    pub fn get_domain_name_len(&self,domidx_ : i64) -> Result<i32,String> { self.task.get_domain_name_len(domidx_) }
    /// Returns the type of the domain.
    ///
    /// # Arguments
    ///
    /// - `domidx_` Index of the domain.
    ///
    /// # Returns
    ///
    ///   - `domtype` The type of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdomaintype>
    pub fn get_domain_type(&self,domidx_ : i64) -> Result<i32,String> { self.task.get_domain_type(domidx_) }
    /// Obtains a double information item.
    ///
    /// # Arguments
    ///
    /// - `whichdinf_` Specifies a double information item.
    ///   
    ///   See [Dinfitem]
    ///
    /// # Returns
    ///
    ///   - `dvalue` The value of the required double information item.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdouinf>
    pub fn get_dou_inf(&self,whichdinf_ : i32) -> Result<f64,String> { self.task.get_dou_inf(whichdinf_) }
    /// Obtains a double parameter.
    ///
    /// # Arguments
    ///
    /// - `param_` Which parameter.
    ///   
    ///   See [Dparam]
    ///
    /// # Returns
    ///
    ///   - `parvalue` Parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdouparam>
    pub fn get_dou_param(&self,param_ : i32) -> Result<f64,String> { self.task.get_dou_param(param_) }
    /// Computes the dual objective value associated with the solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `dualobj_` Objective value corresponding to the dual solution.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdualobj>
    pub fn get_dual_obj(&self,whichsol_ : i32,dualobj_ : &mut f64) -> Result<(),String> { self.task.get_dual_obj(whichsol_,dualobj_) }
    /// Compute norms of the dual solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `nrmy_` The norm of the y vector.
    /// - `nrmslc_` The norm of the slc vector.
    /// - `nrmsuc_` The norm of the suc vector.
    /// - `nrmslx_` The norm of the slx vector.
    /// - `nrmsux_` The norm of the sux vector.
    /// - `nrmsnx_` The norm of the snx vector.
    /// - `nrmbars_` The norm of the bars vector.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdualsolutionnorms>
    pub fn get_dual_solution_norms(&self,whichsol_ : i32,nrmy_ : &mut f64,nrmslc_ : &mut f64,nrmsuc_ : &mut f64,nrmslx_ : &mut f64,nrmsux_ : &mut f64,nrmsnx_ : &mut f64,nrmbars_ : &mut f64) -> Result<(),String> { self.task.get_dual_solution_norms(whichsol_,nrmy_,nrmslc_,nrmsuc_,nrmslx_,nrmsux_,nrmsnx_,nrmbars_) }
    /// Computes the violation of the dual solution for set of affine conic constraints.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `accidxlist_` An array of indexes of conic constraints.
    /// - `viol_` List of violations corresponding to sub.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdviolacc>
    pub fn get_dviol_acc(&self,whichsol_ : i32,accidxlist_ : &[i64],viol_ : &mut[f64]) -> Result<(),String> { self.task.get_dviol_acc(whichsol_,accidxlist_,viol_) }
    /// Computes the violation of dual solution for a set of semidefinite variables.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sub_` An array of indexes of barx variables.
    /// - `viol_` List of violations corresponding to sub.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdviolbarvar>
    pub fn get_dviol_barvar(&self,whichsol_ : i32,sub_ : &[i32],viol_ : &mut[f64]) -> Result<(),String> { self.task.get_dviol_barvar(whichsol_,sub_,viol_) }
    /// Computes the violation of a dual solution associated with a set of constraints.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sub_` An array of indexes of constraints.
    /// - `viol_` List of violations corresponding to sub.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdviolcon>
    pub fn get_dviol_con(&self,whichsol_ : i32,sub_ : &[i32],viol_ : &mut[f64]) -> Result<(),String> { self.task.get_dviol_con(whichsol_,sub_,viol_) }
    /// Computes the violation of a solution for set of dual conic constraints.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sub_` An array of indexes of conic constraints.
    /// - `viol_` List of violations corresponding to sub.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdviolcones>
    pub fn get_dviol_cones(&self,whichsol_ : i32,sub_ : &[i32],viol_ : &mut[f64]) -> Result<(),String> { self.task.get_dviol_cones(whichsol_,sub_,viol_) }
    /// Computes the violation of a dual solution associated with a set of scalar variables.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sub_` An array of indexes of x variables.
    /// - `viol_` List of violations corresponding to sub.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdviolvar>
    pub fn get_dviol_var(&self,whichsol_ : i32,sub_ : &[i32],viol_ : &mut[f64]) -> Result<(),String> { self.task.get_dviol_var(whichsol_,sub_,viol_) }
    /// Obtains the index of a named information item.
    ///
    /// # Arguments
    ///
    /// - `inftype_` Type of the information item.
    ///   
    ///   See [Inftype]
    /// - `infname_` Name of the information item.
    /// - `infindex_` The item index.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getinfindex>
    pub fn get_inf_index(&self,inftype_ : i32,infname_ : &str,infindex_ : &mut i32) -> Result<(),String> { self.task.get_inf_index(inftype_,infname_,infindex_) }
    /// Obtains the maximum index of an information item of a given type.
    ///
    /// # Arguments
    ///
    /// - `inftype_` Type of the information item.
    ///   
    ///   See [Inftype]
    /// - `infmax_` The maximum index (plus 1) requested.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getinfmax>
    pub fn get_inf_max(&self,inftype_ : i32,infmax_ : &mut[i32]) -> Result<(),String> { self.task.get_inf_max(inftype_,infmax_) }
    /// Obtains the name of an information item.
    ///
    /// # Arguments
    ///
    /// - `inftype_` Type of the information item.
    ///   
    ///   See [Inftype]
    /// - `whichinf_` An information item.
    ///
    /// # Returns
    ///
    ///   - `infname` Name of the information item.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getinfname>
    pub fn get_inf_name(&self,inftype_ : i32,whichinf_ : i32) -> Result<String,String> { self.task.get_inf_name(inftype_,whichinf_) }
    /// Obtains an integer information item.
    ///
    /// # Arguments
    ///
    /// - `whichiinf_` Specifies an integer information item.
    ///   
    ///   See [Iinfitem]
    ///
    /// # Returns
    ///
    ///   - `ivalue` The value of the required integer information item.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getintinf>
    pub fn get_int_inf(&self,whichiinf_ : i32) -> Result<i32,String> { self.task.get_int_inf(whichiinf_) }
    /// Obtains an integer parameter.
    ///
    /// # Arguments
    ///
    /// - `param_` Which parameter.
    ///   
    ///   See [Iparam]
    ///
    /// # Returns
    ///
    ///   - `parvalue` Parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getintparam>
    pub fn get_int_param(&self,param_ : i32) -> Result<i32,String> { self.task.get_int_param(param_) }
    /// Obtains the length of one semidefinite variable.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the semidefinite variable whose length if requested.
    ///
    /// # Returns
    ///
    ///   - `lenbarvarj` Number of scalar elements in the lower triangular part of the semidefinite variable.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getlenbarvarj>
    pub fn get_len_barvar_j(&self,j_ : i32) -> Result<i64,String> { self.task.get_len_barvar_j(j_) }
    /// Obtains a long integer information item.
    ///
    /// # Arguments
    ///
    /// - `whichliinf_` Specifies a long information item.
    ///   
    ///   See [Liinfitem]
    ///
    /// # Returns
    ///
    ///   - `ivalue` The value of the required long integer information item.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getlintinf>
    pub fn get_lint_inf(&self,whichliinf_ : i32) -> Result<i64,String> { self.task.get_lint_inf(whichliinf_) }
    /// Obtains the maximum length (not including terminating zero character) of any objective, constraint, variable, domain or cone name.
    ///
    /// # Arguments
    ///
    /// - `maxlen_` The maximum length of any name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getmaxnamelen>
    pub fn get_max_name_len(&self,maxlen_ : &mut i32) -> Result<(),String> { self.task.get_max_name_len(maxlen_) }
    /// Obtains number of preallocated non-zeros in the linear constraint matrix.
    ///
    /// # Returns
    ///
    ///   - `maxnumanz` Number of preallocated non-zero linear matrix elements.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getmaxnumanz64>
    pub fn get_max_num_a_nz(&self) -> Result<i64,String> { self.task.get_max_num_a_nz() }
    /// Obtains maximum number of symmetric matrix variables for which space is currently preallocated.
    ///
    /// # Returns
    ///
    ///   - `maxnumbarvar` Maximum number of symmetric matrix variables for which space is currently preallocated.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getmaxnumbarvar>
    pub fn get_max_num_barvar(&self) -> Result<i32,String> { self.task.get_max_num_barvar() }
    /// Obtains the number of preallocated constraints in the optimization task.
    ///
    /// # Arguments
    ///
    /// - `maxnumcon_` Number of preallocated constraints in the optimization task.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getmaxnumcon>
    pub fn get_max_num_con(&self,maxnumcon_ : &mut i32) -> Result<(),String> { self.task.get_max_num_con(maxnumcon_) }
    /// Obtains the number of preallocated cones in the optimization task.
    ///
    /// # Arguments
    ///
    /// - `maxnumcone_` Number of preallocated conic constraints in the optimization task.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getmaxnumcone>
    pub fn get_max_num_cone(&self,maxnumcone_ : &mut i32) -> Result<(),String> { self.task.get_max_num_cone(maxnumcone_) }
    /// Obtains the number of preallocated non-zeros for all quadratic terms in objective and constraints.
    ///
    /// # Arguments
    ///
    /// - `maxnumqnz_` Number of non-zero elements preallocated in quadratic coefficient matrices.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getmaxnumqnz64>
    pub fn get_max_num_q_nz(&self,maxnumqnz_ : &mut i64) -> Result<(),String> { self.task.get_max_num_q_nz(maxnumqnz_) }
    /// Obtains the maximum number variables allowed.
    ///
    /// # Arguments
    ///
    /// - `maxnumvar_` Number of preallocated variables in the optimization task.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getmaxnumvar>
    pub fn get_max_num_var(&self,maxnumvar_ : &mut i32) -> Result<(),String> { self.task.get_max_num_var(maxnumvar_) }
    /// Obtains information about the amount of memory used by a task.
    ///
    /// # Arguments
    ///
    /// - `meminuse_` Amount of memory currently used by the task.
    /// - `maxmemuse_` Maximum amount of memory used by the task until now.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getmemusagetask>
    pub fn get_mem_usage(&self,meminuse_ : &mut i64,maxmemuse_ : &mut i64) -> Result<(),String> { self.task.get_mem_usage(meminuse_,maxmemuse_) }
    /// Obtains the number of threads used by the mixed integer optimizer.
    ///
    /// # Returns
    ///
    ///   - `numthreads` The number of threads.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getmionumthreads>
    pub fn get_mio_num_threads(&self) -> Result<i32,String> { self.task.get_mio_num_threads() }
    /// Obtains a named double information item.
    ///
    /// # Arguments
    ///
    /// - `infitemname_` The name of a double information item.
    /// - `dvalue_` The value of the required double information item.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnadouinf>
    pub fn get_na_dou_inf(&self,infitemname_ : &str,dvalue_ : &mut f64) -> Result<(),String> { self.task.get_na_dou_inf(infitemname_,dvalue_) }
    /// Obtains a double parameter.
    ///
    /// # Arguments
    ///
    /// - `paramname_` Name of a parameter.
    /// - `parvalue_` Parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnadouparam>
    pub fn get_na_dou_param(&self,paramname_ : &str,parvalue_ : &mut f64) -> Result<(),String> { self.task.get_na_dou_param(paramname_,parvalue_) }
    /// Obtains a named integer information item.
    ///
    /// # Arguments
    ///
    /// - `infitemname_` The name of an integer information item.
    /// - `ivalue_` The value of the required integer information item.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnaintinf>
    pub fn get_na_int_inf(&self,infitemname_ : &str,ivalue_ : &mut i32) -> Result<(),String> { self.task.get_na_int_inf(infitemname_,ivalue_) }
    /// Obtains an integer parameter.
    ///
    /// # Arguments
    ///
    /// - `paramname_` Name of a parameter.
    /// - `parvalue_` Parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnaintparam>
    pub fn get_na_int_param(&self,paramname_ : &str,parvalue_ : &mut i32) -> Result<(),String> { self.task.get_na_int_param(paramname_,parvalue_) }
    /// Obtains a string parameter.
    ///
    /// # Arguments
    ///
    /// - `paramname_` Name of a parameter.
    /// - `sizeparamname_` Size of the name buffer.
    /// - `len_` Returns the length of the parameter value.
    ///
    /// # Returns
    ///
    ///   - `parvalue` Parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnastrparam>
    pub fn get_na_str_param(&self,paramname_ : &str,sizeparamname_ : i32,len_ : &mut i32) -> Result<String,String> { self.task.get_na_str_param(paramname_,sizeparamname_,len_) }
    /// Obtains the number of affine conic constraints.
    ///
    /// # Returns
    ///
    ///   - `num` The number of affine conic constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumacc>
    pub fn get_num_acc(&mut self) -> Result<i64,String> { self.task.get_num_acc() }
    /// Obtains the number of affine expressions.
    ///
    /// # Returns
    ///
    ///   - `numafe` Number of affine expressions.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumafe>
    pub fn get_num_afe(&mut self) -> Result<i64,String> { self.task.get_num_afe() }
    /// Obtains the number of non-zeros in the coefficient matrix.
    ///
    /// # Returns
    ///
    ///   - `numanz` Number of non-zero elements in the linear constraint matrix.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumanz>
    pub fn get_num_a_nz(&self) -> Result<i32,String> { self.task.get_num_a_nz() }
    /// Obtains the number of non-zeros in the coefficient matrix.
    ///
    /// # Returns
    ///
    ///   - `numanz` Number of non-zero elements in the linear constraint matrix.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumanz64>
    pub fn get_num_a_nz_64(&self) -> Result<i64,String> { self.task.get_num_a_nz_64() }
    /// Obtains an upper bound on the number of scalar elements in the block triplet form of bara.
    ///
    /// # Returns
    ///
    ///   - `num` An upper bound on the number of elements in the block triplet form of bara.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumbarablocktriplets>
    pub fn get_num_bara_block_triplets(&self) -> Result<i64,String> { self.task.get_num_bara_block_triplets() }
    /// Get the number of nonzero elements in barA.
    ///
    /// # Returns
    ///
    ///   - `nz` The number of nonzero block elements in barA.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumbaranz>
    pub fn get_num_bara_nz(&self) -> Result<i64,String> { self.task.get_num_bara_nz() }
    /// Obtains an upper bound on the number of elements in the block triplet form of barc.
    ///
    /// # Returns
    ///
    ///   - `num` An upper bound on the number of elements in the block triplet form of barc.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumbarcblocktriplets>
    pub fn get_num_barc_block_triplets(&self) -> Result<i64,String> { self.task.get_num_barc_block_triplets() }
    /// Obtains the number of nonzero elements in barc.
    ///
    /// # Returns
    ///
    ///   - `nz` The number of nonzero elements in barc.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumbarcnz>
    pub fn get_num_barc_nz(&self) -> Result<i64,String> { self.task.get_num_barc_nz() }
    /// Obtains the number of semidefinite variables.
    ///
    /// # Returns
    ///
    ///   - `numbarvar` Number of semidefinite variables in the problem.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumbarvar>
    pub fn get_num_barvar(&self) -> Result<i32,String> { self.task.get_num_barvar() }
    /// Obtains the number of constraints.
    ///
    /// # Returns
    ///
    ///   - `numcon` Number of constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumcon>
    pub fn get_num_con(&self) -> Result<i32,String> { self.task.get_num_con() }
    /// Obtains the number of cones.
    ///
    /// # Returns
    ///
    ///   - `numcone` Number of conic constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumcone>
    pub fn get_num_cone(&self) -> Result<i32,String> { self.task.get_num_cone() }
    /// Obtains the number of members in a cone.
    ///
    /// # Arguments
    ///
    /// - `k_` Index of the cone.
    /// - `nummem_` Number of member variables in the cone.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumconemem>
    pub fn get_num_cone_mem(&self,k_ : i32,nummem_ : &mut i32) -> Result<(),String> { self.task.get_num_cone_mem(k_,nummem_) }
    /// Obtains the number of disjunctive constraints.
    ///
    /// # Returns
    ///
    ///   - `num` The number of disjunctive constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumdjc>
    pub fn get_num_djc(&mut self) -> Result<i64,String> { self.task.get_num_djc() }
    /// Obtain the number of domains defined.
    ///
    /// # Returns
    ///
    ///   - `numdomain` Number of domains in the task.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumdomain>
    pub fn get_num_domain(&mut self) -> Result<i64,String> { self.task.get_num_domain() }
    /// Obtains the number of integer-constrained variables.
    ///
    /// # Returns
    ///
    ///   - `numintvar` Number of integer variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumintvar>
    pub fn get_num_int_var(&self) -> Result<i32,String> { self.task.get_num_int_var() }
    /// Obtains the number of parameters of a given type.
    ///
    /// # Arguments
    ///
    /// - `partype_` Parameter type.
    ///   
    ///   See [Parametertype]
    /// - `numparam_` Returns the number of parameters of the requested type.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumparam>
    pub fn get_num_param(&self,partype_ : i32,numparam_ : &mut i32) -> Result<(),String> { self.task.get_num_param(partype_,numparam_) }
    /// Obtains the number of non-zero quadratic terms in a constraint.
    ///
    /// # Arguments
    ///
    /// - `k_` Index of the constraint for which the number quadratic terms should be obtained.
    ///
    /// # Returns
    ///
    ///   - `numqcnz` Number of quadratic terms.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumqconknz64>
    pub fn get_num_q_con_k_nz(&self,k_ : i32) -> Result<i64,String> { self.task.get_num_q_con_k_nz(k_) }
    /// Obtains the number of non-zero quadratic terms in the objective.
    ///
    /// # Returns
    ///
    ///   - `numqonz` Number of non-zero elements in the quadratic objective terms.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumqobjnz64>
    pub fn get_num_q_obj_nz(&self) -> Result<i64,String> { self.task.get_num_q_obj_nz() }
    /// Obtains the number of symmetric matrices stored.
    ///
    /// # Arguments
    ///
    /// - `num_` The number of symmetric sparse matrices.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumsymmat>
    pub fn get_num_sym_mat(&self,num_ : &mut i64) -> Result<(),String> { self.task.get_num_sym_mat(num_) }
    /// Obtains the number of variables.
    ///
    /// # Returns
    ///
    ///   - `numvar` Number of variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumvar>
    pub fn get_num_var(&self) -> Result<i32,String> { self.task.get_num_var() }
    /// Obtains the name assigned to the objective function.
    ///
    /// # Returns
    ///
    ///   - `objname` Assigned the objective name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getobjname>
    pub fn get_obj_name(&self) -> Result<String,String> { self.task.get_obj_name() }
    /// Obtains the length of the name assigned to the objective function.
    ///
    /// # Returns
    ///
    ///   - `len` Assigned the length of the objective name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getobjnamelen>
    pub fn get_obj_name_len(&self) -> Result<i32,String> { self.task.get_obj_name_len() }
    /// Gets the objective sense.
    ///
    /// # Returns
    ///
    ///   - `sense` The returned objective sense.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getobjsense>
    pub fn get_obj_sense(&self) -> Result<i32,String> { self.task.get_obj_sense() }
    /// Obtains the maximum index of a parameter of a given type.
    ///
    /// # Arguments
    ///
    /// - `partype_` Parameter type.
    ///   
    ///   See [Parametertype]
    /// - `parammax_` The maximum index (plus 1) of the given parameter type.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getparammax>
    pub fn get_param_max(&self,partype_ : i32,parammax_ : &mut i32) -> Result<(),String> { self.task.get_param_max(partype_,parammax_) }
    /// Obtains the name of a parameter.
    ///
    /// # Arguments
    ///
    /// - `partype_` Parameter type.
    ///   
    ///   See [Parametertype]
    /// - `param_` Which parameter.
    ///
    /// # Returns
    ///
    ///   - `parname` Parameter name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getparamname>
    pub fn get_param_name(&self,partype_ : i32,param_ : i32) -> Result<String,String> { self.task.get_param_name(partype_,param_) }
    /// Obtains the exponent vector of a power domain.
    ///
    /// # Arguments
    ///
    /// - `domidx_` Index of the domain.
    /// - `alpha_` The exponent vector of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getpowerdomainalpha>
    pub fn get_power_domain_alpha(&mut self,domidx_ : i64,alpha_ : &mut[f64]) -> Result<(),String> { self.task.get_power_domain_alpha(domidx_,alpha_) }
    /// Obtains structural information about a power domain.
    ///
    /// # Arguments
    ///
    /// - `domidx_` Index of the domain.
    /// - `n_` Dimension of the domain.
    /// - `nleft_` Number of variables on the left hand side.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getpowerdomaininfo>
    pub fn get_power_domain_info(&mut self,domidx_ : i64,n_ : &mut i64,nleft_ : &mut i64) -> Result<(),String> { self.task.get_power_domain_info(domidx_,n_,nleft_) }
    /// Computes the primal objective value for the desired solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    ///
    /// # Returns
    ///
    ///   - `primalobj` Objective value corresponding to the primal solution.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getprimalobj>
    pub fn get_primal_obj(&self,whichsol_ : i32) -> Result<f64,String> { self.task.get_primal_obj(whichsol_) }
    /// Compute norms of the primal solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `nrmxc_` The norm of the xc vector.
    /// - `nrmxx_` The norm of the xx vector.
    /// - `nrmbarx_` The norm of the barX vector.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getprimalsolutionnorms>
    pub fn get_primal_solution_norms(&self,whichsol_ : i32,nrmxc_ : &mut f64,nrmxx_ : &mut f64,nrmbarx_ : &mut f64) -> Result<(),String> { self.task.get_primal_solution_norms(whichsol_,nrmxc_,nrmxx_,nrmbarx_) }
    /// Obtains the problem type.
    ///
    /// # Returns
    ///
    ///   - `probtype` The problem type.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getprobtype>
    pub fn get_prob_type(&self) -> Result<i32,String> { self.task.get_prob_type() }
    /// Obtains the problem status.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    ///
    /// # Returns
    ///
    ///   - `problemsta` Problem status.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getprosta>
    pub fn get_pro_sta(&self,whichsol_ : i32) -> Result<i32,String> { self.task.get_pro_sta(whichsol_) }
    /// Computes the violation of a solution for set of affine conic constraints.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `accidxlist_` An array of indexes of conic constraints.
    /// - `viol_` List of violations corresponding to sub.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getpviolacc>
    pub fn get_pviol_acc(&self,whichsol_ : i32,accidxlist_ : &[i64],viol_ : &mut[f64]) -> Result<(),String> { self.task.get_pviol_acc(whichsol_,accidxlist_,viol_) }
    /// Computes the violation of a primal solution for a list of semidefinite variables.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sub_` An array of indexes of barX variables.
    /// - `viol_` List of violations corresponding to sub.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getpviolbarvar>
    pub fn get_pviol_barvar(&self,whichsol_ : i32,sub_ : &[i32],viol_ : &mut[f64]) -> Result<(),String> { self.task.get_pviol_barvar(whichsol_,sub_,viol_) }
    /// Computes the violation of a primal solution associated to a constraint.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sub_` An array of indexes of constraints.
    /// - `viol_` List of violations corresponding to sub.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getpviolcon>
    pub fn get_pviol_con(&self,whichsol_ : i32,sub_ : &[i32],viol_ : &mut[f64]) -> Result<(),String> { self.task.get_pviol_con(whichsol_,sub_,viol_) }
    /// Computes the violation of a solution for set of conic constraints.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sub_` An array of indexes of conic constraints.
    /// - `viol_` List of violations corresponding to sub.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getpviolcones>
    pub fn get_pviol_cones(&self,whichsol_ : i32,sub_ : &[i32],viol_ : &mut[f64]) -> Result<(),String> { self.task.get_pviol_cones(whichsol_,sub_,viol_) }
    /// Computes the violation of a solution for set of disjunctive constraints.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `djcidxlist_` An array of indexes of disjunctive constraints.
    /// - `viol_` List of violations corresponding to sub.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getpvioldjc>
    pub fn get_pviol_djc(&self,whichsol_ : i32,djcidxlist_ : &[i64],viol_ : &mut[f64]) -> Result<(),String> { self.task.get_pviol_djc(whichsol_,djcidxlist_,viol_) }
    /// Computes the violation of a primal solution for a list of scalar variables.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sub_` An array of indexes of x variables.
    /// - `viol_` List of violations corresponding to sub.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getpviolvar>
    pub fn get_pviol_var(&self,whichsol_ : i32,sub_ : &[i32],viol_ : &mut[f64]) -> Result<(),String> { self.task.get_pviol_var(whichsol_,sub_,viol_) }
    /// Obtains all the quadratic terms in a constraint.
    ///
    /// # Arguments
    ///
    /// - `k_` Which constraint.
    /// - `qcsubi_` Row subscripts for quadratic constraint matrix.
    /// - `qcsubj_` Column subscripts for quadratic constraint matrix.
    /// - `qcval_` Quadratic constraint coefficient values.
    ///
    /// # Returns
    ///
    ///   - `numqcnz` Number of quadratic terms.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getqconk64>
    pub fn get_q_con_k(&self,k_ : i32,qcsubi_ : &mut[i32],qcsubj_ : &mut[i32],qcval_ : &mut[f64]) -> Result<i64,String> { self.task.get_q_con_k(k_,qcsubi_,qcsubj_,qcval_) }
    /// Obtains all the quadratic terms in the objective.
    ///
    /// # Arguments
    ///
    /// - `numqonz_` Number of non-zero elements in the quadratic objective terms.
    /// - `qosubi_` Row subscripts for quadratic objective coefficients.
    /// - `qosubj_` Column subscripts for quadratic objective coefficients.
    /// - `qoval_` Quadratic objective coefficient values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getqobj64>
    pub fn get_q_obj(&self,numqonz_ : &mut i64,qosubi_ : &mut[i32],qosubj_ : &mut[i32],qoval_ : &mut[f64]) -> Result<(),String> { self.task.get_q_obj(numqonz_,qosubi_,qosubj_,qoval_) }
    /// Obtains one coefficient from the quadratic term of the objective
    ///
    /// # Arguments
    ///
    /// - `i_` Row index of the coefficient.
    /// - `j_` Column index of coefficient.
    /// - `qoij_` The required coefficient.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getqobjij>
    pub fn get_q_obj_i_j(&self,i_ : i32,j_ : i32,qoij_ : &mut f64) -> Result<(),String> { self.task.get_q_obj_i_j(i_,j_,qoij_) }
    /// Obtains the reduced costs for a sequence of variables.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` The index of the first variable in the sequence.
    /// - `last_` The index of the last variable in the sequence plus 1.
    /// - `redcosts_` Returns the requested reduced costs.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getreducedcosts>
    pub fn get_reduced_costs(&self,whichsol_ : i32,first_ : i32,last_ : i32,redcosts_ : &mut[f64]) -> Result<(),String> { self.task.get_reduced_costs(whichsol_,first_,last_,redcosts_) }
    /// Obtains the status keys for the constraints.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `skc_` Status keys for the constraints.
    ///   
    ///   See [Stakey]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getskc>
    pub fn get_skc(&self,whichsol_ : i32,skc_ : &mut[i32]) -> Result<(),String> { self.task.get_skc(whichsol_,skc_) }
    /// Obtains the status keys for a slice of the constraints.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `skc_` Status keys for the constraints.
    ///   
    ///   See [Stakey]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getskcslice>
    pub fn get_skc_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,skc_ : &mut[i32]) -> Result<(),String> { self.task.get_skc_slice(whichsol_,first_,last_,skc_) }
    /// Obtains the status keys for the conic constraints.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `skn_` Status keys for the conic constraints.
    ///   
    ///   See [Stakey]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getskn>
    pub fn get_skn(&self,whichsol_ : i32,skn_ : &mut[i32]) -> Result<(),String> { self.task.get_skn(whichsol_,skn_) }
    /// Obtains the status keys for the scalar variables.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `skx_` Status keys for the variables.
    ///   
    ///   See [Stakey]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getskx>
    pub fn get_skx(&self,whichsol_ : i32,skx_ : &mut[i32]) -> Result<(),String> { self.task.get_skx(whichsol_,skx_) }
    /// Obtains the status keys for a slice of the scalar variables.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `skx_` Status keys for the variables.
    ///   
    ///   See [Stakey]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getskxslice>
    pub fn get_skx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,skx_ : &mut[i32]) -> Result<(),String> { self.task.get_skx_slice(whichsol_,first_,last_,skx_) }
    /// Obtains the slc vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `slc_` Dual variables corresponding to the lower bounds on the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getslc>
    pub fn get_slc(&self,whichsol_ : i32,slc_ : &mut[f64]) -> Result<(),String> { self.task.get_slc(whichsol_,slc_) }
    /// Obtains a slice of the slc vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `slc_` Dual variables corresponding to the lower bounds on the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getslcslice>
    pub fn get_slc_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,slc_ : &mut[f64]) -> Result<(),String> { self.task.get_slc_slice(whichsol_,first_,last_,slc_) }
    /// Obtains the slx vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `slx_` Dual variables corresponding to the lower bounds on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getslx>
    pub fn get_slx(&self,whichsol_ : i32,slx_ : &mut[f64]) -> Result<(),String> { self.task.get_slx(whichsol_,slx_) }
    /// Obtains a slice of the slx vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `slx_` Dual variables corresponding to the lower bounds on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getslxslice>
    pub fn get_slx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,slx_ : &mut[f64]) -> Result<(),String> { self.task.get_slx_slice(whichsol_,first_,last_,slx_) }
    /// Obtains the snx vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `snx_` Dual variables corresponding to the conic constraints on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsnx>
    pub fn get_snx(&self,whichsol_ : i32,snx_ : &mut[f64]) -> Result<(),String> { self.task.get_snx(whichsol_,snx_) }
    /// Obtains a slice of the snx vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `snx_` Dual variables corresponding to the conic constraints on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsnxslice>
    pub fn get_snx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,snx_ : &mut[f64]) -> Result<(),String> { self.task.get_snx_slice(whichsol_,first_,last_,snx_) }
    /// Obtains the solution status.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    ///
    /// # Returns
    ///
    ///   - `solutionsta` Solution status.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsolsta>
    pub fn get_sol_sta(&self,whichsol_ : i32) -> Result<i32,String> { self.task.get_sol_sta(whichsol_) }
    /// Obtains the complete solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `problemsta_` Problem status.
    ///   
    ///   See [Prosta]
    /// - `solutionsta_` Solution status.
    ///   
    ///   See [Solsta]
    /// - `skc_` Status keys for the constraints.
    ///   
    ///   See [Stakey]
    /// - `skx_` Status keys for the variables.
    ///   
    ///   See [Stakey]
    /// - `skn_` Status keys for the conic constraints.
    ///   
    ///   See [Stakey]
    /// - `xc_` Primal constraint solution.
    /// - `xx_` Primal variable solution.
    /// - `y_` Vector of dual variables corresponding to the constraints.
    /// - `slc_` Dual variables corresponding to the lower bounds on the constraints.
    /// - `suc_` Dual variables corresponding to the upper bounds on the constraints.
    /// - `slx_` Dual variables corresponding to the lower bounds on the variables.
    /// - `sux_` Dual variables corresponding to the upper bounds on the variables.
    /// - `snx_` Dual variables corresponding to the conic constraints on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsolution>
    pub fn get_solution(&self,whichsol_ : i32,problemsta_ : & mut i32,solutionsta_ : & mut i32,skc_ : &mut[i32],skx_ : &mut[i32],skn_ : &mut[i32],xc_ : &mut[f64],xx_ : &mut[f64],y_ : &mut[f64],slc_ : &mut[f64],suc_ : &mut[f64],slx_ : &mut[f64],sux_ : &mut[f64],snx_ : &mut[f64]) -> Result<(),String> { self.task.get_solution(whichsol_,problemsta_,solutionsta_,skc_,skx_,skn_,xc_,xx_,y_,slc_,suc_,slx_,sux_,snx_) }
    /// Obtains information about of a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `pobj_` The primal objective value.
    /// - `pviolcon_` Maximal primal bound violation for a xc variable.
    /// - `pviolvar_` Maximal primal bound violation for a xx variable.
    /// - `pviolbarvar_` Maximal primal bound violation for a barx variable.
    /// - `pviolcone_` Maximal primal violation of the solution with respect to the conic constraints.
    /// - `pviolitg_` Maximal violation in the integer constraints.
    /// - `dobj_` Dual objective value.
    /// - `dviolcon_` Maximal dual bound violation for a xc variable.
    /// - `dviolvar_` Maximal dual bound violation for a xx variable.
    /// - `dviolbarvar_` Maximal dual bound violation for a bars variable.
    /// - `dviolcone_` Maximum violation of the dual solution in the dual conic constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsolutioninfo>
    pub fn get_solution_info(&self,whichsol_ : i32,pobj_ : &mut f64,pviolcon_ : &mut f64,pviolvar_ : &mut f64,pviolbarvar_ : &mut f64,pviolcone_ : &mut f64,pviolitg_ : &mut f64,dobj_ : &mut f64,dviolcon_ : &mut f64,dviolvar_ : &mut f64,dviolbarvar_ : &mut f64,dviolcone_ : &mut f64) -> Result<(),String> { self.task.get_solution_info(whichsol_,pobj_,pviolcon_,pviolvar_,pviolbarvar_,pviolcone_,pviolitg_,dobj_,dviolcon_,dviolvar_,dviolbarvar_,dviolcone_) }
    /// Obtains information about of a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `pobj_` The primal objective value.
    /// - `pviolcon_` Maximal primal bound violation for a xc variable.
    /// - `pviolvar_` Maximal primal bound violation for a xx variable.
    /// - `pviolbarvar_` Maximal primal bound violation for a barx variable.
    /// - `pviolcone_` Maximal primal violation of the solution with respect to the conic constraints.
    /// - `pviolacc_` Maximal primal violation of the solution with respect to the affine conic constraints.
    /// - `pvioldjc_` Maximal primal violation of the solution with respect to the disjunctive constraints.
    /// - `pviolitg_` Maximal violation in the integer constraints.
    /// - `dobj_` Dual objective value.
    /// - `dviolcon_` Maximal dual bound violation for a xc variable.
    /// - `dviolvar_` Maximal dual bound violation for a xx variable.
    /// - `dviolbarvar_` Maximal dual bound violation for a bars variable.
    /// - `dviolcone_` Maximum violation of the dual solution in the dual conic constraints.
    /// - `dviolacc_` Maximum violation of the dual solution in the dual affine conic constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsolutioninfonew>
    pub fn get_solution_info_new(&self,whichsol_ : i32,pobj_ : &mut f64,pviolcon_ : &mut f64,pviolvar_ : &mut f64,pviolbarvar_ : &mut f64,pviolcone_ : &mut f64,pviolacc_ : &mut f64,pvioldjc_ : &mut f64,pviolitg_ : &mut f64,dobj_ : &mut f64,dviolcon_ : &mut f64,dviolvar_ : &mut f64,dviolbarvar_ : &mut f64,dviolcone_ : &mut f64,dviolacc_ : &mut f64) -> Result<(),String> { self.task.get_solution_info_new(whichsol_,pobj_,pviolcon_,pviolvar_,pviolbarvar_,pviolcone_,pviolacc_,pvioldjc_,pviolitg_,dobj_,dviolcon_,dviolvar_,dviolbarvar_,dviolcone_,dviolacc_) }
    /// Obtains the complete solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `problemsta_` Problem status.
    ///   
    ///   See [Prosta]
    /// - `solutionsta_` Solution status.
    ///   
    ///   See [Solsta]
    /// - `skc_` Status keys for the constraints.
    ///   
    ///   See [Stakey]
    /// - `skx_` Status keys for the variables.
    ///   
    ///   See [Stakey]
    /// - `skn_` Status keys for the conic constraints.
    ///   
    ///   See [Stakey]
    /// - `xc_` Primal constraint solution.
    /// - `xx_` Primal variable solution.
    /// - `y_` Vector of dual variables corresponding to the constraints.
    /// - `slc_` Dual variables corresponding to the lower bounds on the constraints.
    /// - `suc_` Dual variables corresponding to the upper bounds on the constraints.
    /// - `slx_` Dual variables corresponding to the lower bounds on the variables.
    /// - `sux_` Dual variables corresponding to the upper bounds on the variables.
    /// - `snx_` Dual variables corresponding to the conic constraints on the variables.
    /// - `doty_` Dual variables corresponding to affine conic constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsolutionnew>
    pub fn get_solution_new(&self,whichsol_ : i32,problemsta_ : & mut i32,solutionsta_ : & mut i32,skc_ : &mut[i32],skx_ : &mut[i32],skn_ : &mut[i32],xc_ : &mut[f64],xx_ : &mut[f64],y_ : &mut[f64],slc_ : &mut[f64],suc_ : &mut[f64],slx_ : &mut[f64],sux_ : &mut[f64],snx_ : &mut[f64],doty_ : &mut[f64]) -> Result<(),String> { self.task.get_solution_new(whichsol_,problemsta_,solutionsta_,skc_,skx_,skn_,xc_,xx_,y_,slc_,suc_,slx_,sux_,snx_,doty_) }
    /// Obtains a slice of the solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `solitem_` Which part of the solution is required.
    ///   
    ///   See [Solitem]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `values_` The values of the requested solution elements.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsolutionslice>
    pub fn get_solution_slice(&self,whichsol_ : i32,solitem_ : i32,first_ : i32,last_ : i32,values_ : &mut[f64]) -> Result<(),String> { self.task.get_solution_slice(whichsol_,solitem_,first_,last_,values_) }
    /// Gets a single symmetric matrix from the matrix store.
    ///
    /// # Arguments
    ///
    /// - `idx_` Index of the matrix to retrieve.
    /// - `subi_` Row subscripts of the matrix non-zero elements.
    /// - `subj_` Column subscripts of the matrix non-zero elements.
    /// - `valij_` Coefficients of the matrix non-zero elements.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsparsesymmat>
    pub fn get_sparse_sym_mat(&self,idx_ : i64,subi_ : &mut[i32],subj_ : &mut[i32],valij_ : &mut[f64]) -> Result<(),String> { self.task.get_sparse_sym_mat(idx_,subi_,subj_,valij_) }
    /// Obtains the value of a string parameter.
    ///
    /// # Arguments
    ///
    /// - `param_` Which parameter.
    ///   
    ///   See [Sparam]
    /// - `len_` The length of the parameter value.
    ///
    /// # Returns
    ///
    ///   - `parvalue` If this is not a null pointer, the parameter value is stored here.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getstrparam>
    pub fn get_str_param(&self,param_ : i32,len_ : &mut i32) -> Result<String,String> { self.task.get_str_param(param_,len_) }
    /// Obtains the length of a string parameter.
    ///
    /// # Arguments
    ///
    /// - `param_` Which parameter.
    ///   
    ///   See [Sparam]
    ///
    /// # Returns
    ///
    ///   - `len` The length of the parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getstrparamlen>
    pub fn get_str_param_len(&self,param_ : i32) -> Result<i32,String> { self.task.get_str_param_len(param_) }
    /// Obtains the suc vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `suc_` Dual variables corresponding to the upper bounds on the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsuc>
    pub fn get_suc(&self,whichsol_ : i32,suc_ : &mut[f64]) -> Result<(),String> { self.task.get_suc(whichsol_,suc_) }
    /// Obtains a slice of the suc vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `suc_` Dual variables corresponding to the upper bounds on the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsucslice>
    pub fn get_suc_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,suc_ : &mut[f64]) -> Result<(),String> { self.task.get_suc_slice(whichsol_,first_,last_,suc_) }
    /// Obtains the sux vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sux_` Dual variables corresponding to the upper bounds on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsux>
    pub fn get_sux(&self,whichsol_ : i32,sux_ : &mut[f64]) -> Result<(),String> { self.task.get_sux(whichsol_,sux_) }
    /// Obtains a slice of the sux vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `sux_` Dual variables corresponding to the upper bounds on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsuxslice>
    pub fn get_sux_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,sux_ : &mut[f64]) -> Result<(),String> { self.task.get_sux_slice(whichsol_,first_,last_,sux_) }
    /// Obtains a cone type string identifier.
    ///
    /// # Arguments
    ///
    /// - `i_` Index.
    /// - `value_` The corresponding value.
    ///
    /// # Returns
    ///
    ///   - `name` Name of the i'th symbolic constant.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsymbcon>
    pub fn get_symb_con(&self,i_ : i32,value_ : &mut i32) -> Result<String,String> { self.task.get_symb_con(i_,value_) }
    /// Obtains information about a matrix from the symmetric matrix storage.
    ///
    /// # Arguments
    ///
    /// - `idx_` Index of the matrix for which information is requested.
    /// - `dim_` Returns the dimension of the requested matrix.
    /// - `nz_` Returns the number of non-zeros in the requested matrix.
    /// - `mattype_` Returns the type of the requested matrix.
    ///   
    ///   See [Symmattype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsymmatinfo>
    pub fn get_sym_mat_info(&self,idx_ : i64,dim_ : &mut i32,nz_ : &mut i64,mattype_ : & mut i32) -> Result<(),String> { self.task.get_sym_mat_info(idx_,dim_,nz_,mattype_) }
    /// Obtains the task name.
    ///
    /// # Returns
    ///
    ///   - `taskname` Returns the task name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.gettaskname>
    pub fn get_task_name(&self) -> Result<String,String> { self.task.get_task_name() }
    /// Obtains the length the task name.
    ///
    /// # Returns
    ///
    ///   - `len` Returns the length of the task name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.gettasknamelen>
    pub fn get_task_name_len(&self) -> Result<i32,String> { self.task.get_task_name_len() }
    /// Obtains bound information for one variable.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the variable for which the bound information should be obtained.
    /// - `bk_` Bound keys.
    ///   
    ///   See [Boundkey]
    /// - `bl_` Values for lower bounds.
    /// - `bu_` Values for upper bounds.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getvarbound>
    pub fn get_var_bound(&self,i_ : i32,bk_ : & mut i32,bl_ : &mut f64,bu_ : &mut f64) -> Result<(),String> { self.task.get_var_bound(i_,bk_,bl_,bu_) }
    /// Obtains bounds information for a slice of the variables.
    ///
    /// # Arguments
    ///
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `bk_` Bound keys.
    ///   
    ///   See [Boundkey]
    /// - `bl_` Values for lower bounds.
    /// - `bu_` Values for upper bounds.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getvarboundslice>
    pub fn get_var_bound_slice(&self,first_ : i32,last_ : i32,bk_ : &mut[i32],bl_ : &mut[f64],bu_ : &mut[f64]) -> Result<(),String> { self.task.get_var_bound_slice(first_,last_,bk_,bl_,bu_) }
    /// Obtains the name of a variable.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of a variable.
    ///
    /// # Returns
    ///
    ///   - `name` Returns the required name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getvarname>
    pub fn get_var_name(&self,j_ : i32) -> Result<String,String> { self.task.get_var_name(j_) }
    /// Checks whether the name has been assigned to any variable.
    ///
    /// # Arguments
    ///
    /// - `somename_` The name which should be checked.
    /// - `asgn_` Is non-zero if the name somename is assigned to a variable.
    ///
    /// # Returns
    ///
    ///   - `index` If the name somename is assigned to a variable, then return the index of the variable.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getvarnameindex>
    pub fn get_var_name_index(&self,somename_ : &str,asgn_ : &mut i32) -> Result<i32,String> { self.task.get_var_name_index(somename_,asgn_) }
    /// Obtains the length of the name of a variable.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of a variable.
    ///
    /// # Returns
    ///
    ///   - `len` Returns the length of the indicated name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getvarnamelen>
    pub fn get_var_name_len(&self,i_ : i32) -> Result<i32,String> { self.task.get_var_name_len(i_) }
    /// Gets the variable type of one variable.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the variable.
    ///
    /// # Returns
    ///
    ///   - `vartype` Variable type of variable index j.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getvartype>
    pub fn get_var_type(&self,j_ : i32) -> Result<i32,String> { self.task.get_var_type(j_) }
    /// Obtains the variable type for one or more variables.
    ///
    /// # Arguments
    ///
    /// - `subj_` A list of variable indexes.
    /// - `vartype_` Returns the variables types corresponding the variable indexes requested.
    ///   
    ///   See [Variabletype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getvartypelist>
    pub fn get_var_type_list(&self,subj_ : &[i32],vartype_ : &mut[i32]) -> Result<(),String> { self.task.get_var_type_list(subj_,vartype_) }
    /// Obtains the xc vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `xc_` Primal constraint solution.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getxc>
    pub fn get_xc(&self,whichsol_ : i32,xc_ : &mut[f64]) -> Result<(),String> { self.task.get_xc(whichsol_,xc_) }
    /// Obtains a slice of the xc vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `xc_` Primal constraint solution.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getxcslice>
    pub fn get_xc_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,xc_ : &mut[f64]) -> Result<(),String> { self.task.get_xc_slice(whichsol_,first_,last_,xc_) }
    /// Obtains the xx vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `xx_` Primal variable solution.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getxx>
    pub fn get_xx(&self,whichsol_ : i32,xx_ : &mut[f64]) -> Result<(),String> { self.task.get_xx(whichsol_,xx_) }
    /// Obtains a slice of the xx vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `xx_` Primal variable solution.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getxxslice>
    pub fn get_xx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,xx_ : &mut[f64]) -> Result<(),String> { self.task.get_xx_slice(whichsol_,first_,last_,xx_) }
    /// Obtains the y vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `y_` Vector of dual variables corresponding to the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.gety>
    pub fn get_y(&self,whichsol_ : i32,y_ : &mut[f64]) -> Result<(),String> { self.task.get_y(whichsol_,y_) }
    /// Obtains a slice of the y vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `y_` Vector of dual variables corresponding to the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getyslice>
    pub fn get_y_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,y_ : &mut[f64]) -> Result<(),String> { self.task.get_y_slice(whichsol_,first_,last_,y_) }
    /// Prints the infeasibility report to an output stream.
    ///
    /// # Arguments
    ///
    /// - `whichstream_` Index of the stream.
    ///   
    ///   See [Streamtype]
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.infeasibilityreport>
    pub fn infeasibility_report(&mut self,whichstream_ : i32,whichsol_ : i32) -> Result<(),String> { self.task.infeasibility_report(whichstream_,whichsol_) }
    /// Prepare a task for basis solver.
    ///
    /// # Arguments
    ///
    /// - `basis_` The array of basis indexes to use.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.initbasissolve>
    pub fn init_basis_solve(&mut self,basis_ : &mut[i32]) -> Result<(),String> { self.task.init_basis_solve(basis_) }
    /// Input the linear part of an optimization task in one function call.
    ///
    /// # Arguments
    ///
    /// - `maxnumcon_` Number of preallocated constraints in the optimization task.
    /// - `maxnumvar_` Number of preallocated variables in the optimization task.
    /// - `c_` Linear terms of the objective as a dense vector. The length is the number of variables.
    /// - `cfix_` Fixed term in the objective.
    /// - `aptrb_` Row or column start pointers.
    /// - `aptre_` Row or column end pointers.
    /// - `asub_` Coefficient subscripts.
    /// - `aval_` Coefficient values.
    /// - `bkc_` Bound keys for the constraints.
    ///   
    ///   See [Boundkey]
    /// - `blc_` Lower bounds for the constraints.
    /// - `buc_` Upper bounds for the constraints.
    /// - `bkx_` Bound keys for the variables.
    ///   
    ///   See [Boundkey]
    /// - `blx_` Lower bounds for the variables.
    /// - `bux_` Upper bounds for the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.inputdata64>
    pub fn input_data(&mut self,maxnumcon_ : i32,maxnumvar_ : i32,c_ : &[f64],cfix_ : f64,aptrb_ : &[i64],aptre_ : &[i64],asub_ : &[i32],aval_ : &[f64],bkc_ : &[i32],blc_ : &[f64],buc_ : &[f64],bkx_ : &[i32],blx_ : &[f64],bux_ : &[f64]) -> Result<(),String> { self.task.input_data(maxnumcon_,maxnumvar_,c_,cfix_,aptrb_,aptre_,asub_,aval_,bkc_,blc_,buc_,bkx_,blx_,bux_) }
    /// Checks a double parameter name.
    ///
    /// # Arguments
    ///
    /// - `parname_` Parameter name.
    /// - `param_` Returns the parameter corresponding to the name, if one exists.
    ///   
    ///   See [Dparam]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.isdouparname>
    pub fn is_dou_par_name(&self,parname_ : &str,param_ : & mut i32) -> Result<(),String> { self.task.is_dou_par_name(parname_,param_) }
    /// Checks an integer parameter name.
    ///
    /// # Arguments
    ///
    /// - `parname_` Parameter name.
    /// - `param_` Returns the parameter corresponding to the name, if one exists.
    ///   
    ///   See [Iparam]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.isintparname>
    pub fn is_int_par_name(&self,parname_ : &str,param_ : & mut i32) -> Result<(),String> { self.task.is_int_par_name(parname_,param_) }
    /// Checks a string parameter name.
    ///
    /// # Arguments
    ///
    /// - `parname_` Parameter name.
    /// - `param_` Returns the parameter corresponding to the name, if one exists.
    ///   
    ///   See [Sparam]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.isstrparname>
    pub fn is_str_par_name(&self,parname_ : &str,param_ : & mut i32) -> Result<(),String> { self.task.is_str_par_name(parname_,param_) }
    /// Directs all output from a task stream to a file.
    ///
    /// # Arguments
    ///
    /// - `whichstream_` Index of the stream.
    ///   
    ///   See [Streamtype]
    /// - `filename_` A valid file name.
    /// - `append_` If this argument is 0 the output file will be overwritten, otherwise it will be appended to.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.linkfiletotaskstream>
    pub fn link_file_to_stream(&mut self,whichstream_ : i32,filename_ : &str,append_ : i32) -> Result<(),String> { self.task.link_file_to_stream(whichstream_,filename_,append_) }
    /// Prints a short summary of a specified solution.
    ///
    /// # Arguments
    ///
    /// - `whichstream_` Index of the stream.
    ///   
    ///   See [Streamtype]
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.onesolutionsummary>
    pub fn one_solution_summary(&self,whichstream_ : i32,whichsol_ : i32) -> Result<(),String> { self.task.one_solution_summary(whichstream_,whichsol_) }
    /// Offload the optimization task to a solver server and wait for the solution.
    ///
    /// # Arguments
    ///
    /// - `address_` Address of the OptServer.
    /// - `accesstoken_` Access token.
    /// - `trmcode_` Is either OK or a termination response code.
    ///   
    ///   See [Rescode]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.optimizermt>
    pub fn optimize_rmt(&mut self,address_ : &str,accesstoken_ : &str,trmcode_ : & mut i32) -> Result<(),String> { self.task.optimize_rmt(address_,accesstoken_,trmcode_) }
    /// Prints a short summary with optimizer statistics from last optimization.
    ///
    /// # Arguments
    ///
    /// - `whichstream_` Index of the stream.
    ///   
    ///   See [Streamtype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.optimizersummary>
    pub fn optimizer_summary(&self,whichstream_ : i32) -> Result<(),String> { self.task.optimizer_summary(whichstream_) }
    /// Optimizes the problem.
    ///
    /// # Returns
    ///
    ///   - `trmcode` Is either OK or a termination response code.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.optimizetrm>
    pub fn optimize(&mut self) -> Result<i32,String> { self.task.optimize() }
    /// Repairs a primal infeasible optimization problem by adjusting the bounds on the constraints and variables.
    ///
    /// # Arguments
    ///
    /// - `wlc_` Weights associated with relaxing lower bounds on the constraints.
    /// - `wuc_` Weights associated with relaxing the upper bound on the constraints.
    /// - `wlx_` Weights associated with relaxing the lower bounds of the variables.
    /// - `wux_` Weights associated with relaxing the upper bounds of variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.primalrepair>
    pub fn primal_repair(&mut self,wlc_ : &[f64],wuc_ : &[f64],wlx_ : &[f64],wux_ : &[f64]) -> Result<(),String> { self.task.primal_repair(wlc_,wuc_,wlx_,wux_) }
    /// Perform sensitivity analysis on bounds.
    ///
    /// # Arguments
    ///
    /// - `subi_` Indexes of constraints to analyze.
    /// - `marki_` Mark which constraint bounds to analyze.
    ///   
    ///   See [Mark]
    /// - `subj_` Indexes of variables to analyze.
    /// - `markj_` Mark which variable bounds to analyze.
    ///   
    ///   See [Mark]
    /// - `leftpricei_` Left shadow price for constraints.
    /// - `rightpricei_` Right shadow price for constraints.
    /// - `leftrangei_` Left range for constraints.
    /// - `rightrangei_` Right range for constraints.
    /// - `leftpricej_` Left shadow price for variables.
    /// - `rightpricej_` Right shadow price for variables.
    /// - `leftrangej_` Left range for variables.
    /// - `rightrangej_` Right range for variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.primalsensitivity>
    pub fn primal_sensitivity(&mut self,subi_ : &[i32],marki_ : &[i32],subj_ : &[i32],markj_ : &[i32],leftpricei_ : &mut[f64],rightpricei_ : &mut[f64],leftrangei_ : &mut[f64],rightrangei_ : &mut[f64],leftpricej_ : &mut[f64],rightpricej_ : &mut[f64],leftrangej_ : &mut[f64],rightrangej_ : &mut[f64]) -> Result<(),String> { self.task.primal_sensitivity(subi_,marki_,subj_,markj_,leftpricei_,rightpricei_,leftrangei_,rightrangei_,leftpricej_,rightpricej_,leftrangej_,rightrangej_) }
    /// Prints the current parameter settings.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.printparam>
    pub fn print_param(&self) -> Result<(),String> { self.task.print_param() }
    /// Puts an affine conic constraint.
    ///
    /// # Arguments
    ///
    /// - `accidx_` Affine conic constraint index.
    /// - `domidx_` Domain index.
    /// - `afeidxlist_` List of affine expression indexes.
    /// - `b_` The vector of constant terms added to affine expressions. Optional, can be NULL.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putacc>
    pub fn put_acc(&mut self,accidx_ : i64,domidx_ : i64,afeidxlist_ : &[i64],b_ : &[f64]) -> Result<(),String> { self.task.put_acc(accidx_,domidx_,afeidxlist_,b_) }
    /// Puts the constant vector b in an affine conic constraint.
    ///
    /// # Arguments
    ///
    /// - `accidx_` Affine conic constraint index.
    /// - `b_` The vector of constant terms added to affine expressions. Optional, can be NULL.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putaccb>
    pub fn put_acc_b(&mut self,accidx_ : i64,b_ : &[f64]) -> Result<(),String> { self.task.put_acc_b(accidx_,b_) }
    /// Sets one element in the b vector of an affine conic constraint.
    ///
    /// # Arguments
    ///
    /// - `accidx_` Affine conic constraint index.
    /// - `j_` The index of an element in b to change.
    /// - `bj_` The new value of b\[j\].
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putaccbj>
    pub fn put_acc_b_j(&mut self,accidx_ : i64,j_ : i64,bj_ : f64) -> Result<(),String> { self.task.put_acc_b_j(accidx_,j_,bj_) }
    /// Puts the doty vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `accidx_` The index of the affine conic constraint.
    /// - `doty_` The dual values for this affine conic constraint. The array should have length equal to the dimension of the constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putaccdoty>
    pub fn put_acc_dot_y(&self,whichsol_ : i32,accidx_ : i64,doty_ : &mut[f64]) -> Result<(),String> { self.task.put_acc_dot_y(whichsol_,accidx_,doty_) }
    /// Puts a number of affine conic constraints.
    ///
    /// # Arguments
    ///
    /// - `accidxs_` Affine conic constraint indices.
    /// - `domidxs_` Domain indices.
    /// - `afeidxlist_` List of affine expression indexes.
    /// - `b_` The vector of constant terms added to affine expressions. Optional, can be NULL.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putacclist>
    pub fn put_acc_list(&mut self,accidxs_ : &[i64],domidxs_ : &[i64],afeidxlist_ : &[i64],b_ : &[f64]) -> Result<(),String> { self.task.put_acc_list(accidxs_,domidxs_,afeidxlist_,b_) }
    /// Sets the name of an affine conic constraint.
    ///
    /// # Arguments
    ///
    /// - `accidx_` Index of the affine conic constraint.
    /// - `name_` The name of the affine conic constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putaccname>
    pub fn put_acc_name(&mut self,accidx_ : i64,name_ : &str) -> Result<(),String> { self.task.put_acc_name(accidx_,name_) }
    /// Replaces all elements in one column of the linear constraint matrix.
    ///
    /// # Arguments
    ///
    /// - `j_` Column index.
    /// - `subj_` Row indexes of non-zero values in column.
    /// - `valj_` New non-zero values of column.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putacol>
    pub fn put_a_col(&mut self,j_ : i32,subj_ : &[i32],valj_ : &[f64]) -> Result<(),String> { self.task.put_a_col(j_,subj_,valj_) }
    /// Replaces all elements in several columns the linear constraint matrix.
    ///
    /// # Arguments
    ///
    /// - `sub_` Indexes of columns that should be replaced.
    /// - `ptrb_` Array of pointers to the first element in the columns.
    /// - `ptre_` Array of pointers to the last element plus one in the columns.
    /// - `asub_` Row indexes
    /// - `aval_` Coefficient values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putacollist64>
    pub fn put_a_col_list(&mut self,sub_ : &[i32],ptrb_ : &[i64],ptre_ : &[i64],asub_ : &[i32],aval_ : &[f64]) -> Result<(),String> { self.task.put_a_col_list(sub_,ptrb_,ptre_,asub_,aval_) }
    /// Replaces all elements in a sequence of columns the linear constraint matrix.
    ///
    /// # Arguments
    ///
    /// - `first_` First column in the slice.
    /// - `last_` Last column plus one in the slice.
    /// - `ptrb_` Array of pointers to the first element in the columns.
    /// - `ptre_` Array of pointers to the last element plus one in the columns.
    /// - `asub_` Row indexes
    /// - `aval_` Coefficient values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putacolslice64>
    pub fn put_a_col_slice(&mut self,first_ : i32,last_ : i32,ptrb_ : &[i64],ptre_ : &[i64],asub_ : &[i32],aval_ : &[f64]) -> Result<(),String> { self.task.put_a_col_slice(first_,last_,ptrb_,ptre_,asub_,aval_) }
    /// Inputs barF in block triplet form.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Constraint index.
    /// - `barvaridx_` Symmetric matrix variable index.
    /// - `subk_` Block row index.
    /// - `subl_` Block column index.
    /// - `valkl_` The numerical value associated with each block triplet.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putafebarfblocktriplet>
    pub fn put_afe_barf_block_triplet(&mut self,afeidx_ : &[i64],barvaridx_ : &[i32],subk_ : &[i32],subl_ : &[i32],valkl_ : &[f64]) -> Result<(),String> { self.task.put_afe_barf_block_triplet(afeidx_,barvaridx_,subk_,subl_,valkl_) }
    /// Inputs one entry in barF.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row index of barF.
    /// - `barvaridx_` Semidefinite variable index.
    /// - `termidx_` Element indices in matrix storage.
    /// - `termweight_` Weights in the weighted sum.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putafebarfentry>
    pub fn put_afe_barf_entry(&mut self,afeidx_ : i64,barvaridx_ : i32,termidx_ : &[i64],termweight_ : &[f64]) -> Result<(),String> { self.task.put_afe_barf_entry(afeidx_,barvaridx_,termidx_,termweight_) }
    /// Inputs a list of entries in barF.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row indexes of barF.
    /// - `barvaridx_` Semidefinite variable indexes.
    /// - `numterm_` Number of terms in the weighted sums.
    /// - `ptrterm_` Pointer to the terms forming each entry.
    /// - `termidx_` Concatenated element indexes in matrix storage.
    /// - `termweight_` Concatenated weights in the weighted sum.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putafebarfentrylist>
    pub fn put_afe_barf_entry_list(&mut self,afeidx_ : &[i64],barvaridx_ : &[i32],numterm_ : &[i64],ptrterm_ : &[i64],termidx_ : &[i64],termweight_ : &[f64]) -> Result<(),String> { self.task.put_afe_barf_entry_list(afeidx_,barvaridx_,numterm_,ptrterm_,termidx_,termweight_) }
    /// Inputs a row of barF.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row index of barF.
    /// - `barvaridx_` Semidefinite variable indexes.
    /// - `numterm_` Number of terms in the weighted sums.
    /// - `ptrterm_` Pointer to the terms forming each entry.
    /// - `termidx_` Concatenated element indexes in matrix storage.
    /// - `termweight_` Concatenated weights in the weighted sum.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putafebarfrow>
    pub fn put_afe_barf_row(&mut self,afeidx_ : i64,barvaridx_ : &[i32],numterm_ : &[i64],ptrterm_ : &[i64],termidx_ : &[i64],termweight_ : &[f64]) -> Result<(),String> { self.task.put_afe_barf_row(afeidx_,barvaridx_,numterm_,ptrterm_,termidx_,termweight_) }
    /// Replaces all elements in one column of the F matrix in the affine expressions.
    ///
    /// # Arguments
    ///
    /// - `varidx_` Column index.
    /// - `afeidx_` Row indexes of non-zero values in the column.
    /// - `val_` New non-zero values in the column.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putafefcol>
    pub fn put_afe_f_col(&mut self,varidx_ : i32,afeidx_ : &[i64],val_ : &[f64]) -> Result<(),String> { self.task.put_afe_f_col(varidx_,afeidx_,val_) }
    /// Replaces one entry in F.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row index in F.
    /// - `varidx_` Column index in F.
    /// - `value_` Value of the entry.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putafefentry>
    pub fn put_afe_f_entry(&mut self,afeidx_ : i64,varidx_ : i32,value_ : f64) -> Result<(),String> { self.task.put_afe_f_entry(afeidx_,varidx_,value_) }
    /// Replaces a list of entries in F.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row indices in F.
    /// - `varidx_` Column indices in F.
    /// - `val_` Values of the entries in F.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putafefentrylist>
    pub fn put_afe_f_entry_list(&mut self,afeidx_ : &[i64],varidx_ : &[i32],val_ : &[f64]) -> Result<(),String> { self.task.put_afe_f_entry_list(afeidx_,varidx_,val_) }
    /// Replaces all elements in one row of the F matrix in the affine expressions.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row index.
    /// - `varidx_` Column indexes of non-zero values in the row.
    /// - `val_` New non-zero values in the row.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putafefrow>
    pub fn put_afe_f_row(&mut self,afeidx_ : i64,varidx_ : &[i32],val_ : &[f64]) -> Result<(),String> { self.task.put_afe_f_row(afeidx_,varidx_,val_) }
    /// Replaces all elements in a number of rows of the F matrix in the affine expressions.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row indices.
    /// - `numnzrow_` Number of non-zeros in each row.
    /// - `ptrrow_` Pointer to the first nonzero in each row.
    /// - `varidx_` Column indexes of non-zero values.
    /// - `val_` New non-zero values in the rows.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putafefrowlist>
    pub fn put_afe_f_row_list(&mut self,afeidx_ : &[i64],numnzrow_ : &[i32],ptrrow_ : &[i64],varidx_ : &[i32],val_ : &[f64]) -> Result<(),String> { self.task.put_afe_f_row_list(afeidx_,numnzrow_,ptrrow_,varidx_,val_) }
    /// Replaces one element in the g vector in the affine expressions.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row index.
    /// - `g_` New value for the element of g.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putafeg>
    pub fn put_afe_g(&mut self,afeidx_ : i64,g_ : f64) -> Result<(),String> { self.task.put_afe_g(afeidx_,g_) }
    /// Replaces a list of elements in the g vector in the affine expressions.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Indices of entries in g.
    /// - `g_` New values for the elements of g.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putafeglist>
    pub fn put_afe_g_list(&mut self,afeidx_ : &[i64],g_ : &[f64]) -> Result<(),String> { self.task.put_afe_g_list(afeidx_,g_) }
    /// Modifies a slice of the vector g.
    ///
    /// # Arguments
    ///
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `slice_` The slice of g as a dense vector.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putafegslice>
    pub fn put_afe_g_slice(&mut self,first_ : i64,last_ : i64,slice_ : &[f64]) -> Result<(),String> { self.task.put_afe_g_slice(first_,last_,slice_) }
    /// Changes a single value in the linear coefficient matrix.
    ///
    /// # Arguments
    ///
    /// - `i_` Constraint (row) index.
    /// - `j_` Variable (column) index.
    /// - `aij_` New coefficient.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putaij>
    pub fn put_aij(&mut self,i_ : i32,j_ : i32,aij_ : f64) -> Result<(),String> { self.task.put_aij(i_,j_,aij_) }
    /// Changes one or more coefficients in the linear constraint matrix.
    ///
    /// # Arguments
    ///
    /// - `subi_` Constraint (row) indices.
    /// - `subj_` Variable (column) indices.
    /// - `valij_` New coefficient values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putaijlist64>
    pub fn put_aij_list(&mut self,subi_ : &[i32],subj_ : &[i32],valij_ : &[f64]) -> Result<(),String> { self.task.put_aij_list(subi_,subj_,valij_) }
    /// Replaces all elements in one row of the linear constraint matrix.
    ///
    /// # Arguments
    ///
    /// - `i_` Row index.
    /// - `subi_` Column indexes of non-zero values in row.
    /// - `vali_` New non-zero values of row.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putarow>
    pub fn put_a_row(&mut self,i_ : i32,subi_ : &[i32],vali_ : &[f64]) -> Result<(),String> { self.task.put_a_row(i_,subi_,vali_) }
    /// Replaces all elements in several rows of the linear constraint matrix.
    ///
    /// # Arguments
    ///
    /// - `sub_` Indexes of rows or columns that should be replaced.
    /// - `ptrb_` Array of pointers to the first element in the rows.
    /// - `ptre_` Array of pointers to the last element plus one in the rows.
    /// - `asub_` Variable indexes.
    /// - `aval_` Coefficient values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putarowlist64>
    pub fn put_a_row_list(&mut self,sub_ : &[i32],ptrb_ : &[i64],ptre_ : &[i64],asub_ : &[i32],aval_ : &[f64]) -> Result<(),String> { self.task.put_a_row_list(sub_,ptrb_,ptre_,asub_,aval_) }
    /// Replaces all elements in several rows the linear constraint matrix.
    ///
    /// # Arguments
    ///
    /// - `first_` First row in the slice.
    /// - `last_` Last row plus one in the slice.
    /// - `ptrb_` Array of pointers to the first element in the rows.
    /// - `ptre_` Array of pointers to the last element plus one in the rows.
    /// - `asub_` Column indexes of new elements.
    /// - `aval_` Coefficient values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putarowslice64>
    pub fn put_a_row_slice(&mut self,first_ : i32,last_ : i32,ptrb_ : &[i64],ptre_ : &[i64],asub_ : &[i32],aval_ : &[f64]) -> Result<(),String> { self.task.put_a_row_slice(first_,last_,ptrb_,ptre_,asub_,aval_) }
    /// Truncates all elements in A below a certain tolerance to zero.
    ///
    /// # Arguments
    ///
    /// - `tolzero_` Truncation tolerance.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putatruncatetol>
    pub fn put_a_truncate_tol(&mut self,tolzero_ : f64) -> Result<(),String> { self.task.put_a_truncate_tol(tolzero_) }
    /// Inputs barA in block triplet form.
    ///
    /// # Arguments
    ///
    /// - `subi_` Constraint index.
    /// - `subj_` Symmetric matrix variable index.
    /// - `subk_` Block row index.
    /// - `subl_` Block column index.
    /// - `valijkl_` The numerical value associated with each block triplet.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putbarablocktriplet>
    pub fn put_bara_block_triplet(&mut self,subi_ : &[i32],subj_ : &[i32],subk_ : &[i32],subl_ : &[i32],valijkl_ : &[f64]) -> Result<(),String> { self.task.put_bara_block_triplet(subi_,subj_,subk_,subl_,valijkl_) }
    /// Inputs an element of barA.
    ///
    /// # Arguments
    ///
    /// - `i_` Row index of barA.
    /// - `j_` Column index of barA.
    /// - `sub_` Element indexes in matrix storage.
    /// - `weights_` Weights in the weighted sum.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putbaraij>
    pub fn put_bara_ij(&mut self,i_ : i32,j_ : i32,sub_ : &[i64],weights_ : &[f64]) -> Result<(),String> { self.task.put_bara_ij(i_,j_,sub_,weights_) }
    /// Inputs list of elements of barA.
    ///
    /// # Arguments
    ///
    /// - `subi_` Row index of barA.
    /// - `subj_` Column index of barA.
    /// - `alphaptrb_` Start entries for terms in the weighted sum.
    /// - `alphaptre_` End entries for terms in the weighted sum.
    /// - `matidx_` Element indexes in matrix storage.
    /// - `weights_` Weights in the weighted sum.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putbaraijlist>
    pub fn put_bara_ij_list(&mut self,subi_ : &[i32],subj_ : &[i32],alphaptrb_ : &[i64],alphaptre_ : &[i64],matidx_ : &[i64],weights_ : &[f64]) -> Result<(),String> { self.task.put_bara_ij_list(subi_,subj_,alphaptrb_,alphaptre_,matidx_,weights_) }
    /// Replace a set of rows of barA
    ///
    /// # Arguments
    ///
    /// - `subi_` Row indexes of barA.
    /// - `ptrb_` Start of rows in barA.
    /// - `ptre_` End of rows in barA.
    /// - `subj_` Column index of barA.
    /// - `nummat_` Number of entries in weighted sum of matrixes.
    /// - `matidx_` Matrix indexes for weighted sum of matrixes.
    /// - `weights_` Weights for weighted sum of matrixes.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putbararowlist>
    pub fn put_bara_row_list(&mut self,subi_ : &[i32],ptrb_ : &[i64],ptre_ : &[i64],subj_ : &[i32],nummat_ : &[i64],matidx_ : &[i64],weights_ : &[f64]) -> Result<(),String> { self.task.put_bara_row_list(subi_,ptrb_,ptre_,subj_,nummat_,matidx_,weights_) }
    /// Inputs barC in block triplet form.
    ///
    /// # Arguments
    ///
    /// - `subj_` Symmetric matrix variable index.
    /// - `subk_` Block row index.
    /// - `subl_` Block column index.
    /// - `valjkl_` The numerical value associated with each block triplet.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putbarcblocktriplet>
    pub fn put_barc_block_triplet(&mut self,subj_ : &[i32],subk_ : &[i32],subl_ : &[i32],valjkl_ : &[f64]) -> Result<(),String> { self.task.put_barc_block_triplet(subj_,subk_,subl_,valjkl_) }
    /// Changes one element in barc.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the element in barc` that should be changed.
    /// - `sub_` sub is list of indexes of those symmetric matrices appearing in sum.
    /// - `weights_` The weights of the terms in the weighted sum.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putbarcj>
    pub fn put_barc_j(&mut self,j_ : i32,sub_ : &[i64],weights_ : &[f64]) -> Result<(),String> { self.task.put_barc_j(j_,sub_,weights_) }
    /// Sets the dual solution for a semidefinite variable.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `j_` Index of the semidefinite variable.
    /// - `barsj_` Value of the j'th variable of barx.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putbarsj>
    pub fn put_bars_j(&mut self,whichsol_ : i32,j_ : i32,barsj_ : &[f64]) -> Result<(),String> { self.task.put_bars_j(whichsol_,j_,barsj_) }
    /// Sets the name of a semidefinite variable.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the variable.
    /// - `name_` The variable name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putbarvarname>
    pub fn put_barvar_name(&mut self,j_ : i32,name_ : &str) -> Result<(),String> { self.task.put_barvar_name(j_,name_) }
    /// Sets the primal solution for a semidefinite variable.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `j_` Index of the semidefinite variable.
    /// - `barxj_` Value of the j'th variable of barx.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putbarxj>
    pub fn put_barx_j(&mut self,whichsol_ : i32,j_ : i32,barxj_ : &[f64]) -> Result<(),String> { self.task.put_barx_j(whichsol_,j_,barxj_) }
    /// Replaces the fixed term in the objective.
    ///
    /// # Arguments
    ///
    /// - `cfix_` Fixed term in the objective.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putcfix>
    pub fn put_cfix(&mut self,cfix_ : f64) -> Result<(),String> { self.task.put_cfix(cfix_) }
    /// Modifies one linear coefficient in the objective.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the variable whose objective coefficient should be changed.
    /// - `cj_` New coefficient value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putcj>
    pub fn put_c_j(&mut self,j_ : i32,cj_ : f64) -> Result<(),String> { self.task.put_c_j(j_,cj_) }
    /// Modifies a part of the linear objective coefficients.
    ///
    /// # Arguments
    ///
    /// - `subj_` Indices of variables for which objective coefficients should be changed.
    /// - `val_` New numerical values for the objective coefficients that should be modified.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putclist>
    pub fn put_c_list(&mut self,subj_ : &[i32],val_ : &[f64]) -> Result<(),String> { self.task.put_c_list(subj_,val_) }
    /// Changes the bound for one constraint.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the constraint.
    /// - `bkc_` New bound key.
    ///   
    ///   See [Boundkey]
    /// - `blc_` New lower bound.
    /// - `buc_` New upper bound.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putconbound>
    pub fn put_con_bound(&mut self,i_ : i32,bkc_ : i32,blc_ : f64,buc_ : f64) -> Result<(),String> { self.task.put_con_bound(i_,bkc_,blc_,buc_) }
    /// Changes the bounds of a list of constraints.
    ///
    /// # Arguments
    ///
    /// - `sub_` List of constraint indexes.
    /// - `bkc_` Bound keys for the constraints.
    ///   
    ///   See [Boundkey]
    /// - `blc_` Lower bounds for the constraints.
    /// - `buc_` Upper bounds for the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putconboundlist>
    pub fn put_con_bound_list(&mut self,sub_ : &[i32],bkc_ : &[i32],blc_ : &[f64],buc_ : &[f64]) -> Result<(),String> { self.task.put_con_bound_list(sub_,bkc_,blc_,buc_) }
    /// Changes the bounds of a list of constraints.
    ///
    /// # Arguments
    ///
    /// - `sub_` List of constraint indexes.
    /// - `bkc_` New bound key for all constraints in the list.
    ///   
    ///   See [Boundkey]
    /// - `blc_` New lower bound for all constraints in the list.
    /// - `buc_` New upper bound for all constraints in the list.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putconboundlistconst>
    pub fn put_con_bound_list_const(&mut self,sub_ : &[i32],bkc_ : i32,blc_ : f64,buc_ : f64) -> Result<(),String> { self.task.put_con_bound_list_const(sub_,bkc_,blc_,buc_) }
    /// Changes the bounds for a slice of the constraints.
    ///
    /// # Arguments
    ///
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `bkc_` Bound keys for the constraints.
    ///   
    ///   See [Boundkey]
    /// - `blc_` Lower bounds for the constraints.
    /// - `buc_` Upper bounds for the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putconboundslice>
    pub fn put_con_bound_slice(&mut self,first_ : i32,last_ : i32,bkc_ : &[i32],blc_ : &[f64],buc_ : &[f64]) -> Result<(),String> { self.task.put_con_bound_slice(first_,last_,bkc_,blc_,buc_) }
    /// Changes the bounds for a slice of the constraints.
    ///
    /// # Arguments
    ///
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `bkc_` New bound key for all constraints in the slice.
    ///   
    ///   See [Boundkey]
    /// - `blc_` New lower bound for all constraints in the slice.
    /// - `buc_` New upper bound for all constraints in the slice.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putconboundsliceconst>
    pub fn put_con_bound_slice_const(&mut self,first_ : i32,last_ : i32,bkc_ : i32,blc_ : f64,buc_ : f64) -> Result<(),String> { self.task.put_con_bound_slice_const(first_,last_,bkc_,blc_,buc_) }
    /// Replaces a conic constraint.
    ///
    /// # Arguments
    ///
    /// - `k_` Index of the cone.
    /// - `ct_` Specifies the type of the cone.
    ///   
    ///   See [Conetype]
    /// - `conepar_` For the power cone it denotes the exponent alpha. For other cone types it is unused and can be set to 0.
    /// - `submem_` Variable subscripts of the members in the cone.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putcone>
    pub fn put_cone(&mut self,k_ : i32,ct_ : i32,conepar_ : f64,submem_ : &[i32]) -> Result<(),String> { self.task.put_cone(k_,ct_,conepar_,submem_) }
    /// Sets the name of a cone.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the cone.
    /// - `name_` The name of the cone.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putconename>
    pub fn put_cone_name(&mut self,j_ : i32,name_ : &str) -> Result<(),String> { self.task.put_cone_name(j_,name_) }
    /// Sets the name of a constraint.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the constraint.
    /// - `name_` The name of the constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putconname>
    pub fn put_con_name(&mut self,i_ : i32,name_ : &str) -> Result<(),String> { self.task.put_con_name(i_,name_) }
    /// Sets the primal and dual solution information for a single constraint.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the constraint.
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sk_` Status key of the constraint.
    ///   
    ///   See [Stakey]
    /// - `x_` Primal solution value of the constraint.
    /// - `sl_` Solution value of the dual variable associated with the lower bound.
    /// - `su_` Solution value of the dual variable associated with the upper bound.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putconsolutioni>
    pub fn put_con_solution_i(&mut self,i_ : i32,whichsol_ : i32,sk_ : i32,x_ : f64,sl_ : f64,su_ : f64) -> Result<(),String> { self.task.put_con_solution_i(i_,whichsol_,sk_,x_,sl_,su_) }
    /// Modifies a slice of the linear objective coefficients.
    ///
    /// # Arguments
    ///
    /// - `first_` First element in the slice of c.
    /// - `last_` Last element plus 1 of the slice in c to be changed.
    /// - `slice_` New numerical values for the objective coefficients that should be modified.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putcslice>
    pub fn put_c_slice(&mut self,first_ : i32,last_ : i32,slice_ : &[f64]) -> Result<(),String> { self.task.put_c_slice(first_,last_,slice_) }
    /// Inputs a disjunctive constraint.
    ///
    /// # Arguments
    ///
    /// - `djcidx_` Index of the disjunctive constraint.
    /// - `domidxlist_` List of domain indexes.
    /// - `afeidxlist_` List of affine expression indexes.
    /// - `b_` The vector of constant terms added to affine expressions.
    /// - `termsizelist_` List of term sizes.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putdjc>
    pub fn put_djc(&mut self,djcidx_ : i64,domidxlist_ : &[i64],afeidxlist_ : &[i64],b_ : &[f64],termsizelist_ : &[i64]) -> Result<(),String> { self.task.put_djc(djcidx_,domidxlist_,afeidxlist_,b_,termsizelist_) }
    /// Sets the name of a disjunctive constraint.
    ///
    /// # Arguments
    ///
    /// - `djcidx_` Index of the disjunctive constraint.
    /// - `name_` The name of the disjunctive constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putdjcname>
    pub fn put_djc_name(&mut self,djcidx_ : i64,name_ : &str) -> Result<(),String> { self.task.put_djc_name(djcidx_,name_) }
    /// Inputs a slice of disjunctive constraints.
    ///
    /// # Arguments
    ///
    /// - `idxfirst_` Index of the first disjunctive constraint in the slice.
    /// - `idxlast_` Index of the last disjunctive constraint in the slice plus 1.
    /// - `domidxlist_` List of domain indexes.
    /// - `afeidxlist_` List of affine expression indexes.
    /// - `b_` The vector of constant terms added to affine expressions. Optional, may be NULL.
    /// - `termsizelist_` List of term sizes.
    /// - `termsindjc_` Number of terms in each of the disjunctive constraints in the slice.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putdjcslice>
    pub fn put_djc_slice(&mut self,idxfirst_ : i64,idxlast_ : i64,domidxlist_ : &[i64],afeidxlist_ : &[i64],b_ : &[f64],termsizelist_ : &[i64],termsindjc_ : &[i64]) -> Result<(),String> { self.task.put_djc_slice(idxfirst_,idxlast_,domidxlist_,afeidxlist_,b_,termsizelist_,termsindjc_) }
    /// Sets the name of a domain.
    ///
    /// # Arguments
    ///
    /// - `domidx_` Index of the domain.
    /// - `name_` The name of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putdomainname>
    pub fn put_domain_name(&mut self,domidx_ : i64,name_ : &str) -> Result<(),String> { self.task.put_domain_name(domidx_,name_) }
    /// Sets a double parameter.
    ///
    /// # Arguments
    ///
    /// - `param_` Which parameter.
    ///   
    ///   See [Dparam]
    /// - `parvalue_` Parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putdouparam>
    pub fn put_dou_param(&mut self,param_ : i32,parvalue_ : f64) -> Result<(),String> { self.task.put_dou_param(param_,parvalue_) }
    /// Sets an integer parameter.
    ///
    /// # Arguments
    ///
    /// - `param_` Which parameter.
    ///   
    ///   See [Iparam]
    /// - `parvalue_` Parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putintparam>
    pub fn put_int_param(&mut self,param_ : i32,parvalue_ : i32) -> Result<(),String> { self.task.put_int_param(param_,parvalue_) }
    /// Sets the number of preallocated affine conic constraints.
    ///
    /// # Arguments
    ///
    /// - `maxnumacc_` Number of preallocated affine conic constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putmaxnumacc>
    pub fn put_max_num_acc(&mut self,maxnumacc_ : i64) -> Result<(),String> { self.task.put_max_num_acc(maxnumacc_) }
    /// Sets the number of preallocated affine expressions in the optimization task.
    ///
    /// # Arguments
    ///
    /// - `maxnumafe_` Number of preallocated affine expressions.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putmaxnumafe>
    pub fn put_max_num_afe(&mut self,maxnumafe_ : i64) -> Result<(),String> { self.task.put_max_num_afe(maxnumafe_) }
    /// Sets the number of preallocated non-zero entries in the linear coefficient matrix.
    ///
    /// # Arguments
    ///
    /// - `maxnumanz_` New size of the storage reserved for storing the linear coefficient matrix.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putmaxnumanz>
    pub fn put_max_num_a_nz(&mut self,maxnumanz_ : i64) -> Result<(),String> { self.task.put_max_num_a_nz(maxnumanz_) }
    /// Sets the number of preallocated symmetric matrix variables.
    ///
    /// # Arguments
    ///
    /// - `maxnumbarvar_` Number of preallocated symmetric matrix variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putmaxnumbarvar>
    pub fn put_max_num_barvar(&mut self,maxnumbarvar_ : i32) -> Result<(),String> { self.task.put_max_num_barvar(maxnumbarvar_) }
    /// Sets the number of preallocated constraints in the optimization task.
    ///
    /// # Arguments
    ///
    /// - `maxnumcon_` Number of preallocated constraints in the optimization task.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putmaxnumcon>
    pub fn put_max_num_con(&mut self,maxnumcon_ : i32) -> Result<(),String> { self.task.put_max_num_con(maxnumcon_) }
    /// Sets the number of preallocated conic constraints in the optimization task.
    ///
    /// # Arguments
    ///
    /// - `maxnumcone_` Number of preallocated conic constraints in the optimization task.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putmaxnumcone>
    pub fn put_max_num_cone(&mut self,maxnumcone_ : i32) -> Result<(),String> { self.task.put_max_num_cone(maxnumcone_) }
    /// Sets the number of preallocated disjunctive constraints.
    ///
    /// # Arguments
    ///
    /// - `maxnumdjc_` Number of preallocated disjunctive constraints in the task.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putmaxnumdjc>
    pub fn put_max_num_djc(&mut self,maxnumdjc_ : i64) -> Result<(),String> { self.task.put_max_num_djc(maxnumdjc_) }
    /// Sets the number of preallocated domains in the optimization task.
    ///
    /// # Arguments
    ///
    /// - `maxnumdomain_` Number of preallocated domains.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putmaxnumdomain>
    pub fn put_max_num_domain(&mut self,maxnumdomain_ : i64) -> Result<(),String> { self.task.put_max_num_domain(maxnumdomain_) }
    /// Sets the number of preallocated non-zero entries in quadratic terms.
    ///
    /// # Arguments
    ///
    /// - `maxnumqnz_` Number of non-zero elements preallocated in quadratic coefficient matrices.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putmaxnumqnz>
    pub fn put_max_num_q_nz(&mut self,maxnumqnz_ : i64) -> Result<(),String> { self.task.put_max_num_q_nz(maxnumqnz_) }
    /// Sets the number of preallocated variables in the optimization task.
    ///
    /// # Arguments
    ///
    /// - `maxnumvar_` Number of preallocated variables in the optimization task.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putmaxnumvar>
    pub fn put_max_num_var(&mut self,maxnumvar_ : i32) -> Result<(),String> { self.task.put_max_num_var(maxnumvar_) }
    /// Sets a double parameter.
    ///
    /// # Arguments
    ///
    /// - `paramname_` Name of a parameter.
    /// - `parvalue_` Parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putnadouparam>
    pub fn put_na_dou_param(&mut self,paramname_ : &str,parvalue_ : f64) -> Result<(),String> { self.task.put_na_dou_param(paramname_,parvalue_) }
    /// Sets an integer parameter.
    ///
    /// # Arguments
    ///
    /// - `paramname_` Name of a parameter.
    /// - `parvalue_` Parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putnaintparam>
    pub fn put_na_int_param(&mut self,paramname_ : &str,parvalue_ : i32) -> Result<(),String> { self.task.put_na_int_param(paramname_,parvalue_) }
    /// Sets a string parameter.
    ///
    /// # Arguments
    ///
    /// - `paramname_` Name of a parameter.
    /// - `parvalue_` Parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putnastrparam>
    pub fn put_na_str_param(&mut self,paramname_ : &str,parvalue_ : &str) -> Result<(),String> { self.task.put_na_str_param(paramname_,parvalue_) }
    /// Assigns a new name to the objective.
    ///
    /// # Arguments
    ///
    /// - `objname_` Name of the objective.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putobjname>
    pub fn put_obj_name(&mut self,objname_ : &str) -> Result<(),String> { self.task.put_obj_name(objname_) }
    /// Sets the objective sense.
    ///
    /// # Arguments
    ///
    /// - `sense_` The objective sense of the task
    ///   
    ///   See [Objsense]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putobjsense>
    pub fn put_obj_sense(&mut self,sense_ : i32) -> Result<(),String> { self.task.put_obj_sense(sense_) }
    /// Specify an OptServer for remote calls.
    ///
    /// # Arguments
    ///
    /// - `host_` A URL specifying the optimization server to be used.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putoptserverhost>
    pub fn put_optserver_host(&mut self,host_ : &str) -> Result<(),String> { self.task.put_optserver_host(host_) }
    /// Modifies the value of parameter.
    ///
    /// # Arguments
    ///
    /// - `parname_` Parameter name.
    /// - `parvalue_` Parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putparam>
    pub fn put_param(&mut self,parname_ : &str,parvalue_ : &str) -> Result<(),String> { self.task.put_param(parname_,parvalue_) }
    /// Replaces all quadratic terms in constraints.
    ///
    /// # Arguments
    ///
    /// - `qcsubk_` Constraint subscripts for quadratic coefficients.
    /// - `qcsubi_` Row subscripts for quadratic constraint matrix.
    /// - `qcsubj_` Column subscripts for quadratic constraint matrix.
    /// - `qcval_` Quadratic constraint coefficient values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putqcon>
    pub fn put_q_con(&mut self,qcsubk_ : &[i32],qcsubi_ : &[i32],qcsubj_ : &[i32],qcval_ : &[f64]) -> Result<(),String> { self.task.put_q_con(qcsubk_,qcsubi_,qcsubj_,qcval_) }
    /// Replaces all quadratic terms in a single constraint.
    ///
    /// # Arguments
    ///
    /// - `k_` The constraint in which the new quadratic elements are inserted.
    /// - `qcsubi_` Row subscripts for quadratic constraint matrix.
    /// - `qcsubj_` Column subscripts for quadratic constraint matrix.
    /// - `qcval_` Quadratic constraint coefficient values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putqconk>
    pub fn put_q_con_k(&mut self,k_ : i32,qcsubi_ : &[i32],qcsubj_ : &[i32],qcval_ : &[f64]) -> Result<(),String> { self.task.put_q_con_k(k_,qcsubi_,qcsubj_,qcval_) }
    /// Replaces all quadratic terms in the objective.
    ///
    /// # Arguments
    ///
    /// - `qosubi_` Row subscripts for quadratic objective coefficients.
    /// - `qosubj_` Column subscripts for quadratic objective coefficients.
    /// - `qoval_` Quadratic objective coefficient values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putqobj>
    pub fn put_q_obj(&mut self,qosubi_ : &[i32],qosubj_ : &[i32],qoval_ : &[f64]) -> Result<(),String> { self.task.put_q_obj(qosubi_,qosubj_,qoval_) }
    /// Replaces one coefficient in the quadratic term in the objective.
    ///
    /// # Arguments
    ///
    /// - `i_` Row index for the coefficient to be replaced.
    /// - `j_` Column index for the coefficient to be replaced.
    /// - `qoij_` The new coefficient value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putqobjij>
    pub fn put_q_obj_i_j(&mut self,i_ : i32,j_ : i32,qoij_ : f64) -> Result<(),String> { self.task.put_q_obj_i_j(i_,j_,qoij_) }
    /// Sets the status keys for the constraints.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `skc_` Status keys for the constraints.
    ///   
    ///   See [Stakey]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putskc>
    pub fn put_skc(&mut self,whichsol_ : i32,skc_ : &[i32]) -> Result<(),String> { self.task.put_skc(whichsol_,skc_) }
    /// Sets the status keys for a slice of the constraints.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `skc_` Status keys for the constraints.
    ///   
    ///   See [Stakey]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putskcslice>
    pub fn put_skc_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,skc_ : &[i32]) -> Result<(),String> { self.task.put_skc_slice(whichsol_,first_,last_,skc_) }
    /// Sets the status keys for the scalar variables.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `skx_` Status keys for the variables.
    ///   
    ///   See [Stakey]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putskx>
    pub fn put_skx(&mut self,whichsol_ : i32,skx_ : &[i32]) -> Result<(),String> { self.task.put_skx(whichsol_,skx_) }
    /// Sets the status keys for a slice of the variables.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `skx_` Status keys for the variables.
    ///   
    ///   See [Stakey]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putskxslice>
    pub fn put_skx_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,skx_ : &[i32]) -> Result<(),String> { self.task.put_skx_slice(whichsol_,first_,last_,skx_) }
    /// Sets the slc vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `slc_` Dual variables corresponding to the lower bounds on the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putslc>
    pub fn put_slc(&mut self,whichsol_ : i32,slc_ : &[f64]) -> Result<(),String> { self.task.put_slc(whichsol_,slc_) }
    /// Sets a slice of the slc vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `slc_` Dual variables corresponding to the lower bounds on the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putslcslice>
    pub fn put_slc_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,slc_ : &[f64]) -> Result<(),String> { self.task.put_slc_slice(whichsol_,first_,last_,slc_) }
    /// Sets the slx vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `slx_` Dual variables corresponding to the lower bounds on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putslx>
    pub fn put_slx(&mut self,whichsol_ : i32,slx_ : &[f64]) -> Result<(),String> { self.task.put_slx(whichsol_,slx_) }
    /// Sets a slice of the slx vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `slx_` Dual variables corresponding to the lower bounds on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putslxslice>
    pub fn put_slx_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,slx_ : &[f64]) -> Result<(),String> { self.task.put_slx_slice(whichsol_,first_,last_,slx_) }
    /// Sets the snx vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sux_` Dual variables corresponding to the upper bounds on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putsnx>
    pub fn put_snx(&mut self,whichsol_ : i32,sux_ : &[f64]) -> Result<(),String> { self.task.put_snx(whichsol_,sux_) }
    /// Sets a slice of the snx vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `snx_` Dual variables corresponding to the conic constraints on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putsnxslice>
    pub fn put_snx_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,snx_ : &[f64]) -> Result<(),String> { self.task.put_snx_slice(whichsol_,first_,last_,snx_) }
    /// Inserts a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `skc_` Status keys for the constraints.
    ///   
    ///   See [Stakey]
    /// - `skx_` Status keys for the variables.
    ///   
    ///   See [Stakey]
    /// - `skn_` Status keys for the conic constraints.
    ///   
    ///   See [Stakey]
    /// - `xc_` Primal constraint solution.
    /// - `xx_` Primal variable solution.
    /// - `y_` Vector of dual variables corresponding to the constraints.
    /// - `slc_` Dual variables corresponding to the lower bounds on the constraints.
    /// - `suc_` Dual variables corresponding to the upper bounds on the constraints.
    /// - `slx_` Dual variables corresponding to the lower bounds on the variables.
    /// - `sux_` Dual variables corresponding to the upper bounds on the variables.
    /// - `snx_` Dual variables corresponding to the conic constraints on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putsolution>
    pub fn put_solution(&mut self,whichsol_ : i32,skc_ : &[i32],skx_ : &[i32],skn_ : &[i32],xc_ : &[f64],xx_ : &[f64],y_ : &[f64],slc_ : &[f64],suc_ : &[f64],slx_ : &[f64],sux_ : &[f64],snx_ : &[f64]) -> Result<(),String> { self.task.put_solution(whichsol_,skc_,skx_,skn_,xc_,xx_,y_,slc_,suc_,slx_,sux_,snx_) }
    /// Inserts a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `skc_` Status keys for the constraints.
    ///   
    ///   See [Stakey]
    /// - `skx_` Status keys for the variables.
    ///   
    ///   See [Stakey]
    /// - `skn_` Status keys for the conic constraints.
    ///   
    ///   See [Stakey]
    /// - `xc_` Primal constraint solution.
    /// - `xx_` Primal variable solution.
    /// - `y_` Vector of dual variables corresponding to the constraints.
    /// - `slc_` Dual variables corresponding to the lower bounds on the constraints.
    /// - `suc_` Dual variables corresponding to the upper bounds on the constraints.
    /// - `slx_` Dual variables corresponding to the lower bounds on the variables.
    /// - `sux_` Dual variables corresponding to the upper bounds on the variables.
    /// - `snx_` Dual variables corresponding to the conic constraints on the variables.
    /// - `doty_` Dual variables corresponding to affine conic constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putsolutionnew>
    pub fn put_solution_new(&mut self,whichsol_ : i32,skc_ : &[i32],skx_ : &[i32],skn_ : &[i32],xc_ : &[f64],xx_ : &[f64],y_ : &[f64],slc_ : &[f64],suc_ : &[f64],slx_ : &[f64],sux_ : &[f64],snx_ : &[f64],doty_ : &[f64]) -> Result<(),String> { self.task.put_solution_new(whichsol_,skc_,skx_,skn_,xc_,xx_,y_,slc_,suc_,slx_,sux_,snx_,doty_) }
    /// Inputs the dual variable of a solution.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the dual variable.
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `y_` Solution value of the dual variable.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putsolutionyi>
    pub fn put_solution_y_i(&mut self,i_ : i32,whichsol_ : i32,y_ : f64) -> Result<(),String> { self.task.put_solution_y_i(i_,whichsol_,y_) }
    /// Sets a string parameter.
    ///
    /// # Arguments
    ///
    /// - `param_` Which parameter.
    ///   
    ///   See [Sparam]
    /// - `parvalue_` Parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putstrparam>
    pub fn put_str_param(&mut self,param_ : i32,parvalue_ : &str) -> Result<(),String> { self.task.put_str_param(param_,parvalue_) }
    /// Sets the suc vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `suc_` Dual variables corresponding to the upper bounds on the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putsuc>
    pub fn put_suc(&mut self,whichsol_ : i32,suc_ : &[f64]) -> Result<(),String> { self.task.put_suc(whichsol_,suc_) }
    /// Sets a slice of the suc vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `suc_` Dual variables corresponding to the upper bounds on the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putsucslice>
    pub fn put_suc_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,suc_ : &[f64]) -> Result<(),String> { self.task.put_suc_slice(whichsol_,first_,last_,suc_) }
    /// Sets the sux vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sux_` Dual variables corresponding to the upper bounds on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putsux>
    pub fn put_sux(&mut self,whichsol_ : i32,sux_ : &[f64]) -> Result<(),String> { self.task.put_sux(whichsol_,sux_) }
    /// Sets a slice of the sux vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `sux_` Dual variables corresponding to the upper bounds on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putsuxslice>
    pub fn put_sux_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,sux_ : &[f64]) -> Result<(),String> { self.task.put_sux_slice(whichsol_,first_,last_,sux_) }
    /// Assigns a new name to the task.
    ///
    /// # Arguments
    ///
    /// - `taskname_` Name assigned to the task.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.puttaskname>
    pub fn put_task_name(&mut self,taskname_ : &str) -> Result<(),String> { self.task.put_task_name(taskname_) }
    /// Changes the bounds for one variable.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the variable.
    /// - `bkx_` New bound key.
    ///   
    ///   See [Boundkey]
    /// - `blx_` New lower bound.
    /// - `bux_` New upper bound.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putvarbound>
    pub fn put_var_bound(&mut self,j_ : i32,bkx_ : i32,blx_ : f64,bux_ : f64) -> Result<(),String> { self.task.put_var_bound(j_,bkx_,blx_,bux_) }
    /// Changes the bounds of a list of variables.
    ///
    /// # Arguments
    ///
    /// - `sub_` List of variable indexes.
    /// - `bkx_` Bound keys for the variables.
    ///   
    ///   See [Boundkey]
    /// - `blx_` Lower bounds for the variables.
    /// - `bux_` Upper bounds for the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putvarboundlist>
    pub fn put_var_bound_list(&mut self,sub_ : &[i32],bkx_ : &[i32],blx_ : &[f64],bux_ : &[f64]) -> Result<(),String> { self.task.put_var_bound_list(sub_,bkx_,blx_,bux_) }
    /// Changes the bounds of a list of variables.
    ///
    /// # Arguments
    ///
    /// - `sub_` List of variable indexes.
    /// - `bkx_` New bound key for all variables in the list.
    ///   
    ///   See [Boundkey]
    /// - `blx_` New lower bound for all variables in the list.
    /// - `bux_` New upper bound for all variables in the list.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putvarboundlistconst>
    pub fn put_var_bound_list_const(&mut self,sub_ : &[i32],bkx_ : i32,blx_ : f64,bux_ : f64) -> Result<(),String> { self.task.put_var_bound_list_const(sub_,bkx_,blx_,bux_) }
    /// Changes the bounds for a slice of the variables.
    ///
    /// # Arguments
    ///
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `bkx_` Bound keys for the variables.
    ///   
    ///   See [Boundkey]
    /// - `blx_` Lower bounds for the variables.
    /// - `bux_` Upper bounds for the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putvarboundslice>
    pub fn put_var_bound_slice(&mut self,first_ : i32,last_ : i32,bkx_ : &[i32],blx_ : &[f64],bux_ : &[f64]) -> Result<(),String> { self.task.put_var_bound_slice(first_,last_,bkx_,blx_,bux_) }
    /// Changes the bounds for a slice of the variables.
    ///
    /// # Arguments
    ///
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `bkx_` New bound key for all variables in the slice.
    ///   
    ///   See [Boundkey]
    /// - `blx_` New lower bound for all variables in the slice.
    /// - `bux_` New upper bound for all variables in the slice.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putvarboundsliceconst>
    pub fn put_var_bound_slice_const(&mut self,first_ : i32,last_ : i32,bkx_ : i32,blx_ : f64,bux_ : f64) -> Result<(),String> { self.task.put_var_bound_slice_const(first_,last_,bkx_,blx_,bux_) }
    /// Sets the name of a variable.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the variable.
    /// - `name_` The variable name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putvarname>
    pub fn put_var_name(&mut self,j_ : i32,name_ : &str) -> Result<(),String> { self.task.put_var_name(j_,name_) }
    /// Sets the primal and dual solution information for a single variable.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the variable.
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sk_` Status key of the variable.
    ///   
    ///   See [Stakey]
    /// - `x_` Primal solution value of the variable.
    /// - `sl_` Solution value of the dual variable associated with the lower bound.
    /// - `su_` Solution value of the dual variable associated with the upper bound.
    /// - `sn_` Solution value of the dual variable associated with the conic constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putvarsolutionj>
    pub fn put_var_solution_j(&mut self,j_ : i32,whichsol_ : i32,sk_ : i32,x_ : f64,sl_ : f64,su_ : f64,sn_ : f64) -> Result<(),String> { self.task.put_var_solution_j(j_,whichsol_,sk_,x_,sl_,su_,sn_) }
    /// Sets the variable type of one variable.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the variable.
    /// - `vartype_` The new variable type.
    ///   
    ///   See [Variabletype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putvartype>
    pub fn put_var_type(&mut self,j_ : i32,vartype_ : i32) -> Result<(),String> { self.task.put_var_type(j_,vartype_) }
    /// Sets the variable type for one or more variables.
    ///
    /// # Arguments
    ///
    /// - `subj_` A list of variable indexes for which the variable type should be changed.
    /// - `vartype_` A list of variable types.
    ///   
    ///   See [Variabletype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putvartypelist>
    pub fn put_var_type_list(&mut self,subj_ : &[i32],vartype_ : &[i32]) -> Result<(),String> { self.task.put_var_type_list(subj_,vartype_) }
    /// Sets the xc vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `xc_` Primal constraint solution.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putxc>
    pub fn put_xc(&mut self,whichsol_ : i32,xc_ : &mut[f64]) -> Result<(),String> { self.task.put_xc(whichsol_,xc_) }
    /// Sets a slice of the xc vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `xc_` Primal constraint solution.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putxcslice>
    pub fn put_xc_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,xc_ : &[f64]) -> Result<(),String> { self.task.put_xc_slice(whichsol_,first_,last_,xc_) }
    /// Sets the xx vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `xx_` Primal variable solution.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putxx>
    pub fn put_xx(&mut self,whichsol_ : i32,xx_ : &[f64]) -> Result<(),String> { self.task.put_xx(whichsol_,xx_) }
    /// Sets a slice of the xx vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `xx_` Primal variable solution.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putxxslice>
    pub fn put_xx_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,xx_ : &[f64]) -> Result<(),String> { self.task.put_xx_slice(whichsol_,first_,last_,xx_) }
    /// Sets the y vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `y_` Vector of dual variables corresponding to the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.puty>
    pub fn put_y(&mut self,whichsol_ : i32,y_ : &[f64]) -> Result<(),String> { self.task.put_y(whichsol_,y_) }
    /// Sets a slice of the y vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `y_` Vector of dual variables corresponding to the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putyslice>
    pub fn put_y_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,y_ : &[f64]) -> Result<(),String> { self.task.put_y_slice(whichsol_,first_,last_,y_) }
    /// Read a binary dump of the task solution and information items.
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    /// - `compress_` Data compression type.
    ///   
    ///   See [Compresstype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readbsolution>
    pub fn read_b_solution(&self,filename_ : &str,compress_ : i32) -> Result<(),String> { self.task.read_b_solution(filename_,compress_) }
    /// Reads problem data from a file.
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readdataautoformat>
    pub fn read_data(&mut self,filename_ : &str) -> Result<(),String> { self.task.read_data(filename_) }
    /// Reads problem data from a file.
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    /// - `format_` File data format.
    ///   
    ///   See [Dataformat]
    /// - `compress_` File compression type.
    ///   
    ///   See [Compresstype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readdataformat>
    pub fn read_data_format(&mut self,filename_ : &str,format_ : i32,compress_ : i32) -> Result<(),String> { self.task.read_data_format(filename_,format_,compress_) }
    /// Reads a solution from a JSOL file.
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readjsonsol>
    pub fn read_json_sol(&mut self,filename_ : &str) -> Result<(),String> { self.task.read_json_sol(filename_) }
    /// Load task data from a string in JSON format.
    ///
    /// # Arguments
    ///
    /// - `data_` Problem data in text format.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readjsonstring>
    pub fn read_json_string(&mut self,data_ : &str) -> Result<(),String> { self.task.read_json_string(data_) }
    /// Load task data from a string in LP format.
    ///
    /// # Arguments
    ///
    /// - `data_` Problem data in text format.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readlpstring>
    pub fn read_lp_string(&mut self,data_ : &str) -> Result<(),String> { self.task.read_lp_string(data_) }
    /// Load task data from a string in OPF format.
    ///
    /// # Arguments
    ///
    /// - `data_` Problem data in text format.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readopfstring>
    pub fn read_opf_string(&mut self,data_ : &str) -> Result<(),String> { self.task.read_opf_string(data_) }
    /// Reads a parameter file.
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readparamfile>
    pub fn read_param_file(&mut self,filename_ : &str) -> Result<(),String> { self.task.read_param_file(filename_) }
    /// Load task data from a string in PTF format.
    ///
    /// # Arguments
    ///
    /// - `data_` Problem data in text format.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readptfstring>
    pub fn read_ptf_string(&mut self,data_ : &str) -> Result<(),String> { self.task.read_ptf_string(data_) }
    /// Reads a solution from a file.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readsolution>
    pub fn read_solution(&mut self,whichsol_ : i32,filename_ : &str) -> Result<(),String> { self.task.read_solution(whichsol_,filename_) }
    /// Read solution file in format determined by the filename
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readsolutionfile>
    pub fn read_solution_file(&self,filename_ : &str) -> Result<(),String> { self.task.read_solution_file(filename_) }
    /// Prints information about last file read.
    ///
    /// # Arguments
    ///
    /// - `whichstream_` Index of the stream.
    ///   
    ///   See [Streamtype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readsummary>
    pub fn read_summary(&mut self,whichstream_ : i32) -> Result<(),String> { self.task.read_summary(whichstream_) }
    /// Load task data from a file.
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readtask>
    pub fn read_task(&mut self,filename_ : &str) -> Result<(),String> { self.task.read_task(filename_) }
    /// Removes a number of symmetric matrices.
    ///
    /// # Arguments
    ///
    /// - `subset_` Indexes of symmetric matrices which should be removed.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.removebarvars>
    pub fn remove_barvars(&mut self,subset_ : &[i32]) -> Result<(),String> { self.task.remove_barvars(subset_) }
    /// Removes a number of conic constraints from the problem.
    ///
    /// # Arguments
    ///
    /// - `subset_` Indexes of cones which should be removed.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.removecones>
    pub fn remove_cones(&mut self,subset_ : &[i32]) -> Result<(),String> { self.task.remove_cones(subset_) }
    /// Removes a number of constraints.
    ///
    /// # Arguments
    ///
    /// - `subset_` Indexes of constraints which should be removed.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.removecons>
    pub fn remove_cons(&mut self,subset_ : &[i32]) -> Result<(),String> { self.task.remove_cons(subset_) }
    /// Removes a number of variables.
    ///
    /// # Arguments
    ///
    /// - `subset_` Indexes of variables which should be removed.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.removevars>
    pub fn remove_vars(&mut self,subset_ : &[i32]) -> Result<(),String> { self.task.remove_vars(subset_) }
    /// Resizes an optimization task.
    ///
    /// # Arguments
    ///
    /// - `maxnumcon_` New maximum number of constraints.
    /// - `maxnumvar_` New maximum number of variables.
    /// - `maxnumcone_` New maximum number of cones.
    /// - `maxnumanz_` New maximum number of linear non-zero elements.
    /// - `maxnumqnz_` New maximum number of quadratic non-zeros elements.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.resizetask>
    pub fn resize_task(&mut self,maxnumcon_ : i32,maxnumvar_ : i32,maxnumcone_ : i32,maxnumanz_ : i64,maxnumqnz_ : i64) -> Result<(),String> { self.task.resize_task(maxnumcon_,maxnumvar_,maxnumcone_,maxnumanz_,maxnumqnz_) }
    /// Creates a sensitivity report.
    ///
    /// # Arguments
    ///
    /// - `whichstream_` Index of the stream.
    ///   
    ///   See [Streamtype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.sensitivityreport>
    pub fn sensitivity_report(&self,whichstream_ : i32) -> Result<(),String> { self.task.sensitivity_report(whichstream_) }
    /// Resets all parameter values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.setdefaults>
    pub fn set_defaults(&mut self) -> Result<(),String> { self.task.set_defaults() }
    /// Checks whether a solution is defined.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    ///
    /// # Returns
    ///
    ///   - `isdef` Is non-zero if the requested solution is defined.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.solutiondef>
    pub fn solution_def(&self,whichsol_ : i32) -> Result<bool,String> { self.task.solution_def(whichsol_) }
    /// Prints a short summary of the current solutions.
    ///
    /// # Arguments
    ///
    /// - `whichstream_` Index of the stream.
    ///   
    ///   See [Streamtype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.solutionsummary>
    pub fn solution_summary(&self,whichstream_ : i32) -> Result<(),String> { self.task.solution_summary(whichstream_) }
    /// Solve a linear equation system involving a basis matrix.
    ///
    /// # Arguments
    ///
    /// - `transp_` Controls which problem formulation is solved.
    /// - `numnz_` Input (number of non-zeros in right-hand side).
    /// - `sub_` Input (indexes of non-zeros in right-hand side) and output (indexes of non-zeros in solution vector).
    /// - `val_` Input (right-hand side values) and output (solution vector values).
    ///
    /// # Returns
    ///
    ///   - `numnzout` Output (number of non-zeros in solution vector).
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.solvewithbasis>
    pub fn solve_with_basis(&mut self,transp_ : bool,numnz_ : i32,sub_ : &mut[i32],val_ : &mut[f64]) -> Result<i32,String> { self.task.solve_with_basis(transp_,numnz_,sub_,val_) }
    /// Obtains a cone type code.
    ///
    /// # Arguments
    ///
    /// - `str_` String corresponding to the cone type code.
    /// - `conetype_` The cone type corresponding to str.
    ///   
    ///   See [Conetype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.strtoconetype>
    pub fn str_to_cone_type(&self,str_ : &str,conetype_ : & mut i32) -> Result<(),String> { self.task.str_to_cone_type(str_,conetype_) }
    /// Obtains a status key.
    ///
    /// # Arguments
    ///
    /// - `str_` A status key abbreviation string.
    /// - `sk_` Status key corresponding to the string.
    ///   
    ///   See [Stakey]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.strtosk>
    pub fn str_to_sk(&self,str_ : &str,sk_ : & mut i32) -> Result<(),String> { self.task.str_to_sk(str_,sk_) }
    /// In-place reformulation of a QCQO to a conic quadratic problem.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.toconic>
    pub fn toconic(&mut self) -> Result<(),String> { self.task.toconic() }
    /// Disconnects a user-defined function from a task stream.
    ///
    /// # Arguments
    ///
    /// - `whichstream_` Index of the stream.
    ///   
    ///   See [Streamtype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.unlinkfuncfromtaskstream>
    pub fn unlink_func_from_stream(&mut self,whichstream_ : i32) -> Result<(),String> { self.task.unlink_func_from_stream(whichstream_) }
    /// Update the information items related to the solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.updatesolutioninfo>
    pub fn update_solution_info(&mut self,whichsol_ : i32) -> Result<(),String> { self.task.update_solution_info(whichsol_) }
    /// Checks a parameter name.
    ///
    /// # Arguments
    ///
    /// - `parname_` Parameter name.
    /// - `partype_` Parameter type.
    ///   
    ///   See [Parametertype]
    /// - `param_` Which parameter.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.whichparam>
    pub fn which_param(&self,parname_ : &str,partype_ : & mut i32,param_ : &mut i32) -> Result<(),String> { self.task.which_param(parname_,partype_,param_) }
    /// Write a binary dump of the task solution and information items.
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    /// - `compress_` Data compression type.
    ///   
    ///   See [Compresstype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.writebsolution>
    pub fn write_b_solution(&self,filename_ : &str,compress_ : i32) -> Result<(),String> { self.task.write_b_solution(filename_,compress_) }
    /// Writes problem data to a file.
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.writedata>
    pub fn write_data(&self,filename_ : &str) -> Result<(),String> { self.task.write_data(filename_) }
    /// Writes a solution to a JSON file.
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.writejsonsol>
    pub fn write_json_sol(&self,filename_ : &str) -> Result<(),String> { self.task.write_json_sol(filename_) }
    /// Writes all the parameters to a parameter file.
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.writeparamfile>
    pub fn write_param_file(&self,filename_ : &str) -> Result<(),String> { self.task.write_param_file(filename_) }
    /// Write a solution to a file.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.writesolution>
    pub fn write_solution(&self,whichsol_ : i32,filename_ : &str) -> Result<(),String> { self.task.write_solution(whichsol_,filename_) }
    /// Write solution file in format determined by the filename
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.writesolutionfile>
    pub fn write_solution_file(&self,filename_ : &str) -> Result<(),String> { self.task.write_solution_file(filename_) }
    /// Appends a record to the statistics file.
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.writestat>
    pub fn write_stat(&mut self,filename_ : &str) -> Result<(),String> { self.task.write_stat(filename_) }
    /// Write a complete binary dump of the task data.
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.writetask>
    pub fn write_task(&self,filename_ : &str) -> Result<(),String> { self.task.write_task(filename_) }
    /// Internal
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    /// - `compress_` Data compression type.
    ///   
    ///   See [Compresstype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.writetasksolverresult_file>
    pub fn write_task_solver_result_file(&self,filename_ : &str,compress_ : i32) -> Result<(),String> { self.task.write_task_solver_result_file(filename_,compress_) }

}
impl Task {
    /// Create a new task in the given environment or with the default environment with a given capacity
    pub fn with_capacity(env : Option<&Env>, numcon : i32, numvar : i32) -> Option<Task> {
        let mut task : * const u8 = std::ptr::null();
        if 0 != unsafe { MSK_maketask(if let Some(e) = env {(*e).ptr} else {std::ptr::null()},
                                      numcon,
                                      numvar,
                                      & mut task) } {
            None
        }
        else {
            Some(Task { ptr : task })
        }
    }

    /// Create a new task in the given environment or with the default environment
    pub fn from_env(env : Option<&Env>) -> Option<Task> { Task::with_capacity(env,0,0) }

    /// Create a new task in the given environment or with the default environment
    pub fn clone(&self) -> Option<Task> {
        let mut task : * const u8 = std::ptr::null();
        if 0 != unsafe { MSK_clonetask(self.ptr,& mut task) } {
            None
        }
        else {
            Some(Task{ptr : task})
        }
    }

    /// Create a new task in the default environment
    pub fn new()  -> Option<Task> { Task::with_capacity(None,0,0) }


    /// This converts the Task object into a TaskCB object. The main
    /// difference is the the TaskCB enables attaching callback
    /// functions (message printing and information callbacks), and
    /// that it due to the callbacks cannot be shared between multiple
    /// threads.
    pub fn with_callbacks(self) -> TaskCB { TaskCB::new(self) }

    #[allow(unused_parens)]
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



    /// Analyze the names and issue an error for the first invalid name.
    ///
    /// # Arguments
    ///
    /// - `whichstream_` Index of the stream.
    ///   
    ///   See [Streamtype]
    /// - `nametype_` The type of names e.g. valid in MPS or LP files.
    ///   
    ///   See [Nametype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.analyzenames>
    #[allow(unused_parens)]
    pub fn analyze_names(&self,whichstream_ : i32,nametype_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_analyzenames(self.ptr,whichstream_,nametype_) },"analyze_names")?;
      return Result::Ok(());
    } // analyzenames
    /// Analyze the data of a task.
    ///
    /// # Arguments
    ///
    /// - `whichstream_` Index of the stream.
    ///   
    ///   See [Streamtype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.analyzeproblem>
    #[allow(unused_parens)]
    pub fn analyze_problem(&self,whichstream_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_analyzeproblem(self.ptr,whichstream_) },"analyze_problem")?;
      return Result::Ok(());
    } // analyzeproblem
    /// Print information related to the quality of the solution.
    ///
    /// # Arguments
    ///
    /// - `whichstream_` Index of the stream.
    ///   
    ///   See [Streamtype]
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.analyzesolution>
    #[allow(unused_parens)]
    pub fn analyze_solution(&self,whichstream_ : i32,whichsol_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_analyzesolution(self.ptr,whichstream_,whichsol_) },"analyze_solution")?;
      return Result::Ok(());
    } // analyzesolution
    /// Appends an affine conic constraint to the task.
    ///
    /// # Arguments
    ///
    /// - `domidx_` Domain index.
    /// - `afeidxlist_` List of affine expression indexes.
    /// - `b_` The vector of constant terms added to affine expressions. Optional, can be NULL.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendacc>
    #[allow(unused_parens)]
    pub fn append_acc(&mut self,domidx_ : i64,afeidxlist_ : &[i64],b_ : &[f64]) -> Result<(),String> {
      let numafeidx_ : i64 = afeidxlist_.len() as i64;
      if b_.len() != (numafeidx_).try_into().unwrap() {
        return Result::Err("append_acc: Argument 'b' has the wrong length, expected numafeidx_".to_string());
      }
      self.handle_res(unsafe { MSK_appendacc(self.ptr,domidx_,numafeidx_,afeidxlist_.as_ptr(),b_.as_ptr()) },"append_acc")?;
      return Result::Ok(());
    } // appendacc
    /// Appends a number of affine conic constraint to the task.
    ///
    /// # Arguments
    ///
    /// - `domidxs_` Domain indices.
    /// - `afeidxlist_` List of affine expression indexes.
    /// - `b_` The vector of constant terms added to affine expressions. Optional, can be NULL.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendaccs>
    #[allow(unused_parens)]
    pub fn append_accs(&mut self,domidxs_ : &[i64],afeidxlist_ : &[i64],b_ : &[f64]) -> Result<(),String> {
      let numaccs_ : i64 = domidxs_.len() as i64;
      let numafeidx_ : i64 = afeidxlist_.len() as i64;
      if b_.len() != (numafeidx_).try_into().unwrap() {
        return Result::Err("append_accs: Argument 'b' has the wrong length, expected numafeidx_".to_string());
      }
      self.handle_res(unsafe { MSK_appendaccs(self.ptr,numaccs_,domidxs_.as_ptr(),numafeidx_,afeidxlist_.as_ptr(),b_.as_ptr()) },"append_accs")?;
      return Result::Ok(());
    } // appendaccs
    /// Appends an affine conic constraint to the task.
    ///
    /// # Arguments
    ///
    /// - `domidx_` Domain index.
    /// - `afeidxfirst_` Index of the first affine expression.
    /// - `b_` The vector of constant terms added to affine expressions. Optional, can be NULL.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendaccseq>
    #[allow(unused_parens)]
    pub fn append_acc_seq(&mut self,domidx_ : i64,afeidxfirst_ : i64,b_ : &[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getdomainn(self.ptr,domidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getdomainn")?;
      let numafeidx_ : i64 = __tmp_0;
      if b_.len() != (numafeidx_).try_into().unwrap() {
        return Result::Err("append_acc_seq: Argument 'b' has the wrong length, expected numafeidx_".to_string());
      }
      self.handle_res(unsafe { MSK_appendaccseq(self.ptr,domidx_,numafeidx_,afeidxfirst_,b_.as_ptr()) },"append_acc_seq")?;
      return Result::Ok(());
    } // appendaccseq
    /// Appends a number of affine conic constraint to the task.
    ///
    /// # Arguments
    ///
    /// - `domidxs_` Domain indices.
    /// - `numafeidx_` Number of affine expressions in the affine expression list (must equal the sum of dimensions of the domains).
    /// - `afeidxfirst_` Index of the first affine expression.
    /// - `b_` The vector of constant terms added to affine expressions. Optional, can be NULL.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendaccsseq>
    #[allow(unused_parens)]
    pub fn append_accs_seq(&mut self,domidxs_ : &[i64],numafeidx_ : i64,afeidxfirst_ : i64,b_ : &[f64]) -> Result<(),String> {
      let numaccs_ : i64 = domidxs_.len() as i64;
      if b_.len() != (numafeidx_).try_into().unwrap() {
        return Result::Err("append_accs_seq: Argument 'b' has the wrong length, expected numafeidx_".to_string());
      }
      self.handle_res(unsafe { MSK_appendaccsseq(self.ptr,numaccs_,domidxs_.as_ptr(),numafeidx_,afeidxfirst_,b_.as_ptr()) },"append_accs_seq")?;
      return Result::Ok(());
    } // appendaccsseq
    /// Appends a number of empty affine expressions to the optimization task.
    ///
    /// # Arguments
    ///
    /// - `num_` Number of empty affine expressions which should be appended.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendafes>
    #[allow(unused_parens)]
    pub fn append_afes(&mut self,num_ : i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_appendafes(self.ptr,num_) },"append_afes")?;
      return Result::Ok(());
    } // appendafes
    /// Appends semidefinite variables to the problem.
    ///
    /// # Arguments
    ///
    /// - `dim_` Dimensions of symmetric matrix variables to be added.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendbarvars>
    #[allow(unused_parens)]
    pub fn append_barvars(&mut self,dim_ : &[i32]) -> Result<(),String> {
      let num_ : i32 = dim_.len() as i32;
      self.handle_res(unsafe { MSK_appendbarvars(self.ptr,num_,dim_.as_ptr()) },"append_barvars")?;
      return Result::Ok(());
    } // appendbarvars
    /// Appends a new conic constraint to the problem.
    ///
    /// # Arguments
    ///
    /// - `ct_` Specifies the type of the cone.
    ///   
    ///   See [Conetype]
    /// - `conepar_` For the power cone it denotes the exponent alpha. For other cone types it is unused and can be set to 0.
    /// - `submem_` Variable subscripts of the members in the cone.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendcone>
    #[allow(unused_parens)]
    pub fn append_cone(&mut self,ct_ : i32,conepar_ : f64,submem_ : &[i32]) -> Result<(),String> {
      let nummem_ : i32 = submem_.len() as i32;
      self.handle_res(unsafe { MSK_appendcone(self.ptr,ct_,conepar_,nummem_,submem_.as_ptr()) },"append_cone")?;
      return Result::Ok(());
    } // appendcone
    /// Appends a new conic constraint to the problem.
    ///
    /// # Arguments
    ///
    /// - `ct_` Specifies the type of the cone.
    ///   
    ///   See [Conetype]
    /// - `conepar_` For the power cone it denotes the exponent alpha. For other cone types it is unused and can be set to 0.
    /// - `nummem_` Number of member variables in the cone.
    /// - `j_` Index of the first variable in the conic constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendconeseq>
    #[allow(unused_parens)]
    pub fn append_cone_seq(&mut self,ct_ : i32,conepar_ : f64,nummem_ : i32,j_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_appendconeseq(self.ptr,ct_,conepar_,nummem_,j_) },"append_cone_seq")?;
      return Result::Ok(());
    } // appendconeseq
    /// Appends multiple conic constraints to the problem.
    ///
    /// # Arguments
    ///
    /// - `ct_` Specifies the type of the cone.
    ///   
    ///   See [Conetype]
    /// - `conepar_` For the power cone it denotes the exponent alpha. For other cone types it is unused and can be set to 0.
    /// - `nummem_` Numbers of member variables in the cones.
    /// - `j_` Index of the first variable in the first cone to be appended.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendconesseq>
    #[allow(unused_parens)]
    pub fn append_cones_seq(&mut self,ct_ : &[i32],conepar_ : &[f64],nummem_ : &[i32],j_ : i32) -> Result<(),String> {
      let num_ : i32 = std::cmp::min(std::cmp::min(nummem_.len(),ct_.len()),conepar_.len()) as i32;
      self.handle_res(unsafe { MSK_appendconesseq(self.ptr,num_,ct_.as_ptr(),conepar_.as_ptr(),nummem_.as_ptr(),j_) },"append_cones_seq")?;
      return Result::Ok(());
    } // appendconesseq
    /// Appends a number of constraints to the optimization task.
    ///
    /// # Arguments
    ///
    /// - `num_` Number of constraints which should be appended.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendcons>
    #[allow(unused_parens)]
    pub fn append_cons(&mut self,num_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_appendcons(self.ptr,num_) },"append_cons")?;
      return Result::Ok(());
    } // appendcons
    /// Appends a number of empty disjunctive constraints to the task.
    ///
    /// # Arguments
    ///
    /// - `num_` Number of empty disjunctive constraints which should be appended.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appenddjcs>
    #[allow(unused_parens)]
    pub fn append_djcs(&mut self,num_ : i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_appenddjcs(self.ptr,num_) },"append_djcs")?;
      return Result::Ok(());
    } // appenddjcs
    /// Appends the dual exponential cone domain.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appenddualexpconedomain>
    #[allow(unused_parens)]
    pub fn append_dual_exp_cone_domain(&mut self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appenddualexpconedomain(self.ptr,&mut __tmp_0) },"append_dual_exp_cone_domain")?;
      return Result::Ok(__tmp_0);
    } // appenddualexpconedomain
    /// Appends the dual geometric mean cone domain.
    ///
    /// # Arguments
    ///
    /// - `n_` Dimmension of the domain.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appenddualgeomeanconedomain>
    #[allow(unused_parens)]
    pub fn append_dual_geo_mean_cone_domain(&mut self,n_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appenddualgeomeanconedomain(self.ptr,n_,&mut __tmp_0) },"append_dual_geo_mean_cone_domain")?;
      return Result::Ok(__tmp_0);
    } // appenddualgeomeanconedomain
    /// Appends the dual power cone domain.
    ///
    /// # Arguments
    ///
    /// - `n_` Dimension of the domain.
    /// - `alpha_` The sequence proportional to exponents. Must be positive.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appenddualpowerconedomain>
    #[allow(unused_parens)]
    pub fn append_dual_power_cone_domain(&mut self,n_ : i64,alpha_ : &[f64]) -> Result<i64,String> {
      let nleft_ : i64 = alpha_.len() as i64;
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appenddualpowerconedomain(self.ptr,n_,nleft_,alpha_.as_ptr(),&mut __tmp_0) },"append_dual_power_cone_domain")?;
      return Result::Ok(__tmp_0);
    } // appenddualpowerconedomain
    /// Appends the primal exponential cone domain.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendprimalexpconedomain>
    #[allow(unused_parens)]
    pub fn append_primal_exp_cone_domain(&mut self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appendprimalexpconedomain(self.ptr,&mut __tmp_0) },"append_primal_exp_cone_domain")?;
      return Result::Ok(__tmp_0);
    } // appendprimalexpconedomain
    /// Appends the primal geometric mean cone domain.
    ///
    /// # Arguments
    ///
    /// - `n_` Dimmension of the domain.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendprimalgeomeanconedomain>
    #[allow(unused_parens)]
    pub fn append_primal_geo_mean_cone_domain(&mut self,n_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appendprimalgeomeanconedomain(self.ptr,n_,&mut __tmp_0) },"append_primal_geo_mean_cone_domain")?;
      return Result::Ok(__tmp_0);
    } // appendprimalgeomeanconedomain
    /// Appends the primal power cone domain.
    ///
    /// # Arguments
    ///
    /// - `n_` Dimension of the domain.
    /// - `alpha_` The sequence proportional to exponents. Must be positive.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendprimalpowerconedomain>
    #[allow(unused_parens)]
    pub fn append_primal_power_cone_domain(&mut self,n_ : i64,alpha_ : &[f64]) -> Result<i64,String> {
      let nleft_ : i64 = alpha_.len() as i64;
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appendprimalpowerconedomain(self.ptr,n_,nleft_,alpha_.as_ptr(),&mut __tmp_0) },"append_primal_power_cone_domain")?;
      return Result::Ok(__tmp_0);
    } // appendprimalpowerconedomain
    /// Appends the n dimensional quadratic cone domain.
    ///
    /// # Arguments
    ///
    /// - `n_` Dimmension of the domain.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendquadraticconedomain>
    #[allow(unused_parens)]
    pub fn append_quadratic_cone_domain(&mut self,n_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appendquadraticconedomain(self.ptr,n_,&mut __tmp_0) },"append_quadratic_cone_domain")?;
      return Result::Ok(__tmp_0);
    } // appendquadraticconedomain
    /// Appends the n dimensional real number domain.
    ///
    /// # Arguments
    ///
    /// - `n_` Dimmension of the domain.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendrdomain>
    #[allow(unused_parens)]
    pub fn append_r_domain(&mut self,n_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appendrdomain(self.ptr,n_,&mut __tmp_0) },"append_r_domain")?;
      return Result::Ok(__tmp_0);
    } // appendrdomain
    /// Appends the n dimensional negative orthant to the list of domains.
    ///
    /// # Arguments
    ///
    /// - `n_` Dimmension of the domain.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendrminusdomain>
    #[allow(unused_parens)]
    pub fn append_rminus_domain(&mut self,n_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appendrminusdomain(self.ptr,n_,&mut __tmp_0) },"append_rminus_domain")?;
      return Result::Ok(__tmp_0);
    } // appendrminusdomain
    /// Appends the n dimensional positive orthant to the list of domains.
    ///
    /// # Arguments
    ///
    /// - `n_` Dimmension of the domain.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendrplusdomain>
    #[allow(unused_parens)]
    pub fn append_rplus_domain(&mut self,n_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appendrplusdomain(self.ptr,n_,&mut __tmp_0) },"append_rplus_domain")?;
      return Result::Ok(__tmp_0);
    } // appendrplusdomain
    /// Appends the n dimensional rotated quadratic cone domain.
    ///
    /// # Arguments
    ///
    /// - `n_` Dimmension of the domain.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendrquadraticconedomain>
    #[allow(unused_parens)]
    pub fn append_r_quadratic_cone_domain(&mut self,n_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appendrquadraticconedomain(self.ptr,n_,&mut __tmp_0) },"append_r_quadratic_cone_domain")?;
      return Result::Ok(__tmp_0);
    } // appendrquadraticconedomain
    /// Appends the n dimensional 0 domain.
    ///
    /// # Arguments
    ///
    /// - `n_` Dimmension of the domain.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendrzerodomain>
    #[allow(unused_parens)]
    pub fn append_rzero_domain(&mut self,n_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appendrzerodomain(self.ptr,n_,&mut __tmp_0) },"append_rzero_domain")?;
      return Result::Ok(__tmp_0);
    } // appendrzerodomain
    /// Appends a general sparse symmetric matrix to the storage of symmetric matrices.
    ///
    /// # Arguments
    ///
    /// - `dim_` Dimension of the symmetric matrix that is appended.
    /// - `subi_` Row subscript in the triplets.
    /// - `subj_` Column subscripts in the triplets.
    /// - `valij_` Values of each triplet.
    ///
    /// # Returns
    ///
    ///   - `idx` Unique index assigned to the inputted matrix.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendsparsesymmat>
    #[allow(unused_parens)]
    pub fn append_sparse_sym_mat(&mut self,dim_ : i32,subi_ : &[i32],subj_ : &[i32],valij_ : &[f64]) -> Result<i64,String> {
      let nz_ : i64 = std::cmp::min(std::cmp::min(subi_.len(),valij_.len()),subj_.len()) as i64;
      let mut __tmp_0 : i64 = i64::default();
      if subi_.len() != subj_.len() || subi_.len() != valij_.len() { return Err("append_sparse_sym_mat: Mismatching lengths if subi, subj and valij".to_string()); }
      self.handle_res(unsafe { MSK_appendsparsesymmat(self.ptr,dim_,nz_,subi_.as_ptr(),subj_.as_ptr(),valij_.as_ptr(),&mut __tmp_0) },"append_sparse_sym_mat")?;
      return Result::Ok(__tmp_0);
    } // appendsparsesymmat
    /// Appends a general sparse symmetric matrix to the storage of symmetric matrices.
    ///
    /// # Arguments
    ///
    /// - `dims_` Dimensions of the symmetric matrixes.
    /// - `nz_` Number of nonzeros for each matrix.
    /// - `subi_` Row subscript in the triplets.
    /// - `subj_` Column subscripts in the triplets.
    /// - `valij_` Values of each triplet.
    /// - `idx_` Unique index assigned to the inputted matrix.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendsparsesymmatlist>
    #[allow(unused_parens)]
    pub fn append_sparse_sym_mat_list(&mut self,dims_ : &[i32],nz_ : &[i64],subi_ : &[i32],subj_ : &[i32],valij_ : &[f64],idx_ : &mut[i64]) -> Result<(),String> {
      let num_ : i32 = std::cmp::min(dims_.len(),nz_.len()) as i32;
      let mut __tmp_0 : i64 = i64::default();
      for __tmp_1 in nz_ { __tmp_0 += __tmp_1; }
      if subi_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("append_sparse_sym_mat_list: Argument 'subi' has the wrong length, expected __tmp_0".to_string());
      }
      let mut __tmp_2 : i64 = i64::default();
      for __tmp_3 in nz_ { __tmp_2 += __tmp_3; }
      if subj_.len() != (__tmp_2).try_into().unwrap() {
        return Result::Err("append_sparse_sym_mat_list: Argument 'subj' has the wrong length, expected __tmp_2".to_string());
      }
      let mut __tmp_4 : i64 = i64::default();
      for __tmp_5 in nz_ { __tmp_4 += __tmp_5; }
      if valij_.len() != (__tmp_4).try_into().unwrap() {
        return Result::Err("append_sparse_sym_mat_list: Argument 'valij' has the wrong length, expected __tmp_4".to_string());
      }
      if idx_.len() != (num_).try_into().unwrap() {
        return Result::Err("append_sparse_sym_mat_list: Argument 'idx' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_appendsparsesymmatlist(self.ptr,num_,dims_.as_ptr(),nz_.as_ptr(),subi_.as_ptr(),subj_.as_ptr(),valij_.as_ptr(),idx_.as_mut_ptr()) },"append_sparse_sym_mat_list")?;
      return Result::Ok(());
    } // appendsparsesymmatlist
    /// Appends the vectorized SVEC PSD cone domain.
    ///
    /// # Arguments
    ///
    /// - `n_` Dimension of the domain.
    ///
    /// # Returns
    ///
    ///   - `domidx` Index of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendsvecpsdconedomain>
    #[allow(unused_parens)]
    pub fn append_svec_psd_cone_domain(&mut self,n_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appendsvecpsdconedomain(self.ptr,n_,&mut __tmp_0) },"append_svec_psd_cone_domain")?;
      return Result::Ok(__tmp_0);
    } // appendsvecpsdconedomain
    /// Appends a number of variables to the optimization task.
    ///
    /// # Arguments
    ///
    /// - `num_` Number of variables which should be appended.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.appendvars>
    #[allow(unused_parens)]
    pub fn append_vars(&mut self,num_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_appendvars(self.ptr,num_) },"append_vars")?;
      return Result::Ok(());
    } // appendvars
    /// Get the optimizer log from a remote job.
    ///
    /// # Arguments
    ///
    /// - `addr_` Address of the solver server
    /// - `accesstoken_` Access token string or NULL
    /// - `token_` Job token
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.asyncgetlog>
    #[allow(unused_parens)]
    pub fn async_get_log(&mut self,addr_ : &str,accesstoken_ : &str,token_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(addr_).unwrap();
      let __tmp_3 = CString::new(accesstoken_).unwrap();
      let __tmp_5 = CString::new(token_).unwrap();
      self.handle_res(unsafe { MSK_asyncgetlog(self.ptr,__tmp_1.as_ptr(),__tmp_3.as_ptr(),__tmp_5.as_ptr()) },"async_get_log")?;
      return Result::Ok(());
    } // asyncgetlog
    /// Request a solution from a remote job.
    ///
    /// # Arguments
    ///
    /// - `address_` Address of the OptServer.
    /// - `accesstoken_` Access token.
    /// - `token_` The task token.
    /// - `resp_` Is the response code from the remote solver.
    ///   
    ///   See [Rescode]
    /// - `trm_` Is either OK or a termination response code.
    ///   
    ///   See [Rescode]
    ///
    /// # Returns
    ///
    ///   - `respavailable` Indicates if a remote response is available.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.asyncgetresult>
    #[allow(unused_parens)]
    pub fn async_get_result(&mut self,address_ : &str,accesstoken_ : &str,token_ : &str,resp_ : & mut i32,trm_ : & mut i32) -> Result<bool,String> {
      let __tmp_1 = CString::new(address_).unwrap();
      let __tmp_3 = CString::new(accesstoken_).unwrap();
      let __tmp_5 = CString::new(token_).unwrap();
      let mut __tmp_6 : i32 = 0;
      self.handle_res(unsafe { MSK_asyncgetresult(self.ptr,__tmp_1.as_ptr(),__tmp_3.as_ptr(),__tmp_5.as_ptr(),&mut __tmp_6,resp_,trm_) },"async_get_result")?;
      return Result::Ok(__tmp_6 != 0);
    } // asyncgetresult
    /// Offload the optimization task to a solver server in asynchronous mode.
    ///
    /// # Arguments
    ///
    /// - `address_` Address of the OptServer.
    /// - `accesstoken_` Access token.
    ///
    /// # Returns
    ///
    ///   - `token` Returns the task token.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.asyncoptimize>
    #[allow(unused_parens)]
    pub fn async_optimize(&mut self,address_ : &str,accesstoken_ : &str) -> Result<String,String> {
      let __tmp_1 = CString::new(address_).unwrap();
      let __tmp_3 = CString::new(accesstoken_).unwrap();
      let mut token_ = Vec::new(); token_.resize(33 as usize,0);
      self.handle_res(unsafe { MSK_asyncoptimize(self.ptr,__tmp_1.as_ptr(),__tmp_3.as_ptr(),token_.as_mut_ptr()) },"async_optimize")?;
      return Result::Ok(String::from_utf8_lossy(&token_[..token_.iter().position(|&c| c == 0).unwrap_or(33 as usize)]).into_owned());
    } // asyncoptimize
    /// Requests information about the status of the remote job.
    ///
    /// # Arguments
    ///
    /// - `address_` Address of the OptServer.
    /// - `accesstoken_` Access token.
    /// - `token_` The task token.
    /// - `resp_` Is the response code from the remote solver.
    ///   
    ///   See [Rescode]
    /// - `trm_` Is either OK or a termination response code.
    ///   
    ///   See [Rescode]
    ///
    /// # Returns
    ///
    ///   - `respavailable` Indicates if a remote response is available.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.asyncpoll>
    #[allow(unused_parens)]
    pub fn async_poll(&mut self,address_ : &str,accesstoken_ : &str,token_ : &str,resp_ : & mut i32,trm_ : & mut i32) -> Result<bool,String> {
      let __tmp_1 = CString::new(address_).unwrap();
      let __tmp_3 = CString::new(accesstoken_).unwrap();
      let __tmp_5 = CString::new(token_).unwrap();
      let mut __tmp_6 : i32 = 0;
      self.handle_res(unsafe { MSK_asyncpoll(self.ptr,__tmp_1.as_ptr(),__tmp_3.as_ptr(),__tmp_5.as_ptr(),&mut __tmp_6,resp_,trm_) },"async_poll")?;
      return Result::Ok(__tmp_6 != 0);
    } // asyncpoll
    /// Request that the job identified by the token is terminated.
    ///
    /// # Arguments
    ///
    /// - `address_` Address of the OptServer.
    /// - `accesstoken_` Access token.
    /// - `token_` The task token.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.asyncstop>
    #[allow(unused_parens)]
    pub fn async_stop(&mut self,address_ : &str,accesstoken_ : &str,token_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(address_).unwrap();
      let __tmp_3 = CString::new(accesstoken_).unwrap();
      let __tmp_5 = CString::new(token_).unwrap();
      self.handle_res(unsafe { MSK_asyncstop(self.ptr,__tmp_1.as_ptr(),__tmp_3.as_ptr(),__tmp_5.as_ptr()) },"async_stop")?;
      return Result::Ok(());
    } // asyncstop
    /// Computes conditioning information for the basis matrix.
    ///
    /// # Arguments
    ///
    /// - `nrmbasis_` An estimate for the 1-norm of the basis.
    /// - `nrminvbasis_` An estimate for the 1-norm of the inverse of the basis.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.basiscond>
    #[allow(unused_parens)]
    pub fn basis_cond(&mut self,nrmbasis_ : &mut f64,nrminvbasis_ : &mut f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_basiscond(self.ptr,nrmbasis_,nrminvbasis_) },"basis_cond")?;
      return Result::Ok(());
    } // basiscond
    /// Changes the bounds for one constraint.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the constraint for which the bounds should be changed.
    /// - `lower_` If non-zero, then the lower bound is changed, otherwise the upper bound is changed.
    /// - `finite_` If non-zero, then the given value is assumed to be finite.
    /// - `value_` New value for the bound.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.chgconbound>
    #[allow(unused_parens)]
    pub fn chg_con_bound(&mut self,i_ : i32,lower_ : i32,finite_ : i32,value_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_chgconbound(self.ptr,i_,lower_,finite_,value_) },"chg_con_bound")?;
      return Result::Ok(());
    } // chgconbound
    /// Changes the bounds for one variable.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the variable for which the bounds should be changed.
    /// - `lower_` If non-zero, then the lower bound is changed, otherwise the upper bound is changed.
    /// - `finite_` If non-zero, then the given value is assumed to be finite.
    /// - `value_` New value for the bound.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.chgvarbound>
    #[allow(unused_parens)]
    pub fn chg_var_bound(&mut self,j_ : i32,lower_ : i32,finite_ : i32,value_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_chgvarbound(self.ptr,j_,lower_,finite_,value_) },"chg_var_bound")?;
      return Result::Ok(());
    } // chgvarbound
    /// Commits all cached problem changes.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.commitchanges>
    #[allow(unused_parens)]
    pub fn commit_changes(&mut self) -> Result<(),String> {
      self.handle_res(unsafe { MSK_commitchanges(self.ptr) },"commit_changes")?;
      return Result::Ok(());
    } // commitchanges
    /// Undefine a solution and free the memory it uses.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.deletesolution>
    #[allow(unused_parens)]
    pub fn delete_solution(&mut self,whichsol_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_deletesolution(self.ptr,whichsol_) },"delete_solution")?;
      return Result::Ok(());
    } // deletesolution
    /// Performs sensitivity analysis on objective coefficients.
    ///
    /// # Arguments
    ///
    /// - `subj_` Indexes of objective coefficients to analyze.
    /// - `leftpricej_` Left shadow prices for requested coefficients.
    /// - `rightpricej_` Right shadow prices for requested coefficients.
    /// - `leftrangej_` Left range for requested coefficients.
    /// - `rightrangej_` Right range for requested coefficients.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.dualsensitivity>
    #[allow(unused_parens)]
    pub fn dual_sensitivity(&self,subj_ : &[i32],leftpricej_ : &mut[f64],rightpricej_ : &mut[f64],leftrangej_ : &mut[f64],rightrangej_ : &mut[f64]) -> Result<(),String> {
      let numj_ : i32 = subj_.len() as i32;
      if leftpricej_.len() != (numj_).try_into().unwrap() {
        return Result::Err("dual_sensitivity: Argument 'leftpricej' has the wrong length, expected numj_".to_string());
      }
      if rightpricej_.len() != (numj_).try_into().unwrap() {
        return Result::Err("dual_sensitivity: Argument 'rightpricej' has the wrong length, expected numj_".to_string());
      }
      if leftrangej_.len() != (numj_).try_into().unwrap() {
        return Result::Err("dual_sensitivity: Argument 'leftrangej' has the wrong length, expected numj_".to_string());
      }
      if rightrangej_.len() != (numj_).try_into().unwrap() {
        return Result::Err("dual_sensitivity: Argument 'rightrangej' has the wrong length, expected numj_".to_string());
      }
      self.handle_res(unsafe { MSK_dualsensitivity(self.ptr,numj_,subj_.as_ptr(),leftpricej_.as_mut_ptr(),rightpricej_.as_mut_ptr(),leftrangej_.as_mut_ptr(),rightrangej_.as_mut_ptr()) },"dual_sensitivity")?;
      return Result::Ok(());
    } // dualsensitivity
    /// Clears a row in barF
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row index of barF.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.emptyafebarfrow>
    #[allow(unused_parens)]
    pub fn empty_afe_barf_row(&mut self,afeidx_ : i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_emptyafebarfrow(self.ptr,afeidx_) },"empty_afe_barf_row")?;
      return Result::Ok(());
    } // emptyafebarfrow
    /// Clears rows in barF.
    ///
    /// # Arguments
    ///
    /// - `afeidxlist_` Indices of rows in barF to clear.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.emptyafebarfrowlist>
    #[allow(unused_parens)]
    pub fn empty_afe_barf_row_list(&mut self,afeidxlist_ : &[i64]) -> Result<(),String> {
      let numafeidx_ : i64 = afeidxlist_.len() as i64;
      self.handle_res(unsafe { MSK_emptyafebarfrowlist(self.ptr,numafeidx_,afeidxlist_.as_ptr()) },"empty_afe_barf_row_list")?;
      return Result::Ok(());
    } // emptyafebarfrowlist
    /// Clears a column in F.
    ///
    /// # Arguments
    ///
    /// - `varidx_` Variable index.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.emptyafefcol>
    #[allow(unused_parens)]
    pub fn empty_afe_f_col(&mut self,varidx_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_emptyafefcol(self.ptr,varidx_) },"empty_afe_f_col")?;
      return Result::Ok(());
    } // emptyafefcol
    /// Clears columns in F.
    ///
    /// # Arguments
    ///
    /// - `varidx_` Indices of variables in F to clear.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.emptyafefcollist>
    #[allow(unused_parens)]
    pub fn empty_afe_f_col_list(&mut self,varidx_ : &[i32]) -> Result<(),String> {
      let numvaridx_ : i64 = varidx_.len() as i64;
      self.handle_res(unsafe { MSK_emptyafefcollist(self.ptr,numvaridx_,varidx_.as_ptr()) },"empty_afe_f_col_list")?;
      return Result::Ok(());
    } // emptyafefcollist
    /// Clears a row in F.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row index.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.emptyafefrow>
    #[allow(unused_parens)]
    pub fn empty_afe_f_row(&mut self,afeidx_ : i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_emptyafefrow(self.ptr,afeidx_) },"empty_afe_f_row")?;
      return Result::Ok(());
    } // emptyafefrow
    /// Clears rows in F.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Indices of rows in F to clear.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.emptyafefrowlist>
    #[allow(unused_parens)]
    pub fn empty_afe_f_row_list(&mut self,afeidx_ : &[i64]) -> Result<(),String> {
      let numafeidx_ : i64 = afeidx_.len() as i64;
      self.handle_res(unsafe { MSK_emptyafefrowlist(self.ptr,numafeidx_,afeidx_.as_ptr()) },"empty_afe_f_row_list")?;
      return Result::Ok(());
    } // emptyafefrowlist
    /// Evaluates the activity of an affine conic constraint.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `accidx_` The index of the affine conic constraint.
    /// - `activity_` The activity of the affine conic constraint. The array should have length equal to the dimension of the constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.evaluateacc>
    #[allow(unused_parens)]
    pub fn evaluate_acc(&self,whichsol_ : i32,accidx_ : i64,activity_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getaccn(self.ptr,accidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getaccn")?;
      if activity_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("evaluate_acc: Argument 'activity' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_evaluateacc(self.ptr,whichsol_,accidx_,activity_.as_mut_ptr()) },"evaluate_acc")?;
      return Result::Ok(());
    } // evaluateacc
    /// Evaluates the activities of all affine conic constraints.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `activity_` The activity of affine conic constraints. The array should have length equal to the sum of dimensions of all affine conic constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.evaluateaccs>
    #[allow(unused_parens)]
    pub fn evaluate_accs(&self,whichsol_ : i32,activity_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getaccntot(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getaccntot")?;
      if activity_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("evaluate_accs: Argument 'activity' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_evaluateaccs(self.ptr,whichsol_,activity_.as_mut_ptr()) },"evaluate_accs")?;
      return Result::Ok(());
    } // evaluateaccs
    /// Generates systematic names for affine conic constraints.
    ///
    /// # Arguments
    ///
    /// - `sub_` Indexes of the affine conic constraints.
    /// - `fmt_` The variable name formatting string.
    /// - `dims_` Dimensions in the shape.
    /// - `sp_` Items that should be named.
    /// - `namedaxisidxs_` List if named index axes
    /// - `names_` All axis names.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.generateaccnames>
    #[allow(unused_parens)]
    pub fn generate_acc_names(&mut self,sub_ : &[i64],fmt_ : &str,dims_ : &[i32],sp_ : &[i64],namedaxisidxs_ : &[i32],names_ : &[String]) -> Result<(),String> {
      let num_ : i64 = sub_.len() as i64;
      let __tmp_1 = CString::new(fmt_).unwrap();
      let ndims_ : i32 = dims_.len() as i32;
      if sp_.len() != (num_).try_into().unwrap() {
        return Result::Err("generate_acc_names: Argument 'sp' has the wrong length, expected num_".to_string());
      }
      let numnamedaxis_ : i32 = namedaxisidxs_.len() as i32;
      let numnames_ : i64 = names_.len() as i64;
      let cstr_names : Vec<CString> = names_.iter().map(|s| CString::new(s.as_str()).unwrap()).collect();
      let cptr_names : Vec<* const libc::c_char> = cstr_names.iter().map(|s| s.as_ptr()).collect();
      self.handle_res(unsafe { MSK_generateaccnames(self.ptr,num_,sub_.as_ptr(),__tmp_1.as_ptr(),ndims_,dims_.as_ptr(),sp_.as_ptr(),numnamedaxis_,namedaxisidxs_.as_ptr(),numnames_,cptr_names.as_ptr()) },"generate_acc_names")?;
      return Result::Ok(());
    } // generateaccnames
    /// Generates systematic names for variables.
    ///
    /// # Arguments
    ///
    /// - `subj_` Indexes of the variables.
    /// - `fmt_` The variable name formatting string.
    /// - `dims_` Dimensions in the shape.
    /// - `sp_` Items that should be named.
    /// - `namedaxisidxs_` List if named index axes
    /// - `names_` All axis names.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.generatebarvarnames>
    #[allow(unused_parens)]
    pub fn generate_barvar_names(&mut self,subj_ : &[i32],fmt_ : &str,dims_ : &[i32],sp_ : &[i64],namedaxisidxs_ : &[i32],names_ : &[String]) -> Result<(),String> {
      let num_ : i32 = subj_.len() as i32;
      let __tmp_1 = CString::new(fmt_).unwrap();
      let ndims_ : i32 = dims_.len() as i32;
      if sp_.len() != (num_).try_into().unwrap() {
        return Result::Err("generate_barvar_names: Argument 'sp' has the wrong length, expected num_".to_string());
      }
      let numnamedaxis_ : i32 = namedaxisidxs_.len() as i32;
      let numnames_ : i64 = names_.len() as i64;
      let cstr_names : Vec<CString> = names_.iter().map(|s| CString::new(s.as_str()).unwrap()).collect();
      let cptr_names : Vec<* const libc::c_char> = cstr_names.iter().map(|s| s.as_ptr()).collect();
      self.handle_res(unsafe { MSK_generatebarvarnames(self.ptr,num_,subj_.as_ptr(),__tmp_1.as_ptr(),ndims_,dims_.as_ptr(),sp_.as_ptr(),numnamedaxis_,namedaxisidxs_.as_ptr(),numnames_,cptr_names.as_ptr()) },"generate_barvar_names")?;
      return Result::Ok(());
    } // generatebarvarnames
    /// Generates systematic names for cone.
    ///
    /// # Arguments
    ///
    /// - `subk_` Indexes of the cone.
    /// - `fmt_` The cone name formatting string.
    /// - `dims_` Dimensions in the shape.
    /// - `sp_` Items that should be named.
    /// - `namedaxisidxs_` List if named index axes
    /// - `names_` All axis names.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.generateconenames>
    #[allow(unused_parens)]
    pub fn generate_cone_names(&mut self,subk_ : &[i32],fmt_ : &str,dims_ : &[i32],sp_ : &[i64],namedaxisidxs_ : &[i32],names_ : &[String]) -> Result<(),String> {
      let num_ : i32 = subk_.len() as i32;
      let __tmp_1 = CString::new(fmt_).unwrap();
      let ndims_ : i32 = dims_.len() as i32;
      if sp_.len() != (num_).try_into().unwrap() {
        return Result::Err("generate_cone_names: Argument 'sp' has the wrong length, expected num_".to_string());
      }
      let numnamedaxis_ : i32 = namedaxisidxs_.len() as i32;
      let numnames_ : i64 = names_.len() as i64;
      let cstr_names : Vec<CString> = names_.iter().map(|s| CString::new(s.as_str()).unwrap()).collect();
      let cptr_names : Vec<* const libc::c_char> = cstr_names.iter().map(|s| s.as_ptr()).collect();
      self.handle_res(unsafe { MSK_generateconenames(self.ptr,num_,subk_.as_ptr(),__tmp_1.as_ptr(),ndims_,dims_.as_ptr(),sp_.as_ptr(),numnamedaxis_,namedaxisidxs_.as_ptr(),numnames_,cptr_names.as_ptr()) },"generate_cone_names")?;
      return Result::Ok(());
    } // generateconenames
    /// Generates systematic names for constraints.
    ///
    /// # Arguments
    ///
    /// - `subi_` Indexes of the constraints.
    /// - `fmt_` The constraint name formatting string.
    /// - `dims_` Dimensions in the shape.
    /// - `sp_` Items that should be named.
    /// - `namedaxisidxs_` List if named index axes
    /// - `names_` All axis names.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.generateconnames>
    #[allow(unused_parens)]
    pub fn generate_con_names(&mut self,subi_ : &[i32],fmt_ : &str,dims_ : &[i32],sp_ : &[i64],namedaxisidxs_ : &[i32],names_ : &[String]) -> Result<(),String> {
      let num_ : i32 = subi_.len() as i32;
      let __tmp_1 = CString::new(fmt_).unwrap();
      let ndims_ : i32 = dims_.len() as i32;
      if sp_.len() != (num_).try_into().unwrap() {
        return Result::Err("generate_con_names: Argument 'sp' has the wrong length, expected num_".to_string());
      }
      let numnamedaxis_ : i32 = namedaxisidxs_.len() as i32;
      let numnames_ : i64 = names_.len() as i64;
      let cstr_names : Vec<CString> = names_.iter().map(|s| CString::new(s.as_str()).unwrap()).collect();
      let cptr_names : Vec<* const libc::c_char> = cstr_names.iter().map(|s| s.as_ptr()).collect();
      self.handle_res(unsafe { MSK_generateconnames(self.ptr,num_,subi_.as_ptr(),__tmp_1.as_ptr(),ndims_,dims_.as_ptr(),sp_.as_ptr(),numnamedaxis_,namedaxisidxs_.as_ptr(),numnames_,cptr_names.as_ptr()) },"generate_con_names")?;
      return Result::Ok(());
    } // generateconnames
    /// Generates systematic names for affine conic constraints.
    ///
    /// # Arguments
    ///
    /// - `sub_` Indexes of the disjunctive constraints.
    /// - `fmt_` The variable name formatting string.
    /// - `dims_` Dimensions in the shape.
    /// - `sp_` Items that should be named.
    /// - `namedaxisidxs_` List if named index axes
    /// - `names_` All axis names.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.generatedjcnames>
    #[allow(unused_parens)]
    pub fn generate_djc_names(&mut self,sub_ : &[i64],fmt_ : &str,dims_ : &[i32],sp_ : &[i64],namedaxisidxs_ : &[i32],names_ : &[String]) -> Result<(),String> {
      let num_ : i64 = sub_.len() as i64;
      let __tmp_1 = CString::new(fmt_).unwrap();
      let ndims_ : i32 = dims_.len() as i32;
      if sp_.len() != (num_).try_into().unwrap() {
        return Result::Err("generate_djc_names: Argument 'sp' has the wrong length, expected num_".to_string());
      }
      let numnamedaxis_ : i32 = namedaxisidxs_.len() as i32;
      let numnames_ : i64 = names_.len() as i64;
      let cstr_names : Vec<CString> = names_.iter().map(|s| CString::new(s.as_str()).unwrap()).collect();
      let cptr_names : Vec<* const libc::c_char> = cstr_names.iter().map(|s| s.as_ptr()).collect();
      self.handle_res(unsafe { MSK_generatedjcnames(self.ptr,num_,sub_.as_ptr(),__tmp_1.as_ptr(),ndims_,dims_.as_ptr(),sp_.as_ptr(),numnamedaxis_,namedaxisidxs_.as_ptr(),numnames_,cptr_names.as_ptr()) },"generate_djc_names")?;
      return Result::Ok(());
    } // generatedjcnames
    /// Generates systematic names for variables.
    ///
    /// # Arguments
    ///
    /// - `subj_` Indexes of the variables.
    /// - `fmt_` The variable name formatting string.
    /// - `dims_` Dimensions in the shape.
    /// - `sp_` Items that should be named.
    /// - `namedaxisidxs_` List if named index axes
    /// - `names_` All axis names.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.generatevarnames>
    #[allow(unused_parens)]
    pub fn generate_var_names(&mut self,subj_ : &[i32],fmt_ : &str,dims_ : &[i32],sp_ : &[i64],namedaxisidxs_ : &[i32],names_ : &[String]) -> Result<(),String> {
      let num_ : i32 = subj_.len() as i32;
      let __tmp_1 = CString::new(fmt_).unwrap();
      let ndims_ : i32 = dims_.len() as i32;
      if sp_.len() != (num_).try_into().unwrap() {
        return Result::Err("generate_var_names: Argument 'sp' has the wrong length, expected num_".to_string());
      }
      let numnamedaxis_ : i32 = namedaxisidxs_.len() as i32;
      let numnames_ : i64 = names_.len() as i64;
      let cstr_names : Vec<CString> = names_.iter().map(|s| CString::new(s.as_str()).unwrap()).collect();
      let cptr_names : Vec<* const libc::c_char> = cstr_names.iter().map(|s| s.as_ptr()).collect();
      self.handle_res(unsafe { MSK_generatevarnames(self.ptr,num_,subj_.as_ptr(),__tmp_1.as_ptr(),ndims_,dims_.as_ptr(),sp_.as_ptr(),numnamedaxis_,namedaxisidxs_.as_ptr(),numnames_,cptr_names.as_ptr()) },"generate_var_names")?;
      return Result::Ok(());
    } // generatevarnames
    /// Obtains the list of affine expressions appearing in the affine conic constraint.
    ///
    /// # Arguments
    ///
    /// - `accidx_` Index of the affine conic constraint.
    /// - `afeidxlist_` List of indexes of affine expressions appearing in the constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccafeidxlist>
    #[allow(unused_parens)]
    pub fn get_acc_afe_idx_list(&self,accidx_ : i64,afeidxlist_ : &mut[i64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getaccn(self.ptr,accidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getaccn")?;
      if afeidxlist_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_acc_afe_idx_list: Argument 'afeidxlist' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getaccafeidxlist(self.ptr,accidx_,afeidxlist_.as_mut_ptr()) },"get_acc_afe_idx_list")?;
      return Result::Ok(());
    } // getaccafeidxlist
    /// Obtains the additional constant term vector appearing in the affine conic constraint.
    ///
    /// # Arguments
    ///
    /// - `accidx_` Index of the affine conic constraint.
    /// - `b_` The vector b appearing in the constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccb>
    #[allow(unused_parens)]
    pub fn get_acc_b(&self,accidx_ : i64,b_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getaccn(self.ptr,accidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getaccn")?;
      if b_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_acc_b: Argument 'b' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getaccb(self.ptr,accidx_,b_.as_mut_ptr()) },"get_acc_b")?;
      return Result::Ok(());
    } // getaccb
    /// Obtains barF, implied by the ACCs, in block triplet form.
    ///
    /// # Arguments
    ///
    /// - `acc_afe_` Index of the AFE within the concatenated list of AFEs in ACCs.
    /// - `bar_var_` Symmetric matrix variable index.
    /// - `blk_row_` Block row index.
    /// - `blk_col_` Block column index.
    /// - `blk_val_` The numerical value associated with each block triplet.
    ///
    /// # Returns
    ///
    ///   - `numtrip` Number of elements in the block triplet form.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccbarfblocktriplet>
    #[allow(unused_parens)]
    pub fn get_acc_barf_block_triplet(&self,acc_afe_ : &mut[i64],bar_var_ : &mut[i32],blk_row_ : &mut[i32],blk_col_ : &mut[i32],blk_val_ : &mut[f64]) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getaccbarfnumblocktriplets(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getaccbarfnumblocktriplets")?;
      let maxnumtrip_ : i64 = __tmp_0;
      let mut __tmp_2 : i64 = i64::default();
      if acc_afe_.len() != (maxnumtrip_).try_into().unwrap() {
        return Result::Err("get_acc_barf_block_triplet: Argument 'acc_afe' has the wrong length, expected maxnumtrip_".to_string());
      }
      if bar_var_.len() != (maxnumtrip_).try_into().unwrap() {
        return Result::Err("get_acc_barf_block_triplet: Argument 'bar_var' has the wrong length, expected maxnumtrip_".to_string());
      }
      if blk_row_.len() != (maxnumtrip_).try_into().unwrap() {
        return Result::Err("get_acc_barf_block_triplet: Argument 'blk_row' has the wrong length, expected maxnumtrip_".to_string());
      }
      if blk_col_.len() != (maxnumtrip_).try_into().unwrap() {
        return Result::Err("get_acc_barf_block_triplet: Argument 'blk_col' has the wrong length, expected maxnumtrip_".to_string());
      }
      if blk_val_.len() != (maxnumtrip_).try_into().unwrap() {
        return Result::Err("get_acc_barf_block_triplet: Argument 'blk_val' has the wrong length, expected maxnumtrip_".to_string());
      }
      self.handle_res(unsafe { MSK_getaccbarfblocktriplet(self.ptr,maxnumtrip_,&mut __tmp_2,acc_afe_.as_mut_ptr(),bar_var_.as_mut_ptr(),blk_row_.as_mut_ptr(),blk_col_.as_mut_ptr(),blk_val_.as_mut_ptr()) },"get_acc_barf_block_triplet")?;
      return Result::Ok(__tmp_2);
    } // getaccbarfblocktriplet
    /// Obtains an upper bound on the number of elements in the block triplet form of barf, as used within the ACCs.
    ///
    /// # Returns
    ///
    ///   - `numtrip` An upper bound on the number of elements in the block triplet form of barf, as used within the ACCs.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccbarfnumblocktriplets>
    #[allow(unused_parens)]
    pub fn get_acc_barf_num_block_triplets(&self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getaccbarfnumblocktriplets(self.ptr,&mut __tmp_0) },"get_acc_barf_num_block_triplets")?;
      return Result::Ok(__tmp_0);
    } // getaccbarfnumblocktriplets
    /// Obtains the domain appearing in the affine conic constraint.
    ///
    /// # Arguments
    ///
    /// - `accidx_` The index of the affine conic constraint.
    /// - `domidx_` The index of domain in the affine conic constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccdomain>
    #[allow(unused_parens)]
    pub fn get_acc_domain(&mut self,accidx_ : i64,domidx_ : &mut i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getaccdomain(self.ptr,accidx_,domidx_) },"get_acc_domain")?;
      return Result::Ok(());
    } // getaccdomain
    /// Obtains the doty vector for an affine conic constraint.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `accidx_` The index of the affine conic constraint.
    /// - `doty_` The dual values for this affine conic constraint. The array should have length equal to the dimension of the constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccdoty>
    #[allow(unused_parens)]
    pub fn get_acc_dot_y(&self,whichsol_ : i32,accidx_ : i64,doty_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getaccn(self.ptr,accidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getaccn")?;
      if doty_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_acc_dot_y: Argument 'doty' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getaccdoty(self.ptr,whichsol_,accidx_,doty_.as_mut_ptr()) },"get_acc_dot_y")?;
      return Result::Ok(());
    } // getaccdoty
    /// Obtains the doty vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `doty_` The dual values of affine conic constraints. The array should have length equal to the sum of dimensions of all affine conic constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccdotys>
    #[allow(unused_parens)]
    pub fn get_acc_dot_y_s(&self,whichsol_ : i32,doty_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getaccntot(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getaccntot")?;
      if doty_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_acc_dot_y_s: Argument 'doty' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getaccdotys(self.ptr,whichsol_,doty_.as_mut_ptr()) },"get_acc_dot_y_s")?;
      return Result::Ok(());
    } // getaccdotys
    /// Obtains the total number of nonzeros in the ACC implied F matrix.
    ///
    /// # Returns
    ///
    ///   - `accfnnz` Number of nonzeros in the F matrix implied by ACCs.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccfnumnz>
    #[allow(unused_parens)]
    pub fn get_acc_f_numnz(&mut self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getaccfnumnz(self.ptr,&mut __tmp_0) },"get_acc_f_numnz")?;
      return Result::Ok(__tmp_0);
    } // getaccfnumnz
    /// Obtains the F matrix (implied by the AFE ordering within the ACCs) in triplet format.
    ///
    /// # Arguments
    ///
    /// - `frow_` Row indices of nonzeros in the implied F matrix.
    /// - `fcol_` Column indices of nonzeros in the implied F matrix.
    /// - `fval_` Values of nonzero entries in the implied F matrix.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccftrip>
    #[allow(unused_parens)]
    pub fn get_acc_f_trip(&mut self,frow_ : &mut[i64],fcol_ : &mut[i32],fval_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getaccfnumnz(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getaccfnumnz")?;
      if frow_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_acc_f_trip: Argument 'frow' has the wrong length, expected __tmp_0".to_string());
      }
      let mut __tmp_2 : i64 = i64::default();
      let __tmp_3 = unsafe { MSK_getaccfnumnz(self.ptr,&mut __tmp_2) };let _ = self.handle_res(__tmp_3,"getaccfnumnz")?;
      if fcol_.len() != (__tmp_2).try_into().unwrap() {
        return Result::Err("get_acc_f_trip: Argument 'fcol' has the wrong length, expected __tmp_2".to_string());
      }
      let mut __tmp_4 : i64 = i64::default();
      let __tmp_5 = unsafe { MSK_getaccfnumnz(self.ptr,&mut __tmp_4) };let _ = self.handle_res(__tmp_5,"getaccfnumnz")?;
      if fval_.len() != (__tmp_4).try_into().unwrap() {
        return Result::Err("get_acc_f_trip: Argument 'fval' has the wrong length, expected __tmp_4".to_string());
      }
      self.handle_res(unsafe { MSK_getaccftrip(self.ptr,frow_.as_mut_ptr(),fcol_.as_mut_ptr(),fval_.as_mut_ptr()) },"get_acc_f_trip")?;
      return Result::Ok(());
    } // getaccftrip
    /// The g vector as used within the ACCs.
    ///
    /// # Arguments
    ///
    /// - `g_` The g vector as used within the ACCs.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccgvector>
    #[allow(unused_parens)]
    pub fn get_acc_g_vector(&self,g_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getaccntot(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getaccntot")?;
      if g_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_acc_g_vector: Argument 'g' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getaccgvector(self.ptr,g_.as_mut_ptr()) },"get_acc_g_vector")?;
      return Result::Ok(());
    } // getaccgvector
    /// Obtains the dimension of the affine conic constraint.
    ///
    /// # Arguments
    ///
    /// - `accidx_` The index of the affine conic constraint.
    ///
    /// # Returns
    ///
    ///   - `n` The dimension of the affine conic constraint (equal to the dimension of its domain).
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccn>
    #[allow(unused_parens)]
    pub fn get_acc_n(&mut self,accidx_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getaccn(self.ptr,accidx_,&mut __tmp_0) },"get_acc_n")?;
      return Result::Ok(__tmp_0);
    } // getaccn
    /// Obtains the name of an affine conic constraint.
    ///
    /// # Arguments
    ///
    /// - `accidx_` Index of an affine conic constraint.
    ///
    /// # Returns
    ///
    ///   - `name` Returns the required name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccname>
    #[allow(unused_parens)]
    pub fn get_acc_name(&self,accidx_ : i64) -> Result<String,String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getaccnamelen(self.ptr,accidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getaccnamelen")?;
      let sizename_ : i32 = (1+__tmp_0);
      let mut name_ = Vec::new(); name_.resize(sizename_ as usize,0);
      self.handle_res(unsafe { MSK_getaccname(self.ptr,accidx_,sizename_,name_.as_mut_ptr()) },"get_acc_name")?;
      return Result::Ok(String::from_utf8_lossy(&name_[..name_.iter().position(|&c| c == 0).unwrap_or(sizename_ as usize)]).into_owned());
    } // getaccname
    /// Obtains the length of the name of an affine conic constraint.
    ///
    /// # Arguments
    ///
    /// - `accidx_` Index of an affine conic constraint.
    ///
    /// # Returns
    ///
    ///   - `len` Returns the length of the indicated name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccnamelen>
    #[allow(unused_parens)]
    pub fn get_acc_name_len(&self,accidx_ : i64) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getaccnamelen(self.ptr,accidx_,&mut __tmp_0) },"get_acc_name_len")?;
      return Result::Ok(__tmp_0);
    } // getaccnamelen
    /// Obtains the total dimension of all affine conic constraints.
    ///
    /// # Returns
    ///
    ///   - `n` The total dimension of all affine conic constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccntot>
    #[allow(unused_parens)]
    pub fn get_acc_n_tot(&mut self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getaccntot(self.ptr,&mut __tmp_0) },"get_acc_n_tot")?;
      return Result::Ok(__tmp_0);
    } // getaccntot
    /// Obtains full data of all affine conic constraints.
    ///
    /// # Arguments
    ///
    /// - `domidxlist_` The list of domains appearing in all affine conic constraints.
    /// - `afeidxlist_` The concatenation of index lists of affine expressions appearing in all affine conic constraints.
    /// - `b_` The concatenation of vectors b appearing in all affine conic constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaccs>
    #[allow(unused_parens)]
    pub fn get_accs(&self,domidxlist_ : &mut[i64],afeidxlist_ : &mut[i64],b_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getnumacc(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumacc")?;
      if domidxlist_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_accs: Argument 'domidxlist' has the wrong length, expected __tmp_0".to_string());
      }
      let mut __tmp_2 : i64 = i64::default();
      let __tmp_3 = unsafe { MSK_getaccntot(self.ptr,&mut __tmp_2) };let _ = self.handle_res(__tmp_3,"getaccntot")?;
      if afeidxlist_.len() != (__tmp_2).try_into().unwrap() {
        return Result::Err("get_accs: Argument 'afeidxlist' has the wrong length, expected __tmp_2".to_string());
      }
      let mut __tmp_4 : i64 = i64::default();
      let __tmp_5 = unsafe { MSK_getaccntot(self.ptr,&mut __tmp_4) };let _ = self.handle_res(__tmp_5,"getaccntot")?;
      if b_.len() != (__tmp_4).try_into().unwrap() {
        return Result::Err("get_accs: Argument 'b' has the wrong length, expected __tmp_4".to_string());
      }
      self.handle_res(unsafe { MSK_getaccs(self.ptr,domidxlist_.as_mut_ptr(),afeidxlist_.as_mut_ptr(),b_.as_mut_ptr()) },"get_accs")?;
      return Result::Ok(());
    } // getaccs
    /// Obtains one column of the linear constraint matrix.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the column.
    /// - `nzj_` Number of non-zeros in the column obtained.
    /// - `subj_` Row indices of the non-zeros in the column obtained.
    /// - `valj_` Numerical values in the column obtained.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getacol>
    #[allow(unused_parens)]
    pub fn get_a_col(&self,j_ : i32,nzj_ : &mut i32,subj_ : &mut[i32],valj_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_1 : i32 = i32::default();
      let __tmp_2 = unsafe { MSK_getacolnumnz(self.ptr,j_,&mut __tmp_1) };let _ = self.handle_res(__tmp_2,"getacolnumnz")?;
      if subj_.len() != (__tmp_1).try_into().unwrap() {
        return Result::Err("get_a_col: Argument 'subj' has the wrong length, expected __tmp_1".to_string());
      }
      let mut __tmp_3 : i32 = i32::default();
      let __tmp_4 = unsafe { MSK_getacolnumnz(self.ptr,j_,&mut __tmp_3) };let _ = self.handle_res(__tmp_4,"getacolnumnz")?;
      if valj_.len() != (__tmp_3).try_into().unwrap() {
        return Result::Err("get_a_col: Argument 'valj' has the wrong length, expected __tmp_3".to_string());
      }
      self.handle_res(unsafe { MSK_getacol(self.ptr,j_,nzj_,subj_.as_mut_ptr(),valj_.as_mut_ptr()) },"get_a_col")?;
      return Result::Ok(());
    } // getacol
    /// Obtains the number of non-zero elements in one column of the linear constraint matrix
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the column.
    ///
    /// # Returns
    ///
    ///   - `nzj` Number of non-zeros in the j'th column of (A).
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getacolnumnz>
    #[allow(unused_parens)]
    pub fn get_a_col_num_nz(&self,i_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getacolnumnz(self.ptr,i_,&mut __tmp_0) },"get_a_col_num_nz")?;
      return Result::Ok(__tmp_0);
    } // getacolnumnz
    /// Obtains a sequence of columns from the coefficient matrix.
    ///
    /// # Arguments
    ///
    /// - `first_` Index of the first column in the sequence.
    /// - `last_` Index of the last column in the sequence plus one.
    /// - `ptrb_` Column start pointers.
    /// - `ptre_` Column end pointers.
    /// - `sub_` Contains the row subscripts.
    /// - `val_` Contains the coefficient values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getacolslice64>
    #[allow(unused_parens)]
    pub fn get_a_col_slice(&self,first_ : i32,last_ : i32,ptrb_ : &mut[i64],ptre_ : &mut[i64],sub_ : &mut[i32],val_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getacolslicenumnz64(self.ptr,first_,last_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getacolslicenumnz64")?;
      let maxnumnz_ : i64 = __tmp_0;
      if ptrb_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("get_a_col_slice: Argument 'ptrb' has the wrong length, expected (last_-first_)".to_string());
      }
      if ptre_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("get_a_col_slice: Argument 'ptre' has the wrong length, expected (last_-first_)".to_string());
      }
      if sub_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("get_a_col_slice: Argument 'sub' has the wrong length, expected maxnumnz_".to_string());
      }
      if val_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("get_a_col_slice: Argument 'val' has the wrong length, expected maxnumnz_".to_string());
      }
      self.handle_res(unsafe { MSK_getacolslice64(self.ptr,first_,last_,maxnumnz_,ptrb_.as_mut_ptr(),ptre_.as_mut_ptr(),sub_.as_mut_ptr(),val_.as_mut_ptr()) },"get_a_col_slice")?;
      return Result::Ok(());
    } // getacolslice64
    /// Obtains the number of non-zeros in a slice of columns of the coefficient matrix.
    ///
    /// # Arguments
    ///
    /// - `first_` Index of the first column in the sequence.
    /// - `last_` Index of the last column plus one in the sequence.
    ///
    /// # Returns
    ///
    ///   - `numnz` Number of non-zeros in the slice.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getacolslicenumnz64>
    #[allow(unused_parens)]
    pub fn get_a_col_slice_num_nz(&self,first_ : i32,last_ : i32) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getacolslicenumnz64(self.ptr,first_,last_,&mut __tmp_0) },"get_a_col_slice_num_nz")?;
      return Result::Ok(__tmp_0);
    } // getacolslicenumnz64
    /// Obtains a sequence of columns from the coefficient matrix in triplet format.
    ///
    /// # Arguments
    ///
    /// - `first_` Index of the first column in the sequence.
    /// - `last_` Index of the last column in the sequence plus one.
    /// - `subi_` Constraint subscripts.
    /// - `subj_` Column subscripts.
    /// - `val_` Values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getacolslicetrip>
    #[allow(unused_parens)]
    pub fn get_a_col_slice_trip(&self,first_ : i32,last_ : i32,subi_ : &mut[i32],subj_ : &mut[i32],val_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getacolslicenumnz64(self.ptr,first_,last_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getacolslicenumnz64")?;
      let maxnumnz_ : i64 = __tmp_0;
      if subi_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("get_a_col_slice_trip: Argument 'subi' has the wrong length, expected maxnumnz_".to_string());
      }
      if subj_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("get_a_col_slice_trip: Argument 'subj' has the wrong length, expected maxnumnz_".to_string());
      }
      if val_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("get_a_col_slice_trip: Argument 'val' has the wrong length, expected maxnumnz_".to_string());
      }
      self.handle_res(unsafe { MSK_getacolslicetrip(self.ptr,first_,last_,maxnumnz_,subi_.as_mut_ptr(),subj_.as_mut_ptr(),val_.as_mut_ptr()) },"get_a_col_slice_trip")?;
      return Result::Ok(());
    } // getacolslicetrip
    /// Obtains barF in block triplet form.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Constraint index.
    /// - `barvaridx_` Symmetric matrix variable index.
    /// - `subk_` Block row index.
    /// - `subl_` Block column index.
    /// - `valkl_` The numerical value associated with each block triplet.
    ///
    /// # Returns
    ///
    ///   - `numtrip` Number of elements in the block triplet form.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getafebarfblocktriplet>
    #[allow(unused_parens)]
    pub fn get_afe_barf_block_triplet(&self,afeidx_ : &mut[i64],barvaridx_ : &mut[i32],subk_ : &mut[i32],subl_ : &mut[i32],valkl_ : &mut[f64]) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getafebarfnumblocktriplets(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getafebarfnumblocktriplets")?;
      let maxnumtrip_ : i64 = __tmp_0;
      let mut __tmp_2 : i64 = i64::default();
      if afeidx_.len() != (maxnumtrip_).try_into().unwrap() {
        return Result::Err("get_afe_barf_block_triplet: Argument 'afeidx' has the wrong length, expected maxnumtrip_".to_string());
      }
      if barvaridx_.len() != (maxnumtrip_).try_into().unwrap() {
        return Result::Err("get_afe_barf_block_triplet: Argument 'barvaridx' has the wrong length, expected maxnumtrip_".to_string());
      }
      if subk_.len() != (maxnumtrip_).try_into().unwrap() {
        return Result::Err("get_afe_barf_block_triplet: Argument 'subk' has the wrong length, expected maxnumtrip_".to_string());
      }
      if subl_.len() != (maxnumtrip_).try_into().unwrap() {
        return Result::Err("get_afe_barf_block_triplet: Argument 'subl' has the wrong length, expected maxnumtrip_".to_string());
      }
      if valkl_.len() != (maxnumtrip_).try_into().unwrap() {
        return Result::Err("get_afe_barf_block_triplet: Argument 'valkl' has the wrong length, expected maxnumtrip_".to_string());
      }
      self.handle_res(unsafe { MSK_getafebarfblocktriplet(self.ptr,maxnumtrip_,&mut __tmp_2,afeidx_.as_mut_ptr(),barvaridx_.as_mut_ptr(),subk_.as_mut_ptr(),subl_.as_mut_ptr(),valkl_.as_mut_ptr()) },"get_afe_barf_block_triplet")?;
      return Result::Ok(__tmp_2);
    } // getafebarfblocktriplet
    /// Obtains an upper bound on the number of elements in the block triplet form of barf.
    ///
    /// # Returns
    ///
    ///   - `numtrip` An upper bound on the number of elements in the block triplet form of barf.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getafebarfnumblocktriplets>
    #[allow(unused_parens)]
    pub fn get_afe_barf_num_block_triplets(&self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getafebarfnumblocktriplets(self.ptr,&mut __tmp_0) },"get_afe_barf_num_block_triplets")?;
      return Result::Ok(__tmp_0);
    } // getafebarfnumblocktriplets
    /// Obtains the number of nonzero entries in a row of barF.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row index of barF.
    ///
    /// # Returns
    ///
    ///   - `numentr` Number of nonzero entries in a row of barF.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getafebarfnumrowentries>
    #[allow(unused_parens)]
    pub fn get_afe_barf_num_row_entries(&mut self,afeidx_ : i64) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getafebarfnumrowentries(self.ptr,afeidx_,&mut __tmp_0) },"get_afe_barf_num_row_entries")?;
      return Result::Ok(__tmp_0);
    } // getafebarfnumrowentries
    /// Obtains nonzero entries in one row of barF.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row index of barF.
    /// - `barvaridx_` Semidefinite variable indices.
    /// - `ptrterm_` Pointers to the description of entries.
    /// - `numterm_` Number of terms in each entry.
    /// - `termidx_` Indices of semidefinite matrices from E.
    /// - `termweight_` Weights appearing in the weighted sum representation.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getafebarfrow>
    #[allow(unused_parens)]
    pub fn get_afe_barf_row(&mut self,afeidx_ : i64,barvaridx_ : &mut[i32],ptrterm_ : &mut[i64],numterm_ : &mut[i64],termidx_ : &mut[i64],termweight_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let mut __tmp_1 : i64 = i64::default();
      let __tmp_2 = unsafe { MSK_getafebarfrowinfo(self.ptr,afeidx_,&mut __tmp_0,&mut __tmp_1) };let _ = self.handle_res(__tmp_2,"getafebarfrowinfo")?;
      if barvaridx_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_afe_barf_row: Argument 'barvaridx' has the wrong length, expected __tmp_0".to_string());
      }
      let mut __tmp_3 : i32 = i32::default();
      let mut __tmp_4 : i64 = i64::default();
      let __tmp_5 = unsafe { MSK_getafebarfrowinfo(self.ptr,afeidx_,&mut __tmp_3,&mut __tmp_4) };let _ = self.handle_res(__tmp_5,"getafebarfrowinfo")?;
      if ptrterm_.len() != (__tmp_3).try_into().unwrap() {
        return Result::Err("get_afe_barf_row: Argument 'ptrterm' has the wrong length, expected __tmp_3".to_string());
      }
      let mut __tmp_6 : i32 = i32::default();
      let mut __tmp_7 : i64 = i64::default();
      let __tmp_8 = unsafe { MSK_getafebarfrowinfo(self.ptr,afeidx_,&mut __tmp_6,&mut __tmp_7) };let _ = self.handle_res(__tmp_8,"getafebarfrowinfo")?;
      if numterm_.len() != (__tmp_6).try_into().unwrap() {
        return Result::Err("get_afe_barf_row: Argument 'numterm' has the wrong length, expected __tmp_6".to_string());
      }
      let mut __tmp_9 : i32 = i32::default();
      let mut __tmp_10 : i64 = i64::default();
      let __tmp_11 = unsafe { MSK_getafebarfrowinfo(self.ptr,afeidx_,&mut __tmp_9,&mut __tmp_10) };let _ = self.handle_res(__tmp_11,"getafebarfrowinfo")?;
      if termidx_.len() != (__tmp_10).try_into().unwrap() {
        return Result::Err("get_afe_barf_row: Argument 'termidx' has the wrong length, expected __tmp_10".to_string());
      }
      let mut __tmp_12 : i32 = i32::default();
      let mut __tmp_13 : i64 = i64::default();
      let __tmp_14 = unsafe { MSK_getafebarfrowinfo(self.ptr,afeidx_,&mut __tmp_12,&mut __tmp_13) };let _ = self.handle_res(__tmp_14,"getafebarfrowinfo")?;
      if termweight_.len() != (__tmp_13).try_into().unwrap() {
        return Result::Err("get_afe_barf_row: Argument 'termweight' has the wrong length, expected __tmp_13".to_string());
      }
      self.handle_res(unsafe { MSK_getafebarfrow(self.ptr,afeidx_,barvaridx_.as_mut_ptr(),ptrterm_.as_mut_ptr(),numterm_.as_mut_ptr(),termidx_.as_mut_ptr(),termweight_.as_mut_ptr()) },"get_afe_barf_row")?;
      return Result::Ok(());
    } // getafebarfrow
    /// Obtains information about one row of barF.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row index of barF.
    /// - `numentr_` Number of nonzero entries in a row of barF.
    /// - `numterm_` Number of terms in the weighted sums representation of the row of barF.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getafebarfrowinfo>
    #[allow(unused_parens)]
    pub fn get_afe_barf_row_info(&mut self,afeidx_ : i64,numentr_ : &mut i32,numterm_ : &mut i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getafebarfrowinfo(self.ptr,afeidx_,numentr_,numterm_) },"get_afe_barf_row_info")?;
      return Result::Ok(());
    } // getafebarfrowinfo
    /// Obtains the total number of nonzeros in F.
    ///
    /// # Returns
    ///
    ///   - `numnz` Number of nonzeros in F.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getafefnumnz>
    #[allow(unused_parens)]
    pub fn get_afe_f_num_nz(&mut self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getafefnumnz(self.ptr,&mut __tmp_0) },"get_afe_f_num_nz")?;
      return Result::Ok(__tmp_0);
    } // getafefnumnz
    /// Obtains one row of F in sparse format.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row index.
    /// - `numnz_` Number of non-zeros in the row obtained.
    /// - `varidx_` Column indices of the non-zeros in the row obtained.
    /// - `val_` Values of the non-zeros in the row obtained.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getafefrow>
    #[allow(unused_parens)]
    pub fn get_afe_f_row(&mut self,afeidx_ : i64,numnz_ : &mut i32,varidx_ : &mut[i32],val_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_1 : i32 = i32::default();
      let __tmp_2 = unsafe { MSK_getafefrownumnz(self.ptr,afeidx_,&mut __tmp_1) };let _ = self.handle_res(__tmp_2,"getafefrownumnz")?;
      if varidx_.len() != (__tmp_1).try_into().unwrap() {
        return Result::Err("get_afe_f_row: Argument 'varidx' has the wrong length, expected __tmp_1".to_string());
      }
      let mut __tmp_3 : i32 = i32::default();
      let __tmp_4 = unsafe { MSK_getafefrownumnz(self.ptr,afeidx_,&mut __tmp_3) };let _ = self.handle_res(__tmp_4,"getafefrownumnz")?;
      if val_.len() != (__tmp_3).try_into().unwrap() {
        return Result::Err("get_afe_f_row: Argument 'val' has the wrong length, expected __tmp_3".to_string());
      }
      self.handle_res(unsafe { MSK_getafefrow(self.ptr,afeidx_,numnz_,varidx_.as_mut_ptr(),val_.as_mut_ptr()) },"get_afe_f_row")?;
      return Result::Ok(());
    } // getafefrow
    /// Obtains the number of nonzeros in a row of F.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row index.
    ///
    /// # Returns
    ///
    ///   - `numnz` Number of non-zeros in the row.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getafefrownumnz>
    #[allow(unused_parens)]
    pub fn get_afe_f_row_num_nz(&mut self,afeidx_ : i64) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getafefrownumnz(self.ptr,afeidx_,&mut __tmp_0) },"get_afe_f_row_num_nz")?;
      return Result::Ok(__tmp_0);
    } // getafefrownumnz
    /// Obtains the F matrix in triplet format.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row indices of nonzeros.
    /// - `varidx_` Column indices of nonzeros.
    /// - `val_` Values of nonzero entries.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getafeftrip>
    #[allow(unused_parens)]
    pub fn get_afe_f_trip(&mut self,afeidx_ : &mut[i64],varidx_ : &mut[i32],val_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getafefnumnz(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getafefnumnz")?;
      if afeidx_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_afe_f_trip: Argument 'afeidx' has the wrong length, expected __tmp_0".to_string());
      }
      let mut __tmp_2 : i64 = i64::default();
      let __tmp_3 = unsafe { MSK_getafefnumnz(self.ptr,&mut __tmp_2) };let _ = self.handle_res(__tmp_3,"getafefnumnz")?;
      if varidx_.len() != (__tmp_2).try_into().unwrap() {
        return Result::Err("get_afe_f_trip: Argument 'varidx' has the wrong length, expected __tmp_2".to_string());
      }
      let mut __tmp_4 : i64 = i64::default();
      let __tmp_5 = unsafe { MSK_getafefnumnz(self.ptr,&mut __tmp_4) };let _ = self.handle_res(__tmp_5,"getafefnumnz")?;
      if val_.len() != (__tmp_4).try_into().unwrap() {
        return Result::Err("get_afe_f_trip: Argument 'val' has the wrong length, expected __tmp_4".to_string());
      }
      self.handle_res(unsafe { MSK_getafeftrip(self.ptr,afeidx_.as_mut_ptr(),varidx_.as_mut_ptr(),val_.as_mut_ptr()) },"get_afe_f_trip")?;
      return Result::Ok(());
    } // getafeftrip
    /// Obtains a single coefficient in g.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Element index.
    ///
    /// # Returns
    ///
    ///   - `g` The entry in g.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getafeg>
    #[allow(unused_parens)]
    pub fn get_afe_g(&mut self,afeidx_ : i64) -> Result<f64,String> {
      let mut __tmp_0 : f64 = f64::default();
      self.handle_res(unsafe { MSK_getafeg(self.ptr,afeidx_,&mut __tmp_0) },"get_afe_g")?;
      return Result::Ok(__tmp_0);
    } // getafeg
    /// Obtains a sequence of coefficients from the vector g.
    ///
    /// # Arguments
    ///
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `g_` The slice of g as a dense vector.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getafegslice>
    #[allow(unused_parens)]
    pub fn get_afe_g_slice(&self,first_ : i64,last_ : i64,g_ : &mut[f64]) -> Result<(),String> {
      if g_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("get_afe_g_slice: Argument 'g' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getafegslice(self.ptr,first_,last_,g_.as_mut_ptr()) },"get_afe_g_slice")?;
      return Result::Ok(());
    } // getafegslice
    /// Obtains a single coefficient in linear constraint matrix.
    ///
    /// # Arguments
    ///
    /// - `i_` Row index of the coefficient to be returned.
    /// - `j_` Column index of the coefficient to be returned.
    ///
    /// # Returns
    ///
    ///   - `aij` Returns the requested coefficient.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getaij>
    #[allow(unused_parens)]
    pub fn get_aij(&self,i_ : i32,j_ : i32) -> Result<f64,String> {
      let mut __tmp_0 : f64 = f64::default();
      self.handle_res(unsafe { MSK_getaij(self.ptr,i_,j_,&mut __tmp_0) },"get_aij")?;
      return Result::Ok(__tmp_0);
    } // getaij
    /// Obtains the number non-zeros in a rectangular piece of the linear constraint matrix.
    ///
    /// # Arguments
    ///
    /// - `firsti_` Index of the first row in the rectangular piece.
    /// - `lasti_` Index of the last row plus one in the rectangular piece.
    /// - `firstj_` Index of the first column in the rectangular piece.
    /// - `lastj_` Index of the last column plus one in the rectangular piece.
    ///
    /// # Returns
    ///
    ///   - `numnz` Number of non-zero elements in the rectangular piece of the linear constraint matrix.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getapiecenumnz>
    #[allow(unused_parens)]
    pub fn get_a_piece_num_nz(&self,firsti_ : i32,lasti_ : i32,firstj_ : i32,lastj_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getapiecenumnz(self.ptr,firsti_,lasti_,firstj_,lastj_,&mut __tmp_0) },"get_a_piece_num_nz")?;
      return Result::Ok(__tmp_0);
    } // getapiecenumnz
    /// Obtains one row of the linear constraint matrix.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the row.
    /// - `nzi_` Number of non-zeros in the row obtained.
    /// - `subi_` Column indices of the non-zeros in the row obtained.
    /// - `vali_` Numerical values of the row obtained.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getarow>
    #[allow(unused_parens)]
    pub fn get_a_row(&self,i_ : i32,nzi_ : &mut i32,subi_ : &mut[i32],vali_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_1 : i32 = i32::default();
      let __tmp_2 = unsafe { MSK_getarownumnz(self.ptr,i_,&mut __tmp_1) };let _ = self.handle_res(__tmp_2,"getarownumnz")?;
      if subi_.len() != (__tmp_1).try_into().unwrap() {
        return Result::Err("get_a_row: Argument 'subi' has the wrong length, expected __tmp_1".to_string());
      }
      let mut __tmp_3 : i32 = i32::default();
      let __tmp_4 = unsafe { MSK_getarownumnz(self.ptr,i_,&mut __tmp_3) };let _ = self.handle_res(__tmp_4,"getarownumnz")?;
      if vali_.len() != (__tmp_3).try_into().unwrap() {
        return Result::Err("get_a_row: Argument 'vali' has the wrong length, expected __tmp_3".to_string());
      }
      self.handle_res(unsafe { MSK_getarow(self.ptr,i_,nzi_,subi_.as_mut_ptr(),vali_.as_mut_ptr()) },"get_a_row")?;
      return Result::Ok(());
    } // getarow
    /// Obtains the number of non-zero elements in one row of the linear constraint matrix
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the row.
    ///
    /// # Returns
    ///
    ///   - `nzi` Number of non-zeros in the i'th row of `A`.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getarownumnz>
    #[allow(unused_parens)]
    pub fn get_a_row_num_nz(&self,i_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getarownumnz(self.ptr,i_,&mut __tmp_0) },"get_a_row_num_nz")?;
      return Result::Ok(__tmp_0);
    } // getarownumnz
    /// Obtains a sequence of rows from the coefficient matrix.
    ///
    /// # Arguments
    ///
    /// - `first_` Index of the first row in the sequence.
    /// - `last_` Index of the last row in the sequence plus one.
    /// - `ptrb_` Row start pointers.
    /// - `ptre_` Row end pointers.
    /// - `sub_` Contains the column subscripts.
    /// - `val_` Contains the coefficient values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getarowslice64>
    #[allow(unused_parens)]
    pub fn get_a_row_slice(&self,first_ : i32,last_ : i32,ptrb_ : &mut[i64],ptre_ : &mut[i64],sub_ : &mut[i32],val_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getarowslicenumnz64(self.ptr,first_,last_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getarowslicenumnz64")?;
      let maxnumnz_ : i64 = __tmp_0;
      if ptrb_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("get_a_row_slice: Argument 'ptrb' has the wrong length, expected (last_-first_)".to_string());
      }
      if ptre_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("get_a_row_slice: Argument 'ptre' has the wrong length, expected (last_-first_)".to_string());
      }
      if sub_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("get_a_row_slice: Argument 'sub' has the wrong length, expected maxnumnz_".to_string());
      }
      if val_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("get_a_row_slice: Argument 'val' has the wrong length, expected maxnumnz_".to_string());
      }
      self.handle_res(unsafe { MSK_getarowslice64(self.ptr,first_,last_,maxnumnz_,ptrb_.as_mut_ptr(),ptre_.as_mut_ptr(),sub_.as_mut_ptr(),val_.as_mut_ptr()) },"get_a_row_slice")?;
      return Result::Ok(());
    } // getarowslice64
    /// Obtains the number of non-zeros in a slice of rows of the coefficient matrix.
    ///
    /// # Arguments
    ///
    /// - `first_` Index of the first row in the sequence.
    /// - `last_` Index of the last row plus one in the sequence.
    ///
    /// # Returns
    ///
    ///   - `numnz` Number of non-zeros in the slice.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getarowslicenumnz64>
    #[allow(unused_parens)]
    pub fn get_a_row_slice_num_nz(&self,first_ : i32,last_ : i32) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getarowslicenumnz64(self.ptr,first_,last_,&mut __tmp_0) },"get_a_row_slice_num_nz")?;
      return Result::Ok(__tmp_0);
    } // getarowslicenumnz64
    /// Obtains a sequence of rows from the coefficient matrix in sparse triplet format.
    ///
    /// # Arguments
    ///
    /// - `first_` Index of the first row in the sequence.
    /// - `last_` Index of the last row in the sequence plus one.
    /// - `subi_` Constraint subscripts.
    /// - `subj_` Column subscripts.
    /// - `val_` Values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getarowslicetrip>
    #[allow(unused_parens)]
    pub fn get_a_row_slice_trip(&self,first_ : i32,last_ : i32,subi_ : &mut[i32],subj_ : &mut[i32],val_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getarowslicenumnz64(self.ptr,first_,last_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getarowslicenumnz64")?;
      let maxnumnz_ : i64 = __tmp_0;
      if subi_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("get_a_row_slice_trip: Argument 'subi' has the wrong length, expected maxnumnz_".to_string());
      }
      if subj_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("get_a_row_slice_trip: Argument 'subj' has the wrong length, expected maxnumnz_".to_string());
      }
      if val_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("get_a_row_slice_trip: Argument 'val' has the wrong length, expected maxnumnz_".to_string());
      }
      self.handle_res(unsafe { MSK_getarowslicetrip(self.ptr,first_,last_,maxnumnz_,subi_.as_mut_ptr(),subj_.as_mut_ptr(),val_.as_mut_ptr()) },"get_a_row_slice_trip")?;
      return Result::Ok(());
    } // getarowslicetrip
    /// Obtains the A matrix in sparse triplet format.
    ///
    /// # Arguments
    ///
    /// - `subi_` Constraint subscripts.
    /// - `subj_` Column subscripts.
    /// - `val_` Values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getatrip>
    #[allow(unused_parens)]
    pub fn get_a_trip(&self,subi_ : &mut[i32],subj_ : &mut[i32],val_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getnumanz64(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumanz64")?;
      let maxnumnz_ : i64 = __tmp_0;
      if subi_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("get_a_trip: Argument 'subi' has the wrong length, expected maxnumnz_".to_string());
      }
      if subj_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("get_a_trip: Argument 'subj' has the wrong length, expected maxnumnz_".to_string());
      }
      if val_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("get_a_trip: Argument 'val' has the wrong length, expected maxnumnz_".to_string());
      }
      self.handle_res(unsafe { MSK_getatrip(self.ptr,maxnumnz_,subi_.as_mut_ptr(),subj_.as_mut_ptr(),val_.as_mut_ptr()) },"get_a_trip")?;
      return Result::Ok(());
    } // getatrip
    /// Gets the current A matrix truncation threshold.
    ///
    /// # Arguments
    ///
    /// - `tolzero_` Truncation tolerance.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getatruncatetol>
    #[allow(unused_parens)]
    pub fn get_a_truncate_tol(&self,tolzero_ : &mut[f64]) -> Result<(),String> {
      if tolzero_.len() != (1).try_into().unwrap() {
        return Result::Err("get_a_truncate_tol: Argument 'tolzero' has the wrong length, expected 1".to_string());
      }
      self.handle_res(unsafe { MSK_getatruncatetol(self.ptr,tolzero_.as_mut_ptr()) },"get_a_truncate_tol")?;
      return Result::Ok(());
    } // getatruncatetol
    /// Obtains barA in block triplet form.
    ///
    /// # Arguments
    ///
    /// - `subi_` Constraint index.
    /// - `subj_` Symmetric matrix variable index.
    /// - `subk_` Block row index.
    /// - `subl_` Block column index.
    /// - `valijkl_` The numerical value associated with each block triplet.
    ///
    /// # Returns
    ///
    ///   - `num` Number of elements in the block triplet form.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarablocktriplet>
    #[allow(unused_parens)]
    pub fn get_bara_block_triplet(&self,subi_ : &mut[i32],subj_ : &mut[i32],subk_ : &mut[i32],subl_ : &mut[i32],valijkl_ : &mut[f64]) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getnumbarablocktriplets(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumbarablocktriplets")?;
      let maxnum_ : i64 = __tmp_0;
      let mut __tmp_2 : i64 = i64::default();
      if subi_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("get_bara_block_triplet: Argument 'subi' has the wrong length, expected maxnum_".to_string());
      }
      if subj_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("get_bara_block_triplet: Argument 'subj' has the wrong length, expected maxnum_".to_string());
      }
      if subk_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("get_bara_block_triplet: Argument 'subk' has the wrong length, expected maxnum_".to_string());
      }
      if subl_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("get_bara_block_triplet: Argument 'subl' has the wrong length, expected maxnum_".to_string());
      }
      if valijkl_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("get_bara_block_triplet: Argument 'valijkl' has the wrong length, expected maxnum_".to_string());
      }
      self.handle_res(unsafe { MSK_getbarablocktriplet(self.ptr,maxnum_,&mut __tmp_2,subi_.as_mut_ptr(),subj_.as_mut_ptr(),subk_.as_mut_ptr(),subl_.as_mut_ptr(),valijkl_.as_mut_ptr()) },"get_bara_block_triplet")?;
      return Result::Ok(__tmp_2);
    } // getbarablocktriplet
    /// Obtains information about an element in barA.
    ///
    /// # Arguments
    ///
    /// - `idx_` Position of the element in the vectorized form.
    /// - `i_` Row index of the element at position idx.
    /// - `j_` Column index of the element at position idx.
    /// - `sub_` A list indexes of the elements from symmetric matrix storage that appear in the weighted sum.
    /// - `weights_` The weights associated with each term in the weighted sum.
    ///
    /// # Returns
    ///
    ///   - `num` Number of terms in weighted sum that forms the element.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbaraidx>
    #[allow(unused_parens)]
    pub fn get_bara_idx(&self,idx_ : i64,i_ : &mut i32,j_ : &mut i32,sub_ : &mut[i64],weights_ : &mut[f64]) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getbaraidxinfo(self.ptr,idx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getbaraidxinfo")?;
      let maxnum_ : i64 = __tmp_0;
      let mut __tmp_4 : i64 = i64::default();
      if sub_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("get_bara_idx: Argument 'sub' has the wrong length, expected maxnum_".to_string());
      }
      if weights_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("get_bara_idx: Argument 'weights' has the wrong length, expected maxnum_".to_string());
      }
      self.handle_res(unsafe { MSK_getbaraidx(self.ptr,idx_,maxnum_,i_,j_,&mut __tmp_4,sub_.as_mut_ptr(),weights_.as_mut_ptr()) },"get_bara_idx")?;
      return Result::Ok(__tmp_4);
    } // getbaraidx
    /// Obtains information about an element in barA.
    ///
    /// # Arguments
    ///
    /// - `idx_` Position of the element in the vectorized form.
    /// - `i_` Row index of the element at position idx.
    /// - `j_` Column index of the element at position idx.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbaraidxij>
    #[allow(unused_parens)]
    pub fn get_bara_idx_i_j(&self,idx_ : i64,i_ : &mut i32,j_ : &mut i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getbaraidxij(self.ptr,idx_,i_,j_) },"get_bara_idx_i_j")?;
      return Result::Ok(());
    } // getbaraidxij
    /// Obtains the number of terms in the weighted sum that form a particular element in barA.
    ///
    /// # Arguments
    ///
    /// - `idx_` The internal position of the element for which information should be obtained.
    ///
    /// # Returns
    ///
    ///   - `num` Number of terms in the weighted sum that form the specified element in barA.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbaraidxinfo>
    #[allow(unused_parens)]
    pub fn get_bara_idx_info(&self,idx_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getbaraidxinfo(self.ptr,idx_,&mut __tmp_0) },"get_bara_idx_info")?;
      return Result::Ok(__tmp_0);
    } // getbaraidxinfo
    /// Obtains the sparsity pattern of the barA matrix.
    ///
    /// # Arguments
    ///
    /// - `numnz_` Number of nonzero elements in barA.
    /// - `idxij_` Position of each nonzero element in the vector representation of barA.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarasparsity>
    #[allow(unused_parens)]
    pub fn get_bara_sparsity(&self,numnz_ : &mut i64,idxij_ : &mut[i64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getnumbaranz(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumbaranz")?;
      let maxnumnz_ : i64 = __tmp_0;
      if idxij_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("get_bara_sparsity: Argument 'idxij' has the wrong length, expected maxnumnz_".to_string());
      }
      self.handle_res(unsafe { MSK_getbarasparsity(self.ptr,maxnumnz_,numnz_,idxij_.as_mut_ptr()) },"get_bara_sparsity")?;
      return Result::Ok(());
    } // getbarasparsity
    /// Obtains barC in block triplet form.
    ///
    /// # Arguments
    ///
    /// - `subj_` Symmetric matrix variable index.
    /// - `subk_` Block row index.
    /// - `subl_` Block column index.
    /// - `valjkl_` The numerical value associated with each block triplet.
    ///
    /// # Returns
    ///
    ///   - `num` Number of elements in the block triplet form.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarcblocktriplet>
    #[allow(unused_parens)]
    pub fn get_barc_block_triplet(&self,subj_ : &mut[i32],subk_ : &mut[i32],subl_ : &mut[i32],valjkl_ : &mut[f64]) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getnumbarcblocktriplets(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumbarcblocktriplets")?;
      let maxnum_ : i64 = __tmp_0;
      let mut __tmp_2 : i64 = i64::default();
      if subj_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("get_barc_block_triplet: Argument 'subj' has the wrong length, expected maxnum_".to_string());
      }
      if subk_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("get_barc_block_triplet: Argument 'subk' has the wrong length, expected maxnum_".to_string());
      }
      if subl_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("get_barc_block_triplet: Argument 'subl' has the wrong length, expected maxnum_".to_string());
      }
      if valjkl_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("get_barc_block_triplet: Argument 'valjkl' has the wrong length, expected maxnum_".to_string());
      }
      self.handle_res(unsafe { MSK_getbarcblocktriplet(self.ptr,maxnum_,&mut __tmp_2,subj_.as_mut_ptr(),subk_.as_mut_ptr(),subl_.as_mut_ptr(),valjkl_.as_mut_ptr()) },"get_barc_block_triplet")?;
      return Result::Ok(__tmp_2);
    } // getbarcblocktriplet
    /// Obtains information about an element in barc.
    ///
    /// # Arguments
    ///
    /// - `idx_` Index of the element for which information should be obtained.
    /// - `j_` Row index in barc.
    /// - `num_` Number of terms in the weighted sum.
    /// - `sub_` Elements appearing the weighted sum.
    /// - `weights_` Weights of terms in the weighted sum.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarcidx>
    #[allow(unused_parens)]
    pub fn get_barc_idx(&self,idx_ : i64,j_ : &mut i32,num_ : &mut i64,sub_ : &mut[i64],weights_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getbarcidxinfo(self.ptr,idx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getbarcidxinfo")?;
      let maxnum_ : i64 = __tmp_0;
      if sub_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("get_barc_idx: Argument 'sub' has the wrong length, expected maxnum_".to_string());
      }
      if weights_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("get_barc_idx: Argument 'weights' has the wrong length, expected maxnum_".to_string());
      }
      self.handle_res(unsafe { MSK_getbarcidx(self.ptr,idx_,maxnum_,j_,num_,sub_.as_mut_ptr(),weights_.as_mut_ptr()) },"get_barc_idx")?;
      return Result::Ok(());
    } // getbarcidx
    /// Obtains information about an element in barc.
    ///
    /// # Arguments
    ///
    /// - `idx_` Index of the element for which information should be obtained. The value is an index of a symmetric sparse variable.
    ///
    /// # Returns
    ///
    ///   - `num` Number of terms that appear in the weighted sum that forms the requested element.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarcidxinfo>
    #[allow(unused_parens)]
    pub fn get_barc_idx_info(&self,idx_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getbarcidxinfo(self.ptr,idx_,&mut __tmp_0) },"get_barc_idx_info")?;
      return Result::Ok(__tmp_0);
    } // getbarcidxinfo
    /// Obtains the row index of an element in barc.
    ///
    /// # Arguments
    ///
    /// - `idx_` Index of the element for which information should be obtained.
    /// - `j_` Row index in barc.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarcidxj>
    #[allow(unused_parens)]
    pub fn get_barc_idx_j(&self,idx_ : i64,j_ : &mut i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getbarcidxj(self.ptr,idx_,j_) },"get_barc_idx_j")?;
      return Result::Ok(());
    } // getbarcidxj
    /// Get the positions of the nonzero elements in barc.
    ///
    /// # Arguments
    ///
    /// - `numnz_` Number of nonzero elements in barc.
    /// - `idxj_` Internal positions of the nonzeros elements in barc.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarcsparsity>
    #[allow(unused_parens)]
    pub fn get_barc_sparsity(&self,numnz_ : &mut i64,idxj_ : &mut[i64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getnumbarcnz(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumbarcnz")?;
      let maxnumnz_ : i64 = __tmp_0;
      if idxj_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("get_barc_sparsity: Argument 'idxj' has the wrong length, expected maxnumnz_".to_string());
      }
      self.handle_res(unsafe { MSK_getbarcsparsity(self.ptr,maxnumnz_,numnz_,idxj_.as_mut_ptr()) },"get_barc_sparsity")?;
      return Result::Ok(());
    } // getbarcsparsity
    /// Obtains the dual solution for a semidefinite variable.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `j_` Index of the semidefinite variable.
    /// - `barsj_` Value of the j'th dual variable of barx.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarsj>
    #[allow(unused_parens)]
    pub fn get_bars_j(&self,whichsol_ : i32,j_ : i32,barsj_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getlenbarvarj(self.ptr,j_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getlenbarvarj")?;
      if barsj_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_bars_j: Argument 'barsj' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getbarsj(self.ptr,whichsol_,j_,barsj_.as_mut_ptr()) },"get_bars_j")?;
      return Result::Ok(());
    } // getbarsj
    /// Obtains the dual solution for a sequence of semidefinite variables.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` Index of the first semidefinite variable in the slice.
    /// - `last_` Index of the last semidefinite variable in the slice plus one.
    /// - `slicesize_` Denotes the length of the array barsslice.
    /// - `barsslice_` Dual solution values of symmetric matrix variables in the slice, stored sequentially.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarsslice>
    #[allow(unused_parens)]
    pub fn get_bars_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,slicesize_ : i64,barsslice_ : &mut[f64]) -> Result<(),String> {
      if barsslice_.len() != (slicesize_).try_into().unwrap() {
        return Result::Err("get_bars_slice: Argument 'barsslice' has the wrong length, expected slicesize_".to_string());
      }
      self.handle_res(unsafe { MSK_getbarsslice(self.ptr,whichsol_,first_,last_,slicesize_,barsslice_.as_mut_ptr()) },"get_bars_slice")?;
      return Result::Ok(());
    } // getbarsslice
    /// Obtains the name of a semidefinite variable.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the variable.
    ///
    /// # Returns
    ///
    ///   - `name` The requested name is copied to this buffer.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarvarname>
    #[allow(unused_parens)]
    pub fn get_barvar_name(&self,i_ : i32) -> Result<String,String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getbarvarnamelen(self.ptr,i_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getbarvarnamelen")?;
      let sizename_ : i32 = (1+__tmp_0);
      let mut name_ = Vec::new(); name_.resize(sizename_ as usize,0);
      self.handle_res(unsafe { MSK_getbarvarname(self.ptr,i_,sizename_,name_.as_mut_ptr()) },"get_barvar_name")?;
      return Result::Ok(String::from_utf8_lossy(&name_[..name_.iter().position(|&c| c == 0).unwrap_or(sizename_ as usize)]).into_owned());
    } // getbarvarname
    /// Obtains the index of semidefinite variable from its name.
    ///
    /// # Arguments
    ///
    /// - `somename_` The name of the variable.
    /// - `asgn_` Non-zero if the name somename is assigned to some semidefinite variable.
    ///
    /// # Returns
    ///
    ///   - `index` The index of a semidefinite variable with the name somename (if one exists).
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarvarnameindex>
    #[allow(unused_parens)]
    pub fn get_barvar_name_index(&self,somename_ : &str,asgn_ : &mut i32) -> Result<i32,String> {
      let __tmp_1 = CString::new(somename_).unwrap();
      let mut __tmp_3 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getbarvarnameindex(self.ptr,__tmp_1.as_ptr(),asgn_,&mut __tmp_3) },"get_barvar_name_index")?;
      return Result::Ok(__tmp_3);
    } // getbarvarnameindex
    /// Obtains the length of the name of a semidefinite variable.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the variable.
    ///
    /// # Returns
    ///
    ///   - `len` Returns the length of the indicated name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarvarnamelen>
    #[allow(unused_parens)]
    pub fn get_barvar_name_len(&self,i_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getbarvarnamelen(self.ptr,i_,&mut __tmp_0) },"get_barvar_name_len")?;
      return Result::Ok(__tmp_0);
    } // getbarvarnamelen
    /// Obtains the primal solution for a semidefinite variable.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `j_` Index of the semidefinite variable.
    /// - `barxj_` Value of the j'th variable of barx.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarxj>
    #[allow(unused_parens)]
    pub fn get_barx_j(&self,whichsol_ : i32,j_ : i32,barxj_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getlenbarvarj(self.ptr,j_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getlenbarvarj")?;
      if barxj_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_barx_j: Argument 'barxj' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getbarxj(self.ptr,whichsol_,j_,barxj_.as_mut_ptr()) },"get_barx_j")?;
      return Result::Ok(());
    } // getbarxj
    /// Obtains the primal solution for a sequence of semidefinite variables.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` Index of the first semidefinite variable in the slice.
    /// - `last_` Index of the last semidefinite variable in the slice plus one.
    /// - `slicesize_` Denotes the length of the array barxslice.
    /// - `barxslice_` Solution values of symmetric matrix variables in the slice, stored sequentially.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbarxslice>
    #[allow(unused_parens)]
    pub fn get_barx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,slicesize_ : i64,barxslice_ : &mut[f64]) -> Result<(),String> {
      if barxslice_.len() != (slicesize_).try_into().unwrap() {
        return Result::Err("get_barx_slice: Argument 'barxslice' has the wrong length, expected slicesize_".to_string());
      }
      self.handle_res(unsafe { MSK_getbarxslice(self.ptr,whichsol_,first_,last_,slicesize_,barxslice_.as_mut_ptr()) },"get_barx_slice")?;
      return Result::Ok(());
    } // getbarxslice
    /// Obtains all objective coefficients.
    ///
    /// # Arguments
    ///
    /// - `c_` Linear terms of the objective as a dense vector. The length is the number of variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getc>
    #[allow(unused_parens)]
    pub fn get_c(&self,c_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumvar")?;
      if c_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_c: Argument 'c' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getc(self.ptr,c_.as_mut_ptr()) },"get_c")?;
      return Result::Ok(());
    } // getc
    /// Obtains the fixed term in the objective.
    ///
    /// # Returns
    ///
    ///   - `cfix` Fixed term in the objective.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getcfix>
    #[allow(unused_parens)]
    pub fn get_cfix(&self) -> Result<f64,String> {
      let mut __tmp_0 : f64 = f64::default();
      self.handle_res(unsafe { MSK_getcfix(self.ptr,&mut __tmp_0) },"get_cfix")?;
      return Result::Ok(__tmp_0);
    } // getcfix
    /// Obtains one objective coefficient.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the variable for which the c coefficient should be obtained.
    /// - `cj_` The c coefficient value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getcj>
    #[allow(unused_parens)]
    pub fn get_c_j(&self,j_ : i32,cj_ : &mut f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getcj(self.ptr,j_,cj_) },"get_c_j")?;
      return Result::Ok(());
    } // getcj
    /// Obtains a sequence of coefficients from the objective.
    ///
    /// # Arguments
    ///
    /// - `subj_` A list of variable indexes.
    /// - `c_` Linear terms of the requested list of the objective as a dense vector.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getclist>
    #[allow(unused_parens)]
    pub fn get_c_list(&self,subj_ : &[i32],c_ : &mut[f64]) -> Result<(),String> {
      let num_ : i32 = subj_.len() as i32;
      if c_.len() != (num_).try_into().unwrap() {
        return Result::Err("get_c_list: Argument 'c' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_getclist(self.ptr,num_,subj_.as_ptr(),c_.as_mut_ptr()) },"get_c_list")?;
      return Result::Ok(());
    } // getclist
    /// Obtains bound information for one constraint.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the constraint for which the bound information should be obtained.
    /// - `bk_` Bound keys.
    ///   
    ///   See [Boundkey]
    /// - `bl_` Values for lower bounds.
    /// - `bu_` Values for upper bounds.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getconbound>
    #[allow(unused_parens)]
    pub fn get_con_bound(&self,i_ : i32,bk_ : & mut i32,bl_ : &mut f64,bu_ : &mut f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getconbound(self.ptr,i_,bk_,bl_,bu_) },"get_con_bound")?;
      return Result::Ok(());
    } // getconbound
    /// Obtains bounds information for a slice of the constraints.
    ///
    /// # Arguments
    ///
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `bk_` Bound keys.
    ///   
    ///   See [Boundkey]
    /// - `bl_` Values for lower bounds.
    /// - `bu_` Values for upper bounds.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getconboundslice>
    #[allow(unused_parens)]
    pub fn get_con_bound_slice(&self,first_ : i32,last_ : i32,bk_ : &mut[i32],bl_ : &mut[f64],bu_ : &mut[f64]) -> Result<(),String> {
      if bk_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("get_con_bound_slice: Argument 'bk' has the wrong length, expected (last_-first_)".to_string());
      }
      if bl_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("get_con_bound_slice: Argument 'bl' has the wrong length, expected (last_-first_)".to_string());
      }
      if bu_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("get_con_bound_slice: Argument 'bu' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getconboundslice(self.ptr,first_,last_,bk_.as_mut_ptr(),bl_.as_mut_ptr(),bu_.as_mut_ptr()) },"get_con_bound_slice")?;
      return Result::Ok(());
    } // getconboundslice
    /// Obtains a cone.
    ///
    /// # Arguments
    ///
    /// - `k_` Index of the cone.
    /// - `ct_` Specifies the type of the cone.
    ///   
    ///   See [Conetype]
    /// - `conepar_` For the power cone it denotes the exponent alpha. For other cone types it is unused and can be set to 0.
    /// - `nummem_` Number of member variables in the cone.
    /// - `submem_` Variable subscripts of the members in the cone.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getcone>
    #[allow(unused_parens)]
    pub fn get_cone(&mut self,k_ : i32,ct_ : & mut i32,conepar_ : &mut f64,nummem_ : &mut i32,submem_ : &mut[i32]) -> Result<(),String> {
      let mut __tmp_3 : i32 = i32::default();
      let mut __tmp_4 : f64 = f64::default();
      let mut __tmp_5 : i32 = i32::default();
      let __tmp_6 = unsafe { MSK_getconeinfo(self.ptr,k_,&mut __tmp_3,&mut __tmp_4,&mut __tmp_5) };let _ = self.handle_res(__tmp_6,"getconeinfo")?;
      if submem_.len() != (__tmp_5).try_into().unwrap() {
        return Result::Err("get_cone: Argument 'submem' has the wrong length, expected __tmp_5".to_string());
      }
      self.handle_res(unsafe { MSK_getcone(self.ptr,k_,ct_,conepar_,nummem_,submem_.as_mut_ptr()) },"get_cone")?;
      return Result::Ok(());
    } // getcone
    /// Obtains information about a cone.
    ///
    /// # Arguments
    ///
    /// - `k_` Index of the cone.
    /// - `ct_` Specifies the type of the cone.
    ///   
    ///   See [Conetype]
    /// - `conepar_` For the power cone it denotes the exponent alpha. For other cone types it is unused and can be set to 0.
    /// - `nummem_` Number of member variables in the cone.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getconeinfo>
    #[allow(unused_parens)]
    pub fn get_cone_info(&self,k_ : i32,ct_ : & mut i32,conepar_ : &mut f64,nummem_ : &mut i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getconeinfo(self.ptr,k_,ct_,conepar_,nummem_) },"get_cone_info")?;
      return Result::Ok(());
    } // getconeinfo
    /// Obtains the name of a cone.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the cone.
    ///
    /// # Returns
    ///
    ///   - `name` The required name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getconename>
    #[allow(unused_parens)]
    pub fn get_cone_name(&self,i_ : i32) -> Result<String,String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getconenamelen(self.ptr,i_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getconenamelen")?;
      let sizename_ : i32 = (1+__tmp_0);
      let mut name_ = Vec::new(); name_.resize(sizename_ as usize,0);
      self.handle_res(unsafe { MSK_getconename(self.ptr,i_,sizename_,name_.as_mut_ptr()) },"get_cone_name")?;
      return Result::Ok(String::from_utf8_lossy(&name_[..name_.iter().position(|&c| c == 0).unwrap_or(sizename_ as usize)]).into_owned());
    } // getconename
    /// Checks whether the name has been assigned to any cone.
    ///
    /// # Arguments
    ///
    /// - `somename_` The name which should be checked.
    /// - `asgn_` Is non-zero if the name somename is assigned to some cone.
    ///
    /// # Returns
    ///
    ///   - `index` If the name somename is assigned to some cone, this is the index of the cone.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getconenameindex>
    #[allow(unused_parens)]
    pub fn get_cone_name_index(&self,somename_ : &str,asgn_ : &mut i32) -> Result<i32,String> {
      let __tmp_1 = CString::new(somename_).unwrap();
      let mut __tmp_3 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getconenameindex(self.ptr,__tmp_1.as_ptr(),asgn_,&mut __tmp_3) },"get_cone_name_index")?;
      return Result::Ok(__tmp_3);
    } // getconenameindex
    /// Obtains the length of the name of a cone.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the cone.
    ///
    /// # Returns
    ///
    ///   - `len` Returns the length of the indicated name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getconenamelen>
    #[allow(unused_parens)]
    pub fn get_cone_name_len(&self,i_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getconenamelen(self.ptr,i_,&mut __tmp_0) },"get_cone_name_len")?;
      return Result::Ok(__tmp_0);
    } // getconenamelen
    /// Obtains the name of a constraint.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the constraint.
    ///
    /// # Returns
    ///
    ///   - `name` The required name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getconname>
    #[allow(unused_parens)]
    pub fn get_con_name(&self,i_ : i32) -> Result<String,String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getconnamelen(self.ptr,i_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getconnamelen")?;
      let sizename_ : i32 = (1+__tmp_0);
      let mut name_ = Vec::new(); name_.resize(sizename_ as usize,0);
      self.handle_res(unsafe { MSK_getconname(self.ptr,i_,sizename_,name_.as_mut_ptr()) },"get_con_name")?;
      return Result::Ok(String::from_utf8_lossy(&name_[..name_.iter().position(|&c| c == 0).unwrap_or(sizename_ as usize)]).into_owned());
    } // getconname
    /// Checks whether the name has been assigned to any constraint.
    ///
    /// # Arguments
    ///
    /// - `somename_` The name which should be checked.
    /// - `asgn_` Is non-zero if the name somename is assigned to some constraint.
    ///
    /// # Returns
    ///
    ///   - `index` If the name somename is assigned to a constraint, then return the index of the constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getconnameindex>
    #[allow(unused_parens)]
    pub fn get_con_name_index(&self,somename_ : &str,asgn_ : &mut i32) -> Result<i32,String> {
      let __tmp_1 = CString::new(somename_).unwrap();
      let mut __tmp_3 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getconnameindex(self.ptr,__tmp_1.as_ptr(),asgn_,&mut __tmp_3) },"get_con_name_index")?;
      return Result::Ok(__tmp_3);
    } // getconnameindex
    /// Obtains the length of the name of a constraint.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the constraint.
    ///
    /// # Returns
    ///
    ///   - `len` Returns the length of the indicated name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getconnamelen>
    #[allow(unused_parens)]
    pub fn get_con_name_len(&self,i_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getconnamelen(self.ptr,i_,&mut __tmp_0) },"get_con_name_len")?;
      return Result::Ok(__tmp_0);
    } // getconnamelen
    /// Obtains a sequence of coefficients from the objective.
    ///
    /// # Arguments
    ///
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `c_` Linear terms of the requested slice of the objective as a dense vector.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getcslice>
    #[allow(unused_parens)]
    pub fn get_c_slice(&self,first_ : i32,last_ : i32,c_ : &mut[f64]) -> Result<(),String> {
      if c_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("get_c_slice: Argument 'c' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getcslice(self.ptr,first_,last_,c_.as_mut_ptr()) },"get_c_slice")?;
      return Result::Ok(());
    } // getcslice
    /// Obtains the dimension of a symmetric matrix variable.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the semidefinite variable whose dimension is requested.
    ///
    /// # Returns
    ///
    ///   - `dimbarvarj` The dimension of the j'th semidefinite variable.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdimbarvarj>
    #[allow(unused_parens)]
    pub fn get_dim_barvar_j(&self,j_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getdimbarvarj(self.ptr,j_,&mut __tmp_0) },"get_dim_barvar_j")?;
      return Result::Ok(__tmp_0);
    } // getdimbarvarj
    /// Obtains the list of affine expression indexes in a disjunctive constraint.
    ///
    /// # Arguments
    ///
    /// - `djcidx_` Index of the disjunctive constraint.
    /// - `afeidxlist_` List of affine expression indexes.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjcafeidxlist>
    #[allow(unused_parens)]
    pub fn get_djc_afe_idx_list(&self,djcidx_ : i64,afeidxlist_ : &mut[i64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getdjcnumafe(self.ptr,djcidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getdjcnumafe")?;
      if afeidxlist_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_djc_afe_idx_list: Argument 'afeidxlist' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getdjcafeidxlist(self.ptr,djcidx_,afeidxlist_.as_mut_ptr()) },"get_djc_afe_idx_list")?;
      return Result::Ok(());
    } // getdjcafeidxlist
    /// Obtains the optional constant term vector of a disjunctive constraint.
    ///
    /// # Arguments
    ///
    /// - `djcidx_` Index of the disjunctive constraint.
    /// - `b_` The vector b.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjcb>
    #[allow(unused_parens)]
    pub fn get_djc_b(&self,djcidx_ : i64,b_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getdjcnumafe(self.ptr,djcidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getdjcnumafe")?;
      if b_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_djc_b: Argument 'b' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getdjcb(self.ptr,djcidx_,b_.as_mut_ptr()) },"get_djc_b")?;
      return Result::Ok(());
    } // getdjcb
    /// Obtains the list of domain indexes in a disjunctive constraint.
    ///
    /// # Arguments
    ///
    /// - `djcidx_` Index of the disjunctive constraint.
    /// - `domidxlist_` List of term sizes.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjcdomainidxlist>
    #[allow(unused_parens)]
    pub fn get_djc_domain_idx_list(&self,djcidx_ : i64,domidxlist_ : &mut[i64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getdjcnumdomain(self.ptr,djcidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getdjcnumdomain")?;
      if domidxlist_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_djc_domain_idx_list: Argument 'domidxlist' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getdjcdomainidxlist(self.ptr,djcidx_,domidxlist_.as_mut_ptr()) },"get_djc_domain_idx_list")?;
      return Result::Ok(());
    } // getdjcdomainidxlist
    /// Obtains the name of a disjunctive constraint.
    ///
    /// # Arguments
    ///
    /// - `djcidx_` Index of a disjunctive constraint.
    ///
    /// # Returns
    ///
    ///   - `name` Returns the required name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjcname>
    #[allow(unused_parens)]
    pub fn get_djc_name(&self,djcidx_ : i64) -> Result<String,String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getdjcnamelen(self.ptr,djcidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getdjcnamelen")?;
      let sizename_ : i32 = (1+__tmp_0);
      let mut name_ = Vec::new(); name_.resize(sizename_ as usize,0);
      self.handle_res(unsafe { MSK_getdjcname(self.ptr,djcidx_,sizename_,name_.as_mut_ptr()) },"get_djc_name")?;
      return Result::Ok(String::from_utf8_lossy(&name_[..name_.iter().position(|&c| c == 0).unwrap_or(sizename_ as usize)]).into_owned());
    } // getdjcname
    /// Obtains the length of the name of a disjunctive constraint.
    ///
    /// # Arguments
    ///
    /// - `djcidx_` Index of a disjunctive constraint.
    ///
    /// # Returns
    ///
    ///   - `len` Returns the length of the indicated name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjcnamelen>
    #[allow(unused_parens)]
    pub fn get_djc_name_len(&self,djcidx_ : i64) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getdjcnamelen(self.ptr,djcidx_,&mut __tmp_0) },"get_djc_name_len")?;
      return Result::Ok(__tmp_0);
    } // getdjcnamelen
    /// Obtains the number of affine expressions in the disjunctive constraint.
    ///
    /// # Arguments
    ///
    /// - `djcidx_` Index of the disjunctive constraint.
    ///
    /// # Returns
    ///
    ///   - `numafe` Number of affine expressions in the disjunctive constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjcnumafe>
    #[allow(unused_parens)]
    pub fn get_djc_num_afe(&mut self,djcidx_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getdjcnumafe(self.ptr,djcidx_,&mut __tmp_0) },"get_djc_num_afe")?;
      return Result::Ok(__tmp_0);
    } // getdjcnumafe
    /// Obtains the number of affine expressions in all disjunctive constraints.
    ///
    /// # Returns
    ///
    ///   - `numafetot` Number of affine expressions in all disjunctive constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjcnumafetot>
    #[allow(unused_parens)]
    pub fn get_djc_num_afe_tot(&mut self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getdjcnumafetot(self.ptr,&mut __tmp_0) },"get_djc_num_afe_tot")?;
      return Result::Ok(__tmp_0);
    } // getdjcnumafetot
    /// Obtains the number of domains in the disjunctive constraint.
    ///
    /// # Arguments
    ///
    /// - `djcidx_` Index of the disjunctive constraint.
    ///
    /// # Returns
    ///
    ///   - `numdomain` Number of domains in the disjunctive constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjcnumdomain>
    #[allow(unused_parens)]
    pub fn get_djc_num_domain(&mut self,djcidx_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getdjcnumdomain(self.ptr,djcidx_,&mut __tmp_0) },"get_djc_num_domain")?;
      return Result::Ok(__tmp_0);
    } // getdjcnumdomain
    /// Obtains the number of domains in all disjunctive constraints.
    ///
    /// # Returns
    ///
    ///   - `numdomaintot` Number of domains in all disjunctive constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjcnumdomaintot>
    #[allow(unused_parens)]
    pub fn get_djc_num_domain_tot(&mut self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getdjcnumdomaintot(self.ptr,&mut __tmp_0) },"get_djc_num_domain_tot")?;
      return Result::Ok(__tmp_0);
    } // getdjcnumdomaintot
    /// Obtains the number terms in the disjunctive constraint.
    ///
    /// # Arguments
    ///
    /// - `djcidx_` Index of the disjunctive constraint.
    ///
    /// # Returns
    ///
    ///   - `numterm` Number of terms in the disjunctive constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjcnumterm>
    #[allow(unused_parens)]
    pub fn get_djc_num_term(&mut self,djcidx_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getdjcnumterm(self.ptr,djcidx_,&mut __tmp_0) },"get_djc_num_term")?;
      return Result::Ok(__tmp_0);
    } // getdjcnumterm
    /// Obtains the number of terms in all disjunctive constraints.
    ///
    /// # Returns
    ///
    ///   - `numtermtot` Total number of terms in all disjunctive constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjcnumtermtot>
    #[allow(unused_parens)]
    pub fn get_djc_num_term_tot(&mut self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getdjcnumtermtot(self.ptr,&mut __tmp_0) },"get_djc_num_term_tot")?;
      return Result::Ok(__tmp_0);
    } // getdjcnumtermtot
    /// Obtains full data of all disjunctive constraints.
    ///
    /// # Arguments
    ///
    /// - `domidxlist_` The concatenation of index lists of domains appearing in all disjunctive constraints.
    /// - `afeidxlist_` The concatenation of index lists of affine expressions appearing in all disjunctive constraints.
    /// - `b_` The concatenation of vectors b appearing in all disjunctive constraints.
    /// - `termsizelist_` The concatenation of lists of term sizes appearing in all disjunctive constraints.
    /// - `numterms_` The number of terms in each of the disjunctive constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjcs>
    #[allow(unused_parens)]
    pub fn get_djcs(&self,domidxlist_ : &mut[i64],afeidxlist_ : &mut[i64],b_ : &mut[f64],termsizelist_ : &mut[i64],numterms_ : &mut[i64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getdjcnumdomaintot(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getdjcnumdomaintot")?;
      if domidxlist_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_djcs: Argument 'domidxlist' has the wrong length, expected __tmp_0".to_string());
      }
      let mut __tmp_2 : i64 = i64::default();
      let __tmp_3 = unsafe { MSK_getdjcnumafetot(self.ptr,&mut __tmp_2) };let _ = self.handle_res(__tmp_3,"getdjcnumafetot")?;
      if afeidxlist_.len() != (__tmp_2).try_into().unwrap() {
        return Result::Err("get_djcs: Argument 'afeidxlist' has the wrong length, expected __tmp_2".to_string());
      }
      let mut __tmp_4 : i64 = i64::default();
      let __tmp_5 = unsafe { MSK_getdjcnumafetot(self.ptr,&mut __tmp_4) };let _ = self.handle_res(__tmp_5,"getdjcnumafetot")?;
      if b_.len() != (__tmp_4).try_into().unwrap() {
        return Result::Err("get_djcs: Argument 'b' has the wrong length, expected __tmp_4".to_string());
      }
      let mut __tmp_6 : i64 = i64::default();
      let __tmp_7 = unsafe { MSK_getdjcnumtermtot(self.ptr,&mut __tmp_6) };let _ = self.handle_res(__tmp_7,"getdjcnumtermtot")?;
      if termsizelist_.len() != (__tmp_6).try_into().unwrap() {
        return Result::Err("get_djcs: Argument 'termsizelist' has the wrong length, expected __tmp_6".to_string());
      }
      let mut __tmp_8 : i64 = i64::default();
      let __tmp_9 = unsafe { MSK_getnumdjc(self.ptr,&mut __tmp_8) };let _ = self.handle_res(__tmp_9,"getnumdjc")?;
      if numterms_.len() != (__tmp_8).try_into().unwrap() {
        return Result::Err("get_djcs: Argument 'numterms' has the wrong length, expected __tmp_8".to_string());
      }
      self.handle_res(unsafe { MSK_getdjcs(self.ptr,domidxlist_.as_mut_ptr(),afeidxlist_.as_mut_ptr(),b_.as_mut_ptr(),termsizelist_.as_mut_ptr(),numterms_.as_mut_ptr()) },"get_djcs")?;
      return Result::Ok(());
    } // getdjcs
    /// Obtains the list of term sizes in a disjunctive constraint.
    ///
    /// # Arguments
    ///
    /// - `djcidx_` Index of the disjunctive constraint.
    /// - `termsizelist_` List of term sizes.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdjctermsizelist>
    #[allow(unused_parens)]
    pub fn get_djc_term_size_list(&self,djcidx_ : i64,termsizelist_ : &mut[i64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getdjcnumterm(self.ptr,djcidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getdjcnumterm")?;
      if termsizelist_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_djc_term_size_list: Argument 'termsizelist' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getdjctermsizelist(self.ptr,djcidx_,termsizelist_.as_mut_ptr()) },"get_djc_term_size_list")?;
      return Result::Ok(());
    } // getdjctermsizelist
    /// Obtains the dimension of the domain.
    ///
    /// # Arguments
    ///
    /// - `domidx_` Index of the domain.
    ///
    /// # Returns
    ///
    ///   - `n` Dimension of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdomainn>
    #[allow(unused_parens)]
    pub fn get_domain_n(&self,domidx_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getdomainn(self.ptr,domidx_,&mut __tmp_0) },"get_domain_n")?;
      return Result::Ok(__tmp_0);
    } // getdomainn
    /// Obtains the name of a domain.
    ///
    /// # Arguments
    ///
    /// - `domidx_` Index of a domain.
    ///
    /// # Returns
    ///
    ///   - `name` Returns the required name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdomainname>
    #[allow(unused_parens)]
    pub fn get_domain_name(&self,domidx_ : i64) -> Result<String,String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getdomainnamelen(self.ptr,domidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getdomainnamelen")?;
      let sizename_ : i32 = (1+__tmp_0);
      let mut name_ = Vec::new(); name_.resize(sizename_ as usize,0);
      self.handle_res(unsafe { MSK_getdomainname(self.ptr,domidx_,sizename_,name_.as_mut_ptr()) },"get_domain_name")?;
      return Result::Ok(String::from_utf8_lossy(&name_[..name_.iter().position(|&c| c == 0).unwrap_or(sizename_ as usize)]).into_owned());
    } // getdomainname
    /// Obtains the length of the name of a domain.
    ///
    /// # Arguments
    ///
    /// - `domidx_` Index of a domain.
    ///
    /// # Returns
    ///
    ///   - `len` Returns the length of the indicated name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdomainnamelen>
    #[allow(unused_parens)]
    pub fn get_domain_name_len(&self,domidx_ : i64) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getdomainnamelen(self.ptr,domidx_,&mut __tmp_0) },"get_domain_name_len")?;
      return Result::Ok(__tmp_0);
    } // getdomainnamelen
    /// Returns the type of the domain.
    ///
    /// # Arguments
    ///
    /// - `domidx_` Index of the domain.
    ///
    /// # Returns
    ///
    ///   - `domtype` The type of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdomaintype>
    #[allow(unused_parens)]
    pub fn get_domain_type(&self,domidx_ : i64) -> Result<i32,String> {
      let mut __tmp_0 : i32 = 0;
      self.handle_res(unsafe { MSK_getdomaintype(self.ptr,domidx_,&mut __tmp_0) },"get_domain_type")?;
      return Result::Ok(__tmp_0);
    } // getdomaintype
    /// Obtains a double information item.
    ///
    /// # Arguments
    ///
    /// - `whichdinf_` Specifies a double information item.
    ///   
    ///   See [Dinfitem]
    ///
    /// # Returns
    ///
    ///   - `dvalue` The value of the required double information item.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdouinf>
    #[allow(unused_parens)]
    pub fn get_dou_inf(&self,whichdinf_ : i32) -> Result<f64,String> {
      let mut __tmp_0 : f64 = f64::default();
      self.handle_res(unsafe { MSK_getdouinf(self.ptr,whichdinf_,&mut __tmp_0) },"get_dou_inf")?;
      return Result::Ok(__tmp_0);
    } // getdouinf
    /// Obtains a double parameter.
    ///
    /// # Arguments
    ///
    /// - `param_` Which parameter.
    ///   
    ///   See [Dparam]
    ///
    /// # Returns
    ///
    ///   - `parvalue` Parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdouparam>
    #[allow(unused_parens)]
    pub fn get_dou_param(&self,param_ : i32) -> Result<f64,String> {
      let mut __tmp_0 : f64 = f64::default();
      self.handle_res(unsafe { MSK_getdouparam(self.ptr,param_,&mut __tmp_0) },"get_dou_param")?;
      return Result::Ok(__tmp_0);
    } // getdouparam
    /// Computes the dual objective value associated with the solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `dualobj_` Objective value corresponding to the dual solution.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdualobj>
    #[allow(unused_parens)]
    pub fn get_dual_obj(&self,whichsol_ : i32,dualobj_ : &mut f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getdualobj(self.ptr,whichsol_,dualobj_) },"get_dual_obj")?;
      return Result::Ok(());
    } // getdualobj
    /// Compute norms of the dual solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `nrmy_` The norm of the y vector.
    /// - `nrmslc_` The norm of the slc vector.
    /// - `nrmsuc_` The norm of the suc vector.
    /// - `nrmslx_` The norm of the slx vector.
    /// - `nrmsux_` The norm of the sux vector.
    /// - `nrmsnx_` The norm of the snx vector.
    /// - `nrmbars_` The norm of the bars vector.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdualsolutionnorms>
    #[allow(unused_parens)]
    pub fn get_dual_solution_norms(&self,whichsol_ : i32,nrmy_ : &mut f64,nrmslc_ : &mut f64,nrmsuc_ : &mut f64,nrmslx_ : &mut f64,nrmsux_ : &mut f64,nrmsnx_ : &mut f64,nrmbars_ : &mut f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getdualsolutionnorms(self.ptr,whichsol_,nrmy_,nrmslc_,nrmsuc_,nrmslx_,nrmsux_,nrmsnx_,nrmbars_) },"get_dual_solution_norms")?;
      return Result::Ok(());
    } // getdualsolutionnorms
    /// Computes the violation of the dual solution for set of affine conic constraints.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `accidxlist_` An array of indexes of conic constraints.
    /// - `viol_` List of violations corresponding to sub.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdviolacc>
    #[allow(unused_parens)]
    pub fn get_dviol_acc(&self,whichsol_ : i32,accidxlist_ : &[i64],viol_ : &mut[f64]) -> Result<(),String> {
      let numaccidx_ : i64 = accidxlist_.len() as i64;
      if viol_.len() != (numaccidx_).try_into().unwrap() {
        return Result::Err("get_dviol_acc: Argument 'viol' has the wrong length, expected numaccidx_".to_string());
      }
      self.handle_res(unsafe { MSK_getdviolacc(self.ptr,whichsol_,numaccidx_,accidxlist_.as_ptr(),viol_.as_mut_ptr()) },"get_dviol_acc")?;
      return Result::Ok(());
    } // getdviolacc
    /// Computes the violation of dual solution for a set of semidefinite variables.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sub_` An array of indexes of barx variables.
    /// - `viol_` List of violations corresponding to sub.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdviolbarvar>
    #[allow(unused_parens)]
    pub fn get_dviol_barvar(&self,whichsol_ : i32,sub_ : &[i32],viol_ : &mut[f64]) -> Result<(),String> {
      let num_ : i32 = sub_.len() as i32;
      if viol_.len() != (num_).try_into().unwrap() {
        return Result::Err("get_dviol_barvar: Argument 'viol' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_getdviolbarvar(self.ptr,whichsol_,num_,sub_.as_ptr(),viol_.as_mut_ptr()) },"get_dviol_barvar")?;
      return Result::Ok(());
    } // getdviolbarvar
    /// Computes the violation of a dual solution associated with a set of constraints.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sub_` An array of indexes of constraints.
    /// - `viol_` List of violations corresponding to sub.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdviolcon>
    #[allow(unused_parens)]
    pub fn get_dviol_con(&self,whichsol_ : i32,sub_ : &[i32],viol_ : &mut[f64]) -> Result<(),String> {
      let num_ : i32 = sub_.len() as i32;
      if viol_.len() != (num_).try_into().unwrap() {
        return Result::Err("get_dviol_con: Argument 'viol' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_getdviolcon(self.ptr,whichsol_,num_,sub_.as_ptr(),viol_.as_mut_ptr()) },"get_dviol_con")?;
      return Result::Ok(());
    } // getdviolcon
    /// Computes the violation of a solution for set of dual conic constraints.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sub_` An array of indexes of conic constraints.
    /// - `viol_` List of violations corresponding to sub.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdviolcones>
    #[allow(unused_parens)]
    pub fn get_dviol_cones(&self,whichsol_ : i32,sub_ : &[i32],viol_ : &mut[f64]) -> Result<(),String> {
      let num_ : i32 = sub_.len() as i32;
      if viol_.len() != (num_).try_into().unwrap() {
        return Result::Err("get_dviol_cones: Argument 'viol' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_getdviolcones(self.ptr,whichsol_,num_,sub_.as_ptr(),viol_.as_mut_ptr()) },"get_dviol_cones")?;
      return Result::Ok(());
    } // getdviolcones
    /// Computes the violation of a dual solution associated with a set of scalar variables.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sub_` An array of indexes of x variables.
    /// - `viol_` List of violations corresponding to sub.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getdviolvar>
    #[allow(unused_parens)]
    pub fn get_dviol_var(&self,whichsol_ : i32,sub_ : &[i32],viol_ : &mut[f64]) -> Result<(),String> {
      let num_ : i32 = sub_.len() as i32;
      if viol_.len() != (num_).try_into().unwrap() {
        return Result::Err("get_dviol_var: Argument 'viol' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_getdviolvar(self.ptr,whichsol_,num_,sub_.as_ptr(),viol_.as_mut_ptr()) },"get_dviol_var")?;
      return Result::Ok(());
    } // getdviolvar
    /// Obtains the index of a named information item.
    ///
    /// # Arguments
    ///
    /// - `inftype_` Type of the information item.
    ///   
    ///   See [Inftype]
    /// - `infname_` Name of the information item.
    /// - `infindex_` The item index.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getinfindex>
    #[allow(unused_parens)]
    pub fn get_inf_index(&self,inftype_ : i32,infname_ : &str,infindex_ : &mut i32) -> Result<(),String> {
      let __tmp_1 = CString::new(infname_).unwrap();
      self.handle_res(unsafe { MSK_getinfindex(self.ptr,inftype_,__tmp_1.as_ptr(),infindex_) },"get_inf_index")?;
      return Result::Ok(());
    } // getinfindex
    /// Obtains the maximum index of an information item of a given type.
    ///
    /// # Arguments
    ///
    /// - `inftype_` Type of the information item.
    ///   
    ///   See [Inftype]
    /// - `infmax_` The maximum index (plus 1) requested.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getinfmax>
    #[allow(unused_parens)]
    pub fn get_inf_max(&self,inftype_ : i32,infmax_ : &mut[i32]) -> Result<(),String> {
      if infmax_.len() != (Value::MAX_STR_LEN).try_into().unwrap() {
        return Result::Err("get_inf_max: Argument 'infmax' has the wrong length, expected Value::MAX_STR_LEN".to_string());
      }
      self.handle_res(unsafe { MSK_getinfmax(self.ptr,inftype_,infmax_.as_mut_ptr()) },"get_inf_max")?;
      return Result::Ok(());
    } // getinfmax
    /// Obtains the name of an information item.
    ///
    /// # Arguments
    ///
    /// - `inftype_` Type of the information item.
    ///   
    ///   See [Inftype]
    /// - `whichinf_` An information item.
    ///
    /// # Returns
    ///
    ///   - `infname` Name of the information item.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getinfname>
    #[allow(unused_parens)]
    pub fn get_inf_name(&self,inftype_ : i32,whichinf_ : i32) -> Result<String,String> {
      let mut infname_ = Vec::new(); infname_.resize(Value::MAX_STR_LEN as usize,0);
      self.handle_res(unsafe { MSK_getinfname(self.ptr,inftype_,whichinf_,infname_.as_mut_ptr()) },"get_inf_name")?;
      return Result::Ok(String::from_utf8_lossy(&infname_[..infname_.iter().position(|&c| c == 0).unwrap_or(Value::MAX_STR_LEN as usize)]).into_owned());
    } // getinfname
    /// Obtains an integer information item.
    ///
    /// # Arguments
    ///
    /// - `whichiinf_` Specifies an integer information item.
    ///   
    ///   See [Iinfitem]
    ///
    /// # Returns
    ///
    ///   - `ivalue` The value of the required integer information item.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getintinf>
    #[allow(unused_parens)]
    pub fn get_int_inf(&self,whichiinf_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getintinf(self.ptr,whichiinf_,&mut __tmp_0) },"get_int_inf")?;
      return Result::Ok(__tmp_0);
    } // getintinf
    /// Obtains an integer parameter.
    ///
    /// # Arguments
    ///
    /// - `param_` Which parameter.
    ///   
    ///   See [Iparam]
    ///
    /// # Returns
    ///
    ///   - `parvalue` Parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getintparam>
    #[allow(unused_parens)]
    pub fn get_int_param(&self,param_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getintparam(self.ptr,param_,&mut __tmp_0) },"get_int_param")?;
      return Result::Ok(__tmp_0);
    } // getintparam
    /// Obtains the length of one semidefinite variable.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the semidefinite variable whose length if requested.
    ///
    /// # Returns
    ///
    ///   - `lenbarvarj` Number of scalar elements in the lower triangular part of the semidefinite variable.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getlenbarvarj>
    #[allow(unused_parens)]
    pub fn get_len_barvar_j(&self,j_ : i32) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getlenbarvarj(self.ptr,j_,&mut __tmp_0) },"get_len_barvar_j")?;
      return Result::Ok(__tmp_0);
    } // getlenbarvarj
    /// Obtains a long integer information item.
    ///
    /// # Arguments
    ///
    /// - `whichliinf_` Specifies a long information item.
    ///   
    ///   See [Liinfitem]
    ///
    /// # Returns
    ///
    ///   - `ivalue` The value of the required long integer information item.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getlintinf>
    #[allow(unused_parens)]
    pub fn get_lint_inf(&self,whichliinf_ : i32) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getlintinf(self.ptr,whichliinf_,&mut __tmp_0) },"get_lint_inf")?;
      return Result::Ok(__tmp_0);
    } // getlintinf
    /// Obtains the maximum length (not including terminating zero character) of any objective, constraint, variable, domain or cone name.
    ///
    /// # Arguments
    ///
    /// - `maxlen_` The maximum length of any name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getmaxnamelen>
    #[allow(unused_parens)]
    pub fn get_max_name_len(&self,maxlen_ : &mut i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getmaxnamelen(self.ptr,maxlen_) },"get_max_name_len")?;
      return Result::Ok(());
    } // getmaxnamelen
    /// Obtains number of preallocated non-zeros in the linear constraint matrix.
    ///
    /// # Returns
    ///
    ///   - `maxnumanz` Number of preallocated non-zero linear matrix elements.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getmaxnumanz64>
    #[allow(unused_parens)]
    pub fn get_max_num_a_nz(&self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getmaxnumanz64(self.ptr,&mut __tmp_0) },"get_max_num_a_nz")?;
      return Result::Ok(__tmp_0);
    } // getmaxnumanz64
    /// Obtains maximum number of symmetric matrix variables for which space is currently preallocated.
    ///
    /// # Returns
    ///
    ///   - `maxnumbarvar` Maximum number of symmetric matrix variables for which space is currently preallocated.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getmaxnumbarvar>
    #[allow(unused_parens)]
    pub fn get_max_num_barvar(&self) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getmaxnumbarvar(self.ptr,&mut __tmp_0) },"get_max_num_barvar")?;
      return Result::Ok(__tmp_0);
    } // getmaxnumbarvar
    /// Obtains the number of preallocated constraints in the optimization task.
    ///
    /// # Arguments
    ///
    /// - `maxnumcon_` Number of preallocated constraints in the optimization task.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getmaxnumcon>
    #[allow(unused_parens)]
    pub fn get_max_num_con(&self,maxnumcon_ : &mut i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getmaxnumcon(self.ptr,maxnumcon_) },"get_max_num_con")?;
      return Result::Ok(());
    } // getmaxnumcon
    /// Obtains the number of preallocated cones in the optimization task.
    ///
    /// # Arguments
    ///
    /// - `maxnumcone_` Number of preallocated conic constraints in the optimization task.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getmaxnumcone>
    #[allow(unused_parens)]
    pub fn get_max_num_cone(&self,maxnumcone_ : &mut i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getmaxnumcone(self.ptr,maxnumcone_) },"get_max_num_cone")?;
      return Result::Ok(());
    } // getmaxnumcone
    /// Obtains the number of preallocated non-zeros for all quadratic terms in objective and constraints.
    ///
    /// # Arguments
    ///
    /// - `maxnumqnz_` Number of non-zero elements preallocated in quadratic coefficient matrices.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getmaxnumqnz64>
    #[allow(unused_parens)]
    pub fn get_max_num_q_nz(&self,maxnumqnz_ : &mut i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getmaxnumqnz64(self.ptr,maxnumqnz_) },"get_max_num_q_nz")?;
      return Result::Ok(());
    } // getmaxnumqnz64
    /// Obtains the maximum number variables allowed.
    ///
    /// # Arguments
    ///
    /// - `maxnumvar_` Number of preallocated variables in the optimization task.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getmaxnumvar>
    #[allow(unused_parens)]
    pub fn get_max_num_var(&self,maxnumvar_ : &mut i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getmaxnumvar(self.ptr,maxnumvar_) },"get_max_num_var")?;
      return Result::Ok(());
    } // getmaxnumvar
    /// Obtains information about the amount of memory used by a task.
    ///
    /// # Arguments
    ///
    /// - `meminuse_` Amount of memory currently used by the task.
    /// - `maxmemuse_` Maximum amount of memory used by the task until now.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getmemusagetask>
    #[allow(unused_parens)]
    pub fn get_mem_usage(&self,meminuse_ : &mut i64,maxmemuse_ : &mut i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getmemusagetask(self.ptr,meminuse_,maxmemuse_) },"get_mem_usage")?;
      return Result::Ok(());
    } // getmemusagetask
    /// Obtains the number of threads used by the mixed integer optimizer.
    ///
    /// # Returns
    ///
    ///   - `numthreads` The number of threads.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getmionumthreads>
    #[allow(unused_parens)]
    pub fn get_mio_num_threads(&self) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getmionumthreads(self.ptr,&mut __tmp_0) },"get_mio_num_threads")?;
      return Result::Ok(__tmp_0);
    } // getmionumthreads
    /// Obtains a named double information item.
    ///
    /// # Arguments
    ///
    /// - `infitemname_` The name of a double information item.
    /// - `dvalue_` The value of the required double information item.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnadouinf>
    #[allow(unused_parens)]
    pub fn get_na_dou_inf(&self,infitemname_ : &str,dvalue_ : &mut f64) -> Result<(),String> {
      let __tmp_1 = CString::new(infitemname_).unwrap();
      self.handle_res(unsafe { MSK_getnadouinf(self.ptr,__tmp_1.as_ptr(),dvalue_) },"get_na_dou_inf")?;
      return Result::Ok(());
    } // getnadouinf
    /// Obtains a double parameter.
    ///
    /// # Arguments
    ///
    /// - `paramname_` Name of a parameter.
    /// - `parvalue_` Parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnadouparam>
    #[allow(unused_parens)]
    pub fn get_na_dou_param(&self,paramname_ : &str,parvalue_ : &mut f64) -> Result<(),String> {
      let __tmp_1 = CString::new(paramname_).unwrap();
      self.handle_res(unsafe { MSK_getnadouparam(self.ptr,__tmp_1.as_ptr(),parvalue_) },"get_na_dou_param")?;
      return Result::Ok(());
    } // getnadouparam
    /// Obtains a named integer information item.
    ///
    /// # Arguments
    ///
    /// - `infitemname_` The name of an integer information item.
    /// - `ivalue_` The value of the required integer information item.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnaintinf>
    #[allow(unused_parens)]
    pub fn get_na_int_inf(&self,infitemname_ : &str,ivalue_ : &mut i32) -> Result<(),String> {
      let __tmp_1 = CString::new(infitemname_).unwrap();
      self.handle_res(unsafe { MSK_getnaintinf(self.ptr,__tmp_1.as_ptr(),ivalue_) },"get_na_int_inf")?;
      return Result::Ok(());
    } // getnaintinf
    /// Obtains an integer parameter.
    ///
    /// # Arguments
    ///
    /// - `paramname_` Name of a parameter.
    /// - `parvalue_` Parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnaintparam>
    #[allow(unused_parens)]
    pub fn get_na_int_param(&self,paramname_ : &str,parvalue_ : &mut i32) -> Result<(),String> {
      let __tmp_1 = CString::new(paramname_).unwrap();
      self.handle_res(unsafe { MSK_getnaintparam(self.ptr,__tmp_1.as_ptr(),parvalue_) },"get_na_int_param")?;
      return Result::Ok(());
    } // getnaintparam
    /// Obtains a string parameter.
    ///
    /// # Arguments
    ///
    /// - `paramname_` Name of a parameter.
    /// - `sizeparamname_` Size of the name buffer.
    /// - `len_` Returns the length of the parameter value.
    ///
    /// # Returns
    ///
    ///   - `parvalue` Parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnastrparam>
    #[allow(unused_parens)]
    pub fn get_na_str_param(&self,paramname_ : &str,sizeparamname_ : i32,len_ : &mut i32) -> Result<String,String> {
      let __tmp_1 = CString::new(paramname_).unwrap();
      let mut parvalue_ = Vec::new(); parvalue_.resize(sizeparamname_ as usize,0);
      self.handle_res(unsafe { MSK_getnastrparam(self.ptr,__tmp_1.as_ptr(),sizeparamname_,len_,parvalue_.as_mut_ptr()) },"get_na_str_param")?;
      return Result::Ok(String::from_utf8_lossy(&parvalue_[..parvalue_.iter().position(|&c| c == 0).unwrap_or(sizeparamname_ as usize)]).into_owned());
    } // getnastrparam
    /// Obtains the number of affine conic constraints.
    ///
    /// # Returns
    ///
    ///   - `num` The number of affine conic constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumacc>
    #[allow(unused_parens)]
    pub fn get_num_acc(&mut self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getnumacc(self.ptr,&mut __tmp_0) },"get_num_acc")?;
      return Result::Ok(__tmp_0);
    } // getnumacc
    /// Obtains the number of affine expressions.
    ///
    /// # Returns
    ///
    ///   - `numafe` Number of affine expressions.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumafe>
    #[allow(unused_parens)]
    pub fn get_num_afe(&mut self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getnumafe(self.ptr,&mut __tmp_0) },"get_num_afe")?;
      return Result::Ok(__tmp_0);
    } // getnumafe
    /// Obtains the number of non-zeros in the coefficient matrix.
    ///
    /// # Returns
    ///
    ///   - `numanz` Number of non-zero elements in the linear constraint matrix.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumanz>
    #[allow(unused_parens)]
    pub fn get_num_a_nz(&self) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getnumanz(self.ptr,&mut __tmp_0) },"get_num_a_nz")?;
      return Result::Ok(__tmp_0);
    } // getnumanz
    /// Obtains the number of non-zeros in the coefficient matrix.
    ///
    /// # Returns
    ///
    ///   - `numanz` Number of non-zero elements in the linear constraint matrix.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumanz64>
    #[allow(unused_parens)]
    pub fn get_num_a_nz_64(&self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getnumanz64(self.ptr,&mut __tmp_0) },"get_num_a_nz_64")?;
      return Result::Ok(__tmp_0);
    } // getnumanz64
    /// Obtains an upper bound on the number of scalar elements in the block triplet form of bara.
    ///
    /// # Returns
    ///
    ///   - `num` An upper bound on the number of elements in the block triplet form of bara.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumbarablocktriplets>
    #[allow(unused_parens)]
    pub fn get_num_bara_block_triplets(&self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getnumbarablocktriplets(self.ptr,&mut __tmp_0) },"get_num_bara_block_triplets")?;
      return Result::Ok(__tmp_0);
    } // getnumbarablocktriplets
    /// Get the number of nonzero elements in barA.
    ///
    /// # Returns
    ///
    ///   - `nz` The number of nonzero block elements in barA.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumbaranz>
    #[allow(unused_parens)]
    pub fn get_num_bara_nz(&self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getnumbaranz(self.ptr,&mut __tmp_0) },"get_num_bara_nz")?;
      return Result::Ok(__tmp_0);
    } // getnumbaranz
    /// Obtains an upper bound on the number of elements in the block triplet form of barc.
    ///
    /// # Returns
    ///
    ///   - `num` An upper bound on the number of elements in the block triplet form of barc.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumbarcblocktriplets>
    #[allow(unused_parens)]
    pub fn get_num_barc_block_triplets(&self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getnumbarcblocktriplets(self.ptr,&mut __tmp_0) },"get_num_barc_block_triplets")?;
      return Result::Ok(__tmp_0);
    } // getnumbarcblocktriplets
    /// Obtains the number of nonzero elements in barc.
    ///
    /// # Returns
    ///
    ///   - `nz` The number of nonzero elements in barc.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumbarcnz>
    #[allow(unused_parens)]
    pub fn get_num_barc_nz(&self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getnumbarcnz(self.ptr,&mut __tmp_0) },"get_num_barc_nz")?;
      return Result::Ok(__tmp_0);
    } // getnumbarcnz
    /// Obtains the number of semidefinite variables.
    ///
    /// # Returns
    ///
    ///   - `numbarvar` Number of semidefinite variables in the problem.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumbarvar>
    #[allow(unused_parens)]
    pub fn get_num_barvar(&self) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getnumbarvar(self.ptr,&mut __tmp_0) },"get_num_barvar")?;
      return Result::Ok(__tmp_0);
    } // getnumbarvar
    /// Obtains the number of constraints.
    ///
    /// # Returns
    ///
    ///   - `numcon` Number of constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumcon>
    #[allow(unused_parens)]
    pub fn get_num_con(&self) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) },"get_num_con")?;
      return Result::Ok(__tmp_0);
    } // getnumcon
    /// Obtains the number of cones.
    ///
    /// # Returns
    ///
    ///   - `numcone` Number of conic constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumcone>
    #[allow(unused_parens)]
    pub fn get_num_cone(&self) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getnumcone(self.ptr,&mut __tmp_0) },"get_num_cone")?;
      return Result::Ok(__tmp_0);
    } // getnumcone
    /// Obtains the number of members in a cone.
    ///
    /// # Arguments
    ///
    /// - `k_` Index of the cone.
    /// - `nummem_` Number of member variables in the cone.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumconemem>
    #[allow(unused_parens)]
    pub fn get_num_cone_mem(&self,k_ : i32,nummem_ : &mut i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getnumconemem(self.ptr,k_,nummem_) },"get_num_cone_mem")?;
      return Result::Ok(());
    } // getnumconemem
    /// Obtains the number of disjunctive constraints.
    ///
    /// # Returns
    ///
    ///   - `num` The number of disjunctive constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumdjc>
    #[allow(unused_parens)]
    pub fn get_num_djc(&mut self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getnumdjc(self.ptr,&mut __tmp_0) },"get_num_djc")?;
      return Result::Ok(__tmp_0);
    } // getnumdjc
    /// Obtain the number of domains defined.
    ///
    /// # Returns
    ///
    ///   - `numdomain` Number of domains in the task.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumdomain>
    #[allow(unused_parens)]
    pub fn get_num_domain(&mut self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getnumdomain(self.ptr,&mut __tmp_0) },"get_num_domain")?;
      return Result::Ok(__tmp_0);
    } // getnumdomain
    /// Obtains the number of integer-constrained variables.
    ///
    /// # Returns
    ///
    ///   - `numintvar` Number of integer variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumintvar>
    #[allow(unused_parens)]
    pub fn get_num_int_var(&self) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getnumintvar(self.ptr,&mut __tmp_0) },"get_num_int_var")?;
      return Result::Ok(__tmp_0);
    } // getnumintvar
    /// Obtains the number of parameters of a given type.
    ///
    /// # Arguments
    ///
    /// - `partype_` Parameter type.
    ///   
    ///   See [Parametertype]
    /// - `numparam_` Returns the number of parameters of the requested type.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumparam>
    #[allow(unused_parens)]
    pub fn get_num_param(&self,partype_ : i32,numparam_ : &mut i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getnumparam(self.ptr,partype_,numparam_) },"get_num_param")?;
      return Result::Ok(());
    } // getnumparam
    /// Obtains the number of non-zero quadratic terms in a constraint.
    ///
    /// # Arguments
    ///
    /// - `k_` Index of the constraint for which the number quadratic terms should be obtained.
    ///
    /// # Returns
    ///
    ///   - `numqcnz` Number of quadratic terms.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumqconknz64>
    #[allow(unused_parens)]
    pub fn get_num_q_con_k_nz(&self,k_ : i32) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getnumqconknz64(self.ptr,k_,&mut __tmp_0) },"get_num_q_con_k_nz")?;
      return Result::Ok(__tmp_0);
    } // getnumqconknz64
    /// Obtains the number of non-zero quadratic terms in the objective.
    ///
    /// # Returns
    ///
    ///   - `numqonz` Number of non-zero elements in the quadratic objective terms.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumqobjnz64>
    #[allow(unused_parens)]
    pub fn get_num_q_obj_nz(&self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getnumqobjnz64(self.ptr,&mut __tmp_0) },"get_num_q_obj_nz")?;
      return Result::Ok(__tmp_0);
    } // getnumqobjnz64
    /// Obtains the number of symmetric matrices stored.
    ///
    /// # Arguments
    ///
    /// - `num_` The number of symmetric sparse matrices.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumsymmat>
    #[allow(unused_parens)]
    pub fn get_num_sym_mat(&self,num_ : &mut i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getnumsymmat(self.ptr,num_) },"get_num_sym_mat")?;
      return Result::Ok(());
    } // getnumsymmat
    /// Obtains the number of variables.
    ///
    /// # Returns
    ///
    ///   - `numvar` Number of variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getnumvar>
    #[allow(unused_parens)]
    pub fn get_num_var(&self) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getnumvar(self.ptr,&mut __tmp_0) },"get_num_var")?;
      return Result::Ok(__tmp_0);
    } // getnumvar
    /// Obtains the name assigned to the objective function.
    ///
    /// # Returns
    ///
    ///   - `objname` Assigned the objective name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getobjname>
    #[allow(unused_parens)]
    pub fn get_obj_name(&self) -> Result<String,String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getobjnamelen(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getobjnamelen")?;
      let sizeobjname_ : i32 = (1+__tmp_0);
      let mut objname_ = Vec::new(); objname_.resize(sizeobjname_ as usize,0);
      self.handle_res(unsafe { MSK_getobjname(self.ptr,sizeobjname_,objname_.as_mut_ptr()) },"get_obj_name")?;
      return Result::Ok(String::from_utf8_lossy(&objname_[..objname_.iter().position(|&c| c == 0).unwrap_or(sizeobjname_ as usize)]).into_owned());
    } // getobjname
    /// Obtains the length of the name assigned to the objective function.
    ///
    /// # Returns
    ///
    ///   - `len` Assigned the length of the objective name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getobjnamelen>
    #[allow(unused_parens)]
    pub fn get_obj_name_len(&self) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getobjnamelen(self.ptr,&mut __tmp_0) },"get_obj_name_len")?;
      return Result::Ok(__tmp_0);
    } // getobjnamelen
    /// Gets the objective sense.
    ///
    /// # Returns
    ///
    ///   - `sense` The returned objective sense.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getobjsense>
    #[allow(unused_parens)]
    pub fn get_obj_sense(&self) -> Result<i32,String> {
      let mut __tmp_0 : i32 = 0;
      self.handle_res(unsafe { MSK_getobjsense(self.ptr,&mut __tmp_0) },"get_obj_sense")?;
      return Result::Ok(__tmp_0);
    } // getobjsense
    /// Obtains the maximum index of a parameter of a given type.
    ///
    /// # Arguments
    ///
    /// - `partype_` Parameter type.
    ///   
    ///   See [Parametertype]
    /// - `parammax_` The maximum index (plus 1) of the given parameter type.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getparammax>
    #[allow(unused_parens)]
    pub fn get_param_max(&self,partype_ : i32,parammax_ : &mut i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getparammax(self.ptr,partype_,parammax_) },"get_param_max")?;
      return Result::Ok(());
    } // getparammax
    /// Obtains the name of a parameter.
    ///
    /// # Arguments
    ///
    /// - `partype_` Parameter type.
    ///   
    ///   See [Parametertype]
    /// - `param_` Which parameter.
    ///
    /// # Returns
    ///
    ///   - `parname` Parameter name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getparamname>
    #[allow(unused_parens)]
    pub fn get_param_name(&self,partype_ : i32,param_ : i32) -> Result<String,String> {
      let mut parname_ = Vec::new(); parname_.resize(Value::MAX_STR_LEN as usize,0);
      self.handle_res(unsafe { MSK_getparamname(self.ptr,partype_,param_,parname_.as_mut_ptr()) },"get_param_name")?;
      return Result::Ok(String::from_utf8_lossy(&parname_[..parname_.iter().position(|&c| c == 0).unwrap_or(Value::MAX_STR_LEN as usize)]).into_owned());
    } // getparamname
    /// Obtains the exponent vector of a power domain.
    ///
    /// # Arguments
    ///
    /// - `domidx_` Index of the domain.
    /// - `alpha_` The exponent vector of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getpowerdomainalpha>
    #[allow(unused_parens)]
    pub fn get_power_domain_alpha(&mut self,domidx_ : i64,alpha_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let mut __tmp_1 : i64 = i64::default();
      let __tmp_2 = unsafe { MSK_getpowerdomaininfo(self.ptr,domidx_,&mut __tmp_0,&mut __tmp_1) };let _ = self.handle_res(__tmp_2,"getpowerdomaininfo")?;
      if alpha_.len() != (__tmp_1).try_into().unwrap() {
        return Result::Err("get_power_domain_alpha: Argument 'alpha' has the wrong length, expected __tmp_1".to_string());
      }
      self.handle_res(unsafe { MSK_getpowerdomainalpha(self.ptr,domidx_,alpha_.as_mut_ptr()) },"get_power_domain_alpha")?;
      return Result::Ok(());
    } // getpowerdomainalpha
    /// Obtains structural information about a power domain.
    ///
    /// # Arguments
    ///
    /// - `domidx_` Index of the domain.
    /// - `n_` Dimension of the domain.
    /// - `nleft_` Number of variables on the left hand side.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getpowerdomaininfo>
    #[allow(unused_parens)]
    pub fn get_power_domain_info(&mut self,domidx_ : i64,n_ : &mut i64,nleft_ : &mut i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getpowerdomaininfo(self.ptr,domidx_,n_,nleft_) },"get_power_domain_info")?;
      return Result::Ok(());
    } // getpowerdomaininfo
    /// Computes the primal objective value for the desired solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    ///
    /// # Returns
    ///
    ///   - `primalobj` Objective value corresponding to the primal solution.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getprimalobj>
    #[allow(unused_parens)]
    pub fn get_primal_obj(&self,whichsol_ : i32) -> Result<f64,String> {
      let mut __tmp_0 : f64 = f64::default();
      self.handle_res(unsafe { MSK_getprimalobj(self.ptr,whichsol_,&mut __tmp_0) },"get_primal_obj")?;
      return Result::Ok(__tmp_0);
    } // getprimalobj
    /// Compute norms of the primal solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `nrmxc_` The norm of the xc vector.
    /// - `nrmxx_` The norm of the xx vector.
    /// - `nrmbarx_` The norm of the barX vector.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getprimalsolutionnorms>
    #[allow(unused_parens)]
    pub fn get_primal_solution_norms(&self,whichsol_ : i32,nrmxc_ : &mut f64,nrmxx_ : &mut f64,nrmbarx_ : &mut f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getprimalsolutionnorms(self.ptr,whichsol_,nrmxc_,nrmxx_,nrmbarx_) },"get_primal_solution_norms")?;
      return Result::Ok(());
    } // getprimalsolutionnorms
    /// Obtains the problem type.
    ///
    /// # Returns
    ///
    ///   - `probtype` The problem type.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getprobtype>
    #[allow(unused_parens)]
    pub fn get_prob_type(&self) -> Result<i32,String> {
      let mut __tmp_0 : i32 = 0;
      self.handle_res(unsafe { MSK_getprobtype(self.ptr,&mut __tmp_0) },"get_prob_type")?;
      return Result::Ok(__tmp_0);
    } // getprobtype
    /// Obtains the problem status.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    ///
    /// # Returns
    ///
    ///   - `problemsta` Problem status.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getprosta>
    #[allow(unused_parens)]
    pub fn get_pro_sta(&self,whichsol_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = 0;
      self.handle_res(unsafe { MSK_getprosta(self.ptr,whichsol_,&mut __tmp_0) },"get_pro_sta")?;
      return Result::Ok(__tmp_0);
    } // getprosta
    /// Computes the violation of a solution for set of affine conic constraints.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `accidxlist_` An array of indexes of conic constraints.
    /// - `viol_` List of violations corresponding to sub.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getpviolacc>
    #[allow(unused_parens)]
    pub fn get_pviol_acc(&self,whichsol_ : i32,accidxlist_ : &[i64],viol_ : &mut[f64]) -> Result<(),String> {
      let numaccidx_ : i64 = accidxlist_.len() as i64;
      if viol_.len() != (numaccidx_).try_into().unwrap() {
        return Result::Err("get_pviol_acc: Argument 'viol' has the wrong length, expected numaccidx_".to_string());
      }
      self.handle_res(unsafe { MSK_getpviolacc(self.ptr,whichsol_,numaccidx_,accidxlist_.as_ptr(),viol_.as_mut_ptr()) },"get_pviol_acc")?;
      return Result::Ok(());
    } // getpviolacc
    /// Computes the violation of a primal solution for a list of semidefinite variables.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sub_` An array of indexes of barX variables.
    /// - `viol_` List of violations corresponding to sub.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getpviolbarvar>
    #[allow(unused_parens)]
    pub fn get_pviol_barvar(&self,whichsol_ : i32,sub_ : &[i32],viol_ : &mut[f64]) -> Result<(),String> {
      let num_ : i32 = sub_.len() as i32;
      if viol_.len() != (num_).try_into().unwrap() {
        return Result::Err("get_pviol_barvar: Argument 'viol' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_getpviolbarvar(self.ptr,whichsol_,num_,sub_.as_ptr(),viol_.as_mut_ptr()) },"get_pviol_barvar")?;
      return Result::Ok(());
    } // getpviolbarvar
    /// Computes the violation of a primal solution associated to a constraint.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sub_` An array of indexes of constraints.
    /// - `viol_` List of violations corresponding to sub.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getpviolcon>
    #[allow(unused_parens)]
    pub fn get_pviol_con(&self,whichsol_ : i32,sub_ : &[i32],viol_ : &mut[f64]) -> Result<(),String> {
      let num_ : i32 = sub_.len() as i32;
      if viol_.len() != (num_).try_into().unwrap() {
        return Result::Err("get_pviol_con: Argument 'viol' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_getpviolcon(self.ptr,whichsol_,num_,sub_.as_ptr(),viol_.as_mut_ptr()) },"get_pviol_con")?;
      return Result::Ok(());
    } // getpviolcon
    /// Computes the violation of a solution for set of conic constraints.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sub_` An array of indexes of conic constraints.
    /// - `viol_` List of violations corresponding to sub.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getpviolcones>
    #[allow(unused_parens)]
    pub fn get_pviol_cones(&self,whichsol_ : i32,sub_ : &[i32],viol_ : &mut[f64]) -> Result<(),String> {
      let num_ : i32 = sub_.len() as i32;
      if viol_.len() != (num_).try_into().unwrap() {
        return Result::Err("get_pviol_cones: Argument 'viol' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_getpviolcones(self.ptr,whichsol_,num_,sub_.as_ptr(),viol_.as_mut_ptr()) },"get_pviol_cones")?;
      return Result::Ok(());
    } // getpviolcones
    /// Computes the violation of a solution for set of disjunctive constraints.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `djcidxlist_` An array of indexes of disjunctive constraints.
    /// - `viol_` List of violations corresponding to sub.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getpvioldjc>
    #[allow(unused_parens)]
    pub fn get_pviol_djc(&self,whichsol_ : i32,djcidxlist_ : &[i64],viol_ : &mut[f64]) -> Result<(),String> {
      let numdjcidx_ : i64 = djcidxlist_.len() as i64;
      if viol_.len() != (numdjcidx_).try_into().unwrap() {
        return Result::Err("get_pviol_djc: Argument 'viol' has the wrong length, expected numdjcidx_".to_string());
      }
      self.handle_res(unsafe { MSK_getpvioldjc(self.ptr,whichsol_,numdjcidx_,djcidxlist_.as_ptr(),viol_.as_mut_ptr()) },"get_pviol_djc")?;
      return Result::Ok(());
    } // getpvioldjc
    /// Computes the violation of a primal solution for a list of scalar variables.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sub_` An array of indexes of x variables.
    /// - `viol_` List of violations corresponding to sub.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getpviolvar>
    #[allow(unused_parens)]
    pub fn get_pviol_var(&self,whichsol_ : i32,sub_ : &[i32],viol_ : &mut[f64]) -> Result<(),String> {
      let num_ : i32 = sub_.len() as i32;
      if viol_.len() != (num_).try_into().unwrap() {
        return Result::Err("get_pviol_var: Argument 'viol' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_getpviolvar(self.ptr,whichsol_,num_,sub_.as_ptr(),viol_.as_mut_ptr()) },"get_pviol_var")?;
      return Result::Ok(());
    } // getpviolvar
    /// Obtains all the quadratic terms in a constraint.
    ///
    /// # Arguments
    ///
    /// - `k_` Which constraint.
    /// - `qcsubi_` Row subscripts for quadratic constraint matrix.
    /// - `qcsubj_` Column subscripts for quadratic constraint matrix.
    /// - `qcval_` Quadratic constraint coefficient values.
    ///
    /// # Returns
    ///
    ///   - `numqcnz` Number of quadratic terms.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getqconk64>
    #[allow(unused_parens)]
    pub fn get_q_con_k(&self,k_ : i32,qcsubi_ : &mut[i32],qcsubj_ : &mut[i32],qcval_ : &mut[f64]) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getnumqconknz64(self.ptr,k_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumqconknz64")?;
      let maxnumqcnz_ : i64 = __tmp_0;
      let mut __tmp_2 : i64 = i64::default();
      let mut __tmp_3 : i64 = i64::default();
      let __tmp_4 = unsafe { MSK_getnumqconknz64(self.ptr,k_,&mut __tmp_3) };let _ = self.handle_res(__tmp_4,"getnumqconknz64")?;
      if qcsubi_.len() != (__tmp_3).try_into().unwrap() {
        return Result::Err("get_q_con_k: Argument 'qcsubi' has the wrong length, expected __tmp_3".to_string());
      }
      let mut __tmp_5 : i64 = i64::default();
      let __tmp_6 = unsafe { MSK_getnumqconknz64(self.ptr,k_,&mut __tmp_5) };let _ = self.handle_res(__tmp_6,"getnumqconknz64")?;
      if qcsubj_.len() != (__tmp_5).try_into().unwrap() {
        return Result::Err("get_q_con_k: Argument 'qcsubj' has the wrong length, expected __tmp_5".to_string());
      }
      let mut __tmp_7 : i64 = i64::default();
      let __tmp_8 = unsafe { MSK_getnumqconknz64(self.ptr,k_,&mut __tmp_7) };let _ = self.handle_res(__tmp_8,"getnumqconknz64")?;
      if qcval_.len() != (__tmp_7).try_into().unwrap() {
        return Result::Err("get_q_con_k: Argument 'qcval' has the wrong length, expected __tmp_7".to_string());
      }
      self.handle_res(unsafe { MSK_getqconk64(self.ptr,k_,maxnumqcnz_,&mut __tmp_2,qcsubi_.as_mut_ptr(),qcsubj_.as_mut_ptr(),qcval_.as_mut_ptr()) },"get_q_con_k")?;
      return Result::Ok(__tmp_2);
    } // getqconk64
    /// Obtains all the quadratic terms in the objective.
    ///
    /// # Arguments
    ///
    /// - `numqonz_` Number of non-zero elements in the quadratic objective terms.
    /// - `qosubi_` Row subscripts for quadratic objective coefficients.
    /// - `qosubj_` Column subscripts for quadratic objective coefficients.
    /// - `qoval_` Quadratic objective coefficient values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getqobj64>
    #[allow(unused_parens)]
    pub fn get_q_obj(&self,numqonz_ : &mut i64,qosubi_ : &mut[i32],qosubj_ : &mut[i32],qoval_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getnumqobjnz64(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumqobjnz64")?;
      let maxnumqonz_ : i64 = __tmp_0;
      if qosubi_.len() != (maxnumqonz_).try_into().unwrap() {
        return Result::Err("get_q_obj: Argument 'qosubi' has the wrong length, expected maxnumqonz_".to_string());
      }
      if qosubj_.len() != (maxnumqonz_).try_into().unwrap() {
        return Result::Err("get_q_obj: Argument 'qosubj' has the wrong length, expected maxnumqonz_".to_string());
      }
      if qoval_.len() != (maxnumqonz_).try_into().unwrap() {
        return Result::Err("get_q_obj: Argument 'qoval' has the wrong length, expected maxnumqonz_".to_string());
      }
      self.handle_res(unsafe { MSK_getqobj64(self.ptr,maxnumqonz_,numqonz_,qosubi_.as_mut_ptr(),qosubj_.as_mut_ptr(),qoval_.as_mut_ptr()) },"get_q_obj")?;
      return Result::Ok(());
    } // getqobj64
    /// Obtains one coefficient from the quadratic term of the objective
    ///
    /// # Arguments
    ///
    /// - `i_` Row index of the coefficient.
    /// - `j_` Column index of coefficient.
    /// - `qoij_` The required coefficient.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getqobjij>
    #[allow(unused_parens)]
    pub fn get_q_obj_i_j(&self,i_ : i32,j_ : i32,qoij_ : &mut f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getqobjij(self.ptr,i_,j_,qoij_) },"get_q_obj_i_j")?;
      return Result::Ok(());
    } // getqobjij
    /// Obtains the reduced costs for a sequence of variables.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` The index of the first variable in the sequence.
    /// - `last_` The index of the last variable in the sequence plus 1.
    /// - `redcosts_` Returns the requested reduced costs.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getreducedcosts>
    #[allow(unused_parens)]
    pub fn get_reduced_costs(&self,whichsol_ : i32,first_ : i32,last_ : i32,redcosts_ : &mut[f64]) -> Result<(),String> {
      if redcosts_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("get_reduced_costs: Argument 'redcosts' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getreducedcosts(self.ptr,whichsol_,first_,last_,redcosts_.as_mut_ptr()) },"get_reduced_costs")?;
      return Result::Ok(());
    } // getreducedcosts
    /// Obtains the status keys for the constraints.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `skc_` Status keys for the constraints.
    ///   
    ///   See [Stakey]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getskc>
    #[allow(unused_parens)]
    pub fn get_skc(&self,whichsol_ : i32,skc_ : &mut[i32]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if skc_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_skc: Argument 'skc' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getskc(self.ptr,whichsol_,skc_.as_mut_ptr()) },"get_skc")?;
      return Result::Ok(());
    } // getskc
    /// Obtains the status keys for a slice of the constraints.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `skc_` Status keys for the constraints.
    ///   
    ///   See [Stakey]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getskcslice>
    #[allow(unused_parens)]
    pub fn get_skc_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,skc_ : &mut[i32]) -> Result<(),String> {
      if skc_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("get_skc_slice: Argument 'skc' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getskcslice(self.ptr,whichsol_,first_,last_,skc_.as_mut_ptr()) },"get_skc_slice")?;
      return Result::Ok(());
    } // getskcslice
    /// Obtains the status keys for the conic constraints.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `skn_` Status keys for the conic constraints.
    ///   
    ///   See [Stakey]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getskn>
    #[allow(unused_parens)]
    pub fn get_skn(&self,whichsol_ : i32,skn_ : &mut[i32]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcone(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcone")?;
      if skn_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_skn: Argument 'skn' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getskn(self.ptr,whichsol_,skn_.as_mut_ptr()) },"get_skn")?;
      return Result::Ok(());
    } // getskn
    /// Obtains the status keys for the scalar variables.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `skx_` Status keys for the variables.
    ///   
    ///   See [Stakey]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getskx>
    #[allow(unused_parens)]
    pub fn get_skx(&self,whichsol_ : i32,skx_ : &mut[i32]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumvar")?;
      if skx_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_skx: Argument 'skx' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getskx(self.ptr,whichsol_,skx_.as_mut_ptr()) },"get_skx")?;
      return Result::Ok(());
    } // getskx
    /// Obtains the status keys for a slice of the scalar variables.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `skx_` Status keys for the variables.
    ///   
    ///   See [Stakey]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getskxslice>
    #[allow(unused_parens)]
    pub fn get_skx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,skx_ : &mut[i32]) -> Result<(),String> {
      if skx_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("get_skx_slice: Argument 'skx' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getskxslice(self.ptr,whichsol_,first_,last_,skx_.as_mut_ptr()) },"get_skx_slice")?;
      return Result::Ok(());
    } // getskxslice
    /// Obtains the slc vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `slc_` Dual variables corresponding to the lower bounds on the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getslc>
    #[allow(unused_parens)]
    pub fn get_slc(&self,whichsol_ : i32,slc_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if slc_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_slc: Argument 'slc' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getslc(self.ptr,whichsol_,slc_.as_mut_ptr()) },"get_slc")?;
      return Result::Ok(());
    } // getslc
    /// Obtains a slice of the slc vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `slc_` Dual variables corresponding to the lower bounds on the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getslcslice>
    #[allow(unused_parens)]
    pub fn get_slc_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,slc_ : &mut[f64]) -> Result<(),String> {
      if slc_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("get_slc_slice: Argument 'slc' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getslcslice(self.ptr,whichsol_,first_,last_,slc_.as_mut_ptr()) },"get_slc_slice")?;
      return Result::Ok(());
    } // getslcslice
    /// Obtains the slx vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `slx_` Dual variables corresponding to the lower bounds on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getslx>
    #[allow(unused_parens)]
    pub fn get_slx(&self,whichsol_ : i32,slx_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumvar")?;
      if slx_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_slx: Argument 'slx' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getslx(self.ptr,whichsol_,slx_.as_mut_ptr()) },"get_slx")?;
      return Result::Ok(());
    } // getslx
    /// Obtains a slice of the slx vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `slx_` Dual variables corresponding to the lower bounds on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getslxslice>
    #[allow(unused_parens)]
    pub fn get_slx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,slx_ : &mut[f64]) -> Result<(),String> {
      if slx_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("get_slx_slice: Argument 'slx' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getslxslice(self.ptr,whichsol_,first_,last_,slx_.as_mut_ptr()) },"get_slx_slice")?;
      return Result::Ok(());
    } // getslxslice
    /// Obtains the snx vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `snx_` Dual variables corresponding to the conic constraints on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsnx>
    #[allow(unused_parens)]
    pub fn get_snx(&self,whichsol_ : i32,snx_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumvar")?;
      if snx_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_snx: Argument 'snx' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getsnx(self.ptr,whichsol_,snx_.as_mut_ptr()) },"get_snx")?;
      return Result::Ok(());
    } // getsnx
    /// Obtains a slice of the snx vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `snx_` Dual variables corresponding to the conic constraints on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsnxslice>
    #[allow(unused_parens)]
    pub fn get_snx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,snx_ : &mut[f64]) -> Result<(),String> {
      if snx_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("get_snx_slice: Argument 'snx' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getsnxslice(self.ptr,whichsol_,first_,last_,snx_.as_mut_ptr()) },"get_snx_slice")?;
      return Result::Ok(());
    } // getsnxslice
    /// Obtains the solution status.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    ///
    /// # Returns
    ///
    ///   - `solutionsta` Solution status.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsolsta>
    #[allow(unused_parens)]
    pub fn get_sol_sta(&self,whichsol_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = 0;
      self.handle_res(unsafe { MSK_getsolsta(self.ptr,whichsol_,&mut __tmp_0) },"get_sol_sta")?;
      return Result::Ok(__tmp_0);
    } // getsolsta
    /// Obtains the complete solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `problemsta_` Problem status.
    ///   
    ///   See [Prosta]
    /// - `solutionsta_` Solution status.
    ///   
    ///   See [Solsta]
    /// - `skc_` Status keys for the constraints.
    ///   
    ///   See [Stakey]
    /// - `skx_` Status keys for the variables.
    ///   
    ///   See [Stakey]
    /// - `skn_` Status keys for the conic constraints.
    ///   
    ///   See [Stakey]
    /// - `xc_` Primal constraint solution.
    /// - `xx_` Primal variable solution.
    /// - `y_` Vector of dual variables corresponding to the constraints.
    /// - `slc_` Dual variables corresponding to the lower bounds on the constraints.
    /// - `suc_` Dual variables corresponding to the upper bounds on the constraints.
    /// - `slx_` Dual variables corresponding to the lower bounds on the variables.
    /// - `sux_` Dual variables corresponding to the upper bounds on the variables.
    /// - `snx_` Dual variables corresponding to the conic constraints on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsolution>
    #[allow(unused_parens)]
    pub fn get_solution(&self,whichsol_ : i32,problemsta_ : & mut i32,solutionsta_ : & mut i32,skc_ : &mut[i32],skx_ : &mut[i32],skn_ : &mut[i32],xc_ : &mut[f64],xx_ : &mut[f64],y_ : &mut[f64],slc_ : &mut[f64],suc_ : &mut[f64],slx_ : &mut[f64],sux_ : &mut[f64],snx_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_2 : i32 = i32::default();
      let __tmp_3 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_2) };let _ = self.handle_res(__tmp_3,"getnumcon")?;
      if skc_.len() != (__tmp_2).try_into().unwrap() {
        return Result::Err("get_solution: Argument 'skc' has the wrong length, expected __tmp_2".to_string());
      }
      let mut __tmp_4 : i32 = i32::default();
      let __tmp_5 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_4) };let _ = self.handle_res(__tmp_5,"getnumvar")?;
      if skx_.len() != (__tmp_4).try_into().unwrap() {
        return Result::Err("get_solution: Argument 'skx' has the wrong length, expected __tmp_4".to_string());
      }
      let mut __tmp_6 : i32 = i32::default();
      let __tmp_7 = unsafe { MSK_getnumcone(self.ptr,&mut __tmp_6) };let _ = self.handle_res(__tmp_7,"getnumcone")?;
      if skn_.len() != (__tmp_6).try_into().unwrap() {
        return Result::Err("get_solution: Argument 'skn' has the wrong length, expected __tmp_6".to_string());
      }
      let mut __tmp_8 : i32 = i32::default();
      let __tmp_9 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_8) };let _ = self.handle_res(__tmp_9,"getnumcon")?;
      if xc_.len() != (__tmp_8).try_into().unwrap() {
        return Result::Err("get_solution: Argument 'xc' has the wrong length, expected __tmp_8".to_string());
      }
      let mut __tmp_10 : i32 = i32::default();
      let __tmp_11 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_10) };let _ = self.handle_res(__tmp_11,"getnumvar")?;
      if xx_.len() != (__tmp_10).try_into().unwrap() {
        return Result::Err("get_solution: Argument 'xx' has the wrong length, expected __tmp_10".to_string());
      }
      let mut __tmp_12 : i32 = i32::default();
      let __tmp_13 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_12) };let _ = self.handle_res(__tmp_13,"getnumcon")?;
      if y_.len() != (__tmp_12).try_into().unwrap() {
        return Result::Err("get_solution: Argument 'y' has the wrong length, expected __tmp_12".to_string());
      }
      let mut __tmp_14 : i32 = i32::default();
      let __tmp_15 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_14) };let _ = self.handle_res(__tmp_15,"getnumcon")?;
      if slc_.len() != (__tmp_14).try_into().unwrap() {
        return Result::Err("get_solution: Argument 'slc' has the wrong length, expected __tmp_14".to_string());
      }
      let mut __tmp_16 : i32 = i32::default();
      let __tmp_17 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_16) };let _ = self.handle_res(__tmp_17,"getnumcon")?;
      if suc_.len() != (__tmp_16).try_into().unwrap() {
        return Result::Err("get_solution: Argument 'suc' has the wrong length, expected __tmp_16".to_string());
      }
      let mut __tmp_18 : i32 = i32::default();
      let __tmp_19 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_18) };let _ = self.handle_res(__tmp_19,"getnumvar")?;
      if slx_.len() != (__tmp_18).try_into().unwrap() {
        return Result::Err("get_solution: Argument 'slx' has the wrong length, expected __tmp_18".to_string());
      }
      let mut __tmp_20 : i32 = i32::default();
      let __tmp_21 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_20) };let _ = self.handle_res(__tmp_21,"getnumvar")?;
      if sux_.len() != (__tmp_20).try_into().unwrap() {
        return Result::Err("get_solution: Argument 'sux' has the wrong length, expected __tmp_20".to_string());
      }
      let mut __tmp_22 : i32 = i32::default();
      let __tmp_23 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_22) };let _ = self.handle_res(__tmp_23,"getnumvar")?;
      if snx_.len() != (__tmp_22).try_into().unwrap() {
        return Result::Err("get_solution: Argument 'snx' has the wrong length, expected __tmp_22".to_string());
      }
      self.handle_res(unsafe { MSK_getsolution(self.ptr,whichsol_,problemsta_,solutionsta_,skc_.as_mut_ptr(),skx_.as_mut_ptr(),skn_.as_mut_ptr(),xc_.as_mut_ptr(),xx_.as_mut_ptr(),y_.as_mut_ptr(),slc_.as_mut_ptr(),suc_.as_mut_ptr(),slx_.as_mut_ptr(),sux_.as_mut_ptr(),snx_.as_mut_ptr()) },"get_solution")?;
      return Result::Ok(());
    } // getsolution
    /// Obtains information about of a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `pobj_` The primal objective value.
    /// - `pviolcon_` Maximal primal bound violation for a xc variable.
    /// - `pviolvar_` Maximal primal bound violation for a xx variable.
    /// - `pviolbarvar_` Maximal primal bound violation for a barx variable.
    /// - `pviolcone_` Maximal primal violation of the solution with respect to the conic constraints.
    /// - `pviolitg_` Maximal violation in the integer constraints.
    /// - `dobj_` Dual objective value.
    /// - `dviolcon_` Maximal dual bound violation for a xc variable.
    /// - `dviolvar_` Maximal dual bound violation for a xx variable.
    /// - `dviolbarvar_` Maximal dual bound violation for a bars variable.
    /// - `dviolcone_` Maximum violation of the dual solution in the dual conic constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsolutioninfo>
    #[allow(unused_parens)]
    pub fn get_solution_info(&self,whichsol_ : i32,pobj_ : &mut f64,pviolcon_ : &mut f64,pviolvar_ : &mut f64,pviolbarvar_ : &mut f64,pviolcone_ : &mut f64,pviolitg_ : &mut f64,dobj_ : &mut f64,dviolcon_ : &mut f64,dviolvar_ : &mut f64,dviolbarvar_ : &mut f64,dviolcone_ : &mut f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getsolutioninfo(self.ptr,whichsol_,pobj_,pviolcon_,pviolvar_,pviolbarvar_,pviolcone_,pviolitg_,dobj_,dviolcon_,dviolvar_,dviolbarvar_,dviolcone_) },"get_solution_info")?;
      return Result::Ok(());
    } // getsolutioninfo
    /// Obtains information about of a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `pobj_` The primal objective value.
    /// - `pviolcon_` Maximal primal bound violation for a xc variable.
    /// - `pviolvar_` Maximal primal bound violation for a xx variable.
    /// - `pviolbarvar_` Maximal primal bound violation for a barx variable.
    /// - `pviolcone_` Maximal primal violation of the solution with respect to the conic constraints.
    /// - `pviolacc_` Maximal primal violation of the solution with respect to the affine conic constraints.
    /// - `pvioldjc_` Maximal primal violation of the solution with respect to the disjunctive constraints.
    /// - `pviolitg_` Maximal violation in the integer constraints.
    /// - `dobj_` Dual objective value.
    /// - `dviolcon_` Maximal dual bound violation for a xc variable.
    /// - `dviolvar_` Maximal dual bound violation for a xx variable.
    /// - `dviolbarvar_` Maximal dual bound violation for a bars variable.
    /// - `dviolcone_` Maximum violation of the dual solution in the dual conic constraints.
    /// - `dviolacc_` Maximum violation of the dual solution in the dual affine conic constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsolutioninfonew>
    #[allow(unused_parens)]
    pub fn get_solution_info_new(&self,whichsol_ : i32,pobj_ : &mut f64,pviolcon_ : &mut f64,pviolvar_ : &mut f64,pviolbarvar_ : &mut f64,pviolcone_ : &mut f64,pviolacc_ : &mut f64,pvioldjc_ : &mut f64,pviolitg_ : &mut f64,dobj_ : &mut f64,dviolcon_ : &mut f64,dviolvar_ : &mut f64,dviolbarvar_ : &mut f64,dviolcone_ : &mut f64,dviolacc_ : &mut f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getsolutioninfonew(self.ptr,whichsol_,pobj_,pviolcon_,pviolvar_,pviolbarvar_,pviolcone_,pviolacc_,pvioldjc_,pviolitg_,dobj_,dviolcon_,dviolvar_,dviolbarvar_,dviolcone_,dviolacc_) },"get_solution_info_new")?;
      return Result::Ok(());
    } // getsolutioninfonew
    /// Obtains the complete solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `problemsta_` Problem status.
    ///   
    ///   See [Prosta]
    /// - `solutionsta_` Solution status.
    ///   
    ///   See [Solsta]
    /// - `skc_` Status keys for the constraints.
    ///   
    ///   See [Stakey]
    /// - `skx_` Status keys for the variables.
    ///   
    ///   See [Stakey]
    /// - `skn_` Status keys for the conic constraints.
    ///   
    ///   See [Stakey]
    /// - `xc_` Primal constraint solution.
    /// - `xx_` Primal variable solution.
    /// - `y_` Vector of dual variables corresponding to the constraints.
    /// - `slc_` Dual variables corresponding to the lower bounds on the constraints.
    /// - `suc_` Dual variables corresponding to the upper bounds on the constraints.
    /// - `slx_` Dual variables corresponding to the lower bounds on the variables.
    /// - `sux_` Dual variables corresponding to the upper bounds on the variables.
    /// - `snx_` Dual variables corresponding to the conic constraints on the variables.
    /// - `doty_` Dual variables corresponding to affine conic constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsolutionnew>
    #[allow(unused_parens)]
    pub fn get_solution_new(&self,whichsol_ : i32,problemsta_ : & mut i32,solutionsta_ : & mut i32,skc_ : &mut[i32],skx_ : &mut[i32],skn_ : &mut[i32],xc_ : &mut[f64],xx_ : &mut[f64],y_ : &mut[f64],slc_ : &mut[f64],suc_ : &mut[f64],slx_ : &mut[f64],sux_ : &mut[f64],snx_ : &mut[f64],doty_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_2 : i32 = i32::default();
      let __tmp_3 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_2) };let _ = self.handle_res(__tmp_3,"getnumcon")?;
      if skc_.len() != (__tmp_2).try_into().unwrap() {
        return Result::Err("get_solution_new: Argument 'skc' has the wrong length, expected __tmp_2".to_string());
      }
      let mut __tmp_4 : i32 = i32::default();
      let __tmp_5 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_4) };let _ = self.handle_res(__tmp_5,"getnumvar")?;
      if skx_.len() != (__tmp_4).try_into().unwrap() {
        return Result::Err("get_solution_new: Argument 'skx' has the wrong length, expected __tmp_4".to_string());
      }
      let mut __tmp_6 : i32 = i32::default();
      let __tmp_7 = unsafe { MSK_getnumcone(self.ptr,&mut __tmp_6) };let _ = self.handle_res(__tmp_7,"getnumcone")?;
      if skn_.len() != (__tmp_6).try_into().unwrap() {
        return Result::Err("get_solution_new: Argument 'skn' has the wrong length, expected __tmp_6".to_string());
      }
      let mut __tmp_8 : i32 = i32::default();
      let __tmp_9 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_8) };let _ = self.handle_res(__tmp_9,"getnumcon")?;
      if xc_.len() != (__tmp_8).try_into().unwrap() {
        return Result::Err("get_solution_new: Argument 'xc' has the wrong length, expected __tmp_8".to_string());
      }
      let mut __tmp_10 : i32 = i32::default();
      let __tmp_11 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_10) };let _ = self.handle_res(__tmp_11,"getnumvar")?;
      if xx_.len() != (__tmp_10).try_into().unwrap() {
        return Result::Err("get_solution_new: Argument 'xx' has the wrong length, expected __tmp_10".to_string());
      }
      let mut __tmp_12 : i32 = i32::default();
      let __tmp_13 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_12) };let _ = self.handle_res(__tmp_13,"getnumcon")?;
      if y_.len() != (__tmp_12).try_into().unwrap() {
        return Result::Err("get_solution_new: Argument 'y' has the wrong length, expected __tmp_12".to_string());
      }
      let mut __tmp_14 : i32 = i32::default();
      let __tmp_15 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_14) };let _ = self.handle_res(__tmp_15,"getnumcon")?;
      if slc_.len() != (__tmp_14).try_into().unwrap() {
        return Result::Err("get_solution_new: Argument 'slc' has the wrong length, expected __tmp_14".to_string());
      }
      let mut __tmp_16 : i32 = i32::default();
      let __tmp_17 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_16) };let _ = self.handle_res(__tmp_17,"getnumcon")?;
      if suc_.len() != (__tmp_16).try_into().unwrap() {
        return Result::Err("get_solution_new: Argument 'suc' has the wrong length, expected __tmp_16".to_string());
      }
      let mut __tmp_18 : i32 = i32::default();
      let __tmp_19 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_18) };let _ = self.handle_res(__tmp_19,"getnumvar")?;
      if slx_.len() != (__tmp_18).try_into().unwrap() {
        return Result::Err("get_solution_new: Argument 'slx' has the wrong length, expected __tmp_18".to_string());
      }
      let mut __tmp_20 : i32 = i32::default();
      let __tmp_21 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_20) };let _ = self.handle_res(__tmp_21,"getnumvar")?;
      if sux_.len() != (__tmp_20).try_into().unwrap() {
        return Result::Err("get_solution_new: Argument 'sux' has the wrong length, expected __tmp_20".to_string());
      }
      let mut __tmp_22 : i32 = i32::default();
      let __tmp_23 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_22) };let _ = self.handle_res(__tmp_23,"getnumvar")?;
      if snx_.len() != (__tmp_22).try_into().unwrap() {
        return Result::Err("get_solution_new: Argument 'snx' has the wrong length, expected __tmp_22".to_string());
      }
      let mut __tmp_24 : i64 = i64::default();
      let __tmp_25 = unsafe { MSK_getaccntot(self.ptr,&mut __tmp_24) };let _ = self.handle_res(__tmp_25,"getaccntot")?;
      if doty_.len() != (__tmp_24).try_into().unwrap() {
        return Result::Err("get_solution_new: Argument 'doty' has the wrong length, expected __tmp_24".to_string());
      }
      self.handle_res(unsafe { MSK_getsolutionnew(self.ptr,whichsol_,problemsta_,solutionsta_,skc_.as_mut_ptr(),skx_.as_mut_ptr(),skn_.as_mut_ptr(),xc_.as_mut_ptr(),xx_.as_mut_ptr(),y_.as_mut_ptr(),slc_.as_mut_ptr(),suc_.as_mut_ptr(),slx_.as_mut_ptr(),sux_.as_mut_ptr(),snx_.as_mut_ptr(),doty_.as_mut_ptr()) },"get_solution_new")?;
      return Result::Ok(());
    } // getsolutionnew
    /// Obtains a slice of the solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `solitem_` Which part of the solution is required.
    ///   
    ///   See [Solitem]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `values_` The values of the requested solution elements.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsolutionslice>
    #[allow(unused_parens)]
    pub fn get_solution_slice(&self,whichsol_ : i32,solitem_ : i32,first_ : i32,last_ : i32,values_ : &mut[f64]) -> Result<(),String> {
      if values_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("get_solution_slice: Argument 'values' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getsolutionslice(self.ptr,whichsol_,solitem_,first_,last_,values_.as_mut_ptr()) },"get_solution_slice")?;
      return Result::Ok(());
    } // getsolutionslice
    /// Gets a single symmetric matrix from the matrix store.
    ///
    /// # Arguments
    ///
    /// - `idx_` Index of the matrix to retrieve.
    /// - `subi_` Row subscripts of the matrix non-zero elements.
    /// - `subj_` Column subscripts of the matrix non-zero elements.
    /// - `valij_` Coefficients of the matrix non-zero elements.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsparsesymmat>
    #[allow(unused_parens)]
    pub fn get_sparse_sym_mat(&self,idx_ : i64,subi_ : &mut[i32],subj_ : &mut[i32],valij_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let mut __tmp_1 : i64 = i64::default();
      let mut __tmp_2 : i32 = i32::default();
      let __tmp_3 = unsafe { MSK_getsymmatinfo(self.ptr,idx_,&mut __tmp_0,&mut __tmp_1,&mut __tmp_2) };let _ = self.handle_res(__tmp_3,"getsymmatinfo")?;
      let maxlen_ : i64 = __tmp_1;
      if subi_.len() != (maxlen_).try_into().unwrap() {
        return Result::Err("get_sparse_sym_mat: Argument 'subi' has the wrong length, expected maxlen_".to_string());
      }
      if subj_.len() != (maxlen_).try_into().unwrap() {
        return Result::Err("get_sparse_sym_mat: Argument 'subj' has the wrong length, expected maxlen_".to_string());
      }
      if valij_.len() != (maxlen_).try_into().unwrap() {
        return Result::Err("get_sparse_sym_mat: Argument 'valij' has the wrong length, expected maxlen_".to_string());
      }
      self.handle_res(unsafe { MSK_getsparsesymmat(self.ptr,idx_,maxlen_,subi_.as_mut_ptr(),subj_.as_mut_ptr(),valij_.as_mut_ptr()) },"get_sparse_sym_mat")?;
      return Result::Ok(());
    } // getsparsesymmat
    /// Obtains the value of a string parameter.
    ///
    /// # Arguments
    ///
    /// - `param_` Which parameter.
    ///   
    ///   See [Sparam]
    /// - `len_` The length of the parameter value.
    ///
    /// # Returns
    ///
    ///   - `parvalue` If this is not a null pointer, the parameter value is stored here.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getstrparam>
    #[allow(unused_parens)]
    pub fn get_str_param(&self,param_ : i32,len_ : &mut i32) -> Result<String,String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getstrparamlen(self.ptr,param_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getstrparamlen")?;
      let maxlen_ : i32 = (1+__tmp_0);
      let mut parvalue_ = Vec::new(); parvalue_.resize(maxlen_ as usize,0);
      self.handle_res(unsafe { MSK_getstrparam(self.ptr,param_,maxlen_,len_,parvalue_.as_mut_ptr()) },"get_str_param")?;
      return Result::Ok(String::from_utf8_lossy(&parvalue_[..parvalue_.iter().position(|&c| c == 0).unwrap_or(maxlen_ as usize)]).into_owned());
    } // getstrparam
    /// Obtains the length of a string parameter.
    ///
    /// # Arguments
    ///
    /// - `param_` Which parameter.
    ///   
    ///   See [Sparam]
    ///
    /// # Returns
    ///
    ///   - `len` The length of the parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getstrparamlen>
    #[allow(unused_parens)]
    pub fn get_str_param_len(&self,param_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getstrparamlen(self.ptr,param_,&mut __tmp_0) },"get_str_param_len")?;
      return Result::Ok(__tmp_0);
    } // getstrparamlen
    /// Obtains the suc vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `suc_` Dual variables corresponding to the upper bounds on the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsuc>
    #[allow(unused_parens)]
    pub fn get_suc(&self,whichsol_ : i32,suc_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if suc_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_suc: Argument 'suc' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getsuc(self.ptr,whichsol_,suc_.as_mut_ptr()) },"get_suc")?;
      return Result::Ok(());
    } // getsuc
    /// Obtains a slice of the suc vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `suc_` Dual variables corresponding to the upper bounds on the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsucslice>
    #[allow(unused_parens)]
    pub fn get_suc_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,suc_ : &mut[f64]) -> Result<(),String> {
      if suc_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("get_suc_slice: Argument 'suc' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getsucslice(self.ptr,whichsol_,first_,last_,suc_.as_mut_ptr()) },"get_suc_slice")?;
      return Result::Ok(());
    } // getsucslice
    /// Obtains the sux vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sux_` Dual variables corresponding to the upper bounds on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsux>
    #[allow(unused_parens)]
    pub fn get_sux(&self,whichsol_ : i32,sux_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumvar")?;
      if sux_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_sux: Argument 'sux' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getsux(self.ptr,whichsol_,sux_.as_mut_ptr()) },"get_sux")?;
      return Result::Ok(());
    } // getsux
    /// Obtains a slice of the sux vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `sux_` Dual variables corresponding to the upper bounds on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsuxslice>
    #[allow(unused_parens)]
    pub fn get_sux_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,sux_ : &mut[f64]) -> Result<(),String> {
      if sux_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("get_sux_slice: Argument 'sux' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getsuxslice(self.ptr,whichsol_,first_,last_,sux_.as_mut_ptr()) },"get_sux_slice")?;
      return Result::Ok(());
    } // getsuxslice
    /// Obtains a cone type string identifier.
    ///
    /// # Arguments
    ///
    /// - `i_` Index.
    /// - `value_` The corresponding value.
    ///
    /// # Returns
    ///
    ///   - `name` Name of the i'th symbolic constant.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsymbcon>
    #[allow(unused_parens)]
    pub fn get_symb_con(&self,i_ : i32,value_ : &mut i32) -> Result<String,String> {
      let sizevalue_ : i32 = Value::MAX_STR_LEN;
      let mut name_ = Vec::new(); name_.resize(Value::MAX_STR_LEN as usize,0);
      self.handle_res(unsafe { MSK_getsymbcon(self.ptr,i_,sizevalue_,name_.as_mut_ptr(),value_) },"get_symb_con")?;
      return Result::Ok(String::from_utf8_lossy(&name_[..name_.iter().position(|&c| c == 0).unwrap_or(Value::MAX_STR_LEN as usize)]).into_owned());
    } // getsymbcon
    /// Obtains information about a matrix from the symmetric matrix storage.
    ///
    /// # Arguments
    ///
    /// - `idx_` Index of the matrix for which information is requested.
    /// - `dim_` Returns the dimension of the requested matrix.
    /// - `nz_` Returns the number of non-zeros in the requested matrix.
    /// - `mattype_` Returns the type of the requested matrix.
    ///   
    ///   See [Symmattype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getsymmatinfo>
    #[allow(unused_parens)]
    pub fn get_sym_mat_info(&self,idx_ : i64,dim_ : &mut i32,nz_ : &mut i64,mattype_ : & mut i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getsymmatinfo(self.ptr,idx_,dim_,nz_,mattype_) },"get_sym_mat_info")?;
      return Result::Ok(());
    } // getsymmatinfo
    /// Obtains the task name.
    ///
    /// # Returns
    ///
    ///   - `taskname` Returns the task name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.gettaskname>
    #[allow(unused_parens)]
    pub fn get_task_name(&self) -> Result<String,String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_gettasknamelen(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"gettasknamelen")?;
      let sizetaskname_ : i32 = (1+__tmp_0);
      let mut taskname_ = Vec::new(); taskname_.resize(sizetaskname_ as usize,0);
      self.handle_res(unsafe { MSK_gettaskname(self.ptr,sizetaskname_,taskname_.as_mut_ptr()) },"get_task_name")?;
      return Result::Ok(String::from_utf8_lossy(&taskname_[..taskname_.iter().position(|&c| c == 0).unwrap_or(sizetaskname_ as usize)]).into_owned());
    } // gettaskname
    /// Obtains the length the task name.
    ///
    /// # Returns
    ///
    ///   - `len` Returns the length of the task name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.gettasknamelen>
    #[allow(unused_parens)]
    pub fn get_task_name_len(&self) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_gettasknamelen(self.ptr,&mut __tmp_0) },"get_task_name_len")?;
      return Result::Ok(__tmp_0);
    } // gettasknamelen
    /// Obtains bound information for one variable.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the variable for which the bound information should be obtained.
    /// - `bk_` Bound keys.
    ///   
    ///   See [Boundkey]
    /// - `bl_` Values for lower bounds.
    /// - `bu_` Values for upper bounds.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getvarbound>
    #[allow(unused_parens)]
    pub fn get_var_bound(&self,i_ : i32,bk_ : & mut i32,bl_ : &mut f64,bu_ : &mut f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getvarbound(self.ptr,i_,bk_,bl_,bu_) },"get_var_bound")?;
      return Result::Ok(());
    } // getvarbound
    /// Obtains bounds information for a slice of the variables.
    ///
    /// # Arguments
    ///
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `bk_` Bound keys.
    ///   
    ///   See [Boundkey]
    /// - `bl_` Values for lower bounds.
    /// - `bu_` Values for upper bounds.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getvarboundslice>
    #[allow(unused_parens)]
    pub fn get_var_bound_slice(&self,first_ : i32,last_ : i32,bk_ : &mut[i32],bl_ : &mut[f64],bu_ : &mut[f64]) -> Result<(),String> {
      if bk_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("get_var_bound_slice: Argument 'bk' has the wrong length, expected (last_-first_)".to_string());
      }
      if bl_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("get_var_bound_slice: Argument 'bl' has the wrong length, expected (last_-first_)".to_string());
      }
      if bu_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("get_var_bound_slice: Argument 'bu' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getvarboundslice(self.ptr,first_,last_,bk_.as_mut_ptr(),bl_.as_mut_ptr(),bu_.as_mut_ptr()) },"get_var_bound_slice")?;
      return Result::Ok(());
    } // getvarboundslice
    /// Obtains the name of a variable.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of a variable.
    ///
    /// # Returns
    ///
    ///   - `name` Returns the required name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getvarname>
    #[allow(unused_parens)]
    pub fn get_var_name(&self,j_ : i32) -> Result<String,String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getvarnamelen(self.ptr,j_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getvarnamelen")?;
      let sizename_ : i32 = (1+__tmp_0);
      let mut name_ = Vec::new(); name_.resize(sizename_ as usize,0);
      self.handle_res(unsafe { MSK_getvarname(self.ptr,j_,sizename_,name_.as_mut_ptr()) },"get_var_name")?;
      return Result::Ok(String::from_utf8_lossy(&name_[..name_.iter().position(|&c| c == 0).unwrap_or(sizename_ as usize)]).into_owned());
    } // getvarname
    /// Checks whether the name has been assigned to any variable.
    ///
    /// # Arguments
    ///
    /// - `somename_` The name which should be checked.
    /// - `asgn_` Is non-zero if the name somename is assigned to a variable.
    ///
    /// # Returns
    ///
    ///   - `index` If the name somename is assigned to a variable, then return the index of the variable.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getvarnameindex>
    #[allow(unused_parens)]
    pub fn get_var_name_index(&self,somename_ : &str,asgn_ : &mut i32) -> Result<i32,String> {
      let __tmp_1 = CString::new(somename_).unwrap();
      let mut __tmp_3 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getvarnameindex(self.ptr,__tmp_1.as_ptr(),asgn_,&mut __tmp_3) },"get_var_name_index")?;
      return Result::Ok(__tmp_3);
    } // getvarnameindex
    /// Obtains the length of the name of a variable.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of a variable.
    ///
    /// # Returns
    ///
    ///   - `len` Returns the length of the indicated name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getvarnamelen>
    #[allow(unused_parens)]
    pub fn get_var_name_len(&self,i_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getvarnamelen(self.ptr,i_,&mut __tmp_0) },"get_var_name_len")?;
      return Result::Ok(__tmp_0);
    } // getvarnamelen
    /// Gets the variable type of one variable.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the variable.
    ///
    /// # Returns
    ///
    ///   - `vartype` Variable type of variable index j.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getvartype>
    #[allow(unused_parens)]
    pub fn get_var_type(&self,j_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = 0;
      self.handle_res(unsafe { MSK_getvartype(self.ptr,j_,&mut __tmp_0) },"get_var_type")?;
      return Result::Ok(__tmp_0);
    } // getvartype
    /// Obtains the variable type for one or more variables.
    ///
    /// # Arguments
    ///
    /// - `subj_` A list of variable indexes.
    /// - `vartype_` Returns the variables types corresponding the variable indexes requested.
    ///   
    ///   See [Variabletype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getvartypelist>
    #[allow(unused_parens)]
    pub fn get_var_type_list(&self,subj_ : &[i32],vartype_ : &mut[i32]) -> Result<(),String> {
      let num_ : i32 = subj_.len() as i32;
      if vartype_.len() != (num_).try_into().unwrap() {
        return Result::Err("get_var_type_list: Argument 'vartype' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_getvartypelist(self.ptr,num_,subj_.as_ptr(),vartype_.as_mut_ptr()) },"get_var_type_list")?;
      return Result::Ok(());
    } // getvartypelist
    /// Obtains the xc vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `xc_` Primal constraint solution.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getxc>
    #[allow(unused_parens)]
    pub fn get_xc(&self,whichsol_ : i32,xc_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if xc_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_xc: Argument 'xc' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getxc(self.ptr,whichsol_,xc_.as_mut_ptr()) },"get_xc")?;
      return Result::Ok(());
    } // getxc
    /// Obtains a slice of the xc vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `xc_` Primal constraint solution.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getxcslice>
    #[allow(unused_parens)]
    pub fn get_xc_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,xc_ : &mut[f64]) -> Result<(),String> {
      if xc_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("get_xc_slice: Argument 'xc' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getxcslice(self.ptr,whichsol_,first_,last_,xc_.as_mut_ptr()) },"get_xc_slice")?;
      return Result::Ok(());
    } // getxcslice
    /// Obtains the xx vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `xx_` Primal variable solution.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getxx>
    #[allow(unused_parens)]
    pub fn get_xx(&self,whichsol_ : i32,xx_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumvar")?;
      if xx_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_xx: Argument 'xx' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getxx(self.ptr,whichsol_,xx_.as_mut_ptr()) },"get_xx")?;
      return Result::Ok(());
    } // getxx
    /// Obtains a slice of the xx vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `xx_` Primal variable solution.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getxxslice>
    #[allow(unused_parens)]
    pub fn get_xx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,xx_ : &mut[f64]) -> Result<(),String> {
      if xx_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("get_xx_slice: Argument 'xx' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getxxslice(self.ptr,whichsol_,first_,last_,xx_.as_mut_ptr()) },"get_xx_slice")?;
      return Result::Ok(());
    } // getxxslice
    /// Obtains the y vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `y_` Vector of dual variables corresponding to the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.gety>
    #[allow(unused_parens)]
    pub fn get_y(&self,whichsol_ : i32,y_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if y_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("get_y: Argument 'y' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_gety(self.ptr,whichsol_,y_.as_mut_ptr()) },"get_y")?;
      return Result::Ok(());
    } // gety
    /// Obtains a slice of the y vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `y_` Vector of dual variables corresponding to the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getyslice>
    #[allow(unused_parens)]
    pub fn get_y_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,y_ : &mut[f64]) -> Result<(),String> {
      if y_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("get_y_slice: Argument 'y' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getyslice(self.ptr,whichsol_,first_,last_,y_.as_mut_ptr()) },"get_y_slice")?;
      return Result::Ok(());
    } // getyslice
    /// Prints the infeasibility report to an output stream.
    ///
    /// # Arguments
    ///
    /// - `whichstream_` Index of the stream.
    ///   
    ///   See [Streamtype]
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.infeasibilityreport>
    #[allow(unused_parens)]
    pub fn infeasibility_report(&mut self,whichstream_ : i32,whichsol_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_infeasibilityreport(self.ptr,whichstream_,whichsol_) },"infeasibility_report")?;
      return Result::Ok(());
    } // infeasibilityreport
    /// Prepare a task for basis solver.
    ///
    /// # Arguments
    ///
    /// - `basis_` The array of basis indexes to use.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.initbasissolve>
    #[allow(unused_parens)]
    pub fn init_basis_solve(&mut self,basis_ : &mut[i32]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if basis_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("init_basis_solve: Argument 'basis' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_initbasissolve(self.ptr,basis_.as_mut_ptr()) },"init_basis_solve")?;
      return Result::Ok(());
    } // initbasissolve
    /// Input the linear part of an optimization task in one function call.
    ///
    /// # Arguments
    ///
    /// - `maxnumcon_` Number of preallocated constraints in the optimization task.
    /// - `maxnumvar_` Number of preallocated variables in the optimization task.
    /// - `c_` Linear terms of the objective as a dense vector. The length is the number of variables.
    /// - `cfix_` Fixed term in the objective.
    /// - `aptrb_` Row or column start pointers.
    /// - `aptre_` Row or column end pointers.
    /// - `asub_` Coefficient subscripts.
    /// - `aval_` Coefficient values.
    /// - `bkc_` Bound keys for the constraints.
    ///   
    ///   See [Boundkey]
    /// - `blc_` Lower bounds for the constraints.
    /// - `buc_` Upper bounds for the constraints.
    /// - `bkx_` Bound keys for the variables.
    ///   
    ///   See [Boundkey]
    /// - `blx_` Lower bounds for the variables.
    /// - `bux_` Upper bounds for the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.inputdata64>
    #[allow(unused_parens)]
    pub fn input_data(&mut self,maxnumcon_ : i32,maxnumvar_ : i32,c_ : &[f64],cfix_ : f64,aptrb_ : &[i64],aptre_ : &[i64],asub_ : &[i32],aval_ : &[f64],bkc_ : &[i32],blc_ : &[f64],buc_ : &[f64],bkx_ : &[i32],blx_ : &[f64],bux_ : &[f64]) -> Result<(),String> {
      let numcon_ : i32 = std::cmp::min(std::cmp::min(buc_.len(),bkc_.len()),blc_.len()) as i32;
      let numvar_ : i32 = std::cmp::min(std::cmp::min(std::cmp::min(std::cmp::min(std::cmp::min(aptre_.len(),blx_.len()),bux_.len()),aptrb_.len()),c_.len()),bkx_.len()) as i32;
      if asub_.len() != aval_.len() { return Err("input_data: Mismatching asub/aval lengths".to_string()); } 
      if aptrb_.len() != aptre_.len() { return Err("input_data: Mismatching aptrb/aptre lengths".to_string()); } 
      if ! aptrb_.iter().zip(aptre_.iter()).all(|(a,b)| *a <= *b) { return Err("input_data: Invalid aptrb/aptre construction".to_string()); } 
      if let Some(v) = aptrb_.iter().min() { if *v < 0 { return Err("input_data: Invalid aptrb construction".to_string()); } }
      if let Some(v) = aptre_.iter().max() { if *v as usize > asub_.len() { return Err("input_data: Invalid aptre construction".to_string()); } } 
      self.handle_res(unsafe { MSK_inputdata64(self.ptr,maxnumcon_,maxnumvar_,numcon_,numvar_,c_.as_ptr(),cfix_,aptrb_.as_ptr(),aptre_.as_ptr(),asub_.as_ptr(),aval_.as_ptr(),bkc_.as_ptr(),blc_.as_ptr(),buc_.as_ptr(),bkx_.as_ptr(),blx_.as_ptr(),bux_.as_ptr()) },"input_data")?;
      return Result::Ok(());
    } // inputdata64
    /// Checks a double parameter name.
    ///
    /// # Arguments
    ///
    /// - `parname_` Parameter name.
    /// - `param_` Returns the parameter corresponding to the name, if one exists.
    ///   
    ///   See [Dparam]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.isdouparname>
    #[allow(unused_parens)]
    pub fn is_dou_par_name(&self,parname_ : &str,param_ : & mut i32) -> Result<(),String> {
      let __tmp_1 = CString::new(parname_).unwrap();
      self.handle_res(unsafe { MSK_isdouparname(self.ptr,__tmp_1.as_ptr(),param_) },"is_dou_par_name")?;
      return Result::Ok(());
    } // isdouparname
    /// Checks an integer parameter name.
    ///
    /// # Arguments
    ///
    /// - `parname_` Parameter name.
    /// - `param_` Returns the parameter corresponding to the name, if one exists.
    ///   
    ///   See [Iparam]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.isintparname>
    #[allow(unused_parens)]
    pub fn is_int_par_name(&self,parname_ : &str,param_ : & mut i32) -> Result<(),String> {
      let __tmp_1 = CString::new(parname_).unwrap();
      self.handle_res(unsafe { MSK_isintparname(self.ptr,__tmp_1.as_ptr(),param_) },"is_int_par_name")?;
      return Result::Ok(());
    } // isintparname
    /// Checks a string parameter name.
    ///
    /// # Arguments
    ///
    /// - `parname_` Parameter name.
    /// - `param_` Returns the parameter corresponding to the name, if one exists.
    ///   
    ///   See [Sparam]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.isstrparname>
    #[allow(unused_parens)]
    pub fn is_str_par_name(&self,parname_ : &str,param_ : & mut i32) -> Result<(),String> {
      let __tmp_1 = CString::new(parname_).unwrap();
      self.handle_res(unsafe { MSK_isstrparname(self.ptr,__tmp_1.as_ptr(),param_) },"is_str_par_name")?;
      return Result::Ok(());
    } // isstrparname
    /// Directs all output from a task stream to a file.
    ///
    /// # Arguments
    ///
    /// - `whichstream_` Index of the stream.
    ///   
    ///   See [Streamtype]
    /// - `filename_` A valid file name.
    /// - `append_` If this argument is 0 the output file will be overwritten, otherwise it will be appended to.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.linkfiletotaskstream>
    #[allow(unused_parens)]
    pub fn link_file_to_stream(&mut self,whichstream_ : i32,filename_ : &str,append_ : i32) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_linkfiletotaskstream(self.ptr,whichstream_,__tmp_1.as_ptr(),append_) },"link_file_to_stream")?;
      return Result::Ok(());
    } // linkfiletotaskstream
    /// Prints a short summary of a specified solution.
    ///
    /// # Arguments
    ///
    /// - `whichstream_` Index of the stream.
    ///   
    ///   See [Streamtype]
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.onesolutionsummary>
    #[allow(unused_parens)]
    pub fn one_solution_summary(&self,whichstream_ : i32,whichsol_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_onesolutionsummary(self.ptr,whichstream_,whichsol_) },"one_solution_summary")?;
      return Result::Ok(());
    } // onesolutionsummary
    /// Offload the optimization task to a solver server and wait for the solution.
    ///
    /// # Arguments
    ///
    /// - `address_` Address of the OptServer.
    /// - `accesstoken_` Access token.
    /// - `trmcode_` Is either OK or a termination response code.
    ///   
    ///   See [Rescode]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.optimizermt>
    #[allow(unused_parens)]
    pub fn optimize_rmt(&mut self,address_ : &str,accesstoken_ : &str,trmcode_ : & mut i32) -> Result<(),String> {
      let __tmp_1 = CString::new(address_).unwrap();
      let __tmp_3 = CString::new(accesstoken_).unwrap();
      self.handle_res(unsafe { MSK_optimizermt(self.ptr,__tmp_1.as_ptr(),__tmp_3.as_ptr(),trmcode_) },"optimize_rmt")?;
      return Result::Ok(());
    } // optimizermt
    /// Prints a short summary with optimizer statistics from last optimization.
    ///
    /// # Arguments
    ///
    /// - `whichstream_` Index of the stream.
    ///   
    ///   See [Streamtype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.optimizersummary>
    #[allow(unused_parens)]
    pub fn optimizer_summary(&self,whichstream_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_optimizersummary(self.ptr,whichstream_) },"optimizer_summary")?;
      return Result::Ok(());
    } // optimizersummary
    /// Optimizes the problem.
    ///
    /// # Returns
    ///
    ///   - `trmcode` Is either OK or a termination response code.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.optimizetrm>
    #[allow(unused_parens)]
    pub fn optimize(&mut self) -> Result<i32,String> {
      let mut __tmp_0 : i32 = 0;
      self.handle_res(unsafe { MSK_optimizetrm(self.ptr,&mut __tmp_0) },"optimize")?;
      return Result::Ok(__tmp_0);
    } // optimizetrm
    /// Repairs a primal infeasible optimization problem by adjusting the bounds on the constraints and variables.
    ///
    /// # Arguments
    ///
    /// - `wlc_` Weights associated with relaxing lower bounds on the constraints.
    /// - `wuc_` Weights associated with relaxing the upper bound on the constraints.
    /// - `wlx_` Weights associated with relaxing the lower bounds of the variables.
    /// - `wux_` Weights associated with relaxing the upper bounds of variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.primalrepair>
    #[allow(unused_parens)]
    pub fn primal_repair(&mut self,wlc_ : &[f64],wuc_ : &[f64],wlx_ : &[f64],wux_ : &[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if wlc_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("primal_repair: Argument 'wlc' has the wrong length, expected __tmp_0".to_string());
      }
      let mut __tmp_2 : i32 = i32::default();
      let __tmp_3 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_2) };let _ = self.handle_res(__tmp_3,"getnumcon")?;
      if wuc_.len() != (__tmp_2).try_into().unwrap() {
        return Result::Err("primal_repair: Argument 'wuc' has the wrong length, expected __tmp_2".to_string());
      }
      let mut __tmp_4 : i32 = i32::default();
      let __tmp_5 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_4) };let _ = self.handle_res(__tmp_5,"getnumvar")?;
      if wlx_.len() != (__tmp_4).try_into().unwrap() {
        return Result::Err("primal_repair: Argument 'wlx' has the wrong length, expected __tmp_4".to_string());
      }
      let mut __tmp_6 : i32 = i32::default();
      let __tmp_7 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_6) };let _ = self.handle_res(__tmp_7,"getnumvar")?;
      if wux_.len() != (__tmp_6).try_into().unwrap() {
        return Result::Err("primal_repair: Argument 'wux' has the wrong length, expected __tmp_6".to_string());
      }
      self.handle_res(unsafe { MSK_primalrepair(self.ptr,wlc_.as_ptr(),wuc_.as_ptr(),wlx_.as_ptr(),wux_.as_ptr()) },"primal_repair")?;
      return Result::Ok(());
    } // primalrepair
    /// Perform sensitivity analysis on bounds.
    ///
    /// # Arguments
    ///
    /// - `subi_` Indexes of constraints to analyze.
    /// - `marki_` Mark which constraint bounds to analyze.
    ///   
    ///   See [Mark]
    /// - `subj_` Indexes of variables to analyze.
    /// - `markj_` Mark which variable bounds to analyze.
    ///   
    ///   See [Mark]
    /// - `leftpricei_` Left shadow price for constraints.
    /// - `rightpricei_` Right shadow price for constraints.
    /// - `leftrangei_` Left range for constraints.
    /// - `rightrangei_` Right range for constraints.
    /// - `leftpricej_` Left shadow price for variables.
    /// - `rightpricej_` Right shadow price for variables.
    /// - `leftrangej_` Left range for variables.
    /// - `rightrangej_` Right range for variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.primalsensitivity>
    #[allow(unused_parens)]
    pub fn primal_sensitivity(&mut self,subi_ : &[i32],marki_ : &[i32],subj_ : &[i32],markj_ : &[i32],leftpricei_ : &mut[f64],rightpricei_ : &mut[f64],leftrangei_ : &mut[f64],rightrangei_ : &mut[f64],leftpricej_ : &mut[f64],rightpricej_ : &mut[f64],leftrangej_ : &mut[f64],rightrangej_ : &mut[f64]) -> Result<(),String> {
      let numi_ : i32 = std::cmp::min(subi_.len(),marki_.len()) as i32;
      let numj_ : i32 = std::cmp::min(markj_.len(),subj_.len()) as i32;
      if leftpricei_.len() != (numi_).try_into().unwrap() {
        return Result::Err("primal_sensitivity: Argument 'leftpricei' has the wrong length, expected numi_".to_string());
      }
      if rightpricei_.len() != (numi_).try_into().unwrap() {
        return Result::Err("primal_sensitivity: Argument 'rightpricei' has the wrong length, expected numi_".to_string());
      }
      if leftrangei_.len() != (numi_).try_into().unwrap() {
        return Result::Err("primal_sensitivity: Argument 'leftrangei' has the wrong length, expected numi_".to_string());
      }
      if rightrangei_.len() != (numi_).try_into().unwrap() {
        return Result::Err("primal_sensitivity: Argument 'rightrangei' has the wrong length, expected numi_".to_string());
      }
      if leftpricej_.len() != (numj_).try_into().unwrap() {
        return Result::Err("primal_sensitivity: Argument 'leftpricej' has the wrong length, expected numj_".to_string());
      }
      if rightpricej_.len() != (numj_).try_into().unwrap() {
        return Result::Err("primal_sensitivity: Argument 'rightpricej' has the wrong length, expected numj_".to_string());
      }
      if leftrangej_.len() != (numj_).try_into().unwrap() {
        return Result::Err("primal_sensitivity: Argument 'leftrangej' has the wrong length, expected numj_".to_string());
      }
      if rightrangej_.len() != (numj_).try_into().unwrap() {
        return Result::Err("primal_sensitivity: Argument 'rightrangej' has the wrong length, expected numj_".to_string());
      }
      self.handle_res(unsafe { MSK_primalsensitivity(self.ptr,numi_,subi_.as_ptr(),marki_.as_ptr(),numj_,subj_.as_ptr(),markj_.as_ptr(),leftpricei_.as_mut_ptr(),rightpricei_.as_mut_ptr(),leftrangei_.as_mut_ptr(),rightrangei_.as_mut_ptr(),leftpricej_.as_mut_ptr(),rightpricej_.as_mut_ptr(),leftrangej_.as_mut_ptr(),rightrangej_.as_mut_ptr()) },"primal_sensitivity")?;
      return Result::Ok(());
    } // primalsensitivity
    /// Prints the current parameter settings.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.printparam>
    #[allow(unused_parens)]
    pub fn print_param(&self) -> Result<(),String> {
      self.handle_res(unsafe { MSK_printparam(self.ptr) },"print_param")?;
      return Result::Ok(());
    } // printparam
    /// Puts an affine conic constraint.
    ///
    /// # Arguments
    ///
    /// - `accidx_` Affine conic constraint index.
    /// - `domidx_` Domain index.
    /// - `afeidxlist_` List of affine expression indexes.
    /// - `b_` The vector of constant terms added to affine expressions. Optional, can be NULL.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putacc>
    #[allow(unused_parens)]
    pub fn put_acc(&mut self,accidx_ : i64,domidx_ : i64,afeidxlist_ : &[i64],b_ : &[f64]) -> Result<(),String> {
      let numafeidx_ : i64 = afeidxlist_.len() as i64;
      if b_.len() != (numafeidx_).try_into().unwrap() {
        return Result::Err("put_acc: Argument 'b' has the wrong length, expected numafeidx_".to_string());
      }
      self.handle_res(unsafe { MSK_putacc(self.ptr,accidx_,domidx_,numafeidx_,afeidxlist_.as_ptr(),b_.as_ptr()) },"put_acc")?;
      return Result::Ok(());
    } // putacc
    /// Puts the constant vector b in an affine conic constraint.
    ///
    /// # Arguments
    ///
    /// - `accidx_` Affine conic constraint index.
    /// - `b_` The vector of constant terms added to affine expressions. Optional, can be NULL.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putaccb>
    #[allow(unused_parens)]
    pub fn put_acc_b(&mut self,accidx_ : i64,b_ : &[f64]) -> Result<(),String> {
      let lengthb_ : i64 = b_.len() as i64;
      self.handle_res(unsafe { MSK_putaccb(self.ptr,accidx_,lengthb_,b_.as_ptr()) },"put_acc_b")?;
      return Result::Ok(());
    } // putaccb
    /// Sets one element in the b vector of an affine conic constraint.
    ///
    /// # Arguments
    ///
    /// - `accidx_` Affine conic constraint index.
    /// - `j_` The index of an element in b to change.
    /// - `bj_` The new value of b\[j\].
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putaccbj>
    #[allow(unused_parens)]
    pub fn put_acc_b_j(&mut self,accidx_ : i64,j_ : i64,bj_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putaccbj(self.ptr,accidx_,j_,bj_) },"put_acc_b_j")?;
      return Result::Ok(());
    } // putaccbj
    /// Puts the doty vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `accidx_` The index of the affine conic constraint.
    /// - `doty_` The dual values for this affine conic constraint. The array should have length equal to the dimension of the constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putaccdoty>
    #[allow(unused_parens)]
    pub fn put_acc_dot_y(&self,whichsol_ : i32,accidx_ : i64,doty_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getaccn(self.ptr,accidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getaccn")?;
      if doty_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("put_acc_dot_y: Argument 'doty' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_putaccdoty(self.ptr,whichsol_,accidx_,doty_.as_mut_ptr()) },"put_acc_dot_y")?;
      return Result::Ok(());
    } // putaccdoty
    /// Puts a number of affine conic constraints.
    ///
    /// # Arguments
    ///
    /// - `accidxs_` Affine conic constraint indices.
    /// - `domidxs_` Domain indices.
    /// - `afeidxlist_` List of affine expression indexes.
    /// - `b_` The vector of constant terms added to affine expressions. Optional, can be NULL.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putacclist>
    #[allow(unused_parens)]
    pub fn put_acc_list(&mut self,accidxs_ : &[i64],domidxs_ : &[i64],afeidxlist_ : &[i64],b_ : &[f64]) -> Result<(),String> {
      let numaccs_ : i64 = std::cmp::min(accidxs_.len(),domidxs_.len()) as i64;
      let numafeidx_ : i64 = afeidxlist_.len() as i64;
      if b_.len() != (numafeidx_).try_into().unwrap() {
        return Result::Err("put_acc_list: Argument 'b' has the wrong length, expected numafeidx_".to_string());
      }
      self.handle_res(unsafe { MSK_putacclist(self.ptr,numaccs_,accidxs_.as_ptr(),domidxs_.as_ptr(),numafeidx_,afeidxlist_.as_ptr(),b_.as_ptr()) },"put_acc_list")?;
      return Result::Ok(());
    } // putacclist
    /// Sets the name of an affine conic constraint.
    ///
    /// # Arguments
    ///
    /// - `accidx_` Index of the affine conic constraint.
    /// - `name_` The name of the affine conic constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putaccname>
    #[allow(unused_parens)]
    pub fn put_acc_name(&mut self,accidx_ : i64,name_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(name_).unwrap();
      self.handle_res(unsafe { MSK_putaccname(self.ptr,accidx_,__tmp_1.as_ptr()) },"put_acc_name")?;
      return Result::Ok(());
    } // putaccname
    /// Replaces all elements in one column of the linear constraint matrix.
    ///
    /// # Arguments
    ///
    /// - `j_` Column index.
    /// - `subj_` Row indexes of non-zero values in column.
    /// - `valj_` New non-zero values of column.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putacol>
    #[allow(unused_parens)]
    pub fn put_a_col(&mut self,j_ : i32,subj_ : &[i32],valj_ : &[f64]) -> Result<(),String> {
      let nzj_ : i32 = std::cmp::min(valj_.len(),subj_.len()) as i32;
      self.handle_res(unsafe { MSK_putacol(self.ptr,j_,nzj_,subj_.as_ptr(),valj_.as_ptr()) },"put_a_col")?;
      return Result::Ok(());
    } // putacol
    /// Replaces all elements in several columns the linear constraint matrix.
    ///
    /// # Arguments
    ///
    /// - `sub_` Indexes of columns that should be replaced.
    /// - `ptrb_` Array of pointers to the first element in the columns.
    /// - `ptre_` Array of pointers to the last element plus one in the columns.
    /// - `asub_` Row indexes
    /// - `aval_` Coefficient values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putacollist64>
    #[allow(unused_parens)]
    pub fn put_a_col_list(&mut self,sub_ : &[i32],ptrb_ : &[i64],ptre_ : &[i64],asub_ : &[i32],aval_ : &[f64]) -> Result<(),String> {
      let num_ : i32 = std::cmp::min(std::cmp::min(ptrb_.len(),sub_.len()),ptre_.len()) as i32;
      if asub_.len() != aval_.len() { return Err("put_a_col_list: Mismatching asub/aval lengths".to_string()); } 
      if ptrb_.len() != ptre_.len() { return Err("put_a_col_list: Mismatching ptrb/ptre lengths".to_string()); } 
      if ! ptrb_.iter().zip(ptre_.iter()).all(|(a,b)| *a <= *b) { return Err("put_a_col_list: Invalid ptrb/ptre construction".to_string()); } 
      if let Some(v) = ptrb_.iter().min() { if *v < 0 { return Err("put_a_col_list: Invalid ptrb construction".to_string()); } }
      if let Some(v) = ptre_.iter().max() { if *v as usize > asub_.len() { return Err("put_a_col_list: Invalid ptre construction".to_string()); } } 
      self.handle_res(unsafe { MSK_putacollist64(self.ptr,num_,sub_.as_ptr(),ptrb_.as_ptr(),ptre_.as_ptr(),asub_.as_ptr(),aval_.as_ptr()) },"put_a_col_list")?;
      return Result::Ok(());
    } // putacollist64
    /// Replaces all elements in a sequence of columns the linear constraint matrix.
    ///
    /// # Arguments
    ///
    /// - `first_` First column in the slice.
    /// - `last_` Last column plus one in the slice.
    /// - `ptrb_` Array of pointers to the first element in the columns.
    /// - `ptre_` Array of pointers to the last element plus one in the columns.
    /// - `asub_` Row indexes
    /// - `aval_` Coefficient values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putacolslice64>
    #[allow(unused_parens)]
    pub fn put_a_col_slice(&mut self,first_ : i32,last_ : i32,ptrb_ : &[i64],ptre_ : &[i64],asub_ : &[i32],aval_ : &[f64]) -> Result<(),String> {
      if asub_.len() != aval_.len() { return Err("put_a_col_slice: Mismatching asub/aval lengths".to_string()); } 
      if ptrb_.len() != ptre_.len() { return Err("put_a_col_slice: Mismatching ptrb/ptre lengths".to_string()); } 
      if ! ptrb_.iter().zip(ptre_.iter()).all(|(a,b)| *a <= *b) { return Err("put_a_col_slice: Invalid ptrb/ptre construction".to_string()); } 
      if let Some(v) = ptrb_.iter().min() { if *v < 0 { return Err("put_a_col_slice: Invalid ptrb construction".to_string()); } }
      if let Some(v) = ptre_.iter().max() { if *v as usize > asub_.len() { return Err("put_a_col_slice: Invalid ptre construction".to_string()); } } 
      self.handle_res(unsafe { MSK_putacolslice64(self.ptr,first_,last_,ptrb_.as_ptr(),ptre_.as_ptr(),asub_.as_ptr(),aval_.as_ptr()) },"put_a_col_slice")?;
      return Result::Ok(());
    } // putacolslice64
    /// Inputs barF in block triplet form.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Constraint index.
    /// - `barvaridx_` Symmetric matrix variable index.
    /// - `subk_` Block row index.
    /// - `subl_` Block column index.
    /// - `valkl_` The numerical value associated with each block triplet.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putafebarfblocktriplet>
    #[allow(unused_parens)]
    pub fn put_afe_barf_block_triplet(&mut self,afeidx_ : &[i64],barvaridx_ : &[i32],subk_ : &[i32],subl_ : &[i32],valkl_ : &[f64]) -> Result<(),String> {
      let numtrip_ : i64 = std::cmp::min(std::cmp::min(std::cmp::min(std::cmp::min(subk_.len(),barvaridx_.len()),valkl_.len()),afeidx_.len()),subl_.len()) as i64;
      if afeidx_.len() != (numtrip_).try_into().unwrap() {
        return Result::Err("put_afe_barf_block_triplet: Argument 'afeidx' has the wrong length, expected numtrip_".to_string());
      }
      if barvaridx_.len() != (numtrip_).try_into().unwrap() {
        return Result::Err("put_afe_barf_block_triplet: Argument 'barvaridx' has the wrong length, expected numtrip_".to_string());
      }
      if subk_.len() != (numtrip_).try_into().unwrap() {
        return Result::Err("put_afe_barf_block_triplet: Argument 'subk' has the wrong length, expected numtrip_".to_string());
      }
      if subl_.len() != (numtrip_).try_into().unwrap() {
        return Result::Err("put_afe_barf_block_triplet: Argument 'subl' has the wrong length, expected numtrip_".to_string());
      }
      if valkl_.len() != (numtrip_).try_into().unwrap() {
        return Result::Err("put_afe_barf_block_triplet: Argument 'valkl' has the wrong length, expected numtrip_".to_string());
      }
      self.handle_res(unsafe { MSK_putafebarfblocktriplet(self.ptr,numtrip_,afeidx_.as_ptr(),barvaridx_.as_ptr(),subk_.as_ptr(),subl_.as_ptr(),valkl_.as_ptr()) },"put_afe_barf_block_triplet")?;
      return Result::Ok(());
    } // putafebarfblocktriplet
    /// Inputs one entry in barF.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row index of barF.
    /// - `barvaridx_` Semidefinite variable index.
    /// - `termidx_` Element indices in matrix storage.
    /// - `termweight_` Weights in the weighted sum.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putafebarfentry>
    #[allow(unused_parens)]
    pub fn put_afe_barf_entry(&mut self,afeidx_ : i64,barvaridx_ : i32,termidx_ : &[i64],termweight_ : &[f64]) -> Result<(),String> {
      let numterm_ : i64 = std::cmp::min(termweight_.len(),termidx_.len()) as i64;
      self.handle_res(unsafe { MSK_putafebarfentry(self.ptr,afeidx_,barvaridx_,numterm_,termidx_.as_ptr(),termweight_.as_ptr()) },"put_afe_barf_entry")?;
      return Result::Ok(());
    } // putafebarfentry
    /// Inputs a list of entries in barF.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row indexes of barF.
    /// - `barvaridx_` Semidefinite variable indexes.
    /// - `numterm_` Number of terms in the weighted sums.
    /// - `ptrterm_` Pointer to the terms forming each entry.
    /// - `termidx_` Concatenated element indexes in matrix storage.
    /// - `termweight_` Concatenated weights in the weighted sum.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putafebarfentrylist>
    #[allow(unused_parens)]
    pub fn put_afe_barf_entry_list(&mut self,afeidx_ : &[i64],barvaridx_ : &[i32],numterm_ : &[i64],ptrterm_ : &[i64],termidx_ : &[i64],termweight_ : &[f64]) -> Result<(),String> {
      let numafeidx_ : i64 = std::cmp::min(std::cmp::min(std::cmp::min(numterm_.len(),ptrterm_.len()),barvaridx_.len()),afeidx_.len()) as i64;
      let lenterm_ : i64 = std::cmp::min(termweight_.len(),termidx_.len()) as i64;
      self.handle_res(unsafe { MSK_putafebarfentrylist(self.ptr,numafeidx_,afeidx_.as_ptr(),barvaridx_.as_ptr(),numterm_.as_ptr(),ptrterm_.as_ptr(),lenterm_,termidx_.as_ptr(),termweight_.as_ptr()) },"put_afe_barf_entry_list")?;
      return Result::Ok(());
    } // putafebarfentrylist
    /// Inputs a row of barF.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row index of barF.
    /// - `barvaridx_` Semidefinite variable indexes.
    /// - `numterm_` Number of terms in the weighted sums.
    /// - `ptrterm_` Pointer to the terms forming each entry.
    /// - `termidx_` Concatenated element indexes in matrix storage.
    /// - `termweight_` Concatenated weights in the weighted sum.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putafebarfrow>
    #[allow(unused_parens)]
    pub fn put_afe_barf_row(&mut self,afeidx_ : i64,barvaridx_ : &[i32],numterm_ : &[i64],ptrterm_ : &[i64],termidx_ : &[i64],termweight_ : &[f64]) -> Result<(),String> {
      let numentr_ : i32 = std::cmp::min(std::cmp::min(numterm_.len(),ptrterm_.len()),barvaridx_.len()) as i32;
      let lenterm_ : i64 = std::cmp::min(termweight_.len(),termidx_.len()) as i64;
      self.handle_res(unsafe { MSK_putafebarfrow(self.ptr,afeidx_,numentr_,barvaridx_.as_ptr(),numterm_.as_ptr(),ptrterm_.as_ptr(),lenterm_,termidx_.as_ptr(),termweight_.as_ptr()) },"put_afe_barf_row")?;
      return Result::Ok(());
    } // putafebarfrow
    /// Replaces all elements in one column of the F matrix in the affine expressions.
    ///
    /// # Arguments
    ///
    /// - `varidx_` Column index.
    /// - `afeidx_` Row indexes of non-zero values in the column.
    /// - `val_` New non-zero values in the column.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putafefcol>
    #[allow(unused_parens)]
    pub fn put_afe_f_col(&mut self,varidx_ : i32,afeidx_ : &[i64],val_ : &[f64]) -> Result<(),String> {
      let numnz_ : i64 = std::cmp::min(val_.len(),afeidx_.len()) as i64;
      self.handle_res(unsafe { MSK_putafefcol(self.ptr,varidx_,numnz_,afeidx_.as_ptr(),val_.as_ptr()) },"put_afe_f_col")?;
      return Result::Ok(());
    } // putafefcol
    /// Replaces one entry in F.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row index in F.
    /// - `varidx_` Column index in F.
    /// - `value_` Value of the entry.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putafefentry>
    #[allow(unused_parens)]
    pub fn put_afe_f_entry(&mut self,afeidx_ : i64,varidx_ : i32,value_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putafefentry(self.ptr,afeidx_,varidx_,value_) },"put_afe_f_entry")?;
      return Result::Ok(());
    } // putafefentry
    /// Replaces a list of entries in F.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row indices in F.
    /// - `varidx_` Column indices in F.
    /// - `val_` Values of the entries in F.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putafefentrylist>
    #[allow(unused_parens)]
    pub fn put_afe_f_entry_list(&mut self,afeidx_ : &[i64],varidx_ : &[i32],val_ : &[f64]) -> Result<(),String> {
      let numentr_ : i64 = std::cmp::min(std::cmp::min(val_.len(),varidx_.len()),afeidx_.len()) as i64;
      self.handle_res(unsafe { MSK_putafefentrylist(self.ptr,numentr_,afeidx_.as_ptr(),varidx_.as_ptr(),val_.as_ptr()) },"put_afe_f_entry_list")?;
      return Result::Ok(());
    } // putafefentrylist
    /// Replaces all elements in one row of the F matrix in the affine expressions.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row index.
    /// - `varidx_` Column indexes of non-zero values in the row.
    /// - `val_` New non-zero values in the row.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putafefrow>
    #[allow(unused_parens)]
    pub fn put_afe_f_row(&mut self,afeidx_ : i64,varidx_ : &[i32],val_ : &[f64]) -> Result<(),String> {
      let numnz_ : i32 = std::cmp::min(val_.len(),varidx_.len()) as i32;
      self.handle_res(unsafe { MSK_putafefrow(self.ptr,afeidx_,numnz_,varidx_.as_ptr(),val_.as_ptr()) },"put_afe_f_row")?;
      return Result::Ok(());
    } // putafefrow
    /// Replaces all elements in a number of rows of the F matrix in the affine expressions.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row indices.
    /// - `numnzrow_` Number of non-zeros in each row.
    /// - `ptrrow_` Pointer to the first nonzero in each row.
    /// - `varidx_` Column indexes of non-zero values.
    /// - `val_` New non-zero values in the rows.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putafefrowlist>
    #[allow(unused_parens)]
    pub fn put_afe_f_row_list(&mut self,afeidx_ : &[i64],numnzrow_ : &[i32],ptrrow_ : &[i64],varidx_ : &[i32],val_ : &[f64]) -> Result<(),String> {
      let numafeidx_ : i64 = std::cmp::min(std::cmp::min(ptrrow_.len(),numnzrow_.len()),afeidx_.len()) as i64;
      let lenidxval_ : i64 = std::cmp::min(val_.len(),varidx_.len()) as i64;
      if varidx_.len() != val_.len() { return Err("put_afe_f_row_list: Mismatching varidx/val lengths".to_string()); } 
      if let Some(v) = numnzrow_.iter().min() { if *v < 0 { return Err("put_afe_f_row_list: Invalid numnzrow value".to_string()); } }
      if let Some(v) = ptrrow_.iter().min() { if *v < 0 { return Err("put_afe_f_row_list: Invalid ptrrow value".to_string()); } }
      if let Some(v) = ptrrow_.iter().zip(numnzrow_.iter()).map(|(a,b)| *a as usize + *b as usize).max() { if v > varidx_.len() { return Err("put_afe_f_row_list: Invalid ptrrow/numnzrow construction".to_string()); } }
      self.handle_res(unsafe { MSK_putafefrowlist(self.ptr,numafeidx_,afeidx_.as_ptr(),numnzrow_.as_ptr(),ptrrow_.as_ptr(),lenidxval_,varidx_.as_ptr(),val_.as_ptr()) },"put_afe_f_row_list")?;
      return Result::Ok(());
    } // putafefrowlist
    /// Replaces one element in the g vector in the affine expressions.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Row index.
    /// - `g_` New value for the element of g.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putafeg>
    #[allow(unused_parens)]
    pub fn put_afe_g(&mut self,afeidx_ : i64,g_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putafeg(self.ptr,afeidx_,g_) },"put_afe_g")?;
      return Result::Ok(());
    } // putafeg
    /// Replaces a list of elements in the g vector in the affine expressions.
    ///
    /// # Arguments
    ///
    /// - `afeidx_` Indices of entries in g.
    /// - `g_` New values for the elements of g.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putafeglist>
    #[allow(unused_parens)]
    pub fn put_afe_g_list(&mut self,afeidx_ : &[i64],g_ : &[f64]) -> Result<(),String> {
      let numafeidx_ : i64 = std::cmp::min(g_.len(),afeidx_.len()) as i64;
      self.handle_res(unsafe { MSK_putafeglist(self.ptr,numafeidx_,afeidx_.as_ptr(),g_.as_ptr()) },"put_afe_g_list")?;
      return Result::Ok(());
    } // putafeglist
    /// Modifies a slice of the vector g.
    ///
    /// # Arguments
    ///
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `slice_` The slice of g as a dense vector.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putafegslice>
    #[allow(unused_parens)]
    pub fn put_afe_g_slice(&mut self,first_ : i64,last_ : i64,slice_ : &[f64]) -> Result<(),String> {
      if slice_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("put_afe_g_slice: Argument 'slice' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putafegslice(self.ptr,first_,last_,slice_.as_ptr()) },"put_afe_g_slice")?;
      return Result::Ok(());
    } // putafegslice
    /// Changes a single value in the linear coefficient matrix.
    ///
    /// # Arguments
    ///
    /// - `i_` Constraint (row) index.
    /// - `j_` Variable (column) index.
    /// - `aij_` New coefficient.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putaij>
    #[allow(unused_parens)]
    pub fn put_aij(&mut self,i_ : i32,j_ : i32,aij_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putaij(self.ptr,i_,j_,aij_) },"put_aij")?;
      return Result::Ok(());
    } // putaij
    /// Changes one or more coefficients in the linear constraint matrix.
    ///
    /// # Arguments
    ///
    /// - `subi_` Constraint (row) indices.
    /// - `subj_` Variable (column) indices.
    /// - `valij_` New coefficient values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putaijlist64>
    #[allow(unused_parens)]
    pub fn put_aij_list(&mut self,subi_ : &[i32],subj_ : &[i32],valij_ : &[f64]) -> Result<(),String> {
      let num_ : i64 = std::cmp::min(std::cmp::min(subi_.len(),valij_.len()),subj_.len()) as i64;
      self.handle_res(unsafe { MSK_putaijlist64(self.ptr,num_,subi_.as_ptr(),subj_.as_ptr(),valij_.as_ptr()) },"put_aij_list")?;
      return Result::Ok(());
    } // putaijlist64
    /// Replaces all elements in one row of the linear constraint matrix.
    ///
    /// # Arguments
    ///
    /// - `i_` Row index.
    /// - `subi_` Column indexes of non-zero values in row.
    /// - `vali_` New non-zero values of row.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putarow>
    #[allow(unused_parens)]
    pub fn put_a_row(&mut self,i_ : i32,subi_ : &[i32],vali_ : &[f64]) -> Result<(),String> {
      let nzi_ : i32 = std::cmp::min(subi_.len(),vali_.len()) as i32;
      self.handle_res(unsafe { MSK_putarow(self.ptr,i_,nzi_,subi_.as_ptr(),vali_.as_ptr()) },"put_a_row")?;
      return Result::Ok(());
    } // putarow
    /// Replaces all elements in several rows of the linear constraint matrix.
    ///
    /// # Arguments
    ///
    /// - `sub_` Indexes of rows or columns that should be replaced.
    /// - `ptrb_` Array of pointers to the first element in the rows.
    /// - `ptre_` Array of pointers to the last element plus one in the rows.
    /// - `asub_` Variable indexes.
    /// - `aval_` Coefficient values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putarowlist64>
    #[allow(unused_parens)]
    pub fn put_a_row_list(&mut self,sub_ : &[i32],ptrb_ : &[i64],ptre_ : &[i64],asub_ : &[i32],aval_ : &[f64]) -> Result<(),String> {
      let num_ : i32 = std::cmp::min(std::cmp::min(ptrb_.len(),sub_.len()),ptre_.len()) as i32;
      if asub_.len() != aval_.len() { return Err("put_a_row_list: Mismatching asub/aval lengths".to_string()); } 
      if ptrb_.len() != ptre_.len() { return Err("put_a_row_list: Mismatching ptrb/ptre lengths".to_string()); } 
      if ! ptrb_.iter().zip(ptre_.iter()).all(|(a,b)| *a <= *b) { return Err("put_a_row_list: Invalid ptrb/ptre construction".to_string()); } 
      if let Some(v) = ptrb_.iter().min() { if *v < 0 { return Err("put_a_row_list: Invalid ptrb construction".to_string()); } }
      if let Some(v) = ptre_.iter().max() { if *v as usize > asub_.len() { return Err("put_a_row_list: Invalid ptre construction".to_string()); } } 
      self.handle_res(unsafe { MSK_putarowlist64(self.ptr,num_,sub_.as_ptr(),ptrb_.as_ptr(),ptre_.as_ptr(),asub_.as_ptr(),aval_.as_ptr()) },"put_a_row_list")?;
      return Result::Ok(());
    } // putarowlist64
    /// Replaces all elements in several rows the linear constraint matrix.
    ///
    /// # Arguments
    ///
    /// - `first_` First row in the slice.
    /// - `last_` Last row plus one in the slice.
    /// - `ptrb_` Array of pointers to the first element in the rows.
    /// - `ptre_` Array of pointers to the last element plus one in the rows.
    /// - `asub_` Column indexes of new elements.
    /// - `aval_` Coefficient values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putarowslice64>
    #[allow(unused_parens)]
    pub fn put_a_row_slice(&mut self,first_ : i32,last_ : i32,ptrb_ : &[i64],ptre_ : &[i64],asub_ : &[i32],aval_ : &[f64]) -> Result<(),String> {
      if ptrb_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("put_a_row_slice: Argument 'ptrb' has the wrong length, expected (last_-first_)".to_string());
      }
      if ptre_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("put_a_row_slice: Argument 'ptre' has the wrong length, expected (last_-first_)".to_string());
      }
      if asub_.len() != aval_.len() { return Err("put_a_row_slice: Mismatching asub/aval lengths".to_string()); } 
      if ptrb_.len() != ptre_.len() { return Err("put_a_row_slice: Mismatching ptrb/ptre lengths".to_string()); } 
      if ! ptrb_.iter().zip(ptre_.iter()).all(|(a,b)| *a <= *b) { return Err("put_a_row_slice: Invalid ptrb/ptre construction".to_string()); } 
      if let Some(v) = ptrb_.iter().min() { if *v < 0 { return Err("put_a_row_slice: Invalid ptrb construction".to_string()); } }
      if let Some(v) = ptre_.iter().max() { if *v as usize > asub_.len() { return Err("put_a_row_slice: Invalid ptre construction".to_string()); } } 
      self.handle_res(unsafe { MSK_putarowslice64(self.ptr,first_,last_,ptrb_.as_ptr(),ptre_.as_ptr(),asub_.as_ptr(),aval_.as_ptr()) },"put_a_row_slice")?;
      return Result::Ok(());
    } // putarowslice64
    /// Truncates all elements in A below a certain tolerance to zero.
    ///
    /// # Arguments
    ///
    /// - `tolzero_` Truncation tolerance.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putatruncatetol>
    #[allow(unused_parens)]
    pub fn put_a_truncate_tol(&mut self,tolzero_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putatruncatetol(self.ptr,tolzero_) },"put_a_truncate_tol")?;
      return Result::Ok(());
    } // putatruncatetol
    /// Inputs barA in block triplet form.
    ///
    /// # Arguments
    ///
    /// - `subi_` Constraint index.
    /// - `subj_` Symmetric matrix variable index.
    /// - `subk_` Block row index.
    /// - `subl_` Block column index.
    /// - `valijkl_` The numerical value associated with each block triplet.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putbarablocktriplet>
    #[allow(unused_parens)]
    pub fn put_bara_block_triplet(&mut self,subi_ : &[i32],subj_ : &[i32],subk_ : &[i32],subl_ : &[i32],valijkl_ : &[f64]) -> Result<(),String> {
      let num_ : i64 = std::cmp::min(std::cmp::min(std::cmp::min(subl_.len(),subk_.len()),valijkl_.len()),subj_.len()) as i64;
      if subi_.len() != (num_).try_into().unwrap() {
        return Result::Err("put_bara_block_triplet: Argument 'subi' has the wrong length, expected num_".to_string());
      }
      if subj_.len() != (num_).try_into().unwrap() {
        return Result::Err("put_bara_block_triplet: Argument 'subj' has the wrong length, expected num_".to_string());
      }
      if subk_.len() != (num_).try_into().unwrap() {
        return Result::Err("put_bara_block_triplet: Argument 'subk' has the wrong length, expected num_".to_string());
      }
      if subl_.len() != (num_).try_into().unwrap() {
        return Result::Err("put_bara_block_triplet: Argument 'subl' has the wrong length, expected num_".to_string());
      }
      if valijkl_.len() != (num_).try_into().unwrap() {
        return Result::Err("put_bara_block_triplet: Argument 'valijkl' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_putbarablocktriplet(self.ptr,num_,subi_.as_ptr(),subj_.as_ptr(),subk_.as_ptr(),subl_.as_ptr(),valijkl_.as_ptr()) },"put_bara_block_triplet")?;
      return Result::Ok(());
    } // putbarablocktriplet
    /// Inputs an element of barA.
    ///
    /// # Arguments
    ///
    /// - `i_` Row index of barA.
    /// - `j_` Column index of barA.
    /// - `sub_` Element indexes in matrix storage.
    /// - `weights_` Weights in the weighted sum.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putbaraij>
    #[allow(unused_parens)]
    pub fn put_bara_ij(&mut self,i_ : i32,j_ : i32,sub_ : &[i64],weights_ : &[f64]) -> Result<(),String> {
      let num_ : i64 = std::cmp::min(sub_.len(),weights_.len()) as i64;
      self.handle_res(unsafe { MSK_putbaraij(self.ptr,i_,j_,num_,sub_.as_ptr(),weights_.as_ptr()) },"put_bara_ij")?;
      return Result::Ok(());
    } // putbaraij
    /// Inputs list of elements of barA.
    ///
    /// # Arguments
    ///
    /// - `subi_` Row index of barA.
    /// - `subj_` Column index of barA.
    /// - `alphaptrb_` Start entries for terms in the weighted sum.
    /// - `alphaptre_` End entries for terms in the weighted sum.
    /// - `matidx_` Element indexes in matrix storage.
    /// - `weights_` Weights in the weighted sum.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putbaraijlist>
    #[allow(unused_parens)]
    pub fn put_bara_ij_list(&mut self,subi_ : &[i32],subj_ : &[i32],alphaptrb_ : &[i64],alphaptre_ : &[i64],matidx_ : &[i64],weights_ : &[f64]) -> Result<(),String> {
      let num_ : i32 = std::cmp::min(std::cmp::min(std::cmp::min(subi_.len(),alphaptre_.len()),alphaptrb_.len()),subj_.len()) as i32;
      if matidx_.len() != weights_.len() { return Err("put_bara_ij_list: Mismatching matidx/weights lengths".to_string()); } 
      if alphaptrb_.len() != alphaptre_.len() { return Err("put_bara_ij_list: Mismatching alphaptrb/alphaptre lengths".to_string()); } 
      if ! alphaptrb_.iter().zip(alphaptre_.iter()).all(|(a,b)| *a <= *b) { return Err("put_bara_ij_list: Invalid alphaptrb/alphaptre construction".to_string()); } 
      if let Some(v) = alphaptrb_.iter().min() { if *v < 0 { return Err("put_bara_ij_list: Invalid alphaptrb construction".to_string()); } }
      if let Some(v) = alphaptre_.iter().max() { if *v as usize > matidx_.len() { return Err("put_bara_ij_list: Invalid alphaptre construction".to_string()); } } 
      self.handle_res(unsafe { MSK_putbaraijlist(self.ptr,num_,subi_.as_ptr(),subj_.as_ptr(),alphaptrb_.as_ptr(),alphaptre_.as_ptr(),matidx_.as_ptr(),weights_.as_ptr()) },"put_bara_ij_list")?;
      return Result::Ok(());
    } // putbaraijlist
    /// Replace a set of rows of barA
    ///
    /// # Arguments
    ///
    /// - `subi_` Row indexes of barA.
    /// - `ptrb_` Start of rows in barA.
    /// - `ptre_` End of rows in barA.
    /// - `subj_` Column index of barA.
    /// - `nummat_` Number of entries in weighted sum of matrixes.
    /// - `matidx_` Matrix indexes for weighted sum of matrixes.
    /// - `weights_` Weights for weighted sum of matrixes.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putbararowlist>
    #[allow(unused_parens)]
    pub fn put_bara_row_list(&mut self,subi_ : &[i32],ptrb_ : &[i64],ptre_ : &[i64],subj_ : &[i32],nummat_ : &[i64],matidx_ : &[i64],weights_ : &[f64]) -> Result<(),String> {
      let num_ : i32 = std::cmp::min(std::cmp::min(subi_.len(),ptrb_.len()),ptre_.len()) as i32;
      if nummat_.len() != (subj_.len()).try_into().unwrap() {
        return Result::Err("put_bara_row_list: Argument 'nummat' has the wrong length, expected subj_.len()".to_string());
      }
      let mut __tmp_0 : i64 = i64::default();
      for __tmp_1 in nummat_ { __tmp_0 += __tmp_1; }
      if matidx_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("put_bara_row_list: Argument 'matidx' has the wrong length, expected __tmp_0".to_string());
      }
      let mut __tmp_2 : i64 = i64::default();
      for __tmp_3 in nummat_ { __tmp_2 += __tmp_3; }
      if weights_.len() != (__tmp_2).try_into().unwrap() {
        return Result::Err("put_bara_row_list: Argument 'weights' has the wrong length, expected __tmp_2".to_string());
      }
      if subj_.len() != nummat_.len() { return Err("put_bara_row_list: Mismatching subj/nummat lengths".to_string()); } 
      if ptrb_.len() != ptre_.len() { return Err("put_bara_row_list: Mismatching ptrb/ptre lengths".to_string()); } 
      if ! ptrb_.iter().zip(ptre_.iter()).all(|(a,b)| *a <= *b) { return Err("put_bara_row_list: Invalid ptrb/ptre construction".to_string()); } 
      if let Some(v) = ptrb_.iter().min() { if *v < 0 { return Err("put_bara_row_list: Invalid ptrb construction".to_string()); } }
      if let Some(v) = ptre_.iter().max() { if *v as usize > subj_.len() { return Err("put_bara_row_list: Invalid ptre construction".to_string()); } } 
      self.handle_res(unsafe { MSK_putbararowlist(self.ptr,num_,subi_.as_ptr(),ptrb_.as_ptr(),ptre_.as_ptr(),subj_.as_ptr(),nummat_.as_ptr(),matidx_.as_ptr(),weights_.as_ptr()) },"put_bara_row_list")?;
      return Result::Ok(());
    } // putbararowlist
    /// Inputs barC in block triplet form.
    ///
    /// # Arguments
    ///
    /// - `subj_` Symmetric matrix variable index.
    /// - `subk_` Block row index.
    /// - `subl_` Block column index.
    /// - `valjkl_` The numerical value associated with each block triplet.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putbarcblocktriplet>
    #[allow(unused_parens)]
    pub fn put_barc_block_triplet(&mut self,subj_ : &[i32],subk_ : &[i32],subl_ : &[i32],valjkl_ : &[f64]) -> Result<(),String> {
      let num_ : i64 = std::cmp::min(std::cmp::min(std::cmp::min(subl_.len(),valjkl_.len()),subk_.len()),subj_.len()) as i64;
      if subj_.len() != (num_).try_into().unwrap() {
        return Result::Err("put_barc_block_triplet: Argument 'subj' has the wrong length, expected num_".to_string());
      }
      if subk_.len() != (num_).try_into().unwrap() {
        return Result::Err("put_barc_block_triplet: Argument 'subk' has the wrong length, expected num_".to_string());
      }
      if subl_.len() != (num_).try_into().unwrap() {
        return Result::Err("put_barc_block_triplet: Argument 'subl' has the wrong length, expected num_".to_string());
      }
      if valjkl_.len() != (num_).try_into().unwrap() {
        return Result::Err("put_barc_block_triplet: Argument 'valjkl' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_putbarcblocktriplet(self.ptr,num_,subj_.as_ptr(),subk_.as_ptr(),subl_.as_ptr(),valjkl_.as_ptr()) },"put_barc_block_triplet")?;
      return Result::Ok(());
    } // putbarcblocktriplet
    /// Changes one element in barc.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the element in barc` that should be changed.
    /// - `sub_` sub is list of indexes of those symmetric matrices appearing in sum.
    /// - `weights_` The weights of the terms in the weighted sum.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putbarcj>
    #[allow(unused_parens)]
    pub fn put_barc_j(&mut self,j_ : i32,sub_ : &[i64],weights_ : &[f64]) -> Result<(),String> {
      let num_ : i64 = std::cmp::min(sub_.len(),weights_.len()) as i64;
      self.handle_res(unsafe { MSK_putbarcj(self.ptr,j_,num_,sub_.as_ptr(),weights_.as_ptr()) },"put_barc_j")?;
      return Result::Ok(());
    } // putbarcj
    /// Sets the dual solution for a semidefinite variable.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `j_` Index of the semidefinite variable.
    /// - `barsj_` Value of the j'th variable of barx.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putbarsj>
    #[allow(unused_parens)]
    pub fn put_bars_j(&mut self,whichsol_ : i32,j_ : i32,barsj_ : &[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getlenbarvarj(self.ptr,j_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getlenbarvarj")?;
      if barsj_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("put_bars_j: Argument 'barsj' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_putbarsj(self.ptr,whichsol_,j_,barsj_.as_ptr()) },"put_bars_j")?;
      return Result::Ok(());
    } // putbarsj
    /// Sets the name of a semidefinite variable.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the variable.
    /// - `name_` The variable name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putbarvarname>
    #[allow(unused_parens)]
    pub fn put_barvar_name(&mut self,j_ : i32,name_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(name_).unwrap();
      self.handle_res(unsafe { MSK_putbarvarname(self.ptr,j_,__tmp_1.as_ptr()) },"put_barvar_name")?;
      return Result::Ok(());
    } // putbarvarname
    /// Sets the primal solution for a semidefinite variable.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `j_` Index of the semidefinite variable.
    /// - `barxj_` Value of the j'th variable of barx.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putbarxj>
    #[allow(unused_parens)]
    pub fn put_barx_j(&mut self,whichsol_ : i32,j_ : i32,barxj_ : &[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getlenbarvarj(self.ptr,j_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getlenbarvarj")?;
      if barxj_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("put_barx_j: Argument 'barxj' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_putbarxj(self.ptr,whichsol_,j_,barxj_.as_ptr()) },"put_barx_j")?;
      return Result::Ok(());
    } // putbarxj
    /// Replaces the fixed term in the objective.
    ///
    /// # Arguments
    ///
    /// - `cfix_` Fixed term in the objective.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putcfix>
    #[allow(unused_parens)]
    pub fn put_cfix(&mut self,cfix_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putcfix(self.ptr,cfix_) },"put_cfix")?;
      return Result::Ok(());
    } // putcfix
    /// Modifies one linear coefficient in the objective.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the variable whose objective coefficient should be changed.
    /// - `cj_` New coefficient value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putcj>
    #[allow(unused_parens)]
    pub fn put_c_j(&mut self,j_ : i32,cj_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putcj(self.ptr,j_,cj_) },"put_c_j")?;
      return Result::Ok(());
    } // putcj
    /// Modifies a part of the linear objective coefficients.
    ///
    /// # Arguments
    ///
    /// - `subj_` Indices of variables for which objective coefficients should be changed.
    /// - `val_` New numerical values for the objective coefficients that should be modified.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putclist>
    #[allow(unused_parens)]
    pub fn put_c_list(&mut self,subj_ : &[i32],val_ : &[f64]) -> Result<(),String> {
      let num_ : i32 = std::cmp::min(val_.len(),subj_.len()) as i32;
      self.handle_res(unsafe { MSK_putclist(self.ptr,num_,subj_.as_ptr(),val_.as_ptr()) },"put_c_list")?;
      return Result::Ok(());
    } // putclist
    /// Changes the bound for one constraint.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the constraint.
    /// - `bkc_` New bound key.
    ///   
    ///   See [Boundkey]
    /// - `blc_` New lower bound.
    /// - `buc_` New upper bound.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putconbound>
    #[allow(unused_parens)]
    pub fn put_con_bound(&mut self,i_ : i32,bkc_ : i32,blc_ : f64,buc_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putconbound(self.ptr,i_,bkc_,blc_,buc_) },"put_con_bound")?;
      return Result::Ok(());
    } // putconbound
    /// Changes the bounds of a list of constraints.
    ///
    /// # Arguments
    ///
    /// - `sub_` List of constraint indexes.
    /// - `bkc_` Bound keys for the constraints.
    ///   
    ///   See [Boundkey]
    /// - `blc_` Lower bounds for the constraints.
    /// - `buc_` Upper bounds for the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putconboundlist>
    #[allow(unused_parens)]
    pub fn put_con_bound_list(&mut self,sub_ : &[i32],bkc_ : &[i32],blc_ : &[f64],buc_ : &[f64]) -> Result<(),String> {
      let num_ : i32 = std::cmp::min(std::cmp::min(std::cmp::min(buc_.len(),bkc_.len()),sub_.len()),blc_.len()) as i32;
      self.handle_res(unsafe { MSK_putconboundlist(self.ptr,num_,sub_.as_ptr(),bkc_.as_ptr(),blc_.as_ptr(),buc_.as_ptr()) },"put_con_bound_list")?;
      return Result::Ok(());
    } // putconboundlist
    /// Changes the bounds of a list of constraints.
    ///
    /// # Arguments
    ///
    /// - `sub_` List of constraint indexes.
    /// - `bkc_` New bound key for all constraints in the list.
    ///   
    ///   See [Boundkey]
    /// - `blc_` New lower bound for all constraints in the list.
    /// - `buc_` New upper bound for all constraints in the list.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putconboundlistconst>
    #[allow(unused_parens)]
    pub fn put_con_bound_list_const(&mut self,sub_ : &[i32],bkc_ : i32,blc_ : f64,buc_ : f64) -> Result<(),String> {
      let num_ : i32 = sub_.len() as i32;
      self.handle_res(unsafe { MSK_putconboundlistconst(self.ptr,num_,sub_.as_ptr(),bkc_,blc_,buc_) },"put_con_bound_list_const")?;
      return Result::Ok(());
    } // putconboundlistconst
    /// Changes the bounds for a slice of the constraints.
    ///
    /// # Arguments
    ///
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `bkc_` Bound keys for the constraints.
    ///   
    ///   See [Boundkey]
    /// - `blc_` Lower bounds for the constraints.
    /// - `buc_` Upper bounds for the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putconboundslice>
    #[allow(unused_parens)]
    pub fn put_con_bound_slice(&mut self,first_ : i32,last_ : i32,bkc_ : &[i32],blc_ : &[f64],buc_ : &[f64]) -> Result<(),String> {
      if bkc_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("put_con_bound_slice: Argument 'bkc' has the wrong length, expected (last_-first_)".to_string());
      }
      if blc_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("put_con_bound_slice: Argument 'blc' has the wrong length, expected (last_-first_)".to_string());
      }
      if buc_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("put_con_bound_slice: Argument 'buc' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putconboundslice(self.ptr,first_,last_,bkc_.as_ptr(),blc_.as_ptr(),buc_.as_ptr()) },"put_con_bound_slice")?;
      return Result::Ok(());
    } // putconboundslice
    /// Changes the bounds for a slice of the constraints.
    ///
    /// # Arguments
    ///
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `bkc_` New bound key for all constraints in the slice.
    ///   
    ///   See [Boundkey]
    /// - `blc_` New lower bound for all constraints in the slice.
    /// - `buc_` New upper bound for all constraints in the slice.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putconboundsliceconst>
    #[allow(unused_parens)]
    pub fn put_con_bound_slice_const(&mut self,first_ : i32,last_ : i32,bkc_ : i32,blc_ : f64,buc_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putconboundsliceconst(self.ptr,first_,last_,bkc_,blc_,buc_) },"put_con_bound_slice_const")?;
      return Result::Ok(());
    } // putconboundsliceconst
    /// Replaces a conic constraint.
    ///
    /// # Arguments
    ///
    /// - `k_` Index of the cone.
    /// - `ct_` Specifies the type of the cone.
    ///   
    ///   See [Conetype]
    /// - `conepar_` For the power cone it denotes the exponent alpha. For other cone types it is unused and can be set to 0.
    /// - `submem_` Variable subscripts of the members in the cone.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putcone>
    #[allow(unused_parens)]
    pub fn put_cone(&mut self,k_ : i32,ct_ : i32,conepar_ : f64,submem_ : &[i32]) -> Result<(),String> {
      let nummem_ : i32 = submem_.len() as i32;
      self.handle_res(unsafe { MSK_putcone(self.ptr,k_,ct_,conepar_,nummem_,submem_.as_ptr()) },"put_cone")?;
      return Result::Ok(());
    } // putcone
    /// Sets the name of a cone.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the cone.
    /// - `name_` The name of the cone.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putconename>
    #[allow(unused_parens)]
    pub fn put_cone_name(&mut self,j_ : i32,name_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(name_).unwrap();
      self.handle_res(unsafe { MSK_putconename(self.ptr,j_,__tmp_1.as_ptr()) },"put_cone_name")?;
      return Result::Ok(());
    } // putconename
    /// Sets the name of a constraint.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the constraint.
    /// - `name_` The name of the constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putconname>
    #[allow(unused_parens)]
    pub fn put_con_name(&mut self,i_ : i32,name_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(name_).unwrap();
      self.handle_res(unsafe { MSK_putconname(self.ptr,i_,__tmp_1.as_ptr()) },"put_con_name")?;
      return Result::Ok(());
    } // putconname
    /// Sets the primal and dual solution information for a single constraint.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the constraint.
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sk_` Status key of the constraint.
    ///   
    ///   See [Stakey]
    /// - `x_` Primal solution value of the constraint.
    /// - `sl_` Solution value of the dual variable associated with the lower bound.
    /// - `su_` Solution value of the dual variable associated with the upper bound.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putconsolutioni>
    #[allow(unused_parens)]
    pub fn put_con_solution_i(&mut self,i_ : i32,whichsol_ : i32,sk_ : i32,x_ : f64,sl_ : f64,su_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putconsolutioni(self.ptr,i_,whichsol_,sk_,x_,sl_,su_) },"put_con_solution_i")?;
      return Result::Ok(());
    } // putconsolutioni
    /// Modifies a slice of the linear objective coefficients.
    ///
    /// # Arguments
    ///
    /// - `first_` First element in the slice of c.
    /// - `last_` Last element plus 1 of the slice in c to be changed.
    /// - `slice_` New numerical values for the objective coefficients that should be modified.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putcslice>
    #[allow(unused_parens)]
    pub fn put_c_slice(&mut self,first_ : i32,last_ : i32,slice_ : &[f64]) -> Result<(),String> {
      if slice_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("put_c_slice: Argument 'slice' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putcslice(self.ptr,first_,last_,slice_.as_ptr()) },"put_c_slice")?;
      return Result::Ok(());
    } // putcslice
    /// Inputs a disjunctive constraint.
    ///
    /// # Arguments
    ///
    /// - `djcidx_` Index of the disjunctive constraint.
    /// - `domidxlist_` List of domain indexes.
    /// - `afeidxlist_` List of affine expression indexes.
    /// - `b_` The vector of constant terms added to affine expressions.
    /// - `termsizelist_` List of term sizes.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putdjc>
    #[allow(unused_parens)]
    pub fn put_djc(&mut self,djcidx_ : i64,domidxlist_ : &[i64],afeidxlist_ : &[i64],b_ : &[f64],termsizelist_ : &[i64]) -> Result<(),String> {
      let numdomidx_ : i64 = domidxlist_.len() as i64;
      let numafeidx_ : i64 = afeidxlist_.len() as i64;
      if b_.len() != (numafeidx_).try_into().unwrap() {
        return Result::Err("put_djc: Argument 'b' has the wrong length, expected numafeidx_".to_string());
      }
      let numterms_ : i64 = termsizelist_.len() as i64;
      self.handle_res(unsafe { MSK_putdjc(self.ptr,djcidx_,numdomidx_,domidxlist_.as_ptr(),numafeidx_,afeidxlist_.as_ptr(),b_.as_ptr(),numterms_,termsizelist_.as_ptr()) },"put_djc")?;
      return Result::Ok(());
    } // putdjc
    /// Sets the name of a disjunctive constraint.
    ///
    /// # Arguments
    ///
    /// - `djcidx_` Index of the disjunctive constraint.
    /// - `name_` The name of the disjunctive constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putdjcname>
    #[allow(unused_parens)]
    pub fn put_djc_name(&mut self,djcidx_ : i64,name_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(name_).unwrap();
      self.handle_res(unsafe { MSK_putdjcname(self.ptr,djcidx_,__tmp_1.as_ptr()) },"put_djc_name")?;
      return Result::Ok(());
    } // putdjcname
    /// Inputs a slice of disjunctive constraints.
    ///
    /// # Arguments
    ///
    /// - `idxfirst_` Index of the first disjunctive constraint in the slice.
    /// - `idxlast_` Index of the last disjunctive constraint in the slice plus 1.
    /// - `domidxlist_` List of domain indexes.
    /// - `afeidxlist_` List of affine expression indexes.
    /// - `b_` The vector of constant terms added to affine expressions. Optional, may be NULL.
    /// - `termsizelist_` List of term sizes.
    /// - `termsindjc_` Number of terms in each of the disjunctive constraints in the slice.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putdjcslice>
    #[allow(unused_parens)]
    pub fn put_djc_slice(&mut self,idxfirst_ : i64,idxlast_ : i64,domidxlist_ : &[i64],afeidxlist_ : &[i64],b_ : &[f64],termsizelist_ : &[i64],termsindjc_ : &[i64]) -> Result<(),String> {
      let numdomidx_ : i64 = domidxlist_.len() as i64;
      let numafeidx_ : i64 = afeidxlist_.len() as i64;
      if b_.len() != (numafeidx_).try_into().unwrap() {
        return Result::Err("put_djc_slice: Argument 'b' has the wrong length, expected numafeidx_".to_string());
      }
      let numterms_ : i64 = termsizelist_.len() as i64;
      if termsindjc_.len() != ((idxlast_-idxfirst_)).try_into().unwrap() {
        return Result::Err("put_djc_slice: Argument 'termsindjc' has the wrong length, expected (idxlast_-idxfirst_)".to_string());
      }
      self.handle_res(unsafe { MSK_putdjcslice(self.ptr,idxfirst_,idxlast_,numdomidx_,domidxlist_.as_ptr(),numafeidx_,afeidxlist_.as_ptr(),b_.as_ptr(),numterms_,termsizelist_.as_ptr(),termsindjc_.as_ptr()) },"put_djc_slice")?;
      return Result::Ok(());
    } // putdjcslice
    /// Sets the name of a domain.
    ///
    /// # Arguments
    ///
    /// - `domidx_` Index of the domain.
    /// - `name_` The name of the domain.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putdomainname>
    #[allow(unused_parens)]
    pub fn put_domain_name(&mut self,domidx_ : i64,name_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(name_).unwrap();
      self.handle_res(unsafe { MSK_putdomainname(self.ptr,domidx_,__tmp_1.as_ptr()) },"put_domain_name")?;
      return Result::Ok(());
    } // putdomainname
    /// Sets a double parameter.
    ///
    /// # Arguments
    ///
    /// - `param_` Which parameter.
    ///   
    ///   See [Dparam]
    /// - `parvalue_` Parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putdouparam>
    #[allow(unused_parens)]
    pub fn put_dou_param(&mut self,param_ : i32,parvalue_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putdouparam(self.ptr,param_,parvalue_) },"put_dou_param")?;
      return Result::Ok(());
    } // putdouparam
    /// Sets an integer parameter.
    ///
    /// # Arguments
    ///
    /// - `param_` Which parameter.
    ///   
    ///   See [Iparam]
    /// - `parvalue_` Parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putintparam>
    #[allow(unused_parens)]
    pub fn put_int_param(&mut self,param_ : i32,parvalue_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putintparam(self.ptr,param_,parvalue_) },"put_int_param")?;
      return Result::Ok(());
    } // putintparam
    /// Sets the number of preallocated affine conic constraints.
    ///
    /// # Arguments
    ///
    /// - `maxnumacc_` Number of preallocated affine conic constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putmaxnumacc>
    #[allow(unused_parens)]
    pub fn put_max_num_acc(&mut self,maxnumacc_ : i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putmaxnumacc(self.ptr,maxnumacc_) },"put_max_num_acc")?;
      return Result::Ok(());
    } // putmaxnumacc
    /// Sets the number of preallocated affine expressions in the optimization task.
    ///
    /// # Arguments
    ///
    /// - `maxnumafe_` Number of preallocated affine expressions.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putmaxnumafe>
    #[allow(unused_parens)]
    pub fn put_max_num_afe(&mut self,maxnumafe_ : i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putmaxnumafe(self.ptr,maxnumafe_) },"put_max_num_afe")?;
      return Result::Ok(());
    } // putmaxnumafe
    /// Sets the number of preallocated non-zero entries in the linear coefficient matrix.
    ///
    /// # Arguments
    ///
    /// - `maxnumanz_` New size of the storage reserved for storing the linear coefficient matrix.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putmaxnumanz>
    #[allow(unused_parens)]
    pub fn put_max_num_a_nz(&mut self,maxnumanz_ : i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putmaxnumanz(self.ptr,maxnumanz_) },"put_max_num_a_nz")?;
      return Result::Ok(());
    } // putmaxnumanz
    /// Sets the number of preallocated symmetric matrix variables.
    ///
    /// # Arguments
    ///
    /// - `maxnumbarvar_` Number of preallocated symmetric matrix variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putmaxnumbarvar>
    #[allow(unused_parens)]
    pub fn put_max_num_barvar(&mut self,maxnumbarvar_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putmaxnumbarvar(self.ptr,maxnumbarvar_) },"put_max_num_barvar")?;
      return Result::Ok(());
    } // putmaxnumbarvar
    /// Sets the number of preallocated constraints in the optimization task.
    ///
    /// # Arguments
    ///
    /// - `maxnumcon_` Number of preallocated constraints in the optimization task.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putmaxnumcon>
    #[allow(unused_parens)]
    pub fn put_max_num_con(&mut self,maxnumcon_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putmaxnumcon(self.ptr,maxnumcon_) },"put_max_num_con")?;
      return Result::Ok(());
    } // putmaxnumcon
    /// Sets the number of preallocated conic constraints in the optimization task.
    ///
    /// # Arguments
    ///
    /// - `maxnumcone_` Number of preallocated conic constraints in the optimization task.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putmaxnumcone>
    #[allow(unused_parens)]
    pub fn put_max_num_cone(&mut self,maxnumcone_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putmaxnumcone(self.ptr,maxnumcone_) },"put_max_num_cone")?;
      return Result::Ok(());
    } // putmaxnumcone
    /// Sets the number of preallocated disjunctive constraints.
    ///
    /// # Arguments
    ///
    /// - `maxnumdjc_` Number of preallocated disjunctive constraints in the task.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putmaxnumdjc>
    #[allow(unused_parens)]
    pub fn put_max_num_djc(&mut self,maxnumdjc_ : i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putmaxnumdjc(self.ptr,maxnumdjc_) },"put_max_num_djc")?;
      return Result::Ok(());
    } // putmaxnumdjc
    /// Sets the number of preallocated domains in the optimization task.
    ///
    /// # Arguments
    ///
    /// - `maxnumdomain_` Number of preallocated domains.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putmaxnumdomain>
    #[allow(unused_parens)]
    pub fn put_max_num_domain(&mut self,maxnumdomain_ : i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putmaxnumdomain(self.ptr,maxnumdomain_) },"put_max_num_domain")?;
      return Result::Ok(());
    } // putmaxnumdomain
    /// Sets the number of preallocated non-zero entries in quadratic terms.
    ///
    /// # Arguments
    ///
    /// - `maxnumqnz_` Number of non-zero elements preallocated in quadratic coefficient matrices.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putmaxnumqnz>
    #[allow(unused_parens)]
    pub fn put_max_num_q_nz(&mut self,maxnumqnz_ : i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putmaxnumqnz(self.ptr,maxnumqnz_) },"put_max_num_q_nz")?;
      return Result::Ok(());
    } // putmaxnumqnz
    /// Sets the number of preallocated variables in the optimization task.
    ///
    /// # Arguments
    ///
    /// - `maxnumvar_` Number of preallocated variables in the optimization task.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putmaxnumvar>
    #[allow(unused_parens)]
    pub fn put_max_num_var(&mut self,maxnumvar_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putmaxnumvar(self.ptr,maxnumvar_) },"put_max_num_var")?;
      return Result::Ok(());
    } // putmaxnumvar
    /// Sets a double parameter.
    ///
    /// # Arguments
    ///
    /// - `paramname_` Name of a parameter.
    /// - `parvalue_` Parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putnadouparam>
    #[allow(unused_parens)]
    pub fn put_na_dou_param(&mut self,paramname_ : &str,parvalue_ : f64) -> Result<(),String> {
      let __tmp_1 = CString::new(paramname_).unwrap();
      self.handle_res(unsafe { MSK_putnadouparam(self.ptr,__tmp_1.as_ptr(),parvalue_) },"put_na_dou_param")?;
      return Result::Ok(());
    } // putnadouparam
    /// Sets an integer parameter.
    ///
    /// # Arguments
    ///
    /// - `paramname_` Name of a parameter.
    /// - `parvalue_` Parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putnaintparam>
    #[allow(unused_parens)]
    pub fn put_na_int_param(&mut self,paramname_ : &str,parvalue_ : i32) -> Result<(),String> {
      let __tmp_1 = CString::new(paramname_).unwrap();
      self.handle_res(unsafe { MSK_putnaintparam(self.ptr,__tmp_1.as_ptr(),parvalue_) },"put_na_int_param")?;
      return Result::Ok(());
    } // putnaintparam
    /// Sets a string parameter.
    ///
    /// # Arguments
    ///
    /// - `paramname_` Name of a parameter.
    /// - `parvalue_` Parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putnastrparam>
    #[allow(unused_parens)]
    pub fn put_na_str_param(&mut self,paramname_ : &str,parvalue_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(paramname_).unwrap();
      let __tmp_3 = CString::new(parvalue_).unwrap();
      self.handle_res(unsafe { MSK_putnastrparam(self.ptr,__tmp_1.as_ptr(),__tmp_3.as_ptr()) },"put_na_str_param")?;
      return Result::Ok(());
    } // putnastrparam
    /// Assigns a new name to the objective.
    ///
    /// # Arguments
    ///
    /// - `objname_` Name of the objective.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putobjname>
    #[allow(unused_parens)]
    pub fn put_obj_name(&mut self,objname_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(objname_).unwrap();
      self.handle_res(unsafe { MSK_putobjname(self.ptr,__tmp_1.as_ptr()) },"put_obj_name")?;
      return Result::Ok(());
    } // putobjname
    /// Sets the objective sense.
    ///
    /// # Arguments
    ///
    /// - `sense_` The objective sense of the task
    ///   
    ///   See [Objsense]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putobjsense>
    #[allow(unused_parens)]
    pub fn put_obj_sense(&mut self,sense_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putobjsense(self.ptr,sense_) },"put_obj_sense")?;
      return Result::Ok(());
    } // putobjsense
    /// Specify an OptServer for remote calls.
    ///
    /// # Arguments
    ///
    /// - `host_` A URL specifying the optimization server to be used.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putoptserverhost>
    #[allow(unused_parens)]
    pub fn put_optserver_host(&mut self,host_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(host_).unwrap();
      self.handle_res(unsafe { MSK_putoptserverhost(self.ptr,__tmp_1.as_ptr()) },"put_optserver_host")?;
      return Result::Ok(());
    } // putoptserverhost
    /// Modifies the value of parameter.
    ///
    /// # Arguments
    ///
    /// - `parname_` Parameter name.
    /// - `parvalue_` Parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putparam>
    #[allow(unused_parens)]
    pub fn put_param(&mut self,parname_ : &str,parvalue_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(parname_).unwrap();
      let __tmp_3 = CString::new(parvalue_).unwrap();
      self.handle_res(unsafe { MSK_putparam(self.ptr,__tmp_1.as_ptr(),__tmp_3.as_ptr()) },"put_param")?;
      return Result::Ok(());
    } // putparam
    /// Replaces all quadratic terms in constraints.
    ///
    /// # Arguments
    ///
    /// - `qcsubk_` Constraint subscripts for quadratic coefficients.
    /// - `qcsubi_` Row subscripts for quadratic constraint matrix.
    /// - `qcsubj_` Column subscripts for quadratic constraint matrix.
    /// - `qcval_` Quadratic constraint coefficient values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putqcon>
    #[allow(unused_parens)]
    pub fn put_q_con(&mut self,qcsubk_ : &[i32],qcsubi_ : &[i32],qcsubj_ : &[i32],qcval_ : &[f64]) -> Result<(),String> {
      let numqcnz_ : i32 = std::cmp::min(std::cmp::min(qcval_.len(),qcsubi_.len()),qcsubj_.len()) as i32;
      self.handle_res(unsafe { MSK_putqcon(self.ptr,numqcnz_,qcsubk_.as_ptr(),qcsubi_.as_ptr(),qcsubj_.as_ptr(),qcval_.as_ptr()) },"put_q_con")?;
      return Result::Ok(());
    } // putqcon
    /// Replaces all quadratic terms in a single constraint.
    ///
    /// # Arguments
    ///
    /// - `k_` The constraint in which the new quadratic elements are inserted.
    /// - `qcsubi_` Row subscripts for quadratic constraint matrix.
    /// - `qcsubj_` Column subscripts for quadratic constraint matrix.
    /// - `qcval_` Quadratic constraint coefficient values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putqconk>
    #[allow(unused_parens)]
    pub fn put_q_con_k(&mut self,k_ : i32,qcsubi_ : &[i32],qcsubj_ : &[i32],qcval_ : &[f64]) -> Result<(),String> {
      let numqcnz_ : i32 = std::cmp::min(std::cmp::min(qcval_.len(),qcsubi_.len()),qcsubj_.len()) as i32;
      if qcsubi_.len() != qcsubj_.len() || qcsubi_.len() != qcval_.len() { return Err("put_q_con_k: Mismatching lengths if qcsubi, qcsubj and qcval".to_string()); }
      self.handle_res(unsafe { MSK_putqconk(self.ptr,k_,numqcnz_,qcsubi_.as_ptr(),qcsubj_.as_ptr(),qcval_.as_ptr()) },"put_q_con_k")?;
      return Result::Ok(());
    } // putqconk
    /// Replaces all quadratic terms in the objective.
    ///
    /// # Arguments
    ///
    /// - `qosubi_` Row subscripts for quadratic objective coefficients.
    /// - `qosubj_` Column subscripts for quadratic objective coefficients.
    /// - `qoval_` Quadratic objective coefficient values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putqobj>
    #[allow(unused_parens)]
    pub fn put_q_obj(&mut self,qosubi_ : &[i32],qosubj_ : &[i32],qoval_ : &[f64]) -> Result<(),String> {
      let numqonz_ : i32 = std::cmp::min(std::cmp::min(qosubj_.len(),qosubi_.len()),qoval_.len()) as i32;
      if qosubi_.len() != qosubj_.len() || qosubi_.len() != qoval_.len() { return Err("put_q_obj: Mismatching lengths if qosubi, qosubj and qoval".to_string()); }
      self.handle_res(unsafe { MSK_putqobj(self.ptr,numqonz_,qosubi_.as_ptr(),qosubj_.as_ptr(),qoval_.as_ptr()) },"put_q_obj")?;
      return Result::Ok(());
    } // putqobj
    /// Replaces one coefficient in the quadratic term in the objective.
    ///
    /// # Arguments
    ///
    /// - `i_` Row index for the coefficient to be replaced.
    /// - `j_` Column index for the coefficient to be replaced.
    /// - `qoij_` The new coefficient value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putqobjij>
    #[allow(unused_parens)]
    pub fn put_q_obj_i_j(&mut self,i_ : i32,j_ : i32,qoij_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putqobjij(self.ptr,i_,j_,qoij_) },"put_q_obj_i_j")?;
      return Result::Ok(());
    } // putqobjij
    /// Sets the status keys for the constraints.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `skc_` Status keys for the constraints.
    ///   
    ///   See [Stakey]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putskc>
    #[allow(unused_parens)]
    pub fn put_skc(&mut self,whichsol_ : i32,skc_ : &[i32]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if skc_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("put_skc: Argument 'skc' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_putskc(self.ptr,whichsol_,skc_.as_ptr()) },"put_skc")?;
      return Result::Ok(());
    } // putskc
    /// Sets the status keys for a slice of the constraints.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `skc_` Status keys for the constraints.
    ///   
    ///   See [Stakey]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putskcslice>
    #[allow(unused_parens)]
    pub fn put_skc_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,skc_ : &[i32]) -> Result<(),String> {
      if skc_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("put_skc_slice: Argument 'skc' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putskcslice(self.ptr,whichsol_,first_,last_,skc_.as_ptr()) },"put_skc_slice")?;
      return Result::Ok(());
    } // putskcslice
    /// Sets the status keys for the scalar variables.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `skx_` Status keys for the variables.
    ///   
    ///   See [Stakey]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putskx>
    #[allow(unused_parens)]
    pub fn put_skx(&mut self,whichsol_ : i32,skx_ : &[i32]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumvar")?;
      if skx_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("put_skx: Argument 'skx' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_putskx(self.ptr,whichsol_,skx_.as_ptr()) },"put_skx")?;
      return Result::Ok(());
    } // putskx
    /// Sets the status keys for a slice of the variables.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `skx_` Status keys for the variables.
    ///   
    ///   See [Stakey]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putskxslice>
    #[allow(unused_parens)]
    pub fn put_skx_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,skx_ : &[i32]) -> Result<(),String> {
      if skx_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("put_skx_slice: Argument 'skx' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putskxslice(self.ptr,whichsol_,first_,last_,skx_.as_ptr()) },"put_skx_slice")?;
      return Result::Ok(());
    } // putskxslice
    /// Sets the slc vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `slc_` Dual variables corresponding to the lower bounds on the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putslc>
    #[allow(unused_parens)]
    pub fn put_slc(&mut self,whichsol_ : i32,slc_ : &[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if slc_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("put_slc: Argument 'slc' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_putslc(self.ptr,whichsol_,slc_.as_ptr()) },"put_slc")?;
      return Result::Ok(());
    } // putslc
    /// Sets a slice of the slc vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `slc_` Dual variables corresponding to the lower bounds on the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putslcslice>
    #[allow(unused_parens)]
    pub fn put_slc_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,slc_ : &[f64]) -> Result<(),String> {
      if slc_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("put_slc_slice: Argument 'slc' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putslcslice(self.ptr,whichsol_,first_,last_,slc_.as_ptr()) },"put_slc_slice")?;
      return Result::Ok(());
    } // putslcslice
    /// Sets the slx vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `slx_` Dual variables corresponding to the lower bounds on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putslx>
    #[allow(unused_parens)]
    pub fn put_slx(&mut self,whichsol_ : i32,slx_ : &[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumvar")?;
      if slx_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("put_slx: Argument 'slx' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_putslx(self.ptr,whichsol_,slx_.as_ptr()) },"put_slx")?;
      return Result::Ok(());
    } // putslx
    /// Sets a slice of the slx vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `slx_` Dual variables corresponding to the lower bounds on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putslxslice>
    #[allow(unused_parens)]
    pub fn put_slx_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,slx_ : &[f64]) -> Result<(),String> {
      if slx_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("put_slx_slice: Argument 'slx' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putslxslice(self.ptr,whichsol_,first_,last_,slx_.as_ptr()) },"put_slx_slice")?;
      return Result::Ok(());
    } // putslxslice
    /// Sets the snx vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sux_` Dual variables corresponding to the upper bounds on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putsnx>
    #[allow(unused_parens)]
    pub fn put_snx(&mut self,whichsol_ : i32,sux_ : &[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumvar")?;
      if sux_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("put_snx: Argument 'sux' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_putsnx(self.ptr,whichsol_,sux_.as_ptr()) },"put_snx")?;
      return Result::Ok(());
    } // putsnx
    /// Sets a slice of the snx vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `snx_` Dual variables corresponding to the conic constraints on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putsnxslice>
    #[allow(unused_parens)]
    pub fn put_snx_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,snx_ : &[f64]) -> Result<(),String> {
      if snx_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("put_snx_slice: Argument 'snx' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putsnxslice(self.ptr,whichsol_,first_,last_,snx_.as_ptr()) },"put_snx_slice")?;
      return Result::Ok(());
    } // putsnxslice
    /// Inserts a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `skc_` Status keys for the constraints.
    ///   
    ///   See [Stakey]
    /// - `skx_` Status keys for the variables.
    ///   
    ///   See [Stakey]
    /// - `skn_` Status keys for the conic constraints.
    ///   
    ///   See [Stakey]
    /// - `xc_` Primal constraint solution.
    /// - `xx_` Primal variable solution.
    /// - `y_` Vector of dual variables corresponding to the constraints.
    /// - `slc_` Dual variables corresponding to the lower bounds on the constraints.
    /// - `suc_` Dual variables corresponding to the upper bounds on the constraints.
    /// - `slx_` Dual variables corresponding to the lower bounds on the variables.
    /// - `sux_` Dual variables corresponding to the upper bounds on the variables.
    /// - `snx_` Dual variables corresponding to the conic constraints on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putsolution>
    #[allow(unused_parens)]
    pub fn put_solution(&mut self,whichsol_ : i32,skc_ : &[i32],skx_ : &[i32],skn_ : &[i32],xc_ : &[f64],xx_ : &[f64],y_ : &[f64],slc_ : &[f64],suc_ : &[f64],slx_ : &[f64],sux_ : &[f64],snx_ : &[f64]) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putsolution(self.ptr,whichsol_,skc_.as_ptr(),skx_.as_ptr(),skn_.as_ptr(),xc_.as_ptr(),xx_.as_ptr(),y_.as_ptr(),slc_.as_ptr(),suc_.as_ptr(),slx_.as_ptr(),sux_.as_ptr(),snx_.as_ptr()) },"put_solution")?;
      return Result::Ok(());
    } // putsolution
    /// Inserts a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `skc_` Status keys for the constraints.
    ///   
    ///   See [Stakey]
    /// - `skx_` Status keys for the variables.
    ///   
    ///   See [Stakey]
    /// - `skn_` Status keys for the conic constraints.
    ///   
    ///   See [Stakey]
    /// - `xc_` Primal constraint solution.
    /// - `xx_` Primal variable solution.
    /// - `y_` Vector of dual variables corresponding to the constraints.
    /// - `slc_` Dual variables corresponding to the lower bounds on the constraints.
    /// - `suc_` Dual variables corresponding to the upper bounds on the constraints.
    /// - `slx_` Dual variables corresponding to the lower bounds on the variables.
    /// - `sux_` Dual variables corresponding to the upper bounds on the variables.
    /// - `snx_` Dual variables corresponding to the conic constraints on the variables.
    /// - `doty_` Dual variables corresponding to affine conic constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putsolutionnew>
    #[allow(unused_parens)]
    pub fn put_solution_new(&mut self,whichsol_ : i32,skc_ : &[i32],skx_ : &[i32],skn_ : &[i32],xc_ : &[f64],xx_ : &[f64],y_ : &[f64],slc_ : &[f64],suc_ : &[f64],slx_ : &[f64],sux_ : &[f64],snx_ : &[f64],doty_ : &[f64]) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putsolutionnew(self.ptr,whichsol_,skc_.as_ptr(),skx_.as_ptr(),skn_.as_ptr(),xc_.as_ptr(),xx_.as_ptr(),y_.as_ptr(),slc_.as_ptr(),suc_.as_ptr(),slx_.as_ptr(),sux_.as_ptr(),snx_.as_ptr(),doty_.as_ptr()) },"put_solution_new")?;
      return Result::Ok(());
    } // putsolutionnew
    /// Inputs the dual variable of a solution.
    ///
    /// # Arguments
    ///
    /// - `i_` Index of the dual variable.
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `y_` Solution value of the dual variable.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putsolutionyi>
    #[allow(unused_parens)]
    pub fn put_solution_y_i(&mut self,i_ : i32,whichsol_ : i32,y_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putsolutionyi(self.ptr,i_,whichsol_,y_) },"put_solution_y_i")?;
      return Result::Ok(());
    } // putsolutionyi
    /// Sets a string parameter.
    ///
    /// # Arguments
    ///
    /// - `param_` Which parameter.
    ///   
    ///   See [Sparam]
    /// - `parvalue_` Parameter value.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putstrparam>
    #[allow(unused_parens)]
    pub fn put_str_param(&mut self,param_ : i32,parvalue_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(parvalue_).unwrap();
      self.handle_res(unsafe { MSK_putstrparam(self.ptr,param_,__tmp_1.as_ptr()) },"put_str_param")?;
      return Result::Ok(());
    } // putstrparam
    /// Sets the suc vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `suc_` Dual variables corresponding to the upper bounds on the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putsuc>
    #[allow(unused_parens)]
    pub fn put_suc(&mut self,whichsol_ : i32,suc_ : &[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if suc_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("put_suc: Argument 'suc' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_putsuc(self.ptr,whichsol_,suc_.as_ptr()) },"put_suc")?;
      return Result::Ok(());
    } // putsuc
    /// Sets a slice of the suc vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `suc_` Dual variables corresponding to the upper bounds on the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putsucslice>
    #[allow(unused_parens)]
    pub fn put_suc_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,suc_ : &[f64]) -> Result<(),String> {
      if suc_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("put_suc_slice: Argument 'suc' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putsucslice(self.ptr,whichsol_,first_,last_,suc_.as_ptr()) },"put_suc_slice")?;
      return Result::Ok(());
    } // putsucslice
    /// Sets the sux vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sux_` Dual variables corresponding to the upper bounds on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putsux>
    #[allow(unused_parens)]
    pub fn put_sux(&mut self,whichsol_ : i32,sux_ : &[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumvar")?;
      if sux_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("put_sux: Argument 'sux' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_putsux(self.ptr,whichsol_,sux_.as_ptr()) },"put_sux")?;
      return Result::Ok(());
    } // putsux
    /// Sets a slice of the sux vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `sux_` Dual variables corresponding to the upper bounds on the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putsuxslice>
    #[allow(unused_parens)]
    pub fn put_sux_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,sux_ : &[f64]) -> Result<(),String> {
      if sux_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("put_sux_slice: Argument 'sux' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putsuxslice(self.ptr,whichsol_,first_,last_,sux_.as_ptr()) },"put_sux_slice")?;
      return Result::Ok(());
    } // putsuxslice
    /// Assigns a new name to the task.
    ///
    /// # Arguments
    ///
    /// - `taskname_` Name assigned to the task.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.puttaskname>
    #[allow(unused_parens)]
    pub fn put_task_name(&mut self,taskname_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(taskname_).unwrap();
      self.handle_res(unsafe { MSK_puttaskname(self.ptr,__tmp_1.as_ptr()) },"put_task_name")?;
      return Result::Ok(());
    } // puttaskname
    /// Changes the bounds for one variable.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the variable.
    /// - `bkx_` New bound key.
    ///   
    ///   See [Boundkey]
    /// - `blx_` New lower bound.
    /// - `bux_` New upper bound.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putvarbound>
    #[allow(unused_parens)]
    pub fn put_var_bound(&mut self,j_ : i32,bkx_ : i32,blx_ : f64,bux_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putvarbound(self.ptr,j_,bkx_,blx_,bux_) },"put_var_bound")?;
      return Result::Ok(());
    } // putvarbound
    /// Changes the bounds of a list of variables.
    ///
    /// # Arguments
    ///
    /// - `sub_` List of variable indexes.
    /// - `bkx_` Bound keys for the variables.
    ///   
    ///   See [Boundkey]
    /// - `blx_` Lower bounds for the variables.
    /// - `bux_` Upper bounds for the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putvarboundlist>
    #[allow(unused_parens)]
    pub fn put_var_bound_list(&mut self,sub_ : &[i32],bkx_ : &[i32],blx_ : &[f64],bux_ : &[f64]) -> Result<(),String> {
      let num_ : i32 = std::cmp::min(std::cmp::min(std::cmp::min(bux_.len(),blx_.len()),sub_.len()),bkx_.len()) as i32;
      self.handle_res(unsafe { MSK_putvarboundlist(self.ptr,num_,sub_.as_ptr(),bkx_.as_ptr(),blx_.as_ptr(),bux_.as_ptr()) },"put_var_bound_list")?;
      return Result::Ok(());
    } // putvarboundlist
    /// Changes the bounds of a list of variables.
    ///
    /// # Arguments
    ///
    /// - `sub_` List of variable indexes.
    /// - `bkx_` New bound key for all variables in the list.
    ///   
    ///   See [Boundkey]
    /// - `blx_` New lower bound for all variables in the list.
    /// - `bux_` New upper bound for all variables in the list.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putvarboundlistconst>
    #[allow(unused_parens)]
    pub fn put_var_bound_list_const(&mut self,sub_ : &[i32],bkx_ : i32,blx_ : f64,bux_ : f64) -> Result<(),String> {
      let num_ : i32 = sub_.len() as i32;
      self.handle_res(unsafe { MSK_putvarboundlistconst(self.ptr,num_,sub_.as_ptr(),bkx_,blx_,bux_) },"put_var_bound_list_const")?;
      return Result::Ok(());
    } // putvarboundlistconst
    /// Changes the bounds for a slice of the variables.
    ///
    /// # Arguments
    ///
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `bkx_` Bound keys for the variables.
    ///   
    ///   See [Boundkey]
    /// - `blx_` Lower bounds for the variables.
    /// - `bux_` Upper bounds for the variables.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putvarboundslice>
    #[allow(unused_parens)]
    pub fn put_var_bound_slice(&mut self,first_ : i32,last_ : i32,bkx_ : &[i32],blx_ : &[f64],bux_ : &[f64]) -> Result<(),String> {
      if bkx_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("put_var_bound_slice: Argument 'bkx' has the wrong length, expected (last_-first_)".to_string());
      }
      if blx_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("put_var_bound_slice: Argument 'blx' has the wrong length, expected (last_-first_)".to_string());
      }
      if bux_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("put_var_bound_slice: Argument 'bux' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putvarboundslice(self.ptr,first_,last_,bkx_.as_ptr(),blx_.as_ptr(),bux_.as_ptr()) },"put_var_bound_slice")?;
      return Result::Ok(());
    } // putvarboundslice
    /// Changes the bounds for a slice of the variables.
    ///
    /// # Arguments
    ///
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `bkx_` New bound key for all variables in the slice.
    ///   
    ///   See [Boundkey]
    /// - `blx_` New lower bound for all variables in the slice.
    /// - `bux_` New upper bound for all variables in the slice.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putvarboundsliceconst>
    #[allow(unused_parens)]
    pub fn put_var_bound_slice_const(&mut self,first_ : i32,last_ : i32,bkx_ : i32,blx_ : f64,bux_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putvarboundsliceconst(self.ptr,first_,last_,bkx_,blx_,bux_) },"put_var_bound_slice_const")?;
      return Result::Ok(());
    } // putvarboundsliceconst
    /// Sets the name of a variable.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the variable.
    /// - `name_` The variable name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putvarname>
    #[allow(unused_parens)]
    pub fn put_var_name(&mut self,j_ : i32,name_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(name_).unwrap();
      self.handle_res(unsafe { MSK_putvarname(self.ptr,j_,__tmp_1.as_ptr()) },"put_var_name")?;
      return Result::Ok(());
    } // putvarname
    /// Sets the primal and dual solution information for a single variable.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the variable.
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `sk_` Status key of the variable.
    ///   
    ///   See [Stakey]
    /// - `x_` Primal solution value of the variable.
    /// - `sl_` Solution value of the dual variable associated with the lower bound.
    /// - `su_` Solution value of the dual variable associated with the upper bound.
    /// - `sn_` Solution value of the dual variable associated with the conic constraint.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putvarsolutionj>
    #[allow(unused_parens)]
    pub fn put_var_solution_j(&mut self,j_ : i32,whichsol_ : i32,sk_ : i32,x_ : f64,sl_ : f64,su_ : f64,sn_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putvarsolutionj(self.ptr,j_,whichsol_,sk_,x_,sl_,su_,sn_) },"put_var_solution_j")?;
      return Result::Ok(());
    } // putvarsolutionj
    /// Sets the variable type of one variable.
    ///
    /// # Arguments
    ///
    /// - `j_` Index of the variable.
    /// - `vartype_` The new variable type.
    ///   
    ///   See [Variabletype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putvartype>
    #[allow(unused_parens)]
    pub fn put_var_type(&mut self,j_ : i32,vartype_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putvartype(self.ptr,j_,vartype_) },"put_var_type")?;
      return Result::Ok(());
    } // putvartype
    /// Sets the variable type for one or more variables.
    ///
    /// # Arguments
    ///
    /// - `subj_` A list of variable indexes for which the variable type should be changed.
    /// - `vartype_` A list of variable types.
    ///   
    ///   See [Variabletype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putvartypelist>
    #[allow(unused_parens)]
    pub fn put_var_type_list(&mut self,subj_ : &[i32],vartype_ : &[i32]) -> Result<(),String> {
      let num_ : i32 = std::cmp::min(vartype_.len(),subj_.len()) as i32;
      self.handle_res(unsafe { MSK_putvartypelist(self.ptr,num_,subj_.as_ptr(),vartype_.as_ptr()) },"put_var_type_list")?;
      return Result::Ok(());
    } // putvartypelist
    /// Sets the xc vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `xc_` Primal constraint solution.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putxc>
    #[allow(unused_parens)]
    pub fn put_xc(&mut self,whichsol_ : i32,xc_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if xc_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("put_xc: Argument 'xc' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_putxc(self.ptr,whichsol_,xc_.as_mut_ptr()) },"put_xc")?;
      return Result::Ok(());
    } // putxc
    /// Sets a slice of the xc vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `xc_` Primal constraint solution.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putxcslice>
    #[allow(unused_parens)]
    pub fn put_xc_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,xc_ : &[f64]) -> Result<(),String> {
      if xc_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("put_xc_slice: Argument 'xc' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putxcslice(self.ptr,whichsol_,first_,last_,xc_.as_ptr()) },"put_xc_slice")?;
      return Result::Ok(());
    } // putxcslice
    /// Sets the xx vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `xx_` Primal variable solution.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putxx>
    #[allow(unused_parens)]
    pub fn put_xx(&mut self,whichsol_ : i32,xx_ : &[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumvar")?;
      if xx_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("put_xx: Argument 'xx' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_putxx(self.ptr,whichsol_,xx_.as_ptr()) },"put_xx")?;
      return Result::Ok(());
    } // putxx
    /// Sets a slice of the xx vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `xx_` Primal variable solution.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putxxslice>
    #[allow(unused_parens)]
    pub fn put_xx_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,xx_ : &[f64]) -> Result<(),String> {
      if xx_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("put_xx_slice: Argument 'xx' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putxxslice(self.ptr,whichsol_,first_,last_,xx_.as_ptr()) },"put_xx_slice")?;
      return Result::Ok(());
    } // putxxslice
    /// Sets the y vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `y_` Vector of dual variables corresponding to the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.puty>
    #[allow(unused_parens)]
    pub fn put_y(&mut self,whichsol_ : i32,y_ : &[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if y_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("put_y: Argument 'y' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_puty(self.ptr,whichsol_,y_.as_ptr()) },"put_y")?;
      return Result::Ok(());
    } // puty
    /// Sets a slice of the y vector for a solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `first_` First index in the sequence.
    /// - `last_` Last index plus 1 in the sequence.
    /// - `y_` Vector of dual variables corresponding to the constraints.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.putyslice>
    #[allow(unused_parens)]
    pub fn put_y_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,y_ : &[f64]) -> Result<(),String> {
      if y_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("put_y_slice: Argument 'y' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putyslice(self.ptr,whichsol_,first_,last_,y_.as_ptr()) },"put_y_slice")?;
      return Result::Ok(());
    } // putyslice
    /// Read a binary dump of the task solution and information items.
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    /// - `compress_` Data compression type.
    ///   
    ///   See [Compresstype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readbsolution>
    #[allow(unused_parens)]
    pub fn read_b_solution(&self,filename_ : &str,compress_ : i32) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_readbsolution(self.ptr,__tmp_1.as_ptr(),compress_) },"read_b_solution")?;
      return Result::Ok(());
    } // readbsolution
    /// Reads problem data from a file.
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readdataautoformat>
    #[allow(unused_parens)]
    pub fn read_data(&mut self,filename_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_readdataautoformat(self.ptr,__tmp_1.as_ptr()) },"read_data")?;
      return Result::Ok(());
    } // readdataautoformat
    /// Reads problem data from a file.
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    /// - `format_` File data format.
    ///   
    ///   See [Dataformat]
    /// - `compress_` File compression type.
    ///   
    ///   See [Compresstype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readdataformat>
    #[allow(unused_parens)]
    pub fn read_data_format(&mut self,filename_ : &str,format_ : i32,compress_ : i32) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_readdataformat(self.ptr,__tmp_1.as_ptr(),format_,compress_) },"read_data_format")?;
      return Result::Ok(());
    } // readdataformat
    /// Reads a solution from a JSOL file.
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readjsonsol>
    #[allow(unused_parens)]
    pub fn read_json_sol(&mut self,filename_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_readjsonsol(self.ptr,__tmp_1.as_ptr()) },"read_json_sol")?;
      return Result::Ok(());
    } // readjsonsol
    /// Load task data from a string in JSON format.
    ///
    /// # Arguments
    ///
    /// - `data_` Problem data in text format.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readjsonstring>
    #[allow(unused_parens)]
    pub fn read_json_string(&mut self,data_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(data_).unwrap();
      self.handle_res(unsafe { MSK_readjsonstring(self.ptr,__tmp_1.as_ptr()) },"read_json_string")?;
      return Result::Ok(());
    } // readjsonstring
    /// Load task data from a string in LP format.
    ///
    /// # Arguments
    ///
    /// - `data_` Problem data in text format.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readlpstring>
    #[allow(unused_parens)]
    pub fn read_lp_string(&mut self,data_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(data_).unwrap();
      self.handle_res(unsafe { MSK_readlpstring(self.ptr,__tmp_1.as_ptr()) },"read_lp_string")?;
      return Result::Ok(());
    } // readlpstring
    /// Load task data from a string in OPF format.
    ///
    /// # Arguments
    ///
    /// - `data_` Problem data in text format.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readopfstring>
    #[allow(unused_parens)]
    pub fn read_opf_string(&mut self,data_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(data_).unwrap();
      self.handle_res(unsafe { MSK_readopfstring(self.ptr,__tmp_1.as_ptr()) },"read_opf_string")?;
      return Result::Ok(());
    } // readopfstring
    /// Reads a parameter file.
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readparamfile>
    #[allow(unused_parens)]
    pub fn read_param_file(&mut self,filename_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_readparamfile(self.ptr,__tmp_1.as_ptr()) },"read_param_file")?;
      return Result::Ok(());
    } // readparamfile
    /// Load task data from a string in PTF format.
    ///
    /// # Arguments
    ///
    /// - `data_` Problem data in text format.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readptfstring>
    #[allow(unused_parens)]
    pub fn read_ptf_string(&mut self,data_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(data_).unwrap();
      self.handle_res(unsafe { MSK_readptfstring(self.ptr,__tmp_1.as_ptr()) },"read_ptf_string")?;
      return Result::Ok(());
    } // readptfstring
    /// Reads a solution from a file.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readsolution>
    #[allow(unused_parens)]
    pub fn read_solution(&mut self,whichsol_ : i32,filename_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_readsolution(self.ptr,whichsol_,__tmp_1.as_ptr()) },"read_solution")?;
      return Result::Ok(());
    } // readsolution
    /// Read solution file in format determined by the filename
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readsolutionfile>
    #[allow(unused_parens)]
    pub fn read_solution_file(&self,filename_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_readsolutionfile(self.ptr,__tmp_1.as_ptr()) },"read_solution_file")?;
      return Result::Ok(());
    } // readsolutionfile
    /// Prints information about last file read.
    ///
    /// # Arguments
    ///
    /// - `whichstream_` Index of the stream.
    ///   
    ///   See [Streamtype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readsummary>
    #[allow(unused_parens)]
    pub fn read_summary(&mut self,whichstream_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_readsummary(self.ptr,whichstream_) },"read_summary")?;
      return Result::Ok(());
    } // readsummary
    /// Load task data from a file.
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.readtask>
    #[allow(unused_parens)]
    pub fn read_task(&mut self,filename_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_readtask(self.ptr,__tmp_1.as_ptr()) },"read_task")?;
      return Result::Ok(());
    } // readtask
    /// Removes a number of symmetric matrices.
    ///
    /// # Arguments
    ///
    /// - `subset_` Indexes of symmetric matrices which should be removed.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.removebarvars>
    #[allow(unused_parens)]
    pub fn remove_barvars(&mut self,subset_ : &[i32]) -> Result<(),String> {
      let num_ : i32 = subset_.len() as i32;
      self.handle_res(unsafe { MSK_removebarvars(self.ptr,num_,subset_.as_ptr()) },"remove_barvars")?;
      return Result::Ok(());
    } // removebarvars
    /// Removes a number of conic constraints from the problem.
    ///
    /// # Arguments
    ///
    /// - `subset_` Indexes of cones which should be removed.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.removecones>
    #[allow(unused_parens)]
    pub fn remove_cones(&mut self,subset_ : &[i32]) -> Result<(),String> {
      let num_ : i32 = subset_.len() as i32;
      self.handle_res(unsafe { MSK_removecones(self.ptr,num_,subset_.as_ptr()) },"remove_cones")?;
      return Result::Ok(());
    } // removecones
    /// Removes a number of constraints.
    ///
    /// # Arguments
    ///
    /// - `subset_` Indexes of constraints which should be removed.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.removecons>
    #[allow(unused_parens)]
    pub fn remove_cons(&mut self,subset_ : &[i32]) -> Result<(),String> {
      let num_ : i32 = subset_.len() as i32;
      self.handle_res(unsafe { MSK_removecons(self.ptr,num_,subset_.as_ptr()) },"remove_cons")?;
      return Result::Ok(());
    } // removecons
    /// Removes a number of variables.
    ///
    /// # Arguments
    ///
    /// - `subset_` Indexes of variables which should be removed.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.removevars>
    #[allow(unused_parens)]
    pub fn remove_vars(&mut self,subset_ : &[i32]) -> Result<(),String> {
      let num_ : i32 = subset_.len() as i32;
      self.handle_res(unsafe { MSK_removevars(self.ptr,num_,subset_.as_ptr()) },"remove_vars")?;
      return Result::Ok(());
    } // removevars
    /// Resizes an optimization task.
    ///
    /// # Arguments
    ///
    /// - `maxnumcon_` New maximum number of constraints.
    /// - `maxnumvar_` New maximum number of variables.
    /// - `maxnumcone_` New maximum number of cones.
    /// - `maxnumanz_` New maximum number of linear non-zero elements.
    /// - `maxnumqnz_` New maximum number of quadratic non-zeros elements.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.resizetask>
    #[allow(unused_parens)]
    pub fn resize_task(&mut self,maxnumcon_ : i32,maxnumvar_ : i32,maxnumcone_ : i32,maxnumanz_ : i64,maxnumqnz_ : i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_resizetask(self.ptr,maxnumcon_,maxnumvar_,maxnumcone_,maxnumanz_,maxnumqnz_) },"resize_task")?;
      return Result::Ok(());
    } // resizetask
    /// Creates a sensitivity report.
    ///
    /// # Arguments
    ///
    /// - `whichstream_` Index of the stream.
    ///   
    ///   See [Streamtype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.sensitivityreport>
    #[allow(unused_parens)]
    pub fn sensitivity_report(&self,whichstream_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_sensitivityreport(self.ptr,whichstream_) },"sensitivity_report")?;
      return Result::Ok(());
    } // sensitivityreport
    /// Resets all parameter values.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.setdefaults>
    #[allow(unused_parens)]
    pub fn set_defaults(&mut self) -> Result<(),String> {
      self.handle_res(unsafe { MSK_setdefaults(self.ptr) },"set_defaults")?;
      return Result::Ok(());
    } // setdefaults
    /// Checks whether a solution is defined.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    ///
    /// # Returns
    ///
    ///   - `isdef` Is non-zero if the requested solution is defined.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.solutiondef>
    #[allow(unused_parens)]
    pub fn solution_def(&self,whichsol_ : i32) -> Result<bool,String> {
      let mut __tmp_0 : i32 = 0;
      self.handle_res(unsafe { MSK_solutiondef(self.ptr,whichsol_,&mut __tmp_0) },"solution_def")?;
      return Result::Ok(__tmp_0 != 0);
    } // solutiondef
    /// Prints a short summary of the current solutions.
    ///
    /// # Arguments
    ///
    /// - `whichstream_` Index of the stream.
    ///   
    ///   See [Streamtype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.solutionsummary>
    #[allow(unused_parens)]
    pub fn solution_summary(&self,whichstream_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_solutionsummary(self.ptr,whichstream_) },"solution_summary")?;
      return Result::Ok(());
    } // solutionsummary
    /// Solve a linear equation system involving a basis matrix.
    ///
    /// # Arguments
    ///
    /// - `transp_` Controls which problem formulation is solved.
    /// - `numnz_` Input (number of non-zeros in right-hand side).
    /// - `sub_` Input (indexes of non-zeros in right-hand side) and output (indexes of non-zeros in solution vector).
    /// - `val_` Input (right-hand side values) and output (solution vector values).
    ///
    /// # Returns
    ///
    ///   - `numnzout` Output (number of non-zeros in solution vector).
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.solvewithbasis>
    #[allow(unused_parens)]
    pub fn solve_with_basis(&mut self,transp_ : bool,numnz_ : i32,sub_ : &mut[i32],val_ : &mut[f64]) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if sub_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("solve_with_basis: Argument 'sub' has the wrong length, expected __tmp_0".to_string());
      }
      let mut __tmp_2 : i32 = i32::default();
      let __tmp_3 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_2) };let _ = self.handle_res(__tmp_3,"getnumcon")?;
      if val_.len() != (__tmp_2).try_into().unwrap() {
        return Result::Err("solve_with_basis: Argument 'val' has the wrong length, expected __tmp_2".to_string());
      }
      let mut __tmp_4 : i32 = i32::default();
      self.handle_res(unsafe { MSK_solvewithbasis(self.ptr,transp_,numnz_,sub_.as_mut_ptr(),val_.as_mut_ptr(),&mut __tmp_4) },"solve_with_basis")?;
      return Result::Ok(__tmp_4);
    } // solvewithbasis
    /// Obtains a cone type code.
    ///
    /// # Arguments
    ///
    /// - `str_` String corresponding to the cone type code.
    /// - `conetype_` The cone type corresponding to str.
    ///   
    ///   See [Conetype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.strtoconetype>
    #[allow(unused_parens)]
    pub fn str_to_cone_type(&self,str_ : &str,conetype_ : & mut i32) -> Result<(),String> {
      let __tmp_1 = CString::new(str_).unwrap();
      self.handle_res(unsafe { MSK_strtoconetype(self.ptr,__tmp_1.as_ptr(),conetype_) },"str_to_cone_type")?;
      return Result::Ok(());
    } // strtoconetype
    /// Obtains a status key.
    ///
    /// # Arguments
    ///
    /// - `str_` A status key abbreviation string.
    /// - `sk_` Status key corresponding to the string.
    ///   
    ///   See [Stakey]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.strtosk>
    #[allow(unused_parens)]
    pub fn str_to_sk(&self,str_ : &str,sk_ : & mut i32) -> Result<(),String> {
      let __tmp_1 = CString::new(str_).unwrap();
      self.handle_res(unsafe { MSK_strtosk(self.ptr,__tmp_1.as_ptr(),sk_) },"str_to_sk")?;
      return Result::Ok(());
    } // strtosk
    /// In-place reformulation of a QCQO to a conic quadratic problem.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.toconic>
    #[allow(unused_parens)]
    pub fn toconic(&mut self) -> Result<(),String> {
      self.handle_res(unsafe { MSK_toconic(self.ptr) },"toconic")?;
      return Result::Ok(());
    } // toconic
    /// Disconnects a user-defined function from a task stream.
    ///
    /// # Arguments
    ///
    /// - `whichstream_` Index of the stream.
    ///   
    ///   See [Streamtype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.unlinkfuncfromtaskstream>
    #[allow(unused_parens)]
    pub fn unlink_func_from_stream(&mut self,whichstream_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_unlinkfuncfromtaskstream(self.ptr,whichstream_) },"unlink_func_from_stream")?;
      return Result::Ok(());
    } // unlinkfuncfromtaskstream
    /// Update the information items related to the solution.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.updatesolutioninfo>
    #[allow(unused_parens)]
    pub fn update_solution_info(&mut self,whichsol_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_updatesolutioninfo(self.ptr,whichsol_) },"update_solution_info")?;
      return Result::Ok(());
    } // updatesolutioninfo
    /// Checks a parameter name.
    ///
    /// # Arguments
    ///
    /// - `parname_` Parameter name.
    /// - `partype_` Parameter type.
    ///   
    ///   See [Parametertype]
    /// - `param_` Which parameter.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.whichparam>
    #[allow(unused_parens)]
    pub fn which_param(&self,parname_ : &str,partype_ : & mut i32,param_ : &mut i32) -> Result<(),String> {
      let __tmp_1 = CString::new(parname_).unwrap();
      self.handle_res(unsafe { MSK_whichparam(self.ptr,__tmp_1.as_ptr(),partype_,param_) },"which_param")?;
      return Result::Ok(());
    } // whichparam
    /// Write a binary dump of the task solution and information items.
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    /// - `compress_` Data compression type.
    ///   
    ///   See [Compresstype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.writebsolution>
    #[allow(unused_parens)]
    pub fn write_b_solution(&self,filename_ : &str,compress_ : i32) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_writebsolution(self.ptr,__tmp_1.as_ptr(),compress_) },"write_b_solution")?;
      return Result::Ok(());
    } // writebsolution
    /// Writes problem data to a file.
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.writedata>
    #[allow(unused_parens)]
    pub fn write_data(&self,filename_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_writedata(self.ptr,__tmp_1.as_ptr()) },"write_data")?;
      return Result::Ok(());
    } // writedata
    /// Writes a solution to a JSON file.
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.writejsonsol>
    #[allow(unused_parens)]
    pub fn write_json_sol(&self,filename_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_writejsonsol(self.ptr,__tmp_1.as_ptr()) },"write_json_sol")?;
      return Result::Ok(());
    } // writejsonsol
    /// Writes all the parameters to a parameter file.
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.writeparamfile>
    #[allow(unused_parens)]
    pub fn write_param_file(&self,filename_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_writeparamfile(self.ptr,__tmp_1.as_ptr()) },"write_param_file")?;
      return Result::Ok(());
    } // writeparamfile
    /// Write a solution to a file.
    ///
    /// # Arguments
    ///
    /// - `whichsol_` Selects a solution.
    ///   
    ///   See [Soltype]
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.writesolution>
    #[allow(unused_parens)]
    pub fn write_solution(&self,whichsol_ : i32,filename_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_writesolution(self.ptr,whichsol_,__tmp_1.as_ptr()) },"write_solution")?;
      return Result::Ok(());
    } // writesolution
    /// Write solution file in format determined by the filename
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.writesolutionfile>
    #[allow(unused_parens)]
    pub fn write_solution_file(&self,filename_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_writesolutionfile(self.ptr,__tmp_1.as_ptr()) },"write_solution_file")?;
      return Result::Ok(());
    } // writesolutionfile
    /// Appends a record to the statistics file.
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.writestat>
    #[allow(unused_parens)]
    pub fn write_stat(&mut self,filename_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_writestat(self.ptr,__tmp_1.as_ptr()) },"write_stat")?;
      return Result::Ok(());
    } // writestat
    /// Write a complete binary dump of the task data.
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.writetask>
    #[allow(unused_parens)]
    pub fn write_task(&self,filename_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_writetask(self.ptr,__tmp_1.as_ptr()) },"write_task")?;
      return Result::Ok(());
    } // writetask
    /// Internal
    ///
    /// # Arguments
    ///
    /// - `filename_` A valid file name.
    /// - `compress_` Data compression type.
    ///   
    ///   See [Compresstype]
    ///
    /// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.writetasksolverresult_file>
    #[allow(unused_parens)]
    pub fn write_task_solver_result_file(&self,filename_ : &str,compress_ : i32) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_writetasksolverresult_file(self.ptr,__tmp_1.as_ptr(),compress_) },"write_task_solver_result_file")?;
      return Result::Ok(());
    } // writetasksolverresult_file

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


/// Computes vector addition and multiplication by a scalar.
///
/// # Arguments
///
/// - `n_` Length of the vectors.
/// - `alpha_` The scalar that multiplies x.
/// - `x_` The x vector.
/// - `y_` The y vector.
///
/// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.axpy>
pub fn prototypepfx::axpy(n_ : i32,alpha_ : f64,x_ : &[f64],y_ : &mut[f64]) -> Result<(),String> { MSK_GLOBAL_ENV.axpy(n_,alpha_,x_,y_) }
/// Compares a version of the MOSEK DLL with a specified version.
///
/// # Arguments
///
/// - `major_` Major version number.
/// - `minor_` Minor version number.
/// - `revision_` Revision number.
///
/// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.checkversion>
pub fn prototypepfx::check_version(major_ : i32,minor_ : i32,revision_ : i32) -> Result<(),String> { MSK_GLOBAL_ENV.check_version(major_,minor_,revision_) }
/// Computes a Cholesky factorization of sparse matrix.
///
/// # Arguments
///
/// - `numthreads_` The number threads that can be used to do the computation. 0 means the code makes the choice.
/// - `ordermethod_` If nonzero, then a sparsity preserving ordering will be employed.
/// - `tolsingular_` A positive parameter controlling when a pivot is declared zero.
/// - `anzc_` anzc\[j\] is the number of nonzeros in the jth column of A.
/// - `aptrc_` aptrc\[j\] is a pointer to the first element in column j.
/// - `asubc_` Row indexes for each column stored in increasing order.
/// - `avalc_` The value corresponding to row indexed stored in asubc.
/// - `perm_` Permutation array used to specify the permutation matrix P computed by the function.
/// - `diag_` The diagonal elements of matrix D.
/// - `lnzc_` lnzc\[j\] is the number of non zero elements in column j.
/// - `lptrc_` lptrc\[j\] is a pointer to the first row index and value in column j.
/// - `lensubnval_` Number of elements in lsubc and lvalc.
/// - `lsubc_` Row indexes for each column stored in increasing order.
/// - `lvalc_` The values corresponding to row indexed stored in lsubc.
///
/// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.computesparsecholesky>
pub fn prototypepfx::compute_sparse_cholesky(numthreads_ : i32,ordermethod_ : i32,tolsingular_ : f64,anzc_ : &[i32],aptrc_ : &[i64],asubc_ : &[i32],avalc_ : &[f64],perm_ : &mut Vec<i32>,diag_ : &mut Vec<f64>,lnzc_ : &mut Vec<i32>,lptrc_ : &mut Vec<i64>,lensubnval_ : &mut i64,lsubc_ : &mut Vec<i32>,lvalc_ : &mut Vec<f64>) -> Result<(),String> { MSK_GLOBAL_ENV.compute_sparse_cholesky(numthreads_,ordermethod_,tolsingular_,anzc_,aptrc_,asubc_,avalc_,perm_,diag_,lnzc_,lptrc_,lensubnval_,lsubc_,lvalc_) }
/// Computes the inner product of two vectors.
///
/// # Arguments
///
/// - `n_` Length of the vectors.
/// - `x_` The x vector.
/// - `y_` The y vector.
/// - `xty_` The result of the inner product.
///
/// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.dot>
pub fn prototypepfx::dot(n_ : i32,x_ : &[f64],y_ : &[f64],xty_ : &mut f64) -> Result<(),String> { MSK_GLOBAL_ENV.dot(n_,x_,y_,xty_) }
/// Prints an intro to message stream.
///
/// # Arguments
///
/// - `longver_` If non-zero, then the intro is slightly longer.
///
/// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.echointro>
pub fn prototypepfx::echo_intro(longver_ : i32) -> Result<(),String> { MSK_GLOBAL_ENV.echo_intro(longver_) }
/// Performs a dense matrix multiplication.
///
/// # Arguments
///
/// - `transa_` Indicates whether the matrix A must be transposed.
///   
///   See [Transpose]
/// - `transb_` Indicates whether the matrix B must be transposed.
///   
///   See [Transpose]
/// - `m_` Indicates the number of rows of matrix C.
/// - `n_` Indicates the number of columns of matrix C.
/// - `k_` Specifies the common dimension along which op(A) and op(B) are multiplied.
/// - `alpha_` A scalar value multiplying the result of the matrix multiplication.
/// - `a_` The pointer to the array storing matrix A in a column-major format.
/// - `b_` The pointer to the array storing matrix B in a column-major format.
/// - `beta_` A scalar value that multiplies C.
/// - `c_` The pointer to the array storing matrix C in a column-major format.
///
/// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.gemm>
pub fn prototypepfx::gemm(transa_ : i32,transb_ : i32,m_ : i32,n_ : i32,k_ : i32,alpha_ : f64,a_ : &[f64],b_ : &[f64],beta_ : f64,c_ : &mut[f64]) -> Result<(),String> { MSK_GLOBAL_ENV.gemm(transa_,transb_,m_,n_,k_,alpha_,a_,b_,beta_,c_) }
/// Computes dense matrix times a dense vector product.
///
/// # Arguments
///
/// - `transa_` Indicates whether the matrix A must be transposed.
///   
///   See [Transpose]
/// - `m_` Specifies the number of rows of the matrix A.
/// - `n_` Specifies the number of columns of the matrix A.
/// - `alpha_` A scalar value multiplying the matrix A.
/// - `a_` A pointer to the array storing matrix A in a column-major format.
/// - `x_` A pointer to the array storing the vector x.
/// - `beta_` A scalar value multiplying the vector y.
/// - `y_` A pointer to the array storing the vector y.
///
/// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.gemv>
pub fn prototypepfx::gemv(transa_ : i32,m_ : i32,n_ : i32,alpha_ : f64,a_ : &[f64],x_ : &[f64],beta_ : f64,y_ : &mut[f64]) -> Result<(),String> { MSK_GLOBAL_ENV.gemv(transa_,m_,n_,alpha_,a_,x_,beta_,y_) }
/// Obtains build information.
///
/// # Returns
///
///   - `buildstate` State of binaries, i.e. a debug, release candidate or final release.
///   - `builddate` Date when the binaries were built.
///
/// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getbuildinfo>
#[allow(unused_parens)]
pub fn get_build_info() -> Result<(String,String),String> {
  let mut buildstate_ = Vec::new(); buildstate_.resize(Value::MAX_STR_LEN as usize,0);
  let mut builddate_ = Vec::new(); builddate_.resize(Value::MAX_STR_LEN as usize,0);
  handle_res_static(unsafe { MSK_getbuildinfo(buildstate_.as_mut_ptr(),builddate_.as_mut_ptr()) },"get_build_info")?;
  return Result::Ok((String::from_utf8_lossy(&buildstate_[..buildstate_.iter().position(|&c| c == 0).unwrap_or(Value::MAX_STR_LEN as usize)]).into_owned(),String::from_utf8_lossy(&builddate_[..builddate_.iter().position(|&c| c == 0).unwrap_or(Value::MAX_STR_LEN as usize)]).into_owned()));
} // getbuildinfo
/// Obtains a short description of a response code.
///
/// # Arguments
///
/// - `code_` A valid response code.
///   
///   See [Rescode]
///
/// # Returns
///
///   - `symname` Symbolic name corresponding to the code.
///   - `str` Obtains a short description of a response code.
///
/// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getcodedesc>
#[allow(unused_parens)]
pub fn get_code_desc(code_ : i32) -> Result<(String,String),String> {
  let mut symname_ = Vec::new(); symname_.resize(Value::MAX_STR_LEN as usize,0);
  let mut str_ = Vec::new(); str_.resize(Value::MAX_STR_LEN as usize,0);
  handle_res_static(unsafe { MSK_getcodedesc(code_,symname_.as_mut_ptr(),str_.as_mut_ptr()) },"get_code_desc")?;
  return Result::Ok((String::from_utf8_lossy(&symname_[..symname_.iter().position(|&c| c == 0).unwrap_or(Value::MAX_STR_LEN as usize)]).into_owned(),String::from_utf8_lossy(&str_[..str_.iter().position(|&c| c == 0).unwrap_or(Value::MAX_STR_LEN as usize)]).into_owned()));
} // getcodedesc
/// Obtain the class of a response code.
///
/// # Arguments
///
/// - `r_` A response code indicating the result of function call.
///   
///   See [Rescode]
/// - `rc_` The response class.
///   
///   See [Rescodetype]
///
/// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getresponseclass>
#[allow(unused_parens)]
pub fn get_response_class(r_ : i32,rc_ : & mut i32) -> Result<(),String> {
  handle_res_static(unsafe { MSK_getresponseclass(r_,rc_) },"get_response_class")?;
  return Result::Ok(());
} // getresponseclass
/// Obtains MOSEK version information.
///
/// # Arguments
///
/// - `major_` Major version number.
/// - `minor_` Minor version number.
/// - `revision_` Revision number.
///
/// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.getversion>
#[allow(unused_parens)]
pub fn get_version(major_ : &mut i32,minor_ : &mut i32,revision_ : &mut i32) -> Result<(),String> {
  handle_res_static(unsafe { MSK_getversion(major_,minor_,revision_) },"get_version")?;
  return Result::Ok(());
} // getversion
/// Return true if value is considered infinity by MOSEK.
///
/// # Arguments
///
/// - `value_` The value to be checked
///
/// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.isinfinity>
#[allow(unused_parens)]
pub fn is_infinity(value_ : f64) -> Result<(),String> {
  handle_res_static(unsafe { MSK_isinfinity(value_) },"is_infinity")?;
  return Result::Ok(());
} // isinfinity
/// Stops all threads and delete all handles used by the license system.
///
/// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.licensecleanup>
#[allow(unused_parens)]
pub fn license_cleanup() -> Result<(),String> {
  handle_res_static(unsafe { MSK_licensecleanup() },"license_cleanup")?;
  return Result::Ok(());
} // licensecleanup
/// Optimize a number of tasks in parallel using a specified number of threads.
///
/// # Arguments
///
/// - `israce_` If nonzero, then the function is terminated after the first task has been completed.
/// - `maxtime_` Time limit for the function.
/// - `numthreads_` Number of threads to be employed.
/// - `trmcode_` The termination code for each task.
///   
///   See [Rescode]
/// - `rcode_` The response code for each task.
///   
///   See [Rescode]
///
/// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.optimizebatch>
pub fn prototypepfx::optimize_batch(israce_ : bool,maxtime_ : f64,numthreads_ : i32,task_ : &[ & mut Task ],trmcode_ : &mut[i32],rcode_ : &mut[i32]) -> Result<(),String> { MSK_GLOBAL_ENV.optimize_batch(israce_,maxtime_,numthreads_,task_,trmcode_,rcode_) }
/// Computes a Cholesky factorization of a dense matrix.
///
/// # Arguments
///
/// - `uplo_` Indicates whether the upper or lower triangular part of the matrix is stored.
///   
///   See [Uplo]
/// - `n_` Dimension of the symmetric matrix.
/// - `a_` A symmetric matrix stored in column-major order.
///
/// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.potrf>
pub fn prototypepfx::potrf(uplo_ : i32,n_ : i32,a_ : &mut[f64]) -> Result<(),String> { MSK_GLOBAL_ENV.potrf(uplo_,n_,a_) }
/// Solves a sparse triangular system of linear equations.
///
/// # Arguments
///
/// - `transposed_` Controls whether the solve is with L or the transposed L.
///   
///   See [Transpose]
/// - `lnzc_` lnzc\[j\] is the number of nonzeros in column j.
/// - `lptrc_` lptrc\[j\] is a pointer to the first row index and value in column j.
/// - `lsubc_` Row indexes for each column stored sequentially.
/// - `lvalc_` The value corresponding to row indexed stored lsubc.
/// - `b_` The right-hand side of linear equation system to be solved as a dense vector.
///
/// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.sparsetriangularsolvedense>
pub fn prototypepfx::sparse_triangular_solve_dense(transposed_ : i32,lnzc_ : &[i32],lptrc_ : &[i64],lsubc_ : &[i32],lvalc_ : &[f64],b_ : &mut[f64]) -> Result<(),String> { MSK_GLOBAL_ENV.sparse_triangular_solve_dense(transposed_,lnzc_,lptrc_,lsubc_,lvalc_,b_) }
/// Computes all eigenvalues of a symmetric dense matrix.
///
/// # Arguments
///
/// - `uplo_` Indicates whether the upper or lower triangular part is used.
///   
///   See [Uplo]
/// - `n_` Dimension of the symmetric input matrix.
/// - `a_` Input matrix A.
/// - `w_` Array of length at least n containing the eigenvalues of A.
///
/// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.syeig>
pub fn prototypepfx::syeig(uplo_ : i32,n_ : i32,a_ : &[f64],w_ : &mut[f64]) -> Result<(),String> { MSK_GLOBAL_ENV.syeig(uplo_,n_,a_,w_) }
/// Computes all the eigenvalues and eigenvectors of a symmetric dense matrix, and thus its eigenvalue decomposition.
///
/// # Arguments
///
/// - `uplo_` Indicates whether the upper or lower triangular part is used.
///   
///   See [Uplo]
/// - `n_` Dimension of the symmetric input matrix.
/// - `a_` Input matrix A.
/// - `w_` Array of length at least n containing the eigenvalues of A.
///
/// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.syevd>
pub fn prototypepfx::syevd(uplo_ : i32,n_ : i32,a_ : &mut[f64],w_ : &mut[f64]) -> Result<(),String> { MSK_GLOBAL_ENV.syevd(uplo_,n_,a_,w_) }
/// Obtains the value corresponding to a symbolic name defined by MOSEK.
///
/// # Arguments
///
/// - `name_` Symbolic name.
///
/// # Returns
///
///   - `value` The corresponding value.
///
/// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.symnamtovalue>
#[allow(unused_parens)]
pub fn sym_nam_to_value(name_ : &str) -> Result<String,String> {
  let __tmp_1 = CString::new(name_).unwrap();
  let mut value_ = Vec::new(); value_.resize(Value::MAX_STR_LEN as usize,0);
  handle_res_static(unsafe { MSK_symnamtovalue(__tmp_1.as_ptr(),value_.as_mut_ptr()) },"sym_nam_to_value")?;
  return Result::Ok(String::from_utf8_lossy(&value_[..value_.iter().position(|&c| c == 0).unwrap_or(Value::MAX_STR_LEN as usize)]).into_owned());
} // symnamtovalue
/// Performs a rank-k update of a symmetric matrix.
///
/// # Arguments
///
/// - `uplo_` Indicates whether the upper or lower triangular part of C is used.
///   
///   See [Uplo]
/// - `trans_` Indicates whether the matrix A must be transposed.
///   
///   See [Transpose]
/// - `n_` Specifies the order of C.
/// - `k_` Indicates the number of rows or columns of A, and its rank.
/// - `alpha_` A scalar value multiplying the result of the matrix multiplication.
/// - `a_` The pointer to the array storing matrix A in a column-major format.
/// - `beta_` A scalar value that multiplies C.
/// - `c_` The pointer to the array storing matrix C in a column-major format.
///
/// Full documentation: <https://docs.mosek.com/latest/capi/alphabetic-functionalities.html#mosek.env.syrk>
pub fn prototypepfx::syrk(uplo_ : i32,trans_ : i32,n_ : i32,k_ : i32,alpha_ : f64,a_ : &[f64],beta_ : f64,c_ : &mut[f64]) -> Result<(),String> { MSK_GLOBAL_ENV.syrk(uplo_,trans_,n_,k_,alpha_,a_,beta_,c_) }

