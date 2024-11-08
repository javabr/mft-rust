# Use the ARM64 version of Ubuntu
FROM arm64v8/ubuntu:latest

# Install necessary packages
RUN apt-get update && \
  apt-get install -y openssh-server nano rsyslog && \
  apt-get clean

# Create the SSH directory and configure the SSH server
RUN mkdir /var/run/sshd

# Expose the SSH port
EXPOSE 22

# Add a script to run SFTP and allow runtime configuration
COPY files/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

# Set the default command to run the entrypoint script
CMD ["/usr/local/bin/entrypoint.sh"]
