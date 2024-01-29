-- Creating database tables
create table if not exists Member (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    joined TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

create table if not exists Thread (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,    
    author_id INTEGER NOT NULL,
    constraint fk_author foreign key (author_id) references Member
);

create table if not exists Comment (
    id SERIAL PRIMARY KEY,
    message TEXT NOT NULL,
    date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    author_id INTEGER NOT NULL,
    thread_id INTEGER NOT NULL,
    constraint fk_author foreign key (author_id) references Member,
    constraint fk_thread foreign key (thread_id) references Thread
);

-- Inserting example records
insert into Member (username) values
    ('testUser'),
    ('testUser2');

insert into Thread (title, author_id) values 
    ('How do I Lorem Ipsum while using Dolor Sit Amet', 1),
    ('Why wont my Lorem connect to my Ipsum?', 2);

-- creating two new threads via root comments
insert into Comment (message, author_id, thread_id) values
    ( 'basic sample text', 1, 1),
    ( 'basic sample text', 2, 2);

