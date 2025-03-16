
build: 
	cargo build --release
	strip target/release/factor

install: target/release/factor
	cp target/release/factor /usr/bin/rfactor
	chmod +x /usr/bin/rfactor
	echo "alias factor='/usr/bin/rfactor'" >> ~/.bash_aliases
