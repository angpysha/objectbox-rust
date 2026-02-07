# 🎯 Пріоритетні завдання для ObjectBox Rust

*Створено: 2026-02-01*

Цей документ містить структурований список всіх TODO з проекту, організованих за пріоритетом для подальшої роботи агентів.

---

## 🔴 КРИТИЧНИЙ ПРІОРИТЕТ

### 1. Relations (Зв'язки між сутностями) ✅ DONE
**Статус**: Реалізовано (2026-02-06)  
**Файли**: `src/relations/to_one.rs`, `src/relations/to_many.rs`, `src/relations/info.rs`, `generator/src/model_json.rs`, `generator/src/code_gen.rs`, `macros/src/property.rs`, `macros/src/entity.rs`, `src/box.rs`

**Виконано**:
- [x] Створено `ToOne<T>` struct з lazy loading та internal state (`Cell`)
- [x] Створено `ToMany<T>` struct з change tracking та `RefCell`
- [x] Додано `ModelRelation` struct для ToMany в `model_json.rs`
- [x] Макроси парсять `ToOne<T>` → relation property (`customerId`, type 11, flags INDEXED | INDEX_PARTIAL_SKIP_ZERO)
- [x] Макроси парсять `ToMany<T>` → standalone relation
- [x] Генерація коду для серіалізації ToOne (target ID через FlatBuffers)
- [x] Генерація `.property_relation()` для ToOne в model builder
- [x] Генерація `.relation()` для ToMany в model builder
- [x] Підтримка `lastRelationId` у model.json
- [x] Expose relation API в `Box`: `rel_put`, `rel_remove`, `rel_get_ids`, `get_backlink_ids`, `rel_get_backlink_ids`
- [x] Приклади: `Order` з `customer: ToOne<Customer>`, `Student` з `teachers: ToMany<Teacher>`
- [x] Успішна компіляція та запуск example project

### 2. Option<T> Support - Завершити тестування ✅ DONE
**Статус**: Реалізація та тестування завершені (2026-02-06)  
**Файли**: `example/tests/optional_fields.rs`, `example/src/entities.rs`, `generator/src/code_gen.rs`

**Виконано**:
- [x] Тестова сутність `EntityWithOptionals` з 5 Optional полями різних типів
- [x] 14 тестів у `example/tests/optional_fields.rs`:
  - [x] Збереження None значень (`test_save_entity_with_all_none`)
  - [x] Збереження Some значень (`test_save_entity_with_all_some`)
  - [x] Читання None значень (`test_read_none_values`)
  - [x] Читання Some значень (`test_read_some_values`)
  - [x] Оновлення Some → None (`test_update_some_to_none`)
  - [x] Оновлення None → Some (`test_update_none_to_some`)
  - [x] Змішані Some/None (`test_mixed_some_and_none`)
  - [x] put_many з optional полями (`test_put_many_with_optionals`)
  - [x] get_all з optional полями (`test_get_all_with_optionals`)
  - [x] Query: is_null / is_not_null (`test_query_is_null_and_is_not_null`)
  - [x] Query: eq/ne на Optional String (`test_query_eq_on_optional_string`)
  - [x] Query: порівняння на Optional i32 (`test_query_comparison_on_optional_i32`)
  - [x] Edge case: порожній рядок vs None (`test_empty_string_vs_none`)
  - [x] Edge case: Some(0) vs None для числових типів (`test_zero_vs_none_for_optional_numeric`)

**Виправлені баги під час тестування**:
- FlatBuffers: `create_string` викликався всередині table construction для Optional String → виправлено (використовується pre-created offset)
- Optional числові типи: `Some(0)` зчитувався як `None` → виправлено (`push_slot_always` замість `push_slot`)

---

## 🟠 ВИСОКИЙ ПРІОРИТЕТ

### 3. Query String Operations - Виправлення багів
**Статус**: Критичні баги в string операціях  
**Файл**: `example/tests/string_query_ops.rs:56-102`

