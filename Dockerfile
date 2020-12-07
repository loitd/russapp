FROM rust:slim
WORKDIR /usr/src/russapp
COPY . . 
RUN cargo install --path .
CMD ["russapp"]