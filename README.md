#  Space Invaders - Rust Game


================================


- **Rust** (เวอร์ชัน 1.56+)
- **Cargo** (มาพร้อม Rust)



**Clone repository หรือสร้างโปรเจกต์ใหม่:**
   ```bash
   cargo new space_invaders
   cd space_invaders
   ```

**เพิ่ม dependencies ใน `Cargo.toml`:**
   ```toml
   [package]
   name = "space_invaders"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   macroquad = "0.4"
   rand = "0.8"
   ```


**รันเกม:**
   ```bash
   cargo run
   ```

## 🎮 วิธีเล่น

### การควบคุม

| ปุ่ม | การกระทำ |
|------|----------|
| `←` หรือ `A` | เคลื่อนที่ซ้าย |
| `→` หรือ `D` | เคลื่อนที่ขวา |
| `SPACE` | ยิงกระสุน |
| `R` | เริ่มเกมใหม่ |




### โครงสร้างหลัก

```rust
// โครงสร้างข้อมูลหลัก
struct GameState {
    player: Player,          // ข้อมูลผู้เล่น
    enemies: Vec<Enemy>,     // รายการศัตรู
    bullets: Vec<Bullet>,    // รายการกระสุน
    particles: Vec<Particle>, // เอฟเฟกต์ระเบิด
    // ... อื่นๆ
}

// ประเภทศัตรู
enum EnemyType {
    Basic,   // ศัตรูพื้นฐาน
    Fast,    // ศัตรูเร็ว
    Strong,  // ศัตรูแข็งแกร่ง
}
```


- `src/main.rs`
- `Cargo.toml` - การตั้งค่า dependencies



```rust
// ความเร็วผู้เล่น
player.speed = 300.0;

// ความเร็วกระสุน
velocity: Vec2::new(0.0, -500.0);

// คะแนนศัตรู
EnemyType::Basic => 10,
EnemyType::Fast => 20,
EnemyType::Strong => 30,
```


