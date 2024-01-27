-- Add down migration script here
alter table if exists Thread
    drop constraint fk_root;
drop table if exists Comment;
drop table if exists Thread;
drop table if exists Member;

