const std = @import("std");

extern "c" fn init() i32;
extern "c" fn start() void;
extern "c" fn stop_and_print() void;

fn readStrings(allocator: std.mem.Allocator, io: std.Io, path: []const u8) ![][]u8 {
    const file = try std.Io.Dir.cwd().openFile(io, path, .{});
    defer file.close(io);

    var read_buffer: [65536]u8 = undefined;
    var fr = file.reader(io, &read_buffer);

    var buf = std.ArrayListUnmanaged(u8).empty;
    defer buf.deinit(allocator);
    try fr.interface.appendRemaining(allocator, &buf, .unlimited);

    const data = buf.items;
    const n = std.mem.readInt(u64, data[0..8], .little);
    var strings = try allocator.alloc([]u8, n);
    var pos: usize = 8;
    for (0..n) |idx| {
        const len = std.mem.readInt(u32, data[pos..][0..4], .little);
        pos += 4;
        const s = try allocator.dupe(u8, data[pos .. pos + len]);
        pos += len;
        strings[idx] = s;
    }
    return strings;
}

fn reverseString(allocator: std.mem.Allocator, s: []const u8) ![]u8 {
    const result = try allocator.dupe(u8, s);
    std.mem.reverse(u8, result);
    return result;
}

pub fn main(process_init: std.process.Init) !void {
    const io = process_init.io;
    const allocator = std.heap.smp_allocator;

    const strings = try readStrings(allocator, io, "../input.bin");
    defer {
        for (strings) |s| allocator.free(s);
        allocator.free(strings);
    }

    const n = strings.len;
    const init_status = init();
    if (init_status == 0) {
        std.debug.print("Status Init: 0\n", .{});
    }

    var get: i64 = 0;
    var get2: i64 = 0;
    var m = std.StringHashMap(i64).init(allocator);
    defer m.deinit();

    std.debug.print("\n=== BENCHMARK: INSERT ===", .{});
    start();
    for (strings, 0..) |s, i| {
        try m.put(s, @intCast(i));
    }
    stop_and_print();

    std.debug.print("\n=== BENCHMARK: GET HIT ===", .{});
    start();
    for (strings) |s| {
        if (m.get(s)) |val| {
            get += val;
        }
    }
    stop_and_print();

    std.debug.print("\n=== BENCHMARK: GET MISS ===", .{});
    start();
    for (strings) |s| {
        const miss = try reverseString(allocator, s);
        defer allocator.free(miss);
        if (m.get(miss)) |val| {
            get2 += val;
        }
    }
    stop_and_print();

    std.debug.print("\n=== BENCHMARK: RE-INSERT ===", .{});
    start();
    for (strings, 0..) |s, i| {
        try m.put(s, @as(i64, @intCast(i)) * 2);
    }
    stop_and_print();

    std.debug.print("\n=== BENCHMARK: REMOVE ===", .{});
    start();
    for (strings) |s| {
        _ = m.remove(s);
    }
    stop_and_print();

    std.debug.print("\nN: {d}\n", .{n});
    std.debug.print("Get: {d}\n", .{get - get2});
}
