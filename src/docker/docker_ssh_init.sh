#!/bin/bash
apt-get update
apt-get install -y openssh-server
apt-get install -y python3.11
mkdir /var/run/sshd
echo "root:password" | chpasswd
echo "PermitRootLogin yes" >>/etc/ssh/sshd_config
