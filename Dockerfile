FROM rust:slim
WORKDIR /usr/src/
ENV USER=leo
RUN cargo new russapp
WORKDIR /usr/src/russapp/
COPY . . 
RUN cargo install --path .
CMD ["russapp"]