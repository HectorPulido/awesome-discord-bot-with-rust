-- Your SQL goes here
create table resources (
    id SERIAL PRIMARY KEY,
    user_id varchar not null,
    channel_id varchar not null,
    url varchar not null unique,
    description text NOT NULL DEFAULT '',
    resources_type integer NOT NULL DEFAULT 0,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp,
    foreign key (resources_type) references types(id)
);




