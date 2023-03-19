-- SQL down migration generated for DBML up.sql file

ALTER TABLE `hidden_users` DROP FOREIGN KEY `hidden_users_user_id_fkey`;
DROP TABLE `hidden_users`;

ALTER TABLE `move_scores` DROP FOREIGN KEY `move_scores_move_id_fkey`;
DROP TABLE `move_scores`;

ALTER TABLE `password_reset_requests` DROP FOREIGN KEY `password_reset_requests_user_id_fkey`;
DROP TABLE `password_reset_requests`;

ALTER TABLE `times` DROP FOREIGN KEY `times_move_id_fkey`;
DROP TABLE `times`;

ALTER TABLE `points` DROP FOREIGN KEY `points_move_id_fkey`;
DROP TABLE `points`;

DROP TABLE `moves`;

ALTER TABLE `rounds` DROP FOREIGN KEY `rounds_game_id_fkey`;
DROP TABLE `rounds`;

ALTER TABLE `games` DROP FOREIGN KEY `games_creator_id_fkey`;
DROP TABLE `games`;

DROP TABLE `auth_user`;

