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

// Generted for MOSEK v[10, 0, 16]

extern crate libc;
use std::ffi::CString;
use std::ffi::CStr;
use libc::c_void;
use std::convert::TryInto;
use std::default::Default;

//#[link(name = "mosek64")]
extern {
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
    fn MSK_freetask(task_ : * const u8,buffer_ : * mut u8) -> i32;
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
    fn MSK_solvewithbasis(task_ : * const u8,transp_ : i32,numnz_ : i32,sub_ : * mut i32,val_ : * mut f64,numnzout_ : & mut i32) -> i32;
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
    #[allow(dead_code)]
    fn MSK_computesparsecholesky(env_ : * const u8,numthreads_ : i32,ordermethod_ : i32,tolsingular_ : f64,n_ : i32,anzc_ : * const i32,aptrc_ : * const i64,asubc_ : * const i32,avalc_ : * const f64,perm_ : & mut * const i32,diag_ : & mut * const f64,lnzc_ : & mut * const i32,lptrc_ : & mut * const i64,lensubnval_ : & mut i64,lsubc_ : & mut * const i32,lvalc_ : & mut * const f64) -> i32;
    #[allow(dead_code)]
    fn MSK_deleteenv(env_ : & mut * const u8) -> i32;
    fn MSK_dot(env_ : * const u8,n_ : i32,x_ : * const f64,y_ : * const f64,xty_ : & mut f64) -> i32;
    fn MSK_echointro(env_ : * const u8,longver_ : i32) -> i32;
    fn MSK_enablegarcolenv(env_ : * const u8) -> i32;
    fn MSK_expirylicenses(env_ : * const u8,expiry_ : & mut i64) -> i32;
    #[allow(dead_code)]
    fn MSK_freeenv(env_ : * const u8,buffer_ : * mut u8) -> i32;
    fn MSK_gemm(env_ : * const u8,transa_ : i32,transb_ : i32,m_ : i32,n_ : i32,k_ : i32,alpha_ : f64,a_ : * const f64,b_ : * const f64,beta_ : f64,c_ : * mut f64) -> i32;
    fn MSK_getbuildinfo(buildstate : * mut u8,builddate : * mut u8) -> i32;
    fn MSK_getcodedesc(code_ : i32,symname : * mut u8,str : * mut u8) -> i32;
    fn MSK_getresponseclass(r_ : i32,rc_ : &mut i32) -> i32;
    fn MSK_getversion(major_ : & mut i32,minor_ : & mut i32,revision_ : & mut i32) -> i32;
    fn MSK_isinfinity(value_ : f64) -> i32;
    fn MSK_licensecleanup() -> i32;
    fn MSK_linkfiletoenvstream(env_ : * const u8,whichstream_ : i32,filename_ : * const libc::c_char,append_ : i32) -> i32;
    fn MSK_makeemptytask(env_ : * const u8,task_ : & mut * const u8) -> i32;
    #[allow(dead_code)]
    fn MSK_makeenv(env_ : & mut * const u8,dbgfile_ : * const libc::c_char) -> i32;
    #[allow(dead_code)]
    fn MSK_makeenvdebug(env_ : & mut * const u8,maxnumalloc_ : i64,dbgfile_ : * const libc::c_char) -> i32;
    fn MSK_maketask(env_ : * const u8,maxnumcon_ : i32,maxnumvar_ : i32,task_ : & mut * const u8) -> i32;
    #[allow(dead_code)]
    fn MSK_optimizebatch(env_ : * const u8,israce_ : i32,maxtime_ : f64,numthreads_ : i32,numtask_ : i64,task_ : * const * const u8,trmcode_ : * mut i32,rcode_ : * mut i32) -> i32;
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


// basindtype
pub const MSK_BI_NEVER : i32 = 0;
pub const MSK_BI_ALWAYS : i32 = 1;
pub const MSK_BI_NO_ERROR : i32 = 2;
pub const MSK_BI_IF_FEASIBLE : i32 = 3;
pub const MSK_BI_RESERVERED : i32 = 4;

// boundkey
pub const MSK_BK_LO : i32 = 0;
pub const MSK_BK_UP : i32 = 1;
pub const MSK_BK_FX : i32 = 2;
pub const MSK_BK_FR : i32 = 3;
pub const MSK_BK_RA : i32 = 4;

// mark
pub const MSK_MARK_LO : i32 = 0;
pub const MSK_MARK_UP : i32 = 1;

// simdegen
pub const MSK_SIM_DEGEN_NONE : i32 = 0;
pub const MSK_SIM_DEGEN_FREE : i32 = 1;
pub const MSK_SIM_DEGEN_AGGRESSIVE : i32 = 2;
pub const MSK_SIM_DEGEN_MODERATE : i32 = 3;
pub const MSK_SIM_DEGEN_MINIMUM : i32 = 4;

// transpose
pub const MSK_TRANSPOSE_NO : i32 = 0;
pub const MSK_TRANSPOSE_YES : i32 = 1;

// uplo
pub const MSK_UPLO_LO : i32 = 0;
pub const MSK_UPLO_UP : i32 = 1;

// simreform
pub const MSK_SIM_REFORMULATION_OFF : i32 = 0;
pub const MSK_SIM_REFORMULATION_ON : i32 = 1;
pub const MSK_SIM_REFORMULATION_FREE : i32 = 2;
pub const MSK_SIM_REFORMULATION_AGGRESSIVE : i32 = 3;

// simdupvec
pub const MSK_SIM_EXPLOIT_DUPVEC_OFF : i32 = 0;
pub const MSK_SIM_EXPLOIT_DUPVEC_ON : i32 = 1;
pub const MSK_SIM_EXPLOIT_DUPVEC_FREE : i32 = 2;

// simhotstart
pub const MSK_SIM_HOTSTART_NONE : i32 = 0;
pub const MSK_SIM_HOTSTART_FREE : i32 = 1;
pub const MSK_SIM_HOTSTART_STATUS_KEYS : i32 = 2;

// intpnthotstart
pub const MSK_INTPNT_HOTSTART_NONE : i32 = 0;
pub const MSK_INTPNT_HOTSTART_PRIMAL : i32 = 1;
pub const MSK_INTPNT_HOTSTART_DUAL : i32 = 2;
pub const MSK_INTPNT_HOTSTART_PRIMAL_DUAL : i32 = 3;

// purify
pub const MSK_PURIFY_NONE : i32 = 0;
pub const MSK_PURIFY_PRIMAL : i32 = 1;
pub const MSK_PURIFY_DUAL : i32 = 2;
pub const MSK_PURIFY_PRIMAL_DUAL : i32 = 3;
pub const MSK_PURIFY_AUTO : i32 = 4;

// callbackcode
pub const MSK_CALLBACK_BEGIN_BI : i32 = 0;
pub const MSK_CALLBACK_BEGIN_CONIC : i32 = 1;
pub const MSK_CALLBACK_BEGIN_DUAL_BI : i32 = 2;
pub const MSK_CALLBACK_BEGIN_DUAL_SENSITIVITY : i32 = 3;
pub const MSK_CALLBACK_BEGIN_DUAL_SETUP_BI : i32 = 4;
pub const MSK_CALLBACK_BEGIN_DUAL_SIMPLEX : i32 = 5;
pub const MSK_CALLBACK_BEGIN_DUAL_SIMPLEX_BI : i32 = 6;
pub const MSK_CALLBACK_BEGIN_INFEAS_ANA : i32 = 7;
pub const MSK_CALLBACK_BEGIN_INTPNT : i32 = 8;
pub const MSK_CALLBACK_BEGIN_LICENSE_WAIT : i32 = 9;
pub const MSK_CALLBACK_BEGIN_MIO : i32 = 10;
pub const MSK_CALLBACK_BEGIN_OPTIMIZER : i32 = 11;
pub const MSK_CALLBACK_BEGIN_PRESOLVE : i32 = 12;
pub const MSK_CALLBACK_BEGIN_PRIMAL_BI : i32 = 13;
pub const MSK_CALLBACK_BEGIN_PRIMAL_REPAIR : i32 = 14;
pub const MSK_CALLBACK_BEGIN_PRIMAL_SENSITIVITY : i32 = 15;
pub const MSK_CALLBACK_BEGIN_PRIMAL_SETUP_BI : i32 = 16;
pub const MSK_CALLBACK_BEGIN_PRIMAL_SIMPLEX : i32 = 17;
pub const MSK_CALLBACK_BEGIN_PRIMAL_SIMPLEX_BI : i32 = 18;
pub const MSK_CALLBACK_BEGIN_QCQO_REFORMULATE : i32 = 19;
pub const MSK_CALLBACK_BEGIN_READ : i32 = 20;
pub const MSK_CALLBACK_BEGIN_ROOT_CUTGEN : i32 = 21;
pub const MSK_CALLBACK_BEGIN_SIMPLEX : i32 = 22;
pub const MSK_CALLBACK_BEGIN_SIMPLEX_BI : i32 = 23;
pub const MSK_CALLBACK_BEGIN_SOLVE_ROOT_RELAX : i32 = 24;
pub const MSK_CALLBACK_BEGIN_TO_CONIC : i32 = 25;
pub const MSK_CALLBACK_BEGIN_WRITE : i32 = 26;
pub const MSK_CALLBACK_CONIC : i32 = 27;
pub const MSK_CALLBACK_DUAL_SIMPLEX : i32 = 28;
pub const MSK_CALLBACK_END_BI : i32 = 29;
pub const MSK_CALLBACK_END_CONIC : i32 = 30;
pub const MSK_CALLBACK_END_DUAL_BI : i32 = 31;
pub const MSK_CALLBACK_END_DUAL_SENSITIVITY : i32 = 32;
pub const MSK_CALLBACK_END_DUAL_SETUP_BI : i32 = 33;
pub const MSK_CALLBACK_END_DUAL_SIMPLEX : i32 = 34;
pub const MSK_CALLBACK_END_DUAL_SIMPLEX_BI : i32 = 35;
pub const MSK_CALLBACK_END_INFEAS_ANA : i32 = 36;
pub const MSK_CALLBACK_END_INTPNT : i32 = 37;
pub const MSK_CALLBACK_END_LICENSE_WAIT : i32 = 38;
pub const MSK_CALLBACK_END_MIO : i32 = 39;
pub const MSK_CALLBACK_END_OPTIMIZER : i32 = 40;
pub const MSK_CALLBACK_END_PRESOLVE : i32 = 41;
pub const MSK_CALLBACK_END_PRIMAL_BI : i32 = 42;
pub const MSK_CALLBACK_END_PRIMAL_REPAIR : i32 = 43;
pub const MSK_CALLBACK_END_PRIMAL_SENSITIVITY : i32 = 44;
pub const MSK_CALLBACK_END_PRIMAL_SETUP_BI : i32 = 45;
pub const MSK_CALLBACK_END_PRIMAL_SIMPLEX : i32 = 46;
pub const MSK_CALLBACK_END_PRIMAL_SIMPLEX_BI : i32 = 47;
pub const MSK_CALLBACK_END_QCQO_REFORMULATE : i32 = 48;
pub const MSK_CALLBACK_END_READ : i32 = 49;
pub const MSK_CALLBACK_END_ROOT_CUTGEN : i32 = 50;
pub const MSK_CALLBACK_END_SIMPLEX : i32 = 51;
pub const MSK_CALLBACK_END_SIMPLEX_BI : i32 = 52;
pub const MSK_CALLBACK_END_SOLVE_ROOT_RELAX : i32 = 53;
pub const MSK_CALLBACK_END_TO_CONIC : i32 = 54;
pub const MSK_CALLBACK_END_WRITE : i32 = 55;
pub const MSK_CALLBACK_IM_BI : i32 = 56;
pub const MSK_CALLBACK_IM_CONIC : i32 = 57;
pub const MSK_CALLBACK_IM_DUAL_BI : i32 = 58;
pub const MSK_CALLBACK_IM_DUAL_SENSIVITY : i32 = 59;
pub const MSK_CALLBACK_IM_DUAL_SIMPLEX : i32 = 60;
pub const MSK_CALLBACK_IM_INTPNT : i32 = 61;
pub const MSK_CALLBACK_IM_LICENSE_WAIT : i32 = 62;
pub const MSK_CALLBACK_IM_LU : i32 = 63;
pub const MSK_CALLBACK_IM_MIO : i32 = 64;
pub const MSK_CALLBACK_IM_MIO_DUAL_SIMPLEX : i32 = 65;
pub const MSK_CALLBACK_IM_MIO_INTPNT : i32 = 66;
pub const MSK_CALLBACK_IM_MIO_PRIMAL_SIMPLEX : i32 = 67;
pub const MSK_CALLBACK_IM_ORDER : i32 = 68;
pub const MSK_CALLBACK_IM_PRESOLVE : i32 = 69;
pub const MSK_CALLBACK_IM_PRIMAL_BI : i32 = 70;
pub const MSK_CALLBACK_IM_PRIMAL_SENSIVITY : i32 = 71;
pub const MSK_CALLBACK_IM_PRIMAL_SIMPLEX : i32 = 72;
pub const MSK_CALLBACK_IM_QO_REFORMULATE : i32 = 73;
pub const MSK_CALLBACK_IM_READ : i32 = 74;
pub const MSK_CALLBACK_IM_ROOT_CUTGEN : i32 = 75;
pub const MSK_CALLBACK_IM_SIMPLEX : i32 = 76;
pub const MSK_CALLBACK_IM_SIMPLEX_BI : i32 = 77;
pub const MSK_CALLBACK_INTPNT : i32 = 78;
pub const MSK_CALLBACK_NEW_INT_MIO : i32 = 79;
pub const MSK_CALLBACK_PRIMAL_SIMPLEX : i32 = 80;
pub const MSK_CALLBACK_READ_OPF : i32 = 81;
pub const MSK_CALLBACK_READ_OPF_SECTION : i32 = 82;
pub const MSK_CALLBACK_SOLVING_REMOTE : i32 = 83;
pub const MSK_CALLBACK_UPDATE_DUAL_BI : i32 = 84;
pub const MSK_CALLBACK_UPDATE_DUAL_SIMPLEX : i32 = 85;
pub const MSK_CALLBACK_UPDATE_DUAL_SIMPLEX_BI : i32 = 86;
pub const MSK_CALLBACK_UPDATE_PRESOLVE : i32 = 87;
pub const MSK_CALLBACK_UPDATE_PRIMAL_BI : i32 = 88;
pub const MSK_CALLBACK_UPDATE_PRIMAL_SIMPLEX : i32 = 89;
pub const MSK_CALLBACK_UPDATE_PRIMAL_SIMPLEX_BI : i32 = 90;
pub const MSK_CALLBACK_UPDATE_SIMPLEX : i32 = 91;
pub const MSK_CALLBACK_WRITE_OPF : i32 = 92;

// checkconvexitytype
pub const MSK_CHECK_CONVEXITY_NONE : i32 = 0;
pub const MSK_CHECK_CONVEXITY_SIMPLE : i32 = 1;
pub const MSK_CHECK_CONVEXITY_FULL : i32 = 2;

// compresstype
pub const MSK_COMPRESS_NONE : i32 = 0;
pub const MSK_COMPRESS_FREE : i32 = 1;
pub const MSK_COMPRESS_GZIP : i32 = 2;
pub const MSK_COMPRESS_ZSTD : i32 = 3;

// conetype
pub const MSK_CT_QUAD : i32 = 0;
pub const MSK_CT_RQUAD : i32 = 1;
pub const MSK_CT_PEXP : i32 = 2;
pub const MSK_CT_DEXP : i32 = 3;
pub const MSK_CT_PPOW : i32 = 4;
pub const MSK_CT_DPOW : i32 = 5;
pub const MSK_CT_ZERO : i32 = 6;

// domaintype
pub const MSK_DOMAIN_R : i32 = 0;
pub const MSK_DOMAIN_RZERO : i32 = 1;
pub const MSK_DOMAIN_RPLUS : i32 = 2;
pub const MSK_DOMAIN_RMINUS : i32 = 3;
pub const MSK_DOMAIN_QUADRATIC_CONE : i32 = 4;
pub const MSK_DOMAIN_RQUADRATIC_CONE : i32 = 5;
pub const MSK_DOMAIN_PRIMAL_EXP_CONE : i32 = 6;
pub const MSK_DOMAIN_DUAL_EXP_CONE : i32 = 7;
pub const MSK_DOMAIN_PRIMAL_POWER_CONE : i32 = 8;
pub const MSK_DOMAIN_DUAL_POWER_CONE : i32 = 9;
pub const MSK_DOMAIN_PRIMAL_GEO_MEAN_CONE : i32 = 10;
pub const MSK_DOMAIN_DUAL_GEO_MEAN_CONE : i32 = 11;
pub const MSK_DOMAIN_SVEC_PSD_CONE : i32 = 12;

// nametype
pub const MSK_NAME_TYPE_GEN : i32 = 0;
pub const MSK_NAME_TYPE_MPS : i32 = 1;
pub const MSK_NAME_TYPE_LP : i32 = 2;

// symmattype
pub const MSK_SYMMAT_TYPE_SPARSE : i32 = 0;

// dataformat
pub const MSK_DATA_FORMAT_EXTENSION : i32 = 0;
pub const MSK_DATA_FORMAT_MPS : i32 = 1;
pub const MSK_DATA_FORMAT_LP : i32 = 2;
pub const MSK_DATA_FORMAT_OP : i32 = 3;
pub const MSK_DATA_FORMAT_FREE_MPS : i32 = 4;
pub const MSK_DATA_FORMAT_TASK : i32 = 5;
pub const MSK_DATA_FORMAT_PTF : i32 = 6;
pub const MSK_DATA_FORMAT_CB : i32 = 7;
pub const MSK_DATA_FORMAT_JSON_TASK : i32 = 8;

// solformat
pub const MSK_SOL_FORMAT_EXTENSION : i32 = 0;
pub const MSK_SOL_FORMAT_B : i32 = 1;
pub const MSK_SOL_FORMAT_TASK : i32 = 2;
pub const MSK_SOL_FORMAT_JSON_TASK : i32 = 3;

// dinfitem
pub const MSK_DINF_BI_CLEAN_DUAL_TIME : i32 = 0;
pub const MSK_DINF_BI_CLEAN_PRIMAL_TIME : i32 = 1;
pub const MSK_DINF_BI_CLEAN_TIME : i32 = 2;
pub const MSK_DINF_BI_DUAL_TIME : i32 = 3;
pub const MSK_DINF_BI_PRIMAL_TIME : i32 = 4;
pub const MSK_DINF_BI_TIME : i32 = 5;
pub const MSK_DINF_INTPNT_DUAL_FEAS : i32 = 6;
pub const MSK_DINF_INTPNT_DUAL_OBJ : i32 = 7;
pub const MSK_DINF_INTPNT_FACTOR_NUM_FLOPS : i32 = 8;
pub const MSK_DINF_INTPNT_OPT_STATUS : i32 = 9;
pub const MSK_DINF_INTPNT_ORDER_TIME : i32 = 10;
pub const MSK_DINF_INTPNT_PRIMAL_FEAS : i32 = 11;
pub const MSK_DINF_INTPNT_PRIMAL_OBJ : i32 = 12;
pub const MSK_DINF_INTPNT_TIME : i32 = 13;
pub const MSK_DINF_MIO_CLIQUE_SEPARATION_TIME : i32 = 14;
pub const MSK_DINF_MIO_CMIR_SEPARATION_TIME : i32 = 15;
pub const MSK_DINF_MIO_CONSTRUCT_SOLUTION_OBJ : i32 = 16;
pub const MSK_DINF_MIO_DUAL_BOUND_AFTER_PRESOLVE : i32 = 17;
pub const MSK_DINF_MIO_GMI_SEPARATION_TIME : i32 = 18;
pub const MSK_DINF_MIO_IMPLIED_BOUND_TIME : i32 = 19;
pub const MSK_DINF_MIO_INITIAL_FEASIBLE_SOLUTION_OBJ : i32 = 20;
pub const MSK_DINF_MIO_KNAPSACK_COVER_SEPARATION_TIME : i32 = 21;
pub const MSK_DINF_MIO_LIPRO_SEPARATION_TIME : i32 = 22;
pub const MSK_DINF_MIO_OBJ_ABS_GAP : i32 = 23;
pub const MSK_DINF_MIO_OBJ_BOUND : i32 = 24;
pub const MSK_DINF_MIO_OBJ_INT : i32 = 25;
pub const MSK_DINF_MIO_OBJ_REL_GAP : i32 = 26;
pub const MSK_DINF_MIO_PROBING_TIME : i32 = 27;
pub const MSK_DINF_MIO_ROOT_CUTGEN_TIME : i32 = 28;
pub const MSK_DINF_MIO_ROOT_OPTIMIZER_TIME : i32 = 29;
pub const MSK_DINF_MIO_ROOT_PRESOLVE_TIME : i32 = 30;
pub const MSK_DINF_MIO_ROOT_TIME : i32 = 31;
pub const MSK_DINF_MIO_TIME : i32 = 32;
pub const MSK_DINF_MIO_USER_OBJ_CUT : i32 = 33;
pub const MSK_DINF_OPTIMIZER_TIME : i32 = 34;
pub const MSK_DINF_PRESOLVE_ELI_TIME : i32 = 35;
pub const MSK_DINF_PRESOLVE_LINDEP_TIME : i32 = 36;
pub const MSK_DINF_PRESOLVE_TIME : i32 = 37;
pub const MSK_DINF_PRESOLVE_TOTAL_PRIMAL_PERTURBATION : i32 = 38;
pub const MSK_DINF_PRIMAL_REPAIR_PENALTY_OBJ : i32 = 39;
pub const MSK_DINF_QCQO_REFORMULATE_MAX_PERTURBATION : i32 = 40;
pub const MSK_DINF_QCQO_REFORMULATE_TIME : i32 = 41;
pub const MSK_DINF_QCQO_REFORMULATE_WORST_CHOLESKY_COLUMN_SCALING : i32 = 42;
pub const MSK_DINF_QCQO_REFORMULATE_WORST_CHOLESKY_DIAG_SCALING : i32 = 43;
pub const MSK_DINF_RD_TIME : i32 = 44;
pub const MSK_DINF_REMOTE_TIME : i32 = 45;
pub const MSK_DINF_SIM_DUAL_TIME : i32 = 46;
pub const MSK_DINF_SIM_FEAS : i32 = 47;
pub const MSK_DINF_SIM_OBJ : i32 = 48;
pub const MSK_DINF_SIM_PRIMAL_TIME : i32 = 49;
pub const MSK_DINF_SIM_TIME : i32 = 50;
pub const MSK_DINF_SOL_BAS_DUAL_OBJ : i32 = 51;
pub const MSK_DINF_SOL_BAS_DVIOLCON : i32 = 52;
pub const MSK_DINF_SOL_BAS_DVIOLVAR : i32 = 53;
pub const MSK_DINF_SOL_BAS_NRM_BARX : i32 = 54;
pub const MSK_DINF_SOL_BAS_NRM_SLC : i32 = 55;
pub const MSK_DINF_SOL_BAS_NRM_SLX : i32 = 56;
pub const MSK_DINF_SOL_BAS_NRM_SUC : i32 = 57;
pub const MSK_DINF_SOL_BAS_NRM_SUX : i32 = 58;
pub const MSK_DINF_SOL_BAS_NRM_XC : i32 = 59;
pub const MSK_DINF_SOL_BAS_NRM_XX : i32 = 60;
pub const MSK_DINF_SOL_BAS_NRM_Y : i32 = 61;
pub const MSK_DINF_SOL_BAS_PRIMAL_OBJ : i32 = 62;
pub const MSK_DINF_SOL_BAS_PVIOLCON : i32 = 63;
pub const MSK_DINF_SOL_BAS_PVIOLVAR : i32 = 64;
pub const MSK_DINF_SOL_ITG_NRM_BARX : i32 = 65;
pub const MSK_DINF_SOL_ITG_NRM_XC : i32 = 66;
pub const MSK_DINF_SOL_ITG_NRM_XX : i32 = 67;
pub const MSK_DINF_SOL_ITG_PRIMAL_OBJ : i32 = 68;
pub const MSK_DINF_SOL_ITG_PVIOLACC : i32 = 69;
pub const MSK_DINF_SOL_ITG_PVIOLBARVAR : i32 = 70;
pub const MSK_DINF_SOL_ITG_PVIOLCON : i32 = 71;
pub const MSK_DINF_SOL_ITG_PVIOLCONES : i32 = 72;
pub const MSK_DINF_SOL_ITG_PVIOLDJC : i32 = 73;
pub const MSK_DINF_SOL_ITG_PVIOLITG : i32 = 74;
pub const MSK_DINF_SOL_ITG_PVIOLVAR : i32 = 75;
pub const MSK_DINF_SOL_ITR_DUAL_OBJ : i32 = 76;
pub const MSK_DINF_SOL_ITR_DVIOLACC : i32 = 77;
pub const MSK_DINF_SOL_ITR_DVIOLBARVAR : i32 = 78;
pub const MSK_DINF_SOL_ITR_DVIOLCON : i32 = 79;
pub const MSK_DINF_SOL_ITR_DVIOLCONES : i32 = 80;
pub const MSK_DINF_SOL_ITR_DVIOLVAR : i32 = 81;
pub const MSK_DINF_SOL_ITR_NRM_BARS : i32 = 82;
pub const MSK_DINF_SOL_ITR_NRM_BARX : i32 = 83;
pub const MSK_DINF_SOL_ITR_NRM_SLC : i32 = 84;
pub const MSK_DINF_SOL_ITR_NRM_SLX : i32 = 85;
pub const MSK_DINF_SOL_ITR_NRM_SNX : i32 = 86;
pub const MSK_DINF_SOL_ITR_NRM_SUC : i32 = 87;
pub const MSK_DINF_SOL_ITR_NRM_SUX : i32 = 88;
pub const MSK_DINF_SOL_ITR_NRM_XC : i32 = 89;
pub const MSK_DINF_SOL_ITR_NRM_XX : i32 = 90;
pub const MSK_DINF_SOL_ITR_NRM_Y : i32 = 91;
pub const MSK_DINF_SOL_ITR_PRIMAL_OBJ : i32 = 92;
pub const MSK_DINF_SOL_ITR_PVIOLACC : i32 = 93;
pub const MSK_DINF_SOL_ITR_PVIOLBARVAR : i32 = 94;
pub const MSK_DINF_SOL_ITR_PVIOLCON : i32 = 95;
pub const MSK_DINF_SOL_ITR_PVIOLCONES : i32 = 96;
pub const MSK_DINF_SOL_ITR_PVIOLVAR : i32 = 97;
pub const MSK_DINF_TO_CONIC_TIME : i32 = 98;
pub const MSK_DINF_END : i32 = 98;

// feature
pub const MSK_FEATURE_PTS : i32 = 0;
pub const MSK_FEATURE_PTON : i32 = 1;

// dparam
pub const MSK_DPAR_ANA_SOL_INFEAS_TOL : i32 = 0;
pub const MSK_DPAR_BASIS_REL_TOL_S : i32 = 1;
pub const MSK_DPAR_BASIS_TOL_S : i32 = 2;
pub const MSK_DPAR_BASIS_TOL_X : i32 = 3;
pub const MSK_DPAR_CHECK_CONVEXITY_REL_TOL : i32 = 4;
pub const MSK_DPAR_DATA_SYM_MAT_TOL : i32 = 5;
pub const MSK_DPAR_DATA_SYM_MAT_TOL_HUGE : i32 = 6;
pub const MSK_DPAR_DATA_SYM_MAT_TOL_LARGE : i32 = 7;
pub const MSK_DPAR_DATA_TOL_AIJ_HUGE : i32 = 8;
pub const MSK_DPAR_DATA_TOL_AIJ_LARGE : i32 = 9;
pub const MSK_DPAR_DATA_TOL_BOUND_INF : i32 = 10;
pub const MSK_DPAR_DATA_TOL_BOUND_WRN : i32 = 11;
pub const MSK_DPAR_DATA_TOL_C_HUGE : i32 = 12;
pub const MSK_DPAR_DATA_TOL_CJ_LARGE : i32 = 13;
pub const MSK_DPAR_DATA_TOL_QIJ : i32 = 14;
pub const MSK_DPAR_DATA_TOL_X : i32 = 15;
pub const MSK_DPAR_INTPNT_CO_TOL_DFEAS : i32 = 16;
pub const MSK_DPAR_INTPNT_CO_TOL_INFEAS : i32 = 17;
pub const MSK_DPAR_INTPNT_CO_TOL_MU_RED : i32 = 18;
pub const MSK_DPAR_INTPNT_CO_TOL_NEAR_REL : i32 = 19;
pub const MSK_DPAR_INTPNT_CO_TOL_PFEAS : i32 = 20;
pub const MSK_DPAR_INTPNT_CO_TOL_REL_GAP : i32 = 21;
pub const MSK_DPAR_INTPNT_QO_TOL_DFEAS : i32 = 22;
pub const MSK_DPAR_INTPNT_QO_TOL_INFEAS : i32 = 23;
pub const MSK_DPAR_INTPNT_QO_TOL_MU_RED : i32 = 24;
pub const MSK_DPAR_INTPNT_QO_TOL_NEAR_REL : i32 = 25;
pub const MSK_DPAR_INTPNT_QO_TOL_PFEAS : i32 = 26;
pub const MSK_DPAR_INTPNT_QO_TOL_REL_GAP : i32 = 27;
pub const MSK_DPAR_INTPNT_TOL_DFEAS : i32 = 28;
pub const MSK_DPAR_INTPNT_TOL_DSAFE : i32 = 29;
pub const MSK_DPAR_INTPNT_TOL_INFEAS : i32 = 30;
pub const MSK_DPAR_INTPNT_TOL_MU_RED : i32 = 31;
pub const MSK_DPAR_INTPNT_TOL_PATH : i32 = 32;
pub const MSK_DPAR_INTPNT_TOL_PFEAS : i32 = 33;
pub const MSK_DPAR_INTPNT_TOL_PSAFE : i32 = 34;
pub const MSK_DPAR_INTPNT_TOL_REL_GAP : i32 = 35;
pub const MSK_DPAR_INTPNT_TOL_REL_STEP : i32 = 36;
pub const MSK_DPAR_INTPNT_TOL_STEP_SIZE : i32 = 37;
pub const MSK_DPAR_LOWER_OBJ_CUT : i32 = 38;
pub const MSK_DPAR_LOWER_OBJ_CUT_FINITE_TRH : i32 = 39;
pub const MSK_DPAR_MIO_DJC_MAX_BIGM : i32 = 40;
pub const MSK_DPAR_MIO_MAX_TIME : i32 = 41;
pub const MSK_DPAR_MIO_REL_GAP_CONST : i32 = 42;
pub const MSK_DPAR_MIO_TOL_ABS_GAP : i32 = 43;
pub const MSK_DPAR_MIO_TOL_ABS_RELAX_INT : i32 = 44;
pub const MSK_DPAR_MIO_TOL_FEAS : i32 = 45;
pub const MSK_DPAR_MIO_TOL_REL_DUAL_BOUND_IMPROVEMENT : i32 = 46;
pub const MSK_DPAR_MIO_TOL_REL_GAP : i32 = 47;
pub const MSK_DPAR_OPTIMIZER_MAX_TIME : i32 = 48;
pub const MSK_DPAR_PRESOLVE_TOL_ABS_LINDEP : i32 = 49;
pub const MSK_DPAR_PRESOLVE_TOL_AIJ : i32 = 50;
pub const MSK_DPAR_PRESOLVE_TOL_PRIMAL_INFEAS_PERTURBATION : i32 = 51;
pub const MSK_DPAR_PRESOLVE_TOL_REL_LINDEP : i32 = 52;
pub const MSK_DPAR_PRESOLVE_TOL_S : i32 = 53;
pub const MSK_DPAR_PRESOLVE_TOL_X : i32 = 54;
pub const MSK_DPAR_QCQO_REFORMULATE_REL_DROP_TOL : i32 = 55;
pub const MSK_DPAR_SEMIDEFINITE_TOL_APPROX : i32 = 56;
pub const MSK_DPAR_SIM_LU_TOL_REL_PIV : i32 = 57;
pub const MSK_DPAR_SIMPLEX_ABS_TOL_PIV : i32 = 58;
pub const MSK_DPAR_UPPER_OBJ_CUT : i32 = 59;
pub const MSK_DPAR_UPPER_OBJ_CUT_FINITE_TRH : i32 = 60;

// liinfitem
pub const MSK_LIINF_BI_CLEAN_DUAL_DEG_ITER : i32 = 0;
pub const MSK_LIINF_BI_CLEAN_DUAL_ITER : i32 = 1;
pub const MSK_LIINF_BI_CLEAN_PRIMAL_DEG_ITER : i32 = 2;
pub const MSK_LIINF_BI_CLEAN_PRIMAL_ITER : i32 = 3;
pub const MSK_LIINF_BI_DUAL_ITER : i32 = 4;
pub const MSK_LIINF_BI_PRIMAL_ITER : i32 = 5;
pub const MSK_LIINF_INTPNT_FACTOR_NUM_NZ : i32 = 6;
pub const MSK_LIINF_MIO_ANZ : i32 = 7;
pub const MSK_LIINF_MIO_INTPNT_ITER : i32 = 8;
pub const MSK_LIINF_MIO_NUM_DUAL_ILLPOSED_CER : i32 = 9;
pub const MSK_LIINF_MIO_NUM_PRIM_ILLPOSED_CER : i32 = 10;
pub const MSK_LIINF_MIO_PRESOLVED_ANZ : i32 = 11;
pub const MSK_LIINF_MIO_SIMPLEX_ITER : i32 = 12;
pub const MSK_LIINF_RD_NUMACC : i32 = 13;
pub const MSK_LIINF_RD_NUMANZ : i32 = 14;
pub const MSK_LIINF_RD_NUMDJC : i32 = 15;
pub const MSK_LIINF_RD_NUMQNZ : i32 = 16;
pub const MSK_LIINF_SIMPLEX_ITER : i32 = 17;
pub const MSK_LIINF_END : i32 = 17;

// iinfitem
pub const MSK_IINF_ANA_PRO_NUM_CON : i32 = 0;
pub const MSK_IINF_ANA_PRO_NUM_CON_EQ : i32 = 1;
pub const MSK_IINF_ANA_PRO_NUM_CON_FR : i32 = 2;
pub const MSK_IINF_ANA_PRO_NUM_CON_LO : i32 = 3;
pub const MSK_IINF_ANA_PRO_NUM_CON_RA : i32 = 4;
pub const MSK_IINF_ANA_PRO_NUM_CON_UP : i32 = 5;
pub const MSK_IINF_ANA_PRO_NUM_VAR : i32 = 6;
pub const MSK_IINF_ANA_PRO_NUM_VAR_BIN : i32 = 7;
pub const MSK_IINF_ANA_PRO_NUM_VAR_CONT : i32 = 8;
pub const MSK_IINF_ANA_PRO_NUM_VAR_EQ : i32 = 9;
pub const MSK_IINF_ANA_PRO_NUM_VAR_FR : i32 = 10;
pub const MSK_IINF_ANA_PRO_NUM_VAR_INT : i32 = 11;
pub const MSK_IINF_ANA_PRO_NUM_VAR_LO : i32 = 12;
pub const MSK_IINF_ANA_PRO_NUM_VAR_RA : i32 = 13;
pub const MSK_IINF_ANA_PRO_NUM_VAR_UP : i32 = 14;
pub const MSK_IINF_INTPNT_FACTOR_DIM_DENSE : i32 = 15;
pub const MSK_IINF_INTPNT_ITER : i32 = 16;
pub const MSK_IINF_INTPNT_NUM_THREADS : i32 = 17;
pub const MSK_IINF_INTPNT_SOLVE_DUAL : i32 = 18;
pub const MSK_IINF_MIO_ABSGAP_SATISFIED : i32 = 19;
pub const MSK_IINF_MIO_CLIQUE_TABLE_SIZE : i32 = 20;
pub const MSK_IINF_MIO_CONSTRUCT_SOLUTION : i32 = 21;
pub const MSK_IINF_MIO_INITIAL_FEASIBLE_SOLUTION : i32 = 22;
pub const MSK_IINF_MIO_NODE_DEPTH : i32 = 23;
pub const MSK_IINF_MIO_NUM_ACTIVE_NODES : i32 = 24;
pub const MSK_IINF_MIO_NUM_BRANCH : i32 = 25;
pub const MSK_IINF_MIO_NUM_CLIQUE_CUTS : i32 = 26;
pub const MSK_IINF_MIO_NUM_CMIR_CUTS : i32 = 27;
pub const MSK_IINF_MIO_NUM_GOMORY_CUTS : i32 = 28;
pub const MSK_IINF_MIO_NUM_IMPLIED_BOUND_CUTS : i32 = 29;
pub const MSK_IINF_MIO_NUM_INT_SOLUTIONS : i32 = 30;
pub const MSK_IINF_MIO_NUM_KNAPSACK_COVER_CUTS : i32 = 31;
pub const MSK_IINF_MIO_NUM_LIPRO_CUTS : i32 = 32;
pub const MSK_IINF_MIO_NUM_RELAX : i32 = 33;
pub const MSK_IINF_MIO_NUM_REPEATED_PRESOLVE : i32 = 34;
pub const MSK_IINF_MIO_NUMBIN : i32 = 35;
pub const MSK_IINF_MIO_NUMBINCONEVAR : i32 = 36;
pub const MSK_IINF_MIO_NUMCON : i32 = 37;
pub const MSK_IINF_MIO_NUMCONE : i32 = 38;
pub const MSK_IINF_MIO_NUMCONEVAR : i32 = 39;
pub const MSK_IINF_MIO_NUMCONT : i32 = 40;
pub const MSK_IINF_MIO_NUMCONTCONEVAR : i32 = 41;
pub const MSK_IINF_MIO_NUMDEXPCONES : i32 = 42;
pub const MSK_IINF_MIO_NUMDJC : i32 = 43;
pub const MSK_IINF_MIO_NUMDPOWCONES : i32 = 44;
pub const MSK_IINF_MIO_NUMINT : i32 = 45;
pub const MSK_IINF_MIO_NUMINTCONEVAR : i32 = 46;
pub const MSK_IINF_MIO_NUMPEXPCONES : i32 = 47;
pub const MSK_IINF_MIO_NUMPPOWCONES : i32 = 48;
pub const MSK_IINF_MIO_NUMQCONES : i32 = 49;
pub const MSK_IINF_MIO_NUMRQCONES : i32 = 50;
pub const MSK_IINF_MIO_NUMVAR : i32 = 51;
pub const MSK_IINF_MIO_OBJ_BOUND_DEFINED : i32 = 52;
pub const MSK_IINF_MIO_PRESOLVED_NUMBIN : i32 = 53;
pub const MSK_IINF_MIO_PRESOLVED_NUMBINCONEVAR : i32 = 54;
pub const MSK_IINF_MIO_PRESOLVED_NUMCON : i32 = 55;
pub const MSK_IINF_MIO_PRESOLVED_NUMCONE : i32 = 56;
pub const MSK_IINF_MIO_PRESOLVED_NUMCONEVAR : i32 = 57;
pub const MSK_IINF_MIO_PRESOLVED_NUMCONT : i32 = 58;
pub const MSK_IINF_MIO_PRESOLVED_NUMCONTCONEVAR : i32 = 59;
pub const MSK_IINF_MIO_PRESOLVED_NUMDEXPCONES : i32 = 60;
pub const MSK_IINF_MIO_PRESOLVED_NUMDJC : i32 = 61;
pub const MSK_IINF_MIO_PRESOLVED_NUMDPOWCONES : i32 = 62;
pub const MSK_IINF_MIO_PRESOLVED_NUMINT : i32 = 63;
pub const MSK_IINF_MIO_PRESOLVED_NUMINTCONEVAR : i32 = 64;
pub const MSK_IINF_MIO_PRESOLVED_NUMPEXPCONES : i32 = 65;
pub const MSK_IINF_MIO_PRESOLVED_NUMPPOWCONES : i32 = 66;
pub const MSK_IINF_MIO_PRESOLVED_NUMQCONES : i32 = 67;
pub const MSK_IINF_MIO_PRESOLVED_NUMRQCONES : i32 = 68;
pub const MSK_IINF_MIO_PRESOLVED_NUMVAR : i32 = 69;
pub const MSK_IINF_MIO_RELGAP_SATISFIED : i32 = 70;
pub const MSK_IINF_MIO_TOTAL_NUM_CUTS : i32 = 71;
pub const MSK_IINF_MIO_USER_OBJ_CUT : i32 = 72;
pub const MSK_IINF_OPT_NUMCON : i32 = 73;
pub const MSK_IINF_OPT_NUMVAR : i32 = 74;
pub const MSK_IINF_OPTIMIZE_RESPONSE : i32 = 75;
pub const MSK_IINF_PRESOLVE_NUM_PRIMAL_PERTURBATIONS : i32 = 76;
pub const MSK_IINF_PURIFY_DUAL_SUCCESS : i32 = 77;
pub const MSK_IINF_PURIFY_PRIMAL_SUCCESS : i32 = 78;
pub const MSK_IINF_RD_NUMBARVAR : i32 = 79;
pub const MSK_IINF_RD_NUMCON : i32 = 80;
pub const MSK_IINF_RD_NUMCONE : i32 = 81;
pub const MSK_IINF_RD_NUMINTVAR : i32 = 82;
pub const MSK_IINF_RD_NUMQ : i32 = 83;
pub const MSK_IINF_RD_NUMVAR : i32 = 84;
pub const MSK_IINF_RD_PROTYPE : i32 = 85;
pub const MSK_IINF_SIM_DUAL_DEG_ITER : i32 = 86;
pub const MSK_IINF_SIM_DUAL_HOTSTART : i32 = 87;
pub const MSK_IINF_SIM_DUAL_HOTSTART_LU : i32 = 88;
pub const MSK_IINF_SIM_DUAL_INF_ITER : i32 = 89;
pub const MSK_IINF_SIM_DUAL_ITER : i32 = 90;
pub const MSK_IINF_SIM_NUMCON : i32 = 91;
pub const MSK_IINF_SIM_NUMVAR : i32 = 92;
pub const MSK_IINF_SIM_PRIMAL_DEG_ITER : i32 = 93;
pub const MSK_IINF_SIM_PRIMAL_HOTSTART : i32 = 94;
pub const MSK_IINF_SIM_PRIMAL_HOTSTART_LU : i32 = 95;
pub const MSK_IINF_SIM_PRIMAL_INF_ITER : i32 = 96;
pub const MSK_IINF_SIM_PRIMAL_ITER : i32 = 97;
pub const MSK_IINF_SIM_SOLVE_DUAL : i32 = 98;
pub const MSK_IINF_SOL_BAS_PROSTA : i32 = 99;
pub const MSK_IINF_SOL_BAS_SOLSTA : i32 = 100;
pub const MSK_IINF_SOL_ITG_PROSTA : i32 = 101;
pub const MSK_IINF_SOL_ITG_SOLSTA : i32 = 102;
pub const MSK_IINF_SOL_ITR_PROSTA : i32 = 103;
pub const MSK_IINF_SOL_ITR_SOLSTA : i32 = 104;
pub const MSK_IINF_STO_NUM_A_REALLOC : i32 = 105;
pub const MSK_IINF_END : i32 = 105;

// inftype
pub const MSK_INF_DOU_TYPE : i32 = 0;
pub const MSK_INF_INT_TYPE : i32 = 1;
pub const MSK_INF_LINT_TYPE : i32 = 2;

// iomode
pub const MSK_IOMODE_READ : i32 = 0;
pub const MSK_IOMODE_WRITE : i32 = 1;
pub const MSK_IOMODE_READWRITE : i32 = 2;

// iparam
pub const MSK_IPAR_ANA_SOL_BASIS : i32 = 0;
pub const MSK_IPAR_ANA_SOL_PRINT_VIOLATED : i32 = 1;
pub const MSK_IPAR_AUTO_SORT_A_BEFORE_OPT : i32 = 2;
pub const MSK_IPAR_AUTO_UPDATE_SOL_INFO : i32 = 3;
pub const MSK_IPAR_BASIS_SOLVE_USE_PLUS_ONE : i32 = 4;
pub const MSK_IPAR_BI_CLEAN_OPTIMIZER : i32 = 5;
pub const MSK_IPAR_BI_IGNORE_MAX_ITER : i32 = 6;
pub const MSK_IPAR_BI_IGNORE_NUM_ERROR : i32 = 7;
pub const MSK_IPAR_BI_MAX_ITERATIONS : i32 = 8;
pub const MSK_IPAR_CACHE_LICENSE : i32 = 9;
pub const MSK_IPAR_CHECK_CONVEXITY : i32 = 10;
pub const MSK_IPAR_COMPRESS_STATFILE : i32 = 11;
pub const MSK_IPAR_INFEAS_GENERIC_NAMES : i32 = 12;
pub const MSK_IPAR_INFEAS_PREFER_PRIMAL : i32 = 13;
pub const MSK_IPAR_INFEAS_REPORT_AUTO : i32 = 14;
pub const MSK_IPAR_INFEAS_REPORT_LEVEL : i32 = 15;
pub const MSK_IPAR_INTPNT_BASIS : i32 = 16;
pub const MSK_IPAR_INTPNT_DIFF_STEP : i32 = 17;
pub const MSK_IPAR_INTPNT_HOTSTART : i32 = 18;
pub const MSK_IPAR_INTPNT_MAX_ITERATIONS : i32 = 19;
pub const MSK_IPAR_INTPNT_MAX_NUM_COR : i32 = 20;
pub const MSK_IPAR_INTPNT_MAX_NUM_REFINEMENT_STEPS : i32 = 21;
pub const MSK_IPAR_INTPNT_OFF_COL_TRH : i32 = 22;
pub const MSK_IPAR_INTPNT_ORDER_GP_NUM_SEEDS : i32 = 23;
pub const MSK_IPAR_INTPNT_ORDER_METHOD : i32 = 24;
pub const MSK_IPAR_INTPNT_PURIFY : i32 = 25;
pub const MSK_IPAR_INTPNT_REGULARIZATION_USE : i32 = 26;
pub const MSK_IPAR_INTPNT_SCALING : i32 = 27;
pub const MSK_IPAR_INTPNT_SOLVE_FORM : i32 = 28;
pub const MSK_IPAR_INTPNT_STARTING_POINT : i32 = 29;
pub const MSK_IPAR_LICENSE_DEBUG : i32 = 30;
pub const MSK_IPAR_LICENSE_PAUSE_TIME : i32 = 31;
pub const MSK_IPAR_LICENSE_SUPPRESS_EXPIRE_WRNS : i32 = 32;
pub const MSK_IPAR_LICENSE_TRH_EXPIRY_WRN : i32 = 33;
pub const MSK_IPAR_LICENSE_WAIT : i32 = 34;
pub const MSK_IPAR_LOG : i32 = 35;
pub const MSK_IPAR_LOG_ANA_PRO : i32 = 36;
pub const MSK_IPAR_LOG_BI : i32 = 37;
pub const MSK_IPAR_LOG_BI_FREQ : i32 = 38;
pub const MSK_IPAR_LOG_CHECK_CONVEXITY : i32 = 39;
pub const MSK_IPAR_LOG_CUT_SECOND_OPT : i32 = 40;
pub const MSK_IPAR_LOG_EXPAND : i32 = 41;
pub const MSK_IPAR_LOG_FEAS_REPAIR : i32 = 42;
pub const MSK_IPAR_LOG_FILE : i32 = 43;
pub const MSK_IPAR_LOG_INCLUDE_SUMMARY : i32 = 44;
pub const MSK_IPAR_LOG_INFEAS_ANA : i32 = 45;
pub const MSK_IPAR_LOG_INTPNT : i32 = 46;
pub const MSK_IPAR_LOG_LOCAL_INFO : i32 = 47;
pub const MSK_IPAR_LOG_MIO : i32 = 48;
pub const MSK_IPAR_LOG_MIO_FREQ : i32 = 49;
pub const MSK_IPAR_LOG_ORDER : i32 = 50;
pub const MSK_IPAR_LOG_PRESOLVE : i32 = 51;
pub const MSK_IPAR_LOG_RESPONSE : i32 = 52;
pub const MSK_IPAR_LOG_SENSITIVITY : i32 = 53;
pub const MSK_IPAR_LOG_SENSITIVITY_OPT : i32 = 54;
pub const MSK_IPAR_LOG_SIM : i32 = 55;
pub const MSK_IPAR_LOG_SIM_FREQ : i32 = 56;
pub const MSK_IPAR_LOG_SIM_MINOR : i32 = 57;
pub const MSK_IPAR_LOG_STORAGE : i32 = 58;
pub const MSK_IPAR_MAX_NUM_WARNINGS : i32 = 59;
pub const MSK_IPAR_MIO_BRANCH_DIR : i32 = 60;
pub const MSK_IPAR_MIO_CONIC_OUTER_APPROXIMATION : i32 = 61;
pub const MSK_IPAR_MIO_CONSTRUCT_SOL : i32 = 62;
pub const MSK_IPAR_MIO_CUT_CLIQUE : i32 = 63;
pub const MSK_IPAR_MIO_CUT_CMIR : i32 = 64;
pub const MSK_IPAR_MIO_CUT_GMI : i32 = 65;
pub const MSK_IPAR_MIO_CUT_IMPLIED_BOUND : i32 = 66;
pub const MSK_IPAR_MIO_CUT_KNAPSACK_COVER : i32 = 67;
pub const MSK_IPAR_MIO_CUT_LIPRO : i32 = 68;
pub const MSK_IPAR_MIO_CUT_SELECTION_LEVEL : i32 = 69;
pub const MSK_IPAR_MIO_DATA_PERMUTATION_METHOD : i32 = 70;
pub const MSK_IPAR_MIO_FEASPUMP_LEVEL : i32 = 71;
pub const MSK_IPAR_MIO_HEURISTIC_LEVEL : i32 = 72;
pub const MSK_IPAR_MIO_MAX_NUM_BRANCHES : i32 = 73;
pub const MSK_IPAR_MIO_MAX_NUM_RELAXS : i32 = 74;
pub const MSK_IPAR_MIO_MAX_NUM_ROOT_CUT_ROUNDS : i32 = 75;
pub const MSK_IPAR_MIO_MAX_NUM_SOLUTIONS : i32 = 76;
pub const MSK_IPAR_MIO_MEMORY_EMPHASIS_LEVEL : i32 = 77;
pub const MSK_IPAR_MIO_MODE : i32 = 78;
pub const MSK_IPAR_MIO_NODE_OPTIMIZER : i32 = 79;
pub const MSK_IPAR_MIO_NODE_SELECTION : i32 = 80;
pub const MSK_IPAR_MIO_NUMERICAL_EMPHASIS_LEVEL : i32 = 81;
pub const MSK_IPAR_MIO_PERSPECTIVE_REFORMULATE : i32 = 82;
pub const MSK_IPAR_MIO_PRESOLVE_AGGREGATOR_USE : i32 = 83;
pub const MSK_IPAR_MIO_PROBING_LEVEL : i32 = 84;
pub const MSK_IPAR_MIO_PROPAGATE_OBJECTIVE_CONSTRAINT : i32 = 85;
pub const MSK_IPAR_MIO_QCQO_REFORMULATION_METHOD : i32 = 86;
pub const MSK_IPAR_MIO_RINS_MAX_NODES : i32 = 87;
pub const MSK_IPAR_MIO_ROOT_OPTIMIZER : i32 = 88;
pub const MSK_IPAR_MIO_ROOT_REPEAT_PRESOLVE_LEVEL : i32 = 89;
pub const MSK_IPAR_MIO_SEED : i32 = 90;
pub const MSK_IPAR_MIO_SYMMETRY_LEVEL : i32 = 91;
pub const MSK_IPAR_MIO_VB_DETECTION_LEVEL : i32 = 92;
pub const MSK_IPAR_MT_SPINCOUNT : i32 = 93;
pub const MSK_IPAR_NG : i32 = 94;
pub const MSK_IPAR_NUM_THREADS : i32 = 95;
pub const MSK_IPAR_OPF_WRITE_HEADER : i32 = 96;
pub const MSK_IPAR_OPF_WRITE_HINTS : i32 = 97;
pub const MSK_IPAR_OPF_WRITE_LINE_LENGTH : i32 = 98;
pub const MSK_IPAR_OPF_WRITE_PARAMETERS : i32 = 99;
pub const MSK_IPAR_OPF_WRITE_PROBLEM : i32 = 100;
pub const MSK_IPAR_OPF_WRITE_SOL_BAS : i32 = 101;
pub const MSK_IPAR_OPF_WRITE_SOL_ITG : i32 = 102;
pub const MSK_IPAR_OPF_WRITE_SOL_ITR : i32 = 103;
pub const MSK_IPAR_OPF_WRITE_SOLUTIONS : i32 = 104;
pub const MSK_IPAR_OPTIMIZER : i32 = 105;
pub const MSK_IPAR_PARAM_READ_CASE_NAME : i32 = 106;
pub const MSK_IPAR_PARAM_READ_IGN_ERROR : i32 = 107;
pub const MSK_IPAR_PRESOLVE_ELIMINATOR_MAX_FILL : i32 = 108;
pub const MSK_IPAR_PRESOLVE_ELIMINATOR_MAX_NUM_TRIES : i32 = 109;
pub const MSK_IPAR_PRESOLVE_LEVEL : i32 = 110;
pub const MSK_IPAR_PRESOLVE_LINDEP_ABS_WORK_TRH : i32 = 111;
pub const MSK_IPAR_PRESOLVE_LINDEP_REL_WORK_TRH : i32 = 112;
pub const MSK_IPAR_PRESOLVE_LINDEP_USE : i32 = 113;
pub const MSK_IPAR_PRESOLVE_MAX_NUM_PASS : i32 = 114;
pub const MSK_IPAR_PRESOLVE_MAX_NUM_REDUCTIONS : i32 = 115;
pub const MSK_IPAR_PRESOLVE_USE : i32 = 116;
pub const MSK_IPAR_PRIMAL_REPAIR_OPTIMIZER : i32 = 117;
pub const MSK_IPAR_PTF_WRITE_TRANSFORM : i32 = 118;
pub const MSK_IPAR_READ_DEBUG : i32 = 119;
pub const MSK_IPAR_READ_KEEP_FREE_CON : i32 = 120;
pub const MSK_IPAR_READ_LP_DROP_NEW_VARS_IN_BOU : i32 = 121;
pub const MSK_IPAR_READ_LP_QUOTED_NAMES : i32 = 122;
pub const MSK_IPAR_READ_MPS_FORMAT : i32 = 123;
pub const MSK_IPAR_READ_MPS_WIDTH : i32 = 124;
pub const MSK_IPAR_READ_TASK_IGNORE_PARAM : i32 = 125;
pub const MSK_IPAR_REMOTE_USE_COMPRESSION : i32 = 126;
pub const MSK_IPAR_REMOVE_UNUSED_SOLUTIONS : i32 = 127;
pub const MSK_IPAR_SENSITIVITY_ALL : i32 = 128;
pub const MSK_IPAR_SENSITIVITY_OPTIMIZER : i32 = 129;
pub const MSK_IPAR_SENSITIVITY_TYPE : i32 = 130;
pub const MSK_IPAR_SIM_BASIS_FACTOR_USE : i32 = 131;
pub const MSK_IPAR_SIM_DEGEN : i32 = 132;
pub const MSK_IPAR_SIM_DETECT_PWL : i32 = 133;
pub const MSK_IPAR_SIM_DUAL_CRASH : i32 = 134;
pub const MSK_IPAR_SIM_DUAL_PHASEONE_METHOD : i32 = 135;
pub const MSK_IPAR_SIM_DUAL_RESTRICT_SELECTION : i32 = 136;
pub const MSK_IPAR_SIM_DUAL_SELECTION : i32 = 137;
pub const MSK_IPAR_SIM_EXPLOIT_DUPVEC : i32 = 138;
pub const MSK_IPAR_SIM_HOTSTART : i32 = 139;
pub const MSK_IPAR_SIM_HOTSTART_LU : i32 = 140;
pub const MSK_IPAR_SIM_MAX_ITERATIONS : i32 = 141;
pub const MSK_IPAR_SIM_MAX_NUM_SETBACKS : i32 = 142;
pub const MSK_IPAR_SIM_NON_SINGULAR : i32 = 143;
pub const MSK_IPAR_SIM_PRIMAL_CRASH : i32 = 144;
pub const MSK_IPAR_SIM_PRIMAL_PHASEONE_METHOD : i32 = 145;
pub const MSK_IPAR_SIM_PRIMAL_RESTRICT_SELECTION : i32 = 146;
pub const MSK_IPAR_SIM_PRIMAL_SELECTION : i32 = 147;
pub const MSK_IPAR_SIM_REFACTOR_FREQ : i32 = 148;
pub const MSK_IPAR_SIM_REFORMULATION : i32 = 149;
pub const MSK_IPAR_SIM_SAVE_LU : i32 = 150;
pub const MSK_IPAR_SIM_SCALING : i32 = 151;
pub const MSK_IPAR_SIM_SCALING_METHOD : i32 = 152;
pub const MSK_IPAR_SIM_SEED : i32 = 153;
pub const MSK_IPAR_SIM_SOLVE_FORM : i32 = 154;
pub const MSK_IPAR_SIM_STABILITY_PRIORITY : i32 = 155;
pub const MSK_IPAR_SIM_SWITCH_OPTIMIZER : i32 = 156;
pub const MSK_IPAR_SOL_FILTER_KEEP_BASIC : i32 = 157;
pub const MSK_IPAR_SOL_FILTER_KEEP_RANGED : i32 = 158;
pub const MSK_IPAR_SOL_READ_NAME_WIDTH : i32 = 159;
pub const MSK_IPAR_SOL_READ_WIDTH : i32 = 160;
pub const MSK_IPAR_SOLUTION_CALLBACK : i32 = 161;
pub const MSK_IPAR_TIMING_LEVEL : i32 = 162;
pub const MSK_IPAR_WRITE_BAS_CONSTRAINTS : i32 = 163;
pub const MSK_IPAR_WRITE_BAS_HEAD : i32 = 164;
pub const MSK_IPAR_WRITE_BAS_VARIABLES : i32 = 165;
pub const MSK_IPAR_WRITE_COMPRESSION : i32 = 166;
pub const MSK_IPAR_WRITE_DATA_PARAM : i32 = 167;
pub const MSK_IPAR_WRITE_FREE_CON : i32 = 168;
pub const MSK_IPAR_WRITE_GENERIC_NAMES : i32 = 169;
pub const MSK_IPAR_WRITE_GENERIC_NAMES_IO : i32 = 170;
pub const MSK_IPAR_WRITE_IGNORE_INCOMPATIBLE_ITEMS : i32 = 171;
pub const MSK_IPAR_WRITE_INT_CONSTRAINTS : i32 = 172;
pub const MSK_IPAR_WRITE_INT_HEAD : i32 = 173;
pub const MSK_IPAR_WRITE_INT_VARIABLES : i32 = 174;
pub const MSK_IPAR_WRITE_JSON_INDENTATION : i32 = 175;
pub const MSK_IPAR_WRITE_LP_FULL_OBJ : i32 = 176;
pub const MSK_IPAR_WRITE_LP_LINE_WIDTH : i32 = 177;
pub const MSK_IPAR_WRITE_LP_QUOTED_NAMES : i32 = 178;
pub const MSK_IPAR_WRITE_LP_STRICT_FORMAT : i32 = 179;
pub const MSK_IPAR_WRITE_LP_TERMS_PER_LINE : i32 = 180;
pub const MSK_IPAR_WRITE_MPS_FORMAT : i32 = 181;
pub const MSK_IPAR_WRITE_MPS_INT : i32 = 182;
pub const MSK_IPAR_WRITE_SOL_BARVARIABLES : i32 = 183;
pub const MSK_IPAR_WRITE_SOL_CONSTRAINTS : i32 = 184;
pub const MSK_IPAR_WRITE_SOL_HEAD : i32 = 185;
pub const MSK_IPAR_WRITE_SOL_IGNORE_INVALID_NAMES : i32 = 186;
pub const MSK_IPAR_WRITE_SOL_VARIABLES : i32 = 187;
pub const MSK_IPAR_WRITE_TASK_INC_SOL : i32 = 188;
pub const MSK_IPAR_WRITE_XML_MODE : i32 = 189;

// branchdir
pub const MSK_BRANCH_DIR_FREE : i32 = 0;
pub const MSK_BRANCH_DIR_UP : i32 = 1;
pub const MSK_BRANCH_DIR_DOWN : i32 = 2;
pub const MSK_BRANCH_DIR_NEAR : i32 = 3;
pub const MSK_BRANCH_DIR_FAR : i32 = 4;
pub const MSK_BRANCH_DIR_ROOT_LP : i32 = 5;
pub const MSK_BRANCH_DIR_GUIDED : i32 = 6;
pub const MSK_BRANCH_DIR_PSEUDOCOST : i32 = 7;

// miqcqoreformmethod
pub const MSK_MIO_QCQO_REFORMULATION_METHOD_FREE : i32 = 0;
pub const MSK_MIO_QCQO_REFORMULATION_METHOD_NONE : i32 = 1;
pub const MSK_MIO_QCQO_REFORMULATION_METHOD_LINEARIZATION : i32 = 2;
pub const MSK_MIO_QCQO_REFORMULATION_METHOD_EIGEN_VAL_METHOD : i32 = 3;
pub const MSK_MIO_QCQO_REFORMULATION_METHOD_DIAG_SDP : i32 = 4;
pub const MSK_MIO_QCQO_REFORMULATION_METHOD_RELAX_SDP : i32 = 5;

// miodatapermmethod
pub const MSK_MIO_DATA_PERMUTATION_METHOD_NONE : i32 = 0;
pub const MSK_MIO_DATA_PERMUTATION_METHOD_CYCLIC_SHIFT : i32 = 1;
pub const MSK_MIO_DATA_PERMUTATION_METHOD_RANDOM : i32 = 2;

// miocontsoltype
pub const MSK_MIO_CONT_SOL_NONE : i32 = 0;
pub const MSK_MIO_CONT_SOL_ROOT : i32 = 1;
pub const MSK_MIO_CONT_SOL_ITG : i32 = 2;
pub const MSK_MIO_CONT_SOL_ITG_REL : i32 = 3;

// miomode
pub const MSK_MIO_MODE_IGNORED : i32 = 0;
pub const MSK_MIO_MODE_SATISFIED : i32 = 1;

// mionodeseltype
pub const MSK_MIO_NODE_SELECTION_FREE : i32 = 0;
pub const MSK_MIO_NODE_SELECTION_FIRST : i32 = 1;
pub const MSK_MIO_NODE_SELECTION_BEST : i32 = 2;
pub const MSK_MIO_NODE_SELECTION_PSEUDO : i32 = 3;

// mpsformat
pub const MSK_MPS_FORMAT_STRICT : i32 = 0;
pub const MSK_MPS_FORMAT_RELAXED : i32 = 1;
pub const MSK_MPS_FORMAT_FREE : i32 = 2;
pub const MSK_MPS_FORMAT_CPLEX : i32 = 3;

// objsense
pub const MSK_OBJECTIVE_SENSE_MINIMIZE : i32 = 0;
pub const MSK_OBJECTIVE_SENSE_MAXIMIZE : i32 = 1;

// onoffkey
pub const MSK_OFF : i32 = 0;
pub const MSK_ON : i32 = 1;

// optimizertype
pub const MSK_OPTIMIZER_CONIC : i32 = 0;
pub const MSK_OPTIMIZER_DUAL_SIMPLEX : i32 = 1;
pub const MSK_OPTIMIZER_FREE : i32 = 2;
pub const MSK_OPTIMIZER_FREE_SIMPLEX : i32 = 3;
pub const MSK_OPTIMIZER_INTPNT : i32 = 4;
pub const MSK_OPTIMIZER_MIXED_INT : i32 = 5;
pub const MSK_OPTIMIZER_PRIMAL_SIMPLEX : i32 = 6;

// orderingtype
pub const MSK_ORDER_METHOD_FREE : i32 = 0;
pub const MSK_ORDER_METHOD_APPMINLOC : i32 = 1;
pub const MSK_ORDER_METHOD_EXPERIMENTAL : i32 = 2;
pub const MSK_ORDER_METHOD_TRY_GRAPHPAR : i32 = 3;
pub const MSK_ORDER_METHOD_FORCE_GRAPHPAR : i32 = 4;
pub const MSK_ORDER_METHOD_NONE : i32 = 5;

// presolvemode
pub const MSK_PRESOLVE_MODE_OFF : i32 = 0;
pub const MSK_PRESOLVE_MODE_ON : i32 = 1;
pub const MSK_PRESOLVE_MODE_FREE : i32 = 2;

// parametertype
pub const MSK_PAR_INVALID_TYPE : i32 = 0;
pub const MSK_PAR_DOU_TYPE : i32 = 1;
pub const MSK_PAR_INT_TYPE : i32 = 2;
pub const MSK_PAR_STR_TYPE : i32 = 3;

// problemitem
pub const MSK_PI_VAR : i32 = 0;
pub const MSK_PI_CON : i32 = 1;
pub const MSK_PI_CONE : i32 = 2;

// problemtype
pub const MSK_PROBTYPE_LO : i32 = 0;
pub const MSK_PROBTYPE_QO : i32 = 1;
pub const MSK_PROBTYPE_QCQO : i32 = 2;
pub const MSK_PROBTYPE_CONIC : i32 = 3;
pub const MSK_PROBTYPE_MIXED : i32 = 4;

// prosta
pub const MSK_PRO_STA_UNKNOWN : i32 = 0;
pub const MSK_PRO_STA_PRIM_AND_DUAL_FEAS : i32 = 1;
pub const MSK_PRO_STA_PRIM_FEAS : i32 = 2;
pub const MSK_PRO_STA_DUAL_FEAS : i32 = 3;
pub const MSK_PRO_STA_PRIM_INFEAS : i32 = 4;
pub const MSK_PRO_STA_DUAL_INFEAS : i32 = 5;
pub const MSK_PRO_STA_PRIM_AND_DUAL_INFEAS : i32 = 6;
pub const MSK_PRO_STA_ILL_POSED : i32 = 7;
pub const MSK_PRO_STA_PRIM_INFEAS_OR_UNBOUNDED : i32 = 8;

// xmlwriteroutputtype
pub const MSK_WRITE_XML_MODE_ROW : i32 = 0;
pub const MSK_WRITE_XML_MODE_COL : i32 = 1;

// rescode
pub const MSK_RES_OK : i32 = 0;
pub const MSK_RES_WRN_OPEN_PARAM_FILE : i32 = 50;
pub const MSK_RES_WRN_LARGE_BOUND : i32 = 51;
pub const MSK_RES_WRN_LARGE_LO_BOUND : i32 = 52;
pub const MSK_RES_WRN_LARGE_UP_BOUND : i32 = 53;
pub const MSK_RES_WRN_LARGE_CON_FX : i32 = 54;
pub const MSK_RES_WRN_LARGE_CJ : i32 = 57;
pub const MSK_RES_WRN_LARGE_AIJ : i32 = 62;
pub const MSK_RES_WRN_ZERO_AIJ : i32 = 63;
pub const MSK_RES_WRN_NAME_MAX_LEN : i32 = 65;
pub const MSK_RES_WRN_SPAR_MAX_LEN : i32 = 66;
pub const MSK_RES_WRN_MPS_SPLIT_RHS_VECTOR : i32 = 70;
pub const MSK_RES_WRN_MPS_SPLIT_RAN_VECTOR : i32 = 71;
pub const MSK_RES_WRN_MPS_SPLIT_BOU_VECTOR : i32 = 72;
pub const MSK_RES_WRN_LP_OLD_QUAD_FORMAT : i32 = 80;
pub const MSK_RES_WRN_LP_DROP_VARIABLE : i32 = 85;
pub const MSK_RES_WRN_NZ_IN_UPR_TRI : i32 = 200;
pub const MSK_RES_WRN_DROPPED_NZ_QOBJ : i32 = 201;
pub const MSK_RES_WRN_IGNORE_INTEGER : i32 = 250;
pub const MSK_RES_WRN_NO_GLOBAL_OPTIMIZER : i32 = 251;
pub const MSK_RES_WRN_MIO_INFEASIBLE_FINAL : i32 = 270;
pub const MSK_RES_WRN_SOL_FILTER : i32 = 300;
pub const MSK_RES_WRN_UNDEF_SOL_FILE_NAME : i32 = 350;
pub const MSK_RES_WRN_SOL_FILE_IGNORED_CON : i32 = 351;
pub const MSK_RES_WRN_SOL_FILE_IGNORED_VAR : i32 = 352;
pub const MSK_RES_WRN_TOO_FEW_BASIS_VARS : i32 = 400;
pub const MSK_RES_WRN_TOO_MANY_BASIS_VARS : i32 = 405;
pub const MSK_RES_WRN_LICENSE_EXPIRE : i32 = 500;
pub const MSK_RES_WRN_LICENSE_SERVER : i32 = 501;
pub const MSK_RES_WRN_EMPTY_NAME : i32 = 502;
pub const MSK_RES_WRN_USING_GENERIC_NAMES : i32 = 503;
pub const MSK_RES_WRN_INVALID_MPS_NAME : i32 = 504;
pub const MSK_RES_WRN_INVALID_MPS_OBJ_NAME : i32 = 505;
pub const MSK_RES_WRN_LICENSE_FEATURE_EXPIRE : i32 = 509;
pub const MSK_RES_WRN_PARAM_NAME_DOU : i32 = 510;
pub const MSK_RES_WRN_PARAM_NAME_INT : i32 = 511;
pub const MSK_RES_WRN_PARAM_NAME_STR : i32 = 512;
pub const MSK_RES_WRN_PARAM_STR_VALUE : i32 = 515;
pub const MSK_RES_WRN_PARAM_IGNORED_CMIO : i32 = 516;
pub const MSK_RES_WRN_ZEROS_IN_SPARSE_ROW : i32 = 705;
pub const MSK_RES_WRN_ZEROS_IN_SPARSE_COL : i32 = 710;
pub const MSK_RES_WRN_INCOMPLETE_LINEAR_DEPENDENCY_CHECK : i32 = 800;
pub const MSK_RES_WRN_ELIMINATOR_SPACE : i32 = 801;
pub const MSK_RES_WRN_PRESOLVE_OUTOFSPACE : i32 = 802;
pub const MSK_RES_WRN_PRESOLVE_PRIMAL_PERTUBATIONS : i32 = 803;
pub const MSK_RES_WRN_WRITE_CHANGED_NAMES : i32 = 830;
pub const MSK_RES_WRN_WRITE_DISCARDED_CFIX : i32 = 831;
pub const MSK_RES_WRN_DUPLICATE_CONSTRAINT_NAMES : i32 = 850;
pub const MSK_RES_WRN_DUPLICATE_VARIABLE_NAMES : i32 = 851;
pub const MSK_RES_WRN_DUPLICATE_BARVARIABLE_NAMES : i32 = 852;
pub const MSK_RES_WRN_DUPLICATE_CONE_NAMES : i32 = 853;
pub const MSK_RES_WRN_WRITE_LP_INVALID_VAR_NAMES : i32 = 854;
pub const MSK_RES_WRN_WRITE_LP_DUPLICATE_VAR_NAMES : i32 = 855;
pub const MSK_RES_WRN_WRITE_LP_INVALID_CON_NAMES : i32 = 856;
pub const MSK_RES_WRN_WRITE_LP_DUPLICATE_CON_NAMES : i32 = 857;
pub const MSK_RES_WRN_ANA_LARGE_BOUNDS : i32 = 900;
pub const MSK_RES_WRN_ANA_C_ZERO : i32 = 901;
pub const MSK_RES_WRN_ANA_EMPTY_COLS : i32 = 902;
pub const MSK_RES_WRN_ANA_CLOSE_BOUNDS : i32 = 903;
pub const MSK_RES_WRN_ANA_ALMOST_INT_BOUNDS : i32 = 904;
pub const MSK_RES_WRN_NO_INFEASIBILITY_REPORT_WHEN_MATRIX_VARIABLES : i32 = 930;
pub const MSK_RES_WRN_NO_DUALIZER : i32 = 950;
pub const MSK_RES_WRN_SYM_MAT_LARGE : i32 = 960;
pub const MSK_RES_WRN_MODIFIED_DOUBLE_PARAMETER : i32 = 970;
pub const MSK_RES_WRN_LARGE_FIJ : i32 = 980;
pub const MSK_RES_ERR_LICENSE : i32 = 1000;
pub const MSK_RES_ERR_LICENSE_EXPIRED : i32 = 1001;
pub const MSK_RES_ERR_LICENSE_VERSION : i32 = 1002;
pub const MSK_RES_ERR_LICENSE_OLD_SERVER_VERSION : i32 = 1003;
pub const MSK_RES_ERR_SIZE_LICENSE : i32 = 1005;
pub const MSK_RES_ERR_PROB_LICENSE : i32 = 1006;
pub const MSK_RES_ERR_FILE_LICENSE : i32 = 1007;
pub const MSK_RES_ERR_MISSING_LICENSE_FILE : i32 = 1008;
pub const MSK_RES_ERR_SIZE_LICENSE_CON : i32 = 1010;
pub const MSK_RES_ERR_SIZE_LICENSE_VAR : i32 = 1011;
pub const MSK_RES_ERR_SIZE_LICENSE_INTVAR : i32 = 1012;
pub const MSK_RES_ERR_OPTIMIZER_LICENSE : i32 = 1013;
pub const MSK_RES_ERR_FLEXLM : i32 = 1014;
pub const MSK_RES_ERR_LICENSE_SERVER : i32 = 1015;
pub const MSK_RES_ERR_LICENSE_MAX : i32 = 1016;
pub const MSK_RES_ERR_LICENSE_MOSEKLM_DAEMON : i32 = 1017;
pub const MSK_RES_ERR_LICENSE_FEATURE : i32 = 1018;
pub const MSK_RES_ERR_PLATFORM_NOT_LICENSED : i32 = 1019;
pub const MSK_RES_ERR_LICENSE_CANNOT_ALLOCATE : i32 = 1020;
pub const MSK_RES_ERR_LICENSE_CANNOT_CONNECT : i32 = 1021;
pub const MSK_RES_ERR_LICENSE_INVALID_HOSTID : i32 = 1025;
pub const MSK_RES_ERR_LICENSE_SERVER_VERSION : i32 = 1026;
pub const MSK_RES_ERR_LICENSE_NO_SERVER_SUPPORT : i32 = 1027;
pub const MSK_RES_ERR_LICENSE_NO_SERVER_LINE : i32 = 1028;
pub const MSK_RES_ERR_OLDER_DLL : i32 = 1035;
pub const MSK_RES_ERR_NEWER_DLL : i32 = 1036;
pub const MSK_RES_ERR_LINK_FILE_DLL : i32 = 1040;
pub const MSK_RES_ERR_THREAD_MUTEX_INIT : i32 = 1045;
pub const MSK_RES_ERR_THREAD_MUTEX_LOCK : i32 = 1046;
pub const MSK_RES_ERR_THREAD_MUTEX_UNLOCK : i32 = 1047;
pub const MSK_RES_ERR_THREAD_CREATE : i32 = 1048;
pub const MSK_RES_ERR_THREAD_COND_INIT : i32 = 1049;
pub const MSK_RES_ERR_UNKNOWN : i32 = 1050;
pub const MSK_RES_ERR_SPACE : i32 = 1051;
pub const MSK_RES_ERR_FILE_OPEN : i32 = 1052;
pub const MSK_RES_ERR_FILE_READ : i32 = 1053;
pub const MSK_RES_ERR_FILE_WRITE : i32 = 1054;
pub const MSK_RES_ERR_DATA_FILE_EXT : i32 = 1055;
pub const MSK_RES_ERR_INVALID_FILE_NAME : i32 = 1056;
pub const MSK_RES_ERR_INVALID_SOL_FILE_NAME : i32 = 1057;
pub const MSK_RES_ERR_END_OF_FILE : i32 = 1059;
pub const MSK_RES_ERR_NULL_ENV : i32 = 1060;
pub const MSK_RES_ERR_NULL_TASK : i32 = 1061;
pub const MSK_RES_ERR_INVALID_STREAM : i32 = 1062;
pub const MSK_RES_ERR_NO_INIT_ENV : i32 = 1063;
pub const MSK_RES_ERR_INVALID_TASK : i32 = 1064;
pub const MSK_RES_ERR_NULL_POINTER : i32 = 1065;
pub const MSK_RES_ERR_LIVING_TASKS : i32 = 1066;
pub const MSK_RES_ERR_BLANK_NAME : i32 = 1070;
pub const MSK_RES_ERR_DUP_NAME : i32 = 1071;
pub const MSK_RES_ERR_FORMAT_STRING : i32 = 1072;
pub const MSK_RES_ERR_SPARSITY_SPECIFICATION : i32 = 1073;
pub const MSK_RES_ERR_MISMATCHING_DIMENSION : i32 = 1074;
pub const MSK_RES_ERR_INVALID_OBJ_NAME : i32 = 1075;
pub const MSK_RES_ERR_INVALID_CON_NAME : i32 = 1076;
pub const MSK_RES_ERR_INVALID_VAR_NAME : i32 = 1077;
pub const MSK_RES_ERR_INVALID_CONE_NAME : i32 = 1078;
pub const MSK_RES_ERR_INVALID_BARVAR_NAME : i32 = 1079;
pub const MSK_RES_ERR_SPACE_LEAKING : i32 = 1080;
pub const MSK_RES_ERR_SPACE_NO_INFO : i32 = 1081;
pub const MSK_RES_ERR_DIMENSION_SPECIFICATION : i32 = 1082;
pub const MSK_RES_ERR_AXIS_NAME_SPECIFICATION : i32 = 1083;
pub const MSK_RES_ERR_READ_FORMAT : i32 = 1090;
pub const MSK_RES_ERR_MPS_FILE : i32 = 1100;
pub const MSK_RES_ERR_MPS_INV_FIELD : i32 = 1101;
pub const MSK_RES_ERR_MPS_INV_MARKER : i32 = 1102;
pub const MSK_RES_ERR_MPS_NULL_CON_NAME : i32 = 1103;
pub const MSK_RES_ERR_MPS_NULL_VAR_NAME : i32 = 1104;
pub const MSK_RES_ERR_MPS_UNDEF_CON_NAME : i32 = 1105;
pub const MSK_RES_ERR_MPS_UNDEF_VAR_NAME : i32 = 1106;
pub const MSK_RES_ERR_MPS_INVALID_CON_KEY : i32 = 1107;
pub const MSK_RES_ERR_MPS_INVALID_BOUND_KEY : i32 = 1108;
pub const MSK_RES_ERR_MPS_INVALID_SEC_NAME : i32 = 1109;
pub const MSK_RES_ERR_MPS_NO_OBJECTIVE : i32 = 1110;
pub const MSK_RES_ERR_MPS_SPLITTED_VAR : i32 = 1111;
pub const MSK_RES_ERR_MPS_MUL_CON_NAME : i32 = 1112;
pub const MSK_RES_ERR_MPS_MUL_QSEC : i32 = 1113;
pub const MSK_RES_ERR_MPS_MUL_QOBJ : i32 = 1114;
pub const MSK_RES_ERR_MPS_INV_SEC_ORDER : i32 = 1115;
pub const MSK_RES_ERR_MPS_MUL_CSEC : i32 = 1116;
pub const MSK_RES_ERR_MPS_CONE_TYPE : i32 = 1117;
pub const MSK_RES_ERR_MPS_CONE_OVERLAP : i32 = 1118;
pub const MSK_RES_ERR_MPS_CONE_REPEAT : i32 = 1119;
pub const MSK_RES_ERR_MPS_NON_SYMMETRIC_Q : i32 = 1120;
pub const MSK_RES_ERR_MPS_DUPLICATE_Q_ELEMENT : i32 = 1121;
pub const MSK_RES_ERR_MPS_INVALID_OBJSENSE : i32 = 1122;
pub const MSK_RES_ERR_MPS_TAB_IN_FIELD2 : i32 = 1125;
pub const MSK_RES_ERR_MPS_TAB_IN_FIELD3 : i32 = 1126;
pub const MSK_RES_ERR_MPS_TAB_IN_FIELD5 : i32 = 1127;
pub const MSK_RES_ERR_MPS_INVALID_OBJ_NAME : i32 = 1128;
pub const MSK_RES_ERR_MPS_INVALID_KEY : i32 = 1129;
pub const MSK_RES_ERR_MPS_INVALID_INDICATOR_CONSTRAINT : i32 = 1130;
pub const MSK_RES_ERR_MPS_INVALID_INDICATOR_VARIABLE : i32 = 1131;
pub const MSK_RES_ERR_MPS_INVALID_INDICATOR_VALUE : i32 = 1132;
pub const MSK_RES_ERR_MPS_INVALID_INDICATOR_QUADRATIC_CONSTRAINT : i32 = 1133;
pub const MSK_RES_ERR_LP_INCOMPATIBLE : i32 = 1150;
pub const MSK_RES_ERR_LP_EMPTY : i32 = 1151;
pub const MSK_RES_ERR_LP_DUP_SLACK_NAME : i32 = 1152;
pub const MSK_RES_ERR_WRITE_MPS_INVALID_NAME : i32 = 1153;
pub const MSK_RES_ERR_LP_INVALID_VAR_NAME : i32 = 1154;
pub const MSK_RES_ERR_LP_FREE_CONSTRAINT : i32 = 1155;
pub const MSK_RES_ERR_WRITE_OPF_INVALID_VAR_NAME : i32 = 1156;
pub const MSK_RES_ERR_LP_FILE_FORMAT : i32 = 1157;
pub const MSK_RES_ERR_WRITE_LP_FORMAT : i32 = 1158;
pub const MSK_RES_ERR_READ_LP_MISSING_END_TAG : i32 = 1159;
pub const MSK_RES_ERR_LP_FORMAT : i32 = 1160;
pub const MSK_RES_ERR_WRITE_LP_NON_UNIQUE_NAME : i32 = 1161;
pub const MSK_RES_ERR_READ_LP_NONEXISTING_NAME : i32 = 1162;
pub const MSK_RES_ERR_LP_WRITE_CONIC_PROBLEM : i32 = 1163;
pub const MSK_RES_ERR_LP_WRITE_GECO_PROBLEM : i32 = 1164;
pub const MSK_RES_ERR_WRITING_FILE : i32 = 1166;
pub const MSK_RES_ERR_OPF_FORMAT : i32 = 1168;
pub const MSK_RES_ERR_OPF_NEW_VARIABLE : i32 = 1169;
pub const MSK_RES_ERR_INVALID_NAME_IN_SOL_FILE : i32 = 1170;
pub const MSK_RES_ERR_LP_INVALID_CON_NAME : i32 = 1171;
pub const MSK_RES_ERR_OPF_PREMATURE_EOF : i32 = 1172;
pub const MSK_RES_ERR_JSON_SYNTAX : i32 = 1175;
pub const MSK_RES_ERR_JSON_STRING : i32 = 1176;
pub const MSK_RES_ERR_JSON_NUMBER_OVERFLOW : i32 = 1177;
pub const MSK_RES_ERR_JSON_FORMAT : i32 = 1178;
pub const MSK_RES_ERR_JSON_DATA : i32 = 1179;
pub const MSK_RES_ERR_JSON_MISSING_DATA : i32 = 1180;
pub const MSK_RES_ERR_PTF_INCOMPATIBILITY : i32 = 1181;
pub const MSK_RES_ERR_PTF_UNDEFINED_ITEM : i32 = 1182;
pub const MSK_RES_ERR_PTF_INCONSISTENCY : i32 = 1183;
pub const MSK_RES_ERR_PTF_FORMAT : i32 = 1184;
pub const MSK_RES_ERR_ARGUMENT_LENNEQ : i32 = 1197;
pub const MSK_RES_ERR_ARGUMENT_TYPE : i32 = 1198;
pub const MSK_RES_ERR_NUM_ARGUMENTS : i32 = 1199;
pub const MSK_RES_ERR_IN_ARGUMENT : i32 = 1200;
pub const MSK_RES_ERR_ARGUMENT_DIMENSION : i32 = 1201;
pub const MSK_RES_ERR_SHAPE_IS_TOO_LARGE : i32 = 1202;
pub const MSK_RES_ERR_INDEX_IS_TOO_SMALL : i32 = 1203;
pub const MSK_RES_ERR_INDEX_IS_TOO_LARGE : i32 = 1204;
pub const MSK_RES_ERR_INDEX_IS_NOT_UNIQUE : i32 = 1205;
pub const MSK_RES_ERR_PARAM_NAME : i32 = 1206;
pub const MSK_RES_ERR_PARAM_NAME_DOU : i32 = 1207;
pub const MSK_RES_ERR_PARAM_NAME_INT : i32 = 1208;
pub const MSK_RES_ERR_PARAM_NAME_STR : i32 = 1209;
pub const MSK_RES_ERR_PARAM_INDEX : i32 = 1210;
pub const MSK_RES_ERR_PARAM_IS_TOO_LARGE : i32 = 1215;
pub const MSK_RES_ERR_PARAM_IS_TOO_SMALL : i32 = 1216;
pub const MSK_RES_ERR_PARAM_VALUE_STR : i32 = 1217;
pub const MSK_RES_ERR_PARAM_TYPE : i32 = 1218;
pub const MSK_RES_ERR_INF_DOU_INDEX : i32 = 1219;
pub const MSK_RES_ERR_INF_INT_INDEX : i32 = 1220;
pub const MSK_RES_ERR_INDEX_ARR_IS_TOO_SMALL : i32 = 1221;
pub const MSK_RES_ERR_INDEX_ARR_IS_TOO_LARGE : i32 = 1222;
pub const MSK_RES_ERR_INF_LINT_INDEX : i32 = 1225;
pub const MSK_RES_ERR_ARG_IS_TOO_SMALL : i32 = 1226;
pub const MSK_RES_ERR_ARG_IS_TOO_LARGE : i32 = 1227;
pub const MSK_RES_ERR_INVALID_WHICHSOL : i32 = 1228;
pub const MSK_RES_ERR_INF_DOU_NAME : i32 = 1230;
pub const MSK_RES_ERR_INF_INT_NAME : i32 = 1231;
pub const MSK_RES_ERR_INF_TYPE : i32 = 1232;
pub const MSK_RES_ERR_INF_LINT_NAME : i32 = 1234;
pub const MSK_RES_ERR_INDEX : i32 = 1235;
pub const MSK_RES_ERR_WHICHSOL : i32 = 1236;
pub const MSK_RES_ERR_SOLITEM : i32 = 1237;
pub const MSK_RES_ERR_WHICHITEM_NOT_ALLOWED : i32 = 1238;
pub const MSK_RES_ERR_MAXNUMCON : i32 = 1240;
pub const MSK_RES_ERR_MAXNUMVAR : i32 = 1241;
pub const MSK_RES_ERR_MAXNUMBARVAR : i32 = 1242;
pub const MSK_RES_ERR_MAXNUMQNZ : i32 = 1243;
pub const MSK_RES_ERR_TOO_SMALL_MAX_NUM_NZ : i32 = 1245;
pub const MSK_RES_ERR_INVALID_IDX : i32 = 1246;
pub const MSK_RES_ERR_INVALID_MAX_NUM : i32 = 1247;
pub const MSK_RES_ERR_UNALLOWED_WHICHSOL : i32 = 1248;
pub const MSK_RES_ERR_NUMCONLIM : i32 = 1250;
pub const MSK_RES_ERR_NUMVARLIM : i32 = 1251;
pub const MSK_RES_ERR_TOO_SMALL_MAXNUMANZ : i32 = 1252;
pub const MSK_RES_ERR_INV_APTRE : i32 = 1253;
pub const MSK_RES_ERR_MUL_A_ELEMENT : i32 = 1254;
pub const MSK_RES_ERR_INV_BK : i32 = 1255;
pub const MSK_RES_ERR_INV_BKC : i32 = 1256;
pub const MSK_RES_ERR_INV_BKX : i32 = 1257;
pub const MSK_RES_ERR_INV_VAR_TYPE : i32 = 1258;
pub const MSK_RES_ERR_SOLVER_PROBTYPE : i32 = 1259;
pub const MSK_RES_ERR_OBJECTIVE_RANGE : i32 = 1260;
pub const MSK_RES_ERR_BASIS : i32 = 1266;
pub const MSK_RES_ERR_INV_SKC : i32 = 1267;
pub const MSK_RES_ERR_INV_SKX : i32 = 1268;
pub const MSK_RES_ERR_INV_SK_STR : i32 = 1269;
pub const MSK_RES_ERR_INV_SK : i32 = 1270;
pub const MSK_RES_ERR_INV_CONE_TYPE_STR : i32 = 1271;
pub const MSK_RES_ERR_INV_CONE_TYPE : i32 = 1272;
pub const MSK_RES_ERR_INV_SKN : i32 = 1274;
pub const MSK_RES_ERR_INVALID_SURPLUS : i32 = 1275;
pub const MSK_RES_ERR_INV_NAME_ITEM : i32 = 1280;
pub const MSK_RES_ERR_PRO_ITEM : i32 = 1281;
pub const MSK_RES_ERR_INVALID_FORMAT_TYPE : i32 = 1283;
pub const MSK_RES_ERR_FIRSTI : i32 = 1285;
pub const MSK_RES_ERR_LASTI : i32 = 1286;
pub const MSK_RES_ERR_FIRSTJ : i32 = 1287;
pub const MSK_RES_ERR_LASTJ : i32 = 1288;
pub const MSK_RES_ERR_MAX_LEN_IS_TOO_SMALL : i32 = 1289;
pub const MSK_RES_ERR_NONLINEAR_EQUALITY : i32 = 1290;
pub const MSK_RES_ERR_NONCONVEX : i32 = 1291;
pub const MSK_RES_ERR_NONLINEAR_RANGED : i32 = 1292;
pub const MSK_RES_ERR_CON_Q_NOT_PSD : i32 = 1293;
pub const MSK_RES_ERR_CON_Q_NOT_NSD : i32 = 1294;
pub const MSK_RES_ERR_OBJ_Q_NOT_PSD : i32 = 1295;
pub const MSK_RES_ERR_OBJ_Q_NOT_NSD : i32 = 1296;
pub const MSK_RES_ERR_ARGUMENT_PERM_ARRAY : i32 = 1299;
pub const MSK_RES_ERR_CONE_INDEX : i32 = 1300;
pub const MSK_RES_ERR_CONE_SIZE : i32 = 1301;
pub const MSK_RES_ERR_CONE_OVERLAP : i32 = 1302;
pub const MSK_RES_ERR_CONE_REP_VAR : i32 = 1303;
pub const MSK_RES_ERR_MAXNUMCONE : i32 = 1304;
pub const MSK_RES_ERR_CONE_TYPE : i32 = 1305;
pub const MSK_RES_ERR_CONE_TYPE_STR : i32 = 1306;
pub const MSK_RES_ERR_CONE_OVERLAP_APPEND : i32 = 1307;
pub const MSK_RES_ERR_REMOVE_CONE_VARIABLE : i32 = 1310;
pub const MSK_RES_ERR_APPENDING_TOO_BIG_CONE : i32 = 1311;
pub const MSK_RES_ERR_CONE_PARAMETER : i32 = 1320;
pub const MSK_RES_ERR_SOL_FILE_INVALID_NUMBER : i32 = 1350;
pub const MSK_RES_ERR_HUGE_C : i32 = 1375;
pub const MSK_RES_ERR_HUGE_AIJ : i32 = 1380;
pub const MSK_RES_ERR_DUPLICATE_AIJ : i32 = 1385;
pub const MSK_RES_ERR_LOWER_BOUND_IS_A_NAN : i32 = 1390;
pub const MSK_RES_ERR_UPPER_BOUND_IS_A_NAN : i32 = 1391;
pub const MSK_RES_ERR_INFINITE_BOUND : i32 = 1400;
pub const MSK_RES_ERR_INV_QOBJ_SUBI : i32 = 1401;
pub const MSK_RES_ERR_INV_QOBJ_SUBJ : i32 = 1402;
pub const MSK_RES_ERR_INV_QOBJ_VAL : i32 = 1403;
pub const MSK_RES_ERR_INV_QCON_SUBK : i32 = 1404;
pub const MSK_RES_ERR_INV_QCON_SUBI : i32 = 1405;
pub const MSK_RES_ERR_INV_QCON_SUBJ : i32 = 1406;
pub const MSK_RES_ERR_INV_QCON_VAL : i32 = 1407;
pub const MSK_RES_ERR_QCON_SUBI_TOO_SMALL : i32 = 1408;
pub const MSK_RES_ERR_QCON_SUBI_TOO_LARGE : i32 = 1409;
pub const MSK_RES_ERR_QOBJ_UPPER_TRIANGLE : i32 = 1415;
pub const MSK_RES_ERR_QCON_UPPER_TRIANGLE : i32 = 1417;
pub const MSK_RES_ERR_FIXED_BOUND_VALUES : i32 = 1420;
pub const MSK_RES_ERR_TOO_SMALL_A_TRUNCATION_VALUE : i32 = 1421;
pub const MSK_RES_ERR_INVALID_OBJECTIVE_SENSE : i32 = 1445;
pub const MSK_RES_ERR_UNDEFINED_OBJECTIVE_SENSE : i32 = 1446;
pub const MSK_RES_ERR_Y_IS_UNDEFINED : i32 = 1449;
pub const MSK_RES_ERR_NAN_IN_DOUBLE_DATA : i32 = 1450;
pub const MSK_RES_ERR_INF_IN_DOUBLE_DATA : i32 = 1451;
pub const MSK_RES_ERR_NAN_IN_BLC : i32 = 1461;
pub const MSK_RES_ERR_NAN_IN_BUC : i32 = 1462;
pub const MSK_RES_ERR_INVALID_CFIX : i32 = 1469;
pub const MSK_RES_ERR_NAN_IN_C : i32 = 1470;
pub const MSK_RES_ERR_NAN_IN_BLX : i32 = 1471;
pub const MSK_RES_ERR_NAN_IN_BUX : i32 = 1472;
pub const MSK_RES_ERR_INVALID_AIJ : i32 = 1473;
pub const MSK_RES_ERR_INVALID_CJ : i32 = 1474;
pub const MSK_RES_ERR_SYM_MAT_INVALID : i32 = 1480;
pub const MSK_RES_ERR_SYM_MAT_HUGE : i32 = 1482;
pub const MSK_RES_ERR_INV_PROBLEM : i32 = 1500;
pub const MSK_RES_ERR_MIXED_CONIC_AND_NL : i32 = 1501;
pub const MSK_RES_ERR_GLOBAL_INV_CONIC_PROBLEM : i32 = 1503;
pub const MSK_RES_ERR_INV_OPTIMIZER : i32 = 1550;
pub const MSK_RES_ERR_MIO_NO_OPTIMIZER : i32 = 1551;
pub const MSK_RES_ERR_NO_OPTIMIZER_VAR_TYPE : i32 = 1552;
pub const MSK_RES_ERR_FINAL_SOLUTION : i32 = 1560;
pub const MSK_RES_ERR_FIRST : i32 = 1570;
pub const MSK_RES_ERR_LAST : i32 = 1571;
pub const MSK_RES_ERR_SLICE_SIZE : i32 = 1572;
pub const MSK_RES_ERR_NEGATIVE_SURPLUS : i32 = 1573;
pub const MSK_RES_ERR_NEGATIVE_APPEND : i32 = 1578;
pub const MSK_RES_ERR_POSTSOLVE : i32 = 1580;
pub const MSK_RES_ERR_OVERFLOW : i32 = 1590;
pub const MSK_RES_ERR_NO_BASIS_SOL : i32 = 1600;
pub const MSK_RES_ERR_BASIS_FACTOR : i32 = 1610;
pub const MSK_RES_ERR_BASIS_SINGULAR : i32 = 1615;
pub const MSK_RES_ERR_FACTOR : i32 = 1650;
pub const MSK_RES_ERR_FEASREPAIR_CANNOT_RELAX : i32 = 1700;
pub const MSK_RES_ERR_FEASREPAIR_SOLVING_RELAXED : i32 = 1701;
pub const MSK_RES_ERR_FEASREPAIR_INCONSISTENT_BOUND : i32 = 1702;
pub const MSK_RES_ERR_REPAIR_INVALID_PROBLEM : i32 = 1710;
pub const MSK_RES_ERR_REPAIR_OPTIMIZATION_FAILED : i32 = 1711;
pub const MSK_RES_ERR_NAME_MAX_LEN : i32 = 1750;
pub const MSK_RES_ERR_NAME_IS_NULL : i32 = 1760;
pub const MSK_RES_ERR_INVALID_COMPRESSION : i32 = 1800;
pub const MSK_RES_ERR_INVALID_IOMODE : i32 = 1801;
pub const MSK_RES_ERR_NO_PRIMAL_INFEAS_CER : i32 = 2000;
pub const MSK_RES_ERR_NO_DUAL_INFEAS_CER : i32 = 2001;
pub const MSK_RES_ERR_NO_SOLUTION_IN_CALLBACK : i32 = 2500;
pub const MSK_RES_ERR_INV_MARKI : i32 = 2501;
pub const MSK_RES_ERR_INV_MARKJ : i32 = 2502;
pub const MSK_RES_ERR_INV_NUMI : i32 = 2503;
pub const MSK_RES_ERR_INV_NUMJ : i32 = 2504;
pub const MSK_RES_ERR_TASK_INCOMPATIBLE : i32 = 2560;
pub const MSK_RES_ERR_TASK_INVALID : i32 = 2561;
pub const MSK_RES_ERR_TASK_WRITE : i32 = 2562;
pub const MSK_RES_ERR_LU_MAX_NUM_TRIES : i32 = 2800;
pub const MSK_RES_ERR_INVALID_UTF8 : i32 = 2900;
pub const MSK_RES_ERR_INVALID_WCHAR : i32 = 2901;
pub const MSK_RES_ERR_NO_DUAL_FOR_ITG_SOL : i32 = 2950;
pub const MSK_RES_ERR_NO_SNX_FOR_BAS_SOL : i32 = 2953;
pub const MSK_RES_ERR_INTERNAL : i32 = 3000;
pub const MSK_RES_ERR_API_ARRAY_TOO_SMALL : i32 = 3001;
pub const MSK_RES_ERR_API_CB_CONNECT : i32 = 3002;
pub const MSK_RES_ERR_API_FATAL_ERROR : i32 = 3005;
pub const MSK_RES_ERR_SEN_FORMAT : i32 = 3050;
pub const MSK_RES_ERR_SEN_UNDEF_NAME : i32 = 3051;
pub const MSK_RES_ERR_SEN_INDEX_RANGE : i32 = 3052;
pub const MSK_RES_ERR_SEN_BOUND_INVALID_UP : i32 = 3053;
pub const MSK_RES_ERR_SEN_BOUND_INVALID_LO : i32 = 3054;
pub const MSK_RES_ERR_SEN_INDEX_INVALID : i32 = 3055;
pub const MSK_RES_ERR_SEN_INVALID_REGEXP : i32 = 3056;
pub const MSK_RES_ERR_SEN_SOLUTION_STATUS : i32 = 3057;
pub const MSK_RES_ERR_SEN_NUMERICAL : i32 = 3058;
pub const MSK_RES_ERR_SEN_UNHANDLED_PROBLEM_TYPE : i32 = 3080;
pub const MSK_RES_ERR_UNB_STEP_SIZE : i32 = 3100;
pub const MSK_RES_ERR_IDENTICAL_TASKS : i32 = 3101;
pub const MSK_RES_ERR_AD_INVALID_CODELIST : i32 = 3102;
pub const MSK_RES_ERR_INTERNAL_TEST_FAILED : i32 = 3500;
pub const MSK_RES_ERR_XML_INVALID_PROBLEM_TYPE : i32 = 3600;
pub const MSK_RES_ERR_INVALID_AMPL_STUB : i32 = 3700;
pub const MSK_RES_ERR_INT64_TO_INT32_CAST : i32 = 3800;
pub const MSK_RES_ERR_SIZE_LICENSE_NUMCORES : i32 = 3900;
pub const MSK_RES_ERR_INFEAS_UNDEFINED : i32 = 3910;
pub const MSK_RES_ERR_NO_BARX_FOR_SOLUTION : i32 = 3915;
pub const MSK_RES_ERR_NO_BARS_FOR_SOLUTION : i32 = 3916;
pub const MSK_RES_ERR_BAR_VAR_DIM : i32 = 3920;
pub const MSK_RES_ERR_SYM_MAT_INVALID_ROW_INDEX : i32 = 3940;
pub const MSK_RES_ERR_SYM_MAT_INVALID_COL_INDEX : i32 = 3941;
pub const MSK_RES_ERR_SYM_MAT_NOT_LOWER_TRINGULAR : i32 = 3942;
pub const MSK_RES_ERR_SYM_MAT_INVALID_VALUE : i32 = 3943;
pub const MSK_RES_ERR_SYM_MAT_DUPLICATE : i32 = 3944;
pub const MSK_RES_ERR_INVALID_SYM_MAT_DIM : i32 = 3950;
pub const MSK_RES_ERR_API_INTERNAL : i32 = 3999;
pub const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_SYM_MAT : i32 = 4000;
pub const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_CFIX : i32 = 4001;
pub const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_RANGED_CONSTRAINTS : i32 = 4002;
pub const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_FREE_CONSTRAINTS : i32 = 4003;
pub const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_CONES : i32 = 4005;
pub const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_QUADRATIC_TERMS : i32 = 4006;
pub const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_NONLINEAR : i32 = 4010;
pub const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_DISJUNCTIVE_CONSTRAINTS : i32 = 4011;
pub const MSK_RES_ERR_INVALID_FILE_FORMAT_FOR_AFFINE_CONIC_CONSTRAINTS : i32 = 4012;
pub const MSK_RES_ERR_DUPLICATE_CONSTRAINT_NAMES : i32 = 4500;
pub const MSK_RES_ERR_DUPLICATE_VARIABLE_NAMES : i32 = 4501;
pub const MSK_RES_ERR_DUPLICATE_BARVARIABLE_NAMES : i32 = 4502;
pub const MSK_RES_ERR_DUPLICATE_CONE_NAMES : i32 = 4503;
pub const MSK_RES_ERR_DUPLICATE_DOMAIN_NAMES : i32 = 4504;
pub const MSK_RES_ERR_DUPLICATE_DJC_NAMES : i32 = 4505;
pub const MSK_RES_ERR_NON_UNIQUE_ARRAY : i32 = 5000;
pub const MSK_RES_ERR_ARGUMENT_IS_TOO_SMALL : i32 = 5004;
pub const MSK_RES_ERR_ARGUMENT_IS_TOO_LARGE : i32 = 5005;
pub const MSK_RES_ERR_MIO_INTERNAL : i32 = 5010;
pub const MSK_RES_ERR_INVALID_PROBLEM_TYPE : i32 = 6000;
pub const MSK_RES_ERR_UNHANDLED_SOLUTION_STATUS : i32 = 6010;
pub const MSK_RES_ERR_UPPER_TRIANGLE : i32 = 6020;
pub const MSK_RES_ERR_LAU_SINGULAR_MATRIX : i32 = 7000;
pub const MSK_RES_ERR_LAU_NOT_POSITIVE_DEFINITE : i32 = 7001;
pub const MSK_RES_ERR_LAU_INVALID_LOWER_TRIANGULAR_MATRIX : i32 = 7002;
pub const MSK_RES_ERR_LAU_UNKNOWN : i32 = 7005;
pub const MSK_RES_ERR_LAU_ARG_M : i32 = 7010;
pub const MSK_RES_ERR_LAU_ARG_N : i32 = 7011;
pub const MSK_RES_ERR_LAU_ARG_K : i32 = 7012;
pub const MSK_RES_ERR_LAU_ARG_TRANSA : i32 = 7015;
pub const MSK_RES_ERR_LAU_ARG_TRANSB : i32 = 7016;
pub const MSK_RES_ERR_LAU_ARG_UPLO : i32 = 7017;
pub const MSK_RES_ERR_LAU_ARG_TRANS : i32 = 7018;
pub const MSK_RES_ERR_LAU_INVALID_SPARSE_SYMMETRIC_MATRIX : i32 = 7019;
pub const MSK_RES_ERR_CBF_PARSE : i32 = 7100;
pub const MSK_RES_ERR_CBF_OBJ_SENSE : i32 = 7101;
pub const MSK_RES_ERR_CBF_NO_VARIABLES : i32 = 7102;
pub const MSK_RES_ERR_CBF_TOO_MANY_CONSTRAINTS : i32 = 7103;
pub const MSK_RES_ERR_CBF_TOO_MANY_VARIABLES : i32 = 7104;
pub const MSK_RES_ERR_CBF_NO_VERSION_SPECIFIED : i32 = 7105;
pub const MSK_RES_ERR_CBF_SYNTAX : i32 = 7106;
pub const MSK_RES_ERR_CBF_DUPLICATE_OBJ : i32 = 7107;
pub const MSK_RES_ERR_CBF_DUPLICATE_CON : i32 = 7108;
pub const MSK_RES_ERR_CBF_DUPLICATE_VAR : i32 = 7110;
pub const MSK_RES_ERR_CBF_DUPLICATE_INT : i32 = 7111;
pub const MSK_RES_ERR_CBF_INVALID_VAR_TYPE : i32 = 7112;
pub const MSK_RES_ERR_CBF_INVALID_CON_TYPE : i32 = 7113;
pub const MSK_RES_ERR_CBF_INVALID_DOMAIN_DIMENSION : i32 = 7114;
pub const MSK_RES_ERR_CBF_DUPLICATE_OBJACOORD : i32 = 7115;
pub const MSK_RES_ERR_CBF_DUPLICATE_BCOORD : i32 = 7116;
pub const MSK_RES_ERR_CBF_DUPLICATE_ACOORD : i32 = 7117;
pub const MSK_RES_ERR_CBF_TOO_FEW_VARIABLES : i32 = 7118;
pub const MSK_RES_ERR_CBF_TOO_FEW_CONSTRAINTS : i32 = 7119;
pub const MSK_RES_ERR_CBF_TOO_FEW_INTS : i32 = 7120;
pub const MSK_RES_ERR_CBF_TOO_MANY_INTS : i32 = 7121;
pub const MSK_RES_ERR_CBF_INVALID_INT_INDEX : i32 = 7122;
pub const MSK_RES_ERR_CBF_UNSUPPORTED : i32 = 7123;
pub const MSK_RES_ERR_CBF_DUPLICATE_PSDVAR : i32 = 7124;
pub const MSK_RES_ERR_CBF_INVALID_PSDVAR_DIMENSION : i32 = 7125;
pub const MSK_RES_ERR_CBF_TOO_FEW_PSDVAR : i32 = 7126;
pub const MSK_RES_ERR_CBF_INVALID_EXP_DIMENSION : i32 = 7127;
pub const MSK_RES_ERR_CBF_DUPLICATE_POW_CONES : i32 = 7130;
pub const MSK_RES_ERR_CBF_DUPLICATE_POW_STAR_CONES : i32 = 7131;
pub const MSK_RES_ERR_CBF_INVALID_POWER : i32 = 7132;
pub const MSK_RES_ERR_CBF_POWER_CONE_IS_TOO_LONG : i32 = 7133;
pub const MSK_RES_ERR_CBF_INVALID_POWER_CONE_INDEX : i32 = 7134;
pub const MSK_RES_ERR_CBF_INVALID_POWER_STAR_CONE_INDEX : i32 = 7135;
pub const MSK_RES_ERR_CBF_UNHANDLED_POWER_CONE_TYPE : i32 = 7136;
pub const MSK_RES_ERR_CBF_UNHANDLED_POWER_STAR_CONE_TYPE : i32 = 7137;
pub const MSK_RES_ERR_CBF_POWER_CONE_MISMATCH : i32 = 7138;
pub const MSK_RES_ERR_CBF_POWER_STAR_CONE_MISMATCH : i32 = 7139;
pub const MSK_RES_ERR_CBF_INVALID_NUMBER_OF_CONES : i32 = 7140;
pub const MSK_RES_ERR_CBF_INVALID_DIMENSION_OF_CONES : i32 = 7141;
pub const MSK_RES_ERR_CBF_INVALID_NUM_PSDCON : i32 = 7200;
pub const MSK_RES_ERR_CBF_DUPLICATE_PSDCON : i32 = 7201;
pub const MSK_RES_ERR_CBF_INVALID_DIMENSION_OF_PSDCON : i32 = 7202;
pub const MSK_RES_ERR_CBF_INVALID_PSDCON_INDEX : i32 = 7203;
pub const MSK_RES_ERR_CBF_INVALID_PSDCON_VARIABLE_INDEX : i32 = 7204;
pub const MSK_RES_ERR_CBF_INVALID_PSDCON_BLOCK_INDEX : i32 = 7205;
pub const MSK_RES_ERR_CBF_UNSUPPORTED_CHANGE : i32 = 7210;
pub const MSK_RES_ERR_MIO_INVALID_ROOT_OPTIMIZER : i32 = 7700;
pub const MSK_RES_ERR_MIO_INVALID_NODE_OPTIMIZER : i32 = 7701;
pub const MSK_RES_ERR_MPS_WRITE_CPLEX_INVALID_CONE_TYPE : i32 = 7750;
pub const MSK_RES_ERR_TOCONIC_CONSTR_Q_NOT_PSD : i32 = 7800;
pub const MSK_RES_ERR_TOCONIC_CONSTRAINT_FX : i32 = 7801;
pub const MSK_RES_ERR_TOCONIC_CONSTRAINT_RA : i32 = 7802;
pub const MSK_RES_ERR_TOCONIC_CONSTR_NOT_CONIC : i32 = 7803;
pub const MSK_RES_ERR_TOCONIC_OBJECTIVE_NOT_PSD : i32 = 7804;
pub const MSK_RES_ERR_SERVER_CONNECT : i32 = 8000;
pub const MSK_RES_ERR_SERVER_PROTOCOL : i32 = 8001;
pub const MSK_RES_ERR_SERVER_STATUS : i32 = 8002;
pub const MSK_RES_ERR_SERVER_TOKEN : i32 = 8003;
pub const MSK_RES_ERR_SERVER_ADDRESS : i32 = 8004;
pub const MSK_RES_ERR_SERVER_CERTIFICATE : i32 = 8005;
pub const MSK_RES_ERR_SERVER_TLS_CLIENT : i32 = 8006;
pub const MSK_RES_ERR_SERVER_ACCESS_TOKEN : i32 = 8007;
pub const MSK_RES_ERR_SERVER_PROBLEM_SIZE : i32 = 8008;
pub const MSK_RES_ERR_DUPLICATE_FIJ : i32 = 20100;
pub const MSK_RES_ERR_INVALID_FIJ : i32 = 20101;
pub const MSK_RES_ERR_HUGE_FIJ : i32 = 20102;
pub const MSK_RES_ERR_INVALID_G : i32 = 20103;
pub const MSK_RES_ERR_INVALID_B : i32 = 20150;
pub const MSK_RES_ERR_DOMAIN_INVALID_INDEX : i32 = 20400;
pub const MSK_RES_ERR_DOMAIN_DIMENSION : i32 = 20401;
pub const MSK_RES_ERR_DOMAIN_DIMENSION_PSD : i32 = 20402;
pub const MSK_RES_ERR_NOT_POWER_DOMAIN : i32 = 20403;
pub const MSK_RES_ERR_DOMAIN_POWER_INVALID_ALPHA : i32 = 20404;
pub const MSK_RES_ERR_DOMAIN_POWER_NEGATIVE_ALPHA : i32 = 20405;
pub const MSK_RES_ERR_DOMAIN_POWER_NLEFT : i32 = 20406;
pub const MSK_RES_ERR_AFE_INVALID_INDEX : i32 = 20500;
pub const MSK_RES_ERR_ACC_INVALID_INDEX : i32 = 20600;
pub const MSK_RES_ERR_ACC_INVALID_ENTRY_INDEX : i32 = 20601;
pub const MSK_RES_ERR_ACC_AFE_DOMAIN_MISMATCH : i32 = 20602;
pub const MSK_RES_ERR_DJC_INVALID_INDEX : i32 = 20700;
pub const MSK_RES_ERR_DJC_UNSUPPORTED_DOMAIN_TYPE : i32 = 20701;
pub const MSK_RES_ERR_DJC_AFE_DOMAIN_MISMATCH : i32 = 20702;
pub const MSK_RES_ERR_DJC_INVALID_TERM_SIZE : i32 = 20703;
pub const MSK_RES_ERR_DJC_DOMAIN_TERMSIZE_MISMATCH : i32 = 20704;
pub const MSK_RES_ERR_DJC_TOTAL_NUM_TERMS_MISMATCH : i32 = 20705;
pub const MSK_RES_ERR_UNDEF_SOLUTION : i32 = 22000;
pub const MSK_RES_ERR_NO_DOTY : i32 = 22010;
pub const MSK_RES_TRM_MAX_ITERATIONS : i32 = 100000;
pub const MSK_RES_TRM_MAX_TIME : i32 = 100001;
pub const MSK_RES_TRM_OBJECTIVE_RANGE : i32 = 100002;
pub const MSK_RES_TRM_STALL : i32 = 100006;
pub const MSK_RES_TRM_USER_CALLBACK : i32 = 100007;
pub const MSK_RES_TRM_MIO_NUM_RELAXS : i32 = 100008;
pub const MSK_RES_TRM_MIO_NUM_BRANCHES : i32 = 100009;
pub const MSK_RES_TRM_NUM_MAX_NUM_INT_SOLUTIONS : i32 = 100015;
pub const MSK_RES_TRM_MAX_NUM_SETBACKS : i32 = 100020;
pub const MSK_RES_TRM_NUMERICAL_PROBLEM : i32 = 100025;
pub const MSK_RES_TRM_LOST_RACE : i32 = 100027;
pub const MSK_RES_TRM_INTERNAL : i32 = 100030;
pub const MSK_RES_TRM_INTERNAL_STOP : i32 = 100031;

// rescodetype
pub const MSK_RESPONSE_OK : i32 = 0;
pub const MSK_RESPONSE_WRN : i32 = 1;
pub const MSK_RESPONSE_TRM : i32 = 2;
pub const MSK_RESPONSE_ERR : i32 = 3;
pub const MSK_RESPONSE_UNK : i32 = 4;

// scalingtype
pub const MSK_SCALING_FREE : i32 = 0;
pub const MSK_SCALING_NONE : i32 = 1;

// scalingmethod
pub const MSK_SCALING_METHOD_POW2 : i32 = 0;
pub const MSK_SCALING_METHOD_FREE : i32 = 1;

// sensitivitytype
pub const MSK_SENSITIVITY_TYPE_BASIS : i32 = 0;

// simseltype
pub const MSK_SIM_SELECTION_FREE : i32 = 0;
pub const MSK_SIM_SELECTION_FULL : i32 = 1;
pub const MSK_SIM_SELECTION_ASE : i32 = 2;
pub const MSK_SIM_SELECTION_DEVEX : i32 = 3;
pub const MSK_SIM_SELECTION_SE : i32 = 4;
pub const MSK_SIM_SELECTION_PARTIAL : i32 = 5;

// solitem
pub const MSK_SOL_ITEM_XC : i32 = 0;
pub const MSK_SOL_ITEM_XX : i32 = 1;
pub const MSK_SOL_ITEM_Y : i32 = 2;
pub const MSK_SOL_ITEM_SLC : i32 = 3;
pub const MSK_SOL_ITEM_SUC : i32 = 4;
pub const MSK_SOL_ITEM_SLX : i32 = 5;
pub const MSK_SOL_ITEM_SUX : i32 = 6;
pub const MSK_SOL_ITEM_SNX : i32 = 7;

// solsta
pub const MSK_SOL_STA_UNKNOWN : i32 = 0;
pub const MSK_SOL_STA_OPTIMAL : i32 = 1;
pub const MSK_SOL_STA_PRIM_FEAS : i32 = 2;
pub const MSK_SOL_STA_DUAL_FEAS : i32 = 3;
pub const MSK_SOL_STA_PRIM_AND_DUAL_FEAS : i32 = 4;
pub const MSK_SOL_STA_PRIM_INFEAS_CER : i32 = 5;
pub const MSK_SOL_STA_DUAL_INFEAS_CER : i32 = 6;
pub const MSK_SOL_STA_PRIM_ILLPOSED_CER : i32 = 7;
pub const MSK_SOL_STA_DUAL_ILLPOSED_CER : i32 = 8;
pub const MSK_SOL_STA_INTEGER_OPTIMAL : i32 = 9;

// soltype
pub const MSK_SOL_ITR : i32 = 0;
pub const MSK_SOL_BAS : i32 = 1;
pub const MSK_SOL_ITG : i32 = 2;

// solveform
pub const MSK_SOLVE_FREE : i32 = 0;
pub const MSK_SOLVE_PRIMAL : i32 = 1;
pub const MSK_SOLVE_DUAL : i32 = 2;

// sparam
pub const MSK_SPAR_BAS_SOL_FILE_NAME : i32 = 0;
pub const MSK_SPAR_DATA_FILE_NAME : i32 = 1;
pub const MSK_SPAR_DEBUG_FILE_NAME : i32 = 2;
pub const MSK_SPAR_INT_SOL_FILE_NAME : i32 = 3;
pub const MSK_SPAR_ITR_SOL_FILE_NAME : i32 = 4;
pub const MSK_SPAR_MIO_DEBUG_STRING : i32 = 5;
pub const MSK_SPAR_PARAM_COMMENT_SIGN : i32 = 6;
pub const MSK_SPAR_PARAM_READ_FILE_NAME : i32 = 7;
pub const MSK_SPAR_PARAM_WRITE_FILE_NAME : i32 = 8;
pub const MSK_SPAR_READ_MPS_BOU_NAME : i32 = 9;
pub const MSK_SPAR_READ_MPS_OBJ_NAME : i32 = 10;
pub const MSK_SPAR_READ_MPS_RAN_NAME : i32 = 11;
pub const MSK_SPAR_READ_MPS_RHS_NAME : i32 = 12;
pub const MSK_SPAR_REMOTE_OPTSERVER_HOST : i32 = 13;
pub const MSK_SPAR_REMOTE_TLS_CERT : i32 = 14;
pub const MSK_SPAR_REMOTE_TLS_CERT_PATH : i32 = 15;
pub const MSK_SPAR_SENSITIVITY_FILE_NAME : i32 = 16;
pub const MSK_SPAR_SENSITIVITY_RES_FILE_NAME : i32 = 17;
pub const MSK_SPAR_SOL_FILTER_XC_LOW : i32 = 18;
pub const MSK_SPAR_SOL_FILTER_XC_UPR : i32 = 19;
pub const MSK_SPAR_SOL_FILTER_XX_LOW : i32 = 20;
pub const MSK_SPAR_SOL_FILTER_XX_UPR : i32 = 21;
pub const MSK_SPAR_STAT_KEY : i32 = 22;
pub const MSK_SPAR_STAT_NAME : i32 = 23;
pub const MSK_SPAR_WRITE_LP_GEN_VAR_NAME : i32 = 24;

// stakey
pub const MSK_SK_UNK : i32 = 0;
pub const MSK_SK_BAS : i32 = 1;
pub const MSK_SK_SUPBAS : i32 = 2;
pub const MSK_SK_LOW : i32 = 3;
pub const MSK_SK_UPR : i32 = 4;
pub const MSK_SK_FIX : i32 = 5;
pub const MSK_SK_INF : i32 = 6;

// startpointtype
pub const MSK_STARTING_POINT_FREE : i32 = 0;
pub const MSK_STARTING_POINT_GUESS : i32 = 1;
pub const MSK_STARTING_POINT_CONSTANT : i32 = 2;
pub const MSK_STARTING_POINT_SATISFY_BOUNDS : i32 = 3;

// streamtype
pub const MSK_STREAM_LOG : i32 = 0;
pub const MSK_STREAM_MSG : i32 = 1;
pub const MSK_STREAM_ERR : i32 = 2;
pub const MSK_STREAM_WRN : i32 = 3;

// value
pub const MSK_LICENSE_BUFFER_LENGTH : i32 = 21;
pub const MSK_MAX_STR_LEN : i32 = 1024;

// variabletype
pub const MSK_VAR_TYPE_CONT : i32 = 0;
pub const MSK_VAR_TYPE_INT : i32 = 1;


#[allow(unused_parens)]
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
        let dbgfile_cstr = CString::new(dbgfile).unwrap();
        let res = unsafe { MSK_makeenv(& mut env, dbgfile_cstr.as_ptr()) };
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

    #[allow(unused_parens)]
    pub fn axpy(&self,n_ : i32,alpha_ : f64,x_ : &[f64],y_ : &mut[f64]) -> Result<(),String> {
      if x_.len() != (n_).try_into().unwrap() {
        return Result::Err("Argument 'x' has the wrong length, expected n_".to_string());
      }
      if y_.len() != (n_).try_into().unwrap() {
        return Result::Err("Argument 'y' has the wrong length, expected n_".to_string());
      }
      self.handle_res(unsafe { MSK_axpy(self.ptr,n_,alpha_,x_.as_ptr(),y_.as_mut_ptr()) },"axpy")?;
      return Result::Ok(());
    } // axpy
    #[allow(unused_parens)]
    pub fn check_in_all(&mut self) -> Result<(),String> {
      self.handle_res(unsafe { MSK_checkinall(self.ptr) },"check_in_all")?;
      return Result::Ok(());
    } // checkinall
    #[allow(unused_parens)]
    pub fn check_in_license(&mut self,feature_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_checkinlicense(self.ptr,feature_) },"check_in_license")?;
      return Result::Ok(());
    } // checkinlicense
    #[allow(unused_parens)]
    pub fn checkout_license(&mut self,feature_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_checkoutlicense(self.ptr,feature_) },"checkout_license")?;
      return Result::Ok(());
    } // checkoutlicense
    #[allow(unused_parens)]
    pub fn check_version(&self,major_ : i32,minor_ : i32,revision_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_checkversion(self.ptr,major_,minor_,revision_) },"check_version")?;
      return Result::Ok(());
    } // checkversion
    #[allow(unused_parens)]
    pub fn dot(&self,n_ : i32,x_ : &[f64],y_ : &[f64],xty_ : &mut f64) -> Result<(),String> {
      if x_.len() != (n_).try_into().unwrap() {
        return Result::Err("Argument 'x' has the wrong length, expected n_".to_string());
      }
      if y_.len() != (n_).try_into().unwrap() {
        return Result::Err("Argument 'y' has the wrong length, expected n_".to_string());
      }
      self.handle_res(unsafe { MSK_dot(self.ptr,n_,x_.as_ptr(),y_.as_ptr(),xty_) },"dot")?;
      return Result::Ok(());
    } // dot
    #[allow(unused_parens)]
    pub fn echo_intro(&self,longver_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_echointro(self.ptr,longver_) },"echo_intro")?;
      return Result::Ok(());
    } // echointro
    #[allow(unused_parens)]
    pub fn enable_gar_col_env(&mut self) -> Result<(),String> {
      self.handle_res(unsafe { MSK_enablegarcolenv(self.ptr) },"enable_gar_col_env")?;
      return Result::Ok(());
    } // enablegarcolenv
    #[allow(unused_parens)]
    pub fn expirylicenses(&mut self,expiry_ : &mut i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_expirylicenses(self.ptr,expiry_) },"expirylicenses")?;
      return Result::Ok(());
    } // expirylicenses
    #[allow(unused_parens)]
    pub fn gemm(&self,transa_ : i32,transb_ : i32,m_ : i32,n_ : i32,k_ : i32,alpha_ : f64,a_ : &[f64],b_ : &[f64],beta_ : f64,c_ : &mut[f64]) -> Result<(),String> {
      if a_.len() != ((m_*k_)).try_into().unwrap() {
        return Result::Err("Argument 'a' has the wrong length, expected (m_*k_)".to_string());
      }
      if b_.len() != ((k_*n_)).try_into().unwrap() {
        return Result::Err("Argument 'b' has the wrong length, expected (k_*n_)".to_string());
      }
      if c_.len() != ((m_*n_)).try_into().unwrap() {
        return Result::Err("Argument 'c' has the wrong length, expected (m_*n_)".to_string());
      }
      self.handle_res(unsafe { MSK_gemm(self.ptr,transa_,transb_,m_,n_,k_,alpha_,a_.as_ptr(),b_.as_ptr(),beta_,c_.as_mut_ptr()) },"gemm")?;
      return Result::Ok(());
    } // gemm
    #[allow(unused_parens)]
    pub fn linkfiletostream(&mut self,whichstream_ : i32,filename_ : &str,append_ : i32) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_linkfiletoenvstream(self.ptr,whichstream_,__tmp_1.as_ptr(),append_) },"linkfiletostream")?;
      return Result::Ok(());
    } // linkfiletoenvstream
    #[allow(unused_parens)]
    pub fn make_empty_task(&mut self) -> Result<Task,String> {
      let mut ptr_task : * const u8 = std::ptr::null();
      self.handle_res(unsafe { MSK_makeemptytask(self.ptr,& mut ptr_task) },"make_empty_task")?;
      let res_task = Task { ptr : ptr_task, streamcb : [None,None,None,None], valuecb : None };
      return Result::Ok(res_task);
    } // makeemptytask
    #[allow(unused_parens)]
    pub fn make_task(&mut self,maxnumcon_ : i32,maxnumvar_ : i32) -> Result<Task,String> {
      let mut ptr_task : * const u8 = std::ptr::null();
      self.handle_res(unsafe { MSK_maketask(self.ptr,maxnumcon_,maxnumvar_,& mut ptr_task) },"make_task")?;
      let res_task = Task { ptr : ptr_task, streamcb : [None,None,None,None], valuecb : None };
      return Result::Ok(res_task);
    } // maketask
    #[allow(unused_parens)]
    pub fn potrf(&self,uplo_ : i32,n_ : i32,a_ : &mut[f64]) -> Result<(),String> {
      if a_.len() != ((n_*n_)).try_into().unwrap() {
        return Result::Err("Argument 'a' has the wrong length, expected (n_*n_)".to_string());
      }
      self.handle_res(unsafe { MSK_potrf(self.ptr,uplo_,n_,a_.as_mut_ptr()) },"potrf")?;
      return Result::Ok(());
    } // potrf
    #[allow(unused_parens)]
    pub fn put_license_code(&mut self,code_ : &[i32]) -> Result<(),String> {
      if code_.len() != (MSK_LICENSE_BUFFER_LENGTH).try_into().unwrap() {
        return Result::Err("Argument 'code' has the wrong length, expected MSK_LICENSE_BUFFER_LENGTH".to_string());
      }
      self.handle_res(unsafe { MSK_putlicensecode(self.ptr,code_.as_ptr()) },"put_license_code")?;
      return Result::Ok(());
    } // putlicensecode
    #[allow(unused_parens)]
    pub fn put_license_debug(&mut self,licdebug_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putlicensedebug(self.ptr,licdebug_) },"put_license_debug")?;
      return Result::Ok(());
    } // putlicensedebug
    #[allow(unused_parens)]
    pub fn put_license_path(&mut self,licensepath_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(licensepath_).unwrap();
      self.handle_res(unsafe { MSK_putlicensepath(self.ptr,__tmp_1.as_ptr()) },"put_license_path")?;
      return Result::Ok(());
    } // putlicensepath
    #[allow(unused_parens)]
    pub fn put_license_wait(&mut self,licwait_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putlicensewait(self.ptr,licwait_) },"put_license_wait")?;
      return Result::Ok(());
    } // putlicensewait
    #[allow(unused_parens)]
    pub fn resetexpirylicenses(&mut self) -> Result<(),String> {
      self.handle_res(unsafe { MSK_resetexpirylicenses(self.ptr) },"resetexpirylicenses")?;
      return Result::Ok(());
    } // resetexpirylicenses
    #[allow(unused_parens)]
    pub fn sparse_triangular_solve_dense(&self,transposed_ : i32,lnzc_ : &[i32],lptrc_ : &[i64],lsubc_ : &[i32],lvalc_ : &[f64],b_ : &mut[f64]) -> Result<(),String> {
      let n_ : i32 = std::cmp::min(std::cmp::min(lnzc_.len(),lptrc_.len()),b_.len()) as i32;
      if lnzc_.len() != (n_).try_into().unwrap() {
        return Result::Err("Argument 'lnzc' has the wrong length, expected n_".to_string());
      }
      if lptrc_.len() != (n_).try_into().unwrap() {
        return Result::Err("Argument 'lptrc' has the wrong length, expected n_".to_string());
      }
      let lensubnval_ : i64 = std::cmp::min(lvalc_.len(),lsubc_.len()) as i64;
      if lsubc_.len() != (lensubnval_).try_into().unwrap() {
        return Result::Err("Argument 'lsubc' has the wrong length, expected lensubnval_".to_string());
      }
      if lvalc_.len() != (lensubnval_).try_into().unwrap() {
        return Result::Err("Argument 'lvalc' has the wrong length, expected lensubnval_".to_string());
      }
      if b_.len() != (n_).try_into().unwrap() {
        return Result::Err("Argument 'b' has the wrong length, expected n_".to_string());
      }
      self.handle_res(unsafe { MSK_sparsetriangularsolvedense(self.ptr,transposed_,n_,lnzc_.as_ptr(),lptrc_.as_ptr(),lensubnval_,lsubc_.as_ptr(),lvalc_.as_ptr(),b_.as_mut_ptr()) },"sparse_triangular_solve_dense")?;
      return Result::Ok(());
    } // sparsetriangularsolvedense
    #[allow(unused_parens)]
    pub fn syeig(&self,uplo_ : i32,n_ : i32,a_ : &[f64],w_ : &mut[f64]) -> Result<(),String> {
      if a_.len() != ((n_*n_)).try_into().unwrap() {
        return Result::Err("Argument 'a' has the wrong length, expected (n_*n_)".to_string());
      }
      if w_.len() != (n_).try_into().unwrap() {
        return Result::Err("Argument 'w' has the wrong length, expected n_".to_string());
      }
      self.handle_res(unsafe { MSK_syeig(self.ptr,uplo_,n_,a_.as_ptr(),w_.as_mut_ptr()) },"syeig")?;
      return Result::Ok(());
    } // syeig
    #[allow(unused_parens)]
    pub fn syevd(&self,uplo_ : i32,n_ : i32,a_ : &mut[f64],w_ : &mut[f64]) -> Result<(),String> {
      if a_.len() != ((n_*n_)).try_into().unwrap() {
        return Result::Err("Argument 'a' has the wrong length, expected (n_*n_)".to_string());
      }
      if w_.len() != (n_).try_into().unwrap() {
        return Result::Err("Argument 'w' has the wrong length, expected n_".to_string());
      }
      self.handle_res(unsafe { MSK_syevd(self.ptr,uplo_,n_,a_.as_mut_ptr(),w_.as_mut_ptr()) },"syevd")?;
      return Result::Ok(());
    } // syevd
    #[allow(unused_parens)]
    pub fn syrk(&self,uplo_ : i32,trans_ : i32,n_ : i32,k_ : i32,alpha_ : f64,a_ : &[f64],beta_ : f64,c_ : &mut[f64]) -> Result<(),String> {
      if a_.len() != ((n_*k_)).try_into().unwrap() {
        return Result::Err("Argument 'a' has the wrong length, expected (n_*k_)".to_string());
      }
      if c_.len() != ((n_*n_)).try_into().unwrap() {
        return Result::Err("Argument 'c' has the wrong length, expected (n_*n_)".to_string());
      }
      self.handle_res(unsafe { MSK_syrk(self.ptr,uplo_,trans_,n_,k_,alpha_,a_.as_ptr(),beta_,c_.as_mut_ptr()) },"syrk")?;
      return Result::Ok(());
    } // syrk

}

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
                     & std::slice::from_raw_parts(douinf, MSK_DINF_END as usize),
                     & std::slice::from_raw_parts(intinf, MSK_IINF_END as usize),
                     & std::slice::from_raw_parts(lintinf, MSK_LIINF_END as usize));
        return if r { 0 } else { 1 }
    }
}

impl Task
{
    pub fn new_from_env(env : &Env) -> Option<Task> {
        let mut task : * const u8 = std::ptr::null();
        if 0 != unsafe { MSK_maketask(env.ptr, 0,0, & mut task) } {
            return None;
        }

        return Some(Task { ptr      : task,
                           streamcb : [None,None,None,None],
                           valuecb  : None,});
    }

    pub fn new()  -> Option<Task> {
        let mut task : * const u8 = std::ptr::null();
        if 0 != unsafe { MSK_maketask(std::ptr::null(), 0,0, & mut task) } {
            return None;
        }

        return Some(Task { ptr      : task,
                           streamcb : [None,None,None,None],
                           valuecb  : None,});
    }

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


    // NOTE on callback with handles:
    //   http://aatch.github.io/blog/2015/01/17/unboxed-closures-and-ffi-callbacks/
    pub fn put_stream_callback<F>(& mut self,whichstream : i32, func : F) -> Result<(),String>
    where F : 'static+Fn(&str) {
        if whichstream >= 0 && whichstream < 4 {
            self.streamcb[whichstream as usize] = Some(Box::new(Box::new(func)));

            match self.streamcb[whichstream as usize] {
                Some(ref bf) => {
                    let hnd =  &(**bf) as * const _ as * mut libc::c_void;
                    let _ = self.handle_res(unsafe { MSK_linkfunctotaskstream(self.ptr,whichstream, hnd, stream_callback_proxy) },"put_stream_callback")?;
                }
                None => {}
            }
        }
        return Ok(());
    }


    pub fn clear_stream_callback(&mut self,whichstream : i32) -> Result<(),String> {
        match self.streamcb[whichstream as usize] {
            Some(ref _f) => {
                let _ = self.handle_res(unsafe { MSK_unlinkfuncfromtaskstream(self.ptr, whichstream) },"clear_stream_callback")?;
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
                let _ = self.handle_res(unsafe { MSK_putcallbackfunc(self.ptr, callback_proxy, hnd) },"put_callback")?;
            }
            None => {}
        }
        return Ok(());
    }

    #[allow(unused_parens)]
    pub fn analyze_names(&self,whichstream_ : i32,nametype_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_analyzenames(self.ptr,whichstream_,nametype_) },"analyze_names")?;
      return Result::Ok(());
    } // analyzenames
    #[allow(unused_parens)]
    pub fn analyze_problem(&self,whichstream_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_analyzeproblem(self.ptr,whichstream_) },"analyze_problem")?;
      return Result::Ok(());
    } // analyzeproblem
    #[allow(unused_parens)]
    pub fn analyze_solution(&self,whichstream_ : i32,whichsol_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_analyzesolution(self.ptr,whichstream_,whichsol_) },"analyze_solution")?;
      return Result::Ok(());
    } // analyzesolution
    #[allow(unused_parens)]
    pub fn append_acc(&mut self,domidx_ : i64,afeidxlist_ : &[i64],b_ : &[f64]) -> Result<(),String> {
      let numafeidx_ : i64 = afeidxlist_.len() as i64;
      if b_.len() != (numafeidx_).try_into().unwrap() {
        return Result::Err("Argument 'b' has the wrong length, expected numafeidx_".to_string());
      }
      self.handle_res(unsafe { MSK_appendacc(self.ptr,domidx_,numafeidx_,afeidxlist_.as_ptr(),b_.as_ptr()) },"append_acc")?;
      return Result::Ok(());
    } // appendacc
    #[allow(unused_parens)]
    pub fn append_accs(&mut self,domidxs_ : &[i64],afeidxlist_ : &[i64],b_ : &[f64]) -> Result<(),String> {
      let numaccs_ : i64 = domidxs_.len() as i64;
      let numafeidx_ : i64 = afeidxlist_.len() as i64;
      if b_.len() != (numafeidx_).try_into().unwrap() {
        return Result::Err("Argument 'b' has the wrong length, expected numafeidx_".to_string());
      }
      self.handle_res(unsafe { MSK_appendaccs(self.ptr,numaccs_,domidxs_.as_ptr(),numafeidx_,afeidxlist_.as_ptr(),b_.as_ptr()) },"append_accs")?;
      return Result::Ok(());
    } // appendaccs
    #[allow(unused_parens)]
    pub fn append_acc_seq(&mut self,domidx_ : i64,afeidxfirst_ : i64,b_ : &[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getdomainn(self.ptr,domidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getdomainn")?;
      let numafeidx_ : i64 = __tmp_0;
      if b_.len() != (numafeidx_).try_into().unwrap() {
        return Result::Err("Argument 'b' has the wrong length, expected numafeidx_".to_string());
      }
      self.handle_res(unsafe { MSK_appendaccseq(self.ptr,domidx_,numafeidx_,afeidxfirst_,b_.as_ptr()) },"append_acc_seq")?;
      return Result::Ok(());
    } // appendaccseq
    #[allow(unused_parens)]
    pub fn append_accs_seq(&mut self,domidxs_ : &[i64],numafeidx_ : i64,afeidxfirst_ : i64,b_ : &[f64]) -> Result<(),String> {
      let numaccs_ : i64 = domidxs_.len() as i64;
      if b_.len() != (numafeidx_).try_into().unwrap() {
        return Result::Err("Argument 'b' has the wrong length, expected numafeidx_".to_string());
      }
      self.handle_res(unsafe { MSK_appendaccsseq(self.ptr,numaccs_,domidxs_.as_ptr(),numafeidx_,afeidxfirst_,b_.as_ptr()) },"append_accs_seq")?;
      return Result::Ok(());
    } // appendaccsseq
    #[allow(unused_parens)]
    pub fn append_afes(&mut self,num_ : i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_appendafes(self.ptr,num_) },"append_afes")?;
      return Result::Ok(());
    } // appendafes
    #[allow(unused_parens)]
    pub fn append_barvars(&mut self,dim_ : &[i32]) -> Result<(),String> {
      let num_ : i32 = dim_.len() as i32;
      self.handle_res(unsafe { MSK_appendbarvars(self.ptr,num_,dim_.as_ptr()) },"append_barvars")?;
      return Result::Ok(());
    } // appendbarvars
    #[allow(unused_parens)]
    pub fn append_cone(&mut self,ct_ : i32,conepar_ : f64,submem_ : &[i32]) -> Result<(),String> {
      let nummem_ : i32 = submem_.len() as i32;
      self.handle_res(unsafe { MSK_appendcone(self.ptr,ct_,conepar_,nummem_,submem_.as_ptr()) },"append_cone")?;
      return Result::Ok(());
    } // appendcone
    #[allow(unused_parens)]
    pub fn append_cone_seq(&mut self,ct_ : i32,conepar_ : f64,nummem_ : i32,j_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_appendconeseq(self.ptr,ct_,conepar_,nummem_,j_) },"append_cone_seq")?;
      return Result::Ok(());
    } // appendconeseq
    #[allow(unused_parens)]
    pub fn append_cones_seq(&mut self,ct_ : &[i32],conepar_ : &[f64],nummem_ : &[i32],j_ : i32) -> Result<(),String> {
      let num_ : i32 = std::cmp::min(std::cmp::min(ct_.len(),conepar_.len()),nummem_.len()) as i32;
      self.handle_res(unsafe { MSK_appendconesseq(self.ptr,num_,ct_.as_ptr(),conepar_.as_ptr(),nummem_.as_ptr(),j_) },"append_cones_seq")?;
      return Result::Ok(());
    } // appendconesseq
    #[allow(unused_parens)]
    pub fn append_cons(&mut self,num_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_appendcons(self.ptr,num_) },"append_cons")?;
      return Result::Ok(());
    } // appendcons
    #[allow(unused_parens)]
    pub fn append_djcs(&mut self,num_ : i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_appenddjcs(self.ptr,num_) },"append_djcs")?;
      return Result::Ok(());
    } // appenddjcs
    #[allow(unused_parens)]
    pub fn append_dual_exp_cone_domain(&mut self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appenddualexpconedomain(self.ptr,&mut __tmp_0) },"append_dual_exp_cone_domain")?;
      return Result::Ok(__tmp_0);
    } // appenddualexpconedomain
    #[allow(unused_parens)]
    pub fn append_dual_geo_mean_cone_domain(&mut self,n_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appenddualgeomeanconedomain(self.ptr,n_,&mut __tmp_0) },"append_dual_geo_mean_cone_domain")?;
      return Result::Ok(__tmp_0);
    } // appenddualgeomeanconedomain
    #[allow(unused_parens)]
    pub fn append_dual_power_cone_domain(&mut self,n_ : i64,alpha_ : &[f64]) -> Result<i64,String> {
      let nleft_ : i64 = alpha_.len() as i64;
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appenddualpowerconedomain(self.ptr,n_,nleft_,alpha_.as_ptr(),&mut __tmp_0) },"append_dual_power_cone_domain")?;
      return Result::Ok(__tmp_0);
    } // appenddualpowerconedomain
    #[allow(unused_parens)]
    pub fn append_primal_exp_cone_domain(&mut self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appendprimalexpconedomain(self.ptr,&mut __tmp_0) },"append_primal_exp_cone_domain")?;
      return Result::Ok(__tmp_0);
    } // appendprimalexpconedomain
    #[allow(unused_parens)]
    pub fn append_primal_geo_mean_cone_domain(&mut self,n_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appendprimalgeomeanconedomain(self.ptr,n_,&mut __tmp_0) },"append_primal_geo_mean_cone_domain")?;
      return Result::Ok(__tmp_0);
    } // appendprimalgeomeanconedomain
    #[allow(unused_parens)]
    pub fn append_primal_power_cone_domain(&mut self,n_ : i64,alpha_ : &[f64]) -> Result<i64,String> {
      let nleft_ : i64 = alpha_.len() as i64;
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appendprimalpowerconedomain(self.ptr,n_,nleft_,alpha_.as_ptr(),&mut __tmp_0) },"append_primal_power_cone_domain")?;
      return Result::Ok(__tmp_0);
    } // appendprimalpowerconedomain
    #[allow(unused_parens)]
    pub fn append_quadratic_cone_domain(&mut self,n_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appendquadraticconedomain(self.ptr,n_,&mut __tmp_0) },"append_quadratic_cone_domain")?;
      return Result::Ok(__tmp_0);
    } // appendquadraticconedomain
    #[allow(unused_parens)]
    pub fn append_r_domain(&mut self,n_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appendrdomain(self.ptr,n_,&mut __tmp_0) },"append_r_domain")?;
      return Result::Ok(__tmp_0);
    } // appendrdomain
    #[allow(unused_parens)]
    pub fn append_rminus_domain(&mut self,n_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appendrminusdomain(self.ptr,n_,&mut __tmp_0) },"append_rminus_domain")?;
      return Result::Ok(__tmp_0);
    } // appendrminusdomain
    #[allow(unused_parens)]
    pub fn append_rplus_domain(&mut self,n_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appendrplusdomain(self.ptr,n_,&mut __tmp_0) },"append_rplus_domain")?;
      return Result::Ok(__tmp_0);
    } // appendrplusdomain
    #[allow(unused_parens)]
    pub fn append_r_quadratic_cone_domain(&mut self,n_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appendrquadraticconedomain(self.ptr,n_,&mut __tmp_0) },"append_r_quadratic_cone_domain")?;
      return Result::Ok(__tmp_0);
    } // appendrquadraticconedomain
    #[allow(unused_parens)]
    pub fn append_rzero_domain(&mut self,n_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appendrzerodomain(self.ptr,n_,&mut __tmp_0) },"append_rzero_domain")?;
      return Result::Ok(__tmp_0);
    } // appendrzerodomain
    #[allow(unused_parens)]
    pub fn append_sparse_sym_mat(&mut self,dim_ : i32,subi_ : &[i32],subj_ : &[i32],valij_ : &[f64]) -> Result<i64,String> {
      let nz_ : i64 = std::cmp::min(std::cmp::min(subj_.len(),valij_.len()),subi_.len()) as i64;
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appendsparsesymmat(self.ptr,dim_,nz_,subi_.as_ptr(),subj_.as_ptr(),valij_.as_ptr(),&mut __tmp_0) },"append_sparse_sym_mat")?;
      return Result::Ok(__tmp_0);
    } // appendsparsesymmat
    #[allow(unused_parens)]
    pub fn append_sparse_sym_mat_list(&mut self,dims_ : &[i32],nz_ : &[i64],subi_ : &[i32],subj_ : &[i32],valij_ : &[f64],idx_ : &mut[i64]) -> Result<(),String> {
      let num_ : i32 = std::cmp::min(dims_.len(),nz_.len()) as i32;
      let mut __tmp_0 : i64 = i64::default();
      for __tmp_1 in nz_ { __tmp_0 += __tmp_1; }
      if subi_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'subi' has the wrong length, expected __tmp_0".to_string());
      }
      let mut __tmp_2 : i64 = i64::default();
      for __tmp_3 in nz_ { __tmp_2 += __tmp_3; }
      if subj_.len() != (__tmp_2).try_into().unwrap() {
        return Result::Err("Argument 'subj' has the wrong length, expected __tmp_2".to_string());
      }
      let mut __tmp_4 : i64 = i64::default();
      for __tmp_5 in nz_ { __tmp_4 += __tmp_5; }
      if valij_.len() != (__tmp_4).try_into().unwrap() {
        return Result::Err("Argument 'valij' has the wrong length, expected __tmp_4".to_string());
      }
      if idx_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'idx' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_appendsparsesymmatlist(self.ptr,num_,dims_.as_ptr(),nz_.as_ptr(),subi_.as_ptr(),subj_.as_ptr(),valij_.as_ptr(),idx_.as_mut_ptr()) },"append_sparse_sym_mat_list")?;
      return Result::Ok(());
    } // appendsparsesymmatlist
    #[allow(unused_parens)]
    pub fn append_svec_psd_cone_domain(&mut self,n_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_appendsvecpsdconedomain(self.ptr,n_,&mut __tmp_0) },"append_svec_psd_cone_domain")?;
      return Result::Ok(__tmp_0);
    } // appendsvecpsdconedomain
    #[allow(unused_parens)]
    pub fn append_vars(&mut self,num_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_appendvars(self.ptr,num_) },"append_vars")?;
      return Result::Ok(());
    } // appendvars
    #[allow(unused_parens)]
    pub fn async_get_log(&mut self,addr_ : &str,accesstoken_ : &str,token_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(addr_).unwrap();
      let __tmp_3 = CString::new(accesstoken_).unwrap();
      let __tmp_5 = CString::new(token_).unwrap();
      self.handle_res(unsafe { MSK_asyncgetlog(self.ptr,__tmp_1.as_ptr(),__tmp_3.as_ptr(),__tmp_5.as_ptr()) },"async_get_log")?;
      return Result::Ok(());
    } // asyncgetlog
    #[allow(unused_parens)]
    pub fn async_get_result(&mut self,address_ : &str,accesstoken_ : &str,token_ : &str,resp_ : & mut i32,trm_ : & mut i32) -> Result<bool,String> {
      let __tmp_1 = CString::new(address_).unwrap();
      let __tmp_3 = CString::new(accesstoken_).unwrap();
      let __tmp_5 = CString::new(token_).unwrap();
      let mut __tmp_6 : i32 = 0;
      self.handle_res(unsafe { MSK_asyncgetresult(self.ptr,__tmp_1.as_ptr(),__tmp_3.as_ptr(),__tmp_5.as_ptr(),&mut __tmp_6,resp_,trm_) },"async_get_result")?;
      return Result::Ok(__tmp_6 != 0);
    } // asyncgetresult
    #[allow(unused_parens)]
    pub fn async_optimize(&mut self,address_ : &str,accesstoken_ : &str) -> Result<String,String> {
      let __tmp_1 = CString::new(address_).unwrap();
      let __tmp_3 = CString::new(accesstoken_).unwrap();
      let mut token_ = Vec::new(); token_.resize(33 as usize,0);
      self.handle_res(unsafe { MSK_asyncoptimize(self.ptr,__tmp_1.as_ptr(),__tmp_3.as_ptr(),token_.as_mut_ptr()) },"async_optimize")?;
      return Result::Ok(String::from_utf8_lossy(&token_[..]).into_owned());
    } // asyncoptimize
    #[allow(unused_parens)]
    pub fn async_poll(&mut self,address_ : &str,accesstoken_ : &str,token_ : &str,resp_ : & mut i32,trm_ : & mut i32) -> Result<bool,String> {
      let __tmp_1 = CString::new(address_).unwrap();
      let __tmp_3 = CString::new(accesstoken_).unwrap();
      let __tmp_5 = CString::new(token_).unwrap();
      let mut __tmp_6 : i32 = 0;
      self.handle_res(unsafe { MSK_asyncpoll(self.ptr,__tmp_1.as_ptr(),__tmp_3.as_ptr(),__tmp_5.as_ptr(),&mut __tmp_6,resp_,trm_) },"async_poll")?;
      return Result::Ok(__tmp_6 != 0);
    } // asyncpoll
    #[allow(unused_parens)]
    pub fn async_stop(&mut self,address_ : &str,accesstoken_ : &str,token_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(address_).unwrap();
      let __tmp_3 = CString::new(accesstoken_).unwrap();
      let __tmp_5 = CString::new(token_).unwrap();
      self.handle_res(unsafe { MSK_asyncstop(self.ptr,__tmp_1.as_ptr(),__tmp_3.as_ptr(),__tmp_5.as_ptr()) },"async_stop")?;
      return Result::Ok(());
    } // asyncstop
    #[allow(unused_parens)]
    pub fn basis_cond(&mut self,nrmbasis_ : &mut f64,nrminvbasis_ : &mut f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_basiscond(self.ptr,nrmbasis_,nrminvbasis_) },"basis_cond")?;
      return Result::Ok(());
    } // basiscond
    #[allow(unused_parens)]
    pub fn chg_con_bound(&mut self,i_ : i32,lower_ : i32,finite_ : i32,value_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_chgconbound(self.ptr,i_,lower_,finite_,value_) },"chg_con_bound")?;
      return Result::Ok(());
    } // chgconbound
    #[allow(unused_parens)]
    pub fn chg_var_bound(&mut self,j_ : i32,lower_ : i32,finite_ : i32,value_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_chgvarbound(self.ptr,j_,lower_,finite_,value_) },"chg_var_bound")?;
      return Result::Ok(());
    } // chgvarbound
    #[allow(unused_parens)]
    pub fn clone_task(&self) -> Result<Task,String> {
      let mut ptr_clonedtask : * const u8 = std::ptr::null();
      self.handle_res(unsafe { MSK_clonetask(self.ptr,& mut ptr_clonedtask) },"clone_task")?;
      let res_clonedtask = Task { ptr : ptr_clonedtask, streamcb : [None,None,None,None], valuecb : None };
      return Result::Ok(res_clonedtask);
    } // clonetask
    #[allow(unused_parens)]
    pub fn commit_changes(&mut self) -> Result<(),String> {
      self.handle_res(unsafe { MSK_commitchanges(self.ptr) },"commit_changes")?;
      return Result::Ok(());
    } // commitchanges
    #[allow(unused_parens)]
    pub fn delete_solution(&mut self,whichsol_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_deletesolution(self.ptr,whichsol_) },"delete_solution")?;
      return Result::Ok(());
    } // deletesolution
    #[allow(unused_parens)]
    pub fn dual_sensitivity(&self,subj_ : &[i32],leftpricej_ : &mut[f64],rightpricej_ : &mut[f64],leftrangej_ : &mut[f64],rightrangej_ : &mut[f64]) -> Result<(),String> {
      let numj_ : i32 = subj_.len() as i32;
      if leftpricej_.len() != (numj_).try_into().unwrap() {
        return Result::Err("Argument 'leftpricej' has the wrong length, expected numj_".to_string());
      }
      if rightpricej_.len() != (numj_).try_into().unwrap() {
        return Result::Err("Argument 'rightpricej' has the wrong length, expected numj_".to_string());
      }
      if leftrangej_.len() != (numj_).try_into().unwrap() {
        return Result::Err("Argument 'leftrangej' has the wrong length, expected numj_".to_string());
      }
      if rightrangej_.len() != (numj_).try_into().unwrap() {
        return Result::Err("Argument 'rightrangej' has the wrong length, expected numj_".to_string());
      }
      self.handle_res(unsafe { MSK_dualsensitivity(self.ptr,numj_,subj_.as_ptr(),leftpricej_.as_mut_ptr(),rightpricej_.as_mut_ptr(),leftrangej_.as_mut_ptr(),rightrangej_.as_mut_ptr()) },"dual_sensitivity")?;
      return Result::Ok(());
    } // dualsensitivity
    #[allow(unused_parens)]
    pub fn empty_afe_barf_row(&mut self,afeidx_ : i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_emptyafebarfrow(self.ptr,afeidx_) },"empty_afe_barf_row")?;
      return Result::Ok(());
    } // emptyafebarfrow
    #[allow(unused_parens)]
    pub fn empty_afe_barf_row_list(&mut self,afeidxlist_ : &[i64]) -> Result<(),String> {
      let numafeidx_ : i64 = afeidxlist_.len() as i64;
      self.handle_res(unsafe { MSK_emptyafebarfrowlist(self.ptr,numafeidx_,afeidxlist_.as_ptr()) },"empty_afe_barf_row_list")?;
      return Result::Ok(());
    } // emptyafebarfrowlist
    #[allow(unused_parens)]
    pub fn empty_afe_f_col(&mut self,varidx_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_emptyafefcol(self.ptr,varidx_) },"empty_afe_f_col")?;
      return Result::Ok(());
    } // emptyafefcol
    #[allow(unused_parens)]
    pub fn empty_afe_f_col_list(&mut self,varidx_ : &[i32]) -> Result<(),String> {
      let numvaridx_ : i64 = varidx_.len() as i64;
      self.handle_res(unsafe { MSK_emptyafefcollist(self.ptr,numvaridx_,varidx_.as_ptr()) },"empty_afe_f_col_list")?;
      return Result::Ok(());
    } // emptyafefcollist
    #[allow(unused_parens)]
    pub fn empty_afe_f_row(&mut self,afeidx_ : i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_emptyafefrow(self.ptr,afeidx_) },"empty_afe_f_row")?;
      return Result::Ok(());
    } // emptyafefrow
    #[allow(unused_parens)]
    pub fn empty_afe_f_row_list(&mut self,afeidx_ : &[i64]) -> Result<(),String> {
      let numafeidx_ : i64 = afeidx_.len() as i64;
      self.handle_res(unsafe { MSK_emptyafefrowlist(self.ptr,numafeidx_,afeidx_.as_ptr()) },"empty_afe_f_row_list")?;
      return Result::Ok(());
    } // emptyafefrowlist
    #[allow(unused_parens)]
    pub fn evaluate_acc(&self,whichsol_ : i32,accidx_ : i64,activity_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getaccn(self.ptr,accidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getaccn")?;
      if activity_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'activity' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_evaluateacc(self.ptr,whichsol_,accidx_,activity_.as_mut_ptr()) },"evaluate_acc")?;
      return Result::Ok(());
    } // evaluateacc
    #[allow(unused_parens)]
    pub fn evaluate_accs(&self,whichsol_ : i32,activity_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getaccntot(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getaccntot")?;
      if activity_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'activity' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_evaluateaccs(self.ptr,whichsol_,activity_.as_mut_ptr()) },"evaluate_accs")?;
      return Result::Ok(());
    } // evaluateaccs
    #[allow(unused_parens)]
    pub fn generate_acc_names(&mut self,sub_ : &[i64],fmt_ : &str,dims_ : &[i32],sp_ : &[i64],namedaxisidxs_ : &[i32],names_ : &[String]) -> Result<(),String> {
      let num_ : i64 = sub_.len() as i64;
      let __tmp_1 = CString::new(fmt_).unwrap();
      let ndims_ : i32 = dims_.len() as i32;
      if sp_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'sp' has the wrong length, expected num_".to_string());
      }
      let numnamedaxis_ : i32 = namedaxisidxs_.len() as i32;
      let numnames_ : i64 = names_.len() as i64;
      let cstr_names : Vec<CString> = names_.iter().map(|s| CString::new(s.as_str()).unwrap()).collect();
      let cptr_names : Vec<* const libc::c_char> = cstr_names.iter().map(|s| s.as_ptr()).collect();
      self.handle_res(unsafe { MSK_generateaccnames(self.ptr,num_,sub_.as_ptr(),__tmp_1.as_ptr(),ndims_,dims_.as_ptr(),sp_.as_ptr(),numnamedaxis_,namedaxisidxs_.as_ptr(),numnames_,cptr_names.as_ptr()) },"generate_acc_names")?;
      return Result::Ok(());
    } // generateaccnames
    #[allow(unused_parens)]
    pub fn generate_barvar_names(&mut self,subj_ : &[i32],fmt_ : &str,dims_ : &[i32],sp_ : &[i64],namedaxisidxs_ : &[i32],names_ : &[String]) -> Result<(),String> {
      let num_ : i32 = subj_.len() as i32;
      let __tmp_1 = CString::new(fmt_).unwrap();
      let ndims_ : i32 = dims_.len() as i32;
      if sp_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'sp' has the wrong length, expected num_".to_string());
      }
      let numnamedaxis_ : i32 = namedaxisidxs_.len() as i32;
      let numnames_ : i64 = names_.len() as i64;
      let cstr_names : Vec<CString> = names_.iter().map(|s| CString::new(s.as_str()).unwrap()).collect();
      let cptr_names : Vec<* const libc::c_char> = cstr_names.iter().map(|s| s.as_ptr()).collect();
      self.handle_res(unsafe { MSK_generatebarvarnames(self.ptr,num_,subj_.as_ptr(),__tmp_1.as_ptr(),ndims_,dims_.as_ptr(),sp_.as_ptr(),numnamedaxis_,namedaxisidxs_.as_ptr(),numnames_,cptr_names.as_ptr()) },"generate_barvar_names")?;
      return Result::Ok(());
    } // generatebarvarnames
    #[allow(unused_parens)]
    pub fn generate_cone_names(&mut self,subk_ : &[i32],fmt_ : &str,dims_ : &[i32],sp_ : &[i64],namedaxisidxs_ : &[i32],names_ : &[String]) -> Result<(),String> {
      let num_ : i32 = subk_.len() as i32;
      let __tmp_1 = CString::new(fmt_).unwrap();
      let ndims_ : i32 = dims_.len() as i32;
      if sp_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'sp' has the wrong length, expected num_".to_string());
      }
      let numnamedaxis_ : i32 = namedaxisidxs_.len() as i32;
      let numnames_ : i64 = names_.len() as i64;
      let cstr_names : Vec<CString> = names_.iter().map(|s| CString::new(s.as_str()).unwrap()).collect();
      let cptr_names : Vec<* const libc::c_char> = cstr_names.iter().map(|s| s.as_ptr()).collect();
      self.handle_res(unsafe { MSK_generateconenames(self.ptr,num_,subk_.as_ptr(),__tmp_1.as_ptr(),ndims_,dims_.as_ptr(),sp_.as_ptr(),numnamedaxis_,namedaxisidxs_.as_ptr(),numnames_,cptr_names.as_ptr()) },"generate_cone_names")?;
      return Result::Ok(());
    } // generateconenames
    #[allow(unused_parens)]
    pub fn generate_con_names(&mut self,subi_ : &[i32],fmt_ : &str,dims_ : &[i32],sp_ : &[i64],namedaxisidxs_ : &[i32],names_ : &[String]) -> Result<(),String> {
      let num_ : i32 = subi_.len() as i32;
      let __tmp_1 = CString::new(fmt_).unwrap();
      let ndims_ : i32 = dims_.len() as i32;
      if sp_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'sp' has the wrong length, expected num_".to_string());
      }
      let numnamedaxis_ : i32 = namedaxisidxs_.len() as i32;
      let numnames_ : i64 = names_.len() as i64;
      let cstr_names : Vec<CString> = names_.iter().map(|s| CString::new(s.as_str()).unwrap()).collect();
      let cptr_names : Vec<* const libc::c_char> = cstr_names.iter().map(|s| s.as_ptr()).collect();
      self.handle_res(unsafe { MSK_generateconnames(self.ptr,num_,subi_.as_ptr(),__tmp_1.as_ptr(),ndims_,dims_.as_ptr(),sp_.as_ptr(),numnamedaxis_,namedaxisidxs_.as_ptr(),numnames_,cptr_names.as_ptr()) },"generate_con_names")?;
      return Result::Ok(());
    } // generateconnames
    #[allow(unused_parens)]
    pub fn generate_djc_names(&mut self,sub_ : &[i64],fmt_ : &str,dims_ : &[i32],sp_ : &[i64],namedaxisidxs_ : &[i32],names_ : &[String]) -> Result<(),String> {
      let num_ : i64 = sub_.len() as i64;
      let __tmp_1 = CString::new(fmt_).unwrap();
      let ndims_ : i32 = dims_.len() as i32;
      if sp_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'sp' has the wrong length, expected num_".to_string());
      }
      let numnamedaxis_ : i32 = namedaxisidxs_.len() as i32;
      let numnames_ : i64 = names_.len() as i64;
      let cstr_names : Vec<CString> = names_.iter().map(|s| CString::new(s.as_str()).unwrap()).collect();
      let cptr_names : Vec<* const libc::c_char> = cstr_names.iter().map(|s| s.as_ptr()).collect();
      self.handle_res(unsafe { MSK_generatedjcnames(self.ptr,num_,sub_.as_ptr(),__tmp_1.as_ptr(),ndims_,dims_.as_ptr(),sp_.as_ptr(),numnamedaxis_,namedaxisidxs_.as_ptr(),numnames_,cptr_names.as_ptr()) },"generate_djc_names")?;
      return Result::Ok(());
    } // generatedjcnames
    #[allow(unused_parens)]
    pub fn generate_var_names(&mut self,subj_ : &[i32],fmt_ : &str,dims_ : &[i32],sp_ : &[i64],namedaxisidxs_ : &[i32],names_ : &[String]) -> Result<(),String> {
      let num_ : i32 = subj_.len() as i32;
      let __tmp_1 = CString::new(fmt_).unwrap();
      let ndims_ : i32 = dims_.len() as i32;
      if sp_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'sp' has the wrong length, expected num_".to_string());
      }
      let numnamedaxis_ : i32 = namedaxisidxs_.len() as i32;
      let numnames_ : i64 = names_.len() as i64;
      let cstr_names : Vec<CString> = names_.iter().map(|s| CString::new(s.as_str()).unwrap()).collect();
      let cptr_names : Vec<* const libc::c_char> = cstr_names.iter().map(|s| s.as_ptr()).collect();
      self.handle_res(unsafe { MSK_generatevarnames(self.ptr,num_,subj_.as_ptr(),__tmp_1.as_ptr(),ndims_,dims_.as_ptr(),sp_.as_ptr(),numnamedaxis_,namedaxisidxs_.as_ptr(),numnames_,cptr_names.as_ptr()) },"generate_var_names")?;
      return Result::Ok(());
    } // generatevarnames
    #[allow(unused_parens)]
    pub fn get_acc_afe_idx_list(&self,accidx_ : i64,afeidxlist_ : &mut[i64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getaccn(self.ptr,accidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getaccn")?;
      if afeidxlist_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'afeidxlist' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getaccafeidxlist(self.ptr,accidx_,afeidxlist_.as_mut_ptr()) },"get_acc_afe_idx_list")?;
      return Result::Ok(());
    } // getaccafeidxlist
    #[allow(unused_parens)]
    pub fn get_acc_b(&self,accidx_ : i64,b_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getaccn(self.ptr,accidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getaccn")?;
      if b_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'b' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getaccb(self.ptr,accidx_,b_.as_mut_ptr()) },"get_acc_b")?;
      return Result::Ok(());
    } // getaccb
    #[allow(unused_parens)]
    pub fn get_acc_barf_block_triplet(&self,acc_afe_ : &mut[i64],bar_var_ : &mut[i32],blk_row_ : &mut[i32],blk_col_ : &mut[i32],blk_val_ : &mut[f64]) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getaccbarfnumblocktriplets(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getaccbarfnumblocktriplets")?;
      let maxnumtrip_ : i64 = __tmp_0;
      let mut __tmp_2 : i64 = i64::default();
      if acc_afe_.len() != (maxnumtrip_).try_into().unwrap() {
        return Result::Err("Argument 'acc_afe' has the wrong length, expected maxnumtrip_".to_string());
      }
      if bar_var_.len() != (maxnumtrip_).try_into().unwrap() {
        return Result::Err("Argument 'bar_var' has the wrong length, expected maxnumtrip_".to_string());
      }
      if blk_row_.len() != (maxnumtrip_).try_into().unwrap() {
        return Result::Err("Argument 'blk_row' has the wrong length, expected maxnumtrip_".to_string());
      }
      if blk_col_.len() != (maxnumtrip_).try_into().unwrap() {
        return Result::Err("Argument 'blk_col' has the wrong length, expected maxnumtrip_".to_string());
      }
      if blk_val_.len() != (maxnumtrip_).try_into().unwrap() {
        return Result::Err("Argument 'blk_val' has the wrong length, expected maxnumtrip_".to_string());
      }
      self.handle_res(unsafe { MSK_getaccbarfblocktriplet(self.ptr,maxnumtrip_,&mut __tmp_2,acc_afe_.as_mut_ptr(),bar_var_.as_mut_ptr(),blk_row_.as_mut_ptr(),blk_col_.as_mut_ptr(),blk_val_.as_mut_ptr()) },"get_acc_barf_block_triplet")?;
      return Result::Ok(__tmp_2);
    } // getaccbarfblocktriplet
    #[allow(unused_parens)]
    pub fn get_acc_barf_num_block_triplets(&self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getaccbarfnumblocktriplets(self.ptr,&mut __tmp_0) },"get_acc_barf_num_block_triplets")?;
      return Result::Ok(__tmp_0);
    } // getaccbarfnumblocktriplets
    #[allow(unused_parens)]
    pub fn get_acc_domain(&mut self,accidx_ : i64,domidx_ : &mut i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getaccdomain(self.ptr,accidx_,domidx_) },"get_acc_domain")?;
      return Result::Ok(());
    } // getaccdomain
    #[allow(unused_parens)]
    pub fn get_acc_dot_y(&self,whichsol_ : i32,accidx_ : i64,doty_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getaccn(self.ptr,accidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getaccn")?;
      if doty_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'doty' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getaccdoty(self.ptr,whichsol_,accidx_,doty_.as_mut_ptr()) },"get_acc_dot_y")?;
      return Result::Ok(());
    } // getaccdoty
    #[allow(unused_parens)]
    pub fn get_acc_dot_y_s(&self,whichsol_ : i32,doty_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getaccntot(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getaccntot")?;
      if doty_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'doty' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getaccdotys(self.ptr,whichsol_,doty_.as_mut_ptr()) },"get_acc_dot_y_s")?;
      return Result::Ok(());
    } // getaccdotys
    #[allow(unused_parens)]
    pub fn get_acc_f_numnz(&mut self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getaccfnumnz(self.ptr,&mut __tmp_0) },"get_acc_f_numnz")?;
      return Result::Ok(__tmp_0);
    } // getaccfnumnz
    #[allow(unused_parens)]
    pub fn get_acc_f_trip(&mut self,frow_ : &mut[i64],fcol_ : &mut[i32],fval_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getaccfnumnz(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getaccfnumnz")?;
      if frow_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'frow' has the wrong length, expected __tmp_0".to_string());
      }
      let mut __tmp_2 : i64 = i64::default();
      let __tmp_3 = unsafe { MSK_getaccfnumnz(self.ptr,&mut __tmp_2) };let _ = self.handle_res(__tmp_3,"getaccfnumnz")?;
      if fcol_.len() != (__tmp_2).try_into().unwrap() {
        return Result::Err("Argument 'fcol' has the wrong length, expected __tmp_2".to_string());
      }
      let mut __tmp_4 : i64 = i64::default();
      let __tmp_5 = unsafe { MSK_getaccfnumnz(self.ptr,&mut __tmp_4) };let _ = self.handle_res(__tmp_5,"getaccfnumnz")?;
      if fval_.len() != (__tmp_4).try_into().unwrap() {
        return Result::Err("Argument 'fval' has the wrong length, expected __tmp_4".to_string());
      }
      self.handle_res(unsafe { MSK_getaccftrip(self.ptr,frow_.as_mut_ptr(),fcol_.as_mut_ptr(),fval_.as_mut_ptr()) },"get_acc_f_trip")?;
      return Result::Ok(());
    } // getaccftrip
    #[allow(unused_parens)]
    pub fn get_acc_g_vector(&self,g_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getaccntot(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getaccntot")?;
      if g_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'g' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getaccgvector(self.ptr,g_.as_mut_ptr()) },"get_acc_g_vector")?;
      return Result::Ok(());
    } // getaccgvector
    #[allow(unused_parens)]
    pub fn get_acc_n(&mut self,accidx_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getaccn(self.ptr,accidx_,&mut __tmp_0) },"get_acc_n")?;
      return Result::Ok(__tmp_0);
    } // getaccn
    #[allow(unused_parens)]
    pub fn get_acc_name(&self,accidx_ : i64) -> Result<String,String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getaccnamelen(self.ptr,accidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getaccnamelen")?;
      let sizename_ : i32 = (1+__tmp_0);
      let mut name_ = Vec::new(); name_.resize(sizename_ as usize,0);
      self.handle_res(unsafe { MSK_getaccname(self.ptr,accidx_,sizename_,name_.as_mut_ptr()) },"get_acc_name")?;
      return Result::Ok(String::from_utf8_lossy(&name_[..]).into_owned());
    } // getaccname
    #[allow(unused_parens)]
    pub fn get_acc_name_len(&self,accidx_ : i64) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getaccnamelen(self.ptr,accidx_,&mut __tmp_0) },"get_acc_name_len")?;
      return Result::Ok(__tmp_0);
    } // getaccnamelen
    #[allow(unused_parens)]
    pub fn get_acc_n_tot(&mut self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getaccntot(self.ptr,&mut __tmp_0) },"get_acc_n_tot")?;
      return Result::Ok(__tmp_0);
    } // getaccntot
    #[allow(unused_parens)]
    pub fn get_accs(&self,domidxlist_ : &mut[i64],afeidxlist_ : &mut[i64],b_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getnumacc(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumacc")?;
      if domidxlist_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'domidxlist' has the wrong length, expected __tmp_0".to_string());
      }
      let mut __tmp_2 : i64 = i64::default();
      let __tmp_3 = unsafe { MSK_getaccntot(self.ptr,&mut __tmp_2) };let _ = self.handle_res(__tmp_3,"getaccntot")?;
      if afeidxlist_.len() != (__tmp_2).try_into().unwrap() {
        return Result::Err("Argument 'afeidxlist' has the wrong length, expected __tmp_2".to_string());
      }
      let mut __tmp_4 : i64 = i64::default();
      let __tmp_5 = unsafe { MSK_getaccntot(self.ptr,&mut __tmp_4) };let _ = self.handle_res(__tmp_5,"getaccntot")?;
      if b_.len() != (__tmp_4).try_into().unwrap() {
        return Result::Err("Argument 'b' has the wrong length, expected __tmp_4".to_string());
      }
      self.handle_res(unsafe { MSK_getaccs(self.ptr,domidxlist_.as_mut_ptr(),afeidxlist_.as_mut_ptr(),b_.as_mut_ptr()) },"get_accs")?;
      return Result::Ok(());
    } // getaccs
    #[allow(unused_parens)]
    pub fn get_a_col(&self,j_ : i32,nzj_ : &mut i32,subj_ : &mut[i32],valj_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_1 : i32 = i32::default();
      let __tmp_2 = unsafe { MSK_getacolnumnz(self.ptr,j_,&mut __tmp_1) };let _ = self.handle_res(__tmp_2,"getacolnumnz")?;
      if subj_.len() != (__tmp_1).try_into().unwrap() {
        return Result::Err("Argument 'subj' has the wrong length, expected __tmp_1".to_string());
      }
      let mut __tmp_3 : i32 = i32::default();
      let __tmp_4 = unsafe { MSK_getacolnumnz(self.ptr,j_,&mut __tmp_3) };let _ = self.handle_res(__tmp_4,"getacolnumnz")?;
      if valj_.len() != (__tmp_3).try_into().unwrap() {
        return Result::Err("Argument 'valj' has the wrong length, expected __tmp_3".to_string());
      }
      self.handle_res(unsafe { MSK_getacol(self.ptr,j_,nzj_,subj_.as_mut_ptr(),valj_.as_mut_ptr()) },"get_a_col")?;
      return Result::Ok(());
    } // getacol
    #[allow(unused_parens)]
    pub fn get_a_col_num_nz(&self,i_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getacolnumnz(self.ptr,i_,&mut __tmp_0) },"get_a_col_num_nz")?;
      return Result::Ok(__tmp_0);
    } // getacolnumnz
    #[allow(unused_parens)]
    pub fn get_a_col_slice(&self,first_ : i32,last_ : i32,ptrb_ : &mut[i64],ptre_ : &mut[i64],sub_ : &mut[i32],val_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getacolslicenumnz64(self.ptr,first_,last_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getacolslicenumnz64")?;
      let maxnumnz_ : i64 = __tmp_0;
      if ptrb_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'ptrb' has the wrong length, expected (last_-first_)".to_string());
      }
      if ptre_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'ptre' has the wrong length, expected (last_-first_)".to_string());
      }
      if sub_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("Argument 'sub' has the wrong length, expected maxnumnz_".to_string());
      }
      if val_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("Argument 'val' has the wrong length, expected maxnumnz_".to_string());
      }
      self.handle_res(unsafe { MSK_getacolslice64(self.ptr,first_,last_,maxnumnz_,ptrb_.as_mut_ptr(),ptre_.as_mut_ptr(),sub_.as_mut_ptr(),val_.as_mut_ptr()) },"get_a_col_slice")?;
      return Result::Ok(());
    } // getacolslice64
    #[allow(unused_parens)]
    pub fn get_a_col_slice_num_nz(&self,first_ : i32,last_ : i32) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getacolslicenumnz64(self.ptr,first_,last_,&mut __tmp_0) },"get_a_col_slice_num_nz")?;
      return Result::Ok(__tmp_0);
    } // getacolslicenumnz64
    #[allow(unused_parens)]
    pub fn get_a_col_slice_trip(&self,first_ : i32,last_ : i32,subi_ : &mut[i32],subj_ : &mut[i32],val_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getacolslicenumnz64(self.ptr,first_,last_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getacolslicenumnz64")?;
      let maxnumnz_ : i64 = __tmp_0;
      if subi_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("Argument 'subi' has the wrong length, expected maxnumnz_".to_string());
      }
      if subj_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("Argument 'subj' has the wrong length, expected maxnumnz_".to_string());
      }
      if val_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("Argument 'val' has the wrong length, expected maxnumnz_".to_string());
      }
      self.handle_res(unsafe { MSK_getacolslicetrip(self.ptr,first_,last_,maxnumnz_,subi_.as_mut_ptr(),subj_.as_mut_ptr(),val_.as_mut_ptr()) },"get_a_col_slice_trip")?;
      return Result::Ok(());
    } // getacolslicetrip
    #[allow(unused_parens)]
    pub fn get_afe_barf_block_triplet(&self,afeidx_ : &mut[i64],barvaridx_ : &mut[i32],subk_ : &mut[i32],subl_ : &mut[i32],valkl_ : &mut[f64]) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getafebarfnumblocktriplets(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getafebarfnumblocktriplets")?;
      let maxnumtrip_ : i64 = __tmp_0;
      let mut __tmp_2 : i64 = i64::default();
      if afeidx_.len() != (maxnumtrip_).try_into().unwrap() {
        return Result::Err("Argument 'afeidx' has the wrong length, expected maxnumtrip_".to_string());
      }
      if barvaridx_.len() != (maxnumtrip_).try_into().unwrap() {
        return Result::Err("Argument 'barvaridx' has the wrong length, expected maxnumtrip_".to_string());
      }
      if subk_.len() != (maxnumtrip_).try_into().unwrap() {
        return Result::Err("Argument 'subk' has the wrong length, expected maxnumtrip_".to_string());
      }
      if subl_.len() != (maxnumtrip_).try_into().unwrap() {
        return Result::Err("Argument 'subl' has the wrong length, expected maxnumtrip_".to_string());
      }
      if valkl_.len() != (maxnumtrip_).try_into().unwrap() {
        return Result::Err("Argument 'valkl' has the wrong length, expected maxnumtrip_".to_string());
      }
      self.handle_res(unsafe { MSK_getafebarfblocktriplet(self.ptr,maxnumtrip_,&mut __tmp_2,afeidx_.as_mut_ptr(),barvaridx_.as_mut_ptr(),subk_.as_mut_ptr(),subl_.as_mut_ptr(),valkl_.as_mut_ptr()) },"get_afe_barf_block_triplet")?;
      return Result::Ok(__tmp_2);
    } // getafebarfblocktriplet
    #[allow(unused_parens)]
    pub fn get_afe_barf_num_block_triplets(&self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getafebarfnumblocktriplets(self.ptr,&mut __tmp_0) },"get_afe_barf_num_block_triplets")?;
      return Result::Ok(__tmp_0);
    } // getafebarfnumblocktriplets
    #[allow(unused_parens)]
    pub fn get_afe_barf_num_row_entries(&mut self,afeidx_ : i64) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getafebarfnumrowentries(self.ptr,afeidx_,&mut __tmp_0) },"get_afe_barf_num_row_entries")?;
      return Result::Ok(__tmp_0);
    } // getafebarfnumrowentries
    #[allow(unused_parens)]
    pub fn get_afe_barf_row(&mut self,afeidx_ : i64,barvaridx_ : &mut[i32],ptrterm_ : &mut[i64],numterm_ : &mut[i64],termidx_ : &mut[i64],termweight_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let mut __tmp_1 : i64 = i64::default();
      let __tmp_2 = unsafe { MSK_getafebarfrowinfo(self.ptr,afeidx_,&mut __tmp_0,&mut __tmp_1) };let _ = self.handle_res(__tmp_2,"getafebarfrowinfo")?;
      if barvaridx_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'barvaridx' has the wrong length, expected __tmp_0".to_string());
      }
      let mut __tmp_3 : i32 = i32::default();
      let mut __tmp_4 : i64 = i64::default();
      let __tmp_5 = unsafe { MSK_getafebarfrowinfo(self.ptr,afeidx_,&mut __tmp_3,&mut __tmp_4) };let _ = self.handle_res(__tmp_5,"getafebarfrowinfo")?;
      if ptrterm_.len() != (__tmp_3).try_into().unwrap() {
        return Result::Err("Argument 'ptrterm' has the wrong length, expected __tmp_3".to_string());
      }
      let mut __tmp_6 : i32 = i32::default();
      let mut __tmp_7 : i64 = i64::default();
      let __tmp_8 = unsafe { MSK_getafebarfrowinfo(self.ptr,afeidx_,&mut __tmp_6,&mut __tmp_7) };let _ = self.handle_res(__tmp_8,"getafebarfrowinfo")?;
      if numterm_.len() != (__tmp_6).try_into().unwrap() {
        return Result::Err("Argument 'numterm' has the wrong length, expected __tmp_6".to_string());
      }
      let mut __tmp_9 : i32 = i32::default();
      let mut __tmp_10 : i64 = i64::default();
      let __tmp_11 = unsafe { MSK_getafebarfrowinfo(self.ptr,afeidx_,&mut __tmp_9,&mut __tmp_10) };let _ = self.handle_res(__tmp_11,"getafebarfrowinfo")?;
      if termidx_.len() != (__tmp_10).try_into().unwrap() {
        return Result::Err("Argument 'termidx' has the wrong length, expected __tmp_10".to_string());
      }
      let mut __tmp_12 : i32 = i32::default();
      let mut __tmp_13 : i64 = i64::default();
      let __tmp_14 = unsafe { MSK_getafebarfrowinfo(self.ptr,afeidx_,&mut __tmp_12,&mut __tmp_13) };let _ = self.handle_res(__tmp_14,"getafebarfrowinfo")?;
      if termweight_.len() != (__tmp_13).try_into().unwrap() {
        return Result::Err("Argument 'termweight' has the wrong length, expected __tmp_13".to_string());
      }
      self.handle_res(unsafe { MSK_getafebarfrow(self.ptr,afeidx_,barvaridx_.as_mut_ptr(),ptrterm_.as_mut_ptr(),numterm_.as_mut_ptr(),termidx_.as_mut_ptr(),termweight_.as_mut_ptr()) },"get_afe_barf_row")?;
      return Result::Ok(());
    } // getafebarfrow
    #[allow(unused_parens)]
    pub fn get_afe_barf_row_info(&mut self,afeidx_ : i64,numentr_ : &mut i32,numterm_ : &mut i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getafebarfrowinfo(self.ptr,afeidx_,numentr_,numterm_) },"get_afe_barf_row_info")?;
      return Result::Ok(());
    } // getafebarfrowinfo
    #[allow(unused_parens)]
    pub fn get_afe_f_num_nz(&mut self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getafefnumnz(self.ptr,&mut __tmp_0) },"get_afe_f_num_nz")?;
      return Result::Ok(__tmp_0);
    } // getafefnumnz
    #[allow(unused_parens)]
    pub fn get_afe_f_row(&mut self,afeidx_ : i64,numnz_ : &mut i32,varidx_ : &mut[i32],val_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_1 : i32 = i32::default();
      let __tmp_2 = unsafe { MSK_getafefrownumnz(self.ptr,afeidx_,&mut __tmp_1) };let _ = self.handle_res(__tmp_2,"getafefrownumnz")?;
      if varidx_.len() != (__tmp_1).try_into().unwrap() {
        return Result::Err("Argument 'varidx' has the wrong length, expected __tmp_1".to_string());
      }
      let mut __tmp_3 : i32 = i32::default();
      let __tmp_4 = unsafe { MSK_getafefrownumnz(self.ptr,afeidx_,&mut __tmp_3) };let _ = self.handle_res(__tmp_4,"getafefrownumnz")?;
      if val_.len() != (__tmp_3).try_into().unwrap() {
        return Result::Err("Argument 'val' has the wrong length, expected __tmp_3".to_string());
      }
      self.handle_res(unsafe { MSK_getafefrow(self.ptr,afeidx_,numnz_,varidx_.as_mut_ptr(),val_.as_mut_ptr()) },"get_afe_f_row")?;
      return Result::Ok(());
    } // getafefrow
    #[allow(unused_parens)]
    pub fn get_afe_f_row_num_nz(&mut self,afeidx_ : i64) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getafefrownumnz(self.ptr,afeidx_,&mut __tmp_0) },"get_afe_f_row_num_nz")?;
      return Result::Ok(__tmp_0);
    } // getafefrownumnz
    #[allow(unused_parens)]
    pub fn get_afe_f_trip(&mut self,afeidx_ : &mut[i64],varidx_ : &mut[i32],val_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getafefnumnz(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getafefnumnz")?;
      if afeidx_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'afeidx' has the wrong length, expected __tmp_0".to_string());
      }
      let mut __tmp_2 : i64 = i64::default();
      let __tmp_3 = unsafe { MSK_getafefnumnz(self.ptr,&mut __tmp_2) };let _ = self.handle_res(__tmp_3,"getafefnumnz")?;
      if varidx_.len() != (__tmp_2).try_into().unwrap() {
        return Result::Err("Argument 'varidx' has the wrong length, expected __tmp_2".to_string());
      }
      let mut __tmp_4 : i64 = i64::default();
      let __tmp_5 = unsafe { MSK_getafefnumnz(self.ptr,&mut __tmp_4) };let _ = self.handle_res(__tmp_5,"getafefnumnz")?;
      if val_.len() != (__tmp_4).try_into().unwrap() {
        return Result::Err("Argument 'val' has the wrong length, expected __tmp_4".to_string());
      }
      self.handle_res(unsafe { MSK_getafeftrip(self.ptr,afeidx_.as_mut_ptr(),varidx_.as_mut_ptr(),val_.as_mut_ptr()) },"get_afe_f_trip")?;
      return Result::Ok(());
    } // getafeftrip
    #[allow(unused_parens)]
    pub fn get_afe_g(&mut self,afeidx_ : i64) -> Result<f64,String> {
      let mut __tmp_0 : f64 = f64::default();
      self.handle_res(unsafe { MSK_getafeg(self.ptr,afeidx_,&mut __tmp_0) },"get_afe_g")?;
      return Result::Ok(__tmp_0);
    } // getafeg
    #[allow(unused_parens)]
    pub fn get_afe_g_slice(&self,first_ : i64,last_ : i64,g_ : &mut[f64]) -> Result<(),String> {
      if g_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'g' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getafegslice(self.ptr,first_,last_,g_.as_mut_ptr()) },"get_afe_g_slice")?;
      return Result::Ok(());
    } // getafegslice
    #[allow(unused_parens)]
    pub fn get_aij(&self,i_ : i32,j_ : i32) -> Result<f64,String> {
      let mut __tmp_0 : f64 = f64::default();
      self.handle_res(unsafe { MSK_getaij(self.ptr,i_,j_,&mut __tmp_0) },"get_aij")?;
      return Result::Ok(__tmp_0);
    } // getaij
    #[allow(unused_parens)]
    pub fn get_a_piece_num_nz(&self,firsti_ : i32,lasti_ : i32,firstj_ : i32,lastj_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getapiecenumnz(self.ptr,firsti_,lasti_,firstj_,lastj_,&mut __tmp_0) },"get_a_piece_num_nz")?;
      return Result::Ok(__tmp_0);
    } // getapiecenumnz
    #[allow(unused_parens)]
    pub fn get_a_row(&self,i_ : i32,nzi_ : &mut i32,subi_ : &mut[i32],vali_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_1 : i32 = i32::default();
      let __tmp_2 = unsafe { MSK_getarownumnz(self.ptr,i_,&mut __tmp_1) };let _ = self.handle_res(__tmp_2,"getarownumnz")?;
      if subi_.len() != (__tmp_1).try_into().unwrap() {
        return Result::Err("Argument 'subi' has the wrong length, expected __tmp_1".to_string());
      }
      let mut __tmp_3 : i32 = i32::default();
      let __tmp_4 = unsafe { MSK_getarownumnz(self.ptr,i_,&mut __tmp_3) };let _ = self.handle_res(__tmp_4,"getarownumnz")?;
      if vali_.len() != (__tmp_3).try_into().unwrap() {
        return Result::Err("Argument 'vali' has the wrong length, expected __tmp_3".to_string());
      }
      self.handle_res(unsafe { MSK_getarow(self.ptr,i_,nzi_,subi_.as_mut_ptr(),vali_.as_mut_ptr()) },"get_a_row")?;
      return Result::Ok(());
    } // getarow
    #[allow(unused_parens)]
    pub fn get_a_row_num_nz(&self,i_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getarownumnz(self.ptr,i_,&mut __tmp_0) },"get_a_row_num_nz")?;
      return Result::Ok(__tmp_0);
    } // getarownumnz
    #[allow(unused_parens)]
    pub fn get_a_row_slice(&self,first_ : i32,last_ : i32,ptrb_ : &mut[i64],ptre_ : &mut[i64],sub_ : &mut[i32],val_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getarowslicenumnz64(self.ptr,first_,last_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getarowslicenumnz64")?;
      let maxnumnz_ : i64 = __tmp_0;
      if ptrb_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'ptrb' has the wrong length, expected (last_-first_)".to_string());
      }
      if ptre_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'ptre' has the wrong length, expected (last_-first_)".to_string());
      }
      if sub_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("Argument 'sub' has the wrong length, expected maxnumnz_".to_string());
      }
      if val_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("Argument 'val' has the wrong length, expected maxnumnz_".to_string());
      }
      self.handle_res(unsafe { MSK_getarowslice64(self.ptr,first_,last_,maxnumnz_,ptrb_.as_mut_ptr(),ptre_.as_mut_ptr(),sub_.as_mut_ptr(),val_.as_mut_ptr()) },"get_a_row_slice")?;
      return Result::Ok(());
    } // getarowslice64
    #[allow(unused_parens)]
    pub fn get_a_row_slice_num_nz(&self,first_ : i32,last_ : i32) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getarowslicenumnz64(self.ptr,first_,last_,&mut __tmp_0) },"get_a_row_slice_num_nz")?;
      return Result::Ok(__tmp_0);
    } // getarowslicenumnz64
    #[allow(unused_parens)]
    pub fn get_a_row_slice_trip(&self,first_ : i32,last_ : i32,subi_ : &mut[i32],subj_ : &mut[i32],val_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getarowslicenumnz64(self.ptr,first_,last_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getarowslicenumnz64")?;
      let maxnumnz_ : i64 = __tmp_0;
      if subi_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("Argument 'subi' has the wrong length, expected maxnumnz_".to_string());
      }
      if subj_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("Argument 'subj' has the wrong length, expected maxnumnz_".to_string());
      }
      if val_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("Argument 'val' has the wrong length, expected maxnumnz_".to_string());
      }
      self.handle_res(unsafe { MSK_getarowslicetrip(self.ptr,first_,last_,maxnumnz_,subi_.as_mut_ptr(),subj_.as_mut_ptr(),val_.as_mut_ptr()) },"get_a_row_slice_trip")?;
      return Result::Ok(());
    } // getarowslicetrip
    #[allow(unused_parens)]
    pub fn get_a_trip(&self,subi_ : &mut[i32],subj_ : &mut[i32],val_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getnumanz64(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumanz64")?;
      let maxnumnz_ : i64 = __tmp_0;
      if subi_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("Argument 'subi' has the wrong length, expected maxnumnz_".to_string());
      }
      if subj_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("Argument 'subj' has the wrong length, expected maxnumnz_".to_string());
      }
      if val_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("Argument 'val' has the wrong length, expected maxnumnz_".to_string());
      }
      self.handle_res(unsafe { MSK_getatrip(self.ptr,maxnumnz_,subi_.as_mut_ptr(),subj_.as_mut_ptr(),val_.as_mut_ptr()) },"get_a_trip")?;
      return Result::Ok(());
    } // getatrip
    #[allow(unused_parens)]
    pub fn get_a_truncate_tol(&self,tolzero_ : &mut[f64]) -> Result<(),String> {
      if tolzero_.len() != (1).try_into().unwrap() {
        return Result::Err("Argument 'tolzero' has the wrong length, expected 1".to_string());
      }
      self.handle_res(unsafe { MSK_getatruncatetol(self.ptr,tolzero_.as_mut_ptr()) },"get_a_truncate_tol")?;
      return Result::Ok(());
    } // getatruncatetol
    #[allow(unused_parens)]
    pub fn get_bara_block_triplet(&self,subi_ : &mut[i32],subj_ : &mut[i32],subk_ : &mut[i32],subl_ : &mut[i32],valijkl_ : &mut[f64]) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getnumbarablocktriplets(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumbarablocktriplets")?;
      let maxnum_ : i64 = __tmp_0;
      let mut __tmp_2 : i64 = i64::default();
      if subi_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("Argument 'subi' has the wrong length, expected maxnum_".to_string());
      }
      if subj_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("Argument 'subj' has the wrong length, expected maxnum_".to_string());
      }
      if subk_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("Argument 'subk' has the wrong length, expected maxnum_".to_string());
      }
      if subl_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("Argument 'subl' has the wrong length, expected maxnum_".to_string());
      }
      if valijkl_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("Argument 'valijkl' has the wrong length, expected maxnum_".to_string());
      }
      self.handle_res(unsafe { MSK_getbarablocktriplet(self.ptr,maxnum_,&mut __tmp_2,subi_.as_mut_ptr(),subj_.as_mut_ptr(),subk_.as_mut_ptr(),subl_.as_mut_ptr(),valijkl_.as_mut_ptr()) },"get_bara_block_triplet")?;
      return Result::Ok(__tmp_2);
    } // getbarablocktriplet
    #[allow(unused_parens)]
    pub fn get_bara_idx(&self,idx_ : i64,i_ : &mut i32,j_ : &mut i32,sub_ : &mut[i64],weights_ : &mut[f64]) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getbaraidxinfo(self.ptr,idx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getbaraidxinfo")?;
      let maxnum_ : i64 = __tmp_0;
      let mut __tmp_4 : i64 = i64::default();
      if sub_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("Argument 'sub' has the wrong length, expected maxnum_".to_string());
      }
      if weights_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("Argument 'weights' has the wrong length, expected maxnum_".to_string());
      }
      self.handle_res(unsafe { MSK_getbaraidx(self.ptr,idx_,maxnum_,i_,j_,&mut __tmp_4,sub_.as_mut_ptr(),weights_.as_mut_ptr()) },"get_bara_idx")?;
      return Result::Ok(__tmp_4);
    } // getbaraidx
    #[allow(unused_parens)]
    pub fn get_bara_idx_i_j(&self,idx_ : i64,i_ : &mut i32,j_ : &mut i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getbaraidxij(self.ptr,idx_,i_,j_) },"get_bara_idx_i_j")?;
      return Result::Ok(());
    } // getbaraidxij
    #[allow(unused_parens)]
    pub fn get_bara_idx_info(&self,idx_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getbaraidxinfo(self.ptr,idx_,&mut __tmp_0) },"get_bara_idx_info")?;
      return Result::Ok(__tmp_0);
    } // getbaraidxinfo
    #[allow(unused_parens)]
    pub fn get_bara_sparsity(&self,numnz_ : &mut i64,idxij_ : &mut[i64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getnumbaranz(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumbaranz")?;
      let maxnumnz_ : i64 = __tmp_0;
      if idxij_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("Argument 'idxij' has the wrong length, expected maxnumnz_".to_string());
      }
      self.handle_res(unsafe { MSK_getbarasparsity(self.ptr,maxnumnz_,numnz_,idxij_.as_mut_ptr()) },"get_bara_sparsity")?;
      return Result::Ok(());
    } // getbarasparsity
    #[allow(unused_parens)]
    pub fn get_barc_block_triplet(&self,subj_ : &mut[i32],subk_ : &mut[i32],subl_ : &mut[i32],valjkl_ : &mut[f64]) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getnumbarcblocktriplets(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumbarcblocktriplets")?;
      let maxnum_ : i64 = __tmp_0;
      let mut __tmp_2 : i64 = i64::default();
      if subj_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("Argument 'subj' has the wrong length, expected maxnum_".to_string());
      }
      if subk_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("Argument 'subk' has the wrong length, expected maxnum_".to_string());
      }
      if subl_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("Argument 'subl' has the wrong length, expected maxnum_".to_string());
      }
      if valjkl_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("Argument 'valjkl' has the wrong length, expected maxnum_".to_string());
      }
      self.handle_res(unsafe { MSK_getbarcblocktriplet(self.ptr,maxnum_,&mut __tmp_2,subj_.as_mut_ptr(),subk_.as_mut_ptr(),subl_.as_mut_ptr(),valjkl_.as_mut_ptr()) },"get_barc_block_triplet")?;
      return Result::Ok(__tmp_2);
    } // getbarcblocktriplet
    #[allow(unused_parens)]
    pub fn get_barc_idx(&self,idx_ : i64,j_ : &mut i32,num_ : &mut i64,sub_ : &mut[i64],weights_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getbarcidxinfo(self.ptr,idx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getbarcidxinfo")?;
      let maxnum_ : i64 = __tmp_0;
      if sub_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("Argument 'sub' has the wrong length, expected maxnum_".to_string());
      }
      if weights_.len() != (maxnum_).try_into().unwrap() {
        return Result::Err("Argument 'weights' has the wrong length, expected maxnum_".to_string());
      }
      self.handle_res(unsafe { MSK_getbarcidx(self.ptr,idx_,maxnum_,j_,num_,sub_.as_mut_ptr(),weights_.as_mut_ptr()) },"get_barc_idx")?;
      return Result::Ok(());
    } // getbarcidx
    #[allow(unused_parens)]
    pub fn get_barc_idx_info(&self,idx_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getbarcidxinfo(self.ptr,idx_,&mut __tmp_0) },"get_barc_idx_info")?;
      return Result::Ok(__tmp_0);
    } // getbarcidxinfo
    #[allow(unused_parens)]
    pub fn get_barc_idx_j(&self,idx_ : i64,j_ : &mut i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getbarcidxj(self.ptr,idx_,j_) },"get_barc_idx_j")?;
      return Result::Ok(());
    } // getbarcidxj
    #[allow(unused_parens)]
    pub fn get_barc_sparsity(&self,numnz_ : &mut i64,idxj_ : &mut[i64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getnumbarcnz(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumbarcnz")?;
      let maxnumnz_ : i64 = __tmp_0;
      if idxj_.len() != (maxnumnz_).try_into().unwrap() {
        return Result::Err("Argument 'idxj' has the wrong length, expected maxnumnz_".to_string());
      }
      self.handle_res(unsafe { MSK_getbarcsparsity(self.ptr,maxnumnz_,numnz_,idxj_.as_mut_ptr()) },"get_barc_sparsity")?;
      return Result::Ok(());
    } // getbarcsparsity
    #[allow(unused_parens)]
    pub fn get_bars_j(&self,whichsol_ : i32,j_ : i32,barsj_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getlenbarvarj(self.ptr,j_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getlenbarvarj")?;
      if barsj_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'barsj' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getbarsj(self.ptr,whichsol_,j_,barsj_.as_mut_ptr()) },"get_bars_j")?;
      return Result::Ok(());
    } // getbarsj
    #[allow(unused_parens)]
    pub fn get_bars_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,slicesize_ : i64,barsslice_ : &mut[f64]) -> Result<(),String> {
      if barsslice_.len() != (slicesize_).try_into().unwrap() {
        return Result::Err("Argument 'barsslice' has the wrong length, expected slicesize_".to_string());
      }
      self.handle_res(unsafe { MSK_getbarsslice(self.ptr,whichsol_,first_,last_,slicesize_,barsslice_.as_mut_ptr()) },"get_bars_slice")?;
      return Result::Ok(());
    } // getbarsslice
    #[allow(unused_parens)]
    pub fn get_barvar_name(&self,i_ : i32) -> Result<String,String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getbarvarnamelen(self.ptr,i_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getbarvarnamelen")?;
      let sizename_ : i32 = (1+__tmp_0);
      let mut name_ = Vec::new(); name_.resize(sizename_ as usize,0);
      self.handle_res(unsafe { MSK_getbarvarname(self.ptr,i_,sizename_,name_.as_mut_ptr()) },"get_barvar_name")?;
      return Result::Ok(String::from_utf8_lossy(&name_[..]).into_owned());
    } // getbarvarname
    #[allow(unused_parens)]
    pub fn get_barvar_name_index(&self,somename_ : &str,asgn_ : &mut i32) -> Result<i32,String> {
      let __tmp_1 = CString::new(somename_).unwrap();
      let mut __tmp_3 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getbarvarnameindex(self.ptr,__tmp_1.as_ptr(),asgn_,&mut __tmp_3) },"get_barvar_name_index")?;
      return Result::Ok(__tmp_3);
    } // getbarvarnameindex
    #[allow(unused_parens)]
    pub fn get_barvar_name_len(&self,i_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getbarvarnamelen(self.ptr,i_,&mut __tmp_0) },"get_barvar_name_len")?;
      return Result::Ok(__tmp_0);
    } // getbarvarnamelen
    #[allow(unused_parens)]
    pub fn get_barx_j(&self,whichsol_ : i32,j_ : i32,barxj_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getlenbarvarj(self.ptr,j_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getlenbarvarj")?;
      if barxj_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'barxj' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getbarxj(self.ptr,whichsol_,j_,barxj_.as_mut_ptr()) },"get_barx_j")?;
      return Result::Ok(());
    } // getbarxj
    #[allow(unused_parens)]
    pub fn get_barx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,slicesize_ : i64,barxslice_ : &mut[f64]) -> Result<(),String> {
      if barxslice_.len() != (slicesize_).try_into().unwrap() {
        return Result::Err("Argument 'barxslice' has the wrong length, expected slicesize_".to_string());
      }
      self.handle_res(unsafe { MSK_getbarxslice(self.ptr,whichsol_,first_,last_,slicesize_,barxslice_.as_mut_ptr()) },"get_barx_slice")?;
      return Result::Ok(());
    } // getbarxslice
    #[allow(unused_parens)]
    pub fn get_c(&self,c_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumvar")?;
      if c_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'c' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getc(self.ptr,c_.as_mut_ptr()) },"get_c")?;
      return Result::Ok(());
    } // getc
    #[allow(unused_parens)]
    pub fn get_cfix(&self) -> Result<f64,String> {
      let mut __tmp_0 : f64 = f64::default();
      self.handle_res(unsafe { MSK_getcfix(self.ptr,&mut __tmp_0) },"get_cfix")?;
      return Result::Ok(__tmp_0);
    } // getcfix
    #[allow(unused_parens)]
    pub fn get_c_j(&self,j_ : i32,cj_ : &mut f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getcj(self.ptr,j_,cj_) },"get_c_j")?;
      return Result::Ok(());
    } // getcj
    #[allow(unused_parens)]
    pub fn get_c_list(&self,subj_ : &[i32],c_ : &mut[f64]) -> Result<(),String> {
      let num_ : i32 = subj_.len() as i32;
      if c_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'c' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_getclist(self.ptr,num_,subj_.as_ptr(),c_.as_mut_ptr()) },"get_c_list")?;
      return Result::Ok(());
    } // getclist
    #[allow(unused_parens)]
    pub fn get_con_bound(&self,i_ : i32,bk_ : & mut i32,bl_ : &mut f64,bu_ : &mut f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getconbound(self.ptr,i_,bk_,bl_,bu_) },"get_con_bound")?;
      return Result::Ok(());
    } // getconbound
    #[allow(unused_parens)]
    pub fn get_con_bound_slice(&self,first_ : i32,last_ : i32,bk_ : &mut[i32],bl_ : &mut[f64],bu_ : &mut[f64]) -> Result<(),String> {
      if bk_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'bk' has the wrong length, expected (last_-first_)".to_string());
      }
      if bl_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'bl' has the wrong length, expected (last_-first_)".to_string());
      }
      if bu_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'bu' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getconboundslice(self.ptr,first_,last_,bk_.as_mut_ptr(),bl_.as_mut_ptr(),bu_.as_mut_ptr()) },"get_con_bound_slice")?;
      return Result::Ok(());
    } // getconboundslice
    #[allow(unused_parens)]
    pub fn get_cone_info(&self,k_ : i32,ct_ : & mut i32,conepar_ : &mut f64,nummem_ : &mut i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getconeinfo(self.ptr,k_,ct_,conepar_,nummem_) },"get_cone_info")?;
      return Result::Ok(());
    } // getconeinfo
    #[allow(unused_parens)]
    pub fn get_cone_name(&self,i_ : i32) -> Result<String,String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getconenamelen(self.ptr,i_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getconenamelen")?;
      let sizename_ : i32 = (1+__tmp_0);
      let mut name_ = Vec::new(); name_.resize(sizename_ as usize,0);
      self.handle_res(unsafe { MSK_getconename(self.ptr,i_,sizename_,name_.as_mut_ptr()) },"get_cone_name")?;
      return Result::Ok(String::from_utf8_lossy(&name_[..]).into_owned());
    } // getconename
    #[allow(unused_parens)]
    pub fn get_cone_name_index(&self,somename_ : &str,asgn_ : &mut i32) -> Result<i32,String> {
      let __tmp_1 = CString::new(somename_).unwrap();
      let mut __tmp_3 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getconenameindex(self.ptr,__tmp_1.as_ptr(),asgn_,&mut __tmp_3) },"get_cone_name_index")?;
      return Result::Ok(__tmp_3);
    } // getconenameindex
    #[allow(unused_parens)]
    pub fn get_cone_name_len(&self,i_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getconenamelen(self.ptr,i_,&mut __tmp_0) },"get_cone_name_len")?;
      return Result::Ok(__tmp_0);
    } // getconenamelen
    #[allow(unused_parens)]
    pub fn get_con_name(&self,i_ : i32) -> Result<String,String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getconnamelen(self.ptr,i_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getconnamelen")?;
      let sizename_ : i32 = (1+__tmp_0);
      let mut name_ = Vec::new(); name_.resize(sizename_ as usize,0);
      self.handle_res(unsafe { MSK_getconname(self.ptr,i_,sizename_,name_.as_mut_ptr()) },"get_con_name")?;
      return Result::Ok(String::from_utf8_lossy(&name_[..]).into_owned());
    } // getconname
    #[allow(unused_parens)]
    pub fn get_con_name_index(&self,somename_ : &str,asgn_ : &mut i32) -> Result<i32,String> {
      let __tmp_1 = CString::new(somename_).unwrap();
      let mut __tmp_3 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getconnameindex(self.ptr,__tmp_1.as_ptr(),asgn_,&mut __tmp_3) },"get_con_name_index")?;
      return Result::Ok(__tmp_3);
    } // getconnameindex
    #[allow(unused_parens)]
    pub fn get_con_name_len(&self,i_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getconnamelen(self.ptr,i_,&mut __tmp_0) },"get_con_name_len")?;
      return Result::Ok(__tmp_0);
    } // getconnamelen
    #[allow(unused_parens)]
    pub fn get_c_slice(&self,first_ : i32,last_ : i32,c_ : &mut[f64]) -> Result<(),String> {
      if c_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'c' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getcslice(self.ptr,first_,last_,c_.as_mut_ptr()) },"get_c_slice")?;
      return Result::Ok(());
    } // getcslice
    #[allow(unused_parens)]
    pub fn get_dim_barvar_j(&self,j_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getdimbarvarj(self.ptr,j_,&mut __tmp_0) },"get_dim_barvar_j")?;
      return Result::Ok(__tmp_0);
    } // getdimbarvarj
    #[allow(unused_parens)]
    pub fn get_djc_afe_idx_list(&self,djcidx_ : i64,afeidxlist_ : &mut[i64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getdjcnumafe(self.ptr,djcidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getdjcnumafe")?;
      if afeidxlist_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'afeidxlist' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getdjcafeidxlist(self.ptr,djcidx_,afeidxlist_.as_mut_ptr()) },"get_djc_afe_idx_list")?;
      return Result::Ok(());
    } // getdjcafeidxlist
    #[allow(unused_parens)]
    pub fn get_djc_b(&self,djcidx_ : i64,b_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getdjcnumafe(self.ptr,djcidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getdjcnumafe")?;
      if b_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'b' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getdjcb(self.ptr,djcidx_,b_.as_mut_ptr()) },"get_djc_b")?;
      return Result::Ok(());
    } // getdjcb
    #[allow(unused_parens)]
    pub fn get_djc_domain_idx_list(&self,djcidx_ : i64,domidxlist_ : &mut[i64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getdjcnumdomain(self.ptr,djcidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getdjcnumdomain")?;
      if domidxlist_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'domidxlist' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getdjcdomainidxlist(self.ptr,djcidx_,domidxlist_.as_mut_ptr()) },"get_djc_domain_idx_list")?;
      return Result::Ok(());
    } // getdjcdomainidxlist
    #[allow(unused_parens)]
    pub fn get_djc_name(&self,djcidx_ : i64) -> Result<String,String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getdjcnamelen(self.ptr,djcidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getdjcnamelen")?;
      let sizename_ : i32 = (1+__tmp_0);
      let mut name_ = Vec::new(); name_.resize(sizename_ as usize,0);
      self.handle_res(unsafe { MSK_getdjcname(self.ptr,djcidx_,sizename_,name_.as_mut_ptr()) },"get_djc_name")?;
      return Result::Ok(String::from_utf8_lossy(&name_[..]).into_owned());
    } // getdjcname
    #[allow(unused_parens)]
    pub fn get_djc_name_len(&self,djcidx_ : i64) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getdjcnamelen(self.ptr,djcidx_,&mut __tmp_0) },"get_djc_name_len")?;
      return Result::Ok(__tmp_0);
    } // getdjcnamelen
    #[allow(unused_parens)]
    pub fn get_djc_num_afe(&mut self,djcidx_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getdjcnumafe(self.ptr,djcidx_,&mut __tmp_0) },"get_djc_num_afe")?;
      return Result::Ok(__tmp_0);
    } // getdjcnumafe
    #[allow(unused_parens)]
    pub fn get_djc_num_afe_tot(&mut self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getdjcnumafetot(self.ptr,&mut __tmp_0) },"get_djc_num_afe_tot")?;
      return Result::Ok(__tmp_0);
    } // getdjcnumafetot
    #[allow(unused_parens)]
    pub fn get_djc_num_domain(&mut self,djcidx_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getdjcnumdomain(self.ptr,djcidx_,&mut __tmp_0) },"get_djc_num_domain")?;
      return Result::Ok(__tmp_0);
    } // getdjcnumdomain
    #[allow(unused_parens)]
    pub fn get_djc_num_domain_tot(&mut self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getdjcnumdomaintot(self.ptr,&mut __tmp_0) },"get_djc_num_domain_tot")?;
      return Result::Ok(__tmp_0);
    } // getdjcnumdomaintot
    #[allow(unused_parens)]
    pub fn get_djc_num_term(&mut self,djcidx_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getdjcnumterm(self.ptr,djcidx_,&mut __tmp_0) },"get_djc_num_term")?;
      return Result::Ok(__tmp_0);
    } // getdjcnumterm
    #[allow(unused_parens)]
    pub fn get_djc_num_term_tot(&mut self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getdjcnumtermtot(self.ptr,&mut __tmp_0) },"get_djc_num_term_tot")?;
      return Result::Ok(__tmp_0);
    } // getdjcnumtermtot
    #[allow(unused_parens)]
    pub fn get_djcs(&self,domidxlist_ : &mut[i64],afeidxlist_ : &mut[i64],b_ : &mut[f64],termsizelist_ : &mut[i64],numterms_ : &mut[i64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getdjcnumdomaintot(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getdjcnumdomaintot")?;
      if domidxlist_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'domidxlist' has the wrong length, expected __tmp_0".to_string());
      }
      let mut __tmp_2 : i64 = i64::default();
      let __tmp_3 = unsafe { MSK_getdjcnumafetot(self.ptr,&mut __tmp_2) };let _ = self.handle_res(__tmp_3,"getdjcnumafetot")?;
      if afeidxlist_.len() != (__tmp_2).try_into().unwrap() {
        return Result::Err("Argument 'afeidxlist' has the wrong length, expected __tmp_2".to_string());
      }
      let mut __tmp_4 : i64 = i64::default();
      let __tmp_5 = unsafe { MSK_getdjcnumafetot(self.ptr,&mut __tmp_4) };let _ = self.handle_res(__tmp_5,"getdjcnumafetot")?;
      if b_.len() != (__tmp_4).try_into().unwrap() {
        return Result::Err("Argument 'b' has the wrong length, expected __tmp_4".to_string());
      }
      let mut __tmp_6 : i64 = i64::default();
      let __tmp_7 = unsafe { MSK_getdjcnumtermtot(self.ptr,&mut __tmp_6) };let _ = self.handle_res(__tmp_7,"getdjcnumtermtot")?;
      if termsizelist_.len() != (__tmp_6).try_into().unwrap() {
        return Result::Err("Argument 'termsizelist' has the wrong length, expected __tmp_6".to_string());
      }
      let mut __tmp_8 : i64 = i64::default();
      let __tmp_9 = unsafe { MSK_getnumdjc(self.ptr,&mut __tmp_8) };let _ = self.handle_res(__tmp_9,"getnumdjc")?;
      if numterms_.len() != (__tmp_8).try_into().unwrap() {
        return Result::Err("Argument 'numterms' has the wrong length, expected __tmp_8".to_string());
      }
      self.handle_res(unsafe { MSK_getdjcs(self.ptr,domidxlist_.as_mut_ptr(),afeidxlist_.as_mut_ptr(),b_.as_mut_ptr(),termsizelist_.as_mut_ptr(),numterms_.as_mut_ptr()) },"get_djcs")?;
      return Result::Ok(());
    } // getdjcs
    #[allow(unused_parens)]
    pub fn get_djc_term_size_list(&self,djcidx_ : i64,termsizelist_ : &mut[i64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getdjcnumterm(self.ptr,djcidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getdjcnumterm")?;
      if termsizelist_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'termsizelist' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getdjctermsizelist(self.ptr,djcidx_,termsizelist_.as_mut_ptr()) },"get_djc_term_size_list")?;
      return Result::Ok(());
    } // getdjctermsizelist
    #[allow(unused_parens)]
    pub fn get_domain_n(&self,domidx_ : i64) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getdomainn(self.ptr,domidx_,&mut __tmp_0) },"get_domain_n")?;
      return Result::Ok(__tmp_0);
    } // getdomainn
    #[allow(unused_parens)]
    pub fn get_domain_name(&self,domidx_ : i64) -> Result<String,String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getdomainnamelen(self.ptr,domidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getdomainnamelen")?;
      let sizename_ : i32 = (1+__tmp_0);
      let mut name_ = Vec::new(); name_.resize(sizename_ as usize,0);
      self.handle_res(unsafe { MSK_getdomainname(self.ptr,domidx_,sizename_,name_.as_mut_ptr()) },"get_domain_name")?;
      return Result::Ok(String::from_utf8_lossy(&name_[..]).into_owned());
    } // getdomainname
    #[allow(unused_parens)]
    pub fn get_domain_name_len(&self,domidx_ : i64) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getdomainnamelen(self.ptr,domidx_,&mut __tmp_0) },"get_domain_name_len")?;
      return Result::Ok(__tmp_0);
    } // getdomainnamelen
    #[allow(unused_parens)]
    pub fn get_domain_type(&self,domidx_ : i64) -> Result<i32,String> {
      let mut __tmp_0 : i32 = 0;
      self.handle_res(unsafe { MSK_getdomaintype(self.ptr,domidx_,&mut __tmp_0) },"get_domain_type")?;
      return Result::Ok(__tmp_0);
    } // getdomaintype
    #[allow(unused_parens)]
    pub fn get_dou_inf(&self,whichdinf_ : i32) -> Result<f64,String> {
      let mut __tmp_0 : f64 = f64::default();
      self.handle_res(unsafe { MSK_getdouinf(self.ptr,whichdinf_,&mut __tmp_0) },"get_dou_inf")?;
      return Result::Ok(__tmp_0);
    } // getdouinf
    #[allow(unused_parens)]
    pub fn get_dou_param(&self,param_ : i32) -> Result<f64,String> {
      let mut __tmp_0 : f64 = f64::default();
      self.handle_res(unsafe { MSK_getdouparam(self.ptr,param_,&mut __tmp_0) },"get_dou_param")?;
      return Result::Ok(__tmp_0);
    } // getdouparam
    #[allow(unused_parens)]
    pub fn get_dual_obj(&self,whichsol_ : i32,dualobj_ : &mut f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getdualobj(self.ptr,whichsol_,dualobj_) },"get_dual_obj")?;
      return Result::Ok(());
    } // getdualobj
    #[allow(unused_parens)]
    pub fn get_dual_solution_norms(&self,whichsol_ : i32,nrmy_ : &mut f64,nrmslc_ : &mut f64,nrmsuc_ : &mut f64,nrmslx_ : &mut f64,nrmsux_ : &mut f64,nrmsnx_ : &mut f64,nrmbars_ : &mut f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getdualsolutionnorms(self.ptr,whichsol_,nrmy_,nrmslc_,nrmsuc_,nrmslx_,nrmsux_,nrmsnx_,nrmbars_) },"get_dual_solution_norms")?;
      return Result::Ok(());
    } // getdualsolutionnorms
    #[allow(unused_parens)]
    pub fn get_dviol_acc(&self,whichsol_ : i32,accidxlist_ : &[i64],viol_ : &mut[f64]) -> Result<(),String> {
      let numaccidx_ : i64 = accidxlist_.len() as i64;
      if viol_.len() != (numaccidx_).try_into().unwrap() {
        return Result::Err("Argument 'viol' has the wrong length, expected numaccidx_".to_string());
      }
      self.handle_res(unsafe { MSK_getdviolacc(self.ptr,whichsol_,numaccidx_,accidxlist_.as_ptr(),viol_.as_mut_ptr()) },"get_dviol_acc")?;
      return Result::Ok(());
    } // getdviolacc
    #[allow(unused_parens)]
    pub fn get_dviol_barvar(&self,whichsol_ : i32,sub_ : &[i32],viol_ : &mut[f64]) -> Result<(),String> {
      let num_ : i32 = sub_.len() as i32;
      if viol_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'viol' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_getdviolbarvar(self.ptr,whichsol_,num_,sub_.as_ptr(),viol_.as_mut_ptr()) },"get_dviol_barvar")?;
      return Result::Ok(());
    } // getdviolbarvar
    #[allow(unused_parens)]
    pub fn get_dviol_con(&self,whichsol_ : i32,sub_ : &[i32],viol_ : &mut[f64]) -> Result<(),String> {
      let num_ : i32 = sub_.len() as i32;
      if viol_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'viol' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_getdviolcon(self.ptr,whichsol_,num_,sub_.as_ptr(),viol_.as_mut_ptr()) },"get_dviol_con")?;
      return Result::Ok(());
    } // getdviolcon
    #[allow(unused_parens)]
    pub fn get_dviol_cones(&self,whichsol_ : i32,sub_ : &[i32],viol_ : &mut[f64]) -> Result<(),String> {
      let num_ : i32 = sub_.len() as i32;
      if viol_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'viol' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_getdviolcones(self.ptr,whichsol_,num_,sub_.as_ptr(),viol_.as_mut_ptr()) },"get_dviol_cones")?;
      return Result::Ok(());
    } // getdviolcones
    #[allow(unused_parens)]
    pub fn get_dviol_var(&self,whichsol_ : i32,sub_ : &[i32],viol_ : &mut[f64]) -> Result<(),String> {
      let num_ : i32 = sub_.len() as i32;
      if viol_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'viol' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_getdviolvar(self.ptr,whichsol_,num_,sub_.as_ptr(),viol_.as_mut_ptr()) },"get_dviol_var")?;
      return Result::Ok(());
    } // getdviolvar
    #[allow(unused_parens)]
    pub fn get_infeasible_sub_problem(&self,whichsol_ : i32) -> Result<Task,String> {
      let mut ptr_inftask : * const u8 = std::ptr::null();
      self.handle_res(unsafe { MSK_getinfeasiblesubproblem(self.ptr,whichsol_,& mut ptr_inftask) },"get_infeasible_sub_problem")?;
      let res_inftask = Task { ptr : ptr_inftask, streamcb : [None,None,None,None], valuecb : None };
      return Result::Ok(res_inftask);
    } // getinfeasiblesubproblem
    #[allow(unused_parens)]
    pub fn get_inf_index(&self,inftype_ : i32,infname_ : &str,infindex_ : &mut i32) -> Result<(),String> {
      let __tmp_1 = CString::new(infname_).unwrap();
      self.handle_res(unsafe { MSK_getinfindex(self.ptr,inftype_,__tmp_1.as_ptr(),infindex_) },"get_inf_index")?;
      return Result::Ok(());
    } // getinfindex
    #[allow(unused_parens)]
    pub fn get_inf_max(&self,inftype_ : i32,infmax_ : &mut[i32]) -> Result<(),String> {
      if infmax_.len() != (MSK_MAX_STR_LEN).try_into().unwrap() {
        return Result::Err("Argument 'infmax' has the wrong length, expected MSK_MAX_STR_LEN".to_string());
      }
      self.handle_res(unsafe { MSK_getinfmax(self.ptr,inftype_,infmax_.as_mut_ptr()) },"get_inf_max")?;
      return Result::Ok(());
    } // getinfmax
    #[allow(unused_parens)]
    pub fn get_inf_name(&self,inftype_ : i32,whichinf_ : i32) -> Result<String,String> {
      let mut infname_ = Vec::new(); infname_.resize(MSK_MAX_STR_LEN as usize,0);
      self.handle_res(unsafe { MSK_getinfname(self.ptr,inftype_,whichinf_,infname_.as_mut_ptr()) },"get_inf_name")?;
      return Result::Ok(String::from_utf8_lossy(&infname_[..]).into_owned());
    } // getinfname
    #[allow(unused_parens)]
    pub fn get_int_inf(&self,whichiinf_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getintinf(self.ptr,whichiinf_,&mut __tmp_0) },"get_int_inf")?;
      return Result::Ok(__tmp_0);
    } // getintinf
    #[allow(unused_parens)]
    pub fn get_int_param(&self,param_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getintparam(self.ptr,param_,&mut __tmp_0) },"get_int_param")?;
      return Result::Ok(__tmp_0);
    } // getintparam
    #[allow(unused_parens)]
    pub fn get_len_barvar_j(&self,j_ : i32) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getlenbarvarj(self.ptr,j_,&mut __tmp_0) },"get_len_barvar_j")?;
      return Result::Ok(__tmp_0);
    } // getlenbarvarj
    #[allow(unused_parens)]
    pub fn get_lint_inf(&self,whichliinf_ : i32) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getlintinf(self.ptr,whichliinf_,&mut __tmp_0) },"get_lint_inf")?;
      return Result::Ok(__tmp_0);
    } // getlintinf
    #[allow(unused_parens)]
    pub fn get_max_name_len(&self,maxlen_ : &mut i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getmaxnamelen(self.ptr,maxlen_) },"get_max_name_len")?;
      return Result::Ok(());
    } // getmaxnamelen
    #[allow(unused_parens)]
    pub fn get_max_num_a_nz(&self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getmaxnumanz64(self.ptr,&mut __tmp_0) },"get_max_num_a_nz")?;
      return Result::Ok(__tmp_0);
    } // getmaxnumanz64
    #[allow(unused_parens)]
    pub fn get_max_num_barvar(&self) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getmaxnumbarvar(self.ptr,&mut __tmp_0) },"get_max_num_barvar")?;
      return Result::Ok(__tmp_0);
    } // getmaxnumbarvar
    #[allow(unused_parens)]
    pub fn get_max_num_con(&self,maxnumcon_ : &mut i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getmaxnumcon(self.ptr,maxnumcon_) },"get_max_num_con")?;
      return Result::Ok(());
    } // getmaxnumcon
    #[allow(unused_parens)]
    pub fn get_max_num_cone(&self,maxnumcone_ : &mut i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getmaxnumcone(self.ptr,maxnumcone_) },"get_max_num_cone")?;
      return Result::Ok(());
    } // getmaxnumcone
    #[allow(unused_parens)]
    pub fn get_max_num_q_nz(&self,maxnumqnz_ : &mut i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getmaxnumqnz64(self.ptr,maxnumqnz_) },"get_max_num_q_nz")?;
      return Result::Ok(());
    } // getmaxnumqnz64
    #[allow(unused_parens)]
    pub fn get_max_num_var(&self,maxnumvar_ : &mut i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getmaxnumvar(self.ptr,maxnumvar_) },"get_max_num_var")?;
      return Result::Ok(());
    } // getmaxnumvar
    #[allow(unused_parens)]
    pub fn get_mem_usage(&self,meminuse_ : &mut i64,maxmemuse_ : &mut i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getmemusagetask(self.ptr,meminuse_,maxmemuse_) },"get_mem_usage")?;
      return Result::Ok(());
    } // getmemusagetask
    #[allow(unused_parens)]
    pub fn get_mio_num_threads(&self) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getmionumthreads(self.ptr,&mut __tmp_0) },"get_mio_num_threads")?;
      return Result::Ok(__tmp_0);
    } // getmionumthreads
    #[allow(unused_parens)]
    pub fn get_na_dou_inf(&self,infitemname_ : &str,dvalue_ : &mut f64) -> Result<(),String> {
      let __tmp_1 = CString::new(infitemname_).unwrap();
      self.handle_res(unsafe { MSK_getnadouinf(self.ptr,__tmp_1.as_ptr(),dvalue_) },"get_na_dou_inf")?;
      return Result::Ok(());
    } // getnadouinf
    #[allow(unused_parens)]
    pub fn get_na_dou_param(&self,paramname_ : &str,parvalue_ : &mut f64) -> Result<(),String> {
      let __tmp_1 = CString::new(paramname_).unwrap();
      self.handle_res(unsafe { MSK_getnadouparam(self.ptr,__tmp_1.as_ptr(),parvalue_) },"get_na_dou_param")?;
      return Result::Ok(());
    } // getnadouparam
    #[allow(unused_parens)]
    pub fn get_na_int_inf(&self,infitemname_ : &str,ivalue_ : &mut i32) -> Result<(),String> {
      let __tmp_1 = CString::new(infitemname_).unwrap();
      self.handle_res(unsafe { MSK_getnaintinf(self.ptr,__tmp_1.as_ptr(),ivalue_) },"get_na_int_inf")?;
      return Result::Ok(());
    } // getnaintinf
    #[allow(unused_parens)]
    pub fn get_na_int_param(&self,paramname_ : &str,parvalue_ : &mut i32) -> Result<(),String> {
      let __tmp_1 = CString::new(paramname_).unwrap();
      self.handle_res(unsafe { MSK_getnaintparam(self.ptr,__tmp_1.as_ptr(),parvalue_) },"get_na_int_param")?;
      return Result::Ok(());
    } // getnaintparam
    #[allow(unused_parens)]
    pub fn get_na_str_param(&self,paramname_ : &str,sizeparamname_ : i32,len_ : &mut i32) -> Result<String,String> {
      let __tmp_1 = CString::new(paramname_).unwrap();
      let mut parvalue_ = Vec::new(); parvalue_.resize(sizeparamname_ as usize,0);
      self.handle_res(unsafe { MSK_getnastrparam(self.ptr,__tmp_1.as_ptr(),sizeparamname_,len_,parvalue_.as_mut_ptr()) },"get_na_str_param")?;
      return Result::Ok(String::from_utf8_lossy(&parvalue_[..]).into_owned());
    } // getnastrparam
    #[allow(unused_parens)]
    pub fn get_num_acc(&mut self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getnumacc(self.ptr,&mut __tmp_0) },"get_num_acc")?;
      return Result::Ok(__tmp_0);
    } // getnumacc
    #[allow(unused_parens)]
    pub fn get_num_afe(&mut self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getnumafe(self.ptr,&mut __tmp_0) },"get_num_afe")?;
      return Result::Ok(__tmp_0);
    } // getnumafe
    #[allow(unused_parens)]
    pub fn get_num_a_nz(&self) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getnumanz(self.ptr,&mut __tmp_0) },"get_num_a_nz")?;
      return Result::Ok(__tmp_0);
    } // getnumanz
    #[allow(unused_parens)]
    pub fn get_num_a_nz_64(&self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getnumanz64(self.ptr,&mut __tmp_0) },"get_num_a_nz_64")?;
      return Result::Ok(__tmp_0);
    } // getnumanz64
    #[allow(unused_parens)]
    pub fn get_num_bara_block_triplets(&self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getnumbarablocktriplets(self.ptr,&mut __tmp_0) },"get_num_bara_block_triplets")?;
      return Result::Ok(__tmp_0);
    } // getnumbarablocktriplets
    #[allow(unused_parens)]
    pub fn get_num_bara_nz(&self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getnumbaranz(self.ptr,&mut __tmp_0) },"get_num_bara_nz")?;
      return Result::Ok(__tmp_0);
    } // getnumbaranz
    #[allow(unused_parens)]
    pub fn get_num_barc_block_triplets(&self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getnumbarcblocktriplets(self.ptr,&mut __tmp_0) },"get_num_barc_block_triplets")?;
      return Result::Ok(__tmp_0);
    } // getnumbarcblocktriplets
    #[allow(unused_parens)]
    pub fn get_num_barc_nz(&self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getnumbarcnz(self.ptr,&mut __tmp_0) },"get_num_barc_nz")?;
      return Result::Ok(__tmp_0);
    } // getnumbarcnz
    #[allow(unused_parens)]
    pub fn get_num_barvar(&self) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getnumbarvar(self.ptr,&mut __tmp_0) },"get_num_barvar")?;
      return Result::Ok(__tmp_0);
    } // getnumbarvar
    #[allow(unused_parens)]
    pub fn get_num_con(&self) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) },"get_num_con")?;
      return Result::Ok(__tmp_0);
    } // getnumcon
    #[allow(unused_parens)]
    pub fn get_num_cone(&self) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getnumcone(self.ptr,&mut __tmp_0) },"get_num_cone")?;
      return Result::Ok(__tmp_0);
    } // getnumcone
    #[allow(unused_parens)]
    pub fn get_num_cone_mem(&self,k_ : i32,nummem_ : &mut i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getnumconemem(self.ptr,k_,nummem_) },"get_num_cone_mem")?;
      return Result::Ok(());
    } // getnumconemem
    #[allow(unused_parens)]
    pub fn get_num_djc(&mut self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getnumdjc(self.ptr,&mut __tmp_0) },"get_num_djc")?;
      return Result::Ok(__tmp_0);
    } // getnumdjc
    #[allow(unused_parens)]
    pub fn get_num_domain(&mut self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getnumdomain(self.ptr,&mut __tmp_0) },"get_num_domain")?;
      return Result::Ok(__tmp_0);
    } // getnumdomain
    #[allow(unused_parens)]
    pub fn get_num_int_var(&self,numintvar_ : &mut i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getnumintvar(self.ptr,numintvar_) },"get_num_int_var")?;
      return Result::Ok(());
    } // getnumintvar
    #[allow(unused_parens)]
    pub fn get_num_param(&self,partype_ : i32,numparam_ : &mut i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getnumparam(self.ptr,partype_,numparam_) },"get_num_param")?;
      return Result::Ok(());
    } // getnumparam
    #[allow(unused_parens)]
    pub fn get_num_q_con_k_nz(&self,k_ : i32) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getnumqconknz64(self.ptr,k_,&mut __tmp_0) },"get_num_q_con_k_nz")?;
      return Result::Ok(__tmp_0);
    } // getnumqconknz64
    #[allow(unused_parens)]
    pub fn get_num_q_obj_nz(&self) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      self.handle_res(unsafe { MSK_getnumqobjnz64(self.ptr,&mut __tmp_0) },"get_num_q_obj_nz")?;
      return Result::Ok(__tmp_0);
    } // getnumqobjnz64
    #[allow(unused_parens)]
    pub fn get_num_sym_mat(&self,num_ : &mut i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getnumsymmat(self.ptr,num_) },"get_num_sym_mat")?;
      return Result::Ok(());
    } // getnumsymmat
    #[allow(unused_parens)]
    pub fn get_num_var(&self) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getnumvar(self.ptr,&mut __tmp_0) },"get_num_var")?;
      return Result::Ok(__tmp_0);
    } // getnumvar
    #[allow(unused_parens)]
    pub fn get_obj_name(&self) -> Result<String,String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getobjnamelen(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getobjnamelen")?;
      let sizeobjname_ : i32 = (1+__tmp_0);
      let mut objname_ = Vec::new(); objname_.resize(sizeobjname_ as usize,0);
      self.handle_res(unsafe { MSK_getobjname(self.ptr,sizeobjname_,objname_.as_mut_ptr()) },"get_obj_name")?;
      return Result::Ok(String::from_utf8_lossy(&objname_[..]).into_owned());
    } // getobjname
    #[allow(unused_parens)]
    pub fn get_obj_name_len(&self) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getobjnamelen(self.ptr,&mut __tmp_0) },"get_obj_name_len")?;
      return Result::Ok(__tmp_0);
    } // getobjnamelen
    #[allow(unused_parens)]
    pub fn get_obj_sense(&self) -> Result<i32,String> {
      let mut __tmp_0 : i32 = 0;
      self.handle_res(unsafe { MSK_getobjsense(self.ptr,&mut __tmp_0) },"get_obj_sense")?;
      return Result::Ok(__tmp_0);
    } // getobjsense
    #[allow(unused_parens)]
    pub fn get_param_max(&self,partype_ : i32,parammax_ : &mut i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getparammax(self.ptr,partype_,parammax_) },"get_param_max")?;
      return Result::Ok(());
    } // getparammax
    #[allow(unused_parens)]
    pub fn get_param_name(&self,partype_ : i32,param_ : i32) -> Result<String,String> {
      let mut parname_ = Vec::new(); parname_.resize(MSK_MAX_STR_LEN as usize,0);
      self.handle_res(unsafe { MSK_getparamname(self.ptr,partype_,param_,parname_.as_mut_ptr()) },"get_param_name")?;
      return Result::Ok(String::from_utf8_lossy(&parname_[..]).into_owned());
    } // getparamname
    #[allow(unused_parens)]
    pub fn get_power_domain_alpha(&mut self,domidx_ : i64,alpha_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let mut __tmp_1 : i64 = i64::default();
      let __tmp_2 = unsafe { MSK_getpowerdomaininfo(self.ptr,domidx_,&mut __tmp_0,&mut __tmp_1) };let _ = self.handle_res(__tmp_2,"getpowerdomaininfo")?;
      if alpha_.len() != (__tmp_1).try_into().unwrap() {
        return Result::Err("Argument 'alpha' has the wrong length, expected __tmp_1".to_string());
      }
      self.handle_res(unsafe { MSK_getpowerdomainalpha(self.ptr,domidx_,alpha_.as_mut_ptr()) },"get_power_domain_alpha")?;
      return Result::Ok(());
    } // getpowerdomainalpha
    #[allow(unused_parens)]
    pub fn get_power_domain_info(&mut self,domidx_ : i64,n_ : &mut i64,nleft_ : &mut i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getpowerdomaininfo(self.ptr,domidx_,n_,nleft_) },"get_power_domain_info")?;
      return Result::Ok(());
    } // getpowerdomaininfo
    #[allow(unused_parens)]
    pub fn get_primal_obj(&self,whichsol_ : i32) -> Result<f64,String> {
      let mut __tmp_0 : f64 = f64::default();
      self.handle_res(unsafe { MSK_getprimalobj(self.ptr,whichsol_,&mut __tmp_0) },"get_primal_obj")?;
      return Result::Ok(__tmp_0);
    } // getprimalobj
    #[allow(unused_parens)]
    pub fn get_primal_solution_norms(&self,whichsol_ : i32,nrmxc_ : &mut f64,nrmxx_ : &mut f64,nrmbarx_ : &mut f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getprimalsolutionnorms(self.ptr,whichsol_,nrmxc_,nrmxx_,nrmbarx_) },"get_primal_solution_norms")?;
      return Result::Ok(());
    } // getprimalsolutionnorms
    #[allow(unused_parens)]
    pub fn get_prob_type(&self) -> Result<i32,String> {
      let mut __tmp_0 : i32 = 0;
      self.handle_res(unsafe { MSK_getprobtype(self.ptr,&mut __tmp_0) },"get_prob_type")?;
      return Result::Ok(__tmp_0);
    } // getprobtype
    #[allow(unused_parens)]
    pub fn get_pro_sta(&self,whichsol_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = 0;
      self.handle_res(unsafe { MSK_getprosta(self.ptr,whichsol_,&mut __tmp_0) },"get_pro_sta")?;
      return Result::Ok(__tmp_0);
    } // getprosta
    #[allow(unused_parens)]
    pub fn get_pviol_acc(&self,whichsol_ : i32,accidxlist_ : &[i64],viol_ : &mut[f64]) -> Result<(),String> {
      let numaccidx_ : i64 = accidxlist_.len() as i64;
      if viol_.len() != (numaccidx_).try_into().unwrap() {
        return Result::Err("Argument 'viol' has the wrong length, expected numaccidx_".to_string());
      }
      self.handle_res(unsafe { MSK_getpviolacc(self.ptr,whichsol_,numaccidx_,accidxlist_.as_ptr(),viol_.as_mut_ptr()) },"get_pviol_acc")?;
      return Result::Ok(());
    } // getpviolacc
    #[allow(unused_parens)]
    pub fn get_pviol_barvar(&self,whichsol_ : i32,sub_ : &[i32],viol_ : &mut[f64]) -> Result<(),String> {
      let num_ : i32 = sub_.len() as i32;
      if viol_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'viol' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_getpviolbarvar(self.ptr,whichsol_,num_,sub_.as_ptr(),viol_.as_mut_ptr()) },"get_pviol_barvar")?;
      return Result::Ok(());
    } // getpviolbarvar
    #[allow(unused_parens)]
    pub fn get_pviol_con(&self,whichsol_ : i32,sub_ : &[i32],viol_ : &mut[f64]) -> Result<(),String> {
      let num_ : i32 = sub_.len() as i32;
      if viol_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'viol' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_getpviolcon(self.ptr,whichsol_,num_,sub_.as_ptr(),viol_.as_mut_ptr()) },"get_pviol_con")?;
      return Result::Ok(());
    } // getpviolcon
    #[allow(unused_parens)]
    pub fn get_pviol_cones(&self,whichsol_ : i32,sub_ : &[i32],viol_ : &mut[f64]) -> Result<(),String> {
      let num_ : i32 = sub_.len() as i32;
      if viol_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'viol' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_getpviolcones(self.ptr,whichsol_,num_,sub_.as_ptr(),viol_.as_mut_ptr()) },"get_pviol_cones")?;
      return Result::Ok(());
    } // getpviolcones
    #[allow(unused_parens)]
    pub fn get_pviol_djc(&self,whichsol_ : i32,djcidxlist_ : &[i64],viol_ : &mut[f64]) -> Result<(),String> {
      let numdjcidx_ : i64 = djcidxlist_.len() as i64;
      if viol_.len() != (numdjcidx_).try_into().unwrap() {
        return Result::Err("Argument 'viol' has the wrong length, expected numdjcidx_".to_string());
      }
      self.handle_res(unsafe { MSK_getpvioldjc(self.ptr,whichsol_,numdjcidx_,djcidxlist_.as_ptr(),viol_.as_mut_ptr()) },"get_pviol_djc")?;
      return Result::Ok(());
    } // getpvioldjc
    #[allow(unused_parens)]
    pub fn get_pviol_var(&self,whichsol_ : i32,sub_ : &[i32],viol_ : &mut[f64]) -> Result<(),String> {
      let num_ : i32 = sub_.len() as i32;
      if viol_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'viol' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_getpviolvar(self.ptr,whichsol_,num_,sub_.as_ptr(),viol_.as_mut_ptr()) },"get_pviol_var")?;
      return Result::Ok(());
    } // getpviolvar
    #[allow(unused_parens)]
    pub fn get_q_con_k(&self,k_ : i32,qcsubi_ : &mut[i32],qcsubj_ : &mut[i32],qcval_ : &mut[f64]) -> Result<i64,String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getnumqconknz64(self.ptr,k_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumqconknz64")?;
      let maxnumqcnz_ : i64 = __tmp_0;
      let mut __tmp_2 : i64 = i64::default();
      let mut __tmp_3 : i64 = i64::default();
      let __tmp_4 = unsafe { MSK_getnumqconknz64(self.ptr,k_,&mut __tmp_3) };let _ = self.handle_res(__tmp_4,"getnumqconknz64")?;
      if qcsubi_.len() != (__tmp_3).try_into().unwrap() {
        return Result::Err("Argument 'qcsubi' has the wrong length, expected __tmp_3".to_string());
      }
      let mut __tmp_5 : i64 = i64::default();
      let __tmp_6 = unsafe { MSK_getnumqconknz64(self.ptr,k_,&mut __tmp_5) };let _ = self.handle_res(__tmp_6,"getnumqconknz64")?;
      if qcsubj_.len() != (__tmp_5).try_into().unwrap() {
        return Result::Err("Argument 'qcsubj' has the wrong length, expected __tmp_5".to_string());
      }
      let mut __tmp_7 : i64 = i64::default();
      let __tmp_8 = unsafe { MSK_getnumqconknz64(self.ptr,k_,&mut __tmp_7) };let _ = self.handle_res(__tmp_8,"getnumqconknz64")?;
      if qcval_.len() != (__tmp_7).try_into().unwrap() {
        return Result::Err("Argument 'qcval' has the wrong length, expected __tmp_7".to_string());
      }
      self.handle_res(unsafe { MSK_getqconk64(self.ptr,k_,maxnumqcnz_,&mut __tmp_2,qcsubi_.as_mut_ptr(),qcsubj_.as_mut_ptr(),qcval_.as_mut_ptr()) },"get_q_con_k")?;
      return Result::Ok(__tmp_2);
    } // getqconk64
    #[allow(unused_parens)]
    pub fn get_q_obj(&self,numqonz_ : &mut i64,qosubi_ : &mut[i32],qosubj_ : &mut[i32],qoval_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getnumqobjnz64(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumqobjnz64")?;
      let maxnumqonz_ : i64 = __tmp_0;
      if qosubi_.len() != (maxnumqonz_).try_into().unwrap() {
        return Result::Err("Argument 'qosubi' has the wrong length, expected maxnumqonz_".to_string());
      }
      if qosubj_.len() != (maxnumqonz_).try_into().unwrap() {
        return Result::Err("Argument 'qosubj' has the wrong length, expected maxnumqonz_".to_string());
      }
      if qoval_.len() != (maxnumqonz_).try_into().unwrap() {
        return Result::Err("Argument 'qoval' has the wrong length, expected maxnumqonz_".to_string());
      }
      self.handle_res(unsafe { MSK_getqobj64(self.ptr,maxnumqonz_,numqonz_,qosubi_.as_mut_ptr(),qosubj_.as_mut_ptr(),qoval_.as_mut_ptr()) },"get_q_obj")?;
      return Result::Ok(());
    } // getqobj64
    #[allow(unused_parens)]
    pub fn get_q_obj_i_j(&self,i_ : i32,j_ : i32,qoij_ : &mut f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getqobjij(self.ptr,i_,j_,qoij_) },"get_q_obj_i_j")?;
      return Result::Ok(());
    } // getqobjij
    #[allow(unused_parens)]
    pub fn get_reduced_costs(&self,whichsol_ : i32,first_ : i32,last_ : i32,redcosts_ : &mut[f64]) -> Result<(),String> {
      if redcosts_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'redcosts' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getreducedcosts(self.ptr,whichsol_,first_,last_,redcosts_.as_mut_ptr()) },"get_reduced_costs")?;
      return Result::Ok(());
    } // getreducedcosts
    #[allow(unused_parens)]
    pub fn get_skc(&self,whichsol_ : i32,skc_ : &mut[i32]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if skc_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'skc' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getskc(self.ptr,whichsol_,skc_.as_mut_ptr()) },"get_skc")?;
      return Result::Ok(());
    } // getskc
    #[allow(unused_parens)]
    pub fn get_skc_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,skc_ : &mut[i32]) -> Result<(),String> {
      if skc_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'skc' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getskcslice(self.ptr,whichsol_,first_,last_,skc_.as_mut_ptr()) },"get_skc_slice")?;
      return Result::Ok(());
    } // getskcslice
    #[allow(unused_parens)]
    pub fn get_skn(&self,whichsol_ : i32,skn_ : &mut[i32]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcone(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcone")?;
      if skn_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'skn' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getskn(self.ptr,whichsol_,skn_.as_mut_ptr()) },"get_skn")?;
      return Result::Ok(());
    } // getskn
    #[allow(unused_parens)]
    pub fn get_skx(&self,whichsol_ : i32,skx_ : &mut[i32]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumvar")?;
      if skx_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'skx' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getskx(self.ptr,whichsol_,skx_.as_mut_ptr()) },"get_skx")?;
      return Result::Ok(());
    } // getskx
    #[allow(unused_parens)]
    pub fn get_skx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,skx_ : &mut[i32]) -> Result<(),String> {
      if skx_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'skx' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getskxslice(self.ptr,whichsol_,first_,last_,skx_.as_mut_ptr()) },"get_skx_slice")?;
      return Result::Ok(());
    } // getskxslice
    #[allow(unused_parens)]
    pub fn get_slc(&self,whichsol_ : i32,slc_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if slc_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'slc' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getslc(self.ptr,whichsol_,slc_.as_mut_ptr()) },"get_slc")?;
      return Result::Ok(());
    } // getslc
    #[allow(unused_parens)]
    pub fn get_slc_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,slc_ : &mut[f64]) -> Result<(),String> {
      if slc_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'slc' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getslcslice(self.ptr,whichsol_,first_,last_,slc_.as_mut_ptr()) },"get_slc_slice")?;
      return Result::Ok(());
    } // getslcslice
    #[allow(unused_parens)]
    pub fn get_slx(&self,whichsol_ : i32,slx_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumvar")?;
      if slx_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'slx' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getslx(self.ptr,whichsol_,slx_.as_mut_ptr()) },"get_slx")?;
      return Result::Ok(());
    } // getslx
    #[allow(unused_parens)]
    pub fn get_slx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,slx_ : &mut[f64]) -> Result<(),String> {
      if slx_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'slx' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getslxslice(self.ptr,whichsol_,first_,last_,slx_.as_mut_ptr()) },"get_slx_slice")?;
      return Result::Ok(());
    } // getslxslice
    #[allow(unused_parens)]
    pub fn get_snx(&self,whichsol_ : i32,snx_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumvar")?;
      if snx_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'snx' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getsnx(self.ptr,whichsol_,snx_.as_mut_ptr()) },"get_snx")?;
      return Result::Ok(());
    } // getsnx
    #[allow(unused_parens)]
    pub fn get_snx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,snx_ : &mut[f64]) -> Result<(),String> {
      if snx_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'snx' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getsnxslice(self.ptr,whichsol_,first_,last_,snx_.as_mut_ptr()) },"get_snx_slice")?;
      return Result::Ok(());
    } // getsnxslice
    #[allow(unused_parens)]
    pub fn get_sol_sta(&self,whichsol_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = 0;
      self.handle_res(unsafe { MSK_getsolsta(self.ptr,whichsol_,&mut __tmp_0) },"get_sol_sta")?;
      return Result::Ok(__tmp_0);
    } // getsolsta
    #[allow(unused_parens)]
    pub fn get_solution(&self,whichsol_ : i32,problemsta_ : & mut i32,solutionsta_ : & mut i32,skc_ : &mut[i32],skx_ : &mut[i32],skn_ : &mut[i32],xc_ : &mut[f64],xx_ : &mut[f64],y_ : &mut[f64],slc_ : &mut[f64],suc_ : &mut[f64],slx_ : &mut[f64],sux_ : &mut[f64],snx_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_2 : i32 = i32::default();
      let __tmp_3 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_2) };let _ = self.handle_res(__tmp_3,"getnumcon")?;
      if skc_.len() != (__tmp_2).try_into().unwrap() {
        return Result::Err("Argument 'skc' has the wrong length, expected __tmp_2".to_string());
      }
      let mut __tmp_4 : i32 = i32::default();
      let __tmp_5 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_4) };let _ = self.handle_res(__tmp_5,"getnumvar")?;
      if skx_.len() != (__tmp_4).try_into().unwrap() {
        return Result::Err("Argument 'skx' has the wrong length, expected __tmp_4".to_string());
      }
      let mut __tmp_6 : i32 = i32::default();
      let __tmp_7 = unsafe { MSK_getnumcone(self.ptr,&mut __tmp_6) };let _ = self.handle_res(__tmp_7,"getnumcone")?;
      if skn_.len() != (__tmp_6).try_into().unwrap() {
        return Result::Err("Argument 'skn' has the wrong length, expected __tmp_6".to_string());
      }
      let mut __tmp_8 : i32 = i32::default();
      let __tmp_9 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_8) };let _ = self.handle_res(__tmp_9,"getnumcon")?;
      if xc_.len() != (__tmp_8).try_into().unwrap() {
        return Result::Err("Argument 'xc' has the wrong length, expected __tmp_8".to_string());
      }
      let mut __tmp_10 : i32 = i32::default();
      let __tmp_11 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_10) };let _ = self.handle_res(__tmp_11,"getnumvar")?;
      if xx_.len() != (__tmp_10).try_into().unwrap() {
        return Result::Err("Argument 'xx' has the wrong length, expected __tmp_10".to_string());
      }
      let mut __tmp_12 : i32 = i32::default();
      let __tmp_13 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_12) };let _ = self.handle_res(__tmp_13,"getnumcon")?;
      if y_.len() != (__tmp_12).try_into().unwrap() {
        return Result::Err("Argument 'y' has the wrong length, expected __tmp_12".to_string());
      }
      let mut __tmp_14 : i32 = i32::default();
      let __tmp_15 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_14) };let _ = self.handle_res(__tmp_15,"getnumcon")?;
      if slc_.len() != (__tmp_14).try_into().unwrap() {
        return Result::Err("Argument 'slc' has the wrong length, expected __tmp_14".to_string());
      }
      let mut __tmp_16 : i32 = i32::default();
      let __tmp_17 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_16) };let _ = self.handle_res(__tmp_17,"getnumcon")?;
      if suc_.len() != (__tmp_16).try_into().unwrap() {
        return Result::Err("Argument 'suc' has the wrong length, expected __tmp_16".to_string());
      }
      let mut __tmp_18 : i32 = i32::default();
      let __tmp_19 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_18) };let _ = self.handle_res(__tmp_19,"getnumvar")?;
      if slx_.len() != (__tmp_18).try_into().unwrap() {
        return Result::Err("Argument 'slx' has the wrong length, expected __tmp_18".to_string());
      }
      let mut __tmp_20 : i32 = i32::default();
      let __tmp_21 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_20) };let _ = self.handle_res(__tmp_21,"getnumvar")?;
      if sux_.len() != (__tmp_20).try_into().unwrap() {
        return Result::Err("Argument 'sux' has the wrong length, expected __tmp_20".to_string());
      }
      let mut __tmp_22 : i32 = i32::default();
      let __tmp_23 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_22) };let _ = self.handle_res(__tmp_23,"getnumvar")?;
      if snx_.len() != (__tmp_22).try_into().unwrap() {
        return Result::Err("Argument 'snx' has the wrong length, expected __tmp_22".to_string());
      }
      self.handle_res(unsafe { MSK_getsolution(self.ptr,whichsol_,problemsta_,solutionsta_,skc_.as_mut_ptr(),skx_.as_mut_ptr(),skn_.as_mut_ptr(),xc_.as_mut_ptr(),xx_.as_mut_ptr(),y_.as_mut_ptr(),slc_.as_mut_ptr(),suc_.as_mut_ptr(),slx_.as_mut_ptr(),sux_.as_mut_ptr(),snx_.as_mut_ptr()) },"get_solution")?;
      return Result::Ok(());
    } // getsolution
    #[allow(unused_parens)]
    pub fn get_solution_info(&self,whichsol_ : i32,pobj_ : &mut f64,pviolcon_ : &mut f64,pviolvar_ : &mut f64,pviolbarvar_ : &mut f64,pviolcone_ : &mut f64,pviolitg_ : &mut f64,dobj_ : &mut f64,dviolcon_ : &mut f64,dviolvar_ : &mut f64,dviolbarvar_ : &mut f64,dviolcone_ : &mut f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getsolutioninfo(self.ptr,whichsol_,pobj_,pviolcon_,pviolvar_,pviolbarvar_,pviolcone_,pviolitg_,dobj_,dviolcon_,dviolvar_,dviolbarvar_,dviolcone_) },"get_solution_info")?;
      return Result::Ok(());
    } // getsolutioninfo
    #[allow(unused_parens)]
    pub fn get_solution_info_new(&self,whichsol_ : i32,pobj_ : &mut f64,pviolcon_ : &mut f64,pviolvar_ : &mut f64,pviolbarvar_ : &mut f64,pviolcone_ : &mut f64,pviolacc_ : &mut f64,pvioldjc_ : &mut f64,pviolitg_ : &mut f64,dobj_ : &mut f64,dviolcon_ : &mut f64,dviolvar_ : &mut f64,dviolbarvar_ : &mut f64,dviolcone_ : &mut f64,dviolacc_ : &mut f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getsolutioninfonew(self.ptr,whichsol_,pobj_,pviolcon_,pviolvar_,pviolbarvar_,pviolcone_,pviolacc_,pvioldjc_,pviolitg_,dobj_,dviolcon_,dviolvar_,dviolbarvar_,dviolcone_,dviolacc_) },"get_solution_info_new")?;
      return Result::Ok(());
    } // getsolutioninfonew
    #[allow(unused_parens)]
    pub fn get_solution_new(&self,whichsol_ : i32,problemsta_ : & mut i32,solutionsta_ : & mut i32,skc_ : &mut[i32],skx_ : &mut[i32],skn_ : &mut[i32],xc_ : &mut[f64],xx_ : &mut[f64],y_ : &mut[f64],slc_ : &mut[f64],suc_ : &mut[f64],slx_ : &mut[f64],sux_ : &mut[f64],snx_ : &mut[f64],doty_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_2 : i32 = i32::default();
      let __tmp_3 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_2) };let _ = self.handle_res(__tmp_3,"getnumcon")?;
      if skc_.len() != (__tmp_2).try_into().unwrap() {
        return Result::Err("Argument 'skc' has the wrong length, expected __tmp_2".to_string());
      }
      let mut __tmp_4 : i32 = i32::default();
      let __tmp_5 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_4) };let _ = self.handle_res(__tmp_5,"getnumvar")?;
      if skx_.len() != (__tmp_4).try_into().unwrap() {
        return Result::Err("Argument 'skx' has the wrong length, expected __tmp_4".to_string());
      }
      let mut __tmp_6 : i32 = i32::default();
      let __tmp_7 = unsafe { MSK_getnumcone(self.ptr,&mut __tmp_6) };let _ = self.handle_res(__tmp_7,"getnumcone")?;
      if skn_.len() != (__tmp_6).try_into().unwrap() {
        return Result::Err("Argument 'skn' has the wrong length, expected __tmp_6".to_string());
      }
      let mut __tmp_8 : i32 = i32::default();
      let __tmp_9 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_8) };let _ = self.handle_res(__tmp_9,"getnumcon")?;
      if xc_.len() != (__tmp_8).try_into().unwrap() {
        return Result::Err("Argument 'xc' has the wrong length, expected __tmp_8".to_string());
      }
      let mut __tmp_10 : i32 = i32::default();
      let __tmp_11 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_10) };let _ = self.handle_res(__tmp_11,"getnumvar")?;
      if xx_.len() != (__tmp_10).try_into().unwrap() {
        return Result::Err("Argument 'xx' has the wrong length, expected __tmp_10".to_string());
      }
      let mut __tmp_12 : i32 = i32::default();
      let __tmp_13 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_12) };let _ = self.handle_res(__tmp_13,"getnumcon")?;
      if y_.len() != (__tmp_12).try_into().unwrap() {
        return Result::Err("Argument 'y' has the wrong length, expected __tmp_12".to_string());
      }
      let mut __tmp_14 : i32 = i32::default();
      let __tmp_15 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_14) };let _ = self.handle_res(__tmp_15,"getnumcon")?;
      if slc_.len() != (__tmp_14).try_into().unwrap() {
        return Result::Err("Argument 'slc' has the wrong length, expected __tmp_14".to_string());
      }
      let mut __tmp_16 : i32 = i32::default();
      let __tmp_17 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_16) };let _ = self.handle_res(__tmp_17,"getnumcon")?;
      if suc_.len() != (__tmp_16).try_into().unwrap() {
        return Result::Err("Argument 'suc' has the wrong length, expected __tmp_16".to_string());
      }
      let mut __tmp_18 : i32 = i32::default();
      let __tmp_19 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_18) };let _ = self.handle_res(__tmp_19,"getnumvar")?;
      if slx_.len() != (__tmp_18).try_into().unwrap() {
        return Result::Err("Argument 'slx' has the wrong length, expected __tmp_18".to_string());
      }
      let mut __tmp_20 : i32 = i32::default();
      let __tmp_21 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_20) };let _ = self.handle_res(__tmp_21,"getnumvar")?;
      if sux_.len() != (__tmp_20).try_into().unwrap() {
        return Result::Err("Argument 'sux' has the wrong length, expected __tmp_20".to_string());
      }
      let mut __tmp_22 : i32 = i32::default();
      let __tmp_23 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_22) };let _ = self.handle_res(__tmp_23,"getnumvar")?;
      if snx_.len() != (__tmp_22).try_into().unwrap() {
        return Result::Err("Argument 'snx' has the wrong length, expected __tmp_22".to_string());
      }
      let mut __tmp_24 : i64 = i64::default();
      let __tmp_25 = unsafe { MSK_getaccntot(self.ptr,&mut __tmp_24) };let _ = self.handle_res(__tmp_25,"getaccntot")?;
      if doty_.len() != (__tmp_24).try_into().unwrap() {
        return Result::Err("Argument 'doty' has the wrong length, expected __tmp_24".to_string());
      }
      self.handle_res(unsafe { MSK_getsolutionnew(self.ptr,whichsol_,problemsta_,solutionsta_,skc_.as_mut_ptr(),skx_.as_mut_ptr(),skn_.as_mut_ptr(),xc_.as_mut_ptr(),xx_.as_mut_ptr(),y_.as_mut_ptr(),slc_.as_mut_ptr(),suc_.as_mut_ptr(),slx_.as_mut_ptr(),sux_.as_mut_ptr(),snx_.as_mut_ptr(),doty_.as_mut_ptr()) },"get_solution_new")?;
      return Result::Ok(());
    } // getsolutionnew
    #[allow(unused_parens)]
    pub fn get_solution_slice(&self,whichsol_ : i32,solitem_ : i32,first_ : i32,last_ : i32,values_ : &mut[f64]) -> Result<(),String> {
      if values_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'values' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getsolutionslice(self.ptr,whichsol_,solitem_,first_,last_,values_.as_mut_ptr()) },"get_solution_slice")?;
      return Result::Ok(());
    } // getsolutionslice
    #[allow(unused_parens)]
    pub fn get_str_param(&self,param_ : i32,len_ : &mut i32) -> Result<String,String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getstrparamlen(self.ptr,param_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getstrparamlen")?;
      let maxlen_ : i32 = (1+__tmp_0);
      let mut parvalue_ = Vec::new(); parvalue_.resize(maxlen_ as usize,0);
      self.handle_res(unsafe { MSK_getstrparam(self.ptr,param_,maxlen_,len_,parvalue_.as_mut_ptr()) },"get_str_param")?;
      return Result::Ok(String::from_utf8_lossy(&parvalue_[..]).into_owned());
    } // getstrparam
    #[allow(unused_parens)]
    pub fn get_str_param_len(&self,param_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getstrparamlen(self.ptr,param_,&mut __tmp_0) },"get_str_param_len")?;
      return Result::Ok(__tmp_0);
    } // getstrparamlen
    #[allow(unused_parens)]
    pub fn get_suc(&self,whichsol_ : i32,suc_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if suc_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'suc' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getsuc(self.ptr,whichsol_,suc_.as_mut_ptr()) },"get_suc")?;
      return Result::Ok(());
    } // getsuc
    #[allow(unused_parens)]
    pub fn get_suc_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,suc_ : &mut[f64]) -> Result<(),String> {
      if suc_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'suc' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getsucslice(self.ptr,whichsol_,first_,last_,suc_.as_mut_ptr()) },"get_suc_slice")?;
      return Result::Ok(());
    } // getsucslice
    #[allow(unused_parens)]
    pub fn get_sux(&self,whichsol_ : i32,sux_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumvar")?;
      if sux_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'sux' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getsux(self.ptr,whichsol_,sux_.as_mut_ptr()) },"get_sux")?;
      return Result::Ok(());
    } // getsux
    #[allow(unused_parens)]
    pub fn get_sux_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,sux_ : &mut[f64]) -> Result<(),String> {
      if sux_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'sux' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getsuxslice(self.ptr,whichsol_,first_,last_,sux_.as_mut_ptr()) },"get_sux_slice")?;
      return Result::Ok(());
    } // getsuxslice
    #[allow(unused_parens)]
    pub fn get_symb_con(&self,i_ : i32,value_ : &mut i32) -> Result<String,String> {
      let sizevalue_ : i32 = MSK_MAX_STR_LEN;
      let mut name_ = Vec::new(); name_.resize(MSK_MAX_STR_LEN as usize,0);
      self.handle_res(unsafe { MSK_getsymbcon(self.ptr,i_,sizevalue_,name_.as_mut_ptr(),value_) },"get_symb_con")?;
      return Result::Ok(String::from_utf8_lossy(&name_[..]).into_owned());
    } // getsymbcon
    #[allow(unused_parens)]
    pub fn get_sym_mat_info(&self,idx_ : i64,dim_ : &mut i32,nz_ : &mut i64,mattype_ : & mut i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getsymmatinfo(self.ptr,idx_,dim_,nz_,mattype_) },"get_sym_mat_info")?;
      return Result::Ok(());
    } // getsymmatinfo
    #[allow(unused_parens)]
    pub fn get_task_name(&self) -> Result<String,String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_gettasknamelen(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"gettasknamelen")?;
      let sizetaskname_ : i32 = (1+__tmp_0);
      let mut taskname_ = Vec::new(); taskname_.resize(sizetaskname_ as usize,0);
      self.handle_res(unsafe { MSK_gettaskname(self.ptr,sizetaskname_,taskname_.as_mut_ptr()) },"get_task_name")?;
      return Result::Ok(String::from_utf8_lossy(&taskname_[..]).into_owned());
    } // gettaskname
    #[allow(unused_parens)]
    pub fn get_task_name_len(&self) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_gettasknamelen(self.ptr,&mut __tmp_0) },"get_task_name_len")?;
      return Result::Ok(__tmp_0);
    } // gettasknamelen
    #[allow(unused_parens)]
    pub fn get_var_bound(&self,i_ : i32,bk_ : & mut i32,bl_ : &mut f64,bu_ : &mut f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_getvarbound(self.ptr,i_,bk_,bl_,bu_) },"get_var_bound")?;
      return Result::Ok(());
    } // getvarbound
    #[allow(unused_parens)]
    pub fn get_var_bound_slice(&self,first_ : i32,last_ : i32,bk_ : &mut[i32],bl_ : &mut[f64],bu_ : &mut[f64]) -> Result<(),String> {
      if bk_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'bk' has the wrong length, expected (last_-first_)".to_string());
      }
      if bl_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'bl' has the wrong length, expected (last_-first_)".to_string());
      }
      if bu_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'bu' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getvarboundslice(self.ptr,first_,last_,bk_.as_mut_ptr(),bl_.as_mut_ptr(),bu_.as_mut_ptr()) },"get_var_bound_slice")?;
      return Result::Ok(());
    } // getvarboundslice
    #[allow(unused_parens)]
    pub fn get_var_name(&self,j_ : i32) -> Result<String,String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getvarnamelen(self.ptr,j_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getvarnamelen")?;
      let sizename_ : i32 = (1+__tmp_0);
      let mut name_ = Vec::new(); name_.resize(sizename_ as usize,0);
      self.handle_res(unsafe { MSK_getvarname(self.ptr,j_,sizename_,name_.as_mut_ptr()) },"get_var_name")?;
      return Result::Ok(String::from_utf8_lossy(&name_[..]).into_owned());
    } // getvarname
    #[allow(unused_parens)]
    pub fn get_var_name_index(&self,somename_ : &str,asgn_ : &mut i32) -> Result<i32,String> {
      let __tmp_1 = CString::new(somename_).unwrap();
      let mut __tmp_3 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getvarnameindex(self.ptr,__tmp_1.as_ptr(),asgn_,&mut __tmp_3) },"get_var_name_index")?;
      return Result::Ok(__tmp_3);
    } // getvarnameindex
    #[allow(unused_parens)]
    pub fn get_var_name_len(&self,i_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      self.handle_res(unsafe { MSK_getvarnamelen(self.ptr,i_,&mut __tmp_0) },"get_var_name_len")?;
      return Result::Ok(__tmp_0);
    } // getvarnamelen
    #[allow(unused_parens)]
    pub fn get_var_type(&self,j_ : i32) -> Result<i32,String> {
      let mut __tmp_0 : i32 = 0;
      self.handle_res(unsafe { MSK_getvartype(self.ptr,j_,&mut __tmp_0) },"get_var_type")?;
      return Result::Ok(__tmp_0);
    } // getvartype
    #[allow(unused_parens)]
    pub fn get_var_type_list(&self,subj_ : &[i32],vartype_ : &mut[i32]) -> Result<(),String> {
      let num_ : i32 = subj_.len() as i32;
      if vartype_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'vartype' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_getvartypelist(self.ptr,num_,subj_.as_ptr(),vartype_.as_mut_ptr()) },"get_var_type_list")?;
      return Result::Ok(());
    } // getvartypelist
    #[allow(unused_parens)]
    pub fn get_xc(&self,whichsol_ : i32,xc_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if xc_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'xc' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getxc(self.ptr,whichsol_,xc_.as_mut_ptr()) },"get_xc")?;
      return Result::Ok(());
    } // getxc
    #[allow(unused_parens)]
    pub fn get_xc_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,xc_ : &mut[f64]) -> Result<(),String> {
      if xc_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'xc' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getxcslice(self.ptr,whichsol_,first_,last_,xc_.as_mut_ptr()) },"get_xc_slice")?;
      return Result::Ok(());
    } // getxcslice
    #[allow(unused_parens)]
    pub fn get_xx(&self,whichsol_ : i32,xx_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumvar")?;
      if xx_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'xx' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_getxx(self.ptr,whichsol_,xx_.as_mut_ptr()) },"get_xx")?;
      return Result::Ok(());
    } // getxx
    #[allow(unused_parens)]
    pub fn get_xx_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,xx_ : &mut[f64]) -> Result<(),String> {
      if xx_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'xx' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getxxslice(self.ptr,whichsol_,first_,last_,xx_.as_mut_ptr()) },"get_xx_slice")?;
      return Result::Ok(());
    } // getxxslice
    #[allow(unused_parens)]
    pub fn get_y(&self,whichsol_ : i32,y_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if y_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'y' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_gety(self.ptr,whichsol_,y_.as_mut_ptr()) },"get_y")?;
      return Result::Ok(());
    } // gety
    #[allow(unused_parens)]
    pub fn get_y_slice(&self,whichsol_ : i32,first_ : i32,last_ : i32,y_ : &mut[f64]) -> Result<(),String> {
      if y_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'y' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_getyslice(self.ptr,whichsol_,first_,last_,y_.as_mut_ptr()) },"get_y_slice")?;
      return Result::Ok(());
    } // getyslice
    #[allow(unused_parens)]
    pub fn infeasibility_report(&mut self,whichstream_ : i32,whichsol_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_infeasibilityreport(self.ptr,whichstream_,whichsol_) },"infeasibility_report")?;
      return Result::Ok(());
    } // infeasibilityreport
    #[allow(unused_parens)]
    pub fn init_basis_solve(&mut self,basis_ : &mut[i32]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if basis_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'basis' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_initbasissolve(self.ptr,basis_.as_mut_ptr()) },"init_basis_solve")?;
      return Result::Ok(());
    } // initbasissolve
    #[allow(unused_parens)]
    pub fn input_data(&mut self,maxnumcon_ : i32,maxnumvar_ : i32,c_ : &[f64],cfix_ : f64,aptrb_ : &[i64],aptre_ : &[i64],asub_ : &[i32],aval_ : &[f64],bkc_ : &[i32],blc_ : &[f64],buc_ : &[f64],bkx_ : &[i32],blx_ : &[f64],bux_ : &[f64]) -> Result<(),String> {
      let numcon_ : i32 = std::cmp::min(std::cmp::min(buc_.len(),bkc_.len()),blc_.len()) as i32;
      let numvar_ : i32 = std::cmp::min(std::cmp::min(std::cmp::min(std::cmp::min(std::cmp::min(c_.len(),blx_.len()),bux_.len()),aptrb_.len()),aptre_.len()),bkx_.len()) as i32;
      self.handle_res(unsafe { MSK_inputdata64(self.ptr,maxnumcon_,maxnumvar_,numcon_,numvar_,c_.as_ptr(),cfix_,aptrb_.as_ptr(),aptre_.as_ptr(),asub_.as_ptr(),aval_.as_ptr(),bkc_.as_ptr(),blc_.as_ptr(),buc_.as_ptr(),bkx_.as_ptr(),blx_.as_ptr(),bux_.as_ptr()) },"input_data")?;
      return Result::Ok(());
    } // inputdata64
    #[allow(unused_parens)]
    pub fn is_dou_par_name(&self,parname_ : &str,param_ : & mut i32) -> Result<(),String> {
      let __tmp_1 = CString::new(parname_).unwrap();
      self.handle_res(unsafe { MSK_isdouparname(self.ptr,__tmp_1.as_ptr(),param_) },"is_dou_par_name")?;
      return Result::Ok(());
    } // isdouparname
    #[allow(unused_parens)]
    pub fn is_int_par_name(&self,parname_ : &str,param_ : & mut i32) -> Result<(),String> {
      let __tmp_1 = CString::new(parname_).unwrap();
      self.handle_res(unsafe { MSK_isintparname(self.ptr,__tmp_1.as_ptr(),param_) },"is_int_par_name")?;
      return Result::Ok(());
    } // isintparname
    #[allow(unused_parens)]
    pub fn is_str_par_name(&self,parname_ : &str,param_ : & mut i32) -> Result<(),String> {
      let __tmp_1 = CString::new(parname_).unwrap();
      self.handle_res(unsafe { MSK_isstrparname(self.ptr,__tmp_1.as_ptr(),param_) },"is_str_par_name")?;
      return Result::Ok(());
    } // isstrparname
    #[allow(unused_parens)]
    pub fn link_file_to_stream(&mut self,whichstream_ : i32,filename_ : &str,append_ : i32) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_linkfiletotaskstream(self.ptr,whichstream_,__tmp_1.as_ptr(),append_) },"link_file_to_stream")?;
      return Result::Ok(());
    } // linkfiletotaskstream
    #[allow(unused_parens)]
    pub fn one_solution_summary(&self,whichstream_ : i32,whichsol_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_onesolutionsummary(self.ptr,whichstream_,whichsol_) },"one_solution_summary")?;
      return Result::Ok(());
    } // onesolutionsummary
    #[allow(unused_parens)]
    pub fn optimize_rmt(&mut self,address_ : &str,accesstoken_ : &str,trmcode_ : & mut i32) -> Result<(),String> {
      let __tmp_1 = CString::new(address_).unwrap();
      let __tmp_3 = CString::new(accesstoken_).unwrap();
      self.handle_res(unsafe { MSK_optimizermt(self.ptr,__tmp_1.as_ptr(),__tmp_3.as_ptr(),trmcode_) },"optimize_rmt")?;
      return Result::Ok(());
    } // optimizermt
    #[allow(unused_parens)]
    pub fn optimizer_summary(&self,whichstream_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_optimizersummary(self.ptr,whichstream_) },"optimizer_summary")?;
      return Result::Ok(());
    } // optimizersummary
    #[allow(unused_parens)]
    pub fn optimize(&mut self) -> Result<i32,String> {
      let mut __tmp_0 : i32 = 0;
      self.handle_res(unsafe { MSK_optimizetrm(self.ptr,&mut __tmp_0) },"optimize")?;
      return Result::Ok(__tmp_0);
    } // optimizetrm
    #[allow(unused_parens)]
    pub fn primal_repair(&mut self,wlc_ : &[f64],wuc_ : &[f64],wlx_ : &[f64],wux_ : &[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if wlc_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'wlc' has the wrong length, expected __tmp_0".to_string());
      }
      let mut __tmp_2 : i32 = i32::default();
      let __tmp_3 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_2) };let _ = self.handle_res(__tmp_3,"getnumcon")?;
      if wuc_.len() != (__tmp_2).try_into().unwrap() {
        return Result::Err("Argument 'wuc' has the wrong length, expected __tmp_2".to_string());
      }
      let mut __tmp_4 : i32 = i32::default();
      let __tmp_5 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_4) };let _ = self.handle_res(__tmp_5,"getnumvar")?;
      if wlx_.len() != (__tmp_4).try_into().unwrap() {
        return Result::Err("Argument 'wlx' has the wrong length, expected __tmp_4".to_string());
      }
      let mut __tmp_6 : i32 = i32::default();
      let __tmp_7 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_6) };let _ = self.handle_res(__tmp_7,"getnumvar")?;
      if wux_.len() != (__tmp_6).try_into().unwrap() {
        return Result::Err("Argument 'wux' has the wrong length, expected __tmp_6".to_string());
      }
      self.handle_res(unsafe { MSK_primalrepair(self.ptr,wlc_.as_ptr(),wuc_.as_ptr(),wlx_.as_ptr(),wux_.as_ptr()) },"primal_repair")?;
      return Result::Ok(());
    } // primalrepair
    #[allow(unused_parens)]
    pub fn primal_sensitivity(&mut self,subi_ : &[i32],marki_ : &[i32],subj_ : &[i32],markj_ : &[i32],leftpricei_ : &mut[f64],rightpricei_ : &mut[f64],leftrangei_ : &mut[f64],rightrangei_ : &mut[f64],leftpricej_ : &mut[f64],rightpricej_ : &mut[f64],leftrangej_ : &mut[f64],rightrangej_ : &mut[f64]) -> Result<(),String> {
      let numi_ : i32 = std::cmp::min(marki_.len(),subi_.len()) as i32;
      let numj_ : i32 = std::cmp::min(subj_.len(),markj_.len()) as i32;
      if leftpricei_.len() != (numi_).try_into().unwrap() {
        return Result::Err("Argument 'leftpricei' has the wrong length, expected numi_".to_string());
      }
      if rightpricei_.len() != (numi_).try_into().unwrap() {
        return Result::Err("Argument 'rightpricei' has the wrong length, expected numi_".to_string());
      }
      if leftrangei_.len() != (numi_).try_into().unwrap() {
        return Result::Err("Argument 'leftrangei' has the wrong length, expected numi_".to_string());
      }
      if rightrangei_.len() != (numi_).try_into().unwrap() {
        return Result::Err("Argument 'rightrangei' has the wrong length, expected numi_".to_string());
      }
      if leftpricej_.len() != (numj_).try_into().unwrap() {
        return Result::Err("Argument 'leftpricej' has the wrong length, expected numj_".to_string());
      }
      if rightpricej_.len() != (numj_).try_into().unwrap() {
        return Result::Err("Argument 'rightpricej' has the wrong length, expected numj_".to_string());
      }
      if leftrangej_.len() != (numj_).try_into().unwrap() {
        return Result::Err("Argument 'leftrangej' has the wrong length, expected numj_".to_string());
      }
      if rightrangej_.len() != (numj_).try_into().unwrap() {
        return Result::Err("Argument 'rightrangej' has the wrong length, expected numj_".to_string());
      }
      self.handle_res(unsafe { MSK_primalsensitivity(self.ptr,numi_,subi_.as_ptr(),marki_.as_ptr(),numj_,subj_.as_ptr(),markj_.as_ptr(),leftpricei_.as_mut_ptr(),rightpricei_.as_mut_ptr(),leftrangei_.as_mut_ptr(),rightrangei_.as_mut_ptr(),leftpricej_.as_mut_ptr(),rightpricej_.as_mut_ptr(),leftrangej_.as_mut_ptr(),rightrangej_.as_mut_ptr()) },"primal_sensitivity")?;
      return Result::Ok(());
    } // primalsensitivity
    #[allow(unused_parens)]
    pub fn print_param(&self) -> Result<(),String> {
      self.handle_res(unsafe { MSK_printparam(self.ptr) },"print_param")?;
      return Result::Ok(());
    } // printparam
    #[allow(unused_parens)]
    pub fn put_acc(&mut self,accidx_ : i64,domidx_ : i64,afeidxlist_ : &[i64],b_ : &[f64]) -> Result<(),String> {
      let numafeidx_ : i64 = afeidxlist_.len() as i64;
      if b_.len() != (numafeidx_).try_into().unwrap() {
        return Result::Err("Argument 'b' has the wrong length, expected numafeidx_".to_string());
      }
      self.handle_res(unsafe { MSK_putacc(self.ptr,accidx_,domidx_,numafeidx_,afeidxlist_.as_ptr(),b_.as_ptr()) },"put_acc")?;
      return Result::Ok(());
    } // putacc
    #[allow(unused_parens)]
    pub fn put_acc_b(&mut self,accidx_ : i64,b_ : &[f64]) -> Result<(),String> {
      let lengthb_ : i64 = b_.len() as i64;
      self.handle_res(unsafe { MSK_putaccb(self.ptr,accidx_,lengthb_,b_.as_ptr()) },"put_acc_b")?;
      return Result::Ok(());
    } // putaccb
    #[allow(unused_parens)]
    pub fn put_acc_b_j(&mut self,accidx_ : i64,j_ : i64,bj_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putaccbj(self.ptr,accidx_,j_,bj_) },"put_acc_b_j")?;
      return Result::Ok(());
    } // putaccbj
    #[allow(unused_parens)]
    pub fn put_acc_dot_y(&self,whichsol_ : i32,accidx_ : i64,doty_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getaccn(self.ptr,accidx_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getaccn")?;
      if doty_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'doty' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_putaccdoty(self.ptr,whichsol_,accidx_,doty_.as_mut_ptr()) },"put_acc_dot_y")?;
      return Result::Ok(());
    } // putaccdoty
    #[allow(unused_parens)]
    pub fn put_acc_list(&mut self,accidxs_ : &[i64],domidxs_ : &[i64],afeidxlist_ : &[i64],b_ : &[f64]) -> Result<(),String> {
      let numaccs_ : i64 = std::cmp::min(domidxs_.len(),accidxs_.len()) as i64;
      let numafeidx_ : i64 = afeidxlist_.len() as i64;
      if b_.len() != (numafeidx_).try_into().unwrap() {
        return Result::Err("Argument 'b' has the wrong length, expected numafeidx_".to_string());
      }
      self.handle_res(unsafe { MSK_putacclist(self.ptr,numaccs_,accidxs_.as_ptr(),domidxs_.as_ptr(),numafeidx_,afeidxlist_.as_ptr(),b_.as_ptr()) },"put_acc_list")?;
      return Result::Ok(());
    } // putacclist
    #[allow(unused_parens)]
    pub fn put_acc_name(&mut self,accidx_ : i64,name_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(name_).unwrap();
      self.handle_res(unsafe { MSK_putaccname(self.ptr,accidx_,__tmp_1.as_ptr()) },"put_acc_name")?;
      return Result::Ok(());
    } // putaccname
    #[allow(unused_parens)]
    pub fn put_a_col(&mut self,j_ : i32,subj_ : &[i32],valj_ : &[f64]) -> Result<(),String> {
      let nzj_ : i32 = std::cmp::min(subj_.len(),valj_.len()) as i32;
      self.handle_res(unsafe { MSK_putacol(self.ptr,j_,nzj_,subj_.as_ptr(),valj_.as_ptr()) },"put_a_col")?;
      return Result::Ok(());
    } // putacol
    #[allow(unused_parens)]
    pub fn put_a_col_list(&mut self,sub_ : &[i32],ptrb_ : &[i64],ptre_ : &[i64],asub_ : &[i32],aval_ : &[f64]) -> Result<(),String> {
      let num_ : i32 = std::cmp::min(std::cmp::min(ptre_.len(),sub_.len()),ptrb_.len()) as i32;
      self.handle_res(unsafe { MSK_putacollist64(self.ptr,num_,sub_.as_ptr(),ptrb_.as_ptr(),ptre_.as_ptr(),asub_.as_ptr(),aval_.as_ptr()) },"put_a_col_list")?;
      return Result::Ok(());
    } // putacollist64
    #[allow(unused_parens)]
    pub fn put_a_col_slice(&mut self,first_ : i32,last_ : i32,ptrb_ : &[i64],ptre_ : &[i64],asub_ : &[i32],aval_ : &[f64]) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putacolslice64(self.ptr,first_,last_,ptrb_.as_ptr(),ptre_.as_ptr(),asub_.as_ptr(),aval_.as_ptr()) },"put_a_col_slice")?;
      return Result::Ok(());
    } // putacolslice64
    #[allow(unused_parens)]
    pub fn put_afe_barf_block_triplet(&mut self,numtrip_ : i64,afeidx_ : &[i64],barvaridx_ : &[i32],subk_ : &[i32],subl_ : &[i32],valkl_ : &[f64]) -> Result<(),String> {
      if afeidx_.len() != (numtrip_).try_into().unwrap() {
        return Result::Err("Argument 'afeidx' has the wrong length, expected numtrip_".to_string());
      }
      if barvaridx_.len() != (numtrip_).try_into().unwrap() {
        return Result::Err("Argument 'barvaridx' has the wrong length, expected numtrip_".to_string());
      }
      if subk_.len() != (numtrip_).try_into().unwrap() {
        return Result::Err("Argument 'subk' has the wrong length, expected numtrip_".to_string());
      }
      if subl_.len() != (numtrip_).try_into().unwrap() {
        return Result::Err("Argument 'subl' has the wrong length, expected numtrip_".to_string());
      }
      if valkl_.len() != (numtrip_).try_into().unwrap() {
        return Result::Err("Argument 'valkl' has the wrong length, expected numtrip_".to_string());
      }
      self.handle_res(unsafe { MSK_putafebarfblocktriplet(self.ptr,numtrip_,afeidx_.as_ptr(),barvaridx_.as_ptr(),subk_.as_ptr(),subl_.as_ptr(),valkl_.as_ptr()) },"put_afe_barf_block_triplet")?;
      return Result::Ok(());
    } // putafebarfblocktriplet
    #[allow(unused_parens)]
    pub fn put_afe_barf_entry(&mut self,afeidx_ : i64,barvaridx_ : i32,termidx_ : &[i64],termweight_ : &[f64]) -> Result<(),String> {
      let numterm_ : i64 = std::cmp::min(termweight_.len(),termidx_.len()) as i64;
      self.handle_res(unsafe { MSK_putafebarfentry(self.ptr,afeidx_,barvaridx_,numterm_,termidx_.as_ptr(),termweight_.as_ptr()) },"put_afe_barf_entry")?;
      return Result::Ok(());
    } // putafebarfentry
    #[allow(unused_parens)]
    pub fn put_afe_barf_entry_list(&mut self,afeidx_ : &[i64],barvaridx_ : &[i32],numterm_ : &[i64],ptrterm_ : &[i64],termidx_ : &[i64],termweight_ : &[f64]) -> Result<(),String> {
      let numafeidx_ : i64 = std::cmp::min(std::cmp::min(std::cmp::min(afeidx_.len(),numterm_.len()),barvaridx_.len()),ptrterm_.len()) as i64;
      let lenterm_ : i64 = std::cmp::min(termweight_.len(),termidx_.len()) as i64;
      self.handle_res(unsafe { MSK_putafebarfentrylist(self.ptr,numafeidx_,afeidx_.as_ptr(),barvaridx_.as_ptr(),numterm_.as_ptr(),ptrterm_.as_ptr(),lenterm_,termidx_.as_ptr(),termweight_.as_ptr()) },"put_afe_barf_entry_list")?;
      return Result::Ok(());
    } // putafebarfentrylist
    #[allow(unused_parens)]
    pub fn put_afe_barf_row(&mut self,afeidx_ : i64,barvaridx_ : &[i32],numterm_ : &[i64],ptrterm_ : &[i64],termidx_ : &[i64],termweight_ : &[f64]) -> Result<(),String> {
      let numentr_ : i32 = std::cmp::min(std::cmp::min(numterm_.len(),barvaridx_.len()),ptrterm_.len()) as i32;
      let lenterm_ : i64 = std::cmp::min(termweight_.len(),termidx_.len()) as i64;
      self.handle_res(unsafe { MSK_putafebarfrow(self.ptr,afeidx_,numentr_,barvaridx_.as_ptr(),numterm_.as_ptr(),ptrterm_.as_ptr(),lenterm_,termidx_.as_ptr(),termweight_.as_ptr()) },"put_afe_barf_row")?;
      return Result::Ok(());
    } // putafebarfrow
    #[allow(unused_parens)]
    pub fn put_afe_f_col(&mut self,varidx_ : i32,afeidx_ : &[i64],val_ : &[f64]) -> Result<(),String> {
      let numnz_ : i64 = std::cmp::min(val_.len(),afeidx_.len()) as i64;
      self.handle_res(unsafe { MSK_putafefcol(self.ptr,varidx_,numnz_,afeidx_.as_ptr(),val_.as_ptr()) },"put_afe_f_col")?;
      return Result::Ok(());
    } // putafefcol
    #[allow(unused_parens)]
    pub fn put_afe_f_entry(&mut self,afeidx_ : i64,varidx_ : i32,value_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putafefentry(self.ptr,afeidx_,varidx_,value_) },"put_afe_f_entry")?;
      return Result::Ok(());
    } // putafefentry
    #[allow(unused_parens)]
    pub fn put_afe_f_entry_list(&mut self,afeidx_ : &[i64],varidx_ : &[i32],val_ : &[f64]) -> Result<(),String> {
      let numentr_ : i64 = std::cmp::min(std::cmp::min(val_.len(),afeidx_.len()),varidx_.len()) as i64;
      self.handle_res(unsafe { MSK_putafefentrylist(self.ptr,numentr_,afeidx_.as_ptr(),varidx_.as_ptr(),val_.as_ptr()) },"put_afe_f_entry_list")?;
      return Result::Ok(());
    } // putafefentrylist
    #[allow(unused_parens)]
    pub fn put_afe_f_row(&mut self,afeidx_ : i64,varidx_ : &[i32],val_ : &[f64]) -> Result<(),String> {
      let numnz_ : i32 = std::cmp::min(val_.len(),varidx_.len()) as i32;
      self.handle_res(unsafe { MSK_putafefrow(self.ptr,afeidx_,numnz_,varidx_.as_ptr(),val_.as_ptr()) },"put_afe_f_row")?;
      return Result::Ok(());
    } // putafefrow
    #[allow(unused_parens)]
    pub fn put_afe_f_row_list(&mut self,afeidx_ : &[i64],numnzrow_ : &[i32],ptrrow_ : &[i64],varidx_ : &[i32],val_ : &[f64]) -> Result<(),String> {
      let numafeidx_ : i64 = std::cmp::min(std::cmp::min(afeidx_.len(),numnzrow_.len()),ptrrow_.len()) as i64;
      let lenidxval_ : i64 = std::cmp::min(val_.len(),varidx_.len()) as i64;
      self.handle_res(unsafe { MSK_putafefrowlist(self.ptr,numafeidx_,afeidx_.as_ptr(),numnzrow_.as_ptr(),ptrrow_.as_ptr(),lenidxval_,varidx_.as_ptr(),val_.as_ptr()) },"put_afe_f_row_list")?;
      return Result::Ok(());
    } // putafefrowlist
    #[allow(unused_parens)]
    pub fn put_afe_g(&mut self,afeidx_ : i64,g_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putafeg(self.ptr,afeidx_,g_) },"put_afe_g")?;
      return Result::Ok(());
    } // putafeg
    #[allow(unused_parens)]
    pub fn put_afe_g_list(&mut self,afeidx_ : &[i64],g_ : &[f64]) -> Result<(),String> {
      let numafeidx_ : i64 = std::cmp::min(afeidx_.len(),g_.len()) as i64;
      self.handle_res(unsafe { MSK_putafeglist(self.ptr,numafeidx_,afeidx_.as_ptr(),g_.as_ptr()) },"put_afe_g_list")?;
      return Result::Ok(());
    } // putafeglist
    #[allow(unused_parens)]
    pub fn put_afe_g_slice(&mut self,first_ : i64,last_ : i64,slice_ : &[f64]) -> Result<(),String> {
      if slice_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'slice' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putafegslice(self.ptr,first_,last_,slice_.as_ptr()) },"put_afe_g_slice")?;
      return Result::Ok(());
    } // putafegslice
    #[allow(unused_parens)]
    pub fn put_aij(&mut self,i_ : i32,j_ : i32,aij_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putaij(self.ptr,i_,j_,aij_) },"put_aij")?;
      return Result::Ok(());
    } // putaij
    #[allow(unused_parens)]
    pub fn put_aij_list(&mut self,subi_ : &[i32],subj_ : &[i32],valij_ : &[f64]) -> Result<(),String> {
      let num_ : i64 = std::cmp::min(std::cmp::min(subj_.len(),valij_.len()),subi_.len()) as i64;
      self.handle_res(unsafe { MSK_putaijlist64(self.ptr,num_,subi_.as_ptr(),subj_.as_ptr(),valij_.as_ptr()) },"put_aij_list")?;
      return Result::Ok(());
    } // putaijlist64
    #[allow(unused_parens)]
    pub fn put_a_row(&mut self,i_ : i32,subi_ : &[i32],vali_ : &[f64]) -> Result<(),String> {
      let nzi_ : i32 = std::cmp::min(vali_.len(),subi_.len()) as i32;
      self.handle_res(unsafe { MSK_putarow(self.ptr,i_,nzi_,subi_.as_ptr(),vali_.as_ptr()) },"put_a_row")?;
      return Result::Ok(());
    } // putarow
    #[allow(unused_parens)]
    pub fn put_a_row_list(&mut self,sub_ : &[i32],ptrb_ : &[i64],ptre_ : &[i64],asub_ : &[i32],aval_ : &[f64]) -> Result<(),String> {
      let num_ : i32 = std::cmp::min(std::cmp::min(ptre_.len(),sub_.len()),ptrb_.len()) as i32;
      self.handle_res(unsafe { MSK_putarowlist64(self.ptr,num_,sub_.as_ptr(),ptrb_.as_ptr(),ptre_.as_ptr(),asub_.as_ptr(),aval_.as_ptr()) },"put_a_row_list")?;
      return Result::Ok(());
    } // putarowlist64
    #[allow(unused_parens)]
    pub fn put_a_row_slice(&mut self,first_ : i32,last_ : i32,ptrb_ : &[i64],ptre_ : &[i64],asub_ : &[i32],aval_ : &[f64]) -> Result<(),String> {
      if ptrb_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'ptrb' has the wrong length, expected (last_-first_)".to_string());
      }
      if ptre_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'ptre' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putarowslice64(self.ptr,first_,last_,ptrb_.as_ptr(),ptre_.as_ptr(),asub_.as_ptr(),aval_.as_ptr()) },"put_a_row_slice")?;
      return Result::Ok(());
    } // putarowslice64
    #[allow(unused_parens)]
    pub fn put_a_truncate_tol(&mut self,tolzero_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putatruncatetol(self.ptr,tolzero_) },"put_a_truncate_tol")?;
      return Result::Ok(());
    } // putatruncatetol
    #[allow(unused_parens)]
    pub fn put_bara_block_triplet(&mut self,num_ : i64,subi_ : &[i32],subj_ : &[i32],subk_ : &[i32],subl_ : &[i32],valijkl_ : &[f64]) -> Result<(),String> {
      if subi_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'subi' has the wrong length, expected num_".to_string());
      }
      if subj_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'subj' has the wrong length, expected num_".to_string());
      }
      if subk_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'subk' has the wrong length, expected num_".to_string());
      }
      if subl_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'subl' has the wrong length, expected num_".to_string());
      }
      if valijkl_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'valijkl' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_putbarablocktriplet(self.ptr,num_,subi_.as_ptr(),subj_.as_ptr(),subk_.as_ptr(),subl_.as_ptr(),valijkl_.as_ptr()) },"put_bara_block_triplet")?;
      return Result::Ok(());
    } // putbarablocktriplet
    #[allow(unused_parens)]
    pub fn put_bara_ij(&mut self,i_ : i32,j_ : i32,sub_ : &[i64],weights_ : &[f64]) -> Result<(),String> {
      let num_ : i64 = std::cmp::min(weights_.len(),sub_.len()) as i64;
      self.handle_res(unsafe { MSK_putbaraij(self.ptr,i_,j_,num_,sub_.as_ptr(),weights_.as_ptr()) },"put_bara_ij")?;
      return Result::Ok(());
    } // putbaraij
    #[allow(unused_parens)]
    pub fn put_bara_ij_list(&mut self,subi_ : &[i32],subj_ : &[i32],alphaptrb_ : &[i64],alphaptre_ : &[i64],matidx_ : &[i64],weights_ : &[f64]) -> Result<(),String> {
      let num_ : i32 = std::cmp::min(std::cmp::min(std::cmp::min(subj_.len(),alphaptre_.len()),alphaptrb_.len()),subi_.len()) as i32;
      self.handle_res(unsafe { MSK_putbaraijlist(self.ptr,num_,subi_.as_ptr(),subj_.as_ptr(),alphaptrb_.as_ptr(),alphaptre_.as_ptr(),matidx_.as_ptr(),weights_.as_ptr()) },"put_bara_ij_list")?;
      return Result::Ok(());
    } // putbaraijlist
    #[allow(unused_parens)]
    pub fn put_bara_row_list(&mut self,subi_ : &[i32],ptrb_ : &[i64],ptre_ : &[i64],subj_ : &[i32],nummat_ : &[i64],matidx_ : &[i64],weights_ : &[f64]) -> Result<(),String> {
      let num_ : i32 = std::cmp::min(std::cmp::min(ptre_.len(),ptrb_.len()),subi_.len()) as i32;
      if nummat_.len() != (subj_.len()).try_into().unwrap() {
        return Result::Err("Argument 'nummat' has the wrong length, expected subj_.len()".to_string());
      }
      let mut __tmp_0 : i64 = i64::default();
      for __tmp_1 in nummat_ { __tmp_0 += __tmp_1; }
      if matidx_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'matidx' has the wrong length, expected __tmp_0".to_string());
      }
      let mut __tmp_2 : i64 = i64::default();
      for __tmp_3 in nummat_ { __tmp_2 += __tmp_3; }
      if weights_.len() != (__tmp_2).try_into().unwrap() {
        return Result::Err("Argument 'weights' has the wrong length, expected __tmp_2".to_string());
      }
      self.handle_res(unsafe { MSK_putbararowlist(self.ptr,num_,subi_.as_ptr(),ptrb_.as_ptr(),ptre_.as_ptr(),subj_.as_ptr(),nummat_.as_ptr(),matidx_.as_ptr(),weights_.as_ptr()) },"put_bara_row_list")?;
      return Result::Ok(());
    } // putbararowlist
    #[allow(unused_parens)]
    pub fn put_barc_block_triplet(&mut self,num_ : i64,subj_ : &[i32],subk_ : &[i32],subl_ : &[i32],valjkl_ : &[f64]) -> Result<(),String> {
      if subj_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'subj' has the wrong length, expected num_".to_string());
      }
      if subk_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'subk' has the wrong length, expected num_".to_string());
      }
      if subl_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'subl' has the wrong length, expected num_".to_string());
      }
      if valjkl_.len() != (num_).try_into().unwrap() {
        return Result::Err("Argument 'valjkl' has the wrong length, expected num_".to_string());
      }
      self.handle_res(unsafe { MSK_putbarcblocktriplet(self.ptr,num_,subj_.as_ptr(),subk_.as_ptr(),subl_.as_ptr(),valjkl_.as_ptr()) },"put_barc_block_triplet")?;
      return Result::Ok(());
    } // putbarcblocktriplet
    #[allow(unused_parens)]
    pub fn put_barc_j(&mut self,j_ : i32,sub_ : &[i64],weights_ : &[f64]) -> Result<(),String> {
      let num_ : i64 = std::cmp::min(weights_.len(),sub_.len()) as i64;
      self.handle_res(unsafe { MSK_putbarcj(self.ptr,j_,num_,sub_.as_ptr(),weights_.as_ptr()) },"put_barc_j")?;
      return Result::Ok(());
    } // putbarcj
    #[allow(unused_parens)]
    pub fn put_bars_j(&mut self,whichsol_ : i32,j_ : i32,barsj_ : &[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getlenbarvarj(self.ptr,j_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getlenbarvarj")?;
      if barsj_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'barsj' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_putbarsj(self.ptr,whichsol_,j_,barsj_.as_ptr()) },"put_bars_j")?;
      return Result::Ok(());
    } // putbarsj
    #[allow(unused_parens)]
    pub fn put_barvar_name(&mut self,j_ : i32,name_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(name_).unwrap();
      self.handle_res(unsafe { MSK_putbarvarname(self.ptr,j_,__tmp_1.as_ptr()) },"put_barvar_name")?;
      return Result::Ok(());
    } // putbarvarname
    #[allow(unused_parens)]
    pub fn put_barx_j(&mut self,whichsol_ : i32,j_ : i32,barxj_ : &[f64]) -> Result<(),String> {
      let mut __tmp_0 : i64 = i64::default();
      let __tmp_1 = unsafe { MSK_getlenbarvarj(self.ptr,j_,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getlenbarvarj")?;
      if barxj_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'barxj' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_putbarxj(self.ptr,whichsol_,j_,barxj_.as_ptr()) },"put_barx_j")?;
      return Result::Ok(());
    } // putbarxj
    #[allow(unused_parens)]
    pub fn put_cfix(&mut self,cfix_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putcfix(self.ptr,cfix_) },"put_cfix")?;
      return Result::Ok(());
    } // putcfix
    #[allow(unused_parens)]
    pub fn put_c_j(&mut self,j_ : i32,cj_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putcj(self.ptr,j_,cj_) },"put_c_j")?;
      return Result::Ok(());
    } // putcj
    #[allow(unused_parens)]
    pub fn put_c_list(&mut self,subj_ : &[i32],val_ : &[f64]) -> Result<(),String> {
      let num_ : i32 = std::cmp::min(subj_.len(),val_.len()) as i32;
      self.handle_res(unsafe { MSK_putclist(self.ptr,num_,subj_.as_ptr(),val_.as_ptr()) },"put_c_list")?;
      return Result::Ok(());
    } // putclist
    #[allow(unused_parens)]
    pub fn put_con_bound(&mut self,i_ : i32,bkc_ : i32,blc_ : f64,buc_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putconbound(self.ptr,i_,bkc_,blc_,buc_) },"put_con_bound")?;
      return Result::Ok(());
    } // putconbound
    #[allow(unused_parens)]
    pub fn put_con_bound_list(&mut self,sub_ : &[i32],bkc_ : &[i32],blc_ : &[f64],buc_ : &[f64]) -> Result<(),String> {
      let num_ : i32 = std::cmp::min(std::cmp::min(std::cmp::min(buc_.len(),blc_.len()),sub_.len()),bkc_.len()) as i32;
      self.handle_res(unsafe { MSK_putconboundlist(self.ptr,num_,sub_.as_ptr(),bkc_.as_ptr(),blc_.as_ptr(),buc_.as_ptr()) },"put_con_bound_list")?;
      return Result::Ok(());
    } // putconboundlist
    #[allow(unused_parens)]
    pub fn put_con_bound_list_const(&mut self,sub_ : &[i32],bkc_ : i32,blc_ : f64,buc_ : f64) -> Result<(),String> {
      let num_ : i32 = sub_.len() as i32;
      self.handle_res(unsafe { MSK_putconboundlistconst(self.ptr,num_,sub_.as_ptr(),bkc_,blc_,buc_) },"put_con_bound_list_const")?;
      return Result::Ok(());
    } // putconboundlistconst
    #[allow(unused_parens)]
    pub fn put_con_bound_slice(&mut self,first_ : i32,last_ : i32,bkc_ : &[i32],blc_ : &[f64],buc_ : &[f64]) -> Result<(),String> {
      if bkc_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'bkc' has the wrong length, expected (last_-first_)".to_string());
      }
      if blc_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'blc' has the wrong length, expected (last_-first_)".to_string());
      }
      if buc_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'buc' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putconboundslice(self.ptr,first_,last_,bkc_.as_ptr(),blc_.as_ptr(),buc_.as_ptr()) },"put_con_bound_slice")?;
      return Result::Ok(());
    } // putconboundslice
    #[allow(unused_parens)]
    pub fn put_con_bound_slice_const(&mut self,first_ : i32,last_ : i32,bkc_ : i32,blc_ : f64,buc_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putconboundsliceconst(self.ptr,first_,last_,bkc_,blc_,buc_) },"put_con_bound_slice_const")?;
      return Result::Ok(());
    } // putconboundsliceconst
    #[allow(unused_parens)]
    pub fn put_cone(&mut self,k_ : i32,ct_ : i32,conepar_ : f64,submem_ : &[i32]) -> Result<(),String> {
      let nummem_ : i32 = submem_.len() as i32;
      self.handle_res(unsafe { MSK_putcone(self.ptr,k_,ct_,conepar_,nummem_,submem_.as_ptr()) },"put_cone")?;
      return Result::Ok(());
    } // putcone
    #[allow(unused_parens)]
    pub fn put_cone_name(&mut self,j_ : i32,name_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(name_).unwrap();
      self.handle_res(unsafe { MSK_putconename(self.ptr,j_,__tmp_1.as_ptr()) },"put_cone_name")?;
      return Result::Ok(());
    } // putconename
    #[allow(unused_parens)]
    pub fn put_con_name(&mut self,i_ : i32,name_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(name_).unwrap();
      self.handle_res(unsafe { MSK_putconname(self.ptr,i_,__tmp_1.as_ptr()) },"put_con_name")?;
      return Result::Ok(());
    } // putconname
    #[allow(unused_parens)]
    pub fn put_con_solution_i(&mut self,i_ : i32,whichsol_ : i32,sk_ : i32,x_ : f64,sl_ : f64,su_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putconsolutioni(self.ptr,i_,whichsol_,sk_,x_,sl_,su_) },"put_con_solution_i")?;
      return Result::Ok(());
    } // putconsolutioni
    #[allow(unused_parens)]
    pub fn put_c_slice(&mut self,first_ : i32,last_ : i32,slice_ : &[f64]) -> Result<(),String> {
      if slice_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'slice' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putcslice(self.ptr,first_,last_,slice_.as_ptr()) },"put_c_slice")?;
      return Result::Ok(());
    } // putcslice
    #[allow(unused_parens)]
    pub fn put_djc(&mut self,djcidx_ : i64,domidxlist_ : &[i64],afeidxlist_ : &[i64],b_ : &[f64],termsizelist_ : &[i64]) -> Result<(),String> {
      let numdomidx_ : i64 = domidxlist_.len() as i64;
      let numafeidx_ : i64 = afeidxlist_.len() as i64;
      if b_.len() != (numafeidx_).try_into().unwrap() {
        return Result::Err("Argument 'b' has the wrong length, expected numafeidx_".to_string());
      }
      let numterms_ : i64 = termsizelist_.len() as i64;
      self.handle_res(unsafe { MSK_putdjc(self.ptr,djcidx_,numdomidx_,domidxlist_.as_ptr(),numafeidx_,afeidxlist_.as_ptr(),b_.as_ptr(),numterms_,termsizelist_.as_ptr()) },"put_djc")?;
      return Result::Ok(());
    } // putdjc
    #[allow(unused_parens)]
    pub fn put_djc_name(&mut self,djcidx_ : i64,name_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(name_).unwrap();
      self.handle_res(unsafe { MSK_putdjcname(self.ptr,djcidx_,__tmp_1.as_ptr()) },"put_djc_name")?;
      return Result::Ok(());
    } // putdjcname
    #[allow(unused_parens)]
    pub fn put_djc_slice(&mut self,idxfirst_ : i64,idxlast_ : i64,domidxlist_ : &[i64],afeidxlist_ : &[i64],b_ : &[f64],termsizelist_ : &[i64],termsindjc_ : &[i64]) -> Result<(),String> {
      let numdomidx_ : i64 = domidxlist_.len() as i64;
      let numafeidx_ : i64 = afeidxlist_.len() as i64;
      if b_.len() != (numafeidx_).try_into().unwrap() {
        return Result::Err("Argument 'b' has the wrong length, expected numafeidx_".to_string());
      }
      let numterms_ : i64 = termsizelist_.len() as i64;
      if termsindjc_.len() != ((idxlast_-idxfirst_)).try_into().unwrap() {
        return Result::Err("Argument 'termsindjc' has the wrong length, expected (idxlast_-idxfirst_)".to_string());
      }
      self.handle_res(unsafe { MSK_putdjcslice(self.ptr,idxfirst_,idxlast_,numdomidx_,domidxlist_.as_ptr(),numafeidx_,afeidxlist_.as_ptr(),b_.as_ptr(),numterms_,termsizelist_.as_ptr(),termsindjc_.as_ptr()) },"put_djc_slice")?;
      return Result::Ok(());
    } // putdjcslice
    #[allow(unused_parens)]
    pub fn put_domain_name(&mut self,domidx_ : i64,name_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(name_).unwrap();
      self.handle_res(unsafe { MSK_putdomainname(self.ptr,domidx_,__tmp_1.as_ptr()) },"put_domain_name")?;
      return Result::Ok(());
    } // putdomainname
    #[allow(unused_parens)]
    pub fn put_dou_param(&mut self,param_ : i32,parvalue_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putdouparam(self.ptr,param_,parvalue_) },"put_dou_param")?;
      return Result::Ok(());
    } // putdouparam
    #[allow(unused_parens)]
    pub fn put_int_param(&mut self,param_ : i32,parvalue_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putintparam(self.ptr,param_,parvalue_) },"put_int_param")?;
      return Result::Ok(());
    } // putintparam
    #[allow(unused_parens)]
    pub fn put_max_num_acc(&mut self,maxnumacc_ : i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putmaxnumacc(self.ptr,maxnumacc_) },"put_max_num_acc")?;
      return Result::Ok(());
    } // putmaxnumacc
    #[allow(unused_parens)]
    pub fn put_max_num_afe(&mut self,maxnumafe_ : i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putmaxnumafe(self.ptr,maxnumafe_) },"put_max_num_afe")?;
      return Result::Ok(());
    } // putmaxnumafe
    #[allow(unused_parens)]
    pub fn put_max_num_a_nz(&mut self,maxnumanz_ : i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putmaxnumanz(self.ptr,maxnumanz_) },"put_max_num_a_nz")?;
      return Result::Ok(());
    } // putmaxnumanz
    #[allow(unused_parens)]
    pub fn put_max_num_barvar(&mut self,maxnumbarvar_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putmaxnumbarvar(self.ptr,maxnumbarvar_) },"put_max_num_barvar")?;
      return Result::Ok(());
    } // putmaxnumbarvar
    #[allow(unused_parens)]
    pub fn put_max_num_con(&mut self,maxnumcon_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putmaxnumcon(self.ptr,maxnumcon_) },"put_max_num_con")?;
      return Result::Ok(());
    } // putmaxnumcon
    #[allow(unused_parens)]
    pub fn put_max_num_cone(&mut self,maxnumcone_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putmaxnumcone(self.ptr,maxnumcone_) },"put_max_num_cone")?;
      return Result::Ok(());
    } // putmaxnumcone
    #[allow(unused_parens)]
    pub fn put_max_num_djc(&mut self,maxnumdjc_ : i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putmaxnumdjc(self.ptr,maxnumdjc_) },"put_max_num_djc")?;
      return Result::Ok(());
    } // putmaxnumdjc
    #[allow(unused_parens)]
    pub fn put_max_num_domain(&mut self,maxnumdomain_ : i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putmaxnumdomain(self.ptr,maxnumdomain_) },"put_max_num_domain")?;
      return Result::Ok(());
    } // putmaxnumdomain
    #[allow(unused_parens)]
    pub fn put_max_num_q_nz(&mut self,maxnumqnz_ : i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putmaxnumqnz(self.ptr,maxnumqnz_) },"put_max_num_q_nz")?;
      return Result::Ok(());
    } // putmaxnumqnz
    #[allow(unused_parens)]
    pub fn put_max_num_var(&mut self,maxnumvar_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putmaxnumvar(self.ptr,maxnumvar_) },"put_max_num_var")?;
      return Result::Ok(());
    } // putmaxnumvar
    #[allow(unused_parens)]
    pub fn put_na_dou_param(&mut self,paramname_ : &str,parvalue_ : f64) -> Result<(),String> {
      let __tmp_1 = CString::new(paramname_).unwrap();
      self.handle_res(unsafe { MSK_putnadouparam(self.ptr,__tmp_1.as_ptr(),parvalue_) },"put_na_dou_param")?;
      return Result::Ok(());
    } // putnadouparam
    #[allow(unused_parens)]
    pub fn put_na_int_param(&mut self,paramname_ : &str,parvalue_ : i32) -> Result<(),String> {
      let __tmp_1 = CString::new(paramname_).unwrap();
      self.handle_res(unsafe { MSK_putnaintparam(self.ptr,__tmp_1.as_ptr(),parvalue_) },"put_na_int_param")?;
      return Result::Ok(());
    } // putnaintparam
    #[allow(unused_parens)]
    pub fn put_na_str_param(&mut self,paramname_ : &str,parvalue_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(paramname_).unwrap();
      let __tmp_3 = CString::new(parvalue_).unwrap();
      self.handle_res(unsafe { MSK_putnastrparam(self.ptr,__tmp_1.as_ptr(),__tmp_3.as_ptr()) },"put_na_str_param")?;
      return Result::Ok(());
    } // putnastrparam
    #[allow(unused_parens)]
    pub fn put_obj_name(&mut self,objname_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(objname_).unwrap();
      self.handle_res(unsafe { MSK_putobjname(self.ptr,__tmp_1.as_ptr()) },"put_obj_name")?;
      return Result::Ok(());
    } // putobjname
    #[allow(unused_parens)]
    pub fn put_obj_sense(&mut self,sense_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putobjsense(self.ptr,sense_) },"put_obj_sense")?;
      return Result::Ok(());
    } // putobjsense
    #[allow(unused_parens)]
    pub fn put_optserver_host(&mut self,host_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(host_).unwrap();
      self.handle_res(unsafe { MSK_putoptserverhost(self.ptr,__tmp_1.as_ptr()) },"put_optserver_host")?;
      return Result::Ok(());
    } // putoptserverhost
    #[allow(unused_parens)]
    pub fn put_param(&mut self,parname_ : &str,parvalue_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(parname_).unwrap();
      let __tmp_3 = CString::new(parvalue_).unwrap();
      self.handle_res(unsafe { MSK_putparam(self.ptr,__tmp_1.as_ptr(),__tmp_3.as_ptr()) },"put_param")?;
      return Result::Ok(());
    } // putparam
    #[allow(unused_parens)]
    pub fn put_q_con(&mut self,qcsubk_ : &[i32],qcsubi_ : &[i32],qcsubj_ : &[i32],qcval_ : &[f64]) -> Result<(),String> {
      let numqcnz_ : i32 = std::cmp::min(std::cmp::min(qcsubj_.len(),qcval_.len()),qcsubi_.len()) as i32;
      self.handle_res(unsafe { MSK_putqcon(self.ptr,numqcnz_,qcsubk_.as_ptr(),qcsubi_.as_ptr(),qcsubj_.as_ptr(),qcval_.as_ptr()) },"put_q_con")?;
      return Result::Ok(());
    } // putqcon
    #[allow(unused_parens)]
    pub fn put_q_con_k(&mut self,k_ : i32,qcsubi_ : &[i32],qcsubj_ : &[i32],qcval_ : &[f64]) -> Result<(),String> {
      let numqcnz_ : i32 = std::cmp::min(std::cmp::min(qcsubj_.len(),qcval_.len()),qcsubi_.len()) as i32;
      self.handle_res(unsafe { MSK_putqconk(self.ptr,k_,numqcnz_,qcsubi_.as_ptr(),qcsubj_.as_ptr(),qcval_.as_ptr()) },"put_q_con_k")?;
      return Result::Ok(());
    } // putqconk
    #[allow(unused_parens)]
    pub fn put_q_obj(&mut self,qosubi_ : &[i32],qosubj_ : &[i32],qoval_ : &[f64]) -> Result<(),String> {
      let numqonz_ : i32 = std::cmp::min(std::cmp::min(qosubj_.len(),qoval_.len()),qosubi_.len()) as i32;
      self.handle_res(unsafe { MSK_putqobj(self.ptr,numqonz_,qosubi_.as_ptr(),qosubj_.as_ptr(),qoval_.as_ptr()) },"put_q_obj")?;
      return Result::Ok(());
    } // putqobj
    #[allow(unused_parens)]
    pub fn put_q_obj_i_j(&mut self,i_ : i32,j_ : i32,qoij_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putqobjij(self.ptr,i_,j_,qoij_) },"put_q_obj_i_j")?;
      return Result::Ok(());
    } // putqobjij
    #[allow(unused_parens)]
    pub fn put_skc(&mut self,whichsol_ : i32,skc_ : &[i32]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if skc_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'skc' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_putskc(self.ptr,whichsol_,skc_.as_ptr()) },"put_skc")?;
      return Result::Ok(());
    } // putskc
    #[allow(unused_parens)]
    pub fn put_skc_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,skc_ : &[i32]) -> Result<(),String> {
      if skc_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'skc' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putskcslice(self.ptr,whichsol_,first_,last_,skc_.as_ptr()) },"put_skc_slice")?;
      return Result::Ok(());
    } // putskcslice
    #[allow(unused_parens)]
    pub fn put_skx(&mut self,whichsol_ : i32,skx_ : &[i32]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumvar")?;
      if skx_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'skx' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_putskx(self.ptr,whichsol_,skx_.as_ptr()) },"put_skx")?;
      return Result::Ok(());
    } // putskx
    #[allow(unused_parens)]
    pub fn put_skx_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,skx_ : &[i32]) -> Result<(),String> {
      if skx_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'skx' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putskxslice(self.ptr,whichsol_,first_,last_,skx_.as_ptr()) },"put_skx_slice")?;
      return Result::Ok(());
    } // putskxslice
    #[allow(unused_parens)]
    pub fn put_slc(&mut self,whichsol_ : i32,slc_ : &[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if slc_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'slc' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_putslc(self.ptr,whichsol_,slc_.as_ptr()) },"put_slc")?;
      return Result::Ok(());
    } // putslc
    #[allow(unused_parens)]
    pub fn put_slc_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,slc_ : &[f64]) -> Result<(),String> {
      if slc_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'slc' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putslcslice(self.ptr,whichsol_,first_,last_,slc_.as_ptr()) },"put_slc_slice")?;
      return Result::Ok(());
    } // putslcslice
    #[allow(unused_parens)]
    pub fn put_slx(&mut self,whichsol_ : i32,slx_ : &[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumvar")?;
      if slx_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'slx' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_putslx(self.ptr,whichsol_,slx_.as_ptr()) },"put_slx")?;
      return Result::Ok(());
    } // putslx
    #[allow(unused_parens)]
    pub fn put_slx_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,slx_ : &[f64]) -> Result<(),String> {
      if slx_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'slx' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putslxslice(self.ptr,whichsol_,first_,last_,slx_.as_ptr()) },"put_slx_slice")?;
      return Result::Ok(());
    } // putslxslice
    #[allow(unused_parens)]
    pub fn put_snx(&mut self,whichsol_ : i32,sux_ : &[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumvar")?;
      if sux_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'sux' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_putsnx(self.ptr,whichsol_,sux_.as_ptr()) },"put_snx")?;
      return Result::Ok(());
    } // putsnx
    #[allow(unused_parens)]
    pub fn put_snx_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,snx_ : &[f64]) -> Result<(),String> {
      if snx_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'snx' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putsnxslice(self.ptr,whichsol_,first_,last_,snx_.as_ptr()) },"put_snx_slice")?;
      return Result::Ok(());
    } // putsnxslice
    #[allow(unused_parens)]
    pub fn put_solution(&mut self,whichsol_ : i32,skc_ : &[i32],skx_ : &[i32],skn_ : &[i32],xc_ : &[f64],xx_ : &[f64],y_ : &[f64],slc_ : &[f64],suc_ : &[f64],slx_ : &[f64],sux_ : &[f64],snx_ : &[f64]) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putsolution(self.ptr,whichsol_,skc_.as_ptr(),skx_.as_ptr(),skn_.as_ptr(),xc_.as_ptr(),xx_.as_ptr(),y_.as_ptr(),slc_.as_ptr(),suc_.as_ptr(),slx_.as_ptr(),sux_.as_ptr(),snx_.as_ptr()) },"put_solution")?;
      return Result::Ok(());
    } // putsolution
    #[allow(unused_parens)]
    pub fn put_solution_new(&mut self,whichsol_ : i32,skc_ : &[i32],skx_ : &[i32],skn_ : &[i32],xc_ : &[f64],xx_ : &[f64],y_ : &[f64],slc_ : &[f64],suc_ : &[f64],slx_ : &[f64],sux_ : &[f64],snx_ : &[f64],doty_ : &[f64]) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putsolutionnew(self.ptr,whichsol_,skc_.as_ptr(),skx_.as_ptr(),skn_.as_ptr(),xc_.as_ptr(),xx_.as_ptr(),y_.as_ptr(),slc_.as_ptr(),suc_.as_ptr(),slx_.as_ptr(),sux_.as_ptr(),snx_.as_ptr(),doty_.as_ptr()) },"put_solution_new")?;
      return Result::Ok(());
    } // putsolutionnew
    #[allow(unused_parens)]
    pub fn put_solution_y_i(&mut self,i_ : i32,whichsol_ : i32,y_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putsolutionyi(self.ptr,i_,whichsol_,y_) },"put_solution_y_i")?;
      return Result::Ok(());
    } // putsolutionyi
    #[allow(unused_parens)]
    pub fn put_str_param(&mut self,param_ : i32,parvalue_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(parvalue_).unwrap();
      self.handle_res(unsafe { MSK_putstrparam(self.ptr,param_,__tmp_1.as_ptr()) },"put_str_param")?;
      return Result::Ok(());
    } // putstrparam
    #[allow(unused_parens)]
    pub fn put_suc(&mut self,whichsol_ : i32,suc_ : &[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if suc_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'suc' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_putsuc(self.ptr,whichsol_,suc_.as_ptr()) },"put_suc")?;
      return Result::Ok(());
    } // putsuc
    #[allow(unused_parens)]
    pub fn put_suc_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,suc_ : &[f64]) -> Result<(),String> {
      if suc_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'suc' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putsucslice(self.ptr,whichsol_,first_,last_,suc_.as_ptr()) },"put_suc_slice")?;
      return Result::Ok(());
    } // putsucslice
    #[allow(unused_parens)]
    pub fn put_sux(&mut self,whichsol_ : i32,sux_ : &[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumvar")?;
      if sux_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'sux' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_putsux(self.ptr,whichsol_,sux_.as_ptr()) },"put_sux")?;
      return Result::Ok(());
    } // putsux
    #[allow(unused_parens)]
    pub fn put_sux_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,sux_ : &[f64]) -> Result<(),String> {
      if sux_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'sux' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putsuxslice(self.ptr,whichsol_,first_,last_,sux_.as_ptr()) },"put_sux_slice")?;
      return Result::Ok(());
    } // putsuxslice
    #[allow(unused_parens)]
    pub fn put_task_name(&mut self,taskname_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(taskname_).unwrap();
      self.handle_res(unsafe { MSK_puttaskname(self.ptr,__tmp_1.as_ptr()) },"put_task_name")?;
      return Result::Ok(());
    } // puttaskname
    #[allow(unused_parens)]
    pub fn put_var_bound(&mut self,j_ : i32,bkx_ : i32,blx_ : f64,bux_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putvarbound(self.ptr,j_,bkx_,blx_,bux_) },"put_var_bound")?;
      return Result::Ok(());
    } // putvarbound
    #[allow(unused_parens)]
    pub fn put_var_bound_list(&mut self,sub_ : &[i32],bkx_ : &[i32],blx_ : &[f64],bux_ : &[f64]) -> Result<(),String> {
      let num_ : i32 = std::cmp::min(std::cmp::min(std::cmp::min(bux_.len(),bkx_.len()),blx_.len()),sub_.len()) as i32;
      self.handle_res(unsafe { MSK_putvarboundlist(self.ptr,num_,sub_.as_ptr(),bkx_.as_ptr(),blx_.as_ptr(),bux_.as_ptr()) },"put_var_bound_list")?;
      return Result::Ok(());
    } // putvarboundlist
    #[allow(unused_parens)]
    pub fn put_var_bound_list_const(&mut self,sub_ : &[i32],bkx_ : i32,blx_ : f64,bux_ : f64) -> Result<(),String> {
      let num_ : i32 = sub_.len() as i32;
      self.handle_res(unsafe { MSK_putvarboundlistconst(self.ptr,num_,sub_.as_ptr(),bkx_,blx_,bux_) },"put_var_bound_list_const")?;
      return Result::Ok(());
    } // putvarboundlistconst
    #[allow(unused_parens)]
    pub fn put_var_bound_slice(&mut self,first_ : i32,last_ : i32,bkx_ : &[i32],blx_ : &[f64],bux_ : &[f64]) -> Result<(),String> {
      if bkx_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'bkx' has the wrong length, expected (last_-first_)".to_string());
      }
      if blx_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'blx' has the wrong length, expected (last_-first_)".to_string());
      }
      if bux_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'bux' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putvarboundslice(self.ptr,first_,last_,bkx_.as_ptr(),blx_.as_ptr(),bux_.as_ptr()) },"put_var_bound_slice")?;
      return Result::Ok(());
    } // putvarboundslice
    #[allow(unused_parens)]
    pub fn put_var_bound_slice_const(&mut self,first_ : i32,last_ : i32,bkx_ : i32,blx_ : f64,bux_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putvarboundsliceconst(self.ptr,first_,last_,bkx_,blx_,bux_) },"put_var_bound_slice_const")?;
      return Result::Ok(());
    } // putvarboundsliceconst
    #[allow(unused_parens)]
    pub fn put_var_name(&mut self,j_ : i32,name_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(name_).unwrap();
      self.handle_res(unsafe { MSK_putvarname(self.ptr,j_,__tmp_1.as_ptr()) },"put_var_name")?;
      return Result::Ok(());
    } // putvarname
    #[allow(unused_parens)]
    pub fn put_var_solution_j(&mut self,j_ : i32,whichsol_ : i32,sk_ : i32,x_ : f64,sl_ : f64,su_ : f64,sn_ : f64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putvarsolutionj(self.ptr,j_,whichsol_,sk_,x_,sl_,su_,sn_) },"put_var_solution_j")?;
      return Result::Ok(());
    } // putvarsolutionj
    #[allow(unused_parens)]
    pub fn put_var_type(&mut self,j_ : i32,vartype_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_putvartype(self.ptr,j_,vartype_) },"put_var_type")?;
      return Result::Ok(());
    } // putvartype
    #[allow(unused_parens)]
    pub fn put_var_type_list(&mut self,subj_ : &[i32],vartype_ : &[i32]) -> Result<(),String> {
      let num_ : i32 = std::cmp::min(subj_.len(),vartype_.len()) as i32;
      self.handle_res(unsafe { MSK_putvartypelist(self.ptr,num_,subj_.as_ptr(),vartype_.as_ptr()) },"put_var_type_list")?;
      return Result::Ok(());
    } // putvartypelist
    #[allow(unused_parens)]
    pub fn put_xc(&mut self,whichsol_ : i32,xc_ : &mut[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if xc_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'xc' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_putxc(self.ptr,whichsol_,xc_.as_mut_ptr()) },"put_xc")?;
      return Result::Ok(());
    } // putxc
    #[allow(unused_parens)]
    pub fn put_xc_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,xc_ : &[f64]) -> Result<(),String> {
      if xc_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'xc' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putxcslice(self.ptr,whichsol_,first_,last_,xc_.as_ptr()) },"put_xc_slice")?;
      return Result::Ok(());
    } // putxcslice
    #[allow(unused_parens)]
    pub fn put_xx(&mut self,whichsol_ : i32,xx_ : &[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumvar(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumvar")?;
      if xx_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'xx' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_putxx(self.ptr,whichsol_,xx_.as_ptr()) },"put_xx")?;
      return Result::Ok(());
    } // putxx
    #[allow(unused_parens)]
    pub fn put_xx_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,xx_ : &[f64]) -> Result<(),String> {
      if xx_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'xx' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putxxslice(self.ptr,whichsol_,first_,last_,xx_.as_ptr()) },"put_xx_slice")?;
      return Result::Ok(());
    } // putxxslice
    #[allow(unused_parens)]
    pub fn put_y(&mut self,whichsol_ : i32,y_ : &[f64]) -> Result<(),String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if y_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'y' has the wrong length, expected __tmp_0".to_string());
      }
      self.handle_res(unsafe { MSK_puty(self.ptr,whichsol_,y_.as_ptr()) },"put_y")?;
      return Result::Ok(());
    } // puty
    #[allow(unused_parens)]
    pub fn put_y_slice(&mut self,whichsol_ : i32,first_ : i32,last_ : i32,y_ : &[f64]) -> Result<(),String> {
      if y_.len() != ((last_-first_)).try_into().unwrap() {
        return Result::Err("Argument 'y' has the wrong length, expected (last_-first_)".to_string());
      }
      self.handle_res(unsafe { MSK_putyslice(self.ptr,whichsol_,first_,last_,y_.as_ptr()) },"put_y_slice")?;
      return Result::Ok(());
    } // putyslice
    #[allow(unused_parens)]
    pub fn read_b_solution(&self,filename_ : &str,compress_ : i32) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_readbsolution(self.ptr,__tmp_1.as_ptr(),compress_) },"read_b_solution")?;
      return Result::Ok(());
    } // readbsolution
    #[allow(unused_parens)]
    pub fn read_data(&mut self,filename_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_readdataautoformat(self.ptr,__tmp_1.as_ptr()) },"read_data")?;
      return Result::Ok(());
    } // readdataautoformat
    #[allow(unused_parens)]
    pub fn read_data_format(&mut self,filename_ : &str,format_ : i32,compress_ : i32) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_readdataformat(self.ptr,__tmp_1.as_ptr(),format_,compress_) },"read_data_format")?;
      return Result::Ok(());
    } // readdataformat
    #[allow(unused_parens)]
    pub fn read_json_string(&mut self,data_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(data_).unwrap();
      self.handle_res(unsafe { MSK_readjsonstring(self.ptr,__tmp_1.as_ptr()) },"read_json_string")?;
      return Result::Ok(());
    } // readjsonstring
    #[allow(unused_parens)]
    pub fn read_lp_string(&mut self,data_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(data_).unwrap();
      self.handle_res(unsafe { MSK_readlpstring(self.ptr,__tmp_1.as_ptr()) },"read_lp_string")?;
      return Result::Ok(());
    } // readlpstring
    #[allow(unused_parens)]
    pub fn read_opf_string(&mut self,data_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(data_).unwrap();
      self.handle_res(unsafe { MSK_readopfstring(self.ptr,__tmp_1.as_ptr()) },"read_opf_string")?;
      return Result::Ok(());
    } // readopfstring
    #[allow(unused_parens)]
    pub fn read_param_file(&mut self,filename_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_readparamfile(self.ptr,__tmp_1.as_ptr()) },"read_param_file")?;
      return Result::Ok(());
    } // readparamfile
    #[allow(unused_parens)]
    pub fn read_ptf_string(&mut self,data_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(data_).unwrap();
      self.handle_res(unsafe { MSK_readptfstring(self.ptr,__tmp_1.as_ptr()) },"read_ptf_string")?;
      return Result::Ok(());
    } // readptfstring
    #[allow(unused_parens)]
    pub fn read_solution(&mut self,whichsol_ : i32,filename_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_readsolution(self.ptr,whichsol_,__tmp_1.as_ptr()) },"read_solution")?;
      return Result::Ok(());
    } // readsolution
    #[allow(unused_parens)]
    pub fn read_solution_file(&self,filename_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_readsolutionfile(self.ptr,__tmp_1.as_ptr()) },"read_solution_file")?;
      return Result::Ok(());
    } // readsolutionfile
    #[allow(unused_parens)]
    pub fn read_summary(&mut self,whichstream_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_readsummary(self.ptr,whichstream_) },"read_summary")?;
      return Result::Ok(());
    } // readsummary
    #[allow(unused_parens)]
    pub fn read_task(&mut self,filename_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_readtask(self.ptr,__tmp_1.as_ptr()) },"read_task")?;
      return Result::Ok(());
    } // readtask
    #[allow(unused_parens)]
    pub fn remove_barvars(&mut self,subset_ : &[i32]) -> Result<(),String> {
      let num_ : i32 = subset_.len() as i32;
      self.handle_res(unsafe { MSK_removebarvars(self.ptr,num_,subset_.as_ptr()) },"remove_barvars")?;
      return Result::Ok(());
    } // removebarvars
    #[allow(unused_parens)]
    pub fn remove_cones(&mut self,subset_ : &[i32]) -> Result<(),String> {
      let num_ : i32 = subset_.len() as i32;
      self.handle_res(unsafe { MSK_removecones(self.ptr,num_,subset_.as_ptr()) },"remove_cones")?;
      return Result::Ok(());
    } // removecones
    #[allow(unused_parens)]
    pub fn remove_cons(&mut self,subset_ : &[i32]) -> Result<(),String> {
      let num_ : i32 = subset_.len() as i32;
      self.handle_res(unsafe { MSK_removecons(self.ptr,num_,subset_.as_ptr()) },"remove_cons")?;
      return Result::Ok(());
    } // removecons
    #[allow(unused_parens)]
    pub fn remove_vars(&mut self,subset_ : &[i32]) -> Result<(),String> {
      let num_ : i32 = subset_.len() as i32;
      self.handle_res(unsafe { MSK_removevars(self.ptr,num_,subset_.as_ptr()) },"remove_vars")?;
      return Result::Ok(());
    } // removevars
    #[allow(unused_parens)]
    pub fn resize_task(&mut self,maxnumcon_ : i32,maxnumvar_ : i32,maxnumcone_ : i32,maxnumanz_ : i64,maxnumqnz_ : i64) -> Result<(),String> {
      self.handle_res(unsafe { MSK_resizetask(self.ptr,maxnumcon_,maxnumvar_,maxnumcone_,maxnumanz_,maxnumqnz_) },"resize_task")?;
      return Result::Ok(());
    } // resizetask
    #[allow(unused_parens)]
    pub fn sensitivity_report(&self,whichstream_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_sensitivityreport(self.ptr,whichstream_) },"sensitivity_report")?;
      return Result::Ok(());
    } // sensitivityreport
    #[allow(unused_parens)]
    pub fn set_defaults(&mut self) -> Result<(),String> {
      self.handle_res(unsafe { MSK_setdefaults(self.ptr) },"set_defaults")?;
      return Result::Ok(());
    } // setdefaults
    #[allow(unused_parens)]
    pub fn solution_def(&self,whichsol_ : i32) -> Result<bool,String> {
      let mut __tmp_0 : i32 = 0;
      self.handle_res(unsafe { MSK_solutiondef(self.ptr,whichsol_,&mut __tmp_0) },"solution_def")?;
      return Result::Ok(__tmp_0 != 0);
    } // solutiondef
    #[allow(unused_parens)]
    pub fn solution_summary(&self,whichstream_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_solutionsummary(self.ptr,whichstream_) },"solution_summary")?;
      return Result::Ok(());
    } // solutionsummary
    #[allow(unused_parens)]
    pub fn solve_with_basis(&mut self,transp_ : i32,numnz_ : i32,sub_ : &mut[i32],val_ : &mut[f64]) -> Result<i32,String> {
      let mut __tmp_0 : i32 = i32::default();
      let __tmp_1 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_0) };let _ = self.handle_res(__tmp_1,"getnumcon")?;
      if sub_.len() != (__tmp_0).try_into().unwrap() {
        return Result::Err("Argument 'sub' has the wrong length, expected __tmp_0".to_string());
      }
      let mut __tmp_2 : i32 = i32::default();
      let __tmp_3 = unsafe { MSK_getnumcon(self.ptr,&mut __tmp_2) };let _ = self.handle_res(__tmp_3,"getnumcon")?;
      if val_.len() != (__tmp_2).try_into().unwrap() {
        return Result::Err("Argument 'val' has the wrong length, expected __tmp_2".to_string());
      }
      let mut __tmp_4 : i32 = i32::default();
      self.handle_res(unsafe { MSK_solvewithbasis(self.ptr,transp_,numnz_,sub_.as_mut_ptr(),val_.as_mut_ptr(),&mut __tmp_4) },"solve_with_basis")?;
      return Result::Ok(__tmp_4);
    } // solvewithbasis
    #[allow(unused_parens)]
    pub fn str_to_cone_type(&self,str_ : &str,conetype_ : & mut i32) -> Result<(),String> {
      let __tmp_1 = CString::new(str_).unwrap();
      self.handle_res(unsafe { MSK_strtoconetype(self.ptr,__tmp_1.as_ptr(),conetype_) },"str_to_cone_type")?;
      return Result::Ok(());
    } // strtoconetype
    #[allow(unused_parens)]
    pub fn str_to_sk(&self,str_ : &str,sk_ : & mut i32) -> Result<(),String> {
      let __tmp_1 = CString::new(str_).unwrap();
      self.handle_res(unsafe { MSK_strtosk(self.ptr,__tmp_1.as_ptr(),sk_) },"str_to_sk")?;
      return Result::Ok(());
    } // strtosk
    #[allow(unused_parens)]
    pub fn toconic(&mut self) -> Result<(),String> {
      self.handle_res(unsafe { MSK_toconic(self.ptr) },"toconic")?;
      return Result::Ok(());
    } // toconic
    #[allow(unused_parens)]
    pub fn unlink_func_from_stream(&mut self,whichstream_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_unlinkfuncfromtaskstream(self.ptr,whichstream_) },"unlink_func_from_stream")?;
      return Result::Ok(());
    } // unlinkfuncfromtaskstream
    #[allow(unused_parens)]
    pub fn update_solution_info(&mut self,whichsol_ : i32) -> Result<(),String> {
      self.handle_res(unsafe { MSK_updatesolutioninfo(self.ptr,whichsol_) },"update_solution_info")?;
      return Result::Ok(());
    } // updatesolutioninfo
    #[allow(unused_parens)]
    pub fn which_param(&self,parname_ : &str,partype_ : & mut i32,param_ : &mut i32) -> Result<(),String> {
      let __tmp_1 = CString::new(parname_).unwrap();
      self.handle_res(unsafe { MSK_whichparam(self.ptr,__tmp_1.as_ptr(),partype_,param_) },"which_param")?;
      return Result::Ok(());
    } // whichparam
    #[allow(unused_parens)]
    pub fn write_b_solution(&self,filename_ : &str,compress_ : i32) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_writebsolution(self.ptr,__tmp_1.as_ptr(),compress_) },"write_b_solution")?;
      return Result::Ok(());
    } // writebsolution
    #[allow(unused_parens)]
    pub fn write_data(&self,filename_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_writedata(self.ptr,__tmp_1.as_ptr()) },"write_data")?;
      return Result::Ok(());
    } // writedata
    #[allow(unused_parens)]
    pub fn write_json_sol(&self,filename_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_writejsonsol(self.ptr,__tmp_1.as_ptr()) },"write_json_sol")?;
      return Result::Ok(());
    } // writejsonsol
    #[allow(unused_parens)]
    pub fn write_param_file(&self,filename_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_writeparamfile(self.ptr,__tmp_1.as_ptr()) },"write_param_file")?;
      return Result::Ok(());
    } // writeparamfile
    #[allow(unused_parens)]
    pub fn write_solution(&self,whichsol_ : i32,filename_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_writesolution(self.ptr,whichsol_,__tmp_1.as_ptr()) },"write_solution")?;
      return Result::Ok(());
    } // writesolution
    #[allow(unused_parens)]
    pub fn write_solution_file(&self,filename_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_writesolutionfile(self.ptr,__tmp_1.as_ptr()) },"write_solution_file")?;
      return Result::Ok(());
    } // writesolutionfile
    #[allow(unused_parens)]
    pub fn write_stat(&mut self,filename_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_writestat(self.ptr,__tmp_1.as_ptr()) },"write_stat")?;
      return Result::Ok(());
    } // writestat
    #[allow(unused_parens)]
    pub fn write_task(&self,filename_ : &str) -> Result<(),String> {
      let __tmp_1 = CString::new(filename_).unwrap();
      self.handle_res(unsafe { MSK_writetask(self.ptr,__tmp_1.as_ptr()) },"write_task")?;
      return Result::Ok(());
    } // writetask
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


#[allow(unused_parens)]
pub fn get_build_info() -> Result<String,String> {
  let mut buildstate_ = Vec::new(); buildstate_.resize(MSK_MAX_STR_LEN as usize,0);
  let mut builddate_ = Vec::new(); builddate_.resize(MSK_MAX_STR_LEN as usize,0);
  handle_res_static(unsafe { MSK_getbuildinfo(buildstate_.as_mut_ptr(),builddate_.as_mut_ptr()) },"get_build_info")?;
  return Result::Ok(String::from_utf8_lossy(&buildstate_[..]).into_owned());
} // getbuildinfo
#[allow(unused_parens)]
pub fn get_code_desc(code_ : i32) -> Result<String,String> {
  let mut symname_ = Vec::new(); symname_.resize(MSK_MAX_STR_LEN as usize,0);
  let mut str_ = Vec::new(); str_.resize(MSK_MAX_STR_LEN as usize,0);
  handle_res_static(unsafe { MSK_getcodedesc(code_,symname_.as_mut_ptr(),str_.as_mut_ptr()) },"get_code_desc")?;
  return Result::Ok(String::from_utf8_lossy(&symname_[..]).into_owned());
} // getcodedesc
#[allow(unused_parens)]
pub fn get_response_class(r_ : i32,rc_ : & mut i32) -> Result<(),String> {
  handle_res_static(unsafe { MSK_getresponseclass(r_,rc_) },"get_response_class")?;
  return Result::Ok(());
} // getresponseclass
#[allow(unused_parens)]
pub fn get_version(major_ : &mut i32,minor_ : &mut i32,revision_ : &mut i32) -> Result<(),String> {
  handle_res_static(unsafe { MSK_getversion(major_,minor_,revision_) },"get_version")?;
  return Result::Ok(());
} // getversion
#[allow(unused_parens)]
pub fn is_infinity(value_ : f64) -> Result<(),String> {
  handle_res_static(unsafe { MSK_isinfinity(value_) },"is_infinity")?;
  return Result::Ok(());
} // isinfinity
#[allow(unused_parens)]
pub fn license_cleanup() -> Result<(),String> {
  handle_res_static(unsafe { MSK_licensecleanup() },"license_cleanup")?;
  return Result::Ok(());
} // licensecleanup
#[allow(unused_parens)]
pub fn sym_nam_to_value(name_ : &str) -> Result<String,String> {
  let __tmp_1 = CString::new(name_).unwrap();
  let mut value_ = Vec::new(); value_.resize(MSK_MAX_STR_LEN as usize,0);
  handle_res_static(unsafe { MSK_symnamtovalue(__tmp_1.as_ptr(),value_.as_mut_ptr()) },"sym_nam_to_value")?;
  return Result::Ok(String::from_utf8_lossy(&value_[..]).into_owned());
} // symnamtovalue

