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
                sh 'cargo build --all-targets --target aarch64-unknown-linux-gnu'
            }
        }
    }
}
