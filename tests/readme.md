#### build sftp image
docker build -t sftp-server .

#### running:
docker run -d -p 2222:22 -e SFTP_USER=user1 -e SFTP_PASSWORD=pwd123 sftp-server
