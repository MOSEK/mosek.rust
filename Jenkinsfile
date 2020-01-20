
node ("karise") {
    def brn = env.gitlabSourceBranch
    branchname = ( brn == null ?
            "origin/master" :
             ( !brn.startsWith("refs/") ? brn : "origin/$brn" ) )

    stage "checkout" {
        checkout changelog: false,
                 poll: true,
                 scm: ([$class: 'GitSCM',
                        branches: [[name: branchname]],
                        doGenerateSubmoduleConfigurations: false,
                                     extensions: [[$class: 'CloneOption',
                                                   noTags: false,
                                                   reference: '',
                                                   timeout: gitclonetimeout],
                                                  [$class: 'CheckoutOption',
                                                   timeout: gitclonetimeout]],
                        gitTool: 'Default',
                        submoduleCfg: [],
                        userRemoteConfigs: [[credentialsId: '65bca1cb-66bd-4983-9aaa-0aec83b1491b',
                                             url: 'git@gitlab.mosek.intranet:ulfw/mosek.rust.git']]])

        copyArtifacts filter: 'bld/hudson/distro/minidist-linux64x86.tar.bz2',
                      fingerprintArtifacts: true,
                      projectName: 'dev/Distro-pipeline',
                      selector: lastSuccessful()
        sh "tar xf bld/hudson/distro/minidist-linux64x86.tar.bz2"


    }

    gitlabCommitStatus connection: gitLabConnection('gitlab-api'),
                       name:       "Mosek.rs" {
        sh """
export PATH=/remote/public/liux/64-x86/rust/current/bin:$PATH
export LD_LIBRARY_PATH=
cargo test
"""
    }
}
