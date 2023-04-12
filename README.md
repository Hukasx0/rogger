# Rogger - A Rust-Based Minimalist Blog Application
Rogger is a simple blog application written in the Rust programming language. It focuses on simplicity and minimalism, allowing users to easily create and manage their own blog without unnecessary features or complexity.
## Features
- Redis database for storing authorization data (login data, API keys)
- SQLite database for storing all posts
- Cache memory for fast loading of posts from the application's memory without constant querying of the database
- writing posts using MarkDown and getting formatted content as HTML
- WYSIWYG (What You See Is What You Get) editor for posts

## Using Docker to run rogger
1. If you don't have docker installed, then install docker from https://www.docker.com/get-started/
2. Clone project repository
```
git clone https://github.com/hukasx0/rogger
cd rogger/
```
3. Install Redis
```
docker pull redis
```
3. Run the following command to build a Docker image:
```
docker build -t my-rogger-image .
```
Note: Replace "my-rogger-image" with any name you like for your Docker image.
4. Create a Docker network named "rogger" using the command:
```
docker network create rogger
```
This will create a Docker network that containers can use to communicate with each other.
5. Run a Redis container named "rogger_redis" in detached mode (-d) with the network specified as "rogger" using the command:
```
docker run -d --name rogger_redis --network rogger redis
```
This will start a Redis container named "rogger_redis" and connect it to the "rogger" network.
6. Run your application container, based on your "my-rogger-image", with port mapping from host port 1337 to container port 1337 (-p 1337:1337), an environment variable REDIS_URL set to redis://rogger_redis:6379 (-e REDIS_URL=redis://rogger_redis:6379), and the network specified as "rogger" (--network rogger) using the command:
```
docker run -p 1337:1337 -e REDIS_URL=redis://rogger_redis:6379 -d --name rogger_app --network rogger my-rogger-image
```
This will start your application container and connect it to the "rogger" network, with the Redis URL set to the hostname of the Redis container, "rogger_redis", and the port 6379.
7. open your favorite browser and head to http://localhost:1337
(to manage posts go to http://localhost:1337/cms)

## About
Rogger is a simple and minimalist blog application written in Rust. If you are looking for a lightweight and straightforward way to manage your own blog, or create simple diary on local network, Rogger may be the perfect solution for you.

## License
Rogger is open-source software licensed under the MIT License. Feel free to use it, modify it, and share it with others.
