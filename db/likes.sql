create table if not exists `likes` (
    `id` int(10) unsigned not null AUTO_INCREMENT,
    `created_at` datetime not null,
    `updated_at` datetime not null,
    `deleted_at` datetime,
    `user_id` int(10) unsigned not null,
    `track_id` int(10) unsigned not null,
    primary key (`id`),
    unique key `content_unq` (`user_id`, `track_id`)
) engine=InnoDB default charset=utf8;

