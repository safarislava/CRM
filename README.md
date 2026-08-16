# DailyCRM

Система управления проектами для строительных/инженерных организаций. Позволяет вести проекты, разбивать их на этапы и
подэтапы, отслеживать стоимость, дедлайны, подтверждения ГИП и оплаты, прикреплять файлы и акты, а также вести журнал
комментариев по каждому этапу.

## Стек

**Backend** — Rust, Actix-web 4, PostgreSQL (sqlx), MinIO (S3-совместимый объектный storage), In-Memory Caching (с
механизмом инвалидации), JWT (access + refresh токены), bcrypt, lettre (SMTP)

**Frontend** — React 18, TypeScript, Redux Toolkit, Vite, SCSS

**Инфраструктура** — Docker Compose, Caddy (reverse proxy + TLS), GitHub Container Registry

---

## Архитектура (Elegant Objects)

Проект строится строго по принципам Elegant Objects (EO): объекты владеют поведением и представляют себя сами, без геттеров/сеттеров, статических методов, DTO и наследования реализации. 

Действия инкапсулированы в объекты-глаголы (`Task`), а объекты чтения — в Noun-интерфейсы (`Value<T>`).

```
Backend/src/
├── endpoint/          # HTTP-обработчики и маршрутизация
│   ├── extractor/     # Извлечение параметров пути (ProjectId, StageId)
│   ├── auth/          # login, refresh, logout
│   ├── users/         # регистрация и профиль
│   ├── invites/       # создание инвайтов
│   ├── projects/      # проекты, этапы, файлы, акты, комментарии + media.rs
│   └── admin/         # служебные эндпойнты администратора
│
├── model/             # Чистая доменная модель (без зависимостей от Actix)
│   ├── contract/      # Базовые контракты (Task, Value<T>, Printer, BoxError)
│   ├── project/       # Проекты, этапы, инвалидация кэша, файлы, акты, комментарии
│   ├── user/          # Пользователи, роли, инвайты + действия с аккаунтами
│   ├── session/       # Токены (Access/Refresh/Claims/Cookie) + отзывы и чеки
│   ├── credential/    # Username, Password, хэшированные и валидированные обёртки
│   ├── notification/  # Очередь уведомлений, дайджесты и рассылка почты
│   ├── audit/         # AuditAction и декоратор логгирования AuditedTask
│   ├── cache/         # In-Memory cache (MemoryCache) для инвалидации сводок
│   └── schedule/      # Timetable, Schedule, TimeOfDay, PollInterval
│
├── middleware/        # JwtMiddleware, AdminMiddleware, login_governor (rate limiting)
├── state.rs           # AppState: PgPool, Storage, Mailer, Caches
├── storage.rs         # обёртка над aws-sdk-s3 (MinIO)
├── mail.rs            # Mailer через lettre/SMTP (SSL/TLS или STARTTLS)
├── jwt.rs             # подпись и верификация токенов
├── routes.rs          # регистрация всех HTTP-маршрутов
├── db.rs              # пул соединений PostgreSQL
└── cors.rs            # конфигурация CORS

Backend/migrations/    # SQL-миграции (sqlx migrate)

Frontend/src/
├── components/        # UI-компоненты
├── store/             # Redux state-менеджмент
├── styles/            # Стили SCSS/CSS
├── types/             # TS типы данных
├── App.tsx
├── main.tsx
└── App.module.scss

test/
└── load_test.js       # Скрипт нагрузочного тестирования k6
```

**Фоновые задачи** (запускаются при старте сервера):
- **12:00 ежедневно** — параллельная рассылка дайджеста дедлайнов пользователям с включёнными уведомлениями
- **каждую минуту** — параллельная отправка накопленных уведомлений из очереди

---

## Переменные окружения

В `.env` должны быть заполнены следующие значения.

| Переменная         | Описание                                                                            |
|--------------------|-------------------------------------------------------------------------------------|
| `DATABASE_URL`     | PostgreSQL connection string: `postgres://user:password@host:5432/crm`              |
| `JWT_SECRET`       | Секрет для подписи JWT-токенов                                                      |
| `MINIO_ENDPOINT`   | URL MinIO: `http://localhost:9000` (dev) или `http://minio:9000` (prod)             |
| `MINIO_ACCESS_KEY` | Access key MinIO                                                                    |
| `MINIO_SECRET_KEY` | Secret key MinIO                                                                    |
| `SMTP_HOST`        | SMTP-хост для отправки почты (например, `smtp.resend.com`)                          |
| `SMTP_PORT`        | SMTP-порт (`465` — SSL/TLS, `587` — STARTTLS, любой другой — без шифрования)        |
| `SMTP_USERNAME`    | SMTP-логин                                                                          |
| `SMTP_PASSWORD`    | SMTP-пароль                                                                         |
| `MAIL_FROM`        | Адрес отправителя, например `CRM <noreply@example.com>`                             |

Для prod-деплоя дополнительно нужны (используются в `docker-compose.prod.yml`):

| Переменная                | Описание                                  |
|---------------------------|-------------------------------------------|
| `DOMAIN`                  | Домен сайта, например `dailycrm.mooo.com` |
| `POSTGRES_USER`           | Пользователь PostgreSQL                   |
| `POSTGRES_PASSWORD`       | Пароль PostgreSQL                         |
| `GITHUB_REPOSITORY_OWNER` | GitHub-логин для pull образов из GHCR     |

---

## Локальный запуск
```bash
docker compose up -d
```
