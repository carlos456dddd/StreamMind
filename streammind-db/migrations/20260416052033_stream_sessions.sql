-- Add migration script here
CREATE TABLE stream_sessions (
    id           VARCHAR(36)  PRIMARY KEY,
    stream_title VARCHAR(255),
    youtube_chat_id VARCHAR(255),
    started_at   DATETIME     DEFAULT CURRENT_TIMESTAMP,
    ended_at     DATETIME
);