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
                sh 'cargo build --target aarch64-unknown-linux-gnu'
                sh 'cargo test --no-run --target aarch64-unknown-linux-gnu'
                stash includes: '/artifacts/target/aarch64-unknown-linux-gnu', name: 'final_artifacts'
            }
        }
    }
}
