/********************************
How to run files from psql
\i migrations/0001.init/down.sql
\i migrations/0001.init/up.sql
********************************/
/********************************
Some helpful instructions
//create a new user called test
sudo -u postgres createuser --interactive
//connect as sudo psql user postgres
sudo -u postgres psql
//PSQL
\q //exit
\du //list all users
\l //list all databases
\c dbname //connect to database dbname
ALTER USER test WITH PASSWORD 'xxxxxx'; //update user password
create database test; //create database
\c test //connect to database
********************************/
create database test;
\c test
create schema test;
create table test.item(id serial, data text);
/* grating access */
grant usage on schema test to test;
grant select, insert, update, delete on all tables in schema test to test;
grant all privileges on all tables in schema test to test;
grant all privileges on all sequences in schema test to test;
/********************************
Connect as user test
psql -h localhost -U test
\dt test.* //show all tables of schema test
********************************/