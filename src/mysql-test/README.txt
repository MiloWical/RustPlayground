Run the DB container from this directory with the following command:

docker run --rm --name testdb -e MYSQL_ROOT_PASSWORD=password -d -p 3306:3306 -v $(pwd)/data:/var/lib/mysql mariadb

Use this to stop it:

docker stop testdb

If you need to clear out the DB data, run this in this directory:

rm -rf $(pwd)/data/*

You may need to run the container and then stop it to force the 
DB buffers to flush. The files will come around the next time the
container is started.

If you want to run the pre-built DB container, run this:

docker run --rm --name testdb -e MYSQL_ROOT_PASSWORD=password -d -p 3306:3306 mariadb-test-db