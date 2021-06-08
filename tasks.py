from invoke import task


@task
def build(c):
    c.run("pyinstaller --onefile tacview-splitter.py")


@task
def clean(c):
    c.run("rm -rf build __pycache__ tacview-splitter.spec")


@task(pre=[clean])
def distclean(c):
    c.run("rm -rf dist release")


@task(pre=[build])
def release(c, version):
    c.run(f"""
    mkdir -p release/linux/tacview-splitter-{version}
    mkdir -p release/win/tacview-splitter-{version}
    cd release/linux/tacview-splitter-{version}
    cp ../../../README.md .
    cp ../../../LICENSE .
    cp ../../../dist/tacview-splitter .
    cd ..
    tar -cvzf tacview-splitter-linux_x86-64.tar.gz tacview-splitter-{version}
    mv tacview-splitter-linux_x86-64.tar.gz ..

    cd ../win/tacview-splitter-{version}
    cp ../../../README.md .
    cp ../../../LICENSE LICENSE.txt
    echo Copy the windows release file into the following directory:
    pwd
    echo -n then press enter ...
    read
    cd ..
    7z a tacview-splitter-win_x86-64.zip tacview-splitter-{version}
    mv tacview-splitter-win_x86-64.zip ..

    cd ..
    rm -f sha256sums.txt
    sha256sum tacview-splitter-win_x86-64.zip tacview-splitter-linux_x86-64.tar.gz >> sha256sums.txt
    sha256sum --check sha256sums.txt
    """)
