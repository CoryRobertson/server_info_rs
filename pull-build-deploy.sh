docker-compose down
git pull
docker build -t server_info_rs .
docker-compose up -d
