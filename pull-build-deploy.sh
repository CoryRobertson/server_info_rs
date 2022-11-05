docker-compose down -d
git pull
docker build -t server_info_rs .
docker-compose up -d
