#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(clippy::all, clippy::pedantic, clippy::nursery)]

cfg_if::cfg_if! {
    if #[cfg(all(target_os = "linux", target_arch = "x86_64"))] {
        include!("linux_x64.rs");
    } else if #[cfg(all(target_os = "windows", target_arch = "x86_64"))] {
        include!("windows_x64.rs");
    } else{
        std::compile_error!("pre-generated bindings are not avaliable for your target");
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_fibonacci() {
        let  num_devs: u32 = create_device_info_list().unwrap();
        for i_dev in 0..num_devs {
            let (_,devType,_,_,_,desc,_) = get_device_info_detail(i_dev).unwrap();
            if(devType == FT_DEVICE::FT_DEVICE_4222H_0 ||
                devType == FT_DEVICE::FT_DEVICE_4222H_1_2 ||
                devType == FT_DEVICE::FT_DEVICE_4222H_3) {
                println!("FT_DEVICE_4222H_0: {}", desc);
            }
        }



    }
}

