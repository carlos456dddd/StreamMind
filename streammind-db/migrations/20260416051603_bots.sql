-- Add migration script here
CREATE TABLE bots (
    id           VARCHAR(36)  PRIMARY KEY,  -- UUID
    name         VARCHAR(100) NOT NULL,
    voice_id     VARCHAR(100),              -- ID de voz TTS
    model_name   VARCHAR(100) NOT NULL,     -- ej: "llama3.2:3b"
    system_prompt TEXT        NOT NULL,     -- prompt base de personalidad
    openness     FLOAT        DEFAULT 0.7,
    sociability  FLOAT        DEFAULT 0.6,
    retention    FLOAT        DEFAULT 0.8,
    agreeableness FLOAT       DEFAULT 0.5,
    volatility   FLOAT        DEFAULT 0.3,
    loyalty      FLOAT        DEFAULT 0.7,
    max_ctx_tokens INT        DEFAULT 4096,
    is_active    BOOLEAN      DEFAULT TRUE,
    created_at   DATETIME     DEFAULT CURRENT_TIMESTAMP
);