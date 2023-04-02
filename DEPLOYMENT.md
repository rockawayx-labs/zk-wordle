# Deployment
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
