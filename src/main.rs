// Cargo.toml
// [package]
// name = "space_invaders"
// version = "0.1.0"
// edition = "2021"
//
// [dependencies]
// macroquad = "0.4"
// rand = "0.8"

use macroquad::prelude::*;

// ข้อมูลผู้เล่น
#[derive(Clone)]
struct Player {
    position: Vec2,
    size: Vec2,
    speed: f32,
    color: Color,
}

// ข้อมูลศัตรู
#[derive(Clone)]
struct Enemy {
    position: Vec2,
    size: Vec2,
    speed: f32,
    color: Color,
    alive: bool,
    enemy_type: EnemyType,
}

#[derive(Clone)]
enum EnemyType {
    Basic,
    Fast,
    Strong,
}

// ข้อมูลกระสุน
#[derive(Clone)]
struct Bullet {
    position: Vec2,
    velocity: Vec2,
    size: Vec2,
    color: Color,
    is_player_bullet: bool,
}

// ข้อมูลอนุภาค (สำหรับเอฟเฟกต์)
#[derive(Clone)]
struct Particle {
    position: Vec2,
    velocity: Vec2,
    color: Color,
    lifetime: f32,
    max_lifetime: f32,
}

// สถานะเกม
struct GameState {
    player: Player,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    particles: Vec<Particle>,
    score: i32,
    lives: i32,
    game_over: bool,
    wave: i32,
    enemy_direction: f32,
    enemy_shoot_timer: f32,
}

impl GameState {
    fn new() -> Self {
        let mut game_state = GameState {
            player: Player {
                position: Vec2::new(400.0, 550.0),
                size: Vec2::new(50.0, 30.0),
                speed: 300.0,
                color: GREEN,
            },
            enemies: Vec::new(),
            bullets: Vec::new(),
            particles: Vec::new(),
            score: 0,
            lives: 3,
            game_over: false,
            wave: 1,
            enemy_direction: 1.0,
            enemy_shoot_timer: 0.0,
        };
        
        game_state.spawn_enemies();
        game_state
    }
    
    fn spawn_enemies(&mut self) {
        self.enemies.clear();
        
        // สร้างศัตรูหลายแถว
        for row in 0..5 {
            for col in 0..10 {
                let enemy_type = match row {
                    0..=1 => EnemyType::Strong,
                    2..=3 => EnemyType::Fast,
                    _ => EnemyType::Basic,
                };
                
                let color = match enemy_type {
                    EnemyType::Strong => RED,
                    EnemyType::Fast => YELLOW,
                    EnemyType::Basic => BLUE,
                };
                
                self.enemies.push(Enemy {
                    position: Vec2::new(
                        50.0 + col as f32 * 70.0,
                        50.0 + row as f32 * 50.0,
                    ),
                    size: Vec2::new(40.0, 30.0),
                    speed: 50.0 + self.wave as f32 * 10.0,
                    color,
                    alive: true,
                    enemy_type,
                });
            }
        }
    }
    
    fn update(&mut self, dt: f32) {
        if self.game_over {
            return;
        }
        
        self.update_player(dt);
        self.update_bullets(dt);
        self.update_enemies(dt);
        self.update_particles(dt);
        self.check_collisions();
        self.check_game_state();
    }
    
    fn update_player(&mut self, dt: f32) {
        // เคลื่อนที่ซ้าย-ขวา
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.player.position.x -= self.player.speed * dt;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.player.position.x += self.player.speed * dt;
        }
        
        // จำกัดไม่ให้ออกนอกหน้าจอ
        self.player.position.x = self.player.position.x.clamp(0.0, screen_width() - self.player.size.x);
        
