FROM rust:1.86

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
