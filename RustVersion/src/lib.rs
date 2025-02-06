//!
//! 欢迎来到Tank Launcher Module!
//! 本模块使用MIT协议进行开源！各位可以随意使用本模块的函数。
//! 本模块暂未发布至crates.io，因为我不想发！！
//! 本模块中，很多需要用到网络请求的部分，未使用async包围。因为作者的TLM不是GUI程序。各位想加就加上async/await把！

pub mod account;
pub mod download;
pub mod err;

pub use err::{MMCLLError, MMCLLResult};

/// 部分常量值，在程序的任意位置均可直接调用。
pub mod some_const {
    pub const LAUNCHER_NANE: &str = "MMCLL"; //在使用此库时，请自觉将此值改成你的【<启动器名称>】。在使用默认方式启动时，会自动将【${launcher_name}】替换成该值。
    pub const LAUNCHER_VERSION: &str = "0.0.1-Alpha-12"; //在使用此库时，请自觉将此值改成你的【<启动器版本>】【可以加上Alpha、Beta、Pre三个值，因为在启动替换（${launcher_version}）时用到这个值。不过各位可以自行去函数put_arguments进行修改以适配该值。】
    pub const USER_AGENT: &str = "MMCLL/0.0.1.12"; //在使用此库时，请自觉将此值改成你的【<启动器名称>/<启动器版本>】。
}
/**
 * 部分全局变量值。在需要的时候可以使用with_borrow包裹住该变量以便使用，赋值和引用均可。但是你需要为你赋过的值负责！。
 */
pub mod some_var {
    use std::cell::RefCell;
    thread_local! {
        pub static DOWNLOAD_SOURCE: RefCell<i32> = RefCell::new(1);  //下载源：目前仅支持两个数字，1：官方、2：BMCLAPI
        pub static MC_ROOT_JSON: RefCell<serde_json::Value> = RefCell::new(serde_json::Value::Null);  //mc的元数据（可以自己赋值也可以由类库帮忙赋值！）仅能赋值元数据值，如果赋上了别的值，后果自负！
        pub static AUTHLIB_PATH: RefCell<String> = RefCell::new(String::new());  //设置第三方登录的模块jar文件。在使用第三方登录的时候一定要设置该参数！
        pub static BIGGEST_THREAD: RefCell<i32> = RefCell::new(64);  //最大线程，但是在Rust里指的是最大并发量（必须要提前赋值，否则将按照默认64处理。）
        #[allow(unused)]
        pub static AUTHLIB_URL: RefCell<String> = RefCell::new(String::new());
    }
}
/**
 * 许多在启动时可能需要用到的静态函数。（无需初始化，仅需直接调用。）
 */
pub mod main_mod {
    /**
     * 从一个path获取外部文件。
     * 此处使用了encoding转码，以防止有某些大聪明玩家使用GBK方式写文件
     */
    pub fn get_file(path: &str) -> Option<String> {
        use std::io::Read;
        let p = std::path::Path::new(path);
        let mut ss = std::fs::File::open(p).ok()?;
        let mut buf = Vec::new();
        ss.read_to_end(&mut buf).ok()?;
        if let Ok(e) = String::from_utf8(buf.clone()) {
            Some(e)
        } else {
            let (cow, _, h) = encoding_rs::GBK.decode(&buf);
            if h {
                None
            } else {
                Some((&cow).to_string())
            }
        }
    }
    /**
     * 将Vec<u8>写出到文件
     */
    pub fn set_file_vecu8(path: &str, content: &[u8]) -> bool {
        let p = std::path::Path::new(path);
        if p.is_dir() {
            return false;
        }
        let parent = match p.parent() {
            Some(p) => p,
            None => return false,
        };
        if !parent.exists() || parent.exists() && parent.is_file() {
            let q = std::fs::create_dir_all(parent);
            if let Err(_) = q {
                return false;
            }
        }
        let mut f = match std::fs::File::create(p) {
            Ok(f) => f,
            Err(_) => return false,
        };
        use std::io::Write;
        match f.write_all(&content) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    /**
     * 将内容写出到文件
     */
    pub fn set_file(path: &str, content: String) -> bool {
        return set_file_vecu8(path, content.as_bytes());
    }
    /**
     * 删除文件
     */
    pub fn delete_file(path: &str) -> bool {
        let p = std::path::Path::new(path);
        if !p.exists() || p.exists() && p.is_dir() {
            return false;
        }
        match std::fs::remove_file(p) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    /**
     * 获取某一个文件的SHA1值
     */
    pub fn get_sha1(path: &str) -> Option<String> {
        let mut file = std::fs::File::open(path).ok()?;
        use crypto::digest::Digest;
        use std::io::Read;
        let mut sha1 = crypto::sha1::Sha1::new();
        let mut buffer = [0; 1024];
        loop {
            let n = file.read(&mut buffer).ok()?;
            if n == 0 {
                break;
            }
            sha1.input(&buffer[..n]);
        }
        let hash = sha1.result_str();
        Some(hash)
    }
    /**
     * 将16进制字符串转换成Vec<u8>形式
     * 例如【aabbcc】转成【[170, 187, 204]】
     */
    pub fn hex_decode(raw: &str) -> Option<Vec<u8>> {
        let mut res: Vec<u8> = Vec::new();
        if raw.len() % 2 != 0 {
            return None;
        }
        let reg = regex::Regex::new("^(?:[a-f0-9]{2})+$").unwrap();
        if !reg.is_match(raw) {
            return None;
        }
        let chars = raw.chars().collect::<Vec<char>>();
        let mut i = 0;
        while i < chars.len() {
            let r =
                u32::from_str_radix(format!("{}{}", chars[i], chars[i + 1]).as_str(), 16).unwrap();
            res.push(r as u8);
            i += 2;
        }
        Some(res.clone())
    }
    /**
     * 将16进制数组转成String形式
     * 例如【[170, 187, 204]】转成【aabbcc】
     */
    pub fn hex_encode(raw: Vec<u8>) -> String {
        let mut res = String::new();
        for i in raw.into_iter() {
            res.push_str(format!("{:x}", i).as_str());
        }
        res.clone()
    }
    /**
     * 该函数目前仅适用于在离线登录时根据用户名生成一个唯一的UUID。
     */
    pub fn generate_bukkit_uuid(name: &str) -> String {
        use crypto::digest::Digest;
        let mut md5 = crypto::md5::Md5::new();
        md5.input_str(format!("OfflinePlayer:{}", name).as_str());
        let res_str = md5.result_str();
        let mut res_hex = hex_decode(res_str.as_str()).unwrap();
        res_hex[6] = (res_hex[6] & 0x0f) | 0x30;
        res_hex[8] = (res_hex[8] & 0x3f) | 0x80;
        hex_encode(res_hex)
    }
    /**
     * 该函数目前仅适用于在初始化第三方登录时对该皮肤站元数据进行base64编码。
     * 该函数已废弃，如果想获取元数据base64编码，请自行使用account_mod下的登录一次，即可直接异步获取。
     */
    #[allow(dead_code, deprecated)]
    #[deprecated(
        since = "0.0.8",
        note = "Please login thirdparty in account_mod, and auto get base64 code by sync."
    )]
    pub fn generate_thirdparty_metadata_base64(url: &str) -> String {
        use base64::Engine;
        let um = super::account_mod::UrlMethod::new(url);
        let metadata = um.get_default();
        if let None = metadata {
            return String::new();
        }
        let metadata = String::from_utf8(metadata.unwrap());
        if let Err(_) = metadata {
            return String::new();
        }
        let base =
            base64::engine::general_purpose::STANDARD.encode(metadata.unwrap().replace("\\/", "/"));
        base
    }
    /**
     * 截取文件名
     */
    pub fn extract_file_name(file: &str) -> String {
        let rf = file.rfind("\\");
        if let None = rf {
            return String::new();
        }
        let rf = rf.unwrap();
        let versub = file.get((rf + 1)..file.len());
        if let None = versub {
            return String::new();
        }
        let versub = versub.unwrap();
        versub.to_string()
    }
    /**
     * 获取exe的位数（32位或64位）
     */
    pub fn get_file_bit(file: String) -> Option<bool> {
        let path = std::path::Path::new(file.as_str());
        if !path.exists() || path.exists() && path.is_dir() {
            return None;
        }
        let data = pelite::FileMap::open(path).ok()?;
        let file = pelite::PeFile::from_bytes(&data).ok()?;
        match file {
            pelite::Wrap::T64(_) => Some(true),
            pelite::Wrap::T32(_) => Some(false),
        }
    }
    /**
     * 获取exe文件的版本号
     */
    pub fn get_file_version(file: String) -> Option<String> {
        let path = std::path::Path::new(file.as_str());
        if !path.exists() || path.exists() && path.is_dir() {
            return None;
        }
        let data = pelite::FileMap::open(path).ok()?;
        let file = pelite::PeFile::from_bytes(&data).ok()?;
        let file = file.resources().ok()?;
        let fixed_version = file.version_info().ok()?.fixed()?;
        Some(format!(
            "{}.{}.{}.{}",
            fixed_version.dwFileVersion.Major.to_string(),
            fixed_version.dwFileVersion.Minor.to_string(),
            fixed_version.dwFileVersion.Build.to_string(),
            fixed_version.dwFileVersion.Patch.to_string()
        ))
    }
    /**
     * 通过正版用户名获取其UUID
     */
    pub fn name_to_uuid(name: &str) -> Option<String> {
        let url = format!("https://api.mojang.com/users/profiles/minecraft/{}", name);
        let url = super::account_mod::UrlMethod::new(url.as_str());
        let res = url.get_default()?;
        let res = String::from_utf8(res.clone()).ok()?;
        let ser = serde_json::from_str::<serde_json::Value>(res.as_str()).ok()?;
        let ser = ser.get("id")?.as_str()?;
        Some(ser.to_string())
    }
}
/**
 * 专注于启动游戏的模块，所有启动游戏的函数都可以在这里面找到！
 */
