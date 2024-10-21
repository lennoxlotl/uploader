FROM debian:bullseye-slim

WORKDIR /opt/api
COPY ./api .

RUN apt update && apt install -y ca-certificates
RUN chmod +x /opt/api/api

EXPOSE 8000
CMD ["/opt/api/api"]