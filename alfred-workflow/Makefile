all: target/release/{{PROJECTNAME}}
.PHONY: all

target/release/{{PROJECTNAME}}:
	cargo build --release
.PHONY: target/release/{{PROJECTNAME}}

install: target/release/{{PROJECTNAME}}
	./install-workflow.sh target/release/{{PROJECTNAME}}
.PHONY: install

update-plist:
	./install-workflow.sh --update-plist
.PHONY: update-plist

clean:
	cargo clean
.PHONY: clean
