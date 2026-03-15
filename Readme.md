# Rust Practice Plan

Цель: системно повторить и углубить ключевые темы Rust через реализацию структур, трейтов и небольших проектов.

---

# 1. Итераторы

## Цели

- Понимание трейта `Iterator`
- Реализация итераторов для собственных структур
- Работа с lifetime `'a`
- Понимание lazy iteration

## Практика

- RingBuffer
  - `iter()`
  - `iter_mut()`
  - `IntoIterator`

- Tree
  - DFS iterator
  - BFS iterator

- ChunkReader
  - чтение файла кусками через `Iterator`

## Дополнительно

- `DoubleEndedIterator`
- `ExactSizeIterator`
- `FusedIterator`

---

# 2. Реализация популярных трейтов

## Базовые трейты

- `Debug`
- `Display`
- `Default`
- `Clone`
- `Copy`

## Сравнение и хеширование

- `PartialEq`
- `Eq`
- `PartialOrd`
- `Ord`
- `Hash`

## Конвертации

- `From`
- `Into`
- `TryFrom`
- `TryInto`

## Работа с указателями

- `Deref`
- `DerefMut`
- `Borrow`
- `AsRef`
- `Drop`

## Практика

- `MyBox<T>` (аналог `Box`)
- `Email` type
- domain types

---

# 3. Сериализация и десериализация

## Цели

- Понимание архитектуры `serde`
- Ручная реализация сериализации

## Темы

- `Serialize`
- `Deserialize`
- `Serializer`
- `Deserializer`
- `Visitor`

## Практика

- ручная сериализация структуры
- кастомный текстовый формат
- версионирование структур

---

# 4. Многопоточность

## Основные примитивы

- `thread`
- `JoinHandle`
- `Arc`
- `Mutex`
- `RwLock`

## Практика

- producer / consumer
- thread pool

---

# 5. Атомарные операции

## Типы

- `AtomicUsize`
- `AtomicBool`
- `AtomicPtr`

## Темы

- memory ordering
- `Relaxed`
- `Acquire`
- `Release`
- `SeqCst`

## Практика

- atomic counter
- spin lock
- lock-free структуры

---

# 6. Async модель Rust

## Основы

- `Future`
- `Poll`
- `Pin`
- `Waker`
- `Context`

## Практика

- mini executor
- async TCP server
- async pipeline

---

# 7. Concurrency библиотеки

Изучить и использовать:

- `tokio`
- `futures`
- `rayon`
- `crossbeam`
- `parking_lot`

---

# 8. Криптография и структуры блокчейна

## Основные темы

- hashing
- Merkle tree
- транзакции
- блоки
- цепочка блоков

## Практика

- Merkle tree
- Transaction model
- Block structure
- Chain validation

---

# 9. Мини-проекты

Реализовать несколько небольших систем:

- Thread pool
- Async executor
- Merkle tree
- Simple blockchain

---

# 10. Используемые библиотеки

## Основные

- `serde`
- `anyhow`
- `thiserror`
- `bytes`
- `futures`

## Криптография

- `sha2`
- `blake3`
- `ed25519-dalek`

---

# Цель репозитория

Создать **персональную библиотеку знаний Rust**, содержащую:

- собственные реализации структур
- практику трейтов
- эксперименты с concurrency
- изучение async
- подготовку к blockchain-разработке
