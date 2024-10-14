use std::fs::{create_dir, read_dir, DirEntry, File};
use std::io::{BufRead, BufReader, BufWriter, Error, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::map::WumpusMap;

pub mod map;
// this is the function that should create a pddl file

pub fn write_to_pddl_file(dir : &PathBuf) -> Result<(),Error>{
    assert!(dir.is_dir());
    let name = dir.file_name().unwrap().to_str().unwrap();
    let sol_d = String::from(name) + "_pddl_files";
    let handler = std::fs::read_dir(dir)?;
    let mut solution_director = PathBuf::new();
    solution_director.push(sol_d);
    if !solution_director.exists(){
        create_dir(&solution_director)?;
    }
    for i in handler{
        let file = i?.path();
        write_to_single_file(&file,&solution_director)?;
    }

    Ok(())
}

pub fn generate_solutions(path : &PathBuf) -> Result<(),Error>{
    let fast_down_ward = Path::new("fast-downward-23.06/fast-downward.py");
    println!("{:?}",fast_down_ward);
    let name_pddl_dir = path.file_name().unwrap().to_str().unwrap();
    let mut store_pddl_out_put = PathBuf::new();
    store_pddl_out_put.push(String::from(name_pddl_dir) + "pddl_out_put_directory");
    let mut solutions_dir = PathBuf::new();
    solutions_dir.push(String::from(name_pddl_dir) + "solutions_final");
    if !solutions_dir.exists(){
        create_dir(&solutions_dir)?;
    }else { 
        assert!(solutions_dir.is_dir());
    }
    if store_pddl_out_put.exists(){
        assert!(store_pddl_out_put.is_dir());
    }else { 
        create_dir(&store_pddl_out_put)?;
    }
    let mut domain_file = PathBuf::new();
    domain_file.push("domaindownload.pddl");
    let dir = read_dir(&path)?;
    for i in dir{
        let pddl_files = i?.path();
        let name = pddl_files.file_name().unwrap().to_str().unwrap();
        let output_file = String::from(name) + "plan_out";
        let mut file = store_pddl_out_put.clone();
        file.push(output_file);
        println!("{}",name);
        let command = Command::new("python").arg(fast_down_ward).arg("--plan-file").arg(&file).arg("--alias").arg("lama-first").arg(domain_file.as_path()).arg(pddl_files.as_path())
            .output()?;
        println!("{}",command.status);
        let file_to_write = String::from(name.replace(".pddl","-solution.txt"));
        let mut file_path = solutions_dir.clone();
        file_path.push(file_to_write);
        parse_the_output(&file,&file_path)?;
    }
    Ok(())
}

pub fn write_to_single_file(file: &PathBuf,solution_director: &PathBuf) -> Result<(),Error>{
    let map = WumpusMap::new(&file);
    println!("{:?}",map.map);
    println!("{}",map);
    let wumpus_file_name = file.file_name().unwrap().to_str().unwrap().replace(".txt",".pddl");
    let sol_pddl_name = file.file_name().unwrap().to_str().unwrap();
    let mut wumpus_pddl = solution_director.clone();
    wumpus_pddl.push(wumpus_file_name);
    let file_to_create = File::create(&wumpus_pddl)?;
    let mut writer = BufWriter::new(file_to_create);
    let mut sol_str = r"
    (define (problem <name>) (:domain wumpusWorldagain)
    (:objects
        agent - agent
        g - goal
        north south left right - direction
        <objects>
    )
    (:init
        <initialize>
    )
    (:goal (and
        (location agent g)
    ))
    )
    ";
    let replacer = sol_pddl_name.replace(".txt","");
    let intermediate = sol_str.replace("<name>", &replacer);
    let next_inter = intermediate.replace("<objects>",&map.initialize_header());
    let mut init_str = String::new();
    init_str.push_str(map.get_links_goals().as_str());
    init_str.push_str(map.get_links_north_south().as_str());
    init_str.push_str(map.get_links_left_right().as_str());
    init_str.push_str(map.get_is_empty().as_str());
    init_str.push_str(map.add_locations().as_str());
    let next_next_inter = next_inter.replace("<initialize>",&init_str);
    writer.write_all(next_next_inter.as_bytes())?;
    writer.flush()?;

    Ok(())
}


pub fn parse_the_output(path : &PathBuf,file_to_write: &PathBuf) -> Result<(),Error>{
    let file = File::open(path)?;
    let bufreader = BufReader::new(file);
    let file2 = File::create(file_to_write)?;
    let mut writer = BufWriter::new(file2);
    for i in bufreader.lines(){
        let str = i?;
        if str.contains("move"){
            let pr = String::from("walk ") + &helper_to_output(&str);
            writer.write(pr.as_bytes())?;
            writer.write(b"\n")?;
        }else if str.contains("push"){
            let pr = String::from("push ") + &helper_to_output(&str);
            writer.write(pr.as_bytes())?;
            writer.write(b"\n")?;
        }else if str.contains("kill"){
            let pr = String::from("shoot ") + &helper_to_output(&str);
            writer.write(pr.as_bytes())?;
            writer.write(b"\n")?;
        }else if str.contains("unlock"){
            let pr = String::from("unlock ") + &helper_to_output(&str);
            writer.write(pr.as_bytes())?;
            writer.write(b"\n")?;
        }else if str.contains("jump"){
            let pr = String::from("jump ") + &helper_to_output(&str);
            writer.write(pr.as_bytes())?;
            writer.write(b"\n")?;
        }
    }
    writer.flush()?;
    Ok(())
}

pub fn helper_to_output(str : &String) -> String{
    if str.contains("north") {
        String::from("north")
    } else if str.contains("south"){
        String::from("south")
    }else if str.contains("right"){
        String::from("east")
    }else { 
        String::from("west")
    }
}