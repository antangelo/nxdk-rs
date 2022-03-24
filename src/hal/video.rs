// SPDX-License-Identifier: GPL-2.0-only

pub enum RefreshRate {
    Hz50,
    Hz60,
    Default,
}

pub fn xvideo_set_mode(width: i32, height: i32, bpp: i32, refresh_rate: RefreshRate) -> bool {
    let ret;
    let rate = match refresh_rate {
        RefreshRate::Hz50 => nxdk_sys::hal::video::REFRESH_50HZ,
        RefreshRate::Hz60 => nxdk_sys::hal::video::REFRESH_60HZ,
        RefreshRate::Default => nxdk_sys::hal::video::REFRESH_DEFAULT,
    };

    unsafe {
        ret = nxdk_sys::hal::video::XVideoSetMode(width, height, bpp, rate as i32);
    }

    ret != 0
}
