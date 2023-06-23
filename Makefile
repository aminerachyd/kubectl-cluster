install:
	cargo build --release && \
	sudo cp target/release/oc-cluster /usr/local/bin/oc-cluster && \
	sudo chmod a+x /usr/local/bin/oc-cluster