pub mod launcher_mod {
    /**
     * 此方法用于将json libraries里的name值转换为path。
     */
    pub fn convert_name_to_path(name: String) -> Option<String> {
        let mut name = name.clone();
        let suffix: String;
        if name.contains("@") {
            let rf = name.rfind("@")?;
            suffix = name[(rf + 1)..name.len()].to_string();
            name = name[0..rf].to_string();
        } else {
            suffix = String::from("jar")
        }
        let spl: Vec<&str> = name.split(":").collect();
        if spl.len() == 4 {
            Some(format!(
                "{}\\{}\\{}\\{}-{}-{}.{}",
                spl[0].replace(".", "\\"),
                spl[1],
                spl[2],
                spl[1],
                spl[2],
                spl[3],
                suffix
            ))
        } else if spl.len() == 3 {
            Some(format!(
                "{}\\{}\\{}\\{}-{}.{}",
                spl[0].replace(".", "\\"),
                spl[1],
                spl[2],
                spl[1],
                spl[2],
                suffix
            ))
        } else {
            None
        }
    }
    /**
     * 根据一个原版的json，准确的找到原版键值。（只能原版，如果不是原版，则必定返回None）
     * 会按照clientVersion、patches->game|version、metadata->versions->releaseTime、id值进行找。
     * 如果连最终的id值也不符合，则返回必定返回None！
     * 但是最终的id值很可能不是代表着原版值，因为别的启动器很可能会修改文件夹的名字顺带把json里的id值也改了。
     * 所以各位一定要记得做判断！如果想自定义一个类来启动的而不是用game_launch类启动的话。当然也可以用catch_unwind来捕捉panic也就对了！
     */
    pub fn get_mc_vanilla_version(json: String) -> Option<String> {
        let root = serde_json::from_str::<serde_json::Value>(json.as_str()).ok()?;
        if let Some(e) = root["clientVersion"].as_str() {
            if !e.is_empty() {
                return Some(e.to_string());
            }
        }
        if let Some(e) = root["patches"].as_array() {
            for i in e.into_iter() {
                let id = i["id"].as_str();
                if let None = id {
                    continue;
                }
                if id?.eq("game") {
                    let mcid = i["version"].as_str();
                    if let Some(f) = mcid {
                        if !f.is_empty() {
                            return Some(f.to_string());
                        }
                    }
                }
            }
        }
        if let Some(w) = root["releaseTime"].as_str() {
            let v = match super::some_var::DOWNLOAD_SOURCE.with_borrow(|e| e.clone()) {
                2 => "https://bmclapi2.bangbang93.com/mc/game/version_manifest.json",
                _ => "https://piston-meta.mojang.com/mc/game/version_manifest.json",
            };
            if super::some_var::MC_ROOT_JSON
                .with_borrow(|e| e.clone())
                .is_null()
            {
                let url = super::account_mod::UrlMethod::new(v);
                if let Some(e) = url.get_default() {
                    let e = String::from_utf8(e);
                    if let Ok(f) = e {
                        let s = serde_json::from_str::<serde_json::Value>(f.as_str()).ok()?;
                        super::some_var::MC_ROOT_JSON.set(s.clone());
                    }
                }
            }
            if !super::some_var::MC_ROOT_JSON
                .with_borrow(|e| e.clone())
                .is_null()
            {
                let mrj = super::some_var::MC_ROOT_JSON.with_borrow(|e| e.clone());
                if let Some(g) = mrj["version"].as_array() {
                    for h in g.into_iter() {
                        if let Some(j) = h["releaseTime"].as_str() {
                            if j.eq(w) {
                                if let Some(d) = h["id"].as_str() {
                                    return Some(d.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
        if let Some(e) = root["id"].as_str() {
            if !e.is_empty() {
                return Some(e.to_string());
            }
        }
        None
    }
    /**
     * 解压任意文件到路径。
     * 该函数并不会返回进度值，各位可以自行查看该函数并实现自己的回显进度的zip解压。
     * 该函数目前仅会返回bool值，如果解压成功则返回true，反之如果里面出现任何错误，则直接返回false。
     */
    pub fn unzip(zipfile: String, extfile: String) -> bool {
        let zip_path = std::path::Path::new(zipfile.as_str());
        let ext_path = std::path::Path::new(extfile.as_str());
        if !zip_path.exists() || (zip_path.exists() && !zip_path.is_file()) {
            return false;
        }
        if !zip_path.exists() || (zip_path.exists() && zip_path.is_file()) {
            let cd = std::fs::create_dir_all(ext_path);
            if let Err(_) = cd {
                return false;
            }
        }
        let zip_file = std::fs::File::open(zip_path);
        if let Err(_) = zip_file {
            return false;
        }
        let zip_file = zip_file.unwrap();
        let zip_ext = zip::ZipArchive::new(zip_file);
        if let Err(_) = zip_ext {
            return false;
        }
        let mut zip_ext = zip_ext.unwrap();
        for i in 0..zip_ext.len() {
            let f = zip_ext.by_index(i);
            if let Err(_) = f {
                continue;
            }
            let mut f = f.unwrap();
            if f.is_dir() {
                let ext_dir = ext_path.join(std::path::Path::new(&f.name().replace("\\", "")));
                let cd = std::fs::create_dir_all(ext_dir);
                if let Err(_) = cd {
                    continue;
                }
            } else {
                let file_path = ext_path.join(std::path::Path::new(f.name()));
                let ext_file = if !file_path.exists() {
                    std::fs::File::create(file_path)
                } else {
                    std::fs::File::open(file_path)
                };
                if let Err(_) = ext_file {
                    continue;
                }
                let mut ext_file = ext_file.unwrap();
                let res = std::io::copy(&mut f, &mut ext_file);
                if let Err(_) = res {
                    continue;
                }
            }
        }
        true
    }
    /**
     * 删除文件夹中的所有文件，但是保留后缀为suffix的值。该函数用于解压natives时需要删掉除了dll以外的所有文件。
     */
    pub fn delete_file_keep(dir_path: String, suffix: &str) -> bool {
        if dir_path.is_empty() {
            return false;
        }
        if suffix.is_empty() || suffix.eq(".") {
            return false;
        }
        if let None = dir_path.find("\\") {
            return false;
        }
        let suffix = &suffix[1..suffix.len()];
        let dir = walkdir::WalkDir::new(dir_path.as_str());
        for i in dir.into_iter().filter_map(|e| e.ok()) {
            let path = i.path();
            if path.exists() {
                if path.is_dir() {
                    continue;
                }
                let path_ext = path.extension();
                if let None = path_ext {
                    continue;
                }
                let path_ext = path_ext.unwrap();
                if !path_ext.eq(suffix) {
                    let cd = std::fs::remove_file(path);
                    if let Err(_) = cd {
                        continue;
                    }
                }
            }
        }
        true
    }
    /**
     * 任何类都可以用的数字转换！
     * 可以将字符串中的数字提取出来，或者是字符串中的非数字【字符】提取出来！
     */
    pub fn extract_number(ext: String, isnum: bool) -> String {
        ext.chars()
            .filter(|&c| {
                if isnum {
                    c.is_numeric()
                } else {
                    c.is_ascii_alphabetic()
                }
            })
            .collect::<String>()
    }
    /**
     * 根据找到的json中的inheritsFrom或者jar值，准确的找到另一个有关该原版的版本文件夹。
     */
    pub fn get_mc_inherits_from(version_path: String, ioj: &str) -> Option<String> {
        let path = std::path::Path::new(version_path.as_str());
        if path.exists() && path.is_dir() {
            let real_path = get_mc_real_path(version_path.clone(), ".json")?;
            let real_file = super::main_mod::get_file(real_path.as_str())?;
            let root = serde_json::from_str::<serde_json::Value>(real_file.as_str()).ok()?;
            if let Some(e) = root[ioj].as_str() {
                if e.is_empty() {
                    return Some(version_path.clone());
                }
                let parent_path = path.parent()?;
                let dir = walkdir::WalkDir::new(parent_path).min_depth(1).max_depth(1);
                for i in dir.into_iter().filter_map(|e| e.ok()) {
                    let pa = i.path();
                    if pa.is_file() {
                        continue;
                    }
                    let ps = pa.display().to_string();
                    let version_json = get_mc_real_path(ps.clone(), ".json");
                    if let None = version_json {
                        continue;
                    }
                    let json_content = super::main_mod::get_file(version_json?.as_str());
                    if let None = json_content {
                        continue;
                    }
                    let vanilla_version = get_mc_vanilla_version(json_content?);
                    if let None = vanilla_version {
                        continue;
                    }
                    if vanilla_version?.eq(e) {
                        return Some(ps.clone());
                    }
                }
            } else {
                return Some(version_path.clone());
            }
        }
        None
    }
    /**
     * 从inheritsFrom键中找到的json当作原版json，并拼接上inheritsFrom键所在的json，将其合并成一个json！
     */
    pub fn replace_mc_inherits_from(mut raw_json: String, mut ins_json: String) -> Option<String> {
        fn return_some(k: &mut serde_json::Map<String, serde_json::Value>) -> Option<String> {
            Some(serde_json::to_string(&k).ok()?)
        }
        if raw_json.is_empty() || ins_json.is_empty() {
            return None;
        }
        raw_json = raw_json.replace("\\", "");
        ins_json = ins_json.replace("\\", "");
        if raw_json.eq(ins_json.as_str()) {
            return Some(raw_json);
        }
        let rt_raw = serde_json::from_str::<serde_json::Value>(raw_json.as_str()).ok()?;
        let rt_raw = rt_raw.as_object()?;
        let mut rt_ins = serde_json::from_str::<serde_json::Value>(ins_json.as_str()).ok()?;
        let rt_ins = rt_ins.as_object_mut()?;
        let mc = rt_raw["mainClass"].as_str()?;
        rt_ins.remove("mainClass");
        rt_ins.insert(
            "mainClass".to_string(),
            serde_json::Value::String(mc.to_string()),
        );
        let id = rt_raw["id"].as_str()?;
        rt_ins.remove("id");
        rt_ins.insert("id".to_string(), serde_json::Value::String(id.to_string()));
        let raw_lib = rt_raw.get("libraries");
        if let Some(d) = raw_lib {
            if let Some(e) = d.as_array() {
                for i in e.into_iter() {
                    if let Some(f) = i.as_object() {
                        if let Some(h) = rt_ins.get_mut("libraries") {
                            if let Some(g) = h.as_array_mut() {
                                g.push(serde_json::Value::Object(f.clone()));
                            }
                        }
                    }
                }
            }
        }
        if let Some(r1) = rt_raw.get("arguments") {
            if let Some(r2) = r1.get("jvm") {
                if let Some(e) = r2.as_array() {
                    for i in e.into_iter() {
                        if let Some(i1) = rt_ins.get_mut("arguments") {
                            if let Some(i2) = i1.get_mut("jvm") {
                                if let Some(f) = i2.as_array_mut() {
                                    f.push(i.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
        if let Some(r1) = rt_raw.get("arguments") {
            if let Some(r2) = r1.get("game") {
                if let Some(e) = r2.as_array() {
                    for i in e.into_iter() {
                        if let Some(i1) = rt_ins.get_mut("arguments") {
                            if let Some(i2) = i1.get_mut("game") {
                                if let Some(f) = i2.as_array_mut() {
                                    f.push(i.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
        if let Some(m) = rt_raw.get("minecraftArguments") {
            if let Some(e) = m.as_str() {
                rt_ins.remove("minecraftArguments");
                rt_ins.insert(
                    "minecraftArguments".to_string(),
                    serde_json::Value::String(e.to_string()),
                );
            }
        }
        return_some(rt_ins)
    }
    /**
     * 从一个文件夹中根据suffix获取一个准确的文件。
     * 其中当suffix为“.json”的时候逻辑可能会略有不同，请不要在意！
     * suffix一般是以后缀为基础的。如果说不以后缀为基础，也可以用SHA1值做为基础。
     * 目前仅支持SHA1和后缀，如果不以这两个，则很可能会返回None
     */
    pub fn get_mc_real_path(version_path: String, suffix: &str) -> Option<String> {
        let path = std::path::Path::new(version_path.as_str());
        if path.exists() && path.is_dir() {
            let dir = walkdir::WalkDir::new(path).min_depth(1).max_depth(1);
            for i in dir.into_iter().filter_map(|e| e.ok()) {
                let pa = i.path();
                if pa.is_dir() {
                    continue;
                }
                let ps = pa.display().to_string();
                if ps.contains(suffix) {
                    return if suffix.eq(".json") {
                        let file_content = super::main_mod::get_file(ps.as_str());
                        if let None = file_content {
                            continue;
                        }
                        let root =
                            serde_json::from_str::<serde_json::Value>(file_content?.as_str());
                        if let Err(_) = root {
                            continue;
                        }
                        let root = root.unwrap();
                        let libr = root["libraries"].is_array();
                        if !libr {
                            continue;
                        }
                        let mics = root["mainClass"].is_string();
                        if !mics {
                            continue;
                        }
                        let idid = root["id"].is_string();
                        if !idid {
                            continue;
                        }
                        Some(ps)
                    } else {
                        Some(ps)
                    };
                } else if !suffix.contains(".") {
                    let sha = super::main_mod::get_sha1(ps.as_str());
                    if let None = sha {
                        continue;
                    }
                    if sha?.eq(suffix) {
                        return Some(ps);
                    }
                }
            }
        }
        None
    }
    /**
     * 判断参数，以及拼接参数！适用于在整合参数时。
     */
    pub fn judge_arguments(args_json: String, key: &str) -> Option<Vec<String>> {
        let root = serde_json::from_str::<serde_json::Value>(args_json.as_str()).ok()?;
        let argu = root["arguments"][key].as_array()?;
        let mut res: Vec<String> = Vec::new();
        for i in argu.into_iter() {
            let i_str = serde_json::to_string(i);
            if let Err(_) = i_str {
                continue;
            }
            let i_str = i_str.unwrap();
            if i_str.contains("rules") {
                continue;
            }
            let i_str = i.as_str();
            if let None = i_str {
                continue;
            }
            let i_str = i_str?.replace(" ", "");
            res.push(i_str.clone());
        }
        Some(res.clone())
    }

    /**
     * 单纯只是一个判断版本json里的libraries中，有rules的类库，是否allow在windows上。
     * 需要填入一个serde_json的对象Map值！而且该对象必须已经从rules中取了出来！
     */
    pub fn judge_mc_rules(root: &serde_json::Value) -> bool {
        let rules = root["rules"].as_array();
        if let None = rules {
            return true;
        }
        let rules = rules.unwrap();
        for i in rules.into_iter() {
            let action = i["action"].as_str();
            if let None = action {
                continue;
            }
            let action = action.unwrap();
            if action.eq("allow") {
                let os_name = i["os"]["name"].as_str();
                if let None = os_name {
                    continue;
                }
                let os_name = os_name.unwrap();
                if !os_name.eq("windows") {
                    return false;
                }
            } else if action.eq("disallow") {
                let os_name = i["os"]["name"].as_str();
                if let None = os_name {
                    continue;
                }
                let os_name = os_name.unwrap();
                if os_name.eq("windows") {
                    return false;
                }
            }
        }
        true
    }
    /**
     * 获取MC类库（GetCPLibraries）
     */
    pub fn get_mc_libs(raw_json: String, root_path: &str, version_path: &str) -> Option<String> {
        let mut res = String::new();
        let mut raw_list: Vec<String> = Vec::new();
        let mut no_list: Vec<String> = Vec::new();
        let mut no_low: Vec<String> = Vec::new();
        let mut temp_list: Vec<String> = Vec::new();
        let mut no_opt: Vec<String> = Vec::new();
        let root = serde_json::from_str::<serde_json::Value>(raw_json.as_str()).ok()?;
        let json_lib = root["libraries"].as_array()?;
        for i in json_lib.into_iter() {
            let name = i["name"].as_str();
            if let None = name {
                continue;
            }
            let expect_rule = judge_mc_rules(&i.clone());
            let mut expect_downloads = true;
            if let Some(e) = i.get("downloads") {
                if let Some(f) = e.get("classifiers") {
                    if let Some(_) = f.as_object() {
                        expect_downloads = false;
                        if let Some(g) = e.get("artifact") {
                            if let Some(_) = g.as_object() {
                                expect_downloads = true;
                            }
                        }
                    }
                }
            }
            if expect_rule && expect_downloads {
                raw_list.push(name?.to_string())
            }
        }
        for i in raw_list.into_iter() {
            if !no_list.contains(&i) {
                no_list.push(i);
            }
        }
        for i in no_list.into_iter() {
            let nocom = i
                .replace(".", "")
                .replace(":", "")
                .replace("-", "")
                .replace("/", "")
                .replace("@jar", "")
                .replace("@zip", "")
                .replace("@", "");
            let nonum = extract_number(nocom.clone(), false);
            let noword = extract_number(nocom.clone(), true);
            let toint = noword.parse::<u64>();
            if let Err(_) = toint {
                continue;
            }
            let toint = toint.unwrap();
            if !temp_list.contains(&nonum) {
                temp_list.push(nonum);
                no_low.push(i);
            } else {
                let temp_1 = temp_list.iter().position(|x| x == &nonum);
                if let None = temp_1 {
                    continue;
                }
                let temp_2 = no_low.get(temp_1?);
                if let None = temp_2 {
                    continue;
                }
                let temp_2 = extract_number(temp_2?.to_string(), true);
                let temp_3 = temp_2.parse::<u64>();
                if let Err(_) = temp_3 {
                    continue;
                }
                let temp_3 = temp_3.unwrap();
                if temp_3 < toint {
                    no_low.remove(temp_1?);
                    no_low.insert(temp_1?, i);
                }
            }
        }
        //dim you! optifine!
        let mut temp: Vec<String> = Vec::new();
        for i in no_low.into_iter() {
            if i.contains("optifine") {
                temp.push(i.clone());
            } else {
                no_opt.push(i.clone());
            }
        }
        if !temp.is_empty() {
            no_opt.extend(temp.clone());
        }
        //end you! optifine!
        for i in no_opt.into_iter() {
            if let Some(e) = convert_name_to_path(i) {
                res.push_str(
                    format!(
                        "{}\\libraries\\{}{}",
                        root_path, e, "${classpath_separator}"
                    )
                    .as_str(),
                );
            }
        }
        let mut inh = get_mc_inherits_from(version_path.to_string(), "jar")?;
        if inh.eq(version_path) {
            let inhj = get_mc_inherits_from(version_path.to_string(), "inheritsFrom")?;
            inh = inhj.clone();
        }
        let sha = root
            .get("downloads")?
            .get("client")?
            .get("sha1")?
            .as_str()?;
        let tmp = get_mc_real_path(inh, sha);
        if let Some(e) = tmp {
            res.push_str(e.as_str());
        } else {
            res = res[0..res.rfind("$")?].to_string();
        }
        Some(res)
    }
    /**
     * 解压natives。填入原json和根路径和版本路径。解压成功返回true，否则返回false。
     */
    pub fn unzip_native(raw_json: String, root_path: &str, version_path: &str) -> bool {
        let mut raw_list: Vec<String> = Vec::new();
        let mut no_list: Vec<String> = Vec::new();
        let mut no_low: Vec<String> = Vec::new();
        let mut temp_list: Vec<String> = Vec::new();
        let root = serde_json::from_str::<serde_json::Value>(raw_json.as_str());
        if let Err(_) = root {
            return false;
        }
        let root = root.unwrap();
        let json_lib = root["libraries"].as_array();
        if let None = json_lib {
            return false;
        }
        let json_lib = json_lib.unwrap();
        for i in json_lib.into_iter() {
            let expect_rule = judge_mc_rules(&i.clone());
            let lib_name = i["name"].as_str();
            if let None = lib_name {
                continue;
            }
            let lib_name = lib_name.unwrap();
            let lib_arch = i["natives"]["windows"].as_str();
            if let None = lib_arch {
                continue;
            }
            let lib_arch = lib_arch.unwrap();
            if expect_rule {
                raw_list.push(format!("{}:{}", lib_name, lib_arch))
            }
        }
        for i in raw_list.into_iter() {
            if !no_list.contains(&i) {
                no_list.push(i);
            }
        }
        for i in no_list.into_iter() {
            let nocom = i
                .replace(".", "")
                .replace(":", "")
                .replace("-", "")
                .replace("/", "")
                .replace("@jar", "")
                .replace("@zip", "")
                .replace("@", "");
            let nonum = extract_number(nocom.clone(), false);
            let noword = extract_number(nocom.clone(), true);
            let toint = noword.parse::<u64>();
            if let Err(_) = toint {
                continue;
            }
            let toint = toint.unwrap();
            if !temp_list.contains(&nonum) {
                temp_list.push(nonum);
                no_low.push(i);
            } else {
                let temp_1 = temp_list.iter().position(|x| x == &nonum);
                if let None = temp_1 {
                    continue;
                }
                let temp_1 = temp_1.unwrap();
                let temp_2 = no_low.get(temp_1);
                if let None = temp_2 {
                    continue;
                }
                let temp_2 = extract_number(temp_2.unwrap().to_string(), true);
                let temp_3 = temp_2.parse::<u64>();
                if let Err(_) = temp_3 {
                    continue;
                }
                let temp_3 = temp_3.unwrap();
                if temp_3 < toint {
                    no_low.remove(temp_1);
                    no_low.insert(temp_1, i);
                }
            }
        }
        let dir = format!(
            "{}\\{}-{}-natives",
            version_path,
            super::main_mod::extract_file_name(version_path),
            super::some_const::LAUNCHER_NANE
        );
        let ver_file = std::path::Path::new(dir.as_str());
        if !ver_file.exists() || (ver_file.exists() && ver_file.is_file()) {
            let cf = std::fs::create_dir_all(ver_file);
            if let Err(_) = cf {
                return false;
            }
        } else {
            return true;
        }
        if no_low.len() == 0 {
            true
        } else {
            for c in no_low.into_iter() {
                let cvn = convert_name_to_path(c);
                if let None = cvn {
                    continue;
                }
                let cvn = cvn.unwrap();
                let rpath = format!("{}\\libraries\\{}", root_path, cvn);
                let uzp = unzip(rpath.clone(), dir.clone());
                if !uzp {
                    continue;
                }
            }
            delete_file_keep(dir, ".dll")
        }
    }
    /**
     * 自定义启动设置类，各位可以调用！
     * 其中，你需要保证以下两点最重要：
     * root_path里面包含【assets、libraries】两个文件夹
     * version_path里面包含【版本.json、版本.jar】两个文件
     * 后期解压Native是默认解压到version_path路径下的！
     * @param account: 账号类，参见LaunchAccount。
     * @param java_path: Java路径
     * @param root_path: MC根路径（用于查询assets、libraries）
     * @param version_path: MC版本路径（用于查询MC元数据JSON和本体jar）
     * @param game_path: MC游戏文件夹（直接用于存储游戏目录）
     * @param window_height: 游戏窗口高度
     * @param window_width: 游戏窗口宽度
     * @param max_memory: 游戏最大内存
     * @param custom_info: 游戏自定义信息（显示在游戏标题界面的右下角和游戏内f3的基本信息。）
     * @param additional_jvm: 游戏额外JVM参数
     * @param additional_game: 游戏额外game参数
     * @param pre_launch_script: 启动前执行脚本
     */
    #[derive(Clone)]
    pub struct LaunchOption {
        account: LaunchAccount,
        java_path: String,
        root_path: String,
        version_path: String,
        game_path: String,
        window_height: usize,
        window_width: usize,
        min_memory: usize,
        max_memory: usize,
        custom_info: String,
        additional_jvm: String,
        additional_game: String,
    }
    impl LaunchOption {
        pub fn new(
            account: LaunchAccount,
            java_path: &str,
            root_path: &str,
            version_path: &str,
            game_path: &str,
        ) -> Self {
            Self {
                account,
                java_path: java_path.to_string(),
                root_path: root_path.to_string(),
                version_path: version_path.to_string(),
                game_path: game_path.to_string(),
                window_height: 480,
                window_width: 854,
                min_memory: 256,
                max_memory: 4096,
                custom_info: format!(
                    "{}-{}",
                    super::some_const::LAUNCHER_NANE,
                    super::some_const::LAUNCHER_VERSION
                ),
                additional_jvm: String::new(),
                additional_game: String::new(),
            }
        }
        pub fn set_window_height(&mut self, window_height: usize) {
            self.window_height = window_height;
        }
        pub fn set_window_width(&mut self, window_width: usize) {
            self.window_width = window_width;
        }
        pub fn set_min_memory(&mut self, min_memory: usize) {
            self.min_memory = min_memory;
        }
        pub fn set_max_memory(&mut self, max_memory: usize) {
            self.max_memory = max_memory;
        }
        pub fn set_custom_info(&mut self, custom_info: &str) {
            self.custom_info = custom_info.to_string();
        }
        pub fn set_additional_jvm(&mut self, additional_jvm: &str) {
            self.additional_jvm = additional_jvm.to_string();
        }
        pub fn set_additional_game(&mut self, additional_game: &str) {
            self.additional_game = additional_game.to_string();
        }
        pub fn get_account(&self) -> LaunchAccount {
            self.account.clone()
        }
        pub fn get_java_path(&self) -> &str {
            self.java_path.as_str()
        }
        pub fn get_root_path(&self) -> &str {
            self.root_path.as_str()
        }
        pub fn get_version_path(&self) -> &str {
            self.version_path.as_str()
        }
        pub fn get_game_path(&self) -> &str {
            self.game_path.as_str()
        }
        pub fn get_window_height(&self) -> usize {
            self.window_height
        }
        pub fn get_window_width(&self) -> usize {
            self.window_width
        }
        pub fn get_min_memory(&self) -> usize {
            self.min_memory
        }
        pub fn get_max_memory(&self) -> usize {
            self.max_memory
        }
        pub fn get_custom_info(&self) -> &str {
            self.custom_info.as_str()
        }
        pub fn get_additional_jvm(&self) -> &str {
            self.additional_jvm.as_str()
        }
        pub fn get_additional_game(&self) -> &str {
            self.additional_game.as_str()
        }
    }
    use super::some_const::*;
    struct LaunchGame {
        account: LaunchAccount,
        java_path: String,
        root_path: String,
        version_path: String,
        game_path: String,
        window_height: usize,
        window_width: usize,
        min_memory: usize,
        max_memory: usize,
        custom_info: String,
        additional_jvm: String,
        additional_game: String,
        callback: Box<dyn Fn(Vec<&str>)>,
    }
    /// 启动游戏的私有实现类，如果想要调用的话，请直接使用下方的launch_game函数。
    /// 如果你想自己实现启动逻辑，可以看下面启动游戏的逻辑，然后调用相对应的函数。因为除了该私有实现以外，别的函数都是pub的！
    impl LaunchGame {
        fn new<F>(option: LaunchOption, callback: F) -> Self
        where
            F: Fn(Vec<&str>) + 'static,
        {
            Self {
                account: option.get_account(),
                java_path: option.get_java_path().to_string(),
                root_path: option.get_root_path().to_string(),
                version_path: option.get_version_path().to_string(),
                game_path: option.get_game_path().to_string(),
                window_height: option.get_window_height(),
                window_width: option.get_window_width(),
                min_memory: option.get_min_memory(),
                max_memory: option.get_max_memory(),
                custom_info: option.get_custom_info().to_string(),
                additional_jvm: option.get_additional_jvm().to_string(),
                additional_game: option.get_additional_game().to_string(),
                callback: Box::new(callback),
            }
        }
        /// 启动游戏的私有函数，此处为检查是否有错。
        fn check_error(&self) -> i32 {
            let event_loop = winit::event_loop::EventLoop::new();
            let monitor = event_loop.available_monitors().next();
            if let None = monitor {
                return ERR_UNKNOWN_ERROR;
            }
            let monitor = monitor.unwrap();
            let window_size = monitor.size();
            let mut sys = sysinfo::System::new_all();
            sys.refresh_all();
            let mem = (sys.total_memory() as f64 / 1024.0 / 1024.0).ceil() as i32;
            if self.account.get_online() == 0 {
                let regu = regex::Regex::new("^[0-9a-f]{32}$").unwrap();
                if !regu.is_match(self.account.get_uuid()) {
                    return ERR_LAUNCH_ACCOUNT_USERUUID;
                }
                let regn = regex::Regex::new("^[a-zA-Z0-9]{3,16}$").unwrap();
                if !regn.is_match(self.account.get_name()) {
                    return ERR_LAUNCH_ACCOUNT_USERNAME;
                }
            } else if self.account.get_online() == 1 {
                let um = super::account_mod::UrlMethod::new(
                    "https://api.minecraftservices.com/minecraft/profile",
                );
                let ih = um.get(self.account.get_access_token());
                if let None = ih {
                    return ERR_LAUNCH_ACCOUNT_ACCESS_TOKEN;
                }
                let json = serde_json::from_str::<serde_json::Value>(
                    ih.unwrap().replace("\\/", "/").as_str(),
                );
                if let Err(_) = json {
                    return ERR_UNKNOWN_ERROR;
                }
                let json = json.unwrap();
                let name = json["name"].as_str();
                if let None = name {
                    return ERR_LAUNCH_ACCOUNT_NO_LEGAL;
                }
                let name = name.unwrap();
                let uuid = json["id"].as_str();
                if let None = uuid {
                    return ERR_LAUNCH_ACCOUNT_NO_LEGAL;
                }
                let uuid = uuid.unwrap();
                if name != self.account.get_name() && uuid != self.account.get_name() {
                    return ERR_LAUNCH_ACCOUNT_ACCESS_TOKEN;
                }
            } else if self.account.get_online() == 2 {
                if self.account.get_base().is_empty()
                    || !regex::Regex::new(
                        r"^([A-Za-z0-9+/]{4})*([A-Za-z0-9+/]{3}=|[A-Za-z0-9+/]{2}==)?$",
                    )
                    .unwrap()
                    .is_match(self.account.get_base())
                {
                    return ERR_LAUNCH_ACCOUNT_THIRDPARTY_BASE;
                }
                let t = format!("{}/authserver/validate", self.account.get_url());
                let pl = format!(
                    "{}{}{}",
                    "{\"accesstoken\":\"",
                    self.account.get_access_token(),
                    "\"}"
                );
                let po = super::account_mod::UrlMethod::new(t.as_str());
                let pl = po.post(pl.as_str(), true);
                let ap = super::some_var::AUTHLIB_PATH.with_borrow(|e| e.clone());
                let ap = std::path::Path::new(ap.as_str());
                if !ap.exists() || ap.is_dir() {
                    return ERR_LAUNCH_ACCOUNT_AUTHLIB;
                }
                if let None = pl {
                    return ERR_LAUNCH_ACCOUNT_THIRDPARTY_ACCESS_TOKEN_OR_URL;
                }
            }
            let jpath = std::path::Path::new(self.java_path.as_str());
            if !jpath.exists() || !jpath.is_file() {
                return ERR_LAUNCH_JAVA_PATH;
            }
            let rpath = std::path::Path::new(self.root_path.as_str());
            if !rpath.exists() || !rpath.is_dir() {
                return ERR_LAUNCH_ROOT_PATH;
            }
            let vpath = std::path::Path::new(self.version_path.as_str());
            if !vpath.exists() || !vpath.is_dir() {
                return ERR_LAUNCH_VERSION_PATH;
            }
            let gpath = std::path::Path::new(self.game_path.as_str());
            if !gpath.exists() || !gpath.is_dir() {
                return ERR_LAUNCH_GAME_PATH;
            }
            if self.window_width < 854 || self.window_width > (window_size.width as usize) {
                return ERR_LAUNCH_WIDTH;
            }
            if self.window_height < 480 || self.window_height > (window_size.height as usize) {
                return ERR_LAUNCH_HEIGHT;
            }
            if self.min_memory > 1024 || self.min_memory < 256 {
                return ERR_LAUNCH_MIN_MEMORY;
            }
            if self.max_memory < 1024 || self.max_memory > (mem as usize) {
                return ERR_LAUNCH_MAX_MEMORY;
            }
            if self.custom_info == "" {
                return ERR_LAUNCH_CUSTOM_INFO;
            }
            OK
        }
        /// 拼接全局参数
        fn put_arguments(
            &self,
            real_json: String,
            def_jvm: String,
            defn_jvm: String,
        ) -> Result<Vec<String>, i32> {
            let root = serde_json::from_str::<serde_json::Value>(real_json.as_str())
                .map_err(|_| ERR_GAME_RAW_JSON_STRUCTURE)?;
            let mcid = root["id"].as_str().ok_or(ERR_GAME_RAW_JSON_STRUCTURE)?;
            let main_class = root["mainClass"]
                .as_str()
                .ok_or(ERR_GAME_RAW_JSON_STRUCTURE)?;
            let asset_index = root["assetIndex"]["id"]
                .as_str()
                .ok_or(ERR_GAME_RAW_JSON_STRUCTURE)?;
            let mut result: Vec<String> = Vec::new();
            let def_jvm: Vec<String> = def_jvm
                .split_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .map(|e| String::from(*e))
                .collect();
            let defn_jvm: Vec<String> = defn_jvm
                .split_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .map(|e| String::from(*e))
                .collect();
            result.extend(def_jvm.clone());
            result.extend(defn_jvm.clone());
            if !self.additional_jvm.is_empty() {
                let add_jvm: Vec<String> = self
                    .additional_jvm
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|e| String::from(*e))
                    .collect();
                result.extend(add_jvm.clone());
            }
            let judge_argu = judge_arguments(real_json.clone(), "jvm");
            if let Some(e) = judge_argu {
                result.extend(e.clone());
            } else {
                result.push(String::from("-Djava.library.path=${natives_directory}"));
                result.push(String::from("-cp"));
                result.push(String::from("${classpath}"));
            }
            if !self.account.get_base().is_empty() {
                let ap = super::some_var::AUTHLIB_PATH.with_borrow(|e| e.clone());
                if ap.is_empty() {
                    panic!("You're not assign the AUTHLIB_PATH in some_var mod, please retry!")
                }
                let path = std::path::Path::new(ap.as_str());
                if path.exists() && path.is_file() {
                    result.push(format!("-javaagent:{}={}", ap, self.account.get_url()));
                    result.push("-Dauthlibinjector.side=client".to_string());
                    result.push(format!(
                        "-Dauthlibinjector.yggdrasil.prefetched={}",
                        self.account.get_base()
                    ));
                } else {
                    panic!("You're AUTHLIB_PATH file is not exist, please retry!")
                }
            }
            result.push(format!("-Xmn{}m", self.min_memory));
            result.push(format!("-Xmx{}m", self.max_memory));
            result.push(main_class.to_string());
            let mcag = root.get("minecraftArguments");
            if let Some(judge_game) = mcag {
                let judge_game = judge_game.as_str();
                if let Some(judge_game) = judge_game {
                    if !judge_game.is_empty() {
                        let judge_game: Vec<String> = judge_game
                            .split_whitespace()
                            .collect::<Vec<&str>>()
                            .iter()
                            .map(|e| String::from(*e))
                            .collect();
                        result.extend(judge_game);
                    }
                }
                if let Some(judge_game) = judge_arguments(real_json.clone(), "game") {
                    result.extend(judge_game);
                }
            } else {
                result.extend(
                    judge_arguments(real_json.clone(), "game")
                        .ok_or(ERR_GAME_RAW_JSON_STRUCTURE)?,
                );
            }
            if !self.additional_game.contains("--fullScreen") {
                result.push("--width".to_string());
                result.push(self.window_width.to_string());
                result.push("--height".to_string());
                result.push(self.window_height.to_string());
            }
            if !self.additional_game.is_empty() {
                let add_game: Vec<String> = self
                    .additional_game
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|e| String::from(*e))
                    .collect();
                result.extend(add_game.clone());
            }
            if result.contains(&"optifine.OptiFineForgeTweaker".to_string()) {
                let temp_1 = result
                    .iter()
                    .position(|x| x.eq("optifine.OptiFineForgeTweaker"))
                    .unwrap();
                result.remove(temp_1 - 1);
                result.remove(temp_1 - 1);
                result.push("--tweakClass".to_string());
                result.push("optifine.OptiFineForgeTweaker".to_string());
            }
            if result.contains(&"optifine.OptiFineTweaker".to_string()) {
                let temp_1 = result
                    .iter()
                    .position(|x| x.eq("optifine.OptiFineTweaker"))
                    .unwrap();
                result.remove(temp_1 - 1);
                result.remove(temp_1 - 1);
                result.push("--tweakClass".to_string());
                result.push("optifine.OptiFineTweaker".to_string());
            }
            let libs = get_mc_libs(
                real_json.clone(),
                self.root_path.as_str(),
                self.version_path.as_str(),
            )
            .ok_or(ERR_GAME_RAW_JSON_STRUCTURE)?;
            for i in result.iter_mut() {
                *i = i
                    .replace(
                        "${natives_directory}",
                        format!(
                            "{}\\{}-{}-natives",
                            self.version_path,
                            super::main_mod::extract_file_name(self.version_path.as_str()),
                            super::some_const::LAUNCHER_NANE
                        )
                        .as_str(),
                    )
                    .replace("${launcher_name}", super::some_const::LAUNCHER_NANE)
                    .replace(
                        "${launcher_version}",
                        super::some_const::LAUNCHER_VERSION
                            .replace(".", "")
                            .replace("-", "")
                            .replace("Alpha", "")
                            .replace("Beta", "")
                            .replace("Pre", "")
                            .as_str(),
                    )
                    .replace("${classpath}", libs.as_str())
                    .replace("${version_name}", mcid)
                    .replace(
                        "${library_directory}",
                        format!("{}\\libraries", self.root_path).as_str(),
                    )
                    .replace("${auth_player_name}", self.account.get_name())
                    .replace("${game_directory}", format!("{}", self.game_path).as_str())
                    .replace(
                        "${assets_root}",
                        format!("{}\\assets", self.root_path).as_str(),
                    )
                    .replace("${assets_index_name}", asset_index)
                    .replace("${auth_uuid}", self.account.get_uuid())
                    .replace("${uuid}", self.account.get_uuid())
                    .replace("${auth_access_token}", self.account.get_access_token())
                    .replace("${user_type}", self.account.get_atype())
                    .replace("${version_type}", format!("{}", self.custom_info).as_str())
                    .replace("${auth_session}", self.account.get_uuid())
                    .replace(
                        "${game_assets}",
                        format!("{}\\assets\\virtual\\legacy", self.root_path).as_str(),
                    )
                    .replace("${user_properties}", "{}")
                    .replace("${classpath_separator}", ";"); //MacOS 是 冒号【:】
            }
            Ok(result)
        }
        /// 如果没有错误，则会调用该函数。如果启动过程中出现不可预知的错误，则会直接panic掉！
        fn game_launch(&self) -> i32 {
            let def_jvm: String = String::from("-XX:+UseG1GC -XX:-UseAdaptiveSizePolicy -XX:-OmitStackTraceInFastThrow -Dfml.ignoreInvalidMinecraftCertificates=true -Dfml.ignorePatchDiscrepancies=true -Dlog4j2.formatMsgNoLookups=true");
            let defn_jvm: String = String::from("-XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump");
            let version_json_path = get_mc_real_path(self.version_path.clone(), ".json");
            if let None = version_json_path {
                return ERR_GAME_ELIGIBLE_JSON_NOT_FOUND;
            }
            let version_json_path = version_json_path.unwrap();
            let final_json: String;
            let raw_json = super::main_mod::get_file(version_json_path.as_str());
            if let None = raw_json {
                return ERR_GAME_ELIGIBLE_JSON_NOT_FOUND;
            }
            let raw_json = raw_json.unwrap();
            let inherits_json = get_mc_inherits_from(self.version_path.clone(), "inheritsFrom");
            if let None = inherits_json {
                return ERR_GAME_INHERITS_FROM_VERSION_LOSE;
            }
            let inherits_json = inherits_json.unwrap();
            if !inherits_json.eq(self.version_path.as_str()) {
                let file = get_mc_real_path(inherits_json, ".json");
                if let None = file {
                    return ERR_GAME_INHERITS_FROM_VERSION_LOSE;
                }
                let file = file.unwrap();
                let f_json = super::main_mod::get_file(file.as_str());
                if let None = f_json {
                    return ERR_GAME_INHERITS_FROM_VERSION_LOSE;
                }
                final_json = f_json.unwrap();
            } else {
                final_json = raw_json.clone();
            }
            if !unzip_native(
                final_json.clone(),
                self.root_path.as_str(),
                self.version_path.as_str(),
            ) {
                return ERR_GAME_CANNOT_UNZIP_NATIVE;
            }
            let real_json = replace_mc_inherits_from(raw_json, final_json);
            if let None = real_json {
                return ERR_GAME_INHERITS_JSON_STRUCTURE;
            }
            let real_json = real_json.unwrap();
            let param = self.put_arguments(real_json.clone(), def_jvm.clone(), defn_jvm.clone());
            if let Err(e) = param {
                return e;
            }
            let mut param = param.unwrap();
            param.splice(0..0, [self.java_path.clone()]);
            let command = param.iter().map(AsRef::as_ref).collect();
            (self.callback)(command);
            super::some_const::OK
            // return param.unwrap();
        }
    }
    /// 提供了Account登录的启动类模块，该类不是用来登录账号的，只是用来启动游戏时才用到的！
    /// @function new_offline: 为新建了一个离线登录。如果你身处除中国以外的地方，请不要使用该新建函数。
    /// @function new_offline_default: 为新建一个默认的玩家，仅需输入玩家名称，使用bukkit方式生成一个UUID。
    /// @function new_microsoft为新建了一个微软登录。该登录方式适用于全世界。
    /// @function new_thirdparty为新建了一个第三方登录。除非你信任该模块地址，否则你不能使用该新建函数。
    /// @function new_thirdparty_default为新建了一个第三方登录。并且无需填入元数据，仅需多填入一个第三方登录网址。
    /// @param name: 玩家登录名称
    /// @param uuid: 玩家登录UUID（需要符合32位16进制字符）
    /// @param access_token: 登录密钥（仅在使用微软、第三方时才用到。）
    /// @param atype: 登录类型，该参数无需自己填
    /// @param url: 第三方登录网址，该参数填入你的第三方登录域名。
    /// @param base: 第三方登录元数据base64编码方案，如果你想使用第三方快速启动，
    /// @param online: 仅用于标记目前你使用的哪种方式登录，不作为默认参数提供。
    ///
    ///
    /// 离线模式调用示例：LaunchAccount::new_offline("Steve", "1234567890abcdef1234567890abcdef");
    /// 或：LaunchAccount::new_offline_default("Steve");  // UUID会自动按照bukkit方式生成。
    /// 微软登录调用示例：LaunchAccount::new_microsoft("Steve", "1234567890abcdef1234567890abcdef", "<你的access token密钥>")
    /// 第三方外置登录调用示例：LaunchAccount::new_thirdparty(
    ///                      "Steve",
    ///                      "1234567890abcdef1234567890abcdef",
    ///                      "<你的access token密钥>",
    ///                      "<你的皮肤站元数据base64编码>",
    ///                      "https://littleskin.cn/api/yggdrasil"")  # 皮肤站元数据必须得是精确到api/yggdrasil的！
    /// 或：LaunchAccount::new_thirdparty(
    ///                      "Steve",
    ///                      "1234567890abcdef1234567890abcdef",
    ///                      "<你的access token密钥>",
    ///                      "https://littleskin.cn/api/yggdrasil"")  # 此时皮肤站元数据base64编码会自动从api密钥获取。
    #[derive(Clone)]
    pub struct LaunchAccount {
        name: String,
        uuid: String,
        access_token: String,
        atype: String,
        base: String,
        url: String,
        online: i32,
    }
    impl LaunchAccount {
        fn new(
            name: String,
            uuid: String,
            access_token: String,
            atype: String,
            base: String,
            url: String,
            online: i32,
        ) -> Self {
            Self {
                name: name.clone(),
                uuid: uuid.clone(),
                access_token: access_token.clone(),
                atype: atype.clone(),
                base: base.clone(),
                url: url.clone(),
                online,
            }
        }
        pub fn new_offline(name: &str, uuid: &str) -> Self {
            LaunchAccount::new(
                name.to_string(),
                uuid.to_string(),
                uuid.to_string(),
                String::from("Legacy"),
                String::new(),
                String::new(),
                0,
            )
        }
        #[allow(dead_code, deprecated)]
        #[deprecated(
            since = "0.0.8",
            note = "Please use main_mod generate_bukkit_uuid function."
        )]
        pub fn new_offline_default(name: &str) -> Self {
            let uuid = super::main_mod::generate_bukkit_uuid(name);
            LaunchAccount::new(
                name.to_string(),
                uuid.clone(),
                uuid.clone(),
                String::from("Legacy"),
                String::new(),
                String::new(),
                0,
            )
        }
        pub fn new_microsoft(name: &str, uuid: &str, access_token: &str) -> Self {
            LaunchAccount::new(
                name.to_string(),
                uuid.to_string(),
                access_token.to_string(),
                String::from("msa"),
                String::new(),
                String::new(),
                1,
            )
        }
        pub fn new_thirdparty(
            name: &str,
            uuid: &str,
            access_token: &str,
            base: &str,
            url: &str,
        ) -> Self {
            LaunchAccount::new(
                name.to_string(),
                uuid.to_string(),
                access_token.to_string(),
                String::from("msa"),
                base.to_string(),
                url.to_string(),
                2,
            )
        }
        #[allow(dead_code, deprecated)]
        #[deprecated(
            since = "0.0.8",
            note = "Please login thirdparty in account_mod, and auto get base64 code by sync."
        )]
        pub fn new_thirdparty_default(
            name: &str,
            uuid: &str,
            access_token: &str,
            url: &str,
        ) -> Self {
            LaunchAccount::new(
                name.to_string(),
                uuid.to_string(),
                access_token.to_string(),
                String::from("msa"),
                super::main_mod::generate_thirdparty_metadata_base64(url),
                url.to_string(),
                2,
            )
        }
        pub fn get_name(&self) -> &str {
            self.name.as_str()
        }
        pub fn get_uuid(&self) -> &str {
            self.uuid.as_str()
        }
        pub fn get_access_token(&self) -> &str {
            self.access_token.as_str()
        }
        pub fn get_atype(&self) -> &str {
            self.atype.as_str()
        }
        pub fn get_base(&self) -> &str {
            self.base.as_str()
        }
        pub fn get_url(&self) -> &str {
            self.url.as_str()
        }
        fn get_online(&self) -> i32 {
            self.online
        }
    }
    /// 该函数为启动游戏的函数，接受一个LaunchOption函数和一个闭包。
    /// 其中，闭包用于获取启动参数。
    pub fn launch_game<F>(option: LaunchOption, callback: F) -> Result<(), i32>
    where
        F: Fn(Vec<&str>) + 'static,
    {
        let res = LaunchGame::new(option, callback);
        let code = res.check_error();
        if code != 0 {
            Err(code)
        } else {
            let code = res.game_launch();
            if code != 0 {
                Err(code)
            } else {
                Ok(())
            }
        }
    }
}
