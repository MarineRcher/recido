FROM debian:stable-slim

RUN apt-get update && apt-get install -y \
    curl git unzip xz-utils zip \
    && rm -rf /var/lib/apt/lists/*

ARG FLUTTER_VERSION=3.44.2
ARG FLUTTER_SDK=/usr/local/flutter

RUN git clone --depth 1 --branch $FLUTTER_VERSION \
    https://github.com/flutter/flutter.git $FLUTTER_SDK

ENV PATH="$FLUTTER_SDK/bin:$FLUTTER_SDK/bin/cache/dart-sdk/bin:${PATH}"

RUN flutter precache --web

WORKDIR /app

COPY pubspec.* .
RUN flutter pub get

COPY . .

EXPOSE 8080

CMD ["flutter", "run", "-d", "web-server", "--web-port", "8080", "--web-hostname", "0.0.0.0"]
