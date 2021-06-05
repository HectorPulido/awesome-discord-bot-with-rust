# awesome-discord-bot-with-rust

WIP
docker build -t awesome-discord-bot .
docker run -d --name awesome-discord-bot -e "PORT=8765" -e "DEBUG=1" -p 8007:8765 awesome-discord-bot

docker rm awesome-discord-bot
docker kill awesome-discord-bot