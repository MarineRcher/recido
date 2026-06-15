api:
	clear && \
	docker compose -f compose.yml down && docker compose -f compose.yml up --build


clear:
	docker system prune -a --volumes -f


.PHONY: api

