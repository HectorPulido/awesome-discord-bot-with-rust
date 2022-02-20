# Awesome discord bot made with rust and serenity

This is a discord bot that can connect with an brain API, it response as a chatbot and can search resources, memes, jobs offers, etc.


## Features
* Conection with the API
* Send the resources to the API
* Show responses
* Only emojis channels

## How it works
This is a heroku based project

### Setup Django

1. Configure your heroku project, save the name

2. Configure your .env file, use the .env.example file as a template

3. Build the docker development and run it to make sure everything is ok
```
docker build -t worker:latest .
docker run -d --name <herokuname> worker:latest
```

4. You can deactivate like this
```
docker stop <herokuname>
docker rm <herokuname>
```

5. You can upload your project with this commands
```
docker run -d --name <herokuname> worker:latest
heroku container:login
heroku container:push worker -a <herokuname>
heroku container:release worker -a <herokuname>
```


6. You are ready


## More interesting projects
I have a lot of fun projects, check this:

### Machine learning
- https://github.com/HectorPulido/Evolutionary-Neural-Networks-on-unity-for-bots
- https://github.com/HectorPulido/Imitation-learning-in-unity
- https://github.com/HectorPulido/Chatbot-seq2seq-C-

### Games
- https://github.com/HectorPulido/Unity-MMO-Framework
- https://github.com/HectorPulido/Contra-Like-game-made-with-unity
- https://github.com/HectorPulido/Pacman-Online-made-with-unity

### Random
- https://github.com/HectorPulido/Arithmetic-Parser-made-easy
- https://github.com/HectorPulido/Simple-php-blog
- https://github.com/HectorPulido/Decentralized-Twitter-with-blockchain-as-base


## Licence
This proyect uses Django, tweepy, dropbox libraries, also was made to work with heroku but everything else was totally handcrafted by me, so the licence is MIT, use it as you want.

<div align="center">
<h3 align="center">Let's connect 😋</h3>
</div>
<p align="center">
<a href="https://www.linkedin.com/in/hector-pulido-17547369/" target="blank">
<img align="center" width="30px" alt="Hector's LinkedIn" src="https://www.vectorlogo.zone/logos/linkedin/linkedin-icon.svg"/></a> &nbsp; &nbsp;
<a href="https://twitter.com/Hector_Pulido_" target="blank">
<img align="center" width="30px" alt="Hector's Twitter" src="https://www.vectorlogo.zone/logos/twitter/twitter-official.svg"/></a> &nbsp; &nbsp;
<a href="https://www.twitch.tv/hector_pulido_" target="blank">
<img align="center" width="30px" alt="Hector's Twitch" src="https://www.vectorlogo.zone/logos/twitch/twitch-icon.svg"/></a> &nbsp; &nbsp;
<a href="https://www.youtube.com/channel/UCS_iMeH0P0nsIDPvBaJckOw" target="blank">
<img align="center" width="30px" alt="Hector's Youtube" src="https://www.vectorlogo.zone/logos/youtube/youtube-icon.svg"/></a> &nbsp; &nbsp;
