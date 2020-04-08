# https://github.com/casey/just

default: watch

watch FILE="index.yaml":
	cargo watch --ignore www/index.html --ignore www/index.css --clear --exec "run {{FILE}}"

check:
	cargo watch --ignore www/index.html --ignore www/index.css --clear --exec check

serve:
	netlify dev

open:
	open http://localhost:8888

render FILE="index.yaml":
	cargo run {{FILE}}

deploy: render
	grep Verlag index.yaml
	netlify deploy --prod
