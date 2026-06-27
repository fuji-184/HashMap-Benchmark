const std = @import("std");

// Deklarasi fungsi FFI dari libf_count.so
extern "c" fn init() i32;
extern "c" fn start() void;
extern "c" fn stop_and_print() void;

pub fn main() !void {
    var N: i32 = 0;

    const fd = std.posix.open("../input.txt", .{}, 0) catch |err| {
        std.debug.print("Gagal membuka file input.txt: {}\n", .{err});
        return err;
    };
    defer std.posix.close(fd);

    var buf: [16]u8 = undefined;
    const bytes_read = try std.posix.read(fd, &buf);
    const trimmed = std.mem.trim(u8, buf[0..bytes_read], &std.ascii.whitespace);
    N = try std.fmt.parseInt(i32, trimmed, 10);

    // Inisialisasi perf counter
    const init_status = init();
    std.debug.print("Status Init: {d}\n", .{init_status});
    if (init_status != 0) {
        return error.PerfInitFailed;
    }

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const gpa_allocator = gpa.allocator();

    var get: i64 = 0;                                  
    var m = std.AutoHashMap(i64, i64).init(gpa_allocator);
    defer m.deinit();

    std.debug.print("\n=== BENCHMARK: INSERT ===", .{});
    start();
    var i: i64 = 0;
    while (i < N) : (i += 1) {
        try m.put(i, i);
    }
    stop_and_print();

    std.debug.print("\n=== BENCHMARK: GET HIT ===", .{});
    start();
    i = 0;
    while (i < N) : (i += 1) {
        if (m.get(i)) |val| {
            get += val;
        }
    }
    stop_and_print();

    var get2: i64 = 0;
    std.debug.print("\n=== BENCHMARK: GET MISS ===", .{});
    start();
    i = N;
    while (i < N * 2) : (i += 1) {
        if (m.get(i)) |val| {
            get2 += val;
        }
    }
    stop_and_print();

    std.debug.print("\n=== BENCHMARK: UPDATE ===", .{});
    start();
    i = 0;
    while (i < N) : (i += 1) {
        try m.put(i, i * 2);
    }
    stop_and_print();

    std.debug.print("\n=== BENCHMARK: DELETE ===", .{});
    start();
    i = 0;
    while (i < N) : (i += 1) {
        _ = m.remove(i);
    }
    stop_and_print();

    std.debug.print("\nN: {d}\n", .{N});
    std.debug.print("Get: {d}\n", .{get + get2});
}
