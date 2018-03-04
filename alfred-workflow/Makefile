all: build
.PHONY: all

build:
	cargo build --release
.PHONY: build

install: build
	./install-workflow.sh target/release/{{PROJECTNAME}}
.PHONY: install

update-plist:
	./install-workflow.sh --update-plist
.PHONY: update-plist

clean:
	cargo clean
	rm -f {{WORKFLOWNAME}}.alfredworkflow
.PHONY: clean

release: clean build
	cp target/release/{{PROJECTNAME}} .
	zip -9 -r {{WORKFLOWNAME}}.alfredworkflow icon.png info.plist {{PROJECTNAME}}
	rm {{PROJECTNAME}}

