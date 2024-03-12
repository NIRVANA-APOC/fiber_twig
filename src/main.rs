use std::{io::{self, Write}, num::IntErrorKind};

// OFFSET
// 14 关卡
// 30 得分
// 50 锤子
// 67 茶壶
// 83 经验
// 167 盾牌
// 182 幸运草
// 197 生命值
const DATA_TEMPLATE: [u8; 246] = [
        0x76, 0xA8, 0xA5, 0xA5, 0x98, 0xA1, 0xA7, 0x7F, 0x98, 0xA9, 0x98, 0x9F, 0x70, 0x55, 0x63,
        0x55, 0x0D, 0x0A, 0x87, 0xA2, 0xA7, 0x94, 0x9F, 0x86, 0x96, 0xA2, 0xA5, 0x98, 0x70, 0x55,
        0x63, 0x55, 0x0D, 0x0A, 0x87, 0xA2, 0xA7, 0x94, 0x9F, 0x7B, 0x94, 0xA0, 0xA0, 0x98, 0xA5,
        0x81, 0xA8, 0xA0, 0x70, 0x55, 0x63, 0x55, 0x0D, 0x0A, 0x87, 0xA2, 0xA7, 0x94, 0x9F, 0x76,
        0x94, 0xA3, 0x81, 0xA8, 0xA0, 0x70, 0x55, 0x63, 0x55, 0x0D, 0x0A, 0x78, 0xAB, 0xA3, 0x98,
        0xA5, 0x9C, 0x98, 0xA1, 0x96, 0x98, 0x70, 0x55, 0x63, 0x55, 0x0D, 0x0A, 0x83, 0x9F, 0xA8,
        0xA6, 0x82, 0xA7, 0x95, 0x70, 0x55, 0x63, 0x55, 0x0D, 0x0A, 0x83, 0x9F, 0xA8, 0xA6, 0x7F,
        0xA8, 0x96, 0x9E, 0x70, 0x55, 0x63, 0x55, 0x0D, 0x0A, 0x83, 0x9F, 0xA8, 0xA6, 0x76, 0x94,
        0xA3, 0x70, 0x55, 0x63, 0x55, 0x0D, 0x0A, 0x83, 0x9F, 0xA8, 0xA6, 0x7F, 0x9C, 0x99, 0x98,
        0x70, 0x55, 0x63, 0x55, 0x0D, 0x0A, 0x83, 0x9F, 0xA8, 0xA6, 0x7B, 0x94, 0xA0, 0xA0, 0x98,
        0xA5, 0x70, 0x55, 0x63, 0x55, 0x0D, 0x0A, 0x86, 0x9E, 0x9C, 0x9F, 0x9F, 0x82, 0xA7, 0x95,
        0x70, 0x55, 0x63, 0x55, 0x0D, 0x0A, 0x86, 0x9E, 0x9C, 0x9F, 0x9F, 0x7F, 0xA8, 0x96, 0x9E,
        0x70, 0x55, 0x63, 0x55, 0x0D, 0x0A, 0x86, 0x9E, 0x9C, 0x9F, 0x9F, 0x7F, 0x9C, 0x99, 0x98,
        0x70, 0x55, 0x63, 0x55, 0x0D, 0x0A, 0x81, 0xA2, 0x7F, 0xA2, 0xA2, 0xA6, 0x98, 0x70, 0x55,
        0x99, 0x94, 0x9F, 0xA6, 0x98, 0x55, 0x0D, 0x0A, 0x78, 0x94, 0xA6, 0xAC, 0x70, 0x55, 0x99,
        0x94, 0x9F, 0xA6, 0x98, 0x55, 0x0D, 0x0A, 0x7B, 0x94, 0xA5, 0x97, 0x70, 0x55, 0x99, 0x94,
        0x9F, 0xA6, 0x98, 0x55, 0x0D, 0x0A,
    ];

fn create_save_dir(path: &String){
    let path = std::path::Path::new(path.as_str());
    if path.exists(){
        println!("使用已存在的目录 {}", path.to_string_lossy());
    }
    else {
        std::fs::create_dir(path).unwrap();
        println!("未找到目录，已创建新目录 {}", path.to_string_lossy());
    }
}

fn convert_data(data: u32) -> Vec<u8>{
    let mut data_vec = Vec::from(data.to_string().as_bytes().to_owned());
    data_vec.iter_mut().for_each(|x| *x += 51);
    data_vec
}

fn insert_data(src: Vec<u8>, dst: &mut Vec<u8>, offset: usize){
    if src.len() < 1{
        println!("invalid!!!");
        return;
    }
    else {
        dst.splice(offset..offset+1, src);
    }
}

fn save(level: u32, path: &String, save_data: &mut Vec<u8>){
    let mut data = save_data.clone();
    insert_data(convert_data(level), &mut data, 14);
    let mut file =
        std::fs::File::create(format!("{}/LEVEL_{:02}.sav", path, level + 1)).expect("文件创建失败");
    file.write(&data).unwrap();
    println!("生成存档：{}", format!("{}/LEVEL_{:02}.sav", path, level + 1));
}

fn input(msg: String, default_value: u32) -> u32{
    println!("请输入 '{}' 的值, <回车使用默认值，默认值为：{}>", msg, default_value);
    print!(">>> ");
    io::stdout().flush().unwrap();
    let mut read_buf = String::new();
    io::stdin().read_line(&mut read_buf).expect("非法的输入值");
    match read_buf.trim().parse::<u32>() {
        Ok(value) => value,
        Err(msg) => {
            match msg.kind() {
                IntErrorKind::Empty => println!("输入为空，使用默认值：{}", default_value),
                _ => println!("非法的输入值，错误为：{}，使用默认值：{}", msg, default_value),
            }
            default_value
        }
    }
}

fn main() {
    let experience: u32 = input("Experience (经验)".to_string(), 0);
    let score: u32 = input("Score (得分)".to_string(), 0);
    let time_control: u32 = input("Time Control (红心)".to_string(), 0);
    let drop_section_accelerator: u32 = input("Drop Section Accelerator (盾牌)".to_string(), 0);
    let good_luck: u32 = input("Good Luck (辛运草)".to_string(), 0);
    let amphora: u32 = input("Amphora (壶)".to_string(), 0);
    let hammer: u32 = input("Hammer (锤子)".to_string(), 0);
    println!("请输入要生成的存档关卡数(以空格间隔): <回车使用默认值，默认为全部关卡>");
    print!(">>> ");
    io::stdout().flush().unwrap();
    let mut read_buf = String::new();
    io::stdin().read_line(&mut read_buf).expect("非法的输入值");
    let save_vec = read_buf.trim().split_whitespace().collect::<Vec<&str>>();
    let path = String::from("saves");
    create_save_dir(&path);
    let mut save_data = DATA_TEMPLATE.to_vec();
    insert_data(convert_data(time_control), &mut save_data, 197);
    insert_data(convert_data(good_luck), &mut save_data, 182);
    insert_data(convert_data(drop_section_accelerator), &mut save_data, 167);
    insert_data(convert_data(experience), &mut save_data, 83);
    insert_data(convert_data(amphora), &mut save_data, 67);
    insert_data(convert_data(hammer), &mut save_data, 50);
    insert_data(convert_data(score), &mut save_data, 30);
    match save_vec.is_empty() {
        true => for level in 0..18{
            save(level, &path, &mut save_data)
        },
        false => for level in save_vec{
            match level.parse::<u32>(){
                Ok(level) => save(level - 1, &path, &mut save_data),
                Err(msg) => println!("非法的输入值，错误为：{}", msg),
            }
        },
    }
}
