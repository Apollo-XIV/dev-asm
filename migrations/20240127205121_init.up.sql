-- Creating database tables
create table if not exists Member (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    joined TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

create table if not exists Thread (
    id SERIAL PRIMARY KEY,
    root_id INT
);

create table if not exists Comment (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255),
    message TEXT NOT NULL,
    date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    author_id INTEGER NOT NULL,
    thread_id INTEGER NOT NULL,
    constraint fk_author foreign key (author_id) references Member,
    constraint fk_thread foreign key (thread_id) references Thread
);

alter table Thread
    add constraint fk_root foreign key (root_id) references Comment;

-- Inserting example records
insert into Member (username) values
    ('testUser'),
    ('testUser2');

insert into Thread DEFAULT VALUES;

insert into Thread DEFAULT VALUES;

    -- creating two new threads via root comments
insert into Comment (title, message, author_id, thread_id) values
    ('How do I Lorem Ipsum while using Dolor Sit Amet', 'basic sample text', 1, 1),
    ('Why wont my Lorem connect to my Ipsum?', 'basic sample text', 2, 2);

-- setting comments as root of their respective threads
update thread set root_id=1 where id=1;
update thread set root_id=2 where id=2;

     -- creating a comment on each thread
insert into Comment (message, author_id, thread_id) values
    ('Hello testUser', 2, 1),
    ('Hello testUser2', 1, 2);

