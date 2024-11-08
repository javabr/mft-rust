#!/bin/bash

# Set up a default user if none is provided
USER_NAME=${SFTP_USER:-"sftpuser"}
USER_PASSWORD=${SFTP_PASSWORD:-"sftppassword"}

# Create a user with the specified password
useradd -m -d /home/${USER_NAME} -s /bin/bash ${USER_NAME}
echo "${USER_NAME}:${USER_PASSWORD}" | chpasswd

# Create the SSH directory for the user
mkdir -p /home/${USER_NAME}/.ssh
chown ${USER_NAME}:${USER_NAME} /home/${USER_NAME}/.ssh
chmod 700 /home/${USER_NAME}/.ssh

# Configure SSH for SFTP

echo "SyslogFacility AUTH" >> /etc/ssh/sshd_config
echo "LogLevel INFO" >> /etc/ssh/sshd_config

echo "Match User ${USER_NAME}" >> /etc/ssh/sshd_config
echo "    ChrootDirectory /home/${USER_NAME}" >> /etc/ssh/sshd_config
echo "    ForceCommand internal-sftp" >> /etc/ssh/sshd_config
echo "    AllowTcpForwarding no" >> /etc/ssh/sshd_config
echo "    PermitTunnel no" >> /etc/ssh/sshd_config
echo "    X11Forwarding no" >> /etc/ssh/sshd_config



# Start the SSH daemon
/usr/sbin/sshd -D
service rsyslog start