**Завдання**:
- [ ] FIXME: Виправити обробку пробілів у string operations (рядок 56-57)
- [ ] FIXME: Виправити `contains()` операцію (рядок 64)
- [ ] FIXME: Виправити `starts_with()` операцію (рядок 70)
- [ ] FIXME: Виправити `ends_with()` операцію (рядок 73)
- [ ] Виправити логіку `greater()` - завжди повертає 3 елементи (рядки 80, 90, 94, 102)

**Критичність**: Ці операції базові для роботи з текстом, потребують негайного виправлення.

### 4. Async Operations
**Статус**: Частково реалізовано, не протестовано  
**Файл**: `src/async.rs:4-64`

**Завдання**:
- [ ] Завершити реалізацію `put5()` методу
- [ ] Додати тести для `from_box()`
- [ ] Додати тести для `remove_with_id()`
- [ ] Додати тести для `from_box_with_timeout()`
- [ ] Додати тести для `close()`
- [ ] Реалізувати automatic mode detection (PUT/INSERT/UPDATE)
- [ ] Реалізувати automatic ID generation для нових об'єктів
- [ ] Дослідити інтеграцію з Box для putAsync та putQueued

### 5. ID/UID Collision Detection
**Статус**: Не реалізовано  
**Файл**: `generator/src/lib.rs:13-15`

**Завдання**:
- [ ] Реалізувати collision detection для predefined ID/UID
- [ ] Підтримувати set структуру для відстеження існуючих ID/UID
- [ ] При колізії - інкрементувати/генерувати новий ID
- [ ] Додати тести для колізій

---

## 🟡 СЕРЕДНІЙ ПРІОРИТЕТ

### 6. Query Builder - Покращення
**Файл**: `src/query/builder.rs:8-231`, `src/query/query.rs:13-258`

**Завдання**:
- [ ] Додати error checking перед chaining (obx_qb_cond)
- [ ] Реалізувати compile-time визначення дозволених викликів залежно від типу property
- [ ] Додати `all!()` та `any!()` макроси для varargs
- [ ] Реалізувати підтримку `Option<*>` properties у `is_null()` та `not_null()`
- [ ] Додати передачу generic type через closure
- [ ] Реалізувати iterator trait для результатів
- [ ] Додати тести для query operations

### 7. Type Support - Розширення
**Файли**: `generator/src/model_json.rs:384-557`, `macros/src/property.rs:170`

**Завдання**:
- [ ] Додати підтримку інших типів у `as_struct_property_default()` (рядок 384)
- [ ] Додати підтримку інших типів у `encode_to_fb()` (рядок 444)
- [ ] Додати підтримку інших типів у `encode_flatten()` (рядок 472)
- [ ] Додати підтримку інших типів у `as_assigned_property()` (рядок 557)
- [ ] Дискусія: підтримка `Option<Primitive>` для всіх примітивних типів

### 8. Query Membership Operations
**Файл**: `example/tests/basic_query_ops.rs:75-120`

**Завдання**:
- [ ] Дослідити чому `not_member_of` не підтримується (рядок 75)
- [ ] Розділити `not_member_of` та `member_of` тести
- [ ] Вирішити проблему lifetime Vec для умов (можливо boxing?)
- [ ] String не підтримує `not_member_of` - додати пояснення або підтримку

---

## 🟢 НИЗЬКИЙ ПРІОРИТЕТ

### 9. Code Quality & Refactoring

**Завдання**:
- [ ] Додати visibility модифікатори для trait extensions (`generator/src/code_gen.rs:414`)
- [ ] Розділити `model_json.rs` на модулі: `json::{info, entity, property}` (рядок 16)
- [ ] Видалити `unwrap()` і додати proper error handling (`generator/src/model_json.rs:45`)
- [ ] Дослідити clear buffer + read slice замість копіювання (рядок 319-320)
- [ ] Перевірити роботу з 4-byte char (32 bits) у ObjectBox (рядок 326-327)
- [ ] Видалити невикористані imports у згенерованому коді (`macros/src/lib.rs:114`)

### 10. Testing & Validation

