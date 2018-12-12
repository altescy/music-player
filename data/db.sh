export MYSQL_RANDOM_ROOT_PASSWORD=yes
export MYSQL_DATABASE=mysql
export MYSQL_USER=user
export MYSQL_PASSWORD=password

docker run --name "mysql-test" -d -p3306:3306 \
    -e MYSQL_RANDOM_ROOT_PASSWORD=$MYSQL_RANDOM_ROOT_PASSWORD \
    -e MYSQL_DATABASE=$MYSQL_DATABASE \
    -e MYSQL_USER=$MYSQL_USER \
    -e MYSQL_PASSWORD=$MYSQL_PASSWORD \
    -v "$(pwd | xargs dirname)/db":"/docker-entrypoint-initdb.d" \
    mysql:8.0
