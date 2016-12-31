use std::fmt;

enum UnitType { Infantry = 1, Mech = 2, Air = 3, Ship = 4 }

struct UnitData {
    power : u16,
    cost : u16,
    unitType : UnitType,
    name: String,
}

impl fmt::Display for UnitType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = match *self {
            UnitType::Infantry => "Infantry",
            UnitType::Mech => "Mech",
            UnitType::Air => "Air",
            UnitType::Ship => "Ship"
        };
        write!(f, "{}", res)
    }
}

impl fmt::Display for UnitData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unit (\"{}\"):\r\ntype: {}\r\npower: {},\r\ncost: {}", self.name, self.unitType, self.power, self.cost)
    }
}

struct Coordinates {
    x : u16,
    y : u16,
}

struct Unit {
    data: UnitData,
    durability : u8,
    location: Coordinates,
}

struct Player {
    units: Vec<Unit>
}



fn main() {
    let unitDefinition = [UnitData{power: 100, cost: 5000, unitType: UnitType::Infantry, name: String::from("Marine squad")}];
    println!("Unit1 [{}]", unitDefinition[0]);
}
