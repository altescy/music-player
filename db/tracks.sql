create table if not exists `tracks` (
    `id` int(10) unsigned not null AUTO_INCREMENT,
    `created_at` datetime not null,
    `provider_id` int(10) unsigned not null,
    `content_id` varchar(30) not null,
    primary key (`id`),
    unique key `content_unq` (`provider_id`, `content_id`)
) engine=InnoDB default charset=utf8;

