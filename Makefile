IMAGE := rust-get-ip
VERSION := 0.0.1

build:
	docker build -t ${IMAGE} .

run:
	docker run ${IMAGE}

auth:
	grep -v '^#' .env.local | grep -e "CR_PAT" | sed -e 's/.*=//' | docker login ghcr.io -u USERNAME --password-stdin

tag:
	docker tag ${IMAGE} ghcr.io/apinanyogaratnam/${IMAGE}:${VERSION}
	git tag -m "v${VERSION}" v${VERSION}

push:
	docker push ghcr.io/apinanyogaratnam/${IMAGE}:${VERSION}
	git push --tags

all:
	make auth && make tag && make push
