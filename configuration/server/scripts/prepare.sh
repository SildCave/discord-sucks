openssl req -newkey rsa:4096 -nodes -keyout configuration/mongodb/ssl/mongodbkey.key -x509 -days 365000 -out configuration/mongodb/ssl/mongodbkey.crt
cat configuration/mongodb/ssl/mongodbkey.key configuration/mongodb/ssl/mongodbkey.crt > configuration/mongodb/ssl/mongodb.pem
head /dev/urandom | tr -dc A-Za-z0-9 | head -c256 > configuration/mongodb/mongodb_client_pass.txt
head /dev/urandom | tr -dc A-Za-z0-9 | head -c256 > configuration/mongodb/mongodb_admin_pass.txt