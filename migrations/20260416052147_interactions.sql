-- Add migration script here
CREATE TABLE interactions (
    id           VARCHAR(36)  PRIMARY KEY,
    session_id   VARCHAR(36)  REFERENCES stream_sessions(id),
    sender_type  ENUM('streamer', 'chat_user', 'bot') NOT NULL,
    sender_id    VARCHAR(36),
    sender_name  VARCHAR(100),
    content      TEXT         NOT NULL,
    response_bot_id VARCHAR(36) REFERENCES bots(id),
    response_content TEXT,
    filter_decision ENUM('accepted', 'rejected', 'neutral') DEFAULT 'accepted',
    filter_reason VARCHAR(255),
    created_at   DATETIME     DEFAULT CURRENT_TIMESTAMP
);