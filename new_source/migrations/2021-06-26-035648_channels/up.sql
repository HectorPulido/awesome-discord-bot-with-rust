-- Your SQL goes here
create table types (
    id SERIAL PRIMARY KEY,
    type_description varchar not null unique
);

create table channels (
    id SERIAL PRIMARY KEY,
    channel_id varchar not null unique,
    channel_type integer not null,
    foreign key (channel_type) references types(id)
);