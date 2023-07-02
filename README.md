## Пакеты

### yeelight-protocol

Описание протокола. Запрос и ответ происходит в формате JSON. Сериализация / десериализация с помощью `serde`.

### yeelight-rust-sync-client

Синхронный клиент. Используется `std::net::TcpStream`.

### yeelight-rust-async-client

Асинхронный клиент. TODO.

### yeelight-rest-api

REST API для взаимодействия с лампами. Используется `axum`.

## Тестирование

```bash
cargo test -- --test-threads=1
```

Кол-во потоков равно 1, чтобы исключить параллельные запросы.
