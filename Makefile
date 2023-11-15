format:
	cargo fmt && cargo clippy

test:
	cargo test

commit-and-push:
	git add .
	git commit -m "Update parser version"
	git push origin

publish:
	cargo publish

release: format test commit-and-push publish
