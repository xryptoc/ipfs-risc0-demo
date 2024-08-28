docker run -d \
  --name=ipfs \
  -e PUID=1000 \
  -e PGID=1000 \
  -e TZ=Europe/London \
  -p 8089:80 \
  -p 4001:4001 \
  -p 5001:5001 \
  -p 8088:8080 \
  -v /path/to/data:/config \
  --restart unless-stopped \
  lscr.io/linuxserver/ipfs