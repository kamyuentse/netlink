#[allow(unused_attributes)]
#[rustfmt_skip]
lazy_static! {
    static ref BYTES: Vec<u8> = vec!(
        // Header
        0x40, 0x03, 0x00, 0x00, // length = 0x340 = 832
        0x10, 0x00,             // message type = 0x10 = 17 = RTM_NEWLINK
        0x02, 0x00,             // flags = multipart
        0x01, 0x00, 0x00, 0x00, // seq number = 1
        0xcf, 0x72, 0x00, 0x00, // port_id
            // link message header
            0x00,                   // address family
            0x00,                   // reserved
            0x0a, 0x03,             // link layer type = 778 (gre over ip)
            0x07, 0x00, 0x00, 0x00, // interface index = 7
            0x80, 0x00, 0x00, 0x00, // device flags = NOARP
            0x00, 0x00, 0x00, 0x00, // reserved
                // nlas
                0x09, 0x00, 0x03, 0x00, 0x67, 0x72, 0x65, 0x30, 0x00,   // device name gre0
                0x00, 0x00, 0x00,                                       // padding
                0x08, 0x00, 0x0d, 0x00, 0xe8, 0x03, 0x00, 0x00,         // txqueue length = 1000
                0x05, 0x00, 0x10, 0x00, 0x02,                           // oper state = down (2)
                0x00, 0x00, 0x00,                                       // padding
                0x05, 0x00, 0x11, 0x00, 0x00,                           // link mode = 17
                0x00, 0x00, 0x00,                                       // padding
                0x08, 0x00, 0x04, 0x00, 0xc4, 0x05, 0x00, 0x00,         // MTU = 1476
                0x08, 0x00, 0x1b, 0x00, 0x00, 0x00, 0x00, 0x00,         // group = 0
                0x08, 0x00, 0x1e, 0x00, 0x00, 0x00, 0x00, 0x00,         // promiscuity = 0
                0x08, 0x00, 0x1f, 0x00, 0x01, 0x00, 0x00, 0x00,         // nb tx queues = 1
                0x08, 0x00, 0x28, 0x00, 0xff, 0xff, 0x00, 0x00,         // max gso segment count = 0xffff
                0x08, 0x00, 0x29, 0x00, 0x00, 0x00, 0x01, 0x00,         // max gso size = 0x10000
                0x08, 0x00, 0x20, 0x00, 0x01, 0x00, 0x00, 0x00,         // nb rx queues = 1
                0x08, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00,         // link type = 0
                0x05, 0x00, 0x21, 0x00, 0x01,                           // carrier = 1
                0x00, 0x00, 0x00,                                       // padding
                0x09, 0x00, 0x06, 0x00, 0x6e, 0x6f, 0x6f, 0x70, 0x00,   // queue discipline = noop
                0x00, 0x00, 0x00,                                       // padding
                0x08, 0x00, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00,         // carrier change = 0
                0x05, 0x00, 0x27, 0x00, 0x00,                           // proto down = 0
                0x00, 0x00, 0x00,                                       // padding
                0x08, 0x00, 0x2f, 0x00, 0x00, 0x00, 0x00, 0x00,         // carrier up count = 0, 0, 0, 0
                0x08, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00,         // carrier down count = 0, 0, 0, 0
                // map
                0x24, 0x00, 0x0e, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,         // mem start
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,         // mem end
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,         // base address
                0x00, 0x00,                                             // irq
                0x00,                                                   // dma
                0x00,                                                   // port
                0x00, 0x00, 0x00, 0x00, // Padding? 

                0x08, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,         // hw address (it's an ipv4, not a mac!)
                0x08, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,         // broadcast address
                // stats
                0xc4, 0x00, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
                // interface stats
                0x64, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
                0x0c, 0x00, 0x2b, 0x00, 0x05, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, // XDP
                // link info
                0x98, 0x00, 0x12, 0x00, 0x08, 0x00, 0x01, 0x00, 0x67, 0x72, 0x65, 0x00, 0x8c, 0x00, 0x02, 0x00,
                0x08, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x06, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x08, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x08, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x05, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x08, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x06, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x06, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x13, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x05, 0x00, 0x16, 0x00, 0x00, 0x00, 0x00, 0x00,
                // AF_SPEC
                0x88, 0x00, 0x1a, 0x00, 0x84, 0x00, 0x02, 0x00, 0x80, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
                0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x10, 0x27, 0x00, 0x00, 0xe8, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,

        // XXX: other packet
        0xb0, 0x05, 0x00, 0x00,
        0x10, 0x00, 0x02, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0xcf, 0x72, 0x00, 0x00,
        0x00, 0x00, 0x01, 0x00,
        0x08, 0x00, 0x00, 0x00,
        0x02, 0x10, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x0c, 0x00, 0x03, 0x00,
        0x67, 0x72, 0x65, 0x74,
        0x61, 0x70, 0x30, 0x00,
        0x08, 0x00, 0x0d, 0x00,
        0xe8, 0x03, 0x00, 0x00,
        0x05, 0x00, 0x10, 0x00,
        0x02, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x11, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x04, 0x00,
        0xb6, 0x05, 0x00, 0x00,
        0x08, 0x00, 0x1b, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x1e, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x1f, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x28, 0x00,
        0xff, 0xff, 0x00, 0x00,
        0x08, 0x00, 0x29, 0x00,
        0x00, 0x00, 0x01, 0x00,
        0x08, 0x00, 0x20, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x05, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x21, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x09, 0x00, 0x06, 0x00,
        0x6e, 0x6f, 0x6f, 0x70,
        0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x23, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x27, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x2f, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x30, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x24, 0x00, 0x0e, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x0a, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x0a, 0x00, 0x02, 0x00,
        0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0x00, 0x00,
        0xc4, 0x00, 0x17, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x64, 0x00, 0x07, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x0c, 0x00, 0x2b, 0x00,
        0x05, 0x00, 0x02, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x9c, 0x00, 0x12, 0x00,
        0x0b, 0x00, 0x01, 0x00,
        0x67, 0x72, 0x65, 0x74,
        0x61, 0x70, 0x00, 0x00,
        0x8c, 0x00, 0x02, 0x00,
        0x08, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x06, 0x00, 0x02, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x06, 0x00, 0x03, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x04, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x05, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x06, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x07, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x08, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x09, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x0a, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x14, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x06, 0x00, 0x0e, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x06, 0x00, 0x10, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x06, 0x00, 0x11, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x06, 0x00, 0x0f, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x13, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x16, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0xec, 0x02, 0x1a, 0x00,
        0x84, 0x00, 0x02, 0x00,
        0x80, 0x00, 0x01, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x10, 0x27, 0x00, 0x00,
        0xe8, 0x03, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x64, 0x02, 0x0a, 0x00,
        0x08, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x14, 0x00, 0x05, 0x00,
        0xff, 0xff, 0x00, 0x00,
        0x3d, 0x2a, 0x02, 0x00,
        0x92, 0x40, 0x00, 0x00,
        0xe8, 0x03, 0x00, 0x00,
        0xd0, 0x00, 0x02, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x40, 0x00, 0x00, 0x00,
        0xdc, 0x05, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0xff, 0xff, 0xff, 0xff,
        0xa0, 0x0f, 0x00, 0x00,
        0xe8, 0x03, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x80, 0x3a, 0x09, 0x00,
        0x80, 0x51, 0x01, 0x00,
        0x03, 0x00, 0x00, 0x00,
        0x58, 0x02, 0x00, 0x00,
        0x10, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x60, 0xea, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x10, 0x27, 0x00, 0x00,
        0xe8, 0x03, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x80, 0xee, 0x36, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x24, 0x01, 0x03, 0x00,
        0x24, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x34, 0x00, 0x06, 0x00,
        0x06, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x14, 0x00, 0x07, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x08, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0xb0, 0x05, 0x00, 0x00,
        0x10, 0x00, 0x02, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0xcf, 0x72, 0x00, 0x00,
        0x00, 0x00, 0x01, 0x00,
        0x09, 0x00, 0x00, 0x00,
        0x02, 0x10, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x0c, 0x00, 0x03, 0x00,
        0x65, 0x72, 0x73, 0x70,
        0x61, 0x6e, 0x30, 0x00,
        0x08, 0x00, 0x0d, 0x00,
        0xe8, 0x03, 0x00, 0x00,
        0x05, 0x00, 0x10, 0x00,
        0x02, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x11, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x04, 0x00,
        0xa6, 0x05, 0x00, 0x00,
        0x08, 0x00, 0x1b, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x1e, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x1f, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x28, 0x00,
        0xff, 0xff, 0x00, 0x00,
        0x08, 0x00, 0x29, 0x00,
        0x00, 0x00, 0x01, 0x00,
        0x08, 0x00, 0x20, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x05, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x21, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x09, 0x00, 0x06, 0x00,
        0x6e, 0x6f, 0x6f, 0x70,
        0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x23, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x27, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x2f, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x30, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x24, 0x00, 0x0e, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x0a, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x0a, 0x00, 0x02, 0x00,
        0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0x00, 0x00,
        0xc4, 0x00, 0x17, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x64, 0x00, 0x07, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x0c, 0x00, 0x2b, 0x00,
        0x05, 0x00, 0x02, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x9c, 0x00, 0x12, 0x00,
        0x0b, 0x00, 0x01, 0x00,
        0x65, 0x72, 0x73, 0x70,
        0x61, 0x6e, 0x00, 0x00,
        0x8c, 0x00, 0x02, 0x00,
        0x08, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x06, 0x00, 0x02, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x06, 0x00, 0x03, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x04, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x05, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x06, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x07, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x08, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x09, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x0a, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x14, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x06, 0x00, 0x0e, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x06, 0x00, 0x10, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x06, 0x00, 0x11, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x06, 0x00, 0x0f, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x13, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x16, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0xec, 0x02, 0x1a, 0x00,
        0x84, 0x00, 0x02, 0x00,
        0x80, 0x00, 0x01, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x10, 0x27, 0x00, 0x00,
        0xe8, 0x03, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x64, 0x02, 0x0a, 0x00,
        0x08, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x14, 0x00, 0x05, 0x00,
        0xff, 0xff, 0x00, 0x00,
        0x3d, 0x2a, 0x02, 0x00,
        0x1a, 0x7d, 0x00, 0x00,
        0xe8, 0x03, 0x00, 0x00,
        0xd0, 0x00, 0x02, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x40, 0x00, 0x00, 0x00,
        0xdc, 0x05, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0xff, 0xff, 0xff, 0xff,
        0xa0, 0x0f, 0x00, 0x00,
        0xe8, 0x03, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x80, 0x3a, 0x09, 0x00,
        0x80, 0x51, 0x01, 0x00,
        0x03, 0x00, 0x00, 0x00,
        0x58, 0x02, 0x00, 0x00,
        0x10, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x60, 0xea, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x10, 0x27, 0x00, 0x00,
        0xe8, 0x03, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x80, 0xee, 0x36, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x24, 0x01, 0x03, 0x00,
        0x24, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x34, 0x00, 0x06, 0x00,
        0x06, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x14, 0x00, 0x07, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x08, 0x00,
        0x00, 0x00, 0x00, 0x00);
}

