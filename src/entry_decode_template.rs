    fn {FUNC_NAME}(&mut self, env: &Env, mut arg0: i32, mut arg1: i64, mut arg2: i64) {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let mut var9: i32 = 0;
        let mut var10: i32 = 0;
        let mut var11: i32 = 0;
        let mut var12: i32 = 0;
        let mut var13: i32 = 0;
        let mut var14: i32 = 0;
        let mut var15: i32 = 0;
        let mut var16: i32 = 0;
        let mut var17: i32 = 0;
        let mut var18: i32 = 0;
        let mut var19: i32 = 0;
        let mut var20: i32 = 0;
        let mut var21: i32 = 0;
        let mut var22: i32 = 0;
        let mut var23: i32 = 0;
        let mut var24: i32 = 0;
        let mut var25: i32 = 0;
        let mut var26: i32 = 0;
        let mut var27: i32 = 0;
        let mut var28: i32 = 0;
        let mut var29: i32 = 0;
        let mut var30: i32 = 0;
        let mut var31: i32 = 0;
        let mut var32: i32 = 0;
        let mut var33: i32 = 0;
        let mut var34: i32 = 0;
        let mut var35: i32 = 0;
        let mut var36: i32 = 0;
        let mut var37: i32 = 0;
        let mut var38: i32 = 0;
        let mut var39: i32 = 0;
        let mut var40: i32 = 0;
        let mut var41: i64 = 0;
        let mut var42: i64 = 0;
        let mut var43: i64 = 0;
        let mut var44: i64 = 0;
        let mut var45: i64 = 0;
        let mut var46: i64 = 0;
        let mut var47: i64 = 0;
        let mut var48: i64 = 0;
        const ENTRY_SIZE: i32 = 32;
        const ENTRY_ZERO_PTR: i32 = 1049298;
        macro_rules! mload8 {
            ($addr:expr) => {
                self.memory.load8($addr)
            };
        }
        macro_rules! mload16 {
            ($addr:expr) => {
                self.memory.load16($addr)
            };
        }
        macro_rules! mload32 {
            ($addr:expr) => {
                self.memory.load32($addr)
            };
        }
        macro_rules! mload64 {
            ($addr:expr) => {
                self.memory.load64($addr)
            };
        }
        macro_rules! mstore8 {
            ($addr:expr, $value:expr) => {
                self.memory.store8($addr, $value)
            };
        }
        macro_rules! mstore16 {
            ($addr:expr, $value:expr) => {
                self.memory.store16($addr, $value)
            };
        }
        macro_rules! mstore32 {
            ($addr:expr, $value:expr) => {
                self.memory.store32($addr, $value)
            };
        }
        macro_rules! mstore64 {
            ($addr:expr, $value:expr) => {
                self.memory.store64($addr, $value)
            };
        }
        #[derive(Clone, Copy)]
        struct EntryHeader {
            raw64: i64,
            masked64: i64,
            word: i32,
            word_shr8: i32,
            tag: i32,
            off27: i32,
            off23: i32,
            off15: i64,
            off3: i32,
        }
        let read_entry_header = |ptr: i32| -> EntryHeader {
            let v41 = mload64!(ptr as usize + 7) as i64;
            let v42 = v41 & 18446744069414584320;
            let v0 = mload16!(ptr as usize) as i32;
            let v2 = mload8!(ptr.wrapping_add(2) as usize) as i32;
            let v6 = v0 | v2.wrapping_shl(16 as u32);
            let v7 = (v6 as u32).wrapping_shr(8 as u32) as i32;
            let v15 = mload8!(ptr as usize + 31) as i32;
            let v11 = mload32!(ptr as usize + 27) as i32;
            let v9 = mload32!(ptr as usize + 23) as i32;
            let v43 = mload64!(ptr as usize + 15) as i64;
            let v4 = mload32!(ptr as usize + 3) as i32;
            EntryHeader {
                raw64: v41,
                masked64: v42,
                word: v6,
                word_shr8: v7,
                tag: v15,
                off27: v11,
                off23: v9,
                off15: v43,
                off3: v4,
            }
        };
        let copy_payload_3x64 = |src: i32, dst: i32| {
            let v24 = mload64!(src.wrapping_add(24) as usize) as i64;
            mstore64!(dst.wrapping_add(24) as usize, v24 as u64);
            let v16 = mload64!(src.wrapping_add(16) as usize) as i64;
            mstore64!(dst.wrapping_add(16) as usize, v16 as u64);
            let v8 = mload64!(src.wrapping_add(8) as usize) as i64;
            mstore64!(dst.wrapping_add(8) as usize, v8 as u64);
        };
        let memcmp_32 = |a: i32, b: i32| -> i32 { self.func62(env, a, b) };
        let var49 = self.global0;
        var3 = var49.wrapping_sub(576);
        self.global0 = var3;
        let var50 = Vec::<Val>::from_val(env, &val_from_i64(arg1)).len() as i64
        var42 = var50;
        let mut slot_var3_240_i32 = 0 as i32;
        let mut slot_var3_232_i64 = arg1 as i64;
        mstore32!(var3 as usize + 244, (var42 as u64).wrapping_shr(32 as u32) as i64 as u32);
        self.func32(env, var3.wrapping_add(480), var3.wrapping_add(232));
        let var52: i32;
        'label0: loop {
            let var53 = mload8!(var3 as usize + 480) as i32;
            if (var53 == 0) as i32 != 0 {
                var16 = 1;
                var52 = 0;
                break 'label0;
            }
            var4 = var3.wrapping_add(240);
            let mut slot_var4_0_i32 = mload32!(var4 as usize) as i32;
            let mut slot_var3_244_i32 = mload32!(var3 as usize + 244) as i32;
            self.func35(env, var3.wrapping_add(392), slot_var4_0_i32, slot_var3_244_i32);
            let mut slot_var3_392_i32 = mload32!(var3 as usize + 392) as i32;
            var5 = slot_var3_392_i32.wrapping_add(1);
            var5 = { let a = var5; let b = -1; if var5 != 0 { a } else { b } };
            self.func45(env, var3.wrapping_add(72), { let a = 4; let b = var5; if (var5 as u32 <= 4 as u32) as i32 != 0 { a } else { b } }, 1, 32);
            let var56 = mload64!(var3.wrapping_add(489) as usize) as i64;
            arg1 = var56;
            let var57 = mload64!(var3.wrapping_add(497) as usize) as i64;
            var42 = var57;
            let var58 = mload64!(var3.wrapping_add(505) as usize) as i64;
            var41 = var58;
            let mut slot_var3_72_i32 = mload32!(var3 as usize + 72) as i32;
            var5 = slot_var3_72_i32;
            let mut slot_var3_76_i32 = mload32!(var3 as usize + 76) as i32;
            var16 = slot_var3_76_i32;
            let mut slot_var3_481_i64 = mload64!(var3 as usize + 481) as i64;
            let mut slot_var16_0_i64 = slot_var3_481_i64 as i64;
            mstore64!(var16.wrapping_add(24) as usize, var41 as u64);
            mstore64!(var16.wrapping_add(16) as usize, var42 as u64);
            mstore64!(var16.wrapping_add(8) as usize, arg1 as u64);
            let mut slot_var3_456_i32 = 1 as i32;
            let mut slot_var3_452_i32 = var16 as i32;
            let mut slot_var3_448_i32 = var5 as i32;
            mstore64!(var3.wrapping_add(400) as usize, slot_var4_0_i32 as u64);
            let mut slot_var3_392_i64 = slot_var3_232_i64 as i64;
            var4 = var3.wrapping_add(481);
            var7 = ENTRY_SIZE;
            var14 = 1;
            'label1: loop {
                self.func32(env, var3.wrapping_add(480), var3.wrapping_add(392));
                let var60 = mload8!(var3 as usize + 480) as i32;
                if (var60 == 1) as i32 != 0 {
                    if (slot_var3_448_i32 == var14) as i32 != 0 {
                        let mut slot_var3_400_i32 = mload32!(var3 as usize + 400) as i32;
                        let mut slot_var3_404_i32 = mload32!(var3 as usize + 404) as i32;
                        self.func35(env, var3.wrapping_add(544), slot_var3_400_i32, slot_var3_404_i32);
                        let mut slot_var3_544_i32 = mload32!(var3 as usize + 544) as i32;
                        var5 = slot_var3_544_i32.wrapping_add(1);
                        self.func60(env, var3.wrapping_add(448), var14, { let a = var5; let b = -1; if var5 != 0 { a } else { b } }, ENTRY_SIZE);
                        var16 = slot_var3_452_i32;
                    }
                    var5 = var7.wrapping_add(var16);
                    let mut slot_var5_0_i64 = slot_var4_0_i32 as i64;
                    copy_payload_3x64(var4, var5);
                    var14 = var14.wrapping_add(1);
                    slot_var3_456_i32 = var14 as i32;
                    var7 = var7.wrapping_add(ENTRY_SIZE);
                    continue 'label1;
                }
                break;
            }
            var52 = slot_var3_448_i32;
            break;
        }
        var12 = var52;
        let var66 = self.func58(env);
        arg1 = var66;
        self.func45(env, var3.wrapping_sub(-64), 2, 1, ENTRY_SIZE);
        slot_var3_240_i32 = 0 as i32;
        let mut slot_var3_68_i32 = mload32!(var3 as usize + 68) as i32;
        var11 = slot_var3_68_i32;
        let mut slot_var3_236_i32 = var11 as i32;
        let mut slot_var3_64_i32 = mload32!(var3 as usize + 64) as i32;
        var5 = slot_var3_64_i32;
        let mut slot_var3_232_i32 = var5 as i32;
        var4 = 0;
        if (var5 as u32 <= 1 as u32) as i32 != 0 {
            self.func60(env, var3.wrapping_add(232), 0, 2, ENTRY_SIZE);
            var11 = slot_var3_236_i32;
            var4 = slot_var3_240_i32;
        }
        var6 = var4.wrapping_mul(ENTRY_SIZE);
        var7 = 1048792;
        'label2: loop {
            var5 = var3.wrapping_add(392);
            self.func44(env, var5, var7, 20);
            self.func61(env, var3.wrapping_add(480), var5);
            var5 = var6.wrapping_add(var11);
            copy_payload_3x64(var3.wrapping_add(480), var5);
            let mut slot_var3_480_i64 = mload64!(var3 as usize + 480) as i64;
            slot_var5_0_i64 = slot_var3_480_i64 as i64;
            var7 = var7.wrapping_add(20);
            var6 = var6.wrapping_add(ENTRY_SIZE);
            var15 = var15.wrapping_add(1);
            if (var15 != 2) as i32 != 0 {
                continue 'label2;
            }
            break;
        }
        var7 = 2;
        'label3: loop {
            'label4: loop {
                'label5: loop {
                    'label6: loop {
                        'label7: loop {
                            'label8: loop {
                                var15 = var4.wrapping_add(var15);
                                if (var15 as u32 >= 2 as u32) as i32 != 0 {
                                    if (var15 as u32 > 255 as u32) as i32 != 0 {
                                        arg1 = 0;
                                        var42 = 0 /* False */;
                                        var41 = 255;
                                        var4 = var15;
                                        break 'label5;
                                    }
                                    var9 = slot_var3_232_i32;
                                    var8 = var11.wrapping_add(var15.wrapping_mul(ENTRY_SIZE));
                                    var7 = var11;
                                    'label9: loop {
                                        'label10: loop {
                                            var5 = var7;
                                            if (var6 == 0) as i32 != 0 {
                                                break 'label9;
                                            }
                                            var6 = var6.wrapping_sub(ENTRY_SIZE);
                                            var7 = var5.wrapping_add(ENTRY_SIZE);
                                            let var74 = memcmp_32(var5, ENTRY_ZERO_PTR);
                                            if (var74 == 0) as i32 != 0 {
                                                continue 'label10;
                                            }
                                            break;
                                        }
                                        let header = read_entry_header(var5);
                                        var41 = header.raw64;
                                        var42 = header.masked64;
                                        var6 = header.word;
                                        var7 = header.word_shr8;
                                        var15 = header.tag;
                                        var11 = header.off27;
                                        var9 = header.off23;
                                        var43 = header.off15;
                                        var4 = header.off3;
                                        arg1 = 0 /* Symbol() */;
                                        break 'label5;
                                        break;
                                    }
                                    if (var15 == 2) as i32 != 0 {
                                        break 'label8;
                                    }
                                    var4 = var11;
                                    'label11: loop {
                                        if (var4 == var8) as i32 != 0 {
                                            break 'label6;
                                        }
                                        var10 = var4.wrapping_add(ENTRY_SIZE);
                                        var5 = var11;
                                        var13 = var13.wrapping_add(1);
                                        var7 = var13;
                                        'label12: loop {
                                            'label13: loop {
                                                'label14: loop {
                                                    if (var7 == 0) as i32 != 0 {
                                                        var6 = var5;
                                                        if (var5 == var8) as i32 != 0 {
                                                            break 'label14;
                                                        }
                                                        break 'label13;
                                                    }
                                                    var6 = var5.wrapping_add(var7.wrapping_mul(ENTRY_SIZE));
                                                    if ((var8.wrapping_sub(var5) as u32).wrapping_shr(5 as u32) as i32 as u32 > var7 as u32) as i32 != 0 {
                                                        break 'label13;
                                                    }
                                                    break;
                                                }
                                                var4 = var10;
                                                continue 'label11;
                                                break;
                                            }
                                            var5 = var6.wrapping_add(ENTRY_SIZE);
                                            var7 = 0;
                                            let var78 = memcmp_32(var4, var6);
                                            if (var78 == 0) as i32 != 0 {
                                                continue 'label12;
                                            }
                                            break;
                                        }
                                        break;
                                    }
                                    break 'label7;
                                }
                                arg1 = 0;
                                var41 = 0 /* False */;
                                var42 = 0 /* False */;
                                var6 = var15;
                                break 'label5;
                                break;
                            }
                            let var79 = memcmp_32(var11, var11.wrapping_add(ENTRY_SIZE));
                            if (var79 == 0) as i32 != 0 {
                                break 'label6;
                            }
                            var4 = var11;
                            break;
                        }
                        let header = read_entry_header(var4);
                        var41 = header.raw64;
                        var42 = header.masked64;
                        var6 = header.word;
                        var7 = header.word_shr8;
                        var15 = header.tag;
                        var11 = header.off27;
                        var9 = header.off23;
                        var43 = header.off15;
                        var4 = header.off3;
                        arg1 = 15;
                        break 'label5;
                        break;
                    }
                    var6 = 1;
                    if (var14 == 0) as i32 != 0 {
                        arg1 = 16;
                        break 'label3;
                    }
                    var5 = 255;
                    if (var14 as u32 > 255 as u32) as i32 != 0 {
                        arg1 = 17;
                        var4 = var14;
                        break 'label3;
                    }
                    var4 = var14.wrapping_mul(ENTRY_SIZE);
                    var13 = var16.wrapping_add(var4);
                    var5 = 0;
                    'label15: loop {
                        'label16: loop {
                            if (var4 == var5) as i32 != 0 {
                                break 'label15;
                            }
                            let var83 = var5;
                            var5 = var5.wrapping_add(ENTRY_SIZE);
                            let var84 = memcmp_32(var83.wrapping_add(var16), ENTRY_ZERO_PTR);
                            if (var84 == 0) as i32 != 0 {
                                continue 'label16;
                            }
                            break;
                        }
                        var4 = var5.wrapping_add(var16).wrapping_sub(ENTRY_SIZE);
                        arg1 = 18;
                        break 'label4;
                        break;
                    }
                    'label17: loop {
                        'label18: loop {
                            'label19: loop {
                                match (var14 & 255).wrapping_sub(1) {
                                    0 => break 'label17,
                                    1 => break 'label19,
                                    _ => break 'label18,
                                }
                                break;
                            }
                            let var85 = memcmp_32(var16, var16.wrapping_add(ENTRY_SIZE));
                            if (var85 == 0) as i32 != 0 {
                                break 'label17;
                            }
                            arg1 = 19;
                            var4 = var16;
                            break 'label4;
                            break;
                        }
                        var4 = var16;
                        'label20: loop {
                            if (var4 == var13) as i32 != 0 {
                                break 'label17;
                            }
                            var10 = var4.wrapping_add(ENTRY_SIZE);
                            var5 = var16;
                            var18 = var18.wrapping_add(1);
                            var8 = var18;
                            'label21: loop {
                                'label22: loop {
                                    'label23: loop {
                                        if (var8 == 0) as i32 != 0 {
                                            var7 = var5;
                                            if (var13 == var7) as i32 != 0 {
                                                break 'label23;
                                            }
                                            break 'label22;
                                        }
                                        var7 = var5.wrapping_add(var8.wrapping_mul(ENTRY_SIZE));
                                        if ((var13.wrapping_sub(var5) as u32).wrapping_shr(5 as u32) as i32 as u32 > var8 as u32) as i32 != 0 {
                                            break 'label22;
                                        }
                                        break;
                                    }
                                    var4 = var10;
                                    continue 'label20;
                                    break;
                                }
                                var5 = var7.wrapping_add(ENTRY_SIZE);
                                var8 = 0;
                                let var86 = memcmp_32(var4, var7);
                                if (var86 == 0) as i32 != 0 {
                                    continue 'label21;
                                }
                                break;
                            }
                            break;
                        }
                        arg1 = 19;
                        break 'label4;
                        break;
                    }
                    var4 = (arg1 as u64).wrapping_shr(32 as u32) as i64 as i32;
                    var7 = (arg1 as u64).wrapping_shr(0 as u32) as i64 as i32;
                    var8 = 0;
                    var43 = 180000;
                    var5 = 180000;
                    var6 = 0;
                    break 'label3;
                    break;
                }
                var7 = var6 & 255 | var8.wrapping_shl(16 as u32) | (var7 & 255).wrapping_shl(8 as u32);
                var8 = (var42 as u64).wrapping_shr(32 as u32) as i64 as i32;
                var5 = var41 as i32;
                var6 = 1;
                break 'label3;
                break;
            }
            let var87 = mload16!(var4 as usize) as i32;
            let var88 = mload8!(var4.wrapping_add(2) as usize) as i32;
            var7 = var87 | var88.wrapping_shl(16 as u32);
            let var89 = mload8!(var4 as usize + 31) as i32;
            var15 = var89;
            var11 = slot_var4_27_i32;
            var9 = slot_var4_23_i32;
            var43 = slot_var4_15_i64;
            let mut slot_var4_11_i32 = mload32!(var4 as usize + 11) as i32;
            var8 = slot_var4_11_i32;
            var5 = slot_var4_7_i64;
            var4 = slot_var4_3_i32;
            break;
        }
        arg1 = arg1 & 255 | (var4 as u32 as i64).wrapping_shl(32 as u32) | (var7 as u32 as i64 & 16777215).wrapping_shl(0 as u32);
        var48 = var5 as u32 as i64 | (var8 as u32 as i64).wrapping_shl(32 as u32);
        'label24: loop {
            'label25: loop {
                'label26: loop {
                    'label27: loop {
                        'label28: loop {
                            'label29: loop {
                                'label30: loop {
                                    if (var6 == 0) as i32 != 0 {
                                        var4 = var3.wrapping_add(480);
                                        let var90 = Bytes::from_val(env, &val_from_i64(arg2)).len() as i64
                                        self.func63(env, var4, (var90 as u64).wrapping_shr(32 as u32) as i64 as i32);
                                        let mut slot_var3_484_i32 = mload32!(var3 as usize + 484) as i32;
                                        var5 = slot_var3_484_i32;
                                        let mut slot_var3_488_i32 = mload32!(var3 as usize + 488) as i32;
                                        var6 = slot_var3_488_i32;
                                        let var92 = Bytes::from_val(env, &val_from_i64(arg2)).len() as i64
                                        if (var6 == (var92 as u64).wrapping_shr(32 as u32) as i64 as i32) as i32 != 0 {
                                            self.func64(env, arg2, var5, var6);
                                            let var94 = mload32!(var3.wrapping_add(488) as usize) as i32;
                                            mstore32!(var3.wrapping_add(88) as usize, var94 as u32);
                                            let mut slot_var3_80_i64 = slot_var3_480_i64 as i64;
                                            var10 = var3.wrapping_add(80);
                                            self.func65(env, var4, var10, 9);
                                            let mut slot_var3_492_i32 = mload32!(var3 as usize + 492) as i32;
                                            var7 = slot_var3_492_i32;
                                            var17 = slot_var3_488_i32;
                                            let var96 = mload8!(var3 as usize + 480) as i32;
                                            var6 = var96;
                                            if (var6 != 27) as i32 != 0 {
                                                break 'label30;
                                            }
                                            var5 = slot_var3_484_i32;
                                            var6 = 5;
                                            var8 = 0;
                                            if (var7 != 9) as i32 != 0 {
                                                break 'label26;
                                            }
                                            let var97 = self.func129(env, var17, 1049394, 9);
                                            if var97 != 0 {
                                                break 'label26;
                                            }
                                            self.func66(env, var4, var10, 3);
                                            let var99 = mload8!(var3 as usize + 480) as i32;
                                            var6 = var99;
                                            if (var6 == 27) as i32 != 0 {
                                                arg2 = slot_var3_488_i32;
                                                if (arg2 as u64 <= 4294967295 as u64) as i32 != 0 {
                                                    self.func65(env, var4, var10, arg2 as i32);
                                                    let var101 = mload8!(var3 as usize + 480) as i32;
                                                    var6 = var101;
                                                    if (var6 != 27) as i32 != 0 {
                                                        break 'label27;
                                                    }
                                                    self.func66(env, var4, var10, 2);
                                                    let var103 = mload8!(var3 as usize + 480) as i32;
                                                    var6 = var103;
                                                    if (var6 != 27) as i32 != 0 {
                                                        break 'label27;
                                                    }
                                                    arg2 = slot_var3_488_i32;
                                                    if (arg2 as u64 > 4294967295 as u64) as i32 != 0 {
                                                        var6 = 23;
                                                        break 'label25;
                                                    }
                                                } else {
                                                    var6 = 23;
                                                    break 'label25;
                                                }
                                            } else {
                                                break 'label27;
                                            }
                                        }
                                            var27 = arg2 as i32;
                                            self.func45(env, var3.wrapping_add(56), var27, 8, 56);
                                            let mut slot_var3_56_i64 = mload64!(var3 as usize + 56) as i64;
                                            let mut slot_var3_116_i64 = slot_var3_56_i64 as i64;
                                            var20 = var3.wrapping_add(513);
                                            var21 = var3.wrapping_add(481);
                                            var24 = var3.wrapping_add(520);
                                            var22 = var3.wrapping_add(260);
                                            var25 = var3.wrapping_add(535);
                                            var29 = var3.wrapping_add(512);
                                            var30 = var3.wrapping_add(232) | 1;
                                            var31 = var3.wrapping_add(492);
                                            var32 = var3.wrapping_add(245);
                                            var33 = var32.wrapping_add(7);
                                            var23 = var3.wrapping_add(502);
                                            var34 = var3.wrapping_add(490);
                                            var35 = var3.wrapping_add(510);
                                            var6 = 0;
                                            'label33: loop {
                                                'label34: loop {
                                                    let mut slot_var3_124_i32 = var6 as i32;
                                                    let var105: i32;
                                                    'label35: loop {
                                                        'label36: loop {
                                                            'label37: loop {
                                                                'label38: loop {
                                                                    'label39: loop {
                                                                        'label40: loop {
                                                                            'label41: loop {
                                                                                'label42: loop {
                                                                                    if (var26 != var27) as i32 != 0 {
                                                                                        var4 = var3.wrapping_add(480);
                                                                                        var10 = var3.wrapping_add(80);
                                                                                        self.func65(env, var4, var10, 65);
                                                                                        'label43: loop {
                                                                                            let var107 = mload8!(var3 as usize + 480) as i32;
                                                                                            var6 = var107;
                                                                                            if (var6 == 27) as i32 != 0 {
                                                                                                var42 = slot_var3_488_i32;
                                                                                                let mut slot_var3_84_i32 = mload32!(var3 as usize + 84) as i32;
                                                                                                let mut slot_var3_88_i32 = mload32!(var3 as usize + 88) as i32;
                                                                                                self.func44(env, var3.wrapping_add(212), slot_var3_84_i32, slot_var3_88_i32);
                                                                                                self.func66(env, var4, var10, 3);
                                                                                                let var110 = mload8!(var3 as usize + 480) as i32;
                                                                                                var6 = var110;
                                                                                                if (var6 != 27) as i32 != 0 {
                                                                                                    break 'label29;
                                                                                                }
                                                                                                var46 = slot_var3_488_i32;
                                                                                                self.func66(env, var4, var10, 4);
                                                                                                let var112 = mload8!(var3 as usize + 480) as i32;
                                                                                                var6 = var112;
                                                                                                if (var6 != 27) as i32 != 0 {
                                                                                                    break 'label29;
                                                                                                }
                                                                                                var44 = slot_var3_488_i32;
                                                                                                self.func66(env, var4, var10, 6);
                                                                                                let var114 = mload8!(var3 as usize + 480) as i32;
                                                                                                var6 = var114;
                                                                                                if (var6 == 27) as i32 != 0 {
                                                                                                    break 'label43;
                                                                                                }
                                                                                                break 'label29;
                                                                                            }
                                                                                            break 'label29;
                                                                                            break;
                                                                                        }
                                                                                        var41 = var44.wrapping_add(32);
                                                                                        if ((var41 as u64) < var44 as u64) as i32 != 0 {
                                                                                            break 'label39;
                                                                                        }
                                                                                        var47 = slot_var3_488_i32;
                                                                                        self.func130(env, var3.wrapping_add(40), var46, var41);
                                                                                        let mut slot_var3_48_i64 = mload64!(var3 as usize + 48) as i64;
                                                                                        if (slot_var3_48_i64 != 0 /* False */) as i32 != 0 {
                                                                                            break 'label39;
                                                                                        }
                                                                                        let mut slot_var3_40_i64 = mload64!(var3 as usize + 40) as i64;
                                                                                        var45 = slot_var3_40_i64;
                                                                                        var41 = var45.wrapping_add(0);
                                                                                        if ((var41 as u64) < var45 as u64) as i32 != 0 {
                                                                                            break 'label39;
                                                                                        }
                                                                                        var45 = var41.wrapping_add(0);
                                                                                        if (var41 as u64 > var45 as u64) as i32 != 0 {
                                                                                            break 'label39;
                                                                                        }
                                                                                        var41 = var45.wrapping_add(3 /* Error(Contract, #0) */);
                                                                                        if (var45 as u64 > var41 as u64) as i32 != 0 {
                                                                                            break 'label39;
                                                                                        }
                                                                                        if (var41 as u64 <= 4294967295 as u64) as i32 != 0 {
                                                                                            var4 = var3.wrapping_add(480);
                                                                                            self.func65(env, var4, var3.wrapping_add(212), var41 as i32);
                                                                                            let var117 = mload8!(var3 as usize + 480) as i32;
                                                                                            var6 = var117;
                                                                                            if (var6 != 27) as i32 != 0 {
                                                                                                break 'label29;
                                                                                            }
                                                                                            var41 = (var42 as u64).wrapping_shr(32 as u32) as i64;
                                                                                            if (var41 == 65 /* I64(obj#0) */) as i32 != 0 {
                                                                                                var41 = slot_var3_488_i32;
                                                                                                var10 = var21.wrapping_add(24);
                                                                                                var6 = var42 as i32;
                                                                                                let var118 = mload64!(var6.wrapping_add(24) as usize) as i64;
                                                                                                let mut slot_var10_0_i64 = var118 as i64;
                                                                                                var17 = var21.wrapping_add(16);
                                                                                                let var119 = mload64!(var6.wrapping_add(16) as usize) as i64;
                                                                                                let mut slot_var17_0_i64 = var119 as i64;
                                                                                                var19 = var21.wrapping_add(8);
                                                                                                let var120 = mload64!(var6.wrapping_add(8) as usize) as i64;
                                                                                                let mut slot_var19_0_i64 = var120 as i64;
                                                                                                let mut slot_var6_0_i64 = mload64!(var6 as usize) as i64;
                                                                                                let mut slot_var21_0_i64 = slot_var6_0_i64 as i64;
                                                                                                mstore8!(var3 as usize + 480, 0 as u8);
                                                                                                var7 = var3.wrapping_add(448);
                                                                                                self.func67(env, var7, var4);
                                                                                                let var122 = mload64!(var6.wrapping_add(56) as usize) as i64;
                                                                                                slot_var10_0_i64 = var122 as i64;
                                                                                                let var123 = mload64!(var6.wrapping_add(48) as usize) as i64;
                                                                                                slot_var17_0_i64 = var123 as i64;
                                                                                                let var124 = mload64!(var6.wrapping_add(40) as usize) as i64;
                                                                                                slot_var19_0_i64 = var124 as i64;
                                                                                                let mut slot_var6_32_i64 = mload64!(var6 as usize + 32) as i64;
                                                                                                slot_var21_0_i64 = slot_var6_32_i64 as i64;
                                                                                                mstore8!(var3 as usize + 480, 0 as u8);
                                                                                                var10 = var3.wrapping_add(544);
                                                                                                self.func67(env, var10, var4);
                                                                                                let var126 = memcmp_32(var7, ENTRY_ZERO_PTR);
                                                                                                var4 = var126;
                                                                                                let var127 = memcmp_32(var10, ENTRY_ZERO_PTR);
                                                                                                var17 = var127;
                                                                                                'label44: loop {
                                                                                                    let var128 = self.func68(env, var7, 1049330);
                                                                                                    let var129 = self.func68(env, var10, 1049362);
                                                                                                    if ((((var128 & 255) as u32) < 2 as u32) as i32 | var4 | var17 | (var129 & 255 == 1) as i32 == 0) as i32 != 0 {
                                                                                                        let mut slot_var3_376_i32 = -2147483644 as i32;
                                                                                                        break 'label44;
                                                                                                    }
                                                                                                    self.func69(env, var3.wrapping_add(376), var6, 65);
                                                                                                    var7 = slot_var3_376_i32;
                                                                                                    if (var7 != -2147483644) as i32 != 0 {
                                                                                                        break 'label40;
                                                                                                    }
                                                                                                    break;
                                                                                                }
                                                                                                'label45: loop {
                                                                                                    let var131 = mload8!(var6 as usize + 64) as i32;
                                                                                                    var7 = var131;
                                                                                                    if ((var7 as u32) < 2 as u32) as i32 != 0 {
                                                                                                        var4 = var7;
                                                                                                        break 'label45;
                                                                                                    }
                                                                                                    var4 = var7.wrapping_sub(27);
                                                                                                    if (var4 as u32 > 1 as u32) as i32 != 0 {
                                                                                                        break 'label41;
                                                                                                    }
                                                                                                    break;
                                                                                                }
                                                                                                let var132 = val_to_i64(Bytes::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                let var133 = val_to_i64(env.crypto().keccak256(&Bytes::from_val(env, &val_from_i64(var132))).into())
                                                                                                self.func71(env, var3.wrapping_add(392), var133);
                                                                                                var10 = var3.wrapping_add(480);
                                                                                                let var135 = self.func131(env, var10, var6, 64);
                                                                                                var135;
                                                                                                let var136 = val_to_i64(Bytes::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                var42 = var136;
                                                                                                let mut slot_var3_424_i64 = mload64!(var3 as usize + 424) as i64;
                                                                                                let var137 = val_to_i64(env.crypto().secp256k1_recover(&Hash::<32>::from_val(env, &val_from_i64(slot_var3_424_i64)), &BytesN::<64>::from_val(env, &val_from_i64(var42)), (var4 as u32 as i64 & 255).wrapping_shl(32 as u32) | 0 as u32).into())
                                                                                                var41 = var137;
                                                                                                var6 = var3.wrapping_add(544);
                                                                                                self.func63(env, var6, 65);
                                                                                                let mut slot_var3_548_i32 = mload32!(var3 as usize + 548) as i32;
                                                                                                var7 = slot_var3_548_i32;
                                                                                                let mut slot_var3_552_i32 = mload32!(var3 as usize + 552) as i32;
                                                                                                var4 = slot_var3_552_i32;
                                                                                                let var139 = Bytes::from_val(env, &val_from_i64(var41)).len() as i64
                                                                                                if (var4 != (var139 as u64).wrapping_shr(32 as u32) as i64 as i32) as i32 != 0 {
                                                                                                    break 'label31;
                                                                                                }
                                                                                                self.func64(env, var41, var7, var4);
                                                                                                if (var4 == 0) as i32 != 0 {
                                                                                                    break 'label42;
                                                                                                }
                                                                                                var17 = 1;
                                                                                                let var141 = val_to_i64(Bytes::new(env).into_val(env)) /* TODO: linear memory */;
                                                                                                let var142 = val_to_i64(env.crypto().keccak256(&Bytes::from_val(env, &val_from_i64(var141))).into())
                                                                                                self.func71(env, var10, var142);
                                                                                                self.func44(env, var6, var31, 20);
                                                                                                self.func61(env, var30, var6);
                                                                                                let var146 = mload8!(var22.wrapping_add(4) as usize) as i32;
                                                                                                mstore8!(var3.wrapping_add(228) as usize, var146 as u8);
                                                                                                let mut slot_var32_0_i64 = mload64!(var32 as usize) as i64;
                                                                                                let mut slot_var3_304_i64 = slot_var32_0_i64 as i64;
                                                                                                let mut slot_var33_0_i64 = mload64!(var33 as usize) as i64;
                                                                                                let mut slot_var3_311_i64 = slot_var33_0_i64 as i64;
                                                                                                let mut slot_var22_0_i32 = mload32!(var22 as usize) as i32;
                                                                                                let mut slot_var3_224_i32 = slot_var22_0_i32 as i32;
                                                                                                let var147 = mload16!(var3 as usize + 233) as i32;
                                                                                                let var148 = mload8!(var3 as usize + 235) as i32;
                                                                                                var10 = var147 | var148.wrapping_shl(16 as u32);
                                                                                                var42 = slot_var3_236_i32;
                                                                                                let var149 = mload8!(var3 as usize + 244) as i32;
                                                                                                var105 = var149;
                                                                                                break 'label35;
                                                                                            }
                                                                                            mstore32!(var3 as usize + 240, var41 as u32);
                                                                                            slot_var3_236_i32 = -2147483645 as i32;
                                                                                            break 'label36;
                                                                                        }
                                                                                        var6 = 23;
                                                                                        break 'label34;
                                                                                    }
                                                                                    let mut slot_var3_120_i64 = mload64!(var3 as usize + 120) as i64;
                                                                                    arg2 = slot_var3_120_i64;
                                                                                    let mut slot_var3_108_i64 = arg2 as i64;
                                                                                    var10 = slot_var3_116_i64;
                                                                                    let mut slot_var3_104_i32 = var10 as i32;
                                                                                    var5 = slot_var3_88_i32;
                                                                                    if var5 != 0 {
                                                                                        var6 = 6;
                                                                                        break 'label25;
                                                                                    }
                                                                                    let mut slot_var3_92_i32 = var10 as i32;
                                                                                    let mut slot_var3_96_i64 = arg2 as i64;
                                                                                    if ((arg2 as u64) < 4294967296 as u64) as i32 != 0 {
                                                                                        var6 = 2;
                                                                                        break 'label38;
                                                                                    }
                                                                                    var6 = arg2 as i32;
                                                                                    var41 = slot_var6_0_i64;
                                                                                    var42 = var41.wrapping_add(var48);
                                                                                    if (arg1 as u64 > ({ let a = 18446744073709551615; let b = var42; if (var41 as u64 > var42 as u64) as i32 != 0 { a } else { b } }) as u64) as i32 != 0 {
                                                                                        var6 = 9;
                                                                                        break 'label38;
                                                                                    }
                                                                                    var42 = arg1.wrapping_add(var43);
                                                                                    if ((({ let a = 18446744073709551615; let b = var42; if (arg1 as u64 > var42 as u64) as i32 != 0 { a } else { b } }) as u64) < var41 as u64) as i32 != 0 {
                                                                                        var6 = 10;
                                                                                        break 'label38;
                                                                                    }
                                                                                    var8 = ((arg2 as u64).wrapping_shr(32 as u32) as i64 as i32).wrapping_mul(56);
                                                                                    var20 = var6.wrapping_add(var8);
                                                                                    var5 = 56;
                                                                                    'label46: loop {
                                                                                        'label47: loop {
                                                                                            if (var5 == var8) as i32 != 0 {
                                                                                                break 'label46;
                                                                                            }
                                                                                            var4 = var5.wrapping_add(var6);
                                                                                            var5 = var5.wrapping_add(56);
                                                                                            arg1 = slot_var4_0_i32;
                                                                                            if (var41 == arg1) as i32 != 0 {
                                                                                                continue 'label47;
                                                                                            }
                                                                                            break;
                                                                                        }
                                                                                        var6 = 20;
                                                                                        break 'label38;
                                                                                        break;
                                                                                    }
                                                                                    var42 = var15 as u32 as i64;
                                                                                    arg1 = var42.wrapping_mul(var14 as u32 as i64);
                                                                                    if (arg1 as u64).wrapping_shr(32 as u32) as i64 as i32 != 0 {
                                                                                        break 'label39;
                                                                                    }
                                                                                    var4 = arg1 as i32;
                                                                                    self.func72(env, var3.wrapping_add(16), var4, 33);
                                                                                    slot_var3_240_i32 = 0 as i32;
                                                                                    let mut slot_var3_20_i32 = mload32!(var3 as usize + 20) as i32;
                                                                                    var5 = slot_var3_20_i32;
                                                                                    slot_var3_236_i32 = var5 as i32;
                                                                                    let mut slot_var3_16_i32 = mload32!(var3 as usize + 16) as i32;
                                                                                    var9 = slot_var3_16_i32;
                                                                                    slot_var3_232_i32 = var9 as i32;
                                                                                    var18 = 0;
                                                                                    if (var4 as u32 > var9 as u32) as i32 != 0 {
                                                                                        self.func60(env, var3.wrapping_add(232), 0, var4, 33);
                                                                                        var18 = slot_var3_240_i32;
                                                                                        var5 = slot_var3_236_i32;
                                                                                    }
                                                                                    var5 = var5.wrapping_add(var18.wrapping_mul(33));
                                                                                    var9 = { let a = 1; let b = var4; if (var4 as u32 <= 1 as u32) as i32 != 0 { a } else { b } };
                                                                                    var7 = var9.wrapping_sub(1);
                                                                                    var13 = var3.wrapping_add(488);
                                                                                    var17 = var3.wrapping_add(496);
                                                                                    var12 = var3.wrapping_add(504);
                                                                                    'label48: loop {
                                                                                        'label49: loop {
                                                                                            if var7 != 0 {
                                                                                                mstore8!(var5 as usize, 0 as u8);
                                                                                                let mut slot_var5_1_i64 = slot_var3_480_i64 as i64;
                                                                                                let mut slot_var13_0_i64 = mload64!(var13 as usize) as i64;
                                                                                                mstore64!(var5.wrapping_add(9) as usize, slot_var13_0_i64 as u64);
                                                                                                mstore64!(var5.wrapping_add(17) as usize, slot_var17_0_i64 as u64);
                                                                                                let mut slot_var12_0_i64 = mload64!(var12 as usize) as i64;
                                                                                                mstore64!(var5.wrapping_add(25) as usize, slot_var12_0_i64 as u64);
                                                                                                var7 = var7.wrapping_sub(1);
                                                                                                var5 = var5.wrapping_add(33);
                                                                                                continue 'label49;
                                                                                            } else {
                                                                                                'label50: loop {
                                                                                                    var21 = var9.wrapping_add(var18);
                                                                                                    if var4 != 0 {
                                                                                                        break 'label50;
                                                                                                    }
                                                                                                    var21 = var21.wrapping_sub(1);
                                                                                                    break 'label48;
                                                                                                    break;
                                                                                                }
                                                                                            }
                                                                                            break;
                                                                                        }
                                                                                        mstore8!(var5 as usize, 0 as u8);
                                                                                        slot_var5_1_i64 = slot_var3_392_i64 as i64;
                                                                                        let var152 = mload64!(var3.wrapping_add(400) as usize) as i64;
                                                                                        mstore64!(var5.wrapping_add(9) as usize, var152 as u64);
                                                                                        let var153 = mload64!(var3.wrapping_add(408) as usize) as i64;
                                                                                        mstore64!(var5.wrapping_add(17) as usize, var153 as u64);
                                                                                        let var154 = mload64!(var3.wrapping_add(416) as usize) as i64;
                                                                                        mstore64!(var5.wrapping_add(25) as usize, var154 as u64);
                                                                                        break;
                                                                                    }
                                                                                    var25 = slot_var3_236_i32;
                                                                                    let mut slot_var3_388_i32 = var20 as i32;
                                                                                    let mut slot_var3_384_i32 = var10 as i32;
                                                                                    let mut slot_var3_380_i32 = var6 as i32;
                                                                                    slot_var3_376_i32 = var6 as i32;
                                                                                    var17 = var14.wrapping_mul(ENTRY_SIZE);
                                                                                    var19 = var15.wrapping_mul(ENTRY_SIZE);
                                                                                    var8 = var3.wrapping_add(512);
                                                                                    var18 = var3.wrapping_add(413);
                                                                                    var22 = var3.wrapping_add(404);
                                                                                    'label51: loop {
                                                                                        'label52: loop {
                                                                                            if (var6 != var20) as i32 != 0 {
                                                                                                var13 = var6.wrapping_add(56);
                                                                                                slot_var3_380_i32 = var13 as i32;
                                                                                                let mut slot_var6_8_i32 = mload32!(var6 as usize + 8) as i32;
                                                                                                var12 = slot_var6_8_i32;
                                                                                                if (var12 != -2147483648) as i32 != 0 {
                                                                                                    break 'label52;
                                                                                                }
                                                                                            }
                                                                                            slot_var3_384_i32 = 0 as i32;
                                                                                            let mut slot_var3_376_i64 = 4294967296 as i64;
                                                                                            var19 = var3.wrapping_add(233);
                                                                                            var20 = var3.wrapping_add(393);
                                                                                            var8 = var3.wrapping_add(481);
                                                                                            var22 = var3.wrapping_add(512);
                                                                                            var17 = 1;
                                                                                            var13 = 0;
                                                                                            var5 = 0;
                                                                                            var18 = 0;
                                                                                            'label53: loop {
                                                                                                'label54: loop {
                                                                                                    'label55: loop {
                                                                                                        'label56: loop {
                                                                                                            'label57: loop {
                                                                                                                'label58: loop {
                                                                                                                    let var155: i32;
                                                                                                                    'label59: loop {
                                                                                                                        if (var14 as u32 <= var18 as u32) as i32 != 0 {
                                                                                                                            var7 = var3.wrapping_add(192);
                                                                                                                            var9 = var5;
                                                                                                                            var155 = 0;
                                                                                                                            break 'label59;
                                                                                                                        }
                                                                                                                        arg1 = (var18 as u32 as i64).wrapping_mul(var42);
                                                                                                                        if (arg1 as u64).wrapping_shr(32 as u32) as i64 as i32 != 0 {
                                                                                                                            break 'label39;
                                                                                                                        }
                                                                                                                        var4 = arg1 as i32;
                                                                                                                        var6 = var4.wrapping_add(var15);
                                                                                                                        if ((var6 as u32) < var4 as u32) as i32 != 0 {
                                                                                                                            break 'label39;
                                                                                                                        }
                                                                                                                        if (var6 as u32 > var21 as u32) as i32 != 0 {
                                                                                                                            break 'label58;
                                                                                                                        }
                                                                                                                        var9 = var5.wrapping_add(1);
                                                                                                                        if (var9 == 0) as i32 != 0 {
                                                                                                                            break 'label39;
                                                                                                                        }
                                                                                                                        var18 = var18.wrapping_add(1);
                                                                                                                        let mut slot_var3_192_i32 = var25.wrapping_add(var4.wrapping_mul(33)) as i32;
                                                                                                                        var7 = var3.wrapping_add(152);
                                                                                                                        var24 = var5;
                                                                                                                        var155 = var25.wrapping_add(var6.wrapping_mul(33));
                                                                                                                        break;
                                                                                                                    }
                                                                                                                    var4 = var155;
                                                                                                                    let mut slot_var7_0_i32 = var4 as i32;
                                                                                                                    'label60: loop {
                                                                                                                        var5 = slot_var3_192_i32;
                                                                                                                        if var5 != 0 {
                                                                                                                            let mut slot_var3_152_i32 = mload32!(var3 as usize + 152) as i32;
                                                                                                                            let mut slot_var3_252_i32 = slot_var3_152_i32 as i32;
                                                                                                                            let mut slot_var3_248_i32 = var5 as i32;
                                                                                                                            slot_var3_240_i32 = 0 as i32;
                                                                                                                            slot_var3_232_i32 = 0 as i32;
                                                                                                                            self.func73(env, var3.wrapping_add(480), var3.wrapping_add(232));
                                                                                                                            let var157 = mload8!(var3 as usize + 480) as i32;
                                                                                                                            if (var157 == 0) as i32 != 0 {
                                                                                                                                break 'label55;
                                                                                                                            }
                                                                                                                            var7 = ENTRY_SIZE;
                                                                                                                            self.func72(env, var3.wrapping_add(8), 4, ENTRY_SIZE);
                                                                                                                            var10 = var8.wrapping_add(8);
                                                                                                                            arg1 = slot_var10_0_i64;
                                                                                                                            var12 = var8.wrapping_add(16);
                                                                                                                            arg2 = slot_var12_0_i64;
                                                                                                                            var23 = var8.wrapping_add(24);
                                                                                                                            let mut slot_var23_0_i64 = mload64!(var23 as usize) as i64;
                                                                                                                            var43 = slot_var23_0_i64;
                                                                                                                            let mut slot_var3_8_i32 = mload32!(var3 as usize + 8) as i32;
                                                                                                                            var5 = slot_var3_8_i32;
                                                                                                                            let mut slot_var3_12_i32 = mload32!(var3 as usize + 12) as i32;
                                                                                                                            var4 = slot_var3_12_i32;
                                                                                                                            let mut slot_var8_0_i64 = mload64!(var8 as usize) as i64;
                                                                                                                            let mut slot_var4_0_i64 = slot_var8_0_i64 as i64;
                                                                                                                            mstore64!(var4.wrapping_add(24) as usize, var43 as u64);
                                                                                                                            mstore64!(var4.wrapping_add(16) as usize, arg2 as u64);
                                                                                                                            mstore64!(var4.wrapping_add(8) as usize, arg1 as u64);
                                                                                                                            var6 = 1;
                                                                                                                            slot_var3_552_i32 = 1 as i32;
                                                                                                                            slot_var3_548_i32 = var4 as i32;
                                                                                                                            slot_var3_544_i32 = var5 as i32;
                                                                                                                            let var159 = mload64!(var3.wrapping_add(248) as usize) as i64;
                                                                                                                            mstore64!(var3.wrapping_add(408) as usize, var159 as u64);
                                                                                                                            let var160 = mload64!(var3.wrapping_add(240) as usize) as i64;
                                                                                                                            mstore64!(var3.wrapping_add(400) as usize, var160 as u64);
                                                                                                                            slot_var3_392_i64 = slot_var3_232_i64 as i64;
                                                                                                                            var11 = 0;
                                                                                                                            'label61: loop {
                                                                                                                                self.func73(env, var3.wrapping_add(480), var3.wrapping_add(392));
                                                                                                                                let var162 = mload8!(var3 as usize + 480) as i32;
                                                                                                                                if (var162 == 1) as i32 != 0 {
                                                                                                                                    if (slot_var3_544_i32 == var6) as i32 != 0 {
                                                                                                                                        self.func60(env, var3.wrapping_add(544), var6, 1, ENTRY_SIZE);
                                                                                                                                        var4 = slot_var3_548_i32;
                                                                                                                                    }
                                                                                                                                    var5 = var4.wrapping_add(var7);
                                                                                                                                    slot_var5_0_i64 = slot_var8_0_i64 as i64;
                                                                                                                                    mstore64!(var5.wrapping_add(24) as usize, slot_var23_0_i64 as u64);
                                                                                                                                    mstore64!(var5.wrapping_add(16) as usize, slot_var12_0_i64 as u64);
                                                                                                                                    mstore64!(var5.wrapping_add(8) as usize, slot_var10_0_i64 as u64);
                                                                                                                                    var6 = var6.wrapping_add(1);
                                                                                                                                    slot_var3_552_i32 = var6 as i32;
                                                                                                                                    var11 = var11.wrapping_add(ENTRY_SIZE);
                                                                                                                                    var7 = var7.wrapping_add(ENTRY_SIZE);
                                                                                                                                    continue 'label61;
                                                                                                                                }
                                                                                                                                break;
                                                                                                                            }
                                                                                                                            if ((var6 as u32) < 2 as u32) as i32 != 0 {
                                                                                                                                break 'label55;
                                                                                                                            }
                                                                                                                            'label62: loop {
                                                                                                                                'label63: loop {
                                                                                                                                    'label64: loop {
                                                                                                                                        'label65: loop {
                                                                                                                                            'label66: loop {
                                                                                                                                                match var6.wrapping_sub(1) {
                                                                                                                                                    0 => break 'label65,
                                                                                                                                                    1 => break 'label64,
                                                                                                                                                    2 => break 'label63,
                                                                                                                                                    _ => break 'label66,
                                                                                                                                                }
                                                                                                                                                break;
                                                                                                                                            }
                                                                                                                                            if (var6 as u32 >= 21 as u32) as i32 != 0 {
                                                                                                                                                var10 = var3.wrapping_add(480);
                                                                                                                                                let var164 = self.global0;
                                                                                                                                                var7 = var164.wrapping_sub(16);
                                                                                                                                                self.global0 = var7;
                                                                                                                                                'label67: loop {
                                                                                                                                                    'label68: loop {
                                                                                                                                                        'label69: loop {
                                                                                                                                                            let var165 = self.func68(env, var4.wrapping_add(ENTRY_SIZE), var4);
                                                                                                                                                            if (var165 & 255 != 255) as i32 != 0 {
                                                                                                                                                                var5 = var4.wrapping_sub(-64);
                                                                                                                                                                var11 = 2;
                                                                                                                                                                'label70: loop {
                                                                                                                                                                    if (var6 == var11) as i32 != 0 {
                                                                                                                                                                        break 'label67;
                                                                                                                                                                    }
                                                                                                                                                                    let var166 = self.func68(env, var5, var5.wrapping_sub(ENTRY_SIZE));
                                                                                                                                                                    if (var166 & 255 == 255) as i32 != 0 {
                                                                                                                                                                        break 'label69;
                                                                                                                                                                    }
                                                                                                                                                                    var5 = var5.wrapping_add(ENTRY_SIZE);
                                                                                                                                                                    var11 = var11.wrapping_add(1);
                                                                                                                                                                    continue 'label70;
                                                                                                                                                                    break;
                                                                                                                                                                }
                                                                                                                                                                unreachable!();
                                                                                                                                                            }
                                                                                                                                                            var5 = var4.wrapping_sub(-64);
                                                                                                                                                            var11 = 2;
                                                                                                                                                            'label71: loop {
                                                                                                                                                                if (var6 == var11) as i32 != 0 {
                                                                                                                                                                    break 'label68;
                                                                                                                                                                }
                                                                                                                                                                let var167 = self.func68(env, var5, var5.wrapping_sub(ENTRY_SIZE));
                                                                                                                                                                if (var167 & 255 != 255) as i32 != 0 {
                                                                                                                                                                    break 'label69;
                                                                                                                                                                }
                                                                                                                                                                var5 = var5.wrapping_add(ENTRY_SIZE);
                                                                                                                                                                var11 = var11.wrapping_add(1);
                                                                                                                                                                continue 'label71;
                                                                                                                                                                break;
                                                                                                                                                            }
                                                                                                                                                            unreachable!();
                                                                                                                                                            break;
                                                                                                                                                        }
                                                                                                                                                        self.func114(env, var4, var6, 0, ((var6 | 1).leading_zeros() as i32).wrapping_shl(1 as u32) ^ 62, var10);
                                                                                                                                                        break 'label67;
                                                                                                                                                        break;
                                                                                                                                                    }
                                                                                                                                                    var10 = (var6 as u32).wrapping_shr(1 as u32) as i32;
                                                                                                                                                    self.func115(env, var7.wrapping_add(8), var10, var4, var10);
                                                                                                                                                    let mut slot_var7_12_i32 = mload32!(var7 as usize + 12) as i32;
                                                                                                                                                    var23 = slot_var7_12_i32;
                                                                                                                                                    let mut slot_var7_8_i32 = mload32!(var7 as usize + 8) as i32;
                                                                                                                                                    var11 = slot_var7_8_i32;
                                                                                                                                                    var5 = var10.wrapping_mul(ENTRY_SIZE);
                                                                                                                                                    self.func115(env, var7, var10, var4.wrapping_add(var6.wrapping_mul(ENTRY_SIZE)).wrapping_sub(var5), var10);
                                                                                                                                                    var12 = var5.wrapping_add(slot_var7_0_i32).wrapping_sub(ENTRY_SIZE);
                                                                                                                                                    var5 = 0;
                                                                                                                                                    let mut slot_var7_4_i32 = mload32!(var7 as usize + 4) as i32;
                                                                                                                                                    var26 = slot_var7_4_i32;
                                                                                                                                                    'label72: loop {
                                                                                                                                                        'label73: loop {
                                                                                                                                                            var27 = var5.wrapping_add(var10);
                                                                                                                                                            if (var27 == 0) as i32 != 0 {
                                                                                                                                                                break 'label67;
                                                                                                                                                            }
                                                                                                                                                            if (var5.wrapping_add(var23) == 0) as i32 != 0 {
                                                                                                                                                                break 'label72;
                                                                                                                                                            }
                                                                                                                                                            if (var26 as u32 > var27.wrapping_sub(1) as u32) as i32 != 0 {
                                                                                                                                                                self.func116(env, var11, var12);
                                                                                                                                                                var11 = var11.wrapping_add(ENTRY_SIZE);
                                                                                                                                                                var12 = var12.wrapping_sub(ENTRY_SIZE);
                                                                                                                                                                var5 = var5.wrapping_sub(1);
                                                                                                                                                                continue 'label73;
                                                                                                                                                            }
                                                                                                                                                            break;
                                                                                                                                                        }
                                                                                                                                                        unreachable!();
                                                                                                                                                        break;
                                                                                                                                                    }
                                                                                                                                                    unreachable!();
                                                                                                                                                    break;
                                                                                                                                                }
                                                                                                                                                self.global0 = var7.wrapping_add(16);
                                                                                                                                                break 'label57;
                                                                                                                                            }
                                                                                                                                            var5 = var4.wrapping_add(ENTRY_SIZE);
                                                                                                                                            'label74: loop {
                                                                                                                                                if (var11 == 0) as i32 != 0 {
                                                                                                                                                    break 'label57;
                                                                                                                                                }
                                                                                                                                                self.func74(env, var4, var5);
                                                                                                                                                var11 = var11.wrapping_sub(ENTRY_SIZE);
                                                                                                                                                var5 = var5.wrapping_add(ENTRY_SIZE);
                                                                                                                                                continue 'label74;
                                                                                                                                                break;
                                                                                                                                            }
                                                                                                                                            unreachable!();
                                                                                                                                            break;
                                                                                                                                        }
                                                                                                                                        let var173 = self.func75(env, var4, 1, 0);
                                                                                                                                        var4 = var173;
                                                                                                                                        copy_payload_3x64(var4, var3.wrapping_add(480));
                                                                                                                                        slot_var3_480_i64 = slot_var4_0_i64 as i64;
                                                                                                                                        break 'label56;
                                                                                                                                        break;
                                                                                                                                    }
                                                                                                                                    let var177 = self.func75(env, var4, 2, 0);
                                                                                                                                    let var178 = self.func75(env, var4, 2, 1);
                                                                                                                                    self.func76(env, var3.wrapping_add(544), var177, var178);
                                                                                                                                    break 'label62;
                                                                                                                                    break;
                                                                                                                                }
                                                                                                                                let var180 = self.func75(env, var4, 3, 0);
                                                                                                                                let var181 = self.func75(env, var4, 3, 1);
                                                                                                                                let var182 = self.func75(env, var4, 3, 2);
                                                                                                                                self.func77(env, var3.wrapping_add(232), var180, var181, var182);
                                                                                                                                let var184 = mload8!(var3 as usize + 232) as i32;
                                                                                                                                if (var184 == 0) as i32 != 0 {
                                                                                                                                    let var185 = self.func75(env, var4, 3, 1);
                                                                                                                                    let var186 = self.func75(env, var4, 3, 0);
                                                                                                                                    let var187 = self.func75(env, var4, 3, 2);
                                                                                                                                    self.func77(env, var3.wrapping_add(392), var185, var186, var187);
                                                                                                                                    let var189 = mload8!(var3 as usize + 392) as i32;
                                                                                                                                    if (var189 == 0) as i32 != 0 {
                                                                                                                                        let var190 = self.func75(env, var4, 3, 1);
                                                                                                                                        let var191 = self.func75(env, var4, 3, 2);
                                                                                                                                        let var192 = self.func75(env, var4, 3, 0);
                                                                                                                                        self.func77(env, var3.wrapping_add(480), var190, var191, var192);
                                                                                                                                        let var194 = mload8!(var3 as usize + 480) as i32;
                                                                                                                                        if (var194 == 0) as i32 != 0 {
                                                                                                                                            break 'label60;
                                                                                                                                        }
                                                                                                                                        mstore64!(var3.wrapping_add(568) as usize, slot_var23_0_i64 as u64);
                                                                                                                                        mstore64!(var3.wrapping_add(560) as usize, slot_var12_0_i64 as u64);
                                                                                                                                        mstore64!(var3.wrapping_add(552) as usize, slot_var10_0_i64 as u64);
                                                                                                                                        let mut slot_var3_544_i64 = slot_var8_0_i64 as i64;
                                                                                                                                        break 'label62;
                                                                                                                                    }
                                                                                                                                    let var195 = mload64!(var20.wrapping_add(24) as usize) as i64;
                                                                                                                                    mstore64!(var3.wrapping_add(568) as usize, var195 as u64);
                                                                                                                                    let var196 = mload64!(var20.wrapping_add(16) as usize) as i64;
                                                                                                                                    mstore64!(var3.wrapping_add(560) as usize, var196 as u64);
                                                                                                                                    let var197 = mload64!(var20.wrapping_add(8) as usize) as i64;
                                                                                                                                    mstore64!(var3.wrapping_add(552) as usize, var197 as u64);
                                                                                                                                    let mut slot_var20_0_i64 = mload64!(var20 as usize) as i64;
                                                                                                                                    slot_var3_544_i64 = slot_var20_0_i64 as i64;
                                                                                                                                    break 'label62;
                                                                                                                                }
                                                                                                                                let var198 = mload64!(var19.wrapping_add(24) as usize) as i64;
                                                                                                                                mstore64!(var3.wrapping_add(568) as usize, var198 as u64);
                                                                                                                                let var199 = mload64!(var19.wrapping_add(16) as usize) as i64;
                                                                                                                                mstore64!(var3.wrapping_add(560) as usize, var199 as u64);
                                                                                                                                let var200 = mload64!(var19.wrapping_add(8) as usize) as i64;
                                                                                                                                mstore64!(var3.wrapping_add(552) as usize, var200 as u64);
                                                                                                                                slot_var3_544_i64 = slot_var19_0_i64 as i64;
                                                                                                                                break;
                                                                                                                            }
                                                                                                                            let var201 = mload64!(var3.wrapping_add(568) as usize) as i64;
                                                                                                                            mstore64!(var3.wrapping_add(504) as usize, var201 as u64);
                                                                                                                            let var202 = mload64!(var3.wrapping_add(560) as usize) as i64;
                                                                                                                            mstore64!(var3.wrapping_add(496) as usize, var202 as u64);
                                                                                                                            let var203 = mload64!(var3.wrapping_add(552) as usize) as i64;
                                                                                                                            mstore64!(var3.wrapping_add(488) as usize, var203 as u64);
                                                                                                                            slot_var3_480_i64 = slot_var3_544_i64 as i64;
                                                                                                                            break 'label56;
                                                                                                                        }
                                                                                                                        let var204 = mload32!(var3.wrapping_add(384) as usize) as i64;
                                                                                                                        arg2 = var204;
                                                                                                                        arg1 = slot_var3_376_i64;
                                                                                                                        var6 = 27;
                                                                                                                        break 'label37;
                                                                                                                        break;
                                                                                                                    }
                                                                                                                    unreachable!();
                                                                                                                    break;
                                                                                                                }
                                                                                                                unreachable!();
                                                                                                                break;
                                                                                                            }
                                                                                                            var5 = (var6 as u32).wrapping_shr(1 as u32) as i32;
                                                                                                            'label75: loop {
                                                                                                                if (var6 & 1 == 0) as i32 != 0 {
                                                                                                                    let var205 = self.func75(env, var4, var6, var5.wrapping_sub(1));
                                                                                                                    let var206 = self.func75(env, var4, var6, var5);
                                                                                                                    self.func76(env, var3.wrapping_add(544), var205, var206);
                                                                                                                    break 'label75;
                                                                                                                }
                                                                                                                let var208 = self.func75(env, var4, var6, var5);
                                                                                                                var4 = var208;
                                                                                                                copy_payload_3x64(var4, var3.wrapping_add(544));
                                                                                                                slot_var3_544_i64 = slot_var4_0_i64 as i64;
                                                                                                                break;
                                                                                                            }
                                                                                                            copy_payload_3x64(var3.wrapping_add(544), var3.wrapping_add(480));
                                                                                                            slot_var3_480_i64 = slot_var3_544_i64 as i64;
                                                                                                            break;
                                                                                                        }
                                                                                                        var5 = var3.wrapping_add(472);
                                                                                                        var4 = var3.wrapping_add(504);
                                                                                                        slot_var5_0_i64 = slot_var4_0_i64 as i64;
                                                                                                        var6 = var3.wrapping_add(464);
                                                                                                        var11 = var3.wrapping_add(496);
                                                                                                        let mut slot_var11_0_i64 = mload64!(var11 as usize) as i64;
                                                                                                        slot_var6_0_i64 = slot_var11_0_i64 as i64;
                                                                                                        var7 = var3.wrapping_add(456);
                                                                                                        var12 = var3.wrapping_add(488);
                                                                                                        let mut slot_var7_0_i64 = slot_var12_0_i64 as i64;
                                                                                                        let mut slot_var3_448_i64 = slot_var3_480_i64 as i64;
                                                                                                        if (var14 as u32 <= var24 as u32) as i32 != 0 {
                                                                                                            break 'label54;
                                                                                                        }
                                                                                                        let var215 = var4;
                                                                                                        var4 = var16.wrapping_add(var24.wrapping_mul(ENTRY_SIZE));
                                                                                                        let var216 = mload64!(var4.wrapping_add(24) as usize) as i64;
                                                                                                        let mut slot_var215_0_i64 = var216 as i64;
                                                                                                        let var217 = mload64!(var4.wrapping_add(16) as usize) as i64;
                                                                                                        slot_var11_0_i64 = var217 as i64;
                                                                                                        let var218 = mload64!(var4.wrapping_add(8) as usize) as i64;
                                                                                                        slot_var12_0_i64 = var218 as i64;
                                                                                                        arg1 = slot_var4_0_i64;
                                                                                                        let mut slot_var22_0_i64 = slot_var3_448_i64 as i64;
                                                                                                        mstore64!(var22.wrapping_add(8) as usize, slot_var7_0_i64 as u64);
                                                                                                        mstore64!(var22.wrapping_add(16) as usize, slot_var6_0_i64 as u64);
                                                                                                        mstore64!(var22.wrapping_add(24) as usize, slot_var5_0_i64 as u64);
                                                                                                        slot_var3_480_i64 = arg1 as i64;
                                                                                                        if (slot_var3_376_i32 == var13) as i32 != 0 {
                                                                                                            self.func46(env, var3.wrapping_add(376));
                                                                                                            var17 = slot_var3_380_i32;
                                                                                                        }
                                                                                                        let var220 = self.func131(env, var17.wrapping_add(var13.wrapping_shl(6 as u32)), var3.wrapping_add(480), 64);
                                                                                                        var220;
                                                                                                        var13 = var13.wrapping_add(1);
                                                                                                        slot_var3_384_i32 = var13 as i32;
                                                                                                        break;
                                                                                                    }
                                                                                                    var5 = var9;
                                                                                                    continue 'label53;
                                                                                                    break;
                                                                                                }
                                                                                                break;
                                                                                            }
                                                                                            unreachable!();
                                                                                            break;
                                                                                        }
                                                                                        arg1 = slot_var6_0_i64;
                                                                                        slot_var3_400_i32 = var12 as i32;
                                                                                        slot_var3_392_i64 = arg1 as i64;
                                                                                        let var221 = self.func131(env, var22, var6.wrapping_add(12), 44);
                                                                                        var221;
                                                                                        'label76: loop {
                                                                                            let var222 = mload8!(var3 as usize + 412) as i32;
                                                                                            var4 = var222;
                                                                                            if (var4 != 1) as i32 != 0 {
                                                                                                break 'label76;
                                                                                            }
                                                                                            var4 = { let a = var18; let b = 0; if var4 != 0 { a } else { b } };
                                                                                            var9 = 0;
                                                                                            var5 = var19;
                                                                                            var6 = var11;
                                                                                            'label77: loop {
                                                                                                if (var5 == 0) as i32 != 0 {
                                                                                                    break 'label76;
                                                                                                }
                                                                                                let var223 = memcmp_32(var6, var4);
                                                                                                if (var223 == 0) as i32 != 0 {
                                                                                                    var5 = var5.wrapping_sub(ENTRY_SIZE);
                                                                                                    var9 = var9.wrapping_add(1);
                                                                                                    var6 = var6.wrapping_add(ENTRY_SIZE);
                                                                                                    continue 'label77;
                                                                                                }
                                                                                                break;
                                                                                            }
                                                                                            let mut slot_var3_408_i32 = mload32!(var3 as usize + 408) as i32;
                                                                                            var5 = slot_var3_408_i32;
                                                                                            var4 = slot_var3_404_i32;
                                                                                            slot_var3_456_i32 = var12 as i32;
                                                                                            slot_var3_452_i32 = var4 as i32;
                                                                                            slot_var3_448_i32 = var4 as i32;
                                                                                            var23 = var4.wrapping_add(var5.wrapping_shl(6 as u32));
                                                                                            let mut slot_var3_460_i32 = var23 as i32;
                                                                                            'label78: loop {
                                                                                                if (var4 == var23) as i32 != 0 {
                                                                                                    slot_var3_452_i32 = var4 as i32;
                                                                                                    break 'label76;
                                                                                                }
                                                                                                var26 = var3.wrapping_add(552);
                                                                                                var5 = var4.wrapping_add(40);
                                                                                                let mut slot_var26_0_i64 = slot_var5_0_i64 as i64;
                                                                                                var27 = var3.wrapping_add(560);
                                                                                                var6 = var4.wrapping_add(48);
                                                                                                let mut slot_var27_0_i64 = slot_var6_0_i64 as i64;
                                                                                                var24 = var3.wrapping_add(568);
                                                                                                var7 = var4.wrapping_add(56);
                                                                                                let mut slot_var24_0_i64 = slot_var7_0_i64 as i64;
                                                                                                let mut slot_var4_32_i64 = mload64!(var4 as usize + 32) as i64;
                                                                                                slot_var3_544_i64 = slot_var4_32_i64 as i64;
                                                                                                arg1 = slot_var4_15_i64;
                                                                                                arg2 = slot_var4_23_i32;
                                                                                                let var224 = mload8!(var4 as usize + 31) as i32;
                                                                                                var12 = var224;
                                                                                                slot_var8_0_i64 = slot_var4_32_i64 as i64;
                                                                                                mstore64!(var8.wrapping_add(8) as usize, slot_var5_0_i64 as u64);
                                                                                                mstore64!(var8.wrapping_add(16) as usize, slot_var6_0_i64 as u64);
                                                                                                mstore64!(var8.wrapping_add(24) as usize, slot_var7_0_i64 as u64);
                                                                                                mstore8!(var3 as usize + 511, var12 as u8);
                                                                                                let mut slot_var3_503_i64 = arg2 as i64;
                                                                                                let mut slot_var3_495_i64 = arg1 as i64;
                                                                                                var5 = var4.wrapping_add(7);
                                                                                                let mut slot_var3_487_i64 = slot_var5_0_i64 as i64;
                                                                                                slot_var3_480_i64 = slot_var4_0_i64 as i64;
                                                                                                var10 = var4.wrapping_sub(-64);
                                                                                                'label79: loop {
                                                                                                    'label80: loop {
                                                                                                        let var225 = memcmp_32(var8, ENTRY_ZERO_PTR);
                                                                                                        if var225 != 0 {
                                                                                                            break 'label80;
                                                                                                        }
                                                                                                        mstore8!(var3 as usize + 263, var12 as u8);
                                                                                                        let mut slot_var3_255_i64 = arg2 as i64;
                                                                                                        let mut slot_var3_247_i64 = arg1 as i64;
                                                                                                        slot_var3_232_i64 = slot_var4_0_i64 as i64;
                                                                                                        let mut slot_var3_239_i64 = slot_var5_0_i64 as i64;
                                                                                                        var7 = 0;
                                                                                                        var5 = var17;
                                                                                                        var6 = var16;
                                                                                                        'label81: loop {
                                                                                                            if (var5 == 0) as i32 != 0 {
                                                                                                                break 'label80;
                                                                                                            }
                                                                                                            let var226 = memcmp_32(var6, var3.wrapping_add(232));
                                                                                                            if (var226 == 0) as i32 != 0 {
                                                                                                                var5 = var5.wrapping_sub(ENTRY_SIZE);
                                                                                                                var7 = var7.wrapping_add(1);
                                                                                                                var6 = var6.wrapping_add(ENTRY_SIZE);
                                                                                                                continue 'label81;
                                                                                                            }
                                                                                                            break;
                                                                                                        }
                                                                                                        var43 = (var7 as u32 as i64).wrapping_mul(var42);
                                                                                                        if (var43 as u64).wrapping_shr(32 as u32) as i64 as i32 != 0 {
                                                                                                            break 'label39;
                                                                                                        }
                                                                                                        var6 = var43 as i32;
                                                                                                        var5 = var6.wrapping_add(var9);
                                                                                                        if ((var5 as u32) < var6 as u32) as i32 != 0 {
                                                                                                            break 'label39;
                                                                                                        }
                                                                                                        if (var5 as u32 >= var21 as u32) as i32 != 0 {
                                                                                                            unreachable!();
                                                                                                        }
                                                                                                        var5 = var25.wrapping_add(var5.wrapping_mul(33));
                                                                                                        let var227 = mload8!(var5 as usize) as i32;
                                                                                                        if var227 != 0 {
                                                                                                            break 'label79;
                                                                                                        }
                                                                                                        mstore8!(var5 as usize, 1 as u8);
                                                                                                        slot_var5_1_i64 = slot_var3_544_i64 as i64;
                                                                                                        mstore64!(var5.wrapping_add(9) as usize, slot_var26_0_i64 as u64);
                                                                                                        mstore64!(var5.wrapping_add(17) as usize, slot_var27_0_i64 as u64);
                                                                                                        mstore64!(var5.wrapping_add(25) as usize, slot_var24_0_i64 as u64);
                                                                                                        break;
                                                                                                    }
                                                                                                    var4 = var10;
                                                                                                    continue 'label78;
                                                                                                    break;
                                                                                                }
                                                                                                break;
                                                                                            }
                                                                                            slot_var3_452_i32 = var10 as i32;
                                                                                            let mut slot_var3_336_i64 = slot_var4_0_i64 as i64;
                                                                                            let var228 = mload64!(var4.wrapping_add(7) as usize) as i64;
                                                                                            let mut slot_var3_343_i64 = var228 as i64;
                                                                                            var42 = slot_var3_336_i64;
                                                                                            slot_var3_304_i64 = var42 as i64;
                                                                                            slot_var3_311_i64 = slot_var3_343_i64 as i64;
                                                                                            let var229 = mload8!(var3 as usize + 310) as i64;
                                                                                            let var230 = mload16!(var3 as usize + 308) as i64;
                                                                                            var5 = ((var42 & 4294967295 | var229.wrapping_shl(48 as u32) | var230.wrapping_shl(32 as u32)) as u64).wrapping_shr(24 as u32) as i64 as i32;
                                                                                            var41 = slot_var3_311_i64;
                                                                                            var8 = var42 as i32;
                                                                                            var6 = 11;
                                                                                            break 'label37;
                                                                                            break;
                                                                                        }
                                                                                        var6 = var13;
                                                                                        continue 'label51;
                                                                                        break;
                                                                                    }
                                                                                    unreachable!();
                                                                                    break;
                                                                                }
                                                                                unreachable!();
                                                                                break;
                                                                            }
                                                                            mstore8!(var3 as usize + 240, var7 as u8);
                                                                            slot_var3_236_i32 = -2147483648 as i32;
                                                                            break 'label36;
                                                                            break;
                                                                        }
                                                                        let mut slot_var3_240_i64 = slot_var3_380_i32 as i64;
                                                                        slot_var3_236_i32 = var7 as i32;
                                                                        break 'label36;
                                                                        break;
                                                                    }
                                                                    unreachable!();
                                                                    break;
                                                                }
                                                                var5 = 0;
                                                                break;
                                                            }
                                                            var42 = (arg1 as u64).wrapping_shr(32 as u32) as i64;
                                                            if (var6 != 27) as i32 != 0 {
                                                                var9 = (arg1 as u64).wrapping_shr(40 as u32) as i64 as i32;
                                                                var13 = var42 as i32;
                                                                var15 = arg1 as i32;
                                                                break 'label25;
                                                            }
                                                            var14 = var42 as i32;
                                                            var4 = var14.wrapping_add((arg2 as i32).wrapping_shl(6 as u32));
                                                            let var231 = val_to_i64(Vec::<Val>::new(env).into_val(env))
                                                            arg1 = var231;
                                                            'label82: loop {
                                                                if (var4 == var14) as i32 != 0 {
                                                                    mstore64!(arg0 as usize + 16, arg1 as u64);
                                                                    mstore64!(arg0 as usize + 8, var41 as u64);
                                                                    mstore8!(arg0 as usize, 27 as u8);
                                                                    break 'label24;
                                                                }
                                                                let var232 = self.func131(env, var3.wrapping_add(480), var14, 64);
                                                                var232;
                                                                let var233 = mload64!(var14.wrapping_add(56) as usize) as i64;
                                                                mstore64!(var3.wrapping_add(256) as usize, var233 as u64);
                                                                let var234 = mload64!(var14.wrapping_add(48) as usize) as i64;
                                                                mstore64!(var3.wrapping_add(248) as usize, var234 as u64);
                                                                let var235 = mload64!(var14.wrapping_add(40) as usize) as i64;
                                                                mstore64!(var3.wrapping_add(240) as usize, var235 as u64);
                                                                let mut slot_var14_32_i64 = mload64!(var14 as usize + 32) as i64;
                                                                slot_var3_232_i64 = slot_var14_32_i64 as i64;
                                                                let var236 = val_to_i64(Bytes::new(env).into_val(env)) /* TODO: linear memory */;
                                                                let var237 = val_to_i64(U256::from_be_bytes(env, &Bytes::from_val(env, &val_from_i64(var236))).into_val(env))
                                                                arg2 = var237;
                                                                let var238 = mload64!(var3.wrapping_add(504) as usize) as i64;
                                                                mstore64!(var3.wrapping_add(416) as usize, var238 as u64);
                                                                let var239 = mload64!(var3.wrapping_add(496) as usize) as i64;
                                                                mstore64!(var3.wrapping_add(408) as usize, var239 as u64);
                                                                let var240 = mload64!(var3.wrapping_add(488) as usize) as i64;
                                                                mstore64!(var3.wrapping_add(400) as usize, var240 as u64);
                                                                slot_var3_392_i64 = slot_var3_480_i64 as i64;
                                                                var14 = var14.wrapping_sub(-64);
                                                                var6 = 33;
                                                                'label83: loop {
                                                                    'label84: loop {
                                                                        var5 = 0;
                                                                        if (var6 == 1) as i32 != 0 {
                                                                            var7 = 0;
                                                                            break 'label84;
                                                                        }
                                                                        var7 = var6.wrapping_sub(1);
                                                                        var6 = var7;
                                                                        let var241 = mload8!(var3.wrapping_add(392).wrapping_add(var6).wrapping_sub(2) as usize) as i32;
                                                                        if (var241 == 0) as i32 != 0 {
                                                                            continue 'label83;
                                                                        }
                                                                        break;
                                                                    }
                                                                    break;
                                                                }
                                                                'label85: loop {
                                                                    'label86: loop {
                                                                        if (var5 == var7) as i32 != 0 {
                                                                            var5 = 0;
                                                                            break 'label86;
                                                                        }
                                                                        let var242 = mload8!(var3.wrapping_add(392).wrapping_add(var5) as usize) as i32;
                                                                        if var242 != 0 {
                                                                            break 'label86;
                                                                        }
                                                                        var5 = var5.wrapping_add(1);
                                                                        continue 'label85;
                                                                        break;
                                                                    }
                                                                    break;
                                                                }
                                                                if (var5 as u32 <= var7 as u32) as i32 != 0 {
                                                                    var6 = var3.wrapping_add(392);
                                                                    let var243 = val_to_i64(String::from_str(env, "")) /* TODO: linear memory */;
                                                                    var42 = var243;
                                                                    let mut slot_var3_400_i64 = arg2 as i64;
                                                                    slot_var3_392_i64 = var42 as i64;
                                                                    let var244 = val_to_i64(Vec::<Val>::new(env).into_val(env)) /* TODO: linear memory */;
                                                                    let var245 = { let mut v = Vec::<Val>::from_val(env, &val_from_i64(arg1)); v.push_back(val_from_i64(var244)); val_to_i64(v.into_val(env)) }
                                                                    arg1 = var245;
                                                                    continue 'label82;
                                                                }
                                                                break;
                                                            }
                                                            unreachable!();
                                                            break;
                                                        }
                                                        mstore8!(var3 as usize + 232, 1 as u8);
                                                        var17 = 0;
                                                        var10 = var9;
                                                        var42 = arg2;
                                                        var105 = var12;
                                                        break;
                                                    }
                                                    var19 = var105;
                                                    var6 = 23;
                                                    if (var46 as u64 > 4294967295 as u64) as i32 != 0 {
                                                        break 'label34;
                                                    }
                                                    if (var44 as u64 > 4294967295 as u64) as i32 != 0 {
                                                        break 'label28;
                                                    }
                                                    'label87: loop {
                                                        'label88: loop {
                                                            var5 = var46 as i32;
                                                            if ((var5.wrapping_sub(65536) as u32) < -65535 as u32) as i32 != 0 {
                                                                var6 = 4;
                                                                break 'label88;
                                                            }
                                                            var26 = var26.wrapping_add(1);
                                                            var36 = var44 as i32;
                                                            self.func45(env, var3.wrapping_add(32), var5, 1, 64);
                                                            let mut slot_var3_32_i64 = mload64!(var3 as usize + 32) as i64;
                                                            let mut slot_var3_268_i64 = slot_var3_32_i64 as i64;
                                                            var6 = 0;
                                                            var18 = 0;
                                                            'label89: loop {
                                                                'label90: loop {
                                                                    let mut slot_var3_276_i32 = var6 as i32;
                                                                    if (var5 == var18) as i32 != 0 {
                                                                        break 'label87;
                                                                    }
                                                                    var4 = var3.wrapping_add(480);
                                                                    var9 = var3.wrapping_add(80);
                                                                    self.func65(env, var4, var9, var36);
                                                                    let var248 = mload8!(var3 as usize + 480) as i32;
                                                                    var6 = var248;
                                                                    if (var6 == 27) as i32 != 0 {
                                                                        var44 = slot_var3_488_i32;
                                                                        var28 = slot_var3_484_i32;
                                                                        self.func65(env, var4, var9, 32);
                                                                        'label91: loop {
                                                                            let var250: i32;
                                                                            'label92: loop {
                                                                                let var251 = mload8!(var3 as usize + 480) as i32;
                                                                                var6 = var251;
                                                                                if (var6 == 27) as i32 != 0 {
                                                                                    arg2 = slot_var3_488_i32;
                                                                                    var9 = arg2 as i32;
                                                                                    'label93: loop {
                                                                                        if (arg2 as u64 >= 4294967296 as u64) as i32 != 0 {
                                                                                            var4 = (arg2 as u64).wrapping_shr(32 as u32) as i64 as i32;
                                                                                            var7 = 0;
                                                                                            'label94: loop {
                                                                                                'label95: loop {
                                                                                                    if (var4 == var7) as i32 != 0 {
                                                                                                        break 'label94;
                                                                                                    }
                                                                                                    var12 = var7.wrapping_add(var9);
                                                                                                    let var252 = mload8!(var12 as usize) as i32;
                                                                                                    if (var252 == 0) as i32 != 0 {
                                                                                                        var7 = var7.wrapping_add(1);
                                                                                                        continue 'label95;
                                                                                                    }
                                                                                                    break;
                                                                                                }
                                                                                                var13 = var9.wrapping_sub(1);
                                                                                                'label96: loop {
                                                                                                    var9 = var4;
                                                                                                    if (var9 == 0) as i32 != 0 {
                                                                                                        break 'label94;
                                                                                                    }
                                                                                                    var4 = var4.wrapping_sub(1);
                                                                                                    let var253 = mload8!(var9.wrapping_add(var13) as usize) as i32;
                                                                                                    if (var253 == 0) as i32 != 0 {
                                                                                                        continue 'label96;
                                                                                                    }
                                                                                                    break;
                                                                                                }
                                                                                                if (var7 as u32 <= var9 as u32) as i32 != 0 {
                                                                                                    self.func69(env, var3.wrapping_add(352), var12, var9.wrapping_sub(var7));
                                                                                                    break 'label93;
                                                                                                }
                                                                                                unreachable!();
                                                                                                break;
                                                                                            }
                                                                                            let mut slot_var3_360_i32 = 0 as i32;
                                                                                            let mut slot_var3_352_i64 = 4294967296 as i64;
                                                                                            break 'label93;
                                                                                        }
                                                                                        var4 = slot_var3_484_i32;
                                                                                        slot_var3_360_i32 = 0 as i32;
                                                                                        let mut slot_var3_356_i32 = var9 as i32;
                                                                                        let mut slot_var3_352_i32 = var4 as i32;
                                                                                        break;
                                                                                    }
                                                                                    self.func79(env, var3.wrapping_add(364), var3.wrapping_add(352));
                                                                                    mstore64!(var3.wrapping_add(504) as usize, 0 /* False */ as u64);
                                                                                    mstore64!(var3.wrapping_add(496) as usize, 0 /* False */ as u64);
                                                                                    mstore64!(var3.wrapping_add(488) as usize, 0 /* False */ as u64);
                                                                                    slot_var3_480_i64 = 0 /* False */ as i64;
                                                                                    let mut slot_var3_372_i32 = mload32!(var3 as usize + 372) as i32;
                                                                                    var4 = slot_var3_372_i32;
                                                                                    self.func80(env, var3.wrapping_add(24), var3.wrapping_add(480), var4);
                                                                                    let mut slot_var3_24_i32 = mload32!(var3 as usize + 24) as i32;
                                                                                    let mut slot_var3_28_i32 = mload32!(var3 as usize + 28) as i32;
                                                                                    let mut slot_var3_368_i32 = mload32!(var3 as usize + 368) as i32;
                                                                                    self.func81(env, slot_var3_24_i32, slot_var3_28_i32, slot_var3_368_i32, var4);
                                                                                    let var258 = mload16!(var3 as usize + 500) as i32;
                                                                                    let var259 = mload8!(var23 as usize) as i32;
                                                                                    var9 = var258 | var259.wrapping_shl(16 as u32);
                                                                                    let var260 = mload16!(var3 as usize + 480) as i32;
                                                                                    let var261 = mload8!(var3 as usize + 482) as i32;
                                                                                    var8 = var260 | var261.wrapping_shl(16 as u32);
                                                                                    let var262 = mload8!(var3 as usize + 511) as i32;
                                                                                    var12 = var262;
                                                                                    arg2 = slot_var3_503_i64;
                                                                                    let var263 = mload8!(var3 as usize + 499) as i32;
                                                                                    var13 = var263;
                                                                                    var7 = slot_var3_495_i64;
                                                                                    var41 = slot_var3_487_i64;
                                                                                    let mut slot_var3_483_i32 = mload32!(var3 as usize + 483) as i32;
                                                                                    var4 = slot_var3_483_i32;
                                                                                    if (var6 == 27) as i32 != 0 {
                                                                                        break 'label91;
                                                                                    }
                                                                                    var250 = var4;
                                                                                    break 'label92;
                                                                                }
                                                                                slot_var3_232_i64 = slot_var20_0_i64 as i64;
                                                                                let var264 = mload64!(var20.wrapping_add(7) as usize) as i64;
                                                                                slot_var3_239_i64 = var264 as i64;
                                                                                let var265 = mload16!(var3 as usize + 481) as i32;
                                                                                let var266 = mload8!(var3 as usize + 483) as i32;
                                                                                var8 = var265 | var266.wrapping_shl(16 as u32);
                                                                                let var267 = mload16!(var3 as usize + 501) as i32;
                                                                                let var268 = mload8!(var3.wrapping_add(503) as usize) as i32;
                                                                                var9 = var267 | var268.wrapping_shl(16 as u32);
                                                                                let var269 = mload8!(var3 as usize + 512) as i32;
                                                                                var12 = var269;
                                                                                let mut slot_var3_504_i64 = mload64!(var3 as usize + 504) as i64;
                                                                                arg2 = slot_var3_504_i64;
                                                                                let var270 = mload8!(var3 as usize + 500) as i32;
                                                                                var13 = var270;
                                                                                let mut slot_var3_496_i32 = mload32!(var3 as usize + 496) as i32;
                                                                                var7 = slot_var3_496_i32;
                                                                                var41 = slot_var3_488_i32;
                                                                                var250 = slot_var3_484_i32;
                                                                                break;
                                                                            }
                                                                            var5 = var250;
                                                                            let mut slot_var3_455_i64 = slot_var3_239_i64 as i64;
                                                                            slot_var3_448_i64 = slot_var3_232_i64 as i64;
                                                                            break 'label89;
                                                                            break;
                                                                        }
                                                                        mstore16!(var3 as usize + 500, var9 as u16);
                                                                        mstore8!(var23 as usize, (var9 as u32).wrapping_shr(16 as u32) as i32 as u8);
                                                                        mstore8!(var3 as usize + 511, var12 as u8);
                                                                        slot_var3_503_i64 = arg2 as i64;
                                                                        mstore8!(var3 as usize + 499, var13 as u8);
                                                                        let mut slot_var3_495_i32 = var7 as i32;
                                                                        slot_var3_487_i64 = var41 as i64;
                                                                        slot_var3_483_i32 = var4 as i32;
                                                                        mstore16!(var3 as usize + 480, var8 as u16);
                                                                        mstore8!(var3 as usize + 482, (var8 as u32).wrapping_shr(16 as u32) as i32 as u8);
                                                                        let mut slot_var3_236_i64 = var44 as i64;
                                                                        slot_var3_232_i32 = var28 as i32;
                                                                        self.func82(env, var29, var3.wrapping_add(232));
                                                                        let mut slot_var3_296_i32 = slot_var3_480_i64 as i32;
                                                                        let mut slot_var3_299_i32 = slot_var3_483_i32 as i32;
                                                                        slot_var3_448_i64 = slot_var24_0_i64 as i64;
                                                                        let var272 = mload64!(var24.wrapping_add(7) as usize) as i64;
                                                                        slot_var3_455_i64 = var272 as i64;
                                                                        let var273 = mload8!(var34 as usize) as i32;
                                                                        var9 = var273;
                                                                        let var274 = mload8!(var35 as usize) as i32;
                                                                        var7 = var274;
                                                                        let var275 = mload8!(var3 as usize + 487) as i32;
                                                                        var12 = var275;
                                                                        let mut slot_var3_491_i32 = mload32!(var3 as usize + 491) as i32;
                                                                        var13 = slot_var3_491_i32;
                                                                        arg2 = slot_var3_495_i64;
                                                                        var8 = slot_var3_503_i64;
                                                                        let var276 = mload8!(var3 as usize + 507) as i32;
                                                                        var28 = var276;
                                                                        let mut slot_var3_511_i64 = mload64!(var3 as usize + 511) as i64;
                                                                        var41 = slot_var3_511_i64;
                                                                        let var277 = mload8!(var3 as usize + 519) as i32;
                                                                        var37 = var277;
                                                                        let var278 = mload16!(var3 as usize + 488) as i32;
                                                                        var38 = var278;
                                                                        let var279 = mload16!(var3 as usize + 508) as i32;
                                                                        var39 = var279;
                                                                        var4 = var3.wrapping_add(400);
                                                                        let var280 = mload8!(var25.wrapping_add(8) as usize) as i32;
                                                                        mstore8!(var4 as usize, var280 as u8);
                                                                        var40 = var3.wrapping_add(288);
                                                                        let var281 = mload8!(var4 as usize) as i32;
                                                                        mstore8!(var40 as usize, var281 as u8);
                                                                        let mut slot_var25_0_i64 = mload64!(var25 as usize) as i64;
                                                                        slot_var3_392_i64 = slot_var25_0_i64 as i64;
                                                                        let mut slot_var3_383_i64 = slot_var3_455_i64 as i64;
                                                                        slot_var3_376_i64 = slot_var3_448_i64 as i64;
                                                                        let mut slot_var3_280_i64 = slot_var3_392_i64 as i64;
                                                                        slot_var3_544_i64 = slot_var3_376_i64 as i64;
                                                                        let mut slot_var3_551_i64 = slot_var3_383_i64 as i64;
                                                                        var6 = slot_var3_276_i32;
                                                                        if (var6 == slot_var3_268_i64) as i32 != 0 {
                                                                            self.func46(env, var3.wrapping_add(268));
                                                                        }
                                                                        var18 = var18.wrapping_add(1);
                                                                        let mut slot_var3_272_i32 = mload32!(var3 as usize + 272) as i32;
                                                                        var4 = slot_var3_272_i32.wrapping_add(var6.wrapping_shl(6 as u32));
                                                                        var7 = var39 | var7.wrapping_shl(16 as u32);
                                                                        mstore16!(var4 as usize + 28, var7 as u16);
                                                                        var9 = var38 | var9.wrapping_shl(16 as u32);
                                                                        mstore16!(var4 as usize + 8, var9 as u16);
                                                                        slot_var4_0_i32 = slot_var3_296_i32 as i32;
                                                                        mstore8!(var4 as usize + 39, var37 as u8);
                                                                        let mut slot_var4_31_i64 = var41 as i64;
                                                                        mstore8!(var4 as usize + 27, var28 as u8);
                                                                        slot_var4_23_i32 = var8 as i32;
                                                                        slot_var4_15_i64 = arg2 as i64;
                                                                        slot_var4_11_i32 = var13 as i32;
                                                                        mstore8!(var4 as usize + 7, var12 as u8);
                                                                        let mut slot_var4_40_i64 = slot_var3_544_i64 as i64;
                                                                        mstore8!(var4.wrapping_add(30) as usize, (var7 as u32).wrapping_shr(16 as u32) as i32 as u8);
                                                                        mstore8!(var4.wrapping_add(10) as usize, (var9 as u32).wrapping_shr(16 as u32) as i32 as u8);
                                                                        mstore32!(var4.wrapping_add(3) as usize, slot_var3_299_i32 as u32);
                                                                        mstore64!(var4.wrapping_add(47) as usize, slot_var3_551_i64 as u64);
                                                                        let var283 = mload8!(var40 as usize) as i32;
                                                                        mstore8!(var4.wrapping_add(63) as usize, var283 as u8);
                                                                        let mut slot_var4_55_i64 = slot_var3_280_i64 as i64;
                                                                        var6 = var6.wrapping_add(1);
                                                                        continue 'label90;
                                                                    }
                                                                    break;
                                                                }
                                                                let mut slot_var3_513_i64 = mload64!(var3 as usize + 513) as i64;
                                                                slot_var3_448_i64 = slot_var3_513_i64 as i64;
                                                                let var284 = mload64!(var3.wrapping_add(520) as usize) as i64;
                                                                slot_var3_455_i64 = var284 as i64;
                                                                let var285 = mload16!(var3 as usize + 481) as i32;
                                                                let var286 = mload8!(var3 as usize + 483) as i32;
                                                                var8 = var285 | var286.wrapping_shl(16 as u32);
                                                                let var287 = mload16!(var3 as usize + 501) as i32;
                                                                let var288 = mload8!(var3.wrapping_add(503) as usize) as i32;
                                                                var9 = var287 | var288.wrapping_shl(16 as u32);
                                                                let var289 = mload8!(var3 as usize + 512) as i32;
                                                                var12 = var289;
                                                                arg2 = slot_var3_504_i64;
                                                                let var290 = mload8!(var3 as usize + 500) as i32;
                                                                var13 = var290;
                                                                var7 = slot_var3_496_i32;
                                                                var41 = slot_var3_488_i32;
                                                                var5 = slot_var3_484_i32;
                                                                break;
                                                            }
                                                            slot_var3_383_i64 = slot_var3_455_i64 as i64;
                                                            slot_var3_376_i64 = slot_var3_448_i64 as i64;
                                                            slot_var3_336_i64 = slot_var3_376_i64 as i64;
                                                            slot_var3_343_i64 = slot_var3_383_i64 as i64;
                                                            break;
                                                        }
                                                        let mut slot_var3_176_i32 = var7 as i32;
                                                        let mut slot_var3_152_i64 = slot_var3_336_i64 as i64;
                                                        let mut slot_var3_159_i64 = slot_var3_343_i64 as i64;
                                                        break 'label28;
                                                        break;
                                                    }
                                                    var6 = var3.wrapping_add(188);
                                                    let var291 = mload8!(var3.wrapping_add(228) as usize) as i32;
                                                    mstore8!(var6 as usize, var291 as u8);
                                                    slot_var3_152_i64 = slot_var3_304_i64 as i64;
                                                    slot_var3_159_i64 = slot_var3_311_i64 as i64;
                                                    let mut slot_var3_184_i32 = slot_var3_224_i32 as i32;
                                                    arg2 = slot_var3_272_i32;
                                                    let mut slot_var3_172_i64 = arg2 as i64;
                                                    let mut slot_var3_168_i32 = slot_var3_268_i64 as i32;
                                                    var41 = slot_var3_168_i32;
                                                    let mut slot_var3_135_i64 = slot_var3_159_i64 as i64;
                                                    let mut slot_var3_128_i64 = slot_var3_152_i64 as i64;
                                                    var9 = var3.wrapping_add(182);
                                                    let var292 = mload8!(var3.wrapping_add(151) as usize) as i32;
                                                    mstore8!(var9 as usize, var292 as u8);
                                                    let var293 = mload16!(var3 as usize + 149) as i32;
                                                    mstore16!(var3 as usize + 180, var293 as u16);
                                                    let mut slot_var3_199_i64 = slot_var3_135_i64 as i64;
                                                    let mut slot_var3_192_i64 = slot_var3_128_i64 as i64;
                                                    var7 = slot_var3_124_i32;
                                                    if (var7 == slot_var3_116_i64) as i32 != 0 {
                                                        let var294 = self.global0;
                                                        var4 = var294.wrapping_sub(16);
                                                        self.global0 = var4;
                                                        var5 = var3.wrapping_add(116);
                                                        self.func47(env, var4.wrapping_add(8), var5, slot_var5_0_i64, 1, 8, 56);
                                                        let mut slot_var4_8_i32 = mload32!(var4 as usize + 8) as i32;
                                                        var5 = slot_var4_8_i32;
                                                        if (var5 != -2147483647) as i32 != 0 {
                                                            let mut slot_var4_12_i32 = mload32!(var4 as usize + 12) as i32;
                                                            self.func48(env, var5, slot_var4_12_i32);
                                                            unreachable!();
                                                        }
                                                        self.global0 = var4.wrapping_add(16);
                                                    }
                                                    var5 = (var47 as u64).wrapping_shr(32 as u32) as i64 as i32;
                                                    var8 = (var47 as u64).wrapping_shr(0 as u32) as i64 as i32;
                                                    var4 = slot_var3_120_i64.wrapping_add(var7.wrapping_mul(56));
                                                    mstore16!(var4 as usize + 21, var10 as u16);
                                                    mstore8!(var4 as usize + 32, var19 as u8);
                                                    let mut slot_var4_24_i64 = var42 as i64;
                                                    mstore8!(var4 as usize + 20, var17 as u8);
                                                    mstore32!(var4 as usize + 16, (arg2 as u64).wrapping_shr(32 as u32) as i64 as u32);
                                                    let mut slot_var4_8_i64 = var41 as i64;
                                                    slot_var4_0_i64 = var47 as i64;
                                                    let mut slot_var4_33_i64 = slot_var3_192_i64 as i64;
                                                    let mut slot_var4_48_i32 = slot_var3_184_i32 as i32;
                                                    let var297 = mload16!(var3 as usize + 180) as i32;
                                                    mstore16!(var4 as usize + 53, var297 as u16);
                                                    mstore8!(var4.wrapping_add(23) as usize, (var10 as u32).wrapping_shr(16 as u32) as i32 as u8);
                                                    mstore64!(var4.wrapping_add(40) as usize, slot_var3_199_i64 as u64);
                                                    let var298 = mload8!(var6 as usize) as i32;
                                                    mstore8!(var4.wrapping_add(52) as usize, var298 as u8);
                                                    let var299 = mload8!(var9 as usize) as i32;
                                                    mstore8!(var4.wrapping_add(55) as usize, var299 as u8);
                                                    var6 = var7.wrapping_add(1);
                                                    var9 = var10;
                                                    arg2 = var42;
                                                    var12 = var19;
                                                    var13 = var17;
                                                    continue 'label33;
                                                    break;
                                                }
                                                break;
                                            }
                                            break 'label28;
                                            break;
                                        }
                                        unreachable!();
                                    }
                                    mstore32!(arg0 as usize + 44, var14 as u32);
                                    mstore32!(arg0 as usize + 40, var16 as u32);
                                    mstore32!(arg0 as usize + 36, var12 as u32);
                                    mstore32!(arg0 as usize + 32, var15 as u32);
                                    mstore32!(arg0 as usize + 28, var11 as u32);
                                    mstore32!(arg0 as usize + 24, var9 as u32);
                                    mstore64!(arg0 as usize + 16, var43 as u64);
                                    mstore64!(arg0 as usize + 8, var48 as u64);
                                    mstore64!(arg0 as usize, arg1 as u64);
                                    break 'label24;
                                    break;
                                }
                                let mut slot_var3_320_i64 = slot_var3_513_i64 as i64;
                                let var300 = mload64!(var3.wrapping_add(520) as usize) as i64;
                                let mut slot_var3_327_i64 = var300 as i64;
                                let var301 = mload32!(var3 as usize + 481) as i64;
                                arg1 = var301;
                                let var302 = mload8!(var3 as usize + 487) as i64;
                                let var303 = mload16!(var3 as usize + 485) as i64;
                                var5 = ((arg1 | var302.wrapping_shl(48 as u32) | var303.wrapping_shl(32 as u32)) as u64).wrapping_shr(24 as u32) as i64 as i32;
                                let var304 = mload16!(var3 as usize + 501) as i32;
                                let var305 = mload8!(var3.wrapping_add(503) as usize) as i32;
                                var9 = var304 | var305.wrapping_shl(16 as u32);
                                let var306 = mload8!(var3 as usize + 512) as i32;
                                var12 = var306;
                                arg2 = slot_var3_504_i64;
                                let var307 = mload8!(var3 as usize + 500) as i32;
                                var13 = var307;
                                var15 = slot_var3_496_i32;
                                var8 = arg1 as i32;
                                break 'label26;
                                break;
                            }
                            slot_var3_176_i32 = slot_var3_496_i32 as i32;
                            slot_var3_152_i64 = slot_var3_513_i64 as i64;
                            let var308 = mload64!(var3.wrapping_add(520) as usize) as i64;
                            slot_var3_159_i64 = var308 as i64;
                            let var309 = mload16!(var3 as usize + 481) as i32;
                            let var310 = mload8!(var3 as usize + 483) as i32;
                            var8 = var309 | var310.wrapping_shl(16 as u32);
                            let var311 = mload16!(var3 as usize + 501) as i32;
                            let var312 = mload8!(var3.wrapping_add(503) as usize) as i32;
                            var9 = var311 | var312.wrapping_shl(16 as u32);
                            var41 = slot_var3_488_i32;
                            var5 = slot_var3_484_i32;
                            let var313 = mload8!(var3 as usize + 500) as i32;
                            var13 = var313;
                            arg2 = slot_var3_504_i64;
                            let var314 = mload8!(var3 as usize + 512) as i32;
                            var12 = var314;
                            break;
                        }
                        slot_var3_135_i64 = slot_var3_159_i64 as i64;
                        slot_var3_128_i64 = slot_var3_152_i64 as i64;
                        slot_var3_320_i64 = slot_var3_128_i64 as i64;
                        slot_var3_327_i64 = slot_var3_135_i64 as i64;
                        var15 = slot_var3_176_i32;
                        break 'label25;
                        break;
                    }
                    slot_var3_320_i64 = slot_var3_513_i64 as i64;
                    let var315 = mload64!(var3.wrapping_add(520) as usize) as i64;
                    slot_var3_327_i64 = var315 as i64;
                    let var316 = mload16!(var3 as usize + 481) as i32;
                    let var317 = mload8!(var3 as usize + 483) as i32;
                    var8 = var316 | var317.wrapping_shl(16 as u32);
                    let var318 = mload16!(var3 as usize + 501) as i32;
                    let var319 = mload8!(var3.wrapping_add(503) as usize) as i32;
                    var9 = var318 | var319.wrapping_shl(16 as u32);
                    let var320 = mload8!(var3 as usize + 512) as i32;
                    var12 = var320;
                    arg2 = slot_var3_504_i64;
                    let var321 = mload8!(var3 as usize + 500) as i32;
                    var13 = var321;
                    var15 = slot_var3_496_i32;
                    var41 = slot_var3_488_i32;
                    var5 = slot_var3_484_i32;
                    break 'label25;
                    break;
                }
                var41 = var17 as u32 as i64 | (var7 as u32 as i64).wrapping_shl(32 as u32);
                break;
            }
            mstore64!(arg0 as usize + 33, slot_var3_320_i64 as u64);
            mstore8!(arg0 as usize + 32, var12 as u8);
            mstore64!(arg0 as usize + 24, arg2 as u64);
            mstore64!(arg0 as usize + 8, var41 as u64);
            mstore8!(arg0 as usize, var6 as u8);
            mstore64!(arg0.wrapping_add(40) as usize, slot_var3_327_i64 as u64);
            mstore8!(arg0.wrapping_add(7) as usize, (var5 as u32).wrapping_shr(24 as u32) as i32 as u32 as i64 as u8);
            mstore16!(arg0.wrapping_add(5) as usize, (var5 as u32).wrapping_shr(8 as u32) as i32 as u32 as i64 as u16);
            mstore32!(arg0 as usize + 1, (var5.wrapping_shl(24 as u32) as u32 as i64 | var8 as u32 as i64 & 16777215) as u32);
            mstore64!(arg0 as usize + 16, (var15 as u32 as i64 | (var9 as u32 as i64).wrapping_shl(40 as u32) | (var13 as u32 as i64 & 255).wrapping_shl(32 as u32)) as u64);
            break;
        }
        self.global0 = var3.wrapping_add(576);
    }
    fn decode_entries_from_payload(&mut self, env: &Env, feed_ids: i64, payload: i64) -> Result<i64, i64> {
        const FRAME_SIZE: i32 = 96;
        const ERR_PTR_OFF: i32 = 64;
        const ELEM_PTR_OFF: i32 = 80;
        let frame = self.global0.wrapping_sub(FRAME_SIZE);
        self.global0 = frame;
        self.{FUNC_NAME}(env, frame, feed_ids, payload);
        let err_ptr = frame.wrapping_add(ERR_PTR_OFF);
        self.decode_val_or_error(env, err_ptr, frame);
        let err_flag = mload32!(err_ptr as usize) as i32;
        if err_flag != 0 {
            self.global0 = frame.wrapping_add(FRAME_SIZE);
            return Err(payload);
        }
        let entries_val = mload64!(frame.wrapping_add(ELEM_PTR_OFF) as usize) as i64;
        self.global0 = frame.wrapping_add(FRAME_SIZE);
        Ok(entries_val)
    }
