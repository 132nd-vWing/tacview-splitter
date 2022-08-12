VERSION := $(shell rg -o '(?<=^version = ")([0-9.]*)' Cargo.toml)

RELEASE_LINUX=tacview-splitter-linux-x86_64.tar.gz
RELEASE_WINDOWS=tacview-splitter-win10-x86_64.zip

BIN_LINUX=target/x86_64-unknown-linux-gnu/release/tacview-splitter
BIN_WINDOWS=target/x86_64-pc-windows-gnu/release/tacview-splitter.exe

TEST_FILES=Tacview-20210606-222650-DCS-ATRM_2.7.0.443.txt.acmi Tacview-20210606-222650-DCS-ATRM_2.7.0.443.zip.acmi Tacview-20210608-085030-DCS-Georgia_At_War_v3.0.24_afternoon.txt.acmi Tacview-20210608-085030-DCS-Georgia_At_War_v3.0.24_afternoon.zip.acmi

default: release

.built: src/*.rs
	cargo build --release --target x86_64-unknown-linux-gnu
	cargo build --release --target x86_64-pc-windows-gnu
	touch .built

build: .built

.tested: .built
	for file in ${TEST_FILES}; do \
		mkdir -p test; \
		cp testfiles/$$file test; \
		if [[ $$? != 0 ]]; then exit 1; fi;\
		cd test; \
		../${BIN_LINUX}; \
		if [[ $$? != 0 ]]; then exit 1; fi;\
		cd ../ ; \
		rm -rf test ; \
	done
	touch .tested

test: .tested

.released: .tested
	rm -rf release;
	mkdir -p release/{windows,linux}/tacview-splitter-${VERSION};
	cp ${BIN_LINUX} release/linux/tacview-splitter-${VERSION}
	cp ${BIN_WINDOWS} release/windows/tacview-splitter-${VERSION}
	cd release/linux; tar -zcvf ${RELEASE_LINUX} tacview-splitter-${VERSION}; cp ${RELEASE_LINUX} ..
	cd release/windows; 7z a ${RELEASE_WINDOWS} tacview-splitter-${VERSION}; cp ${RELEASE_WINDOWS} ..
	cd release; rm -rf linux windows; sha256sum ${RELEASE_LINUX} ${RELEASE_WINDOWS} > sha256sums.txt
	touch .released

release: .released

clean:
	rm -rf release target test .released .tested .built
