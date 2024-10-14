use std::collections::HashSet;
use std::fmt::{format, Display, Formatter};
use std::fs::{create_dir, File};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub struct WumpusMap{
    pub map: Vec<Vec<char>>,
}

impl WumpusMap {
    pub fn new(path: &PathBuf) -> Self {
        let mut map = Vec::new();
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);
        for i in reader.lines() {
            let str = i.unwrap();
            let another = str.split("");
            let mut vec = Vec::new();
            for j in another {
                if !j.is_empty() {
                    let c: char = j.parse().unwrap();
                    vec.push(c);
                }
            }
            map.push(vec);
        }
        Self {
            map
        }
    }

    pub fn get_links_goals(&self) -> String{
        let mut answer = String::new();
        for i in self.map[0].iter().enumerate(){
            if *i.1 != 'X' {
                answer.push_str(&format!("(link cell_0_{} g)\n", i.0));
                answer.push_str(&format!("(hassameDirection cell_0_{} g north)\n", i.0));
            }
        }
        let num1 = self.map[0].len()-1;
        let num2 = self.map.len() -1;
        for i in self.map.last().unwrap().iter().enumerate(){
            if *i.1 != 'X' {
                answer.push_str(&format!("(link cell_{}_{} g)\n",num2,i.0));
                answer.push_str(&format!("(hassameDirection cell_{}_{} g south)\n",num2,i.0));
            }
        }
        let rows_length = self.map.len();
        for i in 0..rows_length {
            if self.map[i][0] != 'X' {
                if i > 0 && i < rows_length {
                    answer.push_str(&format!("(link cell_{}_0 g)\n", i));
                }
                answer.push_str(&format!("(hassameDirection cell_{}_0 g left)\n", i));
            }
            if self.map[i][num1] != 'X'{
                if  (i > 0 && i < rows_length){
                    answer.push_str(&format!("(link cell_{}_{} g)\n", i, num1));
                }
                answer.push_str(&format!("(hassameDirection cell_{}_{} g right)\n",i,num1));
            }
        }
        answer
    }

    pub fn get_links_north_south(&self) -> String{
        let mut answer = String::new();
        for i in self.map.iter().enumerate(){
            for j in i.1.iter().enumerate(){
                if *j.1 != 'X'{
                    if i.0 != 0{
                        let north = i.0 -1;
                        if let Some(x) = self.map.get(north){
                            if let Some(y) = x.get(j.0){
                                if *y != 'X'{
                                    answer.push_str(&format!("(link cell_{}_{} cell_{}_{})\n",i.0,j.0,north,j.0));
                                    answer.push_str(&format!("(hassameDirection cell_{}_{} cell_{}_{} north)\n",i.0,j.0,north,j.0));
                                }
                            }
                        }
                    }
                    let south = i.0 + 1;
                    if let Some(x) = self.map.get(south){
                        if let Some(y) = x.get(j.0){
                            if *y != 'X'{
                                answer.push_str(&format!("(link cell_{}_{} cell_{}_{})\n",i.0,j.0,south,j.0));
                                answer.push_str(&format!("(hassameDirection cell_{}_{} cell_{}_{} south)\n",i.0,j.0,south,j.0));
                            }
                        }
                    }
                }
            }
        }
        answer
    }

    pub fn get_links_left_right(&self) -> String{
        let mut answer = String::new();
        for i in self.map.iter().enumerate(){
            for j in i.1.iter().enumerate(){
                if *j.1 != 'X'{
                    if j.0 != 0{
                        let left = j.0 -1;
                        if let Some(x) = self.map.get(i.0){
                            if let Some(y) = x.get(left){
                                if *y != 'X'{
                                    answer.push_str(&format!("(link cell_{}_{} cell_{}_{})\n",i.0,j.0,i.0,left));
                                    answer.push_str(&format!("(hassameDirection cell_{}_{} cell_{}_{} left)\n",i.0,j.0,i.0,left));
                                }
                            }
                        }
                    }
                    let right = j.0 + 1;
                    if let Some(x) = self.map.get(i.0){
                        if let Some(y) = x.get(right){
                            if *y != 'X'{
                                answer.push_str(&format!("(link cell_{}_{} cell_{}_{})\n", i.0, j.0, i.0, right));
                                answer.push_str(&format!("(hassameDirection cell_{}_{} cell_{}_{} right)\n", i.0, j.0, i.0, right));
                            }
                        }
                    }
                }
            }
        }
        answer
    }
    pub fn initialize_header(&self) -> String{
        let mut others = String::new();
        let mut wumpus = String::new();
        let mut arrow = String::new();
        let mut keys = String::new();
        let mut doors = String::new();
        let mut crates = String::new();
        let mut trampoline = String::new();
        for i in self.map.iter().enumerate(){
            for j in i.1.iter().enumerate(){
                if *j.1 != 'X'{
                    others.push_str(&format!("cell_{}_{} ",i.0,j.0));
                }
                match *j.1{
                    'W' => {let str = format!("w_{}_{} ",i.0,j.0); wumpus.push_str(&str);},
                    'A' => {let str = format!("a_{}_{} ",i.0,j.0);arrow.push_str(&str)},
                    'K' => {let str = format!("k_{}_{} ",i.0,j.0);keys.push_str(&str);},
                    'D' => {let str = format!("d_{}_{} ",i.0,j.0);doors.push_str(&str);},
                    'C' => {let str = format!("crate_{}_{} ",i.0,j.0);crates.push_str(&str);},
                    'T' => {let str = format!("tramp_{}_{} ",i.0,j.0);trampoline.push_str(&str);},
                    _ => {}
                }
            }
        }
        if !others.is_empty() {
            others.push_str("- other\n");
        }
        if !wumpus.is_empty() {
            wumpus.push_str("- wumpus\n");
        }
        if !arrow.is_empty() {
            arrow.push_str("- arrow\n");
        }
        if !keys.is_empty() {
            keys.push_str("- key\n");
        }
        if !doors.is_empty() {
            doors.push_str("- door\n");
        }
        if !crates.is_empty() {
            crates.push_str("- crate\n");
        }
        if !trampoline.is_empty() {
            trampoline.push_str("- trampoline\n");
        }
        let mut answer = String::new();
        answer.push_str(&others);
        answer.push_str(&wumpus);
        answer.push_str(&arrow);
        answer.push_str(&keys);
        answer.push_str(&doors);
        answer.push_str(&crates);
        answer.push_str(&trampoline);
        answer
    }

    pub fn get_is_empty(&self) -> String{
        let mut answer = String::new();
        answer.push_str("(isempty g)\n");
        answer.push_str("(noTrampoline g)\n");
        for i in self.map.iter().enumerate(){
            for j in i.1.iter().enumerate(){
                if *j.1 == 'S'{
                    answer.push_str(&format!("(noTrampoline cell_{}_{})\n",i.0,j.0));
                }
                if *j.1 == ' ' || *j.1 == 'A' || *j.1 == 'K' || *j.1 == 'T'{
                    answer.push_str(&format!("(isempty cell_{}_{})\n",i.0,j.0));
                    if *j.1 != 'T'{
                        answer.push_str(&format!("(noTrampoline cell_{}_{})\n",i.0,j.0));
                    }
                }
            }
        }
        answer
    }

    pub fn add_locations(&self) -> String{
        let mut answer = String::new();
        for i in self.map.iter().enumerate(){
            for j in i.1.iter().enumerate(){
                match *j.1 {
                    'S' => { answer.push_str(&format!("(location agent cell_{}_{})\n",i.0,j.0));}
                    'A' => {answer.push_str(&format!("(location a_{}_{} cell_{}_{})\n",i.0,j.0,i.0,j.0));}
                    'C' => {answer.push_str(&format!("(location crate_{}_{} cell_{}_{})\n",i.0,j.0,i.0,j.0));},
                    'D' => {answer.push_str(&format!("(location d_{}_{} cell_{}_{})\n",i.0,j.0,i.0,j.0));},
                    'K' => {answer.push_str(&format!("(location k_{}_{} cell_{}_{})\n",i.0,j.0,i.0,j.0));},
                    'T' => {answer.push_str(&format!("(location tramp_{}_{} cell_{}_{})\n",i.0,j.0,i.0,j.0));},
                    'W' => {answer.push_str(&format!("(location w_{}_{} cell_{}_{})\n",i.0,j.0,i.0,j.0));},
                    _ => {}
                }
            }
        }

        answer
    }
}


impl Display for WumpusMap{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in &self.map{
            let mut str = String::new();
            for j in i{
                str.push(*j);
            }
            writeln!(f, "{}", str)?;
        }
        Ok(())
    }
}