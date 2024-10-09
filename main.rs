// трейт персонажа в игре
trait Character {
    type WeaponType: Weapon;    // оружие, ассоциированое с персонажем
    // метод создания оружия, которое ассоциировано с персонажем
    fn create_weapon() -> Self::WeaponType;
}
 
// трейт оружия
trait Weapon {
    fn attack(&self);
}
 
// структура воина
struct Warrior;
// структура мага
struct Mage;
 
// структура, которая представляет меч
struct Sword;
// структура, которая представляет посох
struct Staff;
 
// реализация меча
impl Weapon for Sword {
    fn attack(&self) {
        println!("Атакуем мечом");
    }
}
 
// реализация для посоха
impl Weapon for Staff {
    fn attack(&self) {  
        println!("Применяем магию");
    }
}
 
// реализация воина
impl Character for Warrior { 
    type WeaponType = Sword;    // оружие - меч
    // создаем меч
    fn create_weapon() -> Self::WeaponType { Sword }
}
 
// реализация мага
impl Character for Mage {
    type WeaponType = Staff;    // оружие - магический посох
    // создаем посох
    fn create_weapon() -> Self::WeaponType { Staff }
}
 
// программирование на уровне типа
fn attack<C: Character>() {
    let weapon = C::create_weapon(); // создаем оружие ассоциированного типа
    weapon.attack();  // применяем оружие
}
 
fn main() {
    attack::<Warrior>();
    attack::<Mage>();
}