**Завдання**:
- [ ] Додати тест endianness (`generator/src/code_gen.rs:161`, `src/cursor.rs:229`)
- [ ] Написати тест для Entity без properties (`macros/src/entity.rs:82`)
- [ ] Додати тест для 4-byte char підтримки (`generator/src/model_json.rs:327`)
- [ ] Перевірити безпеку Arc clone (`src/store.rs:100`)
- [ ] Додати тест для obx_store без Copy/Clone (`src/store.rs:105-114`)
- [ ] Додати тест для Transaction (`src/txn.rs:27-60`)
- [ ] Додати тест для Model::from_bytes (`src/model.rs:28`)
- [ ] Перевірити коректність Box operations (`src/box.rs:191`)

### 11. Feature Improvements

**Завдання**:
- [ ] Реалізувати #[transient] attribute для полів (`macros/src/entity.rs:44-54`)
- [ ] Додати перевірку pub keyword для entities (`macros/src/property.rs:85-87`)
- [ ] Додати safety precaution measures для властивостей (рядок 94-95)
- [ ] Реалізувати flags згідно ObjectBox Dart (`macros/src/property.rs:9`)
- [ ] Додати перевірку неприпустимих комбінацій атрибутів (рядок 89)

### 12. Documentation & Cleanup

**Завдання**:
- [ ] Перевірити чи model та opt правильно cleanup у Store (`src/store.rs:19-20`)
- [ ] Визначити чи потрібен Tx для `is_empty()` (`src/cursor.rs:228`)
- [ ] Дослідити чи потрібен Admin HTTP у debug mode (`src/store.rs:35`)
- [ ] Перевірити memory leak у Transaction (`src/txn.rs:27`)
- [ ] Перевірити коректність на всіх платформах (`src/util.rs:17`)

---

## 📚 ТЕХНІЧНИЙ БОРГ

### 13. Архітектурні покращення

**Файли**: `README.md:80-82`, `macros/src/lib.rs:120-121`

