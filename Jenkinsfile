pipeline {
    agent { 
        docker {
            image 'cross-rust-nightly'
            args '-u root:root'
            label 'rust-arm-toolchain'
        }
    }

    stages {
        stage('Build') {
            steps {
                sh 'cargo build --target aarch64-unknown-linux-gnu --target-dir ./target'
                sh 'cargo test --no-run --target aarch64-unknown-linux-gnu --target-dir ./target'
            }
            post {
                always {
                    sh 'ls ./target/CACHEDIR.TAG'
                    stash includes: './target/CACHEDIR.TAG', name: 'final_artifacts'
                }
            }
        }
    }
}
