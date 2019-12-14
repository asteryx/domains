
ngbuild:
	cd src/ng/ && npm run watch
%:
	./runner.sh $(MAKECMDGOALS)