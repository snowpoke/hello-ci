pipeline {
    agent { 
        docker {
            image 'cross-rust-nightly'
            args '-u root:root'
            label 'rust-arm-toolchain'
        }
    }

    stages {
        stage('Analysis') {
            steps {
            // we read the package name from Cargo.toml to determine the artifact names
                // requires `toml-cli`
                sh 'cargo install toml-cli'
                sh 'export PACKAGE_NAME=$(toml get Cargo.toml package.name | sed -e \'s/^"//\' -e \'s/"$//\')'
            }
        }
        stage('Build') {
            steps {
                sh 'cargo build --target aarch64-unknown-linux-gnu'
                sh 'cargo test --no-run --target aarch64-unknown-linux-gnu'

                stash includes: '/out/*', name: 'final_artifacts'
            }
        }
    }
}
