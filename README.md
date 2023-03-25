# Rogger
## Features
## Using Docker to run rogger
1. If you don't have docker installed, then install docker from https://www.docker.com/get-started/
2. Clone project repository
```
git clone https://github.com/hukasx0/rogger
cd rogger/
```
3. Run the following command to build a Docker image:
```
docker build -t my-rogger-image .
```
Note: Replace "my-rogger-image" with any name you like for your Docker image.
4. Once the Docker image has been built, you can run rogger inside a Docker container by running the following command:
```
docker run --rm my-rogger-image
```
Note: The "--rm" option automatically removes the container when it exits.
