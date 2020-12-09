FROM rust:slim
WORKDIR /usr/src/
ENV USER=leo

# Start new rust
RUN cargo new russapp
WORKDIR /usr/src/russapp/

# Copy from local
COPY . . 

# Install russapp to the common path to be call as executable and set as default
# RUN cargo install --path .
# CMD ["russapp"]

# To run a hello bin
RUN cargo install --path . --bin hello
CMD [ "hello" ]