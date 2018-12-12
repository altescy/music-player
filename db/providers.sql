create table if not exists `providers` (
    `id` int(10) unsigned not null AUTO_INCREMENT,
    `name` varchar(20) not null,
    primary key (`id`),
    unique key `name` (`name`)
) engine=InnoDB default charset=utf8;

insert into `providers` (`name`) values ('youtube');
insert into `providers` (`name`) values ('soundcloud');
