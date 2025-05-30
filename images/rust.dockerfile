FROM rust:1.86

RUN apt-get update && \
    apt-get install -y --no-install-recommends apt-utils && \
    apt-get install -y openssh-server && \
    apt-get install -y python3.11 && \
    apt-get install -y curl && \
    apt-get install -y tmux && \
    apt-get install -y ca-certificates && \
    apt-get install -y vim 

RUN mkdir /var/run/sshd
RUN echo 'root:password' | chpasswd
RUN echo 'PermitRootLogin yes' >> /etc/ssh/ssh_config
 
# Export cargo reference to bash profile for tmux session
RUN echo 'export PATH="/usr/local/cargo/bin:$PATH"' >> ~/.bashrc
RUN echo 'export PATH="/usr/local/cargo/bin/rustup:$PATH"' >> ~/.bashrc

# Set rustup toolchain as default for bash profile and only if used with interactive shell (no scp)
RUN { \
    echo 'if [ -n "$PS1" ]; then'; \
    echo '  ln -s /usr/local/rustup/toolchains /root/.rustup/toolchains 2>/dev/null || true'; \
    echo '  rustup default 1.86.0-x86_64-unknown-linux-gnu'; \
    echo 'fi'; \
    } >> ~/.bashrc

CMD ["bash"]
