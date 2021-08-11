pipeline {
    agent none
    stages {
        stage('Build') {
            agent { 
                docker {
                    image 'cross-rust-nightly'
                    args '-u root:root'
                    label 'rust-arm-toolchain'
                }
            }
            steps {
                sh 'cargo build --target aarch64-unknown-linux-gnu'
                sh 'cargo test --no-run --target aarch64-unknown-linux-gnu'
                sh 'whoami'
                sh 'ls .'
                stash includes: '**', name: 'artifacts'
            }
        }
    }
}
