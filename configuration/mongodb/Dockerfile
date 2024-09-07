FROM mongo:5.0

COPY scripts /home/mongodb/scripts
COPY ssl /home/mongodb/ssl
COPY mongod.conf /home/mongodb
COPY mongodb_client_pass.txt /home/mongodb
COPY mongodb_admin_pass.txt /home/mongodb

WORKDIR /home/mongodb

RUN chmod +x /home/mongodb/scripts/*.sh

CMD ["/bin/bash", "/home/mongodb/scripts/run.sh"]