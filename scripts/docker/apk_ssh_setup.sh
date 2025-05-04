#!/bin/sh
apk update
apk add --no-cache openssh python3
mkdir -p /var/run/sshd
echo "root:password" | chpasswd
echo "PermitRootLogin yes" >> /etc/ssh/sshd_config
