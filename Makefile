
build: 
	cargo build --release
	strip target/release/factor

install: target/release/factor
	mv /usr/bin/factor /usr/bin/gnufactor 
	cp target/release/factor /usr/bin/factor
	chmod +x /usr/bin/factor
