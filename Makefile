build:
	cargo lambda build --target=x86_64-unknown-linux-musl  --release;
	mkdir -p target/lambda/brushgame/src;
	cp src/cert.pem target/lambda/brushgame/src;
	cd target/lambda/brushgame/ && zip build.zip ./bootstrap src/cert.pem

watch:
	cargo lambda watch
