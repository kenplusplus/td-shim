// Copyright (c) 2021 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent

// Auto-generated by `td-layout-config`, do not edit manually.

/*
    Mem Layout:
                                            Address
                    +--------------+ <-  0x0
                    |     Legacy   |
                    +--------------+ <-  0x00100000 (1M)
                    |   ........   |
                    +--------------+ <-  0x00800000
                    |  Page Table  | <-  0x00800000
                    +--------------+ <-  0x01000000
                    |    TD HOB    |
                    +--------------+ <-  0x01100000
                    | PAYLOAD PARAM|    (0x00100000)
                    +--------------+ <-  0x01200000
                    |    PAYLOAD   |    (0x02000000)
                    +--------------+
                    |   ........   |
                    +--------------+ <-  0x7EFFE000
                    |     STACK    |    (0x00800000)
                    +--------------+ <-  0x7F7FE000
                    |      SS      |    (0x00200000)
                    +--------------+ <-  0x7F9FE000
                    |  PAYLOAD_HOB |    (0x00400000)
                    +--------------+ <-  0x7FDBE000
                    |  UNACCEPTED  |    (0x00040000)
                    +--------------+ <-  0x7FDFE000
                    |     ACPI     |    (0x00100000)
                    +--------------+ <-  0x7FEFE000
                    |    MAILBOX   |    (0x00002000)
                    +--------------+ <-  0x7FF00000
                    | TD_EVENT_LOG |    (0x00100000)
                    +--------------+ <-  0x80000000 (2G) - for example
*/

pub const TD_PAYLOAD_EVENT_LOG_SIZE: u32 = 0x100000;
pub const TD_PAYLOAD_ACPI_SIZE: u32 = 0x100000;
pub const TD_PAYLOAD_MAILBOX_SIZE: u32 = 0x2000;
pub const TD_PAYLOAD_UNACCEPTED_MEMORY_BITMAP_SIZE: u32 = 0x40000;
pub const TD_PAYLOAD_PARTIAL_ACCEPT_MEMORY_SIZE: u32 = 0x10000000;
pub const TD_PAYLOAD_HOB_SIZE: u32 = 0x400000;
pub const TD_PAYLOAD_SHADOW_STACK_SIZE: u32 = 0x200000;
pub const TD_PAYLOAD_STACK_SIZE: u32 = 0x800000;

pub const TD_PAYLOAD_PAGE_TABLE_BASE: u64 = 0x800000;
pub const TD_PAYLOAD_PAGE_TABLE_SIZE: usize = 0x800000;
pub const TD_HOB_BASE: u64 = 0x1000000;
pub const TD_HOB_SIZE: u64 = 0x20000;
pub const TD_PAYLOAD_PARAM_BASE: u64 = 0x1100000;
pub const TD_PAYLOAD_PARAM_SIZE: u64 = 0x100000;
pub const TD_PAYLOAD_BASE: u64 = 0x1200000;
pub const TD_PAYLOAD_SIZE: usize = 0x2000000;
