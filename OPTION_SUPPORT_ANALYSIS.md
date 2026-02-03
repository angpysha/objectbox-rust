# Аналіз підтримки Option<T> на основі ObjectBox Dart

## Ключові висновки з Dart реалізації

### 1. **Не змінюють JSON модель!**
Dart версія **НЕ** додає нове поле `isOptional` до JSON. Замість цього:
- Використовують існуюче поле `dartFieldType` для зберігання інформації
- Додають `?` в кінці типу для nullable полів (наприклад: `"String?"`, `"int?"`)
- Мають getter `fieldIsNullable` який перевіряє наявність `?` в кінці

### 2. Структура ModelProperty в Dart

```dart
class ModelProperty {
  String? _dartFieldType;  // "String", "String?", "int", "int?" etc.
  
  // Getter що витягує інформацію з _dartFieldType
  bool get fieldIsNullable =>
      _dartFieldType!.substring(_dartFieldType!.length - 1) == '?';
      
  String get fieldType => _dartFieldType!
      .replaceFirst('?', '', _dartFieldType!.length - 1);
}
```

### 3. Як встановлюється dartFieldType

```dart
// entity_resolver.dart, line 378-379
prop.dartFieldType = 
    f.type.element!.displayName + (isNullable(f.type) ? '?' : '');
```

### 4. Використання в генерації коду

```dart
// code_chunks.dart
if (p.fieldIsNullable) {
  buf.write('.vTableGetNullable(buffer, rootOffset, $offset)');
} else {
  buf.write('.vTableGet(buffer, rootOffset, $offset, ${defaultValue})');
}

// Для серіалізації
if (p.fieldIsNullable || p.fieldType == 'dynamic') {
  assignment += '$fieldName == null ? null : ';
  fieldName += '!';
}
```

### 5. Прапорці ObjectBox

Dart **НЕ** використовує `OBXPropertyFlags_NON_PRIMITIVE_TYPE` для nullable примітивів!
Цей прапорець використовується тільки для:
- ToOne relations  
- Інших non-primitive типів

## Правильний підхід для Rust

### Зміни в структурах

1. **У `macros/src/property.rs`**: НЕ додавати поле `is_optional`

2. **У `generator/src/model_json.rs`**: 
   - Використовувати існуюче поле з Dart: **вже є в JSON!**
   - Добавити helper методи для роботи з nullable типами

### Алгоритм

1. **Розпізнавання Option<T>** (macros):
   ```rust
   // Якщо тип = Option<String>
   // Встановити dartFieldType = "String?"  (не "OptionString")
   ```

2. **Генерація коду**:
   ```rust
   fn is_nullable(dart_field_type: &str) -> bool {
       dart_field_type.ends_with('?')
   }
   
   fn base_type(dart_field_type: &str) -> &str {
       dart_field_type.trim_end_matches('?')
   }
   ```

3. **Серіалізація/Десеріалізація**:
   - Використовувати `is_nullable()` для визначення чи генерувати `if let Some()`
   - Базовий тип отримувати через `base_type()`

## Переваги цього підходу

✅ Сумісність з існуючою JSON моделлю  
✅ Не потрібно міграція  
✅ Працює між мовами (Dart, Kotlin, Swift, тощо)  
✅ Простіше в підтримці  

## Реалізація для Rust

### macros/src/property.rs

```rust
// Замість додавання is_optional поля:
let type_str = if is_optional {
    format!("{}?", inner_type)  // "String?" замість "OptionString"
} else {
    ident_joined
};

// Це піде в dartFieldType при серіалізації в JSON
```

### generator/src/model_json.rs

```rust
impl ModelProperty {
    pub(crate) fn is_nullable(&self) -> bool {
        // У Dart вони НЕ зберігають це як окреме поле
        // Вони перевіряють dartFieldType.endsWith('?')
        // Але у нас немає dartFieldType в Rust структурі!
        // Потрібно додати це поле
        false // TODO
    }
}
```

### Проблема: dartFieldType відсутнє в Rust

У Rust структурі `ModelProperty` немає поля `dartFieldType`! 
Це спеціфічне для Dart поле для зберігання Dart типів.

### Рішення

Потрібно додати **Rust-специфічне поле** для зберігання інформації про тип:
- Назва: `rust_field_type` 
- Значення: `"String"`, `"Option<String>"`, `"i32"`, `"Option<i32>"`, тощо.
- Це поле **НЕ** йде в JSON (skip_serializing)
- Використовується тільки для генерації коду

Або простіше: використати існуючу логіку з прапорцями, але **НЕ** змінювати JSON.

## Найкраще рішення

Використовувати підхід як у Dart, але адаптований для Rust:

1. **НЕ** змінювати JSON модель
2. Зберігати інформацію про Option у внутрішньому представленні (після парсингу JSON)
3. Додати transient поле (що не серіалізується) до ModelProperty
4. Визначати nullable з типів під час генерації

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelProperty {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: ob_consts::OBXPropertyType,
    // ... існуючі поля ...
    
    // НЕ серіалізується в JSON!
    #[serde(skip)]
    pub rust_type: Option<String>,  // "Option<String>", "i32", etc.
}
```

Потім в code_gen.rs:

```rust
fn is_optional(prop: &ModelProperty) -> bool {
    prop.rust_type.as_ref()
        .map(|t| t.starts_with("Option<"))
        .unwrap_or(false)
}
```

## Висновок

Ключова відмінність: Dart зберігає типи як строки в JSON (`dartFieldType`),  
Rust не має цього поля, тому нам потрібен інший спосіб передачі інформації  
між макросом та генератором БЕЗ зміни JSON формату.
