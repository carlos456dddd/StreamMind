# 🤖 StreamMind — Plataforma Multi-Bot con IA para Streaming
 
> Sistema de agentes conversacionales con personalidad parametrizable, síntesis de voz, contexto persistente y panel de monitoreo en tiempo real, diseñado para integrarse con streams de YouTube. Desarrollado en **Rust**.
 
---
 
## 📋 Tabla de Contenidos
 
- [Visión General](#visión-general)
- [Características Principales](#características-principales)
- [Arquitectura del Sistema](#arquitectura-del-sistema)
- [Stack Tecnológico y Librerías Rust](#stack-tecnológico-y-librerías-rust)
- [Modelo de Datos](#modelo-de-datos)
- [Sistema de Personalidad y Stats](#sistema-de-personalidad-y-stats)
- [Motor de Contexto y Memoria](#motor-de-contexto-y-memoria)
- [Flujo de Comunicación](#flujo-de-comunicación)
- [Panel de Monitoreo](#panel-de-monitoreo)
- [Síntesis de Voz](#síntesis-de-voz)
- [Integración con YouTube](#integración-con-youtube)
- [Configuración del Entorno](#configuración-del-entorno)
- [Estructura del Proyecto](#estructura-del-proyecto)
- [Roadmap de Desarrollo](#roadmap-de-desarrollo)
- [Consideraciones de Rendimiento](#consideraciones-de-rendimiento)
 
---
 
## Visión General
 
**StreamMind** es una plataforma de escritorio que orquesta hasta **10 bots de IA** con personalidades individuales, capaces de:
 
- Interactuar entre sí y con el streamer en tiempo real
- Escuchar y responder mensajes del chat de YouTube
- Sintetizar respuestas en voz usando TTS local o en la nube
- Mantener su propio contexto de memoria persistido en MySQL
- Filtrar, aceptar o rechazar información según su personalidad
- Ser monitoreados en tiempo real mediante un dashboard de escritorio
 
Cada bot tiene una **personalidad parametrizable con stats numéricos** que determina su comportamiento conversacional, su apertura al aprendizaje, su disposición a interactuar con otros bots o con el streamer, y cómo prioriza o descarta información entrante.
 
---
 
## Características Principales
 
### Bots con Personalidad Parametrizable
Cada bot tiene un perfil de stats (valores de 0.0 a 1.0) que definen su comportamiento:
 
| Stat | Descripción |
|---|---|
| `openness` | Qué tan dispuesto está a recibir nueva información |
| `sociability` | Probabilidad de responder a otro bot o al streamer |
| `retention` | Cuánta información retiene en su contexto de largo plazo |
| `agreeableness` | Si tiende a estar de acuerdo o a generar opinión contraria |
| `volatility` | Qué tan seguido decide hablar sin ser directamente mencionado |
| `loyalty` | Cuánto prioriza lo que dice el streamer sobre el chat |
 
### Contexto Individual por Bot
- Cada bot tiene su propio **buffer de contexto activo** (ventana de tokens)
- Las interacciones pasan por un **filtro de relevancia** basado en personalidad
- La información se clasifica como: `neutral` (siempre retenida), `positive` (retenida según `openness`), `negative` (ignorada o contradecida según `agreeableness`)
- Contexto persistido en MySQL por sesión
 
### Información Neutral Siempre Recordada
La información del stream como horarios, nombre del stream, juego en curso, anuncios del streamer, son marcados automáticamente como **tipo NEUTRAL** y siempre son almacenados y recuperables por cualquier bot, independientemente de sus stats.
 
### Síntesis de Voz por Bot
Cada bot tiene su propia voz TTS configurada. Las respuestas se sintetizan y reproducen, con control de cola para evitar solapamiento.
 
### Panel de Monitoreo en Tiempo Real
Interfaz gráfica de escritorio que muestra por cada bot:
- Uso actual del contexto (tokens usados / máximo permitido)
- Estado de actividad (thinking / speaking / idle / blocked)
- Últimas memorias almacenadas
- Decisiones de filtrado (qué información fue rechazada y por qué)
- Gráficas de interacciones en el tiempo
 
---
 
## Arquitectura del Sistema
 
Se adopta una **Arquitectura de Actores con Bus de Eventos Central**, combinada con capas de servicio bien definidas. Esta arquitectura es especialmente adecuada para Rust por su modelo de ownership y las primitivas de concurrencia de Tokio.
 
```
┌─────────────────────────────────────────────────────────────────┐
│                     APLICACIÓN DE ESCRITORIO                    │
│  ┌──────────────┐  ┌────────────────────────────────────────┐  │
│  │  UI / Panel  │  │           Event Bus (Tokio MPSC)        │  │
│  │  (egui/iced) │◄─┤  StreamEvent | BotMessage | VoiceCmd   │  │
│  └──────┬───────┘  └───────────────────┬────────────────────┘  │
│         │                              │                        │
│  ┌──────▼──────────────────────────────▼───────────────┐       │
│  │              Orquestador Central (BotManager)        │       │
│  │   - Distribuye mensajes según reglas de routing      │       │
│  │   - Controla turnos de habla (anti-colisión)         │       │
│  │   - Gestiona el ciclo de vida de cada bot            │       │
│  └──────┬────────────────────────────────────┬──────────┘       │
│         │                                    │                  │
│  ┌──────▼──────────────────────────────────────────────┐        │
│  │                  Pool de Bot Actors                  │        │
│  │                                                      │        │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐  ┌─────────┐  │        │
│  │  │  Bot 1  │ │  Bot 2  │ │  Bot 3  │..│  Bot N  │  │        │
│  │  │ Actor   │ │ Actor   │ │ Actor   │  │ Actor   │  │        │
│  │  │─────────│ │─────────│ │─────────│  │─────────│  │        │
│  │  │PersonaDB│ │PersonaDB│ │PersonaDB│  │PersonaDB│  │        │
│  │  │Context  │ │Context  │ │Context  │  │Context  │  │        │
│  │  │Filter   │ │Filter   │ │Filter   │  │Filter   │  │        │
│  │  └────┬────┘ └────┬────┘ └────┬────┘  └────┬────┘  │        │
│  └───────┼───────────┼───────────┼─────────────┼───────┘        │
│          │           │           │             │                 │
│  ┌───────▼───────────▼───────────▼─────────────▼───────┐        │
│  │                  Capa de Servicios                   │        │
│  │  ┌────────────┐ ┌────────────┐ ┌──────────────────┐ │        │
│  │  │ LLM Service│ │ TTS Service│ │  Context Service  │ │        │
│  │  │(Ollama/API)│ │(local/nube)│ │  (MySQL + cache)  │ │        │
│  │  └────────────┘ └────────────┘ └──────────────────┘ │        │
│  └──────────────────────────────────────────────────────┘        │
│                                                                  │
│  ┌───────────────────────────────────────────────────────┐       │
│  │              Conectores Externos                       │       │
│  │  ┌──────────────────┐  ┌──────────────────────────┐  │       │
│  │  │  YouTube Live Chat│  │  Micrófono / STT (Whisper)│  │       │
│  │  │  (polling API)   │  │  (voz del streamer)       │  │       │
│  │  └──────────────────┘  └──────────────────────────┘  │       │
│  └───────────────────────────────────────────────────────┘       │
└─────────────────────────────────────────────────────────────────┘
```
 
### Patrón de Actor por Bot
 
Cada bot corre como una **tarea Tokio independiente** con su propio canal MPSC. El orquestador le envía mensajes y el bot decide si responder, ignorar, o encolar para después, todo en base a sus stats.
 
```
[Evento Entrante]
       │
       ▼
[Filtro de Personalidad]
  ¿Debo procesar esto?
  (openness, loyalty, sociability)
       │
   Sí  │  No → [Descartado / Logueado en Monitor]
       ▼
[Clasificación de Info]
  NEUTRAL → siempre al contexto
  POSITIVE → al contexto si pasa umbral
  NEGATIVE → genera opinión contraria o ignora
       │
       ▼
[Construcción de Prompt]
  sistema + contexto histórico + mensaje actual
       │
       ▼
[LLM Request (streaming)]
       │
       ▼
[TTS → Cola de Audio]
       │
       ▼
[Persistencia en MySQL]
```
 
---
 
## Stack Tecnológico y Librerías Rust
 
### Core Async & Concurrencia
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }        # Runtime async, tareas, canales MPSC
tokio-stream = "0.1"                                   # Manejo de streams async (streaming LLM)
rayon = "1"                                            # Paralelismo en CPU (procesamiento de contexto)
```
 
### LLM — Modelos Gratuitos (Ollama local)
```toml
ollama-rs = "0.2"                                      # Cliente oficial para Ollama (local, gratis)
reqwest = { version = "0.12", features = ["json", "stream"] }  # HTTP client + streaming SSE
serde = { version = "1", features = ["derive"] }       # Serialización de prompts y respuestas
serde_json = "1"                                       # JSON para API de Ollama/OpenAI-compat
```
 
> **Nota sobre modelos gratuitos:** Se usa [Ollama](https://ollama.com) como backend LLM local. Modelos recomendados por caso de uso:
> - `llama3.2:3b` → bots ligeros, respuesta rápida
> - `mistral:7b` → balance calidad/velocidad
> - `phi3:mini` → muy liviano, ideal para múltiples bots simultáneos
> - `gemma2:2b` → buena comprensión de contexto largo
>
> También compatible con APIs OpenAI-compatible gratuitas (Groq, OpenRouter, etc.)
 
### Interfaz Gráfica de Escritorio (Panel de Monitoreo)
```toml
egui = "0.29"                                          # UI inmediata, liviana, cross-platform
eframe = "0.29"                                        # Framework de ventana para egui
egui_plot = "0.29"                                     # Gráficas en tiempo real dentro de egui
```
 
> **¿Por qué egui?** Es nativo en Rust, no requiere runtime JS, tiene excelente rendimiento para dashboards de actualización frecuente y permite integración directa con el estado de la app.
 
### Base de Datos — MySQL (Contexto Persistente)
```toml
sqlx = { version = "0.8", features = ["mysql", "runtime-tokio-rustls", "chrono", "uuid"] }
uuid = { version = "1", features = ["v4"] }            # IDs únicos por bot/sesión/memoria
chrono = { version = "0.4", features = ["serde"] }     # Timestamps para memorias y sesiones
```
 
### Síntesis de Voz (TTS)
```toml
# Opción A: TTS local con Coqui/XTTS via subprocess
rodio = "0.19"                                         # Reproducción de audio (WAV/MP3/OGG)
hound = "3"                                            # Lectura/escritura WAV para pipeline TTS
 
# Opción B: TTS en la nube (ElevenLabs, OpenAI TTS — free tier)
# Se usa reqwest para llamadas HTTP a la API
```
 
> **Recomendación TTS gratuita:**
> - **Piper TTS** (local, offline): binario que se llama via `std::process::Command`, voces descargables
> - **Kokoro TTS** (local, alta calidad): modelo ONNX ejecutable localmente
> - **ElevenLabs free tier**: 10,000 caracteres/mes, ideal para prototipar
 
### Reconocimiento de Voz del Streamer (STT)
```toml
whisper-rs = "0.11"                                    # Bindings a whisper.cpp para STT local
cpal = "0.15"                                          # Captura de audio del micrófono (cross-platform)
```
 
### Integración YouTube Live Chat
```toml
reqwest = { version = "0.12", features = ["json"] }   # Polling a YouTube Data API v3
tokio = { version = "1", features = ["time"] }         # Intervalos de polling
```
 
> La YouTube Data API v3 es gratuita con cuota diaria (10,000 unidades/día). El polling de liveChatMessages consume ~1 unidad por request.
 
### Utilidades
```toml
config = "0.14"                                        # Configuración desde TOML/ENV
tracing = "0.1"                                        # Logging estructurado async-aware
tracing-subscriber = "0.3"                             # Salida de logs a consola/archivo
anyhow = "1"                                           # Manejo de errores ergonómico
thiserror = "1"                                        # Tipos de error personalizados
async-trait = "0.1"                                    # Traits async (para interfaces de servicio)
dashmap = "6"                                          # HashMap concurrente sin locks manuales
parking_lot = "0.12"                                   # Mutex/RwLock más rápidos que std
```
 
---
 
## Modelo de Datos
 
### MySQL — Esquema Principal
 
```sql
-- Bots y sus personalidades
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
 
-- Sesiones de stream
CREATE TABLE stream_sessions (
    id           VARCHAR(36)  PRIMARY KEY,
    stream_title VARCHAR(255),
    youtube_chat_id VARCHAR(255),
    started_at   DATETIME     DEFAULT CURRENT_TIMESTAMP,
    ended_at     DATETIME
);
 
-- Memorias individuales por bot
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
 
-- Historial de interacciones
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
 
-- Métricas de monitoreo por bot (para el dashboard)
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
```
 
---
 
## Sistema de Personalidad y Stats
 
Cada stat es un valor `f32` entre `0.0` y `1.0` que actúa como umbral o peso probabilístico en distintas decisiones:
 
```
Decisión de responder a un mensaje:
  prob = sociability * (loyalty si es streamer, else 0.5) + volatility * rand()
  Si prob > RESPONDER_THRESHOLD → genera respuesta
 
Decisión de retener información:
  Si tipo == NEUTRAL → siempre retener
  Si tipo == POSITIVE → retener si rand() < openness * retention
  Si tipo == NEGATIVE → 
    Si agreeableness < 0.4 → generar contraopinión
    Si agreeableness >= 0.4 → ignorar silenciosamente
 
Interacción bot-a-bot:
  Un bot B puede responder a bot A si:
    sociability_B > 0.5 AND rand() < sociability_B * 0.7
    (Los bots muy introvertidos hablan poco entre sí)
```
 
---
 
## Motor de Contexto y Memoria
 
### Ventana de Contexto Activo
Cada bot mantiene en memoria RAM un `VecDeque<Message>` que representa su contexto activo. Tiene un límite de tokens (`max_ctx_tokens`). Cuando el contexto se llena:
 
1. Los mensajes más antiguos de tipo `positive` o `negative` son desalojados primero
2. Los mensajes de tipo `neutral` y `stream_info` son protegidos del desalojo
3. Al desalojar, se persiste en MySQL con su `relevance_score` actualizado
 
### Construcción del Prompt
```
[SYSTEM PROMPT del bot — fijo]
[CONTEXTO STREAM INFO — siempre incluido]
[MEMORIAS RELEVANTES — recuperadas de MySQL por similitud/recencia]
[HISTORIAL RECIENTE — últimas N interacciones del buffer activo]
[MENSAJE ACTUAL]
```
 
### Recuperación de Memorias
Para cada nueva interacción, el `ContextService` hace:
1. Recupera las últimas N memorias del bot de la sesión actual
2. Filtra por relevance_score > umbral según `retention` del bot
3. Siempre incluye todas las memorias de tipo `stream_info`
4. Prioriza memorias recientes con decaimiento exponencial
 
---
 
## Flujo de Comunicación
 
```
[YouTube Chat / Micrófono]
         │
         ▼
   [Event Collector]
    - Normaliza eventos
    - Clasifica fuente (streamer / chat_user)
    - Emite StreamEvent al BotManager
         │
         ▼
   [BotManager — Orquestador]
    - Determina qué bots deben recibir el evento
    - Aplica routing: broadcast a todos o dirigido
    - Controla turno de habla (solo 1 bot habla a la vez)
    - Timeout de respuesta por bot (evita bloqueos)
         │
    ┌────┴────┐
    │  Bot 1  │ ← recibe evento, decide responder o no
    │  Bot 2  │ ← idem
    │   ...   │
    └────┬────┘
         │ (solo el bot seleccionado responde)
         ▼
   [LLM Service — Ollama]
    - Construye prompt con contexto
    - Llama al modelo (streaming)
    - Emite tokens en tiempo real al panel
         │
         ▼
   [TTS Service]
    - Convierte texto a audio
    - Agrega a cola de reproducción
         │
         ▼
   [Audio Output]
    - Reproduce la respuesta en voz
         │
         ▼
   [Context + MySQL]
    - Persiste la interacción
    - Actualiza métricas del bot
    - Actualiza memorias si corresponde
```
 
### Interacción Bot-a-Bot
Cuando un bot emite una respuesta, el BotManager la reinyecta como un `StreamEvent` de tipo `BotMessage`. Los demás bots pueden decidir responderla según sus stats de sociabilidad. Para evitar loops infinitos:
- Máximo de 3 turnos consecutivos de bot-a-bot sin intervención humana
- Cooldown por bot después de responder (configurable)
- El volatility controla si un bot "se mete" en conversaciones ajenas
 
---
 
## Panel de Monitoreo
 
El dashboard en `egui` muestra en tiempo real:
 
```
┌───────────────────────────────────────────────────────┐
│  StreamMind Monitor             [Sesión: 2h 14m]      │
├──────────────┬────────────────────────────────────────┤
│  Bot List    │  Bot: "Aria" (llama3.2:3b)             │
│              │  Estado: 🟢 Thinking...                 │
│  ● Aria      │  ┌─────────────────────────────────┐   │
│  ● Rex       │  │ Contexto: ████████░░ 3.2k/4k tok│   │
│  ○ Miko      │  └─────────────────────────────────┘   │
│  ● ...       │                                        │
│              │  Stats:                                 │
│              │  Openness   ████████░░ 0.82            │
│              │  Sociabilty ██████░░░░ 0.61            │
│              │  Retention  █████████░ 0.90            │
│              │                                        │
│              │  Últimas Memorias:                     │
│              │  [NEUTRAL] "Stream empieza a las 8pm"  │
│              │  [POSITIVE] "Ganamos la partida"       │
│              │  [REJECTED] "El juego es aburrido" ✗   │
│              │                                        │
│              │  Interacciones (últimos 10 min):       │
│              │  📈 [gráfica de barras]                │
├──────────────┴────────────────────────────────────────┤
│  Log de Eventos:                                      │
│  [14:32:01] Chat → Aria: "¿qué opinas del nuevo mapa?"│
│  [14:32:03] Aria → ACEPTADO → generando respuesta...  │
│  [14:32:05] Rex → RECHAZADO (sociability check: 0.21) │
└───────────────────────────────────────────────────────┘
```
 
---
 
## Síntesis de Voz
 
### Pipeline TTS recomendado (gratuito, local)
 
**Piper TTS** — Opción principal:
- Binario standalone, sin GPU requerida
- Más de 40 voces en múltiples idiomas (incluyendo español)
- Se invoca con `std::process::Command` desde Rust
- Salida en WAV, reproducida con `rodio`
 
```rust
// Ejemplo de invocación desde Rust
Command::new("piper")
    .args(["--model", &bot.voice_id, "--output_file", "out.wav"])
    .stdin(Stdio::piped())
    .spawn()?;
```
 
**Cola de audio** — Sin solapamiento entre bots:
- Solo un bot habla a la vez (mutex en el AudioManager)
- Los demás bots que quieren hablar esperan en cola o descartan si el mensaje expira
 
---
 
## Integración con YouTube
 
### YouTube Data API v3 — Live Chat Polling
 
```
GET https://www.googleapis.com/youtube/v3/liveChat/messages
    ?liveChatId={CHAT_ID}
    &part=snippet,authorDetails
    &pageToken={NEXT_PAGE_TOKEN}
    &key={API_KEY}
```
 
- Polling cada 5-8 segundos (respetando `pollingIntervalMillis` de la API)
- Los mensajes nuevos se emiten como `StreamEvent::ChatMessage`
- Se filtra el mensaje del streamer por su channel ID configurado
 
### Speech-to-Text del Streamer (Voz)
- `cpal` captura audio del micrófono en tiempo real
- Chunks de audio se pasan a `whisper-rs` para transcripción
- La transcripción se emite como `StreamEvent::StreamerVoice`
- Tiene prioridad máxima sobre mensajes del chat
 
---
 
## Configuración del Entorno
 
```toml
# config/default.toml
 
[database]
url = "mysql://user:password@localhost:3306/streammind"
pool_size = 10
 
[ollama]
host = "http://localhost:11434"
default_model = "llama3.2:3b"
 
[youtube]
api_key = "TU_API_KEY_AQUI"
live_chat_id = "ID_DEL_CHAT_DE_TU_STREAM"
streamer_channel_id = "TU_CHANNEL_ID"
poll_interval_ms = 6000
 
[tts]
engine = "piper"                        # "piper" | "elevenlabs" | "kokoro"
piper_binary = "/usr/local/bin/piper"
piper_models_dir = "./models/piper"
audio_output_device = "default"
 
[bot_manager]
max_bots = 10
max_bot_to_bot_turns = 3               # Máximo de turnos bot-a-bot sin intervención
bot_cooldown_ms = 3000                 # Tiempo mínimo entre respuestas del mismo bot
response_timeout_ms = 15000           # Timeout para que un bot genere respuesta
 
[monitor]
metrics_interval_ms = 1000            # Frecuencia de actualización del dashboard
log_rejected_messages = true
```
 
---
 
## Estructura del Proyecto
 
```
streammind/
├── Cargo.toml
├── config/
│   ├── default.toml
│   └── bots/                          # Configs TOML de cada bot
│       ├── aria.toml
│       ├── rex.toml
│       └── ...
├── migrations/                        # Migraciones SQL (sqlx migrate)
│   ├── 001_init_schema.sql
│   └── 002_add_metrics.sql
├── models/
│   └── piper/                         # Modelos de voz TTS
├── src/
│   ├── main.rs                        # Entry point: init tokio, egui window
│   ├── app.rs                         # Estado global de la app + loop de UI
│   │
│   ├── bot/
│   │   ├── mod.rs
│   │   ├── actor.rs                   # Tarea Tokio por bot (loop de mensajes)
│   │   ├── personality.rs             # Stats + lógica de filtrado y decisión
│   │   ├── context.rs                 # Buffer de contexto activo (VecDeque)
│   │   └── prompt_builder.rs         # Construcción del prompt con contexto
│   │
│   ├── manager/
│   │   ├── mod.rs
│   │   ├── bot_manager.rs            # Orquestador: routing + turnos de habla
│   │   └── turn_controller.rs        # Control anti-colisión de respuestas
│   │
│   ├── services/
│   │   ├── llm/
│   │   │   ├── mod.rs
│   │   │   ├── ollama_client.rs      # Cliente Ollama con streaming
│   │   │   └── openai_compat.rs      # Cliente para APIs OpenAI-compat (Groq, etc.)
│   │   ├── tts/
│   │   │   ├── mod.rs
│   │   │   ├── piper.rs              # Integración con Piper TTS
│   │   │   └── audio_queue.rs        # Cola de reproducción de audio
│   │   ├── stt/
│   │   │   ├── mod.rs
│   │   │   └── whisper.rs            # Captura + transcripción de voz
│   │   └── context_service.rs        # CRUD de memorias en MySQL
│   │
│   ├── connectors/
│   │   ├── youtube.rs                # Polling de YouTube Live Chat
│   │   └── microphone.rs             # Captura de audio del streamer
│   │
│   ├── events/
│   │   └── mod.rs                    # Definición de StreamEvent + canales
│   │
│   ├── db/
│   │   ├── mod.rs
│   │   ├── models.rs                 # Structs que mapean a tablas MySQL
│   │   └── queries.rs                # Queries reutilizables con sqlx
│   │
│   └── ui/
│       ├── mod.rs
│       ├── dashboard.rs              # Panel principal (egui)
│       ├── bot_panel.rs              # Panel de detalle por bot
│       ├── metrics_chart.rs          # Gráficas de interacciones
│       └── event_log.rs             # Log en tiempo real
└── tests/
    ├── personality_tests.rs
    └── context_tests.rs
```
 
---
 
## Roadmap de Desarrollo
 
### Fase 1 — Infraestructura Base (semanas 1-3)
- [ ] Setup del proyecto Rust + configuración de sqlx migrations
- [ ] Esquema MySQL y modelos de datos
- [ ] Bot Actor básico con canal MPSC
- [ ] Integración con Ollama (sin streaming)
- [ ] BotManager con routing básico
 
### Fase 2 — Personalidad y Contexto (semanas 4-6)
- [ ] Sistema de stats y filtrado de mensajes
- [ ] Buffer de contexto activo con límite de tokens
- [ ] Clasificación de información (neutral/positive/negative)
- [ ] Persistencia de memorias en MySQL
- [ ] Construcción dinámica de prompts con contexto
 
### Fase 3 — Voz y Audio (semanas 7-8)
- [ ] Integración con Piper TTS
- [ ] Cola de audio sin solapamiento (AudioManager)
- [ ] Captura de voz del streamer con CPAL
- [ ] Transcripción con Whisper-rs
 
### Fase 4 — YouTube y Streaming (semanas 9-10)
- [ ] Polling de YouTube Live Chat API
- [ ] Normalización de eventos de chat
- [ ] Detección y priorización de mensajes del streamer
- [ ] Manejo de errores y reconexión automática
 
### Fase 5 — Panel de Monitoreo (semanas 11-13)
- [ ] Ventana egui base con lista de bots
- [ ] Panel de detalle por bot (contexto, stats, memorias)
- [ ] Gráficas de interacciones en tiempo real (egui_plot)
- [ ] Log de eventos filtrado y coloreado
 
### Fase 6 — Pulido y Configuración (semanas 14-16)
- [ ] UI de configuración de bots (editar stats en vivo)
- [ ] Streaming de tokens del LLM al panel
- [ ] Exportación de sesiones y memorias
- [ ] Optimización de uso de memoria y tokens
- [ ] Tests de integración
 
---
 
## Consideraciones de Rendimiento
 
### Por qué Rust es ideal aquí
- **Modelo de ownership**: garantiza que el contexto de cada bot no sea accedido por múltiples threads sin sincronización explícita
- **Tokio async**: hasta 10 bots generando respuestas concurrentes sin bloquear el hilo de UI
- **Bajo consumo de memoria**: crítico cuando se corren múltiples modelos LLM en la misma máquina
- **Latencia predecible**: sin GC pauses que afecten el timing del audio
 
### Gestión de Recursos con Múltiples Bots
- Solo 1-2 bots generan respuesta simultáneamente (controlado por el TurnController)
- Los modelos LLM se cargan una sola vez en Ollama y son compartidos por todos los bots
- El contexto en RAM por bot es limitado: ~4k tokens ≈ ~16KB por bot → 160KB máximo para 10 bots
- Las queries a MySQL son todas async y usan connection pool