        // ยิงกระสุน
        if is_key_pressed(KeyCode::Space) {
            self.player_shoot();
        }
    }
    
    fn player_shoot(&mut self) {
        self.bullets.push(Bullet {
            position: Vec2::new(
                self.player.position.x + self.player.size.x / 2.0 - 2.0,
                self.player.position.y,
            ),
            velocity: Vec2::new(0.0, -500.0),
            size: Vec2::new(4.0, 10.0),
            color: WHITE,
            is_player_bullet: true,
        });
    }
    
    fn update_bullets(&mut self, dt: f32) {
        // อัพเดทตำแหน่งกระสุน
        for bullet in &mut self.bullets {
            bullet.position += bullet.velocity * dt;
        }
        
        // ลบกระสุนที่ออกนอกหน้าจอ
        self.bullets.retain(|bullet| {
            bullet.position.y > -10.0 && bullet.position.y < screen_height() + 10.0
        });
    }
    
    fn update_enemies(&mut self, dt: f32) {
        let mut move_down = false;
        
        // ตรวจสอบว่าศัตรูชนขอบหน้าจอหรือไม่
        for enemy in &self.enemies {
            if !enemy.alive { continue; }
            
            if (enemy.position.x <= 0.0 && self.enemy_direction < 0.0) ||
               (enemy.position.x + enemy.size.x >= screen_width() && self.enemy_direction > 0.0) {
                move_down = true;
                break;
            }
        }
        
        // เคลื่อนที่ศัตรู
        for enemy in &mut self.enemies {
            if !enemy.alive { continue; }
            
            if move_down {
                enemy.position.y += 30.0;
            } else {
                enemy.position.x += enemy.speed * self.enemy_direction * dt;
            }
        }
        
        if move_down {
            self.enemy_direction *= -1.0;
        }
        
        // ศัตรูยิงกระสุน
        self.enemy_shoot_timer += dt;
        if self.enemy_shoot_timer > 2.0 {
            self.enemy_shoot();
            self.enemy_shoot_timer = 0.0;
        }
    }
    
    fn enemy_shoot(&mut self) {
        let alive_enemies: Vec<&Enemy> = self.enemies.iter()
            .filter(|e| e.alive)
            .collect();
            
        if !alive_enemies.is_empty() {
            let random_enemy = alive_enemies[rand::gen_range(0, alive_enemies.len())];
            
            self.bullets.push(Bullet {
                position: Vec2::new(
                    random_enemy.position.x + random_enemy.size.x / 2.0 - 2.0,
                    random_enemy.position.y + random_enemy.size.y,
                ),
                velocity: Vec2::new(0.0, 200.0),
                size: Vec2::new(4.0, 10.0),
                color: RED,
                is_player_bullet: false,
            });
        }
    }
    
    fn update_particles(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.position += particle.velocity * dt;
            particle.lifetime -= dt;
            particle.velocity *= 0.98; // ความต้านอากาศ
        }
        
        self.particles.retain(|p| p.lifetime > 0.0);
    }
    
    fn check_collisions(&mut self) {
        let mut bullets_to_remove = Vec::new();
        let mut enemies_hit = Vec::new();
        
        // กระสุนผู้เล่น vs ศัตรู
        for (bullet_idx, bullet) in self.bullets.iter().enumerate() {
            if !bullet.is_player_bullet { continue; }
            
            for (enemy_idx, enemy) in self.enemies.iter().enumerate() {
                if !enemy.alive { continue; }
                
                if self.rectangles_overlap(
                    bullet.position, bullet.size,
                    enemy.position, enemy.size
                ) {
                    bullets_to_remove.push(bullet_idx);
                    enemies_hit.push((enemy_idx, enemy.position + enemy.size / 2.0, enemy.color, enemy.enemy_type.clone()));
                    break;
                }
            }
        }
        
        // กระสุนศัตรู vs ผู้เล่น
        for (bullet_idx, bullet) in self.bullets.iter().enumerate() {
            if bullet.is_player_bullet { continue; }
            
            if self.rectangles_overlap(
                bullet.position, bullet.size,
                self.player.position, self.player.size
            ) {
                bullets_to_remove.push(bullet_idx);
                self.lives -= 1;
                self.create_explosion(self.player.position + self.player.size / 2.0, WHITE);
                break;
            }
        }
        
        // ประมวลผลการชน
        for (enemy_idx, explosion_pos, explosion_color, enemy_type) in enemies_hit {
            if enemy_idx < self.enemies.len() {
                self.enemies[enemy_idx].alive = false;
                
                // เพิ่มคะแนน
                self.score += match enemy_type {
                    EnemyType::Basic => 10,
                    EnemyType::Fast => 20,
                    EnemyType::Strong => 30,
                };
                
                // สร้างเอฟเฟกต์ระเบิด
                self.create_explosion(explosion_pos, explosion_color);
            }
        }
        
        // ลบกระสุนที่ชน
        bullets_to_remove.sort_by(|a, b| b.cmp(a));
        for idx in bullets_to_remove {
            if idx < self.bullets.len() {
                self.bullets.remove(idx);
            }
        }
    }
    
    fn rectangles_overlap(&self, pos1: Vec2, size1: Vec2, pos2: Vec2, size2: Vec2) -> bool {
        pos1.x < pos2.x + size2.x &&
        pos1.x + size1.x > pos2.x &&
        pos1.y < pos2.y + size2.y &&
        pos1.y + size1.y > pos2.y
    }
    
    fn create_explosion(&mut self, position: Vec2, base_color: Color) {
        for _ in 0..10 {
            let angle = rand::gen_range(0.0, std::f32::consts::TAU);
            let speed = rand::gen_range(50.0, 150.0);
            
            self.particles.push(Particle {
                position,
                velocity: Vec2::new(angle.cos() * speed, angle.sin() * speed),
                color: Color {
                    r: base_color.r + rand::gen_range(-0.2, 0.2),
                    g: base_color.g + rand::gen_range(-0.2, 0.2),
                    b: base_color.b + rand::gen_range(-0.2, 0.2),
                    a: 1.0,
                },
                lifetime: rand::gen_range(0.5, 1.5),
                max_lifetime: 1.0,
            });
        }
    }
    
    fn check_game_state(&mut self) {
        // ตรวจสอบว่าผู้เล่นตายหรือไม่
        if self.lives <= 0 {
            self.game_over = true;
        }
        
        // ตรวจสอบว่าศัตรูหมดหรือไม่
        let alive_enemies = self.enemies.iter().any(|e| e.alive);
        if !alive_enemies {
            self.wave += 1;
            self.spawn_enemies();
            self.score += 100; // โบนัสเวฟ
        }
        
        // ตรวจสอบว่าศัตรูลงมาถึงผู้เล่นหรือไม่
        for enemy in &self.enemies {
            if enemy.alive && enemy.position.y + enemy.size.y >= self.player.position.y {
                self.game_over = true;
                break;
            }
        }
    }
    
    fn draw(&self) {
        clear_background(BLACK);
        
        // วาดดาวพื้นหลัง
        for i in 0..100 {
            let x = (i as f32 * 71.3) % screen_width();
            let y = (i as f32 * 37.7) % screen_height();
            draw_circle(x, y, 1.0, WHITE);
        }
        
        if !self.game_over {
            // วาดผู้เล่น
            draw_rectangle(
                self.player.position.x,
                self.player.position.y,
                self.player.size.x,
                self.player.size.y,
                self.player.color,
            );
            
            // วาดปืนผู้เล่น
            draw_rectangle(
                self.player.position.x + self.player.size.x / 2.0 - 2.0,
                self.player.position.y - 10.0,
                4.0,
                10.0,
                WHITE,
            );
        }
        
        // วาดศัตรู
        for enemy in &self.enemies {
            if enemy.alive {
                draw_rectangle(
                    enemy.position.x,
                    enemy.position.y,
                    enemy.size.x,
                    enemy.size.y,
                    enemy.color,
                );
                
                // วาดตาศัตรู
                draw_circle(enemy.position.x + 10.0, enemy.position.y + 10.0, 3.0, WHITE);
                draw_circle(enemy.position.x + 30.0, enemy.position.y + 10.0, 3.0, WHITE);
            }
        }
        
        // วาดกระสุน
        for bullet in &self.bullets {
            draw_rectangle(
                bullet.position.x,
                bullet.position.y,
                bullet.size.x,
                bullet.size.y,
                bullet.color,
            );
        }
        
        // วาดอนุภาค
        for particle in &self.particles {
            let alpha = particle.lifetime / particle.max_lifetime;
            let color = Color {
                r: particle.color.r,
                g: particle.color.g,
                b: particle.color.b,
                a: alpha,
            };
            draw_circle(particle.position.x, particle.position.y, 2.0, color);
        }
        
        // วาดข้อมูลเกม
        draw_text(&format!("คะแนน: {}", self.score), 20.0, 30.0, 30.0, YELLOW);
        draw_text(&format!("ชีวิต: {}", self.lives), 20.0, 60.0, 30.0, GREEN);
        draw_text(&format!("เวฟ: {}", self.wave), 20.0, 90.0, 30.0, BLUE);
        
        // วาดคำแนะนำ
        draw_text("ใช้ ←→ เคลื่อนที่, SPACE ยิง, R เริ่มใหม่", 20.0, screen_height() - 20.0, 20.0, WHITE);
        
        if self.game_over {
            // หน้าจอ Game Over
            let text = "GAME OVER";
            let font_size = 60.0;
            let text_size = measure_text(text, None, font_size as u16, 1.0);
            
            draw_text(
                text,
                screen_width() / 2.0 - text_size.width / 2.0,
                screen_height() / 2.0 - 50.0,
                font_size,
                RED,
            );
            
            let score_text = &format!("คะแนนสุดท้าย: {}", self.score);
            let score_size = measure_text(score_text, None, 30, 1.0);
            draw_text(
                score_text,
                screen_width() / 2.0 - score_size.width / 2.0,
                screen_height() / 2.0,
                30.0,
                WHITE,
            );
            
            let restart_text = "กด R เพื่อเริ่มใหม่";
            let restart_size = measure_text(restart_text, None, 25, 1.0);
            draw_text(
                restart_text,
                screen_width() / 2.0 - restart_size.width / 2.0,
                screen_height() / 2.0 + 50.0,
                25.0,
                GREEN,
            );
        }
    }
}

#[macroquad::main("Space Invaders")]
async fn main() {
    let mut game_state = GameState::new();
    
    loop {
        let dt = get_frame_time();
        
        // ตรวจสอบปุ่มเริ่มใหม่
        if is_key_pressed(KeyCode::R) {
            game_state = GameState::new();
        }
        
        game_state.update(dt);
        game_state.draw();
        
        next_frame().await;
    }
}