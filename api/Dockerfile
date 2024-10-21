FROM debian:bullseye-slim

WORKDIR /api
COPY ./api .

RUN apt update && apt install -y ca-certificates

EXPOSE 8000
CMD ["api"]