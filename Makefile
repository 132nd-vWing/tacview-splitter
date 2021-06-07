dist/tacview-splitter: tacview-splitter.py
	pyinstaller --onefile tacview-splitter.py

clean:
	rm -rf build __pycache__ tacview-splitter.spec

distclean: clean
	rm -rf dist

