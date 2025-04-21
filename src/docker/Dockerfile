ARG IMAGE="ubuntu:latest"

FROM ${IMAGE}
RUN apt-get update && \
    apt-get install -y openssh-server && \
    #apt-get install -y python3.11 && \
    mkdir /var/run/sshd && \
    echo 'root:password' | chpasswd && \
    echo "PermitRootLogin yes" >>/etc/ssh/sshd_config
EXPOSE 22
CMD ["/usr/sbin/sshd", "-D"]
