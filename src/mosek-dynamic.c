#include "mosek-dynamic.h"
#include <string.h>
#include <stdlib.h>


#define MSK_MAJORVER "12"
#define MSK_MINORVER "0"


#ifdef _WIN32
    #include <windows.h>
    #include <libloaderapi.h>
    #define libname "mosek64_" MSK_MAJORVER "_" MSK_MINORVER ".dll"
    typedef HMODULE libhandle_t;

    libhandle_t __dlopen(const char * filename, const char ** errmsg) {
        libhandle_t h = LoadLibraryA(filename);
        if (!h) {
            *errmsg = "Failed to load MOSEK Core library";
        }
        return h;
    }
    void __dlclose(libhandle_t h) {
        FreeLibrary(h);
    }

    void * __loadsym(libhandle_t h, const char * symname, const char *(* errmsg)) {
        void * symaddr = GetProcAddress(h,symname);
        if (!symaddr)
            *errmsg = "Failed to load symbol";
        return symaddr;
    }
    static const char PATH_SEP = '\\';
#else
    #include <dlfcn.h>
    typedef void * libhandle_t;
    #if defined(__APPLE__)
        #define libname "libmosek64." MSK_MAJORVER "." MSK_MINORVER ".dylib"
    #else
        #define libname "libmosek64.so." MSK_MAJORVER "." MSK_MINORVER
    #endif


    libhandle_t __dlopen(const char * filename, const char ** errmsg) {
        libhandle_t h = dlopen(filename,RTLD_NOW);
        if (!h) {
            *errmsg = dlerror();
        }
        return h;
    }
    void __dlclose(libhandle_t h) {
        dlclose(h);
    }

    void * __loadsym(libhandle_t h, const char * symname, const char ** errmsg) {
        void * symaddr = dlsym(h,symname);
        if (!symaddr)
            *errmsg = dlerror();
        return symaddr;
    }
    static const char PATH_SEP = '/';
#endif

static libhandle_t libmosek_handle = NULL;



static MSKrescodee default_ret_0i32() { return MSK_RES_ERR_UNKNOWN; }
static MSKbooleant default_ret_false() { return 0; }
static void* default_ret_ptr() { return NULL; }
static void default_ret_void() { }

typedef MSKrescodee MSKAPI MSK_analyzeproblem_t(MSKtask_t task,MSKstreamtypee whichstream);
static MSK_analyzeproblem_t * MSK_analyzeproblem_ptr = (MSK_analyzeproblem_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_analyzeproblem(MSKtask_t task,MSKstreamtypee whichstream) {
  return (*MSK_analyzeproblem_ptr)(task,whichstream);
} /* MSKanalyzeproblem */
typedef MSKrescodee MSKAPI MSK_analyzenames_t(MSKtask_t task,MSKstreamtypee whichstream,MSKnametypee nametype);
static MSK_analyzenames_t * MSK_analyzenames_ptr = (MSK_analyzenames_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_analyzenames(MSKtask_t task,MSKstreamtypee whichstream,MSKnametypee nametype) {
  return (*MSK_analyzenames_ptr)(task,whichstream,nametype);
} /* MSKanalyzenames */
typedef MSKrescodee MSKAPI MSK_analyzesolution_t(MSKtask_t task,MSKstreamtypee whichstream,MSKsoltypee whichsol);
static MSK_analyzesolution_t * MSK_analyzesolution_ptr = (MSK_analyzesolution_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_analyzesolution(MSKtask_t task,MSKstreamtypee whichstream,MSKsoltypee whichsol) {
  return (*MSK_analyzesolution_ptr)(task,whichstream,whichsol);
} /* MSKanalyzesolution */
typedef MSKrescodee MSKAPI MSK_initbasissolve_t(MSKtask_t task,MSKint32t * basis);
static MSK_initbasissolve_t * MSK_initbasissolve_ptr = (MSK_initbasissolve_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_initbasissolve(MSKtask_t task,MSKint32t * basis) {
  return (*MSK_initbasissolve_ptr)(task,basis);
} /* MSKinitbasissolve */
typedef MSKrescodee MSKAPI MSK_solvewithbasis_t(MSKtask_t task,MSKbooleant transp,MSKint32t numnz,MSKint32t * sub,MSKrealt * val,MSKint32t * numnzout);
static MSK_solvewithbasis_t * MSK_solvewithbasis_ptr = (MSK_solvewithbasis_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_solvewithbasis(MSKtask_t task,MSKbooleant transp,MSKint32t numnz,MSKint32t * sub,MSKrealt * val,MSKint32t * numnzout) {
  return (*MSK_solvewithbasis_ptr)(task,transp,numnz,sub,val,numnzout);
} /* MSKsolvewithbasis */
typedef MSKrescodee MSKAPI MSK_basiscond_t(MSKtask_t task,MSKrealt * nrmbasis,MSKrealt * nrminvbasis);
static MSK_basiscond_t * MSK_basiscond_ptr = (MSK_basiscond_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_basiscond(MSKtask_t task,MSKrealt * nrmbasis,MSKrealt * nrminvbasis) {
  return (*MSK_basiscond_ptr)(task,nrmbasis,nrminvbasis);
} /* MSKbasiscond */
typedef MSKrescodee MSKAPI MSK_appendcons_t(MSKtask_t task,MSKint32t num);
static MSK_appendcons_t * MSK_appendcons_ptr = (MSK_appendcons_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appendcons(MSKtask_t task,MSKint32t num) {
  return (*MSK_appendcons_ptr)(task,num);
} /* MSKappendcons */
typedef MSKrescodee MSKAPI MSK_appendvars_t(MSKtask_t task,MSKint32t num);
static MSK_appendvars_t * MSK_appendvars_ptr = (MSK_appendvars_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appendvars(MSKtask_t task,MSKint32t num) {
  return (*MSK_appendvars_ptr)(task,num);
} /* MSKappendvars */
typedef MSKrescodee MSKAPI MSK_removecons_t(MSKtask_t task,MSKint32t num,const MSKint32t * subset);
static MSK_removecons_t * MSK_removecons_ptr = (MSK_removecons_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_removecons(MSKtask_t task,MSKint32t num,const MSKint32t * subset) {
  return (*MSK_removecons_ptr)(task,num,subset);
} /* MSKremovecons */
typedef MSKrescodee MSKAPI MSK_removevars_t(MSKtask_t task,MSKint32t num,const MSKint32t * subset);
static MSK_removevars_t * MSK_removevars_ptr = (MSK_removevars_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_removevars(MSKtask_t task,MSKint32t num,const MSKint32t * subset) {
  return (*MSK_removevars_ptr)(task,num,subset);
} /* MSKremovevars */
typedef MSKrescodee MSKAPI MSK_removebarvars_t(MSKtask_t task,MSKint32t num,const MSKint32t * subset);
static MSK_removebarvars_t * MSK_removebarvars_ptr = (MSK_removebarvars_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_removebarvars(MSKtask_t task,MSKint32t num,const MSKint32t * subset) {
  return (*MSK_removebarvars_ptr)(task,num,subset);
} /* MSKremovebarvars */
typedef MSKrescodee MSKAPI MSK_removecones_t(MSKtask_t task,MSKint32t num,const MSKint32t * subset);
static MSK_removecones_t * MSK_removecones_ptr = (MSK_removecones_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_removecones(MSKtask_t task,MSKint32t num,const MSKint32t * subset) {
  return (*MSK_removecones_ptr)(task,num,subset);
} /* MSKremovecones */
typedef MSKrescodee MSKAPI MSK_appendbarvars_t(MSKtask_t task,MSKint32t num,const MSKint32t * dim);
static MSK_appendbarvars_t * MSK_appendbarvars_ptr = (MSK_appendbarvars_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appendbarvars(MSKtask_t task,MSKint32t num,const MSKint32t * dim) {
  return (*MSK_appendbarvars_ptr)(task,num,dim);
} /* MSKappendbarvars */
typedef MSKrescodee MSKAPI MSK_appendcone_t(MSKtask_t task,MSKconetypee ct,MSKrealt conepar,MSKint32t nummem,const MSKint32t * submem);
static MSK_appendcone_t * MSK_appendcone_ptr = (MSK_appendcone_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appendcone(MSKtask_t task,MSKconetypee ct,MSKrealt conepar,MSKint32t nummem,const MSKint32t * submem) {
  return (*MSK_appendcone_ptr)(task,ct,conepar,nummem,submem);
} /* MSKappendcone */
typedef MSKrescodee MSKAPI MSK_appendconeseq_t(MSKtask_t task,MSKconetypee ct,MSKrealt conepar,MSKint32t nummem,MSKint32t j);
static MSK_appendconeseq_t * MSK_appendconeseq_ptr = (MSK_appendconeseq_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appendconeseq(MSKtask_t task,MSKconetypee ct,MSKrealt conepar,MSKint32t nummem,MSKint32t j) {
  return (*MSK_appendconeseq_ptr)(task,ct,conepar,nummem,j);
} /* MSKappendconeseq */
typedef MSKrescodee MSKAPI MSK_appendconesseq_t(MSKtask_t task,MSKint32t num,const MSKconetypee * ct,const MSKrealt * conepar,const MSKint32t * nummem,MSKint32t j);
static MSK_appendconesseq_t * MSK_appendconesseq_ptr = (MSK_appendconesseq_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appendconesseq(MSKtask_t task,MSKint32t num,const MSKconetypee * ct,const MSKrealt * conepar,const MSKint32t * nummem,MSKint32t j) {
  return (*MSK_appendconesseq_ptr)(task,num,ct,conepar,nummem,j);
} /* MSKappendconesseq */
typedef MSKrescodee MSKAPI MSK_bktostr_t(MSKtask_t task,MSKboundkeye bk,char * str);
static MSK_bktostr_t * MSK_bktostr_ptr = (MSK_bktostr_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_bktostr(MSKtask_t task,MSKboundkeye bk,char * str) {
  return (*MSK_bktostr_ptr)(task,bk,str);
} /* MSKbktostr */
typedef void * MSKAPI MSK_calloctask_t(MSKtask_t task,size_t number,size_t size);
static MSK_calloctask_t * MSK_calloctask_ptr = (MSK_calloctask_t*) default_ret_ptr;
void * MSKAPI MSK_calloctask(MSKtask_t task,size_t number,size_t size) {
  return (*MSK_calloctask_ptr)(task,number,size);
} /* MSKcalloctask */
typedef void * MSKAPI MSK_callocdbgtask_t(MSKtask_t task,size_t number,size_t size,const char * file,unsigned line);
static MSK_callocdbgtask_t * MSK_callocdbgtask_ptr = (MSK_callocdbgtask_t*) default_ret_ptr;
void * MSKAPI MSK_callocdbgtask(MSKtask_t task,size_t number,size_t size,const char * file,unsigned line) {
  return (*MSK_callocdbgtask_ptr)(task,number,size,file,line);
} /* MSKcallocdbgtask */
typedef MSKrescodee MSKAPI MSK_chgconbound_t(MSKtask_t task,MSKint32t i,MSKint32t lower,MSKint32t finite,MSKrealt value);
static MSK_chgconbound_t * MSK_chgconbound_ptr = (MSK_chgconbound_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_chgconbound(MSKtask_t task,MSKint32t i,MSKint32t lower,MSKint32t finite,MSKrealt value) {
  return (*MSK_chgconbound_ptr)(task,i,lower,finite,value);
} /* MSKchgconbound */
typedef MSKrescodee MSKAPI MSK_chgvarbound_t(MSKtask_t task,MSKint32t j,MSKint32t lower,MSKint32t finite,MSKrealt value);
static MSK_chgvarbound_t * MSK_chgvarbound_ptr = (MSK_chgvarbound_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_chgvarbound(MSKtask_t task,MSKint32t j,MSKint32t lower,MSKint32t finite,MSKrealt value) {
  return (*MSK_chgvarbound_ptr)(task,j,lower,finite,value);
} /* MSKchgvarbound */
typedef MSKrescodee MSKAPI MSK_conetypetostr_t(MSKtask_t task,MSKconetypee ct,char * str);
static MSK_conetypetostr_t * MSK_conetypetostr_ptr = (MSK_conetypetostr_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_conetypetostr(MSKtask_t task,MSKconetypee ct,char * str) {
  return (*MSK_conetypetostr_ptr)(task,ct,str);
} /* MSKconetypetostr */
typedef MSKrescodee MSKAPI MSK_deletetask_t(MSKtask_t * task);
static MSK_deletetask_t * MSK_deletetask_ptr = (MSK_deletetask_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_deletetask(MSKtask_t * task) {
  return (*MSK_deletetask_ptr)(task);
} /* MSKdeletetask */
typedef void MSKAPI MSK_freetask_t(MSKtask_t task,void * buffer);
static MSK_freetask_t * MSK_freetask_ptr = (MSK_freetask_t*) default_ret_void;
void MSKAPI MSK_freetask(MSKtask_t task,void * buffer) {
  (*MSK_freetask_ptr)(task,buffer);
} /* MSKfreetask */
typedef void MSKAPI MSK_freedbgtask_t(MSKtask_t task,void * buffer,const char * file,unsigned line);
static MSK_freedbgtask_t * MSK_freedbgtask_ptr = (MSK_freedbgtask_t*) default_ret_void;
void MSKAPI MSK_freedbgtask(MSKtask_t task,void * buffer,const char * file,unsigned line) {
  (*MSK_freedbgtask_ptr)(task,buffer,file,line);
} /* MSKfreedbgtask */
typedef MSKrescodee MSKAPI MSK_getaij_t(MSKtask_t task,MSKint32t i,MSKint32t j,MSKrealt * aij);
static MSK_getaij_t * MSK_getaij_ptr = (MSK_getaij_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getaij(MSKtask_t task,MSKint32t i,MSKint32t j,MSKrealt * aij) {
  return (*MSK_getaij_ptr)(task,i,j,aij);
} /* MSKgetaij */
typedef MSKrescodee MSKAPI MSK_getapiecenumnz_t(MSKtask_t task,MSKint32t firsti,MSKint32t lasti,MSKint32t firstj,MSKint32t lastj,MSKint32t * numnz);
static MSK_getapiecenumnz_t * MSK_getapiecenumnz_ptr = (MSK_getapiecenumnz_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getapiecenumnz(MSKtask_t task,MSKint32t firsti,MSKint32t lasti,MSKint32t firstj,MSKint32t lastj,MSKint32t * numnz) {
  return (*MSK_getapiecenumnz_ptr)(task,firsti,lasti,firstj,lastj,numnz);
} /* MSKgetapiecenumnz */
typedef MSKrescodee MSKAPI MSK_getacolnumnz_t(MSKtask_t task,MSKint32t i,MSKint32t * nzj);
static MSK_getacolnumnz_t * MSK_getacolnumnz_ptr = (MSK_getacolnumnz_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getacolnumnz(MSKtask_t task,MSKint32t i,MSKint32t * nzj) {
  return (*MSK_getacolnumnz_ptr)(task,i,nzj);
} /* MSKgetacolnumnz */
typedef MSKrescodee MSKAPI MSK_getacol_t(MSKtask_t task,MSKint32t j,MSKint32t * nzj,MSKint32t * subj,MSKrealt * valj);
static MSK_getacol_t * MSK_getacol_ptr = (MSK_getacol_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getacol(MSKtask_t task,MSKint32t j,MSKint32t * nzj,MSKint32t * subj,MSKrealt * valj) {
  return (*MSK_getacol_ptr)(task,j,nzj,subj,valj);
} /* MSKgetacol */
typedef MSKrescodee MSKAPI MSK_getacolslice_t(MSKtask_t task,MSKint32t first,MSKint32t last,MSKint32t maxnumnz,MSKint32t * ptrb,MSKint32t * ptre,MSKint32t * sub,MSKrealt * val);
static MSK_getacolslice_t * MSK_getacolslice_ptr = (MSK_getacolslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getacolslice(MSKtask_t task,MSKint32t first,MSKint32t last,MSKint32t maxnumnz,MSKint32t * ptrb,MSKint32t * ptre,MSKint32t * sub,MSKrealt * val) {
  return (*MSK_getacolslice_ptr)(task,first,last,maxnumnz,ptrb,ptre,sub,val);
} /* MSKgetacolslice */
typedef MSKrescodee MSKAPI MSK_getacolslice64_t(MSKtask_t task,MSKint32t first,MSKint32t last,MSKint64t maxnumnz,MSKint64t * ptrb,MSKint64t * ptre,MSKint32t * sub,MSKrealt * val);
static MSK_getacolslice64_t * MSK_getacolslice64_ptr = (MSK_getacolslice64_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getacolslice64(MSKtask_t task,MSKint32t first,MSKint32t last,MSKint64t maxnumnz,MSKint64t * ptrb,MSKint64t * ptre,MSKint32t * sub,MSKrealt * val) {
  return (*MSK_getacolslice64_ptr)(task,first,last,maxnumnz,ptrb,ptre,sub,val);
} /* MSKgetacolslice64 */
typedef MSKrescodee MSKAPI MSK_getarownumnz_t(MSKtask_t task,MSKint32t i,MSKint32t * nzi);
static MSK_getarownumnz_t * MSK_getarownumnz_ptr = (MSK_getarownumnz_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getarownumnz(MSKtask_t task,MSKint32t i,MSKint32t * nzi) {
  return (*MSK_getarownumnz_ptr)(task,i,nzi);
} /* MSKgetarownumnz */
typedef MSKrescodee MSKAPI MSK_getarow_t(MSKtask_t task,MSKint32t i,MSKint32t * nzi,MSKint32t * subi,MSKrealt * vali);
static MSK_getarow_t * MSK_getarow_ptr = (MSK_getarow_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getarow(MSKtask_t task,MSKint32t i,MSKint32t * nzi,MSKint32t * subi,MSKrealt * vali) {
  return (*MSK_getarow_ptr)(task,i,nzi,subi,vali);
} /* MSKgetarow */
typedef MSKrescodee MSKAPI MSK_getacolslicenumnz_t(MSKtask_t task,MSKint32t first,MSKint32t last,MSKint32t * numnz);
static MSK_getacolslicenumnz_t * MSK_getacolslicenumnz_ptr = (MSK_getacolslicenumnz_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getacolslicenumnz(MSKtask_t task,MSKint32t first,MSKint32t last,MSKint32t * numnz) {
  return (*MSK_getacolslicenumnz_ptr)(task,first,last,numnz);
} /* MSKgetacolslicenumnz */
typedef MSKrescodee MSKAPI MSK_getacolslicenumnz64_t(MSKtask_t task,MSKint32t first,MSKint32t last,MSKint64t * numnz);
static MSK_getacolslicenumnz64_t * MSK_getacolslicenumnz64_ptr = (MSK_getacolslicenumnz64_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getacolslicenumnz64(MSKtask_t task,MSKint32t first,MSKint32t last,MSKint64t * numnz) {
  return (*MSK_getacolslicenumnz64_ptr)(task,first,last,numnz);
} /* MSKgetacolslicenumnz64 */
typedef MSKrescodee MSKAPI MSK_getarowslicenumnz_t(MSKtask_t task,MSKint32t first,MSKint32t last,MSKint32t * numnz);
static MSK_getarowslicenumnz_t * MSK_getarowslicenumnz_ptr = (MSK_getarowslicenumnz_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getarowslicenumnz(MSKtask_t task,MSKint32t first,MSKint32t last,MSKint32t * numnz) {
  return (*MSK_getarowslicenumnz_ptr)(task,first,last,numnz);
} /* MSKgetarowslicenumnz */
typedef MSKrescodee MSKAPI MSK_getarowslicenumnz64_t(MSKtask_t task,MSKint32t first,MSKint32t last,MSKint64t * numnz);
static MSK_getarowslicenumnz64_t * MSK_getarowslicenumnz64_ptr = (MSK_getarowslicenumnz64_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getarowslicenumnz64(MSKtask_t task,MSKint32t first,MSKint32t last,MSKint64t * numnz) {
  return (*MSK_getarowslicenumnz64_ptr)(task,first,last,numnz);
} /* MSKgetarowslicenumnz64 */
typedef MSKrescodee MSKAPI MSK_getarowslice_t(MSKtask_t task,MSKint32t first,MSKint32t last,MSKint32t maxnumnz,MSKint32t * ptrb,MSKint32t * ptre,MSKint32t * sub,MSKrealt * val);
static MSK_getarowslice_t * MSK_getarowslice_ptr = (MSK_getarowslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getarowslice(MSKtask_t task,MSKint32t first,MSKint32t last,MSKint32t maxnumnz,MSKint32t * ptrb,MSKint32t * ptre,MSKint32t * sub,MSKrealt * val) {
  return (*MSK_getarowslice_ptr)(task,first,last,maxnumnz,ptrb,ptre,sub,val);
} /* MSKgetarowslice */
typedef MSKrescodee MSKAPI MSK_getarowslice64_t(MSKtask_t task,MSKint32t first,MSKint32t last,MSKint64t maxnumnz,MSKint64t * ptrb,MSKint64t * ptre,MSKint32t * sub,MSKrealt * val);
static MSK_getarowslice64_t * MSK_getarowslice64_ptr = (MSK_getarowslice64_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getarowslice64(MSKtask_t task,MSKint32t first,MSKint32t last,MSKint64t maxnumnz,MSKint64t * ptrb,MSKint64t * ptre,MSKint32t * sub,MSKrealt * val) {
  return (*MSK_getarowslice64_ptr)(task,first,last,maxnumnz,ptrb,ptre,sub,val);
} /* MSKgetarowslice64 */
typedef MSKrescodee MSKAPI MSK_getatrip_t(MSKtask_t task,MSKint64t maxnumnz,MSKint32t * subi,MSKint32t * subj,MSKrealt * val);
static MSK_getatrip_t * MSK_getatrip_ptr = (MSK_getatrip_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getatrip(MSKtask_t task,MSKint64t maxnumnz,MSKint32t * subi,MSKint32t * subj,MSKrealt * val) {
  return (*MSK_getatrip_ptr)(task,maxnumnz,subi,subj,val);
} /* MSKgetatrip */
typedef MSKrescodee MSKAPI MSK_getarowslicetrip_t(MSKtask_t task,MSKint32t first,MSKint32t last,MSKint64t maxnumnz,MSKint32t * subi,MSKint32t * subj,MSKrealt * val);
static MSK_getarowslicetrip_t * MSK_getarowslicetrip_ptr = (MSK_getarowslicetrip_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getarowslicetrip(MSKtask_t task,MSKint32t first,MSKint32t last,MSKint64t maxnumnz,MSKint32t * subi,MSKint32t * subj,MSKrealt * val) {
  return (*MSK_getarowslicetrip_ptr)(task,first,last,maxnumnz,subi,subj,val);
} /* MSKgetarowslicetrip */
typedef MSKrescodee MSKAPI MSK_getacolslicetrip_t(MSKtask_t task,MSKint32t first,MSKint32t last,MSKint64t maxnumnz,MSKint32t * subi,MSKint32t * subj,MSKrealt * val);
static MSK_getacolslicetrip_t * MSK_getacolslicetrip_ptr = (MSK_getacolslicetrip_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getacolslicetrip(MSKtask_t task,MSKint32t first,MSKint32t last,MSKint64t maxnumnz,MSKint32t * subi,MSKint32t * subj,MSKrealt * val) {
  return (*MSK_getacolslicetrip_ptr)(task,first,last,maxnumnz,subi,subj,val);
} /* MSKgetacolslicetrip */
typedef MSKrescodee MSKAPI MSK_getconbound_t(MSKtask_t task,MSKint32t i,MSKboundkeye * bk,MSKrealt * bl,MSKrealt * bu);
static MSK_getconbound_t * MSK_getconbound_ptr = (MSK_getconbound_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getconbound(MSKtask_t task,MSKint32t i,MSKboundkeye * bk,MSKrealt * bl,MSKrealt * bu) {
  return (*MSK_getconbound_ptr)(task,i,bk,bl,bu);
} /* MSKgetconbound */
typedef MSKrescodee MSKAPI MSK_getvarbound_t(MSKtask_t task,MSKint32t i,MSKboundkeye * bk,MSKrealt * bl,MSKrealt * bu);
static MSK_getvarbound_t * MSK_getvarbound_ptr = (MSK_getvarbound_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getvarbound(MSKtask_t task,MSKint32t i,MSKboundkeye * bk,MSKrealt * bl,MSKrealt * bu) {
  return (*MSK_getvarbound_ptr)(task,i,bk,bl,bu);
} /* MSKgetvarbound */
typedef MSKrescodee MSKAPI MSK_getconboundslice_t(MSKtask_t task,MSKint32t first,MSKint32t last,MSKboundkeye * bk,MSKrealt * bl,MSKrealt * bu);
static MSK_getconboundslice_t * MSK_getconboundslice_ptr = (MSK_getconboundslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getconboundslice(MSKtask_t task,MSKint32t first,MSKint32t last,MSKboundkeye * bk,MSKrealt * bl,MSKrealt * bu) {
  return (*MSK_getconboundslice_ptr)(task,first,last,bk,bl,bu);
} /* MSKgetconboundslice */
typedef MSKrescodee MSKAPI MSK_getvarboundslice_t(MSKtask_t task,MSKint32t first,MSKint32t last,MSKboundkeye * bk,MSKrealt * bl,MSKrealt * bu);
static MSK_getvarboundslice_t * MSK_getvarboundslice_ptr = (MSK_getvarboundslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getvarboundslice(MSKtask_t task,MSKint32t first,MSKint32t last,MSKboundkeye * bk,MSKrealt * bl,MSKrealt * bu) {
  return (*MSK_getvarboundslice_ptr)(task,first,last,bk,bl,bu);
} /* MSKgetvarboundslice */
typedef MSKrescodee MSKAPI MSK_getcj_t(MSKtask_t task,MSKint32t j,MSKrealt * cj);
static MSK_getcj_t * MSK_getcj_ptr = (MSK_getcj_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getcj(MSKtask_t task,MSKint32t j,MSKrealt * cj) {
  return (*MSK_getcj_ptr)(task,j,cj);
} /* MSKgetcj */
typedef MSKrescodee MSKAPI MSK_getc_t(MSKtask_t task,MSKrealt * c);
static MSK_getc_t * MSK_getc_ptr = (MSK_getc_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getc(MSKtask_t task,MSKrealt * c) {
  return (*MSK_getc_ptr)(task,c);
} /* MSKgetc */
typedef MSKrescodee MSKAPI MSK_getcallbackfunc_t(MSKtask_t task,MSKcallbackfunc * func,MSKuserhandle_t * handle);
static MSK_getcallbackfunc_t * MSK_getcallbackfunc_ptr = (MSK_getcallbackfunc_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getcallbackfunc(MSKtask_t task,MSKcallbackfunc * func,MSKuserhandle_t * handle) {
  return (*MSK_getcallbackfunc_ptr)(task,func,handle);
} /* MSKgetcallbackfunc */
typedef MSKrescodee MSKAPI MSK_getcfix_t(MSKtask_t task,MSKrealt * cfix);
static MSK_getcfix_t * MSK_getcfix_ptr = (MSK_getcfix_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getcfix(MSKtask_t task,MSKrealt * cfix) {
  return (*MSK_getcfix_ptr)(task,cfix);
} /* MSKgetcfix */
typedef MSKrescodee MSKAPI MSK_getcone_t(MSKtask_t task,MSKint32t k,MSKconetypee * ct,MSKrealt * conepar,MSKint32t * nummem,MSKint32t * submem);
static MSK_getcone_t * MSK_getcone_ptr = (MSK_getcone_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getcone(MSKtask_t task,MSKint32t k,MSKconetypee * ct,MSKrealt * conepar,MSKint32t * nummem,MSKint32t * submem) {
  return (*MSK_getcone_ptr)(task,k,ct,conepar,nummem,submem);
} /* MSKgetcone */
typedef MSKrescodee MSKAPI MSK_getconeinfo_t(MSKtask_t task,MSKint32t k,MSKconetypee * ct,MSKrealt * conepar,MSKint32t * nummem);
static MSK_getconeinfo_t * MSK_getconeinfo_ptr = (MSK_getconeinfo_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getconeinfo(MSKtask_t task,MSKint32t k,MSKconetypee * ct,MSKrealt * conepar,MSKint32t * nummem) {
  return (*MSK_getconeinfo_ptr)(task,k,ct,conepar,nummem);
} /* MSKgetconeinfo */
typedef MSKrescodee MSKAPI MSK_getclist_t(MSKtask_t task,MSKint32t num,const MSKint32t * subj,MSKrealt * c);
static MSK_getclist_t * MSK_getclist_ptr = (MSK_getclist_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getclist(MSKtask_t task,MSKint32t num,const MSKint32t * subj,MSKrealt * c) {
  return (*MSK_getclist_ptr)(task,num,subj,c);
} /* MSKgetclist */
typedef MSKrescodee MSKAPI MSK_getcslice_t(MSKtask_t task,MSKint32t first,MSKint32t last,MSKrealt * c);
static MSK_getcslice_t * MSK_getcslice_ptr = (MSK_getcslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getcslice(MSKtask_t task,MSKint32t first,MSKint32t last,MSKrealt * c) {
  return (*MSK_getcslice_ptr)(task,first,last,c);
} /* MSKgetcslice */
typedef MSKrescodee MSKAPI MSK_getdouinf_t(MSKtask_t task,MSKdinfiteme whichdinf,MSKrealt * dvalue);
static MSK_getdouinf_t * MSK_getdouinf_ptr = (MSK_getdouinf_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdouinf(MSKtask_t task,MSKdinfiteme whichdinf,MSKrealt * dvalue) {
  return (*MSK_getdouinf_ptr)(task,whichdinf,dvalue);
} /* MSKgetdouinf */
typedef MSKrescodee MSKAPI MSK_getdouparam_t(MSKtask_t task,MSKdparame param,MSKrealt * parvalue);
static MSK_getdouparam_t * MSK_getdouparam_ptr = (MSK_getdouparam_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdouparam(MSKtask_t task,MSKdparame param,MSKrealt * parvalue) {
  return (*MSK_getdouparam_ptr)(task,param,parvalue);
} /* MSKgetdouparam */
typedef MSKrescodee MSKAPI MSK_getdualobj_t(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * dualobj);
static MSK_getdualobj_t * MSK_getdualobj_ptr = (MSK_getdualobj_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdualobj(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * dualobj) {
  return (*MSK_getdualobj_ptr)(task,whichsol,dualobj);
} /* MSKgetdualobj */
typedef MSKrescodee MSKAPI MSK_getenv_t(MSKtask_t task,MSKenv_t * env);
static MSK_getenv_t * MSK_getenv_ptr = (MSK_getenv_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getenv(MSKtask_t task,MSKenv_t * env) {
  return (*MSK_getenv_ptr)(task,env);
} /* MSKgetenv */
typedef MSKrescodee MSKAPI MSK_getinfindex_t(MSKtask_t task,MSKinftypee inftype,const char * infname,MSKint32t * infindex);
static MSK_getinfindex_t * MSK_getinfindex_ptr = (MSK_getinfindex_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getinfindex(MSKtask_t task,MSKinftypee inftype,const char * infname,MSKint32t * infindex) {
  return (*MSK_getinfindex_ptr)(task,inftype,infname,infindex);
} /* MSKgetinfindex */
typedef MSKrescodee MSKAPI MSK_getinfmax_t(MSKtask_t task,MSKinftypee inftype,MSKint32t * infmax);
static MSK_getinfmax_t * MSK_getinfmax_ptr = (MSK_getinfmax_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getinfmax(MSKtask_t task,MSKinftypee inftype,MSKint32t * infmax) {
  return (*MSK_getinfmax_ptr)(task,inftype,infmax);
} /* MSKgetinfmax */
typedef MSKrescodee MSKAPI MSK_getinfname_t(MSKtask_t task,MSKinftypee inftype,MSKint32t whichinf,char * infname);
static MSK_getinfname_t * MSK_getinfname_ptr = (MSK_getinfname_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getinfname(MSKtask_t task,MSKinftypee inftype,MSKint32t whichinf,char * infname) {
  return (*MSK_getinfname_ptr)(task,inftype,whichinf,infname);
} /* MSKgetinfname */
typedef MSKrescodee MSKAPI MSK_getintinf_t(MSKtask_t task,MSKiinfiteme whichiinf,MSKint32t * ivalue);
static MSK_getintinf_t * MSK_getintinf_ptr = (MSK_getintinf_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getintinf(MSKtask_t task,MSKiinfiteme whichiinf,MSKint32t * ivalue) {
  return (*MSK_getintinf_ptr)(task,whichiinf,ivalue);
} /* MSKgetintinf */
typedef MSKrescodee MSKAPI MSK_getlintinf_t(MSKtask_t task,MSKliinfiteme whichliinf,MSKint64t * ivalue);
static MSK_getlintinf_t * MSK_getlintinf_ptr = (MSK_getlintinf_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getlintinf(MSKtask_t task,MSKliinfiteme whichliinf,MSKint64t * ivalue) {
  return (*MSK_getlintinf_ptr)(task,whichliinf,ivalue);
} /* MSKgetlintinf */
typedef MSKrescodee MSKAPI MSK_getintparam_t(MSKtask_t task,MSKiparame param,MSKint32t * parvalue);
static MSK_getintparam_t * MSK_getintparam_ptr = (MSK_getintparam_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getintparam(MSKtask_t task,MSKiparame param,MSKint32t * parvalue) {
  return (*MSK_getintparam_ptr)(task,param,parvalue);
} /* MSKgetintparam */
typedef MSKrescodee MSKAPI MSK_getlintparam_t(MSKtask_t task,MSKiparame param,MSKint64t * parvalue);
static MSK_getlintparam_t * MSK_getlintparam_ptr = (MSK_getlintparam_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getlintparam(MSKtask_t task,MSKiparame param,MSKint64t * parvalue) {
  return (*MSK_getlintparam_ptr)(task,param,parvalue);
} /* MSKgetlintparam */
typedef MSKrescodee MSKAPI MSK_getmaxnamelen_t(MSKtask_t task,MSKint32t * maxlen);
static MSK_getmaxnamelen_t * MSK_getmaxnamelen_ptr = (MSK_getmaxnamelen_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getmaxnamelen(MSKtask_t task,MSKint32t * maxlen) {
  return (*MSK_getmaxnamelen_ptr)(task,maxlen);
} /* MSKgetmaxnamelen */
typedef MSKrescodee MSKAPI MSK_getmaxnumanz_t(MSKtask_t task,MSKint32t * maxnumanz);
static MSK_getmaxnumanz_t * MSK_getmaxnumanz_ptr = (MSK_getmaxnumanz_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getmaxnumanz(MSKtask_t task,MSKint32t * maxnumanz) {
  return (*MSK_getmaxnumanz_ptr)(task,maxnumanz);
} /* MSKgetmaxnumanz */
typedef MSKrescodee MSKAPI MSK_getmaxnumanz64_t(MSKtask_t task,MSKint64t * maxnumanz);
static MSK_getmaxnumanz64_t * MSK_getmaxnumanz64_ptr = (MSK_getmaxnumanz64_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getmaxnumanz64(MSKtask_t task,MSKint64t * maxnumanz) {
  return (*MSK_getmaxnumanz64_ptr)(task,maxnumanz);
} /* MSKgetmaxnumanz64 */
typedef MSKrescodee MSKAPI MSK_getmaxnumcon_t(MSKtask_t task,MSKint32t * maxnumcon);
static MSK_getmaxnumcon_t * MSK_getmaxnumcon_ptr = (MSK_getmaxnumcon_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getmaxnumcon(MSKtask_t task,MSKint32t * maxnumcon) {
  return (*MSK_getmaxnumcon_ptr)(task,maxnumcon);
} /* MSKgetmaxnumcon */
typedef MSKrescodee MSKAPI MSK_getmaxnumvar_t(MSKtask_t task,MSKint32t * maxnumvar);
static MSK_getmaxnumvar_t * MSK_getmaxnumvar_ptr = (MSK_getmaxnumvar_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getmaxnumvar(MSKtask_t task,MSKint32t * maxnumvar) {
  return (*MSK_getmaxnumvar_ptr)(task,maxnumvar);
} /* MSKgetmaxnumvar */
typedef MSKrescodee MSKAPI MSK_getnadouinf_t(MSKtask_t task,const char * infitemname,MSKrealt * dvalue);
static MSK_getnadouinf_t * MSK_getnadouinf_ptr = (MSK_getnadouinf_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnadouinf(MSKtask_t task,const char * infitemname,MSKrealt * dvalue) {
  return (*MSK_getnadouinf_ptr)(task,infitemname,dvalue);
} /* MSKgetnadouinf */
typedef MSKrescodee MSKAPI MSK_getnadouparam_t(MSKtask_t task,const char * paramname,MSKrealt * parvalue);
static MSK_getnadouparam_t * MSK_getnadouparam_ptr = (MSK_getnadouparam_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnadouparam(MSKtask_t task,const char * paramname,MSKrealt * parvalue) {
  return (*MSK_getnadouparam_ptr)(task,paramname,parvalue);
} /* MSKgetnadouparam */
typedef MSKrescodee MSKAPI MSK_getnaintinf_t(MSKtask_t task,const char * infitemname,MSKint32t * ivalue);
static MSK_getnaintinf_t * MSK_getnaintinf_ptr = (MSK_getnaintinf_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnaintinf(MSKtask_t task,const char * infitemname,MSKint32t * ivalue) {
  return (*MSK_getnaintinf_ptr)(task,infitemname,ivalue);
} /* MSKgetnaintinf */
typedef MSKrescodee MSKAPI MSK_getnaintparam_t(MSKtask_t task,const char * paramname,MSKint32t * parvalue);
static MSK_getnaintparam_t * MSK_getnaintparam_ptr = (MSK_getnaintparam_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnaintparam(MSKtask_t task,const char * paramname,MSKint32t * parvalue) {
  return (*MSK_getnaintparam_ptr)(task,paramname,parvalue);
} /* MSKgetnaintparam */
typedef MSKrescodee MSKAPI MSK_getbarvarnamelen_t(MSKtask_t task,MSKint32t i,MSKint32t * len);
static MSK_getbarvarnamelen_t * MSK_getbarvarnamelen_ptr = (MSK_getbarvarnamelen_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getbarvarnamelen(MSKtask_t task,MSKint32t i,MSKint32t * len) {
  return (*MSK_getbarvarnamelen_ptr)(task,i,len);
} /* MSKgetbarvarnamelen */
typedef MSKrescodee MSKAPI MSK_getbarvarname_t(MSKtask_t task,MSKint32t i,MSKint32t sizename,char * name);
static MSK_getbarvarname_t * MSK_getbarvarname_ptr = (MSK_getbarvarname_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getbarvarname(MSKtask_t task,MSKint32t i,MSKint32t sizename,char * name) {
  return (*MSK_getbarvarname_ptr)(task,i,sizename,name);
} /* MSKgetbarvarname */
typedef MSKrescodee MSKAPI MSK_getbarvarnameindex_t(MSKtask_t task,const char * somename,MSKint32t * asgn,MSKint32t * index);
static MSK_getbarvarnameindex_t * MSK_getbarvarnameindex_ptr = (MSK_getbarvarnameindex_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getbarvarnameindex(MSKtask_t task,const char * somename,MSKint32t * asgn,MSKint32t * index) {
  return (*MSK_getbarvarnameindex_ptr)(task,somename,asgn,index);
} /* MSKgetbarvarnameindex */
typedef MSKrescodee MSKAPI MSK_generatebarvarnames_t(MSKtask_t task,MSKint32t num,const MSKint32t * subj,const char * fmt,MSKint32t ndims,const MSKint32t * dims,const MSKint64t * sp,MSKint32t numnamedaxis,const MSKint32t * namedaxisidxs,MSKint64t numnames,const char *names[]);
static MSK_generatebarvarnames_t * MSK_generatebarvarnames_ptr = (MSK_generatebarvarnames_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_generatebarvarnames(MSKtask_t task,MSKint32t num,const MSKint32t * subj,const char * fmt,MSKint32t ndims,const MSKint32t * dims,const MSKint64t * sp,MSKint32t numnamedaxis,const MSKint32t * namedaxisidxs,MSKint64t numnames,const char *names[]) {
  return (*MSK_generatebarvarnames_ptr)(task,num,subj,fmt,ndims,dims,sp,numnamedaxis,namedaxisidxs,numnames,names);
} /* MSKgeneratebarvarnames */
typedef MSKrescodee MSKAPI MSK_generatevarnames_t(MSKtask_t task,MSKint32t num,const MSKint32t * subj,const char * fmt,MSKint32t ndims,const MSKint32t * dims,const MSKint64t * sp,MSKint32t numnamedaxis,const MSKint32t * namedaxisidxs,MSKint64t numnames,const char *names[]);
static MSK_generatevarnames_t * MSK_generatevarnames_ptr = (MSK_generatevarnames_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_generatevarnames(MSKtask_t task,MSKint32t num,const MSKint32t * subj,const char * fmt,MSKint32t ndims,const MSKint32t * dims,const MSKint64t * sp,MSKint32t numnamedaxis,const MSKint32t * namedaxisidxs,MSKint64t numnames,const char *names[]) {
  return (*MSK_generatevarnames_ptr)(task,num,subj,fmt,ndims,dims,sp,numnamedaxis,namedaxisidxs,numnames,names);
} /* MSKgeneratevarnames */
typedef MSKrescodee MSKAPI MSK_generateconnames_t(MSKtask_t task,MSKint32t num,const MSKint32t * subi,const char * fmt,MSKint32t ndims,const MSKint32t * dims,const MSKint64t * sp,MSKint32t numnamedaxis,const MSKint32t * namedaxisidxs,MSKint64t numnames,const char *names[]);
static MSK_generateconnames_t * MSK_generateconnames_ptr = (MSK_generateconnames_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_generateconnames(MSKtask_t task,MSKint32t num,const MSKint32t * subi,const char * fmt,MSKint32t ndims,const MSKint32t * dims,const MSKint64t * sp,MSKint32t numnamedaxis,const MSKint32t * namedaxisidxs,MSKint64t numnames,const char *names[]) {
  return (*MSK_generateconnames_ptr)(task,num,subi,fmt,ndims,dims,sp,numnamedaxis,namedaxisidxs,numnames,names);
} /* MSKgenerateconnames */
typedef MSKrescodee MSKAPI MSK_generateconenames_t(MSKtask_t task,MSKint32t num,const MSKint32t * subk,const char * fmt,MSKint32t ndims,const MSKint32t * dims,const MSKint64t * sp,MSKint32t numnamedaxis,const MSKint32t * namedaxisidxs,MSKint64t numnames,const char *names[]);
static MSK_generateconenames_t * MSK_generateconenames_ptr = (MSK_generateconenames_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_generateconenames(MSKtask_t task,MSKint32t num,const MSKint32t * subk,const char * fmt,MSKint32t ndims,const MSKint32t * dims,const MSKint64t * sp,MSKint32t numnamedaxis,const MSKint32t * namedaxisidxs,MSKint64t numnames,const char *names[]) {
  return (*MSK_generateconenames_ptr)(task,num,subk,fmt,ndims,dims,sp,numnamedaxis,namedaxisidxs,numnames,names);
} /* MSKgenerateconenames */
typedef MSKrescodee MSKAPI MSK_generateaccnames_t(MSKtask_t task,MSKint64t num,const MSKint64t * sub,const char * fmt,MSKint32t ndims,const MSKint32t * dims,const MSKint64t * sp,MSKint32t numnamedaxis,const MSKint32t * namedaxisidxs,MSKint64t numnames,const char *names[]);
static MSK_generateaccnames_t * MSK_generateaccnames_ptr = (MSK_generateaccnames_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_generateaccnames(MSKtask_t task,MSKint64t num,const MSKint64t * sub,const char * fmt,MSKint32t ndims,const MSKint32t * dims,const MSKint64t * sp,MSKint32t numnamedaxis,const MSKint32t * namedaxisidxs,MSKint64t numnames,const char *names[]) {
  return (*MSK_generateaccnames_ptr)(task,num,sub,fmt,ndims,dims,sp,numnamedaxis,namedaxisidxs,numnames,names);
} /* MSKgenerateaccnames */
typedef MSKrescodee MSKAPI MSK_generatedjcnames_t(MSKtask_t task,MSKint64t num,const MSKint64t * sub,const char * fmt,MSKint32t ndims,const MSKint32t * dims,const MSKint64t * sp,MSKint32t numnamedaxis,const MSKint32t * namedaxisidxs,MSKint64t numnames,const char *names[]);
static MSK_generatedjcnames_t * MSK_generatedjcnames_ptr = (MSK_generatedjcnames_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_generatedjcnames(MSKtask_t task,MSKint64t num,const MSKint64t * sub,const char * fmt,MSKint32t ndims,const MSKint32t * dims,const MSKint64t * sp,MSKint32t numnamedaxis,const MSKint32t * namedaxisidxs,MSKint64t numnames,const char *names[]) {
  return (*MSK_generatedjcnames_ptr)(task,num,sub,fmt,ndims,dims,sp,numnamedaxis,namedaxisidxs,numnames,names);
} /* MSKgeneratedjcnames */
typedef MSKrescodee MSKAPI MSK_putconname_t(MSKtask_t task,MSKint32t i,const char * name);
static MSK_putconname_t * MSK_putconname_ptr = (MSK_putconname_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putconname(MSKtask_t task,MSKint32t i,const char * name) {
  return (*MSK_putconname_ptr)(task,i,name);
} /* MSKputconname */
typedef MSKrescodee MSKAPI MSK_putvarname_t(MSKtask_t task,MSKint32t j,const char * name);
static MSK_putvarname_t * MSK_putvarname_ptr = (MSK_putvarname_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putvarname(MSKtask_t task,MSKint32t j,const char * name) {
  return (*MSK_putvarname_ptr)(task,j,name);
} /* MSKputvarname */
typedef MSKrescodee MSKAPI MSK_putconename_t(MSKtask_t task,MSKint32t j,const char * name);
static MSK_putconename_t * MSK_putconename_ptr = (MSK_putconename_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putconename(MSKtask_t task,MSKint32t j,const char * name) {
  return (*MSK_putconename_ptr)(task,j,name);
} /* MSKputconename */
typedef MSKrescodee MSKAPI MSK_putbarvarname_t(MSKtask_t task,MSKint32t j,const char * name);
static MSK_putbarvarname_t * MSK_putbarvarname_ptr = (MSK_putbarvarname_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putbarvarname(MSKtask_t task,MSKint32t j,const char * name) {
  return (*MSK_putbarvarname_ptr)(task,j,name);
} /* MSKputbarvarname */
typedef MSKrescodee MSKAPI MSK_putdomainname_t(MSKtask_t task,MSKint64t domidx,const char * name);
static MSK_putdomainname_t * MSK_putdomainname_ptr = (MSK_putdomainname_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putdomainname(MSKtask_t task,MSKint64t domidx,const char * name) {
  return (*MSK_putdomainname_ptr)(task,domidx,name);
} /* MSKputdomainname */
typedef MSKrescodee MSKAPI MSK_putdjcname_t(MSKtask_t task,MSKint64t djcidx,const char * name);
static MSK_putdjcname_t * MSK_putdjcname_ptr = (MSK_putdjcname_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putdjcname(MSKtask_t task,MSKint64t djcidx,const char * name) {
  return (*MSK_putdjcname_ptr)(task,djcidx,name);
} /* MSKputdjcname */
typedef MSKrescodee MSKAPI MSK_putaccname_t(MSKtask_t task,MSKint64t accidx,const char * name);
static MSK_putaccname_t * MSK_putaccname_ptr = (MSK_putaccname_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putaccname(MSKtask_t task,MSKint64t accidx,const char * name) {
  return (*MSK_putaccname_ptr)(task,accidx,name);
} /* MSKputaccname */
typedef MSKrescodee MSKAPI MSK_getvarnamelen_t(MSKtask_t task,MSKint32t i,MSKint32t * len);
static MSK_getvarnamelen_t * MSK_getvarnamelen_ptr = (MSK_getvarnamelen_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getvarnamelen(MSKtask_t task,MSKint32t i,MSKint32t * len) {
  return (*MSK_getvarnamelen_ptr)(task,i,len);
} /* MSKgetvarnamelen */
typedef MSKrescodee MSKAPI MSK_getvarname_t(MSKtask_t task,MSKint32t j,MSKint32t sizename,char * name);
static MSK_getvarname_t * MSK_getvarname_ptr = (MSK_getvarname_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getvarname(MSKtask_t task,MSKint32t j,MSKint32t sizename,char * name) {
  return (*MSK_getvarname_ptr)(task,j,sizename,name);
} /* MSKgetvarname */
typedef MSKrescodee MSKAPI MSK_getconnamelen_t(MSKtask_t task,MSKint32t i,MSKint32t * len);
static MSK_getconnamelen_t * MSK_getconnamelen_ptr = (MSK_getconnamelen_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getconnamelen(MSKtask_t task,MSKint32t i,MSKint32t * len) {
  return (*MSK_getconnamelen_ptr)(task,i,len);
} /* MSKgetconnamelen */
typedef MSKrescodee MSKAPI MSK_getconname_t(MSKtask_t task,MSKint32t i,MSKint32t sizename,char * name);
static MSK_getconname_t * MSK_getconname_ptr = (MSK_getconname_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getconname(MSKtask_t task,MSKint32t i,MSKint32t sizename,char * name) {
  return (*MSK_getconname_ptr)(task,i,sizename,name);
} /* MSKgetconname */
typedef MSKrescodee MSKAPI MSK_getconnameindex_t(MSKtask_t task,const char * somename,MSKint32t * asgn,MSKint32t * index);
static MSK_getconnameindex_t * MSK_getconnameindex_ptr = (MSK_getconnameindex_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getconnameindex(MSKtask_t task,const char * somename,MSKint32t * asgn,MSKint32t * index) {
  return (*MSK_getconnameindex_ptr)(task,somename,asgn,index);
} /* MSKgetconnameindex */
typedef MSKrescodee MSKAPI MSK_getvarnameindex_t(MSKtask_t task,const char * somename,MSKint32t * asgn,MSKint32t * index);
static MSK_getvarnameindex_t * MSK_getvarnameindex_ptr = (MSK_getvarnameindex_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getvarnameindex(MSKtask_t task,const char * somename,MSKint32t * asgn,MSKint32t * index) {
  return (*MSK_getvarnameindex_ptr)(task,somename,asgn,index);
} /* MSKgetvarnameindex */
typedef MSKrescodee MSKAPI MSK_getconenamelen_t(MSKtask_t task,MSKint32t i,MSKint32t * len);
static MSK_getconenamelen_t * MSK_getconenamelen_ptr = (MSK_getconenamelen_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getconenamelen(MSKtask_t task,MSKint32t i,MSKint32t * len) {
  return (*MSK_getconenamelen_ptr)(task,i,len);
} /* MSKgetconenamelen */
typedef MSKrescodee MSKAPI MSK_getconename_t(MSKtask_t task,MSKint32t i,MSKint32t sizename,char * name);
static MSK_getconename_t * MSK_getconename_ptr = (MSK_getconename_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getconename(MSKtask_t task,MSKint32t i,MSKint32t sizename,char * name) {
  return (*MSK_getconename_ptr)(task,i,sizename,name);
} /* MSKgetconename */
typedef MSKrescodee MSKAPI MSK_getconenameindex_t(MSKtask_t task,const char * somename,MSKint32t * asgn,MSKint32t * index);
static MSK_getconenameindex_t * MSK_getconenameindex_ptr = (MSK_getconenameindex_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getconenameindex(MSKtask_t task,const char * somename,MSKint32t * asgn,MSKint32t * index) {
  return (*MSK_getconenameindex_ptr)(task,somename,asgn,index);
} /* MSKgetconenameindex */
typedef MSKrescodee MSKAPI MSK_getdomainnamelen_t(MSKtask_t task,MSKint64t domidx,MSKint32t * len);
static MSK_getdomainnamelen_t * MSK_getdomainnamelen_ptr = (MSK_getdomainnamelen_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdomainnamelen(MSKtask_t task,MSKint64t domidx,MSKint32t * len) {
  return (*MSK_getdomainnamelen_ptr)(task,domidx,len);
} /* MSKgetdomainnamelen */
typedef MSKrescodee MSKAPI MSK_getdomainname_t(MSKtask_t task,MSKint64t domidx,MSKint32t sizename,char * name);
static MSK_getdomainname_t * MSK_getdomainname_ptr = (MSK_getdomainname_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdomainname(MSKtask_t task,MSKint64t domidx,MSKint32t sizename,char * name) {
  return (*MSK_getdomainname_ptr)(task,domidx,sizename,name);
} /* MSKgetdomainname */
typedef MSKrescodee MSKAPI MSK_getdjcnamelen_t(MSKtask_t task,MSKint64t djcidx,MSKint32t * len);
static MSK_getdjcnamelen_t * MSK_getdjcnamelen_ptr = (MSK_getdjcnamelen_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdjcnamelen(MSKtask_t task,MSKint64t djcidx,MSKint32t * len) {
  return (*MSK_getdjcnamelen_ptr)(task,djcidx,len);
} /* MSKgetdjcnamelen */
typedef MSKrescodee MSKAPI MSK_getdjcname_t(MSKtask_t task,MSKint64t djcidx,MSKint32t sizename,char * name);
static MSK_getdjcname_t * MSK_getdjcname_ptr = (MSK_getdjcname_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdjcname(MSKtask_t task,MSKint64t djcidx,MSKint32t sizename,char * name) {
  return (*MSK_getdjcname_ptr)(task,djcidx,sizename,name);
} /* MSKgetdjcname */
typedef MSKrescodee MSKAPI MSK_getaccnamelen_t(MSKtask_t task,MSKint64t accidx,MSKint32t * len);
static MSK_getaccnamelen_t * MSK_getaccnamelen_ptr = (MSK_getaccnamelen_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getaccnamelen(MSKtask_t task,MSKint64t accidx,MSKint32t * len) {
  return (*MSK_getaccnamelen_ptr)(task,accidx,len);
} /* MSKgetaccnamelen */
typedef MSKrescodee MSKAPI MSK_getaccname_t(MSKtask_t task,MSKint64t accidx,MSKint32t sizename,char * name);
static MSK_getaccname_t * MSK_getaccname_ptr = (MSK_getaccname_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getaccname(MSKtask_t task,MSKint64t accidx,MSKint32t sizename,char * name) {
  return (*MSK_getaccname_ptr)(task,accidx,sizename,name);
} /* MSKgetaccname */
typedef MSKrescodee MSKAPI MSK_getnastrparam_t(MSKtask_t task,const char * paramname,MSKint32t sizeparamname,MSKint32t * len,char * parvalue);
static MSK_getnastrparam_t * MSK_getnastrparam_ptr = (MSK_getnastrparam_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnastrparam(MSKtask_t task,const char * paramname,MSKint32t sizeparamname,MSKint32t * len,char * parvalue) {
  return (*MSK_getnastrparam_ptr)(task,paramname,sizeparamname,len,parvalue);
} /* MSKgetnastrparam */
typedef MSKrescodee MSKAPI MSK_getnumanz_t(MSKtask_t task,MSKint32t * numanz);
static MSK_getnumanz_t * MSK_getnumanz_ptr = (MSK_getnumanz_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnumanz(MSKtask_t task,MSKint32t * numanz) {
  return (*MSK_getnumanz_ptr)(task,numanz);
} /* MSKgetnumanz */
typedef MSKrescodee MSKAPI MSK_getnumanz64_t(MSKtask_t task,MSKint64t * numanz);
static MSK_getnumanz64_t * MSK_getnumanz64_ptr = (MSK_getnumanz64_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnumanz64(MSKtask_t task,MSKint64t * numanz) {
  return (*MSK_getnumanz64_ptr)(task,numanz);
} /* MSKgetnumanz64 */
typedef MSKrescodee MSKAPI MSK_getnumcon_t(MSKtask_t task,MSKint32t * numcon);
static MSK_getnumcon_t * MSK_getnumcon_ptr = (MSK_getnumcon_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnumcon(MSKtask_t task,MSKint32t * numcon) {
  return (*MSK_getnumcon_ptr)(task,numcon);
} /* MSKgetnumcon */
typedef MSKrescodee MSKAPI MSK_getnumcone_t(MSKtask_t task,MSKint32t * numcone);
static MSK_getnumcone_t * MSK_getnumcone_ptr = (MSK_getnumcone_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnumcone(MSKtask_t task,MSKint32t * numcone) {
  return (*MSK_getnumcone_ptr)(task,numcone);
} /* MSKgetnumcone */
typedef MSKrescodee MSKAPI MSK_getnumconemem_t(MSKtask_t task,MSKint32t k,MSKint32t * nummem);
static MSK_getnumconemem_t * MSK_getnumconemem_ptr = (MSK_getnumconemem_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnumconemem(MSKtask_t task,MSKint32t k,MSKint32t * nummem) {
  return (*MSK_getnumconemem_ptr)(task,k,nummem);
} /* MSKgetnumconemem */
typedef MSKrescodee MSKAPI MSK_getnumintvar_t(MSKtask_t task,MSKint32t * numintvar);
static MSK_getnumintvar_t * MSK_getnumintvar_ptr = (MSK_getnumintvar_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnumintvar(MSKtask_t task,MSKint32t * numintvar) {
  return (*MSK_getnumintvar_ptr)(task,numintvar);
} /* MSKgetnumintvar */
typedef MSKrescodee MSKAPI MSK_getnumparam_t(MSKtask_t task,MSKparametertypee partype,MSKint32t * numparam);
static MSK_getnumparam_t * MSK_getnumparam_ptr = (MSK_getnumparam_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnumparam(MSKtask_t task,MSKparametertypee partype,MSKint32t * numparam) {
  return (*MSK_getnumparam_ptr)(task,partype,numparam);
} /* MSKgetnumparam */
typedef MSKrescodee MSKAPI MSK_getnumqconknz_t(MSKtask_t task,MSKint32t k,MSKint32t * numqcnz);
static MSK_getnumqconknz_t * MSK_getnumqconknz_ptr = (MSK_getnumqconknz_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnumqconknz(MSKtask_t task,MSKint32t k,MSKint32t * numqcnz) {
  return (*MSK_getnumqconknz_ptr)(task,k,numqcnz);
} /* MSKgetnumqconknz */
typedef MSKrescodee MSKAPI MSK_getnumqconknz64_t(MSKtask_t task,MSKint32t k,MSKint64t * numqcnz);
static MSK_getnumqconknz64_t * MSK_getnumqconknz64_ptr = (MSK_getnumqconknz64_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnumqconknz64(MSKtask_t task,MSKint32t k,MSKint64t * numqcnz) {
  return (*MSK_getnumqconknz64_ptr)(task,k,numqcnz);
} /* MSKgetnumqconknz64 */
typedef MSKrescodee MSKAPI MSK_getnumqobjnz_t(MSKtask_t task,MSKint32t * numqonz);
static MSK_getnumqobjnz_t * MSK_getnumqobjnz_ptr = (MSK_getnumqobjnz_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnumqobjnz(MSKtask_t task,MSKint32t * numqonz) {
  return (*MSK_getnumqobjnz_ptr)(task,numqonz);
} /* MSKgetnumqobjnz */
typedef MSKrescodee MSKAPI MSK_getnumqobjnz64_t(MSKtask_t task,MSKint64t * numqonz);
static MSK_getnumqobjnz64_t * MSK_getnumqobjnz64_ptr = (MSK_getnumqobjnz64_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnumqobjnz64(MSKtask_t task,MSKint64t * numqonz) {
  return (*MSK_getnumqobjnz64_ptr)(task,numqonz);
} /* MSKgetnumqobjnz64 */
typedef MSKrescodee MSKAPI MSK_getnumvar_t(MSKtask_t task,MSKint32t * numvar);
static MSK_getnumvar_t * MSK_getnumvar_ptr = (MSK_getnumvar_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnumvar(MSKtask_t task,MSKint32t * numvar) {
  return (*MSK_getnumvar_ptr)(task,numvar);
} /* MSKgetnumvar */
typedef MSKrescodee MSKAPI MSK_getnumbarvar_t(MSKtask_t task,MSKint32t * numbarvar);
static MSK_getnumbarvar_t * MSK_getnumbarvar_ptr = (MSK_getnumbarvar_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnumbarvar(MSKtask_t task,MSKint32t * numbarvar) {
  return (*MSK_getnumbarvar_ptr)(task,numbarvar);
} /* MSKgetnumbarvar */
typedef MSKrescodee MSKAPI MSK_getmaxnumbarvar_t(MSKtask_t task,MSKint32t * maxnumbarvar);
static MSK_getmaxnumbarvar_t * MSK_getmaxnumbarvar_ptr = (MSK_getmaxnumbarvar_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getmaxnumbarvar(MSKtask_t task,MSKint32t * maxnumbarvar) {
  return (*MSK_getmaxnumbarvar_ptr)(task,maxnumbarvar);
} /* MSKgetmaxnumbarvar */
typedef MSKrescodee MSKAPI MSK_getdimbarvarj_t(MSKtask_t task,MSKint32t j,MSKint32t * dimbarvarj);
static MSK_getdimbarvarj_t * MSK_getdimbarvarj_ptr = (MSK_getdimbarvarj_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdimbarvarj(MSKtask_t task,MSKint32t j,MSKint32t * dimbarvarj) {
  return (*MSK_getdimbarvarj_ptr)(task,j,dimbarvarj);
} /* MSKgetdimbarvarj */
typedef MSKrescodee MSKAPI MSK_getlenbarvarj_t(MSKtask_t task,MSKint32t j,MSKint64t * lenbarvarj);
static MSK_getlenbarvarj_t * MSK_getlenbarvarj_ptr = (MSK_getlenbarvarj_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getlenbarvarj(MSKtask_t task,MSKint32t j,MSKint64t * lenbarvarj) {
  return (*MSK_getlenbarvarj_ptr)(task,j,lenbarvarj);
} /* MSKgetlenbarvarj */
typedef MSKrescodee MSKAPI MSK_getobjname_t(MSKtask_t task,MSKint32t sizeobjname,char * objname);
static MSK_getobjname_t * MSK_getobjname_ptr = (MSK_getobjname_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getobjname(MSKtask_t task,MSKint32t sizeobjname,char * objname) {
  return (*MSK_getobjname_ptr)(task,sizeobjname,objname);
} /* MSKgetobjname */
typedef MSKrescodee MSKAPI MSK_getobjnamelen_t(MSKtask_t task,MSKint32t * len);
static MSK_getobjnamelen_t * MSK_getobjnamelen_ptr = (MSK_getobjnamelen_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getobjnamelen(MSKtask_t task,MSKint32t * len) {
  return (*MSK_getobjnamelen_ptr)(task,len);
} /* MSKgetobjnamelen */
typedef MSKrescodee MSKAPI MSK_getparamname_t(MSKtask_t task,MSKparametertypee partype,MSKint32t param,char * parname);
static MSK_getparamname_t * MSK_getparamname_ptr = (MSK_getparamname_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getparamname(MSKtask_t task,MSKparametertypee partype,MSKint32t param,char * parname) {
  return (*MSK_getparamname_ptr)(task,partype,param,parname);
} /* MSKgetparamname */
typedef MSKrescodee MSKAPI MSK_getparammax_t(MSKtask_t task,MSKparametertypee partype,MSKint32t * parammax);
static MSK_getparammax_t * MSK_getparammax_ptr = (MSK_getparammax_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getparammax(MSKtask_t task,MSKparametertypee partype,MSKint32t * parammax) {
  return (*MSK_getparammax_ptr)(task,partype,parammax);
} /* MSKgetparammax */
typedef MSKrescodee MSKAPI MSK_getprimalobj_t(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * primalobj);
static MSK_getprimalobj_t * MSK_getprimalobj_ptr = (MSK_getprimalobj_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getprimalobj(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * primalobj) {
  return (*MSK_getprimalobj_ptr)(task,whichsol,primalobj);
} /* MSKgetprimalobj */
typedef MSKrescodee MSKAPI MSK_getprobtype_t(MSKtask_t task,MSKproblemtypee * probtype);
static MSK_getprobtype_t * MSK_getprobtype_ptr = (MSK_getprobtype_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getprobtype(MSKtask_t task,MSKproblemtypee * probtype) {
  return (*MSK_getprobtype_ptr)(task,probtype);
} /* MSKgetprobtype */
typedef MSKrescodee MSKAPI MSK_getqconk64_t(MSKtask_t task,MSKint32t k,MSKint64t maxnumqcnz,MSKint64t * numqcnz,MSKint32t * qcsubi,MSKint32t * qcsubj,MSKrealt * qcval);
static MSK_getqconk64_t * MSK_getqconk64_ptr = (MSK_getqconk64_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getqconk64(MSKtask_t task,MSKint32t k,MSKint64t maxnumqcnz,MSKint64t * numqcnz,MSKint32t * qcsubi,MSKint32t * qcsubj,MSKrealt * qcval) {
  return (*MSK_getqconk64_ptr)(task,k,maxnumqcnz,numqcnz,qcsubi,qcsubj,qcval);
} /* MSKgetqconk64 */
typedef MSKrescodee MSKAPI MSK_getqconk_t(MSKtask_t task,MSKint32t k,MSKint32t maxnumqcnz,MSKint32t * numqcnz,MSKint32t * qcsubi,MSKint32t * qcsubj,MSKrealt * qcval);
static MSK_getqconk_t * MSK_getqconk_ptr = (MSK_getqconk_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getqconk(MSKtask_t task,MSKint32t k,MSKint32t maxnumqcnz,MSKint32t * numqcnz,MSKint32t * qcsubi,MSKint32t * qcsubj,MSKrealt * qcval) {
  return (*MSK_getqconk_ptr)(task,k,maxnumqcnz,numqcnz,qcsubi,qcsubj,qcval);
} /* MSKgetqconk */
typedef MSKrescodee MSKAPI MSK_getqobj_t(MSKtask_t task,MSKint32t maxnumqonz,MSKint32t * numqonz,MSKint32t * qosubi,MSKint32t * qosubj,MSKrealt * qoval);
static MSK_getqobj_t * MSK_getqobj_ptr = (MSK_getqobj_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getqobj(MSKtask_t task,MSKint32t maxnumqonz,MSKint32t * numqonz,MSKint32t * qosubi,MSKint32t * qosubj,MSKrealt * qoval) {
  return (*MSK_getqobj_ptr)(task,maxnumqonz,numqonz,qosubi,qosubj,qoval);
} /* MSKgetqobj */
typedef MSKrescodee MSKAPI MSK_getqobj64_t(MSKtask_t task,MSKint64t maxnumqonz,MSKint64t * numqonz,MSKint32t * qosubi,MSKint32t * qosubj,MSKrealt * qoval);
static MSK_getqobj64_t * MSK_getqobj64_ptr = (MSK_getqobj64_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getqobj64(MSKtask_t task,MSKint64t maxnumqonz,MSKint64t * numqonz,MSKint32t * qosubi,MSKint32t * qosubj,MSKrealt * qoval) {
  return (*MSK_getqobj64_ptr)(task,maxnumqonz,numqonz,qosubi,qosubj,qoval);
} /* MSKgetqobj64 */
typedef MSKrescodee MSKAPI MSK_getqobjij_t(MSKtask_t task,MSKint32t i,MSKint32t j,MSKrealt * qoij);
static MSK_getqobjij_t * MSK_getqobjij_ptr = (MSK_getqobjij_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getqobjij(MSKtask_t task,MSKint32t i,MSKint32t j,MSKrealt * qoij) {
  return (*MSK_getqobjij_ptr)(task,i,j,qoij);
} /* MSKgetqobjij */
typedef MSKrescodee MSKAPI MSK_getsolution_t(MSKtask_t task,MSKsoltypee whichsol,MSKprostae * problemsta,MSKsolstae * solutionsta,MSKstakeye * skc,MSKstakeye * skx,MSKstakeye * skn,MSKrealt * xc,MSKrealt * xx,MSKrealt * y,MSKrealt * slc,MSKrealt * suc,MSKrealt * slx,MSKrealt * sux,MSKrealt * snx);
static MSK_getsolution_t * MSK_getsolution_ptr = (MSK_getsolution_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getsolution(MSKtask_t task,MSKsoltypee whichsol,MSKprostae * problemsta,MSKsolstae * solutionsta,MSKstakeye * skc,MSKstakeye * skx,MSKstakeye * skn,MSKrealt * xc,MSKrealt * xx,MSKrealt * y,MSKrealt * slc,MSKrealt * suc,MSKrealt * slx,MSKrealt * sux,MSKrealt * snx) {
  return (*MSK_getsolution_ptr)(task,whichsol,problemsta,solutionsta,skc,skx,skn,xc,xx,y,slc,suc,slx,sux,snx);
} /* MSKgetsolution */
typedef MSKrescodee MSKAPI MSK_getsolutionnew_t(MSKtask_t task,MSKsoltypee whichsol,MSKprostae * problemsta,MSKsolstae * solutionsta,MSKstakeye * skc,MSKstakeye * skx,MSKstakeye * skn,MSKrealt * xc,MSKrealt * xx,MSKrealt * y,MSKrealt * slc,MSKrealt * suc,MSKrealt * slx,MSKrealt * sux,MSKrealt * snx,MSKrealt * doty);
static MSK_getsolutionnew_t * MSK_getsolutionnew_ptr = (MSK_getsolutionnew_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getsolutionnew(MSKtask_t task,MSKsoltypee whichsol,MSKprostae * problemsta,MSKsolstae * solutionsta,MSKstakeye * skc,MSKstakeye * skx,MSKstakeye * skn,MSKrealt * xc,MSKrealt * xx,MSKrealt * y,MSKrealt * slc,MSKrealt * suc,MSKrealt * slx,MSKrealt * sux,MSKrealt * snx,MSKrealt * doty) {
  return (*MSK_getsolutionnew_ptr)(task,whichsol,problemsta,solutionsta,skc,skx,skn,xc,xx,y,slc,suc,slx,sux,snx,doty);
} /* MSKgetsolutionnew */
typedef MSKrescodee MSKAPI MSK_getsolsta_t(MSKtask_t task,MSKsoltypee whichsol,MSKsolstae * solutionsta);
static MSK_getsolsta_t * MSK_getsolsta_ptr = (MSK_getsolsta_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getsolsta(MSKtask_t task,MSKsoltypee whichsol,MSKsolstae * solutionsta) {
  return (*MSK_getsolsta_ptr)(task,whichsol,solutionsta);
} /* MSKgetsolsta */
typedef MSKrescodee MSKAPI MSK_getprosta_t(MSKtask_t task,MSKsoltypee whichsol,MSKprostae * problemsta);
static MSK_getprosta_t * MSK_getprosta_ptr = (MSK_getprosta_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getprosta(MSKtask_t task,MSKsoltypee whichsol,MSKprostae * problemsta) {
  return (*MSK_getprosta_ptr)(task,whichsol,problemsta);
} /* MSKgetprosta */
typedef MSKrescodee MSKAPI MSK_getskc_t(MSKtask_t task,MSKsoltypee whichsol,MSKstakeye * skc);
static MSK_getskc_t * MSK_getskc_ptr = (MSK_getskc_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getskc(MSKtask_t task,MSKsoltypee whichsol,MSKstakeye * skc) {
  return (*MSK_getskc_ptr)(task,whichsol,skc);
} /* MSKgetskc */
typedef MSKrescodee MSKAPI MSK_getskx_t(MSKtask_t task,MSKsoltypee whichsol,MSKstakeye * skx);
static MSK_getskx_t * MSK_getskx_ptr = (MSK_getskx_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getskx(MSKtask_t task,MSKsoltypee whichsol,MSKstakeye * skx) {
  return (*MSK_getskx_ptr)(task,whichsol,skx);
} /* MSKgetskx */
typedef MSKrescodee MSKAPI MSK_getskn_t(MSKtask_t task,MSKsoltypee whichsol,MSKstakeye * skn);
static MSK_getskn_t * MSK_getskn_ptr = (MSK_getskn_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getskn(MSKtask_t task,MSKsoltypee whichsol,MSKstakeye * skn) {
  return (*MSK_getskn_ptr)(task,whichsol,skn);
} /* MSKgetskn */
typedef MSKrescodee MSKAPI MSK_getxc_t(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * xc);
static MSK_getxc_t * MSK_getxc_ptr = (MSK_getxc_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getxc(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * xc) {
  return (*MSK_getxc_ptr)(task,whichsol,xc);
} /* MSKgetxc */
typedef MSKrescodee MSKAPI MSK_getxx_t(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * xx);
static MSK_getxx_t * MSK_getxx_ptr = (MSK_getxx_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getxx(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * xx) {
  return (*MSK_getxx_ptr)(task,whichsol,xx);
} /* MSKgetxx */
typedef MSKrescodee MSKAPI MSK_gety_t(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * y);
static MSK_gety_t * MSK_gety_ptr = (MSK_gety_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_gety(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * y) {
  return (*MSK_gety_ptr)(task,whichsol,y);
} /* MSKgety */
typedef MSKrescodee MSKAPI MSK_getslc_t(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * slc);
static MSK_getslc_t * MSK_getslc_ptr = (MSK_getslc_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getslc(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * slc) {
  return (*MSK_getslc_ptr)(task,whichsol,slc);
} /* MSKgetslc */
typedef MSKrescodee MSKAPI MSK_getaccdoty_t(MSKtask_t task,MSKsoltypee whichsol,MSKint64t accidx,MSKrealt * doty);
static MSK_getaccdoty_t * MSK_getaccdoty_ptr = (MSK_getaccdoty_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getaccdoty(MSKtask_t task,MSKsoltypee whichsol,MSKint64t accidx,MSKrealt * doty) {
  return (*MSK_getaccdoty_ptr)(task,whichsol,accidx,doty);
} /* MSKgetaccdoty */
typedef MSKrescodee MSKAPI MSK_getaccdotys_t(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * doty);
static MSK_getaccdotys_t * MSK_getaccdotys_ptr = (MSK_getaccdotys_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getaccdotys(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * doty) {
  return (*MSK_getaccdotys_ptr)(task,whichsol,doty);
} /* MSKgetaccdotys */
typedef MSKrescodee MSKAPI MSK_evaluateacc_t(MSKtask_t task,MSKsoltypee whichsol,MSKint64t accidx,MSKrealt * activity);
static MSK_evaluateacc_t * MSK_evaluateacc_ptr = (MSK_evaluateacc_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_evaluateacc(MSKtask_t task,MSKsoltypee whichsol,MSKint64t accidx,MSKrealt * activity) {
  return (*MSK_evaluateacc_ptr)(task,whichsol,accidx,activity);
} /* MSKevaluateacc */
typedef MSKrescodee MSKAPI MSK_evaluateaccs_t(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * activity);
static MSK_evaluateaccs_t * MSK_evaluateaccs_ptr = (MSK_evaluateaccs_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_evaluateaccs(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * activity) {
  return (*MSK_evaluateaccs_ptr)(task,whichsol,activity);
} /* MSKevaluateaccs */
typedef MSKrescodee MSKAPI MSK_getsuc_t(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * suc);
static MSK_getsuc_t * MSK_getsuc_ptr = (MSK_getsuc_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getsuc(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * suc) {
  return (*MSK_getsuc_ptr)(task,whichsol,suc);
} /* MSKgetsuc */
typedef MSKrescodee MSKAPI MSK_getslx_t(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * slx);
static MSK_getslx_t * MSK_getslx_ptr = (MSK_getslx_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getslx(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * slx) {
  return (*MSK_getslx_ptr)(task,whichsol,slx);
} /* MSKgetslx */
typedef MSKrescodee MSKAPI MSK_getsux_t(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * sux);
static MSK_getsux_t * MSK_getsux_ptr = (MSK_getsux_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getsux(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * sux) {
  return (*MSK_getsux_ptr)(task,whichsol,sux);
} /* MSKgetsux */
typedef MSKrescodee MSKAPI MSK_getsnx_t(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * snx);
static MSK_getsnx_t * MSK_getsnx_ptr = (MSK_getsnx_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getsnx(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * snx) {
  return (*MSK_getsnx_ptr)(task,whichsol,snx);
} /* MSKgetsnx */
typedef MSKrescodee MSKAPI MSK_getskcslice_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKstakeye * skc);
static MSK_getskcslice_t * MSK_getskcslice_ptr = (MSK_getskcslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getskcslice(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKstakeye * skc) {
  return (*MSK_getskcslice_ptr)(task,whichsol,first,last,skc);
} /* MSKgetskcslice */
typedef MSKrescodee MSKAPI MSK_getskxslice_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKstakeye * skx);
static MSK_getskxslice_t * MSK_getskxslice_ptr = (MSK_getskxslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getskxslice(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKstakeye * skx) {
  return (*MSK_getskxslice_ptr)(task,whichsol,first,last,skx);
} /* MSKgetskxslice */
typedef MSKrescodee MSKAPI MSK_getxcslice_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKrealt * xc);
static MSK_getxcslice_t * MSK_getxcslice_ptr = (MSK_getxcslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getxcslice(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKrealt * xc) {
  return (*MSK_getxcslice_ptr)(task,whichsol,first,last,xc);
} /* MSKgetxcslice */
typedef MSKrescodee MSKAPI MSK_getxxslice_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKrealt * xx);
static MSK_getxxslice_t * MSK_getxxslice_ptr = (MSK_getxxslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getxxslice(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKrealt * xx) {
  return (*MSK_getxxslice_ptr)(task,whichsol,first,last,xx);
} /* MSKgetxxslice */
typedef MSKrescodee MSKAPI MSK_getyslice_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKrealt * y);
static MSK_getyslice_t * MSK_getyslice_ptr = (MSK_getyslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getyslice(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKrealt * y) {
  return (*MSK_getyslice_ptr)(task,whichsol,first,last,y);
} /* MSKgetyslice */
typedef MSKrescodee MSKAPI MSK_getslcslice_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKrealt * slc);
static MSK_getslcslice_t * MSK_getslcslice_ptr = (MSK_getslcslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getslcslice(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKrealt * slc) {
  return (*MSK_getslcslice_ptr)(task,whichsol,first,last,slc);
} /* MSKgetslcslice */
typedef MSKrescodee MSKAPI MSK_getsucslice_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKrealt * suc);
static MSK_getsucslice_t * MSK_getsucslice_ptr = (MSK_getsucslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getsucslice(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKrealt * suc) {
  return (*MSK_getsucslice_ptr)(task,whichsol,first,last,suc);
} /* MSKgetsucslice */
typedef MSKrescodee MSKAPI MSK_getslxslice_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKrealt * slx);
static MSK_getslxslice_t * MSK_getslxslice_ptr = (MSK_getslxslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getslxslice(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKrealt * slx) {
  return (*MSK_getslxslice_ptr)(task,whichsol,first,last,slx);
} /* MSKgetslxslice */
typedef MSKrescodee MSKAPI MSK_getsuxslice_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKrealt * sux);
static MSK_getsuxslice_t * MSK_getsuxslice_ptr = (MSK_getsuxslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getsuxslice(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKrealt * sux) {
  return (*MSK_getsuxslice_ptr)(task,whichsol,first,last,sux);
} /* MSKgetsuxslice */
typedef MSKrescodee MSKAPI MSK_getsnxslice_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKrealt * snx);
static MSK_getsnxslice_t * MSK_getsnxslice_ptr = (MSK_getsnxslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getsnxslice(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKrealt * snx) {
  return (*MSK_getsnxslice_ptr)(task,whichsol,first,last,snx);
} /* MSKgetsnxslice */
typedef MSKrescodee MSKAPI MSK_getbarxj_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t j,MSKrealt * barxj);
static MSK_getbarxj_t * MSK_getbarxj_ptr = (MSK_getbarxj_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getbarxj(MSKtask_t task,MSKsoltypee whichsol,MSKint32t j,MSKrealt * barxj) {
  return (*MSK_getbarxj_ptr)(task,whichsol,j,barxj);
} /* MSKgetbarxj */
typedef MSKrescodee MSKAPI MSK_getbarxslice_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKint64t slicesize,MSKrealt * barxslice);
static MSK_getbarxslice_t * MSK_getbarxslice_ptr = (MSK_getbarxslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getbarxslice(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKint64t slicesize,MSKrealt * barxslice) {
  return (*MSK_getbarxslice_ptr)(task,whichsol,first,last,slicesize,barxslice);
} /* MSKgetbarxslice */
typedef MSKrescodee MSKAPI MSK_getbarsj_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t j,MSKrealt * barsj);
static MSK_getbarsj_t * MSK_getbarsj_ptr = (MSK_getbarsj_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getbarsj(MSKtask_t task,MSKsoltypee whichsol,MSKint32t j,MSKrealt * barsj) {
  return (*MSK_getbarsj_ptr)(task,whichsol,j,barsj);
} /* MSKgetbarsj */
typedef MSKrescodee MSKAPI MSK_getbarsslice_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKint64t slicesize,MSKrealt * barsslice);
static MSK_getbarsslice_t * MSK_getbarsslice_ptr = (MSK_getbarsslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getbarsslice(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKint64t slicesize,MSKrealt * barsslice) {
  return (*MSK_getbarsslice_ptr)(task,whichsol,first,last,slicesize,barsslice);
} /* MSKgetbarsslice */
typedef MSKrescodee MSKAPI MSK_putskc_t(MSKtask_t task,MSKsoltypee whichsol,const MSKstakeye * skc);
static MSK_putskc_t * MSK_putskc_ptr = (MSK_putskc_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putskc(MSKtask_t task,MSKsoltypee whichsol,const MSKstakeye * skc) {
  return (*MSK_putskc_ptr)(task,whichsol,skc);
} /* MSKputskc */
typedef MSKrescodee MSKAPI MSK_putskx_t(MSKtask_t task,MSKsoltypee whichsol,const MSKstakeye * skx);
static MSK_putskx_t * MSK_putskx_ptr = (MSK_putskx_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putskx(MSKtask_t task,MSKsoltypee whichsol,const MSKstakeye * skx) {
  return (*MSK_putskx_ptr)(task,whichsol,skx);
} /* MSKputskx */
typedef MSKrescodee MSKAPI MSK_putxc_t(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * xc);
static MSK_putxc_t * MSK_putxc_ptr = (MSK_putxc_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putxc(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * xc) {
  return (*MSK_putxc_ptr)(task,whichsol,xc);
} /* MSKputxc */
typedef MSKrescodee MSKAPI MSK_putxx_t(MSKtask_t task,MSKsoltypee whichsol,const MSKrealt * xx);
static MSK_putxx_t * MSK_putxx_ptr = (MSK_putxx_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putxx(MSKtask_t task,MSKsoltypee whichsol,const MSKrealt * xx) {
  return (*MSK_putxx_ptr)(task,whichsol,xx);
} /* MSKputxx */
typedef MSKrescodee MSKAPI MSK_puty_t(MSKtask_t task,MSKsoltypee whichsol,const MSKrealt * y);
static MSK_puty_t * MSK_puty_ptr = (MSK_puty_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_puty(MSKtask_t task,MSKsoltypee whichsol,const MSKrealt * y) {
  return (*MSK_puty_ptr)(task,whichsol,y);
} /* MSKputy */
typedef MSKrescodee MSKAPI MSK_putslc_t(MSKtask_t task,MSKsoltypee whichsol,const MSKrealt * slc);
static MSK_putslc_t * MSK_putslc_ptr = (MSK_putslc_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putslc(MSKtask_t task,MSKsoltypee whichsol,const MSKrealt * slc) {
  return (*MSK_putslc_ptr)(task,whichsol,slc);
} /* MSKputslc */
typedef MSKrescodee MSKAPI MSK_putsuc_t(MSKtask_t task,MSKsoltypee whichsol,const MSKrealt * suc);
static MSK_putsuc_t * MSK_putsuc_ptr = (MSK_putsuc_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putsuc(MSKtask_t task,MSKsoltypee whichsol,const MSKrealt * suc) {
  return (*MSK_putsuc_ptr)(task,whichsol,suc);
} /* MSKputsuc */
typedef MSKrescodee MSKAPI MSK_putslx_t(MSKtask_t task,MSKsoltypee whichsol,const MSKrealt * slx);
static MSK_putslx_t * MSK_putslx_ptr = (MSK_putslx_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putslx(MSKtask_t task,MSKsoltypee whichsol,const MSKrealt * slx) {
  return (*MSK_putslx_ptr)(task,whichsol,slx);
} /* MSKputslx */
typedef MSKrescodee MSKAPI MSK_putsux_t(MSKtask_t task,MSKsoltypee whichsol,const MSKrealt * sux);
static MSK_putsux_t * MSK_putsux_ptr = (MSK_putsux_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putsux(MSKtask_t task,MSKsoltypee whichsol,const MSKrealt * sux) {
  return (*MSK_putsux_ptr)(task,whichsol,sux);
} /* MSKputsux */
typedef MSKrescodee MSKAPI MSK_putsnx_t(MSKtask_t task,MSKsoltypee whichsol,const MSKrealt * sux);
static MSK_putsnx_t * MSK_putsnx_ptr = (MSK_putsnx_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putsnx(MSKtask_t task,MSKsoltypee whichsol,const MSKrealt * sux) {
  return (*MSK_putsnx_ptr)(task,whichsol,sux);
} /* MSKputsnx */
typedef MSKrescodee MSKAPI MSK_putaccdoty_t(MSKtask_t task,MSKsoltypee whichsol,MSKint64t accidx,MSKrealt * doty);
static MSK_putaccdoty_t * MSK_putaccdoty_ptr = (MSK_putaccdoty_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putaccdoty(MSKtask_t task,MSKsoltypee whichsol,MSKint64t accidx,MSKrealt * doty) {
  return (*MSK_putaccdoty_ptr)(task,whichsol,accidx,doty);
} /* MSKputaccdoty */
typedef MSKrescodee MSKAPI MSK_putskcslice_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,const MSKstakeye * skc);
static MSK_putskcslice_t * MSK_putskcslice_ptr = (MSK_putskcslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putskcslice(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,const MSKstakeye * skc) {
  return (*MSK_putskcslice_ptr)(task,whichsol,first,last,skc);
} /* MSKputskcslice */
typedef MSKrescodee MSKAPI MSK_putskxslice_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,const MSKstakeye * skx);
static MSK_putskxslice_t * MSK_putskxslice_ptr = (MSK_putskxslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putskxslice(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,const MSKstakeye * skx) {
  return (*MSK_putskxslice_ptr)(task,whichsol,first,last,skx);
} /* MSKputskxslice */
typedef MSKrescodee MSKAPI MSK_putxcslice_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,const MSKrealt * xc);
static MSK_putxcslice_t * MSK_putxcslice_ptr = (MSK_putxcslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putxcslice(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,const MSKrealt * xc) {
  return (*MSK_putxcslice_ptr)(task,whichsol,first,last,xc);
} /* MSKputxcslice */
typedef MSKrescodee MSKAPI MSK_putxxslice_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,const MSKrealt * xx);
static MSK_putxxslice_t * MSK_putxxslice_ptr = (MSK_putxxslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putxxslice(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,const MSKrealt * xx) {
  return (*MSK_putxxslice_ptr)(task,whichsol,first,last,xx);
} /* MSKputxxslice */
typedef MSKrescodee MSKAPI MSK_putyslice_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,const MSKrealt * y);
static MSK_putyslice_t * MSK_putyslice_ptr = (MSK_putyslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putyslice(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,const MSKrealt * y) {
  return (*MSK_putyslice_ptr)(task,whichsol,first,last,y);
} /* MSKputyslice */
typedef MSKrescodee MSKAPI MSK_putslcslice_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,const MSKrealt * slc);
static MSK_putslcslice_t * MSK_putslcslice_ptr = (MSK_putslcslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putslcslice(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,const MSKrealt * slc) {
  return (*MSK_putslcslice_ptr)(task,whichsol,first,last,slc);
} /* MSKputslcslice */
typedef MSKrescodee MSKAPI MSK_putsucslice_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,const MSKrealt * suc);
static MSK_putsucslice_t * MSK_putsucslice_ptr = (MSK_putsucslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putsucslice(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,const MSKrealt * suc) {
  return (*MSK_putsucslice_ptr)(task,whichsol,first,last,suc);
} /* MSKputsucslice */
typedef MSKrescodee MSKAPI MSK_putslxslice_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,const MSKrealt * slx);
static MSK_putslxslice_t * MSK_putslxslice_ptr = (MSK_putslxslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putslxslice(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,const MSKrealt * slx) {
  return (*MSK_putslxslice_ptr)(task,whichsol,first,last,slx);
} /* MSKputslxslice */
typedef MSKrescodee MSKAPI MSK_putsuxslice_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,const MSKrealt * sux);
static MSK_putsuxslice_t * MSK_putsuxslice_ptr = (MSK_putsuxslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putsuxslice(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,const MSKrealt * sux) {
  return (*MSK_putsuxslice_ptr)(task,whichsol,first,last,sux);
} /* MSKputsuxslice */
typedef MSKrescodee MSKAPI MSK_putsnxslice_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,const MSKrealt * snx);
static MSK_putsnxslice_t * MSK_putsnxslice_ptr = (MSK_putsnxslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putsnxslice(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,const MSKrealt * snx) {
  return (*MSK_putsnxslice_ptr)(task,whichsol,first,last,snx);
} /* MSKputsnxslice */
typedef MSKrescodee MSKAPI MSK_putbarxj_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t j,const MSKrealt * barxj);
static MSK_putbarxj_t * MSK_putbarxj_ptr = (MSK_putbarxj_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putbarxj(MSKtask_t task,MSKsoltypee whichsol,MSKint32t j,const MSKrealt * barxj) {
  return (*MSK_putbarxj_ptr)(task,whichsol,j,barxj);
} /* MSKputbarxj */
typedef MSKrescodee MSKAPI MSK_putbarsj_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t j,const MSKrealt * barsj);
static MSK_putbarsj_t * MSK_putbarsj_ptr = (MSK_putbarsj_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putbarsj(MSKtask_t task,MSKsoltypee whichsol,MSKint32t j,const MSKrealt * barsj) {
  return (*MSK_putbarsj_ptr)(task,whichsol,j,barsj);
} /* MSKputbarsj */
typedef MSKrescodee MSKAPI MSK_getpviolcon_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t num,const MSKint32t * sub,MSKrealt * viol);
static MSK_getpviolcon_t * MSK_getpviolcon_ptr = (MSK_getpviolcon_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getpviolcon(MSKtask_t task,MSKsoltypee whichsol,MSKint32t num,const MSKint32t * sub,MSKrealt * viol) {
  return (*MSK_getpviolcon_ptr)(task,whichsol,num,sub,viol);
} /* MSKgetpviolcon */
typedef MSKrescodee MSKAPI MSK_getpviolvar_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t num,const MSKint32t * sub,MSKrealt * viol);
static MSK_getpviolvar_t * MSK_getpviolvar_ptr = (MSK_getpviolvar_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getpviolvar(MSKtask_t task,MSKsoltypee whichsol,MSKint32t num,const MSKint32t * sub,MSKrealt * viol) {
  return (*MSK_getpviolvar_ptr)(task,whichsol,num,sub,viol);
} /* MSKgetpviolvar */
typedef MSKrescodee MSKAPI MSK_getpviolbarvar_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t num,const MSKint32t * sub,MSKrealt * viol);
static MSK_getpviolbarvar_t * MSK_getpviolbarvar_ptr = (MSK_getpviolbarvar_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getpviolbarvar(MSKtask_t task,MSKsoltypee whichsol,MSKint32t num,const MSKint32t * sub,MSKrealt * viol) {
  return (*MSK_getpviolbarvar_ptr)(task,whichsol,num,sub,viol);
} /* MSKgetpviolbarvar */
typedef MSKrescodee MSKAPI MSK_getpviolcones_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t num,const MSKint32t * sub,MSKrealt * viol);
static MSK_getpviolcones_t * MSK_getpviolcones_ptr = (MSK_getpviolcones_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getpviolcones(MSKtask_t task,MSKsoltypee whichsol,MSKint32t num,const MSKint32t * sub,MSKrealt * viol) {
  return (*MSK_getpviolcones_ptr)(task,whichsol,num,sub,viol);
} /* MSKgetpviolcones */
typedef MSKrescodee MSKAPI MSK_getpviolacc_t(MSKtask_t task,MSKsoltypee whichsol,MSKint64t numaccidx,const MSKint64t * accidxlist,MSKrealt * viol);
static MSK_getpviolacc_t * MSK_getpviolacc_ptr = (MSK_getpviolacc_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getpviolacc(MSKtask_t task,MSKsoltypee whichsol,MSKint64t numaccidx,const MSKint64t * accidxlist,MSKrealt * viol) {
  return (*MSK_getpviolacc_ptr)(task,whichsol,numaccidx,accidxlist,viol);
} /* MSKgetpviolacc */
typedef MSKrescodee MSKAPI MSK_getpvioldjc_t(MSKtask_t task,MSKsoltypee whichsol,MSKint64t numdjcidx,const MSKint64t * djcidxlist,MSKrealt * viol);
static MSK_getpvioldjc_t * MSK_getpvioldjc_ptr = (MSK_getpvioldjc_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getpvioldjc(MSKtask_t task,MSKsoltypee whichsol,MSKint64t numdjcidx,const MSKint64t * djcidxlist,MSKrealt * viol) {
  return (*MSK_getpvioldjc_ptr)(task,whichsol,numdjcidx,djcidxlist,viol);
} /* MSKgetpvioldjc */
typedef MSKrescodee MSKAPI MSK_getdviolcon_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t num,const MSKint32t * sub,MSKrealt * viol);
static MSK_getdviolcon_t * MSK_getdviolcon_ptr = (MSK_getdviolcon_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdviolcon(MSKtask_t task,MSKsoltypee whichsol,MSKint32t num,const MSKint32t * sub,MSKrealt * viol) {
  return (*MSK_getdviolcon_ptr)(task,whichsol,num,sub,viol);
} /* MSKgetdviolcon */
typedef MSKrescodee MSKAPI MSK_getdviolvar_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t num,const MSKint32t * sub,MSKrealt * viol);
static MSK_getdviolvar_t * MSK_getdviolvar_ptr = (MSK_getdviolvar_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdviolvar(MSKtask_t task,MSKsoltypee whichsol,MSKint32t num,const MSKint32t * sub,MSKrealt * viol) {
  return (*MSK_getdviolvar_ptr)(task,whichsol,num,sub,viol);
} /* MSKgetdviolvar */
typedef MSKrescodee MSKAPI MSK_getdviolbarvar_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t num,const MSKint32t * sub,MSKrealt * viol);
static MSK_getdviolbarvar_t * MSK_getdviolbarvar_ptr = (MSK_getdviolbarvar_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdviolbarvar(MSKtask_t task,MSKsoltypee whichsol,MSKint32t num,const MSKint32t * sub,MSKrealt * viol) {
  return (*MSK_getdviolbarvar_ptr)(task,whichsol,num,sub,viol);
} /* MSKgetdviolbarvar */
typedef MSKrescodee MSKAPI MSK_getdviolcones_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t num,const MSKint32t * sub,MSKrealt * viol);
static MSK_getdviolcones_t * MSK_getdviolcones_ptr = (MSK_getdviolcones_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdviolcones(MSKtask_t task,MSKsoltypee whichsol,MSKint32t num,const MSKint32t * sub,MSKrealt * viol) {
  return (*MSK_getdviolcones_ptr)(task,whichsol,num,sub,viol);
} /* MSKgetdviolcones */
typedef MSKrescodee MSKAPI MSK_getdviolacc_t(MSKtask_t task,MSKsoltypee whichsol,MSKint64t numaccidx,const MSKint64t * accidxlist,MSKrealt * viol);
static MSK_getdviolacc_t * MSK_getdviolacc_ptr = (MSK_getdviolacc_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdviolacc(MSKtask_t task,MSKsoltypee whichsol,MSKint64t numaccidx,const MSKint64t * accidxlist,MSKrealt * viol) {
  return (*MSK_getdviolacc_ptr)(task,whichsol,numaccidx,accidxlist,viol);
} /* MSKgetdviolacc */
typedef MSKrescodee MSKAPI MSK_getsolutioninfo_t(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * pobj,MSKrealt * pviolcon,MSKrealt * pviolvar,MSKrealt * pviolbarvar,MSKrealt * pviolcone,MSKrealt * pviolitg,MSKrealt * dobj,MSKrealt * dviolcon,MSKrealt * dviolvar,MSKrealt * dviolbarvar,MSKrealt * dviolcone);
static MSK_getsolutioninfo_t * MSK_getsolutioninfo_ptr = (MSK_getsolutioninfo_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getsolutioninfo(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * pobj,MSKrealt * pviolcon,MSKrealt * pviolvar,MSKrealt * pviolbarvar,MSKrealt * pviolcone,MSKrealt * pviolitg,MSKrealt * dobj,MSKrealt * dviolcon,MSKrealt * dviolvar,MSKrealt * dviolbarvar,MSKrealt * dviolcone) {
  return (*MSK_getsolutioninfo_ptr)(task,whichsol,pobj,pviolcon,pviolvar,pviolbarvar,pviolcone,pviolitg,dobj,dviolcon,dviolvar,dviolbarvar,dviolcone);
} /* MSKgetsolutioninfo */
typedef MSKrescodee MSKAPI MSK_getsolutioninfonew_t(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * pobj,MSKrealt * pviolcon,MSKrealt * pviolvar,MSKrealt * pviolbarvar,MSKrealt * pviolcone,MSKrealt * pviolacc,MSKrealt * pvioldjc,MSKrealt * pviolitg,MSKrealt * dobj,MSKrealt * dviolcon,MSKrealt * dviolvar,MSKrealt * dviolbarvar,MSKrealt * dviolcone,MSKrealt * dviolacc);
static MSK_getsolutioninfonew_t * MSK_getsolutioninfonew_ptr = (MSK_getsolutioninfonew_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getsolutioninfonew(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * pobj,MSKrealt * pviolcon,MSKrealt * pviolvar,MSKrealt * pviolbarvar,MSKrealt * pviolcone,MSKrealt * pviolacc,MSKrealt * pvioldjc,MSKrealt * pviolitg,MSKrealt * dobj,MSKrealt * dviolcon,MSKrealt * dviolvar,MSKrealt * dviolbarvar,MSKrealt * dviolcone,MSKrealt * dviolacc) {
  return (*MSK_getsolutioninfonew_ptr)(task,whichsol,pobj,pviolcon,pviolvar,pviolbarvar,pviolcone,pviolacc,pvioldjc,pviolitg,dobj,dviolcon,dviolvar,dviolbarvar,dviolcone,dviolacc);
} /* MSKgetsolutioninfonew */
typedef MSKrescodee MSKAPI MSK_getdualsolutionnorms_t(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * nrmy,MSKrealt * nrmslc,MSKrealt * nrmsuc,MSKrealt * nrmslx,MSKrealt * nrmsux,MSKrealt * nrmsnx,MSKrealt * nrmbars);
static MSK_getdualsolutionnorms_t * MSK_getdualsolutionnorms_ptr = (MSK_getdualsolutionnorms_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdualsolutionnorms(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * nrmy,MSKrealt * nrmslc,MSKrealt * nrmsuc,MSKrealt * nrmslx,MSKrealt * nrmsux,MSKrealt * nrmsnx,MSKrealt * nrmbars) {
  return (*MSK_getdualsolutionnorms_ptr)(task,whichsol,nrmy,nrmslc,nrmsuc,nrmslx,nrmsux,nrmsnx,nrmbars);
} /* MSKgetdualsolutionnorms */
typedef MSKrescodee MSKAPI MSK_getprimalsolutionnorms_t(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * nrmxc,MSKrealt * nrmxx,MSKrealt * nrmbarx);
static MSK_getprimalsolutionnorms_t * MSK_getprimalsolutionnorms_ptr = (MSK_getprimalsolutionnorms_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getprimalsolutionnorms(MSKtask_t task,MSKsoltypee whichsol,MSKrealt * nrmxc,MSKrealt * nrmxx,MSKrealt * nrmbarx) {
  return (*MSK_getprimalsolutionnorms_ptr)(task,whichsol,nrmxc,nrmxx,nrmbarx);
} /* MSKgetprimalsolutionnorms */
typedef MSKrescodee MSKAPI MSK_getsolutionslice_t(MSKtask_t task,MSKsoltypee whichsol,MSKsoliteme solitem,MSKint32t first,MSKint32t last,MSKrealt * values);
static MSK_getsolutionslice_t * MSK_getsolutionslice_ptr = (MSK_getsolutionslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getsolutionslice(MSKtask_t task,MSKsoltypee whichsol,MSKsoliteme solitem,MSKint32t first,MSKint32t last,MSKrealt * values) {
  return (*MSK_getsolutionslice_ptr)(task,whichsol,solitem,first,last,values);
} /* MSKgetsolutionslice */
typedef MSKrescodee MSKAPI MSK_getreducedcosts_t(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKrealt * redcosts);
static MSK_getreducedcosts_t * MSK_getreducedcosts_ptr = (MSK_getreducedcosts_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getreducedcosts(MSKtask_t task,MSKsoltypee whichsol,MSKint32t first,MSKint32t last,MSKrealt * redcosts) {
  return (*MSK_getreducedcosts_ptr)(task,whichsol,first,last,redcosts);
} /* MSKgetreducedcosts */
typedef MSKrescodee MSKAPI MSK_getstrparam_t(MSKtask_t task,MSKsparame param,MSKint32t maxlen,MSKint32t * len,char * parvalue);
static MSK_getstrparam_t * MSK_getstrparam_ptr = (MSK_getstrparam_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getstrparam(MSKtask_t task,MSKsparame param,MSKint32t maxlen,MSKint32t * len,char * parvalue) {
  return (*MSK_getstrparam_ptr)(task,param,maxlen,len,parvalue);
} /* MSKgetstrparam */
typedef MSKrescodee MSKAPI MSK_getstrparamlen_t(MSKtask_t task,MSKsparame param,MSKint32t * len);
static MSK_getstrparamlen_t * MSK_getstrparamlen_ptr = (MSK_getstrparamlen_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getstrparamlen(MSKtask_t task,MSKsparame param,MSKint32t * len) {
  return (*MSK_getstrparamlen_ptr)(task,param,len);
} /* MSKgetstrparamlen */
typedef MSKrescodee MSKAPI MSK_getstrparamal_t(MSKtask_t task,MSKsparame param,MSKint32t numaddchr,MSKstring_t * value);
static MSK_getstrparamal_t * MSK_getstrparamal_ptr = (MSK_getstrparamal_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getstrparamal(MSKtask_t task,MSKsparame param,MSKint32t numaddchr,MSKstring_t * value) {
  return (*MSK_getstrparamal_ptr)(task,param,numaddchr,value);
} /* MSKgetstrparamal */
typedef MSKrescodee MSKAPI MSK_getnastrparamal_t(MSKtask_t task,const char * paramname,MSKint32t numaddchr,MSKstring_t * value);
static MSK_getnastrparamal_t * MSK_getnastrparamal_ptr = (MSK_getnastrparamal_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnastrparamal(MSKtask_t task,const char * paramname,MSKint32t numaddchr,MSKstring_t * value) {
  return (*MSK_getnastrparamal_ptr)(task,paramname,numaddchr,value);
} /* MSKgetnastrparamal */
typedef MSKrescodee MSKAPI MSK_getsymbcon_t(MSKtask_t task,MSKint32t i,MSKint32t sizevalue,char * name,MSKint32t * value);
static MSK_getsymbcon_t * MSK_getsymbcon_ptr = (MSK_getsymbcon_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getsymbcon(MSKtask_t task,MSKint32t i,MSKint32t sizevalue,char * name,MSKint32t * value) {
  return (*MSK_getsymbcon_ptr)(task,i,sizevalue,name,value);
} /* MSKgetsymbcon */
typedef MSKrescodee MSKAPI MSK_gettasknamelen_t(MSKtask_t task,MSKint32t * len);
static MSK_gettasknamelen_t * MSK_gettasknamelen_ptr = (MSK_gettasknamelen_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_gettasknamelen(MSKtask_t task,MSKint32t * len) {
  return (*MSK_gettasknamelen_ptr)(task,len);
} /* MSKgettasknamelen */
typedef MSKrescodee MSKAPI MSK_gettaskname_t(MSKtask_t task,MSKint32t sizetaskname,char * taskname);
static MSK_gettaskname_t * MSK_gettaskname_ptr = (MSK_gettaskname_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_gettaskname(MSKtask_t task,MSKint32t sizetaskname,char * taskname) {
  return (*MSK_gettaskname_ptr)(task,sizetaskname,taskname);
} /* MSKgettaskname */
typedef MSKrescodee MSKAPI MSK_getmionumthreads_t(MSKtask_t task,MSKint32t * numthreads);
static MSK_getmionumthreads_t * MSK_getmionumthreads_ptr = (MSK_getmionumthreads_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getmionumthreads(MSKtask_t task,MSKint32t * numthreads) {
  return (*MSK_getmionumthreads_ptr)(task,numthreads);
} /* MSKgetmionumthreads */
typedef MSKrescodee MSKAPI MSK_getvartype_t(MSKtask_t task,MSKint32t j,MSKvariabletypee * vartype);
static MSK_getvartype_t * MSK_getvartype_ptr = (MSK_getvartype_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getvartype(MSKtask_t task,MSKint32t j,MSKvariabletypee * vartype) {
  return (*MSK_getvartype_ptr)(task,j,vartype);
} /* MSKgetvartype */
typedef MSKrescodee MSKAPI MSK_getvartypelist_t(MSKtask_t task,MSKint32t num,const MSKint32t * subj,MSKvariabletypee * vartype);
static MSK_getvartypelist_t * MSK_getvartypelist_ptr = (MSK_getvartypelist_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getvartypelist(MSKtask_t task,MSKint32t num,const MSKint32t * subj,MSKvariabletypee * vartype) {
  return (*MSK_getvartypelist_ptr)(task,num,subj,vartype);
} /* MSKgetvartypelist */
typedef MSKrescodee MSKAPI MSK_inputdata_t(MSKtask_t task,MSKint32t maxnumcon,MSKint32t maxnumvar,MSKint32t numcon,MSKint32t numvar,const MSKrealt * c,MSKrealt cfix,const MSKint32t * aptrb,const MSKint32t * aptre,const MSKint32t * asub,const MSKrealt * aval,const MSKboundkeye * bkc,const MSKrealt * blc,const MSKrealt * buc,const MSKboundkeye * bkx,const MSKrealt * blx,const MSKrealt * bux);
static MSK_inputdata_t * MSK_inputdata_ptr = (MSK_inputdata_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_inputdata(MSKtask_t task,MSKint32t maxnumcon,MSKint32t maxnumvar,MSKint32t numcon,MSKint32t numvar,const MSKrealt * c,MSKrealt cfix,const MSKint32t * aptrb,const MSKint32t * aptre,const MSKint32t * asub,const MSKrealt * aval,const MSKboundkeye * bkc,const MSKrealt * blc,const MSKrealt * buc,const MSKboundkeye * bkx,const MSKrealt * blx,const MSKrealt * bux) {
  return (*MSK_inputdata_ptr)(task,maxnumcon,maxnumvar,numcon,numvar,c,cfix,aptrb,aptre,asub,aval,bkc,blc,buc,bkx,blx,bux);
} /* MSKinputdata */
typedef MSKrescodee MSKAPI MSK_inputdata64_t(MSKtask_t task,MSKint32t maxnumcon,MSKint32t maxnumvar,MSKint32t numcon,MSKint32t numvar,const MSKrealt * c,MSKrealt cfix,const MSKint64t * aptrb,const MSKint64t * aptre,const MSKint32t * asub,const MSKrealt * aval,const MSKboundkeye * bkc,const MSKrealt * blc,const MSKrealt * buc,const MSKboundkeye * bkx,const MSKrealt * blx,const MSKrealt * bux);
static MSK_inputdata64_t * MSK_inputdata64_ptr = (MSK_inputdata64_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_inputdata64(MSKtask_t task,MSKint32t maxnumcon,MSKint32t maxnumvar,MSKint32t numcon,MSKint32t numvar,const MSKrealt * c,MSKrealt cfix,const MSKint64t * aptrb,const MSKint64t * aptre,const MSKint32t * asub,const MSKrealt * aval,const MSKboundkeye * bkc,const MSKrealt * blc,const MSKrealt * buc,const MSKboundkeye * bkx,const MSKrealt * blx,const MSKrealt * bux) {
  return (*MSK_inputdata64_ptr)(task,maxnumcon,maxnumvar,numcon,numvar,c,cfix,aptrb,aptre,asub,aval,bkc,blc,buc,bkx,blx,bux);
} /* MSKinputdata64 */
typedef MSKrescodee MSKAPI MSK_isdouparname_t(MSKtask_t task,const char * parname,MSKdparame * param);
static MSK_isdouparname_t * MSK_isdouparname_ptr = (MSK_isdouparname_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_isdouparname(MSKtask_t task,const char * parname,MSKdparame * param) {
  return (*MSK_isdouparname_ptr)(task,parname,param);
} /* MSKisdouparname */
typedef MSKrescodee MSKAPI MSK_isintparname_t(MSKtask_t task,const char * parname,MSKiparame * param);
static MSK_isintparname_t * MSK_isintparname_ptr = (MSK_isintparname_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_isintparname(MSKtask_t task,const char * parname,MSKiparame * param) {
  return (*MSK_isintparname_ptr)(task,parname,param);
} /* MSKisintparname */
typedef MSKrescodee MSKAPI MSK_isstrparname_t(MSKtask_t task,const char * parname,MSKsparame * param);
static MSK_isstrparname_t * MSK_isstrparname_ptr = (MSK_isstrparname_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_isstrparname(MSKtask_t task,const char * parname,MSKsparame * param) {
  return (*MSK_isstrparname_ptr)(task,parname,param);
} /* MSKisstrparname */
typedef MSKrescodee MSKAPI MSK_linkfiletotaskstream_t(MSKtask_t task,MSKstreamtypee whichstream,const char * filename,MSKint32t append);
static MSK_linkfiletotaskstream_t * MSK_linkfiletotaskstream_ptr = (MSK_linkfiletotaskstream_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_linkfiletotaskstream(MSKtask_t task,MSKstreamtypee whichstream,const char * filename,MSKint32t append) {
  return (*MSK_linkfiletotaskstream_ptr)(task,whichstream,filename,append);
} /* MSKlinkfiletotaskstream */
typedef MSKrescodee MSKAPI MSK_linkfunctotaskstream_t(MSKtask_t task,MSKstreamtypee whichstream,MSKuserhandle_t handle,MSKstreamfunc func);
static MSK_linkfunctotaskstream_t * MSK_linkfunctotaskstream_ptr = (MSK_linkfunctotaskstream_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_linkfunctotaskstream(MSKtask_t task,MSKstreamtypee whichstream,MSKuserhandle_t handle,MSKstreamfunc func) {
  return (*MSK_linkfunctotaskstream_ptr)(task,whichstream,handle,func);
} /* MSKlinkfunctotaskstream */
typedef MSKrescodee MSKAPI MSK_unlinkfuncfromtaskstream_t(MSKtask_t task,MSKstreamtypee whichstream);
static MSK_unlinkfuncfromtaskstream_t * MSK_unlinkfuncfromtaskstream_ptr = (MSK_unlinkfuncfromtaskstream_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_unlinkfuncfromtaskstream(MSKtask_t task,MSKstreamtypee whichstream) {
  return (*MSK_unlinkfuncfromtaskstream_ptr)(task,whichstream);
} /* MSKunlinkfuncfromtaskstream */
typedef MSKrescodee MSKAPI MSK_clonetask_t(MSKtask_t task,MSKtask_t * clonedtask);
static MSK_clonetask_t * MSK_clonetask_ptr = (MSK_clonetask_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_clonetask(MSKtask_t task,MSKtask_t * clonedtask) {
  return (*MSK_clonetask_ptr)(task,clonedtask);
} /* MSKclonetask */
typedef MSKrescodee MSKAPI MSK_primalrepair_t(MSKtask_t task,const MSKrealt * wlc,const MSKrealt * wuc,const MSKrealt * wlx,const MSKrealt * wux);
static MSK_primalrepair_t * MSK_primalrepair_ptr = (MSK_primalrepair_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_primalrepair(MSKtask_t task,const MSKrealt * wlc,const MSKrealt * wuc,const MSKrealt * wlx,const MSKrealt * wux) {
  return (*MSK_primalrepair_ptr)(task,wlc,wuc,wlx,wux);
} /* MSKprimalrepair */
typedef MSKrescodee MSKAPI MSK_infeasibilityreport_t(MSKtask_t task,MSKstreamtypee whichstream,MSKsoltypee whichsol);
static MSK_infeasibilityreport_t * MSK_infeasibilityreport_ptr = (MSK_infeasibilityreport_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_infeasibilityreport(MSKtask_t task,MSKstreamtypee whichstream,MSKsoltypee whichsol) {
  return (*MSK_infeasibilityreport_ptr)(task,whichstream,whichsol);
} /* MSKinfeasibilityreport */
typedef MSKrescodee MSKAPI MSK_toconic_t(MSKtask_t task);
static MSK_toconic_t * MSK_toconic_ptr = (MSK_toconic_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_toconic(MSKtask_t task) {
  return (*MSK_toconic_ptr)(task);
} /* MSKtoconic */
typedef MSKrescodee MSKAPI MSK_optimize_t(MSKtask_t task);
static MSK_optimize_t * MSK_optimize_ptr = (MSK_optimize_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_optimize(MSKtask_t task) {
  return (*MSK_optimize_ptr)(task);
} /* MSKoptimize */
typedef MSKrescodee MSKAPI MSK_optimizetrm_t(MSKtask_t task,MSKrescodee * trmcode);
static MSK_optimizetrm_t * MSK_optimizetrm_ptr = (MSK_optimizetrm_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_optimizetrm(MSKtask_t task,MSKrescodee * trmcode) {
  return (*MSK_optimizetrm_ptr)(task,trmcode);
} /* MSKoptimizetrm */
typedef MSKrescodee MSKAPI MSK_printparam_t(MSKtask_t task);
static MSK_printparam_t * MSK_printparam_ptr = (MSK_printparam_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_printparam(MSKtask_t task) {
  return (*MSK_printparam_ptr)(task);
} /* MSKprintparam */
typedef MSKrescodee MSKAPI MSK_probtypetostr_t(MSKtask_t task,MSKproblemtypee probtype,char * str);
static MSK_probtypetostr_t * MSK_probtypetostr_ptr = (MSK_probtypetostr_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_probtypetostr(MSKtask_t task,MSKproblemtypee probtype,char * str) {
  return (*MSK_probtypetostr_ptr)(task,probtype,str);
} /* MSKprobtypetostr */
typedef MSKrescodee MSKAPI MSK_prostatostr_t(MSKtask_t task,MSKprostae problemsta,char * str);
static MSK_prostatostr_t * MSK_prostatostr_ptr = (MSK_prostatostr_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_prostatostr(MSKtask_t task,MSKprostae problemsta,char * str) {
  return (*MSK_prostatostr_ptr)(task,problemsta,str);
} /* MSKprostatostr */
typedef MSKrescodee MSKAPI MSK_putresponsefunc_t(MSKtask_t task,MSKresponsefunc responsefunc,MSKuserhandle_t handle);
static MSK_putresponsefunc_t * MSK_putresponsefunc_ptr = (MSK_putresponsefunc_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putresponsefunc(MSKtask_t task,MSKresponsefunc responsefunc,MSKuserhandle_t handle) {
  return (*MSK_putresponsefunc_ptr)(task,responsefunc,handle);
} /* MSKputresponsefunc */
typedef MSKrescodee MSKAPI MSK_commitchanges_t(MSKtask_t task);
static MSK_commitchanges_t * MSK_commitchanges_ptr = (MSK_commitchanges_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_commitchanges(MSKtask_t task) {
  return (*MSK_commitchanges_ptr)(task);
} /* MSKcommitchanges */
typedef MSKrescodee MSKAPI MSK_getatruncatetol_t(MSKtask_t task,MSKrealt * tolzero);
static MSK_getatruncatetol_t * MSK_getatruncatetol_ptr = (MSK_getatruncatetol_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getatruncatetol(MSKtask_t task,MSKrealt * tolzero) {
  return (*MSK_getatruncatetol_ptr)(task,tolzero);
} /* MSKgetatruncatetol */
typedef MSKrescodee MSKAPI MSK_putatruncatetol_t(MSKtask_t task,MSKrealt tolzero);
static MSK_putatruncatetol_t * MSK_putatruncatetol_ptr = (MSK_putatruncatetol_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putatruncatetol(MSKtask_t task,MSKrealt tolzero) {
  return (*MSK_putatruncatetol_ptr)(task,tolzero);
} /* MSKputatruncatetol */
typedef MSKrescodee MSKAPI MSK_putaij_t(MSKtask_t task,MSKint32t i,MSKint32t j,MSKrealt aij);
static MSK_putaij_t * MSK_putaij_ptr = (MSK_putaij_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putaij(MSKtask_t task,MSKint32t i,MSKint32t j,MSKrealt aij) {
  return (*MSK_putaij_ptr)(task,i,j,aij);
} /* MSKputaij */
typedef MSKrescodee MSKAPI MSK_putaijlist_t(MSKtask_t task,MSKint32t num,const MSKint32t * subi,const MSKint32t * subj,const MSKrealt * valij);
static MSK_putaijlist_t * MSK_putaijlist_ptr = (MSK_putaijlist_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putaijlist(MSKtask_t task,MSKint32t num,const MSKint32t * subi,const MSKint32t * subj,const MSKrealt * valij) {
  return (*MSK_putaijlist_ptr)(task,num,subi,subj,valij);
} /* MSKputaijlist */
typedef MSKrescodee MSKAPI MSK_putaijlist64_t(MSKtask_t task,MSKint64t num,const MSKint32t * subi,const MSKint32t * subj,const MSKrealt * valij);
static MSK_putaijlist64_t * MSK_putaijlist64_ptr = (MSK_putaijlist64_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putaijlist64(MSKtask_t task,MSKint64t num,const MSKint32t * subi,const MSKint32t * subj,const MSKrealt * valij) {
  return (*MSK_putaijlist64_ptr)(task,num,subi,subj,valij);
} /* MSKputaijlist64 */
typedef MSKrescodee MSKAPI MSK_putacol_t(MSKtask_t task,MSKint32t j,MSKint32t nzj,const MSKint32t * subj,const MSKrealt * valj);
static MSK_putacol_t * MSK_putacol_ptr = (MSK_putacol_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putacol(MSKtask_t task,MSKint32t j,MSKint32t nzj,const MSKint32t * subj,const MSKrealt * valj) {
  return (*MSK_putacol_ptr)(task,j,nzj,subj,valj);
} /* MSKputacol */
typedef MSKrescodee MSKAPI MSK_putarow_t(MSKtask_t task,MSKint32t i,MSKint32t nzi,const MSKint32t * subi,const MSKrealt * vali);
static MSK_putarow_t * MSK_putarow_ptr = (MSK_putarow_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putarow(MSKtask_t task,MSKint32t i,MSKint32t nzi,const MSKint32t * subi,const MSKrealt * vali) {
  return (*MSK_putarow_ptr)(task,i,nzi,subi,vali);
} /* MSKputarow */
typedef MSKrescodee MSKAPI MSK_putarowslice_t(MSKtask_t task,MSKint32t first,MSKint32t last,const MSKint32t * ptrb,const MSKint32t * ptre,const MSKint32t * asub,const MSKrealt * aval);
static MSK_putarowslice_t * MSK_putarowslice_ptr = (MSK_putarowslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putarowslice(MSKtask_t task,MSKint32t first,MSKint32t last,const MSKint32t * ptrb,const MSKint32t * ptre,const MSKint32t * asub,const MSKrealt * aval) {
  return (*MSK_putarowslice_ptr)(task,first,last,ptrb,ptre,asub,aval);
} /* MSKputarowslice */
typedef MSKrescodee MSKAPI MSK_putarowslice64_t(MSKtask_t task,MSKint32t first,MSKint32t last,const MSKint64t * ptrb,const MSKint64t * ptre,const MSKint32t * asub,const MSKrealt * aval);
static MSK_putarowslice64_t * MSK_putarowslice64_ptr = (MSK_putarowslice64_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putarowslice64(MSKtask_t task,MSKint32t first,MSKint32t last,const MSKint64t * ptrb,const MSKint64t * ptre,const MSKint32t * asub,const MSKrealt * aval) {
  return (*MSK_putarowslice64_ptr)(task,first,last,ptrb,ptre,asub,aval);
} /* MSKputarowslice64 */
typedef MSKrescodee MSKAPI MSK_putarowlist_t(MSKtask_t task,MSKint32t num,const MSKint32t * sub,const MSKint32t * ptrb,const MSKint32t * ptre,const MSKint32t * asub,const MSKrealt * aval);
static MSK_putarowlist_t * MSK_putarowlist_ptr = (MSK_putarowlist_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putarowlist(MSKtask_t task,MSKint32t num,const MSKint32t * sub,const MSKint32t * ptrb,const MSKint32t * ptre,const MSKint32t * asub,const MSKrealt * aval) {
  return (*MSK_putarowlist_ptr)(task,num,sub,ptrb,ptre,asub,aval);
} /* MSKputarowlist */
typedef MSKrescodee MSKAPI MSK_putarowlist64_t(MSKtask_t task,MSKint32t num,const MSKint32t * sub,const MSKint64t * ptrb,const MSKint64t * ptre,const MSKint32t * asub,const MSKrealt * aval);
static MSK_putarowlist64_t * MSK_putarowlist64_ptr = (MSK_putarowlist64_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putarowlist64(MSKtask_t task,MSKint32t num,const MSKint32t * sub,const MSKint64t * ptrb,const MSKint64t * ptre,const MSKint32t * asub,const MSKrealt * aval) {
  return (*MSK_putarowlist64_ptr)(task,num,sub,ptrb,ptre,asub,aval);
} /* MSKputarowlist64 */
typedef MSKrescodee MSKAPI MSK_putacolslice_t(MSKtask_t task,MSKint32t first,MSKint32t last,const MSKint32t * ptrb,const MSKint32t * ptre,const MSKint32t * asub,const MSKrealt * aval);
static MSK_putacolslice_t * MSK_putacolslice_ptr = (MSK_putacolslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putacolslice(MSKtask_t task,MSKint32t first,MSKint32t last,const MSKint32t * ptrb,const MSKint32t * ptre,const MSKint32t * asub,const MSKrealt * aval) {
  return (*MSK_putacolslice_ptr)(task,first,last,ptrb,ptre,asub,aval);
} /* MSKputacolslice */
typedef MSKrescodee MSKAPI MSK_putacolslice64_t(MSKtask_t task,MSKint32t first,MSKint32t last,const MSKint64t * ptrb,const MSKint64t * ptre,const MSKint32t * asub,const MSKrealt * aval);
static MSK_putacolslice64_t * MSK_putacolslice64_ptr = (MSK_putacolslice64_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putacolslice64(MSKtask_t task,MSKint32t first,MSKint32t last,const MSKint64t * ptrb,const MSKint64t * ptre,const MSKint32t * asub,const MSKrealt * aval) {
  return (*MSK_putacolslice64_ptr)(task,first,last,ptrb,ptre,asub,aval);
} /* MSKputacolslice64 */
typedef MSKrescodee MSKAPI MSK_putacollist_t(MSKtask_t task,MSKint32t num,const MSKint32t * sub,const MSKint32t * ptrb,const MSKint32t * ptre,const MSKint32t * asub,const MSKrealt * aval);
static MSK_putacollist_t * MSK_putacollist_ptr = (MSK_putacollist_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putacollist(MSKtask_t task,MSKint32t num,const MSKint32t * sub,const MSKint32t * ptrb,const MSKint32t * ptre,const MSKint32t * asub,const MSKrealt * aval) {
  return (*MSK_putacollist_ptr)(task,num,sub,ptrb,ptre,asub,aval);
} /* MSKputacollist */
typedef MSKrescodee MSKAPI MSK_putacollist64_t(MSKtask_t task,MSKint32t num,const MSKint32t * sub,const MSKint64t * ptrb,const MSKint64t * ptre,const MSKint32t * asub,const MSKrealt * aval);
static MSK_putacollist64_t * MSK_putacollist64_ptr = (MSK_putacollist64_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putacollist64(MSKtask_t task,MSKint32t num,const MSKint32t * sub,const MSKint64t * ptrb,const MSKint64t * ptre,const MSKint32t * asub,const MSKrealt * aval) {
  return (*MSK_putacollist64_ptr)(task,num,sub,ptrb,ptre,asub,aval);
} /* MSKputacollist64 */
typedef MSKrescodee MSKAPI MSK_putbaraij_t(MSKtask_t task,MSKint32t i,MSKint32t j,MSKint64t num,const MSKint64t * sub,const MSKrealt * weights);
static MSK_putbaraij_t * MSK_putbaraij_ptr = (MSK_putbaraij_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putbaraij(MSKtask_t task,MSKint32t i,MSKint32t j,MSKint64t num,const MSKint64t * sub,const MSKrealt * weights) {
  return (*MSK_putbaraij_ptr)(task,i,j,num,sub,weights);
} /* MSKputbaraij */
typedef MSKrescodee MSKAPI MSK_putbaraijlist_t(MSKtask_t task,MSKint32t num,const MSKint32t * subi,const MSKint32t * subj,const MSKint64t * alphaptrb,const MSKint64t * alphaptre,const MSKint64t * matidx,const MSKrealt * weights);
static MSK_putbaraijlist_t * MSK_putbaraijlist_ptr = (MSK_putbaraijlist_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putbaraijlist(MSKtask_t task,MSKint32t num,const MSKint32t * subi,const MSKint32t * subj,const MSKint64t * alphaptrb,const MSKint64t * alphaptre,const MSKint64t * matidx,const MSKrealt * weights) {
  return (*MSK_putbaraijlist_ptr)(task,num,subi,subj,alphaptrb,alphaptre,matidx,weights);
} /* MSKputbaraijlist */
typedef MSKrescodee MSKAPI MSK_putbararowlist_t(MSKtask_t task,MSKint32t num,const MSKint32t * subi,const MSKint64t * ptrb,const MSKint64t * ptre,const MSKint32t * subj,const MSKint64t * nummat,const MSKint64t * matidx,const MSKrealt * weights);
static MSK_putbararowlist_t * MSK_putbararowlist_ptr = (MSK_putbararowlist_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putbararowlist(MSKtask_t task,MSKint32t num,const MSKint32t * subi,const MSKint64t * ptrb,const MSKint64t * ptre,const MSKint32t * subj,const MSKint64t * nummat,const MSKint64t * matidx,const MSKrealt * weights) {
  return (*MSK_putbararowlist_ptr)(task,num,subi,ptrb,ptre,subj,nummat,matidx,weights);
} /* MSKputbararowlist */
typedef MSKrescodee MSKAPI MSK_getnumbarcnz_t(MSKtask_t task,MSKint64t * nz);
static MSK_getnumbarcnz_t * MSK_getnumbarcnz_ptr = (MSK_getnumbarcnz_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnumbarcnz(MSKtask_t task,MSKint64t * nz) {
  return (*MSK_getnumbarcnz_ptr)(task,nz);
} /* MSKgetnumbarcnz */
typedef MSKrescodee MSKAPI MSK_getnumbaranz_t(MSKtask_t task,MSKint64t * nz);
static MSK_getnumbaranz_t * MSK_getnumbaranz_ptr = (MSK_getnumbaranz_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnumbaranz(MSKtask_t task,MSKint64t * nz) {
  return (*MSK_getnumbaranz_ptr)(task,nz);
} /* MSKgetnumbaranz */
typedef MSKrescodee MSKAPI MSK_getbarcsparsity_t(MSKtask_t task,MSKint64t maxnumnz,MSKint64t * numnz,MSKint64t * idxj);
static MSK_getbarcsparsity_t * MSK_getbarcsparsity_ptr = (MSK_getbarcsparsity_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getbarcsparsity(MSKtask_t task,MSKint64t maxnumnz,MSKint64t * numnz,MSKint64t * idxj) {
  return (*MSK_getbarcsparsity_ptr)(task,maxnumnz,numnz,idxj);
} /* MSKgetbarcsparsity */
typedef MSKrescodee MSKAPI MSK_getbarasparsity_t(MSKtask_t task,MSKint64t maxnumnz,MSKint64t * numnz,MSKint64t * idxij);
static MSK_getbarasparsity_t * MSK_getbarasparsity_ptr = (MSK_getbarasparsity_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getbarasparsity(MSKtask_t task,MSKint64t maxnumnz,MSKint64t * numnz,MSKint64t * idxij) {
  return (*MSK_getbarasparsity_ptr)(task,maxnumnz,numnz,idxij);
} /* MSKgetbarasparsity */
typedef MSKrescodee MSKAPI MSK_getbarcidxinfo_t(MSKtask_t task,MSKint64t idx,MSKint64t * num);
static MSK_getbarcidxinfo_t * MSK_getbarcidxinfo_ptr = (MSK_getbarcidxinfo_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getbarcidxinfo(MSKtask_t task,MSKint64t idx,MSKint64t * num) {
  return (*MSK_getbarcidxinfo_ptr)(task,idx,num);
} /* MSKgetbarcidxinfo */
typedef MSKrescodee MSKAPI MSK_getbarcidxj_t(MSKtask_t task,MSKint64t idx,MSKint32t * j);
static MSK_getbarcidxj_t * MSK_getbarcidxj_ptr = (MSK_getbarcidxj_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getbarcidxj(MSKtask_t task,MSKint64t idx,MSKint32t * j) {
  return (*MSK_getbarcidxj_ptr)(task,idx,j);
} /* MSKgetbarcidxj */
typedef MSKrescodee MSKAPI MSK_getbarcidx_t(MSKtask_t task,MSKint64t idx,MSKint64t maxnum,MSKint32t * j,MSKint64t * num,MSKint64t * sub,MSKrealt * weights);
static MSK_getbarcidx_t * MSK_getbarcidx_ptr = (MSK_getbarcidx_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getbarcidx(MSKtask_t task,MSKint64t idx,MSKint64t maxnum,MSKint32t * j,MSKint64t * num,MSKint64t * sub,MSKrealt * weights) {
  return (*MSK_getbarcidx_ptr)(task,idx,maxnum,j,num,sub,weights);
} /* MSKgetbarcidx */
typedef MSKrescodee MSKAPI MSK_getbaraidxinfo_t(MSKtask_t task,MSKint64t idx,MSKint64t * num);
static MSK_getbaraidxinfo_t * MSK_getbaraidxinfo_ptr = (MSK_getbaraidxinfo_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getbaraidxinfo(MSKtask_t task,MSKint64t idx,MSKint64t * num) {
  return (*MSK_getbaraidxinfo_ptr)(task,idx,num);
} /* MSKgetbaraidxinfo */
typedef MSKrescodee MSKAPI MSK_getbaraidxij_t(MSKtask_t task,MSKint64t idx,MSKint32t * i,MSKint32t * j);
static MSK_getbaraidxij_t * MSK_getbaraidxij_ptr = (MSK_getbaraidxij_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getbaraidxij(MSKtask_t task,MSKint64t idx,MSKint32t * i,MSKint32t * j) {
  return (*MSK_getbaraidxij_ptr)(task,idx,i,j);
} /* MSKgetbaraidxij */
typedef MSKrescodee MSKAPI MSK_getbaraidx_t(MSKtask_t task,MSKint64t idx,MSKint64t maxnum,MSKint32t * i,MSKint32t * j,MSKint64t * num,MSKint64t * sub,MSKrealt * weights);
static MSK_getbaraidx_t * MSK_getbaraidx_ptr = (MSK_getbaraidx_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getbaraidx(MSKtask_t task,MSKint64t idx,MSKint64t maxnum,MSKint32t * i,MSKint32t * j,MSKint64t * num,MSKint64t * sub,MSKrealt * weights) {
  return (*MSK_getbaraidx_ptr)(task,idx,maxnum,i,j,num,sub,weights);
} /* MSKgetbaraidx */
typedef MSKrescodee MSKAPI MSK_getnumbarcblocktriplets_t(MSKtask_t task,MSKint64t * num);
static MSK_getnumbarcblocktriplets_t * MSK_getnumbarcblocktriplets_ptr = (MSK_getnumbarcblocktriplets_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnumbarcblocktriplets(MSKtask_t task,MSKint64t * num) {
  return (*MSK_getnumbarcblocktriplets_ptr)(task,num);
} /* MSKgetnumbarcblocktriplets */
typedef MSKrescodee MSKAPI MSK_putbarcblocktriplet_t(MSKtask_t task,MSKint64t num,const MSKint32t * subj,const MSKint32t * subk,const MSKint32t * subl,const MSKrealt * valjkl);
static MSK_putbarcblocktriplet_t * MSK_putbarcblocktriplet_ptr = (MSK_putbarcblocktriplet_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putbarcblocktriplet(MSKtask_t task,MSKint64t num,const MSKint32t * subj,const MSKint32t * subk,const MSKint32t * subl,const MSKrealt * valjkl) {
  return (*MSK_putbarcblocktriplet_ptr)(task,num,subj,subk,subl,valjkl);
} /* MSKputbarcblocktriplet */
typedef MSKrescodee MSKAPI MSK_getbarcblocktriplet_t(MSKtask_t task,MSKint64t maxnum,MSKint64t * num,MSKint32t * subj,MSKint32t * subk,MSKint32t * subl,MSKrealt * valjkl);
static MSK_getbarcblocktriplet_t * MSK_getbarcblocktriplet_ptr = (MSK_getbarcblocktriplet_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getbarcblocktriplet(MSKtask_t task,MSKint64t maxnum,MSKint64t * num,MSKint32t * subj,MSKint32t * subk,MSKint32t * subl,MSKrealt * valjkl) {
  return (*MSK_getbarcblocktriplet_ptr)(task,maxnum,num,subj,subk,subl,valjkl);
} /* MSKgetbarcblocktriplet */
typedef MSKrescodee MSKAPI MSK_putbarablocktriplet_t(MSKtask_t task,MSKint64t num,const MSKint32t * subi,const MSKint32t * subj,const MSKint32t * subk,const MSKint32t * subl,const MSKrealt * valijkl);
static MSK_putbarablocktriplet_t * MSK_putbarablocktriplet_ptr = (MSK_putbarablocktriplet_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putbarablocktriplet(MSKtask_t task,MSKint64t num,const MSKint32t * subi,const MSKint32t * subj,const MSKint32t * subk,const MSKint32t * subl,const MSKrealt * valijkl) {
  return (*MSK_putbarablocktriplet_ptr)(task,num,subi,subj,subk,subl,valijkl);
} /* MSKputbarablocktriplet */
typedef MSKrescodee MSKAPI MSK_getnumbarablocktriplets_t(MSKtask_t task,MSKint64t * num);
static MSK_getnumbarablocktriplets_t * MSK_getnumbarablocktriplets_ptr = (MSK_getnumbarablocktriplets_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnumbarablocktriplets(MSKtask_t task,MSKint64t * num) {
  return (*MSK_getnumbarablocktriplets_ptr)(task,num);
} /* MSKgetnumbarablocktriplets */
typedef MSKrescodee MSKAPI MSK_getbarablocktriplet_t(MSKtask_t task,MSKint64t maxnum,MSKint64t * num,MSKint32t * subi,MSKint32t * subj,MSKint32t * subk,MSKint32t * subl,MSKrealt * valijkl);
static MSK_getbarablocktriplet_t * MSK_getbarablocktriplet_ptr = (MSK_getbarablocktriplet_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getbarablocktriplet(MSKtask_t task,MSKint64t maxnum,MSKint64t * num,MSKint32t * subi,MSKint32t * subj,MSKint32t * subk,MSKint32t * subl,MSKrealt * valijkl) {
  return (*MSK_getbarablocktriplet_ptr)(task,maxnum,num,subi,subj,subk,subl,valijkl);
} /* MSKgetbarablocktriplet */
typedef MSKrescodee MSKAPI MSK_putmaxnumafe_t(MSKtask_t task,MSKint64t maxnumafe);
static MSK_putmaxnumafe_t * MSK_putmaxnumafe_ptr = (MSK_putmaxnumafe_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putmaxnumafe(MSKtask_t task,MSKint64t maxnumafe) {
  return (*MSK_putmaxnumafe_ptr)(task,maxnumafe);
} /* MSKputmaxnumafe */
typedef MSKrescodee MSKAPI MSK_getnumafe_t(MSKtask_t task,MSKint64t * numafe);
static MSK_getnumafe_t * MSK_getnumafe_ptr = (MSK_getnumafe_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnumafe(MSKtask_t task,MSKint64t * numafe) {
  return (*MSK_getnumafe_ptr)(task,numafe);
} /* MSKgetnumafe */
typedef MSKrescodee MSKAPI MSK_appendafes_t(MSKtask_t task,MSKint64t num);
static MSK_appendafes_t * MSK_appendafes_ptr = (MSK_appendafes_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appendafes(MSKtask_t task,MSKint64t num) {
  return (*MSK_appendafes_ptr)(task,num);
} /* MSKappendafes */
typedef MSKrescodee MSKAPI MSK_putafefentry_t(MSKtask_t task,MSKint64t afeidx,MSKint32t varidx,MSKrealt value);
static MSK_putafefentry_t * MSK_putafefentry_ptr = (MSK_putafefentry_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putafefentry(MSKtask_t task,MSKint64t afeidx,MSKint32t varidx,MSKrealt value) {
  return (*MSK_putafefentry_ptr)(task,afeidx,varidx,value);
} /* MSKputafefentry */
typedef MSKrescodee MSKAPI MSK_putafefentrylist_t(MSKtask_t task,MSKint64t numentr,const MSKint64t * afeidx,const MSKint32t * varidx,const MSKrealt * val);
static MSK_putafefentrylist_t * MSK_putafefentrylist_ptr = (MSK_putafefentrylist_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putafefentrylist(MSKtask_t task,MSKint64t numentr,const MSKint64t * afeidx,const MSKint32t * varidx,const MSKrealt * val) {
  return (*MSK_putafefentrylist_ptr)(task,numentr,afeidx,varidx,val);
} /* MSKputafefentrylist */
typedef MSKrescodee MSKAPI MSK_emptyafefrow_t(MSKtask_t task,MSKint64t afeidx);
static MSK_emptyafefrow_t * MSK_emptyafefrow_ptr = (MSK_emptyafefrow_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_emptyafefrow(MSKtask_t task,MSKint64t afeidx) {
  return (*MSK_emptyafefrow_ptr)(task,afeidx);
} /* MSKemptyafefrow */
typedef MSKrescodee MSKAPI MSK_emptyafefcol_t(MSKtask_t task,MSKint32t varidx);
static MSK_emptyafefcol_t * MSK_emptyafefcol_ptr = (MSK_emptyafefcol_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_emptyafefcol(MSKtask_t task,MSKint32t varidx) {
  return (*MSK_emptyafefcol_ptr)(task,varidx);
} /* MSKemptyafefcol */
typedef MSKrescodee MSKAPI MSK_emptyafefrowlist_t(MSKtask_t task,MSKint64t numafeidx,const MSKint64t * afeidx);
static MSK_emptyafefrowlist_t * MSK_emptyafefrowlist_ptr = (MSK_emptyafefrowlist_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_emptyafefrowlist(MSKtask_t task,MSKint64t numafeidx,const MSKint64t * afeidx) {
  return (*MSK_emptyafefrowlist_ptr)(task,numafeidx,afeidx);
} /* MSKemptyafefrowlist */
typedef MSKrescodee MSKAPI MSK_emptyafefcollist_t(MSKtask_t task,MSKint64t numvaridx,const MSKint32t * varidx);
static MSK_emptyafefcollist_t * MSK_emptyafefcollist_ptr = (MSK_emptyafefcollist_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_emptyafefcollist(MSKtask_t task,MSKint64t numvaridx,const MSKint32t * varidx) {
  return (*MSK_emptyafefcollist_ptr)(task,numvaridx,varidx);
} /* MSKemptyafefcollist */
typedef MSKrescodee MSKAPI MSK_putafefrow_t(MSKtask_t task,MSKint64t afeidx,MSKint32t numnz,const MSKint32t * varidx,const MSKrealt * val);
static MSK_putafefrow_t * MSK_putafefrow_ptr = (MSK_putafefrow_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putafefrow(MSKtask_t task,MSKint64t afeidx,MSKint32t numnz,const MSKint32t * varidx,const MSKrealt * val) {
  return (*MSK_putafefrow_ptr)(task,afeidx,numnz,varidx,val);
} /* MSKputafefrow */
typedef MSKrescodee MSKAPI MSK_putafefrowlist_t(MSKtask_t task,MSKint64t numafeidx,const MSKint64t * afeidx,const MSKint32t * numnzrow,const MSKint64t * ptrrow,MSKint64t lenidxval,const MSKint32t * varidx,const MSKrealt * val);
static MSK_putafefrowlist_t * MSK_putafefrowlist_ptr = (MSK_putafefrowlist_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putafefrowlist(MSKtask_t task,MSKint64t numafeidx,const MSKint64t * afeidx,const MSKint32t * numnzrow,const MSKint64t * ptrrow,MSKint64t lenidxval,const MSKint32t * varidx,const MSKrealt * val) {
  return (*MSK_putafefrowlist_ptr)(task,numafeidx,afeidx,numnzrow,ptrrow,lenidxval,varidx,val);
} /* MSKputafefrowlist */
typedef MSKrescodee MSKAPI MSK_putafefcol_t(MSKtask_t task,MSKint32t varidx,MSKint64t numnz,const MSKint64t * afeidx,const MSKrealt * val);
static MSK_putafefcol_t * MSK_putafefcol_ptr = (MSK_putafefcol_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putafefcol(MSKtask_t task,MSKint32t varidx,MSKint64t numnz,const MSKint64t * afeidx,const MSKrealt * val) {
  return (*MSK_putafefcol_ptr)(task,varidx,numnz,afeidx,val);
} /* MSKputafefcol */
typedef MSKrescodee MSKAPI MSK_getafefrownumnz_t(MSKtask_t task,MSKint64t afeidx,MSKint32t * numnz);
static MSK_getafefrownumnz_t * MSK_getafefrownumnz_ptr = (MSK_getafefrownumnz_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getafefrownumnz(MSKtask_t task,MSKint64t afeidx,MSKint32t * numnz) {
  return (*MSK_getafefrownumnz_ptr)(task,afeidx,numnz);
} /* MSKgetafefrownumnz */
typedef MSKrescodee MSKAPI MSK_getafefnumnz_t(MSKtask_t task,MSKint64t * numnz);
static MSK_getafefnumnz_t * MSK_getafefnumnz_ptr = (MSK_getafefnumnz_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getafefnumnz(MSKtask_t task,MSKint64t * numnz) {
  return (*MSK_getafefnumnz_ptr)(task,numnz);
} /* MSKgetafefnumnz */
typedef MSKrescodee MSKAPI MSK_getafefrow_t(MSKtask_t task,MSKint64t afeidx,MSKint32t * numnz,MSKint32t * varidx,MSKrealt * val);
static MSK_getafefrow_t * MSK_getafefrow_ptr = (MSK_getafefrow_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getafefrow(MSKtask_t task,MSKint64t afeidx,MSKint32t * numnz,MSKint32t * varidx,MSKrealt * val) {
  return (*MSK_getafefrow_ptr)(task,afeidx,numnz,varidx,val);
} /* MSKgetafefrow */
typedef MSKrescodee MSKAPI MSK_getafeftrip_t(MSKtask_t task,MSKint64t * afeidx,MSKint32t * varidx,MSKrealt * val);
static MSK_getafeftrip_t * MSK_getafeftrip_ptr = (MSK_getafeftrip_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getafeftrip(MSKtask_t task,MSKint64t * afeidx,MSKint32t * varidx,MSKrealt * val) {
  return (*MSK_getafeftrip_ptr)(task,afeidx,varidx,val);
} /* MSKgetafeftrip */
typedef MSKrescodee MSKAPI MSK_putafebarfentry_t(MSKtask_t task,MSKint64t afeidx,MSKint32t barvaridx,MSKint64t numterm,const MSKint64t * termidx,const MSKrealt * termweight);
static MSK_putafebarfentry_t * MSK_putafebarfentry_ptr = (MSK_putafebarfentry_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putafebarfentry(MSKtask_t task,MSKint64t afeidx,MSKint32t barvaridx,MSKint64t numterm,const MSKint64t * termidx,const MSKrealt * termweight) {
  return (*MSK_putafebarfentry_ptr)(task,afeidx,barvaridx,numterm,termidx,termweight);
} /* MSKputafebarfentry */
typedef MSKrescodee MSKAPI MSK_putafebarfentrylist_t(MSKtask_t task,MSKint64t numafeidx,const MSKint64t * afeidx,const MSKint32t * barvaridx,const MSKint64t * numterm,const MSKint64t * ptrterm,MSKint64t lenterm,const MSKint64t * termidx,const MSKrealt * termweight);
static MSK_putafebarfentrylist_t * MSK_putafebarfentrylist_ptr = (MSK_putafebarfentrylist_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putafebarfentrylist(MSKtask_t task,MSKint64t numafeidx,const MSKint64t * afeidx,const MSKint32t * barvaridx,const MSKint64t * numterm,const MSKint64t * ptrterm,MSKint64t lenterm,const MSKint64t * termidx,const MSKrealt * termweight) {
  return (*MSK_putafebarfentrylist_ptr)(task,numafeidx,afeidx,barvaridx,numterm,ptrterm,lenterm,termidx,termweight);
} /* MSKputafebarfentrylist */
typedef MSKrescodee MSKAPI MSK_putafebarfrow_t(MSKtask_t task,MSKint64t afeidx,MSKint32t numentr,const MSKint32t * barvaridx,const MSKint64t * numterm,const MSKint64t * ptrterm,MSKint64t lenterm,const MSKint64t * termidx,const MSKrealt * termweight);
static MSK_putafebarfrow_t * MSK_putafebarfrow_ptr = (MSK_putafebarfrow_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putafebarfrow(MSKtask_t task,MSKint64t afeidx,MSKint32t numentr,const MSKint32t * barvaridx,const MSKint64t * numterm,const MSKint64t * ptrterm,MSKint64t lenterm,const MSKint64t * termidx,const MSKrealt * termweight) {
  return (*MSK_putafebarfrow_ptr)(task,afeidx,numentr,barvaridx,numterm,ptrterm,lenterm,termidx,termweight);
} /* MSKputafebarfrow */
typedef MSKrescodee MSKAPI MSK_emptyafebarfrow_t(MSKtask_t task,MSKint64t afeidx);
static MSK_emptyafebarfrow_t * MSK_emptyafebarfrow_ptr = (MSK_emptyafebarfrow_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_emptyafebarfrow(MSKtask_t task,MSKint64t afeidx) {
  return (*MSK_emptyafebarfrow_ptr)(task,afeidx);
} /* MSKemptyafebarfrow */
typedef MSKrescodee MSKAPI MSK_emptyafebarfrowlist_t(MSKtask_t task,MSKint64t numafeidx,const MSKint64t * afeidxlist);
static MSK_emptyafebarfrowlist_t * MSK_emptyafebarfrowlist_ptr = (MSK_emptyafebarfrowlist_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_emptyafebarfrowlist(MSKtask_t task,MSKint64t numafeidx,const MSKint64t * afeidxlist) {
  return (*MSK_emptyafebarfrowlist_ptr)(task,numafeidx,afeidxlist);
} /* MSKemptyafebarfrowlist */
typedef MSKrescodee MSKAPI MSK_putafebarfblocktriplet_t(MSKtask_t task,MSKint64t numtrip,const MSKint64t * afeidx,const MSKint32t * barvaridx,const MSKint32t * subk,const MSKint32t * subl,const MSKrealt * valkl);
static MSK_putafebarfblocktriplet_t * MSK_putafebarfblocktriplet_ptr = (MSK_putafebarfblocktriplet_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putafebarfblocktriplet(MSKtask_t task,MSKint64t numtrip,const MSKint64t * afeidx,const MSKint32t * barvaridx,const MSKint32t * subk,const MSKint32t * subl,const MSKrealt * valkl) {
  return (*MSK_putafebarfblocktriplet_ptr)(task,numtrip,afeidx,barvaridx,subk,subl,valkl);
} /* MSKputafebarfblocktriplet */
typedef MSKrescodee MSKAPI MSK_getafebarfnumblocktriplets_t(MSKtask_t task,MSKint64t * numtrip);
static MSK_getafebarfnumblocktriplets_t * MSK_getafebarfnumblocktriplets_ptr = (MSK_getafebarfnumblocktriplets_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getafebarfnumblocktriplets(MSKtask_t task,MSKint64t * numtrip) {
  return (*MSK_getafebarfnumblocktriplets_ptr)(task,numtrip);
} /* MSKgetafebarfnumblocktriplets */
typedef MSKrescodee MSKAPI MSK_getafebarfblocktriplet_t(MSKtask_t task,MSKint64t maxnumtrip,MSKint64t * numtrip,MSKint64t * afeidx,MSKint32t * barvaridx,MSKint32t * subk,MSKint32t * subl,MSKrealt * valkl);
static MSK_getafebarfblocktriplet_t * MSK_getafebarfblocktriplet_ptr = (MSK_getafebarfblocktriplet_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getafebarfblocktriplet(MSKtask_t task,MSKint64t maxnumtrip,MSKint64t * numtrip,MSKint64t * afeidx,MSKint32t * barvaridx,MSKint32t * subk,MSKint32t * subl,MSKrealt * valkl) {
  return (*MSK_getafebarfblocktriplet_ptr)(task,maxnumtrip,numtrip,afeidx,barvaridx,subk,subl,valkl);
} /* MSKgetafebarfblocktriplet */
typedef MSKrescodee MSKAPI MSK_getafebarfnumrowentries_t(MSKtask_t task,MSKint64t afeidx,MSKint32t * numentr);
static MSK_getafebarfnumrowentries_t * MSK_getafebarfnumrowentries_ptr = (MSK_getafebarfnumrowentries_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getafebarfnumrowentries(MSKtask_t task,MSKint64t afeidx,MSKint32t * numentr) {
  return (*MSK_getafebarfnumrowentries_ptr)(task,afeidx,numentr);
} /* MSKgetafebarfnumrowentries */
typedef MSKrescodee MSKAPI MSK_getafebarfrowinfo_t(MSKtask_t task,MSKint64t afeidx,MSKint32t * numentr,MSKint64t * numterm);
static MSK_getafebarfrowinfo_t * MSK_getafebarfrowinfo_ptr = (MSK_getafebarfrowinfo_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getafebarfrowinfo(MSKtask_t task,MSKint64t afeidx,MSKint32t * numentr,MSKint64t * numterm) {
  return (*MSK_getafebarfrowinfo_ptr)(task,afeidx,numentr,numterm);
} /* MSKgetafebarfrowinfo */
typedef MSKrescodee MSKAPI MSK_getafebarfrow_t(MSKtask_t task,MSKint64t afeidx,MSKint32t * barvaridx,MSKint64t * ptrterm,MSKint64t * numterm,MSKint64t * termidx,MSKrealt * termweight);
static MSK_getafebarfrow_t * MSK_getafebarfrow_ptr = (MSK_getafebarfrow_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getafebarfrow(MSKtask_t task,MSKint64t afeidx,MSKint32t * barvaridx,MSKint64t * ptrterm,MSKint64t * numterm,MSKint64t * termidx,MSKrealt * termweight) {
  return (*MSK_getafebarfrow_ptr)(task,afeidx,barvaridx,ptrterm,numterm,termidx,termweight);
} /* MSKgetafebarfrow */
typedef MSKrescodee MSKAPI MSK_putafeg_t(MSKtask_t task,MSKint64t afeidx,MSKrealt g);
static MSK_putafeg_t * MSK_putafeg_ptr = (MSK_putafeg_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putafeg(MSKtask_t task,MSKint64t afeidx,MSKrealt g) {
  return (*MSK_putafeg_ptr)(task,afeidx,g);
} /* MSKputafeg */
typedef MSKrescodee MSKAPI MSK_putafeglist_t(MSKtask_t task,MSKint64t numafeidx,const MSKint64t * afeidx,const MSKrealt * g);
static MSK_putafeglist_t * MSK_putafeglist_ptr = (MSK_putafeglist_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putafeglist(MSKtask_t task,MSKint64t numafeidx,const MSKint64t * afeidx,const MSKrealt * g) {
  return (*MSK_putafeglist_ptr)(task,numafeidx,afeidx,g);
} /* MSKputafeglist */
typedef MSKrescodee MSKAPI MSK_getafeg_t(MSKtask_t task,MSKint64t afeidx,MSKrealt * g);
static MSK_getafeg_t * MSK_getafeg_ptr = (MSK_getafeg_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getafeg(MSKtask_t task,MSKint64t afeidx,MSKrealt * g) {
  return (*MSK_getafeg_ptr)(task,afeidx,g);
} /* MSKgetafeg */
typedef MSKrescodee MSKAPI MSK_getafegslice_t(MSKtask_t task,MSKint64t first,MSKint64t last,MSKrealt * g);
static MSK_getafegslice_t * MSK_getafegslice_ptr = (MSK_getafegslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getafegslice(MSKtask_t task,MSKint64t first,MSKint64t last,MSKrealt * g) {
  return (*MSK_getafegslice_ptr)(task,first,last,g);
} /* MSKgetafegslice */
typedef MSKrescodee MSKAPI MSK_putafegslice_t(MSKtask_t task,MSKint64t first,MSKint64t last,const MSKrealt * slice);
static MSK_putafegslice_t * MSK_putafegslice_ptr = (MSK_putafegslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putafegslice(MSKtask_t task,MSKint64t first,MSKint64t last,const MSKrealt * slice) {
  return (*MSK_putafegslice_ptr)(task,first,last,slice);
} /* MSKputafegslice */
typedef MSKrescodee MSKAPI MSK_putmaxnumdjc_t(MSKtask_t task,MSKint64t maxnumdjc);
static MSK_putmaxnumdjc_t * MSK_putmaxnumdjc_ptr = (MSK_putmaxnumdjc_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putmaxnumdjc(MSKtask_t task,MSKint64t maxnumdjc) {
  return (*MSK_putmaxnumdjc_ptr)(task,maxnumdjc);
} /* MSKputmaxnumdjc */
typedef MSKrescodee MSKAPI MSK_getnumdjc_t(MSKtask_t task,MSKint64t * num);
static MSK_getnumdjc_t * MSK_getnumdjc_ptr = (MSK_getnumdjc_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnumdjc(MSKtask_t task,MSKint64t * num) {
  return (*MSK_getnumdjc_ptr)(task,num);
} /* MSKgetnumdjc */
typedef MSKrescodee MSKAPI MSK_getdjcnumdomain_t(MSKtask_t task,MSKint64t djcidx,MSKint64t * numdomain);
static MSK_getdjcnumdomain_t * MSK_getdjcnumdomain_ptr = (MSK_getdjcnumdomain_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdjcnumdomain(MSKtask_t task,MSKint64t djcidx,MSKint64t * numdomain) {
  return (*MSK_getdjcnumdomain_ptr)(task,djcidx,numdomain);
} /* MSKgetdjcnumdomain */
typedef MSKrescodee MSKAPI MSK_getdjcnumdomaintot_t(MSKtask_t task,MSKint64t * numdomaintot);
static MSK_getdjcnumdomaintot_t * MSK_getdjcnumdomaintot_ptr = (MSK_getdjcnumdomaintot_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdjcnumdomaintot(MSKtask_t task,MSKint64t * numdomaintot) {
  return (*MSK_getdjcnumdomaintot_ptr)(task,numdomaintot);
} /* MSKgetdjcnumdomaintot */
typedef MSKrescodee MSKAPI MSK_getdjcnumafe_t(MSKtask_t task,MSKint64t djcidx,MSKint64t * numafe);
static MSK_getdjcnumafe_t * MSK_getdjcnumafe_ptr = (MSK_getdjcnumafe_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdjcnumafe(MSKtask_t task,MSKint64t djcidx,MSKint64t * numafe) {
  return (*MSK_getdjcnumafe_ptr)(task,djcidx,numafe);
} /* MSKgetdjcnumafe */
typedef MSKrescodee MSKAPI MSK_getdjcnumafetot_t(MSKtask_t task,MSKint64t * numafetot);
static MSK_getdjcnumafetot_t * MSK_getdjcnumafetot_ptr = (MSK_getdjcnumafetot_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdjcnumafetot(MSKtask_t task,MSKint64t * numafetot) {
  return (*MSK_getdjcnumafetot_ptr)(task,numafetot);
} /* MSKgetdjcnumafetot */
typedef MSKrescodee MSKAPI MSK_getdjcnumterm_t(MSKtask_t task,MSKint64t djcidx,MSKint64t * numterm);
static MSK_getdjcnumterm_t * MSK_getdjcnumterm_ptr = (MSK_getdjcnumterm_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdjcnumterm(MSKtask_t task,MSKint64t djcidx,MSKint64t * numterm) {
  return (*MSK_getdjcnumterm_ptr)(task,djcidx,numterm);
} /* MSKgetdjcnumterm */
typedef MSKrescodee MSKAPI MSK_getdjcnumtermtot_t(MSKtask_t task,MSKint64t * numtermtot);
static MSK_getdjcnumtermtot_t * MSK_getdjcnumtermtot_ptr = (MSK_getdjcnumtermtot_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdjcnumtermtot(MSKtask_t task,MSKint64t * numtermtot) {
  return (*MSK_getdjcnumtermtot_ptr)(task,numtermtot);
} /* MSKgetdjcnumtermtot */
typedef MSKrescodee MSKAPI MSK_putmaxnumacc_t(MSKtask_t task,MSKint64t maxnumacc);
static MSK_putmaxnumacc_t * MSK_putmaxnumacc_ptr = (MSK_putmaxnumacc_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putmaxnumacc(MSKtask_t task,MSKint64t maxnumacc) {
  return (*MSK_putmaxnumacc_ptr)(task,maxnumacc);
} /* MSKputmaxnumacc */
typedef MSKrescodee MSKAPI MSK_getnumacc_t(MSKtask_t task,MSKint64t * num);
static MSK_getnumacc_t * MSK_getnumacc_ptr = (MSK_getnumacc_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnumacc(MSKtask_t task,MSKint64t * num) {
  return (*MSK_getnumacc_ptr)(task,num);
} /* MSKgetnumacc */
typedef MSKrescodee MSKAPI MSK_appendacc_t(MSKtask_t task,MSKint64t domidx,MSKint64t numafeidx,const MSKint64t * afeidxlist,const MSKrealt * b);
static MSK_appendacc_t * MSK_appendacc_ptr = (MSK_appendacc_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appendacc(MSKtask_t task,MSKint64t domidx,MSKint64t numafeidx,const MSKint64t * afeidxlist,const MSKrealt * b) {
  return (*MSK_appendacc_ptr)(task,domidx,numafeidx,afeidxlist,b);
} /* MSKappendacc */
typedef MSKrescodee MSKAPI MSK_appendaccs_t(MSKtask_t task,MSKint64t numaccs,const MSKint64t * domidxs,MSKint64t numafeidx,const MSKint64t * afeidxlist,const MSKrealt * b);
static MSK_appendaccs_t * MSK_appendaccs_ptr = (MSK_appendaccs_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appendaccs(MSKtask_t task,MSKint64t numaccs,const MSKint64t * domidxs,MSKint64t numafeidx,const MSKint64t * afeidxlist,const MSKrealt * b) {
  return (*MSK_appendaccs_ptr)(task,numaccs,domidxs,numafeidx,afeidxlist,b);
} /* MSKappendaccs */
typedef MSKrescodee MSKAPI MSK_appendaccseq_t(MSKtask_t task,MSKint64t domidx,MSKint64t numafeidx,MSKint64t afeidxfirst,const MSKrealt * b);
static MSK_appendaccseq_t * MSK_appendaccseq_ptr = (MSK_appendaccseq_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appendaccseq(MSKtask_t task,MSKint64t domidx,MSKint64t numafeidx,MSKint64t afeidxfirst,const MSKrealt * b) {
  return (*MSK_appendaccseq_ptr)(task,domidx,numafeidx,afeidxfirst,b);
} /* MSKappendaccseq */
typedef MSKrescodee MSKAPI MSK_appendaccsseq_t(MSKtask_t task,MSKint64t numaccs,const MSKint64t * domidxs,MSKint64t numafeidx,MSKint64t afeidxfirst,const MSKrealt * b);
static MSK_appendaccsseq_t * MSK_appendaccsseq_ptr = (MSK_appendaccsseq_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appendaccsseq(MSKtask_t task,MSKint64t numaccs,const MSKint64t * domidxs,MSKint64t numafeidx,MSKint64t afeidxfirst,const MSKrealt * b) {
  return (*MSK_appendaccsseq_ptr)(task,numaccs,domidxs,numafeidx,afeidxfirst,b);
} /* MSKappendaccsseq */
typedef MSKrescodee MSKAPI MSK_putacc_t(MSKtask_t task,MSKint64t accidx,MSKint64t domidx,MSKint64t numafeidx,const MSKint64t * afeidxlist,const MSKrealt * b);
static MSK_putacc_t * MSK_putacc_ptr = (MSK_putacc_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putacc(MSKtask_t task,MSKint64t accidx,MSKint64t domidx,MSKint64t numafeidx,const MSKint64t * afeidxlist,const MSKrealt * b) {
  return (*MSK_putacc_ptr)(task,accidx,domidx,numafeidx,afeidxlist,b);
} /* MSKputacc */
typedef MSKrescodee MSKAPI MSK_putacclist_t(MSKtask_t task,MSKint64t numaccs,const MSKint64t * accidxs,const MSKint64t * domidxs,MSKint64t numafeidx,const MSKint64t * afeidxlist,const MSKrealt * b);
static MSK_putacclist_t * MSK_putacclist_ptr = (MSK_putacclist_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putacclist(MSKtask_t task,MSKint64t numaccs,const MSKint64t * accidxs,const MSKint64t * domidxs,MSKint64t numafeidx,const MSKint64t * afeidxlist,const MSKrealt * b) {
  return (*MSK_putacclist_ptr)(task,numaccs,accidxs,domidxs,numafeidx,afeidxlist,b);
} /* MSKputacclist */
typedef MSKrescodee MSKAPI MSK_putaccb_t(MSKtask_t task,MSKint64t accidx,MSKint64t lengthb,const MSKrealt * b);
static MSK_putaccb_t * MSK_putaccb_ptr = (MSK_putaccb_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putaccb(MSKtask_t task,MSKint64t accidx,MSKint64t lengthb,const MSKrealt * b) {
  return (*MSK_putaccb_ptr)(task,accidx,lengthb,b);
} /* MSKputaccb */
typedef MSKrescodee MSKAPI MSK_putaccbj_t(MSKtask_t task,MSKint64t accidx,MSKint64t j,MSKrealt bj);
static MSK_putaccbj_t * MSK_putaccbj_ptr = (MSK_putaccbj_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putaccbj(MSKtask_t task,MSKint64t accidx,MSKint64t j,MSKrealt bj) {
  return (*MSK_putaccbj_ptr)(task,accidx,j,bj);
} /* MSKputaccbj */
typedef MSKrescodee MSKAPI MSK_getaccdomain_t(MSKtask_t task,MSKint64t accidx,MSKint64t * domidx);
static MSK_getaccdomain_t * MSK_getaccdomain_ptr = (MSK_getaccdomain_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getaccdomain(MSKtask_t task,MSKint64t accidx,MSKint64t * domidx) {
  return (*MSK_getaccdomain_ptr)(task,accidx,domidx);
} /* MSKgetaccdomain */
typedef MSKrescodee MSKAPI MSK_getaccn_t(MSKtask_t task,MSKint64t accidx,MSKint64t * n);
static MSK_getaccn_t * MSK_getaccn_ptr = (MSK_getaccn_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getaccn(MSKtask_t task,MSKint64t accidx,MSKint64t * n) {
  return (*MSK_getaccn_ptr)(task,accidx,n);
} /* MSKgetaccn */
typedef MSKrescodee MSKAPI MSK_getaccntot_t(MSKtask_t task,MSKint64t * n);
static MSK_getaccntot_t * MSK_getaccntot_ptr = (MSK_getaccntot_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getaccntot(MSKtask_t task,MSKint64t * n) {
  return (*MSK_getaccntot_ptr)(task,n);
} /* MSKgetaccntot */
typedef MSKrescodee MSKAPI MSK_getaccafeidxlist_t(MSKtask_t task,MSKint64t accidx,MSKint64t * afeidxlist);
static MSK_getaccafeidxlist_t * MSK_getaccafeidxlist_ptr = (MSK_getaccafeidxlist_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getaccafeidxlist(MSKtask_t task,MSKint64t accidx,MSKint64t * afeidxlist) {
  return (*MSK_getaccafeidxlist_ptr)(task,accidx,afeidxlist);
} /* MSKgetaccafeidxlist */
typedef MSKrescodee MSKAPI MSK_getaccb_t(MSKtask_t task,MSKint64t accidx,MSKrealt * b);
static MSK_getaccb_t * MSK_getaccb_ptr = (MSK_getaccb_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getaccb(MSKtask_t task,MSKint64t accidx,MSKrealt * b) {
  return (*MSK_getaccb_ptr)(task,accidx,b);
} /* MSKgetaccb */
typedef MSKrescodee MSKAPI MSK_getaccs_t(MSKtask_t task,MSKint64t * domidxlist,MSKint64t * afeidxlist,MSKrealt * b);
static MSK_getaccs_t * MSK_getaccs_ptr = (MSK_getaccs_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getaccs(MSKtask_t task,MSKint64t * domidxlist,MSKint64t * afeidxlist,MSKrealt * b) {
  return (*MSK_getaccs_ptr)(task,domidxlist,afeidxlist,b);
} /* MSKgetaccs */
typedef MSKrescodee MSKAPI MSK_getaccfnumnz_t(MSKtask_t task,MSKint64t * accfnnz);
static MSK_getaccfnumnz_t * MSK_getaccfnumnz_ptr = (MSK_getaccfnumnz_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getaccfnumnz(MSKtask_t task,MSKint64t * accfnnz) {
  return (*MSK_getaccfnumnz_ptr)(task,accfnnz);
} /* MSKgetaccfnumnz */
typedef MSKrescodee MSKAPI MSK_getaccftrip_t(MSKtask_t task,MSKint64t * frow,MSKint32t * fcol,MSKrealt * fval);
static MSK_getaccftrip_t * MSK_getaccftrip_ptr = (MSK_getaccftrip_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getaccftrip(MSKtask_t task,MSKint64t * frow,MSKint32t * fcol,MSKrealt * fval) {
  return (*MSK_getaccftrip_ptr)(task,frow,fcol,fval);
} /* MSKgetaccftrip */
typedef MSKrescodee MSKAPI MSK_getaccgvector_t(MSKtask_t task,MSKrealt * g);
static MSK_getaccgvector_t * MSK_getaccgvector_ptr = (MSK_getaccgvector_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getaccgvector(MSKtask_t task,MSKrealt * g) {
  return (*MSK_getaccgvector_ptr)(task,g);
} /* MSKgetaccgvector */
typedef MSKrescodee MSKAPI MSK_getaccbarfnumblocktriplets_t(MSKtask_t task,MSKint64t * numtrip);
static MSK_getaccbarfnumblocktriplets_t * MSK_getaccbarfnumblocktriplets_ptr = (MSK_getaccbarfnumblocktriplets_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getaccbarfnumblocktriplets(MSKtask_t task,MSKint64t * numtrip) {
  return (*MSK_getaccbarfnumblocktriplets_ptr)(task,numtrip);
} /* MSKgetaccbarfnumblocktriplets */
typedef MSKrescodee MSKAPI MSK_getaccbarfblocktriplet_t(MSKtask_t task,MSKint64t maxnumtrip,MSKint64t * numtrip,MSKint64t * acc_afe,MSKint32t * bar_var,MSKint32t * blk_row,MSKint32t * blk_col,MSKrealt * blk_val);
static MSK_getaccbarfblocktriplet_t * MSK_getaccbarfblocktriplet_ptr = (MSK_getaccbarfblocktriplet_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getaccbarfblocktriplet(MSKtask_t task,MSKint64t maxnumtrip,MSKint64t * numtrip,MSKint64t * acc_afe,MSKint32t * bar_var,MSKint32t * blk_row,MSKint32t * blk_col,MSKrealt * blk_val) {
  return (*MSK_getaccbarfblocktriplet_ptr)(task,maxnumtrip,numtrip,acc_afe,bar_var,blk_row,blk_col,blk_val);
} /* MSKgetaccbarfblocktriplet */
typedef MSKrescodee MSKAPI MSK_appenddjcs_t(MSKtask_t task,MSKint64t num);
static MSK_appenddjcs_t * MSK_appenddjcs_ptr = (MSK_appenddjcs_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appenddjcs(MSKtask_t task,MSKint64t num) {
  return (*MSK_appenddjcs_ptr)(task,num);
} /* MSKappenddjcs */
typedef MSKrescodee MSKAPI MSK_putdjc_t(MSKtask_t task,MSKint64t djcidx,MSKint64t numdomidx,const MSKint64t * domidxlist,MSKint64t numafeidx,const MSKint64t * afeidxlist,const MSKrealt * b,MSKint64t numterms,const MSKint64t * termsizelist);
static MSK_putdjc_t * MSK_putdjc_ptr = (MSK_putdjc_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putdjc(MSKtask_t task,MSKint64t djcidx,MSKint64t numdomidx,const MSKint64t * domidxlist,MSKint64t numafeidx,const MSKint64t * afeidxlist,const MSKrealt * b,MSKint64t numterms,const MSKint64t * termsizelist) {
  return (*MSK_putdjc_ptr)(task,djcidx,numdomidx,domidxlist,numafeidx,afeidxlist,b,numterms,termsizelist);
} /* MSKputdjc */
typedef MSKrescodee MSKAPI MSK_putdjcslice_t(MSKtask_t task,MSKint64t idxfirst,MSKint64t idxlast,MSKint64t numdomidx,const MSKint64t * domidxlist,MSKint64t numafeidx,const MSKint64t * afeidxlist,const MSKrealt * b,MSKint64t numterms,const MSKint64t * termsizelist,const MSKint64t * termsindjc);
static MSK_putdjcslice_t * MSK_putdjcslice_ptr = (MSK_putdjcslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putdjcslice(MSKtask_t task,MSKint64t idxfirst,MSKint64t idxlast,MSKint64t numdomidx,const MSKint64t * domidxlist,MSKint64t numafeidx,const MSKint64t * afeidxlist,const MSKrealt * b,MSKint64t numterms,const MSKint64t * termsizelist,const MSKint64t * termsindjc) {
  return (*MSK_putdjcslice_ptr)(task,idxfirst,idxlast,numdomidx,domidxlist,numafeidx,afeidxlist,b,numterms,termsizelist,termsindjc);
} /* MSKputdjcslice */
typedef MSKrescodee MSKAPI MSK_getdjcdomainidxlist_t(MSKtask_t task,MSKint64t djcidx,MSKint64t * domidxlist);
static MSK_getdjcdomainidxlist_t * MSK_getdjcdomainidxlist_ptr = (MSK_getdjcdomainidxlist_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdjcdomainidxlist(MSKtask_t task,MSKint64t djcidx,MSKint64t * domidxlist) {
  return (*MSK_getdjcdomainidxlist_ptr)(task,djcidx,domidxlist);
} /* MSKgetdjcdomainidxlist */
typedef MSKrescodee MSKAPI MSK_getdjcafeidxlist_t(MSKtask_t task,MSKint64t djcidx,MSKint64t * afeidxlist);
static MSK_getdjcafeidxlist_t * MSK_getdjcafeidxlist_ptr = (MSK_getdjcafeidxlist_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdjcafeidxlist(MSKtask_t task,MSKint64t djcidx,MSKint64t * afeidxlist) {
  return (*MSK_getdjcafeidxlist_ptr)(task,djcidx,afeidxlist);
} /* MSKgetdjcafeidxlist */
typedef MSKrescodee MSKAPI MSK_getdjcb_t(MSKtask_t task,MSKint64t djcidx,MSKrealt * b);
static MSK_getdjcb_t * MSK_getdjcb_ptr = (MSK_getdjcb_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdjcb(MSKtask_t task,MSKint64t djcidx,MSKrealt * b) {
  return (*MSK_getdjcb_ptr)(task,djcidx,b);
} /* MSKgetdjcb */
typedef MSKrescodee MSKAPI MSK_getdjctermsizelist_t(MSKtask_t task,MSKint64t djcidx,MSKint64t * termsizelist);
static MSK_getdjctermsizelist_t * MSK_getdjctermsizelist_ptr = (MSK_getdjctermsizelist_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdjctermsizelist(MSKtask_t task,MSKint64t djcidx,MSKint64t * termsizelist) {
  return (*MSK_getdjctermsizelist_ptr)(task,djcidx,termsizelist);
} /* MSKgetdjctermsizelist */
typedef MSKrescodee MSKAPI MSK_getdjcs_t(MSKtask_t task,MSKint64t * domidxlist,MSKint64t * afeidxlist,MSKrealt * b,MSKint64t * termsizelist,MSKint64t * numterms);
static MSK_getdjcs_t * MSK_getdjcs_ptr = (MSK_getdjcs_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdjcs(MSKtask_t task,MSKint64t * domidxlist,MSKint64t * afeidxlist,MSKrealt * b,MSKint64t * termsizelist,MSKint64t * numterms) {
  return (*MSK_getdjcs_ptr)(task,domidxlist,afeidxlist,b,termsizelist,numterms);
} /* MSKgetdjcs */
typedef MSKrescodee MSKAPI MSK_putconbound_t(MSKtask_t task,MSKint32t i,MSKboundkeye bkc,MSKrealt blc,MSKrealt buc);
static MSK_putconbound_t * MSK_putconbound_ptr = (MSK_putconbound_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putconbound(MSKtask_t task,MSKint32t i,MSKboundkeye bkc,MSKrealt blc,MSKrealt buc) {
  return (*MSK_putconbound_ptr)(task,i,bkc,blc,buc);
} /* MSKputconbound */
typedef MSKrescodee MSKAPI MSK_putconboundlist_t(MSKtask_t task,MSKint32t num,const MSKint32t * sub,const MSKboundkeye * bkc,const MSKrealt * blc,const MSKrealt * buc);
static MSK_putconboundlist_t * MSK_putconboundlist_ptr = (MSK_putconboundlist_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putconboundlist(MSKtask_t task,MSKint32t num,const MSKint32t * sub,const MSKboundkeye * bkc,const MSKrealt * blc,const MSKrealt * buc) {
  return (*MSK_putconboundlist_ptr)(task,num,sub,bkc,blc,buc);
} /* MSKputconboundlist */
typedef MSKrescodee MSKAPI MSK_putconboundlistconst_t(MSKtask_t task,MSKint32t num,const MSKint32t * sub,MSKboundkeye bkc,MSKrealt blc,MSKrealt buc);
static MSK_putconboundlistconst_t * MSK_putconboundlistconst_ptr = (MSK_putconboundlistconst_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putconboundlistconst(MSKtask_t task,MSKint32t num,const MSKint32t * sub,MSKboundkeye bkc,MSKrealt blc,MSKrealt buc) {
  return (*MSK_putconboundlistconst_ptr)(task,num,sub,bkc,blc,buc);
} /* MSKputconboundlistconst */
typedef MSKrescodee MSKAPI MSK_putconboundslice_t(MSKtask_t task,MSKint32t first,MSKint32t last,const MSKboundkeye * bkc,const MSKrealt * blc,const MSKrealt * buc);
static MSK_putconboundslice_t * MSK_putconboundslice_ptr = (MSK_putconboundslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putconboundslice(MSKtask_t task,MSKint32t first,MSKint32t last,const MSKboundkeye * bkc,const MSKrealt * blc,const MSKrealt * buc) {
  return (*MSK_putconboundslice_ptr)(task,first,last,bkc,blc,buc);
} /* MSKputconboundslice */
typedef MSKrescodee MSKAPI MSK_putconboundsliceconst_t(MSKtask_t task,MSKint32t first,MSKint32t last,MSKboundkeye bkc,MSKrealt blc,MSKrealt buc);
static MSK_putconboundsliceconst_t * MSK_putconboundsliceconst_ptr = (MSK_putconboundsliceconst_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putconboundsliceconst(MSKtask_t task,MSKint32t first,MSKint32t last,MSKboundkeye bkc,MSKrealt blc,MSKrealt buc) {
  return (*MSK_putconboundsliceconst_ptr)(task,first,last,bkc,blc,buc);
} /* MSKputconboundsliceconst */
typedef MSKrescodee MSKAPI MSK_putvarbound_t(MSKtask_t task,MSKint32t j,MSKboundkeye bkx,MSKrealt blx,MSKrealt bux);
static MSK_putvarbound_t * MSK_putvarbound_ptr = (MSK_putvarbound_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putvarbound(MSKtask_t task,MSKint32t j,MSKboundkeye bkx,MSKrealt blx,MSKrealt bux) {
  return (*MSK_putvarbound_ptr)(task,j,bkx,blx,bux);
} /* MSKputvarbound */
typedef MSKrescodee MSKAPI MSK_putvarboundlist_t(MSKtask_t task,MSKint32t num,const MSKint32t * sub,const MSKboundkeye * bkx,const MSKrealt * blx,const MSKrealt * bux);
static MSK_putvarboundlist_t * MSK_putvarboundlist_ptr = (MSK_putvarboundlist_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putvarboundlist(MSKtask_t task,MSKint32t num,const MSKint32t * sub,const MSKboundkeye * bkx,const MSKrealt * blx,const MSKrealt * bux) {
  return (*MSK_putvarboundlist_ptr)(task,num,sub,bkx,blx,bux);
} /* MSKputvarboundlist */
typedef MSKrescodee MSKAPI MSK_putvarboundlistconst_t(MSKtask_t task,MSKint32t num,const MSKint32t * sub,MSKboundkeye bkx,MSKrealt blx,MSKrealt bux);
static MSK_putvarboundlistconst_t * MSK_putvarboundlistconst_ptr = (MSK_putvarboundlistconst_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putvarboundlistconst(MSKtask_t task,MSKint32t num,const MSKint32t * sub,MSKboundkeye bkx,MSKrealt blx,MSKrealt bux) {
  return (*MSK_putvarboundlistconst_ptr)(task,num,sub,bkx,blx,bux);
} /* MSKputvarboundlistconst */
typedef MSKrescodee MSKAPI MSK_putvarboundslice_t(MSKtask_t task,MSKint32t first,MSKint32t last,const MSKboundkeye * bkx,const MSKrealt * blx,const MSKrealt * bux);
static MSK_putvarboundslice_t * MSK_putvarboundslice_ptr = (MSK_putvarboundslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putvarboundslice(MSKtask_t task,MSKint32t first,MSKint32t last,const MSKboundkeye * bkx,const MSKrealt * blx,const MSKrealt * bux) {
  return (*MSK_putvarboundslice_ptr)(task,first,last,bkx,blx,bux);
} /* MSKputvarboundslice */
typedef MSKrescodee MSKAPI MSK_putvarboundsliceconst_t(MSKtask_t task,MSKint32t first,MSKint32t last,MSKboundkeye bkx,MSKrealt blx,MSKrealt bux);
static MSK_putvarboundsliceconst_t * MSK_putvarboundsliceconst_ptr = (MSK_putvarboundsliceconst_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putvarboundsliceconst(MSKtask_t task,MSKint32t first,MSKint32t last,MSKboundkeye bkx,MSKrealt blx,MSKrealt bux) {
  return (*MSK_putvarboundsliceconst_ptr)(task,first,last,bkx,blx,bux);
} /* MSKputvarboundsliceconst */
typedef MSKrescodee MSKAPI MSK_putcallbackfunc_t(MSKtask_t task,MSKcallbackfunc func,MSKuserhandle_t handle);
static MSK_putcallbackfunc_t * MSK_putcallbackfunc_ptr = (MSK_putcallbackfunc_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putcallbackfunc(MSKtask_t task,MSKcallbackfunc func,MSKuserhandle_t handle) {
  return (*MSK_putcallbackfunc_ptr)(task,func,handle);
} /* MSKputcallbackfunc */
typedef MSKrescodee MSKAPI MSK_putcfix_t(MSKtask_t task,MSKrealt cfix);
static MSK_putcfix_t * MSK_putcfix_ptr = (MSK_putcfix_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putcfix(MSKtask_t task,MSKrealt cfix) {
  return (*MSK_putcfix_ptr)(task,cfix);
} /* MSKputcfix */
typedef MSKrescodee MSKAPI MSK_putcj_t(MSKtask_t task,MSKint32t j,MSKrealt cj);
static MSK_putcj_t * MSK_putcj_ptr = (MSK_putcj_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putcj(MSKtask_t task,MSKint32t j,MSKrealt cj) {
  return (*MSK_putcj_ptr)(task,j,cj);
} /* MSKputcj */
typedef MSKrescodee MSKAPI MSK_putobjsense_t(MSKtask_t task,MSKobjsensee sense);
static MSK_putobjsense_t * MSK_putobjsense_ptr = (MSK_putobjsense_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putobjsense(MSKtask_t task,MSKobjsensee sense) {
  return (*MSK_putobjsense_ptr)(task,sense);
} /* MSKputobjsense */
typedef MSKrescodee MSKAPI MSK_getobjsense_t(MSKtask_t task,MSKobjsensee * sense);
static MSK_getobjsense_t * MSK_getobjsense_ptr = (MSK_getobjsense_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getobjsense(MSKtask_t task,MSKobjsensee * sense) {
  return (*MSK_getobjsense_ptr)(task,sense);
} /* MSKgetobjsense */
typedef MSKrescodee MSKAPI MSK_putclist_t(MSKtask_t task,MSKint32t num,const MSKint32t * subj,const MSKrealt * val);
static MSK_putclist_t * MSK_putclist_ptr = (MSK_putclist_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putclist(MSKtask_t task,MSKint32t num,const MSKint32t * subj,const MSKrealt * val) {
  return (*MSK_putclist_ptr)(task,num,subj,val);
} /* MSKputclist */
typedef MSKrescodee MSKAPI MSK_putcslice_t(MSKtask_t task,MSKint32t first,MSKint32t last,const MSKrealt * slice);
static MSK_putcslice_t * MSK_putcslice_ptr = (MSK_putcslice_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putcslice(MSKtask_t task,MSKint32t first,MSKint32t last,const MSKrealt * slice) {
  return (*MSK_putcslice_ptr)(task,first,last,slice);
} /* MSKputcslice */
typedef MSKrescodee MSKAPI MSK_putbarcj_t(MSKtask_t task,MSKint32t j,MSKint64t num,const MSKint64t * sub,const MSKrealt * weights);
static MSK_putbarcj_t * MSK_putbarcj_ptr = (MSK_putbarcj_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putbarcj(MSKtask_t task,MSKint32t j,MSKint64t num,const MSKint64t * sub,const MSKrealt * weights) {
  return (*MSK_putbarcj_ptr)(task,j,num,sub,weights);
} /* MSKputbarcj */
typedef MSKrescodee MSKAPI MSK_putcone_t(MSKtask_t task,MSKint32t k,MSKconetypee ct,MSKrealt conepar,MSKint32t nummem,const MSKint32t * submem);
static MSK_putcone_t * MSK_putcone_ptr = (MSK_putcone_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putcone(MSKtask_t task,MSKint32t k,MSKconetypee ct,MSKrealt conepar,MSKint32t nummem,const MSKint32t * submem) {
  return (*MSK_putcone_ptr)(task,k,ct,conepar,nummem,submem);
} /* MSKputcone */
typedef MSKrescodee MSKAPI MSK_putmaxnumdomain_t(MSKtask_t task,MSKint64t maxnumdomain);
static MSK_putmaxnumdomain_t * MSK_putmaxnumdomain_ptr = (MSK_putmaxnumdomain_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putmaxnumdomain(MSKtask_t task,MSKint64t maxnumdomain) {
  return (*MSK_putmaxnumdomain_ptr)(task,maxnumdomain);
} /* MSKputmaxnumdomain */
typedef MSKrescodee MSKAPI MSK_getnumdomain_t(MSKtask_t task,MSKint64t * numdomain);
static MSK_getnumdomain_t * MSK_getnumdomain_ptr = (MSK_getnumdomain_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnumdomain(MSKtask_t task,MSKint64t * numdomain) {
  return (*MSK_getnumdomain_ptr)(task,numdomain);
} /* MSKgetnumdomain */
typedef MSKrescodee MSKAPI MSK_appendrplusdomain_t(MSKtask_t task,MSKint64t n,MSKint64t * domidx);
static MSK_appendrplusdomain_t * MSK_appendrplusdomain_ptr = (MSK_appendrplusdomain_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appendrplusdomain(MSKtask_t task,MSKint64t n,MSKint64t * domidx) {
  return (*MSK_appendrplusdomain_ptr)(task,n,domidx);
} /* MSKappendrplusdomain */
typedef MSKrescodee MSKAPI MSK_appendrminusdomain_t(MSKtask_t task,MSKint64t n,MSKint64t * domidx);
static MSK_appendrminusdomain_t * MSK_appendrminusdomain_ptr = (MSK_appendrminusdomain_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appendrminusdomain(MSKtask_t task,MSKint64t n,MSKint64t * domidx) {
  return (*MSK_appendrminusdomain_ptr)(task,n,domidx);
} /* MSKappendrminusdomain */
typedef MSKrescodee MSKAPI MSK_appendrdomain_t(MSKtask_t task,MSKint64t n,MSKint64t * domidx);
static MSK_appendrdomain_t * MSK_appendrdomain_ptr = (MSK_appendrdomain_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appendrdomain(MSKtask_t task,MSKint64t n,MSKint64t * domidx) {
  return (*MSK_appendrdomain_ptr)(task,n,domidx);
} /* MSKappendrdomain */
typedef MSKrescodee MSKAPI MSK_appendrzerodomain_t(MSKtask_t task,MSKint64t n,MSKint64t * domidx);
static MSK_appendrzerodomain_t * MSK_appendrzerodomain_ptr = (MSK_appendrzerodomain_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appendrzerodomain(MSKtask_t task,MSKint64t n,MSKint64t * domidx) {
  return (*MSK_appendrzerodomain_ptr)(task,n,domidx);
} /* MSKappendrzerodomain */
typedef MSKrescodee MSKAPI MSK_appendquadraticconedomain_t(MSKtask_t task,MSKint64t n,MSKint64t * domidx);
static MSK_appendquadraticconedomain_t * MSK_appendquadraticconedomain_ptr = (MSK_appendquadraticconedomain_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appendquadraticconedomain(MSKtask_t task,MSKint64t n,MSKint64t * domidx) {
  return (*MSK_appendquadraticconedomain_ptr)(task,n,domidx);
} /* MSKappendquadraticconedomain */
typedef MSKrescodee MSKAPI MSK_appendrquadraticconedomain_t(MSKtask_t task,MSKint64t n,MSKint64t * domidx);
static MSK_appendrquadraticconedomain_t * MSK_appendrquadraticconedomain_ptr = (MSK_appendrquadraticconedomain_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appendrquadraticconedomain(MSKtask_t task,MSKint64t n,MSKint64t * domidx) {
  return (*MSK_appendrquadraticconedomain_ptr)(task,n,domidx);
} /* MSKappendrquadraticconedomain */
typedef MSKrescodee MSKAPI MSK_appendprimalexpconedomain_t(MSKtask_t task,MSKint64t * domidx);
static MSK_appendprimalexpconedomain_t * MSK_appendprimalexpconedomain_ptr = (MSK_appendprimalexpconedomain_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appendprimalexpconedomain(MSKtask_t task,MSKint64t * domidx) {
  return (*MSK_appendprimalexpconedomain_ptr)(task,domidx);
} /* MSKappendprimalexpconedomain */
typedef MSKrescodee MSKAPI MSK_appenddualexpconedomain_t(MSKtask_t task,MSKint64t * domidx);
static MSK_appenddualexpconedomain_t * MSK_appenddualexpconedomain_ptr = (MSK_appenddualexpconedomain_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appenddualexpconedomain(MSKtask_t task,MSKint64t * domidx) {
  return (*MSK_appenddualexpconedomain_ptr)(task,domidx);
} /* MSKappenddualexpconedomain */
typedef MSKrescodee MSKAPI MSK_appendprimalgeomeanconedomain_t(MSKtask_t task,MSKint64t n,MSKint64t * domidx);
static MSK_appendprimalgeomeanconedomain_t * MSK_appendprimalgeomeanconedomain_ptr = (MSK_appendprimalgeomeanconedomain_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appendprimalgeomeanconedomain(MSKtask_t task,MSKint64t n,MSKint64t * domidx) {
  return (*MSK_appendprimalgeomeanconedomain_ptr)(task,n,domidx);
} /* MSKappendprimalgeomeanconedomain */
typedef MSKrescodee MSKAPI MSK_appenddualgeomeanconedomain_t(MSKtask_t task,MSKint64t n,MSKint64t * domidx);
static MSK_appenddualgeomeanconedomain_t * MSK_appenddualgeomeanconedomain_ptr = (MSK_appenddualgeomeanconedomain_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appenddualgeomeanconedomain(MSKtask_t task,MSKint64t n,MSKint64t * domidx) {
  return (*MSK_appenddualgeomeanconedomain_ptr)(task,n,domidx);
} /* MSKappenddualgeomeanconedomain */
typedef MSKrescodee MSKAPI MSK_appendprimalpowerconedomain_t(MSKtask_t task,MSKint64t n,MSKint64t nleft,const MSKrealt * alpha,MSKint64t * domidx);
static MSK_appendprimalpowerconedomain_t * MSK_appendprimalpowerconedomain_ptr = (MSK_appendprimalpowerconedomain_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appendprimalpowerconedomain(MSKtask_t task,MSKint64t n,MSKint64t nleft,const MSKrealt * alpha,MSKint64t * domidx) {
  return (*MSK_appendprimalpowerconedomain_ptr)(task,n,nleft,alpha,domidx);
} /* MSKappendprimalpowerconedomain */
typedef MSKrescodee MSKAPI MSK_appendprimalpowerconedomainseq_t(MSKtask_t task,MSKint64t num,const MSKint64t * n,const MSKint64t * nleft,const MSKrealt * alpha,MSKint64t * domidxlist);
static MSK_appendprimalpowerconedomainseq_t * MSK_appendprimalpowerconedomainseq_ptr = (MSK_appendprimalpowerconedomainseq_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appendprimalpowerconedomainseq(MSKtask_t task,MSKint64t num,const MSKint64t * n,const MSKint64t * nleft,const MSKrealt * alpha,MSKint64t * domidxlist) {
  return (*MSK_appendprimalpowerconedomainseq_ptr)(task,num,n,nleft,alpha,domidxlist);
} /* MSKappendprimalpowerconedomainseq */
typedef MSKrescodee MSKAPI MSK_appenddualpowerconedomain_t(MSKtask_t task,MSKint64t n,MSKint64t nleft,const MSKrealt * alpha,MSKint64t * domidx);
static MSK_appenddualpowerconedomain_t * MSK_appenddualpowerconedomain_ptr = (MSK_appenddualpowerconedomain_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appenddualpowerconedomain(MSKtask_t task,MSKint64t n,MSKint64t nleft,const MSKrealt * alpha,MSKint64t * domidx) {
  return (*MSK_appenddualpowerconedomain_ptr)(task,n,nleft,alpha,domidx);
} /* MSKappenddualpowerconedomain */
typedef MSKrescodee MSKAPI MSK_appenddualpowerconedomainseq_t(MSKtask_t task,MSKint64t num,const MSKint64t * n,const MSKint64t * nleft,const MSKrealt * alpha,MSKint64t * domidxlist);
static MSK_appenddualpowerconedomainseq_t * MSK_appenddualpowerconedomainseq_ptr = (MSK_appenddualpowerconedomainseq_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appenddualpowerconedomainseq(MSKtask_t task,MSKint64t num,const MSKint64t * n,const MSKint64t * nleft,const MSKrealt * alpha,MSKint64t * domidxlist) {
  return (*MSK_appenddualpowerconedomainseq_ptr)(task,num,n,nleft,alpha,domidxlist);
} /* MSKappenddualpowerconedomainseq */
typedef MSKrescodee MSKAPI MSK_appendsvecpsdconedomain_t(MSKtask_t task,MSKint64t n,MSKint64t * domidx);
static MSK_appendsvecpsdconedomain_t * MSK_appendsvecpsdconedomain_ptr = (MSK_appendsvecpsdconedomain_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appendsvecpsdconedomain(MSKtask_t task,MSKint64t n,MSKint64t * domidx) {
  return (*MSK_appendsvecpsdconedomain_ptr)(task,n,domidx);
} /* MSKappendsvecpsdconedomain */
typedef MSKrescodee MSKAPI MSK_getdomaintype_t(MSKtask_t task,MSKint64t domidx,MSKdomaintypee * domtype);
static MSK_getdomaintype_t * MSK_getdomaintype_ptr = (MSK_getdomaintype_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdomaintype(MSKtask_t task,MSKint64t domidx,MSKdomaintypee * domtype) {
  return (*MSK_getdomaintype_ptr)(task,domidx,domtype);
} /* MSKgetdomaintype */
typedef MSKrescodee MSKAPI MSK_getdomainn_t(MSKtask_t task,MSKint64t domidx,MSKint64t * n);
static MSK_getdomainn_t * MSK_getdomainn_ptr = (MSK_getdomainn_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdomainn(MSKtask_t task,MSKint64t domidx,MSKint64t * n) {
  return (*MSK_getdomainn_ptr)(task,domidx,n);
} /* MSKgetdomainn */
typedef MSKrescodee MSKAPI MSK_getpowerdomaininfo_t(MSKtask_t task,MSKint64t domidx,MSKint64t * n,MSKint64t * nleft);
static MSK_getpowerdomaininfo_t * MSK_getpowerdomaininfo_ptr = (MSK_getpowerdomaininfo_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getpowerdomaininfo(MSKtask_t task,MSKint64t domidx,MSKint64t * n,MSKint64t * nleft) {
  return (*MSK_getpowerdomaininfo_ptr)(task,domidx,n,nleft);
} /* MSKgetpowerdomaininfo */
typedef MSKrescodee MSKAPI MSK_getpowerdomainalpha_t(MSKtask_t task,MSKint64t domidx,MSKrealt * alpha);
static MSK_getpowerdomainalpha_t * MSK_getpowerdomainalpha_ptr = (MSK_getpowerdomainalpha_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getpowerdomainalpha(MSKtask_t task,MSKint64t domidx,MSKrealt * alpha) {
  return (*MSK_getpowerdomainalpha_ptr)(task,domidx,alpha);
} /* MSKgetpowerdomainalpha */
typedef MSKrescodee MSKAPI MSK_appendsparsesymmat_t(MSKtask_t task,MSKint32t dim,MSKint64t nz,const MSKint32t * subi,const MSKint32t * subj,const MSKrealt * valij,MSKint64t * idx);
static MSK_appendsparsesymmat_t * MSK_appendsparsesymmat_ptr = (MSK_appendsparsesymmat_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appendsparsesymmat(MSKtask_t task,MSKint32t dim,MSKint64t nz,const MSKint32t * subi,const MSKint32t * subj,const MSKrealt * valij,MSKint64t * idx) {
  return (*MSK_appendsparsesymmat_ptr)(task,dim,nz,subi,subj,valij,idx);
} /* MSKappendsparsesymmat */
typedef MSKrescodee MSKAPI MSK_appendsparsesymmatlist_t(MSKtask_t task,MSKint32t num,const MSKint32t * dims,const MSKint64t * nz,const MSKint32t * subi,const MSKint32t * subj,const MSKrealt * valij,MSKint64t * idx);
static MSK_appendsparsesymmatlist_t * MSK_appendsparsesymmatlist_ptr = (MSK_appendsparsesymmatlist_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_appendsparsesymmatlist(MSKtask_t task,MSKint32t num,const MSKint32t * dims,const MSKint64t * nz,const MSKint32t * subi,const MSKint32t * subj,const MSKrealt * valij,MSKint64t * idx) {
  return (*MSK_appendsparsesymmatlist_ptr)(task,num,dims,nz,subi,subj,valij,idx);
} /* MSKappendsparsesymmatlist */
typedef MSKrescodee MSKAPI MSK_getsymmatinfo_t(MSKtask_t task,MSKint64t idx,MSKint32t * dim,MSKint64t * nz,MSKsymmattypee * mattype);
static MSK_getsymmatinfo_t * MSK_getsymmatinfo_ptr = (MSK_getsymmatinfo_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getsymmatinfo(MSKtask_t task,MSKint64t idx,MSKint32t * dim,MSKint64t * nz,MSKsymmattypee * mattype) {
  return (*MSK_getsymmatinfo_ptr)(task,idx,dim,nz,mattype);
} /* MSKgetsymmatinfo */
typedef MSKrescodee MSKAPI MSK_getnumsymmat_t(MSKtask_t task,MSKint64t * num);
static MSK_getnumsymmat_t * MSK_getnumsymmat_ptr = (MSK_getnumsymmat_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getnumsymmat(MSKtask_t task,MSKint64t * num) {
  return (*MSK_getnumsymmat_ptr)(task,num);
} /* MSKgetnumsymmat */
typedef MSKrescodee MSKAPI MSK_getsparsesymmat_t(MSKtask_t task,MSKint64t idx,MSKint64t maxlen,MSKint32t * subi,MSKint32t * subj,MSKrealt * valij);
static MSK_getsparsesymmat_t * MSK_getsparsesymmat_ptr = (MSK_getsparsesymmat_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getsparsesymmat(MSKtask_t task,MSKint64t idx,MSKint64t maxlen,MSKint32t * subi,MSKint32t * subj,MSKrealt * valij) {
  return (*MSK_getsparsesymmat_ptr)(task,idx,maxlen,subi,subj,valij);
} /* MSKgetsparsesymmat */
typedef MSKrescodee MSKAPI MSK_putdouparam_t(MSKtask_t task,MSKdparame param,MSKrealt parvalue);
static MSK_putdouparam_t * MSK_putdouparam_ptr = (MSK_putdouparam_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putdouparam(MSKtask_t task,MSKdparame param,MSKrealt parvalue) {
  return (*MSK_putdouparam_ptr)(task,param,parvalue);
} /* MSKputdouparam */
typedef MSKrescodee MSKAPI MSK_resetdouparam_t(MSKtask_t task,MSKdparame param);
static MSK_resetdouparam_t * MSK_resetdouparam_ptr = (MSK_resetdouparam_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_resetdouparam(MSKtask_t task,MSKdparame param) {
  return (*MSK_resetdouparam_ptr)(task,param);
} /* MSKresetdouparam */
typedef MSKrescodee MSKAPI MSK_putintparam_t(MSKtask_t task,MSKiparame param,MSKint32t parvalue);
static MSK_putintparam_t * MSK_putintparam_ptr = (MSK_putintparam_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putintparam(MSKtask_t task,MSKiparame param,MSKint32t parvalue) {
  return (*MSK_putintparam_ptr)(task,param,parvalue);
} /* MSKputintparam */
typedef MSKrescodee MSKAPI MSK_putlintparam_t(MSKtask_t task,MSKiparame param,MSKint64t parvalue);
static MSK_putlintparam_t * MSK_putlintparam_ptr = (MSK_putlintparam_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putlintparam(MSKtask_t task,MSKiparame param,MSKint64t parvalue) {
  return (*MSK_putlintparam_ptr)(task,param,parvalue);
} /* MSKputlintparam */
typedef MSKrescodee MSKAPI MSK_resetintparam_t(MSKtask_t task,MSKiparame param);
static MSK_resetintparam_t * MSK_resetintparam_ptr = (MSK_resetintparam_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_resetintparam(MSKtask_t task,MSKiparame param) {
  return (*MSK_resetintparam_ptr)(task,param);
} /* MSKresetintparam */
typedef MSKrescodee MSKAPI MSK_putmaxnumcon_t(MSKtask_t task,MSKint32t maxnumcon);
static MSK_putmaxnumcon_t * MSK_putmaxnumcon_ptr = (MSK_putmaxnumcon_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putmaxnumcon(MSKtask_t task,MSKint32t maxnumcon) {
  return (*MSK_putmaxnumcon_ptr)(task,maxnumcon);
} /* MSKputmaxnumcon */
typedef MSKrescodee MSKAPI MSK_putmaxnumcon64_t(MSKtask_t task,MSKint64t maxnumcon);
static MSK_putmaxnumcon64_t * MSK_putmaxnumcon64_ptr = (MSK_putmaxnumcon64_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putmaxnumcon64(MSKtask_t task,MSKint64t maxnumcon) {
  return (*MSK_putmaxnumcon64_ptr)(task,maxnumcon);
} /* MSKputmaxnumcon64 */
typedef MSKrescodee MSKAPI MSK_putmaxnumcone_t(MSKtask_t task,MSKint32t maxnumcone);
static MSK_putmaxnumcone_t * MSK_putmaxnumcone_ptr = (MSK_putmaxnumcone_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putmaxnumcone(MSKtask_t task,MSKint32t maxnumcone) {
  return (*MSK_putmaxnumcone_ptr)(task,maxnumcone);
} /* MSKputmaxnumcone */
typedef MSKrescodee MSKAPI MSK_getmaxnumcone_t(MSKtask_t task,MSKint32t * maxnumcone);
static MSK_getmaxnumcone_t * MSK_getmaxnumcone_ptr = (MSK_getmaxnumcone_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getmaxnumcone(MSKtask_t task,MSKint32t * maxnumcone) {
  return (*MSK_getmaxnumcone_ptr)(task,maxnumcone);
} /* MSKgetmaxnumcone */
typedef MSKrescodee MSKAPI MSK_putmaxnumvar_t(MSKtask_t task,MSKint32t maxnumvar);
static MSK_putmaxnumvar_t * MSK_putmaxnumvar_ptr = (MSK_putmaxnumvar_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putmaxnumvar(MSKtask_t task,MSKint32t maxnumvar) {
  return (*MSK_putmaxnumvar_ptr)(task,maxnumvar);
} /* MSKputmaxnumvar */
typedef MSKrescodee MSKAPI MSK_putmaxnumvar64_t(MSKtask_t task,MSKint64t maxnumvar);
static MSK_putmaxnumvar64_t * MSK_putmaxnumvar64_ptr = (MSK_putmaxnumvar64_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putmaxnumvar64(MSKtask_t task,MSKint64t maxnumvar) {
  return (*MSK_putmaxnumvar64_ptr)(task,maxnumvar);
} /* MSKputmaxnumvar64 */
typedef MSKrescodee MSKAPI MSK_putmaxnumbarvar_t(MSKtask_t task,MSKint32t maxnumbarvar);
static MSK_putmaxnumbarvar_t * MSK_putmaxnumbarvar_ptr = (MSK_putmaxnumbarvar_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putmaxnumbarvar(MSKtask_t task,MSKint32t maxnumbarvar) {
  return (*MSK_putmaxnumbarvar_ptr)(task,maxnumbarvar);
} /* MSKputmaxnumbarvar */
typedef MSKrescodee MSKAPI MSK_putmaxnumanz_t(MSKtask_t task,MSKint64t maxnumanz);
static MSK_putmaxnumanz_t * MSK_putmaxnumanz_ptr = (MSK_putmaxnumanz_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putmaxnumanz(MSKtask_t task,MSKint64t maxnumanz) {
  return (*MSK_putmaxnumanz_ptr)(task,maxnumanz);
} /* MSKputmaxnumanz */
typedef MSKrescodee MSKAPI MSK_putmaxnumqnz_t(MSKtask_t task,MSKint64t maxnumqnz);
static MSK_putmaxnumqnz_t * MSK_putmaxnumqnz_ptr = (MSK_putmaxnumqnz_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putmaxnumqnz(MSKtask_t task,MSKint64t maxnumqnz) {
  return (*MSK_putmaxnumqnz_ptr)(task,maxnumqnz);
} /* MSKputmaxnumqnz */
typedef MSKrescodee MSKAPI MSK_getmaxnumqnz_t(MSKtask_t task,MSKint32t * maxnumqnz);
static MSK_getmaxnumqnz_t * MSK_getmaxnumqnz_ptr = (MSK_getmaxnumqnz_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getmaxnumqnz(MSKtask_t task,MSKint32t * maxnumqnz) {
  return (*MSK_getmaxnumqnz_ptr)(task,maxnumqnz);
} /* MSKgetmaxnumqnz */
typedef MSKrescodee MSKAPI MSK_getmaxnumqnz64_t(MSKtask_t task,MSKint64t * maxnumqnz);
static MSK_getmaxnumqnz64_t * MSK_getmaxnumqnz64_ptr = (MSK_getmaxnumqnz64_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getmaxnumqnz64(MSKtask_t task,MSKint64t * maxnumqnz) {
  return (*MSK_getmaxnumqnz64_ptr)(task,maxnumqnz);
} /* MSKgetmaxnumqnz64 */
typedef MSKrescodee MSKAPI MSK_putnadouparam_t(MSKtask_t task,const char * paramname,MSKrealt parvalue);
static MSK_putnadouparam_t * MSK_putnadouparam_ptr = (MSK_putnadouparam_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putnadouparam(MSKtask_t task,const char * paramname,MSKrealt parvalue) {
  return (*MSK_putnadouparam_ptr)(task,paramname,parvalue);
} /* MSKputnadouparam */
typedef MSKrescodee MSKAPI MSK_putnaintparam_t(MSKtask_t task,const char * paramname,MSKint32t parvalue);
static MSK_putnaintparam_t * MSK_putnaintparam_ptr = (MSK_putnaintparam_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putnaintparam(MSKtask_t task,const char * paramname,MSKint32t parvalue) {
  return (*MSK_putnaintparam_ptr)(task,paramname,parvalue);
} /* MSKputnaintparam */
typedef MSKrescodee MSKAPI MSK_putnastrparam_t(MSKtask_t task,const char * paramname,const char * parvalue);
static MSK_putnastrparam_t * MSK_putnastrparam_ptr = (MSK_putnastrparam_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putnastrparam(MSKtask_t task,const char * paramname,const char * parvalue) {
  return (*MSK_putnastrparam_ptr)(task,paramname,parvalue);
} /* MSKputnastrparam */
typedef MSKrescodee MSKAPI MSK_putobjname_t(MSKtask_t task,const char * objname);
static MSK_putobjname_t * MSK_putobjname_ptr = (MSK_putobjname_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putobjname(MSKtask_t task,const char * objname) {
  return (*MSK_putobjname_ptr)(task,objname);
} /* MSKputobjname */
typedef MSKrescodee MSKAPI MSK_putparam_t(MSKtask_t task,const char * parname,const char * parvalue);
static MSK_putparam_t * MSK_putparam_ptr = (MSK_putparam_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putparam(MSKtask_t task,const char * parname,const char * parvalue) {
  return (*MSK_putparam_ptr)(task,parname,parvalue);
} /* MSKputparam */
typedef MSKrescodee MSKAPI MSK_putqcon_t(MSKtask_t task,MSKint32t numqcnz,const MSKint32t * qcsubk,const MSKint32t * qcsubi,const MSKint32t * qcsubj,const MSKrealt * qcval);
static MSK_putqcon_t * MSK_putqcon_ptr = (MSK_putqcon_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putqcon(MSKtask_t task,MSKint32t numqcnz,const MSKint32t * qcsubk,const MSKint32t * qcsubi,const MSKint32t * qcsubj,const MSKrealt * qcval) {
  return (*MSK_putqcon_ptr)(task,numqcnz,qcsubk,qcsubi,qcsubj,qcval);
} /* MSKputqcon */
typedef MSKrescodee MSKAPI MSK_putqconk_t(MSKtask_t task,MSKint32t k,MSKint32t numqcnz,const MSKint32t * qcsubi,const MSKint32t * qcsubj,const MSKrealt * qcval);
static MSK_putqconk_t * MSK_putqconk_ptr = (MSK_putqconk_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putqconk(MSKtask_t task,MSKint32t k,MSKint32t numqcnz,const MSKint32t * qcsubi,const MSKint32t * qcsubj,const MSKrealt * qcval) {
  return (*MSK_putqconk_ptr)(task,k,numqcnz,qcsubi,qcsubj,qcval);
} /* MSKputqconk */
typedef MSKrescodee MSKAPI MSK_putqobj_t(MSKtask_t task,MSKint32t numqonz,const MSKint32t * qosubi,const MSKint32t * qosubj,const MSKrealt * qoval);
static MSK_putqobj_t * MSK_putqobj_ptr = (MSK_putqobj_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putqobj(MSKtask_t task,MSKint32t numqonz,const MSKint32t * qosubi,const MSKint32t * qosubj,const MSKrealt * qoval) {
  return (*MSK_putqobj_ptr)(task,numqonz,qosubi,qosubj,qoval);
} /* MSKputqobj */
typedef MSKrescodee MSKAPI MSK_putqobjij_t(MSKtask_t task,MSKint32t i,MSKint32t j,MSKrealt qoij);
static MSK_putqobjij_t * MSK_putqobjij_ptr = (MSK_putqobjij_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putqobjij(MSKtask_t task,MSKint32t i,MSKint32t j,MSKrealt qoij) {
  return (*MSK_putqobjij_ptr)(task,i,j,qoij);
} /* MSKputqobjij */
typedef MSKrescodee MSKAPI MSK_putsolution_t(MSKtask_t task,MSKsoltypee whichsol,const MSKstakeye * skc,const MSKstakeye * skx,const MSKstakeye * skn,const MSKrealt * xc,const MSKrealt * xx,const MSKrealt * y,const MSKrealt * slc,const MSKrealt * suc,const MSKrealt * slx,const MSKrealt * sux,const MSKrealt * snx);
static MSK_putsolution_t * MSK_putsolution_ptr = (MSK_putsolution_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putsolution(MSKtask_t task,MSKsoltypee whichsol,const MSKstakeye * skc,const MSKstakeye * skx,const MSKstakeye * skn,const MSKrealt * xc,const MSKrealt * xx,const MSKrealt * y,const MSKrealt * slc,const MSKrealt * suc,const MSKrealt * slx,const MSKrealt * sux,const MSKrealt * snx) {
  return (*MSK_putsolution_ptr)(task,whichsol,skc,skx,skn,xc,xx,y,slc,suc,slx,sux,snx);
} /* MSKputsolution */
typedef MSKrescodee MSKAPI MSK_putsolutionnew_t(MSKtask_t task,MSKsoltypee whichsol,const MSKstakeye * skc,const MSKstakeye * skx,const MSKstakeye * skn,const MSKrealt * xc,const MSKrealt * xx,const MSKrealt * y,const MSKrealt * slc,const MSKrealt * suc,const MSKrealt * slx,const MSKrealt * sux,const MSKrealt * snx,const MSKrealt * doty);
static MSK_putsolutionnew_t * MSK_putsolutionnew_ptr = (MSK_putsolutionnew_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putsolutionnew(MSKtask_t task,MSKsoltypee whichsol,const MSKstakeye * skc,const MSKstakeye * skx,const MSKstakeye * skn,const MSKrealt * xc,const MSKrealt * xx,const MSKrealt * y,const MSKrealt * slc,const MSKrealt * suc,const MSKrealt * slx,const MSKrealt * sux,const MSKrealt * snx,const MSKrealt * doty) {
  return (*MSK_putsolutionnew_ptr)(task,whichsol,skc,skx,skn,xc,xx,y,slc,suc,slx,sux,snx,doty);
} /* MSKputsolutionnew */
typedef MSKrescodee MSKAPI MSK_putconsolutioni_t(MSKtask_t task,MSKint32t i,MSKsoltypee whichsol,MSKstakeye sk,MSKrealt x,MSKrealt sl,MSKrealt su);
static MSK_putconsolutioni_t * MSK_putconsolutioni_ptr = (MSK_putconsolutioni_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putconsolutioni(MSKtask_t task,MSKint32t i,MSKsoltypee whichsol,MSKstakeye sk,MSKrealt x,MSKrealt sl,MSKrealt su) {
  return (*MSK_putconsolutioni_ptr)(task,i,whichsol,sk,x,sl,su);
} /* MSKputconsolutioni */
typedef MSKrescodee MSKAPI MSK_putvarsolutionj_t(MSKtask_t task,MSKint32t j,MSKsoltypee whichsol,MSKstakeye sk,MSKrealt x,MSKrealt sl,MSKrealt su,MSKrealt sn);
static MSK_putvarsolutionj_t * MSK_putvarsolutionj_ptr = (MSK_putvarsolutionj_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putvarsolutionj(MSKtask_t task,MSKint32t j,MSKsoltypee whichsol,MSKstakeye sk,MSKrealt x,MSKrealt sl,MSKrealt su,MSKrealt sn) {
  return (*MSK_putvarsolutionj_ptr)(task,j,whichsol,sk,x,sl,su,sn);
} /* MSKputvarsolutionj */
typedef MSKrescodee MSKAPI MSK_putsolutionyi_t(MSKtask_t task,MSKint32t i,MSKsoltypee whichsol,MSKrealt y);
static MSK_putsolutionyi_t * MSK_putsolutionyi_ptr = (MSK_putsolutionyi_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putsolutionyi(MSKtask_t task,MSKint32t i,MSKsoltypee whichsol,MSKrealt y) {
  return (*MSK_putsolutionyi_ptr)(task,i,whichsol,y);
} /* MSKputsolutionyi */
typedef MSKrescodee MSKAPI MSK_putstrparam_t(MSKtask_t task,MSKsparame param,const char * parvalue);
static MSK_putstrparam_t * MSK_putstrparam_ptr = (MSK_putstrparam_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putstrparam(MSKtask_t task,MSKsparame param,const char * parvalue) {
  return (*MSK_putstrparam_ptr)(task,param,parvalue);
} /* MSKputstrparam */
typedef MSKrescodee MSKAPI MSK_resetstrparam_t(MSKtask_t task,MSKsparame param);
static MSK_resetstrparam_t * MSK_resetstrparam_ptr = (MSK_resetstrparam_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_resetstrparam(MSKtask_t task,MSKsparame param) {
  return (*MSK_resetstrparam_ptr)(task,param);
} /* MSKresetstrparam */
typedef MSKrescodee MSKAPI MSK_puttaskname_t(MSKtask_t task,const char * taskname);
static MSK_puttaskname_t * MSK_puttaskname_ptr = (MSK_puttaskname_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_puttaskname(MSKtask_t task,const char * taskname) {
  return (*MSK_puttaskname_ptr)(task,taskname);
} /* MSKputtaskname */
typedef MSKrescodee MSKAPI MSK_putvartype_t(MSKtask_t task,MSKint32t j,MSKvariabletypee vartype);
static MSK_putvartype_t * MSK_putvartype_ptr = (MSK_putvartype_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putvartype(MSKtask_t task,MSKint32t j,MSKvariabletypee vartype) {
  return (*MSK_putvartype_ptr)(task,j,vartype);
} /* MSKputvartype */
typedef MSKrescodee MSKAPI MSK_putvartypelist_t(MSKtask_t task,MSKint32t num,const MSKint32t * subj,const MSKvariabletypee * vartype);
static MSK_putvartypelist_t * MSK_putvartypelist_ptr = (MSK_putvartypelist_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putvartypelist(MSKtask_t task,MSKint32t num,const MSKint32t * subj,const MSKvariabletypee * vartype) {
  return (*MSK_putvartypelist_ptr)(task,num,subj,vartype);
} /* MSKputvartypelist */
typedef MSKrescodee MSKAPI MSK_readdata_t(MSKtask_t task,const char * filename);
static MSK_readdata_t * MSK_readdata_ptr = (MSK_readdata_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_readdata(MSKtask_t task,const char * filename) {
  return (*MSK_readdata_ptr)(task,filename);
} /* MSKreaddata */
typedef MSKrescodee MSKAPI MSK_readdatahandle_t(MSKtask_t task,MSKhreadfunc hread,MSKuserhandle_t h,MSKdataformate format,MSKcompresstypee compress,const char * path);
static MSK_readdatahandle_t * MSK_readdatahandle_ptr = (MSK_readdatahandle_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_readdatahandle(MSKtask_t task,MSKhreadfunc hread,MSKuserhandle_t h,MSKdataformate format,MSKcompresstypee compress,const char * path) {
  return (*MSK_readdatahandle_ptr)(task,hread,h,format,compress,path);
} /* MSKreaddatahandle */
typedef MSKrescodee MSKAPI MSK_writedatahandle_t(MSKtask_t task,MSKhwritefunc func,MSKuserhandle_t handle,MSKdataformate format,MSKcompresstypee compress);
static MSK_writedatahandle_t * MSK_writedatahandle_ptr = (MSK_writedatahandle_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_writedatahandle(MSKtask_t task,MSKhwritefunc func,MSKuserhandle_t handle,MSKdataformate format,MSKcompresstypee compress) {
  return (*MSK_writedatahandle_ptr)(task,func,handle,format,compress);
} /* MSKwritedatahandle */
typedef MSKrescodee MSKAPI MSK_readdataformat_t(MSKtask_t task,const char * filename,MSKdataformate format,MSKcompresstypee compress);
static MSK_readdataformat_t * MSK_readdataformat_ptr = (MSK_readdataformat_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_readdataformat(MSKtask_t task,const char * filename,MSKdataformate format,MSKcompresstypee compress) {
  return (*MSK_readdataformat_ptr)(task,filename,format,compress);
} /* MSKreaddataformat */
typedef MSKrescodee MSKAPI MSK_readdataautoformat_t(MSKtask_t task,const char * filename);
static MSK_readdataautoformat_t * MSK_readdataautoformat_ptr = (MSK_readdataautoformat_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_readdataautoformat(MSKtask_t task,const char * filename) {
  return (*MSK_readdataautoformat_ptr)(task,filename);
} /* MSKreaddataautoformat */
typedef MSKrescodee MSKAPI MSK_readparamfile_t(MSKtask_t task,const char * filename);
static MSK_readparamfile_t * MSK_readparamfile_ptr = (MSK_readparamfile_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_readparamfile(MSKtask_t task,const char * filename) {
  return (*MSK_readparamfile_ptr)(task,filename);
} /* MSKreadparamfile */
typedef MSKrescodee MSKAPI MSK_readsolution_t(MSKtask_t task,MSKsoltypee whichsol,const char * filename);
static MSK_readsolution_t * MSK_readsolution_ptr = (MSK_readsolution_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_readsolution(MSKtask_t task,MSKsoltypee whichsol,const char * filename) {
  return (*MSK_readsolution_ptr)(task,whichsol,filename);
} /* MSKreadsolution */
typedef MSKrescodee MSKAPI MSK_readjsonsol_t(MSKtask_t task,const char * filename);
static MSK_readjsonsol_t * MSK_readjsonsol_ptr = (MSK_readjsonsol_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_readjsonsol(MSKtask_t task,const char * filename) {
  return (*MSK_readjsonsol_ptr)(task,filename);
} /* MSKreadjsonsol */
typedef MSKrescodee MSKAPI MSK_readsummary_t(MSKtask_t task,MSKstreamtypee whichstream);
static MSK_readsummary_t * MSK_readsummary_ptr = (MSK_readsummary_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_readsummary(MSKtask_t task,MSKstreamtypee whichstream) {
  return (*MSK_readsummary_ptr)(task,whichstream);
} /* MSKreadsummary */
typedef MSKrescodee MSKAPI MSK_resizetask_t(MSKtask_t task,MSKint32t maxnumcon,MSKint32t maxnumvar,MSKint32t maxnumcone,MSKint64t maxnumanz,MSKint64t maxnumqnz);
static MSK_resizetask_t * MSK_resizetask_ptr = (MSK_resizetask_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_resizetask(MSKtask_t task,MSKint32t maxnumcon,MSKint32t maxnumvar,MSKint32t maxnumcone,MSKint64t maxnumanz,MSKint64t maxnumqnz) {
  return (*MSK_resizetask_ptr)(task,maxnumcon,maxnumvar,maxnumcone,maxnumanz,maxnumqnz);
} /* MSKresizetask */
typedef MSKrescodee MSKAPI MSK_checkmemtask_t(MSKtask_t task,const char * file,MSKint32t line);
static MSK_checkmemtask_t * MSK_checkmemtask_ptr = (MSK_checkmemtask_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_checkmemtask(MSKtask_t task,const char * file,MSKint32t line) {
  return (*MSK_checkmemtask_ptr)(task,file,line);
} /* MSKcheckmemtask */
typedef MSKrescodee MSKAPI MSK_getmemusagetask_t(MSKtask_t task,MSKint64t * meminuse,MSKint64t * maxmemuse);
static MSK_getmemusagetask_t * MSK_getmemusagetask_ptr = (MSK_getmemusagetask_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getmemusagetask(MSKtask_t task,MSKint64t * meminuse,MSKint64t * maxmemuse) {
  return (*MSK_getmemusagetask_ptr)(task,meminuse,maxmemuse);
} /* MSKgetmemusagetask */
typedef MSKrescodee MSKAPI MSK_resetparameters_t(MSKtask_t task);
static MSK_resetparameters_t * MSK_resetparameters_ptr = (MSK_resetparameters_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_resetparameters(MSKtask_t task) {
  return (*MSK_resetparameters_ptr)(task);
} /* MSKresetparameters */
typedef MSKrescodee MSKAPI MSK_sktostr_t(MSKtask_t task,MSKstakeye sk,char * str);
static MSK_sktostr_t * MSK_sktostr_ptr = (MSK_sktostr_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_sktostr(MSKtask_t task,MSKstakeye sk,char * str) {
  return (*MSK_sktostr_ptr)(task,sk,str);
} /* MSKsktostr */
typedef MSKrescodee MSKAPI MSK_solstatostr_t(MSKtask_t task,MSKsolstae solutionsta,char * str);
static MSK_solstatostr_t * MSK_solstatostr_ptr = (MSK_solstatostr_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_solstatostr(MSKtask_t task,MSKsolstae solutionsta,char * str) {
  return (*MSK_solstatostr_ptr)(task,solutionsta,str);
} /* MSKsolstatostr */
typedef MSKrescodee MSKAPI MSK_solutiondef_t(MSKtask_t task,MSKsoltypee whichsol,MSKbooleant * isdef);
static MSK_solutiondef_t * MSK_solutiondef_ptr = (MSK_solutiondef_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_solutiondef(MSKtask_t task,MSKsoltypee whichsol,MSKbooleant * isdef) {
  return (*MSK_solutiondef_ptr)(task,whichsol,isdef);
} /* MSKsolutiondef */
typedef MSKrescodee MSKAPI MSK_deletesolution_t(MSKtask_t task,MSKsoltypee whichsol);
static MSK_deletesolution_t * MSK_deletesolution_ptr = (MSK_deletesolution_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_deletesolution(MSKtask_t task,MSKsoltypee whichsol) {
  return (*MSK_deletesolution_ptr)(task,whichsol);
} /* MSKdeletesolution */
typedef MSKrescodee MSKAPI MSK_writestat_t(MSKtask_t task,const char * filename);
static MSK_writestat_t * MSK_writestat_ptr = (MSK_writestat_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_writestat(MSKtask_t task,const char * filename) {
  return (*MSK_writestat_ptr)(task,filename);
} /* MSKwritestat */
typedef MSKrescodee MSKAPI MSK_writesolutionstat_t(MSKtask_t task,const char * filename);
static MSK_writesolutionstat_t * MSK_writesolutionstat_ptr = (MSK_writesolutionstat_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_writesolutionstat(MSKtask_t task,const char * filename) {
  return (*MSK_writesolutionstat_ptr)(task,filename);
} /* MSKwritesolutionstat */
typedef MSKrescodee MSKAPI MSK_onesolutionsummary_t(MSKtask_t task,MSKstreamtypee whichstream,MSKsoltypee whichsol);
static MSK_onesolutionsummary_t * MSK_onesolutionsummary_ptr = (MSK_onesolutionsummary_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_onesolutionsummary(MSKtask_t task,MSKstreamtypee whichstream,MSKsoltypee whichsol) {
  return (*MSK_onesolutionsummary_ptr)(task,whichstream,whichsol);
} /* MSKonesolutionsummary */
typedef MSKrescodee MSKAPI MSK_solutionsummary_t(MSKtask_t task,MSKstreamtypee whichstream);
static MSK_solutionsummary_t * MSK_solutionsummary_ptr = (MSK_solutionsummary_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_solutionsummary(MSKtask_t task,MSKstreamtypee whichstream) {
  return (*MSK_solutionsummary_ptr)(task,whichstream);
} /* MSKsolutionsummary */
typedef MSKrescodee MSKAPI MSK_updatesolutioninfo_t(MSKtask_t task,MSKsoltypee whichsol);
static MSK_updatesolutioninfo_t * MSK_updatesolutioninfo_ptr = (MSK_updatesolutioninfo_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_updatesolutioninfo(MSKtask_t task,MSKsoltypee whichsol) {
  return (*MSK_updatesolutioninfo_ptr)(task,whichsol);
} /* MSKupdatesolutioninfo */
typedef MSKrescodee MSKAPI MSK_optimizersummary_t(MSKtask_t task,MSKstreamtypee whichstream);
static MSK_optimizersummary_t * MSK_optimizersummary_ptr = (MSK_optimizersummary_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_optimizersummary(MSKtask_t task,MSKstreamtypee whichstream) {
  return (*MSK_optimizersummary_ptr)(task,whichstream);
} /* MSKoptimizersummary */
typedef MSKrescodee MSKAPI MSK_strtoconetype_t(MSKtask_t task,const char * str,MSKconetypee * conetype);
static MSK_strtoconetype_t * MSK_strtoconetype_ptr = (MSK_strtoconetype_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_strtoconetype(MSKtask_t task,const char * str,MSKconetypee * conetype) {
  return (*MSK_strtoconetype_ptr)(task,str,conetype);
} /* MSKstrtoconetype */
typedef MSKrescodee MSKAPI MSK_strtosk_t(MSKtask_t task,const char * str,MSKstakeye * sk);
static MSK_strtosk_t * MSK_strtosk_ptr = (MSK_strtosk_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_strtosk(MSKtask_t task,const char * str,MSKstakeye * sk) {
  return (*MSK_strtosk_ptr)(task,str,sk);
} /* MSKstrtosk */
typedef MSKrescodee MSKAPI MSK_whichparam_t(MSKtask_t task,const char * parname,MSKparametertypee * partype,MSKint32t * param);
static MSK_whichparam_t * MSK_whichparam_ptr = (MSK_whichparam_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_whichparam(MSKtask_t task,const char * parname,MSKparametertypee * partype,MSKint32t * param) {
  return (*MSK_whichparam_ptr)(task,parname,partype,param);
} /* MSKwhichparam */
typedef MSKrescodee MSKAPI MSK_writedata_t(MSKtask_t task,const char * filename);
static MSK_writedata_t * MSK_writedata_ptr = (MSK_writedata_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_writedata(MSKtask_t task,const char * filename) {
  return (*MSK_writedata_ptr)(task,filename);
} /* MSKwritedata */
typedef MSKrescodee MSKAPI MSK_writetask_t(MSKtask_t task,const char * filename);
static MSK_writetask_t * MSK_writetask_ptr = (MSK_writetask_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_writetask(MSKtask_t task,const char * filename) {
  return (*MSK_writetask_ptr)(task,filename);
} /* MSKwritetask */
typedef MSKrescodee MSKAPI MSK_writebsolution_t(MSKtask_t task,const char * filename,MSKcompresstypee compress);
static MSK_writebsolution_t * MSK_writebsolution_ptr = (MSK_writebsolution_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_writebsolution(MSKtask_t task,const char * filename,MSKcompresstypee compress) {
  return (*MSK_writebsolution_ptr)(task,filename,compress);
} /* MSKwritebsolution */
typedef MSKrescodee MSKAPI MSK_writebsolutionhandle_t(MSKtask_t task,MSKhwritefunc func,MSKuserhandle_t handle,MSKcompresstypee compress);
static MSK_writebsolutionhandle_t * MSK_writebsolutionhandle_ptr = (MSK_writebsolutionhandle_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_writebsolutionhandle(MSKtask_t task,MSKhwritefunc func,MSKuserhandle_t handle,MSKcompresstypee compress) {
  return (*MSK_writebsolutionhandle_ptr)(task,func,handle,compress);
} /* MSKwritebsolutionhandle */
typedef MSKrescodee MSKAPI MSK_readbupdate_t(MSKtask_t task,const char * filename,MSKcompresstypee compress);
static MSK_readbupdate_t * MSK_readbupdate_ptr = (MSK_readbupdate_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_readbupdate(MSKtask_t task,const char * filename,MSKcompresstypee compress) {
  return (*MSK_readbupdate_ptr)(task,filename,compress);
} /* MSKreadbupdate */
typedef MSKrescodee MSKAPI MSK_writeb_t(MSKtask_t task,const char * filename,MSKcompresstypee compress);
static MSK_writeb_t * MSK_writeb_ptr = (MSK_writeb_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_writeb(MSKtask_t task,const char * filename,MSKcompresstypee compress) {
  return (*MSK_writeb_ptr)(task,filename,compress);
} /* MSKwriteb */
typedef MSKrescodee MSKAPI MSK_readbupdatehandle_t(MSKtask_t task,MSKhreadfunc hread,MSKuserhandle_t handle,MSKcompresstypee compress);
static MSK_readbupdatehandle_t * MSK_readbupdatehandle_ptr = (MSK_readbupdatehandle_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_readbupdatehandle(MSKtask_t task,MSKhreadfunc hread,MSKuserhandle_t handle,MSKcompresstypee compress) {
  return (*MSK_readbupdatehandle_ptr)(task,hread,handle,compress);
} /* MSKreadbupdatehandle */
typedef MSKrescodee MSKAPI MSK_readbsolution_t(MSKtask_t task,const char * filename,MSKcompresstypee compress);
static MSK_readbsolution_t * MSK_readbsolution_ptr = (MSK_readbsolution_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_readbsolution(MSKtask_t task,const char * filename,MSKcompresstypee compress) {
  return (*MSK_readbsolution_ptr)(task,filename,compress);
} /* MSKreadbsolution */
typedef MSKrescodee MSKAPI MSK_writesolutionfile_t(MSKtask_t task,const char * filename);
static MSK_writesolutionfile_t * MSK_writesolutionfile_ptr = (MSK_writesolutionfile_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_writesolutionfile(MSKtask_t task,const char * filename) {
  return (*MSK_writesolutionfile_ptr)(task,filename);
} /* MSKwritesolutionfile */
typedef MSKrescodee MSKAPI MSK_readsolutionfile_t(MSKtask_t task,const char * filename);
static MSK_readsolutionfile_t * MSK_readsolutionfile_ptr = (MSK_readsolutionfile_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_readsolutionfile(MSKtask_t task,const char * filename) {
  return (*MSK_readsolutionfile_ptr)(task,filename);
} /* MSKreadsolutionfile */
typedef MSKrescodee MSKAPI MSK_readtask_t(MSKtask_t task,const char * filename);
static MSK_readtask_t * MSK_readtask_ptr = (MSK_readtask_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_readtask(MSKtask_t task,const char * filename) {
  return (*MSK_readtask_ptr)(task,filename);
} /* MSKreadtask */
typedef MSKrescodee MSKAPI MSK_readopfstring_t(MSKtask_t task,const char * data);
static MSK_readopfstring_t * MSK_readopfstring_ptr = (MSK_readopfstring_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_readopfstring(MSKtask_t task,const char * data) {
  return (*MSK_readopfstring_ptr)(task,data);
} /* MSKreadopfstring */
typedef MSKrescodee MSKAPI MSK_readlpstring_t(MSKtask_t task,const char * data);
static MSK_readlpstring_t * MSK_readlpstring_ptr = (MSK_readlpstring_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_readlpstring(MSKtask_t task,const char * data) {
  return (*MSK_readlpstring_ptr)(task,data);
} /* MSKreadlpstring */
typedef MSKrescodee MSKAPI MSK_readjsonstring_t(MSKtask_t task,const char * data);
static MSK_readjsonstring_t * MSK_readjsonstring_ptr = (MSK_readjsonstring_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_readjsonstring(MSKtask_t task,const char * data) {
  return (*MSK_readjsonstring_ptr)(task,data);
} /* MSKreadjsonstring */
typedef MSKrescodee MSKAPI MSK_readptfstring_t(MSKtask_t task,const char * data);
static MSK_readptfstring_t * MSK_readptfstring_ptr = (MSK_readptfstring_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_readptfstring(MSKtask_t task,const char * data) {
  return (*MSK_readptfstring_ptr)(task,data);
} /* MSKreadptfstring */
typedef MSKrescodee MSKAPI MSK_writeparamfile_t(MSKtask_t task,const char * filename);
static MSK_writeparamfile_t * MSK_writeparamfile_ptr = (MSK_writeparamfile_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_writeparamfile(MSKtask_t task,const char * filename) {
  return (*MSK_writeparamfile_ptr)(task,filename);
} /* MSKwriteparamfile */
typedef MSKrescodee MSKAPI MSK_getinfeasiblesubproblem_t(MSKtask_t task,MSKsoltypee whichsol,MSKtask_t * inftask);
static MSK_getinfeasiblesubproblem_t * MSK_getinfeasiblesubproblem_ptr = (MSK_getinfeasiblesubproblem_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getinfeasiblesubproblem(MSKtask_t task,MSKsoltypee whichsol,MSKtask_t * inftask) {
  return (*MSK_getinfeasiblesubproblem_ptr)(task,whichsol,inftask);
} /* MSKgetinfeasiblesubproblem */
typedef MSKrescodee MSKAPI MSK_getdualproblem_t(MSKtask_t task,MSKtask_t * dualtask);
static MSK_getdualproblem_t * MSK_getdualproblem_ptr = (MSK_getdualproblem_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getdualproblem(MSKtask_t task,MSKtask_t * dualtask) {
  return (*MSK_getdualproblem_ptr)(task,dualtask);
} /* MSKgetdualproblem */
typedef MSKrescodee MSKAPI MSK_getfixedproblem_t(MSKtask_t task,MSKtask_t * fixedtask);
static MSK_getfixedproblem_t * MSK_getfixedproblem_ptr = (MSK_getfixedproblem_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getfixedproblem(MSKtask_t task,MSKtask_t * fixedtask) {
  return (*MSK_getfixedproblem_ptr)(task,fixedtask);
} /* MSKgetfixedproblem */
typedef MSKrescodee MSKAPI MSK_tofixedproblem_t(MSKtask_t task);
static MSK_tofixedproblem_t * MSK_tofixedproblem_ptr = (MSK_tofixedproblem_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_tofixedproblem(MSKtask_t task) {
  return (*MSK_tofixedproblem_ptr)(task);
} /* MSKtofixedproblem */
typedef MSKrescodee MSKAPI MSK_writesolution_t(MSKtask_t task,MSKsoltypee whichsol,const char * filename);
static MSK_writesolution_t * MSK_writesolution_ptr = (MSK_writesolution_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_writesolution(MSKtask_t task,MSKsoltypee whichsol,const char * filename) {
  return (*MSK_writesolution_ptr)(task,whichsol,filename);
} /* MSKwritesolution */
typedef MSKrescodee MSKAPI MSK_writejsonsol_t(MSKtask_t task,const char * filename);
static MSK_writejsonsol_t * MSK_writejsonsol_ptr = (MSK_writejsonsol_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_writejsonsol(MSKtask_t task,const char * filename) {
  return (*MSK_writejsonsol_ptr)(task,filename);
} /* MSKwritejsonsol */
typedef MSKrescodee MSKAPI MSK_primalsensitivity_t(MSKtask_t task,MSKint32t numi,const MSKint32t * subi,const MSKmarke * marki,MSKint32t numj,const MSKint32t * subj,const MSKmarke * markj,MSKrealt * leftpricei,MSKrealt * rightpricei,MSKrealt * leftrangei,MSKrealt * rightrangei,MSKrealt * leftpricej,MSKrealt * rightpricej,MSKrealt * leftrangej,MSKrealt * rightrangej);
static MSK_primalsensitivity_t * MSK_primalsensitivity_ptr = (MSK_primalsensitivity_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_primalsensitivity(MSKtask_t task,MSKint32t numi,const MSKint32t * subi,const MSKmarke * marki,MSKint32t numj,const MSKint32t * subj,const MSKmarke * markj,MSKrealt * leftpricei,MSKrealt * rightpricei,MSKrealt * leftrangei,MSKrealt * rightrangei,MSKrealt * leftpricej,MSKrealt * rightpricej,MSKrealt * leftrangej,MSKrealt * rightrangej) {
  return (*MSK_primalsensitivity_ptr)(task,numi,subi,marki,numj,subj,markj,leftpricei,rightpricei,leftrangei,rightrangei,leftpricej,rightpricej,leftrangej,rightrangej);
} /* MSKprimalsensitivity */
typedef MSKrescodee MSKAPI MSK_sensitivityreport_t(MSKtask_t task,MSKstreamtypee whichstream);
static MSK_sensitivityreport_t * MSK_sensitivityreport_ptr = (MSK_sensitivityreport_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_sensitivityreport(MSKtask_t task,MSKstreamtypee whichstream) {
  return (*MSK_sensitivityreport_ptr)(task,whichstream);
} /* MSKsensitivityreport */
typedef MSKrescodee MSKAPI MSK_dualsensitivity_t(MSKtask_t task,MSKint32t numj,const MSKint32t * subj,MSKrealt * leftpricej,MSKrealt * rightpricej,MSKrealt * leftrangej,MSKrealt * rightrangej);
static MSK_dualsensitivity_t * MSK_dualsensitivity_ptr = (MSK_dualsensitivity_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_dualsensitivity(MSKtask_t task,MSKint32t numj,const MSKint32t * subj,MSKrealt * leftpricej,MSKrealt * rightpricej,MSKrealt * leftrangej,MSKrealt * rightrangej) {
  return (*MSK_dualsensitivity_ptr)(task,numj,subj,leftpricej,rightpricej,leftrangej,rightrangej);
} /* MSKdualsensitivity */
typedef MSKrescodee MSKAPI MSK_getlasterror_t(MSKtask_t task,MSKrescodee * lastrescode,MSKint32t sizelastmsg,MSKint32t * lastmsglen,char * lastmsg);
static MSK_getlasterror_t * MSK_getlasterror_ptr = (MSK_getlasterror_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getlasterror(MSKtask_t task,MSKrescodee * lastrescode,MSKint32t sizelastmsg,MSKint32t * lastmsglen,char * lastmsg) {
  return (*MSK_getlasterror_ptr)(task,lastrescode,sizelastmsg,lastmsglen,lastmsg);
} /* MSKgetlasterror */
typedef MSKrescodee MSKAPI MSK_getlasterror64_t(MSKtask_t task,MSKrescodee * lastrescode,MSKint64t sizelastmsg,MSKint64t * lastmsglen,char * lastmsg);
static MSK_getlasterror64_t * MSK_getlasterror64_ptr = (MSK_getlasterror64_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getlasterror64(MSKtask_t task,MSKrescodee * lastrescode,MSKint64t sizelastmsg,MSKint64t * lastmsglen,char * lastmsg) {
  return (*MSK_getlasterror64_ptr)(task,lastrescode,sizelastmsg,lastmsglen,lastmsg);
} /* MSKgetlasterror64 */
typedef MSKrescodee MSKAPI MSK_writetasksolverresult_file_t(MSKtask_t task,const char * filename,MSKcompresstypee compress);
static MSK_writetasksolverresult_file_t * MSK_writetasksolverresult_file_ptr = (MSK_writetasksolverresult_file_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_writetasksolverresult_file(MSKtask_t task,const char * filename,MSKcompresstypee compress) {
  return (*MSK_writetasksolverresult_file_ptr)(task,filename,compress);
} /* MSKwritetasksolverresult_file */
typedef MSKrescodee MSKAPI MSK_optimizermt_t(MSKtask_t task,const char * address,const char * accesstoken,MSKrescodee * trmcode);
static MSK_optimizermt_t * MSK_optimizermt_ptr = (MSK_optimizermt_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_optimizermt(MSKtask_t task,const char * address,const char * accesstoken,MSKrescodee * trmcode) {
  return (*MSK_optimizermt_ptr)(task,address,accesstoken,trmcode);
} /* MSKoptimizermt */
typedef MSKrescodee MSKAPI MSK_optimizecb_t(MSKtask_t task,const char * access_token,const char * addr,MSKuserhandle_t h,MSKhreadfunc hread,MSKhwritefunc hwrite,MSKrescodee * trmcode);
static MSK_optimizecb_t * MSK_optimizecb_ptr = (MSK_optimizecb_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_optimizecb(MSKtask_t task,const char * access_token,const char * addr,MSKuserhandle_t h,MSKhreadfunc hread,MSKhwritefunc hwrite,MSKrescodee * trmcode) {
  return (*MSK_optimizecb_ptr)(task,access_token,addr,h,hread,hwrite,trmcode);
} /* MSKoptimizecb */
typedef MSKrescodee MSKAPI MSK_asyncoptimizecb_t(MSKtask_t task,const char * access_token,const char * address,MSKuserhandle_t h,MSKhreadfunc hread,MSKhwritefunc hwrite,char * token);
static MSK_asyncoptimizecb_t * MSK_asyncoptimizecb_ptr = (MSK_asyncoptimizecb_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_asyncoptimizecb(MSKtask_t task,const char * access_token,const char * address,MSKuserhandle_t h,MSKhreadfunc hread,MSKhwritefunc hwrite,char * token) {
  return (*MSK_asyncoptimizecb_ptr)(task,access_token,address,h,hread,hwrite,token);
} /* MSKasyncoptimizecb */
typedef MSKrescodee MSKAPI MSK_asyncgetlogcb_t(MSKtask_t task,const char * access_token,const char * addr,MSKuserhandle_t h,MSKhreadfunc hread,MSKhwritefunc hwrite,const char * token);
static MSK_asyncgetlogcb_t * MSK_asyncgetlogcb_ptr = (MSK_asyncgetlogcb_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_asyncgetlogcb(MSKtask_t task,const char * access_token,const char * addr,MSKuserhandle_t h,MSKhreadfunc hread,MSKhwritefunc hwrite,const char * token) {
  return (*MSK_asyncgetlogcb_ptr)(task,access_token,addr,h,hread,hwrite,token);
} /* MSKasyncgetlogcb */
typedef MSKrescodee MSKAPI MSK_asyncstopcb_t(MSKtask_t task,const char * access_token,const char * addr,MSKuserhandle_t h,MSKhreadfunc hread,MSKhwritefunc hwrite,const char * token);
static MSK_asyncstopcb_t * MSK_asyncstopcb_ptr = (MSK_asyncstopcb_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_asyncstopcb(MSKtask_t task,const char * access_token,const char * addr,MSKuserhandle_t h,MSKhreadfunc hread,MSKhwritefunc hwrite,const char * token) {
  return (*MSK_asyncstopcb_ptr)(task,access_token,addr,h,hread,hwrite,token);
} /* MSKasyncstopcb */
typedef MSKrescodee MSKAPI MSK_asyncgetresultcb_t(MSKtask_t task,const char * access_token,const char * addr,MSKuserhandle_t h,MSKhreadfunc hread,MSKhwritefunc hwrite,const char * token,MSKbooleant * respavailable,MSKrescodee * resp,MSKrescodee * trm);
static MSK_asyncgetresultcb_t * MSK_asyncgetresultcb_ptr = (MSK_asyncgetresultcb_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_asyncgetresultcb(MSKtask_t task,const char * access_token,const char * addr,MSKuserhandle_t h,MSKhreadfunc hread,MSKhwritefunc hwrite,const char * token,MSKbooleant * respavailable,MSKrescodee * resp,MSKrescodee * trm) {
  return (*MSK_asyncgetresultcb_ptr)(task,access_token,addr,h,hread,hwrite,token,respavailable,resp,trm);
} /* MSKasyncgetresultcb */
typedef MSKrescodee MSKAPI MSK_asyncpollcb_t(MSKtask_t task,const char * access_token,const char * addr,MSKuserhandle_t h,MSKhreadfunc hread,MSKhwritefunc hwrite,const char * token,MSKbooleant * respavailable,MSKrescodee * resp,MSKrescodee * trm);
static MSK_asyncpollcb_t * MSK_asyncpollcb_ptr = (MSK_asyncpollcb_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_asyncpollcb(MSKtask_t task,const char * access_token,const char * addr,MSKuserhandle_t h,MSKhreadfunc hread,MSKhwritefunc hwrite,const char * token,MSKbooleant * respavailable,MSKrescodee * resp,MSKrescodee * trm) {
  return (*MSK_asyncpollcb_ptr)(task,access_token,addr,h,hread,hwrite,token,respavailable,resp,trm);
} /* MSKasyncpollcb */
typedef MSKrescodee MSKAPI MSK_asyncoptimize_t(MSKtask_t task,const char * address,const char * accesstoken,char * token);
static MSK_asyncoptimize_t * MSK_asyncoptimize_ptr = (MSK_asyncoptimize_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_asyncoptimize(MSKtask_t task,const char * address,const char * accesstoken,char * token) {
  return (*MSK_asyncoptimize_ptr)(task,address,accesstoken,token);
} /* MSKasyncoptimize */
typedef MSKrescodee MSKAPI MSK_asyncgetlog_t(MSKtask_t task,const char * addr,const char * accesstoken,const char * token);
static MSK_asyncgetlog_t * MSK_asyncgetlog_ptr = (MSK_asyncgetlog_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_asyncgetlog(MSKtask_t task,const char * addr,const char * accesstoken,const char * token) {
  return (*MSK_asyncgetlog_ptr)(task,addr,accesstoken,token);
} /* MSKasyncgetlog */
typedef MSKrescodee MSKAPI MSK_asyncstop_t(MSKtask_t task,const char * address,const char * accesstoken,const char * token);
static MSK_asyncstop_t * MSK_asyncstop_ptr = (MSK_asyncstop_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_asyncstop(MSKtask_t task,const char * address,const char * accesstoken,const char * token) {
  return (*MSK_asyncstop_ptr)(task,address,accesstoken,token);
} /* MSKasyncstop */
typedef MSKrescodee MSKAPI MSK_asyncpoll_t(MSKtask_t task,const char * address,const char * accesstoken,const char * token,MSKbooleant * respavailable,MSKrescodee * resp,MSKrescodee * trm);
static MSK_asyncpoll_t * MSK_asyncpoll_ptr = (MSK_asyncpoll_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_asyncpoll(MSKtask_t task,const char * address,const char * accesstoken,const char * token,MSKbooleant * respavailable,MSKrescodee * resp,MSKrescodee * trm) {
  return (*MSK_asyncpoll_ptr)(task,address,accesstoken,token,respavailable,resp,trm);
} /* MSKasyncpoll */
typedef MSKrescodee MSKAPI MSK_asyncgetresult_t(MSKtask_t task,const char * address,const char * accesstoken,const char * token,MSKbooleant * respavailable,MSKrescodee * resp,MSKrescodee * trm);
static MSK_asyncgetresult_t * MSK_asyncgetresult_ptr = (MSK_asyncgetresult_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_asyncgetresult(MSKtask_t task,const char * address,const char * accesstoken,const char * token,MSKbooleant * respavailable,MSKrescodee * resp,MSKrescodee * trm) {
  return (*MSK_asyncgetresult_ptr)(task,address,accesstoken,token,respavailable,resp,trm);
} /* MSKasyncgetresult */
typedef MSKrescodee MSKAPI MSK_putoptserverhost_t(MSKtask_t task,const char * host);
static MSK_putoptserverhost_t * MSK_putoptserverhost_ptr = (MSK_putoptserverhost_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putoptserverhost(MSKtask_t task,const char * host) {
  return (*MSK_putoptserverhost_ptr)(task,host);
} /* MSKputoptserverhost */
typedef MSKrescodee MSKAPI MSK_optimizebatch_t(MSKenv_t env,MSKbooleant israce,MSKrealt maxtime,MSKint32t numthreads,MSKint64t numtask,const MSKtask_t * task,MSKrescodee * trmcode,MSKrescodee * rcode);
static MSK_optimizebatch_t * MSK_optimizebatch_ptr = (MSK_optimizebatch_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_optimizebatch(MSKenv_t env,MSKbooleant israce,MSKrealt maxtime,MSKint32t numthreads,MSKint64t numtask,const MSKtask_t * task,MSKrescodee * trmcode,MSKrescodee * rcode) {
  return (*MSK_optimizebatch_ptr)(env,israce,maxtime,numthreads,numtask,task,trmcode,rcode);
} /* MSKoptimizebatch */
typedef MSKrescodee MSKAPI MSK_callbackcodetostr_t(MSKcallbackcodee code,char * callbackcodestr);
static MSK_callbackcodetostr_t * MSK_callbackcodetostr_ptr = (MSK_callbackcodetostr_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_callbackcodetostr(MSKcallbackcodee code,char * callbackcodestr) {
  return (*MSK_callbackcodetostr_ptr)(code,callbackcodestr);
} /* MSKcallbackcodetostr */
typedef MSKbooleant MSKAPI MSK_isinfinity_t(MSKrealt value);
static MSK_isinfinity_t * MSK_isinfinity_ptr = (MSK_isinfinity_t*) default_ret_false;
MSKbooleant MSKAPI MSK_isinfinity(MSKrealt value) {
  return (*MSK_isinfinity_ptr)(value);
} /* MSKisinfinity */
typedef MSKrescodee MSKAPI MSK_enablegarcolenv_t(MSKenv_t env);
static MSK_enablegarcolenv_t * MSK_enablegarcolenv_ptr = (MSK_enablegarcolenv_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_enablegarcolenv(MSKenv_t env) {
  return (*MSK_enablegarcolenv_ptr)(env);
} /* MSKenablegarcolenv */
typedef MSKrescodee MSKAPI MSK_makeenvdebug_t(MSKenv_t * env,MSKint64t maxnumalloc,const char * dbgfile);
static MSK_makeenvdebug_t * MSK_makeenvdebug_ptr = (MSK_makeenvdebug_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_makeenvdebug(MSKenv_t * env,MSKint64t maxnumalloc,const char * dbgfile) {
  return (*MSK_makeenvdebug_ptr)(env,maxnumalloc,dbgfile);
} /* MSKmakeenvdebug */
typedef int MSKAPI MSK_unittestmain_t(int cuintinterface,const char * fileroot,char * debuglogstring,int loglevel,char * suite,char * test);
static MSK_unittestmain_t * MSK_unittestmain_ptr = (MSK_unittestmain_t*) default_ret_0i32;
int MSKAPI MSK_unittestmain(int cuintinterface,const char * fileroot,char * debuglogstring,int loglevel,char * suite,char * test) {
  return (*MSK_unittestmain_ptr)(cuintinterface,fileroot,debuglogstring,loglevel,suite,test);
} /* MSKunittestmain */
typedef int MSKAPI MSK_runsimplexfromfile_t(const char * filename);
static MSK_runsimplexfromfile_t * MSK_runsimplexfromfile_ptr = (MSK_runsimplexfromfile_t*) default_ret_0i32;
int MSKAPI MSK_runsimplexfromfile(const char * filename) {
  return (*MSK_runsimplexfromfile_ptr)(filename);
} /* MSKrunsimplexfromfile */
typedef MSKrescodee MSKAPI MSK_globalenvinitialize_t(MSKint64t maxnumalloc,const char * dbgfile);
static MSK_globalenvinitialize_t * MSK_globalenvinitialize_ptr = (MSK_globalenvinitialize_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_globalenvinitialize(MSKint64t maxnumalloc,const char * dbgfile) {
  return (*MSK_globalenvinitialize_ptr)(maxnumalloc,dbgfile);
} /* MSKglobalenvinitialize */
typedef MSKrescodee MSKAPI MSK_globalenvfinalize_t();
static MSK_globalenvfinalize_t * MSK_globalenvfinalize_ptr = (MSK_globalenvfinalize_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_globalenvfinalize() {
  return (*MSK_globalenvfinalize_ptr)();
} /* MSKglobalenvfinalize */
typedef MSKrescodee MSKAPI MSK_checkoutlicense_t(MSKenv_t env,MSKfeaturee feature);
static MSK_checkoutlicense_t * MSK_checkoutlicense_ptr = (MSK_checkoutlicense_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_checkoutlicense(MSKenv_t env,MSKfeaturee feature) {
  return (*MSK_checkoutlicense_ptr)(env,feature);
} /* MSKcheckoutlicense */
typedef MSKrescodee MSKAPI MSK_checkinlicense_t(MSKenv_t env,MSKfeaturee feature);
static MSK_checkinlicense_t * MSK_checkinlicense_ptr = (MSK_checkinlicense_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_checkinlicense(MSKenv_t env,MSKfeaturee feature) {
  return (*MSK_checkinlicense_ptr)(env,feature);
} /* MSKcheckinlicense */
typedef MSKrescodee MSKAPI MSK_checkinall_t(MSKenv_t env);
static MSK_checkinall_t * MSK_checkinall_ptr = (MSK_checkinall_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_checkinall(MSKenv_t env) {
  return (*MSK_checkinall_ptr)(env);
} /* MSKcheckinall */
typedef MSKrescodee MSKAPI MSK_expirylicenses_t(MSKenv_t env,MSKint64t * expiry);
static MSK_expirylicenses_t * MSK_expirylicenses_ptr = (MSK_expirylicenses_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_expirylicenses(MSKenv_t env,MSKint64t * expiry) {
  return (*MSK_expirylicenses_ptr)(env,expiry);
} /* MSKexpirylicenses */
typedef MSKrescodee MSKAPI MSK_resetexpirylicenses_t(MSKenv_t env);
static MSK_resetexpirylicenses_t * MSK_resetexpirylicenses_ptr = (MSK_resetexpirylicenses_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_resetexpirylicenses(MSKenv_t env) {
  return (*MSK_resetexpirylicenses_ptr)(env);
} /* MSKresetexpirylicenses */
typedef MSKrescodee MSKAPI MSK_getbuildinfo_t(char * buildstate,char * builddate);
static MSK_getbuildinfo_t * MSK_getbuildinfo_ptr = (MSK_getbuildinfo_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getbuildinfo(char * buildstate,char * builddate) {
  return (*MSK_getbuildinfo_ptr)(buildstate,builddate);
} /* MSKgetbuildinfo */
typedef MSKrescodee MSKAPI MSK_getresponseclass_t(MSKrescodee r,MSKrescodetypee * rc);
static MSK_getresponseclass_t * MSK_getresponseclass_ptr = (MSK_getresponseclass_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getresponseclass(MSKrescodee r,MSKrescodetypee * rc) {
  return (*MSK_getresponseclass_ptr)(r,rc);
} /* MSKgetresponseclass */
typedef void * MSKAPI MSK_callocenv_t(MSKenv_t env,size_t number,size_t size);
static MSK_callocenv_t * MSK_callocenv_ptr = (MSK_callocenv_t*) default_ret_ptr;
void * MSKAPI MSK_callocenv(MSKenv_t env,size_t number,size_t size) {
  return (*MSK_callocenv_ptr)(env,number,size);
} /* MSKcallocenv */
typedef void * MSKAPI MSK_callocdbgenv_t(MSKenv_t env,size_t number,size_t size,const char * file,unsigned line);
static MSK_callocdbgenv_t * MSK_callocdbgenv_ptr = (MSK_callocdbgenv_t*) default_ret_ptr;
void * MSKAPI MSK_callocdbgenv(MSKenv_t env,size_t number,size_t size,const char * file,unsigned line) {
  return (*MSK_callocdbgenv_ptr)(env,number,size,file,line);
} /* MSKcallocdbgenv */
typedef MSKrescodee MSKAPI MSK_deleteenv_t(MSKenv_t * env);
static MSK_deleteenv_t * MSK_deleteenv_ptr = (MSK_deleteenv_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_deleteenv(MSKenv_t * env) {
  return (*MSK_deleteenv_ptr)(env);
} /* MSKdeleteenv */
typedef MSKrescodee MSKAPI MSK_echointro_t(MSKenv_t env,MSKint32t longver);
static MSK_echointro_t * MSK_echointro_ptr = (MSK_echointro_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_echointro(MSKenv_t env,MSKint32t longver) {
  return (*MSK_echointro_ptr)(env,longver);
} /* MSKechointro */
typedef void MSKAPI MSK_freeenv_t(MSKenv_t env,void * buffer);
static MSK_freeenv_t * MSK_freeenv_ptr = (MSK_freeenv_t*) default_ret_void;
void MSKAPI MSK_freeenv(MSKenv_t env,void * buffer) {
  (*MSK_freeenv_ptr)(env,buffer);
} /* MSKfreeenv */
typedef void MSKAPI MSK_freedbgenv_t(MSKenv_t env,void * buffer,const char * file,unsigned line);
static MSK_freedbgenv_t * MSK_freedbgenv_ptr = (MSK_freedbgenv_t*) default_ret_void;
void MSKAPI MSK_freedbgenv(MSKenv_t env,void * buffer,const char * file,unsigned line) {
  (*MSK_freedbgenv_ptr)(env,buffer,file,line);
} /* MSKfreedbgenv */
typedef MSKrescodee MSKAPI MSK_getcodedesc_t(MSKrescodee code,char * symname,char * str);
static MSK_getcodedesc_t * MSK_getcodedesc_ptr = (MSK_getcodedesc_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getcodedesc(MSKrescodee code,char * symname,char * str) {
  return (*MSK_getcodedesc_ptr)(code,symname,str);
} /* MSKgetcodedesc */
typedef MSKrescodee MSKAPI MSK_getsymbcondim_t(MSKenv_t env,MSKint32t * num,size_t * maxlen);
static MSK_getsymbcondim_t * MSK_getsymbcondim_ptr = (MSK_getsymbcondim_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getsymbcondim(MSKenv_t env,MSKint32t * num,size_t * maxlen) {
  return (*MSK_getsymbcondim_ptr)(env,num,maxlen);
} /* MSKgetsymbcondim */
typedef MSKrescodee MSKAPI MSK_rescodetostr_t(MSKrescodee res,char * str);
static MSK_rescodetostr_t * MSK_rescodetostr_ptr = (MSK_rescodetostr_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_rescodetostr(MSKrescodee res,char * str) {
  return (*MSK_rescodetostr_ptr)(res,str);
} /* MSKrescodetostr */
typedef MSKrescodee MSKAPI MSK_iinfitemtostr_t(MSKiinfiteme item,char * str);
static MSK_iinfitemtostr_t * MSK_iinfitemtostr_ptr = (MSK_iinfitemtostr_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_iinfitemtostr(MSKiinfiteme item,char * str) {
  return (*MSK_iinfitemtostr_ptr)(item,str);
} /* MSKiinfitemtostr */
typedef MSKrescodee MSKAPI MSK_dinfitemtostr_t(MSKdinfiteme item,char * str);
static MSK_dinfitemtostr_t * MSK_dinfitemtostr_ptr = (MSK_dinfitemtostr_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_dinfitemtostr(MSKdinfiteme item,char * str) {
  return (*MSK_dinfitemtostr_ptr)(item,str);
} /* MSKdinfitemtostr */
typedef MSKrescodee MSKAPI MSK_liinfitemtostr_t(MSKliinfiteme item,char * str);
static MSK_liinfitemtostr_t * MSK_liinfitemtostr_ptr = (MSK_liinfitemtostr_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_liinfitemtostr(MSKliinfiteme item,char * str) {
  return (*MSK_liinfitemtostr_ptr)(item,str);
} /* MSKliinfitemtostr */
typedef MSKrescodee MSKAPI MSK_getversion_t(MSKint32t * major,MSKint32t * minor,MSKint32t * revision);
static MSK_getversion_t * MSK_getversion_ptr = (MSK_getversion_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_getversion(MSKint32t * major,MSKint32t * minor,MSKint32t * revision) {
  return (*MSK_getversion_ptr)(major,minor,revision);
} /* MSKgetversion */
typedef MSKrescodee MSKAPI MSK_checkversion_t(MSKenv_t env,MSKint32t major,MSKint32t minor,MSKint32t revision);
static MSK_checkversion_t * MSK_checkversion_ptr = (MSK_checkversion_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_checkversion(MSKenv_t env,MSKint32t major,MSKint32t minor,MSKint32t revision) {
  return (*MSK_checkversion_ptr)(env,major,minor,revision);
} /* MSKcheckversion */
typedef MSKrescodee MSKAPI MSK_iparvaltosymnam_t(MSKenv_t env,MSKiparame whichparam,MSKint32t whichvalue,char * symbolicname);
static MSK_iparvaltosymnam_t * MSK_iparvaltosymnam_ptr = (MSK_iparvaltosymnam_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_iparvaltosymnam(MSKenv_t env,MSKiparame whichparam,MSKint32t whichvalue,char * symbolicname) {
  return (*MSK_iparvaltosymnam_ptr)(env,whichparam,whichvalue,symbolicname);
} /* MSKiparvaltosymnam */
typedef MSKrescodee MSKAPI MSK_linkfiletoenvstream_t(MSKenv_t env,MSKstreamtypee whichstream,const char * filename,MSKint32t append);
static MSK_linkfiletoenvstream_t * MSK_linkfiletoenvstream_ptr = (MSK_linkfiletoenvstream_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_linkfiletoenvstream(MSKenv_t env,MSKstreamtypee whichstream,const char * filename,MSKint32t append) {
  return (*MSK_linkfiletoenvstream_ptr)(env,whichstream,filename,append);
} /* MSKlinkfiletoenvstream */
typedef MSKrescodee MSKAPI MSK_linkfunctoenvstream_t(MSKenv_t env,MSKstreamtypee whichstream,MSKuserhandle_t handle,MSKstreamfunc func);
static MSK_linkfunctoenvstream_t * MSK_linkfunctoenvstream_ptr = (MSK_linkfunctoenvstream_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_linkfunctoenvstream(MSKenv_t env,MSKstreamtypee whichstream,MSKuserhandle_t handle,MSKstreamfunc func) {
  return (*MSK_linkfunctoenvstream_ptr)(env,whichstream,handle,func);
} /* MSKlinkfunctoenvstream */
typedef MSKrescodee MSKAPI MSK_unlinkfuncfromenvstream_t(MSKenv_t env,MSKstreamtypee whichstream);
static MSK_unlinkfuncfromenvstream_t * MSK_unlinkfuncfromenvstream_ptr = (MSK_unlinkfuncfromenvstream_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_unlinkfuncfromenvstream(MSKenv_t env,MSKstreamtypee whichstream) {
  return (*MSK_unlinkfuncfromenvstream_ptr)(env,whichstream);
} /* MSKunlinkfuncfromenvstream */
typedef MSKrescodee MSKAPI MSK_makeenv_t(MSKenv_t * env,const char * dbgfile);
static MSK_makeenv_t * MSK_makeenv_ptr = (MSK_makeenv_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_makeenv(MSKenv_t * env,const char * dbgfile) {
  return (*MSK_makeenv_ptr)(env,dbgfile);
} /* MSKmakeenv */
typedef MSKrescodee MSKAPI MSK_putlicensedebug_t(MSKenv_t env,MSKint32t licdebug);
static MSK_putlicensedebug_t * MSK_putlicensedebug_ptr = (MSK_putlicensedebug_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putlicensedebug(MSKenv_t env,MSKint32t licdebug) {
  return (*MSK_putlicensedebug_ptr)(env,licdebug);
} /* MSKputlicensedebug */
typedef MSKrescodee MSKAPI MSK_putlicensecode_t(MSKenv_t env,const MSKint32t * code);
static MSK_putlicensecode_t * MSK_putlicensecode_ptr = (MSK_putlicensecode_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putlicensecode(MSKenv_t env,const MSKint32t * code) {
  return (*MSK_putlicensecode_ptr)(env,code);
} /* MSKputlicensecode */
typedef MSKrescodee MSKAPI MSK_putlicensewait_t(MSKenv_t env,MSKint32t licwait);
static MSK_putlicensewait_t * MSK_putlicensewait_ptr = (MSK_putlicensewait_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putlicensewait(MSKenv_t env,MSKint32t licwait) {
  return (*MSK_putlicensewait_ptr)(env,licwait);
} /* MSKputlicensewait */
typedef MSKrescodee MSKAPI MSK_putlicensepath_t(MSKenv_t env,const char * licensepath);
static MSK_putlicensepath_t * MSK_putlicensepath_ptr = (MSK_putlicensepath_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putlicensepath(MSKenv_t env,const char * licensepath) {
  return (*MSK_putlicensepath_ptr)(env,licensepath);
} /* MSKputlicensepath */
typedef MSKrescodee MSKAPI MSK_maketask_t(MSKenv_t env,MSKint32t maxnumcon,MSKint32t maxnumvar,MSKtask_t * task);
static MSK_maketask_t * MSK_maketask_ptr = (MSK_maketask_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_maketask(MSKenv_t env,MSKint32t maxnumcon,MSKint32t maxnumvar,MSKtask_t * task) {
  return (*MSK_maketask_ptr)(env,maxnumcon,maxnumvar,task);
} /* MSKmaketask */
typedef MSKrescodee MSKAPI MSK_makeemptytask_t(MSKenv_t env,MSKtask_t * task);
static MSK_makeemptytask_t * MSK_makeemptytask_ptr = (MSK_makeemptytask_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_makeemptytask(MSKenv_t env,MSKtask_t * task) {
  return (*MSK_makeemptytask_ptr)(env,task);
} /* MSKmakeemptytask */
typedef MSKrescodee MSKAPI MSK_putexitfunc_t(MSKenv_t env,MSKexitfunc exitfunc,MSKuserhandle_t handle);
static MSK_putexitfunc_t * MSK_putexitfunc_ptr = (MSK_putexitfunc_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_putexitfunc(MSKenv_t env,MSKexitfunc exitfunc,MSKuserhandle_t handle) {
  return (*MSK_putexitfunc_ptr)(env,exitfunc,handle);
} /* MSKputexitfunc */
typedef MSKrescodee MSKAPI MSK_utf8towchar_t(size_t outputlen,size_t * len,size_t * conv,MSKwchart * output,const char * input);
static MSK_utf8towchar_t * MSK_utf8towchar_ptr = (MSK_utf8towchar_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_utf8towchar(size_t outputlen,size_t * len,size_t * conv,MSKwchart * output,const char * input) {
  return (*MSK_utf8towchar_ptr)(outputlen,len,conv,output,input);
} /* MSKutf8towchar */
typedef MSKrescodee MSKAPI MSK_wchartoutf8_t(size_t outputlen,size_t * len,size_t * conv,char * output,const MSKwchart * input);
static MSK_wchartoutf8_t * MSK_wchartoutf8_ptr = (MSK_wchartoutf8_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_wchartoutf8(size_t outputlen,size_t * len,size_t * conv,char * output,const MSKwchart * input) {
  return (*MSK_wchartoutf8_ptr)(outputlen,len,conv,output,input);
} /* MSKwchartoutf8 */
typedef MSKrescodee MSKAPI MSK_checkmemenv_t(MSKenv_t env,const char * file,MSKint32t line);
static MSK_checkmemenv_t * MSK_checkmemenv_ptr = (MSK_checkmemenv_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_checkmemenv(MSKenv_t env,const char * file,MSKint32t line) {
  return (*MSK_checkmemenv_ptr)(env,file,line);
} /* MSKcheckmemenv */
typedef MSKbooleant MSKAPI MSK_symnamtovalue_t(const char * name,char * value);
static MSK_symnamtovalue_t * MSK_symnamtovalue_ptr = (MSK_symnamtovalue_t*) default_ret_false;
MSKbooleant MSKAPI MSK_symnamtovalue(const char * name,char * value) {
  return (*MSK_symnamtovalue_ptr)(name,value);
} /* MSKsymnamtovalue */
typedef MSKrescodee MSKAPI MSK_axpy_t(MSKenv_t env,MSKint32t n,MSKrealt alpha,const MSKrealt * x,MSKrealt * y);
static MSK_axpy_t * MSK_axpy_ptr = (MSK_axpy_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_axpy(MSKenv_t env,MSKint32t n,MSKrealt alpha,const MSKrealt * x,MSKrealt * y) {
  return (*MSK_axpy_ptr)(env,n,alpha,x,y);
} /* MSKaxpy */
typedef MSKrescodee MSKAPI MSK_dot_t(MSKenv_t env,MSKint32t n,const MSKrealt * x,const MSKrealt * y,MSKrealt * xty);
static MSK_dot_t * MSK_dot_ptr = (MSK_dot_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_dot(MSKenv_t env,MSKint32t n,const MSKrealt * x,const MSKrealt * y,MSKrealt * xty) {
  return (*MSK_dot_ptr)(env,n,x,y,xty);
} /* MSKdot */
typedef MSKrescodee MSKAPI MSK_gemv_t(MSKenv_t env,MSKtransposee transa,MSKint32t m,MSKint32t n,MSKrealt alpha,const MSKrealt * a,const MSKrealt * x,MSKrealt beta,MSKrealt * y);
static MSK_gemv_t * MSK_gemv_ptr = (MSK_gemv_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_gemv(MSKenv_t env,MSKtransposee transa,MSKint32t m,MSKint32t n,MSKrealt alpha,const MSKrealt * a,const MSKrealt * x,MSKrealt beta,MSKrealt * y) {
  return (*MSK_gemv_ptr)(env,transa,m,n,alpha,a,x,beta,y);
} /* MSKgemv */
typedef MSKrescodee MSKAPI MSK_gemm_t(MSKenv_t env,MSKtransposee transa,MSKtransposee transb,MSKint32t m,MSKint32t n,MSKint32t k,MSKrealt alpha,const MSKrealt * a,const MSKrealt * b,MSKrealt beta,MSKrealt * c);
static MSK_gemm_t * MSK_gemm_ptr = (MSK_gemm_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_gemm(MSKenv_t env,MSKtransposee transa,MSKtransposee transb,MSKint32t m,MSKint32t n,MSKint32t k,MSKrealt alpha,const MSKrealt * a,const MSKrealt * b,MSKrealt beta,MSKrealt * c) {
  return (*MSK_gemm_ptr)(env,transa,transb,m,n,k,alpha,a,b,beta,c);
} /* MSKgemm */
typedef MSKrescodee MSKAPI MSK_syrk_t(MSKenv_t env,MSKuploe uplo,MSKtransposee trans,MSKint32t n,MSKint32t k,MSKrealt alpha,const MSKrealt * a,MSKrealt beta,MSKrealt * c);
static MSK_syrk_t * MSK_syrk_ptr = (MSK_syrk_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_syrk(MSKenv_t env,MSKuploe uplo,MSKtransposee trans,MSKint32t n,MSKint32t k,MSKrealt alpha,const MSKrealt * a,MSKrealt beta,MSKrealt * c) {
  return (*MSK_syrk_ptr)(env,uplo,trans,n,k,alpha,a,beta,c);
} /* MSKsyrk */
typedef MSKrescodee MSKAPI MSK_computesparsecholesky_t(MSKenv_t env,MSKint32t numthreads,MSKint32t ordermethod,MSKrealt tolsingular,MSKint32t n,const MSKint32t * anzc,const MSKint64t * aptrc,const MSKint32t * asubc,const MSKrealt * avalc,MSKint32t  ** perm,MSKrealt  ** diag,MSKint32t  ** lnzc,MSKint64t  ** lptrc,MSKint64t * lensubnval,MSKint32t  ** lsubc,MSKrealt  ** lvalc);
static MSK_computesparsecholesky_t * MSK_computesparsecholesky_ptr = (MSK_computesparsecholesky_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_computesparsecholesky(MSKenv_t env,MSKint32t numthreads,MSKint32t ordermethod,MSKrealt tolsingular,MSKint32t n,const MSKint32t * anzc,const MSKint64t * aptrc,const MSKint32t * asubc,const MSKrealt * avalc,MSKint32t  ** perm,MSKrealt  ** diag,MSKint32t  ** lnzc,MSKint64t  ** lptrc,MSKint64t * lensubnval,MSKint32t  ** lsubc,MSKrealt  ** lvalc) {
  return (*MSK_computesparsecholesky_ptr)(env,numthreads,ordermethod,tolsingular,n,anzc,aptrc,asubc,avalc,perm,diag,lnzc,lptrc,lensubnval,lsubc,lvalc);
} /* MSKcomputesparsecholesky */
typedef MSKrescodee MSKAPI MSK_sparsetriangularsolvedense_t(MSKenv_t env,MSKtransposee transposed,MSKint32t n,const MSKint32t * lnzc,const MSKint64t * lptrc,MSKint64t lensubnval,const MSKint32t * lsubc,const MSKrealt * lvalc,MSKrealt * b);
static MSK_sparsetriangularsolvedense_t * MSK_sparsetriangularsolvedense_ptr = (MSK_sparsetriangularsolvedense_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_sparsetriangularsolvedense(MSKenv_t env,MSKtransposee transposed,MSKint32t n,const MSKint32t * lnzc,const MSKint64t * lptrc,MSKint64t lensubnval,const MSKint32t * lsubc,const MSKrealt * lvalc,MSKrealt * b) {
  return (*MSK_sparsetriangularsolvedense_ptr)(env,transposed,n,lnzc,lptrc,lensubnval,lsubc,lvalc,b);
} /* MSKsparsetriangularsolvedense */
typedef MSKrescodee MSKAPI MSK_potrf_t(MSKenv_t env,MSKuploe uplo,MSKint32t n,MSKrealt * a);
static MSK_potrf_t * MSK_potrf_ptr = (MSK_potrf_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_potrf(MSKenv_t env,MSKuploe uplo,MSKint32t n,MSKrealt * a) {
  return (*MSK_potrf_ptr)(env,uplo,n,a);
} /* MSKpotrf */
typedef MSKrescodee MSKAPI MSK_syeig_t(MSKenv_t env,MSKuploe uplo,MSKint32t n,const MSKrealt * a,MSKrealt * w);
static MSK_syeig_t * MSK_syeig_ptr = (MSK_syeig_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_syeig(MSKenv_t env,MSKuploe uplo,MSKint32t n,const MSKrealt * a,MSKrealt * w) {
  return (*MSK_syeig_ptr)(env,uplo,n,a,w);
} /* MSKsyeig */
typedef MSKrescodee MSKAPI MSK_syevd_t(MSKenv_t env,MSKuploe uplo,MSKint32t n,MSKrealt * a,MSKrealt * w);
static MSK_syevd_t * MSK_syevd_ptr = (MSK_syevd_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_syevd(MSKenv_t env,MSKuploe uplo,MSKint32t n,MSKrealt * a,MSKrealt * w) {
  return (*MSK_syevd_ptr)(env,uplo,n,a,w);
} /* MSKsyevd */
typedef MSKrescodee MSKAPI MSK_licensecleanup_t();
static MSK_licensecleanup_t * MSK_licensecleanup_ptr = (MSK_licensecleanup_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_licensecleanup() {
  return (*MSK_licensecleanup_ptr)();
} /* MSKlicensecleanup */
typedef MSKrescodee MSKAPI MSK_shutdownglobalthreadpool_t();
static MSK_shutdownglobalthreadpool_t * MSK_shutdownglobalthreadpool_ptr = (MSK_shutdownglobalthreadpool_t*) default_ret_0i32;
MSKrescodee MSKAPI MSK_shutdownglobalthreadpool() {
  return (*MSK_shutdownglobalthreadpool_ptr)();
} /* MSKshutdownglobalthreadpool */


int MSK_isinitialized() {
    return libmosek_handle != NULL;
}
MSKrescodee MSK_initializedynamicwithpaths(int num_paths, const char * paths[]) {
    const char * errmsg = NULL;
    char * buf = NULL;
    if (! libmosek_handle) {
        if (num_paths > 0) {
            size_t libnamelen = strlen(libname);
            size_t maxpathlen = 0;
            for (int i = 0; i < num_paths; ++i) {
                size_t n = strlen(paths[i]);
                maxpathlen = maxpathlen > n ? maxpathlen : n;
            }
            if (maxpathlen > 0) {
                buf = (char*)calloc(maxpathlen+libnamelen+2,1);
                for (int i = 0; paths[i] && ! libmosek_handle; ++i) {
                    size_t n = strlen(paths[i]);
                    memcpy(buf, paths[i], n);
                    buf[n] = PATH_SEP;
                    memcpy(buf+n+1,libname,libnamelen);
                    buf[n+libnamelen+1] = 0;

                    libmosek_handle = __dlopen(buf,&errmsg);
                }
            }
        }

        if (!libmosek_handle)
            libmosek_handle = __dlopen(libname,&errmsg);

        if (!libmosek_handle) goto EXIT_ERROR;

        if (NULL == (MSK_analyzeproblem_ptr = (MSK_analyzeproblem_t*)__loadsym(libmosek_handle,"MSK_analyzeproblem",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_analyzenames_ptr = (MSK_analyzenames_t*)__loadsym(libmosek_handle,"MSK_analyzenames",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_analyzesolution_ptr = (MSK_analyzesolution_t*)__loadsym(libmosek_handle,"MSK_analyzesolution",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_initbasissolve_ptr = (MSK_initbasissolve_t*)__loadsym(libmosek_handle,"MSK_initbasissolve",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_solvewithbasis_ptr = (MSK_solvewithbasis_t*)__loadsym(libmosek_handle,"MSK_solvewithbasis",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_basiscond_ptr = (MSK_basiscond_t*)__loadsym(libmosek_handle,"MSK_basiscond",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appendcons_ptr = (MSK_appendcons_t*)__loadsym(libmosek_handle,"MSK_appendcons",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appendvars_ptr = (MSK_appendvars_t*)__loadsym(libmosek_handle,"MSK_appendvars",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_removecons_ptr = (MSK_removecons_t*)__loadsym(libmosek_handle,"MSK_removecons",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_removevars_ptr = (MSK_removevars_t*)__loadsym(libmosek_handle,"MSK_removevars",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_removebarvars_ptr = (MSK_removebarvars_t*)__loadsym(libmosek_handle,"MSK_removebarvars",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_removecones_ptr = (MSK_removecones_t*)__loadsym(libmosek_handle,"MSK_removecones",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appendbarvars_ptr = (MSK_appendbarvars_t*)__loadsym(libmosek_handle,"MSK_appendbarvars",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appendcone_ptr = (MSK_appendcone_t*)__loadsym(libmosek_handle,"MSK_appendcone",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appendconeseq_ptr = (MSK_appendconeseq_t*)__loadsym(libmosek_handle,"MSK_appendconeseq",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appendconesseq_ptr = (MSK_appendconesseq_t*)__loadsym(libmosek_handle,"MSK_appendconesseq",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_bktostr_ptr = (MSK_bktostr_t*)__loadsym(libmosek_handle,"MSK_bktostr",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_calloctask_ptr = (MSK_calloctask_t*)__loadsym(libmosek_handle,"MSK_calloctask",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_callocdbgtask_ptr = (MSK_callocdbgtask_t*)__loadsym(libmosek_handle,"MSK_callocdbgtask",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_chgconbound_ptr = (MSK_chgconbound_t*)__loadsym(libmosek_handle,"MSK_chgconbound",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_chgvarbound_ptr = (MSK_chgvarbound_t*)__loadsym(libmosek_handle,"MSK_chgvarbound",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_conetypetostr_ptr = (MSK_conetypetostr_t*)__loadsym(libmosek_handle,"MSK_conetypetostr",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_deletetask_ptr = (MSK_deletetask_t*)__loadsym(libmosek_handle,"MSK_deletetask",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_freetask_ptr = (MSK_freetask_t*)__loadsym(libmosek_handle,"MSK_freetask",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_freedbgtask_ptr = (MSK_freedbgtask_t*)__loadsym(libmosek_handle,"MSK_freedbgtask",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getaij_ptr = (MSK_getaij_t*)__loadsym(libmosek_handle,"MSK_getaij",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getapiecenumnz_ptr = (MSK_getapiecenumnz_t*)__loadsym(libmosek_handle,"MSK_getapiecenumnz",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getacolnumnz_ptr = (MSK_getacolnumnz_t*)__loadsym(libmosek_handle,"MSK_getacolnumnz",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getacol_ptr = (MSK_getacol_t*)__loadsym(libmosek_handle,"MSK_getacol",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getacolslice_ptr = (MSK_getacolslice_t*)__loadsym(libmosek_handle,"MSK_getacolslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getacolslice64_ptr = (MSK_getacolslice64_t*)__loadsym(libmosek_handle,"MSK_getacolslice64",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getarownumnz_ptr = (MSK_getarownumnz_t*)__loadsym(libmosek_handle,"MSK_getarownumnz",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getarow_ptr = (MSK_getarow_t*)__loadsym(libmosek_handle,"MSK_getarow",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getacolslicenumnz_ptr = (MSK_getacolslicenumnz_t*)__loadsym(libmosek_handle,"MSK_getacolslicenumnz",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getacolslicenumnz64_ptr = (MSK_getacolslicenumnz64_t*)__loadsym(libmosek_handle,"MSK_getacolslicenumnz64",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getarowslicenumnz_ptr = (MSK_getarowslicenumnz_t*)__loadsym(libmosek_handle,"MSK_getarowslicenumnz",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getarowslicenumnz64_ptr = (MSK_getarowslicenumnz64_t*)__loadsym(libmosek_handle,"MSK_getarowslicenumnz64",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getarowslice_ptr = (MSK_getarowslice_t*)__loadsym(libmosek_handle,"MSK_getarowslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getarowslice64_ptr = (MSK_getarowslice64_t*)__loadsym(libmosek_handle,"MSK_getarowslice64",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getatrip_ptr = (MSK_getatrip_t*)__loadsym(libmosek_handle,"MSK_getatrip",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getarowslicetrip_ptr = (MSK_getarowslicetrip_t*)__loadsym(libmosek_handle,"MSK_getarowslicetrip",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getacolslicetrip_ptr = (MSK_getacolslicetrip_t*)__loadsym(libmosek_handle,"MSK_getacolslicetrip",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getconbound_ptr = (MSK_getconbound_t*)__loadsym(libmosek_handle,"MSK_getconbound",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getvarbound_ptr = (MSK_getvarbound_t*)__loadsym(libmosek_handle,"MSK_getvarbound",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getconboundslice_ptr = (MSK_getconboundslice_t*)__loadsym(libmosek_handle,"MSK_getconboundslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getvarboundslice_ptr = (MSK_getvarboundslice_t*)__loadsym(libmosek_handle,"MSK_getvarboundslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getcj_ptr = (MSK_getcj_t*)__loadsym(libmosek_handle,"MSK_getcj",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getc_ptr = (MSK_getc_t*)__loadsym(libmosek_handle,"MSK_getc",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getcallbackfunc_ptr = (MSK_getcallbackfunc_t*)__loadsym(libmosek_handle,"MSK_getcallbackfunc",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getcfix_ptr = (MSK_getcfix_t*)__loadsym(libmosek_handle,"MSK_getcfix",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getcone_ptr = (MSK_getcone_t*)__loadsym(libmosek_handle,"MSK_getcone",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getconeinfo_ptr = (MSK_getconeinfo_t*)__loadsym(libmosek_handle,"MSK_getconeinfo",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getclist_ptr = (MSK_getclist_t*)__loadsym(libmosek_handle,"MSK_getclist",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getcslice_ptr = (MSK_getcslice_t*)__loadsym(libmosek_handle,"MSK_getcslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdouinf_ptr = (MSK_getdouinf_t*)__loadsym(libmosek_handle,"MSK_getdouinf",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdouparam_ptr = (MSK_getdouparam_t*)__loadsym(libmosek_handle,"MSK_getdouparam",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdualobj_ptr = (MSK_getdualobj_t*)__loadsym(libmosek_handle,"MSK_getdualobj",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getenv_ptr = (MSK_getenv_t*)__loadsym(libmosek_handle,"MSK_getenv",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getinfindex_ptr = (MSK_getinfindex_t*)__loadsym(libmosek_handle,"MSK_getinfindex",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getinfmax_ptr = (MSK_getinfmax_t*)__loadsym(libmosek_handle,"MSK_getinfmax",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getinfname_ptr = (MSK_getinfname_t*)__loadsym(libmosek_handle,"MSK_getinfname",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getintinf_ptr = (MSK_getintinf_t*)__loadsym(libmosek_handle,"MSK_getintinf",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getlintinf_ptr = (MSK_getlintinf_t*)__loadsym(libmosek_handle,"MSK_getlintinf",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getintparam_ptr = (MSK_getintparam_t*)__loadsym(libmosek_handle,"MSK_getintparam",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getlintparam_ptr = (MSK_getlintparam_t*)__loadsym(libmosek_handle,"MSK_getlintparam",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getmaxnamelen_ptr = (MSK_getmaxnamelen_t*)__loadsym(libmosek_handle,"MSK_getmaxnamelen",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getmaxnumanz_ptr = (MSK_getmaxnumanz_t*)__loadsym(libmosek_handle,"MSK_getmaxnumanz",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getmaxnumanz64_ptr = (MSK_getmaxnumanz64_t*)__loadsym(libmosek_handle,"MSK_getmaxnumanz64",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getmaxnumcon_ptr = (MSK_getmaxnumcon_t*)__loadsym(libmosek_handle,"MSK_getmaxnumcon",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getmaxnumvar_ptr = (MSK_getmaxnumvar_t*)__loadsym(libmosek_handle,"MSK_getmaxnumvar",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnadouinf_ptr = (MSK_getnadouinf_t*)__loadsym(libmosek_handle,"MSK_getnadouinf",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnadouparam_ptr = (MSK_getnadouparam_t*)__loadsym(libmosek_handle,"MSK_getnadouparam",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnaintinf_ptr = (MSK_getnaintinf_t*)__loadsym(libmosek_handle,"MSK_getnaintinf",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnaintparam_ptr = (MSK_getnaintparam_t*)__loadsym(libmosek_handle,"MSK_getnaintparam",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getbarvarnamelen_ptr = (MSK_getbarvarnamelen_t*)__loadsym(libmosek_handle,"MSK_getbarvarnamelen",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getbarvarname_ptr = (MSK_getbarvarname_t*)__loadsym(libmosek_handle,"MSK_getbarvarname",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getbarvarnameindex_ptr = (MSK_getbarvarnameindex_t*)__loadsym(libmosek_handle,"MSK_getbarvarnameindex",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_generatebarvarnames_ptr = (MSK_generatebarvarnames_t*)__loadsym(libmosek_handle,"MSK_generatebarvarnames",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_generatevarnames_ptr = (MSK_generatevarnames_t*)__loadsym(libmosek_handle,"MSK_generatevarnames",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_generateconnames_ptr = (MSK_generateconnames_t*)__loadsym(libmosek_handle,"MSK_generateconnames",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_generateconenames_ptr = (MSK_generateconenames_t*)__loadsym(libmosek_handle,"MSK_generateconenames",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_generateaccnames_ptr = (MSK_generateaccnames_t*)__loadsym(libmosek_handle,"MSK_generateaccnames",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_generatedjcnames_ptr = (MSK_generatedjcnames_t*)__loadsym(libmosek_handle,"MSK_generatedjcnames",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putconname_ptr = (MSK_putconname_t*)__loadsym(libmosek_handle,"MSK_putconname",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putvarname_ptr = (MSK_putvarname_t*)__loadsym(libmosek_handle,"MSK_putvarname",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putconename_ptr = (MSK_putconename_t*)__loadsym(libmosek_handle,"MSK_putconename",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putbarvarname_ptr = (MSK_putbarvarname_t*)__loadsym(libmosek_handle,"MSK_putbarvarname",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putdomainname_ptr = (MSK_putdomainname_t*)__loadsym(libmosek_handle,"MSK_putdomainname",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putdjcname_ptr = (MSK_putdjcname_t*)__loadsym(libmosek_handle,"MSK_putdjcname",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putaccname_ptr = (MSK_putaccname_t*)__loadsym(libmosek_handle,"MSK_putaccname",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getvarnamelen_ptr = (MSK_getvarnamelen_t*)__loadsym(libmosek_handle,"MSK_getvarnamelen",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getvarname_ptr = (MSK_getvarname_t*)__loadsym(libmosek_handle,"MSK_getvarname",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getconnamelen_ptr = (MSK_getconnamelen_t*)__loadsym(libmosek_handle,"MSK_getconnamelen",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getconname_ptr = (MSK_getconname_t*)__loadsym(libmosek_handle,"MSK_getconname",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getconnameindex_ptr = (MSK_getconnameindex_t*)__loadsym(libmosek_handle,"MSK_getconnameindex",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getvarnameindex_ptr = (MSK_getvarnameindex_t*)__loadsym(libmosek_handle,"MSK_getvarnameindex",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getconenamelen_ptr = (MSK_getconenamelen_t*)__loadsym(libmosek_handle,"MSK_getconenamelen",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getconename_ptr = (MSK_getconename_t*)__loadsym(libmosek_handle,"MSK_getconename",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getconenameindex_ptr = (MSK_getconenameindex_t*)__loadsym(libmosek_handle,"MSK_getconenameindex",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdomainnamelen_ptr = (MSK_getdomainnamelen_t*)__loadsym(libmosek_handle,"MSK_getdomainnamelen",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdomainname_ptr = (MSK_getdomainname_t*)__loadsym(libmosek_handle,"MSK_getdomainname",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdjcnamelen_ptr = (MSK_getdjcnamelen_t*)__loadsym(libmosek_handle,"MSK_getdjcnamelen",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdjcname_ptr = (MSK_getdjcname_t*)__loadsym(libmosek_handle,"MSK_getdjcname",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getaccnamelen_ptr = (MSK_getaccnamelen_t*)__loadsym(libmosek_handle,"MSK_getaccnamelen",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getaccname_ptr = (MSK_getaccname_t*)__loadsym(libmosek_handle,"MSK_getaccname",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnastrparam_ptr = (MSK_getnastrparam_t*)__loadsym(libmosek_handle,"MSK_getnastrparam",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnumanz_ptr = (MSK_getnumanz_t*)__loadsym(libmosek_handle,"MSK_getnumanz",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnumanz64_ptr = (MSK_getnumanz64_t*)__loadsym(libmosek_handle,"MSK_getnumanz64",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnumcon_ptr = (MSK_getnumcon_t*)__loadsym(libmosek_handle,"MSK_getnumcon",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnumcone_ptr = (MSK_getnumcone_t*)__loadsym(libmosek_handle,"MSK_getnumcone",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnumconemem_ptr = (MSK_getnumconemem_t*)__loadsym(libmosek_handle,"MSK_getnumconemem",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnumintvar_ptr = (MSK_getnumintvar_t*)__loadsym(libmosek_handle,"MSK_getnumintvar",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnumparam_ptr = (MSK_getnumparam_t*)__loadsym(libmosek_handle,"MSK_getnumparam",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnumqconknz_ptr = (MSK_getnumqconknz_t*)__loadsym(libmosek_handle,"MSK_getnumqconknz",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnumqconknz64_ptr = (MSK_getnumqconknz64_t*)__loadsym(libmosek_handle,"MSK_getnumqconknz64",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnumqobjnz_ptr = (MSK_getnumqobjnz_t*)__loadsym(libmosek_handle,"MSK_getnumqobjnz",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnumqobjnz64_ptr = (MSK_getnumqobjnz64_t*)__loadsym(libmosek_handle,"MSK_getnumqobjnz64",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnumvar_ptr = (MSK_getnumvar_t*)__loadsym(libmosek_handle,"MSK_getnumvar",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnumbarvar_ptr = (MSK_getnumbarvar_t*)__loadsym(libmosek_handle,"MSK_getnumbarvar",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getmaxnumbarvar_ptr = (MSK_getmaxnumbarvar_t*)__loadsym(libmosek_handle,"MSK_getmaxnumbarvar",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdimbarvarj_ptr = (MSK_getdimbarvarj_t*)__loadsym(libmosek_handle,"MSK_getdimbarvarj",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getlenbarvarj_ptr = (MSK_getlenbarvarj_t*)__loadsym(libmosek_handle,"MSK_getlenbarvarj",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getobjname_ptr = (MSK_getobjname_t*)__loadsym(libmosek_handle,"MSK_getobjname",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getobjnamelen_ptr = (MSK_getobjnamelen_t*)__loadsym(libmosek_handle,"MSK_getobjnamelen",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getparamname_ptr = (MSK_getparamname_t*)__loadsym(libmosek_handle,"MSK_getparamname",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getparammax_ptr = (MSK_getparammax_t*)__loadsym(libmosek_handle,"MSK_getparammax",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getprimalobj_ptr = (MSK_getprimalobj_t*)__loadsym(libmosek_handle,"MSK_getprimalobj",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getprobtype_ptr = (MSK_getprobtype_t*)__loadsym(libmosek_handle,"MSK_getprobtype",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getqconk64_ptr = (MSK_getqconk64_t*)__loadsym(libmosek_handle,"MSK_getqconk64",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getqconk_ptr = (MSK_getqconk_t*)__loadsym(libmosek_handle,"MSK_getqconk",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getqobj_ptr = (MSK_getqobj_t*)__loadsym(libmosek_handle,"MSK_getqobj",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getqobj64_ptr = (MSK_getqobj64_t*)__loadsym(libmosek_handle,"MSK_getqobj64",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getqobjij_ptr = (MSK_getqobjij_t*)__loadsym(libmosek_handle,"MSK_getqobjij",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getsolution_ptr = (MSK_getsolution_t*)__loadsym(libmosek_handle,"MSK_getsolution",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getsolutionnew_ptr = (MSK_getsolutionnew_t*)__loadsym(libmosek_handle,"MSK_getsolutionnew",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getsolsta_ptr = (MSK_getsolsta_t*)__loadsym(libmosek_handle,"MSK_getsolsta",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getprosta_ptr = (MSK_getprosta_t*)__loadsym(libmosek_handle,"MSK_getprosta",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getskc_ptr = (MSK_getskc_t*)__loadsym(libmosek_handle,"MSK_getskc",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getskx_ptr = (MSK_getskx_t*)__loadsym(libmosek_handle,"MSK_getskx",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getskn_ptr = (MSK_getskn_t*)__loadsym(libmosek_handle,"MSK_getskn",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getxc_ptr = (MSK_getxc_t*)__loadsym(libmosek_handle,"MSK_getxc",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getxx_ptr = (MSK_getxx_t*)__loadsym(libmosek_handle,"MSK_getxx",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_gety_ptr = (MSK_gety_t*)__loadsym(libmosek_handle,"MSK_gety",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getslc_ptr = (MSK_getslc_t*)__loadsym(libmosek_handle,"MSK_getslc",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getaccdoty_ptr = (MSK_getaccdoty_t*)__loadsym(libmosek_handle,"MSK_getaccdoty",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getaccdotys_ptr = (MSK_getaccdotys_t*)__loadsym(libmosek_handle,"MSK_getaccdotys",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_evaluateacc_ptr = (MSK_evaluateacc_t*)__loadsym(libmosek_handle,"MSK_evaluateacc",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_evaluateaccs_ptr = (MSK_evaluateaccs_t*)__loadsym(libmosek_handle,"MSK_evaluateaccs",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getsuc_ptr = (MSK_getsuc_t*)__loadsym(libmosek_handle,"MSK_getsuc",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getslx_ptr = (MSK_getslx_t*)__loadsym(libmosek_handle,"MSK_getslx",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getsux_ptr = (MSK_getsux_t*)__loadsym(libmosek_handle,"MSK_getsux",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getsnx_ptr = (MSK_getsnx_t*)__loadsym(libmosek_handle,"MSK_getsnx",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getskcslice_ptr = (MSK_getskcslice_t*)__loadsym(libmosek_handle,"MSK_getskcslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getskxslice_ptr = (MSK_getskxslice_t*)__loadsym(libmosek_handle,"MSK_getskxslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getxcslice_ptr = (MSK_getxcslice_t*)__loadsym(libmosek_handle,"MSK_getxcslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getxxslice_ptr = (MSK_getxxslice_t*)__loadsym(libmosek_handle,"MSK_getxxslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getyslice_ptr = (MSK_getyslice_t*)__loadsym(libmosek_handle,"MSK_getyslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getslcslice_ptr = (MSK_getslcslice_t*)__loadsym(libmosek_handle,"MSK_getslcslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getsucslice_ptr = (MSK_getsucslice_t*)__loadsym(libmosek_handle,"MSK_getsucslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getslxslice_ptr = (MSK_getslxslice_t*)__loadsym(libmosek_handle,"MSK_getslxslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getsuxslice_ptr = (MSK_getsuxslice_t*)__loadsym(libmosek_handle,"MSK_getsuxslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getsnxslice_ptr = (MSK_getsnxslice_t*)__loadsym(libmosek_handle,"MSK_getsnxslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getbarxj_ptr = (MSK_getbarxj_t*)__loadsym(libmosek_handle,"MSK_getbarxj",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getbarxslice_ptr = (MSK_getbarxslice_t*)__loadsym(libmosek_handle,"MSK_getbarxslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getbarsj_ptr = (MSK_getbarsj_t*)__loadsym(libmosek_handle,"MSK_getbarsj",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getbarsslice_ptr = (MSK_getbarsslice_t*)__loadsym(libmosek_handle,"MSK_getbarsslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putskc_ptr = (MSK_putskc_t*)__loadsym(libmosek_handle,"MSK_putskc",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putskx_ptr = (MSK_putskx_t*)__loadsym(libmosek_handle,"MSK_putskx",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putxc_ptr = (MSK_putxc_t*)__loadsym(libmosek_handle,"MSK_putxc",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putxx_ptr = (MSK_putxx_t*)__loadsym(libmosek_handle,"MSK_putxx",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_puty_ptr = (MSK_puty_t*)__loadsym(libmosek_handle,"MSK_puty",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putslc_ptr = (MSK_putslc_t*)__loadsym(libmosek_handle,"MSK_putslc",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putsuc_ptr = (MSK_putsuc_t*)__loadsym(libmosek_handle,"MSK_putsuc",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putslx_ptr = (MSK_putslx_t*)__loadsym(libmosek_handle,"MSK_putslx",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putsux_ptr = (MSK_putsux_t*)__loadsym(libmosek_handle,"MSK_putsux",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putsnx_ptr = (MSK_putsnx_t*)__loadsym(libmosek_handle,"MSK_putsnx",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putaccdoty_ptr = (MSK_putaccdoty_t*)__loadsym(libmosek_handle,"MSK_putaccdoty",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putskcslice_ptr = (MSK_putskcslice_t*)__loadsym(libmosek_handle,"MSK_putskcslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putskxslice_ptr = (MSK_putskxslice_t*)__loadsym(libmosek_handle,"MSK_putskxslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putxcslice_ptr = (MSK_putxcslice_t*)__loadsym(libmosek_handle,"MSK_putxcslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putxxslice_ptr = (MSK_putxxslice_t*)__loadsym(libmosek_handle,"MSK_putxxslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putyslice_ptr = (MSK_putyslice_t*)__loadsym(libmosek_handle,"MSK_putyslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putslcslice_ptr = (MSK_putslcslice_t*)__loadsym(libmosek_handle,"MSK_putslcslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putsucslice_ptr = (MSK_putsucslice_t*)__loadsym(libmosek_handle,"MSK_putsucslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putslxslice_ptr = (MSK_putslxslice_t*)__loadsym(libmosek_handle,"MSK_putslxslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putsuxslice_ptr = (MSK_putsuxslice_t*)__loadsym(libmosek_handle,"MSK_putsuxslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putsnxslice_ptr = (MSK_putsnxslice_t*)__loadsym(libmosek_handle,"MSK_putsnxslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putbarxj_ptr = (MSK_putbarxj_t*)__loadsym(libmosek_handle,"MSK_putbarxj",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putbarsj_ptr = (MSK_putbarsj_t*)__loadsym(libmosek_handle,"MSK_putbarsj",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getpviolcon_ptr = (MSK_getpviolcon_t*)__loadsym(libmosek_handle,"MSK_getpviolcon",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getpviolvar_ptr = (MSK_getpviolvar_t*)__loadsym(libmosek_handle,"MSK_getpviolvar",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getpviolbarvar_ptr = (MSK_getpviolbarvar_t*)__loadsym(libmosek_handle,"MSK_getpviolbarvar",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getpviolcones_ptr = (MSK_getpviolcones_t*)__loadsym(libmosek_handle,"MSK_getpviolcones",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getpviolacc_ptr = (MSK_getpviolacc_t*)__loadsym(libmosek_handle,"MSK_getpviolacc",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getpvioldjc_ptr = (MSK_getpvioldjc_t*)__loadsym(libmosek_handle,"MSK_getpvioldjc",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdviolcon_ptr = (MSK_getdviolcon_t*)__loadsym(libmosek_handle,"MSK_getdviolcon",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdviolvar_ptr = (MSK_getdviolvar_t*)__loadsym(libmosek_handle,"MSK_getdviolvar",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdviolbarvar_ptr = (MSK_getdviolbarvar_t*)__loadsym(libmosek_handle,"MSK_getdviolbarvar",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdviolcones_ptr = (MSK_getdviolcones_t*)__loadsym(libmosek_handle,"MSK_getdviolcones",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdviolacc_ptr = (MSK_getdviolacc_t*)__loadsym(libmosek_handle,"MSK_getdviolacc",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getsolutioninfo_ptr = (MSK_getsolutioninfo_t*)__loadsym(libmosek_handle,"MSK_getsolutioninfo",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getsolutioninfonew_ptr = (MSK_getsolutioninfonew_t*)__loadsym(libmosek_handle,"MSK_getsolutioninfonew",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdualsolutionnorms_ptr = (MSK_getdualsolutionnorms_t*)__loadsym(libmosek_handle,"MSK_getdualsolutionnorms",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getprimalsolutionnorms_ptr = (MSK_getprimalsolutionnorms_t*)__loadsym(libmosek_handle,"MSK_getprimalsolutionnorms",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getsolutionslice_ptr = (MSK_getsolutionslice_t*)__loadsym(libmosek_handle,"MSK_getsolutionslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getreducedcosts_ptr = (MSK_getreducedcosts_t*)__loadsym(libmosek_handle,"MSK_getreducedcosts",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getstrparam_ptr = (MSK_getstrparam_t*)__loadsym(libmosek_handle,"MSK_getstrparam",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getstrparamlen_ptr = (MSK_getstrparamlen_t*)__loadsym(libmosek_handle,"MSK_getstrparamlen",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getstrparamal_ptr = (MSK_getstrparamal_t*)__loadsym(libmosek_handle,"MSK_getstrparamal",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnastrparamal_ptr = (MSK_getnastrparamal_t*)__loadsym(libmosek_handle,"MSK_getnastrparamal",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getsymbcon_ptr = (MSK_getsymbcon_t*)__loadsym(libmosek_handle,"MSK_getsymbcon",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_gettasknamelen_ptr = (MSK_gettasknamelen_t*)__loadsym(libmosek_handle,"MSK_gettasknamelen",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_gettaskname_ptr = (MSK_gettaskname_t*)__loadsym(libmosek_handle,"MSK_gettaskname",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getmionumthreads_ptr = (MSK_getmionumthreads_t*)__loadsym(libmosek_handle,"MSK_getmionumthreads",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getvartype_ptr = (MSK_getvartype_t*)__loadsym(libmosek_handle,"MSK_getvartype",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getvartypelist_ptr = (MSK_getvartypelist_t*)__loadsym(libmosek_handle,"MSK_getvartypelist",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_inputdata_ptr = (MSK_inputdata_t*)__loadsym(libmosek_handle,"MSK_inputdata",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_inputdata64_ptr = (MSK_inputdata64_t*)__loadsym(libmosek_handle,"MSK_inputdata64",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_isdouparname_ptr = (MSK_isdouparname_t*)__loadsym(libmosek_handle,"MSK_isdouparname",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_isintparname_ptr = (MSK_isintparname_t*)__loadsym(libmosek_handle,"MSK_isintparname",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_isstrparname_ptr = (MSK_isstrparname_t*)__loadsym(libmosek_handle,"MSK_isstrparname",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_linkfiletotaskstream_ptr = (MSK_linkfiletotaskstream_t*)__loadsym(libmosek_handle,"MSK_linkfiletotaskstream",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_linkfunctotaskstream_ptr = (MSK_linkfunctotaskstream_t*)__loadsym(libmosek_handle,"MSK_linkfunctotaskstream",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_unlinkfuncfromtaskstream_ptr = (MSK_unlinkfuncfromtaskstream_t*)__loadsym(libmosek_handle,"MSK_unlinkfuncfromtaskstream",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_clonetask_ptr = (MSK_clonetask_t*)__loadsym(libmosek_handle,"MSK_clonetask",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_primalrepair_ptr = (MSK_primalrepair_t*)__loadsym(libmosek_handle,"MSK_primalrepair",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_infeasibilityreport_ptr = (MSK_infeasibilityreport_t*)__loadsym(libmosek_handle,"MSK_infeasibilityreport",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_toconic_ptr = (MSK_toconic_t*)__loadsym(libmosek_handle,"MSK_toconic",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_optimize_ptr = (MSK_optimize_t*)__loadsym(libmosek_handle,"MSK_optimize",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_optimizetrm_ptr = (MSK_optimizetrm_t*)__loadsym(libmosek_handle,"MSK_optimizetrm",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_printparam_ptr = (MSK_printparam_t*)__loadsym(libmosek_handle,"MSK_printparam",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_probtypetostr_ptr = (MSK_probtypetostr_t*)__loadsym(libmosek_handle,"MSK_probtypetostr",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_prostatostr_ptr = (MSK_prostatostr_t*)__loadsym(libmosek_handle,"MSK_prostatostr",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putresponsefunc_ptr = (MSK_putresponsefunc_t*)__loadsym(libmosek_handle,"MSK_putresponsefunc",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_commitchanges_ptr = (MSK_commitchanges_t*)__loadsym(libmosek_handle,"MSK_commitchanges",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getatruncatetol_ptr = (MSK_getatruncatetol_t*)__loadsym(libmosek_handle,"MSK_getatruncatetol",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putatruncatetol_ptr = (MSK_putatruncatetol_t*)__loadsym(libmosek_handle,"MSK_putatruncatetol",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putaij_ptr = (MSK_putaij_t*)__loadsym(libmosek_handle,"MSK_putaij",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putaijlist_ptr = (MSK_putaijlist_t*)__loadsym(libmosek_handle,"MSK_putaijlist",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putaijlist64_ptr = (MSK_putaijlist64_t*)__loadsym(libmosek_handle,"MSK_putaijlist64",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putacol_ptr = (MSK_putacol_t*)__loadsym(libmosek_handle,"MSK_putacol",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putarow_ptr = (MSK_putarow_t*)__loadsym(libmosek_handle,"MSK_putarow",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putarowslice_ptr = (MSK_putarowslice_t*)__loadsym(libmosek_handle,"MSK_putarowslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putarowslice64_ptr = (MSK_putarowslice64_t*)__loadsym(libmosek_handle,"MSK_putarowslice64",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putarowlist_ptr = (MSK_putarowlist_t*)__loadsym(libmosek_handle,"MSK_putarowlist",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putarowlist64_ptr = (MSK_putarowlist64_t*)__loadsym(libmosek_handle,"MSK_putarowlist64",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putacolslice_ptr = (MSK_putacolslice_t*)__loadsym(libmosek_handle,"MSK_putacolslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putacolslice64_ptr = (MSK_putacolslice64_t*)__loadsym(libmosek_handle,"MSK_putacolslice64",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putacollist_ptr = (MSK_putacollist_t*)__loadsym(libmosek_handle,"MSK_putacollist",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putacollist64_ptr = (MSK_putacollist64_t*)__loadsym(libmosek_handle,"MSK_putacollist64",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putbaraij_ptr = (MSK_putbaraij_t*)__loadsym(libmosek_handle,"MSK_putbaraij",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putbaraijlist_ptr = (MSK_putbaraijlist_t*)__loadsym(libmosek_handle,"MSK_putbaraijlist",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putbararowlist_ptr = (MSK_putbararowlist_t*)__loadsym(libmosek_handle,"MSK_putbararowlist",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnumbarcnz_ptr = (MSK_getnumbarcnz_t*)__loadsym(libmosek_handle,"MSK_getnumbarcnz",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnumbaranz_ptr = (MSK_getnumbaranz_t*)__loadsym(libmosek_handle,"MSK_getnumbaranz",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getbarcsparsity_ptr = (MSK_getbarcsparsity_t*)__loadsym(libmosek_handle,"MSK_getbarcsparsity",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getbarasparsity_ptr = (MSK_getbarasparsity_t*)__loadsym(libmosek_handle,"MSK_getbarasparsity",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getbarcidxinfo_ptr = (MSK_getbarcidxinfo_t*)__loadsym(libmosek_handle,"MSK_getbarcidxinfo",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getbarcidxj_ptr = (MSK_getbarcidxj_t*)__loadsym(libmosek_handle,"MSK_getbarcidxj",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getbarcidx_ptr = (MSK_getbarcidx_t*)__loadsym(libmosek_handle,"MSK_getbarcidx",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getbaraidxinfo_ptr = (MSK_getbaraidxinfo_t*)__loadsym(libmosek_handle,"MSK_getbaraidxinfo",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getbaraidxij_ptr = (MSK_getbaraidxij_t*)__loadsym(libmosek_handle,"MSK_getbaraidxij",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getbaraidx_ptr = (MSK_getbaraidx_t*)__loadsym(libmosek_handle,"MSK_getbaraidx",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnumbarcblocktriplets_ptr = (MSK_getnumbarcblocktriplets_t*)__loadsym(libmosek_handle,"MSK_getnumbarcblocktriplets",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putbarcblocktriplet_ptr = (MSK_putbarcblocktriplet_t*)__loadsym(libmosek_handle,"MSK_putbarcblocktriplet",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getbarcblocktriplet_ptr = (MSK_getbarcblocktriplet_t*)__loadsym(libmosek_handle,"MSK_getbarcblocktriplet",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putbarablocktriplet_ptr = (MSK_putbarablocktriplet_t*)__loadsym(libmosek_handle,"MSK_putbarablocktriplet",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnumbarablocktriplets_ptr = (MSK_getnumbarablocktriplets_t*)__loadsym(libmosek_handle,"MSK_getnumbarablocktriplets",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getbarablocktriplet_ptr = (MSK_getbarablocktriplet_t*)__loadsym(libmosek_handle,"MSK_getbarablocktriplet",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putmaxnumafe_ptr = (MSK_putmaxnumafe_t*)__loadsym(libmosek_handle,"MSK_putmaxnumafe",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnumafe_ptr = (MSK_getnumafe_t*)__loadsym(libmosek_handle,"MSK_getnumafe",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appendafes_ptr = (MSK_appendafes_t*)__loadsym(libmosek_handle,"MSK_appendafes",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putafefentry_ptr = (MSK_putafefentry_t*)__loadsym(libmosek_handle,"MSK_putafefentry",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putafefentrylist_ptr = (MSK_putafefentrylist_t*)__loadsym(libmosek_handle,"MSK_putafefentrylist",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_emptyafefrow_ptr = (MSK_emptyafefrow_t*)__loadsym(libmosek_handle,"MSK_emptyafefrow",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_emptyafefcol_ptr = (MSK_emptyafefcol_t*)__loadsym(libmosek_handle,"MSK_emptyafefcol",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_emptyafefrowlist_ptr = (MSK_emptyafefrowlist_t*)__loadsym(libmosek_handle,"MSK_emptyafefrowlist",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_emptyafefcollist_ptr = (MSK_emptyafefcollist_t*)__loadsym(libmosek_handle,"MSK_emptyafefcollist",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putafefrow_ptr = (MSK_putafefrow_t*)__loadsym(libmosek_handle,"MSK_putafefrow",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putafefrowlist_ptr = (MSK_putafefrowlist_t*)__loadsym(libmosek_handle,"MSK_putafefrowlist",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putafefcol_ptr = (MSK_putafefcol_t*)__loadsym(libmosek_handle,"MSK_putafefcol",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getafefrownumnz_ptr = (MSK_getafefrownumnz_t*)__loadsym(libmosek_handle,"MSK_getafefrownumnz",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getafefnumnz_ptr = (MSK_getafefnumnz_t*)__loadsym(libmosek_handle,"MSK_getafefnumnz",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getafefrow_ptr = (MSK_getafefrow_t*)__loadsym(libmosek_handle,"MSK_getafefrow",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getafeftrip_ptr = (MSK_getafeftrip_t*)__loadsym(libmosek_handle,"MSK_getafeftrip",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putafebarfentry_ptr = (MSK_putafebarfentry_t*)__loadsym(libmosek_handle,"MSK_putafebarfentry",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putafebarfentrylist_ptr = (MSK_putafebarfentrylist_t*)__loadsym(libmosek_handle,"MSK_putafebarfentrylist",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putafebarfrow_ptr = (MSK_putafebarfrow_t*)__loadsym(libmosek_handle,"MSK_putafebarfrow",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_emptyafebarfrow_ptr = (MSK_emptyafebarfrow_t*)__loadsym(libmosek_handle,"MSK_emptyafebarfrow",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_emptyafebarfrowlist_ptr = (MSK_emptyafebarfrowlist_t*)__loadsym(libmosek_handle,"MSK_emptyafebarfrowlist",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putafebarfblocktriplet_ptr = (MSK_putafebarfblocktriplet_t*)__loadsym(libmosek_handle,"MSK_putafebarfblocktriplet",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getafebarfnumblocktriplets_ptr = (MSK_getafebarfnumblocktriplets_t*)__loadsym(libmosek_handle,"MSK_getafebarfnumblocktriplets",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getafebarfblocktriplet_ptr = (MSK_getafebarfblocktriplet_t*)__loadsym(libmosek_handle,"MSK_getafebarfblocktriplet",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getafebarfnumrowentries_ptr = (MSK_getafebarfnumrowentries_t*)__loadsym(libmosek_handle,"MSK_getafebarfnumrowentries",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getafebarfrowinfo_ptr = (MSK_getafebarfrowinfo_t*)__loadsym(libmosek_handle,"MSK_getafebarfrowinfo",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getafebarfrow_ptr = (MSK_getafebarfrow_t*)__loadsym(libmosek_handle,"MSK_getafebarfrow",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putafeg_ptr = (MSK_putafeg_t*)__loadsym(libmosek_handle,"MSK_putafeg",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putafeglist_ptr = (MSK_putafeglist_t*)__loadsym(libmosek_handle,"MSK_putafeglist",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getafeg_ptr = (MSK_getafeg_t*)__loadsym(libmosek_handle,"MSK_getafeg",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getafegslice_ptr = (MSK_getafegslice_t*)__loadsym(libmosek_handle,"MSK_getafegslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putafegslice_ptr = (MSK_putafegslice_t*)__loadsym(libmosek_handle,"MSK_putafegslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putmaxnumdjc_ptr = (MSK_putmaxnumdjc_t*)__loadsym(libmosek_handle,"MSK_putmaxnumdjc",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnumdjc_ptr = (MSK_getnumdjc_t*)__loadsym(libmosek_handle,"MSK_getnumdjc",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdjcnumdomain_ptr = (MSK_getdjcnumdomain_t*)__loadsym(libmosek_handle,"MSK_getdjcnumdomain",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdjcnumdomaintot_ptr = (MSK_getdjcnumdomaintot_t*)__loadsym(libmosek_handle,"MSK_getdjcnumdomaintot",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdjcnumafe_ptr = (MSK_getdjcnumafe_t*)__loadsym(libmosek_handle,"MSK_getdjcnumafe",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdjcnumafetot_ptr = (MSK_getdjcnumafetot_t*)__loadsym(libmosek_handle,"MSK_getdjcnumafetot",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdjcnumterm_ptr = (MSK_getdjcnumterm_t*)__loadsym(libmosek_handle,"MSK_getdjcnumterm",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdjcnumtermtot_ptr = (MSK_getdjcnumtermtot_t*)__loadsym(libmosek_handle,"MSK_getdjcnumtermtot",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putmaxnumacc_ptr = (MSK_putmaxnumacc_t*)__loadsym(libmosek_handle,"MSK_putmaxnumacc",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnumacc_ptr = (MSK_getnumacc_t*)__loadsym(libmosek_handle,"MSK_getnumacc",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appendacc_ptr = (MSK_appendacc_t*)__loadsym(libmosek_handle,"MSK_appendacc",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appendaccs_ptr = (MSK_appendaccs_t*)__loadsym(libmosek_handle,"MSK_appendaccs",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appendaccseq_ptr = (MSK_appendaccseq_t*)__loadsym(libmosek_handle,"MSK_appendaccseq",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appendaccsseq_ptr = (MSK_appendaccsseq_t*)__loadsym(libmosek_handle,"MSK_appendaccsseq",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putacc_ptr = (MSK_putacc_t*)__loadsym(libmosek_handle,"MSK_putacc",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putacclist_ptr = (MSK_putacclist_t*)__loadsym(libmosek_handle,"MSK_putacclist",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putaccb_ptr = (MSK_putaccb_t*)__loadsym(libmosek_handle,"MSK_putaccb",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putaccbj_ptr = (MSK_putaccbj_t*)__loadsym(libmosek_handle,"MSK_putaccbj",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getaccdomain_ptr = (MSK_getaccdomain_t*)__loadsym(libmosek_handle,"MSK_getaccdomain",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getaccn_ptr = (MSK_getaccn_t*)__loadsym(libmosek_handle,"MSK_getaccn",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getaccntot_ptr = (MSK_getaccntot_t*)__loadsym(libmosek_handle,"MSK_getaccntot",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getaccafeidxlist_ptr = (MSK_getaccafeidxlist_t*)__loadsym(libmosek_handle,"MSK_getaccafeidxlist",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getaccb_ptr = (MSK_getaccb_t*)__loadsym(libmosek_handle,"MSK_getaccb",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getaccs_ptr = (MSK_getaccs_t*)__loadsym(libmosek_handle,"MSK_getaccs",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getaccfnumnz_ptr = (MSK_getaccfnumnz_t*)__loadsym(libmosek_handle,"MSK_getaccfnumnz",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getaccftrip_ptr = (MSK_getaccftrip_t*)__loadsym(libmosek_handle,"MSK_getaccftrip",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getaccgvector_ptr = (MSK_getaccgvector_t*)__loadsym(libmosek_handle,"MSK_getaccgvector",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getaccbarfnumblocktriplets_ptr = (MSK_getaccbarfnumblocktriplets_t*)__loadsym(libmosek_handle,"MSK_getaccbarfnumblocktriplets",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getaccbarfblocktriplet_ptr = (MSK_getaccbarfblocktriplet_t*)__loadsym(libmosek_handle,"MSK_getaccbarfblocktriplet",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appenddjcs_ptr = (MSK_appenddjcs_t*)__loadsym(libmosek_handle,"MSK_appenddjcs",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putdjc_ptr = (MSK_putdjc_t*)__loadsym(libmosek_handle,"MSK_putdjc",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putdjcslice_ptr = (MSK_putdjcslice_t*)__loadsym(libmosek_handle,"MSK_putdjcslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdjcdomainidxlist_ptr = (MSK_getdjcdomainidxlist_t*)__loadsym(libmosek_handle,"MSK_getdjcdomainidxlist",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdjcafeidxlist_ptr = (MSK_getdjcafeidxlist_t*)__loadsym(libmosek_handle,"MSK_getdjcafeidxlist",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdjcb_ptr = (MSK_getdjcb_t*)__loadsym(libmosek_handle,"MSK_getdjcb",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdjctermsizelist_ptr = (MSK_getdjctermsizelist_t*)__loadsym(libmosek_handle,"MSK_getdjctermsizelist",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdjcs_ptr = (MSK_getdjcs_t*)__loadsym(libmosek_handle,"MSK_getdjcs",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putconbound_ptr = (MSK_putconbound_t*)__loadsym(libmosek_handle,"MSK_putconbound",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putconboundlist_ptr = (MSK_putconboundlist_t*)__loadsym(libmosek_handle,"MSK_putconboundlist",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putconboundlistconst_ptr = (MSK_putconboundlistconst_t*)__loadsym(libmosek_handle,"MSK_putconboundlistconst",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putconboundslice_ptr = (MSK_putconboundslice_t*)__loadsym(libmosek_handle,"MSK_putconboundslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putconboundsliceconst_ptr = (MSK_putconboundsliceconst_t*)__loadsym(libmosek_handle,"MSK_putconboundsliceconst",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putvarbound_ptr = (MSK_putvarbound_t*)__loadsym(libmosek_handle,"MSK_putvarbound",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putvarboundlist_ptr = (MSK_putvarboundlist_t*)__loadsym(libmosek_handle,"MSK_putvarboundlist",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putvarboundlistconst_ptr = (MSK_putvarboundlistconst_t*)__loadsym(libmosek_handle,"MSK_putvarboundlistconst",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putvarboundslice_ptr = (MSK_putvarboundslice_t*)__loadsym(libmosek_handle,"MSK_putvarboundslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putvarboundsliceconst_ptr = (MSK_putvarboundsliceconst_t*)__loadsym(libmosek_handle,"MSK_putvarboundsliceconst",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putcallbackfunc_ptr = (MSK_putcallbackfunc_t*)__loadsym(libmosek_handle,"MSK_putcallbackfunc",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putcfix_ptr = (MSK_putcfix_t*)__loadsym(libmosek_handle,"MSK_putcfix",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putcj_ptr = (MSK_putcj_t*)__loadsym(libmosek_handle,"MSK_putcj",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putobjsense_ptr = (MSK_putobjsense_t*)__loadsym(libmosek_handle,"MSK_putobjsense",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getobjsense_ptr = (MSK_getobjsense_t*)__loadsym(libmosek_handle,"MSK_getobjsense",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putclist_ptr = (MSK_putclist_t*)__loadsym(libmosek_handle,"MSK_putclist",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putcslice_ptr = (MSK_putcslice_t*)__loadsym(libmosek_handle,"MSK_putcslice",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putbarcj_ptr = (MSK_putbarcj_t*)__loadsym(libmosek_handle,"MSK_putbarcj",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putcone_ptr = (MSK_putcone_t*)__loadsym(libmosek_handle,"MSK_putcone",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putmaxnumdomain_ptr = (MSK_putmaxnumdomain_t*)__loadsym(libmosek_handle,"MSK_putmaxnumdomain",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnumdomain_ptr = (MSK_getnumdomain_t*)__loadsym(libmosek_handle,"MSK_getnumdomain",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appendrplusdomain_ptr = (MSK_appendrplusdomain_t*)__loadsym(libmosek_handle,"MSK_appendrplusdomain",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appendrminusdomain_ptr = (MSK_appendrminusdomain_t*)__loadsym(libmosek_handle,"MSK_appendrminusdomain",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appendrdomain_ptr = (MSK_appendrdomain_t*)__loadsym(libmosek_handle,"MSK_appendrdomain",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appendrzerodomain_ptr = (MSK_appendrzerodomain_t*)__loadsym(libmosek_handle,"MSK_appendrzerodomain",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appendquadraticconedomain_ptr = (MSK_appendquadraticconedomain_t*)__loadsym(libmosek_handle,"MSK_appendquadraticconedomain",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appendrquadraticconedomain_ptr = (MSK_appendrquadraticconedomain_t*)__loadsym(libmosek_handle,"MSK_appendrquadraticconedomain",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appendprimalexpconedomain_ptr = (MSK_appendprimalexpconedomain_t*)__loadsym(libmosek_handle,"MSK_appendprimalexpconedomain",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appenddualexpconedomain_ptr = (MSK_appenddualexpconedomain_t*)__loadsym(libmosek_handle,"MSK_appenddualexpconedomain",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appendprimalgeomeanconedomain_ptr = (MSK_appendprimalgeomeanconedomain_t*)__loadsym(libmosek_handle,"MSK_appendprimalgeomeanconedomain",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appenddualgeomeanconedomain_ptr = (MSK_appenddualgeomeanconedomain_t*)__loadsym(libmosek_handle,"MSK_appenddualgeomeanconedomain",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appendprimalpowerconedomain_ptr = (MSK_appendprimalpowerconedomain_t*)__loadsym(libmosek_handle,"MSK_appendprimalpowerconedomain",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appendprimalpowerconedomainseq_ptr = (MSK_appendprimalpowerconedomainseq_t*)__loadsym(libmosek_handle,"MSK_appendprimalpowerconedomainseq",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appenddualpowerconedomain_ptr = (MSK_appenddualpowerconedomain_t*)__loadsym(libmosek_handle,"MSK_appenddualpowerconedomain",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appenddualpowerconedomainseq_ptr = (MSK_appenddualpowerconedomainseq_t*)__loadsym(libmosek_handle,"MSK_appenddualpowerconedomainseq",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appendsvecpsdconedomain_ptr = (MSK_appendsvecpsdconedomain_t*)__loadsym(libmosek_handle,"MSK_appendsvecpsdconedomain",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdomaintype_ptr = (MSK_getdomaintype_t*)__loadsym(libmosek_handle,"MSK_getdomaintype",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdomainn_ptr = (MSK_getdomainn_t*)__loadsym(libmosek_handle,"MSK_getdomainn",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getpowerdomaininfo_ptr = (MSK_getpowerdomaininfo_t*)__loadsym(libmosek_handle,"MSK_getpowerdomaininfo",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getpowerdomainalpha_ptr = (MSK_getpowerdomainalpha_t*)__loadsym(libmosek_handle,"MSK_getpowerdomainalpha",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appendsparsesymmat_ptr = (MSK_appendsparsesymmat_t*)__loadsym(libmosek_handle,"MSK_appendsparsesymmat",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_appendsparsesymmatlist_ptr = (MSK_appendsparsesymmatlist_t*)__loadsym(libmosek_handle,"MSK_appendsparsesymmatlist",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getsymmatinfo_ptr = (MSK_getsymmatinfo_t*)__loadsym(libmosek_handle,"MSK_getsymmatinfo",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getnumsymmat_ptr = (MSK_getnumsymmat_t*)__loadsym(libmosek_handle,"MSK_getnumsymmat",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getsparsesymmat_ptr = (MSK_getsparsesymmat_t*)__loadsym(libmosek_handle,"MSK_getsparsesymmat",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putdouparam_ptr = (MSK_putdouparam_t*)__loadsym(libmosek_handle,"MSK_putdouparam",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_resetdouparam_ptr = (MSK_resetdouparam_t*)__loadsym(libmosek_handle,"MSK_resetdouparam",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putintparam_ptr = (MSK_putintparam_t*)__loadsym(libmosek_handle,"MSK_putintparam",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putlintparam_ptr = (MSK_putlintparam_t*)__loadsym(libmosek_handle,"MSK_putlintparam",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_resetintparam_ptr = (MSK_resetintparam_t*)__loadsym(libmosek_handle,"MSK_resetintparam",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putmaxnumcon_ptr = (MSK_putmaxnumcon_t*)__loadsym(libmosek_handle,"MSK_putmaxnumcon",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putmaxnumcon64_ptr = (MSK_putmaxnumcon64_t*)__loadsym(libmosek_handle,"MSK_putmaxnumcon64",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putmaxnumcone_ptr = (MSK_putmaxnumcone_t*)__loadsym(libmosek_handle,"MSK_putmaxnumcone",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getmaxnumcone_ptr = (MSK_getmaxnumcone_t*)__loadsym(libmosek_handle,"MSK_getmaxnumcone",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putmaxnumvar_ptr = (MSK_putmaxnumvar_t*)__loadsym(libmosek_handle,"MSK_putmaxnumvar",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putmaxnumvar64_ptr = (MSK_putmaxnumvar64_t*)__loadsym(libmosek_handle,"MSK_putmaxnumvar64",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putmaxnumbarvar_ptr = (MSK_putmaxnumbarvar_t*)__loadsym(libmosek_handle,"MSK_putmaxnumbarvar",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putmaxnumanz_ptr = (MSK_putmaxnumanz_t*)__loadsym(libmosek_handle,"MSK_putmaxnumanz",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putmaxnumqnz_ptr = (MSK_putmaxnumqnz_t*)__loadsym(libmosek_handle,"MSK_putmaxnumqnz",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getmaxnumqnz_ptr = (MSK_getmaxnumqnz_t*)__loadsym(libmosek_handle,"MSK_getmaxnumqnz",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getmaxnumqnz64_ptr = (MSK_getmaxnumqnz64_t*)__loadsym(libmosek_handle,"MSK_getmaxnumqnz64",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putnadouparam_ptr = (MSK_putnadouparam_t*)__loadsym(libmosek_handle,"MSK_putnadouparam",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putnaintparam_ptr = (MSK_putnaintparam_t*)__loadsym(libmosek_handle,"MSK_putnaintparam",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putnastrparam_ptr = (MSK_putnastrparam_t*)__loadsym(libmosek_handle,"MSK_putnastrparam",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putobjname_ptr = (MSK_putobjname_t*)__loadsym(libmosek_handle,"MSK_putobjname",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putparam_ptr = (MSK_putparam_t*)__loadsym(libmosek_handle,"MSK_putparam",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putqcon_ptr = (MSK_putqcon_t*)__loadsym(libmosek_handle,"MSK_putqcon",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putqconk_ptr = (MSK_putqconk_t*)__loadsym(libmosek_handle,"MSK_putqconk",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putqobj_ptr = (MSK_putqobj_t*)__loadsym(libmosek_handle,"MSK_putqobj",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putqobjij_ptr = (MSK_putqobjij_t*)__loadsym(libmosek_handle,"MSK_putqobjij",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putsolution_ptr = (MSK_putsolution_t*)__loadsym(libmosek_handle,"MSK_putsolution",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putsolutionnew_ptr = (MSK_putsolutionnew_t*)__loadsym(libmosek_handle,"MSK_putsolutionnew",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putconsolutioni_ptr = (MSK_putconsolutioni_t*)__loadsym(libmosek_handle,"MSK_putconsolutioni",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putvarsolutionj_ptr = (MSK_putvarsolutionj_t*)__loadsym(libmosek_handle,"MSK_putvarsolutionj",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putsolutionyi_ptr = (MSK_putsolutionyi_t*)__loadsym(libmosek_handle,"MSK_putsolutionyi",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putstrparam_ptr = (MSK_putstrparam_t*)__loadsym(libmosek_handle,"MSK_putstrparam",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_resetstrparam_ptr = (MSK_resetstrparam_t*)__loadsym(libmosek_handle,"MSK_resetstrparam",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_puttaskname_ptr = (MSK_puttaskname_t*)__loadsym(libmosek_handle,"MSK_puttaskname",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putvartype_ptr = (MSK_putvartype_t*)__loadsym(libmosek_handle,"MSK_putvartype",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putvartypelist_ptr = (MSK_putvartypelist_t*)__loadsym(libmosek_handle,"MSK_putvartypelist",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_readdata_ptr = (MSK_readdata_t*)__loadsym(libmosek_handle,"MSK_readdata",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_readdatahandle_ptr = (MSK_readdatahandle_t*)__loadsym(libmosek_handle,"MSK_readdatahandle",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_writedatahandle_ptr = (MSK_writedatahandle_t*)__loadsym(libmosek_handle,"MSK_writedatahandle",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_readdataformat_ptr = (MSK_readdataformat_t*)__loadsym(libmosek_handle,"MSK_readdataformat",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_readdataautoformat_ptr = (MSK_readdataautoformat_t*)__loadsym(libmosek_handle,"MSK_readdataautoformat",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_readparamfile_ptr = (MSK_readparamfile_t*)__loadsym(libmosek_handle,"MSK_readparamfile",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_readsolution_ptr = (MSK_readsolution_t*)__loadsym(libmosek_handle,"MSK_readsolution",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_readjsonsol_ptr = (MSK_readjsonsol_t*)__loadsym(libmosek_handle,"MSK_readjsonsol",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_readsummary_ptr = (MSK_readsummary_t*)__loadsym(libmosek_handle,"MSK_readsummary",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_resizetask_ptr = (MSK_resizetask_t*)__loadsym(libmosek_handle,"MSK_resizetask",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_checkmemtask_ptr = (MSK_checkmemtask_t*)__loadsym(libmosek_handle,"MSK_checkmemtask",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getmemusagetask_ptr = (MSK_getmemusagetask_t*)__loadsym(libmosek_handle,"MSK_getmemusagetask",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_resetparameters_ptr = (MSK_resetparameters_t*)__loadsym(libmosek_handle,"MSK_resetparameters",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_sktostr_ptr = (MSK_sktostr_t*)__loadsym(libmosek_handle,"MSK_sktostr",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_solstatostr_ptr = (MSK_solstatostr_t*)__loadsym(libmosek_handle,"MSK_solstatostr",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_solutiondef_ptr = (MSK_solutiondef_t*)__loadsym(libmosek_handle,"MSK_solutiondef",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_deletesolution_ptr = (MSK_deletesolution_t*)__loadsym(libmosek_handle,"MSK_deletesolution",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_writestat_ptr = (MSK_writestat_t*)__loadsym(libmosek_handle,"MSK_writestat",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_writesolutionstat_ptr = (MSK_writesolutionstat_t*)__loadsym(libmosek_handle,"MSK_writesolutionstat",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_onesolutionsummary_ptr = (MSK_onesolutionsummary_t*)__loadsym(libmosek_handle,"MSK_onesolutionsummary",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_solutionsummary_ptr = (MSK_solutionsummary_t*)__loadsym(libmosek_handle,"MSK_solutionsummary",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_updatesolutioninfo_ptr = (MSK_updatesolutioninfo_t*)__loadsym(libmosek_handle,"MSK_updatesolutioninfo",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_optimizersummary_ptr = (MSK_optimizersummary_t*)__loadsym(libmosek_handle,"MSK_optimizersummary",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_strtoconetype_ptr = (MSK_strtoconetype_t*)__loadsym(libmosek_handle,"MSK_strtoconetype",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_strtosk_ptr = (MSK_strtosk_t*)__loadsym(libmosek_handle,"MSK_strtosk",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_whichparam_ptr = (MSK_whichparam_t*)__loadsym(libmosek_handle,"MSK_whichparam",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_writedata_ptr = (MSK_writedata_t*)__loadsym(libmosek_handle,"MSK_writedata",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_writetask_ptr = (MSK_writetask_t*)__loadsym(libmosek_handle,"MSK_writetask",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_writebsolution_ptr = (MSK_writebsolution_t*)__loadsym(libmosek_handle,"MSK_writebsolution",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_writebsolutionhandle_ptr = (MSK_writebsolutionhandle_t*)__loadsym(libmosek_handle,"MSK_writebsolutionhandle",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_readbupdate_ptr = (MSK_readbupdate_t*)__loadsym(libmosek_handle,"MSK_readbupdate",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_writeb_ptr = (MSK_writeb_t*)__loadsym(libmosek_handle,"MSK_writeb",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_readbupdatehandle_ptr = (MSK_readbupdatehandle_t*)__loadsym(libmosek_handle,"MSK_readbupdatehandle",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_readbsolution_ptr = (MSK_readbsolution_t*)__loadsym(libmosek_handle,"MSK_readbsolution",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_writesolutionfile_ptr = (MSK_writesolutionfile_t*)__loadsym(libmosek_handle,"MSK_writesolutionfile",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_readsolutionfile_ptr = (MSK_readsolutionfile_t*)__loadsym(libmosek_handle,"MSK_readsolutionfile",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_readtask_ptr = (MSK_readtask_t*)__loadsym(libmosek_handle,"MSK_readtask",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_readopfstring_ptr = (MSK_readopfstring_t*)__loadsym(libmosek_handle,"MSK_readopfstring",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_readlpstring_ptr = (MSK_readlpstring_t*)__loadsym(libmosek_handle,"MSK_readlpstring",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_readjsonstring_ptr = (MSK_readjsonstring_t*)__loadsym(libmosek_handle,"MSK_readjsonstring",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_readptfstring_ptr = (MSK_readptfstring_t*)__loadsym(libmosek_handle,"MSK_readptfstring",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_writeparamfile_ptr = (MSK_writeparamfile_t*)__loadsym(libmosek_handle,"MSK_writeparamfile",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getinfeasiblesubproblem_ptr = (MSK_getinfeasiblesubproblem_t*)__loadsym(libmosek_handle,"MSK_getinfeasiblesubproblem",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getdualproblem_ptr = (MSK_getdualproblem_t*)__loadsym(libmosek_handle,"MSK_getdualproblem",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getfixedproblem_ptr = (MSK_getfixedproblem_t*)__loadsym(libmosek_handle,"MSK_getfixedproblem",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_tofixedproblem_ptr = (MSK_tofixedproblem_t*)__loadsym(libmosek_handle,"MSK_tofixedproblem",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_writesolution_ptr = (MSK_writesolution_t*)__loadsym(libmosek_handle,"MSK_writesolution",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_writejsonsol_ptr = (MSK_writejsonsol_t*)__loadsym(libmosek_handle,"MSK_writejsonsol",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_primalsensitivity_ptr = (MSK_primalsensitivity_t*)__loadsym(libmosek_handle,"MSK_primalsensitivity",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_sensitivityreport_ptr = (MSK_sensitivityreport_t*)__loadsym(libmosek_handle,"MSK_sensitivityreport",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_dualsensitivity_ptr = (MSK_dualsensitivity_t*)__loadsym(libmosek_handle,"MSK_dualsensitivity",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getlasterror_ptr = (MSK_getlasterror_t*)__loadsym(libmosek_handle,"MSK_getlasterror",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getlasterror64_ptr = (MSK_getlasterror64_t*)__loadsym(libmosek_handle,"MSK_getlasterror64",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_writetasksolverresult_file_ptr = (MSK_writetasksolverresult_file_t*)__loadsym(libmosek_handle,"MSK_writetasksolverresult_file",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_optimizermt_ptr = (MSK_optimizermt_t*)__loadsym(libmosek_handle,"MSK_optimizermt",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_optimizecb_ptr = (MSK_optimizecb_t*)__loadsym(libmosek_handle,"MSK_optimizecb",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_asyncoptimizecb_ptr = (MSK_asyncoptimizecb_t*)__loadsym(libmosek_handle,"MSK_asyncoptimizecb",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_asyncgetlogcb_ptr = (MSK_asyncgetlogcb_t*)__loadsym(libmosek_handle,"MSK_asyncgetlogcb",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_asyncstopcb_ptr = (MSK_asyncstopcb_t*)__loadsym(libmosek_handle,"MSK_asyncstopcb",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_asyncgetresultcb_ptr = (MSK_asyncgetresultcb_t*)__loadsym(libmosek_handle,"MSK_asyncgetresultcb",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_asyncpollcb_ptr = (MSK_asyncpollcb_t*)__loadsym(libmosek_handle,"MSK_asyncpollcb",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_asyncoptimize_ptr = (MSK_asyncoptimize_t*)__loadsym(libmosek_handle,"MSK_asyncoptimize",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_asyncgetlog_ptr = (MSK_asyncgetlog_t*)__loadsym(libmosek_handle,"MSK_asyncgetlog",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_asyncstop_ptr = (MSK_asyncstop_t*)__loadsym(libmosek_handle,"MSK_asyncstop",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_asyncpoll_ptr = (MSK_asyncpoll_t*)__loadsym(libmosek_handle,"MSK_asyncpoll",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_asyncgetresult_ptr = (MSK_asyncgetresult_t*)__loadsym(libmosek_handle,"MSK_asyncgetresult",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putoptserverhost_ptr = (MSK_putoptserverhost_t*)__loadsym(libmosek_handle,"MSK_putoptserverhost",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_optimizebatch_ptr = (MSK_optimizebatch_t*)__loadsym(libmosek_handle,"MSK_optimizebatch",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_callbackcodetostr_ptr = (MSK_callbackcodetostr_t*)__loadsym(libmosek_handle,"MSK_callbackcodetostr",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_isinfinity_ptr = (MSK_isinfinity_t*)__loadsym(libmosek_handle,"MSK_isinfinity",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_enablegarcolenv_ptr = (MSK_enablegarcolenv_t*)__loadsym(libmosek_handle,"MSK_enablegarcolenv",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_makeenvdebug_ptr = (MSK_makeenvdebug_t*)__loadsym(libmosek_handle,"MSK_makeenvdebug",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_unittestmain_ptr = (MSK_unittestmain_t*)__loadsym(libmosek_handle,"MSK_unittestmain",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_runsimplexfromfile_ptr = (MSK_runsimplexfromfile_t*)__loadsym(libmosek_handle,"MSK_runsimplexfromfile",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_globalenvinitialize_ptr = (MSK_globalenvinitialize_t*)__loadsym(libmosek_handle,"MSK_globalenvinitialize",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_globalenvfinalize_ptr = (MSK_globalenvfinalize_t*)__loadsym(libmosek_handle,"MSK_globalenvfinalize",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_checkoutlicense_ptr = (MSK_checkoutlicense_t*)__loadsym(libmosek_handle,"MSK_checkoutlicense",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_checkinlicense_ptr = (MSK_checkinlicense_t*)__loadsym(libmosek_handle,"MSK_checkinlicense",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_checkinall_ptr = (MSK_checkinall_t*)__loadsym(libmosek_handle,"MSK_checkinall",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_expirylicenses_ptr = (MSK_expirylicenses_t*)__loadsym(libmosek_handle,"MSK_expirylicenses",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_resetexpirylicenses_ptr = (MSK_resetexpirylicenses_t*)__loadsym(libmosek_handle,"MSK_resetexpirylicenses",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getbuildinfo_ptr = (MSK_getbuildinfo_t*)__loadsym(libmosek_handle,"MSK_getbuildinfo",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getresponseclass_ptr = (MSK_getresponseclass_t*)__loadsym(libmosek_handle,"MSK_getresponseclass",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_callocenv_ptr = (MSK_callocenv_t*)__loadsym(libmosek_handle,"MSK_callocenv",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_callocdbgenv_ptr = (MSK_callocdbgenv_t*)__loadsym(libmosek_handle,"MSK_callocdbgenv",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_deleteenv_ptr = (MSK_deleteenv_t*)__loadsym(libmosek_handle,"MSK_deleteenv",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_echointro_ptr = (MSK_echointro_t*)__loadsym(libmosek_handle,"MSK_echointro",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_freeenv_ptr = (MSK_freeenv_t*)__loadsym(libmosek_handle,"MSK_freeenv",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_freedbgenv_ptr = (MSK_freedbgenv_t*)__loadsym(libmosek_handle,"MSK_freedbgenv",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getcodedesc_ptr = (MSK_getcodedesc_t*)__loadsym(libmosek_handle,"MSK_getcodedesc",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getsymbcondim_ptr = (MSK_getsymbcondim_t*)__loadsym(libmosek_handle,"MSK_getsymbcondim",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_rescodetostr_ptr = (MSK_rescodetostr_t*)__loadsym(libmosek_handle,"MSK_rescodetostr",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_iinfitemtostr_ptr = (MSK_iinfitemtostr_t*)__loadsym(libmosek_handle,"MSK_iinfitemtostr",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_dinfitemtostr_ptr = (MSK_dinfitemtostr_t*)__loadsym(libmosek_handle,"MSK_dinfitemtostr",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_liinfitemtostr_ptr = (MSK_liinfitemtostr_t*)__loadsym(libmosek_handle,"MSK_liinfitemtostr",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_getversion_ptr = (MSK_getversion_t*)__loadsym(libmosek_handle,"MSK_getversion",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_checkversion_ptr = (MSK_checkversion_t*)__loadsym(libmosek_handle,"MSK_checkversion",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_iparvaltosymnam_ptr = (MSK_iparvaltosymnam_t*)__loadsym(libmosek_handle,"MSK_iparvaltosymnam",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_linkfiletoenvstream_ptr = (MSK_linkfiletoenvstream_t*)__loadsym(libmosek_handle,"MSK_linkfiletoenvstream",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_linkfunctoenvstream_ptr = (MSK_linkfunctoenvstream_t*)__loadsym(libmosek_handle,"MSK_linkfunctoenvstream",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_unlinkfuncfromenvstream_ptr = (MSK_unlinkfuncfromenvstream_t*)__loadsym(libmosek_handle,"MSK_unlinkfuncfromenvstream",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_makeenv_ptr = (MSK_makeenv_t*)__loadsym(libmosek_handle,"MSK_makeenv",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putlicensedebug_ptr = (MSK_putlicensedebug_t*)__loadsym(libmosek_handle,"MSK_putlicensedebug",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putlicensecode_ptr = (MSK_putlicensecode_t*)__loadsym(libmosek_handle,"MSK_putlicensecode",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putlicensewait_ptr = (MSK_putlicensewait_t*)__loadsym(libmosek_handle,"MSK_putlicensewait",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putlicensepath_ptr = (MSK_putlicensepath_t*)__loadsym(libmosek_handle,"MSK_putlicensepath",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_maketask_ptr = (MSK_maketask_t*)__loadsym(libmosek_handle,"MSK_maketask",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_makeemptytask_ptr = (MSK_makeemptytask_t*)__loadsym(libmosek_handle,"MSK_makeemptytask",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_putexitfunc_ptr = (MSK_putexitfunc_t*)__loadsym(libmosek_handle,"MSK_putexitfunc",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_utf8towchar_ptr = (MSK_utf8towchar_t*)__loadsym(libmosek_handle,"MSK_utf8towchar",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_wchartoutf8_ptr = (MSK_wchartoutf8_t*)__loadsym(libmosek_handle,"MSK_wchartoutf8",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_checkmemenv_ptr = (MSK_checkmemenv_t*)__loadsym(libmosek_handle,"MSK_checkmemenv",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_symnamtovalue_ptr = (MSK_symnamtovalue_t*)__loadsym(libmosek_handle,"MSK_symnamtovalue",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_axpy_ptr = (MSK_axpy_t*)__loadsym(libmosek_handle,"MSK_axpy",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_dot_ptr = (MSK_dot_t*)__loadsym(libmosek_handle,"MSK_dot",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_gemv_ptr = (MSK_gemv_t*)__loadsym(libmosek_handle,"MSK_gemv",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_gemm_ptr = (MSK_gemm_t*)__loadsym(libmosek_handle,"MSK_gemm",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_syrk_ptr = (MSK_syrk_t*)__loadsym(libmosek_handle,"MSK_syrk",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_computesparsecholesky_ptr = (MSK_computesparsecholesky_t*)__loadsym(libmosek_handle,"MSK_computesparsecholesky",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_sparsetriangularsolvedense_ptr = (MSK_sparsetriangularsolvedense_t*)__loadsym(libmosek_handle,"MSK_sparsetriangularsolvedense",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_potrf_ptr = (MSK_potrf_t*)__loadsym(libmosek_handle,"MSK_potrf",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_syeig_ptr = (MSK_syeig_t*)__loadsym(libmosek_handle,"MSK_syeig",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_syevd_ptr = (MSK_syevd_t*)__loadsym(libmosek_handle,"MSK_syevd",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_licensecleanup_ptr = (MSK_licensecleanup_t*)__loadsym(libmosek_handle,"MSK_licensecleanup",&errmsg))) goto EXIT_ERROR;
        if (NULL == (MSK_shutdownglobalthreadpool_ptr = (MSK_shutdownglobalthreadpool_t*)__loadsym(libmosek_handle,"MSK_shutdownglobalthreadpool",&errmsg))) goto EXIT_ERROR;

    }

    goto EXIT_OK;
EXIT_ERROR:
    if (libmosek_handle) {
        __dlclose(libmosek_handle);
        libmosek_handle = NULL;
        MSK_analyzeproblem_ptr = (MSK_analyzeproblem_t*)default_ret_0i32;
        MSK_analyzenames_ptr = (MSK_analyzenames_t*)default_ret_0i32;
        MSK_analyzesolution_ptr = (MSK_analyzesolution_t*)default_ret_0i32;
        MSK_initbasissolve_ptr = (MSK_initbasissolve_t*)default_ret_0i32;
        MSK_solvewithbasis_ptr = (MSK_solvewithbasis_t*)default_ret_0i32;
        MSK_basiscond_ptr = (MSK_basiscond_t*)default_ret_0i32;
        MSK_appendcons_ptr = (MSK_appendcons_t*)default_ret_0i32;
        MSK_appendvars_ptr = (MSK_appendvars_t*)default_ret_0i32;
        MSK_removecons_ptr = (MSK_removecons_t*)default_ret_0i32;
        MSK_removevars_ptr = (MSK_removevars_t*)default_ret_0i32;
        MSK_removebarvars_ptr = (MSK_removebarvars_t*)default_ret_0i32;
        MSK_removecones_ptr = (MSK_removecones_t*)default_ret_0i32;
        MSK_appendbarvars_ptr = (MSK_appendbarvars_t*)default_ret_0i32;
        MSK_appendcone_ptr = (MSK_appendcone_t*)default_ret_0i32;
        MSK_appendconeseq_ptr = (MSK_appendconeseq_t*)default_ret_0i32;
        MSK_appendconesseq_ptr = (MSK_appendconesseq_t*)default_ret_0i32;
        MSK_bktostr_ptr = (MSK_bktostr_t*)default_ret_0i32;
        MSK_calloctask_ptr = (MSK_calloctask_t*)default_ret_ptr;
        MSK_callocdbgtask_ptr = (MSK_callocdbgtask_t*)default_ret_ptr;
        MSK_chgconbound_ptr = (MSK_chgconbound_t*)default_ret_0i32;
        MSK_chgvarbound_ptr = (MSK_chgvarbound_t*)default_ret_0i32;
        MSK_conetypetostr_ptr = (MSK_conetypetostr_t*)default_ret_0i32;
        MSK_deletetask_ptr = (MSK_deletetask_t*)default_ret_0i32;
        MSK_freetask_ptr = (MSK_freetask_t*)default_ret_void;
        MSK_freedbgtask_ptr = (MSK_freedbgtask_t*)default_ret_void;
        MSK_getaij_ptr = (MSK_getaij_t*)default_ret_0i32;
        MSK_getapiecenumnz_ptr = (MSK_getapiecenumnz_t*)default_ret_0i32;
        MSK_getacolnumnz_ptr = (MSK_getacolnumnz_t*)default_ret_0i32;
        MSK_getacol_ptr = (MSK_getacol_t*)default_ret_0i32;
        MSK_getacolslice_ptr = (MSK_getacolslice_t*)default_ret_0i32;
        MSK_getacolslice64_ptr = (MSK_getacolslice64_t*)default_ret_0i32;
        MSK_getarownumnz_ptr = (MSK_getarownumnz_t*)default_ret_0i32;
        MSK_getarow_ptr = (MSK_getarow_t*)default_ret_0i32;
        MSK_getacolslicenumnz_ptr = (MSK_getacolslicenumnz_t*)default_ret_0i32;
        MSK_getacolslicenumnz64_ptr = (MSK_getacolslicenumnz64_t*)default_ret_0i32;
        MSK_getarowslicenumnz_ptr = (MSK_getarowslicenumnz_t*)default_ret_0i32;
        MSK_getarowslicenumnz64_ptr = (MSK_getarowslicenumnz64_t*)default_ret_0i32;
        MSK_getarowslice_ptr = (MSK_getarowslice_t*)default_ret_0i32;
        MSK_getarowslice64_ptr = (MSK_getarowslice64_t*)default_ret_0i32;
        MSK_getatrip_ptr = (MSK_getatrip_t*)default_ret_0i32;
        MSK_getarowslicetrip_ptr = (MSK_getarowslicetrip_t*)default_ret_0i32;
        MSK_getacolslicetrip_ptr = (MSK_getacolslicetrip_t*)default_ret_0i32;
        MSK_getconbound_ptr = (MSK_getconbound_t*)default_ret_0i32;
        MSK_getvarbound_ptr = (MSK_getvarbound_t*)default_ret_0i32;
        MSK_getconboundslice_ptr = (MSK_getconboundslice_t*)default_ret_0i32;
        MSK_getvarboundslice_ptr = (MSK_getvarboundslice_t*)default_ret_0i32;
        MSK_getcj_ptr = (MSK_getcj_t*)default_ret_0i32;
        MSK_getc_ptr = (MSK_getc_t*)default_ret_0i32;
        MSK_getcallbackfunc_ptr = (MSK_getcallbackfunc_t*)default_ret_0i32;
        MSK_getcfix_ptr = (MSK_getcfix_t*)default_ret_0i32;
        MSK_getcone_ptr = (MSK_getcone_t*)default_ret_0i32;
        MSK_getconeinfo_ptr = (MSK_getconeinfo_t*)default_ret_0i32;
        MSK_getclist_ptr = (MSK_getclist_t*)default_ret_0i32;
        MSK_getcslice_ptr = (MSK_getcslice_t*)default_ret_0i32;
        MSK_getdouinf_ptr = (MSK_getdouinf_t*)default_ret_0i32;
        MSK_getdouparam_ptr = (MSK_getdouparam_t*)default_ret_0i32;
        MSK_getdualobj_ptr = (MSK_getdualobj_t*)default_ret_0i32;
        MSK_getenv_ptr = (MSK_getenv_t*)default_ret_0i32;
        MSK_getinfindex_ptr = (MSK_getinfindex_t*)default_ret_0i32;
        MSK_getinfmax_ptr = (MSK_getinfmax_t*)default_ret_0i32;
        MSK_getinfname_ptr = (MSK_getinfname_t*)default_ret_0i32;
        MSK_getintinf_ptr = (MSK_getintinf_t*)default_ret_0i32;
        MSK_getlintinf_ptr = (MSK_getlintinf_t*)default_ret_0i32;
        MSK_getintparam_ptr = (MSK_getintparam_t*)default_ret_0i32;
        MSK_getlintparam_ptr = (MSK_getlintparam_t*)default_ret_0i32;
        MSK_getmaxnamelen_ptr = (MSK_getmaxnamelen_t*)default_ret_0i32;
        MSK_getmaxnumanz_ptr = (MSK_getmaxnumanz_t*)default_ret_0i32;
        MSK_getmaxnumanz64_ptr = (MSK_getmaxnumanz64_t*)default_ret_0i32;
        MSK_getmaxnumcon_ptr = (MSK_getmaxnumcon_t*)default_ret_0i32;
        MSK_getmaxnumvar_ptr = (MSK_getmaxnumvar_t*)default_ret_0i32;
        MSK_getnadouinf_ptr = (MSK_getnadouinf_t*)default_ret_0i32;
        MSK_getnadouparam_ptr = (MSK_getnadouparam_t*)default_ret_0i32;
        MSK_getnaintinf_ptr = (MSK_getnaintinf_t*)default_ret_0i32;
        MSK_getnaintparam_ptr = (MSK_getnaintparam_t*)default_ret_0i32;
        MSK_getbarvarnamelen_ptr = (MSK_getbarvarnamelen_t*)default_ret_0i32;
        MSK_getbarvarname_ptr = (MSK_getbarvarname_t*)default_ret_0i32;
        MSK_getbarvarnameindex_ptr = (MSK_getbarvarnameindex_t*)default_ret_0i32;
        MSK_generatebarvarnames_ptr = (MSK_generatebarvarnames_t*)default_ret_0i32;
        MSK_generatevarnames_ptr = (MSK_generatevarnames_t*)default_ret_0i32;
        MSK_generateconnames_ptr = (MSK_generateconnames_t*)default_ret_0i32;
        MSK_generateconenames_ptr = (MSK_generateconenames_t*)default_ret_0i32;
        MSK_generateaccnames_ptr = (MSK_generateaccnames_t*)default_ret_0i32;
        MSK_generatedjcnames_ptr = (MSK_generatedjcnames_t*)default_ret_0i32;
        MSK_putconname_ptr = (MSK_putconname_t*)default_ret_0i32;
        MSK_putvarname_ptr = (MSK_putvarname_t*)default_ret_0i32;
        MSK_putconename_ptr = (MSK_putconename_t*)default_ret_0i32;
        MSK_putbarvarname_ptr = (MSK_putbarvarname_t*)default_ret_0i32;
        MSK_putdomainname_ptr = (MSK_putdomainname_t*)default_ret_0i32;
        MSK_putdjcname_ptr = (MSK_putdjcname_t*)default_ret_0i32;
        MSK_putaccname_ptr = (MSK_putaccname_t*)default_ret_0i32;
        MSK_getvarnamelen_ptr = (MSK_getvarnamelen_t*)default_ret_0i32;
        MSK_getvarname_ptr = (MSK_getvarname_t*)default_ret_0i32;
        MSK_getconnamelen_ptr = (MSK_getconnamelen_t*)default_ret_0i32;
        MSK_getconname_ptr = (MSK_getconname_t*)default_ret_0i32;
        MSK_getconnameindex_ptr = (MSK_getconnameindex_t*)default_ret_0i32;
        MSK_getvarnameindex_ptr = (MSK_getvarnameindex_t*)default_ret_0i32;
        MSK_getconenamelen_ptr = (MSK_getconenamelen_t*)default_ret_0i32;
        MSK_getconename_ptr = (MSK_getconename_t*)default_ret_0i32;
        MSK_getconenameindex_ptr = (MSK_getconenameindex_t*)default_ret_0i32;
        MSK_getdomainnamelen_ptr = (MSK_getdomainnamelen_t*)default_ret_0i32;
        MSK_getdomainname_ptr = (MSK_getdomainname_t*)default_ret_0i32;
        MSK_getdjcnamelen_ptr = (MSK_getdjcnamelen_t*)default_ret_0i32;
        MSK_getdjcname_ptr = (MSK_getdjcname_t*)default_ret_0i32;
        MSK_getaccnamelen_ptr = (MSK_getaccnamelen_t*)default_ret_0i32;
        MSK_getaccname_ptr = (MSK_getaccname_t*)default_ret_0i32;
        MSK_getnastrparam_ptr = (MSK_getnastrparam_t*)default_ret_0i32;
        MSK_getnumanz_ptr = (MSK_getnumanz_t*)default_ret_0i32;
        MSK_getnumanz64_ptr = (MSK_getnumanz64_t*)default_ret_0i32;
        MSK_getnumcon_ptr = (MSK_getnumcon_t*)default_ret_0i32;
        MSK_getnumcone_ptr = (MSK_getnumcone_t*)default_ret_0i32;
        MSK_getnumconemem_ptr = (MSK_getnumconemem_t*)default_ret_0i32;
        MSK_getnumintvar_ptr = (MSK_getnumintvar_t*)default_ret_0i32;
        MSK_getnumparam_ptr = (MSK_getnumparam_t*)default_ret_0i32;
        MSK_getnumqconknz_ptr = (MSK_getnumqconknz_t*)default_ret_0i32;
        MSK_getnumqconknz64_ptr = (MSK_getnumqconknz64_t*)default_ret_0i32;
        MSK_getnumqobjnz_ptr = (MSK_getnumqobjnz_t*)default_ret_0i32;
        MSK_getnumqobjnz64_ptr = (MSK_getnumqobjnz64_t*)default_ret_0i32;
        MSK_getnumvar_ptr = (MSK_getnumvar_t*)default_ret_0i32;
        MSK_getnumbarvar_ptr = (MSK_getnumbarvar_t*)default_ret_0i32;
        MSK_getmaxnumbarvar_ptr = (MSK_getmaxnumbarvar_t*)default_ret_0i32;
        MSK_getdimbarvarj_ptr = (MSK_getdimbarvarj_t*)default_ret_0i32;
        MSK_getlenbarvarj_ptr = (MSK_getlenbarvarj_t*)default_ret_0i32;
        MSK_getobjname_ptr = (MSK_getobjname_t*)default_ret_0i32;
        MSK_getobjnamelen_ptr = (MSK_getobjnamelen_t*)default_ret_0i32;
        MSK_getparamname_ptr = (MSK_getparamname_t*)default_ret_0i32;
        MSK_getparammax_ptr = (MSK_getparammax_t*)default_ret_0i32;
        MSK_getprimalobj_ptr = (MSK_getprimalobj_t*)default_ret_0i32;
        MSK_getprobtype_ptr = (MSK_getprobtype_t*)default_ret_0i32;
        MSK_getqconk64_ptr = (MSK_getqconk64_t*)default_ret_0i32;
        MSK_getqconk_ptr = (MSK_getqconk_t*)default_ret_0i32;
        MSK_getqobj_ptr = (MSK_getqobj_t*)default_ret_0i32;
        MSK_getqobj64_ptr = (MSK_getqobj64_t*)default_ret_0i32;
        MSK_getqobjij_ptr = (MSK_getqobjij_t*)default_ret_0i32;
        MSK_getsolution_ptr = (MSK_getsolution_t*)default_ret_0i32;
        MSK_getsolutionnew_ptr = (MSK_getsolutionnew_t*)default_ret_0i32;
        MSK_getsolsta_ptr = (MSK_getsolsta_t*)default_ret_0i32;
        MSK_getprosta_ptr = (MSK_getprosta_t*)default_ret_0i32;
        MSK_getskc_ptr = (MSK_getskc_t*)default_ret_0i32;
        MSK_getskx_ptr = (MSK_getskx_t*)default_ret_0i32;
        MSK_getskn_ptr = (MSK_getskn_t*)default_ret_0i32;
        MSK_getxc_ptr = (MSK_getxc_t*)default_ret_0i32;
        MSK_getxx_ptr = (MSK_getxx_t*)default_ret_0i32;
        MSK_gety_ptr = (MSK_gety_t*)default_ret_0i32;
        MSK_getslc_ptr = (MSK_getslc_t*)default_ret_0i32;
        MSK_getaccdoty_ptr = (MSK_getaccdoty_t*)default_ret_0i32;
        MSK_getaccdotys_ptr = (MSK_getaccdotys_t*)default_ret_0i32;
        MSK_evaluateacc_ptr = (MSK_evaluateacc_t*)default_ret_0i32;
        MSK_evaluateaccs_ptr = (MSK_evaluateaccs_t*)default_ret_0i32;
        MSK_getsuc_ptr = (MSK_getsuc_t*)default_ret_0i32;
        MSK_getslx_ptr = (MSK_getslx_t*)default_ret_0i32;
        MSK_getsux_ptr = (MSK_getsux_t*)default_ret_0i32;
        MSK_getsnx_ptr = (MSK_getsnx_t*)default_ret_0i32;
        MSK_getskcslice_ptr = (MSK_getskcslice_t*)default_ret_0i32;
        MSK_getskxslice_ptr = (MSK_getskxslice_t*)default_ret_0i32;
        MSK_getxcslice_ptr = (MSK_getxcslice_t*)default_ret_0i32;
        MSK_getxxslice_ptr = (MSK_getxxslice_t*)default_ret_0i32;
        MSK_getyslice_ptr = (MSK_getyslice_t*)default_ret_0i32;
        MSK_getslcslice_ptr = (MSK_getslcslice_t*)default_ret_0i32;
        MSK_getsucslice_ptr = (MSK_getsucslice_t*)default_ret_0i32;
        MSK_getslxslice_ptr = (MSK_getslxslice_t*)default_ret_0i32;
        MSK_getsuxslice_ptr = (MSK_getsuxslice_t*)default_ret_0i32;
        MSK_getsnxslice_ptr = (MSK_getsnxslice_t*)default_ret_0i32;
        MSK_getbarxj_ptr = (MSK_getbarxj_t*)default_ret_0i32;
        MSK_getbarxslice_ptr = (MSK_getbarxslice_t*)default_ret_0i32;
        MSK_getbarsj_ptr = (MSK_getbarsj_t*)default_ret_0i32;
        MSK_getbarsslice_ptr = (MSK_getbarsslice_t*)default_ret_0i32;
        MSK_putskc_ptr = (MSK_putskc_t*)default_ret_0i32;
        MSK_putskx_ptr = (MSK_putskx_t*)default_ret_0i32;
        MSK_putxc_ptr = (MSK_putxc_t*)default_ret_0i32;
        MSK_putxx_ptr = (MSK_putxx_t*)default_ret_0i32;
        MSK_puty_ptr = (MSK_puty_t*)default_ret_0i32;
        MSK_putslc_ptr = (MSK_putslc_t*)default_ret_0i32;
        MSK_putsuc_ptr = (MSK_putsuc_t*)default_ret_0i32;
        MSK_putslx_ptr = (MSK_putslx_t*)default_ret_0i32;
        MSK_putsux_ptr = (MSK_putsux_t*)default_ret_0i32;
        MSK_putsnx_ptr = (MSK_putsnx_t*)default_ret_0i32;
        MSK_putaccdoty_ptr = (MSK_putaccdoty_t*)default_ret_0i32;
        MSK_putskcslice_ptr = (MSK_putskcslice_t*)default_ret_0i32;
        MSK_putskxslice_ptr = (MSK_putskxslice_t*)default_ret_0i32;
        MSK_putxcslice_ptr = (MSK_putxcslice_t*)default_ret_0i32;
        MSK_putxxslice_ptr = (MSK_putxxslice_t*)default_ret_0i32;
        MSK_putyslice_ptr = (MSK_putyslice_t*)default_ret_0i32;
        MSK_putslcslice_ptr = (MSK_putslcslice_t*)default_ret_0i32;
        MSK_putsucslice_ptr = (MSK_putsucslice_t*)default_ret_0i32;
        MSK_putslxslice_ptr = (MSK_putslxslice_t*)default_ret_0i32;
        MSK_putsuxslice_ptr = (MSK_putsuxslice_t*)default_ret_0i32;
        MSK_putsnxslice_ptr = (MSK_putsnxslice_t*)default_ret_0i32;
        MSK_putbarxj_ptr = (MSK_putbarxj_t*)default_ret_0i32;
        MSK_putbarsj_ptr = (MSK_putbarsj_t*)default_ret_0i32;
        MSK_getpviolcon_ptr = (MSK_getpviolcon_t*)default_ret_0i32;
        MSK_getpviolvar_ptr = (MSK_getpviolvar_t*)default_ret_0i32;
        MSK_getpviolbarvar_ptr = (MSK_getpviolbarvar_t*)default_ret_0i32;
        MSK_getpviolcones_ptr = (MSK_getpviolcones_t*)default_ret_0i32;
        MSK_getpviolacc_ptr = (MSK_getpviolacc_t*)default_ret_0i32;
        MSK_getpvioldjc_ptr = (MSK_getpvioldjc_t*)default_ret_0i32;
        MSK_getdviolcon_ptr = (MSK_getdviolcon_t*)default_ret_0i32;
        MSK_getdviolvar_ptr = (MSK_getdviolvar_t*)default_ret_0i32;
        MSK_getdviolbarvar_ptr = (MSK_getdviolbarvar_t*)default_ret_0i32;
        MSK_getdviolcones_ptr = (MSK_getdviolcones_t*)default_ret_0i32;
        MSK_getdviolacc_ptr = (MSK_getdviolacc_t*)default_ret_0i32;
        MSK_getsolutioninfo_ptr = (MSK_getsolutioninfo_t*)default_ret_0i32;
        MSK_getsolutioninfonew_ptr = (MSK_getsolutioninfonew_t*)default_ret_0i32;
        MSK_getdualsolutionnorms_ptr = (MSK_getdualsolutionnorms_t*)default_ret_0i32;
        MSK_getprimalsolutionnorms_ptr = (MSK_getprimalsolutionnorms_t*)default_ret_0i32;
        MSK_getsolutionslice_ptr = (MSK_getsolutionslice_t*)default_ret_0i32;
        MSK_getreducedcosts_ptr = (MSK_getreducedcosts_t*)default_ret_0i32;
        MSK_getstrparam_ptr = (MSK_getstrparam_t*)default_ret_0i32;
        MSK_getstrparamlen_ptr = (MSK_getstrparamlen_t*)default_ret_0i32;
        MSK_getstrparamal_ptr = (MSK_getstrparamal_t*)default_ret_0i32;
        MSK_getnastrparamal_ptr = (MSK_getnastrparamal_t*)default_ret_0i32;
        MSK_getsymbcon_ptr = (MSK_getsymbcon_t*)default_ret_0i32;
        MSK_gettasknamelen_ptr = (MSK_gettasknamelen_t*)default_ret_0i32;
        MSK_gettaskname_ptr = (MSK_gettaskname_t*)default_ret_0i32;
        MSK_getmionumthreads_ptr = (MSK_getmionumthreads_t*)default_ret_0i32;
        MSK_getvartype_ptr = (MSK_getvartype_t*)default_ret_0i32;
        MSK_getvartypelist_ptr = (MSK_getvartypelist_t*)default_ret_0i32;
        MSK_inputdata_ptr = (MSK_inputdata_t*)default_ret_0i32;
        MSK_inputdata64_ptr = (MSK_inputdata64_t*)default_ret_0i32;
        MSK_isdouparname_ptr = (MSK_isdouparname_t*)default_ret_0i32;
        MSK_isintparname_ptr = (MSK_isintparname_t*)default_ret_0i32;
        MSK_isstrparname_ptr = (MSK_isstrparname_t*)default_ret_0i32;
        MSK_linkfiletotaskstream_ptr = (MSK_linkfiletotaskstream_t*)default_ret_0i32;
        MSK_linkfunctotaskstream_ptr = (MSK_linkfunctotaskstream_t*)default_ret_0i32;
        MSK_unlinkfuncfromtaskstream_ptr = (MSK_unlinkfuncfromtaskstream_t*)default_ret_0i32;
        MSK_clonetask_ptr = (MSK_clonetask_t*)default_ret_0i32;
        MSK_primalrepair_ptr = (MSK_primalrepair_t*)default_ret_0i32;
        MSK_infeasibilityreport_ptr = (MSK_infeasibilityreport_t*)default_ret_0i32;
        MSK_toconic_ptr = (MSK_toconic_t*)default_ret_0i32;
        MSK_optimize_ptr = (MSK_optimize_t*)default_ret_0i32;
        MSK_optimizetrm_ptr = (MSK_optimizetrm_t*)default_ret_0i32;
        MSK_printparam_ptr = (MSK_printparam_t*)default_ret_0i32;
        MSK_probtypetostr_ptr = (MSK_probtypetostr_t*)default_ret_0i32;
        MSK_prostatostr_ptr = (MSK_prostatostr_t*)default_ret_0i32;
        MSK_putresponsefunc_ptr = (MSK_putresponsefunc_t*)default_ret_0i32;
        MSK_commitchanges_ptr = (MSK_commitchanges_t*)default_ret_0i32;
        MSK_getatruncatetol_ptr = (MSK_getatruncatetol_t*)default_ret_0i32;
        MSK_putatruncatetol_ptr = (MSK_putatruncatetol_t*)default_ret_0i32;
        MSK_putaij_ptr = (MSK_putaij_t*)default_ret_0i32;
        MSK_putaijlist_ptr = (MSK_putaijlist_t*)default_ret_0i32;
        MSK_putaijlist64_ptr = (MSK_putaijlist64_t*)default_ret_0i32;
        MSK_putacol_ptr = (MSK_putacol_t*)default_ret_0i32;
        MSK_putarow_ptr = (MSK_putarow_t*)default_ret_0i32;
        MSK_putarowslice_ptr = (MSK_putarowslice_t*)default_ret_0i32;
        MSK_putarowslice64_ptr = (MSK_putarowslice64_t*)default_ret_0i32;
        MSK_putarowlist_ptr = (MSK_putarowlist_t*)default_ret_0i32;
        MSK_putarowlist64_ptr = (MSK_putarowlist64_t*)default_ret_0i32;
        MSK_putacolslice_ptr = (MSK_putacolslice_t*)default_ret_0i32;
        MSK_putacolslice64_ptr = (MSK_putacolslice64_t*)default_ret_0i32;
        MSK_putacollist_ptr = (MSK_putacollist_t*)default_ret_0i32;
        MSK_putacollist64_ptr = (MSK_putacollist64_t*)default_ret_0i32;
        MSK_putbaraij_ptr = (MSK_putbaraij_t*)default_ret_0i32;
        MSK_putbaraijlist_ptr = (MSK_putbaraijlist_t*)default_ret_0i32;
        MSK_putbararowlist_ptr = (MSK_putbararowlist_t*)default_ret_0i32;
        MSK_getnumbarcnz_ptr = (MSK_getnumbarcnz_t*)default_ret_0i32;
        MSK_getnumbaranz_ptr = (MSK_getnumbaranz_t*)default_ret_0i32;
        MSK_getbarcsparsity_ptr = (MSK_getbarcsparsity_t*)default_ret_0i32;
        MSK_getbarasparsity_ptr = (MSK_getbarasparsity_t*)default_ret_0i32;
        MSK_getbarcidxinfo_ptr = (MSK_getbarcidxinfo_t*)default_ret_0i32;
        MSK_getbarcidxj_ptr = (MSK_getbarcidxj_t*)default_ret_0i32;
        MSK_getbarcidx_ptr = (MSK_getbarcidx_t*)default_ret_0i32;
        MSK_getbaraidxinfo_ptr = (MSK_getbaraidxinfo_t*)default_ret_0i32;
        MSK_getbaraidxij_ptr = (MSK_getbaraidxij_t*)default_ret_0i32;
        MSK_getbaraidx_ptr = (MSK_getbaraidx_t*)default_ret_0i32;
        MSK_getnumbarcblocktriplets_ptr = (MSK_getnumbarcblocktriplets_t*)default_ret_0i32;
        MSK_putbarcblocktriplet_ptr = (MSK_putbarcblocktriplet_t*)default_ret_0i32;
        MSK_getbarcblocktriplet_ptr = (MSK_getbarcblocktriplet_t*)default_ret_0i32;
        MSK_putbarablocktriplet_ptr = (MSK_putbarablocktriplet_t*)default_ret_0i32;
        MSK_getnumbarablocktriplets_ptr = (MSK_getnumbarablocktriplets_t*)default_ret_0i32;
        MSK_getbarablocktriplet_ptr = (MSK_getbarablocktriplet_t*)default_ret_0i32;
        MSK_putmaxnumafe_ptr = (MSK_putmaxnumafe_t*)default_ret_0i32;
        MSK_getnumafe_ptr = (MSK_getnumafe_t*)default_ret_0i32;
        MSK_appendafes_ptr = (MSK_appendafes_t*)default_ret_0i32;
        MSK_putafefentry_ptr = (MSK_putafefentry_t*)default_ret_0i32;
        MSK_putafefentrylist_ptr = (MSK_putafefentrylist_t*)default_ret_0i32;
        MSK_emptyafefrow_ptr = (MSK_emptyafefrow_t*)default_ret_0i32;
        MSK_emptyafefcol_ptr = (MSK_emptyafefcol_t*)default_ret_0i32;
        MSK_emptyafefrowlist_ptr = (MSK_emptyafefrowlist_t*)default_ret_0i32;
        MSK_emptyafefcollist_ptr = (MSK_emptyafefcollist_t*)default_ret_0i32;
        MSK_putafefrow_ptr = (MSK_putafefrow_t*)default_ret_0i32;
        MSK_putafefrowlist_ptr = (MSK_putafefrowlist_t*)default_ret_0i32;
        MSK_putafefcol_ptr = (MSK_putafefcol_t*)default_ret_0i32;
        MSK_getafefrownumnz_ptr = (MSK_getafefrownumnz_t*)default_ret_0i32;
        MSK_getafefnumnz_ptr = (MSK_getafefnumnz_t*)default_ret_0i32;
        MSK_getafefrow_ptr = (MSK_getafefrow_t*)default_ret_0i32;
        MSK_getafeftrip_ptr = (MSK_getafeftrip_t*)default_ret_0i32;
        MSK_putafebarfentry_ptr = (MSK_putafebarfentry_t*)default_ret_0i32;
        MSK_putafebarfentrylist_ptr = (MSK_putafebarfentrylist_t*)default_ret_0i32;
        MSK_putafebarfrow_ptr = (MSK_putafebarfrow_t*)default_ret_0i32;
        MSK_emptyafebarfrow_ptr = (MSK_emptyafebarfrow_t*)default_ret_0i32;
        MSK_emptyafebarfrowlist_ptr = (MSK_emptyafebarfrowlist_t*)default_ret_0i32;
        MSK_putafebarfblocktriplet_ptr = (MSK_putafebarfblocktriplet_t*)default_ret_0i32;
        MSK_getafebarfnumblocktriplets_ptr = (MSK_getafebarfnumblocktriplets_t*)default_ret_0i32;
        MSK_getafebarfblocktriplet_ptr = (MSK_getafebarfblocktriplet_t*)default_ret_0i32;
        MSK_getafebarfnumrowentries_ptr = (MSK_getafebarfnumrowentries_t*)default_ret_0i32;
        MSK_getafebarfrowinfo_ptr = (MSK_getafebarfrowinfo_t*)default_ret_0i32;
        MSK_getafebarfrow_ptr = (MSK_getafebarfrow_t*)default_ret_0i32;
        MSK_putafeg_ptr = (MSK_putafeg_t*)default_ret_0i32;
        MSK_putafeglist_ptr = (MSK_putafeglist_t*)default_ret_0i32;
        MSK_getafeg_ptr = (MSK_getafeg_t*)default_ret_0i32;
        MSK_getafegslice_ptr = (MSK_getafegslice_t*)default_ret_0i32;
        MSK_putafegslice_ptr = (MSK_putafegslice_t*)default_ret_0i32;
        MSK_putmaxnumdjc_ptr = (MSK_putmaxnumdjc_t*)default_ret_0i32;
        MSK_getnumdjc_ptr = (MSK_getnumdjc_t*)default_ret_0i32;
        MSK_getdjcnumdomain_ptr = (MSK_getdjcnumdomain_t*)default_ret_0i32;
        MSK_getdjcnumdomaintot_ptr = (MSK_getdjcnumdomaintot_t*)default_ret_0i32;
        MSK_getdjcnumafe_ptr = (MSK_getdjcnumafe_t*)default_ret_0i32;
        MSK_getdjcnumafetot_ptr = (MSK_getdjcnumafetot_t*)default_ret_0i32;
        MSK_getdjcnumterm_ptr = (MSK_getdjcnumterm_t*)default_ret_0i32;
        MSK_getdjcnumtermtot_ptr = (MSK_getdjcnumtermtot_t*)default_ret_0i32;
        MSK_putmaxnumacc_ptr = (MSK_putmaxnumacc_t*)default_ret_0i32;
        MSK_getnumacc_ptr = (MSK_getnumacc_t*)default_ret_0i32;
        MSK_appendacc_ptr = (MSK_appendacc_t*)default_ret_0i32;
        MSK_appendaccs_ptr = (MSK_appendaccs_t*)default_ret_0i32;
        MSK_appendaccseq_ptr = (MSK_appendaccseq_t*)default_ret_0i32;
        MSK_appendaccsseq_ptr = (MSK_appendaccsseq_t*)default_ret_0i32;
        MSK_putacc_ptr = (MSK_putacc_t*)default_ret_0i32;
        MSK_putacclist_ptr = (MSK_putacclist_t*)default_ret_0i32;
        MSK_putaccb_ptr = (MSK_putaccb_t*)default_ret_0i32;
        MSK_putaccbj_ptr = (MSK_putaccbj_t*)default_ret_0i32;
        MSK_getaccdomain_ptr = (MSK_getaccdomain_t*)default_ret_0i32;
        MSK_getaccn_ptr = (MSK_getaccn_t*)default_ret_0i32;
        MSK_getaccntot_ptr = (MSK_getaccntot_t*)default_ret_0i32;
        MSK_getaccafeidxlist_ptr = (MSK_getaccafeidxlist_t*)default_ret_0i32;
        MSK_getaccb_ptr = (MSK_getaccb_t*)default_ret_0i32;
        MSK_getaccs_ptr = (MSK_getaccs_t*)default_ret_0i32;
        MSK_getaccfnumnz_ptr = (MSK_getaccfnumnz_t*)default_ret_0i32;
        MSK_getaccftrip_ptr = (MSK_getaccftrip_t*)default_ret_0i32;
        MSK_getaccgvector_ptr = (MSK_getaccgvector_t*)default_ret_0i32;
        MSK_getaccbarfnumblocktriplets_ptr = (MSK_getaccbarfnumblocktriplets_t*)default_ret_0i32;
        MSK_getaccbarfblocktriplet_ptr = (MSK_getaccbarfblocktriplet_t*)default_ret_0i32;
        MSK_appenddjcs_ptr = (MSK_appenddjcs_t*)default_ret_0i32;
        MSK_putdjc_ptr = (MSK_putdjc_t*)default_ret_0i32;
        MSK_putdjcslice_ptr = (MSK_putdjcslice_t*)default_ret_0i32;
        MSK_getdjcdomainidxlist_ptr = (MSK_getdjcdomainidxlist_t*)default_ret_0i32;
        MSK_getdjcafeidxlist_ptr = (MSK_getdjcafeidxlist_t*)default_ret_0i32;
        MSK_getdjcb_ptr = (MSK_getdjcb_t*)default_ret_0i32;
        MSK_getdjctermsizelist_ptr = (MSK_getdjctermsizelist_t*)default_ret_0i32;
        MSK_getdjcs_ptr = (MSK_getdjcs_t*)default_ret_0i32;
        MSK_putconbound_ptr = (MSK_putconbound_t*)default_ret_0i32;
        MSK_putconboundlist_ptr = (MSK_putconboundlist_t*)default_ret_0i32;
        MSK_putconboundlistconst_ptr = (MSK_putconboundlistconst_t*)default_ret_0i32;
        MSK_putconboundslice_ptr = (MSK_putconboundslice_t*)default_ret_0i32;
        MSK_putconboundsliceconst_ptr = (MSK_putconboundsliceconst_t*)default_ret_0i32;
        MSK_putvarbound_ptr = (MSK_putvarbound_t*)default_ret_0i32;
        MSK_putvarboundlist_ptr = (MSK_putvarboundlist_t*)default_ret_0i32;
        MSK_putvarboundlistconst_ptr = (MSK_putvarboundlistconst_t*)default_ret_0i32;
        MSK_putvarboundslice_ptr = (MSK_putvarboundslice_t*)default_ret_0i32;
        MSK_putvarboundsliceconst_ptr = (MSK_putvarboundsliceconst_t*)default_ret_0i32;
        MSK_putcallbackfunc_ptr = (MSK_putcallbackfunc_t*)default_ret_0i32;
        MSK_putcfix_ptr = (MSK_putcfix_t*)default_ret_0i32;
        MSK_putcj_ptr = (MSK_putcj_t*)default_ret_0i32;
        MSK_putobjsense_ptr = (MSK_putobjsense_t*)default_ret_0i32;
        MSK_getobjsense_ptr = (MSK_getobjsense_t*)default_ret_0i32;
        MSK_putclist_ptr = (MSK_putclist_t*)default_ret_0i32;
        MSK_putcslice_ptr = (MSK_putcslice_t*)default_ret_0i32;
        MSK_putbarcj_ptr = (MSK_putbarcj_t*)default_ret_0i32;
        MSK_putcone_ptr = (MSK_putcone_t*)default_ret_0i32;
        MSK_putmaxnumdomain_ptr = (MSK_putmaxnumdomain_t*)default_ret_0i32;
        MSK_getnumdomain_ptr = (MSK_getnumdomain_t*)default_ret_0i32;
        MSK_appendrplusdomain_ptr = (MSK_appendrplusdomain_t*)default_ret_0i32;
        MSK_appendrminusdomain_ptr = (MSK_appendrminusdomain_t*)default_ret_0i32;
        MSK_appendrdomain_ptr = (MSK_appendrdomain_t*)default_ret_0i32;
        MSK_appendrzerodomain_ptr = (MSK_appendrzerodomain_t*)default_ret_0i32;
        MSK_appendquadraticconedomain_ptr = (MSK_appendquadraticconedomain_t*)default_ret_0i32;
        MSK_appendrquadraticconedomain_ptr = (MSK_appendrquadraticconedomain_t*)default_ret_0i32;
        MSK_appendprimalexpconedomain_ptr = (MSK_appendprimalexpconedomain_t*)default_ret_0i32;
        MSK_appenddualexpconedomain_ptr = (MSK_appenddualexpconedomain_t*)default_ret_0i32;
        MSK_appendprimalgeomeanconedomain_ptr = (MSK_appendprimalgeomeanconedomain_t*)default_ret_0i32;
        MSK_appenddualgeomeanconedomain_ptr = (MSK_appenddualgeomeanconedomain_t*)default_ret_0i32;
        MSK_appendprimalpowerconedomain_ptr = (MSK_appendprimalpowerconedomain_t*)default_ret_0i32;
        MSK_appendprimalpowerconedomainseq_ptr = (MSK_appendprimalpowerconedomainseq_t*)default_ret_0i32;
        MSK_appenddualpowerconedomain_ptr = (MSK_appenddualpowerconedomain_t*)default_ret_0i32;
        MSK_appenddualpowerconedomainseq_ptr = (MSK_appenddualpowerconedomainseq_t*)default_ret_0i32;
        MSK_appendsvecpsdconedomain_ptr = (MSK_appendsvecpsdconedomain_t*)default_ret_0i32;
        MSK_getdomaintype_ptr = (MSK_getdomaintype_t*)default_ret_0i32;
        MSK_getdomainn_ptr = (MSK_getdomainn_t*)default_ret_0i32;
        MSK_getpowerdomaininfo_ptr = (MSK_getpowerdomaininfo_t*)default_ret_0i32;
        MSK_getpowerdomainalpha_ptr = (MSK_getpowerdomainalpha_t*)default_ret_0i32;
        MSK_appendsparsesymmat_ptr = (MSK_appendsparsesymmat_t*)default_ret_0i32;
        MSK_appendsparsesymmatlist_ptr = (MSK_appendsparsesymmatlist_t*)default_ret_0i32;
        MSK_getsymmatinfo_ptr = (MSK_getsymmatinfo_t*)default_ret_0i32;
        MSK_getnumsymmat_ptr = (MSK_getnumsymmat_t*)default_ret_0i32;
        MSK_getsparsesymmat_ptr = (MSK_getsparsesymmat_t*)default_ret_0i32;
        MSK_putdouparam_ptr = (MSK_putdouparam_t*)default_ret_0i32;
        MSK_resetdouparam_ptr = (MSK_resetdouparam_t*)default_ret_0i32;
        MSK_putintparam_ptr = (MSK_putintparam_t*)default_ret_0i32;
        MSK_putlintparam_ptr = (MSK_putlintparam_t*)default_ret_0i32;
        MSK_resetintparam_ptr = (MSK_resetintparam_t*)default_ret_0i32;
        MSK_putmaxnumcon_ptr = (MSK_putmaxnumcon_t*)default_ret_0i32;
        MSK_putmaxnumcon64_ptr = (MSK_putmaxnumcon64_t*)default_ret_0i32;
        MSK_putmaxnumcone_ptr = (MSK_putmaxnumcone_t*)default_ret_0i32;
        MSK_getmaxnumcone_ptr = (MSK_getmaxnumcone_t*)default_ret_0i32;
        MSK_putmaxnumvar_ptr = (MSK_putmaxnumvar_t*)default_ret_0i32;
        MSK_putmaxnumvar64_ptr = (MSK_putmaxnumvar64_t*)default_ret_0i32;
        MSK_putmaxnumbarvar_ptr = (MSK_putmaxnumbarvar_t*)default_ret_0i32;
        MSK_putmaxnumanz_ptr = (MSK_putmaxnumanz_t*)default_ret_0i32;
        MSK_putmaxnumqnz_ptr = (MSK_putmaxnumqnz_t*)default_ret_0i32;
        MSK_getmaxnumqnz_ptr = (MSK_getmaxnumqnz_t*)default_ret_0i32;
        MSK_getmaxnumqnz64_ptr = (MSK_getmaxnumqnz64_t*)default_ret_0i32;
        MSK_putnadouparam_ptr = (MSK_putnadouparam_t*)default_ret_0i32;
        MSK_putnaintparam_ptr = (MSK_putnaintparam_t*)default_ret_0i32;
        MSK_putnastrparam_ptr = (MSK_putnastrparam_t*)default_ret_0i32;
        MSK_putobjname_ptr = (MSK_putobjname_t*)default_ret_0i32;
        MSK_putparam_ptr = (MSK_putparam_t*)default_ret_0i32;
        MSK_putqcon_ptr = (MSK_putqcon_t*)default_ret_0i32;
        MSK_putqconk_ptr = (MSK_putqconk_t*)default_ret_0i32;
        MSK_putqobj_ptr = (MSK_putqobj_t*)default_ret_0i32;
        MSK_putqobjij_ptr = (MSK_putqobjij_t*)default_ret_0i32;
        MSK_putsolution_ptr = (MSK_putsolution_t*)default_ret_0i32;
        MSK_putsolutionnew_ptr = (MSK_putsolutionnew_t*)default_ret_0i32;
        MSK_putconsolutioni_ptr = (MSK_putconsolutioni_t*)default_ret_0i32;
        MSK_putvarsolutionj_ptr = (MSK_putvarsolutionj_t*)default_ret_0i32;
        MSK_putsolutionyi_ptr = (MSK_putsolutionyi_t*)default_ret_0i32;
        MSK_putstrparam_ptr = (MSK_putstrparam_t*)default_ret_0i32;
        MSK_resetstrparam_ptr = (MSK_resetstrparam_t*)default_ret_0i32;
        MSK_puttaskname_ptr = (MSK_puttaskname_t*)default_ret_0i32;
        MSK_putvartype_ptr = (MSK_putvartype_t*)default_ret_0i32;
        MSK_putvartypelist_ptr = (MSK_putvartypelist_t*)default_ret_0i32;
        MSK_readdata_ptr = (MSK_readdata_t*)default_ret_0i32;
        MSK_readdatahandle_ptr = (MSK_readdatahandle_t*)default_ret_0i32;
        MSK_writedatahandle_ptr = (MSK_writedatahandle_t*)default_ret_0i32;
        MSK_readdataformat_ptr = (MSK_readdataformat_t*)default_ret_0i32;
        MSK_readdataautoformat_ptr = (MSK_readdataautoformat_t*)default_ret_0i32;
        MSK_readparamfile_ptr = (MSK_readparamfile_t*)default_ret_0i32;
        MSK_readsolution_ptr = (MSK_readsolution_t*)default_ret_0i32;
        MSK_readjsonsol_ptr = (MSK_readjsonsol_t*)default_ret_0i32;
        MSK_readsummary_ptr = (MSK_readsummary_t*)default_ret_0i32;
        MSK_resizetask_ptr = (MSK_resizetask_t*)default_ret_0i32;
        MSK_checkmemtask_ptr = (MSK_checkmemtask_t*)default_ret_0i32;
        MSK_getmemusagetask_ptr = (MSK_getmemusagetask_t*)default_ret_0i32;
        MSK_resetparameters_ptr = (MSK_resetparameters_t*)default_ret_0i32;
        MSK_sktostr_ptr = (MSK_sktostr_t*)default_ret_0i32;
        MSK_solstatostr_ptr = (MSK_solstatostr_t*)default_ret_0i32;
        MSK_solutiondef_ptr = (MSK_solutiondef_t*)default_ret_0i32;
        MSK_deletesolution_ptr = (MSK_deletesolution_t*)default_ret_0i32;
        MSK_writestat_ptr = (MSK_writestat_t*)default_ret_0i32;
        MSK_writesolutionstat_ptr = (MSK_writesolutionstat_t*)default_ret_0i32;
        MSK_onesolutionsummary_ptr = (MSK_onesolutionsummary_t*)default_ret_0i32;
        MSK_solutionsummary_ptr = (MSK_solutionsummary_t*)default_ret_0i32;
        MSK_updatesolutioninfo_ptr = (MSK_updatesolutioninfo_t*)default_ret_0i32;
        MSK_optimizersummary_ptr = (MSK_optimizersummary_t*)default_ret_0i32;
        MSK_strtoconetype_ptr = (MSK_strtoconetype_t*)default_ret_0i32;
        MSK_strtosk_ptr = (MSK_strtosk_t*)default_ret_0i32;
        MSK_whichparam_ptr = (MSK_whichparam_t*)default_ret_0i32;
        MSK_writedata_ptr = (MSK_writedata_t*)default_ret_0i32;
        MSK_writetask_ptr = (MSK_writetask_t*)default_ret_0i32;
        MSK_writebsolution_ptr = (MSK_writebsolution_t*)default_ret_0i32;
        MSK_writebsolutionhandle_ptr = (MSK_writebsolutionhandle_t*)default_ret_0i32;
        MSK_readbupdate_ptr = (MSK_readbupdate_t*)default_ret_0i32;
        MSK_writeb_ptr = (MSK_writeb_t*)default_ret_0i32;
        MSK_readbupdatehandle_ptr = (MSK_readbupdatehandle_t*)default_ret_0i32;
        MSK_readbsolution_ptr = (MSK_readbsolution_t*)default_ret_0i32;
        MSK_writesolutionfile_ptr = (MSK_writesolutionfile_t*)default_ret_0i32;
        MSK_readsolutionfile_ptr = (MSK_readsolutionfile_t*)default_ret_0i32;
        MSK_readtask_ptr = (MSK_readtask_t*)default_ret_0i32;
        MSK_readopfstring_ptr = (MSK_readopfstring_t*)default_ret_0i32;
        MSK_readlpstring_ptr = (MSK_readlpstring_t*)default_ret_0i32;
        MSK_readjsonstring_ptr = (MSK_readjsonstring_t*)default_ret_0i32;
        MSK_readptfstring_ptr = (MSK_readptfstring_t*)default_ret_0i32;
        MSK_writeparamfile_ptr = (MSK_writeparamfile_t*)default_ret_0i32;
        MSK_getinfeasiblesubproblem_ptr = (MSK_getinfeasiblesubproblem_t*)default_ret_0i32;
        MSK_getdualproblem_ptr = (MSK_getdualproblem_t*)default_ret_0i32;
        MSK_getfixedproblem_ptr = (MSK_getfixedproblem_t*)default_ret_0i32;
        MSK_tofixedproblem_ptr = (MSK_tofixedproblem_t*)default_ret_0i32;
        MSK_writesolution_ptr = (MSK_writesolution_t*)default_ret_0i32;
        MSK_writejsonsol_ptr = (MSK_writejsonsol_t*)default_ret_0i32;
        MSK_primalsensitivity_ptr = (MSK_primalsensitivity_t*)default_ret_0i32;
        MSK_sensitivityreport_ptr = (MSK_sensitivityreport_t*)default_ret_0i32;
        MSK_dualsensitivity_ptr = (MSK_dualsensitivity_t*)default_ret_0i32;
        MSK_getlasterror_ptr = (MSK_getlasterror_t*)default_ret_0i32;
        MSK_getlasterror64_ptr = (MSK_getlasterror64_t*)default_ret_0i32;
        MSK_writetasksolverresult_file_ptr = (MSK_writetasksolverresult_file_t*)default_ret_0i32;
        MSK_optimizermt_ptr = (MSK_optimizermt_t*)default_ret_0i32;
        MSK_optimizecb_ptr = (MSK_optimizecb_t*)default_ret_0i32;
        MSK_asyncoptimizecb_ptr = (MSK_asyncoptimizecb_t*)default_ret_0i32;
        MSK_asyncgetlogcb_ptr = (MSK_asyncgetlogcb_t*)default_ret_0i32;
        MSK_asyncstopcb_ptr = (MSK_asyncstopcb_t*)default_ret_0i32;
        MSK_asyncgetresultcb_ptr = (MSK_asyncgetresultcb_t*)default_ret_0i32;
        MSK_asyncpollcb_ptr = (MSK_asyncpollcb_t*)default_ret_0i32;
        MSK_asyncoptimize_ptr = (MSK_asyncoptimize_t*)default_ret_0i32;
        MSK_asyncgetlog_ptr = (MSK_asyncgetlog_t*)default_ret_0i32;
        MSK_asyncstop_ptr = (MSK_asyncstop_t*)default_ret_0i32;
        MSK_asyncpoll_ptr = (MSK_asyncpoll_t*)default_ret_0i32;
        MSK_asyncgetresult_ptr = (MSK_asyncgetresult_t*)default_ret_0i32;
        MSK_putoptserverhost_ptr = (MSK_putoptserverhost_t*)default_ret_0i32;
        MSK_optimizebatch_ptr = (MSK_optimizebatch_t*)default_ret_0i32;
        MSK_callbackcodetostr_ptr = (MSK_callbackcodetostr_t*)default_ret_0i32;
        MSK_isinfinity_ptr = (MSK_isinfinity_t*)default_ret_false;
        MSK_enablegarcolenv_ptr = (MSK_enablegarcolenv_t*)default_ret_0i32;
        MSK_makeenvdebug_ptr = (MSK_makeenvdebug_t*)default_ret_0i32;
        MSK_unittestmain_ptr = (MSK_unittestmain_t*)default_ret_0i32;
        MSK_runsimplexfromfile_ptr = (MSK_runsimplexfromfile_t*)default_ret_0i32;
        MSK_globalenvinitialize_ptr = (MSK_globalenvinitialize_t*)default_ret_0i32;
        MSK_globalenvfinalize_ptr = (MSK_globalenvfinalize_t*)default_ret_0i32;
        MSK_checkoutlicense_ptr = (MSK_checkoutlicense_t*)default_ret_0i32;
        MSK_checkinlicense_ptr = (MSK_checkinlicense_t*)default_ret_0i32;
        MSK_checkinall_ptr = (MSK_checkinall_t*)default_ret_0i32;
        MSK_expirylicenses_ptr = (MSK_expirylicenses_t*)default_ret_0i32;
        MSK_resetexpirylicenses_ptr = (MSK_resetexpirylicenses_t*)default_ret_0i32;
        MSK_getbuildinfo_ptr = (MSK_getbuildinfo_t*)default_ret_0i32;
        MSK_getresponseclass_ptr = (MSK_getresponseclass_t*)default_ret_0i32;
        MSK_callocenv_ptr = (MSK_callocenv_t*)default_ret_ptr;
        MSK_callocdbgenv_ptr = (MSK_callocdbgenv_t*)default_ret_ptr;
        MSK_deleteenv_ptr = (MSK_deleteenv_t*)default_ret_0i32;
        MSK_echointro_ptr = (MSK_echointro_t*)default_ret_0i32;
        MSK_freeenv_ptr = (MSK_freeenv_t*)default_ret_void;
        MSK_freedbgenv_ptr = (MSK_freedbgenv_t*)default_ret_void;
        MSK_getcodedesc_ptr = (MSK_getcodedesc_t*)default_ret_0i32;
        MSK_getsymbcondim_ptr = (MSK_getsymbcondim_t*)default_ret_0i32;
        MSK_rescodetostr_ptr = (MSK_rescodetostr_t*)default_ret_0i32;
        MSK_iinfitemtostr_ptr = (MSK_iinfitemtostr_t*)default_ret_0i32;
        MSK_dinfitemtostr_ptr = (MSK_dinfitemtostr_t*)default_ret_0i32;
        MSK_liinfitemtostr_ptr = (MSK_liinfitemtostr_t*)default_ret_0i32;
        MSK_getversion_ptr = (MSK_getversion_t*)default_ret_0i32;
        MSK_checkversion_ptr = (MSK_checkversion_t*)default_ret_0i32;
        MSK_iparvaltosymnam_ptr = (MSK_iparvaltosymnam_t*)default_ret_0i32;
        MSK_linkfiletoenvstream_ptr = (MSK_linkfiletoenvstream_t*)default_ret_0i32;
        MSK_linkfunctoenvstream_ptr = (MSK_linkfunctoenvstream_t*)default_ret_0i32;
        MSK_unlinkfuncfromenvstream_ptr = (MSK_unlinkfuncfromenvstream_t*)default_ret_0i32;
        MSK_makeenv_ptr = (MSK_makeenv_t*)default_ret_0i32;
        MSK_putlicensedebug_ptr = (MSK_putlicensedebug_t*)default_ret_0i32;
        MSK_putlicensecode_ptr = (MSK_putlicensecode_t*)default_ret_0i32;
        MSK_putlicensewait_ptr = (MSK_putlicensewait_t*)default_ret_0i32;
        MSK_putlicensepath_ptr = (MSK_putlicensepath_t*)default_ret_0i32;
        MSK_maketask_ptr = (MSK_maketask_t*)default_ret_0i32;
        MSK_makeemptytask_ptr = (MSK_makeemptytask_t*)default_ret_0i32;
        MSK_putexitfunc_ptr = (MSK_putexitfunc_t*)default_ret_0i32;
        MSK_utf8towchar_ptr = (MSK_utf8towchar_t*)default_ret_0i32;
        MSK_wchartoutf8_ptr = (MSK_wchartoutf8_t*)default_ret_0i32;
        MSK_checkmemenv_ptr = (MSK_checkmemenv_t*)default_ret_0i32;
        MSK_symnamtovalue_ptr = (MSK_symnamtovalue_t*)default_ret_false;
        MSK_axpy_ptr = (MSK_axpy_t*)default_ret_0i32;
        MSK_dot_ptr = (MSK_dot_t*)default_ret_0i32;
        MSK_gemv_ptr = (MSK_gemv_t*)default_ret_0i32;
        MSK_gemm_ptr = (MSK_gemm_t*)default_ret_0i32;
        MSK_syrk_ptr = (MSK_syrk_t*)default_ret_0i32;
        MSK_computesparsecholesky_ptr = (MSK_computesparsecholesky_t*)default_ret_0i32;
        MSK_sparsetriangularsolvedense_ptr = (MSK_sparsetriangularsolvedense_t*)default_ret_0i32;
        MSK_potrf_ptr = (MSK_potrf_t*)default_ret_0i32;
        MSK_syeig_ptr = (MSK_syeig_t*)default_ret_0i32;
        MSK_syevd_ptr = (MSK_syevd_t*)default_ret_0i32;
        MSK_licensecleanup_ptr = (MSK_licensecleanup_t*)default_ret_0i32;
        MSK_shutdownglobalthreadpool_ptr = (MSK_shutdownglobalthreadpool_t*)default_ret_0i32;

    }
    if (buf) free(buf);
    return 1;
EXIT_OK:
    if (buf) free(buf);
    return 0;
}
