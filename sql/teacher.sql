create table teacher
(
    id          serial
        constraint course_pk
            primary key,
    name       varchar(140) not null,
    picture_url   varchar(140) not null,
    profile  varchar(140)
);


create unique index teacher_id_uindex
    on teacher (id);