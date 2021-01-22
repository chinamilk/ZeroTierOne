/*
 * Copyright (c)2013-2020 ZeroTier, Inc.
 *
 * Use of this software is governed by the Business Source License included
 * in the LICENSE.TXT file in the project's root directory.
 *
 * Change Date: 2025-01-01
 *
 * On the date above, in accordance with the Business Source License, use
 * of this software will be governed by version 2.0 of the Apache License.
 */
/****/

use zerotier_core::InetAddress;
use std::ffi::CStr;
use std::ptr::{null_mut, copy_nonoverlapping};
use std::mem::size_of;

pub struct PhysicalLink {
    pub address: InetAddress,
    pub device: String
}

impl PhysicalLink {
    #[cfg(unix)]
    pub fn map<F: FnMut(PhysicalLink)>(mut f: F) {
        unsafe {
            let mut ifap: *mut libc::ifaddrs = null_mut();
            if libc::getifaddrs((&mut ifap as *mut *mut libc::ifaddrs).cast()) == 0 {
                let mut i = ifap;
                while !i.is_null() {
                    if !(*i).ifa_addr.is_null() {
                        let mut a = InetAddress::new();
                        if (*(*i).ifa_addr).sa_family == libc::AF_INET as u8 {
                            copy_nonoverlapping((*i).ifa_addr.cast::<u8>(), (&mut a as *mut InetAddress).cast::<u8>(), size_of::<libc::sockaddr_in>());
                        } else if (*(*i).ifa_addr).sa_family == libc::AF_INET6 as u8 {
                            copy_nonoverlapping((*i).ifa_addr.cast::<u8>(), (&mut a as *mut InetAddress).cast::<u8>(), size_of::<libc::sockaddr_in6>());
                        } else {
                            continue;
                        }
                        a.set_port(0);
                        f(PhysicalLink{
                            address: a,
                            device: if (*i).ifa_name.is_null() { String::new() } else { String::from(CStr::from_ptr((*i).ifa_name).to_str().unwrap()) }
                        });
                    }
                    i = (*i).ifa_next;
                }
                libc::freeifaddrs(ifap.cast());
            }
        }
    }
}
