# Rogger - A Rust-Based Minimalist Blog Application
Rogger is a simple blog application written in the Rust programming language. It focuses on simplicity and minimalism, allowing users to easily create and manage their own blog without unnecessary features or complexity.
## Features
- Content management system - manage blog content, pages, api keys and login data in a graphical way, without the need to write a line of code
- Single binary file - Rogger packs its required files into one binary during compilation, which makes it easy to transport, also works more securely (no problem with accidentally deleting files)
- It's fast - written in Rust, using a cache to save the last 100 posts, so it doesn't have to repeatedly query the database
- Easy transport - just install Redis, copy the binary and the "rogger.db" file to move your blog to another computer/server
- WYSIWYG editor - Rogger posts are written in Markdown so you can insert headings, lists, bolds, italics, etc. But you don't need to know Markdown at all! The posts and page editor automatically shows a preview of the typed text
- It's safe - files are packed in one binary (no risk of accidentally deleting something), using Redis for authorization and sqlite for holding posts. Since rogger is compiled there is also no risk of evaluating malicious code, Markdown code, html is also safely evaluated

## Running rogger locally
1. Clone project repository
```
git clone https://github.com/hukasx0/rogger
cd rogger/
```
2. Install sqlite, redis and cargo if they don't exist, compile as release and run
```
sudo ./build_run.sh 
```
- Same as the previous point, except it fetches the latest rogger code from GitHub
```
sudo ./build_run.sh --latest
```

3. open your favorite browser and head to http://localhost:80 (to manage posts go to http://localhost:80/cms)

## Using Docker compose to run rogger
1. If you don't have docker installed, then install docker from https://www.docker.com/get-started/
2. Clone project repository
```
git clone https://github.com/hukasx0/rogger
cd rogger/
```
3. Build and run containers with docker compose:
```
docker-compose up
```
4. open your favorite browser and head to http://localhost:80
(to manage posts go to http://localhost:80/cms)

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
4. Run the following command to build a Docker image:
```
docker build -t my-rogger-image .
```
Note: Replace "my-rogger-image" with any name you like for your Docker image.
5. Create a Docker network named "rogger" using the command:
```
docker network create rogger
```
This will create a Docker network that containers can use to communicate with each other.
6. Run a Redis container named "rogger_redis" in detached mode (-d) with the network specified as "rogger" using the command:
```
docker run -d --name rogger_redis --network rogger redis
```
This will start a Redis container named "rogger_redis" and connect it to the "rogger" network.
7. Run your application container, based on your "my-rogger-image", with port mapping from host port 80 to container port 80 (-p 80:80), an environment variable REDIS_URL set to redis://rogger_redis:6379 (-e REDIS_URL=redis://rogger_redis:6379), and the network specified as "rogger" (--network rogger) using the command:
```
docker run -p 80:80 -e REDIS_URL=redis://rogger_redis:6379 -d --name rogger_app --network rogger my-rogger-image
```
This will start your application container and connect it to the "rogger" network, with the Redis URL set to the hostname of the Redis container, "rogger_redis", and the port 6379.
8. open your favorite browser and head to http://localhost:80
(to manage posts go to http://localhost:80/cms)

## About
Rogger is a simple and minimalist blog application written in Rust. If you are looking for a lightweight and straightforward way to manage your own blog, or create simple diary on local network, Rogger may be the perfect solution for you.

## License
Rogger is open-source software licensed under the MIT License. Feel free to use it, modify it, and share it with others.
