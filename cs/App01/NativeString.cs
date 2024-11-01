using System.Runtime.InteropServices;
using System.Text;

namespace FyroxLite;

public partial struct NativeString
{
    
    private unsafe byte* begin;
    private int length;
    internal List<byte>? Fetched;

    internal unsafe byte_slice(byte* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe void Fetch(ref byte_slice self)
    {
        var fetched = new List<byte>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        self.Fetched = fetched;
    }

    internal static unsafe string ToFacade(in NativeString self)
    {
        return Encoding.UTF8.GetString(self.begin, self.length);
    }

    internal static byte_slice FromFacade(in string self)
    {
        var bytes = Encoding.UTF8.GetBytes();
        Marshal.AllocHGlobal()
        // __item
        throw new Exception("slice serialization not implemented yet");
    }
    
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial byte_slice fyrox_lite_lite_physics_LitePhysics_cast_ray(int length);
    
}