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
                //sh 'ls /artifacts/target/aarch64-unknown-linux-gnu/debug'
                stash includes: './target/**', name: 'final_artifacts'
            }
        }
    }
}
