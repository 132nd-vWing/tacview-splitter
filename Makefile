VERSION := $(shell rg -o '(?<=^version = ")([0-9.]*)' Cargo.toml)

RELEASE_LINUX=tacview-splitter-linux-x86_64.tar.gz
RELEASE_WINDOWS=tacview-splitter-win10-x86_64.zip

BIN_LINUX=target/x86_64-unknown-linux-gnu/release/tacview-splitter
BIN_WINDOWS=target/x86_64-pc-windows-gnu/release/tacview-splitter.exe

default: release

build:
	cargo build --release --target x86_64-unknown-linux-gnu
	cargo build --release --target x86_64-pc-windows-gnu

strip: build
	strip ${BIN_LINUX} ${BIN_WINDOWS}

release: build strip
	rm -rf release;
	mkdir -p release/{windows,linux}/tacview-splitter-${VERSION};
	cp ${BIN_LINUX} release/linux/tacview-splitter-${VERSION}
	cp ${BIN_WINDOWS} release/windows/tacview-splitter-${VERSION}
	cd release/linux; tar -zcvf ${RELEASE_LINUX} tacview-splitter-${VERSION}; cp ${RELEASE_LINUX} ..
	cd release/windows; 7z a ${RELEASE_WINDOWS} tacview-splitter-${VERSION}; cp ${RELEASE_WINDOWS} ..
	cd release; sha256sum ${RELEASE_LINUX} ${RELEASE_WINDOWS} > sha256sums.txt