use packet::link::LinkLayerType;
use packet::link::LinkMessageBuffer;
use packet::link::LinkNla;
use packet::link::Map;
use packet::link::IFF_NOARP;
use packet::Buffer;
use packet::Error;
use packet::MessageType;
use packet::Nla;

#[test]
fn truncated() {
    assert_eq!(Buffer::new_checked(&BYTES[..831]), Err(Error::Truncated));
    assert!(Buffer::new_checked(&BYTES[..832]).is_ok());
}
#[test]
fn decode_all() {
    let header_buf = Buffer::new_checked(&BYTES[..832]).unwrap();
    assert_eq!(header_buf.length(), 0x340);
    assert_eq!(header_buf.message_type(), MessageType::NewLink);
    assert!(header_buf.flags().has_multipart());
    assert_eq!(header_buf.sequence_number(), 1);

    // check the payload length and the last bytes of the payload, to make sure our calculations
    // are right.
    let payload = header_buf.payload();
    assert_eq!(payload.len(), 832 - 16);
    assert_eq!(
        payload[800..816].to_vec(),
        vec![
            0xe8, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ]
    );

    let newlink_buf = LinkMessageBuffer::new(payload);
    assert_eq!(newlink_buf.address_family(), 0);
    assert_eq!(newlink_buf.reserved_1(), 0);
    assert_eq!(newlink_buf.link_layer_type(), LinkLayerType::IpGre);
    assert_eq!(newlink_buf.link_index(), 7);
    assert_eq!(newlink_buf.flags().0, IFF_NOARP);

    let mut nlas = newlink_buf.nlas();

    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    assert_eq!(nla, LinkNla::IfName("gre0".into()));

    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    assert_eq!(nla, LinkNla::TxQueueLen(1000));

    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    assert_eq!(nla, LinkNla::OperState(2));

    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    assert_eq!(nla, LinkNla::LinkMode(0));

    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    assert_eq!(nla, LinkNla::Mtu(1476));

    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    assert_eq!(nla, LinkNla::Group(0));

    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    assert_eq!(nla, LinkNla::Promiscuity(0));

    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    assert_eq!(nla, LinkNla::NumTxQueues(1));

    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    assert_eq!(nla, LinkNla::GsoMaxSegs(0xffff));

    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    assert_eq!(nla, LinkNla::GsoMaxSize(0x1_0000));

    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    assert_eq!(nla, LinkNla::NumRxQueues(1));

    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    assert_eq!(nla, LinkNla::Link(0));

    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    assert_eq!(nla, LinkNla::Carrier(1));

    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    assert_eq!(nla, LinkNla::Qdisc("noop".into()));

    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    assert_eq!(nla, LinkNla::CarrierChanges(0));

    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    assert_eq!(nla, LinkNla::ProtoDown(0));

    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    assert_eq!(nla, LinkNla::CarrierUpCount(vec![0, 0, 0, 0]));

    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    assert_eq!(nla, LinkNla::CarrierDownCount(vec![0, 0, 0, 0]));

    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    // MAP
    assert_eq!(
        nla,
        LinkNla::Map(Map {
            memory_start: 0,
            memory_end: 0,
            base_address: 0,
            irq: 0,
            dma: 0,
            port: 0,
        })
    );

    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    assert_eq!(nla, LinkNla::Address(vec![0, 0, 0, 0]));

    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    assert_eq!(nla, LinkNla::Broadcast(vec![0, 0, 0, 0]));

    // skip the stats and interface stats
    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    assert_eq!(nla.value_len(), 192);
    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    assert_eq!(nla.value_len(), 96);

    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    assert_eq!(nla, LinkNla::Xdp(vec![5, 0, 2, 0, 0, 0, 0, 0]));

    // link info
    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    assert_eq!(nla.value_len(), 148);

    // af_spec
    let nla = LinkNla::parse(&nlas.next().unwrap().unwrap()).unwrap();
    // XXX: for af_spec, the packet we captured take the padding into account which is why the
    // value is 132 bytes long. But out parsing ignores the padding, so it's only 128 bytes long
    assert_eq!(nla.value_len(), 128);

    assert!(nlas.next().is_none());
}