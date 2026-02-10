pub struct FingerprintEntry {
    pub hash: u64,
    pub suggested_name: &'static str,
}

const ENTRIES: &[FingerprintEntry] = &[
    FingerprintEntry {
        hash: 0xb1d5ab9d54552386,
        suggested_name: "entry_decode",
    },
    FingerprintEntry {
        hash: 0x1009eee0145251ab,
        suggested_name: "vec_pair_iter",
    },
    FingerprintEntry {
        hash: 0x879c5f6c8e7ccaf7,
        suggested_name: "decode_val_or_error",
    },
    FingerprintEntry {
        hash: 0xdb7c15da4a0100b1,
        suggested_name: "copy_val_if_present",
    },
    FingerprintEntry {
        hash: 0x4dc2a27373875c29,
        suggested_name: "write_ok_val",
    },
    FingerprintEntry {
        hash: 0x7123f936ad42eb61,
        suggested_name: "storage_get_val",
    },
    FingerprintEntry {
        hash: 0xa2a432aa2d1fc87d,
        suggested_name: "map_unpack_to_val",
    },
    FingerprintEntry {
        hash: 0xb6b622399f6cd662,
        suggested_name: "val_to_i64_checked",
    },
    FingerprintEntry {
        hash: 0x0b46848e2c70879d,
        suggested_name: "check_recent_timestamp",
    },
    FingerprintEntry {
        hash: 0xcb8bc1b0363e5c87,
        suggested_name: "storage_set_val",
    },
    FingerprintEntry {
        hash: 0x0bba25866190612f,
        suggested_name: "storage_key_from_str",
    },
    FingerprintEntry {
        hash: 0x9d0bda6242cf05dd,
        suggested_name: "result_unwrap_or_panic",
    },
    FingerprintEntry {
        hash: 0xd9b6ed776a84028c,
        suggested_name: "result_from_val_unchecked",
    },
    FingerprintEntry {
        hash: 0xa1ffe0a52367d6e1,
        suggested_name: "result_from_val",
    },
    FingerprintEntry {
        hash: 0x937cf98725c5ce93,
        suggested_name: "require_owner_auth",
    },
    FingerprintEntry {
        hash: 0x9a262c0072f8c409,
        suggested_name: "require_auth_for_key",
    },
    FingerprintEntry {
        hash: 0x4272f3bcef47b5f6,
        suggested_name: "read_prices_loop",
    },
    FingerprintEntry {
        hash: 0x9e5b7725c3501de7,
        suggested_name: "write_prices_impl",
    },
    FingerprintEntry {
        hash: 0xda81fcbe9ede3f70,
        suggested_name: "map_new_val",
    },
    FingerprintEntry {
        hash: 0x7896ef8f57401c71,
        suggested_name: "init_owner_internal",
    },
    FingerprintEntry {
        hash: 0xfe1f3f06a749dd3c,
        suggested_name: "set_pending_owner_internal",
    },
    FingerprintEntry {
        hash: 0x65894f641b4e7637,
        suggested_name: "accept_ownership_internal",
    },
    FingerprintEntry {
        hash: 0x99ee79378e08c33d,
        suggested_name: "storage_remove_val",
    },
    FingerprintEntry {
        hash: 0x950c57d29576441f,
        suggested_name: "cancel_pending_owner_internal",
    },
    FingerprintEntry {
        hash: 0x10e1cafa5cc73328,
        suggested_name: "upgrade_wasm_internal",
    },
    FingerprintEntry {
        hash: 0x30fb0e969eaddad1,
        suggested_name: "read_timestamp_val",
    },
    FingerprintEntry {
        hash: 0xc01829002e3dc9f7,
        suggested_name: "read_price_data_for_feed_val",
    },
    FingerprintEntry {
        hash: 0xe2f2ba431b096cb6,
        suggested_name: "check_price_data_val",
    },
    FingerprintEntry {
        hash: 0x647b009204d6332d,
        suggested_name: "decode_error_from_val",
    },
    FingerprintEntry {
        hash: 0x1f151b05224cb69f,
        suggested_name: "ledger_timestamp_val",
    },
    FingerprintEntry {
        hash: 0x5061910c5e571ce1,
        suggested_name: "bytes_to_fixed32_struct",
    },
    FingerprintEntry {
        hash: 0x00b4ff08e04e697f,
        suggested_name: "get_prices_impl",
    },
    FingerprintEntry {
        hash: 0x86b014f96e71aec5,
        suggested_name: "dispatch_table",
    },
    FingerprintEntry {
        hash: 0x9acb1bdbb4f5953e,
        suggested_name: "alloc_trap",
    },
    FingerprintEntry {
        hash: 0x0b022d3cb5c62c80,
        suggested_name: "alloc_realloc",
    },
    FingerprintEntry {
        hash: 0x07026fc0ea266fa7,
        suggested_name: "alloc_size_align",
    },
    FingerprintEntry {
        hash: 0xa925555186d08445,
        suggested_name: "alloc_core",
    },
    FingerprintEntry {
        hash: 0x07699f75c970dbb6,
        suggested_name: "build_entry_from_bytes",
    },
    FingerprintEntry {
        hash: 0x627d5b4ca0d455a5,
        suggested_name: "entry_match_copy",
    },
    FingerprintEntry {
        hash: 0x0452222ee75faff8,
        suggested_name: "entry_copy_if_ok",
    },
    FingerprintEntry {
        hash: 0x3bcac7da12f21925,
        suggested_name: "entry_from_bytes_val",
    },
    FingerprintEntry {
        hash: 0xc5d0fa81f6ec8912,
        suggested_name: "alloc_range_fill",
    },
    FingerprintEntry {
        hash: 0x81246b72c9f87605,
        suggested_name: "alloc_range",
    },
    FingerprintEntry {
        hash: 0x263e051d2f5ebb27,
        suggested_name: "alloc_range_one",
    },
    FingerprintEntry {
        hash: 0xec0e4fe013b81985,
        suggested_name: "alloc_copy",
    },
    FingerprintEntry {
        hash: 0xf7d64f41080eec9b,
        suggested_name: "require_alloc",
    },
    FingerprintEntry {
        hash: 0x24749d211cc73413,
        suggested_name: "span_from_range",
    },
    FingerprintEntry {
        hash: 0x12d3e998ce20e0f3,
        suggested_name: "span_take",
    },
    FingerprintEntry {
        hash: 0x10d6fbb841cd9d43,
        suggested_name: "span_set",
    },
    FingerprintEntry {
        hash: 0x1847b78abdb23830,
        suggested_name: "memcpy_checked",
    },
    FingerprintEntry {
        hash: 0xf5e1308c33e7f13d,
        suggested_name: "memmove",
    },
    FingerprintEntry {
        hash: 0x3cc5936dca5012a7,
        suggested_name: "memcmp",
    },
    FingerprintEntry {
        hash: 0xca687ee8429aafd5,
        suggested_name: "memcmp_sign32",
    },
    FingerprintEntry {
        hash: 0x2562423c21425ae1,
        suggested_name: "memeq32",
    },
    FingerprintEntry {
        hash: 0xb7569875fc80cae4,
        suggested_name: "ptr_index32",
    },
    FingerprintEntry {
        hash: 0x9050d9a54155ed34,
        suggested_name: "copy_to_linear_memory",
    },
    FingerprintEntry {
        hash: 0x62de53692a71ca0a,
        suggested_name: "vec_next_string_flag",
    },
    FingerprintEntry {
        hash: 0x6154e70a5ff920c0,
        suggested_name: "guard_nonzero_ptr",
    },
    FingerprintEntry {
        hash: 0x0974f7c88ff97554,
        suggested_name: "vec_next_string_to_linear",
    },
    FingerprintEntry {
        hash: 0x6a35c4e8ca5e1033,
        suggested_name: "bytes_to_fixed32",
    },
    FingerprintEntry {
        hash: 0x256576a6ea8f6e64,
        suggested_name: "pack_u32_call_import",
    },
    FingerprintEntry {
        hash: 0x653f3babe0f53681,
        suggested_name: "call_eq_one",
    },
    FingerprintEntry {
        hash: 0x364ac65856297d45,
        suggested_name: "call_unreachable",
    },
    FingerprintEntry {
        hash: 0x953734e6ccf68b58,
        suggested_name: "unreachable_stub",
    },
    FingerprintEntry {
        hash: 0x4855642fa55278c5,
        suggested_name: "metadata_const",
    },
    FingerprintEntry {
        hash: 0x819b4f325e09747a,
        suggested_name: "map_new_val",
    },
    FingerprintEntry {
        hash: 0x723d448934b55a4d,
        suggested_name: "vec_pair_builder",
    },
    FingerprintEntry {
        hash: 0x2d306f27650054bb,
        suggested_name: "decode_i128_parts",
    },
    FingerprintEntry {
        hash: 0x0ce72655a8b1bcd5,
        suggested_name: "ledger_sequence_u32",
    },
    FingerprintEntry {
        hash: 0xf07ae05ea744eebe,
        suggested_name: "pack_i128_val",
    },
    FingerprintEntry {
        hash: 0x0a57abb8d800e027,
        suggested_name: "spend_allowance",
    },
    FingerprintEntry {
        hash: 0x6339c4d7de17febc,
        suggested_name: "i128_parts_to_val",
    },
    FingerprintEntry {
        hash: 0xcb71620ad6e87dc0,
        suggested_name: "credit_balance",
    },
    FingerprintEntry {
        hash: 0xf7a69f1021768c3a,
        suggested_name: "debit_balance",
    },
    FingerprintEntry {
        hash: 0x0b51a052bcad8c9c,
        suggested_name: "require_nonnegative_i128",
    },
    FingerprintEntry {
        hash: 0xd3de3686ca37c414,
        suggested_name: "bump_instance_ttl",
    },
    FingerprintEntry {
        hash: 0xdd1f19f3b17fd52e,
        suggested_name: "event_topic_pair",
    },
    FingerprintEntry {
        hash: 0xc3f3102f92c17b79,
        suggested_name: "event_topic_triplet",
    },
    FingerprintEntry {
        hash: 0x636bd338efbe8f31,
        suggested_name: "publish_transfer_event",
    },
    FingerprintEntry {
        hash: 0xa86da469508afa6f,
        suggested_name: "publish_burn_event",
    },
    FingerprintEntry {
        hash: 0x311152eb6c4f0e3c,
        suggested_name: "u64_to_val",
    },
    FingerprintEntry {
        hash: 0x83567199dbfa6b76,
        suggested_name: "fail_with_error",
    },
    FingerprintEntry {
        hash: 0x3a59ad93af338e74,
        suggested_name: "decode_u64_from_val",
    },
    FingerprintEntry {
        hash: 0xffb21e3d292ae614,
        suggested_name: "decode_i128_parts",
    },
    FingerprintEntry {
        hash: 0xd0828ce533ea1d59,
        suggested_name: "storage_has_data_key",
    },
    FingerprintEntry {
        hash: 0x27aba27c28f9f4d9,
        suggested_name: "event_topic_self_pair",
    },
    FingerprintEntry {
        hash: 0xcb2464ca45755f94,
        suggested_name: "ledger_timestamp_u64",
    },
    FingerprintEntry {
        hash: 0x5dea95607b7a91e6,
        suggested_name: "invoke_contract_void",
    },
    FingerprintEntry {
        hash: 0x75d8485f9ec99925,
        suggested_name: "data_key_to_val",
    },
    FingerprintEntry {
        hash: 0x9f48c5c5a88ceeec,
        suggested_name: "contract_info_to_val",
    },
    FingerprintEntry {
        hash: 0x92ae8c0024c5217a,
        suggested_name: "pack_i128_val",
    },
    FingerprintEntry {
        hash: 0xb95f16f0b12a3030,
        suggested_name: "launch_record_to_val",
    },
    FingerprintEntry {
        hash: 0xa51ce7193772156e,
        suggested_name: "write_launch_key",
    },
    FingerprintEntry {
        hash: 0x720763d0fbbd3b3e,
        suggested_name: "launch_key_event_payload",
    },
    FingerprintEntry {
        hash: 0xe09e9ff933ea6ac2,
        suggested_name: "launch_snapshot_to_val",
    },
    FingerprintEntry {
        hash: 0x583aa3ce7b16ea8c,
        suggested_name: "check_launch_state",
    },
    FingerprintEntry {
        hash: 0xf855585deee73ff3,
        suggested_name: "compute_reward_and_fees",
    },
    FingerprintEntry {
        hash: 0x4f543c99ae62c910,
        suggested_name: "claim_launch_balance_impl",
    },
    FingerprintEntry {
        hash: 0x970eaf5dd96453ca,
        suggested_name: "token_transfer_checked",
    },
    FingerprintEntry {
        hash: 0xb4cbe5da437d2cb1,
        suggested_name: "decode_launch_from_storage",
    },
    FingerprintEntry {
        hash: 0xb1ba1df3a51582bb,
        suggested_name: "persist_launch_state",
    },
    FingerprintEntry {
        hash: 0x053e64cb3bfcf756,
        suggested_name: "buy_flow_impl",
    },
    FingerprintEntry {
        hash: 0xff99b957925a294d,
        suggested_name: "verify_launch_window",
    },
    FingerprintEntry {
        hash: 0x619300ff6d14db31,
        suggested_name: "sell_flow_impl",
    },
    FingerprintEntry {
        hash: 0x2a5af3f986d16dc9,
        suggested_name: "claim_flow_impl",
    },
    FingerprintEntry {
        hash: 0x0db294ab5049420d,
        suggested_name: "nop",
    },
];

pub fn suggested_name(hash: u64) -> Option<&'static str> {
    ENTRIES
        .iter()
        .find(|entry| entry.hash == hash)
        .map(|entry| entry.suggested_name)
}
