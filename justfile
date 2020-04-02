# https://github.com/casey/just

default: watch

watch:
	cargo watch --ignore www/index.html --ignore www/index.css --clear --exec run

serve:
	netlify dev

open:
	open http://localhost:8888

render:
	cargo run

deploy: render
	grep Verlag index.yaml
	netlify deploy --prod
