-- Add download mirror source settings
-- game_file_source: 0=Mirror, 1=Auto (default), 2=Official
-- community_source: 0=Mirror, 1=Auto (default), 2=Official
ALTER TABLE settings ADD COLUMN game_file_source INTEGER NOT NULL DEFAULT 1;
ALTER TABLE settings ADD COLUMN community_source INTEGER NOT NULL DEFAULT 1;
