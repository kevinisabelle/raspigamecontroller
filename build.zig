const Builder = @import("std").build;

pub fn build(b: *Builder) void {
    // Define the build mode (e.g., Debug, ReleaseSafe, etc.)
    const mode = b.standardReleaseOptions();

    // Set the target program name
    const program_name = "hidkeyboard";

    // Define the source file path
    const source = "src/hidkeyboard.zig";

    // Define the sysroot path
    const sysroot = "sysrootrasp";

    // Create an executable target
    const exe = b.addExecutable(program_name, source);
    exe.setBuildMode(mode);

    // Set the target architecture and OS
    exe.setTarget(.{
        .arch = .arm,
        .os = .linux,
        .abi = .gnueabihf,
    });

    // Add sysroot configuration
    exe.setSystemRoot(sysroot);

    // Add libraries
    exe.linkLibC();
    exe.linkLib("bluetooth");
    exe.linkLib("pigpio");

    // Add include and library paths
    exe.addIncludePath(sysroot ++ "/usr/include");
    exe.addLibPath(sysroot ++ "/usr/lib");
    exe.addLibPath(sysroot ++ "/lib");
    exe.addLibPath(sysroot ++ "/lib/arm-linux-gnueabihf");

    // Set the output directory for the binary
    exe.setOutputDir("bin");

    // Enable reference trace debugging
    exe.addUserLandFlag("-freference-trace=4");

    // Install the binary
    exe.install();

    // Set the default target to be built
    b.default_step = "build";
}
