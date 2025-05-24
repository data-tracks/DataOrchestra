FROM rust:1.74

# Export cargo reference to bash profile for tmux session
RUN echo 'export PATH="/usr/local/cargo/bin:$PATH"' >> ~/.bashrc

CMD ["bash"]

