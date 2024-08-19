create table course
(
    id         serial,
    teacher_id integer      not null,
    name       varchar(140) not null,
    time       timestamp default now(),
    constraint course_pk
        primary key (id)
);

create unique index course_id_uindex
    on course (id);

alter table course
    add description varchar(2000);

alter table course
    add format varchar(30);

alter table course
    add structure varchar(200);

alter table course
    add duration varchar(30);

alter table course
    add price integer;

alter table course
    add language varchar(30);

alter table course
    add level varchar(30);