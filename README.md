# Stargaze

Stargaze is a 2D video game built in Rust using the Bevy game engine. You play as a spaceship with the goal being to collect as many stars as possible before colliding with an asteroid.

## Overview

Stargaze showcases the power and simplicity of the data-driven Bevy game engine, which uses an Entity Component System (ECS) to map in-game elements and mechanisms to Rust structs and functions. `Entities` serve as identifiers for individual objects within the game world, including the player's ship and each individual star and asteroid. These entities are assigned `Components` which represent that object's associated data. In this game, the components used range from simple identifiers for entities to more complex ones that provide detailed information about styling, positioning, and movement direction. `Systems` in Stargaze are Rust functions that control the behavior of entities. They operate on components and can be grouped into sets to control execution scheduling, providing a flexible and efficient mechanism for game logic.

Stargaze also makes use of Bevy's Events, Resources, and States to further customize the behavior and parameters of its in-game systems. `Events` allow systems to communicate with other systems, using either an EventWriter to send messages once an action has taken place, or an EventReader to listen for messages on a specified schedule. The GameOver event, for example, is triggered when the user's sprite collides with an asteroid and is despawned, sending a message to capture and display the user's final score. `Resources` are globally available data structures that can be accessed and mutated by a variety of systems across the program, and are ideal for storing items such as spawn timers and scores. `States` represent the possible stages that the game can be in and are used to control which systems are running at any given time.



## Getting Started

### Game Instructions

* Use the WASD or arrow keys to move the player around.
* Press the spacebar to play and pause the game.
* Press Esc to quit the game.

### To run this project locally:

Make sure you have Docker and Docker Compose installed. If not, you can download and install Docker from [here](https://docs.docker.com/get-docker/) and Docker Compose from [here](https://docs.docker.com/compose/install/).

Once Docker and Docker Compose are installed, you can build and run the application using the following command from the project root directory:

```zsh
docker-compose up --build
``` 

The application will be available at http://localhost:1334.
