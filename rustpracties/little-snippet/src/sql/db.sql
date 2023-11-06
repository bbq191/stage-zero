drop table if exists course;

create table courses (
    id serial primary key,
    teacher_id INT not null,
    name varchar(140) not null,
    time timestamp default now()
);

insert into courses(id,teacher_id,name,time) values(1,1,'first','2023-08-07 16:00:00');
insert into courses(id,teacher_id,name,time) values(2,1,'second','2023-08-07 16:00:00');


SELECT * FROM courses;