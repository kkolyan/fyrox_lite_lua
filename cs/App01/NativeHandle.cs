using System.Runtime.InteropServices;

namespace FyroxLite;

[StructLayout(LayoutKind.Sequential)]
public struct NativeHandle {
    public ulong high;
    public ulong low;
}
