use std::os::raw::c_void;
use std::net::UdpSocket;


#[link(name="pcre2-8")]
extern "C" {
    // 参考demo用到的函数
    fn pcre2_compile_8(pattern: *const u8, length: usize, options: u32, errorcode: *mut i32, erroroffset: *mut usize, ccontext: *mut c_void) -> *mut c_void;

    fn pcre2_match_data_create_from_pattern_8(code: *const c_void, gcontext: *mut c_void) -> *mut c_void;

    fn pcre2_match_8(code: *const c_void, subject: *const u8, length: usize, startoffset: usize, options: u32, match_data: *mut c_void, mcontext: *mut c_void) -> i32;

    fn pcre2_get_ovector_pointer_8(match_data: *mut c_void) -> *mut usize;
}

// todo
// pcre2-16
// pcre2-32


// 仿照demo流程写法
// pcre2demo man page： https://pcre2project.github.io/pcre2/doc/html/pcre2demo.html
//  流程：
//       1. pcre2_compile() 
//       2. match_data = pcre2_match_data_create_from_pattern(re, NULL);
//       3. pcre2_match()
//       4. ovector = pcre2_get_ovector_pointer(match_data);
//       5. ...
// find 根据正则从目标字符串中提取一个子串
// args: 
//     - pattern   正则表达式
//     - subject   目标字符串
// return:
//     - vec<string> 匹配结果
pub fn find(pattern: &str, subject: &str) -> Vec<String>{
    // 编译正则比表达式
    let mut errorcode: i32 = 0;
    let mut erroroffset: usize = 0;
    
    let re = unsafe {
        pcre2_compile_8(pattern.as_ptr(), pattern.len(), 0, &mut errorcode, &mut erroroffset, std::ptr::null_mut())
    };

    if re.is_null() {
        panic!("pcre2_compile_8 error");
    }

    let match_data = unsafe {
        pcre2_match_data_create_from_pattern_8(re, std::ptr::null_mut())
    };


    let rc = unsafe {
        pcre2_match_8(re, subject.as_ptr(), subject.len(), 0, 0, match_data, std::ptr::null_mut())
    };

    if rc < 0 {
        panic!("pcre2_match_8 error");
    } 


    // 取存放索引的vector
    let ovector = unsafe {
        pcre2_get_ovector_pointer_8(match_data)
    };
    
    if ovector.is_null() {
        panic!("pcre2_get_ovector_pointer_8 error");
    }

    // 处理结果
    // 
    // C 源码： 
    // for (i = 0; i < rc; i++)
    // {
    // PCRE2_SPTR substring_start = subject + ovector[2*i];
    // PCRE2_SIZE substring_length = ovector[2*i+1] - ovector[2*i];
    // printf("%2d: %.*s\n", i, (int)substring_length, (char *)substring_start);
    // }
    let mut match_result = Vec::<String>::new();
    
    for i in 0..rc {
        let begin = unsafe { *ovector.offset(i as isize * 2) };
        let end = unsafe { *ovector.offset(i as isize * 2 + 1) };
        println!("boxi2: {}: {} - {}", i, begin, end);

        match_result.push(subject[begin + 4..end].to_string());
    }

    println!("match resutl: {:?}", match_result);

    match_result
}




// 发送匹配结果
pub fn send_by_udp(from: &str, to: &str, data: Vec<String>) {
    if data.len() == 0 {
        return
    }
    // send result by socket udp 
    let udpocket = UdpSocket::bind(from).expect("couldn't bind to address");   
    for i in data.iter() {
        udpocket.send_to(i.as_bytes(), to).expect("couldn't send data");
    } 
}


#[cfg(test)]
mod tests{
    use super::{find};

    #[test]
    fn test_find() {
        let pattern = r"\d{4}[^\d\s]{3,11}";
        let target = "a;jhgoqoghqoj0329 u0tyu10hg0h9Y0Y9827342482y(Y0y(G)_)lajf;lqjfgqhgpqjopjqa=)*(^!@#$%^&*())9999999";
        find(pattern, target);
    }
}