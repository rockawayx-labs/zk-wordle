# Deployment

We deployed the app both to App Platform and Droplet but the latter seems to be much faster.

## App Platform

If you already built the image on Linux machine, you just need to tag it:
```
docker tag zk-wordle registry.digitalocean.com/rockawayx-labs/zk-wordle
```
Otherwise, if you are on another operating system, you need to build it for Linux:
```
docker build --platform linux/amd64 --tag registry.digitalocean.com/rockawayx-labs/zk-wordle .
```
Before pushing, you need to log in:
```
docker login registry.digitalocean.com
```
And finally, push the image to the container registry:
```
docker push registry.digitalocean.com/rockawayx-labs/zk-wordle
```
The app will get deployed to https://zk-wordle-r5bo5.ondigitalocean.app/

## Droplet

SSH into the Droplet in Digital Ocean:
```
ssh root@161.35.160.141
```

Log into the Container Registry:
```
docker login registry.digitalocean.com
```

Pull the latest version of the Docker image:
```
docker pull registry.digitalocean.com/rockawayx-labs/cosmos-sensor
```

Run the image:
```
docker run -d -p 8080:8080 -e PUBLIC_URL=http://161.35.160.141:8080 registry.digitalocean.com/rockawayx-labs/zk-wordle
```

If you get an error saying that the port is already allocated, stop the running container and try it again.
