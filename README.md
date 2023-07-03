## Пакеты

### yeelight-protocol

Описание протокола. Запрос и ответ происходит в формате JSON. Сериализация / десериализация с помощью [serde](https://github.com/serde-rs/json).

### yeelight-rust-sync-client

Синхронный клиент. Используется [std::net::TcpStream](https://doc.rust-lang.org/std/net/struct.TcpStream.html).

### yeelight-rust-async-client

Асинхронный клиент. TODO.

### yeelight-rest-api

REST API для взаимодействия с лампами. Используется [axum](https://github.com/tokio-rs/axum).

## Тестирование

```bash
cargo test -- --test-threads=1
```

Кол-во потоков 1, чтобы исключить параллельные запросы к лампе.
