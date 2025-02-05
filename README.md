# Fast Parabolic SAR in Rust

### **Обзор**
Этот проект реализует **Parabolic SAR (Stop and Reverse) индикатор** на Rust, оптимизированный для обработки финансовых данных в реальном времени. Основное внимание уделяется **скорости**, чтобы SAR мог обрабатывать большие объемы данных максимально эффективно.

### **Ключевые особенности**
- **Оптимизированные O(1) обновления SAR** – храним только последние SAR, AF и EP.
- **Эффективное представление данных** – используем только **high** и **low** цены.
- **Бенчмаркинг через Criterion** – измеряем скорость обновления SAR.
- **Минимальное использование памяти** – нет необходимости хранить всю историю свечей.

---

### **Сравнение: [`tars`](https://docs.rs/crate/tars/latest) vs. эта реализация**

| Функция | Реализация `tars` | Эта реализация | Преимущество |
|---------|-------------------|----------------|-------------|
| **Хранимые данные** | Полный внутренний буфер | Только последние `SAR`, `AF`, `EP` | Меньше использования памяти |
| **AF (Acceleration Factor)** | 0.02 → 0.20 | Такой же | Без изменений |
| **OHLC входные данные** | Полные данные OHLC | Оптимизировано до **High/Low** | Более быстрые вычисления |
| **Метод вычисления** | Использует историю свечей | Только последние значения | Меньше памяти, O(1) обновления |
| **Фокус на производительность** | Универсальный | **Оптимизирован для скорости** | Быстрее для потоковых данных |

---

### **Запуск проекта**
#### **Запуск основной программы**
```sh
cargo run --release
```
Инициализирует Parabolic SAR с **50 свечами** и обрабатывает новую входящую свечу.

#### **Тестирование производительности**
```sh
cargo bench
```
Измеряет время выполнения обновления SAR.

# Benchmark Results

##  Тесты

### Cargo Bench Output

```sh
❯ cargo bench

   Compiling parabolic_sar v0.1.0 (/Users/L/Documents/dev/psar_test)
    Finished `bench` profile [optimized] target(s) in 1.61s
     Running unittests src/lib.rs (target/release/deps/parabolic_sar-45052d7cfcc6d3fe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/release/deps/parabolic_sar-55a621e85ac8e8bc)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

##  Производительность

###  Обновление SAR в 5 наносекунд

```sh
Running benches/sar_benchmark.rs (target/release/deps/sar_benchmark-769b55a65c140cad)
Gnuplot not found, using plotters backend
SAR Update              time:   [5.1336 ns 5.1457 ns 5.1588 ns]
                        change: [-0.7066% -0.3945% -0.0541%] (p = 0.02 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe
```
