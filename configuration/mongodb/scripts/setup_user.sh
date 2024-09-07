#!/bin/bash

echo "************************************************************"
echo "   users..."
echo "************************************************************"
admin_pass=`cat mongodb_admin_pass.txt`
client_pass=`cat mongodb_client_pass.txt`
echo "************************************************************"
echo "   admin_pass: $admin_pass"
echo "   client_pass: $client_pass"
echo "************************************************************"
# create root user
nohup gosu mongodb mongo admin --eval "
    db.createUser({
        user: '$admin_user',
        pwd: '$admin_pass',
        roles: [
            { role: 'root', db: 'admin' },
            { role: 'read', db: 'local' }
        ]
    });
"
nohup gosu mongodb mongo data --eval "
    db.createUser({
        user: '$db_user',
        pwd: '$db_pass',
        roles: [
            { role: 'readWrite', db: 'data' },
            { role: 'read', db: 'local' }
        ]
    });
"
# create app user/database
nohup gosu mongodb mongo data --eval "db.createUser({ user: 'db_client', pwd: '&client_pass', roles: [{ role: 'readWrite', db: 'data' }, { role: 'read', db: 'local' }]});"

echo "************************************************************"
echo "Shutting down"
echo "************************************************************"
nohup gosu mongodb mongo admin --eval "db.shutdownServer();"