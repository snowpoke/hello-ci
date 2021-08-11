pipeline {
    agent { 
        docker {
            image 'cross-rust-nightly'
            args '-u root:root'
            label 'rust-arm-toolchain'
        }
    }
    stages {
        stage('Build unit tests as executable') {
            steps {
                sh 'cargo test --no-run --target aarch64-unknown-linux-gnu'

                // stash always uses relative paths, so we have to cd into the target folder first
                dir('/artifacts/target/aarch64-unknown-linux-gnu/debug'){
                    sh 'chmod --recursive 777 .' // ensure that controller has read permission
                    stash includes: '**', name: 'unit_tests'
                }
            }
        }

        stage('TODO: Run unit tests on arm64 machine') {
            agent {
                label 'arm64' // any node with arm64 label
            }
            steps {
                sh 'echo This still needs to be implemented'
            }
        }

        stage('Build'){
            sh 'cargo build --target aarch64-unknown-linux-gnu'
        }
    }
}
