# To run the demo:
#
#     $ CONTAINER=$(docker run -e "REALM_ID=sid_74a40187e2790" -e "USER_ID=sid_c233df00c07b9" -P -d tozny-pam-demo)
#
# The container will include a running SSH server, whose port is forwarded to
# some port in your host OS.  To log in:
#
#     $ ssh -p $(docker port $CONTAINER 22 | cut -d: -f2) gregory@localhost
#
# To stop the server:
#
#     $ docker stop $CONTAINER
#

FROM ubuntu:18.04

ENV DEBIAN_FRONTEND noninteractive

## Install dependencies

RUN apt-get update && \
    apt-get install -y openssh-server rsyslog sudo && \
    apt-get clean

## Install and configure the module


RUN mkdir -p /lib/security
ENV abc=12345
COPY target/release/libiam_auth_pam.so /lib/security/libiam_auth_pam.so
RUN sed -i "1s/^/auth sufficient libiam_auth_pam.so -p\n\n/" /etc/pam.d/sshd
RUN sed -i "1s/^/account sufficient libiam_auth_pam.so -p\n\n/" /etc/pam.d/sshd
RUN sed -i "1s/^/session sufficient libiam_auth_pam.so -p\n\n/" /etc/pam.d/sshd

## Create a regular user account

RUN adduser --quiet --gecos '' --disabled-password test
RUN adduser test sudo

## Run OpenSSH server

RUN mkdir -p /var/run/sshd && chmod 0755 /var/run/sshd

EXPOSE 22
CMD /usr/sbin/sshd -D