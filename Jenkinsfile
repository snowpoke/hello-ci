pipeline {
    agent none
    stages {
        stage('Build') {
            agent { 
                docker {
                    image 'cross-rust-nightly'
                    args '-u root:root -v /out:/artifacts/target'
                    label 'rust-arm-toolchain'
                }
            }
            steps {
                sh 'cargo build --target aarch64-unknown-linux-gnu'
                sh 'cargo test --no-run --target aarch64-unknown-linux-gnu'
            }
        }
        stage('Stash') {
            agent {
                label 'rust-arm-toolchain'
            }
            steps {
                stash includes: '/out/**', name: 'artifacts'
            }
        }
    }
}
