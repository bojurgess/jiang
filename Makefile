.PHONY: all bundler web node build_dev playground publish publish-dry clean

all: clean bundler web node
	bash scripts/cleanup.sh

bundler:
	wasm-pack build --target bundler --scope bojurgess --out-dir pkg/bundler

web:
	wasm-pack build --target web --scope bojurgess --out-dir pkg/web

node:
	wasm-pack build --target nodejs --scope bojurgess --out-dir pkg/node

build_dev: clean
	wasm-pack build --target bundler --dev --scope bojurgess --out-dir pkg/bundler

playground: build_dev
	bun install --cwd web
	bun --cwd web dev --host

publish: all
	npm publish ./pkg --access public

publish-dry: all
	npm publish ./pkg --access public --dry-run

clean:
	rm -rf pkg/bundler pkg/web pkg/node