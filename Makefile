.PHONY: all bundler web node playground publish publish-dry clean

all: bundler web node

bundler:
	wasm-pack build --target bundler --out-dir pkg/bundler

web:
	wasm-pack build --target web --out-dir pkg/web

node:
	wasm-pack build --target nodejs --out-dir pkg/node

playground: bundler
	bun install --cwd playground
	bun --cwd playground dev

publish: all
	npm publish ./pkg --access public

publish-dry: all
	npm publish ./pkg --access public --dry-run

clean:
	rm -rf pkg/bundler pkg/web pkg/node⏎      