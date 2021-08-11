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
                sh 'cargo test --no-run --target aarch64-unknown-linux-gnu --target-dir ./target'
                sh 'chmod --recursive 777 ./target' // ensure that controller has access rights

                sh 'ls target'
                sh 'ls target/aarch64-unknown-linux-gnu/debug'
                stash includes: 'target/aarch64-unknown-linux-gnu/debug/**', name:'unit_tests'
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
            steps {
                sh 'cargo build --target aarch64-unknown-linux-gnu'
            }
        }
    }
}
