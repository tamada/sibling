PWD := ${CURDIR}
CONTAINER_NAME := "wwwsibling"
CONTAINER_REMOVE_FLAG := "--rm"
BASE_URL := "https://tamada.github.io/sibling"
JOJOMI_VERSION := "0.69.0"

build:
	docker run ${CONTAINER_REMOVE_FLAG} --name ${CONTAINER_NAME}_build -v "${PWD}":/src -v ${PWD}/public:/output -e HUGO_THEME=cayman-hugo-theme -e HUGO_BASEURL=${BASE_URL} jojomi/hugo:${JOJOMI_VERSION}
	rm -f public/favicon* public/apple-touch-icon.png

start:
	docker run ${CONTAINER_REMOVE_FLAG} -d --name ${CONTAINER_NAME} -p 1313:1313 -v "${PWD}":/src -v "$(PWD)"/public:/output -e HUGO_THEME=cayman-hugo-theme -e HUGO_WATCH="true" -e HUGO_BASEURL=${BASE_URL} jojomi/hugo:${JOJOMI_VERSION}

stop:
	docker stop ${CONTAINER_NAME}

build_with_hugo:
	hugo
	rm -f public/favicon* public/apple-touch-icon.png

restart: stop start
