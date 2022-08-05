use std::ptr;
use pcre2call::*;
use std::net::UdpSocket;


fn process_pcre2(sample: String, pattern: String) -> Vec<String> {
    let mut num_buf = Vec::<String>::new();
    let mut sent_buf = Vec::<String>::new();
    let mut valid_buf = Vec::<String>::new();
    let mut start_offset = 0;

    let (mut error_code, mut error_offset) = (0, 0);

    unsafe {
        let pcre_code: *mut pcre2_code_8 = pcre2_compile_8(pattern.as_ptr(), pattern.len(),
                                            0, &mut error_code, &mut error_offset, ptr::null_mut());

        let match_data: *mut pcre2_match_data_8 = pcre2_match_data_create_from_pattern_8(pcre_code, ptr::null_mut());

        //依次查找4个数字的字符
        loop {
            let rc = pcre2_match_8(pcre_code, sample.as_ptr(), sample.len(), start_offset, 
                            0, match_data, ptr::null_mut()); 

            if rc <= 0 {
                break;
            }

            let pgo_vec = pcre2_get_ovector_pointer_8(match_data);

            for i in 0..rc {
                let (st, end) = (*pgo_vec.offset(i.try_into().unwrap()), 
                                 *pgo_vec.offset((i+1).try_into().unwrap() ) );

                //左侧相邻的字符串4个数字
                num_buf.push(sample[st..end].to_string());

                start_offset = end as usize;
            }
        }

        println!("num_buf len{}, num_buf {:?}", num_buf.len(), num_buf);

        //从4个数字位置开始往后找
        let mut i = 0;
        loop {

            let fl_start = sample.find(&num_buf[i]).unwrap();

            let mut j = 0;
            loop {
                if sample.get(fl_start+num_buf[i].len()..fl_start+num_buf[i].len()+j).is_none() 
                {
                    break;
                }

                j += 1;

                //结果字符3到11位
                if j>10 {
                    break;
                }
            }

            i += 1;
            if i == num_buf.len() 
            {
                break;
            }

            sent_buf.push( sample[fl_start+num_buf[i-1].len()..fl_start+num_buf[i-1].len()+j].to_string() );
        }
        
        for i in sent_buf.clone() 
        {
            let mut valid_nums = 0;

            for j in i.as_bytes() 
            {
                //结果字符串不包含数字
                if j>=&48 && j<=&57 
                {
                    break;
                }
                valid_nums += 1;
            }

            //结果字符串
            if valid_nums >= 3 
            {
                valid_buf.push(i.get(..valid_nums).unwrap().to_string());
            }
        }
    }

    return valid_buf;
}


fn main() {
    //目标文本
    let sample= "a;jhgoqoghqoj0329 u0tyu10hg0h9Y0Y9827342482y(Y0y(G)_)lajf;lqjfgqhgpqjopjqa=)*(^!@#$%^&*())9999999";
    //正则查找4位以上数字
    let pattern = "\\d{4,}";

    let valid_buf = process_pcre2(sample.to_string(), pattern.to_string());

    // send result by socket udp 
    let socketid = UdpSocket::bind("127.0.0.1:37779").expect("couldn't bind to address");   
    for i in valid_buf {
        println!("send {:?}", i);
        socketid.send_to(i.as_bytes(), "127.0.0.1:37778").expect("couldn't send data");
    }
    
}
