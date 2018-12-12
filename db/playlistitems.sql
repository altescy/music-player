create table if not exists `playlistitems` (
    `id` int(10) unsigned not null AUTO_INCREMENT,
    `created_at` datetime not null,
    `updated_at` datetime not null,
    `deleted_at` datetime,
    `playlist_id` int(10) unsigned not null,
    `track_id` int(10) unsigned not null,
    `rankorder` int(10) unsigned not null,
    primary key (`id`),
    unique key `playlistitem_unq` (`playlist_id`, `rankorder`)
) engine=InnoDB default charset=utf8;

