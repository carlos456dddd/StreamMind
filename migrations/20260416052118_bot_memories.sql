-- Add migration script here
CREATE TABLE bot_memories (
    id           VARCHAR(36)  PRIMARY KEY,
    bot_id       VARCHAR(36)  NOT NULL REFERENCES bots(id),
    session_id   VARCHAR(36)  REFERENCES stream_sessions(id),
    content      TEXT         NOT NULL,
    memory_type  ENUM('neutral', 'positive', 'negative', 'stream_info') NOT NULL,
    relevance_score FLOAT     DEFAULT 1.0,
    source       ENUM('streamer', 'chat', 'bot', 'system') NOT NULL,
    source_name  VARCHAR(100),
    created_at   DATETIME     DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_bot_session (bot_id, session_id),
    INDEX idx_memory_type (bot_id, memory_type)
);