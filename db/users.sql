create table if not exists `users` (
    `id` int(10) unsigned not null AUTO_INCREMENT,
    `created_at` datetime not null,
    `updated_at` datetime not null,
    `email` varchar(256) not null,
    primary key (`id`),
    unique key `email` (`email`)
) engine=InnoDB default charset=utf8;

