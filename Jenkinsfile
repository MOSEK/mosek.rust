
node ("karise") {
    def brn = env.gitlabSourceBranch
    branchname = ( brn == null ?
            "origin/master" :
             ( !brn.startsWith("refs/") ? brn : "origin/$brn" ) )

    //stage "checkout" {
    checkout changelog: false,
             poll: true,
             scm: ([$class: 'GitSCM',
                    branches: [[name: branchname]],
                    doGenerateSubmoduleConfigurations: false,
                    extensions: [[$class: 'CloneOption',
                                  noTags: false,
                                  reference: ''],
                                 [$class: 'CheckoutOption'],
                                 //[$class: 'RelativeTargetDirectory',
                                 // relativeTargetDir: 'Mosek.rs'],
                                 [$class: 'SubmoduleOption',
                                  disableSubmodules: false,
                                  parentCredentials: false,
                                  recursiveSubmodules: true,
                                  reference: '',
                                  trackingSubmodules: false]],
                        gitTool: 'Default',
                        userRemoteConfigs: [[credentialsId: '65bca1cb-66bd-4983-9aaa-0aec83b1491b',
                                             url: 'git@git-lab.mosek.intranet:ulfw/mosek.rust.git']]])

    def mosekver = readFile "MOSEKVERSION"
    mosekver = mosekver.trim()
    copyArtifacts filter: 'bld/hudson/distro/mosektoolslinux64x86.tar.bz2',
                  fingerprintArtifacts: true,
                  projectName: "${mosekver}/Distro-pipeline",
                  selector: lastSuccessful()
        sh "rm -rf minidist && tar xf bld/hudson/distro/mosketoolslinux64x86.tar.bz2"
    //}

    gitlabCommitStatus (connection: gitLabConnection('gitlab-api'),
                        name: "Mosek.rs") {

      sh """
export PATH=/remote/public/linux/64-x86/rust/current/bin:$PATH
export LD_LIBRARY_PATH=../mosek/
export MOSEK_INST_BASE=../
cargo test
"""
    }
}
