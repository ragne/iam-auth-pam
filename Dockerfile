FROM rust:latest

RUN apt-get update && apt-get install libpam-dev && apt-get clean

