# Alias
alias install := install-deps
alias config:= configure
alias d := dev
alias c := clean
alias rs := restart 
alias rb := rebuild
alias lt := lint
alias lg := logs
alias s := stop

set dotenv-required := false
set dotenv-load := true
set dotenv-path := ".env"
set export := true

# constants 
DOCKER_CMD := "docker compose -f docker-compose.yaml"


# Default Shows the default commands 
@default:
    @just --list --list-heading $'Available commands\n'

# format the code 
@lint:
    cargo fmt
    cargo group-imports --fix
    cargo sort -w 

@dev:
    {{ DOCKER_CMD }} up -d
    @just logs

# see docker logs, this is called internally when you run just dev
@logs:
    {{ DOCKER_CMD }} logs -f --tail='30' app


# destroy the running docker instance and clean the cache 
@kill:
    {{ DOCKER_CMD }} down -v 

# stop the running docker instance without cleaning the cache, called internally when you restart the project
@stop:
    {{ DOCKER_CMD }} down 

# stop and start the project without removing cache and local data 
restart:
    @just stop
    @just dev

# delete the project, the cached data, target dir and restart 
@rebuild:
    @just kill
    @just clean
    {{ DOCKER_CMD }} up --build  -d
    @just logs


#execute all initial setup after cloning the project
@configure:
    @just install-deps
    cp .env.example .env 


#remove the target dir from local file system 
@clean: 
    cargo clean


#install the local dependencies 
@install-deps:
    go install github.com/air-verse/air@latest



