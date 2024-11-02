up:
    podman-compose up --build
down:
    podman-compose down
kill:
    podman-compose kill --all

up-d:
    sudo docker compose up --build -d
down-d:
    sudo docker compose down

ls-cont:
    sudo docker container ls
ls-net:
    sudo docker network ls
