-- Add migration script here
CREATE TABLE bot_metrics (
    id           BIGINT       AUTO_INCREMENT PRIMARY KEY,
    bot_id       VARCHAR(36)  NOT NULL REFERENCES bots(id),
    session_id   VARCHAR(36)  REFERENCES stream_sessions(id),
    ctx_tokens_used INT,
    ctx_tokens_max  INT,
    messages_received INT     DEFAULT 0,
    messages_accepted INT     DEFAULT 0,
    messages_rejected INT     DEFAULT 0,
    responses_generated INT   DEFAULT 0,
    recorded_at  DATETIME     DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_bot_time (bot_id, recorded_at)
);