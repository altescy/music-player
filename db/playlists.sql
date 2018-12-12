create table if not exists `playlists` (
    `id` int(10) unsigned not null AUTO_INCREMENT,
    `created_at` datetime not null,
    `updated_at` datetime not null,
    `deleted_at` datetime,
    `user_id` int(10) unsigned not null,
    `name` varchar(100) not null,
    `token` varchar(10) not null,
    primary key (`id`),
    unique key `token` (`token`)
) engine=InnoDB default charset=utf8;

create index `playlist_user_id` on `playlists` (`user_id`);
