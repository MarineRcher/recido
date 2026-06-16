docker:
	clear && \
	docker compose -f compose.yml down && docker compose -f compose.yml up --build

mobile:
	cd recido-mobile && flutter pub get && flutter run -d emulator-5554

clear:
	docker system prune -a --volumes -f


.PHONY: docker

