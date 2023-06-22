release:
	cargo build --release && \
	sudo cp target/release/kubectl-cluster /usr/local/bin/kubectl-cluster && \
	sudo chmod a+x /usr/local/bin/kubectl-cluster
