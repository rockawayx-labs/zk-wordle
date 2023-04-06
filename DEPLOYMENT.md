# Deployment

## Container Registry

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
docker pull registry.digitalocean.com/rockawayx-labs/zk-wordle
```

Run the image:
```
docker run -d -p 80:8080 -p 8080:8081 -e PUBLIC_URL=http://http://zkwordle.rockawayx.com -e "ALCHEMY_MUMBAI_API_KEY=https://polygon-mumbai.g.alchemy.com/v2/VDEtXZglGFw5AoR48KaAj-ngFWYUehMY" -e "OWNER_PRIVATE_KEY=6aea615c3d873b52514bed23b33011ffea8d1e99a242fa3d459b24dc21c93f3d" -e "OWNER_ADDRESS=0x6c29233170269A10283c15ee2dFbB2d70Ba4E5B7" -e "CONTRACT_ADDRESS=0x307B04Fd818eD3620847cE88fAfa73b80e090E79" registry.digitalocean.com/rockawayx-labs/zk-wordle
```

If you get an error saying that the port is already allocated, stop the running container and try it again.