**Завдання**:
- [ ] Переписати macros з використанням [darling](https://github.com/TedDriggs/darling)
- [ ] Інтегрувати [cleaner abstractions](https://github.com/Buggaboo/lean_buffer)
- [ ] Додати підтримку параметрів id/uid для entity macro
- [ ] Додати перевірку конфліктів атрибутів (`macros/src/entity.rs:9`)
- [ ] Перевірити як працюють generics з entity macro (рядок 7)

### 14. Retired IDs Management

**Файл**: `generator/src/model_json.rs:71-78`

**Завдання**:
- [ ] Імплементувати `lastRelationId` tracking
- [ ] Імплементувати `lastSequenceId` tracking
- [ ] Підтримка `retiredEntityUids` array
- [ ] Підтримка `retiredIndexUids` array
- [ ] Підтримка `retiredPropertyUids` array
- [ ] Підтримка `retiredRelationUids` array

### 15. Store Integration

**Файл**: `generator/src/code_gen.rs:348`

**Завдання**:
- [ ] Додати використання Store для relations
- [ ] Імплементувати cascade операції
- [ ] Додати reference integrity checks

---

## 📋 СТРУКТУРА РОБОТИ ДЛЯ АГЕНТІВ

### Рекомендований порядок виконання:

1. **Спочатку** → Завершити тестування Option<T> (#2)
2. **Потім** → Виправити String Query баги (#3) - блокує базовий функціонал
3. **Після** → Реалізувати Relations (#1) - найбільша feature
4. **Паралельно** → Async operations (#4) та ID collision (#5)
5. **Наостанок** → Покращення, рефакторинг, тех борг

### Оцінка складності:

- ✅ **Relations**: DONE (2026-02-06)
- ✅ **Option<T> Tests**: DONE (2026-02-06)
- 🟠 **String Query Fixes**: 2-3 дні (потребує debugging)
- 🟠 **Async Operations**: 3-4 дні
- 🟠 **ID Collision**: 1-2 дні
- 🟡 **Query Builder**: 3-4 дні
- 🟡 **Type Support**: 2-3 дні
- 🟢 **Refactoring**: ongoing

---

## 📝 ПРИМІТКИ

### Залежності між завданнями:
- ✅ **Relations** - реалізовано (ToOne, ToMany, rel_put/rel_remove/rel_get_ids API)
- **Backlinks** залежать від Relations (тепер можна реалізувати)
- **Option<T>** має бути протестований (nullable foreign keys)
- **Async** потребує стабільної роботи базових Box operations

### Технологічний стек:
- Rust Edition 2021 (без nightly features)
- ObjectBox C API (objectbox.h)
- FlatBuffers для серіалізації
- Genco для code generation

### Корисні посилання:
- ObjectBox Dart generator: `/Users/andrii/Projects/objectbox-dart`
- Dart entity_resolver: для прикладів relations та nullable properties
- Cleaner abstractions: https://github.com/Buggaboo/lean_buffer

---

## 🆕 ДОДАТКОВІ КРИТИЧНІ FEATURES (відсутні в Rust, є в Dart)

Ці features присутні в ObjectBox Dart, але **повністю відсутні** в Rust реалізації:

### 16. 🧠 Vector Search (HNSW Index) - AI/ML підтримка
**Статус**: ❌ Не реалізовано  
**Пріоритет**: 🔴 КРИТИЧНИЙ (для конкурентоспроможності)  
**Референс**: ObjectBox Dart - перша on-device vector база даних

**Dart має**:
```dart
@Entity()
class Document {
  @Id()
  int id;
  
  @HnswIndex(
    dimensions: 384,
    neighborsPerNode: 30,
    indexingSearchCount: 100,
    distanceType: VectorDistanceType.cosine
  )
  @Property(type: PropertyType.floatVector)
  List<double>? embedding;
}

// Використання
final query = box.query().nearestNeighbors(
  embedding, 
  maxResults: 10
).build();
```

**Що потрібно для Rust**:
- [ ] Додати `@HnswIndex` annotation в `macros/src/property.rs`
- [ ] Підтримка `VectorDistanceType` (euclidean, cosine, dotProduct, geo)
- [ ] Реалізувати `HnswFlags` configuration
- [ ] Додати `nearestNeighbors()` метод у Query Builder
- [ ] C bindings для `obx_query_nearest_neighbors()`
- [ ] Підтримка `Float32List` / `Vec<f32>` для векторів
- [ ] Документація та приклади використання

**Use cases**:
- 🤖 RAG (Retrieval Augmented Generation) AI
- 🔍 Semantic search
- 📸 Image similarity
- 🎵 Audio fingerprinting
- 📊 Recommendation systems

**Оцінка складності**: 7-10 днів (потребує глибокої інтеграції з C API)

---

### 17. 👁️ Observable Queries (Watch / Streams)
**Статус**: ❌ Не реалізовано  
**Пріоритет**: 🟠 ВИСОКИЙ (для reactive UI)  
**Референс**: `objectbox-dart/objectbox/lib/src/native/query/builder.dart:52-79`

**Dart має**:
```dart
// Reactive stream - автоматично оновлюється при змінах
final Stream<List<Person>> peopleStream = box
    .query()
    .watch(triggerImmediately: true)
    .map((query) => query.find());

// UI автоматично оновлюється
StreamBuilder<List<Person>>(
  stream: peopleStream,
  builder: (context, snapshot) => ListView(...)
);
```

**Що потрібно для Rust**:
- [ ] Реалізувати `watch()` метод у `QueryBuilder`
- [ ] Підтримка async Streams (Tokio / async-std)
- [ ] Observer pattern для entity changes
- [ ] C bindings: `obx_observe()`, `obx_observe_single_type()`
- [ ] Thread-safe notification mechanism
- [ ] Документація з прикладами для UI frameworks (egui, iced)

**Use cases**:
- 📱 Reactive UI (Flutter-like в Rust)
- 🔄 Real-time data synchronization
- 📊 Live dashboards
- 🎮 Game state updates

**Оцінка складності**: 4-5 днів

---

### 18. 🔗 Backlink Relations (Двосторонні зв'язки)
**Статус**: ❌ Не реалізовано (базові Relations ✅ готові)  
**Пріоритет**: 🟡 СЕРЕДНІЙ (після ToOne/ToMany)  
**Референс**: `objectbox-dart/objectbox/lib/src/annotations.dart:319-365`

**Dart має**:
```dart
@Entity()
class Order {
  final customer = ToOne<Customer>();
}

@Entity()
class Customer {
  @Backlink('customer')  // ← Автоматичний reverse relation
  final orders = ToMany<Order>();
}
```

**Що потрібно для Rust**:
- [ ] `#[backlink("field_name")]` attribute macro
- [ ] Автоматична генерація reverse relations
- [ ] "Updatable view" - зміни відображаються в обох напрямках
- [ ] Не зберігає додаткові дані (тільки view)

**Залежить від**: ✅ Реалізації базових ToOne/ToMany (#1) - DONE

**Оцінка складності**: 2-3 дні (після Relations)

---

### 19. 🎯 Advanced Property Types
**Статус**: ⚠️ Частково реалізовано  
**Пріоритет**: 🟡 СЕРЕДНІЙ

**Відсутні типи в Rust**:

| Тип | Dart | Rust | Важливість |
|-----|------|------|------------|
| `DateTime` | ✅ 4 варіанти (date, dateNano, dateUtc, dateNanoUtc) | ❌ | 🔴 Критично |
| `List<int>` vectors | ✅ (byteVector, shortVector, intVector, charVector) | ⚠️ Частково | 🟠 Високо |
| `List<double>` | ✅ floatVector | ❌ | 🟠 Високо |
| `List<String>` | ✅ stringVector | ✅ | ✅ |
| `Uint8List` / typed arrays | ✅ Ефективні типізовані масиви | ❌ | 🟠 Високо |
| Flex (dynamic JSON-like) | ✅ FlexBuffer | ❌ | 🟡 Середньо |
| UUID | ✅ (uuid, uuidV4, uuidString) | ❌ | 🟡 Середньо |
| MongoDB types | ✅ (ObjectId, BSON, etc.) | ❌ | 🟢 Низько |

**Завдання**:
- [ ] Додати підтримку `DateTime` / `chrono::DateTime`
- [ ] Реалізувати typed vector properties (`Vec<u8>`, `Vec<i16>`, `Vec<f32>`)
- [ ] Додати FlexBuffer підтримку для dynamic types
- [ ] UUID type з `uuid` crate
- [ ] External types annotation

**Оцінка складності**: 5-6 днів

---

### 20. 🔐 Advanced Index Types
**Статус**: ⚠️ Базова підтримка є  
**Пріоритет**: 🟡 СЕРЕДНІЙ

**Dart має**:
```dart
@Entity()
class Person {
  @Index(type: IndexType.hash)    // 32-bit hash (default для String)
  String? email;
  
  @Index(type: IndexType.hash64)  // 64-bit hash
  String? username;
  
  @Index(type: IndexType.value)   // Value index (для "starts with")
  String? name;
}
```

**Rust має тільки**:
- ✅ `#[index]` (базовий, без типу)
- ❌ Hash indexes
- ❌ Value vs Hash розрізнення

**Завдання**:
- [ ] Додати `#[index(type = "hash")]` / `#[index(type = "value")]`
- [ ] Оптимізація query планування на основі типу індексу
- [ ] Документація коли використовувати який тип

**Оцінка складності**: 2-3 дні

---

### 21. 🛡️ Unique Constraints з Conflict Resolution
**Статус**: ⚠️ `#[unique]` є, але без ConflictStrategy  
**Пріоритет**: 🟡 СЕРЕДНІЙ

**Dart має**:
```dart
@Entity()
class User {
  @Unique(onConflict: ConflictStrategy.replace)  // Замінити існуючий
  String email;
  
  @Unique(onConflict: ConflictStrategy.fail)     // Викинути помилку (default)
  String username;
}
```

**Rust має**:
- ✅ `#[unique]` базовий
- ❌ Conflict strategies

**Завдання**:
- [ ] Додати `#[unique(on_conflict = "replace")]` / `"fail"`
- [ ] Реалізувати `ConflictStrategy` enum
- [ ] Генерація коду для handling conflicts
- [ ] Тести для різних strategies

**Оцінка складності**: 2 дні

---

### 22. 🎛️ Admin UI & Debug Tools
**Статус**: ⚠️ TODO коментар у `src/store.rs:35`  
**Пріоритет**: 🟢 НИЗЬКИЙ (але корисно для розробки)

**Dart має**:
- ✅ Admin HTTP server для debug
- ✅ Data browser
- ✅ Query testing
- ✅ Performance monitoring

**Rust має**:
```rust
// src/store.rs:35
// TODO Bonus: start admin http in debug from store?
```

**Завдання**:
- [ ] Додати `Store::start_admin_server(port)` для debug builds
- [ ] C bindings: `obx_admin_start()`, `obx_admin_opt_*`
- [ ] Web UI (може бути shared з Dart?)
- [ ] Performance metrics endpoint
- [ ] Query execution plan visualization

**Оцінка складності**: 3-4 дні

---

### 23. 🔄 External Type Mapping
**Статус**: ❌ Не реалізовано  
**Пріоритет**: 🟢 НИЗЬКИЙ (для cross-platform sync)

**Dart має**:
```dart
@Entity()
class Document {
  @ExternalType(type: ExternalPropertyType.uuid)
  List<int>? id;
  
  @ExternalName(name: "_id")
  @ExternalType(type: ExternalPropertyType.mongoId)
  List<int>? mongoId;
}
```

**Використання**: ObjectBox Sync з MongoDB, PostgreSQL, тощо

**Завдання**:
- [ ] `#[external_type(uuid)]` attribute
- [ ] `#[external_name("_id")]` для mapping
- [ ] Підтримка external property types enum
- [ ] Конвертери між Rust і external типами

**Оцінка складності**: 3-4 дні

---

## 📊 ПОРІВНЯЛЬНА ТАБЛИЦЯ: Rust vs Dart

| Feature | Dart ObjectBox | Rust ObjectBox | Gap Size |
|---------|----------------|----------------|----------|
| **Core Features** |
| Entities & Properties | ✅ | ✅ | - |
| CRUD Operations | ✅ | ✅ | - |
| Queries | ✅ | ✅ | - |
| Indexes | ✅ | ⚠️ Basic | 🟡 Medium |
| Unique constraints | ✅ Full | ⚠️ Basic | 🟡 Medium |
| **Relations** |
| ToOne | ✅ | ✅ (new!) | ✅ |
| ToMany | ✅ | ✅ (new!) | ✅ |
| Backlinks | ✅ | ❌ | 🟡 Medium |
| Lazy loading | ✅ | ✅ (new!) | ✅ |
| **Advanced Features** |
| Vector Search (HNSW) | ✅ | ❌ | 🔴 Critical |
| Observable Queries | ✅ | ❌ | 🟠 High |
| Async Operations | ✅ | ⚠️ Partial | 🟠 High |
| Transactions | ✅ | ✅ | - |
| **Data Types** |
| Nullable fields | ✅ | ✅ (new!) | ✅ |
| DateTime | ✅ 4 types | ❌ | 🔴 Critical |
| Typed vectors | ✅ | ⚠️ Partial | 🟠 High |
| FlexBuffers | ✅ | ❌ | 🟡 Medium |
| UUID | ✅ | ❌ | 🟡 Medium |
| **Developer Experience** |
| Code generation | ✅ | ✅ | - |
| Admin UI | ✅ | ❌ | 🟢 Nice-to-have |
| Documentation | ✅ Rich | ⚠️ Basic | 🟡 Medium |
| Examples | ✅ Many | ⚠️ Few | 🟡 Medium |
| **Platform Support** |
| Mobile | ✅ | ✅ | - |
| Desktop | ✅ | ✅ | - |
| Web | ✅ | ❌ | 🟡 Medium |
| **Sync** |
| ObjectBox Sync | ✅ | ❓ Unknown | ❓ |

---

## 🎯 НОВІ РЕКОМЕНДОВАНІ ПРІОРИТЕТИ (з урахуванням Dart)

### Фаза 1: Foundation (4-6 тижнів)
1. ✅ **Option<T>** - тестування DONE (2026-02-06)
2. ✅ **Relations** (ToOne/ToMany) - DONE (2026-02-06)
3. 🔴 **DateTime support** - 2-3 дні
4. 🟠 **String Query fixes** - 2-3 дні
5. 🟠 **Typed vectors** (Vec<f32>, Vec<i16>) - 3-4 дні

### Фаза 2: Advanced Features (3-4 тижні)
6. 🔴 **Vector Search (HNSW)** - 7-10 днів ⭐ **Killer feature**
7. 🟠 **Observable Queries** - 4-5 днів
8. 🟠 **Async Operations** - 3-4 дні
9. 🟡 **Backlinks** - 2-3 дні
10. 🟡 **Advanced Index Types** - 2-3 дні

### Фаза 3: Polish & Ecosystem (2-3 тижні)
11. 🟡 **Unique with ConflictStrategy** - 2 дні
12. 🟡 **FlexBuffer support** - 3-4 дні
13. 🟡 **UUID types** - 2 дні
14. 🟢 **Admin UI** - 3-4 дні
15. 🟢 **Documentation & Examples** - ongoing

---

## 💡 РЕКОМЕНДАЦІЇ ДЛЯ АГЕНТІВ

### Чому Vector Search критично важливий?

1. **Конкурентна перевага**: ObjectBox Dart - перша on-device vector база
2. **AI/ML тренд**: RAG, semantic search, embeddings - must-have для 2026
3. **Use cases**: 
   - 🤖 Local AI assistants
   - 📚 Document search в embedded системах
   - 🎯 Recommendation engines без cloud
   - 🔍 Similarity search для IoT

### Пріоритизація:
- **Якщо фокус на Mobile/IoT AI** → Vector Search на 1 місце
- **Якщо фокус на Web/Desktop apps** → Relations + Observable Queries
- **Якщо фокус на Enterprise** → Transactions + Sync + Admin UI

### Dart як референс:
- ✅ Використовуйте Dart код для розуміння архітектури
- ✅ Копіюйте API design (але адаптуйте до Rust idioms)
- ✅ Тести з Dart можна портувати в Rust
- ⚠️ Не копіюйте сліпо - Rust має інші можливості (zero-cost, lifetimes)

### Rust переваги над Dart:
- 🚀 Zero-copy access (використати FlatBuffers ефективніше)
- 🔒 Memory safety без GC
- ⚡ SIMD можливості для vector operations
- 🎯 Embedded systems підтримка

---

## 🔗 КОРИСНІ ПОСИЛАННЯ

### Dart Codebase (для референсу):
- **Annotations**: `/Users/andrii/Projects/objectbox-dart/objectbox/lib/src/annotations.dart`
- **Relations**: `/Users/andrii/Projects/objectbox-dart/objectbox/lib/src/relations/`
- **Query Builder**: `/Users/andrii/Projects/objectbox-dart/objectbox/lib/src/native/query/builder.dart`
- **Examples**: `/Users/andrii/Projects/objectbox-dart/objectbox/example/`

### ObjectBox Resources:
- 📚 [ObjectBox Docs](https://docs.objectbox.io)
- 🎥 [Vector Search Intro](https://objectbox.io/vector-database/)
- 🔧 [C API Reference](https://github.com/objectbox/objectbox-c)
- 🦀 [Rust Best Practices](https://rust-unofficial.github.io/patterns/)

### AI/ML Resources (для Vector Search):
- 🧠 [HNSW Algorithm](https://arxiv.org/abs/1603.09320)
- 📊 [Vector Embeddings Guide](https://www.pinecone.io/learn/vector-embeddings/)
- 🔍 [Semantic Search](https://www.sbert.net/)

---

*Документ оновлено з урахуванням аналізу ObjectBox Dart реалізації.*  
*Останнє оновлення: 2026-02-06 (Relations #1, Option<T> Tests #2 marked as DONE)*
