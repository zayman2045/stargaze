# Stargaze

Stargaze is a 2D video game built in Rust using the Bevy game engine.

## Overview

Stargaze showcases the power and simplicity of the data-driven Bevy game engine, which uses an Entity Component System (ECS) to the map in-game elements and mechanisms to Rust structs and functions. Entities serve as identifiers for individual objects within the game world, and are assigned Components which represent that object's associated data.

## Getting Started

### To run this project locally:

Make sure you have Docker and Docker Compose installed. If not, you can download and install Docker from [here](https://docs.docker.com/get-docker/) and Docker Compose from [here](https://docs.docker.com/compose/install/).

Once Docker and Docker Compose are installed, you can build and run the application using the following command from the project root directory:

```zsh
docker-compose up --build
``` 

The application will be available at http://localhost:1334.