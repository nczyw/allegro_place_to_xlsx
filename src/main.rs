use std::fs;
use std::path::PathBuf;
use clap::Parser;
use clap::CommandFactory;

#[derive(Parser, Debug)]
#[command(
    name = concat!(
        env!("BUILD_NAME"),
    ) ,
    version = concat!(
        env!("BUILD_VERSION"),
    ),
    author = concat!(
        env!("BUILD_AUTHOR"),
    ),
    about = concat!(
        env!("BUILD_ABOUT"),
    ),
    long_about = concat!(
        env!("BUILD_ABOUT"), "\n",
        "Program: ", env!("BUILD_NAME"), "\n",
        "Version: ", env!("BUILD_VERSION"), "\n",
        "Author: ", env!("BUILD_AUTHOR"), "\n",
        "Build Time: ", env!("BUILD_TIME"), "\n",
        "Build sHash:", env!("BUILD_SHASH"), "\n",
        "Build lHash:", env!("BUILD_LHASH"), "\n",
    ),
    after_help = env!("BUILD_COPYRIGHT"),
)]
struct Args {
    /// Input file path
    #[arg(short, long, default_value = "place_txt.txt")]
    source: PathBuf,

    /// Output file path (optional, if not specified, use input filename)
    #[arg(short, long)]
    output: Option<PathBuf>,
}

struct PlacementEntry {
    designator: String,
    mid_x: String,
    mid_y: String,
    rotation: String,
    layer: String,
    footprint: String,
}

fn main() {
    // 解析命令行参数并处理错误
    
    let args = match Args::try_parse() {
        Ok(args) => args,
        Err(e) => {
            if e.use_stderr(){
                eprintln!("{}",e);
                //println!("Print help");
                Args::command().print_help().unwrap();
                std::process::exit(0); 
            }
            else {
                println!("{}",e);
                
                std::process::exit(0); // 返回0而不是错误代码
            }
            
        }
    };
    
    let source_file = &args.source;
    // 如果没有指定输出路径，则根据输入路径生成输出路径
    let output_file = if let Some(output_path) = args.output {
        ensure_xlsx_extension(&output_path)
    } else {
        generate_output_path(&source_file)
    };
    
    let output_file_str = output_file.to_string_lossy();
    
    // 读取输入文件
    let content = match fs::read_to_string(&source_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading input file '{}': {}", source_file.display(), e);
            std::process::exit(0); // 返回0而不是错误代码
        }
    };
    
    // 解析放置数据
    let (placement_data, headers) = parse_placement(&content);
    
    // 将数据写入XLSX文件
    if let Err(e) = write_to_xlsx(&placement_data, &headers, &output_file_str) {
        eprintln!("Error writing to xlsx file '{}': {}", output_file_str, e);
        std::process::exit(0); // 返回0而不是错误代码
    }
    
    println!("Placement data has been successfully converted to xlsx file: {}", output_file_str);
}

fn generate_output_path(input_path: &PathBuf) -> PathBuf {
    let mut output_path = input_path.clone();
    
    // 移除原扩展名，添加.xlsx扩展名
    if let Some(stem) = output_path.file_stem() {
        let new_name = format!("{}.xlsx", stem.to_string_lossy());
        output_path.set_file_name(new_name);
    } else {
        // 如果没有文件名（不太可能），则直接添加.xlsx
        output_path.set_extension("xlsx");
    }
    
    output_path
}

fn ensure_xlsx_extension(path: &PathBuf) -> PathBuf {
    let mut path = path.clone();
    if path.extension().map_or(true, |ext| ext != "xlsx") {
        path.set_extension("xlsx");
    }
    path
}

fn parse_placement(content: &str) -> (Vec<PlacementEntry>, Vec<String>) {
    let mut placement_entries = Vec::new();
    let headers = vec!["Designator", "Mid x", "Mid y", "Rotation", "Layer", "Footprint"]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    
    // 解析数据行
    for line in content.lines() {
        // 跳过空行和注释行
        if line.trim().is_empty() || line.starts_with("VERSION") || line.starts_with("#") || line.starts_with("---") {
            continue;
        }
        
        // 按感叹号分割行
        let parts: Vec<&str> = line.split('!').collect();
        if parts.len() < 6 {
            continue;
        }
        
        // 提取前6列数据
        let designator = parts[0].trim().to_string();
        let mid_x = parts[1].trim().to_string();
        let mid_y = parts[2].trim().to_string();
        let rotation = parts[3].trim().to_string();
        let mirror = parts[4].trim();
        let footprint = format!("'{}", parts[5].trim()); // 在Footprint前添加'防止0被优化
        
        // 确定Layer列的值:如果mirror为空则为T，否则为B
        let layer = if mirror.is_empty() { "T".to_string() } else { "B".to_string() };
        
        placement_entries.push(PlacementEntry {
            designator,
            mid_x,
            mid_y,
            rotation,
            layer,
            footprint,
        });
    }
    
    (placement_entries, headers)
}

fn write_to_xlsx(placement_data: &[PlacementEntry], headers: &[String], output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    use umya_spreadsheet::*;
    use std::path::Path;

    // 创建新的工作簿
    let mut book = new_file();
    
    // 获取默认工作表并重命名
    let sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();
    sheet.set_name("Placement");

    // 设置表头 - 使用预定义的表头
    for (i, header) in headers.iter().enumerate() {
        book.get_sheet_by_name_mut("Placement")
            .unwrap()
            .get_cell_mut(((i + 1) as u32, 1 as u32))
            .set_value(header);
    }

    // 写入数据行
    for (row_index, entry) in placement_data.iter().enumerate() {
        let row = (row_index + 2) as u32; // 从第2行开始(第1行是表头)
        let sheet = book.get_sheet_by_name_mut("Placement").unwrap();
        
        sheet.get_cell_mut((1, row)).set_value(&entry.designator);
        sheet.get_cell_mut((2, row)).set_value(&entry.mid_x);
        sheet.get_cell_mut((3, row)).set_value(&entry.mid_y);
        sheet.get_cell_mut((4, row)).set_value(&entry.rotation);
        sheet.get_cell_mut((5, row)).set_value(&entry.layer);
        sheet.get_cell_mut((6, row)).set_value(&entry.footprint);
    }

    // 保存工作簿
    let path = Path::new(output_file);
    if let Err(e) = writer::xlsx::write(&book, path){
        // 尝试删除临时文件 (.xlsxtmp)
        let tmp_path = Path::new(output_file).with_extension("xlsxtmp");
        if tmp_path.exists() {
            if let Err(del_err) = std::fs::remove_file(&tmp_path) {
               eprint!("Unable to delete temporary file {}: {}",tmp_path.display(),del_err);
            }
        }
        return Err(e.into());
    }
    
    Ok(())
}



