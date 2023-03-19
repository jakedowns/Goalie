-- SQL dump generated using DBML (dbml-lang.org)
-- Database: MySQL
-- Generated at: 2023-03-19T05:10:14.754Z

PRAGMA foreign_keys = ON;

CREATE TABLE `auth_user` (
  `id` UUID PRIMARY KEY,
  `username` varchar(255) UNIQUE NOT NULL,
  `email` varchar(255) UNIQUE NOT NULL,
  `password_hash` varchar(255) NOT NULL,
  `verified` boolean DEFAULT false,
  `remember_me` boolean DEFAULT false,
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now()),
  `deleted_at` timestamp
);

CREATE TABLE `games` (
  `id` UUID PRIMARY KEY,
  `name` varchar(255) NOT NULL,
  `creator_id` UUID NOT NULL,
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now()),
  `deleted_at` timestamp
);

CREATE TABLE `rounds` (
  `id` UUID PRIMARY KEY,
  `game_id` UUID NOT NULL,
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now()),
  `deleted_at` timestamp
);

CREATE TABLE `moves` (
  `id` UUID PRIMARY KEY,
  `round_id` UUID NOT NULL,
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now()),
  `deleted_at` timestamp
);

CREATE TABLE `points` (
  `id` UUID PRIMARY KEY,
  `move_id` UUID NOT NULL,
  `value` int NOT NULL,
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now()),
  `deleted_at` timestamp
);

CREATE TABLE `times` (
  `id` UUID PRIMARY KEY,
  `move_id` UUID NOT NULL,
  `value` timestamp NOT NULL,
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now()),
  `deleted_at` timestamp
);

CREATE TABLE `password_reset_requests` (
  `id` UUID PRIMARY KEY,
  `user_id` UUID NOT NULL,
  `token` varchar(255) UNIQUE NOT NULL,
  `expires_at` timestamp NOT NULL,
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now()),
  `deleted_at` timestamp
);

CREATE TABLE `move_scores` (
  `id` UUID PRIMARY KEY,
  `move_id` UUID NOT NULL,
  `value` int NOT NULL,
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now()),
  `deleted_at` timestamp
);

CREATE TABLE `hidden_users` (
  `id` UUID PRIMARY KEY,
  `user_id` UUID NOT NULL,
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now())
);

ALTER TABLE `games` ADD FOREIGN KEY (`creator_id`) REFERENCES `auth_user` (`id`);

ALTER TABLE `rounds` ADD FOREIGN KEY (`game_id`) REFERENCES `games` (`id`);

ALTER TABLE `moves` ADD FOREIGN KEY (`round_id`) REFERENCES `rounds` (`id`);

ALTER TABLE `points` ADD FOREIGN KEY (`move_id`) REFERENCES `moves` (`id`);

ALTER TABLE `times` ADD FOREIGN KEY (`move_id`) REFERENCES `moves` (`id`);

ALTER TABLE `password_reset_requests` ADD FOREIGN KEY (`user_id`) REFERENCES `auth_user` (`id`);

ALTER TABLE `move_scores` ADD FOREIGN KEY (`move_id`) REFERENCES `moves` (`id`);

ALTER TABLE `hidden_users` ADD FOREIGN KEY (`user_id`) REFERENCES `auth_user` (`id`);
