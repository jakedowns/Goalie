-- SQL dump generated using DBML (dbml-lang.org)
-- Database: MySQL
-- Generated at: 2023-03-19T05:10:14.754Z

PRAGMA foreign_keys = ON;

CREATE TABLE `users` (
  `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  `username` varchar(255) UNIQUE NOT NULL,
  `email` varchar(255) UNIQUE NOT NULL,
  `password_hash` varchar(255) NOT NULL,
  `verified` boolean DEFAULT false,
  `remember_me` boolean DEFAULT false,
  `verification_token` varchar(255) UNIQUE,
  `password_reset_token` varchar(255) UNIQUE,
  `last_logged_in_at` timestamp DEFAULT null,
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now()),
  `deleted_at` timestamp
);

CREATE TABLE `games` (
  `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  `name` varchar(255) NOT NULL,
  `creator_id` int NOT NULL,
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now()),
  `deleted_at` timestamp
);

CREATE TABLE `rounds` (
  `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  `game_id` int NOT NULL,
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now()),
  `deleted_at` timestamp
);

CREATE TABLE `moves` (
  `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  `player_id` int NOT NULL,
  `round_id` int,
  `game_id` int NOT NULL,
  `points_id` int,
  `times_id` int,
  `move_type` int NOT NULL,
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now()),
  `deleted_at` timestamp
);

CREATE TABLE `points` (
  `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  `move_id` int NOT NULL,
  `value` int NOT NULL,
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now()),
  `deleted_at` timestamp
);

CREATE TABLE `times` (
  `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  `move_id` int NOT NULL,
  `value` timestamp NOT NULL,
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now()),
  `deleted_at` timestamp
);

CREATE TABLE `password_reset_requests` (
  `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  `user_id` int NOT NULL,
  `token` varchar(255) UNIQUE NOT NULL,
  `expires_at` timestamp NOT NULL,
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now()),
  `deleted_at` timestamp
);

/* pre-defined scores for move types */
CREATE TABLE `move_scores` (
  `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  `move_type` int NOT NULL,
  `value` int NOT NULL,
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now()),
  `deleted_at` timestamp
);

CREATE TABLE `hidden_users` (
  `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  `user_id` int NOT NULL,
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now())
);
