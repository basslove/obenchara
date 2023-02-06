oapi_up:
	cd ./api/oapi && \
	docker-compose down && \
	docker-compose build --no-cache && \
	docker-compose up -d --build && \
	cd ../../ && \
	echo 'oapi start'

oapi_down:
	cd ./api/oapi && \
	docker-compose down && \
	cd ../../ && \
    echo 'oapi stop'
